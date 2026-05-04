# Pixel Studio Pro v2 File Format Specification

The `.psp` file is a JSON-based project file format used by Pixel Studio Pro. It supports multi-layer, multi-frame animations and static illustrations.

## High-Level Structure

The document follows a hierarchical structure:

1.  **[Document](document.md)**: The root object containing global settings (width, height, version).
2.  **[Clip](clip.md)**: A collection of frames, typically representing an animation sequence.
3.  **[Frame](frame.md)**: A single point in time within a clip, containing layers and layer groups.
4.  **[Layer](layer.md)**: A single drawing surface within a frame.
5.  **[Layer Group](layer-group.md)**: A logical grouping of layers for organization.
6.  **[History & Actions](history.md)**: Encapsulated drawing commands and image data stored within layers.
7.  **[Tools](tools.md)**: Detailed behavior of individual drawing tools.

## Key Concepts

### Versioning
The current specification covers Version 2 of the format.

### Data Storage
Pixel data is often stored as Base64 encoded PNG strings within the `_historyJson` or `_source` fields of a layer, rather than as separate image files.

### Coordinate Systems
Coordinates for drawing actions are stored in encoded strings within the history actions.

### Linked Layers
Layers can be "linked" across frames, allowing content to be shared between frames without duplication in the JSON file.
