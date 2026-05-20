package com.example.convertpixelart

object RustInterop {
    init {
        System.loadLibrary("pixelartconvert_android")
    }

    external fun convertFile(inputPath: String, outputPath: String, timelapse: Boolean): String
}
