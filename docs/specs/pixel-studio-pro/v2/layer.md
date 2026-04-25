# Layer Object

A Layer is an individual drawing surface within a frame.

| Property | Type | Description |
| :--- | :--- | :--- |
| `Id` | String | A unique GUID/identifier for the layer. |
| `Name` | String | The display name of the layer. |
| `Opacity` | Float | Layer opacity (0.0 to 1.0). |
| `Transparency` | Float | Transparency setting (often `-1.0` for default). |
| `Hidden` | Boolean | Whether the layer is hidden. |
| `Linked` | Boolean | Whether the layer content is linked to the previous frame. |
| `Outline` | Integer | Outline setting. |
| `Lock` | Integer | Lock status. |
| `Sx`, `Sy` | Integer | Offset/Translation of the layer. |
| `Scripts` | Array | Layer-specific scripts. |
| `Version` | Integer | Layer data version. |
| `_historyJson` | String | (Optional) A JSON string containing drawing [Actions](history.md). |

## Linked Layers
When `Linked` is `true`, the layer typically does not contain `_historyJson`. Instead, it inherits the content from the same layer ID in a previous frame. This allows for efficient storage of static elements in an animation.
