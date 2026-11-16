#!/usr/bin/env python3
"""Generate portable SIMD Base64 byte kernels and exact private byte-table gathers."""
from __future__ import annotations

import argparse
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
MARKER = "  -- Generated pure Base64 SIMD kernels and private exact table gathers.\n"


def vector(kind: str, values: list[int]) -> str:
    """Render a typed LLVM constant vector without host-endian assumptions."""
    return "<" + ", ".join(f"{kind} {value}" for value in values) + ">"


def splat(kind: str, value: int) -> str:
    """Return one constant in each of eight independent lanes."""
    return vector(kind, [value] * 8)


def body(lines: list[str], endian: str) -> str:
    """Wrap a complete checked inline body with an explicit target-endian selector."""
    return "    llvm {\n" + "\n".join("      " + line for line in lines) + f"\n    }} when endian {endian}\n"


def input_vector(endian: str) -> list[str]:
    """Normalize a packed little-endian byte word before an LLVM vector bitcast."""
    first = "%input = or i64 %0, 0" if endian == "little" else "%input = call i64 @llvm.bswap.i64(i64 %0)"
    return [first, "%v = bitcast i64 %input to <8 x i8>"]


def result_vector(name: str, endian: str) -> list[str]:
    """Normalize a result vector into the same low-byte-first scalar convention."""
    lines = [f"%packed = bitcast <8 x i8> {name} to i64"]
    if endian == "big":
        lines += ["%result = call i64 @llvm.bswap.i64(i64 %packed)", "ret i64 %result"]
    else:
        lines += ["ret i64 %packed"]
    return lines


def encode_values(endian: str) -> list[str]:
    """Extract eight six-bit values from two complete three-byte groups."""
    lines = input_vector(endian)
    lines += [f"%a = shufflevector <8 x i8> %v, <8 x i8> zeroinitializer, <8 x i32> {vector('i32', [0, 0, 1, 2, 3, 3, 4, 5])}",
              f"%b = shufflevector <8 x i8> %v, <8 x i8> zeroinitializer, <8 x i32> {vector('i32', [8, 1, 2, 8, 8, 4, 5, 8])}",
              "%aw = zext <8 x i8> %a to <8 x i16>", "%bw = zext <8 x i8> %b to <8 x i16>",
              f"%hi = shl <8 x i16> %aw, {splat('i16', 8)}", "%pair = or <8 x i16> %hi, %bw",
              f"%shifted = lshr <8 x i16> %pair, {vector('i16', [10, 4, 6, 8, 10, 4, 6, 8])}",
              "%narrow = trunc <8 x i16> %shifted to <8 x i8>", f"%values = and <8 x i8> %narrow, {splat('i8', 63)}"]
    return lines + result_vector("%values", endian)


def map_fixed(endian: str) -> list[str]:
    """Map six-bit lanes to either fixed alphabet without memory lookups."""
    lines = input_vector(endian)
    lines += ["%url = icmp ne i8 %1, 0", f"%upper = icmp ult <8 x i8> %v, {splat('i8', 26)}",
              f"%lower = icmp ult <8 x i8> %v, {splat('i8', 52)}", f"%digit = icmp ult <8 x i8> %v, {splat('i8', 62)}",
              f"%special = icmp eq <8 x i8> %v, {splat('i8', 62)}",
              f"%last = select i1 %url, <8 x i8> {splat('i8', 32)}, <8 x i8> {splat('i8', -16)}",
              f"%next = select i1 %url, <8 x i8> {splat('i8', -17)}, <8 x i8> {splat('i8', -19)}",
              "%specialbase = select <8 x i1> %special, <8 x i8> %next, <8 x i8> %last",
              f"%digitbase = select <8 x i1> %digit, <8 x i8> {splat('i8', -4)}, <8 x i8> %specialbase",
              f"%lowerbase = select <8 x i1> %lower, <8 x i8> {splat('i8', 71)}, <8 x i8> %digitbase",
              f"%base = select <8 x i1> %upper, <8 x i8> {splat('i8', 65)}, <8 x i8> %lowerbase",
              "%mapped = add <8 x i8> %v, %base"]
    return lines + result_vector("%mapped", endian)


def classify_fixed(endian: str) -> list[str]:
    """Classify standard or URL bytes in eight lanes, retaining 255 for invalid input."""
    lines = input_vector(endian) + ["%url = icmp ne i8 %1, 0"]
    for name, low, high in [("upper", 65, 90), ("lower", 97, 122), ("digit", 48, 57)]:
        lines += [f"%{name}lo = icmp uge <8 x i8> %v, {splat('i8', low)}",
                  f"%{name}hi = icmp ule <8 x i8> %v, {splat('i8', high)}",
                  f"%{name} = and <8 x i1> %{name}lo, %{name}hi"]
    lines += [f"%char62 = select i1 %url, <8 x i8> {splat('i8', 45)}, <8 x i8> {splat('i8', 43)}",
              f"%char63 = select i1 %url, <8 x i8> {splat('i8', 95)}, <8 x i8> {splat('i8', 47)}",
              "%is62 = icmp eq <8 x i8> %v, %char62", "%is63 = icmp eq <8 x i8> %v, %char63",
              f"%a = select <8 x i1> %is63, <8 x i8> {splat('i8', 63)}, <8 x i8> {splat('i8', 255)}",
              f"%b = select <8 x i1> %is62, <8 x i8> {splat('i8', 62)}, <8 x i8> %a",
              f"%uv = sub <8 x i8> %v, {splat('i8', 65)}", f"%lv = sub <8 x i8> %v, {splat('i8', 71)}",
              f"%dv = add <8 x i8> %v, {splat('i8', 4)}", "%c = select <8 x i1> %digit, <8 x i8> %dv, <8 x i8> %b",
              "%d = select <8 x i1> %lower, <8 x i8> %lv, <8 x i8> %c", "%values = select <8 x i1> %upper, <8 x i8> %uv, <8 x i8> %d"]
    return lines + result_vector("%values", endian)


