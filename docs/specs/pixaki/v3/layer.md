# Layer Object (v3)

| Property | Type | Description |
| :--- | :--- | :--- |
| `name` | String | Display name. |
| `isVisible` | Boolean | Visibility status. |
| `opacity` | Number | Opacity (0.0 to 1.0). |
| `blendMode` | String | CSS-like blend mode (e.g., "normal"). |
| `clips` | Array | A collection of [Clip](#clip-object) objects. |

## Clip Object

A Clip maps a [Cel](cel.md) to a specific frame range on the timeline.

| Property | Type | Description |
| :--- | :--- | :--- |
| `itemIdentifier` | String | The identifier of the [Cel](cel.md) to display. |
| `range` | Object | `{ start, end }` frame indices (inclusive). |
