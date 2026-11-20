#!/usr/bin/env python3
"""Generate four bounded, independently observed quoted-field consumers without discarding error rows."""
from pathlib import Path
import hashlib
import json
ROOT = Path(__file__).resolve().parents[1]
REFERENCE = ROOT/'compiler/tests/fixtures/json-codec-error-context-vectors.json'
TEMPLATE = r'''-- | Independent quoted-field output, error metadata and callback observations include every policy and formatting combination.
module Main where
  import Text.Json.Codec
  import Text.Json.Value
  import Data.Machine

  -- | Keep application booleans opaque to frontend evaluation.
  prim runtime-bool :: Bool -> Bool where
    llvm {
      %result = or i8 %0, 0
      ret i8 %result
    }

  -- | Keep application integer values opaque to frontend evaluation.
  prim runtime-int :: I64 -> I64 where
    llvm {
      %result = or i64 %0, 0
      ret i64 %result
    }

  -- | Keep source byte lengths opaque to frontend evaluation.
  prim runtime-size :: U32 -> U32 where
    llvm {
      %result = or i32 %0, 0
      ret i32 %result
    }

  -- | Produce independently owned runtime-sized source bytes.
  input :: String -> Bytes
  input text = let original = string-to-bytes text in
    let size = u32-to-nat (runtime-size (wrapping-i64-to-u32 (nat-to-i64 (bytes-len original)))) in
    let output = bytes-new size in
    if bytes-len original <= bytes-len output then let copied = bytes-copy-range output original 0 0 (bytes-len original) in output else bytes-new 0

  -- | Observe an entire encoded prefix rather than a rendered diagnostic string.
  same :: Bytes -> String -> Bool
  same actual expected = eq-i64 (bytes-compare actual (string-to-bytes expected)) 0

  -- | Increment a private fixture counter without introducing library continuation storage.
  called :: Bytes -> Unit
  called counter =
    if bytes-len counter > 0 then
      let count = bytes-get counter 0 in
      if count < 255 then bytes-set counter 0 (count + 1) else ()
    else ()

  -- | Attach the application's explicit type label without changing ordinary conversion behavior.
  named :: forall a. String -> JsonType a -> JsonType a
  named label descriptor = json-structural-type label (json-type-kind descriptor) (\unit -> json-type-default descriptor) (json-type-is-zero descriptor) (json-type-is-empty descriptor) (json-marshal-value descriptor) (json-update-value descriptor)

  -- | Seal a quoted value and optional surrounding fields in an immutable record descriptor.
  field-record :: forall a. Bool -> JsonType a -> JsonType a
  field-record before descriptor =
    let member = json-field-with "value" (json-field-options False False True 0 False JsonFieldDefaultMatch) descriptor (\value -> value) (\next prior -> next) in
    let fields = if before then [json-field "before" json-string-type (\value -> "α<&") (\next prior -> prior), member, json-field "after" json-int64-type (\value -> 9) (\next prior -> prior)] else [member] in
    json-record-type "ExplicitRecord" (\unit -> json-type-default descriptor) (json-type-is-zero descriptor) fields

  -- | Seal one containing object without changing the child's application value.
  nested :: forall a. JsonType a -> JsonType a
  nested descriptor = json-record-type "OuterRecord" (\unit -> json-type-default descriptor) (json-type-is-zero descriptor) [json-field "outer" descriptor (\value -> value) (\next prior -> next)]

  -- | A complete independent row stores context, three policies, format, escaping, output, error metadata and callback count.
  data Observation where
    -- | Private original row fields; error text randomization is not treated as error identity.
    Observe :: String -> Bool -> Bool -> Bool -> String -> Bool -> String -> Bool -> Nat -> String -> Nat -> Observation

  -- | Render one observed context through real public descriptors and the partial-output outcome.
  encode :: forall a. JsonType a -> a -> String -> JsonWriteOptions -> JsonEncoded
  encode descriptor value context options =
    let field = field-record (context == "prefixed" || context == "prefixed_pointer") descriptor in
    if context == "quoted_pointer" || context == "prefixed_pointer" then json-marshal-attempt (json-nullable-type field) options (Some value)
    else if context == "nested" then json-marshal-attempt (nested field) options value
    else if context == "slice" then json-marshal-attempt (json-list-type field) options [value]
    else json-marshal-attempt field options value

  -- | Validate structured error identity, exact byte offsets and pointer spelling independently of diagnostic prose.
  error-matches :: Option JsonError -> Bool -> Nat -> String -> String -> Bool
  error-matches result expected offset pointer label = case result of
    None -> not expected
    Some error -> expected && (case error of
      JsonSemanticError info cause -> json-semantic-action info == "marshal" && json-semantic-offset info == offset
        && json-semantic-pointer info == pointer && json-semantic-type info == label && eq-u8 (json-semantic-kind-byte info) 0
        && bytes-len (json-semantic-bytes info) == 0 && bytes-len (json-semantic-reported-bytes info) == 0
        && json-error-message cause == "invalid use of `string` tag option"
      other -> False)

  -- | Compare the encoding and callback count for one complete independent observation.
  check :: JsonType APP -> APP -> String -> Bytes -> Observation -> Bool
  check descriptor value label counter row = case row of
    Observe context methods reporting stringify style html expected refused offset pointer count ->
      let options = json-write-options html (not (style == "compact")) (if style == "pretty" then "\t " else "") (if style == "pretty" then "  " else "") (json-policies [json-addressable-methods-only methods, json-continue-after-type-errors reporting, json-quote-scalar-fields stringify]) in
      let output = encode descriptor value context options in
      same (json-encoded-bytes output) expected && error-matches (json-encoded-error output) refused offset pointer label
        && (if bytes-len counter > 0 then bytes-get counter 0 == count else False)

  -- | Construct the concrete application descriptor with an independent method-call observer.
  descriptor :: Bytes -> JsonType APP
  descriptor counter = DESCRIPTOR

  -- | Check one row with fresh mutable callback-count storage and an opaque application value.
  observe :: Observation -> Bool
  observe row = let counter = bytes-new 1 in check (descriptor counter) (VALUE) LABEL counter row

  -- | Traverse one bounded row group with an observable nonzero original index on failure.
  group :: List Observation -> Nat -> Nat
  group rows at = case rows of Nil -> 0; Cons first rest -> if observe first then group rest (at + 1) else at + 1
'''

