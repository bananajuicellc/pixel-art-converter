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
| `Tool` | Integer | The tool ID used (e.g., `0`: Pen, `1`: Eraser, `2`: Selection, `10`: Move, `20`: Rectangle/Import). |
| `ColorIndexes` | Array | (Optional) Indexes of colors used. |
| `Positions` | String | Base64 encoded coordinate data. |
| `Colors` | String | Base64 encoded color data (usually RGBA). |
| `Meta` | String | (Optional) A JSON string containing tool-specific metadata (e.g., `Rect`, `Pixels`). |
| `Invalid` | Boolean | Whether the action is considered invalid. |

### Tool-Specific Metadata (Meta)
The `Meta` field contains a JSON string that varies depending on the `Tool` ID:

- **Tool 10 (Move):** Contains `{ "From": { "X", "Y" }, "To": { "X", "Y" } }`.
- **Tool 20 (Rectangle/Import):** Contains:
  - `Rect`: The destination rectangle `{From: {X, Y}, To: {X, Y}}`.
  - `Pixels`: Base64 encoded PNG data for the specific region.
  - `RectSource`: The source rectangle in the original image.

### Encoding
`Positions` and `Colors` use a custom Base64 binary encoding to store coordinate and color arrays efficiently:

- `Positions`: Decodes into a sequence of 16-bit little-endian unsigned integers. Each pair of integers represents an `(X, Y)` coordinate.
- `Colors`: Decodes into a sequence of 4-byte RGBA color values (1 byte each for Red, Green, Blue, and Alpha).
