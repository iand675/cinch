#!/usr/bin/env python3
"""Render deterministic public JSON consumers from independent reference fixture bytes."""
from __future__ import annotations

import argparse
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
FIXTURES = ROOT / "compiler/tests/fixtures"
HELPERS = r'''-- | Independent JSON value and formatting consumer with runtime inputs and complete byte comparisons.
module Main where
  import Text.Json.Value
  import Data.Machine
  -- | Keep input preparation live through front-end evaluation.
  prim runtime-size :: U32 -> U32 where
    llvm {
      %value = or i32 %0, 0
      ret i32 %value
    }
  -- | Observe a finite float's complete IEEE bit pattern without decimal rendering.
  prim float-bits :: F64 -> U64 where
    llvm {
      %value = bitcast double %0 to i64
      ret i64 %value
    }
  -- | Snapshot fixture bytes through a runtime-sized destination.
  runtime-bytes :: Bytes -> Bytes
  runtime-bytes input =
    let size = u32-to-nat (runtime-size (wrapping-i64-to-u32 (nat-to-i64 (bytes-len input)))) in
    let out = bytes-new size in
    if bytes-len input <= bytes-len out then let copied = bytes-copy-range out input 0 0 (bytes-len input) in out else bytes-new 0
  -- | Compare complete bytes, including embedded NUL and malformed UTF-8.
  same-bytes :: Bytes -> Bytes -> Bool
  same-bytes left right = eq-i64 (bytes-compare left right) 0
  -- | Compare syntax failures using original offsets and neutral JSON diagnostic text.
  syntax-error :: JsonError -> Nat -> String -> Bool
  syntax-error actual offset message = case actual of JsonSyntaxError at text -> at == offset && text == message; other -> False
  -- | Check validation and exact retained raw source for a complete valid document.
  valid-case :: Bytes -> Bytes -> Bool
  valid-case input raw = json-valid input && (case json-document input of Err error -> False; Ok document -> same-bytes (json-value-source (json-document-value document)) raw && same-bytes (json-document-input document) input)
  -- | Check named syntax failure before a document or output can escape.
  invalid-case :: Bytes -> Nat -> String -> Bool
  invalid-case input offset message = not (json-valid input) && (case json-document input of Err error -> syntax-error error offset message; Ok document -> False)
  -- | Compare the entire compact output or its exact original syntax refusal.
  compact-case :: Bytes -> Bytes -> Bool -> Nat -> String -> Bool
  compact-case input expected valid offset message = case json-compact input of
    Ok out -> valid && same-bytes out expected
    Err error -> not valid && syntax-error error offset message
  -- | Compare indentation, including arbitrary line prefixes and preserved trailing whitespace.
  indent-case :: Bytes -> String -> String -> Bytes -> Bool -> Nat -> String -> Bool
  indent-case input prefix step expected valid offset message = case json-indent input prefix step of
    Ok out -> valid && same-bytes out expected
    Err error -> not valid && syntax-error error offset message
  -- | Compare raw-byte indentation fragments independently of scalar text validity.
  indent-byte-case :: Bytes -> Bytes -> Bytes -> Bytes -> Bool -> Nat -> String -> Bool
  indent-byte-case input prefix step expected valid offset message = case json-indent-bytes input prefix step of
    Ok out -> valid && same-bytes out expected
    Err error -> not valid && syntax-error error offset message
  -- | Compare quoted arbitrary bytes and their decoded replacement-scalar text.
  quote-case :: Bytes -> Bool -> Bytes -> Bytes -> Bool
  quote-case input html expected decoded =
    let out = json-quote-bytes input html in same-bytes out expected && (case json-document out of
      Err error -> False
      Ok document -> case json-value-string (json-document-value document) of None -> False; Some text -> same-bytes (string-to-bytes text) decoded)
  -- | Compare valid number spellings and integer/float conversions without a rounding-based oracle.
  number-case :: String -> Bool -> I64 -> Bool -> U64 -> Bool
  number-case text integer-ok integer float-ok bits = case json-number text of
    Err error -> False
    Ok number -> json-number-text number == text
      && (case json-number-int64 number of Ok actual -> integer-ok && eq-i64 actual integer; Err error -> not integer-ok)
      && (case json-number-float64 number of Ok actual -> float-ok && eq-u64 (float-bits actual) bits; Err error -> not float-ok)
  -- | Reject malformed number spellings at construction rather than admitting an invalid number value.
  number-invalid :: String -> Bool
  number-invalid text = case json-number text of Err error -> True; Ok value -> False
  -- | Preserve a failing fixture index within a bounded group.
  first-failure :: List Bool -> Nat -> Nat
  first-failure checks index = case checks of Nil -> 0; Cons first rest -> if first then first-failure rest (index + 1) else index
  -- | Preserve the first failing group without constructing one enormous evaluation body.
  first-result :: List Nat -> Nat
  first-result groups = case groups of Nil -> 0; Cons first rest -> if first > 0 then first else first-result rest
'''


