#!/usr/bin/env python3
"""Generate SDK-free consumers of the immutable 902-row syntax-partial observation tape."""
from pathlib import Path
import hashlib,json
ROOT=Path(__file__).resolve().parents[1]
ORACLE=ROOT/'compiler/tests/fixtures/json-codec-syntax-partial-vectors.json'
assert hashlib.sha256(ORACLE.read_bytes()).hexdigest()=='7257d12bb9947b25b8fc3afcc29d4a365ec94c585f12b19a9bcf657eb34c2d8d'
ROWS=json.loads(ORACLE.read_text());assert len(ROWS)==902
q=lambda x:json.dumps(x,ensure_ascii=False)
HEADER=r'''-- | Independent partial-update observations retain complete application state, callbacks and structured refusals.
module Main where
  import Text.Json.Value
  import Text.Json.Codec
  import Text.Json.Dynamic
  import Data.Array
  import Data.Machine

  -- | Keep the selected source length opaque to front-end evaluation.
  prim runtime-width :: U32 -> U32 where
    llvm {
      %value = or i32 %0, 0
      ret i32 %value
    }

  -- | Snapshot source bytes through a runtime-sized owner.
  input-bytes :: String -> Bytes
  input-bytes text =
    let input = string-to-bytes text in
    let count = u32-to-nat (runtime-width (wrapping-i64-to-u32 (nat-to-i64 (bytes-len input)))) in
    let output = bytes-new count in
    if bytes-len input <= bytes-len output then let copied = bytes-copy-range output input 0 0 (bytes-len input) in output else bytes-new 0

  -- | Compare every byte without truncating at embedded zeroes.
  same :: Bytes -> String -> Bool
  same bytes text = eq-i64 (bytes-compare bytes (string-to-bytes text)) 0

  -- | Retain the exact independent structured error family and its owned metadata.
  data ExpectedError where
    -- | A completed operation without a diagnostic.
    NoError :: ExpectedError
    -- | Established syntax reporting retains its original cursor.
    LegacySyntax :: Nat -> ExpectedError
    -- | Precise syntax reporting retains both byte checkpoint and escaped pointer.
    NativeSyntax :: Nat -> String -> ExpectedError
    -- | A semantic conversion keeps kind, optional application identity and reported bytes independently.
    Semantic :: Nat -> String -> U8 -> Option String -> String -> ExpectedError

  -- | Compare optional application identity without conflating absence with empty text.
  identity-is :: Option String -> Option String -> Bool
  identity-is actual expected = case expected of
    None -> case actual of None -> True; Some name -> False
    Some name -> case actual of None -> False; Some value -> value == name

  -- | Recognize the independent no-error outcome without a nested ambiguous case expression.
  no-error :: ExpectedError -> Bool
  no-error expected = case expected of NoError -> True; other -> False

  -- | Observe the original public error family, common getters and dedicated metadata together.
  error-is :: JsonError -> ExpectedError -> Bool
  error-is actual expected = case expected of
    NoError -> False
    LegacySyntax offset -> case actual of JsonSyntaxError at message -> at == offset && json-error-offset actual == offset; other -> False
    NativeSyntax offset pointer -> case actual of
      JsonSyntacticError info cause -> json-syntactic-offset info == offset && json-syntactic-pointer info == pointer && json-error-offset actual == offset && json-error-path actual == pointer
      other -> False
    Semantic offset pointer kind label reported -> case actual of
      JsonSemanticError info cause -> json-semantic-offset info == offset && json-semantic-pointer info == pointer && eq-u8 (json-semantic-kind-byte info) kind && identity-is (json-semantic-type-option info) label && same (json-semantic-reported-bytes info) reported && json-error-offset actual == offset
      other -> False

  -- | Inspect both public decoded-result entry points without replacing the original diagnostic.
  outcome-is :: forall a. JsonDecoded a -> ExpectedError -> Bool
  outcome-is decoded expected =
    (case json-decoded-error decoded of None -> no-error expected; Some error -> error-is error expected)
      && (case json-decoded-result decoded of Ok value -> no-error expected; Err error -> error-is error expected)

  -- | One unchanged reference row contains its index, policy, source, full state, error and callback observations.
  data Observation where
    -- | Private global row index, legacy policy, exact input, complete state, error, call count and callback bytes.
    Observe :: Nat -> Bool -> String -> String -> ExpectedError -> Nat -> String -> Observation

  -- | Change only the requested reporting policy while retaining all other established defaults.
  options-for :: Bool -> JsonReadOptions
  options-for legacy = json-read-options False False (json-configure-policies json-default-policies [json-continue-after-type-errors legacy])

  -- | Give integer conversions their independently observed application identity.
  integer-type :: JsonType I64
  integer-type = json-type-with-read-identity "int" "" json-int64-type

  -- | Compare full serialized application state after conversion.
  state-is :: forall a. JsonType a -> a -> String -> Bool
  state-is descriptor value expected = case json-marshal descriptor value of Err error -> False; Ok bytes -> same bytes expected
'''
RECORD=r'''
  -- | Explicit prior/default record state keeps unrelated fields independently observable.
  data Record where
    -- | Before, selected and after application field values.
    RecordValue :: I64 -> FIELD -> I64 -> Record

  -- | Create independent zero/default fields.
  initial-record :: Unit -> Record
  initial-record unit = RecordValue 0 DEFAULT 0

  -- | Record zero classification is irrelevant to source conversion.
  record-zero :: Record -> Bool
  record-zero value = False

  -- | Inspect the before field.
  before :: Record -> I64
  before value = case value of RecordValue first selected last -> first

  -- | Replace only the before field.
  set-before :: I64 -> Record -> Record
  set-before next value = case value of RecordValue first selected last -> RecordValue next selected last

  -- | Inspect the selected field.
  selected :: Record -> FIELD
  selected value = case value of RecordValue first selected last -> selected

  -- | Replace only the selected field.
  set-selected :: FIELD -> Record -> Record
  set-selected next value = case value of RecordValue first selected last -> RecordValue first next last

  -- | Inspect the after field.
  after :: Record -> I64
  after value = case value of RecordValue first selected last -> last

  -- | Replace only the after field.
  set-after :: I64 -> Record -> Record
  set-after next value = case value of RecordValue first selected last -> RecordValue first selected next

  -- | Describe all observed fields without inferring application identities from source text.
  selected-type :: JsonType Record
  selected-type = json-record-type LABEL initial-record record-zero [json-field "before" integer-type before set-before,json-field "value" FIELDTYPE selected set-selected,json-field "after" integer-type after set-after]

  -- | Preserve the exact independently observed prior application record.
  prior-value :: Record
  prior-value = RecordValue 7 PRIOR 11
'''
HOOK=r'''
  -- | Callback observations travel in an immutable advanced application value.
  data Hook where
    -- | Payload, exact invocation count and last complete callback argument.
    HookValue :: PAYLOAD -> Nat -> Bytes -> Hook

  -- | Construct the independent prior state without an earlier callback.
  prior-value :: Hook
  prior-value = HookValue PRIOR 0 (bytes-new 0)

  -- | Encode only application payload, retaining callback observations separately.
  hook-encode :: JsonWriteOptions -> Hook -> Result JsonError Bytes
  hook-encode options value = case value of HookValue payload count argument -> json-marshal-value BASE options payload

  -- | Return the exact callback count from the advanced application value.
  hook-count :: Hook -> Nat
  hook-count value = case value of HookValue payload count argument -> count

  -- | Return independently owned callback bytes from the advanced application value.
  hook-argument :: Hook -> Bytes
  hook-argument value = case value of HookValue payload count argument -> argument
'''
JSONHOOK=r'''
  -- | Record a complete JSON callback and its exact source without accepting partial values.
  hook-update :: JsonReadOptions -> JsonValue -> Hook -> JsonDecoded Hook
  hook-update options source prior = case prior of HookValue payload count argument -> json-decoded-ok (HookValue 52 (count + 1) (json-value-source source))

  -- | Install the observed complete-fragment JSON conversion capability.
  selected-type :: JsonType Hook
  selected-type = json-custom-type "main.jsonHook" (\unit -> HookValue 0 0 (bytes-new 0)) (\value -> False) (\value -> False) hook-encode hook-update
'''
TEXHOOK=r'''
  -- | Preserve the ordinary payload conversion when no text callback is eligible.
  ordinary-update :: JsonReadOptions -> JsonValue -> Hook -> JsonDecoded Hook
  ordinary-update options source prior = case prior of
    HookValue payload count argument ->
      let decoded = json-update-value json-string-type options source payload in
      case json-decoded-error decoded of None -> json-decoded-ok (HookValue (json-decoded-value decoded) count argument); Some error -> if json-decoded-fatal decoded then json-decoded-failure prior error else json-decoded-with-error prior error

  -- | Record a complete decoded text argument and its exact invocation count.
  text-update :: JsonReadOptions -> Bytes -> Hook -> JsonDecoded Hook
  text-update options input prior = case prior of
    HookValue payload count argument -> json-decoded-ok (HookValue (string-concat "method:" (case string-from-bytes input of None -> ""; Some value -> value)) (count + 1) input)

  -- | Install text conversion without changing ordinary null behavior or callback selection.
  selected-type :: JsonType Hook
  selected-type = json-type-with-methods (json-structural-type "main.textHook" JsonStringKind (\unit -> HookValue "" 0 (bytes-new 0)) (\value -> False) (\value -> False) hook-encode ordinary-update) (json-methods None None None (Some text-update))
'''
def err_expr(e):
 f=e['family']
 if f=='none':return 'NoError'
 if f=='*json.SyntaxError':return f"(LegacySyntax {e['offset']})"
 if f=='*jsontext.SyntacticError':return f"(NativeSyntax {e['offset']} {q(bytes.fromhex(e['pointer_hex']).decode())})"
 assert f=='*json.SemanticError',e
 label='None' if e.get('type_nil') else f"(Some {q(e['type'])})"
 return f"(Semantic {e['offset']} {q(bytes.fromhex(e['pointer_hex']).decode())} {e['kind']} {label} {q(bytes.fromhex(e['reported_hex']).decode())})"
