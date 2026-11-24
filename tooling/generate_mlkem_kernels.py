#!/usr/bin/env python3
"""Generate the ML-KEM polynomial kernels of Crypto.MlKem.

A polynomial's 256 coefficients live in thirty-two U128 lanes of eight 16-bit
coefficients each; lane i of a U128 is bits 16i..16i+15 (the little-endian
reading of the coefficients in memory order). Vector bodies view a U128 as
`<8 x i16>`, reversing lanes after the bitcast on big-endian targets so lane
numbering is the same everywhere.

Arithmetic is exact modulo q = 3329 in 32-bit lanes: products of two reduced
coefficients stay below 2^24, sums below 2q are reduced by one conditional
subtraction, and other reductions are unsigned remainders by the constant q,
which compilers lower to multiplications and shifts, so every kernel is free
of secret-dependent branches and divisions. The forward and inverse
transforms (FIPS 203 Algorithms 9 and 10) are emitted as straight-line Cinch
over the thirty-two lanes: layers whose butterflies span whole lanes combine
two lanes with one twiddle factor, and the last two layers rearrange
coefficients inside a lane. The generator checks its lane schedule against
the reference transforms in tooling/mlkem_reference.py before writing.

    python3 tooling/generate_mlkem_kernels.py          # rewrite the generated suffix
    python3 tooling/generate_mlkem_kernels.py --check  # fail when it is stale
"""
from __future__ import annotations

import argparse
import random
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import mlkem_reference as ref  # noqa: E402

ROOT = Path(__file__).resolve().parents[1]
TARGET = ROOT / "seed/prelude/Crypto/MlKem.cinch"
MARKER = "  -- Generated ML-KEM kernels: lane arithmetic, transforms and packing.\n"
Q = 3329
ZETAS, GAMMAS = ref.ZETAS, ref.GAMMAS


def splat32(v: int) -> str:
    """An eight-lane i32 constant."""
    return "<" + ", ".join(f"i32 {v}" for _ in range(8)) + ">"


def mask(lanes: list[int], width: int = 32) -> str:
    """A shuffle mask."""
    return f"<{len(lanes)} x i32> <" + ", ".join(f"i32 {i}" for i in lanes) + ">"


class Body:
    """An LLVM body under construction for one endianness."""

    def __init__(self, big: bool) -> None:
        self.big, self.lines, self.n = big, [], 0

    def tmp(self) -> str:
        self.n += 1
        return f"%v{self.n}"

    def emit(self, text: str) -> str:
        name = self.tmp()
        self.lines.append(f"      {name} = {text}")
        return name

    def vec32(self, arg: str) -> str:
        """A U128 argument as eight zero-extended 32-bit lanes in canonical order."""
        v = self.emit(f"bitcast i128 {arg} to <8 x i16>")
        if self.big:
            v = self.emit(f"shufflevector <8 x i16> {v}, <8 x i16> poison, {mask(list(range(7, -1, -1)))}")
        return self.emit(f"zext <8 x i16> {v} to <8 x i32>")

    def ret32(self, v: str) -> None:
        """Return eight 32-bit lanes (each below 2^16) as a U128."""
        t = self.emit(f"trunc <8 x i32> {v} to <8 x i16>")
        if self.big:
            t = self.emit(f"shufflevector <8 x i16> {t}, <8 x i16> poison, {mask(list(range(7, -1, -1)))}")
        r = self.emit(f"bitcast <8 x i16> {t} to i128")
        self.lines.append(f"      ret i128 {r}")

    def scalar(self, arg: str) -> str:
        """An I64 argument splatted across eight 32-bit lanes."""
        t = self.emit(f"trunc i64 {arg} to i32")
        i = self.emit(f"insertelement <8 x i32> poison, i32 {t}, i64 0")
        return self.emit(f"shufflevector <8 x i32> {i}, <8 x i32> poison, <8 x i32> zeroinitializer")

    def lanes(self, args: list[str]) -> str:
        """Eight I64 arguments as the lanes of one vector, in order."""
        v = "poison"
        for i, a in enumerate(args):
            t = self.emit(f"trunc i64 {a} to i32")
            v = self.emit(f"insertelement <8 x i32> {v}, i32 {t}, i64 {i}")
        return v

    def mulmod(self, a: str, b: str) -> str:
        """a * b mod q for reduced lanes."""
        p = self.emit(f"mul <8 x i32> {a}, {b}")
        return self.emit(f"urem <8 x i32> {p}, {splat32(Q)}")

    def reduce2q(self, v: str) -> str:
        """One conditional subtraction of q, for lanes below 2q."""
        ge = self.emit(f"icmp uge <8 x i32> {v}, {splat32(Q)}")
        s = self.emit(f"sub <8 x i32> {v}, {splat32(Q)}")
        return self.emit(f"select <8 x i1> {ge}, <8 x i32> {s}, <8 x i32> {v}")

    def add(self, a: str, b: str) -> str:
        return self.reduce2q(self.emit(f"add <8 x i32> {a}, {b}"))

    def sub(self, a: str, b: str) -> str:
        t = self.emit(f"add <8 x i32> {a}, {splat32(Q)}")
        return self.reduce2q(self.emit(f"sub <8 x i32> {t}, {b}"))

    def shuffle(self, a: str, b: str, lanes: list[int]) -> str:
        return self.emit(f"shufflevector <8 x i32> {a}, <8 x i32> {b}, {mask(lanes)}")


