#!/usr/bin/env python3
"""Generate pure SHA-512 compression fragments with two-lane SIMD message schedules.

Each round is one private primitive that returns the two new working words
(`a` and `e`) packed into a U128, so every ordered round is evaluated once and
the remaining six words are renamed in generated Cinch code. The schedule
expands two 64-bit words per primitive in `<2 x i64>` lanes: for an even index
t, words t and t+1 depend only on words below t, so both lanes are independent.
Fragments are straight-line, perform no memory operations, and call only the
checked funnel-shift intrinsic. Constants are derived exactly from prime roots,
and the truncated-variant initial values from the FIPS 180-4 generation function.
"""
from __future__ import annotations

import argparse
import math
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
MARKER = "  -- | Generated SHA-512 compression: consume sixteen network-order words without retaining inputs.\n"
MASK64 = (1 << 64) - 1


def primes(count: int) -> list[int]:
    """Return the first count primes in increasing order using exact arithmetic."""
    result: list[int] = []
    candidate = 2
    while len(result) < count:
        if all(candidate % divisor for divisor in result if divisor * divisor <= candidate):
            result.append(candidate)
        candidate += 1
    return result


def cube_root(value: int) -> int:
    """Return floor(cuberoot(value)) without platform-dependent floating point."""
    low, high = 0, 1 << ((value.bit_length() + 2) // 3 + 1)
    while low + 1 < high:
        middle = (low + high) // 2
        if middle * middle * middle <= value:
            low = middle
        else:
            high = middle
    return low


def round_constants() -> list[int]:
    """Return the eighty 64-bit fractional cube roots of the first eighty primes."""
    return [cube_root(prime << 192) & MASK64 for prime in primes(80)]


def sqrt_words(selected: list[int]) -> list[int]:
    """Return 64-bit fractional square roots of the selected primes."""
    return [math.isqrt(prime << 128) & MASK64 for prime in selected]


def rotr(value: int, amount: int) -> int:
    """Rotate a 64-bit word right by a constant amount."""
    return ((value >> amount) | (value << (64 - amount))) & MASK64


def compress(state: list[int], block: bytes) -> list[int]:
    """Reference compression used only to derive the truncated-variant initial values."""
    k = round_constants()
    w = [int.from_bytes(block[i * 8:i * 8 + 8], "big") for i in range(16)]
    for t in range(16, 80):
        s0 = rotr(w[t - 15], 1) ^ rotr(w[t - 15], 8) ^ (w[t - 15] >> 7)
        s1 = rotr(w[t - 2], 19) ^ rotr(w[t - 2], 61) ^ (w[t - 2] >> 6)
        w.append((w[t - 16] + s0 + w[t - 7] + s1) & MASK64)
    a, b, c, d, e, f, g, h = state
    for t in range(80):
        t1 = (h + (rotr(e, 14) ^ rotr(e, 18) ^ rotr(e, 41)) + ((e & f) ^ (~e & g)) + k[t] + w[t]) & MASK64
        t2 = ((rotr(a, 28) ^ rotr(a, 34) ^ rotr(a, 39)) + ((a & b) ^ (a & c) ^ (b & c))) & MASK64
        a, b, c, d, e, f, g, h = (t1 + t2) & MASK64, a, b, c, (d + t1) & MASK64, e, f, g
    return [(x + y) & MASK64 for x, y in zip(state, [a, b, c, d, e, f, g, h])]


def digest(state: list[int], message: bytes) -> list[int]:
    """Hash a short message from an explicit initial state with standard padding."""
    padded = message + b"\x80" + b"\x00" * ((111 - len(message)) % 128) + (len(message) * 8).to_bytes(16, "big")
    for offset in range(0, len(padded), 128):
        state = compress(state, padded[offset:offset + 128])
    return state


def initial_values() -> dict[str, list[int]]:
    """Derive all four standard initial chaining values."""
    values = primes(16)
    full = sqrt_words(values[:8])
    generator = [word ^ 0xA5A5A5A5A5A5A5A5 for word in full]
    return {
        "512": full,
        "384": sqrt_words(values[8:16]),
        "512/224": digest(generator, b"SHA-512/224"),
        "512/256": digest(generator, b"SHA-512/256"),
    }


class Instructions:
    """Build deterministic straight-line SSA with uniquely named scalar and vector operations."""

    def __init__(self) -> None:
        """Start an empty fragment with no module-level state."""
        self.lines: list[str] = []

    def emit(self, name: str, expression: str) -> str:
        """Append a named instruction and return its SSA reference."""
        self.lines.append(f"      %{name} = {expression}")
        return f"%{name}"

    def rotate(self, name: str, value: str, amount: int, vector: bool = False) -> str:
        """Rotate right with the funnel-shift intrinsic, which lowers to one native rotate."""
        if vector:
            return self.emit(name, f"call <2 x i64> @llvm.fshr.v2i64(<2 x i64> {value}, <2 x i64> {value}, <2 x i64> <i64 {amount}, i64 {amount}>)")
        return self.emit(name, f"call i64 @llvm.fshr.i64(i64 {value}, i64 {value}, i64 {amount})")

    def sigma(self, name: str, value: str, amounts: tuple[int, int, int], vector: bool = False, small: bool = False) -> str:
        """Emit an upper (three rotations) or schedule (two rotations and a shift) sigma."""
        ty = "<2 x i64>" if vector else "i64"
        a = self.rotate(name + "a", value, amounts[0], vector)
        b = self.rotate(name + "b", value, amounts[1], vector)
        if small:
            count = f"<i64 {amounts[2]}, i64 {amounts[2]}>" if vector else str(amounts[2])
            c = self.emit(name + "c", f"lshr {ty} {value}, {count}")
        else:
            c = self.rotate(name + "c", value, amounts[2], vector)
        pair = self.emit(name + "ab", f"xor {ty} {a}, {b}")
        return self.emit(name, f"xor {ty} {pair}, {c}")

    def lanes(self, name: str, wide: str) -> str:
        """Split a U128 into a two-lane vector with the low half in lane 0, independent of byte order."""
        low = self.emit(name + "lo", f"trunc i128 {wide} to i64")
        shifted = self.emit(name + "s", f"lshr i128 {wide}, 64")
        high = self.emit(name + "hi", f"trunc i128 {shifted} to i64")
        first = self.emit(name + "v0", f"insertelement <2 x i64> poison, i64 {low}, i64 0")
        return self.emit(name, f"insertelement <2 x i64> {first}, i64 {high}, i64 1")

    def pack(self, name: str, low: str, high: str) -> str:
        """Pack two scalar words into a U128 with the first word in the low half."""
        wl = self.emit(name + "l", f"zext i64 {low} to i128")
        wh = self.emit(name + "h", f"zext i64 {high} to i128")
        up = self.emit(name + "u", f"shl i128 {wh}, 64")
        return self.emit(name, f"or i128 {wl}, {up}")

    def declaration(self, name: str, types: list[str], doc: str) -> str:
        """Wrap one checked private LLVM body in a canonically documented declaration."""
        return f"  -- | {doc}\n  prim {name} :: " + " -> ".join(types) + " where\n" + "    llvm {\n" + "\n".join(self.lines) + "\n    }\n"


def round_primitive() -> str:
    """Emit one complete ordered round returning the new a (low) and e (high) words."""
    ir = Instructions()
    s1 = ir.sigma("s1", "%4", (14, 18, 41))
    ef = ir.emit("ef", "and i64 %4, %5")
    inverse = ir.emit("not", "xor i64 %4, -1")
    ng = ir.emit("ng", f"and i64 {inverse}, %6")
    choice = ir.emit("ch", f"xor i64 {ef}, {ng}")
    t1 = ir.emit("t1a", f"add i64 %7, {s1}")
    t1 = ir.emit("t1b", f"add i64 {t1}, {choice}")
    t1 = ir.emit("t1c", f"add i64 {t1}, %8")
    t1 = ir.emit("t1", f"add i64 {t1}, %9")
    s0 = ir.sigma("s0", "%0", (28, 34, 39))
    ab = ir.emit("ab", "and i64 %0, %1")
    ac = ir.emit("ac", "and i64 %0, %2")
    bc = ir.emit("bc", "and i64 %1, %2")
    pair = ir.emit("pair", f"xor i64 {ab}, {ac}")
    majority = ir.emit("maj", f"xor i64 {pair}, {bc}")
    t2 = ir.emit("t2", f"add i64 {s0}, {majority}")
    a = ir.emit("na", f"add i64 {t1}, {t2}")
    e = ir.emit("ne", f"add i64 %3, {t1}")
    value = ir.pack("value", a, e)
    ir.lines.append(f"      ret i128 {value}")
    return ir.declaration("sha512-round", ["U64"] * 10 + ["U128"],
                          "Evaluate one ordered round over a..h, the message word and the round constant; return new a low and new e high.")


def schedule_primitive() -> str:
    """Expand words t and t+1 from pairs (t-16,t-15), (t-14,t-13), (t-8,t-7), (t-6,t-5) and (t-2,t-1)."""
    ir = Instructions()
    p16 = ir.lanes("p16", "%0")
    p14 = ir.lanes("p14", "%1")
    p8 = ir.lanes("p8", "%2")
    p6 = ir.lanes("p6", "%3")
    p2 = ir.lanes("p2", "%4")
    w15 = ir.emit("w15", f"shufflevector <2 x i64> {p16}, <2 x i64> {p14}, <2 x i32> <i32 1, i32 2>")
    w7 = ir.emit("w7", f"shufflevector <2 x i64> {p8}, <2 x i64> {p6}, <2 x i32> <i32 1, i32 2>")
    s0 = ir.sigma("s0", w15, (1, 8, 7), True, True)
    s1 = ir.sigma("s1", p2, (19, 61, 6), True, True)
    base = ir.emit("base0", f"add <2 x i64> {p16}, {s0}")
    base = ir.emit("base1", f"add <2 x i64> {base}, {w7}")
    complete = ir.emit("complete", f"add <2 x i64> {base}, {s1}")
    low = ir.emit("low", f"extractelement <2 x i64> {complete}, i64 0")
    high = ir.emit("high", f"extractelement <2 x i64> {complete}, i64 1")
    value = ir.pack("value", low, high)
    ir.lines.append(f"      ret i128 {value}")
    return ir.declaration("sha512-schedule", ["U128"] * 6,
                          "Expand two schedule words in independent SIMD lanes from five earlier word pairs.")


def helpers() -> str:
    """Return pure lane access and pair packing helpers."""
    output = []
    for lane in range(2):
        ir = Instructions()
        source = ir.emit("shift", "lshr i128 %0, 64") if lane else "%0"
        value = ir.emit("value", f"trunc i128 {source} to i64")
        ir.lines.append(f"      ret i64 {value}")
        output.append(ir.declaration(f"sha512-lane{lane}", ["U128", "U64"], f"Select word {lane} of a packed pair without narrowing the other word's bits."))
    ir = Instructions()
    value = ir.pack("value", "%0", "%1")
    ir.lines.append(f"      ret i128 {value}")
    output.append(ir.declaration("sha512-group", ["U64", "U64", "U128"], "Pack two consecutive message words with the first word in the low half."))
    return "\n".join(output)


def generated() -> str:
    """Generate the compression, its five sixteen-round phases, initial values and private primitives."""
    lines = [MARKER.rstrip(), "  -- Regenerate with python3 tooling/generate_sha512_kernels.py; do not edit this suffix.",
             "  sha512-compress :: Sha512Words -> " + " -> ".join(["U64"] * 16 + ["Sha512Words"]),
             "  sha512-compress prior " + " ".join(f"w{i}" for i in range(16)) + " = case prior of",
             "    Words512 a b c d e f g h ->"]
    for group in range(8):
        lines.append(f"      let g{group} = sha512-group w{group * 2} w{group * 2 + 1} in")
    for j in range(8, 40):
        lines.append(f"      let g{j} = sha512-schedule g{j - 8} g{j - 7} g{j - 4} g{j - 3} g{j - 1} in")
    for phase, letter in enumerate("abcde"):
        previous = "prior" if phase == 0 else f"s{phase - 1}"
        words = " ".join(f"(sha512-lane{index % 2} g{index // 2})" for index in range(phase * 16, phase * 16 + 16))
        lines.append(f"      let s{phase} = sha512-rounds-{letter} {previous} {words} in")
    lines += ["      case s4 of",
              "        Words512 na nb nc nd ne nf ng nh ->",
              "          Words512 (wrapping-add-u64 a na) (wrapping-add-u64 b nb) (wrapping-add-u64 c nc) (wrapping-add-u64 d nd)",
              "            (wrapping-add-u64 e ne) (wrapping-add-u64 f nf) (wrapping-add-u64 g ng) (wrapping-add-u64 h nh)", ""]
    constants = round_constants()
    for phase, letter in enumerate("abcde"):
        lines += ["  -- | Advance sixteen ordered rounds over scalar message words, preserving the prior working value.",
                  f"  sha512-rounds-{letter} :: Sha512Words -> " + " -> ".join(["U64"] * 16 + ["Sha512Words"]),
                  f"  sha512-rounds-{letter} prior " + " ".join(f"m{index}" for index in range(16)) + " = case prior of",
                  "    Words512 i0 i1 i2 i3 i4 i5 i6 i7 ->"]
        state = [f"i{index}" for index in range(8)]
        for index in range(16):
            a, b, c, d, e, f, g, h = state
            key = constants[phase * 16 + index]
            lines += [f"      let r{index} = sha512-round {a} {b} {c} {d} {e} {f} {g} {h} m{index} {key} in",
                      f"      let a{index} = sha512-lane0 r{index} in",
                      f"      let e{index} = sha512-lane1 r{index} in"]
            state = [f"a{index}", a, b, c, f"e{index}", e, f, g]
        lines += ["      Words512 " + " ".join(state), ""]
    iv = initial_values()
    lines += ["  -- | Initial chaining words for a checkpoint variant code, derived exactly from the standard definitions.",
              "  -- Code 4 is SHA-384, 5 is SHA-512/224, 6 is SHA-512/256 and every other code is SHA-512.",
              "  sha512-initial-words :: Nat -> Sha512Words",
              "  sha512-initial-words variant ="]
    words = lambda name: "Words512 " + " ".join(str(word) for word in iv[name])
    lines += [f"    if variant == 4 then {words('384')}",
              f"    else if variant == 5 then {words('512/224')}",
              f"    else if variant == 6 then {words('512/256')}",
              f"    else {words('512')}", ""]
    return "\n".join(lines) + "\n" + helpers() + "\n" + round_primitive() + "\n" + schedule_primitive()


def main() -> None:
    """Write or verify the generated suffix while preserving the handwritten checked public API."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--module", type=Path, default=ROOT / "seed/prelude/Text/Sha512.cinch")
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    existing = args.module.read_text() if args.module.exists() else ""
    if not existing:
        parser.error("the handwritten module must exist before generating its private core")
    if existing.count(MARKER) > 1:
        parser.error("the module contains duplicate generated-core markers")
    if MARKER not in existing and "  sha512-compress ::" in existing:
        parser.error("a compression core exists without its generation marker; refusing to append another")
    expected = existing.split(MARKER, 1)[0].rstrip() + "\n\n" + generated()
    if args.check:
        if existing != expected:
            parser.exit(1, f"Generated SHA-512 core is stale: {args.module}\n")
    elif existing != expected:
        args.module.write_text(expected)


if __name__ == "__main__":
    main()
