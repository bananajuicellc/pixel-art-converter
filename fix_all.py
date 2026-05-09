import sys
with open("crates/pixel-studio-pro-v2-converter/src/lib.rs", "r") as f:
    content = f.read()

# Fix bounds logic
content = content.replace(
"""    let mut max_x: i32 = doc_width as i32;
    let mut max_y: i32 = doc_height as i32;""",
"""    let mut max_x: i32 = doc_width as i32;
    let mut max_y: i32 = doc_height as i32;

    // Helper to get normalized top-down coordinates for rect
    let get_rect_bounds = |rect: &RectData| {
        let sel_min_x = rect.from.x.min(rect.to.as_ref().map_or(rect.from.x, |t| t.x));
        let sel_max_x = rect.from.x.max(rect.to.as_ref().map_or(rect.from.x, |t| t.x));

        // Y in Meta.Rect is already top-down! We do NOT invert it here.
        let sel_min_y = rect.from.y.min(rect.to.as_ref().map_or(rect.from.y, |t| t.y));
        let sel_max_y = rect.from.y.max(rect.to.as_ref().map_or(rect.from.y, |t| t.y));

        (sel_min_x, sel_min_y, sel_max_x, sel_max_y)
    };"""
)


content = content.replace(
"""                                let w = rect.width.unwrap_or_else(|| {
                                    rect.to.as_ref().map_or(0, |to| to.x - rect.from.x)
                                });
                                let h = rect.height.unwrap_or_else(|| {
                                    rect.to.as_ref().map_or(0, |to| to.y - rect.from.y)
                                });

                                let dst_min_x = rect.from.x;
                                let dst_max_x = rect.from.x + h; // swapped width/height
                                let dst_max_y = doc_height as i32 - 1 - rect.from.y;
                                let dst_min_y = dst_max_y - w; // swapped width/height""",
"""                                let (sel_min_x, sel_min_y, sel_max_x, sel_max_y) = get_rect_bounds(rect);

                                let w = sel_max_x - sel_min_x + 1;
                                let h = sel_max_y - sel_min_y + 1;

                                let dst_min_x = sel_min_x;
                                let dst_max_x = sel_min_x + h - 1; // swapped width/height
                                let dst_min_y = sel_min_y;
                                let dst_max_y = sel_min_y + w - 1; // swapped width/height"""
)

content = content.replace(
"""                                    let sel_min_x = from.x.min(to.x);
                                    let sel_max_x = from.x.max(to.x);
                                    let sel_min_y = from.y.min(to.y);
                                    let sel_max_y = from.y.max(to.y);

                                    let top_down_min_y = doc_height as i32 - 1 - sel_max_y;
                                    let top_down_max_y = doc_height as i32 - 1 - sel_min_y;""",
"""                                    let sel_min_x = from.x.min(to.x);
                                    let sel_max_x = from.x.max(to.x);
                                    let top_down_min_y = from.y.min(to.y);
                                    let top_down_max_y = from.y.max(to.y);"""
)

content = content.replace(
"""                                        let dst_min_x = rect.from.x;
                                        let dst_max_x = rect.from.x + img.width() as i32;
                                        // Y is inverted (bottom-up in .psp files)
                                        let dst_max_y = doc_height as i32 - rect.from.y;
                                        let dst_min_y = dst_max_y - img.height() as i32;""",
"""                                        let dst_min_x = rect.from.x.min(rect.to.as_ref().map_or(rect.from.x, |t| t.x));
                                        let dst_max_x = dst_min_x + img.width() as i32 - 1;
                                        let dst_min_y = rect.from.y.min(rect.to.as_ref().map_or(rect.from.y, |t| t.y));
                                        let dst_max_y = dst_min_y + img.height() as i32 - 1;"""
)

content = content.replace(
"""                    let sel_min_x = from.x.min(to.x);
                    let sel_max_x = from.x.max(to.x);
                    let sel_min_y = from.y.min(to.y);
                    let sel_max_y = from.y.max(to.y);

                    let top_down_min_y = doc_height as i32 - 1 - sel_max_y;
                    let top_down_max_y = doc_height as i32 - 1 - sel_min_y;""",
"""                    let sel_min_x = from.x.min(to.x);
                    let sel_max_x = from.x.max(to.x);
                    let top_down_min_y = from.y.min(to.y);
                    let top_down_max_y = from.y.max(to.y);"""
)

content = content.replace(
"""                        let start_x = rect.from.x - min_x;
                        let start_y =
                            (doc_height as i32 - rect.from.y - rgba_patch.height() as i32) - min_y;""",
"""                        let sel_min_x = rect.from.x.min(rect.to.as_ref().map_or(rect.from.x, |t| t.x));
                        let sel_min_y = rect.from.y.min(rect.to.as_ref().map_or(rect.from.y, |t| t.y));
                        let start_x = sel_min_x - min_x;
                        let start_y = sel_min_y - min_y;"""
)

