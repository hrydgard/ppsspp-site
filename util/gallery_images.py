#!/usr/bin/env python3
"""Generates the WebP images for the screenshot gallery (pages/media.hbs).

For every entry in data/gallery.json, reads <src>/<file> and writes
  static/img/gallery/full/<name>.webp   (as is, normally 1920x1080/1088)
  static/img/gallery/thumb/<name>.webp  (480x272, cropped to fill)
where <name> is <file> without its extension. Existing outputs newer than the source are skipped.

The sources (~1080p PNG/JPG, already cropped) aren't in the repo. They're prepared by
process.py in the source folder, which renames the submissions to ID_number_author and crops
off black bars and phone overlays.

Usage: python3 util/gallery_images.py [src_dir]   (default ~/ppsspp-screens/1080)
Needs cwebp (apt install webp) and ImageMagick.
"""
import json, os, subprocess, sys, tempfile
from concurrent.futures import ThreadPoolExecutor

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SRC = sys.argv[1] if len(sys.argv) > 1 else os.path.expanduser("~/ppsspp-screens/1080")
OUT = os.path.join(ROOT, "static/img/gallery")
FULL_Q, THUMB_Q = 82, 80
THUMB_W, THUMB_H = 480, 272  # PSP native resolution.


def newer(dst, src):
    return os.path.exists(dst) and os.path.getmtime(dst) >= os.path.getmtime(src)


def process(file):
    src = os.path.join(SRC, file)
    name = os.path.splitext(file)[0]
    full = os.path.join(OUT, "full", name + ".webp")
    thumb = os.path.join(OUT, "thumb", name + ".webp")
    done = []
    with tempfile.TemporaryDirectory() as tmp:
        if not newer(full, src):
            # cwebp can't read JPEG on all builds, so go through PNG.
            png = os.path.join(tmp, "full.png")
            subprocess.run(["convert", src, "-strip", png], check=True)
            subprocess.run(["cwebp", "-quiet", "-q", str(FULL_Q), "-m", "6", "-sharp_yuv",
                            png, "-o", full], check=True)
            done.append("full")
        if not newer(thumb, src):
            png = os.path.join(tmp, "thumb.png")
            subprocess.run(["convert", src, "-strip", "-filter", "Lanczos",
                            "-resize", f"{THUMB_W}x{THUMB_H}^", "-gravity", "center",
                            "-extent", f"{THUMB_W}x{THUMB_H}", png], check=True)
            subprocess.run(["cwebp", "-quiet", "-q", str(THUMB_Q), "-m", "6", "-sharp_yuv",
                            png, "-o", thumb], check=True)
            done.append("thumb")
    return name, done


def main():
    entries = json.load(open(os.path.join(ROOT, "data/gallery.json"), encoding="utf-8"))
    files = [e["file"] for e in entries]
    missing = [f for f in files if not os.path.exists(os.path.join(SRC, f))]
    if missing:
        sys.exit("Missing sources in %s:\n  %s" % (SRC, "\n  ".join(missing)))
    for d in ("full", "thumb"):
        os.makedirs(os.path.join(OUT, d), exist_ok=True)
    with ThreadPoolExecutor(os.cpu_count()) as pool:
        for name, done in pool.map(process, files):
            if done:
                print(f"{name}: {', '.join(done)}")
    # Images no longer in gallery.json. Not deleted automatically, in case that was a mistake.
    names = {os.path.splitext(f)[0] + ".webp" for f in files}
    for d in ("full", "thumb"):
        for f in sorted(os.listdir(os.path.join(OUT, d))):
            if f not in names:
                print(f"Not in gallery.json, delete if unwanted: static/img/gallery/{d}/{f}")


if __name__ == "__main__":
    main()
