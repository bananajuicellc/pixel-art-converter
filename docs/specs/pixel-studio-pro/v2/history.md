# History & Actions

The `_historyJson` field in a [Layer](layer.md) contains the actual drawing data and a sequence of tool actions.

## History Structure

| Property | Type | Description |
| :--- | :--- | :--- |
| `Actions` | Array | A sequence of [Action](#action-object) objects. |
| `Index` | Integer | The current position in the action history. |
| `_source` | String | (Optional) A Base64 encoded PNG of the flattened layer content. |

## Action Object

An individual drawing command.

| Property | Type | Description |
| :--- | :--- | :--- |
| `Tool` | Integer | The tool ID used (see [Tools](tools.md) for a full list). |
| `ColorIndexes` | Array | (Optional) Indexes of colors used. |
| `Positions` | String | Base64 encoded coordinate data. |
| `Colors` | String | Base64 encoded color data (usually RGBA). |
| `Meta` | String | (Optional) A JSON string containing tool-specific metadata (e.g., `Rect`, `Pixels`, `PasteAction`). |
| `Invalid` | Boolean | Whether the action is considered invalid. |

### Tool Details

Different tools utilize the properties of the Action object in specific ways. For a complete list of tools and their logic, see the **[Tools Specification](tools.md)**.

* **Pen (0) / Eraser (2)**:
  * `Positions`: Contains an array of `(X, Y)` coordinate pairs.
  * `Colors`: Contains the color(s) used.
* **Fill (3)**:
  * `Positions`: Contains a single `(X, Y)` coordinate representing the starting point of the fill.
  * `Colors`: Contains a single 4-byte RGBA value representing the fill color.
  * `Meta`: Contains the tolerance value as a string (stored in `action.Float`).
* **Move (10)**:
  * `Positions`: Contains two `(X, Y)` coordinate pairs representing the movement vector (start and end), and optionally more pairs for specific pixels.
  * `Meta`: Contains a JSON string with a rectangle defining the selection bounds.
* **PasteImage (20)**:
  * `Meta`: Contains a JSON string detailing the `PasteAction`.

### Metadata Structures

The `Meta` field contains JSON-serialized objects. Common structures include:

#### ImageRect
Defines a rectangular area.
```json
{
  "From": { "X": 0, "Y": 0 },
  "To": { "X": 10, "Y": 10 }
}
```
*   `X`, `Y`: 16-bit integers.
*   Note: `Width` and `Height` are calculated as `To.X - From.X` and `To.Y - From.Y`.

#### PasteAction
Used by `PasteImage` (20) and `Paste` (9).
```json
{
  "Rect": { "From": {...}, "To": {...} },
  "Buffer": [ ... ],
  "HasMask": false,
  "Mask": [ ... ],
  "HasRectSource": false,
  "RectSource": { "From": {...}, "To": {...} }
}
```
*   `Rect`: Destination rectangle.
*   `Buffer`: Array of RGBA colors (Color32).
*   `Mask`: (Optional) Array of `Position` objects.
*   `RectSource`: (Optional) Source rectangle.

### Encoding
`Positions` and `Colors` use a custom Base64 binary encoding to store coordinate and color arrays efficiently. `Positions` encodes a sequence of 16-bit little-endian signed integers forming `(X, Y)` coordinate pairs, while `Colors` encodes a sequence of 4-byte RGBA values.
