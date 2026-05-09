import sys

with open("crates/pixel-studio-pro-v2-converter/src/lib.rs", "r") as f:
    content = f.read()

content = content.replace(
"""                    let offset_x = info.px3 - info.rect_min_x;
                    let offset_y = info.py3 - info.rect_min_y;
                    let final_x = info.rect_min_x + offset_x + rx + info.min_rx.floor() as i32 - min_x;
                    let final_y = info.rect_min_y + offset_y + ry + info.min_ry.floor() as i32 - min_y;""",
"""                    let final_x = info.px3 + rx + info.min_rx.floor() as i32 - min_x;
                    let final_y = info.py3 + ry + info.min_ry.floor() as i32 - min_y;"""
)

# And get_rotate_rect_info should have offset: px3 - px1, py3 - py1
# Wait, the offset is used to compute bounds:
# let offset_x = px3 - px1;
# let offset_y = py3 - py1;
# let final_min_x = min_rx.floor() as i32 + rect_min_x + offset_x;
# But if it's px3 - px1, and final_min_x is based on rect_min_x...

# Wait, the original code had:
# let offset_x = px3 - rect_min_x;
# let offset_y = py3 - rect_min_y;

# If offset_x is based on rect_min_x, then final_x = info.px3 + rx ...
# Actually, the original get_rotate_rect_info was:
# let offset_x = px3 - rect_min_x;
# let offset_y = py3 - rect_min_y;
# Let's revert that part too

content = content.replace(
"""    let offset_x = px3 - px1;
    let offset_y = py3 - py1;

    let final_min_x = min_rx.floor() as i32 + rect_min_x + offset_x;
    let final_min_y = min_ry.floor() as i32 + rect_min_y + offset_y;
    let final_max_x = max_rx.ceil() as i32 + rect_min_x + offset_x;
    let final_max_y = max_ry.ceil() as i32 + rect_min_y + offset_y;""",
"""    let offset_x = px3 - px1;
    let offset_y = py3 - py1;

    let final_min_x = min_rx.floor() as i32 + px1 + offset_x;
    let final_min_y = min_ry.floor() as i32 + py1 + offset_y;
    let final_max_x = max_rx.ceil() as i32 + px1 + offset_x;
    let final_max_y = max_ry.ceil() as i32 + py1 + offset_y;"""
)

# And then in apply_rotate_rect_action:
content = content.replace(
"""                    let final_x = info.px3 + rx + info.min_rx.floor() as i32 - min_x;
                    let final_y = info.py3 + ry + info.min_ry.floor() as i32 - min_y;""",
"""                    let final_x = info.px3 + rx + info.min_rx.floor() as i32 - min_x;
                    let final_y = info.py3 + ry + info.min_ry.floor() as i32 - min_y;"""
)

with open("crates/pixel-studio-pro-v2-converter/src/lib.rs", "w") as f:
    f.write(content)
