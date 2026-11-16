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


TYPED_HELPERS = r'''
  -- | Application record used by independent field update/omission observations.
  data ApplicationRecord where
    -- | Identifier, display text, enabled flag, quoted integer and omitted-zero integer.
    App :: I64 -> String -> Bool -> I64 -> I64 -> ApplicationRecord
  -- | Read the application's identifier field.
  app-id :: ApplicationRecord -> I64
  app-id value = case value of App ident name enabled quoted zero -> ident
  -- | Functionally install an identifier while preserving unrelated fields.
  app-id-set :: I64 -> ApplicationRecord -> ApplicationRecord
  app-id-set next value = case value of App ident name enabled quoted zero -> App next name enabled quoted zero
  -- | Read the application's display name.
  app-name :: ApplicationRecord -> String
  app-name value = case value of App ident name enabled quoted zero -> name
  -- | Functionally install display text while preserving unrelated fields.
  app-name-set :: String -> ApplicationRecord -> ApplicationRecord
  app-name-set next value = case value of App ident name enabled quoted zero -> App ident next enabled quoted zero
  -- | Read the application's enabled flag.
  app-enabled :: ApplicationRecord -> Bool
  app-enabled value = case value of App ident name enabled quoted zero -> enabled
  -- | Functionally install the enabled flag while preserving unrelated fields.
  app-enabled-set :: Bool -> ApplicationRecord -> ApplicationRecord
  app-enabled-set next value = case value of App ident name enabled quoted zero -> App ident name next quoted zero
  -- | Read the quoted integer field.
  app-quoted :: ApplicationRecord -> I64
  app-quoted value = case value of App ident name enabled quoted zero -> quoted
  -- | Functionally install the quoted integer while preserving unrelated fields.
  app-quoted-set :: I64 -> ApplicationRecord -> ApplicationRecord
  app-quoted-set next value = case value of App ident name enabled quoted zero -> App ident name enabled next zero
  -- | Read the omitted-zero integer field.
  app-zero :: ApplicationRecord -> I64
  app-zero value = case value of App ident name enabled quoted zero -> zero
  -- | Functionally install the omitted-zero field while preserving unrelated fields.
  app-zero-set :: I64 -> ApplicationRecord -> ApplicationRecord
  app-zero-set next value = case value of App ident name enabled quoted zero -> App ident name enabled quoted next
  -- | Construct an independent application zero value.
  app-default :: Unit -> ApplicationRecord
  app-default unit = App 0 "" False 0 0
  -- | Classify the application's actual complete zero value.
  app-is-zero :: ApplicationRecord -> Bool
  app-is-zero value = case value of App ident name enabled quoted zero -> eq-i64 ident 0 && name == "" && not enabled && eq-i64 quoted 0 && eq-i64 zero 0
  -- | Describe explicit application fields with omission, naming and quoted-number metadata.
  app-type :: Unit -> JsonType ApplicationRecord
  app-type unit = json-record-type "ApplicationRecord" app-default app-is-zero [
    json-field-with "id" (json-field-options False False False 0 True JsonFieldDefaultMatch) json-int64-type app-id app-id-set,
    json-field-with "name" (json-field-options False False False 0 True JsonFieldDefaultMatch) json-string-type app-name app-name-set,
    json-field-with "enabled" (json-field-options True False False 0 True JsonFieldDefaultMatch) json-bool-type app-enabled app-enabled-set,
    json-field-with "quoted" (json-field-options False False True 0 True JsonFieldDefaultMatch) json-int64-type app-quoted app-quoted-set,
    json-field-with "zero" (json-field-options False True False 0 True JsonFieldDefaultMatch) json-int64-type app-zero app-zero-set]
  -- | Construct a known valid initial exact number through the public checked constructor.
  initial-number :: Unit -> JsonNumber
  initial-number unit = case json-number "7" of Ok value -> value; Err error -> json-empty-number
  -- | Inspect the semantic error family without conflating path context with an underlying cause.
  error-family :: JsonError -> String
  error-family error = case error of
    JsonSyntaxError at text -> "syntax"
    JsonTypeError expected actual at path -> "type"
    JsonTypeCause expected actual at path cause -> "type"
    JsonPathError path cause -> error-family cause
    JsonCustomError label cause -> "custom"
    JsonValueError label text -> "value"
    other -> "other"
  -- | Compare the complete remarshal of a partially updated value.
  typed-value-ok :: forall a. JsonType a -> a -> Bytes -> Bool
  typed-value-ok descriptor value expected = case json-marshal descriptor value of
    Ok output -> same-bytes output expected
    Err error -> False
  -- | Treat absent optional diagnostic metadata as the empty reference field.
  optional-text :: Option String -> String
  optional-text value = case value of None -> ""; Some text -> text
  -- | Compare the original structured error independently of conversion success.
  typed-error-ok :: Option JsonError -> String -> Nat -> String -> String -> String -> Bool
  typed-error-ok error family offset actual expected-type path = case error of
    None -> family == ""
    Some value -> error-family value == family && json-error-offset value == offset && optional-text (json-error-value value) == actual && optional-text (json-error-type value) == expected-type && json-error-path value == path
  -- | Compare complete partial-update state and original error metadata against an independent observation.
  typed-case :: forall a. JsonType a -> Bytes -> a -> Bytes -> String -> Nat -> String -> String -> String -> Bool
  typed-case descriptor input prior expected family offset actual expected-type path =
    let decoded = json-update descriptor (runtime-bytes input) prior in
    typed-value-ok descriptor (json-decoded-value decoded) expected && typed-error-ok (json-decoded-error decoded) family offset actual expected-type path
'''