def prim(name: str, sig: str, doc: str, build, vector: bool = True) -> list[str]:
    """A primitive with little- and big-endian bodies (or one body for scalar arithmetic)."""
    out = [f"  -- | {doc}", f"  prim {name} :: {sig} where"]
    for big in ([False, True] if vector else [False]):
        b = Body(big)
        build(b)
        out += ["    llvm {"] + b.lines + ["    }" + ((" when endian big" if big else " when endian little") if vector else "")]
    return out


def bf_sum(b: Body) -> None:
    b.ret32(b.add(b.vec32("%0"), b.mulmod(b.vec32("%1"), b.scalar("%2"))))


def bf_diff(b: Body) -> None:
    b.ret32(b.sub(b.vec32("%0"), b.mulmod(b.vec32("%1"), b.scalar("%2"))))


def ib_sum(b: Body) -> None:
    b.ret32(b.add(b.vec32("%0"), b.vec32("%1")))


def ib_diff(b: Body) -> None:
    b.ret32(b.mulmod(b.sub(b.vec32("%1"), b.vec32("%0")), b.scalar("%2")))


def sub_q(b: Body) -> None:
    b.ret32(b.sub(b.vec32("%0"), b.vec32("%1")))


def scale(b: Body) -> None:
    b.ret32(b.mulmod(b.vec32("%0"), b.scalar("%1")))


def intra(b: Body, forward: bool, half: int) -> None:
    """The layers inside a lane: blocks of 2 * half coefficients, one twiddle per block."""
    x = b.vec32("%0")
    if half == 4:
        lo, hi, z = [0, 1, 2, 3, 0, 1, 2, 3], [4, 5, 6, 7, 4, 5, 6, 7], b.scalar("%1")
        pick = [0, 1, 2, 3, 12, 13, 14, 15]
    else:
        lo, hi = [0, 1, 0, 1, 4, 5, 4, 5], [2, 3, 2, 3, 6, 7, 6, 7]
        z = b.lanes(["%1"] * 4 + ["%2"] * 4)
        pick = [0, 1, 10, 11, 4, 5, 14, 15]
    a, c = b.shuffle(x, x, lo), b.shuffle(x, x, hi)
    if forward:
        t = b.mulmod(c, z)
        s, d = b.add(a, t), b.sub(a, t)
    else:
        s, d = b.add(a, c), b.mulmod(b.sub(c, a), z)
    b.ret32(b.shuffle(s, d, pick))


def basemul_add(b: Body) -> None:
    """acc + a * b in the NTT domain for four degree-one products with twiddles gamma0..gamma3."""
    acc, x, y = b.vec32("%0"), b.vec32("%1"), b.vec32("%2")
    swapped = [1, 0, 3, 2, 5, 4, 7, 6]
    p = b.emit(f"mul <8 x i32> {x}, {y}")
    qv = b.emit(f"mul <8 x i32> {x}, {b.shuffle(y, y, swapped)}")
    odd = b.emit(f"urem <8 x i32> {b.shuffle(p, p, [1, 1, 3, 3, 5, 5, 7, 7])}, {splat32(Q)}")
    gam = b.lanes(["%3", "%3", "%4", "%4", "%5", "%5", "%6", "%6"])
    even = b.emit(f"add <8 x i32> {p}, {b.emit(f'mul <8 x i32> {odd}, {gam}')}")
    cross = b.emit(f"add <8 x i32> {qv}, {b.shuffle(qv, qv, swapped)}")
    pickmask = "<" + ", ".join("i1 true" if i % 2 == 0 else "i1 false" for i in range(8)) + ">"
    h = b.emit(f"select <8 x i1> {pickmask}, <8 x i32> {even}, <8 x i32> {cross}")
    b.ret32(b.add(acc, b.emit(f"urem <8 x i32> {h}, {splat32(Q)}")))


