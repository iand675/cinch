"""A lane DSL for generated SIMD kernels: one interface, two back ends.

Kernel bodies are written once against values of LLVM vector types. The IR back end emits an LLVM body for
one endianness (U128 and integer arguments are bitcast to vectors and, on big-endian targets, lane-reversed so
lane 0 always holds the least significant bits; little-endian memory loads are byte-swapped on big-endian
targets). The evaluator runs the same operations on Python integers, so a generator can check the exact
program it emits against a scalar model before writing it.
"""
from __future__ import annotations

import re

TY = re.compile(r"<(\d+) x (i\d+|double)>")


def parse(ty: str) -> tuple[int, str]:
    m = TY.fullmatch(ty)
    if not m:
        raise ValueError(ty)
    return int(m.group(1)), m.group(2)


def bits(elem: str) -> int:
    return 64 if elem == "double" else int(elem[1:])


def splat(ty: str, v) -> str:
    n, e = parse(ty)
    return "<" + ", ".join(f"{e} {v}" for _ in range(n)) + ">"


def const(ty: str, vals: list[int]) -> str:
    n, e = parse(ty)
    assert len(vals) == n
    return "<" + ", ".join(f"{e} {v}" for v in vals) + ">"


class IR:
    """Emits one LLVM body."""

    def __init__(self, big: bool) -> None:
        self.big, self.lines, self.n = big, [], 0

    def emit(self, text: str) -> str:
        self.n += 1
        name = f"%v{self.n}"
        self.lines.append(f"      {name} = {text}")
        return name

    def raw(self, text: str) -> None:
        self.lines.append(f"      {text}")

    # Values are (type, name) pairs.
    def c(self, ty, vals):
        return (ty, const(ty, vals) if isinstance(vals, list) else splat(ty, vals))

    def bin(self, op, x, y):
        assert x[0] == y[0], (op, x[0], y[0])
        return (x[0], self.emit(f"{op} {x[0]} {x[1]}, {y[1]}"))

    def icmp(self, pred, x, y):
        n, _ = parse(x[0])
        return (f"<{n} x i1>", self.emit(f"icmp {pred} {x[0]} {x[1]}, {y[1]}"))

    def select(self, m, x, y):
        return (x[0], self.emit(f"select {m[0]} {m[1]}, {x[0]} {x[1]}, {x[0]} {y[1]}"))

    def cast(self, op, x, ty):
        return (ty, self.emit(f"{op} {x[0]} {x[1]} to {ty}"))

    def shuf(self, x, y, lanes):
        n, e = parse(x[0])
        ty = f"<{len(lanes)} x {e}>"
        other = y[1] if y is not None else "poison"
        return (ty, self.emit(f"shufflevector {x[0]} {x[1]}, {x[0]} {other}, <{len(lanes)} x i32> <" + ", ".join(f"i32 {i}" for i in lanes) + ">"))

    def intr(self, name, x, y=None):
        n, e = parse(x[0])
        args = f"{x[0]} {x[1]}" + (f", {y[0]} {y[1]}" if y is not None else "")
        return (x[0], self.emit(f"call {x[0]} @llvm.{name}.v{n}{e}({args})"))

    def reduce_umin(self, x):
        n, e = parse(x[0])
        return ("i32", self.emit(f"call {e} @llvm.vector.reduce.umin.v{n}{e}({x[0]} {x[1]})"))

    def extract(self, x, idx):
        n, e = parse(x[0])
        i = idx[1] if isinstance(idx, tuple) else str(idx)
        it = idx[0] if isinstance(idx, tuple) else "i64"
        return (e, self.emit(f"extractelement {x[0]} {x[1]}, {it} {i}"))

    # Boundaries.
    def arg128(self, arg, ty):
        """A U128 argument as canonical lanes (lane 0 in the low bits)."""
        n, e = parse(ty)
        v = (ty, self.emit(f"bitcast i128 {arg} to {ty}"))
        return self.shuf(v, None, list(range(n - 1, -1, -1))) if self.big else v

    def arg_int(self, arg, width, ty):
        """A width-bit integer argument as canonical byte lanes."""
        n, e = parse(ty)
        v = (ty, self.emit(f"bitcast i{width} {arg} to {ty}"))
        return self.shuf(v, None, list(range(n - 1, -1, -1))) if self.big else v

    def ret128(self, v):
        n, e = parse(v[0])
        if self.big:
            v = self.shuf(v, None, list(range(n - 1, -1, -1)))
        r = self.emit(f"bitcast {v[0]} {v[1]} to i128")
        self.raw(f"ret i128 {r}")

    def ret_int(self, v, width):
        n, e = parse(v[0])
        if self.big:
            v = self.shuf(v, None, list(range(n - 1, -1, -1)))
        r = self.emit(f"bitcast {v[0]} {v[1]} to i{width}")
        self.raw(f"ret i{width} {r}")

    def load_le(self, base, offset, ty):
        """Load little-endian elements from base + offset bytes."""
        n, e = parse(ty)
        p = self.emit(f"getelementptr i8, ptr {base}, i64 {offset}")
        v = (ty, self.emit(f"load {ty}, ptr {p}, align 1"))
        if self.big and bits(e) > 8:
            v = self.intr("bswap", v)
        return v


