import pathlib

current_dir = pathlib.Path(__file__).parent
art_dir = current_dir.parent

for filepath in current_dir.glob("**/*.aseprite"):
    outpath = art_dir / filepath.relative_to(current_dir)
    outpath.parent.mkdir(parents=True, exist_ok=True)
    filepath.rename(outpath)
    print(outpath)