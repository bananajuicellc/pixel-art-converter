# Layer Group Object

Layer Groups are used to organize layers into a hierarchy.

| Property | Type | Description |
| :--- | :--- | :--- |
| `Id` | String | A unique GUID/identifier for the group. |
| `Name` | String | The display name of the group. |
| `Index` | Integer | The display index/order of the group. |
| `Hidden` | Boolean | Whether the group (and its members) is hidden. |
| `Collapsed` | Boolean | Whether the group is collapsed in the UI. |
| `Layers` | Array | An array of layer `Id` strings belonging to this group. |
