#!/usr/bin/env python3
"""Write tooling/fixtures/jpeg-inputs.json: JPEG files from an independent encoder (libjpeg through Pillow).

The files cover baseline and progressive coding, 4:4:4, 4:2:2 and 4:2:0 chroma, restart intervals, odd
sizes that leave partial MCUs, grayscale, Adobe CMYK, optimized Huffman tables and quality extremes. Two
files are derived by editing headers: removing the JFIF marker and naming the components R, G and B makes
an RGB-coded file, and setting the Adobe transform of a CMYK file to 2 makes a YCCK file. The reference
decoder in tooling/jpeg_vectors.go gives the expected samples.

    python3 tooling/jpeg_inputs.py > tooling/fixtures/jpeg-inputs.json
"""
from __future__ import annotations

import io
import json
import random
import sys

from PIL import Image


def picture(mode: str, w: int, h: int, seed: int) -> Image.Image:
    """A smooth picture with some texture, so every frequency band carries data."""
    rng = random.Random(seed)
    img = Image.new("RGB", (w, h))
    px = img.load()
    for y in range(h):
        for x in range(w):
            r = (x * 255 // max(1, w - 1) + rng.randrange(-20, 21)) % 256
            g = (y * 255 // max(1, h - 1) + rng.randrange(-20, 21)) % 256
            b = ((x * y) % 256 + (40 if (x // 5 + y // 7) % 2 else 0)) % 256
            px[x, y] = (r, g, b)
    return img.convert(mode)


def save(img: Image.Image, **kw) -> bytes:
    out = io.BytesIO()
    img.save(out, "JPEG", **kw)
    return out.getvalue()


def segments(data: bytes):
    """Marker segments before the first scan: (offset, marker, length including the length field)."""
    i = 2
    while i < len(data):
        assert data[i] == 0xFF
        m = data[i + 1]
        n = (data[i + 2] << 8) | data[i + 3]
        yield i, m, n
        if m == 0xDA:
            return
        i += 2 + n


def as_rgb(data: bytes) -> bytes:
    """Drop the JFIF marker and rename the three components R, G and B."""
    out = bytearray(data)
    for off, m, n in list(segments(data)):
        if m in (0xC0, 0xC2):
            for c in range(3):
                out[off + 10 + 3 * c] = b"RGB"[c]
        if m == 0xDA:
            for c in range(3):
                out[off + 5 + 2 * c] = b"RGB"[c]
    for off, m, n in list(segments(data)):
        if m == 0xE0:
            return bytes(out[:off] + out[off + 2 + n:])
    return bytes(out)


def as_ycck(data: bytes) -> bytes:
    """Set the Adobe transform of a four-component file to 2."""
    out = bytearray(data)
    for off, m, n in segments(data):
        if m == 0xEE:
            out[off + 4 + 11] = 2
    return bytes(out)


def main() -> None:
    files = []
    def add(name, data):
        files.append({"name": name, "data": data.hex()})
    rgb = picture("RGB", 45, 29, 1)
    add("444", save(rgb, quality=90, subsampling=0))
    add("422", save(rgb, quality=80, subsampling=1))
    add("420", save(rgb, quality=75, subsampling=2))
    add("420-progressive", save(rgb, quality=75, subsampling=2, progressive=True))
    add("444-progressive", save(rgb, quality=95, subsampling=0, progressive=True))
    add("422-optimized", save(rgb, quality=60, subsampling=1, optimize=True))
    add("420-restart", save(rgb, quality=70, subsampling=2, restart_marker_blocks=1))
    add("444-restart-rows", save(rgb, quality=70, subsampling=0, restart_marker_rows=1))
    add("progressive-restart", save(rgb, quality=70, subsampling=2, progressive=True, restart_marker_blocks=2))
    add("q100", save(picture("RGB", 17, 9, 2), quality=100, subsampling=0))
    add("q1", save(picture("RGB", 23, 31, 3), quality=1, subsampling=2))
    add("tiny", save(picture("RGB", 1, 1, 4), quality=75))
    add("wide", save(picture("RGB", 70, 3, 5), quality=85, subsampling=2, progressive=True))
    gray = picture("L", 33, 21, 6)
    add("gray", save(gray, quality=80))
    add("gray-progressive", save(gray, quality=50, progressive=True))
    add("gray-restart", save(gray, quality=60, restart_marker_blocks=3))
    cmyk = picture("CMYK", 19, 14, 7)
    cmyk_data = save(cmyk, quality=85)
    add("cmyk", cmyk_data)
    add("cmyk-progressive", save(cmyk, quality=85, progressive=True))
    add("ycck", as_ycck(cmyk_data))
    add("rgb", as_rgb(save(picture("RGB", 26, 18, 8), quality=90, subsampling=0)))
    json.dump(files, sys.stdout, indent=0)
    sys.stdout.write("\n")


if __name__ == "__main__":
    main()
