#!/usr/bin/env python3
"""Generate the SIMD kernels of Image.Jpeg.

Every kernel is written once against a small lane DSL with two back ends: one
emits LLVM bodies (little- and big-endian variants, so lane numbering matches
on every target) and one evaluates the same operations on Python integers. The
generator runs the evaluator against literal scalar models of the codec's
integer arithmetic on random and extreme inputs before writing anything, so the
emitted IR is the code that was checked.

Kernels:

  * idct-rows0..3 dequantize one coefficient block (sixty-four little-endian
    i32 values in natural order) against a natural-order quantization table,
    run the two-pass integer inverse DCT in <8 x i32> lanes (the block is
    transposed between passes) and return two finished sample rows (rows 2k and
    2k+1, clamped to 0..255) as sixteen bytes. Each call recomputes the whole
    transform; compilers drop the stage-four outputs a quarter does not use.
  * ycc-rgb4 converts four YCbCr samples to twelve RGB bytes.
  * ff-span finds the first 0xFF among sixteen entropy-coded bytes.
  * huff-decode resolves one Huffman code from a 64-bit window: the eight-bit
    lookup table first, then a branch-free comparison of the sixteen-bit window
    against all sixteen left-justified code limits, and extends the following
    magnitude bits; ac-decode does the same for one step of a first-scan AC
    band read straight from the destuffed segment, also deciding whether the
    code places a coefficient (with its natural position), skips zeros or ends
    the band.
  * fdct-row is the first forward-DCT pass of one sample row; the pass is
    linear modulo 2^32 up to its final shifts, so it is evaluated as an integer
    matrix product. fdct-quant0..7 finish column pass k for all eight columns
    and quantize with the encoder's rounding division, which is exact in f64
    lanes because quotients of numbers below 2^16 by divisors below 2^11 never
    lie within one ulp of an integer.
  * zz-mask finds the nonzero quantized coefficients of a block in zig-zag
    order as a 64-bit mask, zz-coef reads one through the zig-zag order, and
    rle-bits builds the combined code and magnitude bits of a run and value.
  * rgba-ycc4 and rgb-ycc4 convert four non-premultiplied RGBA or four RGB
    pixels to Y, Cb and Cr bytes, premultiplying alpha the way sixteen-bit
    colour readers do; chroma-down averages 2x2 chroma neighbourhoods.

The inverse transform follows the MPEG Software Simulation Group's integer
IDCT; the forward transform follows the Independent JPEG Group's jfdctint.

    python3 tooling/generate_jpeg_kernels.py          # rewrite the generated suffix
    python3 tooling/generate_jpeg_kernels.py --check  # fail when it is stale
"""
from __future__ import annotations

import argparse
import random
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

ROOT = Path(__file__).resolve().parents[1]
TARGET = ROOT / "seed/prelude/Image/Jpeg.cinch"
MARKER = "  -- Generated JPEG kernels: transforms, colour conversion, entropy scanning and Huffman lookup.\n"

UNZIG = [0, 1, 8, 16, 9, 2, 3, 10, 17, 24, 32, 25, 18, 11, 4, 5, 12, 19, 26, 33, 40, 48, 41, 34, 27, 20, 13, 6, 7, 14, 21, 28,
         35, 42, 49, 56, 57, 50, 43, 36, 29, 22, 15, 23, 30, 37, 44, 51, 58, 59, 52, 45, 38, 31, 39, 46, 53, 60, 61, 54, 47, 55, 62, 63]

W1, W2, W3, W5, W6, W7, R2 = 2841, 2676, 2408, 1609, 1108, 565, 181
FIX = dict(f0298=2446, f0390=3196, f0541=4433, f0765=6270, f0899=7373, f1175=9633, f1501=12299, f1847=15137, f1961=16069,
           f2053=16819, f2562=20995, f3072=25172)


def s32(x: int) -> int:
    """Wrap to a signed 32-bit value."""
    x &= 0xFFFFFFFF
    return x - (1 << 32) if x & 0x80000000 else x


# ---------------------------------------------------------------------------
# Literal scalar models.
# ---------------------------------------------------------------------------

def idct_model(b: list[int]) -> list[int]:
    """The two-pass integer IDCT on a dequantized block, in 32-bit wrapping arithmetic; returns clamped samples."""
    b = [s32(v) for v in b]
    for y in range(8):
        s = b[8 * y:8 * y + 8]
        if all(v == 0 for v in s[1:]):
            dc = s32(s[0] << 3)
            b[8 * y:8 * y + 8] = [dc] * 8
            continue
        x0 = s32((s[0] << 11) + 128); x1 = s32(s[4] << 11); x2 = s[6]; x3 = s[2]; x4 = s[1]; x5 = s[7]; x6 = s[5]; x7 = s[3]
        x8 = s32(W7 * (x4 + x5)); x4 = s32(x8 + (W1 - W7) * x4); x5 = s32(x8 - (W1 + W7) * x5)
        x8 = s32(W3 * (x6 + x7)); x6 = s32(x8 - (W3 - W5) * x6); x7 = s32(x8 - (W3 + W5) * x7)
        x8 = s32(x0 + x1); x0 = s32(x0 - x1); x1 = s32(W6 * (x3 + x2)); x2 = s32(x1 - (W2 + W6) * x2); x3 = s32(x1 + (W2 - W6) * x3)
        x1 = s32(x4 + x6); x4 = s32(x4 - x6); x6 = s32(x5 + x7); x5 = s32(x5 - x7)
        x7 = s32(x8 + x3); x8 = s32(x8 - x3); x3 = s32(x0 + x2); x0 = s32(x0 - x2)
        x2 = s32(R2 * (x4 + x5) + 128) >> 8; x4 = s32(R2 * (x4 - x5) + 128) >> 8
        b[8 * y:8 * y + 8] = [s32(x7 + x1) >> 8, s32(x3 + x2) >> 8, s32(x0 + x4) >> 8, s32(x8 + x6) >> 8,
                              s32(x8 - x6) >> 8, s32(x0 - x4) >> 8, s32(x3 - x2) >> 8, s32(x7 - x1) >> 8]
    for x in range(8):
        s = [b[8 * j + x] for j in range(8)]
        y0 = s32((s[0] << 8) + 8192); y1 = s32(s[4] << 8); y2 = s[6]; y3 = s[2]; y4 = s[1]; y5 = s[7]; y6 = s[5]; y7 = s[3]
        y8 = s32(W7 * (y4 + y5) + 4); y4 = s32(y8 + (W1 - W7) * y4) >> 3; y5 = s32(y8 - (W1 + W7) * y5) >> 3
        y8 = s32(W3 * (y6 + y7) + 4); y6 = s32(y8 - (W3 - W5) * y6) >> 3; y7 = s32(y8 - (W3 + W5) * y7) >> 3
        y8 = s32(y0 + y1); y0 = s32(y0 - y1); y1 = s32(W6 * (y3 + y2) + 4); y2 = s32(y1 - (W2 + W6) * y2) >> 3; y3 = s32(y1 + (W2 - W6) * y3) >> 3
        y1 = s32(y4 + y6); y4 = s32(y4 - y6); y6 = s32(y5 + y7); y5 = s32(y5 - y7)
        y7 = s32(y8 + y3); y8 = s32(y8 - y3); y3 = s32(y0 + y2); y0 = s32(y0 - y2)
        y2 = s32(R2 * (y4 + y5) + 128) >> 8; y4 = s32(R2 * (y4 - y5) + 128) >> 8
        out = [s32(y7 + y1) >> 14, s32(y3 + y2) >> 14, s32(y0 + y4) >> 14, s32(y8 + y6) >> 14,
               s32(y8 - y6) >> 14, s32(y0 - y4) >> 14, s32(y3 - y2) >> 14, s32(y7 - y1) >> 14]
        for j in range(8):
            b[8 * j + x] = out[j]
    return [0 if c < -128 else 255 if c > 127 else c + 128 for c in b]


