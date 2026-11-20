#!/usr/bin/env python3
"""Render deterministic checked CSV consumer programs from independent fixture bytes."""
from __future__ import annotations

import argparse
import json
from pathlib import Path
import re

ROOT = Path(__file__).resolve().parents[1]
FIXTURES = ROOT / "seed/tests/fixtures"

HELPERS = r'''-- | Configurable CSV fixture consumer with runtime input and precise expected metadata.
module Main where
  import Text.Csv.Record
  import Data.Char
  import Data.Machine
  -- | Keep immutable fixture input in runtime execution rather than constant evaluation.
  prim runtime-size :: U32 -> U32 where
    llvm {
      %value = or i32 %0, 0
      ret i32 %value
    }
  -- | Rebuild fixture text through a runtime-sized copied byte interval.
  runtime-text :: String -> String
  runtime-text text =
    let input = string-to-bytes text in
    let n = u32-to-nat (runtime-size (wrapping-i64-to-u32 (nat-to-i64 (bytes-len input)))) in
    let out = bytes-new n in
    if bytes-len input <= bytes-len out then
      let copied = bytes-copy-range out input 0 0 (bytes-len input) in
      case string-from-bytes out of Some result -> result; None -> ""
    else ""
  -- | Obtain a validated scalar from compact fixture data.
  scalar :: Nat -> Char
  scalar code = case char-from-code code of Some value -> value; None -> ','
  -- | Build a checked fixture dialect without embedding representation constructors.
  config :: Nat -> Nat -> CsvFieldPolicy -> Bool -> Bool -> CsvOptions
  config delimiter comment policy lazy trim =
    case csv-options (scalar delimiter) (if comment == 0 then None else Some (scalar comment)) policy lazy trim of
      Ok selected -> selected
      Err error -> csv-default-options
  -- | Compare immutable field lists without requiring an orphan list equality instance.
  fields-equal :: List String -> List String -> Bool
  fields-equal left right = case left of
    Nil -> case right of Nil -> True; Cons first rest -> False
    Cons a more -> case right of Nil -> False; Cons b rest -> a == b && fields-equal more rest
  -- | Compare every expected field position through the safe public indexing API.
  positions-equal :: CsvRecord -> List (Nat, Nat) -> Nat -> Bool
  positions-equal row positions index = case positions of
    Nil -> case csv-record-field-position row index of None -> True; Some value -> False
    Cons first rest -> case first of
      (line, column) -> case csv-record-field-position row index of
        None -> False
        Some actual -> case actual of (actual-line, actual-column) -> actual-line == line && actual-column == column && positions-equal row rest (index + 1)
  -- | Complete expected read progress and structured error details.
  data Expected where
    -- | Fields, positions, input offset, error kind/location and expected/actual counts.
    Expect :: List String -> List (Nat, Nat) -> Nat -> String -> Nat -> Nat -> Nat -> Nat -> Nat -> Expected
  -- | Compare precise error locations while treating both quote failures as one external category.
  error-equal :: Option CsvRecordError -> String -> Nat -> Nat -> Nat -> Nat -> Nat -> Bool
  error-equal actual kind start line column expected count = case actual of
    None -> kind == ""
    Some failure -> csv-record-error-start-line failure == start && csv-record-error-line failure == line && csv-record-error-column failure == column
      && (case failure of
        CsvBareQuote a b c -> kind == "bare"
        CsvQuote a b c -> kind == "quote"
        CsvUnclosedQuote a b c -> kind == "quote"
        CsvFieldCount a b c required received -> kind == "fields" && required == expected && received == count
        CsvFieldEncoding a b c index -> False)
  -- | Compare every advanced read, including continuing after malformed physical records.
  read-equal :: CsvReader -> List Expected -> Bool
  read-equal reader expected = case expected of
    Nil -> case csv-reader-read reader of (CsvEnd, next) -> True; other -> False
    Cons first rest -> case first of
      Expect fields positions offset kind start line column required count -> case csv-reader-read reader of
        (CsvReadRecord row error, next) -> fields-equal (csv-record-fields row) fields
          && positions-equal row positions 0 && csv-reader-input-offset next == offset
          && error-equal error kind start line column required count && read-equal next rest
        other -> False
  -- | Give all writer fixtures runtime text as well as their static expected output.
  runtime-fields :: List String -> List String
  runtime-fields fields = case fields of Nil -> Nil; Cons first rest -> Cons (runtime-text first) (runtime-fields rest)
  -- | Identify the first failed fixture without obscuring later source positions.
  first-failure :: List Bool -> Nat -> Nat
  first-failure checks index = case checks of
    Nil -> 0
    Cons first rest -> if first then first-failure rest (index + 1) else index
  -- | Preserve a bounded group's original failing index without expanding all fixtures in one body.
  first-result :: List Nat -> Nat
  first-result groups = case groups of Nil -> 0; Cons first rest -> if first > 0 then first else first-result rest
'''

