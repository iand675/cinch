#!/usr/bin/env python3
"""Generate independent finite-word observations for the canonical scalar intrinsic subset."""
from __future__ import annotations

import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
TYPES = [(f"{prefix}{width}", width, prefix == "I") for prefix in ["I", "U"] for width in [8, 16, 32, 64]]


def normalize(value: int, width: int, signed: bool) -> int:
    """Interpret mathematical arithmetic modulo the destination carrier and its signed range."""
    value %= 1 << width
    return value - (1 << width) if signed and value >= 1 << (width - 1) else value


def literal(value: int, name: str) -> str:
    """Fix the input nominal explicitly so default mathematical integers cannot satisfy a machine test."""
    return f"({value} :: {name})"


def vectors() -> list[dict]:
    """Exercise arithmetic boundaries, signed order, shift masking and every supported wrapping cast."""
    rows = []

    def add(name: str, expression: str, expected: int | bool, result: str) -> None:
        """Retain one independently computed observation with an exact result nominal."""
        rows.append({"name": name, "expression": expression, "expected": expected, "result": result})

    for name, width, signed in TYPES:
        low = -(1 << (width - 1)) if signed else 0
        high = (1 << (width - 1)) - 1 if signed else (1 << width) - 1
        pairs = [(high, 1), (low, -1 if signed else high), (-1 if signed else high, high)]
        for index, (left, right) in enumerate(pairs):
            args = f"{literal(left, name)} {literal(right, name)}"
            for op, result in [("wrapping-add", left + right), ("wrapping-sub", left - right),
                               ("wrapping-mul", left * right), ("bit-and", left & right),
                               ("bit-or", left | right), ("bit-xor", left ^ right)]:
                add(f"{op}-{name}-{index}", f"W.{op}-{name.lower()} {args}", normalize(result, width, signed), name)
            for op, result in [("eq", left == right), ("ne", left != right), ("lt", left < right),
                               ("le", left <= right), ("gt", left > right), ("ge", left >= right)]:
                add(f"{op}-{name}-{index}", f"W.{op}-{name.lower()} {args}", result, "Bool")
        for index, value in enumerate([low, high, -1 if signed else high]):
            add(f"not-{name}-{index}", f"W.bit-not-{name.lower()} {literal(value, name)}", normalize(~value, width, signed), name)
        for value in [low if signed else 1 << (width - 1), high]:
            for count in [0, width - 1, width, width + 1, (1 << 32) - 1]:
                masked = count % width
                for direction, result in [("shl", value << masked), ("shr", value >> masked)]:
                    add(f"{direction}-{name}-{value}-{count}",
                        f"W.wrapping-{direction}-{name.lower()} {literal(value, name)} {literal(count, 'U32')}",
                        normalize(result, width, signed), name)
        for destination, destination_width, destination_signed in TYPES:
            for index, value in enumerate([low, high, -1 if signed else high]):
                add(f"cast-{name}-{destination}-{index}",
                    f"W.wrapping-{name.lower()}-to-{destination.lower()} {literal(value, name)}",
                    normalize(value, destination_width, destination_signed), destination)
    add("higher-order-u64", "apply-word W.wrapping-add-u64 18446744073709551615 1", 0, "U64")
    add("partial-u64", "(let increment = W.wrapping-add-u64 18446744073709551615 in increment 2)", 1, "U64")
    add("higher-order-unary-u8", "apply-byte W.bit-not-u8 128", 127, "U8")
    return rows


def check_expression(row: dict) -> str:
    """Observe exact literals with the preexisting pattern matcher, independent of the tested equality helpers."""
    expected = str(row["expected"])
    return f"(case ({row['expression']}) of {expected} -> True; _ -> False)"


def consumer(rows: list[dict]) -> str:
    """Keep every test declaration documented and the qualified import visible."""
    return '''-- | Observe exact scalar carrier boundaries through checked imported operations.
module Main where
  export main
  import Data.Machine as W
  -- | Consume a binary scalar function through the ordinary higher-order ABI.
  apply-word :: (U64 -> U64 -> U64) -> U64 -> U64 -> U64
  apply-word function left right = function left right
  -- | Consume a unary narrow scalar function through the ordinary higher-order ABI.
  apply-byte :: (U8 -> U8) -> U8 -> U8
  apply-byte function value = function value
  -- | Preserve every complete literal observation in source order.
  all :: List Bool -> Bool
  all values = case values of Nil -> True; Cons first rest -> first && all rest
  -- | Return failure when any independent scalar observation differs.
  main :: Int
  main = if all [
''' + ",\n".join("    " + check_expression(row) for row in rows) + "\n    ] then 0 else 1\n"


def main() -> None:
    """Store the finite reference observations without depending on either compiler's arithmetic lowering."""
    target = ROOT / "seed/tests/fixtures/native-scalar-intrinsic-vectors.json"
    target.write_text(json.dumps(vectors(), indent=2) + "\n")
    print(f"{len(vectors())} scalar observations written to {target}")


if __name__ == "__main__":
    main()
