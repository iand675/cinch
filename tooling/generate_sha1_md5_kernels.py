#!/usr/bin/env python3
"""Generate pure SHA-1 and MD5 compression fragments.

SHA-1 expands its message schedule four words at a time in `<4 x i32>` lanes.
The fourth lane of each group depends on the first lane's result, and because
a one-bit rotation distributes over exclusive-or, that lane is completed by
exclusive-or with the rotated first result instead of a scalar recomputation.
Each SHA-1 round is one primitive returning the new `a` word and the rotated
old `b` word packed into a U64. MD5 rounds are inherently sequential; each is
one primitive, and the per-round rotation amount folds to a constant after
inlining. Fragments are straight-line and perform no memory operations.
"""
from __future__ import annotations

import argparse
import math
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SHA1_MARKER = "  -- | Generated SHA-1 compression: consume sixteen network-order words without retaining inputs.\n"
MD5_MARKER = "  -- | Generated MD5 compression: consume sixteen little-endian words without retaining inputs.\n"
MASK32 = (1 << 32) - 1


def emit_prim(name: str, types: list[str], doc: str, lines: list[str]) -> str:
    """Render one documented private LLVM primitive."""
    body = "\n".join(f"      {line}" for line in lines)
    return f"  -- | {doc}\n  prim {name} :: " + " -> ".join(types) + " where\n    llvm {\n" + body + "\n    }\n"


def sha1_constants() -> list[int]:
    """The four SHA-1 round constants: floor(2^30 * sqrt(n)) for n = 2, 3, 5, 10."""
    return [math.isqrt(n << 60) for n in (2, 3, 5, 10)]


