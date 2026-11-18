#!/usr/bin/env python3
"""Render independent map observations without reproducing the map algorithms."""
from __future__ import annotations

import argparse
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
OPERATIONS = ("Collect", "Insert", "Copy", "DeleteFunc", "Clone", "Equal", "EqualFunc", "All", "Keys", "Values")


def pairs(rows: list[dict] | None, boolean: bool = False) -> str:
    """Render reference pairs in their recorded order, retaining input duplicates."""
    return "[" + ",".join(f"({row['key']},{str(row['value']) if not boolean else 'True' if row['value'] else 'False'})" for row in rows or []) + "]"


def source(index: int, vectors: Path | None = None) -> str:
    """Embed one trace as plain inputs and expected observations in a fixed consumer."""
    recorded = json.loads((vectors or ROOT / "compiler/tests/fixtures/maps-vectors.json").read_text())
    trace = recorded["traces"][index]
    assert recorded["format"] == 1 and len(trace["steps"]) == 100
    steps = []
    for row in trace["steps"]:
        steps.append("TraceStep " + " ".join((str(OPERATIONS.index(row["operation"])), str(row["key"]), str(row["value"]), pairs(row["pairs"]), pairs(row["expected"]), pairs(row["auxiliary"]), pairs(row["other"], True), str(row["observed"]))))
    template = (ROOT / "compiler/tests/fixtures/mutable-map-trace.cinch.in").read_text()
    assert template.count("@STEPS@") == 1
    return template.replace("@STEPS@", "[" + ",\n    ".join(steps) + "]")


def float_source(width: int, vectors: Path | None = None) -> str:
    """Render full-bit floating observations while retaining the declared input patterns."""
    recorded = json.loads((vectors or ROOT / "compiler/tests/fixtures/maps-vectors.json").read_text())
    rows = [row for row in recorded["floats"] if row["width"] == width]
    assert len(rows) == 7
    names = ("repeat-nan", "distinct-nans", "replace-signed-zero", "equal-self", "clone", "copy-fresh", "delete-all-by-key")
    assert tuple(row["operation"] for row in rows) == names
    patterns = (0x7FF8000000000001, 0x7FF0000000000001, 0xFFF8000000000017, 0x8000000000000000) if width == 64 else (0x7FC00001, 0x7F800001, 0xFFC00017, 0x80000000)
    steps = [f"FloatStep {index} {pairs(row['expected'])} {row['observed']}" for index, row in enumerate(rows)]
    replacements = {
        "@FLOAT@": f"F{width}", "@POLICY@": f"map-f{width}-ieee-policy",
        "@NAN@": str(patterns[0]), "@OTHER_NAN@": str(patterns[1]),
        "@NEGATIVE_NAN@": str(patterns[2]), "@NEGATIVE_ZERO@": str(patterns[3]),
        "@STEPS@": "[" + ",\n    ".join(steps) + "]",
        "@FROM_BITS@": "      %value = bitcast i64 %0 to double\n      ret double %value" if width == 64 else "      %bits = trunc i64 %0 to i32\n      %value = bitcast i32 %bits to float\n      ret float %value",
        "@TO_BITS@": "      %bits = bitcast double %0 to i64\n      ret i64 %bits" if width == 64 else "      %bits = bitcast float %0 to i32\n      %wide = zext i32 %bits to i64\n      ret i64 %wide",
    }
    template = (ROOT / "compiler/tests/fixtures/mutable-map-float.cinch.in").read_text()
    for name, replacement in replacements.items():
        template = template.replace(name, replacement)
    assert "@" not in template
    return template


def edge_source(vectors: Path | None = None) -> str:
    """Render actual SDK value-NaN and callback-mutation observations without reproducing map logic."""
    recorded = json.loads((vectors or ROOT / "compiler/tests/fixtures/maps-edge-vectors.json").read_text())
    assert recorded["format"] == 1 and recorded["sdk"] == "go1.27.4"
    nan = recorded["value_nan"]
    deletion = recorded["delete_current_replacement"]
    equality = recorded["equality_current_lookup"]
    replacements = {
        "@ORIGINAL_BITS@": nan["original_bits"], "@REPLACEMENT_BITS@": nan["replacement_bits"],
        "@SELF_EQUAL@": nan["self_equal"], "@CLONE_EQUAL@": nan["clone_equal"],
        "@CLONE_BIT_EQUAL@": nan["clone_bit_equal"], "@REPLACEMENT_BIT_SAME@": nan["replacement_bit_same"],
        "@ORIGINAL_SIZE@": nan["original_size"], "@CLONE_SIZE@": nan["clone_size"],
        "@ORIGINAL_VALUES_MATCH@": deletion["original_values_match"],
        "@DELETE_CALLBACKS@": deletion["callbacks"], "@DELETE_FINAL_SIZE@": deletion["final_size"],
        "@EQUALITY_RESULT@": equality["equal"], "@EQUALITY_CALLBACKS@": equality["callbacks"],
        "@LEFT_SIZE@": equality["left_size"], "@RIGHT_SIZE@": equality["right_size"],
    }
    template = (ROOT / "compiler/tests/fixtures/mutable-map-edge-control.cinch.in").read_text()
    for token, value in replacements.items():
        assert template.count(token) == 1, token
        template = template.replace(token, str(value))
    assert "@" not in template
    return template


def main() -> None:
    """Write one independently generated trace consumer for either frontend."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--case", type=int, choices=range(8), default=0)
    parser.add_argument("--vectors", type=Path)
    parser.add_argument("--float-width", type=int, choices=(32, 64))
    parser.add_argument("--edge-control", action="store_true")
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    if args.edge_control and args.float_width:
        parser.error("select either floating key rows or the supplemental edge consumer")
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(edge_source(args.vectors) if args.edge_control else float_source(args.float_width, args.vectors) if args.float_width else source(args.case, args.vectors))


if __name__ == "__main__":
    main()
