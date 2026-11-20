#!/usr/bin/env python3
"""Generate addressability assertions from the retained independent method observations."""
from pathlib import Path
import hashlib
import json

ROOT = Path(__file__).resolve().parents[1]
reference = ROOT / 'seed/tests/fixtures/json-codec-methods-vectors.json'
rows = [row for row in json.loads(reference.read_text())['encoding'] if row['type'] == 'main.pointerMethods']
assert len(rows) == 80
assert all(row['bool'] for row in rows)
pending = [row for row in rows if row['error']['kind'] != 'none']
assert len(pending) == 1 and pending[0]['context'] == 'quoted'
assert pending[0]['error']['cause'] == 'invalid use of `string` tag option'
rows = [row for row in rows if row['error']['kind'] == 'none']
assert len(rows) == 79

header = r'''-- | Independent addressability observations exercise the same method/category decision through every container boundary.
module Main where
  import Text.Json.Codec
  import Text.Json.Value
  import Text.Json.Dynamic
  import Data.Array
  import Data.Machine

  -- | Keep application values opaque to frontend evaluation.
  prim runtime-bool :: Bool -> Bool where
    llvm {
      %result = or i8 %0, 0
      ret i8 %result
    }

  -- | Compare a successful complete encoding against an independently observed spelling.
  matches :: Result JsonError Bytes -> String -> Bool
  matches result expected = case result of Err error -> False; Ok output -> eq-i64 (bytes-compare output (string-to-bytes expected)) 0

  -- | Count each callback through a private test buffer without storing library continuation state.
  called :: Bytes -> Unit
  called counter =
    if bytes-len counter > 0 then
      let count = bytes-get counter 0 in
      if count < 255 then bytes-set counter 0 (count + 1) else ()
    else ()

  -- | Model an explicitly addressable-only JSON method over an ordinary bool.
  method-type :: Bytes -> JsonType Bool
  method-type counter = json-type-with-addressable-encoder json-bool-type (\options value -> let counted = called counter in Ok (string-to-bytes "\"pointer-json\""))

  -- | Seal one application field while preserving the containing addressability context.
  record-type :: Bool -> Bytes -> JsonType Bool
  record-type quoted counter = json-record-type "MethodRecord" (\unit -> False) not [json-field-with "value" (json-field-options False False quoted 0 False JsonFieldDefaultMatch) (method-type counter) (\value -> value) (\next prior -> next)]

  -- | Select independent method, reporting and quoted-field policies without changing defaults.
  options :: Bool -> Bool -> Bool -> JsonWriteOptions
  options methods reporting stringify = json-write-options True False "" "" (json-policies [json-addressable-methods-only methods, json-continue-after-type-errors reporting, json-quote-scalar-fields stringify])

'''

def literal(text):
    return json.dumps(text, ensure_ascii=False)

def boolean(value):
    return 'True' if value else 'False'

def expression(context):
    return {
        'value': 'json-marshal-with (method-type counter) config value',
        'pointer': 'json-marshal-with (json-nullable-type (method-type counter)) config (Some value)',
        'record': 'json-marshal-with (record-type False counter) config value',
        'record_pointer': 'json-marshal-with (json-nullable-type (record-type False counter)) config (Some value)',
        'quoted': 'json-marshal-with (record-type True counter) config value',
        'quoted_pointer': 'json-marshal-with (json-nullable-type (record-type True counter)) config (Some value)',
        'array': 'json-marshal-with (json-fixed-array-type (method-type counter) 1) config (array-from-list [value])',
        'array_pointer': 'json-marshal-with (json-nullable-type (json-fixed-array-type (method-type counter) 1)) config (Some (array-from-list [value]))',
        'slice': 'json-marshal-with (json-list-type (method-type counter)) config [value]',
        'map': 'json-marshal-with (json-map-type json-string-key (method-type counter)) config (json-map-from-list json-string-key [("value", value)])',
    }[context]

pieces = [header]
for at, row in enumerate(rows):
    expected = bytes.fromhex(row['output_hex']).decode()
    pieces.append(f'  -- | Reference observation {at + 1}: {row["context"]}, method/reporting/quoted policies {row["legacy_methods"]}/{row["legacy_errors"]}/{row["legacy_stringify"]}.\n')
    pieces.append(f'  observe{at} :: Unit -> Bool\n  observe{at} unit =\n')
    pieces.append('    let value = runtime-bool True in\n    let counter = bytes-new 1 in\n')
    pieces.append(f'    let config = options {boolean(row["legacy_methods"])} {boolean(row["legacy_errors"])} {boolean(row["legacy_stringify"])} in\n')
    pieces.append(f'    let output = {expression(row["context"])} in\n    matches output {literal(expected)} && (if bytes-len counter > 0 then bytes-get counter 0 == {len(row["calls"])} else False)\n\n')
# Typed payloads are an explicit interface boundary even when their parent options are addressable.
pieces.append(r'''  -- | Opaque typed values do not inherit the containing reference's addressability.
  interface-boundary :: Unit -> Bool
  interface-boundary unit =
    let counter = bytes-new 1 in
    let payload = json-any-typed (method-type counter) (runtime-bool True) in
    matches (json-marshal-with json-any-type (json-write-with-addressability json-default-write-options True) payload) "true"
      && matches (json-marshal-with json-any-type (options False True True) payload) "\"pointer-json\""
      && (if bytes-len counter > 0 then bytes-get counter 0 == 1 else False)

''')
for group in range(5):
    names = [f'observe{at} ()' for at in range(group * 16, min((group + 1) * 16, len(rows)))]
    pieces.append(f'  -- | Check independent addressability observations {group * 16 + 1} through {min((group + 1) * 16, len(rows))}.\n  group{group} :: Unit -> Bool\n  group{group} unit = ' + ' && '.join(names) + '\n\n')
pieces.append('  -- | Fail with the first independent group index or the explicit interface-boundary status.\n  main :: Int\n  main = ' + ''.join(f'if not (group{group} ()) then {group + 1} else ' for group in range(5)) + 'if not (interface-boundary ()) then 6 else 0\n')
output = ROOT / 'seed/tests/fixtures/json-codec-addressability.cinch'
output.write_text(''.join(pieces))
(ROOT / 'seed/tests/fixtures/json-codec-addressability-pending.json').write_text(json.dumps({'description': 'Mandatory full-scope reporting-disabled invalid quoted-field descriptor error; deferred to complete error-context increment, not asserted by the successful addressability consumer.', 'reference_sha256': hashlib.sha256(reference.read_bytes()).hexdigest(), 'rows': pending}, indent=2) + '\n')
print(json.dumps({'rows': len(rows), 'pending_error_rows': len(pending), 'extra_interface_group': 1, 'callback_counts_asserted': True, 'reference_sha256': hashlib.sha256(reference.read_bytes()).hexdigest(), 'fixture_sha256': hashlib.sha256(output.read_bytes()).hexdigest()}))
