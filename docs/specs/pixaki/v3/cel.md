# Cel Object (v3)

A Cel represents a unique drawing (image file).

| Property | Type | Description |
| :--- | :--- | :--- |
| `identifier` | String | Unique ID matching a file in `images/drawings/`. |
| `frame` | Array | `[[x, y], [w, h]]` defining the location and size on canvas. |
| `type` | String | Usually "drawing". |
| `opacity` | Number | Instance-specific opacity. |
| `isVisible` | Boolean | Instance-specific visibility. |
| `requiresTrim` | Boolean | Metadata for Pixaki's renderer. |
