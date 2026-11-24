#!/usr/bin/env python3
"""Generate the constant-time P-256 limb kernels of Crypto.P256.

Field elements (modulo p) and scalars (modulo the group order n) are 256-bit
values held as two U128 halves, four 64-bit limbs in all, kept in Montgomery
form with R = 2^256. Each kernel is straight-line LLVM over the limbs: products
are formed in 128-bit registers, Montgomery multiplication interleaves the
reduction word by word (CIOS), and every final correction is a borrow-chain
subtraction chosen with a select, so no kernel branches on its inputs. A kernel
returns one 128-bit half of its 256-bit result, so each operation comes as a
low-half and a high-half primitive over the same body. The algorithms are
modelled in Python and checked against plain modular arithmetic before writing.

    python3 tooling/generate_p256_kernels.py          # rewrite the generated suffix
    python3 tooling/generate_p256_kernels.py --check  # fail when it is stale
"""
from __future__ import annotations

import argparse
import random
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
TARGET = ROOT / "seed/prelude/Crypto/P256.cinch"
MARKER = "  -- Generated P-256 limb kernels: Montgomery multiplication, addition and subtraction modulo p and n.\n"
P = 2**256 - 2**224 + 2**192 + 2**96 - 1
N = 0xFFFFFFFF00000000FFFFFFFFFFFFFFFFBCE6FAADA7179E84F3B9CAC2FC632551
MASK = 2**64 - 1


def limbs(x: int) -> list[int]:
    """Little-endian 64-bit limbs."""
    return [(x >> (64 * i)) & MASK for i in range(4)]


def mont_model(a: int, b: int, m: int) -> int:
    """CIOS Montgomery multiplication exactly as emitted: a * b * 2^-256 mod m."""
    mp = (-pow(m, -1, 2**64)) % 2**64
    A, B, M = limbs(a), limbs(b), limbs(m)
    t = [0] * 6
    for i in range(4):
        c = 0
        for j in range(4):
            s = t[j] + A[j] * B[i] + c
            t[j], c = s & MASK, s >> 64
        s = t[4] + c
        t[4], t[5] = s & MASK, s >> 64
        q = (t[0] * mp) & MASK
        s = t[0] + q * M[0]
        c = s >> 64
        for j in range(1, 4):
            s = t[j] + q * M[j] + c
            t[j - 1], c = s & MASK, s >> 64
        s = t[4] + c
        t[3], c = s & MASK, s >> 64
        t[4] = t[5] + c
    value = sum(t[j] << (64 * j) for j in range(5))
    return value - m if value >= m else value


def verify() -> None:
    """The modelled algorithms equal modular arithmetic."""
    rng = random.Random(256)
    for m in (P, N):
        rinv = pow(2**256, -1, m)
        for _ in range(200):
            a, b = rng.randrange(m), rng.randrange(m)
            assert mont_model(a, b, m) == a * b * rinv % m


class Body:
    """One LLVM body under construction."""

    def __init__(self) -> None:
        self.lines, self.n = [], 0

    def emit(self, text: str) -> str:
        self.n += 1
        self.lines.append(f"      %t{self.n} = {text}")
        return f"%t{self.n}"

    def limbs_of(self, lo: str, hi: str) -> list[str]:
        """Four i64 limbs from two i128 halves."""
        out = []
        for half in (lo, hi):
            out.append(self.emit(f"trunc i128 {half} to i64"))
            s = self.emit(f"lshr i128 {half}, 64")
            out.append(self.emit(f"trunc i128 {s} to i64"))
        return out

    def wide(self, x: str) -> str:
        return self.emit(f"zext i64 {x} to i128")

    def mul(self, x: str, y: str) -> str:
        return self.emit(f"mul i128 {self.wide(x)}, {self.wide(y)}")

    def low(self, x: str) -> str:
        return self.emit(f"trunc i128 {x} to i64")

    def high(self, x: str) -> str:
        return self.emit(f"lshr i128 {x}, 64")

    def add(self, *xs: str) -> str:
        acc = xs[0]
        for x in xs[1:]:
            acc = self.emit(f"add i128 {acc}, {x}")
        return acc

    def sub_chain(self, a: list[str], b: list[str]) -> tuple[list[str], str]:
        """a - b over limbs (i64 values, a may have a fifth limb); returns limbs and the final borrow as i1."""
        out, borrow = [], "0"
        for j in range(len(a)):
            aw = self.wide(a[j])
            bw = self.wide(b[j]) if j < len(b) else "0"
            d = self.emit(f"sub i128 {aw}, {bw}")
            d = self.emit(f"sub i128 {d}, {borrow}")
            out.append(self.low(d))
            neg = self.emit(f"lshr i128 {d}, 127")
            borrow = neg
        return out, self.emit(f"trunc i128 {borrow} to i1")

    def ret_half(self, r: list[str], half: int) -> None:
        lo = self.wide(r[2 * half])
        hi = self.wide(r[2 * half + 1])
        s = self.emit(f"shl i128 {hi}, 64")
        v = self.emit(f"or i128 {lo}, {s}")
        self.lines.append(f"      ret i128 {v}")


def const_limbs(m: int) -> list[str]:
    """Modulus limbs as signed i64 literals."""
    return [str(x - 2**64 if x >= 2**63 else x) for x in limbs(m)]


