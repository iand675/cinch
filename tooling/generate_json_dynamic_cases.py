#!/usr/bin/env python3
"""Render independent dynamic observations as public API consumers with runtime-opaque inputs."""
from __future__ import annotations

import argparse
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
FIXTURES = ROOT / "compiler/tests/fixtures"

HELPERS = r'''-- | Independent dynamic values, partial errors and complete encoding observations.
module Main where
  import Text.Json.Value
  import Text.Json.Codec
  import Text.Json.Dynamic
  import Data.Machine
  -- | Keep fixture input sizes opaque to frontend evaluation.
  prim runtime-size :: U32 -> U32 where
    llvm {
      %result = or i32 %0, 0
      ret i32 %result
    }
  -- | Construct a precise independently observed binary64 payload.
  prim from-bits :: U64 -> F64 where
    llvm {
      %result = bitcast i64 %0 to double
      ret double %result
    }
  -- | Observe every binary64 bit, including signed zero and subnormals.
  prim float-bits :: F64 -> U64 where
    llvm {
      %result = bitcast double %0 to i64
      ret i64 %result
    }
  -- | Materialize runtime-owned inputs without exposing fixture constants to partial evaluation.
  runtime-bytes :: Bytes -> Bytes
  runtime-bytes input =
    let count = u32-to-nat (runtime-size (wrapping-i64-to-u32 (nat-to-i64 (bytes-len input)))) in
    let output = bytes-new count in
    if bytes-len input <= bytes-len output then let copied = bytes-copy-range output input 0 0 (bytes-len input) in output else bytes-new 0
  -- | Compare complete arbitrary bytes, including malformed text and embedded NUL.
  same :: Bytes -> Bytes -> Bool
  same left right = eq-i64 (bytes-compare left right) 0
  -- | Compare an optional scalar label without discarding absence.
  optional-is :: Option String -> String -> Bool
  optional-is actual expected = case actual of None -> expected == ""; Some text -> text == expected
  -- | Count an already materialized list independently of its element type.
  count :: forall a. List a -> Nat -> Nat
  count values total = case values of Nil -> total; Cons first rest -> count rest (total + 1)
  -- | Queue corresponding array children after checking their exact arity.
  array-pairs :: List JsonAny -> List JsonAny -> List (JsonAny, JsonAny) -> Option (List (JsonAny, JsonAny))
  array-pairs left right pending = case left of
    Nil -> case right of Nil -> Some pending; other -> None
    Cons first rest -> case right of Nil -> None; Cons expected more -> array-pairs rest more (Cons (first, expected) pending)
  -- | Queue object children only when the exact raw byte names and member counts agree.
  object-pairs :: List (Bytes, JsonAny) -> List (Bytes, JsonAny) -> List (JsonAny, JsonAny) -> Option (List (JsonAny, JsonAny))
  object-pairs left right pending = case left of
    Nil -> case right of Nil -> Some pending; other -> None
    Cons first rest -> case right of
      Nil -> None
      Cons expected more -> case first of (name, value) -> case expected of (wanted, child) -> if same name wanted then object-pairs rest more (Cons (value, child) pending) else None
  -- | Compare dynamic representations using explicit work, preserving float/exact-number and null-container distinctions.
  equal-values :: List (JsonAny, JsonAny) -> Bool
  equal-values pending = case pending of
    Nil -> True
    Cons pair rest -> case pair of
      (actual, expected) -> case json-any-kind actual of
        JsonNullKind -> json-any-is-null expected && equal-values rest
        JsonBooleanKind -> (case json-any-as-bool actual of None -> False; Some value -> case json-any-as-bool expected of None -> False; Some wanted -> value == wanted) && equal-values rest
        JsonNumberKind -> (case json-any-as-number actual of
          Some value -> case json-any-as-number expected of None -> False; Some wanted -> same (json-number-bytes value) (json-number-bytes wanted)
          None -> case json-any-as-float64 actual of None -> False; Some value -> case json-any-as-float64 expected of None -> False; Some wanted -> if not (feq-f64 value value) then not (feq-f64 wanted wanted) else eq-u64 (float-bits value) (float-bits wanted)) && equal-values rest
        JsonStringKind -> (case json-any-as-bytes actual of None -> False; Some value -> case json-any-as-bytes expected of None -> False; Some wanted -> same value wanted) && equal-values rest
        JsonArrayKind -> json-any-container-is-null actual == json-any-container-is-null expected && (case json-any-array-values actual of None -> False; Some values -> case json-any-array-values expected of None -> False; Some wanted -> case array-pairs values wanted rest of None -> False; Some more -> equal-values more)
        JsonObjectKind -> json-any-container-is-null actual == json-any-container-is-null expected && (case json-any-object-members actual of None -> False; Some values -> case json-any-object-members expected of None -> False; Some wanted -> case object-pairs values wanted rest of None -> False; Some more -> equal-values more)
        other -> False
  -- | Independent error metadata: family, offset, native label, value description, path and exact semantic token.
  data ErrorSpec where
    -- | Explicit reference fields, with independently adapted native type labels.
    ErrorSpec :: Nat -> Nat -> String -> String -> String -> Bytes -> ErrorSpec
  -- | Remove containing-path context while preserving a custom conversion wrapper.
  error-family :: JsonError -> Nat
  error-family error = case error of
    JsonPathError path cause -> error-family cause
    JsonSyntaxError at text -> 1
    JsonTypeError label value at path -> 2
    JsonTypeCause label value at path cause -> 2
    JsonSemanticError info cause -> 3
    JsonCustomError label cause -> 4
    JsonValueError label message -> 5
    other -> 99
  -- | Check original error family and every stable structured field rather than only success status.
  error-matches :: Option JsonError -> ErrorSpec -> Bool
  error-matches actual expected = case expected of
    ErrorSpec kind offset label value path token -> case actual of
      None -> kind == 0
      Some error -> kind == error-family error && json-error-offset error == offset
        && (if kind == 1 then True else optional-is (json-error-type error) label)
        && (if kind == 2 || kind == 3 || kind == 5 then optional-is (json-error-value error) value else True)
        && json-error-path error == path
        && (if kind == 3 then (case json-error-semantic error of None -> False; Some info -> json-semantic-action info == "unmarshal" && json-semantic-type info == label && json-semantic-offset info == offset && json-semantic-pointer info == path && same (json-semantic-bytes info) token) else True)
  -- | Configure a single present dynamic field with established empty-field omission.
  wrapped-type :: JsonType JsonAny
  wrapped-type = json-record-type "DynamicRecord" (\unit -> json-any-null) json-any-is-null [json-field-with "value" (json-field-options True False False 0 False JsonFieldDefaultMatch) json-any-type (\value -> value) (\value prior -> value)]
  -- | Compare one complete public encoding or its exact structured refusal.
  encoding-matches :: JsonType JsonAny -> JsonAny -> Bool -> Bytes -> ErrorSpec -> Bool
  encoding-matches descriptor value html output expected = case json-marshal-with descriptor (json-write-options html False "" "" json-default-policies) value of
    Ok actual -> error-matches None expected && same actual output
    Err error -> error-matches (Some error) expected
  -- | Independent compact output and containing-field omission for both HTML policies.
  data EncodingSpec where
    -- | Escape policy, complete output/refusal and complete containing-record output/refusal.
    EncodingSpec :: Bool -> Bytes -> ErrorSpec -> Bytes -> ErrorSpec -> EncodingSpec
  -- | Compare independent complete encodings without observing partial fragments on refusal.
  encodings-match :: JsonAny -> List EncodingSpec -> Bool
  encodings-match value observations = case observations of
    Nil -> True
    Cons first rest -> case first of EncodingSpec html output error wrapped wrapped-error -> encoding-matches json-any-type value html output error && encoding-matches wrapped-type value True wrapped wrapped-error && encodings-match value rest
  -- | Construct each prior independently, including typed null collections distinct from absent payloads.
  prior-value :: Nat -> JsonAny
  prior-value kind = if kind == 0 then json-any-null else if kind == 1 then json-any-bool True
    else if kind == 2 then json-any-float64 (from-bits 13843220829606772736) else if kind == 3 then json-any-string "prior"
    else if kind == 4 then json-any-array [json-any-bool True, json-any-string "prior", json-any-object [(string-to-bytes "old", json-any-string "kept")]]
    else if kind == 5 then json-any-object [(string-to-bytes "old", json-any-string "kept"), (string-to-bytes "a", json-any-object [(string-to-bytes "old", json-any-bool True)])]
    else if kind == 6 then json-any-null-array else json-any-null-object
  -- | Check default or exact-number decoding, complete partial shape, error metadata and every remarshal observation.
  decode-case :: Bytes -> Nat -> Bool -> Bool -> JsonAny -> ErrorSpec -> List EncodingSpec -> Bool
  decode-case input prior numbers continue expected error encodings =
    let options = json-read-options numbers False (json-policies [json-continue-after-type-errors continue]) in
    let decoded = json-update-with json-any-type options (runtime-bytes input) (prior-value prior) in
    equal-values [(json-decoded-value decoded, expected)] && error-matches (json-decoded-error decoded) error && encodings-match (json-decoded-value decoded) encodings
  -- | Check a directly constructed payload, its exact representation and all independently observed encoding policies.
  application-case :: JsonAny -> List EncodingSpec -> Bool
  application-case value encodings = encodings-match value encodings
  -- | Stop on the first failed observation while preserving its positive index.
  first-failure :: List Bool -> Nat -> Nat
  first-failure results index = case results of Nil -> 0; Cons first rest -> if first then first-failure rest (index + 1) else index
  -- | Preserve a positive failure index across bounded generated groups.
  first-result :: List Nat -> Nat
  first-result results = case results of Nil -> 0; Cons first rest -> if first == 0 then first-result rest else first
'''


