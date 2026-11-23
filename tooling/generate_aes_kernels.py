#!/usr/bin/env python3
"""Generate constant-time portable AES and GHASH fragments and hardware-instruction bodies.

Portable AES works on all sixteen state bytes at once in `<16 x i8>` lanes.
SubBytes is computed arithmetically, never by table lookup: the GF(2^8)
inverse is x^254 by an addition chain of seven squarings (a linear map,
applied with lane masks) and four lane-parallel shift-and-add products,
followed by the affine map with lane rotations. ShiftRows is one constant
shuffle and MixColumns three rotations within each column plus one doubling.
Every operation is data-independent, so the fallback is free of cache and
branch timing channels.

Hardware bodies use AES-NI on x86-64 after CPUID detection and the ARMv8
cryptography extension when a target profile supplies `ArmAes` evidence; each
processes a whole block with the round keys as register operands. GHASH has a
portable constant-time body that forms carry-less products with masked integer
multiplication and a PCLMULQDQ body on x86-64.

Blocks travel as U128 values whose byte i occupies bits 8i..8i+7, the
little-endian reading of the bytes in memory order; vector bodies convert
explicitly so big-endian targets agree.
"""
from __future__ import annotations

import argparse
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
AES_MARKER = "  -- Generated AES round kernels: portable constant-time lanes and hardware block bodies.\n"
GHASH_MARKER = "  -- Generated GHASH multiplication: constant-time portable and carry-less-instruction bodies.\n"
ROUNDS = {10: 11, 12: 13, 14: 15}


def gf_mul(a: int, b: int) -> int:
    """Reference GF(2^8) product modulo x^8 + x^4 + x^3 + x + 1."""
    p = 0
    for _ in range(8):
        if b & 1:
            p ^= a
        a = ((a << 1) ^ (0x1B if a & 0x80 else 0)) & 0xFF
        b >>= 1
    return p


def sbox(x: int) -> int:
    """Reference S-box from inversion and the affine map."""
    inv = 1
    for _ in range(254):
        inv = gf_mul(inv, x)
    if x == 0:
        inv = 0
    rot = lambda v, n: ((v << n) | (v >> (8 - n))) & 0xFF
    return inv ^ rot(inv, 1) ^ rot(inv, 2) ^ rot(inv, 3) ^ rot(inv, 4) ^ 0x63


def self_check() -> None:
    """Check the arithmetic S-box against published entries."""
    assert [sbox(x) for x in (0x00, 0x01, 0x53, 0xFF)] == [0x63, 0x7C, 0xED, 0x16]


def splat(value: int, lanes: int = 16, bits: int = 8) -> str:
    """A constant vector with every lane equal to value."""
    return "<" + ", ".join(f"i{bits} {value}" for _ in range(lanes)) + ">"