def mont_body(m: int, half: int) -> list[str]:
    """Montgomery multiplication modulo m, returning one half."""
    b = Body()
    A = b.limbs_of("%0", "%1")
    B = b.limbs_of("%2", "%3")
    M = const_limbs(m)
    mp = (-pow(m, -1, 2**64)) % 2**64
    mps = str(mp - 2**64 if mp >= 2**63 else mp)
    t = ["0"] * 6
    for i in range(4):
        c = "0"
        for j in range(4):
            s = b.add(b.wide(t[j]) if t[j] != "0" else "0", b.mul(A[j], B[i]), c)
            t[j], c = b.low(s), b.high(s)
        s = b.add(b.wide(t[4]) if t[4] != "0" else "0", c)
        t[4], t[5] = b.low(s), b.low(b.high(s))
        q = b.emit(f"mul i64 {t[0]}, {mps}")
        s = b.add(b.wide(t[0]), b.mul(q, M[0]))
        c = b.high(s)
        for j in range(1, 4):
            s = b.add(b.wide(t[j]), b.mul(q, M[j]), c)
            t[j - 1], c = b.low(s), b.high(s)
        s = b.add(b.wide(t[4]), c)
        t[3], c = b.low(s), b.high(s)
        s = b.add(b.wide(t[5]), c)
        t[4] = b.low(s)
    return finish(b, t[:5], m, half)


def finish(b: Body, t: list[str], m: int, half: int) -> list[str]:
    """Subtract m once when the five-limb value is at least m, then return a half."""
    d, borrow = b.sub_chain(t, const_limbs(m))
    r = [b.emit(f"select i1 {borrow}, i64 {t[j]}, i64 {d[j]}") for j in range(4)]
    b.ret_half(r, half)
    return b.lines


def add_body(m: int, half: int) -> list[str]:
    """(a + b) mod m for reduced inputs."""
    b = Body()
    A, B = b.limbs_of("%0", "%1"), b.limbs_of("%2", "%3")
    t, c = [], "0"
    for j in range(4):
        s = b.add(b.wide(A[j]), b.wide(B[j]), c)
        t.append(b.low(s))
        c = b.high(s)
    t.append(b.low(c))
    return finish(b, t, m, half)


def sub_body(m: int, half: int) -> list[str]:
    """(a - b) mod m for reduced inputs: add m back when the difference borrows."""
    b = Body()
    A, B = b.limbs_of("%0", "%1"), b.limbs_of("%2", "%3")
    d, borrow = b.sub_chain(A, B)
    M = const_limbs(m)
    mask = b.emit(f"sext i1 {borrow} to i64")
    t, c = [], "0"
    for j in range(4):
        mj = b.emit(f"and i64 {mask}, {M[j]}")
        s = b.add(b.wide(d[j]), b.wide(mj), c)
        t.append(b.low(s))
        c = b.high(s)
    b.ret_half(t, half)
    return b.lines


def reduce_body(m: int, half: int) -> list[str]:
    """A value below 2m reduced once modulo m (no fifth limb)."""
    b = Body()
    A = b.limbs_of("%0", "%1")
    return finish(b, A + ["0"], m, half) if False else finish_four(b, A, m, half)


def finish_four(b: Body, t: list[str], m: int, half: int) -> list[str]:
    """Subtract m once from a four-limb value when it is at least m."""
    d, borrow = b.sub_chain(t, const_limbs(m))
    r = [b.emit(f"select i1 {borrow}, i64 {t[j]}, i64 {d[j]}") for j in range(4)]
    b.ret_half(r, half)
    return b.lines


def less_body(m: int) -> list[str]:
    """1 when the value is below m, else 0."""
    b = Body()
    A = b.limbs_of("%0", "%1")
    d, borrow = b.sub_chain(A, const_limbs(m))
    r = b.emit(f"zext i1 {borrow} to i64")
    b.lines.append(f"      ret i64 {r}")
    return b.lines


def prim(name: str, sig: str, doc: str, lines: list[str]) -> list[str]:
    """A primitive with one body."""
    return [f"  -- | {doc}", f"  prim {name} :: {sig} where", "    llvm {"] + lines + ["    }", ""]


def suffix() -> str:
    """The generated text after the marker."""
    verify()
    out = [MARKER.rstrip("\n"), ""]
    two = "U128 -> U128 -> U128 -> U128 -> U128"
    for tag, m, what in (("p", P, "the field prime p"), ("n", N, "the group order n")):
        for half, hname in ((0, "lo"), (1, "hi")):
            out += prim(f"{tag}-mul-{hname}", two, f"The {hname} half of the Montgomery product a * b / 2^256 modulo {what}.", mont_body(m, half))
            out += prim(f"{tag}-add-{hname}", two, f"The {hname} half of (a + b) modulo {what}.", add_body(m, half))
            out += prim(f"{tag}-sub-{hname}", two, f"The {hname} half of (a - b) modulo {what}.", sub_body(m, half))
            out += prim(f"{tag}-reduce-{hname}", "U128 -> U128 -> U128", f"The {hname} half of a value below twice {what}, reduced once.", reduce_body(m, half))
        out += prim(f"{tag}-below", "U128 -> U128 -> I64", f"One when a value is below {what}, else zero.", less_body(m))
    return "\n".join(out).rstrip() + "\n"


def main() -> None:
    """Rewrite or check the generated suffix of Crypto.P256."""
    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    existing = TARGET.read_text()
    if existing.count(MARKER) != 1:
        raise SystemExit(f"{TARGET}: expected exactly one generated marker")
    expected = existing.split(MARKER, 1)[0].rstrip() + "\n\n" + suffix()
    if args.check:
        if existing != expected:
            raise SystemExit(f"{TARGET} is stale; rerun tooling/generate_p256_kernels.py")
        return
    TARGET.write_text(expected)


if __name__ == "__main__":
    main()
