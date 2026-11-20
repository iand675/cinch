#!/usr/bin/env python3
from pathlib import Path
import json,hashlib
ROOT=Path('/workspace/dasl-stdlib-json')
REFERENCE=ROOT/'compiler/tests/fixtures/json-codec-error-context-vectors.json'
TEMPLATE=r'''-- | Independent complete read observations retain partial state, exact error identity and callback bytes.
module Main where
  import Text.Json.Value
  import Text.Json.Codec
  import Data.Machine

  -- | Keep complete input bytes opaque to frontend evaluation.
  prim runtime-size :: U32 -> U32 where
    llvm {
      %result = or i32 %0, 0
      ret i32 %result
    }

  -- | Copy source bytes into a runtime-sized independent buffer.
  runtime-bytes :: Bytes -> Bytes
  runtime-bytes input =
    let size = u32-to-nat (runtime-size (wrapping-i64-to-u32 (nat-to-i64 (bytes-len input)))) in
    let output = bytes-new size in
    if bytes-len input <= bytes-len output then let copied = bytes-copy-range output input 0 0 (bytes-len input) in output else bytes-new 0

  -- | Application scalar, later field and prior text are independently retained across a stopped update.
  data Application where
    -- | Current scalar, later integer and leading text field.
    App :: Bool -> I64 -> String -> Application

  -- | Construct the independently observed prior state.
  initial :: Unit -> Application
  initial unit = App False 9 "prior"

  -- | Inspect the application scalar.
  scalar :: Application -> Bool
  scalar value = case value of App flag after before -> flag

  -- | Inspect the later application field.
  later :: Application -> I64
  later value = case value of App flag after before -> after

  -- | Inspect the earlier application field.
  earlier :: Application -> String
  earlier value = case value of App flag after before -> before

  -- | Replace only the scalar while preserving unrelated fields.
  set-scalar :: Bool -> Application -> Application
  set-scalar next prior = case prior of App flag after before -> App next after before

  -- | Replace only the later integer field.
  set-later :: I64 -> Application -> Application
  set-later next prior = case prior of App flag after before -> App flag next before

  -- | Replace only the earlier text field.
  set-earlier :: String -> Application -> Application
  set-earlier next prior = case prior of App flag after before -> App flag after next

  -- | Delegate the ordinary scalar representation without reconstructing its conversion rules in the fixture.
  ordinary-update :: JsonReadOptions -> JsonValue -> Application -> JsonDecoded Application
  ordinary-update options source prior =
    let decoded = json-update-value json-bool-type options source (scalar prior) in
    let value = set-scalar (json-decoded-value decoded) prior in
    case json-decoded-error decoded of
      None -> json-decoded-ok value
      Some error -> if json-decoded-fatal decoded then json-decoded-failure value error else json-decoded-with-error value error

  -- | Count calls and retain the exact bytes passed to an application method.
  remember :: Bytes -> Bytes -> Unit
  remember counter input =
    if bytes-len counter == 16 && bytes-len input <= 14 then
      let count = bytes-get counter 0 in
      if count < 255 then
        let counted = bytes-set counter 0 (count + 1) in
        let sized = bytes-set counter 15 (bytes-len input) in
        let copied = bytes-copy-range counter input 1 0 (bytes-len input) in ()
      else ()
    else ()

  -- | Preserve the original application refusal and genuinely absent semantic type.
  returned :: JsonError
  returned = JsonSemanticError (json-semantic-info-with-optional-type "" 2 "/inner~0" (string-to-bytes "12") None 48 (string-to-bytes "12")) (JsonEncodingError 0 "method-refusal")

  -- | Observe raw JSON method input before returning an advanced scalar and its original cause.
  json-read :: Bytes -> JsonReadOptions -> JsonValue -> Application -> JsonDecoded Application
  json-read counter options source prior = let called = remember counter (json-value-source source) in json-decoded-with-error (set-scalar True prior) returned

  -- | Observe decoded text method input before returning an advanced scalar and its original cause.
  text-read :: Bytes -> JsonReadOptions -> Bytes -> Application -> JsonDecoded Application
  text-read counter options source prior = let called = remember counter source in json-decoded-with-error (set-scalar True prior) returned

  -- | Supply independent application and root reporting identities for each declared scalar family.
  scalar-type :: String -> Bytes -> JsonType Application
  scalar-type flavor counter =
    let base = json-structural-type "Bool" JsonBooleanKind initial (\value -> not (scalar value)) (\value -> not (scalar value)) (\options value -> json-marshal-value json-bool-type options (scalar value)) ordinary-update in
    let methods = if flavor == "main.jsonSemantic" then json-methods None None (Some (json-read counter)) None else if flavor == "main.textSemantic" then json-methods None None None (Some (text-read counter)) else json-no-methods in
    let owner-name = if flavor == "main.jsonSemantic" then "jsonSemantic" else if flavor == "main.textSemantic" then "textSemantic" else "plainBool" in
    json-type-with-read-identity flavor owner-name (json-type-with-methods base methods)

  -- | Construct the containing record with its independently named root identity.
  fields :: String -> JsonType Application -> JsonType Application
  fields flavor value-type =
    let label = string-concat-all ["main.ordinary[", flavor, "]"] in
    let owner-name = string-concat-all ["ordinary[", flavor, "]"] in
    json-type-with-read-identity label owner-name (json-record-type label initial (\value -> False) [json-field "before" json-string-type earlier set-earlier, json-field "a/b~" value-type (\value -> value) (\next prior -> next), json-field "after" json-int64-type later set-later])

  -- | Construct a second containing record without changing the independent application state.
  descriptor :: String -> String -> Bytes -> JsonType Application
  descriptor flavor context counter =
    let value-type = scalar-type flavor counter in
    if context == "field" then fields flavor value-type
    else if context == "nested" then
      let label = string-concat-all ["main.nested[main.ordinary[", flavor, "]]"] in
      let owner-name = string-concat-all ["nested[main.ordinary[", flavor, "]]"] in
      json-type-with-read-identity label owner-name (json-record-type label initial (\value -> False) [json-field "outer" (fields flavor value-type) (\value -> value) (\next prior -> next)])
    else value-type

  -- | Compare complete bytes without scalar text normalization.
  same :: Bytes -> String -> Bool
  same actual expected = eq-i64 (bytes-compare actual (string-to-bytes expected)) 0

  -- | Compare retained callback payload bytes inside their checked observer window.
  payload-same :: Bytes -> Bytes -> Nat -> Bool
  payload-same counter expected at =
    if at >= bytes-len expected then True
    else if at + 1 < bytes-len counter then bytes-get counter (at + 1) == bytes-get expected at && payload-same counter expected (at + 1)
    else False

  -- | Observe callback count and exact JSON or decoded-text input independently for each public entry.
  calls-match :: Bytes -> Nat -> String -> Bool
  calls-match counter calls payload =
    if bytes-len counter == 16 then bytes-get counter 0 == calls && bytes-get counter 15 == bytes-len (string-to-bytes payload) && payload-same counter (string-to-bytes payload) 0 else False

  -- | Preserve original public type or semantic alternatives rather than accepting every non-success outcome.
  family :: JsonError -> String
  family error = case error of
    JsonTypeError label actual offset path -> "type"
    JsonTypeCause label actual offset path cause -> "type"
    JsonTypeInfoError info cause -> "type"
    JsonPathError path cause -> family cause
    JsonSemanticError info cause -> "semantic"
    other -> "other"

  -- | Observe the original application refusal through its structured cause chain.
  refused :: JsonError -> Bool
  refused error = case error of
    JsonEncodingError offset text -> text == "method-refusal" && offset == 0
    other -> case json-error-cause error of None -> False; Some cause -> refused cause

  -- | Retain literal optional type identity independently from an empty application label.
  type-matches :: JsonError -> Bool -> String -> Bool
  type-matches error present expected = case json-error-type error of None -> not present; Some label -> present && label == expected

  -- | Exact observed error metadata, including retained reference rendering-panic outcomes.
  data ErrorObservation where
    -- | Family, optional type, position, actual description, record/field, semantic pointer/kind/value, original refusal and reference panic flag.
    CheckError :: String -> Bool -> String -> Nat -> String -> String -> String -> String -> Nat -> String -> Bool -> Bool -> ErrorObservation

  -- | Compare all actual public error metadata while treating recorded reference panics as nil-type observations.
  error-status :: Option JsonError -> ErrorObservation -> Nat
  error-status error expected = case expected of
    CheckError expected-family present label offset actual owner-name field pointer kind reported sentinel render-panic -> case error of
      None -> if expected-family == "none" then 0 else 1
      Some cause ->
        if family cause == "other" || not (family cause == expected-family) then 1
        else if not (type-matches cause present label) then 2
        else if not (json-error-offset cause == offset) then 3
        else if not (refused cause == sentinel) then 4
        else if expected-family == "type" then
          if not ((case json-error-value cause of None -> ""; Some value -> value) == actual) then 5
          else if not (json-error-record cause == owner-name) then 6
          else if not (json-error-path cause == field) then 7
          else if render-panic && present then 8
          else 0
        else case json-error-semantic cause of
          None -> 8
          Some info -> if json-semantic-pointer info == pointer && u8-to-nat (json-semantic-kind-byte info) == kind && same (json-semantic-reported-bytes info) reported then 0 else 9

  -- | One original input, policy, partial application state, callback payload and exact error observation.
  data UpdateObservation where
    -- | Flavor, containing context, original input, reporting policy, scalar/later state, callback count/payload and error.
    ObserveUpdate :: String -> String -> String -> Bool -> Bool -> I64 -> Nat -> String -> ErrorObservation -> UpdateObservation

  -- | Observe state and public Result reporting with independent callback counters.
  observe :: UpdateObservation -> Nat
  observe row = case row of
    ObserveUpdate flavor context input legacy expected-value expected-after calls payload expected-error ->
      let options = json-read-options False False (json-policies [json-continue-after-type-errors legacy]) in
      let counter-a = bytes-new 16 in
      let decoded = json-update-with (descriptor flavor context counter-a) options (runtime-bytes (string-to-bytes input)) (initial ()) in
      let counter-b = bytes-new 16 in
      let result = json-unmarshal-with (descriptor flavor context counter-b) options (runtime-bytes (string-to-bytes input)) in
      let result-error = case result of Ok value -> None; Err cause -> Some cause in
      let status-a = error-status (json-decoded-error decoded) expected-error in
      let status-b = error-status result-error expected-error in
      if not (scalar (json-decoded-value decoded) == expected-value) then 1
      else if not (eq-i64 (later (json-decoded-value decoded)) expected-after) then 2
      else if not (calls-match counter-a calls payload && calls-match counter-b calls payload) then 3
      else if status-a > 0 then 10 + status-a
      else if status-b > 0 then 25 + status-b
      else case result of Ok value -> if scalar value == expected-value && eq-i64 (later value) expected-after then 0 else 4; Err cause -> 0

  -- | Preserve the first full row/property code within bounded observation groups.
  group :: List UpdateObservation -> Nat -> Nat
  group rows at = case rows of Nil -> 0; Cons first rest -> let code = observe first in if code > 0 then at * 40 + code else group rest (at + 1)
'''

