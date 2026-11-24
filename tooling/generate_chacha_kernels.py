#!/usr/bin/env python3
"""Generate the SIMD kernels of Crypto.ChaCha20Poly1305.

ChaCha20 keeps its state as four rows of four 32-bit words, each row one <4 x i32> vector: a column round is a
quarter round on the four vectors, and a diagonal round rotates rows 1-3 by one, two and three lanes, applies
the same quarter round and rotates them back. A U128 holds one row with word i in bits 32i..32i+31 (the
little-endian reading of the row's bytes). chacha-row0..3 return one row of the keystream block (the state
after twenty rounds plus the initial state); each call recomputes the rounds, since a kernel returns at most
128 bits. hchacha-row0 and hchacha-row3 return the rows HChaCha20 keeps (no final addition).

Poly1305 accumulates in a 131-bit value held as a U128 and a U64: poly-lo and poly-hi give the two parts of
(h + m + hibit * 2^128) * r reduced once modulo 2^130 - 5 (exact 256-bit products, so the result stays below
2^131), and poly-tag fully reduces h and adds the second key half. The vector programs are evaluated against
a scalar ChaCha20 before writing; Poly1305 is checked end to end against independent implementations.

    python3 tooling/generate_chacha_kernels.py          # rewrite the generated suffix
    python3 tooling/generate_chacha_kernels.py --check  # fail when it is stale
"""
from __future__ import annotations

import argparse
import random
import struct
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from lane_dsl import IR, Ev  # noqa: E402

ROOT = Path(__file__).resolve().parents[1]
TARGET = ROOT / "seed/prelude/Crypto/ChaCha20Poly1305.cinch"
MARKER = "  -- Generated ChaCha20 and Poly1305 kernels.\n"
SIGMA = [0x61707865, 0x3320646E, 0x79622D32, 0x6B206574]
V4 = "<4 x i32>"


