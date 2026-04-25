# Clip Object

A Clip represents a sequence of frames, such as an animation or a layered illustration.

| Property | Type | Description |
| :--- | :--- | :--- |
| `Id` | String | A unique GUID/identifier for the clip. |
| `Name` | String | The name of the clip (e.g., "Clip 1"). |
| `Frames` | Array | An array of [Frame](frame.md) objects. |
| `LayerTypes` | Array | An array of integers indicating the type of each layer index. |
| `ActiveFrameIndex` | Integer | The index of the currently active frame in the clip. |