def scalar(value: str) -> str:
    """Encode a scalar literal using Cinch escapes and retain non-ASCII scalar text."""
    pieces = []
    for character in value:
        code = ord(character)
        if character == '"':
            pieces.append('\\"')
        elif character == "\\":
            pieces.append("\\\\")
        elif character == "\n":
            pieces.append("\\n")
        elif character == "\r":
            pieces.append("\\r")
        elif character == "\t":
            pieces.append("\\t")
        elif code < 32 or code in (127, 0x2028, 0x2029):
            pieces.append(f"\\u{{{code:x}}}")
        else:
            if 0xD800 <= code <= 0xDFFF:
                raise ValueError("a reference metadata string is not scalar Unicode")
            pieces.append(character)
    return '"' + "".join(pieces) + '"'


def binary(value: str) -> str:
    """Render exact bytes without requiring the source literal to contain malformed UTF-8."""
    return "(bytes-from-list [" + ",".join(str(byte) for byte in bytes.fromhex(value)) + "])"


def payload(nodes: list[dict], offset: int = 0) -> tuple[str, int]:
    """Build a small independently observed application payload from its flat reference traversal."""
    node = nodes[offset]
    kind = node["kind"]
    offset += 1
    if kind == "null":
        return "json-any-null", offset
    if kind == "bool":
        return f"(json-any-bool {node['value']})", offset
    if kind == "float":
        return f"(json-any-float64 (from-bits {int(node['bits'], 16)}))", offset
    if kind == "number":
        return f"(json-any-number (json-number-from-bytes {binary(node['hex'])}))", offset
    if kind == "string":
        return f"(json-any-bytes {binary(node['hex'])})", offset
    if kind == "array":
        values = []
        for _ in range(node["length"]):
            child, offset = payload(nodes, offset)
            values.append(child)
        assert nodes[offset]["kind"] == "end-array"
        return ("json-any-null-array" if node["nil"] else "(json-any-array [" + ", ".join(values) + "])"), offset + 1
    if kind == "object":
        values = []
        for _ in range(node["length"]):
            name = nodes[offset]
            assert name["kind"] == "key"
            child, offset = payload(nodes, offset + 1)
            values.append(f"({binary(name['hex'])}, {child})")
        assert nodes[offset]["kind"] == "end-object"
        return ("json-any-null-object" if node["nil"] else "(json-any-object [" + ", ".join(values) + "])"), offset + 1
    raise ValueError(node)


