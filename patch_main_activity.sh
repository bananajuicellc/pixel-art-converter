#!/bin/bash
cat << 'INNER_EOF' > app/src/androidMain/kotlin/com/example/convertpixelart/MainActivity.kt
package com.example.convertpixelart

import android.content.Context
import android.content.Intent
import android.net.Uri
import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.rememberLauncherForActivityResult
import androidx.activity.compose.setContent
import androidx.activity.result.contract.ActivityResultContracts
import androidx.compose.foundation.layout.*
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.unit.dp
import androidx.core.content.FileProvider
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext
import java.io.File
import java.io.FileOutputStream

class MainActivity : ComponentActivity() {

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        // Handle incoming intent if it was started from "Share"
        val initialUri = if (intent?.action == Intent.ACTION_SEND) {
            intent.getParcelableExtra<Uri>(Intent.EXTRA_STREAM)
        } else null

        setContent {
            com.example.convertpixelart.ui.theme.ConvertPixelArtTheme {
                Surface(
                    modifier = Modifier.fillMaxSize(),
                    color = MaterialTheme.colorScheme.background
                ) {
                    ConverterScreen(initialUri = initialUri)
                }
            }
        }
    }
}

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun ConverterScreen(initialUri: Uri?) {
    val context = LocalContext.current
    val coroutineScope = rememberCoroutineScope()

    var selectedUri by remember { mutableStateOf(initialUri) }
    var selectedFormat by remember { mutableStateOf(".aseprite") }
    var extractTimelapse by remember { mutableStateOf(false) }
    var conversionStatus by remember { mutableStateOf("") }
    var outputFilePath by remember { mutableStateOf<String?>(null) }
    var isConverting by remember { mutableStateOf(false) }

    val filePickerLauncher = rememberLauncherForActivityResult(
        contract = ActivityResultContracts.OpenDocument()
    ) { uri: Uri? ->
        if (uri != null) {
            selectedUri = uri
            conversionStatus = ""
            outputFilePath = null
        }
    }

    Column(
        modifier = Modifier
            .fillMaxSize()
            .padding(16.dp),
        horizontalAlignment = Alignment.CenterHorizontally,
        verticalArrangement = Arrangement.spacedBy(16.dp)
    ) {
        Text("Convert Pixel Art", style = MaterialTheme.typography.headlineMedium)

        Button(onClick = { filePickerLauncher.launch(arrayOf("*/*")) }) {
            Text("Select Input File")
        }

        if (selectedUri != null) {
            Text("Selected File: ${selectedUri?.lastPathSegment ?: "Unknown"}")
        }

        Row(verticalAlignment = Alignment.CenterVertically) {
            Text("Output Format: ")
            var expanded by remember { mutableStateOf(false) }
            ExposedDropdownMenuBox(
                expanded = expanded,
                onExpandedChange = { expanded = it }
            ) {
                OutlinedTextField(
                    value = selectedFormat,
                    onValueChange = {},
                    readOnly = true,
                    trailingIcon = { ExposedDropdownMenuDefaults.TrailingIcon(expanded = expanded) },
                    modifier = Modifier.menuAnchor()
                )
                ExposedDropdownMenu(
                    expanded = expanded,
                    onDismissRequest = { expanded = false }
                ) {
                    listOf(".aseprite", ".ase", ".png").forEach { format ->
                        DropdownMenuItem(
                            text = { Text(format) },
                            onClick = {
                                selectedFormat = format
                                expanded = false
                            }
                        )
                    }
                }
            }
        }

        Row(verticalAlignment = Alignment.CenterVertically) {
            Checkbox(
                checked = extractTimelapse,
                onCheckedChange = { extractTimelapse = it }
            )
            Text("Extract Timelapse (for supported formats like .psp)")
        }

        Button(
            onClick = {
                selectedUri?.let { uri ->
                    isConverting = true
                    conversionStatus = "Converting..."
                    outputFilePath = null

                    coroutineScope.launch {
                        val result = performConversion(context, uri, selectedFormat, extractTimelapse)
                        isConverting = false
                        if (result.startsWith("Success|")) {
                            outputFilePath = result.split("|")[1]
                            conversionStatus = "Conversion successful!"
                        } else {
                            conversionStatus = result
                        }
                    }
                }
            },
            enabled = selectedUri != null && !isConverting
        ) {
            Text("Convert")
        }

        if (isConverting) {
            CircularProgressIndicator()
        }

        if (conversionStatus.isNotEmpty()) {
            Text(conversionStatus)
        }

        if (outputFilePath != null) {
            Button(onClick = { shareFile(context, File(outputFilePath!!)) }) {
                Text("Share / Save Output")
            }
        }
    }
}