STREAM_HELPERS = r'''
  -- | Limit every successful source read while preserving its actual advanced continuation.
  chunked :: forall r. using (io :: Io r) => Nat -> ByteReader r -> ByteReader r
  chunked maximum source = stream-reader (\n -> case stream-read source (if maximum < n then maximum else n) of
    (result, next) -> (result, chunked maximum next))
  -- | Compare every streamed record, source offset and parse error against independent results.
  stream-read-equal :: forall r. using (io :: Io r) => CsvStreamReader r -> List Expected ->{IO r} Bool
  stream-read-equal reader expected = case expected of
    Nil -> case csv-stream-read reader of (Ok CsvEnd, next) -> True; other -> False
    Cons first rest -> case first of
      Expect fields positions offset kind start line column required count -> case csv-stream-read reader of
        (Ok (CsvReadRecord row error), next) -> fields-equal (csv-record-fields row) fields
          && positions-equal row positions 0 && csv-stream-input-offset next == offset
          && error-equal error kind start line column required count && stream-read-equal next rest
        other -> False
  -- | Observe the exact bytes emitted by synchronous record writing and completion.
  stream-write-equal :: forall r. using (io :: Io r) => CsvOptions -> Bool -> List String -> String ->{IO r} Bool
  stream-write-equal selected crlf fields expected = case csv-stream-write (csv-stream-writer selected crlf writer-buffer) fields of
    (Ok unit, next) -> case csv-stream-flush next of
      (Ok unit, final) -> case writer-bytes (csv-stream-writer-sink final) of
        None -> False
        Some bytes -> case string-from-bytes bytes of None -> False; Some actual -> actual == expected
      other -> False
    other -> False
'''