class Ev:
    """Evaluates the same operations on integer lanes."""

    def __init__(self, memory: bytes = b"") -> None:
        self.memory = memory

    @staticmethod
    def norm(e, v):
        if e == "double":
            return float(v)
        w = bits(e)
        return v & ((1 << w) - 1)

    @staticmethod
    def signed(e, v):
        w = bits(e)
        return v - (1 << w) if v & (1 << (w - 1)) else v

    def c(self, ty, vals):
        n, e = parse(ty)
        vals = vals if isinstance(vals, list) else [vals] * n
        return (ty, [self.norm(e, v) for v in vals])

    def bin(self, op, x, y):
        ty = x[0]
        n, e = parse(ty)
        out = []
        for a, b in zip(x[1], y[1]):
            if op == "add": r = a + b
            elif op == "sub": r = a - b
            elif op == "mul": r = a * b
            elif op == "and": r = a & b
            elif op == "or": r = a | b
            elif op == "xor": r = a ^ b
            elif op == "shl": r = a << b
            elif op == "ashr": r = self.signed(e, a) >> b
            elif op == "lshr": r = a >> b
            elif op == "udiv": r = a // b
            elif op == "fdiv": r = a / b
            else: raise ValueError(op)
            out.append(self.norm(e, r))
        return (ty, out)

    def icmp(self, pred, x, y):
        n, e = parse(x[0])
        res = []
        for a, b in zip(x[1], y[1]):
            sa, sb = self.signed(e, a), self.signed(e, b)
            res.append({"eq": a == b, "ne": a != b, "slt": sa < sb, "sgt": sa > sb, "sle": sa <= sb, "sge": sa >= sb,
                        "ult": a < b, "ugt": a > b, "ule": a <= b, "uge": a >= b}[pred])
        return (f"<{n} x i1>", res)

    def select(self, m, x, y):
        return (x[0], [a if k else b for k, a, b in zip(m[1], x[1], y[1])])

    def cast(self, op, x, ty):
        n, e = parse(x[0])
        n2, e2 = parse(ty)
        out = []
        for a in x[1]:
            if op == "sext": r = self.signed(e, a)
            elif op in ("zext", "trunc"): r = a
            elif op == "uitofp": r = float(a)
            elif op == "fptoui": r = int(a)
            else: raise ValueError(op)
            out.append(self.norm(e2, r))
        return (ty, out)

    def shuf(self, x, y, lanes):
        n, e = parse(x[0])
        src = x[1] + (y[1] if y is not None else [0] * n)
        return (f"<{len(lanes)} x {e}>", [src[i] for i in lanes])

    def intr(self, name, x, y=None):
        n, e = parse(x[0])
        if name == "smin":
            return (x[0], [a if self.signed(e, a) <= self.signed(e, b) else b for a, b in zip(x[1], y[1])])
        if name == "smax":
            return (x[0], [a if self.signed(e, a) >= self.signed(e, b) else b for a, b in zip(x[1], y[1])])
        if name == "bswap":
            return x
        raise ValueError(name)

    def reduce_umin(self, x):
        return ("i32", min(x[1]))

    def extract(self, x, idx):
        i = idx[1] if isinstance(idx, tuple) else idx
        return (parse(x[0])[1], x[1][i])

    def arg128(self, arg, ty):
        n, e = parse(ty)
        w = bits(e)
        return (ty, [(arg >> (w * i)) & ((1 << w) - 1) for i in range(n)])

    def arg_int(self, arg, width, ty):
        return self.arg128(arg, ty)

    def ret128(self, v):
        n, e = parse(v[0])
        w = bits(e)
        self.result = sum((int(x) & ((1 << w) - 1)) << (w * i) for i, x in enumerate(v[1]))

    def ret_int(self, v, width):
        self.ret128(v)

    def load_le(self, base, offset, ty):
        n, e = parse(ty)
        w = bits(e) // 8
        o = base + offset
        return (ty, [int.from_bytes(self.memory[o + w * i:o + w * i + w], "little") for i in range(n)])