def failure(value: dict) -> str:
    """Adapt only native type spelling while retaining actual source offsets, values and paths."""
    kinds = {"none": 0, "*json.SyntaxError": 1, "*json.UnmarshalTypeError": 2,
             "*json.SemanticError": 3, "*json.MarshalerError": 4, "*json.UnsupportedValueError": 5}
    kind = kinds[value["kind"]]
    label = {"float64": "F64", "*json.Number": "JsonNumber"}.get(value.get("type", ""), value.get("type", ""))
    if kind == 5:
        label = "F64"
    actual = value.get("value", "")
    if kind == 3:
        actual = "number " + bytes.fromhex(value["value_hex"]).decode()
        if "pointer_hex" in value:
            assert bytes.fromhex(value["pointer_hex"]).decode("utf-8") == value["pointer"]
    path = value.get("pointer", value.get("field", ""))
    return f"(ErrorSpec {kind} {value.get('offset', 0)} {scalar(label)} {scalar(actual)} {scalar(path)} {binary(value.get('value_hex', ''))})"


def encodings(observations: list[dict]) -> str:
    """Retain both complete HTML modes and complete containing-field omission outputs."""
    rows = []
    for row in observations:
        wrapped = dict(row["omitted_error"])
        if wrapped["kind"] != "none":
            # Native explicit field descriptors also retain the containing field path.
            wrapped["field"] = "value"
        rows.append(f"(EncodingSpec {row['html']} {binary(row['output_hex'])} {failure(row['error'])} {binary(row['omitted_hex'])} {failure(wrapped)})")
    return "[" + ", ".join(rows) + "]"


