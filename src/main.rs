use aseprite::{AsepriteFile, ColorMode, Pixels, BlendMode, LayerOptions};
use clap::Parser;
use pixaki_converter::pixaki::{Document, DocumentV2};
use std::collections::HashMap;
use std::fs;
use std::io::{Result, Error, ErrorKind};
use std::path::Path;

/// A simple program to convert Pixaki files to Aseprite files.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// The path to the .pixaki directory
    pixaki_path: std::path::PathBuf,

    /// The path to the output .aseprite file
    aseprite_path: std::path::PathBuf,
}

fn map_blend_mode(s: &str) -> BlendMode {
    match s {
        "normal" => BlendMode::Normal,
        "multiply" => BlendMode::Multiply,
        "screen" => BlendMode::Screen,
        "overlay" => BlendMode::Overlay,
        "darken" => BlendMode::Darken,
        "lighten" => BlendMode::Lighten,
        "colorDodge" => BlendMode::ColorDodge,
        "colorBurn" => BlendMode::ColorBurn,
        "hardLight" => BlendMode::HardLight,
        "softLight" => BlendMode::SoftLight,
        "difference" => BlendMode::Difference,
        "exclusion" => BlendMode::Exclusion,
        "hue" => BlendMode::Hue,
        "saturation" => BlendMode::Saturation,
        "color" => BlendMode::Color,
        "luminosity" => BlendMode::Luminosity,
        _ => BlendMode::Normal,
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.pixaki_path.join("document.json").exists() {
        handle_modern_format(&cli.pixaki_path, &cli.aseprite_path)
    } else if cli.pixaki_path.join("DocumentInfo.plist").exists() {
        handle_legacy_format(&cli.pixaki_path, &cli.aseprite_path)
    } else {
        Err(Error::new(ErrorKind::NotFound, "No document.json or DocumentInfo.plist found in the pixaki directory"))
    }
}

fn handle_modern_format(pixaki_path: &Path, aseprite_path: &Path) -> Result<()> {
    // 1. Read and parse document.json
    let document_path = pixaki_path.join("document.json");
    let json_str = fs::read_to_string(document_path)?;
    let document: Document =
        serde_json::from_str(&json_str).expect("Unable to parse document.json");

    // 2. Extract data
    let sprite = document.sprites.get(0).expect("No sprite found");
    let width = sprite.size.width as u16;
    let height = sprite.size.height as u16;
    let layers = &sprite.layers;
    let num_frames = sprite.duration;

    // 3. Create Aseprite file
    let mut aseprite = AsepriteFile::new(width, height, ColorMode::Rgba);

    // 4. Add Layers
    let mut layer_handles = Vec::new();
    for layer in layers {
        let opts = LayerOptions {
            opacity: (layer.opacity * 255.0) as u8,
            blend_mode: map_blend_mode(layer.blend_mode.as_deref().unwrap_or("normal")),
            visible: layer.is_visible,
            ..Default::default()
        };
        let handle = aseprite.add_layer_with(&layer.name, opts);
        layer_handles.push(handle);
    }

    // 5. Add Frames
    let mut frame_handles = Vec::new();
    for _ in 0..num_frames {
        let handle = aseprite.add_frame(100);
        frame_handles.push(handle);
    }

    // Create a map of cels for easy lookup
    let cel_map: HashMap<_, _> = sprite
        .cels
        .iter()
        .map(|c| (c.identifier.clone(), c))
        .collect();

    // Determine the image directory
    let image_dir = if pixaki_path.join("images").join("drawings").is_dir() {
        pixaki_path.join("images").join("drawings")
    } else {
        pixaki_path.to_path_buf()
    };

    // 6. Loop through frames and layers to set cels
    for frame_index in 0..num_frames {
        let frame_handle = frame_handles[frame_index as usize];
        for (layer_index, layer) in layers.iter().enumerate() {
            let layer_handle = layer_handles[layer_index];
            for clip in &layer.clips {
                let in_range = match &clip.range {
                    Some(range) => frame_index >= range.start && frame_index < range.end,
                    None => frame_index == 0, // Assume frame 0 if range is null
                };

                if in_range {
                    if let Some(cel_info) = cel_map.get(&clip.item_identifier) {
                        let image_path = image_dir.join(format!("{}.png", cel_info.identifier));

                        if let Ok(img) = image::open(&image_path) {
                            let rgba_img = img.to_rgba8();
                            let (img_width, img_height) = rgba_img.dimensions();
                            let x = cel_info.frame[0][0] as i16;
                            let y = cel_info.frame[0][1] as i16;

                            let pixels = Pixels::new(rgba_img.into_raw(), img_width as u16, img_height as u16, ColorMode::Rgba)
                                .map_err(|e| Error::new(ErrorKind::Other, format!("Failed to create Pixels: {}", e)))?;
                            aseprite.set_cel(layer_handle, frame_handle, pixels, x, y)
                                .map_err(|e| Error::new(ErrorKind::Other, format!("Failed to set cel: {}", e)))?;
                        } else {
                            eprintln!("Failed to load image: {:?}", image_path);
                        }
                    }
                }
            }
        }
    }

    // 7. Write the file
    let mut buffer = Vec::new();
    aseprite.write_to(&mut buffer)
        .map_err(|e| Error::new(ErrorKind::Other, format!("Failed to write Aseprite file: {}", e)))?;
    fs::write(aseprite_path, buffer)?;

    println!(
        "Successfully wrote Aseprite file to {:?}",
        aseprite_path
    );

    Ok(())
}

fn handle_legacy_format(pixaki_path: &Path, aseprite_path: &Path) -> Result<()> {
    // 1. Read and parse DocumentInfo.plist
    let plist_path = pixaki_path.join("DocumentInfo.plist");
    let document: DocumentV2 = plist::from_file(plist_path)
        .map_err(|e| Error::new(ErrorKind::Other, format!("Failed to parse DocumentInfo.plist: {}", e)))?;

    // 2. Extract data
    let width = document.size.width as u16;
    let height = document.size.height as u16;
    let symbols = &document.symbols;
    
    // 3. Create Aseprite file
    let mut aseprite = AsepriteFile::new(width, height, ColorMode::Rgba);

    // 4. Create layers and frames
    if let Some(symbol) = symbols.get(0) {
        // Add layers based on the first frame
        let mut layer_handles = Vec::new();
        if let Some(first_frame) = symbol.frames.get(0) {
            for (i, layer) in first_frame.layers.iter().enumerate() {
                let opts = LayerOptions {
                    opacity: (layer.alpha * 255.0) as u8,
                    visible: layer.visible,
                    ..Default::default()
                };
                let handle = aseprite.add_layer_with(&format!("Layer {}", i), opts);
                layer_handles.push(handle);
            }
        }

        // Add frames
        let mut frame_handles = Vec::new();
        for _ in &symbol.frames {
            let handle = aseprite.add_frame(100);
            frame_handles.push(handle);
        }

        // Set cels
        for (frame_index, frame_v2) in symbol.frames.iter().enumerate() {
            let frame_handle = frame_handles[frame_index];
            for (layer_index, layer_v2) in frame_v2.layers.iter().enumerate() {
                if layer_index < layer_handles.len() {
                    let layer_handle = layer_handles[layer_index];
                    let image_path = pixaki_path.join(&layer_v2.image_filename);
                    
                    if let Ok(img) = image::open(&image_path) {
                        let rgba_img = img.to_rgba8();
                        let (img_width, img_height) = rgba_img.dimensions();
                        
                        let pixels = Pixels::new(rgba_img.into_raw(), img_width as u16, img_height as u16, ColorMode::Rgba)
                            .map_err(|e| Error::new(ErrorKind::Other, format!("Failed to create Pixels: {}", e)))?;
                        aseprite.set_cel(layer_handle, frame_handle, pixels, 0, 0)
                            .map_err(|e| Error::new(ErrorKind::Other, format!("Failed to set cel: {}", e)))?;
                    }
                }
            }
        }
    }

    // 5. Write the file
    let mut buffer = Vec::new();
    aseprite.write_to(&mut buffer)
        .map_err(|e| Error::new(ErrorKind::Other, format!("Failed to write Aseprite file: {}", e)))?;
    fs::write(aseprite_path, buffer)?;

    println!(
        "Successfully wrote Aseprite file to {:?}",
        aseprite_path
    );

    Ok(())
}