def literal(value):
    # JSON string spellings agree for this fixture alphabet except unicode: keep it literal.
    return json.dumps(value,ensure_ascii=False)
def boolean(value):return 'True' if value else 'False'

def main():
    reference=json.loads(REFERENCE.read_text())
    assert len(reference['tagged'])==1152
    types=[('bool','main.plainBool','Bool','named "main.plainBool" json-bool-type','runtime-bool True'),
           ('pointer','main.pointerMethod','Bool','json-type-with-methods (named "main.pointerMethod" json-bool-type) (json-methods (Some (True, (\\options value -> let count = called counter in Ok (string-to-bytes "\\\"method\\\"")))) None None None)','runtime-bool True'),
           ('string','string','String','named "string" json-string-type','(case string-from-bytes (input "text") of Some text -> text; None -> "")'),
           ('integer','int64','I64','named "int64" json-int64-type','runtime-int 7')]
    manifest={'reference_sha256':hashlib.sha256(REFERENCE.read_bytes()).hexdigest(),'observed_quoted_rows':1152,'consumers':{}}
    for suite,label,app,descriptor,value in types:
        rows=[row for row in reference['tagged'] if row['type']==label]
        assert len(rows)==288
        text=TEMPLATE.replace('APP',app).replace('DESCRIPTOR',descriptor).replace('VALUE',value).replace('LABEL',literal(label))
        grouped=[]
        for i in range(0,len(rows),16):
            name=f'rows{ i//16 }'
            entries=[]
            for row in rows[i:i+16]:
                error=row['error'];refused=error['kind']!='none'
                assert not refused or error['cause']=='invalid use of `string` tag option'
                assert set(row['calls']).issubset({'pointer-json'})
                entries.append('Observe '+ ' '.join([literal(row['context']),boolean(row['legacy_methods']),boolean(row['legacy_errors']),boolean(row['legacy_stringify']),literal(row['style']),boolean(row['html']),literal(bytes.fromhex(row['output_hex']).decode()),boolean(refused),str(error.get('offset',0)),literal(bytes.fromhex(error.get('pointer_hex','')).decode()),str(len(row['calls']))]))
            text+=f'\n  -- | Preserve independent observations {i+1} through {i+len(entries)} in a bounded literal group.\n  {name} :: List Observation\n  {name} = [\n    '+',\n    '.join(entries)+'\n  ]\n'
            grouped.append((name,i))
        text+='\n  -- | Preserve the first independent row index as a nonzero process result.\n  status :: Nat\n  status =\n'
        for n,(name,offset) in enumerate(grouped):
            text+=f'    let result{n} = group {name} {offset} in\n    if result{n} > 0 then result{n} else\n'
        text+='    0\n'
        if suite=='pointer':
            pending_path=ROOT/'compiler/tests/fixtures/json-codec-addressability-pending.json'
            pending=json.loads(pending_path.read_text())['rows'][0]
            assert pending['error']['offset']==9 and pending['calls']==[]
            original_descriptor=descriptor.replace('main.pointerMethod',pending['type'])
            text+='\n  -- | Preserve the original reporting-disabled descriptor refusal with its exact published application type label.\n  retained :: Unit -> Bool\n  retained unit =\n    let counter = bytes-new 1 in\n    let selected = '+original_descriptor+' in\n    check selected (runtime-bool True) '+literal(pending['type'])+' counter (Observe \"quoted\" True False False \"compact\" True '+literal(bytes.fromhex(pending['output_hex']).decode())+' True 9 \"/value\" 0)\n'
            text+='\n  -- | Check the retained descriptor refusal before every new policy/format observation.\n  main :: Int\n  main = if retained () then nat-to-int status else 10001\n'
            manifest['retained_pending_source_sha256']=hashlib.sha256(pending_path.read_bytes()).hexdigest()
        else:
            text+='\n  -- | Report an exact nonzero row on any output, callback or semantic metadata mismatch.\n  main :: Int\n  main = nat-to-int status\n'
        target=ROOT/f'compiler/tests/fixtures/json-codec-error-{suite}.cinch'
        target.write_text(text)
        manifest['consumers'][suite]={'source_sha256':hashlib.sha256(target.read_bytes()).hexdigest(),'rows':len(rows),'source_bytes':target.stat().st_size}
    (ROOT/'bootstrap/evidence/stdlib/stdlib-json-error-context-cases.json').write_text(json.dumps(manifest,indent=2)+'\n')
    print(json.dumps(manifest,indent=2))
if __name__=='__main__':main()