def sha1_generated() -> str:
    """Generate SHA-1 compression in five twenty-round-or-smaller phases plus private primitives."""
    lines = [SHA1_MARKER.rstrip(), "  -- Regenerate with python3 tooling/generate_sha1_md5_kernels.py; do not edit this suffix.",
             "  sha1-compress :: Sha1Words -> " + " -> ".join(["U64"] * 8 + ["Sha1Words"]),
             "  sha1-compress prior " + " ".join(f"w{i}" for i in range(8)) + " = case prior of",
             "    Words160 a b c d e ->"]
    for group in range(4):
        lines.append(f"      let g{group} = sha1-input-group w{group * 2} w{group * 2 + 1} in")
    for group in range(4, 20):
        lines.append(f"      let g{group} = sha1-schedule g{group - 4} g{group - 3} g{group - 2} g{group - 1} in")
    phases = [(0, 20, "a"), (20, 40, "b"), (40, 60, "c"), (60, 80, "d")]
    for index, (first, last, letter) in enumerate(phases):
        previous = "prior" if index == 0 else f"s{index - 1}"
        words = " ".join(f"(sha1-lane{t % 4} g{t // 4})" for t in range(first, last))
        lines.append(f"      let s{index} = sha1-rounds-{letter} {previous} {words} in")
    lines += ["      case s3 of",
              "        Words160 na nb nc nd ne ->",
              "          Words160 (wrapping-add-u32 a na) (wrapping-add-u32 b nb) (wrapping-add-u32 c nc) (wrapping-add-u32 d nd) (wrapping-add-u32 e ne)", ""]
    functions = ["choose", "parity", "majority", "parity"]
    constants = sha1_constants()
    for index, (first, last, letter) in enumerate(phases):
        count = last - first
        lines += [f"  -- | Advance twenty ordered rounds using the {functions[index]} function, preserving the prior working value.",
                  f"  sha1-rounds-{letter} :: Sha1Words -> " + " -> ".join(["U32"] * count + ["Sha1Words"]),
                  f"  sha1-rounds-{letter} prior " + " ".join(f"m{i}" for i in range(count)) + " = case prior of",
                  "    Words160 i0 i1 i2 i3 i4 ->"]
        a, b, c, d, e = "i0", "i1", "i2", "i3", "i4"
        for i in range(count):
            lines += [f"      let r{i} = sha1-{functions[index]} {a} {b} {c} {d} {e} m{i} {constants[index]} in",
                      f"      let a{i} = sha1-low r{i} in",
                      f"      let b{i} = sha1-high r{i} in"]
            a, b, c, d, e = f"a{i}", a, f"b{i}", c, d
        lines += [f"      Words160 {a} {b} {c} {d} {e}", ""]
    output = "\n".join(lines) + "\n"
    rounds = {
        "choose": ["%bc = and i32 %1, %2", "%nb = xor i32 %1, -1", "%nd = and i32 %nb, %3", "%f = xor i32 %bc, %nd"],
        "parity": ["%bc = xor i32 %1, %2", "%f = xor i32 %bc, %3"],
        "majority": ["%bc = and i32 %1, %2", "%bd = and i32 %1, %3", "%cd = and i32 %2, %3", "%x = or i32 %bc, %bd", "%f = or i32 %x, %cd"],
    }
    for name, body in rounds.items():
        output += emit_prim(f"sha1-{name}", ["U32"] * 7 + ["U64"],
                            f"One ordered round with the {name} function over a..e, a message word and the round constant; return new a low and rotated b high.",
                            body + ["%ra = call i32 @llvm.fshl.i32(i32 %0, i32 %0, i32 5)", "%s0 = add i32 %ra, %f", "%s1 = add i32 %s0, %4",
                                    "%s2 = add i32 %s1, %5", "%na = add i32 %s2, %6", "%rb = call i32 @llvm.fshl.i32(i32 %1, i32 %1, i32 30)",
                                    "%lo = zext i32 %na to i64", "%hi0 = zext i32 %rb to i64", "%hi = shl i64 %hi0, 32", "%value = or i64 %lo, %hi", "ret i64 %value"]) + "\n"
    output += emit_prim("sha1-low", ["U64", "U32"], "Select the new working word a from a packed round result.", ["%value = trunc i64 %0 to i32", "ret i32 %value"]) + "\n"
    output += emit_prim("sha1-high", ["U64", "U32"], "Select the rotated former working word b from a packed round result.", ["%shift = lshr i64 %0, 32", "%value = trunc i64 %shift to i32", "ret i32 %value"]) + "\n"
    for lane in range(4):
        body = ([f"%shift = lshr i128 %0, {lane * 32}", "%value = trunc i128 %shift to i32"] if lane else ["%value = trunc i128 %0 to i32"]) + ["ret i32 %value"]
        output += emit_prim(f"sha1-lane{lane}", ["U128", "U32"], f"Select schedule word {lane} of a packed group of four.", body) + "\n"
    output += emit_prim("sha1-input-group", ["U64", "U64", "U128"], "Pack four network-order words from two words of two each, first word in the lowest lane.",
                        ["%l0 = trunc i64 %0 to i32", "%s0 = lshr i64 %0, 32", "%h0 = trunc i64 %s0 to i32", "%l1 = trunc i64 %1 to i32", "%s1 = lshr i64 %1, 32", "%h1 = trunc i64 %s1 to i32",
                         "%v0 = insertelement <4 x i32> poison, i32 %h0, i64 0", "%v1 = insertelement <4 x i32> %v0, i32 %l0, i64 1",
                         "%v2 = insertelement <4 x i32> %v1, i32 %h1, i64 2", "%v3 = insertelement <4 x i32> %v2, i32 %l1, i64 3"] + pack_vector("%v3")) + "\n"
    schedule = []
    for index in range(4):
        schedule += unpack_vector(f"%{index}", f"g{index}")
    schedule += ["%w3 = shufflevector <4 x i32> %g3, <4 x i32> zeroinitializer, <4 x i32> <i32 1, i32 2, i32 3, i32 4>",
                 "%w14 = shufflevector <4 x i32> %g0, <4 x i32> %g1, <4 x i32> <i32 2, i32 3, i32 4, i32 5>",
                 "%x0 = xor <4 x i32> %w3, %g2", "%x1 = xor <4 x i32> %x0, %w14", "%x2 = xor <4 x i32> %x1, %g0",
                 "%r = call <4 x i32> @llvm.fshl.v4i32(<4 x i32> %x2, <4 x i32> %x2, <4 x i32> <i32 1, i32 1, i32 1, i32 1>)",
                 "%first = extractelement <4 x i32> %r, i64 0",
                 "%rot = call i32 @llvm.fshl.i32(i32 %first, i32 %first, i32 1)",
                 "%last = extractelement <4 x i32> %r, i64 3",
                 "%done = xor i32 %last, %rot",
                 "%complete = insertelement <4 x i32> %r, i32 %done, i64 3"] + pack_vector("%complete")
    output += emit_prim("sha1-schedule", ["U128"] * 5,
                        "Expand four schedule words from groups t-16, t-12, t-8 and t-4 in SIMD lanes; the last lane, which depends on the first, is completed by one rotated exclusive-or.", schedule)
    return output