suspend fun performConversion(context: Context, inputUri: Uri, outputFormat: String, timelapse: Boolean): String {
    return withContext(Dispatchers.IO) {
        try {
            // Copy input to a temporary local file so Rust can read it by path
            val inputStream = context.contentResolver.openInputStream(inputUri)
                ?: return@withContext "Error: Cannot open input file"

            val cacheDir = context.cacheDir
            val inputFileName = "temp_input" // In a real app we'd preserve extension to help the parser
            // Try to extract original name to preserve extension
            var ext = ".tmp"
            context.contentResolver.query(inputUri, null, null, null, null)?.use { cursor ->
                if (cursor.moveToFirst()) {
                    val displayNameIndex = cursor.getColumnIndex(android.provider.OpenableColumns.DISPLAY_NAME)
                    if (displayNameIndex != -1) {
                        val displayName = cursor.getString(displayNameIndex)
                        if (displayName.contains(".")) {
                            ext = displayName.substring(displayName.lastIndexOf("."))
                        }
                    }
                }
            }

            val tempInputFile = File(cacheDir, "temp_input$ext")
            FileOutputStream(tempInputFile).use { outputStream ->
                inputStream.copyTo(outputStream)
            }

            val tempOutputFile = File(cacheDir, "output$outputFormat")

            if (ext.equals(".pixaki", ignoreCase = true)) {
                val unzipDir = File(cacheDir, "temp_input_pixaki_dir")
                unzipDir.deleteRecursively()
                unzipDir.mkdirs()
                java.util.zip.ZipInputStream(java.io.FileInputStream(tempInputFile)).use { zis ->
                    var zipEntry = zis.nextEntry
                    while (zipEntry != null) {
                        val newFile = File(unzipDir, zipEntry.name)
                        if (zipEntry.isDirectory) {
                            newFile.mkdirs()
                        } else {
                            newFile.parentFile?.mkdirs()
                            java.io.FileOutputStream(newFile).use { fos -> zis.copyTo(fos) }
                        }
                        zipEntry = zis.nextEntry
                    }
                    zis.closeEntry()
                }
                val result = RustInterop.convertFile(unzipDir.absolutePath, tempOutputFile.absolutePath, timelapse)
                return@withContext if (result == "Success") "Success|${tempOutputFile.absolutePath}" else result
            }

            val result = RustInterop.convertFile(tempInputFile.absolutePath, tempOutputFile.absolutePath, timelapse)

            if (result == "Success") {
                "Success|${tempOutputFile.absolutePath}"
            } else {
                result
            }
        } catch (e: Exception) {
            "Error: ${e.message}"
        }
    }
}

fun shareFile(context: Context, file: File) {
    val uri = FileProvider.getUriForFile(
        context,
        "${context.packageName}.fileprovider",
        file
    )
    val intent = Intent(Intent.ACTION_SEND).apply {
        type = "application/octet-stream"
        putExtra(Intent.EXTRA_STREAM, uri)
        addFlags(Intent.FLAG_GRANT_READ_URI_PERMISSION)
    }
    context.startActivity(Intent.createChooser(intent, "Share output file"))
}
INNER_EOF