def below_q(b: Body) -> None:
    x = b.vec32("%0")
    lt = b.emit(f"icmp ult <8 x i32> {x}, {splat32(Q)}")
    a = b.emit(f"call i1 @llvm.vector.reduce.and.v8i1(<8 x i1> {lt})")
    r = b.emit(f"zext i1 {a} to i64")
    b.lines.append(f"      ret i64 {r}")


def lane_fields(b: Body, arg: str, width: int, place: int) -> list[str]:
    """Extract eight fields of width bits at stride place bits from an i128."""
    out = []
    for i in range(8):
        s = b.emit(f"lshr i128 {arg}, {place * i}") if i else arg
        out.append(b.emit(f"and i128 {s}, {(1 << width) - 1}"))
    return out


def assemble(b: Body, fields: list[str], place: int) -> None:
    """OR eight i128 fields at stride place bits and return."""
    acc = fields[0]
    for i in range(1, 8):
        acc = b.emit(f"or i128 {acc}, {b.emit(f'shl i128 {fields[i]}, {place * i}')}")
    b.lines.append(f"      ret i128 {acc}")


def pack12(b: Body) -> None:
    assemble(b, lane_fields(b, "%0", 12, 16), 12)


def unpack12(b: Body) -> None:
    assemble(b, lane_fields(b, "%0", 12, 12), 16)


def reduce_q(b: Body) -> None:
    b.ret32(b.reduce2q(b.vec32("%0")))


def compress(d: int):
    """Compress_d of eight lanes, packed at d bits per coefficient."""
    def build(b: Body) -> None:
        x = b.vec32("%0")
        s = b.emit(f"shl <8 x i32> {x}, {splat32(d)}")
        r = b.emit(f"add <8 x i32> {s}, {splat32(Q // 2)}")
        u = b.emit(f"udiv <8 x i32> {r}, {splat32(Q)}")
        c = b.emit(f"and <8 x i32> {u}, {splat32((1 << d) - 1)}")
        fields = []
        for i in range(8):
            e = b.emit(f"extractelement <8 x i32> {c}, i64 {i}")
            fields.append(b.emit(f"zext i32 {e} to i128"))
        assemble(b, fields, d)
    return build


def decompress(d: int):
    """Decompress_d of eight packed d-bit fields into lanes."""
    def build(b: Body) -> None:
        out = []
        for f in lane_fields(b, "%0", d, d):
            m = b.emit(f"mul i128 {f}, {Q}")
            a = b.emit(f"add i128 {m}, {1 << (d - 1)}")
            out.append(b.emit(f"lshr i128 {a}, {d}"))
        assemble(b, out, 16)
    return build


def message(b: Body) -> None:
    """Decompress_1 of eight message bits: 0 or 1665."""
    w = b.emit("zext i64 %0 to i128")
    out = []
    for i in range(8):
        s = b.emit(f"lshr i128 {w}, {i}") if i else w
        bit = b.emit(f"and i128 {s}, 1")
        out.append(b.emit(f"mul i128 {bit}, 1665"))
    assemble(b, out, 16)


def cbd2(b: Body) -> None:
    """Centered binomial coefficients (eta = 2) from 32 bits: pair sums, then a difference modulo q."""
    w = b.emit("zext i64 %0 to i128")
    even = b.emit(f"and i128 {w}, {0x55555555}")
    sh = b.emit(f"lshr i128 {w}, 1")
    odd = b.emit(f"and i128 {sh}, {0x55555555}")
    t = b.emit(f"add i128 {even}, {odd}")
    out = []
    for i in range(8):
        xs = b.emit(f"lshr i128 {t}, {4 * i}") if i else t
        x = b.emit(f"and i128 {xs}, 3")
        ys = b.emit(f"lshr i128 {t}, {4 * i + 2}")
        y = b.emit(f"and i128 {ys}, 3")
        xq = b.emit(f"add i128 {x}, {Q}")
        diff = b.emit(f"sub i128 {xq}, {y}")
        ge = b.emit(f"icmp uge i128 {diff}, {Q}")
        red = b.emit(f"sub i128 {diff}, {Q}")
        out.append(b.emit(f"select i1 {ge}, i128 {red}, i128 {diff}"))
    assemble(b, out, 16)