def chacha_model(key: bytes, block: bytes, rounds: int = 20, add: bool = True) -> list[int]:
    """The ChaCha state words after the rounds (plus the initial state when add is set)."""
    st = SIGMA + list(struct.unpack("<8I", key)) + list(struct.unpack("<4I", block))
    x = list(st)
    def rotl(v, n):
        return ((v << n) | (v >> (32 - n))) & 0xFFFFFFFF
    def qr(a, b, c, d):
        x[a] = (x[a] + x[b]) & 0xFFFFFFFF; x[d] = rotl(x[d] ^ x[a], 16)
        x[c] = (x[c] + x[d]) & 0xFFFFFFFF; x[b] = rotl(x[b] ^ x[c], 12)
        x[a] = (x[a] + x[b]) & 0xFFFFFFFF; x[d] = rotl(x[d] ^ x[a], 8)
        x[c] = (x[c] + x[d]) & 0xFFFFFFFF; x[b] = rotl(x[b] ^ x[c], 7)
    for _ in range(rounds // 2):
        qr(0, 4, 8, 12); qr(1, 5, 9, 13); qr(2, 6, 10, 14); qr(3, 7, 11, 15)
        qr(0, 5, 10, 15); qr(1, 6, 11, 12); qr(2, 7, 8, 13); qr(3, 4, 9, 14)
    return [(a + b) & 0xFFFFFFFF for a, b in zip(x, st)] if add else x


def rounds(b, add: bool):
    """The twenty-round ChaCha permutation on the four rows; returns the final rows."""
    def kk(v):
        return b.c(V4, v)
    def rot(v, n):
        return b.bin("or", b.bin("shl", v, kk(n)), b.bin("lshr", v, kk(32 - n)))
    def qr(a, bb, c, d):
        a = b.bin("add", a, bb); d = rot(b.bin("xor", d, a), 16)
        c = b.bin("add", c, d); bb = rot(b.bin("xor", bb, c), 12)
        a = b.bin("add", a, bb); d = rot(b.bin("xor", d, a), 8)
        c = b.bin("add", c, d); bb = rot(b.bin("xor", bb, c), 7)
        return a, bb, c, d
    def lanes(v, n):
        return b.shuf(v, None, [(i + n) % 4 for i in range(4)])
    def build(key0, key1, block):
        r0 = kk(SIGMA)
        r1 = b.arg128(key0, V4)
        r2 = b.arg128(key1, V4)
        r3 = b.arg128(block, V4)
        a, bb, c, d = r0, r1, r2, r3
        for _ in range(10):
            a, bb, c, d = qr(a, bb, c, d)
            bb, c, d = lanes(bb, 1), lanes(c, 2), lanes(d, 3)
            a, bb, c, d = qr(a, bb, c, d)
            bb, c, d = lanes(bb, 3), lanes(c, 2), lanes(d, 1)
        if add:
            a, bb, c, d = b.bin("add", a, r0), b.bin("add", bb, r1), b.bin("add", c, r2), b.bin("add", d, r3)
        return [a, bb, c, d]
    return build


def row_kernel(q: int, add: bool):
    def build(b, key0, key1, block):
        b.ret128(rounds(b, add)(key0, key1, block)[q])
    return build


def u128(bs: bytes) -> int:
    return int.from_bytes(bs, "little")


def verify() -> None:
    """Every generated row program reproduces the scalar ChaCha20 and HChaCha20 state."""
    rng = random.Random(8439)
    for _ in range(300):
        key = bytes(rng.randrange(256) for _ in range(32))
        block = bytes(rng.randrange(256) for _ in range(16))
        for add in (True, False):
            want = chacha_model(key, block, 20, add)
            for q in range(4):
                e = Ev()
                row_kernel(q, add)(e, u128(key[:16]), u128(key[16:]), u128(block))
                got = [(e.result >> (32 * i)) & 0xFFFFFFFF for i in range(4)]
                assert got == want[4 * q:4 * q + 4], ("chacha row", q, add)


def poly_body(part: str):
    """(h + m + hibit * 2^128) * r reduced once modulo 2^130 - 5, low 128 bits (lo) or the rest (hi)."""
    def body(big: bool) -> list[str]:
        lines = [
            "      %hl = zext i128 %0 to i256",
            "      %hh0 = zext i64 %1 to i256",
            "      %hh = shl i256 %hh0, 128",
            "      %h = or i256 %hl, %hh",
            "      %m = zext i128 %3 to i256",
            "      %hb0 = zext i64 %4 to i256",
            "      %hb = shl i256 %hb0, 128",
            "      %s0 = add i256 %h, %m",
            "      %s = add i256 %s0, %hb",
            "      %r = zext i128 %2 to i256",
            "      %p = mul i256 %s, %r",
            "      %lo130 = and i256 %p, 1361129467683753853853498429727072845823",
            "      %top = lshr i256 %p, 130",
            "      %five = mul i256 %top, 5",
            "      %f = add i256 %lo130, %five",
        ]
        if part == "lo":
            lines += ["      %t = trunc i256 %f to i128", "      ret i128 %t"]
        else:
            lines += ["      %u = lshr i256 %f, 128", "      %t = trunc i256 %u to i64", "      ret i64 %t"]
        return lines
    return body


def tag_body(big: bool) -> list[str]:
    """Fully reduce h below 2^130 - 5 and add the second key half modulo 2^128."""
    return [
        "      %hl = zext i128 %0 to i256",
        "      %hh0 = zext i64 %1 to i256",
        "      %hh = shl i256 %hh0, 128",
        "      %h0 = or i256 %hl, %hh",
        "      %lo130 = and i256 %h0, 1361129467683753853853498429727072845823",
        "      %top = lshr i256 %h0, 130",
        "      %five = mul i256 %top, 5",
        "      %h = add i256 %lo130, %five",
        "      %g = add i256 %h, 5",
        "      %ge = lshr i256 %g, 130",
        "      %big = icmp ne i256 %ge, 0",
        "      %gm = and i256 %g, 1361129467683753853853498429727072845823",
        "      %red = select i1 %big, i256 %gm, i256 %h",
        "      %t0 = trunc i256 %red to i128",
        "      %t = add i128 %t0, %2",
        "      ret i128 %t",
    ]


def prim(name: str, sig: str, doc: str, build, args: list[str]) -> list[str]:
    out = [f"  -- | {doc}", f"  prim {name} :: {sig} where"]
    for big in (False, True):
        b = IR(big)
        build(b, *args)
        out += ["    llvm {"] + b.lines + ["    }" + (" when endian big" if big else " when endian little")]
    return out


def scalar_prim(name: str, sig: str, doc: str, body) -> list[str]:
    return [f"  -- | {doc}", f"  prim {name} :: {sig} where", "    llvm {"] + body(False) + ["    }"]


def suffix() -> str:
    verify()
    lines = [MARKER.rstrip("\n"), ""]
    for q in range(4):
        lines += prim(f"chacha-row{q}", "U128 -> U128 -> U128 -> U128",
                      f"Row {q} of the ChaCha20 keystream block for the key halves and the counter-and-nonce row: the state after twenty rounds plus the initial state, word i in bits 32i up.",
                      row_kernel(q, True), ["%0", "%1", "%2"]) + [""]
    for q in (0, 3):
        lines += prim(f"hchacha-row{q}", "U128 -> U128 -> U128 -> U128",
                      f"Row {q} of the ChaCha20 state after twenty rounds without the final addition (HChaCha20 keeps rows 0 and 3).",
                      row_kernel(q, False), ["%0", "%1", "%2"]) + [""]
    lines += scalar_prim("poly-lo", "U128 -> U64 -> U128 -> U128 -> U64 -> U128",
                         "The low 128 bits of (h + m + hibit * 2^128) * r reduced once modulo 2^130 - 5, for h given as its low 128 bits and the rest, the clamped r, the block m and hibit.",
                         poly_body("lo")) + [""]
    lines += scalar_prim("poly-hi", "U128 -> U64 -> U128 -> U128 -> U64 -> U64",
                         "The bits above 128 of the same once-reduced product (below 8).",
                         poly_body("hi")) + [""]
    lines += scalar_prim("poly-tag", "U128 -> U64 -> U128 -> U128",
                         "The tag: h reduced fully modulo 2^130 - 5, plus the second key half, modulo 2^128.", tag_body)
    return "\n".join(lines).rstrip() + "\n"


def main() -> None:
    """Rewrite or check the generated suffix of Crypto.ChaCha20Poly1305."""
    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    existing = TARGET.read_text()
    if existing.count(MARKER) != 1:
        raise SystemExit(f"{TARGET}: expected exactly one generated marker")
    expected = existing.split(MARKER, 1)[0].rstrip() + "\n\n" + suffix()
    if args.check:
        if existing != expected:
            raise SystemExit(f"{TARGET} is stale; rerun tooling/generate_chacha_kernels.py")
        return
    TARGET.write_text(expected)


if __name__ == "__main__":
    main()
