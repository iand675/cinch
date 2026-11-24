"""A small typed language for binary64 kernels, with an LLVM backend and a Python backend.

Kernels are written as Python function source over five types: ``f`` (binary64), ``i`` (signed 64-bit), ``u``
(unsigned 64-bit), ``b`` (truth value) and tuples of 64-bit values as results. Arithmetic on ``f`` is IEEE
round-to-nearest with no contraction, so both backends compute identical bits.

* The LLVM backend lowers the whole body without jumps: both arms of an ``if`` run, assignments merge through
  ``select``, and ``return`` records the first value reached. Calls to other kernels are inlined. The result is a
  prim body (``%0``, ``%1`` ... are the parameters); a pair of 64-bit results is packed into an ``i128`` (first in
  the low half).
* The Python backend keeps the control flow and wraps every operation in its typed meaning (wrapping integers,
  IEEE division by zero, shifts of 64 or more giving zero), so a kernel can run as an executable model.

Integer shifts follow the reference semantics: shifting by 64 or more yields zero (or the sign for a signed right
shift); the LLVM backend guards variable shift amounts with ``select``. Builtins: ``bits`` (f to u), ``fbits``
(u to f), ``itof``, ``utof``, ``ftoi``, ``ftou``, ``u_`` and ``i_`` (reinterpret), ``isnan``, ``sqrt``, ``fabs``,
``floor``, ``ceil``, ``trunc``, ``copysign``, ``clz``, ``mul`` (unsigned 64 by 64 to a (hi, lo) pair) and
``pick(TABLE, index)`` for constant unsigned tables (``fpick`` reads the word as binary64). Names not bound locally resolve to the constant table given to
the compiler. ``x: u = 0`` annotates a variable's type. ``round`` rounds halves away from zero and ``roundeven``
to even.
"""
from __future__ import annotations

import ast
import math
import struct
from dataclasses import dataclass, field

M64 = (1 << 64) - 1
LL = {"f": "double", "i": "i64", "u": "i64", "b": "i1", "w": "i128"}
CINCH = {"f": "F64", "i": "I64", "u": "U64"}


def hexf(x: float) -> str:
    """A binary64 constant in LLVM's exact hexadecimal form."""
    return "0x%016X" % struct.unpack("<Q", struct.pack("<d", x))[0]


# ---------------------------------------------------------------------------
# Python-backend runtime.
# ---------------------------------------------------------------------------

def _wrap_i(v: int) -> int:
    v &= M64
    return v - (1 << 64) if v >> 63 else v


def _fdiv(a: float, b: float) -> float:
    try:
        return a / b
    except ZeroDivisionError:
        if a == 0 or a != a:
            return math.nan
        return math.copysign(math.inf, a) * math.copysign(1.0, b)


def _ftoi(x: float) -> int:
    if x != x or math.isinf(x) or not -2.0**63 <= x < 2.0**63:
        return -(1 << 63)
    return int(x)


def _ftou(x: float) -> int:
    if x != x or math.isinf(x) or not 0 <= x < 2.0**64:
        return 0
    return int(x)


def _fround(fn, x: float) -> float:
    if x != x or math.isinf(x) or x == 0:
        return x
    r = float(fn(x))
    return math.copysign(r, x) if r == 0 else r


