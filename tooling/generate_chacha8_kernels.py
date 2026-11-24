#!/usr/bin/env python3
"""Generate the SIMD kernels of Data.ChaCha8.

The ChaCha8 generator computes four ChaCha blocks with eight rounds at once, word-interleaved: vector w holds
word w of blocks 0 to 3, so every quarter round is a lane-wise <4 x i32> operation on four vectors. Words 0-3
start as the ChaCha constants, 4-11 as the seed's eight 32-bit halves (splatted), 12 as the four counters and
13-15 as zero; after the rounds only the seed words add their initial values. chacha8-word0..15 each return
one final vector as a U128 (lane i in bits 32i up), which is exactly two consecutive 64-bit outputs; each call
recomputes the rounds. The lane programs are evaluated against a literal scalar model before writing.

    python3 tooling/generate_chacha8_kernels.py          # rewrite the generated suffix
    python3 tooling/generate_chacha8_kernels.py --check  # fail when it is stale
"""
from __future__ import annotations

import argparse
import random
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from lane_dsl import IR, Ev  # noqa: E402

ROOT = Path(__file__).resolve().parents[1]
TARGET = ROOT / "seed/prelude/Data/ChaCha8.cinch"
MARKER = "  -- Generated ChaCha8 kernels.\n"
SIGMA = [0x61707865, 0x3320646E, 0x79622D32, 0x6B206574]
V4 = "<4 x i32>"
M32 = 0xFFFFFFFF


def model(seed: list[int], counter: int) -> list[int]:
    """The thirty-two 64-bit outputs of one block call."""
    b = [[0] * 4 for _ in range(16)]
    for w in range(4):
        b[w] = [SIGMA[w]] * 4
    for k in range(4):
        b[4 + 2 * k] = [seed[k] & M32] * 4
        b[5 + 2 * k] = [seed[k] >> 32] * 4
    b[12] = [(counter + i) & M32 for i in range(4)]
    init = [list(r) for r in b]
    def rotl(v, n):
        return ((v << n) | (v >> (32 - n))) & M32
    for i in range(4):
        x = [b[w][i] for w in range(16)]
        def qr(a, bb, c, d):
            x[a] = (x[a] + x[bb]) & M32; x[d] = rotl(x[d] ^ x[a], 16)
            x[c] = (x[c] + x[d]) & M32; x[bb] = rotl(x[bb] ^ x[c], 12)
            x[a] = (x[a] + x[bb]) & M32; x[d] = rotl(x[d] ^ x[a], 8)
            x[c] = (x[c] + x[d]) & M32; x[bb] = rotl(x[bb] ^ x[c], 7)
        for _ in range(4):
            qr(0, 4, 8, 12); qr(1, 5, 9, 13); qr(2, 6, 10, 14); qr(3, 7, 11, 15)
            qr(0, 5, 10, 15); qr(1, 6, 11, 12); qr(2, 7, 8, 13); qr(3, 4, 9, 14)
        for w in range(16):
            b[w][i] = (x[w] + init[w][i]) & M32 if 4 <= w < 12 else x[w]
    return [b[j // 2][0 + 2 * (j % 2)] | (b[j // 2][1 + 2 * (j % 2)] << 32) for j in range(32)]


def word_kernel(target: int):
    def build(b, s01, s23, counter):
        kk = lambda v: b.c(V4, v)
        seed = b.arg128(s01, V4)
        seed2 = b.arg128(s23, V4)
        v = [kk(SIGMA[w]) for w in range(4)]
        for src in (seed, seed2):
            for lane in range(4):
                v.append(b.shuf(src, None, [lane] * 4))
        v.append(b.bin("add", b.splat_i64(counter), kk([0, 1, 2, 3])))
        v += [kk(0), kk(0), kk(0)]
        init = list(v)
        def rot(x, n):
            return b.bin("or", b.bin("shl", x, kk(n)), b.bin("lshr", x, kk(32 - n)))
        def qr(a, bb, c, d):
            v[a] = b.bin("add", v[a], v[bb]); v[d] = rot(b.bin("xor", v[d], v[a]), 16)
            v[c] = b.bin("add", v[c], v[d]); v[bb] = rot(b.bin("xor", v[bb], v[c]), 12)
            v[a] = b.bin("add", v[a], v[bb]); v[d] = rot(b.bin("xor", v[d], v[a]), 8)
            v[c] = b.bin("add", v[c], v[d]); v[bb] = rot(b.bin("xor", v[bb], v[c]), 7)
        for _ in range(4):
            qr(0, 4, 8, 12); qr(1, 5, 9, 13); qr(2, 6, 10, 14); qr(3, 7, 11, 15)
            qr(0, 5, 10, 15); qr(1, 6, 11, 12); qr(2, 7, 8, 13); qr(3, 4, 9, 14)
        out = b.bin("add", v[target], init[target]) if 4 <= target < 12 else v[target]
        b.ret128(out)
    return build


class ChaChaIR(IR):
    """The IR back end with a counter splat."""

    def splat_i64(self, arg):
        t = self.emit(f"trunc i64 {arg} to i32")
        i = self.emit(f"insertelement <4 x i32> poison, i32 {t}, i64 0")
        return (V4, self.emit(f"shufflevector <4 x i32> {i}, <4 x i32> poison, <4 x i32> zeroinitializer"))


class ChaChaEv(Ev):
    """The evaluator with a counter splat."""

    def splat_i64(self, arg):
        return (V4, [arg & M32] * 4)


def verify() -> None:
    """Every word kernel reproduces the scalar block."""
    rng = random.Random(8)
    for _ in range(120):
        seed = [rng.getrandbits(64) for _ in range(4)]
        counter = rng.choice([0, 4, 8, 12, rng.getrandbits(32)])
        want = model(seed, counter)
        for w in range(16):
            e = ChaChaEv()
            word_kernel(w)(e, seed[0] | (seed[1] << 64), seed[2] | (seed[3] << 64), counter)
            assert [e.result & ((1 << 64) - 1), e.result >> 64] == want[2 * w:2 * w + 2], ("word", w)


def suffix() -> str:
    verify()
    lines = [MARKER.rstrip("\n"), ""]
    for w in range(16):
        lines += [f"  -- | Final word {w} of the four interleaved blocks (outputs {2 * w} and {2 * w + 1}) for the seed halves and counter.",
                  f"  prim chacha8-word{w} :: U128 -> U128 -> I64 -> U128 where"]
        for big in (False, True):
            b = ChaChaIR(big)
            word_kernel(w)(b, "%0", "%1", "%2")
            lines += ["    llvm {"] + b.lines + ["    }" + (" when endian big" if big else " when endian little")]
        lines.append("")
    return "\n".join(lines).rstrip() + "\n"


def main() -> None:
    """Rewrite or check the generated suffix of Data.ChaCha8."""
    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    existing = TARGET.read_text()
    if existing.count(MARKER) != 1:
        raise SystemExit(f"{TARGET}: expected exactly one generated marker")
    expected = existing.split(MARKER, 1)[0].rstrip() + "\n\n" + suffix()
    if args.check:
        if existing != expected:
            raise SystemExit(f"{TARGET} is stale; rerun tooling/generate_chacha8_kernels.py")
        return
    TARGET.write_text(expected)


if __name__ == "__main__":
    main()