def unpack_vector(wide: str, name: str) -> list[str]:
    """Split a U128 into four 32-bit lanes, lowest bits in lane 0, independent of byte order."""
    lines = []
    for lane in range(4):
        source = f"%{name}s{lane}"
        lines.append(f"{source} = lshr i128 {wide}, {lane * 32}" if lane else f"{source} = or i128 {wide}, 0")
        lines.append(f"%{name}w{lane} = trunc i128 {source} to i32")
    previous = "poison"
    for lane in range(4):
        target = f"%{name}" if lane == 3 else f"%{name}v{lane}"
        lines.append(f"{target} = insertelement <4 x i32> {previous}, i32 %{name}w{lane}, i64 {lane}")
        previous = target
    return lines


def pack_vector(vector: str) -> list[str]:
    """Pack four 32-bit lanes into a U128 with lane 0 lowest, independent of byte order."""
    lines = []
    for lane in range(4):
        lines += [f"%e{lane} = extractelement <4 x i32> {vector}, i64 {lane}", f"%z{lane} = zext i32 %e{lane} to i128"]
        if lane:
            lines.append(f"%p{lane} = shl i128 %z{lane}, {lane * 32}")
    lines += ["%o1 = or i128 %z0, %p1", "%o2 = or i128 %o1, %p2", "%value = or i128 %o2, %p3", "ret i128 %value"]
    return lines


MD5_SHIFTS = [7, 12, 17, 22] * 4 + [5, 9, 14, 20] * 4 + [4, 11, 16, 23] * 4 + [6, 10, 15, 21] * 4


def md5_constants() -> list[int]:
    """floor(2^32 * |sin(i + 1)|), checked against the published table's first and last entries."""
    table = [int(abs(math.sin(i + 1)) * (1 << 32)) & MASK32 for i in range(64)]
    assert table[0] == 0xD76AA478 and table[63] == 0xEB86D391
    return table


