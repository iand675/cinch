#!/usr/bin/env python3
"""Prepare all288 existing method-error observations with three public encoding entry points."""
from pathlib import Path
import hashlib,json
ROOT=Path('/workspace/dasl-stdlib-json')
REFERENCE=ROOT/'seed/tests/fixtures/json-codec-error-context-vectors.json'
TEMPLATE=r'''-- | Independent method refusal metadata and partial output exercise every public encoding boundary.
module Main where
  import Text.Json.Value
  import Text.Json.Codec
  import Data.Machine

  -- | Keep application values opaque to frontend evaluation.
  prim runtime-bool :: Bool -> Bool where
    llvm {
      %result = or i8 %0, 0
      ret i8 %result
    }

  -- | Compare every output byte with the independent observed prefix.
  same :: Bytes -> String -> Bool
  same actual expected = eq-i64 (bytes-compare actual (string-to-bytes expected)) 0

  -- | Count application callbacks through a private checked test buffer.
  called :: Bytes -> Unit
  called counter =
    if bytes-len counter > 0 then
      let count = bytes-get counter 0 in
      if count < 255 then bytes-set counter 0 (count + 1) else ()
    else ()

  -- | Observe callback count without narrowing an already bounded natural byte.
  count-matches :: Bytes -> Nat -> Bool
  count-matches counter expected = if bytes-len counter > 0 then bytes-get counter 0 == expected else False

  -- | Preserve a distinctive original structured application refusal.
  refusal :: JsonError
  refusal = JsonEncodingError 0 "method-refusal"

  -- | Construct a user-owned semantic carrier with explicitly unspecified application type metadata.
  returned :: Bool -> JsonError
  returned semantic = if semantic then JsonSemanticError (json-semantic-info-with-value "" 2 "/inner~0" (string-to-bytes "12") "" 48 (string-to-bytes "12")) refusal else refusal

  -- | Invoke exactly one pure application method and retain its original structured cause.
  refuse :: Bytes -> Bool -> JsonWriteOptions -> Bool -> Result JsonError Bytes
  refuse counter semantic options value = let observed = called counter in Err (returned semantic)

  -- | Attach explicit application metadata without inventing an implicit runtime type inspection.
  base :: String -> JsonType Bool
  base label = json-structural-type label JsonBooleanKind (\unit -> False) not (json-type-is-empty json-bool-type) (json-marshal-value json-bool-type) (json-update-value json-bool-type)

  -- | Choose the independent JSON or text capability declared by the original method family.
  descriptor :: String -> Bytes -> JsonType Bool
  descriptor flavor counter =
    let semantic = flavor == "main.jsonSemantic" || flavor == "main.textSemantic" in
    let callback = refuse counter semantic in
    if flavor == "main.jsonRefusal" || flavor == "main.jsonSemantic" then json-type-with-methods (base flavor) (json-methods (Some (False, callback)) None None None)
    else json-type-with-methods (base flavor) (json-methods None (Some (False, callback)) None None)

  -- | Preserve completed surrounding fields and an escaped member component around a method call.
  fields :: JsonType Bool -> JsonType Bool
  fields value-type = json-record-type "MethodRecord" (\unit -> False) not [json-field "before" json-string-type (\value -> "α<&") (\next prior -> prior), json-field "a/b~" value-type (\value -> value) (\next prior -> next), json-field "after" json-int64-type (\value -> 9) (\next prior -> prior)]

  -- | Seal one containing application object without changing the child value.
  nested :: JsonType Bool -> JsonType Bool
  nested value-type = json-record-type "OuterRecord" (\unit -> False) not [json-field "outer" (fields value-type) (\value -> value) (\next prior -> next)]

  -- | Encode one context through the explicit partial-output capability.
  attempt :: JsonType Bool -> Bool -> String -> JsonWriteOptions -> JsonEncoded
  attempt value-type value context options =
    if context == "pointer" then json-marshal-attempt (json-nullable-type value-type) options (Some value)
    else if context == "prefixed" then json-marshal-attempt (fields value-type) options value
    else if context == "nested" then json-marshal-attempt (nested value-type) options value
    else if context == "slice" then json-marshal-attempt (json-list-type value-type) options [value]
    else if context == "map" then json-marshal-attempt (json-map-type json-string-key value-type) options (json-map-set "a/b~" value (json-map-empty json-string-key))
    else json-marshal-attempt value-type options value

  -- | Select the complete or compact public Result boundary with explicit argument grouping.
  public-result :: forall a. JsonType a -> a -> JsonWriteOptions -> Bool -> Result JsonError Bytes
  public-result value-type value options compact = if compact then json-marshal-value value-type options value else json-marshal-with value-type options value

  -- | Observe the actual complete or compact Result family without substituting the partial outcome.
  encode-result :: JsonType Bool -> Bool -> String -> JsonWriteOptions -> Bool -> Result JsonError Bytes
  encode-result value-type value context options compact =
    if context == "pointer" then public-result (json-nullable-type value-type) (Some value) options compact
    else if context == "prefixed" then public-result (fields value-type) value options compact
    else if context == "nested" then public-result (nested value-type) value options compact
    else if context == "slice" then public-result (json-list-type value-type) [value] options compact
    else if context == "map" then public-result (json-map-type json-string-key value-type) (json-map-set "a/b~" value (json-map-empty json-string-key)) options compact
    else public-result value-type value options compact

  -- | Recover the original refusal through the public cause tree without flattening it to prose.
  original :: JsonError -> Bool
  original error = case error of
    JsonEncodingError offset message -> offset == 0 && message == "method-refusal"
    other -> case json-error-cause error of None -> False; Some cause -> original cause

  -- | Observe a Result's actual error alternative.
  result-error :: Result JsonError Bytes -> Option JsonError
  result-error encoded = case encoded of Ok output -> None; Err error -> Some error

  -- | Compare the public optional application type, preserving absence rather than normalizing an empty label.
  type-matches :: JsonError -> Bool -> String -> Bool
  type-matches error present expected = case json-error-type error of None -> not present; Some label -> present && label == expected

  -- | Classify an original public error family rather than accepting an internal output wrapper.
  family-matches :: JsonError -> Bool -> Bool
  family-matches error legacy =
    if legacy then
      case error of JsonCustomError label cause -> True; other -> False
    else
      case error of JsonSemanticError info cause -> True; other -> False

  -- | Compare all exposed semantic metadata without relying on randomized diagnostic wording.
  semantic-matches :: JsonError -> Nat -> String -> Nat -> String -> Bool
  semantic-matches error offset pointer kind reported = case json-error-semantic error of
    None -> False
    Some info -> json-semantic-offset info == offset && json-semantic-pointer info == pointer
      && u8-to-nat (json-semantic-kind-byte info) == kind && same (json-semantic-reported-bytes info) reported

  -- | Return a named property code for the first disagreement while retaining the original row index.
  error-status :: Option JsonError -> Bool -> Bool -> String -> Nat -> String -> Nat -> String -> Nat
  error-status error legacy has-type label offset pointer kind reported = case error of
    None -> 1
    Some cause ->
      if not (family-matches cause legacy) then 1
      else if not (type-matches cause has-type label) then 2
      else if not (original cause) then 3
      else if not legacy && not (semantic-matches cause offset pointer kind reported) then 4
      else 0

  -- | Retain every original method observation, including literal type metadata and compact offset comparison.
  data Observation where
    -- | Flavor, context, reporting, layout, HTML, actual prefix, optional type, semantic context and callback count.
    Observe :: String -> String -> Bool -> String -> Bool -> String -> Bool -> String -> Nat -> Nat -> String -> Nat -> String -> Nat -> Observation

  -- | Invoke each public entry point with its own callback observer and compare all available metadata.
  observe :: Observation -> Nat
  observe row = case row of
    Observe flavor context legacy style html expected has-type label offset compact-offset pointer kind reported calls ->
      let options = json-write-options html (not (style == "compact")) (if style == "pretty" then "\t " else "") (if style == "pretty" then "  " else "") (json-policies [json-continue-after-type-errors legacy]) in
      let value = runtime-bool False in
      let counter-a = bytes-new 1 in let output = attempt (descriptor flavor counter-a) value context options in
      let counter-b = bytes-new 1 in let complete = encode-result (descriptor flavor counter-b) value context options False in
      let counter-c = bytes-new 1 in let compact = encode-result (descriptor flavor counter-c) value context options True in
      let status-a = error-status (json-encoded-error output) legacy has-type label offset pointer kind reported in
      let status-b = error-status (result-error complete) legacy has-type label offset pointer kind reported in
      let status-c = error-status (result-error compact) legacy has-type label compact-offset pointer kind reported in
      if not (same (json-encoded-bytes output) expected) then 1
      else if not (count-matches counter-a calls && count-matches counter-b calls && count-matches counter-c calls) then 2
      else if status-a > 0 then 10 + status-a
      else if status-b > 0 then 20 + status-b
      else if status-c > 0 then 30 + status-c
      else 0

  -- | Check bounded literal groups without raising the language's existing nesting limit.
  group :: List Observation -> Nat -> Nat
  group rows at = case rows of
    Nil -> 0
    Cons first rest -> let status = observe first in if status > 0 then at * 40 + status else group rest (at + 1)
'''

