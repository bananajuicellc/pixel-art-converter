# Document Object (v2)

The root object in `DocumentInfo.plist`.

| Key | Type | Description |
| :--- | :--- | :--- |
| `Version` | Number | Format version (e.g., `2.0`). |
| `Size` | Dictionary | `{ Width, Height }` (floats). |
| `AnimationSpeed` | Number | Global playback speed (e.g., `15`). |
| `Symbols` | Array | A collection of [Symbol](symbol.md) objects. |
| `SelectedSymbolIndex` | Integer | Currently active symbol. |
| `ColorPresets` | Dictionary | Palette information. |
| `GridSettings` | Dictionary | Canvas grid configuration. |
