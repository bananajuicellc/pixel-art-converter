# Pixaki v3 File Format Specification

Pixaki v3 uses a modernized package structure with JSON metadata and a nested image directory.

## Package Structure

- `document.json`: The primary project definition (JSON).
- `metadata.json`: High-level file metadata.
- `images/`: Directory containing pixel data.
    - `preview.png`: Flattened document preview.
    - `drawings/`: Individual PNG files for each unique [Cel](cel.md).

## High-Level Structure

The `document.json` follows this hierarchy:

1.  **[Document](document.md)**: Global project settings.
2.  **[Sprite](sprite.md)**: A collection of layers and cels (the main canvas).
3.  **[Layer](layer.md)**: Defines drawing surfaces and how they reference [Cels](cel.md).
4.  **[Cel](cel.md)**: A pointer to a PNG file in `images/drawings/` with positioning data.
5.  **[Clip](clip.md)**: Defines the visibility of a layer across specific frame ranges.

## Key Concepts

### Cel-Based Architecture
Unlike v2, which duplicated images for every frame/layer intersection, v3 uses "Cels". A Cel is a unique drawing that can be reused across different frames via [Clips](clip.md).

### Frame Ranges
Layers use `Clips` to define which `Cel` is visible at which frame index.

### Coordinate System
Cels include a `frame` property (e.g., `[[x, y], [width, height]]`) that allows layers to be smaller than the canvas and positioned arbitrarily.