FLOAT_HELPERS = r'''
  -- | Reconstruct an independently supplied binary64 payload without decimal evaluation.
  prim float-from-bits :: U64 -> F64 where
    llvm {
      %value = bitcast i64 %0 to double
      ret double %value
    }
  -- | Compare exact shortest JSON output or the structured unsupported nonfinite value.
  float-encode-case :: U64 -> Bytes -> String -> String -> Bool
  float-encode-case bits expected family description = case json-marshal json-float64-type (float-from-bits bits) of
    Ok output -> family == "" && same-bytes output expected
    Err error -> error-family error == family && optional-text (json-error-value error) == description
'''


FIELD_HELPERS = r'''
  -- | Resolve a runtime-constructed field descriptor and compare the complete independent result.
  field-case :: String -> Bytes -> I64 -> Bytes -> Bool
  field-case name input expected output =
    let descriptor = json-record-type "Named" (\unit -> 0) (\value -> eq-i64 value 0) [json-field name json-int64-type (\value -> value) (\value prior -> value)] in
    let decoded = json-update descriptor (runtime-bytes input) 0 in
    eq-i64 (json-decoded-value decoded) expected && (case json-decoded-error decoded of None -> True; Some error -> False) && typed-value-ok descriptor (json-decoded-value decoded) output
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
    elif suite == "field":
        for row in fixture["field_matching"]:
            assert not row["error"]["kind"] and not row["output_error"]["kind"]
            checks.append(f"field-case {text(row['name'])} {wire(row['input_hex'])} ({row['value']} :: I64) {wire(row['output_hex'])}")
    elif suite in {"typed", "float"}:
        descriptors = {
            "F64": ("json-float64-type", "(i64-to-f64 7)"),
            "Bool": ("json-bool-type", "True"),
            "String": ("json-string-type", '"before"'),
            "I64": ("json-int64-type", "(7 :: I64)"),
            "JsonNumber": ("json-number-type", "(initial-number ())"),
            "List I64": ("(json-slice-type json-int64-type)", "(Some [7,8,9])"),
            "Array3 I64": ("(json-fixed-array-type json-int64-type 3)", "(array-from-list [7,8,9])"),
            "JsonRawMessage": ("json-raw-type", '(json-raw-from-bytes (string-to-bytes "{\\\"before\\\":true}"))'),
            "ApplicationRecord": ("(app-type ())", '(App 7 "before" False 9 0)'),
        }
        labels = {"float64":"F64", "bool":"Bool", "string":"String", "int64":"I64", "json.Number":"JsonNumber", "[]int64":"List I64", "[3]int64":"Array", "main.applicationRecord":"ApplicationRecord"}
        for row in fixture["typed"]:
            if row["type"] not in descriptors:
                continue
            descriptor, initial = descriptors[row["type"]]
            err = row["error"]
            actual = err.get("value", "")
            if actual == "bool":
                actual = "boolean"
            field = err.get("field", "")
            if row["type"] in ["List I64", "Array3 I64"] and field.isdigit():
                field = "[" + field + "]"
            checks.append(f"typed-case {descriptor} {wire(row['input_hex'])} {initial} {wire(row['output_hex'])} {text(err['kind'])} {err['offset']} {text(actual)} {text(labels.get(err.get('type',''),err.get('type','')))} {text(field)}")
        if suite == "float":
            for row in fixture["float_encoding"]:
                err = row["error"]
                checks.append(f"float-encode-case ({int(row['bits'], 16)} :: U64) {wire(row['output_hex'])} {text(err['kind'])} {text(err.get('value',''))}")
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
    helpers = HELPERS
    if suite in {"typed", "float", "field"}:
        helpers = helpers.replace("  import Text.Json.Value\n", "  import Text.Json.Value\n  import Text.Json.Codec\n  import Data.Array\n") + TYPED_HELPERS + (FLOAT_HELPERS if suite == "float" else FIELD_HELPERS if suite == "field" else "")
    return helpers + "".join(batches) + f'''
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
    parser.add_argument("--suites", nargs="+", default=["syntax", "quote", "number", "indent", "typed"])
    args = parser.parse_args()
    fixture = json.loads((FIXTURES / "json-codec-vectors.json").read_text())
    for suite in args.suites:
        selected = json.loads((FIXTURES / f"json-codec-{suite}-vectors.json").read_text()) if suite in {"float", "field"} else fixture
        path = FIXTURES / f"json-codec-{suite}.cinch"
        contents = program(suite, selected)
        if args.check:
            if path.read_text() != contents:
                raise SystemExit(f"stale generated consumer: {path}")
        else:
            path.write_text(contents)


if __name__ == "__main__":
    main()
