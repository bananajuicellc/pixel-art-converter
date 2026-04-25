# Pixaki v2 File Format Specification

Pixaki v2 files are stored as macOS/iOS "Packages" (folders with a `.pixaki` extension).

## Package Structure

The package contains a central metadata file and individual PNG files for every layer of every frame.

- `DocumentInfo.plist`: The main project file (XML Property List).
- `Layer[GUID].png`: Image data for individual layers.
- `Preview.png`: A flattened preview of the document.

## High-Level Structure

The `DocumentInfo.plist` follows this hierarchy:

1.  **[Document](document.md)**: Global settings like canvas size and animation speed.
2.  **[Symbol](symbol.md)**: A collection of frames (often used for different animation loops).
3.  **[Frame](frame.md)**: A single point in time within a symbol.
4.  **[Layer](layer.md)**: A reference to a PNG file representing a drawing surface.

## Key Concepts

### External Image Storage
Unlike some other formats, Pixaki v2 stores every layer as a separate PNG file on disk. The JSON only references these files by name.

### Versioning
This specification covers Version 2 of the format, as indicated by the `Version` key in the plist.