RUNTIME = {
    "_wrap_i": _wrap_i, "_fdiv": _fdiv, "_ftoi": _ftoi, "_ftou": _ftou, "M64": M64, "math": math,
    "_bits": lambda x: struct.unpack("<Q", struct.pack("<d", x))[0],
    "_fbits": lambda u: struct.unpack("<d", struct.pack("<Q", u & M64))[0],
    "_sqrt": lambda x: math.nan if x != x or x < 0 else (x if x == 0 or math.isinf(x) else math.sqrt(x)),
    "_floor": lambda x: _fround(math.floor, x), "_ceil": lambda x: _fround(math.ceil, x),
    "_trunc": lambda x: _fround(math.trunc, x),
    "_clz": lambda u: 64 - (u & M64).bit_length(),
    "_sdiv": lambda a, b: (abs(a) // abs(b)) * (1 if (a < 0) == (b < 0) else -1),
    "_round": lambda x: _fround(lambda v: v if abs(v) >= 2.0**52 else math.copysign(math.floor(abs(v)) + (1 if abs(v) - math.floor(abs(v)) >= 0.5 else 0), v), x),
    "_roundeven": lambda x: _fround(lambda v: round(v) if abs(v) < 2.0**52 else v, x),
    "_shl": lambda a, s: 0 if s >= 64 or s < 0 else (a << s) & M64,
    "_shr": lambda a, s: 0 if s >= 64 or s < 0 else (a & M64) >> s,
    "_sar": lambda a, s: (-1 if a < 0 else 0) if s >= 64 or s < 0 else a >> s,
    "_mul": lambda a, b: (((a * b) >> 64) & M64, (a * b) & M64),
}


# ---------------------------------------------------------------------------
# Compiler.
# ---------------------------------------------------------------------------

@dataclass
class V:
    """A typed value: an LLVM operand or a Python expression."""
    t: str
    s: str
    const: object = None


@dataclass
class Kernel:
    """A kernel: Cinch name, parameter types, source and documentation."""
    name: str
    params: list[str]
    source: str
    doc: str
    entry: bool = True
    fn: ast.FunctionDef = field(init=False)

    def __post_init__(self) -> None:
        tree = ast.parse(self.source.strip("\n"))
        [self.fn] = tree.body
        assert len(self.fn.args.args) == len(self.params), self.name


class Program:
    """A set of kernels sharing constants and tables."""

    def __init__(self, kernels: list[Kernel], consts: dict, tables: dict[str, list[int]]):
        self.kernels = {k.fn.name: k for k in kernels}
        self.consts = consts
        self.tables = tables

    # -- LLVM -----------------------------------------------------------------

    def llvm(self, k: Kernel) -> tuple[str, str]:
        """The prim body and Cinch result type of an entry kernel."""
        e = LBody(self)
        args = [V(t, f"%{i}") for i, t in enumerate(k.params)]
        res = e.inline(k.fn, args)
        if isinstance(res, tuple):
            a, b = res
            lo = e.zext128(e.as_int(a))
            hi = e.zext128(e.as_int(b))
            sh = e.emit(f"shl i128 {hi}, 64")
            out = e.emit(f"or i128 {sh}, {lo}")
            e.lines.append(f"ret i128 {out}")
            return "\n".join(e.lines), "U128"
        e.lines.append(f"ret {LL[res.t]} {res.s}")
        return "\n".join(e.lines), CINCH[res.t]

    # -- Python ---------------------------------------------------------------

    def python(self) -> dict:
        """Every kernel as a Python function (with true control flow)."""
        src = []
        for k in self.kernels.values():
            src.append(PBody(self, k).function())
        ns = dict(RUNTIME)
        ns.update({f"T_{n}": v for n, v in self.tables.items()})
        exec("\n\n".join(src), ns)  # noqa: S102 - generated from the kernels in this repository
        return {k.fn.name: ns[k.fn.name] for k in self.kernels.values()}


BINF = {ast.Add: "fadd", ast.Sub: "fsub", ast.Mult: "fmul", ast.Div: "fdiv"}
BINI = {ast.Add: "add", ast.Sub: "sub", ast.Mult: "mul", ast.BitAnd: "and", ast.BitOr: "or", ast.BitXor: "xor"}
CMPF = {ast.Lt: "olt", ast.LtE: "ole", ast.Gt: "ogt", ast.GtE: "oge", ast.Eq: "oeq", ast.NotEq: "une"}
CMPS = {ast.Lt: "slt", ast.LtE: "sle", ast.Gt: "sgt", ast.GtE: "sge", ast.Eq: "eq", ast.NotEq: "ne"}
CMPU = {ast.Lt: "ult", ast.LtE: "ule", ast.Gt: "ugt", ast.GtE: "uge", ast.Eq: "eq", ast.NotEq: "ne"}
PYOP = {ast.Add: "+", ast.Sub: "-", ast.Mult: "*", ast.BitAnd: "&", ast.BitOr: "|", ast.BitXor: "^",
        ast.Lt: "<", ast.LtE: "<=", ast.Gt: ">", ast.GtE: ">=", ast.Eq: "==", ast.NotEq: "!="}


class Typing:
    """Literal typing shared by both backends."""

    def literal(self, value, want: str | None) -> V:
        if isinstance(value, bool):
            return V("b", "true" if value else "false", value)
        if isinstance(value, float) or want == "f":
            return V("f", hexf(float(value)), float(value))
        t = want or "i"
        return V(t, str(value if t == "i" else value & M64), value)

    def constant(self, name: str, want: str | None):
        v = self.prog.consts[name]
        if isinstance(v, tuple):
            return self.literal(v[1], v[0])
        return self.literal(v, want)


class LBody(Typing):
    """Branch-free LLVM lowering."""

    def __init__(self, prog: Program):
        self.prog = prog
        self.lines: list[str] = []
        self.n = 0

    def emit(self, text: str) -> str:
        self.n += 1
        name = f"%v{self.n}"
        self.lines.append(f"{name} = {text}")
        return name

    def zext128(self, s: str) -> str:
        return self.emit(f"zext i64 {s} to i128")

    def as_int(self, v: V) -> str:
        return self.emit(f"bitcast double {v.s} to i64") if v.t == "f" else v.s

    def sel(self, c: V, a: V, b: V) -> V:
        if a.s == b.s:
            return a
        assert a.t == b.t, (a, b)
        return V(a.t, self.emit(f"select i1 {c.s}, {LL[a.t]} {a.s}, {LL[a.t]} {b.s}"))

    def inline(self, fn: ast.FunctionDef, args: list):
        env = {a.arg: v for a, v in zip(fn.args.args, args)}
        self.block(fn.body, env)
        assert "__res" in env, fn.name
        return env["__res"]

    def block(self, stmts, env) -> None:
        for st in stmts:
            self.stmt(st, env)

    def stmt(self, st, env) -> None:
        if isinstance(st, ast.Expr) and isinstance(st.value, ast.Constant):
            return
        if isinstance(st, ast.AnnAssign):
            env[st.target.id] = self.expr(st.value, env, st.annotation.id)
            return
        if isinstance(st, ast.Assign):
            [target] = st.targets
            if isinstance(target, ast.Tuple):
                vals = self.expr(st.value, env, None)
                for t, v in zip(target.elts, vals):
                    env[t.id] = v
            else:
                want = env[target.id].t if target.id in env else None
                env[target.id] = self.expr(st.value, env, want)
            return
        if isinstance(st, ast.AugAssign):
            node = ast.BinOp(left=ast.Name(id=st.target.id, ctx=ast.Load()), op=st.op, right=st.value)
            env[st.target.id] = self.expr(node, env, env[st.target.id].t)
            return
        if isinstance(st, ast.Return):
            val = self.expr(st.value, env, None)
            if "__res" not in env:
                env["__res"] = val
            elif isinstance(val, tuple):
                d = env["__done"]
                env["__res"] = tuple(self.sel(d, a, b) for a, b in zip(env["__res"], val))
            else:
                env["__res"] = self.sel(env["__done"], env["__res"], val)
            env["__done"] = V("b", "true", True)
            return
        if isinstance(st, ast.If):
            c = self.expr(st.test, env, None)
            assert c.t == "b"
            a, b = dict(env), dict(env)
            self.block(st.body, a)
            self.block(st.orelse, b)
            for name in list(a) + [n for n in b if n not in a]:
                if name in a and name in b:
                    if name == "__done":
                        env[name] = self.sel(c, a[name], b[name])
                    elif isinstance(a[name], tuple):
                        env[name] = tuple(self.sel(c, x, y) for x, y in zip(a[name], b[name]))
                    else:
                        env[name] = self.sel(c, a[name], b[name])
                elif name == "__done":
                    other = V("b", "false", False)
                    env[name] = self.sel(c, a[name], other) if name in a else self.sel(c, other, b[name])
                else:
                    env[name] = a.get(name, b.get(name))
            return
        raise SyntaxError(ast.dump(st))

    def expr(self, node, env, want):
        if isinstance(node, ast.Constant):
            return self.literal(node.value, want)
        if isinstance(node, ast.Name):
            return env[node.id] if node.id in env else self.constant(node.id, want)
        if isinstance(node, ast.Tuple):
            return tuple(self.expr(e, env, None) for e in node.elts)
        if isinstance(node, ast.UnaryOp):
            if isinstance(node.op, ast.Not):
                v = self.expr(node.operand, env, "b")
                return V("b", self.emit(f"xor i1 {v.s}, true"))
            v = self.expr(node.operand, env, want)
            if isinstance(node.op, ast.Invert):
                if v.const is not None:
                    return self.literal(~v.const, v.t)
                return V(v.t, self.emit(f"xor i64 {v.s}, -1"))
            if v.const is not None:
                return self.literal(-v.const, v.t)
            if v.t == "f":
                return V("f", self.emit(f"fneg double {v.s}"))
            return V(v.t, self.emit(f"sub i64 0, {v.s}"))
        if isinstance(node, ast.BoolOp):
            vals = [self.expr(e, env, "b") for e in node.values]
            op = "and" if isinstance(node.op, ast.And) else "or"
            acc = vals[0]
            for v in vals[1:]:
                acc = V("b", self.emit(f"{op} i1 {acc.s}, {v.s}"))
            return acc
        if isinstance(node, ast.IfExp):
            c = self.expr(node.test, env, "b")
            a = self.expr(node.body, env, want)
            b = self.expr(node.orelse, env, a.t)
            return self.sel(c, a, b)
        if isinstance(node, ast.Compare):
            [op] = node.ops
            a, b = self.operands(node.left, node.comparators[0], env, None)
            if a.t == "f":
                return V("b", self.emit(f"fcmp {CMPF[type(op)]} double {a.s}, {b.s}"))
            if a.t == "b":
                return V("b", self.emit(f"icmp {CMPS[type(op)]} i1 {a.s}, {b.s}"))
            table = CMPU if a.t == "u" else CMPS
            return V("b", self.emit(f"icmp {table[type(op)]} i64 {a.s}, {b.s}"))
        if isinstance(node, ast.BinOp):
            a, b = self.operands(node.left, node.right, env, want)
            if a.const is not None and b.const is not None and a.t == "f":
                return self.literal(eval_float(type(node.op), a.const, b.const), "f")
            if a.t == "f":
                return V("f", self.emit(f"{BINF[type(node.op)]} double {a.s}, {b.s}"))
            op = type(node.op)
            if op in BINI:
                return V(a.t, self.emit(f"{BINI[op]} i64 {a.s}, {b.s}"))
            if op is ast.Div:
                return V(a.t, self.emit(f"{'udiv' if a.t == 'u' else 'sdiv'} i64 {a.s}, {b.s}"))
            if op is ast.Mod:
                return V(a.t, self.emit(f"{'urem' if a.t == 'u' else 'srem'} i64 {a.s}, {b.s}"))
            if op in (ast.LShift, ast.RShift):
                kind = "shl" if op is ast.LShift else ("lshr" if a.t == "u" else "ashr")
                if b.const is not None and 0 <= b.const < 64:
                    return V(a.t, self.emit(f"{kind} i64 {a.s}, {b.s}"))
                masked = self.emit(f"and i64 {b.s}, 63")
                raw = self.emit(f"{kind} i64 {a.s}, {masked}")
                big = self.emit(f"icmp uge i64 {b.s}, 64")
                fill = self.emit(f"ashr i64 {a.s}, 63") if kind == "ashr" else "0"
                return V(a.t, self.emit(f"select i1 {big}, i64 {fill}, i64 {raw}"))
            raise SyntaxError(ast.dump(node))
        if isinstance(node, ast.Call):
            return self.call(node, env, want)
        raise SyntaxError(ast.dump(node))

    def operands(self, left, right, env, want):
        """Type two operands, letting a literal take the other side's type."""
        lit = (ast.Constant,)
        if isinstance(left, lit) and not isinstance(right, lit):
            b = self.expr(right, env, want)
            return self.expr(left, env, b.t), b
        a = self.expr(left, env, want)
        return a, self.expr(right, env, a.t)

    def call(self, node, env, want):
        name = node.func.id
        if name in ("pick", "fpick"):
            table = self.prog.tables[node.args[0].id]
            idx = self.expr(node.args[1], env, "u")
            vec = ", ".join(f"i64 {v}" for v in table)
            word = self.emit(f"extractelement <{len(table)} x i64> <{vec}>, i64 {idx.s}")
            return V("u", word) if name == "pick" else V("f", self.emit(f"bitcast i64 {word} to double"))
        if name in self.prog.kernels:
            fn = self.prog.kernels[name]
            args = [self.expr(a, env, t) for a, t in zip(node.args, fn.params)]
            return self.inline(fn.fn, args)
        args = [self.expr(a, env, None) for a in node.args]
        x = args[0]
        if name == "bits":
            return V("u", self.emit(f"bitcast double {x.s} to i64"))
        if name == "fbits":
            return V("f", self.emit(f"bitcast i64 {x.s} to double"))
        if name == "itof":
            return V("f", self.emit(f"sitofp i64 {x.s} to double"))
        if name == "utof":
            return V("f", self.emit(f"uitofp i64 {x.s} to double"))
        if name == "ftoi":
            return V("i", self.emit(f"call i64 @llvm.fptosi.sat.i64.f64(double {x.s})"))
        if name == "ftou":
            return V("u", self.emit(f"call i64 @llvm.fptoui.sat.i64.f64(double {x.s})"))
        if name == "u_":
            return V("u", x.s, x.const)
        if name == "i_":
            return V("i", x.s, x.const)
        if name == "isnan":
            return V("b", self.emit(f"fcmp uno double {x.s}, 0.0"))
        if name in ("sqrt", "fabs", "floor", "ceil", "trunc", "round", "roundeven"):
            return V("f", self.emit(f"call double @llvm.{name}.f64(double {x.s})"))
        if name == "copysign":
            return V("f", self.emit(f"call double @llvm.copysign.f64(double {x.s}, double {args[1].s})"))
        if name == "clz":
            return V("u", self.emit(f"call i64 @llvm.ctlz.i64(i64 {x.s}, i1 false)"))
        if name == "mul":
            a = self.zext128(x.s)
            b = self.zext128(args[1].s)
            p = self.emit(f"mul i128 {a}, {b}")
            h = self.emit(f"lshr i128 {p}, 64")
            return V("u", self.emit(f"trunc i128 {h} to i64")), V("u", self.emit(f"trunc i128 {p} to i64"))
        raise SyntaxError(name)


def eval_float(op, a: float, b: float) -> float:
    """Constant folding of binary64 literals."""
    return {ast.Add: lambda: a + b, ast.Sub: lambda: a - b, ast.Mult: lambda: a * b, ast.Div: lambda: _fdiv(a, b)}[op]()


class PBody(Typing):
    """The Python backend: structured control flow with typed operations."""

    def __init__(self, prog: Program, k: Kernel):
        self.prog = prog
        self.k = k
        self.out: list[str] = []

    def function(self) -> str:
        fn = self.k.fn
        env = {a.arg: t for a, t in zip(fn.args.args, self.k.params)}
        self.out.append(f"def {fn.name}({', '.join(a.arg for a in fn.args.args)}):")
        self.block(fn.body, env, 1)
        return "\n".join(self.out)

    def block(self, stmts, env, depth) -> None:
        pad = "    " * depth
        if not stmts:
            self.out.append(pad + "pass")
        for st in stmts:
            if isinstance(st, ast.Expr) and isinstance(st.value, ast.Constant):
                continue
            if isinstance(st, ast.AnnAssign):
                v = self.expr(st.value, env, st.annotation.id)
                env[st.target.id] = v.t
                self.out.append(f"{pad}{st.target.id} = {v.s}")
            elif isinstance(st, ast.Assign):
                [target] = st.targets
                if isinstance(target, ast.Tuple):
                    vals = self.expr(st.value, env, None)
                    names = [t.id for t in target.elts]
                    for n, v in zip(names, vals.t):
                        env[n] = v
                    self.out.append(f"{pad}{', '.join(names)} = {vals.s}")
                else:
                    v = self.expr(st.value, env, env.get(target.id))
                    env[target.id] = v.t
                    self.out.append(f"{pad}{target.id} = {v.s}")
            elif isinstance(st, ast.AugAssign):
                node = ast.BinOp(left=ast.Name(id=st.target.id, ctx=ast.Load()), op=st.op, right=st.value)
                v = self.expr(node, env, env[st.target.id])
                self.out.append(f"{pad}{st.target.id} = {v.s}")
            elif isinstance(st, ast.Return):
                v = self.expr(st.value, env, None)
                self.out.append(f"{pad}return {v.s}")
            elif isinstance(st, ast.If):
                c = self.expr(st.test, env, "b")
                self.out.append(f"{pad}if {c.s}:")
                a = dict(env)
                self.block(st.body, a, depth + 1)
                b = dict(env)
                if st.orelse:
                    self.out.append(f"{pad}else:")
                    self.block(st.orelse, b, depth + 1)
                env.update(b)
                env.update(a)
            else:
                raise SyntaxError(ast.dump(st))

    def expr(self, node, env, want) -> V:
        if isinstance(node, ast.Constant):
            return self.py_const(self.literal(node.value, want))
        if isinstance(node, ast.Name):
            if node.id in env:
                return V(env[node.id], node.id)
            return self.py_const(self.constant(node.id, want))
        if isinstance(node, ast.Tuple):
            vals = [self.expr(e, env, None) for e in node.elts]
            return V(tuple(v.t for v in vals), "(" + ", ".join(v.s for v in vals) + ")")
        if isinstance(node, ast.UnaryOp):
            if isinstance(node.op, ast.Not):
                return V("b", f"(not {self.expr(node.operand, env, 'b').s})")
            v = self.expr(node.operand, env, want)
            if isinstance(node.op, ast.Invert):
                return V(v.t, f"((~{v.s}) & M64)" if v.t == "u" else f"(~{v.s})")
            if v.t == "f":
                return V("f", f"(-{v.s})")
            return V(v.t, f"((-{v.s}) & M64)" if v.t == "u" else f"_wrap_i(-{v.s})")
        if isinstance(node, ast.BoolOp):
            op = " and " if isinstance(node.op, ast.And) else " or "
            return V("b", "(" + op.join(self.expr(e, env, "b").s for e in node.values) + ")")
        if isinstance(node, ast.IfExp):
            c = self.expr(node.test, env, "b")
            a = self.expr(node.body, env, want)
            b = self.expr(node.orelse, env, a.t)
            return V(a.t, f"({a.s} if {c.s} else {b.s})")
        if isinstance(node, ast.Compare):
            [op] = node.ops
            a, b = self.operands(node.left, node.comparators[0], env, None)
            return V("b", f"({a.s} {PYOP[type(op)]} {b.s})")
        if isinstance(node, ast.BinOp):
            a, b = self.operands(node.left, node.right, env, want)
            op = type(node.op)
            if a.t == "f":
                if op is ast.Div:
                    return V("f", f"_fdiv({a.s}, {b.s})")
                return V("f", f"({a.s} {PYOP[op]} {b.s})")
            if op in BINI:
                raw = f"({a.s} {PYOP[op]} {b.s})"
            elif op is ast.Div:
                raw = f"({a.s} // {b.s})" if a.t == "u" else f"_sdiv({a.s}, {b.s})"
            elif op is ast.Mod:
                raw = f"({a.s} % {b.s})" if a.t == "u" else f"math.fmod({a.s}, {b.s})"
            elif op is ast.LShift:
                return V(a.t, f"_shl({a.s}, {b.s})" if a.t == "u" else f"_wrap_i(_shl({a.s} & M64, {b.s}))")
            else:
                return V(a.t, f"_shr({a.s}, {b.s})" if a.t == "u" else f"_sar({a.s}, {b.s})")
            return V(a.t, f"({raw} & M64)" if a.t == "u" else f"_wrap_i({raw})")
        if isinstance(node, ast.Call):
            return self.call(node, env, want)
        raise SyntaxError(ast.dump(node))

    def py_const(self, v: V) -> V:
        """A literal as Python source: binary64 by its bits, integers in their type's range."""
        if v.t == "f":
            return V("f", f"_fbits({v.s})", v.const)
        if v.t == "u":
            return V("u", str(v.const & M64), v.const)
        return V(v.t, repr(v.const), v.const)

    def operands(self, left, right, env, want):
        if isinstance(left, ast.Constant) and not isinstance(right, ast.Constant):
            b = self.expr(right, env, want)
            return self.expr(left, env, b.t), b
        a = self.expr(left, env, want)
        return a, self.expr(right, env, a.t)

    def call(self, node, env, want) -> V:
        name = node.func.id
        if name == "pick":
            return V("u", f"T_{node.args[0].id}[{self.expr(node.args[1], env, 'u').s}]")
        if name == "fpick":
            return V("f", f"_fbits(T_{node.args[0].id}[{self.expr(node.args[1], env, 'u').s}])")
        if name in self.prog.kernels:
            fn = self.prog.kernels[name]
            args = [self.expr(a, env, t) for a, t in zip(node.args, fn.params)]
            rt = result_types(self.prog, fn)
            return V(rt, f"{name}({', '.join(a.s for a in args)})")
        args = [self.expr(a, env, None) for a in node.args]
        x = args[0].s
        simple = {"bits": ("u", f"_bits({x})"), "fbits": ("f", f"_fbits({x})"), "itof": ("f", f"float({x})"),
                  "utof": ("f", f"float({x})"), "ftoi": ("i", f"_ftoi({x})"), "ftou": ("u", f"_ftou({x})"),
                  "u_": ("u", f"(({x}) & M64)"), "i_": ("i", f"_wrap_i({x})"), "isnan": ("b", f"({x} != {x})"),
                  "sqrt": ("f", f"_sqrt({x})"), "fabs": ("f", f"abs({x})"), "floor": ("f", f"_floor({x})"),
                  "ceil": ("f", f"_ceil({x})"), "trunc": ("f", f"_trunc({x})"), "clz": ("u", f"_clz({x})"),
                  "round": ("f", f"_round({x})"), "roundeven": ("f", f"_roundeven({x})")}
        if name in simple:
            t, s = simple[name]
            return V(t, s)
        if name == "copysign":
            return V("f", f"math.copysign({x}, {args[1].s})")
        if name == "mul":
            return V(("u", "u"), f"_mul({x}, {args[1].s})")
        raise SyntaxError(name)


def result_types(prog: Program, k: Kernel):
    """The result type(s) of a kernel, found by lowering it."""
    e = LBody(prog)
    res = e.inline(k.fn, [V(t, f"%{i}") for i, t in enumerate(k.params)])
    return tuple(v.t for v in res) if isinstance(res, tuple) else res.t
