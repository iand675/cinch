#!/usr/bin/env python3
"""Verify split-epoch JSON error observations without rebuilding or promoting the package."""
from pathlib import Path
import gzip,hashlib,json,re,stat,tarfile
ROOT=Path(__file__).resolve().parents[1]
RUST='855df83617f2ea84e7f0e3cf90b9755685bc23226594d81cd43ccd6168071b4e'
RUNTIME='c4b1097b4b8e26541400228bdbbdcba1376101014823aeb2f5af619fbede57ac'
def digest(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def load(p):return json.loads((ROOT/p).read_text())
def stream_hash(stream):
 h=hashlib.sha256()
 while chunk:=stream.read(1024*1024):h.update(chunk)
 return h.hexdigest()
def verify(suite,epoch,rows,current_quoted=False):
 suffix=f'current-rust-v{epoch}' if current_quoted else f'rust-v{epoch}'
 name=f'bootstrap/evidence/stdlib/stdlib-json-error-{suite}-{suffix}-results.json';r=load(name)
 assert r['status']=='passed' and r['frontend']=='rust' and r['compiler_sha256']==RUST and r['observed_runtime_archive_sha256']==RUNTIME
 assert r['independent_rows']==rows and r['reference_sha256']==reference_hash and r['cpu_affinity']==[2] and r['opt_level']==2
 assert r['artifact_bytes']<=r['new_output_limit_bytes']<=20*1024*1024 and r['consumer_stack_limit']=='8M'
 frozen=Path(r['frozen_inputs']);source=json.loads((frozen/'report.json').read_text())
 assert source['status']=='passed' and source['exit_code']==0 and source['diagnostic']=='' and digest(frozen/'report.json')==r['declaration_receipt_sha256']
 for module,expected in r['sources'].items():assert digest(frozen/(module+'.cinch'))==expected
 fixture='json-codec-error-owner-control.cinch' if suite=='owner' else f'json-codec-error-{suite}.cinch'
 assert digest(frozen/'Main.cinch')==r['fixture_sha256']==digest(ROOT/'compiler/tests/fixtures'/fixture)
 for poisoned,row in zip([False,True],r['runs']):
  assert row['poisoned_free_validation']==poisoned and row['heap_limit']==('256M' if poisoned else '32M')
  assert row['exit_code']==0 and row['stdout']=='' and row['stats']['live']==0 and row['leaked']==0
 assert len(r['runs'])==2
 ledger_name=f'bootstrap/evidence/stdlib/stdlib-json-error-{suite}-{suffix}-functional-archive.json';ledger=load(ledger_name)
 assert ledger['execution_receipt_sha256']==digest(ROOT/name) and ledger['verified_before_unlink']
 archive=ROOT/ledger['archive_path'];assert digest(archive)==ledger['archive_sha256']
 with gzip.open(archive,'rb') as stream:assert stream_hash(stream)==ledger['decompressed_tar_sha256']
 expected_hashes={r['llvm_sha256'],r['executable_sha256']};assert {x['sha256'] for x in ledger['members']}==expected_hashes
 with tarfile.open(archive,'r:gz') as tar:
  entries=tar.getmembers();assert [x.name for x in entries]==[x['path'] for x in ledger['members']]
  for actual,expected in zip(entries,ledger['members']):
   assert actual.isfile() and actual.size==expected['bytes'] and stat.S_IMODE(actual.mode)==expected['mode']
   stream=tar.extractfile(actual);assert stream is not None and stream_hash(stream)==expected['sha256']
 return {'runtime_receipt':name,'runtime_receipt_sha256':digest(ROOT/name),'source_receipt_sha256':r['declaration_receipt_sha256'],'sources':r['sources'],'fixture_sha256':r['fixture_sha256'],'rows':rows,'bounded_memory_processes':2,'archive_index':ledger_name,'archive_index_sha256':digest(ROOT/ledger_name)}
reference=ROOT/'compiler/tests/fixtures/json-codec-error-context-vectors.json';reference_hash=digest(reference)
assert reference_hash=='c4525fda0a20808f082b657575f89240e7b543f48fad18ed58507dabce27f2d7'
oracle=json.loads(reference.read_text());assert {k:len(v) for k,v in oracle.items()}=={'tagged':1152,'method_errors':288,'updates':54}
assert sum('text_panic' in x['error'] for x in oracle['updates'])==12
assert all(x['error'].get('type_nil') and 'type' not in x['error'] for x in oracle['updates'] if 'text_panic' in x['error'])
receipts={'methods':verify('methods',6,288),'updates':verify('updates',1,54)}
for suite in ['pointer','owner','bool','string','integer']:
 receipts[suite]=verify(suite,2,None if suite=='owner' else 288,True)
 if suite=='owner':assert load(receipts[suite]['runtime_receipt'])['ownership_control_groups']==4
 if suite=='pointer':assert load(receipts[suite]['runtime_receipt'])['retained_original_descriptor_rows']==1
current=receipts['methods']['sources'];assert all(row['sources']==current for row in receipts.values())
for module,expected in current.items():assert digest(ROOT/'compiler/prelude'/(module.replace('.','/')+'.cinch'))==expected
for module in ['Value','Codec']:
 assert current['Text.Json.'+module]==load('bootstrap/evidence/stdlib/stdlib-json-error-docs-rust-v2.json')['modules'][module]['source_sha256']
# Earlier observations remain attached to their actual frozen source epoch, never relabeled as current.
quoted_name='bootstrap/evidence/stdlib/stdlib-json-quoted-rust-stage-checkpoint.json';quoted=load(quoted_name)
assert quoted['status']=='passed-rust-quoted-stage-only' and quoted['qualified_quoted_reference_rows']==1152 and quoted['bounded_memory_processes']==10
assert quoted['sources']['Text.Json.Value']=='bcb0702439d3520158155418fb35e41e9378b5d7bea602d620b24351cb5e5daa'
assert quoted['sources']['Text.Json.Codec']=='379ad24bb14efdf22fbbe4c776131fa521ceec770c7bd8667995272f8c44e881'
# Preserve every first genuine mismatch, including plain failed executable/LLVM bytes.
failed=load('bootstrap/evidence/stdlib/stdlib-json-method-v5-baseline.json')['immutable_failure_files']
for name,expected in failed.items():
 p=ROOT/name;assert p.is_file() and p.stat().st_size==expected['bytes'] and stat.S_IMODE(p.stat().st_mode)==expected['mode'] and digest(p)==expected['sha256'],name

def walk(rows):
 for row in rows:yield row;yield from walk(row.get('children',[]))
docs=load('bootstrap/evidence/stdlib/stdlib-json-error-docs-rust-v2.json');assert docs['status']=='passed' and docs['compiler_sha256']==RUST
canonical=0;doc_rows={}
for module in ['Value','Codec','Dynamic','Binary32','Legacy']:
 folder='json-error-docs-rust-v2' if module in ['Value','Codec'] else 'json-method-protocol-docs-v1' if module=='Dynamic' else 'json-dynamic-docs'
 p=ROOT/'build'/folder/(module+'.json');d=json.loads(p.read_text())
 assert d['sourceSha256']==digest(ROOT/f'compiler/prelude/Text/Json/{module}.cinch') and not d['diagnostics'] and not d['orphans']
 rows=list(walk(d['declarations']));assert all(x.get('doc') and not x['doc']['legacy'] and not re.search(r'\b(?:Go|Golang|Rust|Python)\b|C\+\+',x['doc']['markdown']) for x in rows)
 canonical+=len(rows);doc_rows[module]={'canonical_declarations':len(rows),'source_sha256':d['sourceSha256'],'docs_sha256':digest(p)}
assert canonical==854
plan=load('bootstrap/evidence/stdlib/stdlib-json-capability-plan.json');assert plan['reference_declarations']==92 and len(plan['declarations'])==92 and plan['status'].startswith('partial-')
report={'status':'passed-rust-error-context-stage-only','compiler_sha256':RUST,'runtime_sha256':RUNTIME,'current_sources':current,'current_source_reference_rows':1494,'current_source_bounded_memory_processes':14,'additional_original_pending_row':1,'additional_owner_groups':4,'current_receipts':receipts,'canonical_docs':doc_rows,'canonical_json_declarations':canonical,'earlier_quoted_epoch':{'checkpoint':'c5a8c0ecef7b5b50a3be0abb2390900da7fdbc95','receipt':quoted_name,'receipt_sha256':digest(ROOT/quoted_name),'sources':quoted['sources'],'reference_rows':1152,'additional_original_pending_row':1,'additional_owner_groups':4,'bounded_memory_processes':10},'all_reference_rows_accounted_by_actual_epoch':1494,'matched_current_source_full1494_claim':True,'sdk_render_panic_outcomes_retained':12,'panic_adaptation':'Observe literal absent type and original error metadata; reference panic strings are preserved. No panic/prose equality or deliberate Cinch crash is claimed.','first_failure_epochs_preserved_plain':4,'failed_plain_files_verified':len(failed),'historical_methods_v5_receipt_preserved':'bootstrap/evidence/stdlib/stdlib-json-error-methods-rust-v5-results.json','native_executed':False,'production_promoted':False,'full92_complete':False,'performance_claim':None,'remaining':['Value-sensitive nullable, interface and key method semantics','Reporting-disabled malformed-syntax partial updates remain unimplemented','Bounded Effect.Json source/decoder/encoder/token/More/Buffered and original IO causes','Actual public pipeline qualification and paired benchmarking','Native qualification and full92 reconciliation']}
(ROOT/'bootstrap/evidence/stdlib/stdlib-json-error-context-stage-checkpoint.json').write_text(json.dumps(report,indent=2)+'\n')
print('PASS: current1494 reference rows+retained1+owner4/14 memory processes;854 canonical docs; historical epochs preserved, no promotion.')