def decode_values(endian: str) -> list[str]:
    """Compact eight validated six-bit lanes into six bytes and retain invalid-lane flags."""
    lines = input_vector(endian)
    lines += [f"%bad = icmp ugt <8 x i8> %v, {splat('i8', 63)}", "%mask = bitcast <8 x i1> %bad to i8",
              f"%a = shufflevector <8 x i8> %v, <8 x i8> zeroinitializer, <8 x i32> {vector('i32', [0, 1, 2, 4, 5, 6, 8, 8])}",
              f"%b = shufflevector <8 x i8> %v, <8 x i8> zeroinitializer, <8 x i32> {vector('i32', [1, 2, 3, 5, 6, 7, 8, 8])}",
              "%aw = zext <8 x i8> %a to <8 x i16>", "%bw = zext <8 x i8> %b to <8 x i16>",
              f"%hi = shl <8 x i16> %aw, {vector('i16', [2, 4, 6, 2, 4, 6, 0, 0])}",
              f"%lo = lshr <8 x i16> %bw, {vector('i16', [4, 2, 0, 4, 2, 0, 0, 0])}",
              "%joined = or <8 x i16> %hi, %lo", "%bytes = trunc <8 x i16> %joined to <8 x i8>", "%packed = bitcast <8 x i8> %bytes to i64"]
    packed = "%packed"
    if endian == "big":
        lines += ["%normalized = call i64 @llvm.bswap.i64(i64 %packed)"]
        packed = "%normalized"
    lines += ["%error = zext i8 %mask to i64", "%flag = shl i64 %error, 56", f"%result = or i64 {packed}, %flag", "ret i64 %result"]
    return lines


def gather() -> str:
    """Generate eight exact byte loads from a caller-owned complete 64/256-byte table."""
    lines = ["%base = inttoptr i64 %0 to ptr"]
    previous = "0"
    for lane in range(8):
        lines += [f"%s{lane} = lshr i64 %1, {lane * 8}", f"%i{lane} = and i64 %s{lane}, 255",
                  f"%p{lane} = getelementptr i8, ptr %base, i64 %i{lane}", f"%v{lane} = load i8, ptr %p{lane}, align 1",
                  f"%w{lane} = zext i8 %v{lane} to i64", f"%o{lane} = shl i64 %w{lane}, {lane * 8}", f"%r{lane} = or i64 {previous}, %o{lane}"]
        previous = f"%r{lane}"
    lines += [f"ret i64 {previous}"]
    return "  -- | Load eight caller-proved table indices exactly, retaining their owner at the checked call site.\n  prim lookup-eight :: I64 -> U64 -> U64 where\n    unsafe llvm {\n" + "\n".join("      " + x for x in lines) + "\n    }\n"


def generated() -> str:
    """Return deterministically generated private kernels with canonical documentation."""
    output = [MARKER + "  -- Regenerate with python3 tooling/generate_base64_kernels.py; do not edit this suffix.\n"]
    for name, signature, doc, function in [
        ("encode-values", "U64 -> U64", "Extract eight six-bit lanes from six packed input bytes.", encode_values),
        ("map-fixed", "U64 -> U8 -> U64", "Map eight value lanes to the selected fixed byte alphabet.", map_fixed),
        ("classify-fixed", "U64 -> U8 -> U64", "Classify eight fixed-alphabet bytes, marking every invalid lane with 255.", classify_fixed),
        ("decode-values", "U64 -> U64", "Compact eight six-bit lanes into six bytes and flag invalid lanes above the output.", decode_values),
    ]:
        output.append(f"  -- | {doc}\n  prim {name} :: {signature} where\n" + body(function("little"), "little") + body(function("big"), "big"))
    output.append(gather())
    return "\n".join(output)


def main() -> None:
    """Replace only the generated suffix, or check that the source is current without writing."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--module", type=Path, default=ROOT / "compiler/prelude/Text/Base64/Encoding.cinch")
    parser.add_argument("--fragment", type=Path)
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    target = args.fragment or args.module
    old = target.read_text() if target.exists() else ""
    if args.fragment:
        expected = generated()
    else:
        if not old or old.count(MARKER) > 1:
            parser.error("the handwritten module must exist and contain at most one generation marker")
        if MARKER not in old and "  prim encode-values ::" in old:
            parser.error("an existing kernel has no generation marker; refusing to append a duplicate")
        expected = old.split(MARKER, 1)[0].rstrip() + "\n\n" + generated()
    if args.check:
        if old != expected:
            parser.exit(1, f"Generated Base64 kernels are stale: {target}\n")
    elif old != expected:
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_text(expected)


if __name__ == "__main__":
    main()