BYTE_HELPERS = r'''
  -- | Rebuild arbitrary fixture bytes through a runtime-sized independently owned interval.
  runtime-bytes :: Bytes -> Bytes
  runtime-bytes input =
    let n = u32-to-nat (runtime-size (wrapping-i64-to-u32 (nat-to-i64 (bytes-len input)))) in
    let out = bytes-new n in
    if bytes-len input <= bytes-len out then let copied = bytes-copy-range out input 0 0 (bytes-len input) in out else bytes-new 0
  -- | Compare byte fields without decoding invalid UTF-8 or treating NUL as a terminator.
  raw-fields-equal :: List Bytes -> List Bytes -> Bool
  raw-fields-equal left right = case left of
    Nil -> case right of Nil -> True; Cons first rest -> False
    Cons first rest -> case right of Nil -> False; Cons actual more -> eq-i64 (bytes-compare first actual) 0 && raw-fields-equal rest more
  -- | Keep arbitrary byte writer inputs in runtime execution as well as their reader counterparts.
  runtime-raw-fields :: List Bytes -> List Bytes
  runtime-raw-fields fields = case fields of Nil -> Nil; Cons first rest -> Cons (runtime-bytes first) (runtime-raw-fields rest)
  -- | Compare all safe raw field position observations, including the out-of-range boundary.
  raw-positions-equal :: CsvByteRecord -> List (Nat, Nat) -> Nat -> Bool
  raw-positions-equal row positions index = case positions of
    Nil -> case csv-byte-record-field-position row index of None -> True; Some value -> False
    Cons first rest -> case first of
      (line, column) -> case csv-byte-record-field-position row index of
        None -> False
        Some actual -> case actual of (actual-line, actual-column) -> actual-line == line && actual-column == column && raw-positions-equal row rest (index + 1)
  -- | Expected arbitrary-byte fields and structured source/error metadata.
  data RawExpected where
    -- | Field bytes, positions, offset, error kind/location and field-count mismatch details.
    ExpectBytes :: List Bytes -> List (Nat, Nat) -> Nat -> String -> Nat -> Nat -> Nat -> Nat -> Nat -> RawExpected
  -- | Compare each pure byte-reader step without converting its fields to text.
  raw-read-equal :: CsvByteReader -> List RawExpected -> Bool
  raw-read-equal reader expected = case expected of
    Nil -> case csv-byte-reader-read reader of (CsvByteEnd, next) -> True; other -> False
    Cons first rest -> case first of
      ExpectBytes fields positions offset kind start line column required count -> case csv-byte-reader-read reader of
        (CsvReadByteRecord row error, next) -> raw-fields-equal (csv-byte-record-fields row) fields
          && raw-positions-equal row positions 0 && csv-byte-reader-input-offset next == offset
          && error-equal error kind start line column required count && raw-read-equal next rest
        other -> False
  -- | Compare streamed arbitrary-byte fields and positions across source chunk boundaries.
  raw-stream-equal :: forall r. using (io :: Io r) => CsvByteStreamReader r -> List RawExpected ->{IO r} Bool
  raw-stream-equal reader expected = case expected of
    Nil -> case csv-byte-stream-read reader of (Ok CsvByteEnd, next) -> True; other -> False
    Cons first rest -> case first of
      ExpectBytes fields positions offset kind start line column required count -> case csv-byte-stream-read reader of
        (Ok (CsvReadByteRecord row error), next) -> raw-fields-equal (csv-byte-record-fields row) fields
          && raw-positions-equal row positions 0 && csv-byte-stream-input-offset next == offset
          && error-equal error kind start line column required count && raw-stream-equal next rest
        other -> False
  -- | Observe lossless byte writing through the same inspectable sink and completion protocol.
  raw-write-equal :: forall r. using (io :: Io r) => CsvOptions -> Bool -> List Bytes -> Bytes ->{IO r} Bool
  raw-write-equal selected crlf fields expected = case csv-byte-stream-write (csv-stream-writer selected crlf writer-buffer) fields of
    (Ok unit, next) -> case writer-bytes (csv-stream-writer-sink next) of
      None -> False
      Some actual -> eq-i64 (bytes-compare actual expected) 0
    other -> False
'''


def text(value: str) -> str:
    """Encode scalar text with the source language's braced Unicode escape syntax."""
    encoded = json.dumps(value, ensure_ascii=False)
    return re.sub(r"\\u([0-9a-fA-F]{4})", lambda match: r"\u{" + match[1] + "}", encoded)


def values(items: list[str]) -> str:
    """Render a finite list of independently recorded immutable field strings."""
    return "[" + ",".join(map(text, items)) + "]"


def byte_value(encoded: str) -> str:
    """Construct independent arbitrary byte values directly from recorded hexadecimal octets."""
    return "(bytes-from-list [" + ",".join(str(value) for value in bytes.fromhex(encoded)) + "])"


def byte_fields(items: list[str]) -> str:
    """Preserve every raw field byte without relying on the codec under test."""
    return "[" + ",".join(map(byte_value, items)) + "]"


def configuration(item: dict) -> str:
    """Map the reference count flag to the explicit immutable field policy."""
    count = item["fields"]
    policy = "CsvAnyFields" if count < 0 else "CsvInferFields" if count == 0 else f"(CsvExactFields {count})"
    return f"(config {item['delimiter']} {item['comment']} {policy} {str(item['lazy'])} {str(item['trim'])})"