def fdct_rows_model(b: list[int]) -> list[int]:
    """The forward DCT's row pass (samples 0..255 in, 32-bit results)."""
    f = FIX
    out = list(b)
    for y in range(8):
        x0, x1, x2, x3, x4, x5, x6, x7 = b[8 * y:8 * y + 8]
        tmp0, tmp1, tmp2, tmp3 = x0 + x7, x1 + x6, x2 + x5, x3 + x4
        tmp10, tmp12, tmp11, tmp13 = tmp0 + tmp3, tmp0 - tmp3, tmp1 + tmp2, tmp1 - tmp2
        tmp0, tmp1, tmp2, tmp3 = x0 - x7, x1 - x6, x2 - x5, x3 - x4
        s = [0] * 8
        s[0] = s32((tmp10 + tmp11 - 8 * 128) << 2)
        s[4] = s32((tmp10 - tmp11) << 2)
        z1 = s32((tmp12 + tmp13) * f["f0541"] + (1 << 10))
        s[2] = s32(z1 + tmp12 * f["f0765"]) >> 11
        s[6] = s32(z1 - tmp13 * f["f1847"]) >> 11
        tmp10, tmp11, tmp12, tmp13 = tmp0 + tmp3, tmp1 + tmp2, tmp0 + tmp2, tmp1 + tmp3
        z1 = s32((tmp12 + tmp13) * f["f1175"] + (1 << 10))
        tmp0 *= f["f1501"]; tmp1 *= f["f3072"]; tmp2 *= f["f2053"]; tmp3 *= f["f0298"]
        tmp10 *= -f["f0899"]; tmp11 *= -f["f2562"]; tmp12 *= -f["f0390"]; tmp13 *= -f["f1961"]
        tmp12 += z1; tmp13 += z1
        s[1] = s32(tmp0 + tmp10 + tmp12) >> 11
        s[3] = s32(tmp1 + tmp11 + tmp13) >> 11
        s[5] = s32(tmp2 + tmp11 + tmp12) >> 11
        s[7] = s32(tmp3 + tmp10 + tmp13) >> 11
        out[8 * y:8 * y + 8] = s
    return out


def fdct_cols_model(b: list[int]) -> list[int]:
    """The forward DCT's column pass over row-pass results."""
    f = FIX
    out = list(b)
    for x in range(8):
        c = [b[8 * j + x] for j in range(8)]
        tmp0, tmp1, tmp2, tmp3 = c[0] + c[7], c[1] + c[6], c[2] + c[5], c[3] + c[4]
        tmp10, tmp12, tmp11, tmp13 = tmp0 + tmp3 + 2, tmp0 - tmp3, tmp1 + tmp2, tmp1 - tmp2
        tmp0, tmp1, tmp2, tmp3 = c[0] - c[7], c[1] - c[6], c[2] - c[5], c[3] - c[4]
        r = [0] * 8
        r[0] = s32(tmp10 + tmp11) >> 2
        r[4] = s32(tmp10 - tmp11) >> 2
        z1 = s32((tmp12 + tmp13) * f["f0541"] + (1 << 14))
        r[2] = s32(z1 + tmp12 * f["f0765"]) >> 15
        r[6] = s32(z1 - tmp13 * f["f1847"]) >> 15
        tmp10, tmp11, tmp12, tmp13 = tmp0 + tmp3, tmp1 + tmp2, tmp0 + tmp2, tmp1 + tmp3
        z1 = s32((tmp12 + tmp13) * f["f1175"] + (1 << 14))
        tmp0 *= f["f1501"]; tmp1 *= f["f3072"]; tmp2 *= f["f2053"]; tmp3 *= f["f0298"]
        tmp10 *= -f["f0899"]; tmp11 *= -f["f2562"]; tmp12 *= -f["f0390"]; tmp13 *= -f["f1961"]
        tmp12 += z1; tmp13 += z1
        r[1] = s32(tmp0 + tmp10 + tmp12) >> 15
        r[3] = s32(tmp1 + tmp11 + tmp13) >> 15
        r[5] = s32(tmp2 + tmp11 + tmp12) >> 15
        r[7] = s32(tmp3 + tmp10 + tmp13) >> 15
        for j in range(8):
            out[8 * j + x] = r[j]
    return out


def quant_div(a: int, b: int) -> int:
    """The encoder's rounding division of a coefficient by a positive divisor."""
    return (a + (b >> 1)) // b if a >= 0 else -((-a + (b >> 1)) // b)


def clamp24(v: int) -> int:
    """The colour converters' clamp of a 16.16 value to a byte."""
    v = s32(v)
    return v >> 16 if 0 <= v < (1 << 24) else (0 if v < 0 else 255)


def ycbcr_to_rgb(y: int, cb: int, cr: int) -> tuple[int, int, int]:
    """8-bit YCbCr to RGB."""
    yy, cb1, cr1 = y * 0x10101, cb - 128, cr - 128
    return clamp24(yy + 91881 * cr1), clamp24(yy - 22554 * cb1 - 46802 * cr1), clamp24(yy + 116130 * cb1)


def rgb_to_ycbcr(r: int, g: int, b: int) -> tuple[int, int, int]:
    """8-bit RGB to YCbCr."""
    yy = (19595 * r + 38470 * g + 7471 * b + (1 << 15)) >> 16
    return yy, clamp24(-11056 * r - 21712 * g + 32768 * b + (257 << 15)), clamp24(32768 * r - 27440 * g - 5328 * b + (257 << 15))