def lit(x):return json.dumps(x,ensure_ascii=False)
def flag(x):return 'True' if x else 'False'
rows=json.loads(REFERENCE.read_text())['updates'];assert len(rows)==54
text=TEMPLATE;groups=[];panics=0
for start in range(0,len(rows),16):
 entries=[]
 for row in rows[start:start+16]:
  e=row['error'];kind=e['kind'];family='none' if kind=='none' else 'type' if kind=='*json.UnmarshalTypeError' else 'semantic'
  panic='text_panic' in e;panics+=panic
  if panic:assert e['type_nil'] and 'type' not in e
  calls=row['calls'];assert len(calls)<=1
  payload=bytes.fromhex(calls[0].split(':',1)[1]).decode() if calls else ''
  args=[lit(family),flag('type' in e),lit(e.get('type','')),str(e.get('offset',0)),lit(e.get('value','')),lit(e.get('struct','')),lit(e.get('field','')),lit(bytes.fromhex(e.get('pointer_hex','')).decode()),str(e.get('json_kind',0)),lit(bytes.fromhex(e.get('value_hex','')).decode()),flag(e.get('is_sentinel',False)),flag(panic)]
  entries.append('ObserveUpdate '+' '.join([lit(row['type']),lit(row['context']),lit(bytes.fromhex(row['input_hex']).decode()),flag(row['legacy_errors']),flag(row['value']),str(row['after']),str(len(calls)),lit(payload),'(CheckError '+' '.join(args)+')']))
 name='rows'+str(start//16);groups.append((name,start));text+=f'\n  -- | Retain original update observations {start+1} through {start+len(entries)}.\n  {name} :: List UpdateObservation\n  {name} = [\n    '+',\n    '.join(entries)+'\n  ]\n'
assert panics==12
text+='\n  -- | Compare every retained read observation with a complete diagnostic code.\n  status :: Nat\n  status =\n'
for i,(name,start) in enumerate(groups):text+=f'    let code{i} = group {name} {start} in\n    if code{i} > 0 then code{i} else\n'
text+='    0\n\n  -- | Print the complete first diagnostic code on failure while success remains silent.\n  main :: Unit ->{IO r} Int\n  main unit = with (io = newIO ()) in\n    let failed = status in\n    if failed == 0 then 0 else\n      let printed = ioWrite (string-to-bytes (string-concat-all ["json-update-error:", nat-to-string failed, "\\n"])) in 1\n'
out=ROOT/'compiler/tests/fixtures/json-codec-error-updates.cinch';out.write_text(text)
manifest={'status':'prepared-not-qualified','fixture_sha256':hashlib.sha256(out.read_bytes()).hexdigest(),'oracle_sha256':hashlib.sha256(REFERENCE.read_bytes()).hexdigest(),'rows':54,'public_entries':['json-update-with','json-unmarshal-with'],'comparison':'Actual partial scalar/later state; JSON/text callback count and exact input bytes; original public type/semantic families; optional type, offset, root record/field, semantic pointer/kind/reported bytes and original refusal. Every original reference rendering panic explicitly retains nil type; no deliberately crashing renderer or expected normalization.','reference_render_panics_retained':panics,'all_original_oracle_rows_retained':1494,'source_changed':True,'status_encoding':'zero success, otherwise zero-based row*40+property, printed completely before exit1'}
(ROOT/'bootstrap/evidence/stdlib/stdlib-json-update-error-cases-v1.json').write_text(json.dumps(manifest,indent=2)+'\n')
print(json.dumps(manifest,indent=2))
