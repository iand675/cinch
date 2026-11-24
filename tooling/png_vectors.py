#!/usr/bin/env python3
"""Write independent PNG decoding vectors.

An encoder written here from the PNG specification produces images of every
color type and bit depth, with each of the five filter types (and a per-row
mix), with and without Adam7 interlacing, and with tRNS transparency. Each
vector records the expected raster in Image.Raster's conventions: sub-byte
gray is scaled to eight bits, sub-byte palette indices are widened, gray with
a tRNS key becomes gray-alpha, RGB with a key becomes RGBA, and palettes are
RGBA quadruples. PNG files written by PIL are included as a second producer.

    python3 tooling/png_vectors.py > seed/tests/fixtures/png-vectors.json
"""
from __future__ import annotations

import io
import warnings
import json
import random
import struct
import sys
import zlib

CHANNELS = {0: 1, 2: 3, 3: 1, 4: 2, 6: 4}
PASSES = [(0, 0, 8, 8), (4, 0, 8, 8), (0, 4, 4, 8), (2, 0, 4, 4), (0, 2, 2, 4), (1, 0, 2, 2), (0, 1, 1, 2)]
FORMATS = {"Gray8": 0, "Gray16": 1, "GrayAlpha8": 2, "GrayAlpha16": 3, "Rgb8": 4, "Rgb16": 5, "Rgba8": 6, "Rgba16": 7, "Indexed": 8}


def chunk(kind: bytes, data: bytes) -> bytes:
    """One length-prefixed, CRC-terminated chunk."""
    return struct.pack(">I", len(data)) + kind + data + struct.pack(">I", zlib.crc32(kind + data) & 0xFFFFFFFF)


def pack_row(samples: list[int], depth: int) -> bytes:
    """Pack samples of a bit depth MSB-first into bytes."""
    if depth == 16:
        return b"".join(struct.pack(">H", s) for s in samples)
    if depth == 8:
        return bytes(samples)
    out, acc, bits = bytearray(), 0, 0
    for s in samples:
        acc = (acc << depth) | s
        bits += depth
        if bits == 8:
            out.append(acc)
            acc, bits = 0, 0
    if bits:
        out.append(acc << (8 - bits))
    return bytes(out)


def paeth(a: int, b: int, c: int) -> int:
    """The Paeth predictor."""
    p = a + b - c
    pa, pb, pc = abs(p - a), abs(p - b), abs(p - c)
    return a if pa <= pb and pa <= pc else (b if pb <= pc else c)


