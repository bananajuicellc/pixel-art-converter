# Frame Object

A Frame represents a single snapshot in time.

| Property | Type | Description |
| :--- | :--- | :--- |
| `Id` | String | A unique GUID/identifier for the frame. |
| `Delay` | Float | The duration to display this frame in seconds (e.g., `0.3`). |
| `Layers` | Array | An array of [Layer](layer.md) objects. |
| `LayerGroups` | Array | An array of [Layer Group](layer-group.md) objects. |
| `ActiveLayerIndex` | Integer | The index of the currently active layer in this frame. |
| `_reference` | String | (Optional) A Base64 encoded JSON string containing a reference/preview image. |

## _reference JSON Structure
If present, the `_reference` field is a JSON string with:
- `Id`: A unique identifier.
- `Binary`: Base64 encoded JPEG/PNG data for the frame preview.