def premultiply(c: int, a: int) -> int:
    """An 8-bit non-premultiplied channel as the high byte of its sixteen-bit premultiplied value."""
    return ((c * 0x101 * a) // 0xFF) >> 8


from lane_dsl import IR, Ev, parse, bits, splat, const  # noqa: E402


V8 = "<8 x i32>"


def k(b, v, ty=V8):
    return b.c(ty, v)


# ---------------------------------------------------------------------------
# Kernel bodies.
# ---------------------------------------------------------------------------

def transpose(b, rows):
    """Transpose eight <8 x i32> rows."""
    lo, hi = [0, 8, 1, 9, 4, 12, 5, 13], [2, 10, 3, 11, 6, 14, 7, 15]
    t = []
    for i in range(0, 8, 2):
        t += [b.shuf(rows[i], rows[i + 1], lo), b.shuf(rows[i], rows[i + 1], hi)]
    q_lo, q_hi = [0, 1, 8, 9, 4, 5, 12, 13], [2, 3, 10, 11, 6, 7, 14, 15]
    tt = [b.shuf(t[0], t[2], q_lo), b.shuf(t[0], t[2], q_hi), b.shuf(t[1], t[3], q_lo), b.shuf(t[1], t[3], q_hi),
          b.shuf(t[4], t[6], q_lo), b.shuf(t[4], t[6], q_hi), b.shuf(t[5], t[7], q_lo), b.shuf(t[5], t[7], q_hi)]
    p_lo, p_hi = [0, 1, 2, 3, 8, 9, 10, 11], [4, 5, 6, 7, 12, 13, 14, 15]
    return [b.shuf(tt[i], tt[i + 4], p_lo) for i in range(4)] + [b.shuf(tt[i], tt[i + 4], p_hi) for i in range(4)]


def idct_pass(b, s, horizontal: bool):
    """One lane-wise 1-D IDCT over eight vectors."""
    add = lambda x, y: b.bin("add", x, y)
    sub = lambda x, y: b.bin("sub", x, y)
    mul = lambda c, x: b.bin("mul", k(b, c), x)
    sh = lambda op, x, n: b.bin(op, x, k(b, n))
    if horizontal:
        x0 = add(sh("shl", s[0], 11), k(b, 128)); x1 = sh("shl", s[4], 11)
    else:
        x0 = add(sh("shl", s[0], 8), k(b, 8192)); x1 = sh("shl", s[4], 8)
    x2, x3, x4, x5, x6, x7 = s[6], s[2], s[1], s[7], s[5], s[3]
    if horizontal:
        x8 = mul(W7, add(x4, x5)); x4 = add(x8, mul(W1 - W7, x4)); x5 = sub(x8, mul(W1 + W7, x5))
        x8 = mul(W3, add(x6, x7)); x6 = sub(x8, mul(W3 - W5, x6)); x7 = sub(x8, mul(W3 + W5, x7))
        x8 = add(x0, x1); x0 = sub(x0, x1); x1 = mul(W6, add(x3, x2)); x2 = sub(x1, mul(W2 + W6, x2)); x3 = add(x1, mul(W2 - W6, x3))
    else:
        x8 = add(mul(W7, add(x4, x5)), k(b, 4)); x4 = sh("ashr", add(x8, mul(W1 - W7, x4)), 3); x5 = sh("ashr", sub(x8, mul(W1 + W7, x5)), 3)
        x8 = add(mul(W3, add(x6, x7)), k(b, 4)); x6 = sh("ashr", sub(x8, mul(W3 - W5, x6)), 3); x7 = sh("ashr", sub(x8, mul(W3 + W5, x7)), 3)
        x8 = add(x0, x1); x0 = sub(x0, x1); x1 = add(mul(W6, add(x3, x2)), k(b, 4))
        x2 = sh("ashr", sub(x1, mul(W2 + W6, x2)), 3); x3 = sh("ashr", add(x1, mul(W2 - W6, x3)), 3)
    x1, x4 = add(x4, x6), sub(x4, x6)
    x6, x5 = add(x5, x7), sub(x5, x7)
    x7, x8 = add(x8, x3), sub(x8, x3)
    x3, x0 = add(x0, x2), sub(x0, x2)
    x2 = sh("ashr", add(mul(R2, add(x4, x5)), k(b, 128)), 8)
    x4 = sh("ashr", add(mul(R2, sub(x4, x5)), k(b, 128)), 8)
    s_out = 8 if horizontal else 14
    pairs = [(x7, x1, "add"), (x3, x2, "add"), (x0, x4, "add"), (x8, x6, "add"), (x8, x6, "sub"), (x0, x4, "sub"), (x3, x2, "sub"), (x7, x1, "sub")]
    return [sh("ashr", b.bin(op, a, c), s_out) for a, c, op in pairs]


def idct_quarter(q: int):
    def build(b, blk, qt):
        rows = [b.bin("mul", b.load_le(blk, 32 * y, V8), b.load_le(qt, 32 * y, V8)) for y in range(8)]
        cols = transpose(b, rows)
        h = idct_pass(b, cols, True)
        acs = cols[1]
        for c in cols[2:]:
            acs = b.bin("or", acs, c)
        flat = b.icmp("eq", acs, k(b, 0))
        dc = b.bin("shl", cols[0], k(b, 3))
        h = [b.select(flat, dc, v) for v in h]
        out = idct_pass(b, transpose(b, h), False)
        pair = []
        for r in (2 * q, 2 * q + 1):
            c = b.intr("smax", b.intr("smin", out[r], k(b, 127)), k(b, -128))
            pair.append(b.cast("trunc", b.bin("add", c, k(b, 128)), "<8 x i8>"))
        b.ret128(b.shuf(pair[0], pair[1], list(range(16))))
    return build


def ycc_rgb4(b, y, cb, cr):
    """Four YCbCr samples (low bytes of three U32 words) to twelve RGB bytes."""
    lanes = lambda a: b.cast("zext", b.arg_int(a, 32, "<4 x i8>"), "<4 x i32>")
    T = "<4 x i32>"
    yy = b.bin("mul", lanes(y), k(b, 0x10101, T))
    cb1 = b.bin("sub", lanes(cb), k(b, 128, T))
    cr1 = b.bin("sub", lanes(cr), k(b, 128, T))
    def clamp(v):
        v = b.intr("smax", b.intr("smin", v, k(b, 0xFFFFFF, T)), k(b, 0, T))
        return b.cast("trunc", b.bin("lshr", v, k(b, 16, T)), "<4 x i8>")
    r = clamp(b.bin("add", yy, b.bin("mul", k(b, 91881, T), cr1)))
    g = clamp(b.bin("sub", b.bin("sub", yy, b.bin("mul", k(b, 22554, T), cb1)), b.bin("mul", k(b, 46802, T), cr1)))
    bl = clamp(b.bin("add", yy, b.bin("mul", k(b, 116130, T), cb1)))
    rg = b.shuf(r, g, list(range(8)))
    bz = b.shuf(bl, b.c("<4 x i8>", 0), list(range(8)))
    all16 = b.shuf(rg, bz, list(range(16)))
    order = []
    for i in range(4):
        order += [i, 4 + i, 8 + i]
    b.ret128(b.shuf(all16, None, order + [12, 13, 14, 15]))


def rgba_ycc4(b, px, rgb: bool):
    """Four pixels to Y0..3, Cb0..3, Cr0..3 (twelve bytes)."""
    v = b.arg128(px, "<16 x i8>")
    T = "<4 x i32>"
    step = 3 if rgb else 4
    ch = lambda c: b.cast("zext", b.shuf(v, None, [step * i + c for i in range(4)]), T)
    r, g, bl = ch(0), ch(1), ch(2)
    if not rgb:
        a = ch(3)
        pm = lambda c: b.bin("lshr", b.bin("udiv", b.bin("mul", b.bin("mul", c, k(b, 0x101, T)), a), k(b, 255, T)), k(b, 8, T))
        r, g, bl = pm(r), pm(g), pm(bl)
    lin = lambda cr, cg, cb, off: b.bin("add", b.bin("add", b.bin("add", b.bin("mul", k(b, cr, T), r), b.bin("mul", k(b, cg, T), g)), b.bin("mul", k(b, cb, T), bl)), k(b, off, T))
    yy = b.cast("trunc", b.bin("ashr", lin(19595, 38470, 7471, 1 << 15), k(b, 16, T)), "<4 x i8>")
    def clamp(x):
        x = b.intr("smax", b.intr("smin", x, k(b, 0xFFFFFF, T)), k(b, 0, T))
        return b.cast("trunc", b.bin("lshr", x, k(b, 16, T)), "<4 x i8>")
    cb = clamp(lin(-11056, -21712, 32768, 257 << 15))
    cr = clamp(lin(32768, -27440, -5328, 257 << 15))
    ycb = b.shuf(yy, cb, list(range(8)))
    crz = b.shuf(cr, b.c("<4 x i8>", 0), list(range(8)))
    b.ret128(b.shuf(ycb, crz, list(range(16))))


def chroma_down(b, top, bottom):
    """Rounded means of 2x2 neighbourhoods across two sixteen-byte rows."""
    T = "<16 x i16>"
    a = b.cast("zext", b.arg128(top, "<16 x i8>"), T)
    c = b.cast("zext", b.arg128(bottom, "<16 x i8>"), T)
    s = b.bin("add", a, c)
    ev = b.shuf(s, None, [2 * i for i in range(8)])
    od = b.shuf(s, None, [2 * i + 1 for i in range(8)])
    m = b.bin("lshr", b.bin("add", b.bin("add", ev, od), b.c("<8 x i16>", 2)), b.c("<8 x i16>", 2))
    b.ret_int(b.cast("trunc", m, "<8 x i8>"), 64)


def fdct_row_linear():
    """Matrix M, offsets o and shifts s with row output u = (sum_x M[u][x] * x_x + o[u]) >> s[u] (s may be 0 with a <<2 folded in)."""
    f = FIX
    def pre(x):
        x0, x1, x2, x3, x4, x5, x6, x7 = x
        tmp0, tmp1, tmp2, tmp3 = x0 + x7, x1 + x6, x2 + x5, x3 + x4
        tmp10, tmp12, tmp11, tmp13 = tmp0 + tmp3, tmp0 - tmp3, tmp1 + tmp2, tmp1 - tmp2
        tmp0, tmp1, tmp2, tmp3 = x0 - x7, x1 - x6, x2 - x5, x3 - x4
        p = [0] * 8
        p[0] = (tmp10 + tmp11 - 1024) * 4
        p[4] = (tmp10 - tmp11) * 4
        z1 = (tmp12 + tmp13) * f["f0541"] + 1024
        p[2] = z1 + tmp12 * f["f0765"]
        p[6] = z1 - tmp13 * f["f1847"]
        tmp10, tmp11, tmp12, tmp13 = tmp0 + tmp3, tmp1 + tmp2, tmp0 + tmp2, tmp1 + tmp3
        z1 = (tmp12 + tmp13) * f["f1175"] + 1024
        tmp0 *= f["f1501"]; tmp1 *= f["f3072"]; tmp2 *= f["f2053"]; tmp3 *= f["f0298"]
        tmp10 *= -f["f0899"]; tmp11 *= -f["f2562"]; tmp12 *= -f["f0390"]; tmp13 *= -f["f1961"]
        tmp12 += z1; tmp13 += z1
        p[1] = tmp0 + tmp10 + tmp12; p[3] = tmp1 + tmp11 + tmp13; p[5] = tmp2 + tmp11 + tmp12; p[7] = tmp3 + tmp10 + tmp13
        return p
    o = pre([0] * 8)
    m = [[pre([1 if i == x else 0 for i in range(8)])[u] - o[u] for x in range(8)] for u in range(8)]
    return m, o, [0, 11, 11, 11, 0, 11, 11, 11]


def fdct_col_linear():
    """Matrix, offsets and shifts of the column pass (input index v, output index u)."""
    f = FIX
    def pre(c):
        tmp0, tmp1, tmp2, tmp3 = c[0] + c[7], c[1] + c[6], c[2] + c[5], c[3] + c[4]
        tmp10, tmp12, tmp11, tmp13 = tmp0 + tmp3 + 2, tmp0 - tmp3, tmp1 + tmp2, tmp1 - tmp2
        tmp0, tmp1, tmp2, tmp3 = c[0] - c[7], c[1] - c[6], c[2] - c[5], c[3] - c[4]
        r = [0] * 8
        r[0] = tmp10 + tmp11
        r[4] = tmp10 - tmp11
        z1 = (tmp12 + tmp13) * f["f0541"] + (1 << 14)
        r[2] = z1 + tmp12 * f["f0765"]
        r[6] = z1 - tmp13 * f["f1847"]
        tmp10, tmp11, tmp12, tmp13 = tmp0 + tmp3, tmp1 + tmp2, tmp0 + tmp2, tmp1 + tmp3
        z1 = (tmp12 + tmp13) * f["f1175"] + (1 << 14)
        tmp0 *= f["f1501"]; tmp1 *= f["f3072"]; tmp2 *= f["f2053"]; tmp3 *= f["f0298"]
        tmp10 *= -f["f0899"]; tmp11 *= -f["f2562"]; tmp12 *= -f["f0390"]; tmp13 *= -f["f1961"]
        tmp12 += z1; tmp13 += z1
        r[1] = tmp0 + tmp10 + tmp12; r[3] = tmp1 + tmp11 + tmp13; r[5] = tmp2 + tmp11 + tmp12; r[7] = tmp3 + tmp10 + tmp13
        return r
    o = pre([0] * 8)
    m = [[pre([1 if i == v else 0 for i in range(8)])[u] - o[u] for v in range(8)] for u in range(8)]
    return m, o, [2, 15, 15, 15, 2, 15, 15, 15]


def fdct_row(b, word):
    """The row pass of one row of eight samples, as eight 16-bit lanes."""
    m, o, s = fdct_row_linear()
    x = b.cast("zext", b.arg_int(word, 64, "<8 x i8>"), V8)
    acc = k(b, o)
    for i in range(8):
        lane = b.shuf(x, None, [i] * 8)
        acc = b.bin("add", acc, b.bin("mul", lane, k(b, [m[u][i] for u in range(8)])))
    acc = b.bin("ashr", acc, k(b, s))
    b.ret128(b.cast("trunc", acc, "<8 x i16>"))


def fdct_quant(u: int):
    def build(b, *args):
        rows, divisors = args[:8], args[8]
        m, o, s = fdct_col_linear()
        acc = k(b, o[u])
        for v in range(8):
            p = b.cast("sext", b.arg128(rows[v], "<8 x i16>"), V8)
            if m[u][v] != 0:
                acc = b.bin("add", acc, b.bin("mul", k(b, m[u][v]), p))
        a = b.bin("ashr", acc, k(b, s[u]))
        d = b.cast("zext", b.arg128(divisors, "<8 x i16>"), V8)
        neg = b.icmp("slt", a, k(b, 0))
        mag = b.select(neg, b.bin("sub", k(b, 0), a), a)
        n = b.bin("add", mag, b.bin("lshr", d, k(b, 1)))
        D = "<8 x double>"
        q = b.cast("fptoui", b.bin("fdiv", b.cast("uitofp", n, D), b.cast("uitofp", d, D)), V8)
        res = b.select(neg, b.bin("sub", k(b, 0), q), q)
        b.ret128(b.cast("trunc", res, "<8 x i16>"))
    return build


# ---------------------------------------------------------------------------
# Kernels with hand-written bodies (scanning and Huffman lookup).
# ---------------------------------------------------------------------------

def ff_span_body(big: bool) -> list[str]:
    ff = "<" + ", ".join("i8 -1" for _ in range(16)) + ">"
    idx = "<" + ", ".join(f"i8 {i}" for i in range(16)) + ">"
    sixteen = "<" + ", ".join("i8 16" for _ in range(16)) + ">"
    return [
        "      %base = inttoptr i64 %0 to ptr",
        "      %at = getelementptr i8, ptr %base, i64 %1",
        "      %v = load <16 x i8>, ptr %at, align 1",
        f"      %hit = icmp eq <16 x i8> %v, {ff}",
        f"      %pos = select <16 x i1> %hit, <16 x i8> {idx}, <16 x i8> {sixteen}",
        "      %m = call i8 @llvm.vector.reduce.umin.v16i8(<16 x i8> %pos)",
        "      %r = zext i8 %m to i64",
        "      ret i64 %r",
    ]


HUFF_DOC = (
    "Resolve one Huffman code from a table at the first address: the 64-bit word is the little-endian load at the byte\n"
    "  -- holding the bit position and the third argument is the bit offset within it (0 to 7). The table holds a 256-entry\n"
    "  -- lookup of eight-bit prefixes (value * 256 + length + 1, zero for longer codes) at offset 0, the code values at\n"
    "  -- 512, sixteen limits at 768 (one past the largest code of each length, left-justified to sixteen bits and\n"
    "  -- capped at 2^16; a sixteen-bit window below limit l holds a code of length l + 1 when no shorter one matches)\n"
    "  -- and sixteen value-index offsets at 832. The fourth argument chooses the\n"
    "  -- magnitude size in its low two bits (0 for none, 1 for the code value itself as DC differences use, 2 for its low\n"
    "  -- four bits as AC coefficients use) and a left shift for the magnitude in the next four; higher bits are ignored.\n"
    "  -- The result is length + 256 * value + 65536 * shifted extended magnitude (a wrapping 32-bit field), or zero when\n"
    "  -- no code of at most sixteen bits matches.")


def huff_body(big: bool) -> list[str]:
    lines = [
        "      %tab = inttoptr i64 %0 to ptr",
        "      %w = call i64 @llvm.bswap.i64(i64 %1)",
        "      %win = shl i64 %w, %2",
    ]
    lines += resolve_lines(big)
    return lines + huff_tail()


def resolve_lines(big: bool) -> list[str]:
    """Resolve the code at the top of %win against the table at %tab into %n (0 when none) and %val."""
    lines = [
        "      %top = lshr i64 %win, 56",
        "      %lutp = getelementptr i16, ptr %tab, i64 %top",
        "      %lraw = load i16, ptr %lutp, align 1",
    ]
    if big:
        lines.append("      %lut = call i16 @llvm.bswap.i16(i16 %lraw)")
    else:
        lines.append("      %lut = add i16 %lraw, 0")
    lines += [
        "      %lut64 = zext i16 %lut to i64",
        "      %fast = icmp ne i64 %lut64, 0",
        "      %flen = and i64 %lut64, 255",
        "      %fn = sub i64 %flen, 1",
        "      %fval = lshr i64 %lut64, 8",
        "      %top16 = lshr i64 %win, 48",
        "      %w16 = trunc i64 %top16 to i32",
        "      %ins = insertelement <16 x i32> poison, i32 %w16, i64 0",
        "      %wv = shufflevector <16 x i32> %ins, <16 x i32> poison, <16 x i32> zeroinitializer",
        "      %limp = getelementptr i8, ptr %tab, i64 768",
        "      %limraw = load <16 x i32>, ptr %limp, align 1",
    ]
    if big:
        lines.append("      %lim = call <16 x i32> @llvm.bswap.v16i32(<16 x i32> %limraw)")
    else:
        lines.append("      %lim = add <16 x i32> %limraw, zeroinitializer")
    lines += [
        "      %ok = icmp ult <16 x i32> %wv, %lim",
        "      %okb = select <16 x i1> %ok, <16 x i32> <" + ", ".join(f"i32 {1 << l}" for l in range(16)) + ">, <16 x i32> zeroinitializer",
        "      %okw = call i32 @llvm.vector.reduce.or.v16i32(<16 x i32> %okb)",
        "      %okx = or i32 %okw, 65536",
        "      %first = call i32 @llvm.cttz.i32(i32 %okx, i1 true)",
        "      %found = icmp ult i32 %first, 16",
        "      %li = and i32 %first, 15",
        "      %cs = sub i32 15, %li",
        "      %code = lshr i32 %w16, %cs",
        "      %offp0 = getelementptr i8, ptr %tab, i64 832",
        "      %li64 = zext i32 %li to i64",
        "      %offp = getelementptr i32, ptr %offp0, i64 %li64",
        "      %offraw = load i32, ptr %offp, align 1",
    ]
    if big:
        lines.append("      %off = call i32 @llvm.bswap.i32(i32 %offraw)")
    else:
        lines.append("      %off = add i32 %offraw, 0")
    lines += [
        "      %vi = add i32 %code, %off",
        "      %vim = and i32 %vi, 255",
        "      %vi64 = zext i32 %vim to i64",
        "      %valsp0 = getelementptr i8, ptr %tab, i64 512",
        "      %valsp = getelementptr i8, ptr %valsp0, i64 %vi64",
        "      %sval8 = load i8, ptr %valsp, align 1",
        "      %sval = zext i8 %sval8 to i64",
        "      %slen = add i64 %li64, 1",
        "      %sn = select i1 %found, i64 %slen, i64 0",
        "      %n = select i1 %fast, i64 %fn, i64 %sn",
        "      %val = select i1 %fast, i64 %fval, i64 %sval",
    ]
    return lines


def huff_tail() -> list[str]:
    """Magnitude extension and packing for huff-decode."""
    return [
        "      %kind = and i64 %3, 3",
        "      %isdc = icmp eq i64 %kind, 1",
        "      %isac = icmp eq i64 %kind, 2",
        "      %al0 = lshr i64 %3, 2",
        "      %al = and i64 %al0, 15",
        "      %low4 = and i64 %val, 15",
        "      %dcsize = select i1 %isdc, i64 %val, i64 0",
        "      %t0 = select i1 %isac, i64 %low4, i64 %dcsize",
        "      %tbig = icmp ugt i64 %t0, 16",
        "      %t = select i1 %tbig, i64 0, i64 %t0",
        "      %after = shl i64 %win, %n",
        "      %tz = icmp eq i64 %t, 0",
        "      %rs = sub i64 64, %t",
        "      %rs2 = select i1 %tz, i64 63, i64 %rs",
        "      %raw0 = lshr i64 %after, %rs2",
        "      %raw = select i1 %tz, i64 0, i64 %raw0",
        "      %tm1 = sub i64 %t, 1",
        "      %tm1s = select i1 %tz, i64 0, i64 %tm1",
        "      %half = shl i64 1, %tm1s",
        "      %neg = icmp ult i64 %raw, %half",
        "      %full = shl i64 1, %t",
        "      %adj = sub i64 %raw, %full",
        "      %adj1 = add i64 %adj, 1",
        "      %negt = and i1 %neg, true",
        "      %use = select i1 %tz, i1 false, i1 %negt",
        "      %ext = select i1 %use, i64 %adj1, i64 %raw",
        "      %exts = shl i64 %ext, %al",
        "      %ext32 = and i64 %exts, 4294967295",
        "      %vs = shl i64 %val, 8",
        "      %es = shl i64 %ext32, 16",
        "      %r0 = or i64 %n, %vs",
        "      %r1 = or i64 %r0, %es",
        "      %nz = icmp eq i64 %n, 0",
        "      %r = select i1 %nz, i64 0, i64 %r1",
        "      ret i64 %r",
    ]


AC_DOC = (
    "One step of a first-scan AC band: resolve the Huffman code at bit position p (third argument) of the destuffed\n"
    "  -- segment at the second address with the table at the first (its zig-zag order at offset 960); the fourth\n"
    "  -- argument holds the point transform (bits 0-3), the band's last index (4-9) and the current index (10-16).\n"
    "  -- The result packs the bits consumed (0-5), the next index (6-12), the kind (13-14: 0 a coefficient at a\n"
    "  -- natural position, 1 a skip, 2 an end-of-band run, 3 the band already finished), the natural position or\n"
    "  -- the end-of-band run (16-31) and the shifted coefficient as a wrapping 32-bit field (32-63); zero when no\n"
    "  -- code matches.")


def ac_body(big: bool) -> list[str]:
    lines = [
        "      %tab = inttoptr i64 %0 to ptr",
        "      %seg = inttoptr i64 %1 to ptr",
        "      %byte = lshr i64 %2, 3",
        "      %sh = and i64 %2, 7",
        "      %wp = getelementptr i8, ptr %seg, i64 %byte",
        "      %wraw = load i64, ptr %wp, align 1",
        ("      %w = call i64 @llvm.bswap.i64(i64 %wraw)" if not big else "      %w = add i64 %wraw, 0"),
        "      %win = shl i64 %w, %sh",
    ]
    lines += resolve_lines(big)
    lines += [
        "      %al = and i64 %3, 15",
        "      %ze0 = lshr i64 %3, 4",
        "      %ze = and i64 %ze0, 63",
        "      %zig0 = lshr i64 %3, 10",
        "      %zig = and i64 %zig0, 127",
        "      %done = icmp ugt i64 %zig, %ze",
        "      %size = and i64 %val, 15",
        "      %run = lshr i64 %val, 4",
        "      %z = add i64 %zig, %run",
        "      %hasv = icmp ne i64 %size, 0",
        "      %over = icmp ugt i64 %z, %ze",
        "      %zrl = icmp eq i64 %run, 15",
        "      %t = select i1 %hasv, i64 %size, i64 %run",
        "      %after = shl i64 %win, %n",
        "      %tz = icmp eq i64 %t, 0",
        "      %rs = sub i64 64, %t",
        "      %rs2 = select i1 %tz, i64 63, i64 %rs",
        "      %raw0 = lshr i64 %after, %rs2",
        "      %raw = select i1 %tz, i64 0, i64 %raw0",
        "      %tm1 = sub i64 %t, 1",
        "      %tm1s = select i1 %tz, i64 0, i64 %tm1",
        "      %half = shl i64 1, %tm1s",
        "      %neg = icmp ult i64 %raw, %half",
        "      %full = shl i64 1, %t",
        "      %adj = sub i64 %raw, %full",
        "      %adj1 = add i64 %adj, 1",
        "      %ext = select i1 %neg, i64 %adj1, i64 %raw",
        "      %exts = shl i64 %ext, %al",
        "      %value = and i64 %exts, 4294967295",
        "      %zm = and i64 %z, 63",
        "      %zzp0 = getelementptr i8, ptr %tab, i64 960",
        "      %zzp = getelementptr i8, ptr %zzp0, i64 %zm",
        "      %nat8 = load i8, ptr %zzp, align 1",
        "      %nat = zext i8 %nat8 to i64",
        "      %eob = add i64 %full, %raw",
        "      %ccons = add i64 %n, %size",
        "      %cnext = add i64 %z, 1",
        "      %ze1 = add i64 %ze, 1",
        "      %zig16 = add i64 %zig, 16",
        "      %erun = add i64 %n, %run",
        "      %vcons = select i1 %over, i64 %n, i64 %ccons",
        "      %vnext = select i1 %over, i64 %ze1, i64 %cnext",
        "      %vkind = select i1 %over, i64 1, i64 0",
        "      %zcons = select i1 %zrl, i64 %n, i64 %erun",
        "      %znext = select i1 %zrl, i64 %zig16, i64 0",
        "      %zkind = select i1 %zrl, i64 1, i64 2",
        "      %cons = select i1 %hasv, i64 %vcons, i64 %zcons",
        "      %next = select i1 %hasv, i64 %vnext, i64 %znext",
        "      %kind = select i1 %hasv, i64 %vkind, i64 %zkind",
        "      %isv = and i1 %hasv, true",
        "      %placing = icmp eq i64 %kind, 0",
        "      %field0 = select i1 %placing, i64 %nat, i64 0",
        "      %iseob = icmp eq i64 %kind, 2",
        "      %field = select i1 %iseob, i64 %eob, i64 %field0",
        "      %valf = select i1 %placing, i64 %value, i64 0",
        "      %p1 = shl i64 %next, 6",
        "      %p2 = shl i64 %kind, 13",
        "      %p3 = shl i64 %field, 16",
        "      %p4 = shl i64 %valf, 32",
        "      %q1 = or i64 %cons, %p1",
        "      %q2 = or i64 %q1, %p2",
        "      %q3 = or i64 %q2, %p3",
        "      %q4 = or i64 %q3, %p4",
        "      %none = icmp eq i64 %n, 0",
        "      %r0 = select i1 %none, i64 0, i64 %q4",
        "      %r = select i1 %done, i64 24576, i64 %r0",
        "      ret i64 %r",
    ]
    return lines


def zz_mask_body(big: bool) -> list[str]:
    """Nonzero coefficients of a natural-order block of sixty-four little-endian i16 values, in zig-zag order."""
    lines = ["      %p = inttoptr i64 %0 to ptr", "      %v = load <64 x i16>, ptr %p, align 1"]
    acc = None
    for g in range(4):
        lanes = ", ".join(f"i32 {UNZIG[16 * g + i]}" for i in range(16))
        weights = ", ".join(f"i16 {(1 << i) - (1 << 16) if i == 15 else 1 << i}" for i in range(16))
        lines += [
            f"      %g{g} = shufflevector <64 x i16> %v, <64 x i16> poison, <16 x i32> <{lanes}>",
            f"      %n{g} = icmp ne <16 x i16> %g{g}, zeroinitializer",
            f"      %b{g} = select <16 x i1> %n{g}, <16 x i16> <{weights}>, <16 x i16> zeroinitializer",
            f"      %m{g} = call i16 @llvm.vector.reduce.or.v16i16(<16 x i16> %b{g})",
            f"      %w{g} = zext i16 %m{g} to i64",
            f"      %s{g} = shl i64 %w{g}, {16 * g}",
        ]
        if acc is None:
            acc = f"%s{g}"
        else:
            lines.append(f"      %a{g} = or i64 {acc}, %s{g}")
            acc = f"%a{g}"
    lines.append(f"      ret i64 {acc}")
    return lines


def zz_coef_body(big: bool) -> list[str]:
    """The coefficient at zig-zag position z of a natural-order block, sign-extended."""
    lines = [
        "      %coef = inttoptr i64 %0 to ptr",
        "      %order = inttoptr i64 %1 to ptr",
        "      %zm = and i64 %2, 63",
        "      %op = getelementptr i8, ptr %order, i64 %zm",
        "      %n8 = load i8, ptr %op, align 1",
        "      %n = zext i8 %n8 to i64",
        "      %cp = getelementptr i16, ptr %coef, i64 %n",
        "      %raw = load i16, ptr %cp, align 1",
    ]
    lines.append("      %c = call i16 @llvm.bswap.i16(i16 %raw)" if big else "      %c = add i16 %raw, 0")
    lines += ["      %r = sext i16 %c to i64", "      ret i64 %r"]
    return lines


def rle_bits_body(big: bool) -> list[str]:
    """Code and magnitude bits for a zero run and a signed value: (bits << 6) | length."""
    lines = [
        "      %tab = inttoptr i64 %0 to ptr",
        "      %neg = icmp slt i64 %2, 0",
        "      %nv = sub i64 0, %2",
        "      %a = select i1 %neg, i64 %nv, i64 %2",
        "      %a32 = trunc i64 %a to i32",
        "      %lz = call i32 @llvm.ctlz.i32(i32 %a32, i1 false)",
        "      %nb32 = sub i32 32, %lz",
        "      %nb = zext i32 %nb32 to i64",
        "      %runs = shl i64 %1, 4",
        "      %sym0 = or i64 %runs, %nb",
        "      %sym = and i64 %sym0, 255",
        "      %ep = getelementptr i32, ptr %tab, i64 %sym",
        "      %eraw = load i32, ptr %ep, align 1",
    ]
    lines.append("      %e32 = call i32 @llvm.bswap.i32(i32 %eraw)" if big else "      %e32 = add i32 %eraw, 0")
    lines += [
        "      %e = zext i32 %e32 to i64",
        "      %clen = lshr i64 %e, 24",
        "      %code = and i64 %e, 16777215",
        "      %one = shl i64 1, %nb",
        "      %mask = sub i64 %one, 1",
        "      %vm1 = sub i64 %2, 1",
        "      %mag0 = select i1 %neg, i64 %vm1, i64 %2",
        "      %mag = and i64 %mag0, %mask",
        "      %cs = shl i64 %code, %nb",
        "      %bits = or i64 %cs, %mag",
        "      %len = add i64 %clen, %nb",
        "      %bs = shl i64 %bits, 6",
        "      %r = or i64 %bs, %len",
        "      ret i64 %r",
    ]
    return lines


def window_lines(big: bool, seg: str, pos: str) -> list[str]:
    """Load the 64-bit window at bit position pos of the segment at address seg into %win."""
    return [
        f"      %seg = inttoptr i64 {seg} to ptr",
        f"      %byte = lshr i64 {pos}, 3",
        f"      %sh = and i64 {pos}, 7",
        "      %wp = getelementptr i8, ptr %seg, i64 %byte",
        "      %wraw = load i64, ptr %wp, align 1",
        ("      %w = call i64 @llvm.bswap.i64(i64 %wraw)" if not big else "      %w = add i64 %wraw, 0"),
        "      %win = shl i64 %w, %sh",
    ]


def sym_at_body(big: bool) -> list[str]:
    """huff-decode reading its window straight from a destuffed segment."""
    return ["      %tab = inttoptr i64 %0 to ptr"] + window_lines(big, "%1", "%2") + resolve_lines(big) + [
        line.replace("%3", "%3") for line in huff_tail()]


def seg_bits_body(big: bool) -> list[str]:
    """The k bits (0 to 32) at bit position p of a destuffed segment."""
    return window_lines(big, "%0", "%1") + [
        "      %z = icmp eq i64 %2, 0",
        "      %rs = sub i64 64, %2",
        "      %rs2 = select i1 %z, i64 63, i64 %rs",
        "      %v = lshr i64 %win, %rs2",
        "      %r = select i1 %z, i64 0, i64 %v",
        "      ret i64 %r",
    ]


def coef_at_body(big: bool) -> list[str]:
    """The 32-bit coefficient at zig-zag position z of a natural-order block, as (value << 8) | natural index."""
    lines = [
        "      %blk = inttoptr i64 %0 to ptr",
        "      %order = inttoptr i64 %1 to ptr",
        "      %zm = and i64 %2, 63",
        "      %op = getelementptr i8, ptr %order, i64 %zm",
        "      %n8 = load i8, ptr %op, align 1",
        "      %n = zext i8 %n8 to i64",
        "      %cp = getelementptr i32, ptr %blk, i64 %n",
        "      %raw = load i32, ptr %cp, align 1",
    ]
    lines.append("      %c = call i32 @llvm.bswap.i32(i32 %raw)" if big else "      %c = add i32 %raw, 0")
    lines += ["      %v = sext i32 %c to i64", "      %vs = shl i64 %v, 8", "      %r = or i64 %vs, %n", "      ret i64 %r"]
    return lines


# ---------------------------------------------------------------------------
# Verification.
# ---------------------------------------------------------------------------

def run(build, *args, memory: bytes = b""):
    e = Ev(memory)
    build(e, *args)
    return e.result


def verify() -> None:
    """Check every generated lane program against the scalar models."""
    rng = random.Random(1987)
    # Transpose.
    e = Ev()
    rows = [("<8 x i32>", [8 * r + c for c in range(8)]) for r in range(8)]
    cols = transpose(e, rows)
    assert [c[1] for c in cols] == [[8 * r + c for r in range(8)] for c in range(8)], "transpose"
    # Inverse DCT quarters, including sparse rows (the flat-row shortcut), extremes and wrapping inputs.
    cases = []
    for _ in range(300):
        kind = rng.randrange(5)
        if kind == 0:
            coef = [rng.randrange(-2048, 2048) for _ in range(64)]
        elif kind == 1:
            coef = [rng.randrange(-2048, 2048) if rng.random() < 0.2 else 0 for _ in range(64)]
        elif kind == 2:
            coef = [rng.randrange(-(1 << 31), 1 << 31) for _ in range(64)]
        elif kind == 3:
            coef = [0] * 64
            coef[0] = rng.randrange(-(1 << 22), 1 << 22)
        else:
            coef = [rng.randrange(-64, 64) if i < 10 else 0 for i in range(64)]
        quant = [rng.randrange(1, 256) for _ in range(64)] if rng.random() < 0.7 else [rng.randrange(1, 65536) for _ in range(64)]
        cases.append((coef, quant))
    for coef, quant in cases:
        deq = [s32(c * q) for c, q in zip(coef, quant)]
        want = idct_model(deq)
        mem = b"".join((c & 0xFFFFFFFF).to_bytes(4, "little") for c in coef) + b"".join(q.to_bytes(4, "little") for q in quant)
        for q in range(4):
            got = run(idct_quarter(q), 0, 256, memory=mem)
            assert [(got >> (8 * i)) & 255 for i in range(16)] == want[16 * q:16 * q + 16], ("idct", q)
    # Colour conversion.
    for _ in range(4000):
        ys, cbs, crs = [rng.randrange(256) for _ in range(4)], [rng.randrange(256) for _ in range(4)], [rng.randrange(256) for _ in range(4)]
        word = lambda v: sum(x << (8 * i) for i, x in enumerate(v))
        got = run(ycc_rgb4, word(ys), word(cbs), word(crs))
        want = [c for i in range(4) for c in ycbcr_to_rgb(ys[i], cbs[i], crs[i])]
        assert [(got >> (8 * i)) & 255 for i in range(12)] == want and got >> 96 == 0, "ycc-rgb4"
    for y in (0, 255):
        for cb in range(256):
            for cr in range(0, 256, 5):
                word = lambda x: x * 0x01010101
                got = run(ycc_rgb4, word(y), word(cb), word(cr))
                assert [(got >> (8 * i)) & 255 for i in range(3)] == list(ycbcr_to_rgb(y, cb, cr)), "ycc-rgb4 extremes"
    for rgb in (False, True):
        for _ in range(4000):
            px = [[rng.randrange(256) for _ in range(4)] for _ in range(4)]
            if rng.random() < 0.3:
                for p in px:
                    p[3] = 255
            if rgb:
                raw = [c for p in px for c in p[:3]] + [rng.randrange(256) for _ in range(4)]
            else:
                raw = [c for p in px for c in p]
            got = run(lambda b, a: rgba_ycc4(b, a, rgb), sum(x << (8 * i) for i, x in enumerate(raw)))
            ys, cbs, crs = [], [], []
            for p in px:
                r, g, bl = (p[0], p[1], p[2]) if rgb else (premultiply(p[0], p[3]), premultiply(p[1], p[3]), premultiply(p[2], p[3]))
                yy, cb, cr = rgb_to_ycbcr(r, g, bl)
                ys.append(yy); cbs.append(cb); crs.append(cr)
            assert [(got >> (8 * i)) & 255 for i in range(12)] == ys + cbs + crs and got >> 96 == 0, ("rgba-ycc4", rgb)
    for _ in range(2000):
        t, u = [rng.randrange(256) for _ in range(16)], [rng.randrange(256) for _ in range(16)]
        got = run(chroma_down, sum(x << (8 * i) for i, x in enumerate(t)), sum(x << (8 * i) for i, x in enumerate(u)))
        want = [(t[2 * i] + t[2 * i + 1] + u[2 * i] + u[2 * i + 1] + 2) >> 2 for i in range(8)]
        assert [(got >> (8 * i)) & 255 for i in range(8)] == want, "chroma-down"
    # Forward DCT and quantization, against the literal passes, with extreme blocks and pass-one range checks.
    blocks = [[rng.randrange(256) for _ in range(64)] for _ in range(400)]
    blocks += [[255 * ((x + y) & 1) for y in range(8) for x in range(8)], [0] * 64, [255] * 64,
               [255 * (x < 4) for y in range(8) for x in range(8)], [255 * ((x ^ y) & 1) for y in range(8) for x in range(8)]]
    m1, o1, _ = fdct_row_linear()
    for u in range(8):
        lo = o1[u] + sum(min(0, 255 * c) for c in m1[u])
        hi = o1[u] + sum(max(0, 255 * c) for c in m1[u])
        assert -(1 << 15) <= (lo >> [0, 11, 11, 11, 0, 11, 11, 11][u]) and (hi >> [0, 11, 11, 11, 0, 11, 11, 11][u]) < (1 << 15), "row pass range"
    for blk in blocks:
        rows_model = fdct_rows_model(blk)
        full = fdct_cols_model(rows_model)
        words = []
        for y in range(8):
            got = run(fdct_row, sum(v << (8 * i) for i, v in enumerate(blk[8 * y:8 * y + 8])))
            lanes = [Ev.signed("i16", (got >> (16 * i)) & 0xFFFF) for i in range(8)]
            assert lanes == rows_model[8 * y:8 * y + 8], "fdct-row"
            words.append(got)
        for quality_divs in ([8] * 64, [rng.randrange(1, 256) * 8 for _ in range(64)], [255 * 8] * 64):
            for u in range(8):
                dv = sum(d << (16 * i) for i, d in enumerate(quality_divs[8 * u:8 * u + 8]))
                got = run(fdct_quant(u), *words, dv)
                lanes = [Ev.signed("i16", (got >> (16 * i)) & 0xFFFF) for i in range(8)]
                assert lanes == [quant_div(full[8 * u + x], quality_divs[8 * u + x]) for x in range(8)], ("fdct-quant", u)
    # Exhaustive exactness of the f64 rounding division over its whole domain.
    for d in range(8, 2041, 8):
        for n in range(0, (1 << 15) + 1024, 7):
            assert int(float(n) / float(d)) == n // d


# ---------------------------------------------------------------------------
# Emission.
# ---------------------------------------------------------------------------

def prim(name: str, sig: str, doc: str, build, args: list[str], unsafe: bool = False) -> list[str]:
    out = [f"  -- | {doc}", f"  prim {name} :: {sig} where"]
    for big in (False, True):
        b = IR(big)
        if unsafe:
            b.raw("%blk = inttoptr i64 %0 to ptr")
            b.raw("%qt = inttoptr i64 %1 to ptr")
            build(b, "%blk", "%qt")
        else:
            build(b, *args)
        out += [("    unsafe llvm {" if unsafe else "    llvm {")] + b.lines + ["    }" + (" when endian big" if big else " when endian little")]
    return out


def raw_prim(name: str, sig: str, doc: str, body, unsafe: bool, endian: bool) -> list[str]:
    out = [f"  -- | {doc}", f"  prim {name} :: {sig} where"]
    for big in ((False, True) if endian else (False,)):
        out += [("    unsafe llvm {" if unsafe else "    llvm {")] + body(big) + ["    }" + ((" when endian big" if big else " when endian little") if endian else "")]
    return out


def suffix() -> str:
    verify()
    lines = [MARKER.rstrip("\n"), ""]
    for q in range(4):
        lines += prim(f"idct-rows{q}", "I64 -> I64 -> U128",
                      f"Dequantize the block at the first address by the table at the second (both sixty-four little-endian i32 values in natural order), inverse-transform it and return sample rows {2 * q} and {2 * q + 1} as sixteen bytes.",
                      idct_quarter(q), [], unsafe=True) + [""]
    lines += prim("ycc-rgb4", "U32 -> U32 -> U32 -> U128", "Four Y, Cb and Cr samples (one per byte of each word) as twelve interleaved RGB bytes.",
                  ycc_rgb4, ["%0", "%1", "%2"]) + [""]
    lines += prim("rgba-ycc4", "U128 -> U128", "Four non-premultiplied RGBA pixels as their Y, Cb and Cr bytes (twelve bytes, Y first), after premultiplying alpha.",
                  lambda b, a: rgba_ycc4(b, a, False), ["%0"]) + [""]
    lines += prim("rgb-ycc4", "U128 -> U128", "Four RGB pixels (the low twelve bytes) as their Y, Cb and Cr bytes (twelve bytes, Y first).",
                  lambda b, a: rgba_ycc4(b, a, True), ["%0"]) + [""]
    lines += prim("chroma-down", "U128 -> U128 -> U64", "Rounded means of the eight 2x2 neighbourhoods spanned by two rows of sixteen chroma samples.",
                  chroma_down, ["%0", "%1"]) + [""]
    lines += prim("fdct-row", "U64 -> U128", "The forward transform's row pass over eight samples, as eight signed 16-bit lanes.",
                  fdct_row, ["%0"]) + [""]
    for u in range(8):
        lines += prim(f"fdct-quant{u}", " -> ".join(["U128"] * 10),
                      f"Column pass output row {u} from the eight row-pass results, quantized by the eight 16-bit divisors of the last argument with rounding away from zero at halves.",
                      fdct_quant(u), [f"%{i}" for i in range(9)]) + [""]
    lines += raw_prim("ff-span", "I64 -> I64 -> I64", "The index of the first 0xFF among the sixteen bytes at base + offset, or 16 when there is none.",
                      ff_span_body, unsafe=True, endian=False) + [""]
    lines += raw_prim("huff-decode", "I64 -> U64 -> I64 -> I64 -> I64", HUFF_DOC, huff_body, unsafe=True, endian=True) + [""]
    lines += raw_prim("ac-decode", "I64 -> I64 -> I64 -> I64 -> I64", AC_DOC, ac_body, unsafe=True, endian=True) + [""]
    lines += raw_prim("sym-at", "I64 -> I64 -> I64 -> I64 -> I64", "huff-decode with its window read at bit position p (third argument) of the destuffed segment at the second address; the table and mode are as for huff-decode.",
                      sym_at_body, unsafe=True, endian=True) + [""]
    lines += raw_prim("seg-bits", "I64 -> I64 -> I64 -> I64", "The k bits (third argument, 0 to 32) at bit position p (second) of the destuffed segment at the address, most significant first.",
                      seg_bits_body, unsafe=True, endian=True) + [""]
    lines += raw_prim("coef-at", "I64 -> I64 -> I64 -> I64", "The 32-bit little-endian coefficient at zig-zag position z (third argument) of the natural-order block at the first address, through the zig-zag order at the second: (sign-extended value << 8) | natural index.",
                      coef_at_body, unsafe=True, endian=True) + [""]
    lines += raw_prim("zz-mask", "I64 -> I64", "Bit z set when the coefficient at zig-zag position z of the natural-order block of sixty-four little-endian i16 values at the address is nonzero.",
                      zz_mask_body, unsafe=True, endian=False) + [""]
    lines += raw_prim("zz-coef", "I64 -> I64 -> I64 -> I64", "The coefficient at zig-zag position z (third argument) of the natural-order block at the first address, through the zig-zag order at the second, sign-extended.",
                      zz_coef_body, unsafe=True, endian=True) + [""]
    lines += raw_prim("rle-bits", "I64 -> I64 -> I64 -> I64", "The Huffman code for a zero run (second argument) and the size of a signed value (third), from the 256-entry table of length * 2^24 + code words at the address, followed by the value's magnitude bits (one's complement when negative): (bits << 6) | total length.",
                      rle_bits_body, unsafe=True, endian=True)
    return "\n".join(lines).rstrip() + "\n"


def main() -> None:
    """Rewrite or check the generated suffix of Image.Jpeg."""
    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    existing = TARGET.read_text()
    if existing.count(MARKER) != 1:
        raise SystemExit(f"{TARGET}: expected exactly one generated marker")
    expected = existing.split(MARKER, 1)[0].rstrip() + "\n\n" + suffix()
    if args.check:
        if existing != expected:
            raise SystemExit(f"{TARGET} is stale; rerun tooling/generate_jpeg_kernels.py")
        return
    TARGET.write_text(expected)


if __name__ == "__main__":
    main()