class Ir:
    """Straight-line SSA builder with unique names."""

    def __init__(self) -> None:
        """Start an empty body."""
        self.lines: list[str] = []
        self.count = 0

    def emit(self, expression: str, hint: str = "t") -> str:
        """Append an instruction and return its result name."""
        self.count += 1
        name = f"%{hint}{self.count}"
        self.lines.append(f"{name} = {expression}")
        return name

    def ret(self, ty: str, value: str) -> None:
        """Return a value."""
        self.lines.append(f"ret {ty} {value}")

    def to_vector(self, wide: str, endian: str) -> str:
        """U128 to sixteen byte lanes in memory order."""
        vector = self.emit(f"bitcast i128 {wide} to <16 x i8>", "v")
        if endian == "big":
            vector = self.emit(f"shufflevector <16 x i8> {vector}, <16 x i8> zeroinitializer, <16 x i32> <{', '.join(f'i32 {15 - i}' for i in range(16))}>", "v")
        return vector

    def to_wide(self, vector: str, endian: str) -> str:
        """Sixteen byte lanes in memory order to U128."""
        if endian == "big":
            vector = self.emit(f"shufflevector <16 x i8> {vector}, <16 x i8> zeroinitializer, <16 x i32> <{', '.join(f'i32 {15 - i}' for i in range(16))}>", "v")
        return self.emit(f"bitcast <16 x i8> {vector} to i128", "w")

    def shuffle(self, vector: str, sources: list[int]) -> str:
        """Permute lanes with a constant mask."""
        mask = ", ".join(f"i32 {s}" for s in sources)
        return self.emit(f"shufflevector <16 x i8> {vector}, <16 x i8> zeroinitializer, <16 x i32> <{mask}>", "p")

    def xor(self, a: str, b: str) -> str:
        """Lane-wise exclusive-or."""
        return self.emit(f"xor <16 x i8> {a}, {b}", "x")

    def xtime(self, v: str) -> str:
        """Lane-wise multiplication by x, reducing with 0x1b through a sign mask."""
        top = self.emit(f"lshr <16 x i8> {v}, {splat(7)}", "h")
        mask = self.emit(f"sub <16 x i8> zeroinitializer, {top}", "m")
        reduce = self.emit(f"and <16 x i8> {mask}, {splat(0x1B)}", "r")
        shifted = self.emit(f"shl <16 x i8> {v}, {splat(1)}", "s")
        return self.xor(shifted, reduce)

    def bit_mask(self, v: str, bit: int) -> str:
        """All-ones lanes where the given bit of the lane is set."""
        shifted = self.emit(f"lshr <16 x i8> {v}, {splat(bit)}", "b") if bit else v
        low = self.emit(f"and <16 x i8> {shifted}, {splat(1)}", "b")
        return self.emit(f"sub <16 x i8> zeroinitializer, {low}", "b")

    def mul(self, a: str, b: str) -> str:
        """Lane-parallel GF(2^8) product by eight masked shift-and-add steps."""
        product = None
        for bit in range(8):
            term = self.emit(f"and <16 x i8> {a}, {self.bit_mask(b, bit)}", "q")
            product = term if product is None else self.xor(product, term)
            if bit < 7:
                a = self.xtime(a)
        return product

    def square(self, a: str) -> str:
        """Lane-parallel squaring, a linear map: sum of bit_i(a) * x^(2i) mod the AES polynomial."""
        result = None
        for bit in range(8):
            constant = 1
            for _ in range(2 * bit):
                constant = gf_mul(constant, 2)
            term = self.emit(f"and <16 x i8> {self.bit_mask(a, bit)}, {splat(constant)}", "q")
            result = term if result is None else self.xor(result, term)
        return result

    def rotate(self, v: str, amount: int) -> str:
        """Rotate every byte lane left."""
        return self.emit(f"call <16 x i8> @llvm.fshl.v16i8(<16 x i8> {v}, <16 x i8> {v}, <16 x i8> {splat(amount)})", "o")

    def inverse(self, x: str) -> str:
        """x^254 = x^-1 (and 0 for 0) by the chain 2, 3, 6, 12, 15, 30, 60, 120, 240, 252, 254."""
        x2 = self.square(x)
        x3 = self.mul(x2, x)
        x6 = self.square(x3)
        x12 = self.square(x6)
        x15 = self.mul(x12, x3)
        x30 = self.square(x15)
        x60 = self.square(x30)
        x120 = self.square(x60)
        x240 = self.square(x120)
        x252 = self.mul(x240, x12)
        return self.mul(x252, x2)

    def sub_bytes(self, v: str) -> str:
        """Forward S-box in every lane."""
        inv = self.inverse(v)
        acc = inv
        for amount in (1, 2, 3, 4):
            acc = self.xor(acc, self.rotate(inv, amount))
        return self.xor(acc, splat(0x63))

    def inv_sub_bytes(self, v: str) -> str:
        """Inverse S-box: inverse affine map, then inversion."""
        acc = self.xor(self.rotate(v, 1), self.rotate(v, 3))
        acc = self.xor(acc, self.rotate(v, 6))
        acc = self.xor(acc, splat(0x05))
        return self.inverse(acc)

    def shift_rows(self, v: str, inverse: bool = False) -> str:
        """Row r of the column-major state rotates by r columns (or back)."""
        step = -1 if inverse else 1
        return self.shuffle(v, [i % 4 + 4 * ((i // 4 + step * (i % 4)) % 4) for i in range(16)])

    def column_rotate(self, v: str, k: int) -> str:
        """Lane 4c + r takes lane 4c + (r + k) mod 4."""
        return self.shuffle(v, [4 * (i // 4) + (i % 4 + k) % 4 for i in range(16)])

    def mix_columns(self, v: str) -> str:
        """b_r = xtime(a_r xor a_r+1) xor a_r+1 xor a_r+2 xor a_r+3 in every column."""
        r1, r2, r3 = (self.column_rotate(v, k) for k in (1, 2, 3))
        return self.xor(self.xor(self.xtime(self.xor(v, r1)), r1), self.xor(r2, r3))

    def inv_mix_columns(self, v: str) -> str:
        """Inverse MixColumns as a doubled-twice correction followed by MixColumns."""
        r2 = self.column_rotate(v, 2)
        return self.mix_columns(self.xor(v, self.xtime(self.xtime(self.xor(v, r2)))))


def prim(name: str, types: list[str], doc: str, bodies: list[tuple[list[str], str]]) -> str:
    """Render a documented primitive with one or more conditional bodies."""
    out = f"  -- | {doc}\n  prim {name} :: " + " -> ".join(types) + " where\n"
    rendered = []
    for lines, condition in bodies:
        unsafe = "unsafe " if any(" asm " in line for line in lines) else ""
        text = "\n".join(f"      {line}" for line in lines)
        rendered.append(f"    {unsafe}llvm {{\n{text}\n    }}" + (f" {condition}" if condition else ""))
    return out + "\n".join(rendered) + "\n"


def round_prim(name: str, doc: str, transform, keyed: bool) -> str:
    """A portable round primitive with little- and big-endian bodies."""
    bodies = []
    for endian in ("little", "big"):
        ir = Ir()
        state = ir.to_vector("%0", endian)
        out = transform(ir, state)
        if keyed:
            out = ir.xor(out, ir.to_vector("%1", endian))
        ir.ret("i128", ir.to_wide(out, endian))
        bodies.append((ir.lines, f"when endian {endian}"))
    return prim(name, ["U128", "U128", "U128"] if keyed else ["U128", "U128"], doc, bodies)


def hardware_block(rounds: int, decrypt: bool) -> str:
    """Whole-block AES-NI and ARMv8 bodies over round-key operands in application order."""
    keys = ROUNDS[rounds]
    x86 = [f"%k{i} = bitcast i128 %{i + 1} to <2 x i64>" for i in range(keys)]
    x86 += ["%s0 = bitcast i128 %0 to <2 x i64>", "%a0 = xor <2 x i64> %s0, %k0"]
    op = "aesdec" if decrypt else "aesenc"
    for i in range(1, keys - 1):
        x86.append(f'%a{i} = call <2 x i64> asm "{op} $2, $0", "=x,0,x"(<2 x i64> %a{i - 1}, <2 x i64> %k{i})')
    x86.append(f'%a{keys - 1} = call <2 x i64> asm "{op}last $2, $0", "=x,0,x"(<2 x i64> %a{keys - 2}, <2 x i64> %k{keys - 1})')
    x86 += [f"%value = bitcast <2 x i64> %a{keys - 1} to i128", "ret i128 %value"]
    arm = [f"%k{i} = bitcast i128 %{i + 1} to <16 x i8>" for i in range(keys)] + ["%a0 = bitcast i128 %0 to <16 x i8>"]
    first, mix = ("aesd", "aesimc") if decrypt else ("aese", "aesmc")
    for i in range(keys - 2):
        arm.append(f'%a{i + 1} = call <16 x i8> asm ".arch_extension aes\\0A{first} $0.16b, $2.16b\\0A{mix} $0.16b, $0.16b", "=w,0,w"(<16 x i8> %a{i}, <16 x i8> %k{i})')
    arm.append(f'%a{keys - 1} = call <16 x i8> asm ".arch_extension aes\\0A{first} $0.16b, $2.16b", "=w,0,w"(<16 x i8> %a{keys - 2}, <16 x i8> %k{keys - 2})')
    arm += [f"%last = xor <16 x i8> %a{keys - 1}, %k{keys - 1}", "%value = bitcast <16 x i8> %last to i128", "ret i128 %value"]
    direction = "Decrypt" if decrypt else "Encrypt"
    keys_doc = "equivalent-inverse round keys" if decrypt else "round keys"
    return prim(f"hardware-{'decrypt' if decrypt else 'encrypt'}{rounds}", ["U128"] * (keys + 2),
                f"{direction} one block with {rounds} rounds of hardware AES over {keys} {keys_doc} in application order. "
                "Callers select this primitive only after hardware detection succeeds; other targets keep the identity body, which is never selected.",
                [(x86, "when arch x86_64"), (arm, "when arch aarch64 when Available ArmAes"), (["ret i128 %0"], "")])


def aes_generated() -> str:
    """Generate portable round primitives, composed block functions and hardware bodies."""
    out = [AES_MARKER.rstrip(), "  -- Regenerate with python3 tooling/generate_aes_kernels.py; do not edit this suffix."]
    for rounds, keys in ROUNDS.items():
        args = " ".join(f"k{i}" for i in range(keys))
        out += [f"  -- | Encrypt one block with {rounds} portable constant-time rounds over round keys k0..k{keys - 1}.",
                f"  portable-encrypt{rounds} :: " + " -> ".join(["U128"] * (keys + 2)),
                f"  portable-encrypt{rounds} block {args} ="]
        expr = "(bit-xor-u128 block k0)"
        for i in range(1, keys - 1):
            expr = f"(aes-encrypt-round {expr} k{i})"
        out += [f"    aes-encrypt-last {expr} k{keys - 1}", ""]
        out += [f"  -- | Decrypt one block with {rounds} portable constant-time inverse rounds over round keys in application order.",
                f"  portable-decrypt{rounds} :: " + " -> ".join(["U128"] * (keys + 2)),
                f"  portable-decrypt{rounds} block {args} ="]
        expr = "(bit-xor-u128 block k0)"
        for i in range(1, keys - 1):
            expr = f"(aes-decrypt-round {expr} k{i})"
        out += [f"    aes-decrypt-last {expr} k{keys - 1}", ""]
    text = "\n".join(out) + "\n"
    text += round_prim("aes-encrypt-round", "One full encryption round: SubBytes, ShiftRows and MixColumns in byte lanes, then AddRoundKey.",
                       lambda ir, v: ir.mix_columns(ir.shift_rows(ir.sub_bytes(v))), True) + "\n"
    text += round_prim("aes-encrypt-last", "The final encryption round, which omits MixColumns.",
                       lambda ir, v: ir.shift_rows(ir.sub_bytes(v)), True) + "\n"
    bodies = []
    for endian in ("little", "big"):
        ir = Ir()
        state = ir.to_vector("%0", endian)
        keyed = ir.xor(ir.inv_sub_bytes(ir.shift_rows(state, True)), ir.to_vector("%1", endian))
        ir.ret("i128", ir.to_wide(ir.inv_mix_columns(keyed), endian))
        bodies.append((ir.lines, f"when endian {endian}"))
    text += prim("aes-decrypt-round", ["U128"] * 3, "One inverse round: InvShiftRows and InvSubBytes in byte lanes, AddRoundKey, then InvMixColumns.", bodies) + "\n"
    text += round_prim("aes-decrypt-last", "The final inverse round, which omits InvMixColumns.",
                       lambda ir, v: ir.inv_sub_bytes(ir.shift_rows(v, True)), True) + "\n"
    text += round_prim("aes-sub-bytes", "Apply the S-box to every byte; key expansion uses the low word.",
                       lambda ir, v: ir.sub_bytes(v), False) + "\n"
    text += round_prim("aes-inverse-mix", "Apply InvMixColumns to a round key, forming the equivalent inverse cipher's keys.",
                       lambda ir, v: ir.inv_mix_columns(v), False) + "\n"
    for rounds in ROUNDS:
        text += hardware_block(rounds, False) + "\n" + hardware_block(rounds, True) + "\n"
    text += prim("hardware-aes-available", ["U32", "U32"],
                 "Report whether hardware AES may execute: CPUID's AES-NI bit on x86-64, ArmAes target evidence on AArch64, and zero elsewhere. "
                 "Detection is feature discovery and never depends on key or message bytes.",
                 [(['%regs = call { i32, i32, i32, i32 } asm "cpuid", "={ax},={bx},={cx},={dx},{ax},{cx}"(i32 1, i32 0)',
                    "%ecx = extractvalue { i32, i32, i32, i32 } %regs, 2", "%bit = and i32 %ecx, 33554432", "ret i32 %bit"], "when arch x86_64"),
                  (["ret i32 1"], "when arch aarch64 when Available ArmAes"), (["ret i32 0"], "")])
    return text


def ghash_portable() -> list[str]:
    """Constant-time GHASH product: masked-integer carry-less multiplication and reflected reduction."""
    ir = Ir()
    lines = ir.lines
    def scalar(expr: str, hint: str = "g") -> str:
        return ir.emit(expr, hint)
    y = scalar("call i128 @llvm.bswap.i128(i128 %0)")
    h = scalar("call i128 @llvm.bswap.i128(i128 %1)")
    y0 = scalar(f"trunc i128 {y} to i64")
    y1 = scalar(f"trunc i128 {scalar(f'lshr i128 {y}, 64')} to i64")
    h0 = scalar(f"trunc i128 {h} to i64")
    h1 = scalar(f"trunc i128 {scalar(f'lshr i128 {h}, 64')} to i64")
    rev = lambda v: scalar(f"call i64 @llvm.bitreverse.i64(i64 {v})", "r")
    masks = [0x1111111111111111, 0x2222222222222222, 0x4444444444444444, 0x8888888888888888]

    def bmul(x: str, z: str) -> str:
        xs = [scalar(f"and i64 {x}, {m}", "a") for m in masks]
        zs = [scalar(f"and i64 {z}, {m}", "b") for m in masks]
        parts = []
        for k in range(4):
            acc = None
            for i in range(4):
                term = scalar(f"mul i64 {xs[i]}, {zs[(k - i) % 4]}", "c")
                acc = term if acc is None else scalar(f"xor i64 {acc}, {term}", "c")
            parts.append(scalar(f"and i64 {acc}, {masks[k]}", "d"))
        return scalar(f"or i64 {scalar(f'or i64 {parts[0]}, {parts[1]}', 'e')}, {scalar(f'or i64 {parts[2]}, {parts[3]}', 'e')}", "e")

    h0r, h1r = rev(h0), rev(h1)
    h2 = scalar(f"xor i64 {h0}, {h1}")
    h2r = scalar(f"xor i64 {h0r}, {h1r}")
    y0r, y1r = rev(y0), rev(y1)
    y2 = scalar(f"xor i64 {y0}, {y1}")
    y2r = scalar(f"xor i64 {y0r}, {y1r}")
    z0, z1, z2 = bmul(y0, h0), bmul(y1, h1), bmul(y2, h2)
    z0h, z1h, z2h = bmul(y0r, h0r), bmul(y1r, h1r), bmul(y2r, h2r)
    z2 = scalar(f"xor i64 {z2}, {scalar(f'xor i64 {z0}, {z1}')}")
    z2h = scalar(f"xor i64 {z2h}, {scalar(f'xor i64 {z0h}, {z1h}')}")
    z0h = scalar(f"lshr i64 {rev(z0h)}, 1")
    z1h = scalar(f"lshr i64 {rev(z1h)}, 1")
    z2h = scalar(f"lshr i64 {rev(z2h)}, 1")
    return reduce(ir, z0, scalar(f"xor i64 {z0h}, {z2}"), scalar(f"xor i64 {z1}, {z2h}"), z1h)


def reduce(ir: Ir, v0: str, v1: str, v2: str, v3: str) -> list[str]:
    """Shift the 255-bit reflected product left by one, reduce modulo x^128 + x^7 + x^2 + x + 1, and return canonical bytes."""
    s = lambda e: ir.emit(e, "n")
    v3 = s(f"or i64 {s(f'shl i64 {v3}, 1')}, {s(f'lshr i64 {v2}, 63')}")
    v2 = s(f"or i64 {s(f'shl i64 {v2}, 1')}, {s(f'lshr i64 {v1}, 63')}")
    v1 = s(f"or i64 {s(f'shl i64 {v1}, 1')}, {s(f'lshr i64 {v0}, 63')}")
    v0 = s(f"shl i64 {v0}, 1")
    def fold(low: str, high: str, upper: str) -> tuple[str, str]:
        a = s(f"xor i64 {low}, {s(f'lshr i64 {low}, 1')}")
        b = s(f"xor i64 {s(f'lshr i64 {low}, 2')}, {s(f'lshr i64 {low}, 7')}")
        upper = s(f"xor i64 {upper}, {s(f'xor i64 {a}, {b}')}")
        c = s(f"xor i64 {s(f'shl i64 {low}, 63')}, {s(f'shl i64 {low}, 62')}")
        high = s(f"xor i64 {high}, {s(f'xor i64 {c}, {s(f'shl i64 {low}, 57')}')}")
        return high, upper
    v1, v2 = fold(v0, v1, v2)
    v2, v3 = fold(v1, v2, v3)
    lo = s(f"zext i64 {v2} to i128")
    hi = s(f"shl i128 {s(f'zext i64 {v3} to i128')}, 64")
    out = s(f"or i128 {lo}, {hi}")
    ir.ret("i128", s(f"call i128 @llvm.bswap.i128(i128 {out})"))
    return ir.lines


def ghash_clmul() -> list[str]:
    """PCLMULQDQ product of the byte-reversed operands followed by the same reflected reduction."""
    ir = Ir()
    s = lambda e, h="g": ir.emit(e, h)
    y = s("call i128 @llvm.bswap.i128(i128 %0)")
    h = s("call i128 @llvm.bswap.i128(i128 %1)")
    yv = s(f"bitcast i128 {y} to <2 x i64>")
    hv = s(f"bitcast i128 {h} to <2 x i64>")
    def clmul(selector: int) -> str:
        return s(f'call <2 x i64> asm "pclmulqdq $$' + str(selector) + f', $2, $0", "=x,0,x"(<2 x i64> {yv}, <2 x i64> {hv})', "q")
    low, high = clmul(0x00), clmul(0x11)
    mid = s(f"xor <2 x i64> {clmul(0x01)}, {clmul(0x10)}")
    lane = lambda v, i: s(f"extractelement <2 x i64> {v}, i64 {i}", "l")
    v0 = lane(low, 0)
    v1 = s(f"xor i64 {lane(low, 1)}, {lane(mid, 0)}")
    v2 = s(f"xor i64 {lane(high, 0)}, {lane(mid, 1)}")
    v3 = lane(high, 1)
    return reduce(ir, v0, v1, v2, v3)


def ghash_generated() -> str:
    """Generate GHASH primitives and carry-less instruction detection."""
    text = GHASH_MARKER.rstrip() + "\n  -- Regenerate with python3 tooling/generate_aes_kernels.py; do not edit this suffix.\n"
    text += prim("ghash-multiply", ["U128", "U128", "U128"],
                 "Multiply two GHASH field elements in canonical byte order without data-dependent branches or memory access.",
                 [(ghash_portable(), "")]) + "\n"
    text += prim("ghash-multiply-clmul", ["U128", "U128", "U128"],
                 "Multiply two GHASH field elements with PCLMULQDQ. Callers select it only after detection succeeds; "
                 "other targets keep the identity body, which is never selected.",
                 [(ghash_clmul(), "when arch x86_64"), (["ret i128 %0"], "")]) + "\n"
    text += prim("clmul-available", ["U32", "U32"],
                 "Report CPUID's PCLMULQDQ bit on x86-64 and zero elsewhere; detection never depends on key or message bytes.",
                 [(['%regs = call { i32, i32, i32, i32 } asm "cpuid", "={ax},={bx},={cx},={dx},{ax},{cx}"(i32 1, i32 0)',
                    "%ecx = extractvalue { i32, i32, i32, i32 } %regs, 2", "%bit = and i32 %ecx, 2", "ret i32 %bit"], "when arch x86_64"),
                  (["ret i32 0"], "")])
    return text


def refresh(path: Path, marker: str, suffix: str, check: bool) -> None:
    """Replace or verify a generated suffix."""
    existing = path.read_text()
    if existing.count(marker) > 1:
        raise SystemExit(f"{path}: duplicate generated markers")
    expected = existing.split(marker, 1)[0].rstrip() + "\n\n" + suffix
    if check:
        if existing != expected:
            raise SystemExit(f"Generated kernels are stale: {path}")
    elif existing != expected:
        path.write_text(expected)


def main() -> None:
    """Write or verify both generated suffixes."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    self_check()
    refresh(ROOT / "seed/prelude/Crypto/Aes.cinch", AES_MARKER, aes_generated(), args.check)
    refresh(ROOT / "seed/prelude/Crypto/Cipher.cinch", GHASH_MARKER, ghash_generated(), args.check)


if __name__ == "__main__":
    main()
