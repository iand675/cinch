#!/usr/bin/env python3
"""Generate pure SHA-256 compression fragments with four-word SIMD schedules.

Small private round primitives compute the two sums needed by each round.
Four-word SIMD schedule primitives return one packed wide value. Generated Cinch
code assembles a complete compression without duplicate round computations.
The fragments have integer inputs, no memory operations and only the checked
byte-swap intrinsic. The checked caller owns input loading and storage.
"""
from __future__ import annotations

import argparse
import math
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
MARKER = "  -- | Generated SHA-256 compression: consume sixteen network-order words without retaining inputs.\n"
MASK32 = (1 << 32) - 1


def primes(count: int) -> list[int]:
    """Return the first count primes in increasing order using exact arithmetic."""
    result = []
    candidate = 2
    while len(result) < count:
        if all(candidate % divisor for divisor in result if divisor * divisor <= candidate):
            result.append(candidate)
        candidate += 1
    return result


def cube_root(value: int) -> int:
    """Return floor(cuberoot(value)) without platform-dependent floating point."""
    low, high = 0, 1 << ((value.bit_length() + 2) // 3)
    while low + 1 < high:
        middle = (low + high) // 2
        if middle * middle * middle <= value:
            low = middle
        else:
            high = middle
    return low


def constants() -> tuple[list[int], list[int], list[int]]:
    """Derive FIPS 180-4 round constants and both initial states from prime roots."""
    values = primes(64)
    rounds = [cube_root(prime << 96) & MASK32 for prime in values]
    initial256 = [math.isqrt(prime << 64) & MASK32 for prime in values[:8]]
    initial224 = [math.isqrt(prime << 128) & MASK32 for prime in values[8:16]]
    return rounds, initial256, initial224


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
        """Emit a defined constant rotate in a scalar or four independent 32-bit lanes."""
        ty = "<4 x i32>" if vector else "i32"
        shift = lambda n: "<" + ", ".join(f"i32 {n}" for _ in range(4)) + ">" if vector else str(n)
        right = self.emit(name + "r", f"lshr {ty} {value}, {shift(amount)}")
        left = self.emit(name + "l", f"shl {ty} {value}, {shift(32 - amount)}")
        return self.emit(name, f"or {ty} {right}, {left}")

    def sigma(self, name: str, value: str, amounts: tuple[int, int, int], vector: bool = False, small: bool = False) -> str:
        """Emit the schedule or round sigma with exact unsigned 32-bit wrapping."""
        ty = "<4 x i32>" if vector else "i32"
        a = self.rotate(name + "a", value, amounts[0], vector)
        b = self.rotate(name + "b", value, amounts[1], vector)
        if small:
            count = "<" + ", ".join(f"i32 {amounts[2]}" for _ in range(4)) + ">" if vector else str(amounts[2])
            c = self.emit(name + "c", f"lshr {ty} {value}, {count}")
        else:
            c = self.rotate(name + "c", value, amounts[2], vector)
        pair = self.emit(name + "ab", f"xor {ty} {a}, {b}")
        return self.emit(name, f"xor {ty} {pair}, {c}")

    def declaration(self, name: str, types: list[str], doc: str, endian: str | None = None) -> str:
        """Wrap one checked private LLVM body in a canonically documented declaration."""
        return f"  -- | {doc}\n  prim {name} :: " + " -> ".join(types) + " where\n" + self.body(endian)

    def body(self, endian: str | None = None) -> str:
        """Render the complete body with an optional explicit endian selection."""
        return "    llvm {\n" + "\n".join(self.lines) + "\n    }" + (f" when endian {endian}" if endian else "") + "\n"


def round_primitives() -> str:
    """Emit small scalar round sums that fit ordinary backend inlining budgets."""
    ir = Instructions()
    s1 = ir.sigma("s1", "%0", (6, 11, 25))
    ef = ir.emit("ef", "and i32 %0, %1")
    inverse = ir.emit("not", "xor i32 %0, -1")
    ng = ir.emit("ng", f"and i32 {inverse}, %2")
    choice = ir.emit("ch", f"xor i32 {ef}, {ng}")
    t1 = ir.emit("t1a", f"add i32 %3, {s1}")
    t1 = ir.emit("t1b", f"add i32 {t1}, {choice}")
    t1 = ir.emit("t1c", f"add i32 {t1}, %4")
    t1 = ir.emit("value", f"add i32 {t1}, %5")
    ir.lines.append(f"      ret i32 {t1}")
    first = ir.declaration("sha256-t1", ["U32"] * 7, "Compute the choice, upper sigma and message contribution to one ordered round.")
    ir = Instructions()
    s0 = ir.sigma("s0", "%0", (2, 13, 22))
    ab = ir.emit("ab", "and i32 %0, %1")
    ac = ir.emit("ac", "and i32 %0, %2")
    bc = ir.emit("bc", "and i32 %1, %2")
    pair = ir.emit("pair", f"xor i32 {ab}, {ac}")
    majority = ir.emit("maj", f"xor i32 {pair}, {bc}")
    t2 = ir.emit("value", f"add i32 {s0}, {majority}")
    ir.lines.append(f"      ret i32 {t2}")
    return first + "\n" + ir.declaration("sha256-t2", ["U32"] * 4, "Compute the majority and lower sigma contribution to one ordered round.")


def schedule_body(endian: str) -> Instructions:
    """Expand four schedule words, resolving the two dependent upper lanes in a second step."""
    ir = Instructions()
    groups = []
    for index in range(4):
        vector = ir.emit(f"v{index}", f"bitcast i128 %{index} to <4 x i32>")
        if endian == "big":
            vector = ir.emit(f"g{index}", f"shufflevector <4 x i32> {vector}, <4 x i32> zeroinitializer, <4 x i32> <i32 3, i32 2, i32 1, i32 0>")
        groups.append(vector)
    a = ir.emit("a", f"shufflevector <4 x i32> {groups[0]}, <4 x i32> {groups[1]}, <4 x i32> <i32 1, i32 2, i32 3, i32 4>")
    b = ir.emit("b", f"shufflevector <4 x i32> {groups[2]}, <4 x i32> {groups[3]}, <4 x i32> <i32 1, i32 2, i32 3, i32 4>")
    c = ir.emit("c", f"shufflevector <4 x i32> {groups[3]}, <4 x i32> zeroinitializer, <4 x i32> <i32 2, i32 3, i32 4, i32 4>")
    s0 = ir.sigma("s0", a, (7, 18, 3), True, True)
    s1 = ir.sigma("s1", c, (17, 19, 10), True, True)
    base = ir.emit("base0", f"add <4 x i32> {groups[0]}, {b}")
    base = ir.emit("base1", f"add <4 x i32> {base}, {s0}")
    first = ir.emit("first", f"add <4 x i32> {base}, {s1}")
    next_pair = ir.emit("next", f"shufflevector <4 x i32> {first}, <4 x i32> zeroinitializer, <4 x i32> <i32 4, i32 4, i32 0, i32 1>")
    s2 = ir.sigma("s2", next_pair, (17, 19, 10), True, True)
    complete = ir.emit("complete", f"add <4 x i32> {first}, {s2}")
    if endian == "big":
        complete = ir.emit("host", f"shufflevector <4 x i32> {complete}, <4 x i32> zeroinitializer, <4 x i32> <i32 3, i32 2, i32 1, i32 0>")
    value = ir.emit("value", f"bitcast <4 x i32> {complete} to i128")
    ir.lines.append(f"      ret i128 {value}")
    return ir


def helpers() -> str:
    """Return pure scalar lane access, word packing and endian-independent input grouping."""
    output = []
    for high in [False, True]:
        ir = Instructions()
        source = ir.emit("shift", "lshr i64 %0, 32") if high else "%0"
        value = ir.emit("value", f"trunc i64 {source} to i32")
        ir.lines.append(f"      ret i32 {value}")
        output.append(ir.declaration("sha256-word-" + ("high" if high else "low"), ["U64", "U32"], "Extract one complete U32 lane from a stored pair of chaining words."))
    for lane in range(4):
        ir = Instructions()
        source = ir.emit("shift", f"lshr i128 %0, {lane * 32}") if lane else "%0"
        value = ir.emit("value", f"trunc i128 {source} to i32")
        ir.lines.append(f"      ret i32 {value}")
        output.append(ir.declaration(f"sha256-lane{lane}", ["U128", "U32"], f"Select schedule lane {lane} without narrowing any other lane's stored bits."))
    ir = Instructions()
    low = ir.emit("low", "zext i32 %0 to i64")
    high = ir.emit("high", "zext i32 %1 to i64")
    upper = ir.emit("upper", f"shl i64 {high}, 32")
    value = ir.emit("value", f"or i64 {low}, {upper}")
    ir.lines.append(f"      ret i64 {value}")
    output.append(ir.declaration("sha256-pair", ["U32", "U32", "U64"], "Pack two chaining words with the first word in the low 32 bits."))
    ir = Instructions()
    halves = []
    for index in range(2):
        swapped = ir.emit(f"swap{index}", f"call i64 @llvm.bswap.i64(i64 %{index})")
        a = ir.emit(f"left{index}", f"shl i64 {swapped}, 32")
        b = ir.emit(f"right{index}", f"lshr i64 {swapped}, 32")
        value = ir.emit(f"half{index}", f"or i64 {a}, {b}")
        halves.append(ir.emit(f"wide{index}", f"zext i64 {value} to i128"))
    upper = ir.emit("upper", f"shl i128 {halves[1]}, 64")
    value = ir.emit("value", f"or i128 {halves[0]}, {upper}")
    ir.lines.append(f"      ret i128 {value}")
    output.append(ir.declaration("sha256-input-group", ["U64", "U64", "U128"], "Decode four network-order words from two packed little-endian input byte words."))
    ir = Instructions()
    a0 = ir.emit("a0", "trunc i64 %0 to i32")
    a1 = ir.emit("as", "lshr i64 %0, 32")
    a1 = ir.emit("a1", f"trunc i64 {a1} to i32")
    b0 = ir.emit("b0", "trunc i64 %1 to i32")
    b1 = ir.emit("bs", "lshr i64 %1, 32")
    b1 = ir.emit("b1", f"trunc i64 {b1} to i32")
    lo = ir.emit("lo", f"add i32 {a0}, {b0}")
    hi = ir.emit("hi", f"add i32 {a1}, {b1}")
    lo = ir.emit("low", f"zext i32 {lo} to i64")
    hi = ir.emit("high", f"zext i32 {hi} to i64")
    hi = ir.emit("upper", f"shl i64 {hi}, 32")
    value = ir.emit("value", f"or i64 {lo}, {hi}")
    ir.lines.append(f"      ret i64 {value}")
    output.append(ir.declaration("sha256-feed-pair", ["U64", "U64", "U64"], "Feed forward two chaining words with independent U32 wrapping rather than a cross-lane carry."))
    return "\n".join(output)


def generated() -> str:
    """Generate bounded-size checked helpers with one evaluation of every ordered round."""
    lines = [MARKER.rstrip(), "  -- Regenerate with python3 tooling/generate_sha256_kernels.py; do not edit this suffix.",
             "  sha256-compress :: Sha256Words -> " + " -> ".join(["U64"] * 8 + ["Sha256Words"]),
             "  sha256-compress prior w0 w1 w2 w3 w4 w5 w6 w7 = case prior of",
             "    ShaWords ab cd ef gh ->"]
    for group in range(4):
        lines.append(f"      let g{group} = sha256-input-group w{group * 2} w{group * 2 + 1} in")
    for group in range(4, 16):
        lines.append(f"      let g{group} = sha256-schedule g{group - 4} g{group - 3} g{group - 2} g{group - 1} in")
    rounds = constants()[0]
    for phase, letter in enumerate("abcd"):
        previous = "(ShaWords ab cd ef gh)" if phase == 0 else f"s{phase - 1}"
        words = " ".join(f"(sha256-lane{index % 4} g{index // 4})" for index in range(phase * 16, phase * 16 + 16))
        lines.append(f"      let s{phase} = sha256-rounds-{letter} {previous} {words} in")
    lines += ["      case s3 of", "        ShaWords last-ab last-cd last-ef last-gh ->",
              "          ShaWords (sha256-feed-pair ab last-ab) (sha256-feed-pair cd last-cd)",
              "            (sha256-feed-pair ef last-ef) (sha256-feed-pair gh last-gh)", ""]
    for phase, letter in enumerate("abcd"):
        lines += ["  -- | Advance sixteen ordered rounds over scalar message words, preserving the prior chaining value.",
                  f"  sha256-rounds-{letter} :: Sha256Words -> " + " -> ".join(["U32"] * 16 + ["Sha256Words"]),
                  f"  sha256-rounds-{letter} prior " + " ".join(f"m{index}" for index in range(16)) + " = case prior of",
                  "    ShaWords ab cd ef gh ->"]
        initial = []
        for pair in ["ab", "cd", "ef", "gh"]:
            for half in ["low", "high"]:
                name = f"initial{len(initial)}"
                initial.append(name)
                lines.append(f"      let {name} = sha256-word-{half} {pair} in")
        state = initial[:]
        for index in range(16):
            a, b, c, d, e, f, g, h = state
            key = rounds[phase * 16 + index]
            lines += [f"      let first{index} = sha256-t1 {e} {f} {g} {h} m{index} {key} in",
                      f"      let second{index} = sha256-t2 {a} {b} {c} in",
                      f"      let a{index} = wrapping-add-u32 first{index} second{index} in",
                      f"      let e{index} = wrapping-add-u32 {d} first{index} in"]
            state = [f"a{index}", a, b, c, f"e{index}", e, f, g]
        pairs = [f"(sha256-pair {state[index * 2]} {state[index * 2 + 1]})" for index in range(4)]
        lines += ["      ShaWords " + " ".join(pairs), ""]
    lines += ["  -- | Initial chaining words derived from the exact FIPS prime-root definitions.",
              "  sha256-initial-words :: Bool -> Sha256Words",
              "  sha256-initial-words short ="]
    _, initial256, initial224 = constants()
    pairs = lambda words: " ".join(str(words[index * 2] | (words[index * 2 + 1] << 32)) for index in range(4))
    lines += [f"    if short then ShaWords {pairs(initial224)}", f"    else ShaWords {pairs(initial256)}", ""]
    schedule=schedule_body("little").declaration("sha256-schedule", ["U128"] * 5,
        "Expand four schedule lanes in SIMD, completing dependent lanes after the first two are available.", "little")
    schedule+=schedule_body("big").body("big")
    return "\n".join(lines) + "\n" + helpers() + "\n" + round_primitives() + "\n" + schedule


def main() -> None:
    """Write or verify the generated suffix while preserving the handwritten checked public API."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--module", type=Path, default=ROOT / "compiler/prelude/Text/Sha256.cinch")
    parser.add_argument("--fragment", type=Path, help="Write generated declarations for an isolated kernel probe")
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    target = args.fragment or args.module
    existing = target.read_text() if target.exists() else ""
    if args.fragment:
        expected = generated()
    else:
        if not existing:
            parser.error("the handwritten module must exist before generating its private core")
        if existing.count(MARKER) > 1:
            parser.error("the module contains duplicate generated-core markers")
        if MARKER not in existing and "  sha256-compress ::" in existing:
            parser.error("a compression core exists without its generation marker; refusing to append another")
        expected = existing.split(MARKER, 1)[0].rstrip() + "\n\n" + generated()
    if args.check:
        if existing != expected:
            parser.exit(1, f"Generated SHA-256 core is stale: {target}\n")
    elif existing != expected:
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_text(expected)


if __name__ == "__main__":
    main()
