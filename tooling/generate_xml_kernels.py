#!/usr/bin/env python3
"""Generate the name-character classes and SIMD span kernels of Text.Xml.

The name classes are XML 1.0's: a name starts with a letter, '_' or ':' and
continues with letters, digits, '.', '-', '_', ':', combining characters and
extenders (Appendix B of the fourth edition), recorded as merged code-point
ranges in tooling/fixtures/xml-name-ranges.json. Each class becomes a balanced
tree of comparisons, so a check costs about eight comparisons and no table.

The span kernels load sixteen bytes as an `<16 x i8>` vector, mark stop bytes
with lane compares, and find the first stop with an unsigned minimum over lane
indices, which is independent of target byte order.

    python3 tooling/generate_xml_kernels.py          # rewrite the generated suffix
    python3 tooling/generate_xml_kernels.py --check  # fail when it is stale
"""
from __future__ import annotations

import argparse
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
TARGET = ROOT / "seed/prelude/Text/Xml.cinch"
RANGES = ROOT / "tooling/fixtures/xml-name-ranges.json"
MARKER = "  -- Generated XML kernels: name-character classes and sixteen-byte span scanners.\n"


def tree(ranges: list[list[int]], indent: str) -> list[str]:
    """A balanced comparison tree deciding membership of c in sorted, disjoint ranges."""
    if len(ranges) == 1:
        lo, hi = ranges[0]
        return [f"{indent}c >= {lo} && c <= {hi}" if lo != hi else f"{indent}c == {lo}"]
    mid = len(ranges) // 2
    return ([f"{indent}if c < {ranges[mid][0]} then"] + tree(ranges[:mid], indent + "  ")
            + [f"{indent}else"] + tree(ranges[mid:], indent + "  "))


def splat(v: int) -> str:
    """A sixteen-lane constant."""
    return "<" + ", ".join(f"i8 {v}" for _ in range(16)) + ">"


def span(name: str, doc: list[str], stops: list[tuple[str, int]], allow: list[int]) -> list[str]:
    """A span kernel: OR of lane compares, minus allowed control bytes, then the first stop index."""
    idx = "<" + ", ".join(f"i8 {i}" for i in range(16)) + ">"
    out = [f"  -- | {doc[0]}"] + [f"  -- {line}" for line in doc[1:]] + [
        f"  prim {name} :: I64 -> I64 -> I64 -> I64 where", "    unsafe llvm {",
        "      %base = inttoptr i64 %0 to ptr", "      %at = getelementptr i8, ptr %base, i64 %1",
        "      %v = load <16 x i8>, ptr %at, align 1",
        "      %e8 = trunc i64 %2 to i8", "      %e1 = insertelement <16 x i8> poison, i8 %e8, i64 0",
        "      %extra = shufflevector <16 x i8> %e1, <16 x i8> poison, <16 x i32> zeroinitializer",
        "      %s0 = icmp eq <16 x i8> %v, %extra",
        f"      %low = icmp ult <16 x i8> %v, {splat(32)}"]
    keep = "%low"
    for n, a in enumerate(allow):
        out += [f"      %a{n} = icmp ne <16 x i8> %v, {splat(a)}", f"      %k{n} = and <16 x i1> {keep}, %a{n}"]
        keep = f"%k{n}"
    out += [f"      %s1 = or <16 x i1> %s0, {keep}", f"      %hi = icmp uge <16 x i8> %v, {splat(128)}", "      %s2 = or <16 x i1> %s1, %hi"]
    last = "%s2"
    for n, (kind, v) in enumerate(stops):
        out += [f"      %t{n} = icmp {kind} <16 x i8> %v, {splat(v)}", f"      %u{n} = or <16 x i1> {last}, %t{n}"]
        last = f"%u{n}"
    out += [f"      %pick = select <16 x i1> {last}, <16 x i8> {idx}, <16 x i8> {splat(16)}",
            "      %first = call i8 @llvm.vector.reduce.umin.v16i8(<16 x i8> %pick)",
            "      %r = zext i8 %first to i64", "      ret i64 %r", "    }"]
    return out


def suffix() -> str:
    """The generated text that follows the marker."""
    ranges = json.loads(RANGES.read_text())
    lines = [MARKER.rstrip("\n"), "",
             "  -- | Whether a code point may start an XML name.",
             "  name-first :: Nat -> Bool", "  name-first c ="] + tree(ranges["first"], "    ") + [
             "", "  -- | Whether a code point may continue an XML name (beyond those that may start one).",
             "  name-second :: Nat -> Bool", "  name-second c ="] + tree(ranges["second"], "    ") + [""]
    lines += span("text-span", ["Leading plain text bytes among the sixteen at base + offset (16 when all are). Stops at",
                                "controls other than tab and line feed, non-ASCII bytes, '<', '&', carriage return, ']' and one",
                                "extra byte (an attribute's quote, or 0 for none)."],
                  [("eq", 60), ("eq", 38), ("eq", 93)], [9, 10])
    lines += [""] + span("escape-span", ["Leading bytes that escaping leaves unchanged among the sixteen at base + offset (16 when all",
                                         "are). Stops at every control, non-ASCII bytes, the quotes, '&', '<', '>' and one extra byte."],
                         [("eq", 34), ("eq", 39), ("eq", 38), ("eq", 60), ("eq", 62)], [])
    return "\n".join(lines) + "\n"


def main() -> None:
    """Rewrite or check the generated suffix of Text.Xml."""
    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    existing = TARGET.read_text()
    if existing.count(MARKER) != 1:
        raise SystemExit(f"{TARGET}: expected exactly one generated marker")
    expected = existing.split(MARKER, 1)[0].rstrip() + "\n\n" + suffix()
    if args.check:
        if existing != expected:
            raise SystemExit(f"{TARGET} is stale; rerun tooling/generate_xml_kernels.py")
        return
    TARGET.write_text(expected)


if __name__ == "__main__":
    main()
