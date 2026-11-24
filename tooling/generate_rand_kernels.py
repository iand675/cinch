#!/usr/bin/env python3
"""Generate the ziggurat tables of Data.Rand.

The normal (128 strips) and exponential (256 strips) ziggurats of Marsaglia and Tsang are built from their
defining recurrences: strip edges from the base edge and the common strip area, k[i] as the fraction of each
strip that lies wholly under the density (scaled to 31 or 32 bits), w[i] as the strip width per unit of the
random integer and f[i] as the density at the strip's edge, the last two rounded to binary32. The recurrences
use the bit-exact binary64 exp and log models of tooling/generate_float_kernels.py, so the tables do not depend
on the host's libm. Each table becomes a prim that reads one element of a constant vector.

    python3 tooling/generate_rand_kernels.py          # rewrite the generated suffix
    python3 tooling/generate_rand_kernels.py --check  # fail when it is stale
"""
from __future__ import annotations

import argparse
import math
import struct
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import generate_float_kernels as fk  # noqa: E402

ROOT = Path(__file__).resolve().parents[1]
TARGET = ROOT / "seed/prelude/Data/Rand.cinch"
MARKER = "  -- Generated ziggurat tables.\n"
MODELS = fk.PROGRAM.python()
EXP, LOG = MODELS["exp"], MODELS["log"]


def f32(x: float) -> float:
    """Round to binary32 (nearest, ties to even)."""
    return struct.unpack("<f", struct.pack("<f", x))[0]


def normal_tables() -> tuple[list[int], list[float], list[float]]:
    """The 128-strip normal ziggurat: base edge 3.442619855899, strip area 9.91256303526217e-3."""
    dn, vn, m1 = 3.442619855899, 9.91256303526217e-3, 2147483648.0
    tn = dn
    k, w, f = [0] * 128, [0.0] * 128, [0.0] * 128
    q = vn / EXP(-0.5 * dn * dn)
    k[0] = int((dn / q) * m1)
    w[0], w[127] = q / m1, dn / m1
    f[0], f[127] = 1.0, EXP(-0.5 * dn * dn)
    for i in range(126, 0, -1):
        dn = math.sqrt(-2 * LOG(vn / dn + EXP(-0.5 * dn * dn)))
        k[i + 1] = int((dn / tn) * m1)
        tn = dn
        f[i] = EXP(-0.5 * dn * dn)
        w[i] = dn / m1
    return k, [f32(v) for v in w], [f32(v) for v in f]


def exp_tables() -> tuple[list[int], list[float], list[float]]:
    """The 256-strip exponential ziggurat: base edge 7.697117470131487, strip area 3.949659822581572e-3."""
    de, ve, m2 = 7.697117470131487, 3.949659822581572e-3, 4294967296.0
    te = de
    k, w, f = [0] * 256, [0.0] * 256, [0.0] * 256
    q = ve / EXP(-de)
    k[0] = int((de / q) * m2)
    w[0], w[255] = q / m2, de / m2
    f[0], f[255] = 1.0, EXP(-de)
    for i in range(254, 0, -1):
        de = -LOG(ve / de + EXP(-de))
        k[i + 1] = int((de / te) * m2)
        te = de
        f[i] = EXP(-de)
        w[i] = de / m2
    return k, [f32(v) for v in w], [f32(v) for v in f]


def hexf(x: float) -> str:
    return "0x%016X" % struct.unpack("<Q", struct.pack("<d", x))[0]


def table(name: str, doc: str, values: list, kind: str) -> str:
    """A prim returning element i of a constant vector."""
    n = len(values)
    if kind == "u32":
        vec = ", ".join(f"i32 {v}" for v in values)
        ty, cin = "i32", "U32"
    else:
        vec = ", ".join(f"float {hexf(v)}" for v in values)
        ty, cin = "float", "F32"
    return (f"\n  -- | {doc}\n  prim {name} :: U64 -> {cin} where\n    llvm {{\n"
            f"      %r = extractelement <{n} x {ty}> <{vec}>, i64 %0\n      ret {ty} %r\n    }}\n")


def render() -> str:
    nk, nw, nf = normal_tables()
    ek, ew, ef = exp_tables()
    return MARKER + "".join([
        table("normal-k", "Normal strip i: the 31-bit threshold below which a point lies wholly under the density.", nk, "u32"),
        table("normal-w", "Normal strip i: the width per unit of the signed 32-bit position.", nw, "f32"),
        table("normal-f", "Normal strip i: the density at the strip's outer edge.", nf, "f32"),
        table("exp-k", "Exponential strip i: the 32-bit threshold below which a point lies wholly under the density.", ek, "u32"),
        table("exp-w", "Exponential strip i: the width per unit of the unsigned 32-bit position.", ew, "f32"),
        table("exp-f", "Exponential strip i: the density at the strip's outer edge.", ef, "f32"),
    ])


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    text = TARGET.read_text()
    new = text.split(MARKER)[0] + render()
    if args.check:
        if new != text:
            sys.exit(f"{TARGET} is stale; run {Path(__file__).name}")
        return
    TARGET.write_text(new)


if __name__ == "__main__":
    main()