def text(value: str) -> str:
    """Encode scalar fixture text using supported string escapes rather than JSON-only Unicode syntax."""
    out = '"'
    for char in value:
        n = ord(char)
        out += {34: '\\"', 92: '\\\\', 10: '\\n', 13: '\\r', 9: '\\t'}.get(n, char if n >= 32 else f"\\u{{{n:x}}}")
    return out + '"'


def wire(value: str) -> str:
    """Render arbitrary reference bytes without interpreting them as text."""
    data = bytes.fromhex(value)
    try:
        decoded = data.decode("utf-8")
    except UnicodeDecodeError:
        return "(bytes-from-list [" + ",".join(str(x) for x in data) + "])"
    return "(string-to-bytes " + text(decoded) + ")"


def boolean(value: bool) -> str:
    """Use the language's explicit boolean constructors."""
    return "True" if value else "False"


def program(suite: str, fixture: dict) -> str:
    """Render complete bounded consumer groups for one implemented core operation family."""
    checks = []
    if suite == "syntax":
        for row in fixture["syntax"]:
            raw = f"(runtime-bytes {wire(row['input_hex'])})"
            error = row["error"]
            if row["valid"]:
                checks.append(f"valid-case {raw} {wire(row['raw_hex'])}")
            else:
                checks.append(f"invalid-case {raw} {error['offset']} {text(error['text'])}")
            err = row["compact_error"]
            checks.append(f"compact-case {raw} {wire(row['compact_hex'])} {boolean(not err['kind'])} {err['offset']} {text(err.get('text', ''))}")
            checks.append(f"same-bytes (json-html-escape {raw}) {wire(row['html_hex'])}")
            for item in row["indents"]:
                err = item["error"]
                checks.append(f"indent-case {raw} {text(item['prefix'])} {text(item['step'])} {wire(item['output_hex'])} {boolean(not err['kind'])} {err['offset']} {text(err.get('text', ''))}")
    elif suite == "indent":
        for row in fixture["indent_bytes"]:
            err = row["error"]
            checks.append(f"indent-byte-case (runtime-bytes {wire(row['input_hex'])}) {wire(row['prefix_hex'])} {wire(row['step_hex'])} {wire(row['output_hex'])} {boolean(not err['kind'])} {err['offset']} {text(err.get('text', ''))}")
    elif suite == "quote":
        for row in fixture["quote"]:
            for item in row["results"]:
                checks.append(f"quote-case (runtime-bytes {wire(row['input_hex'])}) {boolean(item['html'])} {wire(item['output_hex'])} {wire(item['decoded_hex'])}")
    elif suite == "number":
        for row in fixture["number"]:
            # The opaque number factory accepts grammar-valid numbers; malformed raw casts are refused early.
            if row["text"] and not row["marshal_error"]["kind"]:
                integer_ok = not row["integer_error"]["kind"]
                floating_ok = not row["float_error"]["kind"]
                integer = row["integer"] if integer_ok else "0"
                checks.append(f"number-case {text(row['text'])} {boolean(integer_ok)} ({integer} :: I64) {boolean(floating_ok)} ({int(row['float_bits'], 16)} :: U64)")
            else:
                checks.append(f"number-invalid {text(row['text'])}")
    else:
        raise ValueError(f"consumer suite is not implemented: {suite}")
    batches = []
    names = []
    for start in range(0, len(checks), 6):
        name = f"check-batch{start // 6}"
        names.append(name + " ()")
        body = ",\n      ".join("(" + check + ")" for check in checks[start:start + 6])
        batches.append(f"\n  -- | Evaluate bounded independent {suite} checks with global failure indexes.\n  {name} :: Unit -> Nat\n  {name} unit = first-failure [\n      {body}] {start + 1}\n")
    return HELPERS + "".join(batches) + f'''
  -- | Print the original failing observation and return a nonzero status on failure.
  main :: Unit ->{{IO r}} Int
  main unit = with (io = newIO ()) in
    let failed = first-result [{','.join(names)}] in
    let printed = ioWrite (string-to-bytes (string-concat (nat-to-string failed) "\\n")) in
    if failed == 0 then 0 else 1
'''


def main() -> None:
    """Regenerate or verify every implemented core consumer without running a compiler."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    fixture = json.loads((FIXTURES / "json-codec-vectors.json").read_text())
    for suite in ["syntax", "quote", "number", "indent"]:
        path = FIXTURES / f"json-codec-{suite}.cinch"
        contents = program(suite, fixture)
        if args.check:
            if path.read_text() != contents:
                raise SystemExit(f"stale generated consumer: {path}")
        else:
            path.write_text(contents)


if __name__ == "__main__":
    main()
