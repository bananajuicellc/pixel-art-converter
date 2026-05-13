use anyhow::{anyhow, Result};
use image::RgbaImage;
use pixel_art::{BlendMode, Cel, Document, Frame, Image, Layer};
use pixel_studio_pro_v2::{self, History};
use pixel_studio_pro_v2_converter::{
    apply_move_action, apply_paste_import_action, apply_positions_to_image,
    apply_replace_color_action, apply_rotate_rect_action, apply_transform_action, calculate_bounds,
};

pub fn create_timelapse(doc: pixel_studio_pro_v2::Document) -> Result<Document> {
    let mut out_layers: Vec<Layer> = Vec::new();
    let mut out_frames: Vec<Frame> = Vec::new();
    let mut out_cels: Vec<Cel> = Vec::new();
    let mut out_images: Vec<Image> = Vec::new();

    let clip = doc
        .clips
        .first()
        .ok_or_else(|| anyhow!("No clips found in document"))?;
    let first_frame = clip
        .frames
        .first()
        .ok_or_else(|| anyhow!("No frames found in document"))?;

    let doc_width = doc.width as u32;
    let doc_height = doc.height as u32;

    for psp_layer in &first_frame.layers {
        out_layers.push(Layer {
            name: psp_layer.name.clone(),
            opacity: (psp_layer.opacity * 255.0).clamp(0.0, 255.0) as u8,
            visible: !psp_layer.hidden,
            blend_mode: BlendMode::Normal,
        });
    }

    let mut current_frame_index = 0;

    // To keep previously drawn layers visible, we track the latest generated image index
    // and bounds for each layer. For every new frame created during the active layer's timelapse,
    // we also insert linked cels for all previously completed layers.
    let mut latest_cel_per_layer: Vec<Option<(usize, i16, i16)>> = vec![None; first_frame.layers.len()];

    for (psp_layer_idx, psp_layer) in first_frame.layers.iter().enumerate() {
        if let Some(history_str) = &psp_layer.history_json {
            let history = serde_json::from_str::<History>(history_str).map_err(|e| {
                anyhow!(
                    "Failed to parse history JSON for layer {}: {}",
                    psp_layer_idx,
                    e
                )
            })?;

            let (min_x, min_y, max_x, max_y, source_img_opt) =
                calculate_bounds(&history, doc_width, doc_height);

            let img_width = (max_x - min_x).clamp(1, 4096) as u32;
            let img_height = (max_y - min_y).clamp(1, 4096) as u32;

            let mut final_img = RgbaImage::new(img_width, img_height);
            let mut layer_has_drawn_anything = false;

            // Replay actions one by one
            let replay_count = std::cmp::min(history.index as usize, history.actions.len());

            if replay_count == 0 && source_img_opt.is_some() {
                // If a layer has no actions but has a _source image, we treat that as a single step
                let src_img = source_img_opt.unwrap();
                let offset_x = -min_x;
                let offset_y = -min_y;
                for y in 0..src_img.height() {
                    for x in 0..src_img.width() {
                        let p = *src_img.get_pixel(x, y);
                        let dst_x = offset_x + x as i32;
                        let dst_y = offset_y + y as i32;
                        if dst_x >= 0
                            && dst_y >= 0
                            && (dst_x as u32) < img_width
                            && (dst_y as u32) < img_height
                        {
                            final_img.put_pixel(dst_x as u32, dst_y as u32, p);
                            layer_has_drawn_anything = true;
                        }
                    }
                }

                if layer_has_drawn_anything {
                    out_frames.push(Frame { duration_ms: 100 });

                    let image_index = out_images.len();
                    out_images.push(Image {
                        width: u16::try_from(img_width).unwrap_or(u16::MAX),
                        height: u16::try_from(img_height).unwrap_or(u16::MAX),
                        rgba: final_img.clone().into_raw(),
                    });

                    let cx = (psp_layer.sx + min_x).clamp(i16::MIN as i32, i16::MAX as i32) as i16;
                    let cy = (psp_layer.sy + min_y).clamp(i16::MIN as i32, i16::MAX as i32) as i16;

                    latest_cel_per_layer[psp_layer_idx] = Some((image_index, cx, cy));

                    for (prev_layer_idx, latest_cel) in latest_cel_per_layer.iter().enumerate() {
                        if let Some((prev_image_index, px, py)) = latest_cel {
                            out_cels.push(Cel {
                                frame_index: current_frame_index,
                                layer_index: prev_layer_idx,
                                x: *px,
                                y: *py,
                                image_index: *prev_image_index,
                            });
                        }
                    }

                    current_frame_index += 1;
                }
            } else {
                for action in history.actions.iter().take(replay_count) {
                    if let Ok(tool_type) = pixel_studio_pro_v2::Tool::try_from(action.tool) {
                        let mut action_has_data = false;
                        match tool_type {
                            pixel_studio_pro_v2::Tool::Move => {
                                apply_move_action(
                                    action,
                                    &mut final_img,
                                    min_x,
                                    min_y,
                                    img_width,
                                    img_height,
                                    doc_height,
                                    &mut action_has_data,
                                );
                            }
                            pixel_studio_pro_v2::Tool::RotateRect => {
                                apply_rotate_rect_action(
                                    action,
                                    &mut final_img,
                                    min_x,
                                    min_y,
                                    img_width,
                                    img_height,
                                    doc_height,
                                    &mut action_has_data,
                                );
                            }
                            pixel_studio_pro_v2::Tool::PasteImage => {
                                apply_paste_import_action(
                                    tool_type,
                                    action,
                                    &mut final_img,
                                    min_x,
                                    min_y,
                                    img_width,
                                    img_height,
                                    doc_height,
                                    &mut action_has_data,
                                );
                            }
                            pixel_studio_pro_v2::Tool::Pen
                            | pixel_studio_pro_v2::Tool::DotPen
                            | pixel_studio_pro_v2::Tool::DitheringPen
                            | pixel_studio_pro_v2::Tool::Brush
                            | pixel_studio_pro_v2::Tool::OutlineTool
                            | pixel_studio_pro_v2::Tool::Fill
                            | pixel_studio_pro_v2::Tool::Eraser
                            | pixel_studio_pro_v2::Tool::Clear
                            | pixel_studio_pro_v2::Tool::EraserPen
                            | pixel_studio_pro_v2::Tool::Cut => {
                                apply_positions_to_image(
                                    tool_type,
                                    action,
                                    &mut final_img,
                                    min_x,
                                    min_y,
                                    img_width,
                                    img_height,
                                    doc_height,
                                    &mut action_has_data,
                                );
                            }
                            pixel_studio_pro_v2::Tool::MirrorByX
                            | pixel_studio_pro_v2::Tool::MirrorByY
                            | pixel_studio_pro_v2::Tool::FlipByX
                            | pixel_studio_pro_v2::Tool::FlipByY
                            | pixel_studio_pro_v2::Tool::RotateLeft
                            | pixel_studio_pro_v2::Tool::RotateRight => {
                                apply_transform_action(
                                    tool_type,
                                    action,
                                    &mut final_img,
                                    min_x,
                                    min_y,
                                    img_width,
                                    img_height,
                                    doc_height,
                                    &mut action_has_data,
                                );
                            }
                            pixel_studio_pro_v2::Tool::ReplaceColor => {
                                apply_replace_color_action(
                                    action,
                                    &mut final_img,
                                    min_x,
                                    min_y,
                                    img_width,
                                    img_height,
                                    doc_height,
                                );
                                // ReplaceColor always considers something might have changed
                                action_has_data = true;
                            }
                            _ => {}
                        }

                        if action_has_data {
                            out_frames.push(Frame { duration_ms: 100 });

                            let image_index = out_images.len();
                            out_images.push(Image {
                                width: u16::try_from(img_width).unwrap_or(u16::MAX),
                                height: u16::try_from(img_height).unwrap_or(u16::MAX),
                                rgba: final_img.clone().into_raw(),
                            });

                            let cx = (psp_layer.sx + min_x).clamp(i16::MIN as i32, i16::MAX as i32) as i16;
                            let cy = (psp_layer.sy + min_y).clamp(i16::MIN as i32, i16::MAX as i32) as i16;

                            latest_cel_per_layer[psp_layer_idx] = Some((image_index, cx, cy));

                            for (prev_layer_idx, latest_cel) in latest_cel_per_layer.iter().enumerate() {
                                if let Some((prev_image_index, px, py)) = latest_cel {
                                    out_cels.push(Cel {
                                        frame_index: current_frame_index,
                                        layer_index: prev_layer_idx,
                                        x: *px,
                                        y: *py,
                                        image_index: *prev_image_index,
                                    });
                                }
                            }

                            current_frame_index += 1;
                        }
                    }
                }
            }
        }
    }

    Ok(Document {
        width: u16::try_from(doc.width).unwrap_or(u16::MAX),
        height: u16::try_from(doc.height).unwrap_or(u16::MAX),
        layers: out_layers,
        frames: out_frames,
        cels: out_cels,
        images: out_images,
    })
}