def filter_rows(rows: list[bytes], bpp: int, choice) -> bytes:
    """Filter each row with the chosen filter type."""
    out = bytearray()
    prior = bytes(len(rows[0])) if rows else b""
    for y, row in enumerate(rows):
        f = choice(y)
        out.append(f)
        for i, x in enumerate(row):
            a = row[i - bpp] if i >= bpp else 0
            b = prior[i]
            c = prior[i - bpp] if i >= bpp else 0
            pred = [0, a, b, (a + b) // 2, paeth(a, b, c)][f]
            out.append((x - pred) & 0xFF)
        prior = row
    return bytes(out)


def encode(width, height, ctype, depth, samples, interlace, choice, palette=None, trns=None) -> bytes:
    """Encode samples[y][x] (lists of channel values) as a PNG file."""
    channels = CHANNELS[ctype]
    bpp = max(1, channels * depth // 8)
    def image_rows(xs, ys):
        return [pack_row([v for x in xs for v in samples[y][x]], depth) for y in ys]
    if interlace:
        raw = b""
        for x0, y0, dx, dy in PASSES:
            xs, ys = list(range(x0, width, dx)), list(range(y0, height, dy))
            if xs and ys:
                raw += filter_rows(image_rows(xs, ys), bpp, choice)
    else:
        raw = filter_rows(image_rows(range(width), range(height)), bpp, choice)
    out = b"\x89PNG\r\n\x1a\n" + chunk(b"IHDR", struct.pack(">IIBBBBB", width, height, depth, ctype, 0, 0, interlace))
    if palette is not None:
        out += chunk(b"PLTE", bytes(v for rgb in palette for v in rgb))
    if trns is not None:
        out += chunk(b"tRNS", trns)
    compressed = zlib.compress(raw, 6)
    half = len(compressed) // 2
    out += chunk(b"IDAT", compressed[:half]) + chunk(b"tEXt", b"Comment\x00split") + chunk(b"IDAT", compressed[half:])
    return out + chunk(b"IEND", b"")


def expected(width, height, ctype, depth, samples, palette=None, trns=None) -> tuple[str, bytes, bytes]:
    """The raster an Image.Raster decoder must produce."""
    out = bytearray()
    pal = b""
    def be(v, d):
        return struct.pack(">H", v) if d == 16 else bytes([v])
    if ctype == 0:
        scale = 255 // ((1 << depth) - 1) if depth < 8 else 1
        key = struct.unpack(">H", trns)[0] if trns else None
        fmt = ("GrayAlpha16" if depth == 16 else "GrayAlpha8") if trns else ("Gray16" if depth == 16 else "Gray8")
        for y in range(height):
            for x in range(width):
                v = samples[y][x][0]
                out += be(v * scale if depth < 8 else v, 16 if depth == 16 else 8)
                if trns:
                    out += be(0 if v == key else (65535 if depth == 16 else 255), 16 if depth == 16 else 8)
        return fmt, bytes(out), pal
    if ctype == 2:
        key = struct.unpack(">HHH", trns) if trns else None
        fmt = ("Rgba16" if depth == 16 else "Rgba8") if trns else ("Rgb16" if depth == 16 else "Rgb8")
        for y in range(height):
            for x in range(width):
                v = samples[y][x]
                out += b"".join(be(c, depth) for c in v)
                if trns:
                    out += be(0 if tuple(v) == key else (65535 if depth == 16 else 255), depth)
        return fmt, bytes(out), pal
    if ctype == 3:
        alphas = list(trns or b"")
        pal = b"".join(bytes(rgb) + bytes([alphas[i] if i < len(alphas) else 255]) for i, rgb in enumerate(palette))
        for y in range(height):
            for x in range(width):
                out.append(samples[y][x][0])
        return "Indexed", bytes(out), pal
    fmt = {(4, 8): "GrayAlpha8", (4, 16): "GrayAlpha16", (6, 8): "Rgba8", (6, 16): "Rgba16"}[(ctype, depth)]
    for y in range(height):
        for x in range(width):
            out += b"".join(be(c, depth) for c in samples[y][x])
    return fmt, bytes(out), pal


def main() -> None:
    """Emit decoding vectors across the format space, PIL files, and malformed inputs."""
    warnings.simplefilter("ignore")
    rng = random.Random(1999)
    vectors = []
    combos = [(0, d) for d in (1, 2, 4, 8, 16)] + [(2, 8), (2, 16), (3, 1), (3, 2), (3, 4), (3, 8), (4, 8), (4, 16), (6, 8), (6, 16)]
    choices = [lambda y, f=f: f for f in range(5)] + [lambda y: y % 5]
    for ctype, depth in combos:
        for size in [(1, 1), (13, 7), (37, 21)]:
            width, height = size
            channels = CHANNELS[ctype]
            limit = (1 << depth) - 1
            palette = [(rng.randrange(256), rng.randrange(256), rng.randrange(256)) for _ in range(min(256, 1 << depth))] if ctype == 3 else None
            samples = [[[rng.randrange(min(limit, len(palette) - 1) + 1) if ctype == 3 else rng.randrange(limit + 1) for _ in range(channels)] for _ in range(width)] for _ in range(height)]
            for interlace in (0, 1):
                for index, choice in enumerate(choices):
                    if size == (1, 1) and index > 0:
                        continue
                    if size == (37, 21) and index != 5:
                        continue
                    trns_options = [None]
                    if ctype == 0:
                        trns_options.append(struct.pack(">H", samples[0][0][0]))
                    if ctype == 2:
                        trns_options.append(struct.pack(">HHH", *samples[0][0]))
                    if ctype == 3:
                        trns_options.append(bytes(rng.randrange(256) for _ in range(min(len(palette), 5))))
                    for trns in trns_options:
                        if trns is not None and (index > 1 or interlace):
                            continue
                        png = encode(width, height, ctype, depth, samples, interlace, choice, palette, trns)
                        fmt, pixels, pal = expected(width, height, ctype, depth, samples, palette, trns)
                        vectors.append({"png": png.hex(), "format": FORMATS[fmt], "width": width, "height": height, "pixels": pixels.hex(), "palette": pal.hex()})
    try:
        from PIL import Image
        for mode in ("L", "RGB", "RGBA", "LA", "P", "1", "I;16"):
            img = Image.new(mode, (11, 6))
            for y in range(6):
                for x in range(11):
                    if mode in ("L", "P"):
                        img.putpixel((x, y), (x * 23 + y * 7) % 256)
                    elif mode == "1":
                        img.putpixel((x, y), (x + y) % 2)
                    elif mode == "I;16":
                        img.putpixel((x, y), (x * 5000 + y * 977) % 65536)
                    elif mode == "LA":
                        img.putpixel((x, y), ((x * 23) % 256, (y * 40) % 256))
                    else:
                        img.putpixel((x, y), tuple((x * 23 + y * 7 + k * 50) % 256 for k in range(len(mode))))
            buf = io.BytesIO()
            img.save(buf, "PNG")
            data = buf.getvalue()
            fmt, w, h, px, pal = from_pil(Image.open(io.BytesIO(data)), mode)
            vectors.append({"png": data.hex(), "format": FORMATS[fmt], "width": w, "height": h, "pixels": px.hex(), "palette": pal.hex()})
    except ImportError:
        pass
    good = bytes.fromhex(vectors[10]["png"])
    bad_crc = bytearray(good)
    bad_crc[29] ^= 1
    invalid = [b"not a png".hex(), good[:40].hex(), bytes(bad_crc).hex(), (good[:8] + chunk(b"IHDR", struct.pack(">IIBBBBB", 1, 1, 3, 0, 0, 0, 0)) + chunk(b"IEND", b"")).hex(),
               (good[:8] + chunk(b"IHDR", struct.pack(">IIBBBBB", 0, 1, 8, 0, 0, 0, 0)) + chunk(b"IEND", b"")).hex()]
    json.dump({"vectors": vectors, "invalid": invalid}, sys.stdout)
    sys.stdout.write("\n")


def from_pil(img, mode):
    """Expected raster for a PIL-decoded image of a known mode."""
    w, h = img.size
    if mode in ("L", "1"):
        return "Gray8", w, h, bytes(img.convert("L").getdata()), b""
    if mode == "I;16":
        return "Gray16", w, h, b"".join(struct.pack(">H", v) for v in img.getdata()), b""
    if mode == "LA":
        return "GrayAlpha8", w, h, bytes(v for px in img.getdata() for v in px), b""
    if mode == "P":
        raw = img.getpalette()
        count = len(raw) // 3
        alpha = img.info.get("transparency")
        pal = b"".join(bytes(raw[i * 3:i * 3 + 3]) + bytes([255 if alpha is None or (isinstance(alpha, int) and alpha != i) else 0]) for i in range(count))
        return "Indexed", w, h, bytes(img.getdata()), pal
    if mode == "RGB":
        return "Rgb8", w, h, bytes(v for px in img.getdata() for v in px), b""
    return "Rgba8", w, h, bytes(v for px in img.getdata() for v in px), b""


if __name__ == "__main__":
    main()