def join64(b: Body) -> None:
    lo = b.emit("zext i64 %0 to i128")
    hi = b.emit("zext i64 %1 to i128")
    s = b.emit(f"shl i128 {hi}, 64")
    r = b.emit(f"or i128 {lo}, {s}")
    b.lines.append(f"      ret i128 {r}")


def low64(b: Body) -> None:
    r = b.emit("trunc i128 %0 to i64")
    b.lines.append(f"      ret i64 {r}")


def high64(b: Body) -> None:
    s = b.emit("lshr i128 %0, 64")
    r = b.emit(f"trunc i128 {s} to i64")
    b.lines.append(f"      ret i64 {r}")


def forward_schedule() -> list[tuple]:
    """The forward transform as lane operations: ('pair', i, j, zeta), ('four', i, zeta), ('two', i, z1, z2)."""
    ops, k = [], 1
    for span in (16, 8, 4, 2, 1):
        for start in range(0, 32, 2 * span):
            ops += [("pair", start + j, start + j + span, ZETAS[k]) for j in range(span)]
            k += 1
    ops += [("four", v, ZETAS[32 + v]) for v in range(32)]
    ops += [("two", v, ZETAS[64 + 2 * v], ZETAS[65 + 2 * v]) for v in range(32)]
    return ops


def inverse_schedule() -> list[tuple]:
    """The inverse transform as lane operations, then the final scaling by 128^-1."""
    ops = [("two", v, ZETAS[127 - 2 * v], ZETAS[126 - 2 * v]) for v in range(32)]
    ops += [("four", v, ZETAS[63 - v]) for v in range(32)]
    k = 31
    for span in (1, 2, 4, 8, 16):
        for start in range(0, 32, 2 * span):
            ops += [("pair", start + j, start + j + span, ZETAS[k]) for j in range(span)]
            k -= 1
    return ops


def model(ops: list[tuple], f: list[int], forward: bool) -> list[int]:
    """Run a lane schedule on plain coefficients, as the kernels compute it."""
    f = list(f)
    for op in ops:
        if op[0] == "pair":
            _, i, j, z = op
            for t in range(8):
                a, c = f[8 * i + t], f[8 * j + t]
                if forward:
                    f[8 * i + t], f[8 * j + t] = (a + z * c) % Q, (a - z * c) % Q
                else:
                    f[8 * i + t], f[8 * j + t] = (a + c) % Q, z * (c - a) % Q
        else:
            v, half = op[1], 4 if op[0] == "four" else 2
            zs = [op[2]] if op[0] == "four" else [op[2], op[3]]
            for blk, z in enumerate(zs):
                base = 8 * v + 2 * half * blk
                for t in range(half):
                    a, c = f[base + t], f[base + half + t]
                    if forward:
                        f[base + t], f[base + half + t] = (a + z * c) % Q, (a - z * c) % Q
                    else:
                        f[base + t], f[base + half + t] = (a + c) % Q, z * (c - a) % Q
    return f if forward else [x * 3303 % Q for x in f]


def verify() -> None:
    """The lane schedules reproduce the reference transforms."""
    rng = random.Random(9)
    for _ in range(20):
        f = [rng.randrange(Q) for _ in range(256)]
        assert model(forward_schedule(), f, True) == ref.ntt(f)
        assert model(inverse_schedule(), f, False) == ref.ntt_inverse(f)


def layers(ops: list[tuple]) -> list[list[tuple]]:
    """Split a schedule into layers whose operations touch disjoint lanes."""
    out, current, used = [], [], set()
    for op in ops:
        touched = {op[1], op[2]} if op[0] == "pair" else {op[1]}
        if touched & used:
            out.append(current)
            current, used = [], set()
        current.append(op)
        used |= touched
    return out + [current]