def render(edge: bool = False) -> str:
    """Produce deterministic bounded groups from every independent dynamic observation."""
    reference = json.loads((FIXTURES / ("json-codec-dynamic-edge-vectors.json" if edge else "json-codec-dynamic-vectors.json")).read_text())
    priors = {name: index for index, name in enumerate(["null", "bool", "float", "string", "array", "object", "nil-array", "nil-object"])}
    checks = []
    for category in ["decode", "abort"]:
        for row in reference[category]:
            expected, end = payload(row["nodes"])
            assert end == len(row["nodes"])
            checks.append(f"decode-case {binary(row['input_hex'])} {priors[row['prior']]} {row.get('use_number', False)} {category == 'decode'} {expected} {failure(row['error'])} {encodings(row.get('marshal', []))}")
    for row in reference["application"]:
        expected, end = payload(row["nodes"])
        assert end == len(row["nodes"])
        checks.append(f"application-case {expected} {encodings(row['marshal'])}")
    result = HELPERS
    groups = []
    for start in range(0, len(checks), 16):
        name = f"group{start // 16}"
        groups.append(name + " ()")
        result += f"\n  -- | Independent observations {start + 1} through {min(start + 16, len(checks))}.\n  {name} :: Unit -> Nat\n  {name} unit = first-failure [\n    " + ",\n    ".join(checks[start:start + 16]) + f"\n    ] {start + 1}\n"
    result += "\n  -- | Normalize any positive observation failure to a nonzero process status.\n  main :: Int\n  main = if first-result [" + ", ".join(groups) + "] == 0 then 0 else 1\n"
    return result


def main() -> None:
    """Write the consumer or check that every byte remains fresh against the independent reference."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true")
    parser.add_argument("--edge", action="store_true")
    args = parser.parse_args()
    path = FIXTURES / ("json-codec-dynamic-edge.cinch" if args.edge else "json-codec-dynamic.cinch")
    result = render(args.edge)
    if args.check:
        if path.read_text() != result:
            raise SystemExit("dynamic consumer is stale")
    else:
        path.write_text(result)


if __name__ == "__main__":
    main()