content = content.replace(
"""                let sel_min_x = rect.from.x - min_x;
                let sel_max_x = rect
                    .to
                    .as_ref()
                    .map_or(sel_min_x, |to| to.x - min_x)
                    .max(sel_min_x + rect.width.unwrap_or(0) - 1);
                let mut sel_min_y = (doc_height as i32 - 1 - rect.from.y) - min_y;
                let mut sel_max_y =
                    (doc_height as i32 - 1 - rect.to.as_ref().map_or(rect.from.y, |to| to.y))
                        - min_y;

                if sel_min_y > sel_max_y {
                    std::mem::swap(&mut sel_min_y, &mut sel_max_y);
                }
                sel_max_y = sel_max_y.max(sel_min_y + rect.height.unwrap_or(0) - 1);

                let sel_w = sel_max_x - sel_min_x + 1;
                let sel_h = sel_max_y - sel_min_y + 1;""",
"""                let mut sel_min_x = rect.from.x.min(rect.to.as_ref().map_or(rect.from.x, |t| t.x));
                let mut sel_max_x = rect.from.x.max(rect.to.as_ref().map_or(rect.from.x, |t| t.x));
                let mut sel_min_y = rect.from.y.min(rect.to.as_ref().map_or(rect.from.y, |t| t.y));
                let mut sel_max_y = rect.from.y.max(rect.to.as_ref().map_or(rect.from.y, |t| t.y));

                sel_max_x = sel_max_x.max(sel_min_x + rect.width.unwrap_or(0) - 1);
                sel_max_y = sel_max_y.max(sel_min_y + rect.height.unwrap_or(0) - 1);

                let sel_w = sel_max_x - sel_min_x + 1;
                let sel_h = sel_max_y - sel_min_y + 1;

                sel_min_x -= min_x;
                sel_min_y -= min_y;"""
)

# RotateRect has px1 as the origin/anchor for rotation in source.
# The offset was originally px3 - rect_min_x. Let's make it px3 - px1 since px1 is the anchor, BUT
# min_rx/min_ry are computed relative to `cx`, `cy` which is `(w-1)/2`, `(h-1)/2`.
# `w` and `h` are based on `rect_max_x - rect_min_x`. So the bounding box is indeed `rect_min_x` to `rect_max_x`.
# So the translation offset of the *bounding box* to the new location should be `px3 - rect_min_x + (px1 - rect_min_x)` ?
# No, if px1, py1 is the source anchor, and px3, py3 is the destination anchor...
# The difference `dx = px3 - px1`, `dy = py3 - py1`.
# So the original source rectangle `rect_min_x, rect_min_y` will be translated by `dx, dy`.
# That means `final_min_x = min_rx.floor() as i32 + rect_min_x + dx;`

content = content.replace(
"""    let offset_x = px3 - rect_min_x;
    let offset_y = py3 - rect_min_y;

    let final_min_x = min_rx.floor() as i32 + rect_min_x + offset_x;
    let final_min_y = min_ry.floor() as i32 + rect_min_y + offset_y;
    let final_max_x = max_rx.ceil() as i32 + rect_min_x + offset_x;
    let final_max_y = max_ry.ceil() as i32 + rect_min_y + offset_y;""",
"""    let offset_x = px3 - px1;
    let offset_y = py3 - py1;

    let final_min_x = min_rx.floor() as i32 + rect_min_x + offset_x;
    let final_min_y = min_ry.floor() as i32 + rect_min_y + offset_y;
    let final_max_x = max_rx.ceil() as i32 + rect_min_x + offset_x;
    let final_max_y = max_ry.ceil() as i32 + rect_min_y + offset_y;"""
)

content = content.replace(
"""                    let final_x = info.px3 + rx + info.min_rx.floor() as i32 - min_x;
                    let final_y = info.py3 + ry + info.min_ry.floor() as i32 - min_y;""",
"""                    let offset_x = info.px3 - info.px1;
                    let offset_y = info.py3 - info.py1;
                    let final_x = info.rect_min_x + offset_x + rx + info.min_rx.floor() as i32 - min_x;
                    let final_y = info.rect_min_y + offset_y + ry + info.min_ry.floor() as i32 - min_y;"""
)

# Unused variable fix
content = content.replace(
"""    img_width: u32,
    img_height: u32,
    doc_height: u32,
    has_data: &mut bool,
) {
    if let Some(meta_str) = &action.meta {""",
"""    img_width: u32,
    img_height: u32,
    _doc_height: u32,
    has_data: &mut bool,
) {
    if let Some(meta_str) = &action.meta {"""
)

# Replace px1 and py1 to RotateRectInfo
content = content.replace(
"""    max_rx: f32,
    max_ry: f32,""",
"""    max_rx: f32,
    max_ry: f32,
    px1: i32,
    py1: i32,"""
)

content = content.replace(
"""        rect_min_y,
        w,
        h,""",
"""        rect_min_y,
        w,
        h,
        px1,
        py1,"""
)


with open("crates/pixel-studio-pro-v2-converter/src/lib.rs", "w") as f:
    f.write(content)