def transform(name: str, doc: str, ops: list[tuple], forward: bool) -> list[str]:
    """A transform as one function per layer: each layer rebuilds the thirty-two lanes in one constructor."""
    groups = layers(ops)
    if not forward:
        groups.append([("scale", i) for i in range(32)])
    lines: list[str] = []
    names = []
    for n, group in enumerate(groups):
        layer = f"{name}-layer{n}"
        names.append(layer)
        expr = [f"x{i}" for i in range(32)]
        for op in group:
            if op[0] == "pair":
                _, i, j, z = op
                if forward:
                    expr[i], expr[j] = f"(bf-sum x{i} x{j} {z})", f"(bf-diff x{i} x{j} {z})"
                else:
                    expr[i], expr[j] = f"(ib-sum x{i} x{j})", f"(ib-diff x{i} x{j} {z})"
            elif op[0] == "scale":
                expr[op[1]] = f"(scale x{op[1]} 3303)"
            else:
                kernel = ("ntt" if forward else "inv") + ("4" if op[0] == "four" else "2")
                expr[op[1]] = f"({kernel} x{op[1]} " + " ".join(str(z) for z in op[2:]) + ")"
        lines += [f"  -- | Layer {n} of {name}.", f"  {layer} :: Poly -> Poly", f"  {layer} p = case p of",
                  "    Poly " + " ".join(f"x{i}" for i in range(32)) + " ->", "      Poly " + " ".join(expr), ""]
    body = "p"
    for layer in names:
        body = f"({layer} {body})" if body != "p" else f"({layer} p)"
    lines += [f"  -- | {doc}", f"  {name} :: Poly -> Poly", f"  {name} p = {body[1:-1]}"]
    return lines


def lanewise(name: str, doc: str, kernel: str, arity: int) -> list[str]:
    """A Cinch function applying a lane kernel across polynomials."""
    vs = [[f"{chr(97 + k)}{i}" for i in range(32)] for k in range(arity)]
    args = " ".join(chr(112 + k) for k in range(arity))
    lines = [f"  -- | {doc}", f"  {name} :: " + " -> ".join(["Poly"] * (arity + 1)), f"  {name} {args} ="]
    indent = "    "
    for k in range(arity):
        lines.append(f"{indent}case {chr(112 + k)} of")
        lines.append(f"{indent}  Poly " + " ".join(vs[k]) + " ->")
        indent += "    "
    lines.append(indent + "Poly " + " ".join(f"({kernel} " + " ".join(vs[k][i] for k in range(arity)) + ")" for i in range(32)))
    return lines


def basemul_fn() -> list[str]:
    """acc + a * b in the NTT domain across all lanes, with each lane's four twiddles."""
    a = [f"a{i}" for i in range(32)]
    b = [f"b{i}" for i in range(32)]
    c = [f"c{i}" for i in range(32)]
    terms = " ".join(f"(basemul-add {c[i]} {a[i]} {b[i]} " + " ".join(str(GAMMAS[4 * i + j]) for j in range(4)) + ")" for i in range(32))
    return ["  -- | Accumulate the NTT-domain product of two polynomials (FIPS 203 Algorithm 11) into acc.",
            "  poly-basemul-add :: Poly -> Poly -> Poly -> Poly",
            "  poly-basemul-add acc p r =",
            "    case acc of",
            "      Poly " + " ".join(c) + " ->",
            "        case p of",
            "          Poly " + " ".join(a) + " ->",
            "            case r of",
            "              Poly " + " ".join(b) + " ->",
            "                Poly " + terms]