def lit(x):return json.dumps(x,ensure_ascii=False)
def flag(x):return 'True' if x else 'False'
root=json.loads(REFERENCE.read_text());rows=root['method_errors'];assert len(rows)==288
compact={(r['type'],r['context'],r['legacy_errors'],r['html']):r for r in rows if r['style']=='compact'}
text=TEMPLATE;groups=[]
for start in range(0,len(rows),16):
 name='rows'+str(start//16);entries=[]
 for row in rows[start:start+16]:
  e=row['error'];assert e['kind']==('*json.MarshalerError' if row['legacy_errors'] else '*json.SemanticError')
  assert e['is_sentinel'] and len(row['calls'])==1
  ce=compact[(row['type'],row['context'],row['legacy_errors'],row['html'])]['error']
  entries.append('Observe '+' '.join([lit(row['type']),lit(row['context']),flag(row['legacy_errors']),lit(row['style']),flag(row['html']),lit(bytes.fromhex(row['output_hex']).decode()),flag('type' in e),lit(e.get('type','')),str(e.get('offset',0)),str(ce.get('offset',0)),lit(bytes.fromhex(e.get('pointer_hex','')).decode()),str(e.get('json_kind',0)),lit(bytes.fromhex(e.get('value_hex','')).decode()),str(len(row['calls']))]))
 text+='\n  -- | Preserve original method observations '+str(start+1)+' through '+str(start+len(entries))+'.\n  '+name+' :: List Observation\n  '+name+' = [\n    '+',\n    '.join(entries)+'\n  ]\n';groups.append((name,start))
text+='\n  -- | Preserve a nonzero row/property result for any independent mismatch.\n  status :: Nat\n  status =\n'
for index,(name,start) in enumerate(groups):text+=f'    let code{index} = group {name} {start} in\n    if code{index} > 0 then code{index} else\n'
text+='    0\n\n  -- | Return a direct failure status while successful runs remain silent.\n  main :: Int\n  main = nat-to-int status\n'
out=ROOT/'seed/tests/fixtures/json-codec-error-methods.cinch';out.write_text(text)
manifest={'status':'prepared-not-qualified','source_sha256':hashlib.sha256(out.read_bytes()).hexdigest(),'oracle_sha256':hashlib.sha256(REFERENCE.read_bytes()).hexdigest(),'rows':288,'public_entries':['json-marshal-attempt','json-marshal-with','json-marshal-value'],'comparison':'Literal original optional type metadata, public family, semantic offset/pointer/kind/reported bytes, original refusal, partial prefix and per-entry callback counts; no pointer-label/absence normalization or prose equivalence','status_encoding':'zero=success; zero-based row*40 + property, where1=prefix,2=callbacks,11/21/31=family,12/22/32=optional type,13/23/33=original cause,14/24/34=semantic metadata. Process exit may truncate later nonzero statuses; preserve exact program for diagnosis.','original_total_rows_retained':1494,'sdk_panic_rows_retained':12,'updates54_status':'pending independent generator; no row dropped','production_sources_changed':False}
(ROOT/'compiler/evidence/stdlib/stdlib-json-method-error-cases-v1.json').write_text(json.dumps(manifest,indent=2)+'\n')
print(json.dumps(manifest,indent=2))
