# Document Object

The root object of a `.psp` file.

| Property | Type | Description |
| :--- | :--- | :--- |
| `Version` | Integer | The version of the file format (e.g., `2`). |
| `Id` | String | A unique GUID/identifier for the document. |
| `Name` | String | The display name of the project. |
| `Source` | String | (Optional) The original file path on the device. |
| `Width` | Integer | The width of the canvas in pixels. |
| `Height` | Integer | The height of the canvas in pixels. |
| `Type` | Integer | Document type indicator. `0` for illustrations, `1` for animations. |
| `Clips` | Array | An array of [Clip](clip.md) objects. |
| `Background` | Boolean | Whether a background layer is enabled. |
| `BackgroundColor` | Object | An RGBA color object `{r, g, b, a}` (floats 0.0-1.0). |
| `TileMode` | Boolean | Whether tiling mode is enabled. |
| `TileFade` | Integer | Tiling fade intensity (0-100). |
| `ActiveClipIndex` | Integer | The index of the currently active clip. |