def suffix() -> str:
    """The generated text that follows the marker."""
    verify()
    lines = [MARKER.rstrip("\n"), "",
             "  -- | A polynomial: 256 coefficients modulo q as thirty-two lanes of eight.",
             "  data Poly where",
             "    -- | Lanes in coefficient order.",
             "    Poly :: " + " -> ".join(["U128"] * 32) + " -> Poly", "",
             "  -- | The lanes of a polynomial in order.",
             "  poly-lanes :: Poly -> List U128",
             "  poly-lanes p = case p of Poly " + " ".join(f"x{i}" for i in range(32)) + " -> [" + ", ".join(f"x{i}" for i in range(32)) + "]", "",
             "  -- | A polynomial from its first thirty-two lanes (zero when fewer are given).",
             "  poly-of-lanes :: List U128 -> Poly",
             "  poly-of-lanes l ="]
    indent, src = "    ", "l"
    for i in range(32):
        lines.append(f"{indent}case {src} of")
        lines.append(f"{indent}  Nil -> zero-poly")
        lines.append(f"{indent}  Cons x{i} r{i} ->")
        indent, src = indent + "    ", f"r{i}"
    lines.append(indent + "Poly " + " ".join(f"x{i}" for i in range(32)))
    lines.append("")
    lines += ["  -- | The all-zero polynomial.", "  zero-poly :: Poly", "  zero-poly = Poly " + " ".join(["(join64 0 0)"] * 32), ""]
    lines += transform("poly-ntt", "The number-theoretic transform (FIPS 203 Algorithm 9).", forward_schedule(), True) + [""]
    lines += transform("poly-inv-ntt", "The inverse transform, including the final scaling by 128^-1 (FIPS 203 Algorithm 10).", inverse_schedule(), False) + [""]
    lines += lanewise("poly-add", "Coefficient-wise sum modulo q.", "ib-sum", 2) + [""]
    lines += lanewise("poly-sub", "Coefficient-wise difference modulo q.", "sub-q", 2) + [""]
    lines += basemul_fn() + [""]
    kernels = [
        ("bf-sum", "U128 -> U128 -> I64 -> U128", "Forward butterfly sum: a + zeta * b modulo q in every lane.", bf_sum, True),
        ("bf-diff", "U128 -> U128 -> I64 -> U128", "Forward butterfly difference: a - zeta * b modulo q in every lane.", bf_diff, True),
        ("ib-sum", "U128 -> U128 -> U128", "Lane-wise a + b modulo q (also the inverse butterfly sum).", ib_sum, True),
        ("ib-diff", "U128 -> U128 -> I64 -> U128", "Inverse butterfly difference: zeta * (b - a) modulo q in every lane.", ib_diff, True),
        ("sub-q", "U128 -> U128 -> U128", "Lane-wise a - b modulo q.", sub_q, True),
        ("scale", "U128 -> I64 -> U128", "Lane-wise multiplication by a constant modulo q.", scale, True),
        ("ntt4", "U128 -> I64 -> U128", "The forward layer of length four inside a lane: coefficients 0-3 against 4-7.", lambda b: intra(b, True, 4), True),
        ("ntt2", "U128 -> I64 -> I64 -> U128", "The forward layer of length two inside a lane, with one twiddle per half.", lambda b: intra(b, True, 2), True),
        ("inv4", "U128 -> I64 -> U128", "The inverse layer of length four inside a lane.", lambda b: intra(b, False, 4), True),
        ("inv2", "U128 -> I64 -> I64 -> U128", "The inverse layer of length two inside a lane, with one twiddle per half.", lambda b: intra(b, False, 2), True),
        ("basemul-add", "U128 -> U128 -> U128 -> I64 -> I64 -> I64 -> I64 -> U128", "acc + a * b for four degree-one products modulo X^2 - gamma (FIPS 203 Algorithm 12).", basemul_add, True),
        ("below-q", "U128 -> I64", "One when every lane is below q, else zero.", below_q, True),
        ("reduce-q", "U128 -> U128", "Reduce lanes below 2q into [0, q).", reduce_q, True),
        ("pack12", "U128 -> U128", "Pack eight 12-bit lanes into 96 bits (ByteEncode_12 of one lane).", pack12, False),
        ("unpack12", "U128 -> U128", "Unpack 96 bits into eight 12-bit lanes (ByteDecode_12 before reduction).", unpack12, False),
        ("message-lane", "I64 -> U128", "Decompress_1 of eight message bits: each bit becomes 0 or 1665.", message, False),
        ("cbd2", "I64 -> U128", "Eight centered binomial coefficients (eta = 2) from 32 bits, modulo q.", cbd2, False),
        ("join64", "U64 -> U64 -> U128", "A U128 from its low and high halves.", join64, False),
        ("low64", "U128 -> U64", "The low half of a U128.", low64, False),
        ("high64", "U128 -> U64", "The high half of a U128.", high64, False),
    ]
    for d in (1, 4, 5, 10, 11):
        kernels.append((f"compress{d}", "U128 -> U128", f"Compress_{d} of eight lanes, packed at {d} bits per coefficient.", compress(d), True))
        kernels.append((f"decompress{d}", "U128 -> U128", f"Decompress_{d} of eight packed {d}-bit fields into lanes.", decompress(d), False))
    for name, sig, doc, build, vector in kernels:
        lines += prim(name, sig, doc, build, vector) + [""]
    return "\n".join(lines).rstrip() + "\n"


def main() -> None:
    """Rewrite or check the generated suffix of Crypto.MlKem."""
    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    existing = TARGET.read_text()
    if existing.count(MARKER) != 1:
        raise SystemExit(f"{TARGET}: expected exactly one generated marker")
    expected = existing.split(MARKER, 1)[0].rstrip() + "\n\n" + suffix()
    if args.check:
        if existing != expected:
            raise SystemExit(f"{TARGET} is stale; rerun tooling/generate_mlkem_kernels.py")
        return
    TARGET.write_text(expected)


if __name__ == "__main__":
    main()
