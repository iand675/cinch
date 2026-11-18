#!/usr/bin/env python3
"""Render SDK-free component fixtures from preserved independent public observations."""
from __future__ import annotations
import argparse
import json
from pathlib import Path

PREAMBLE = """-- | Fixed-block component checks from independent public archive observations.
module Main where
  import Data.Core
  import Data.Bytes
  import Data.Machine
  import Data.Nat
  import Data.Tar
  import Text.Tar.Block

  -- | Apply only range-proved sparse byte assignments to a fresh block.
  fill :: Bytes -> List (Nat, InRange 0 255 Nat) -> Unit
  fill out writes = case writes of
    Nil -> ()
    Cons (at, value) rest ->
      if at < bytes-len out then
        let stored = bytes-set out at value in fill out rest
      else ()

  -- | Construct the complete 512-byte fixture from its independent nonzero bytes.
  block-from :: List (Nat, InRange 0 255 Nat) -> Bytes
  block-from writes =
    let out = bytes-new 512 in
    let filled = fill out writes in out

  -- | Observe raw magic without comparing constructor representation tags.
  format-code :: TarBlockFormat -> Nat
  format-code value = case value of
    TarV7Block -> 0
    TarUstarPaxBlock -> 1
    TarGnuBlock -> 2
    TarStarBlock -> 3

  -- | Compare the independently observed numeric success or refusal.
  number-ok :: TarBlock -> Nat -> Nat -> Bool -> I64 -> Bool
  number-ok block start width refused expected = case tar-block-number block start width of
    Err TarInvalidNumber -> refused
    Err other -> False
    Ok actual -> if refused then False else eq-i64 actual expected

  -- | Verify text tails and that neither source nor extracted buffers can mutate retained metadata.
  ownership-ok :: Bytes -> TarBlock -> Bytes -> Bool
  ownership-ok source block expected-name = case tar-block-text block 0 100 of
    Err error -> False
    Ok name ->
      let first = tar-text-bytes name in
      let matched = eq-i64 (bytes-compare first expected-name) 0 in
      let source-change = if bytes-len source > 0 then bytes-set source 0 42 else () in
      let first-change = if bytes-len first > 0 then bytes-set first 0 42 else () in
      let exported = tar-block-bytes block in
      let export-change = if bytes-len exported > 156 then bytes-set exported 156 0 else () in
      let still-name = tar-text-bytes name in
      let still-block = tar-block-bytes block in
      matched && eq-i64 (bytes-compare still-name expected-name) 0 &&
        (if bytes-len still-block > 156 then bytes-get still-block 156 == 48 else False)

  -- | Check exact sums, checksum acceptance, raw format, numeric field and isolated text snapshots.
  check-block :: List (Nat, InRange 0 255 Nat) -> I64 -> I64 -> Bool -> Nat -> Bool -> Nat -> Nat -> Bool -> I64 -> Bytes -> Bool
  check-block writes expected-unsigned expected-signed valid wire numeric start width refused number name =
    let source = block-from writes in
    case tar-block-checksums source of
      Err error -> False
      Ok (unsigned, signed) ->
        eq-i64 unsigned expected-unsigned && eq-i64 signed expected-signed &&
        (case tar-parse-block source of
          Err TarInvalidHeader -> if valid then False else True
          Err other -> False
          Ok None -> False
          Ok (Some block) ->
            valid && format-code (tar-block-format block) == wire &&
            (if numeric then number-ok block start width refused number else True) &&
            (if refused then True else ownership-ok source block name))

  -- | Check exact independent writer numeric bytes and named representability refusals.
  check-encoding :: Nat -> I64 -> Bool -> Bytes -> Bool
  check-encoding width value refused expected = case tar-encode-number width value of
    Err (TarFieldTooLong field) -> refused
    Err other -> False
    Ok actual -> if refused then False else eq-i64 (bytes-compare actual expected) 0

  -- | Run each case once and return its stable first-failure index.
  run :: List (Unit -> Bool) -> Int -> Int
  run cases index = case cases of
    Nil -> 0
    Cons test rest -> if test () then run rest (index + 1) else index
"""


def byte_list(hex_text: str) -> str:
    """Render recorded bytes through the typed byte constructor without reinterpretation."""
    return "(bytes-from-list [" + ", ".join(str(byte) for byte in bytes.fromhex(hex_text)) + "])"


def source(vectors: dict, start: int, count: int, encodings: bool) -> str:
    """Generate one bounded fixture; the independent source observations remain unchanged."""
    rows = vectors["Encodings" if encodings else "Cases"][start:start + count]
    declarations = []
    for i, row in enumerate(rows):
        if encodings:
            arguments = f"check-encoding {row['Width']} ({row['Value']}) {'True' if row['Error'] else 'False'} {byte_list(row['BytesHex'])}"
        else:
            writes = "[" + ", ".join(f"({write['At']}, {write['Value']})" for write in row["Writes"]) + "]"
            boolean = lambda key: "True" if row[key] else "False"
            arguments = f"check-block {writes} ({row['Unsigned']}) ({row['Signed']}) {boolean('BlockValid')} {row['WireFormat']} {boolean('Numeric')} {row['Start']} {row['Width']} {boolean('NumberError')} ({row['Number']}) {byte_list(row['NameHex'])}"
        declarations.append(f"\n  -- | Independent case {start+i}: {row.get('ID', 'writer-numeric')}.\n  row-a{i} :: Unit -> Bool\n  row-a{i} ignored = {arguments}\n")
    tests = "Nil"
    for i in reversed(range(len(rows))):
        tests = f"(Cons row-a{i} {tests})"
    return PREAMBLE + "".join(declarations) + f"\n  -- | Return zero only when the complete selected independent unit passes.\n  main :: Int\n  main = run {tests} 1\n"


def main() -> None:
    """Select a bounded slice and emit a standalone documented consumer."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--vectors", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--start", type=int, default=0)
    parser.add_argument("--count", type=int, default=32)
    parser.add_argument("--encodings", action="store_true")
    args = parser.parse_args()
    args.output.write_text(source(json.loads(args.vectors.read_text()), args.start, args.count, args.encodings))


if __name__ == "__main__":
    main()