def md5_index(t: int) -> int:
    """Message word selected by round t."""
    return [t, (5 * t + 1) % 16, (3 * t + 5) % 16, (7 * t) % 16][t // 16]


def md5_generated() -> str:
    """Generate MD5 compression in four sixteen-round phases plus its private round primitives."""
    lines = [MD5_MARKER.rstrip(), "  -- Regenerate with python3 tooling/generate_sha1_md5_kernels.py; do not edit this suffix.",
             "  md5-compress :: Md5Words -> " + " -> ".join(["U64"] * 8 + ["Md5Words"]),
             "  md5-compress prior " + " ".join(f"w{i}" for i in range(8)) + " = case prior of",
             "    Words128 a b c d ->"]
    for i in range(16):
        half = "low" if i % 2 == 0 else "high"
        lines.append(f"      let m{i} = md5-{half} w{i // 2} in")
    for phase, letter in enumerate("abcd"):
        previous = "prior" if phase == 0 else f"s{phase - 1}"
        words = " ".join(f"m{md5_index(t)}" for t in range(phase * 16, phase * 16 + 16))
        lines.append(f"      let s{phase} = md5-rounds-{letter} {previous} {words} in")
    lines += ["      case s3 of",
              "        Words128 na nb nc nd ->",
              "          Words128 (wrapping-add-u32 a na) (wrapping-add-u32 b nb) (wrapping-add-u32 c nc) (wrapping-add-u32 d nd)", ""]
    names = ["f", "g", "h", "i"]
    constants = md5_constants()
    for phase, letter in enumerate("abcd"):
        lines += [f"  -- | Advance sixteen ordered rounds with the {names[phase].upper()} function over message words in schedule order.",
                  f"  md5-rounds-{letter} :: Md5Words -> " + " -> ".join(["U32"] * 16 + ["Md5Words"]),
                  f"  md5-rounds-{letter} prior " + " ".join(f"m{i}" for i in range(16)) + " = case prior of",
                  "    Words128 i0 i1 i2 i3 ->"]
        a, b, c, d = "i0", "i1", "i2", "i3"
        for i in range(16):
            t = phase * 16 + i
            lines.append(f"      let x{i} = md5-{names[phase]} {a} {b} {c} {d} m{i} {constants[t]} {MD5_SHIFTS[t]} in")
            a, b, c, d = d, f"x{i}", b, c
        lines += [f"      Words128 {a} {b} {c} {d}", ""]
    output = "\n".join(lines) + "\n"
    bodies = {
        "f": ["%bc = and i32 %1, %2", "%nb = xor i32 %1, -1", "%nd = and i32 %nb, %3", "%fn = or i32 %bc, %nd"],
        "g": ["%bd = and i32 %1, %3", "%nd0 = xor i32 %3, -1", "%cn = and i32 %2, %nd0", "%fn = or i32 %bd, %cn"],
        "h": ["%bc = xor i32 %1, %2", "%fn = xor i32 %bc, %3"],
        "i": ["%nd0 = xor i32 %3, -1", "%bn = or i32 %1, %nd0", "%fn = xor i32 %2, %bn"],
    }
    for name, body in bodies.items():
        output += emit_prim(f"md5-{name}", ["U32"] * 7 + ["U32"],
                            f"One ordered round with the {name.upper()} function: b + rotl(a + {name.upper()}(b, c, d) + message + constant, shift).",
                            body + ["%s0 = add i32 %0, %fn", "%s1 = add i32 %s0, %4", "%s2 = add i32 %s1, %5",
                                    "%r = call i32 @llvm.fshl.i32(i32 %s2, i32 %s2, i32 %6)", "%value = add i32 %1, %r", "ret i32 %value"]) + "\n"
    output += emit_prim("md5-low", ["U64", "U32"], "Select the first little-endian message word of a loaded pair.", ["%value = trunc i64 %0 to i32", "ret i32 %value"]) + "\n"
    output += emit_prim("md5-high", ["U64", "U32"], "Select the second little-endian message word of a loaded pair.", ["%shift = lshr i64 %0, 32", "%value = trunc i64 %shift to i32", "ret i32 %value"])
    return output


def refresh(path: Path, marker: str, suffix: str, check: bool, symbol: str) -> None:
    """Replace or verify the generated suffix of one handwritten module."""
    existing = path.read_text()
    if existing.count(marker) > 1:
        raise SystemExit(f"{path}: duplicate generated-core markers")
    if marker not in existing and symbol in existing:
        raise SystemExit(f"{path}: a compression core exists without its generation marker")
    expected = existing.split(marker, 1)[0].rstrip() + "\n\n" + suffix
    if check:
        if existing != expected:
            raise SystemExit(f"Generated core is stale: {path}")
    elif existing != expected:
        path.write_text(expected)


def main() -> None:
    """Write or verify both generated suffixes."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    refresh(ROOT / "seed/prelude/Text/Sha1.cinch", SHA1_MARKER, sha1_generated(), args.check, "  sha1-compress ::")
    refresh(ROOT / "seed/prelude/Text/Md5.cinch", MD5_MARKER, md5_generated(), args.check, "  md5-compress ::")


if __name__ == "__main__":
    main()