def program(fixture: dict, suite: str) -> str:
    """Create one executable consumer with a stable one-based failing fixture index."""
    checks = []
    if suite in ["read", "stream"]:
        for index, case in enumerate(fixture["read"]):
            expected = []
            for row in case["rows"]:
                positions = "[" + ",".join(f"({line},{column})" for line, column in row["positions"]) + "]"
                expected.append(f"(Expect {values(row['fields'])} {positions} {row['offset']} {text(row['kind'])} {row['start_line']} {row['line']} {row['column']} {row['expected']} {row['actual']})")
            wanted = "[" + ",".join(expected) + "]"
            if suite == "read":
                checks.append(f"read-equal (csv-record-reader {configuration(case['config'])} (runtime-text {text(case['input'])})) {wanted}")
            else:
                for chunk in [1, 7, 4096] if index % 7 == 0 else [1]:
                    checks.append(f"stream-read-equal (csv-stream-reader {configuration(case['config'])} (chunked {chunk} (reader-from-string (runtime-text {text(case['input'])})))) {wanted}")
    if suite in ["write", "stream"]:
        for row in fixture["write"]:
            selected = f"(config {row['delimiter']} 0 CsvAnyFields False False)"
            if suite == "write":
                checks.append(f"csv-render-record {selected} {row['crlf']} (runtime-fields {values(row['fields'])}) == {text(row['output'])}")
            else:
                checks.append(f"stream-write-equal {selected} {row['crlf']} (runtime-fields {values(row['fields'])}) {text(row['output'])}")
    if suite == "bytes":
        for index, case in enumerate(fixture["read_bytes"]):
            expected = []
            for row in case["rows"]:
                positions = "[" + ",".join(f"({line},{column})" for line, column in row["positions"]) + "]"
                expected.append(f"(ExpectBytes {byte_fields(row['fields_hex'])} {positions} {row['offset']} {text(row['kind'])} {row['start_line']} {row['line']} {row['column']} {row['expected']} {row['actual']})")
            wanted = "[" + ",".join(expected) + "]"
            selected = configuration(case["config"])
            input_bytes = f"(runtime-bytes {byte_value(case['input_hex'])})"
            checks.append(f"raw-read-equal (csv-byte-reader {selected} {input_bytes}) {wanted}")
            for chunk in [1, 7, 4096] if index % 7 == 0 else [1]:
                checks.append(f"raw-stream-equal (csv-byte-stream-reader {selected} (chunked {chunk} (reader-from-bytes {input_bytes}))) {wanted}")
        for row in fixture["write_bytes"]:
            selected = f"(config {row['delimiter']} 0 CsvAnyFields False False)"
            fields = f"(runtime-raw-fields {byte_fields(row['fields_hex'])})"
            expected = byte_value(row["output_hex"])
            checks.append(f"eq-i64 (bytes-compare (csv-render-byte-record {selected} {row['crlf']} {fields}) {expected}) 0")
            checks.append(f"raw-write-equal {selected} {row['crlf']} {fields} {expected}")
    helpers = HELPERS
    if suite in ["stream", "bytes"]:
        helpers = helpers.replace("  import Text.Csv.Record\n", "  import Text.Csv.Record\n  import Effect.Csv\n  import Effect.Stream\n") + STREAM_HELPERS
    if suite == "bytes":
        helpers += BYTE_HELPERS
    batches = []
    names = []
    for start in range(0, len(checks), 12):
        name = f"check-batch{start // 12}"
        names.append(f"{name} ()")
        signature = "forall r. using (io :: Io r) => Unit ->{IO r} Nat" if suite in ["stream", "bytes"] else "Unit -> Nat"
        body = ",\n      ".join(f"({check})" for check in checks[start:start + 12])
        batches.append(f"\n  -- | Evaluate a bounded group of independent {suite} fixtures with original failing indices.\n  {name} :: {signature}\n  {name} unit = first-failure [\n      {body}] {start + 1}\n")
    calls = ",".join(names)
    return helpers + "".join(batches) + f'''\n  -- | Print a failing fixture index and return a nonzero exit status for failed observations.
  main :: Unit ->{{IO r}} Int
  main unit = with (io = newIO ()) in
    let result = first-result [{calls}] in
    let printed = ioWrite (string-to-bytes (string-concat (nat-to-string result) "\\n")) in
    if result == 0 then 0 else 1
'''


def main() -> None:
    """Regenerate or check both fixture consumers without running a compiler."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    fixture = json.loads((FIXTURES / "csv-record-vectors.json").read_text())
    for suite in ["read", "write", "stream", "bytes"]:
        path = FIXTURES / f"csv-record-{suite}.cinch"
        expected = program(fixture, suite)
        if args.check:
            if not path.exists() or path.read_text() != expected:
                parser.exit(1, f"Stale generated CSV consumer: {path}\n")
        else:
            path.write_text(expected)


if __name__ == "__main__":
    main()
