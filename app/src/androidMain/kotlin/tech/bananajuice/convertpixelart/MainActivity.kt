package tech.bananajuice.convertpixelart

import android.content.Context
import android.content.Intent
import android.net.Uri
import android.os.Build
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
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.unit.dp
import androidx.core.content.FileProvider
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext
import java.io.File
import java.io.FileOutputStream

sealed class ConversionResult {
    data class Success(val filePath: String) : ConversionResult()
    data class Error(val message: String) : ConversionResult()
}

class MainActivity : ComponentActivity() {

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        // Handle incoming intent if it was started from "Share"
        val initialUri = if (intent?.action == Intent.ACTION_SEND) {
            if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU) {
                intent.getParcelableExtra(Intent.EXTRA_STREAM, Uri::class.java)
            } else {
                @Suppress("DEPRECATION")
                intent.getParcelableExtra(Intent.EXTRA_STREAM)
            }
        } else null

        setContent {
            tech.bananajuice.convertpixelart.ui.theme.ConvertPixelArtTheme {
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
        Text(stringResource(id = R.string.app_name), style = MaterialTheme.typography.headlineMedium)

        Button(onClick = { filePickerLauncher.launch(arrayOf("*/*")) }) {
            Text(stringResource(id = R.string.select_input_file))
        }

        if (selectedUri != null) {
            Text(stringResource(id = R.string.selected_file_label) + " " + (selectedUri?.lastPathSegment ?: "Unknown"))
        }

        Row(verticalAlignment = Alignment.CenterVertically) {
            Text(stringResource(id = R.string.output_format_label))
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
            Text(stringResource(id = R.string.extract_timelapse_label))
        }

        Button(
            onClick = {
                selectedUri?.let { uri ->
                    isConverting = true
                    conversionStatus = context.getString(R.string.converting_status)
                    outputFilePath = null

                    coroutineScope.launch {
                        val result = performConversion(context, uri, selectedFormat, extractTimelapse)
                        isConverting = false
                        when (result) {
                            is ConversionResult.Success -> {
                                outputFilePath = result.filePath
                                conversionStatus = context.getString(R.string.conversion_success_status)
                            }
                            is ConversionResult.Error -> {
                                conversionStatus = result.message
                            }
                        }
                    }
                }
            },
            enabled = selectedUri != null && !isConverting
        ) {
            Text(stringResource(id = R.string.convert_button))
        }

        if (isConverting) {
            CircularProgressIndicator()
        }

        if (conversionStatus.isNotEmpty()) {
            Text(conversionStatus)
        }

        if (outputFilePath != null) {
            Button(onClick = { shareFile(context, File(outputFilePath!!)) }) {
                Text(stringResource(id = R.string.share_save_button))
            }
        }
    }
}

suspend fun performConversion(context: Context, inputUri: Uri, outputFormat: String, timelapse: Boolean): ConversionResult {
    return withContext(Dispatchers.IO) {
        var inputStream: java.io.InputStream? = null
        var tempInputFile: File? = null
        var tempOutputFile: File? = null
        var unzipDir: File? = null

        try {
            inputStream = context.contentResolver.openInputStream(inputUri)
            if (inputStream == null) return@withContext ConversionResult.Error("Error: Cannot open input file")

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

            tempInputFile = File.createTempFile("temp_input", ext, context.cacheDir)
            FileOutputStream(tempInputFile).use { outputStream ->
                inputStream.copyTo(outputStream)
            }

            tempOutputFile = File.createTempFile("output", outputFormat, context.cacheDir)

            if (ext.equals(".pixaki", ignoreCase = true)) {
                unzipDir = File.createTempFile("temp_input_pixaki_dir", "", context.cacheDir)
                unzipDir.delete() // remove file so we can recreate it as dir
                unzipDir.mkdirs()
                java.util.zip.ZipInputStream(java.io.FileInputStream(tempInputFile)).use { zis ->
                    var zipEntry = zis.nextEntry
                    while (zipEntry != null) {
                        val newFile = File(unzipDir, zipEntry.name)
                        // Vulnerability Fix: Zip Slip
                        if (!newFile.canonicalPath.startsWith(unzipDir.canonicalPath + File.separator)) {
                            throw SecurityException("Entry is outside of the target dir: ${zipEntry.name}")
                        }

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
                val rustResult = RustInterop.convertFile(unzipDir.absolutePath, tempOutputFile.absolutePath, timelapse)
                return@withContext if (rustResult == "Success") ConversionResult.Success(tempOutputFile.absolutePath) else ConversionResult.Error(rustResult)
            }

            val rustResult = RustInterop.convertFile(tempInputFile.absolutePath, tempOutputFile.absolutePath, timelapse)

            if (rustResult == "Success") {
                return@withContext ConversionResult.Success(tempOutputFile.absolutePath)
            } else {
                return@withContext ConversionResult.Error(rustResult)
            }
        } catch (e: Exception) {
            return@withContext ConversionResult.Error("Error: ${e.message}")
        } finally {
            inputStream?.close()
            tempInputFile?.delete()
            unzipDir?.deleteRecursively()
            // We intentionally do NOT delete tempOutputFile here if successful,
            // because it needs to be accessed by the user (shared/saved).
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
