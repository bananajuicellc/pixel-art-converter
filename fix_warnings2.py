import sys
with open("crates/pixel-studio-pro-v2-converter/src/lib.rs", "r") as f:
    content = f.read()

content = content.replace("let get_rect_bounds", "let _get_rect_bounds")
content = content.replace("    px1: i32,\n    py1: i32,", "    _px1: i32,\n    _py1: i32,")
content = content.replace("        px1,\n        py1,", "        _px1: px1,\n        _py1: py1,")

with open("crates/pixel-studio-pro-v2-converter/src/lib.rs", "w") as f:
    f.write(content)