manifest={'oracle_path':str(ORACLE.relative_to(ROOT)),'oracle_sha256':hashlib.sha256(ORACLE.read_bytes()).hexdigest(),'rows':902,'scope':'Generated SDK-free literal observation consumers only; no reference regeneration or qualification claim','consumers':[]}
models=list(dict.fromkeys(r['model'] for r in ROWS))
for model in models:
 s=HEADER
 if model in ['record','pointer-record']:
  field='I64' if model=='record' else '(Option I64)'
  b=RECORD.replace('FIELDTYPE','integer-type' if model=='record' else '(json-nullable-type integer-type)').replace('FIELD',field).replace('DEFAULT','0' if model=='record' else 'None').replace('PRIOR','9' if model=='record' else 'None').replace('LABEL',q('main.record' if model=='record' else 'main.pointerRecord'))
  s+=b;ty='Record'
 elif model in ['json-hook','text-hook']:
  s+=HOOK.replace('PAYLOAD','I64' if model=='json-hook' else 'String').replace('PRIOR','7' if model=='json-hook' else '"prior"').replace('BASE','integer-type' if model=='json-hook' else 'json-string-type')
  s+=JSONHOOK if model=='json-hook' else TEXHOOK;ty='Hook'
 else:
  ty,descriptor,prior={
    'int':('I64','integer-type','7'),
    'slice':('Option (List I64)','json-nullable-type (json-type-with-read-identity "[]int" "" (json-list-type integer-type))','Some [7,8,9]'),
    'array':('Array I64','json-type-with-read-identity "[3]int" "" (json-fixed-array-type integer-type 3)','array-from-list [7,8,9]'),
    'map':('JsonMap String I64','json-type-with-read-identity "map[string]int" "" (json-map-type json-string-key integer-type)','json-map-set "a" 9 (json-map-set "keep" 7 (json-map-empty json-string-key))'),
    'any':('JsonAny','json-any-type','json-any-object [(string-to-bytes "old",json-any-float64 (i64-to-f64 7))]'),
    'raw':('JsonRawMessage','json-raw-type','json-raw-from-bytes (string-to-bytes "7")'),
  }[model]
  s+=f'\n  -- | Use the exact observed application descriptor.\n  selected-type :: JsonType ({ty})\n  selected-type = {descriptor}\n\n  -- | Restore the independent prior value for every observation.\n  prior-value :: {ty}\n  prior-value = {prior}\n'
 s+=f'\n  -- | Compare every state byte while preserving separate callback observations.\n  model-state :: {ty} -> String -> Bool\n'
 if model=='raw':s+='  model-state value expected = case json-raw-bytes value of None -> False; Some bytes -> same bytes expected\n'
 else:s+='  model-state value expected = state-is selected-type value expected\n'
 s+=f'\n  -- | Preserve the first original global row and mismatching property.\n  check-row :: Observation -> Nat\n  check-row row = case row of\n    Observe index legacy wire expected error calls argument ->\n      let decoded = json-update-with selected-type (options-for legacy) (input-bytes wire) prior-value in\n      let value = json-decoded-value decoded in\n      if not (model-state value expected) then index * 16 + 1\n      else if not (outcome-is decoded error) then index * 16 + 2\n'
 if model in ['json-hook','text-hook']:
  s+='      else if not (hook-count value == calls) then index * 16 + 3\n      else if not (same (hook-argument value) argument) then index * 16 + 4\n'
 else:s+='      else if not (calls == 0) then index * 16 + 3\n'
 s+='      else 0\n\n  -- | Traverse a bounded literal group without constructing a deep generated let chain.\n  check-group :: List Observation -> Nat\n  check-group rows = case rows of Nil -> 0; Cons first rest -> let result = check-row first in if result > 0 then result else check-group rest\n'
 selected=[(i,row) for i,row in enumerate(ROWS) if row['model']==model]
 for n in range(0,len(selected),16):
  group=n//16;es=[]
  for i,row in selected[n:n+16]:
   wire=bytes.fromhex(row['input_hex']).decode();state=bytes.fromhex(row['state']).decode() if model=='raw' else json.dumps(row['state'],ensure_ascii=False,separators=(',',':'))
   calls=row['calls'];assert len(calls)<=1
   argument=''
   if calls:
    prefix,raw=calls[0].split(':',1);assert prefix==('json' if model=='json-hook' else 'text');argument=bytes.fromhex(raw).decode()
   es.append(f'Observe {i} {str(row["legacy_errors"])} {q(wire)} {q(state)} {err_expr(row["error"])} {len(calls)} {q(argument)}')
  s+=f'\n  -- | Preserve sixteen or fewer unchanged independent rows in original order.\n  rows-a{group:03d} :: List Observation\n  rows-a{group:03d} = ['+',\n    '.join(es)+']\n'
 s+='\n  -- | Stop at the earliest complete original row/property disagreement.\n  status :: Nat\n  status =\n'
 for group in range((len(selected)+15)//16):s+=f'    let result-a{group:03d} = check-group rows-a{group:03d} in\n    if result-a{group:03d} > 0 then result-a{group:03d} else\n'
 s+='    0\n\n  -- | Emit a normalized failing exit while keeping the complete original row/property code.\n  main :: Unit ->{IO r} Int\n  main unit = with (io = newIO ()) in\n    let failed = status in\n    if failed == 0 then 0 else let printed = ioWrite (string-to-bytes (string-concat-all ["json-syntax-partial:", nat-to-string failed, "\\n"])) in 1\n'
 p=ROOT/f'compiler/tests/fixtures/json-syntax-partial-{model}-observations-v2.cinch';p.write_text(s)
 manifest['consumers'].append({'model':model,'path':str(p.relative_to(ROOT)),'sha256':hashlib.sha256(p.read_bytes()).hexdigest(),'rows':len(selected),'indices':[i for i,_ in selected]})
p=ROOT/'bootstrap/evidence/stdlib/stdlib-json-syntax-partial-consumer-manifest-v2.json';p.write_text(json.dumps(manifest,indent=2)+'\n')
print('Generated',sum(x['rows'] for x in manifest['consumers']),'unchanged rows across',len(models),'consumers')
