#!/usr/bin/env python3
"""Verify the frozen Rust-only quoted stage, including archived outputs, without promoting it."""
from pathlib import Path
import gzip,hashlib,json,stat,tarfile
ROOT=Path(__file__).resolve().parents[1]
RUST='855df83617f2ea84e7f0e3cf90b9755685bc23226594d81cd43ccd6168071b4e'
RUNTIME='c4b1097b4b8e26541400228bdbbdcba1376101014823aeb2f5af619fbede57ac'
def digest(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def load(p):return json.loads((ROOT/p).read_text())
def stream_hash(stream):
 h=hashlib.sha256()
 while chunk:=stream.read(1024*1024):h.update(chunk)
 return h.hexdigest()
common=None;receipts={};processes=0;qualified_rows=0
for suite in ['pointer','owner','bool','string','integer']:
 path=f'compiler/evidence/stdlib/stdlib-json-error-{suite}-rust-v1-results.json';r=load(path)
 assert r['status']=='passed' and r['frontend']=='rust' and r['compiler_sha256']==RUST
 assert r['observed_runtime_archive_sha256']==RUNTIME and r['opt_level']==2 and r['cpu_affinity']==[2] and r['consumer_stack_limit']=='8M'
 assert r['artifact_bytes']<=r['new_output_limit_bytes']<=20*1024*1024
 assert r['free_bytes_before']>=(250 if suite=='pointer' else 350)*1024*1024
 frozen=Path(r['frozen_inputs']);source=json.loads((frozen/'report.json').read_text())
 assert source['exit_code']==0 and source['diagnostic']=='' and digest(frozen/'report.json')==r['declaration_receipt_sha256']
 if common is None:common=r['sources']
 assert r['sources']==common
 for name,expected in common.items():assert digest(frozen/(name+'.cinch'))==expected,(suite,name)
 assert digest(frozen/'Main.cinch')==r['fixture_sha256']
 fixture='json-codec-error-owner-control.cinch' if suite=='owner' else f'json-codec-error-{suite}.cinch'
 assert digest(ROOT/'seed/tests/fixtures'/fixture)==r['fixture_sha256']
 assert len(r['runs'])==2
 for poisoned,row in zip([False,True],r['runs']):
  assert row['poisoned_free_validation']==poisoned and row['heap_limit']==('256M' if poisoned else '32M')
  assert row['exit_code']==0 and row['stdout']=='' and row['stats']['live']==0 and row['leaked']==0
  processes+=1
 ledger_path=f'compiler/evidence/stdlib/stdlib-json-error-{suite}-rust-v1-functional-archive.json';ledger=load(ledger_path)
 assert ledger['verified_before_unlink'] and ledger['execution_receipt_sha256']==digest(ROOT/path)
 archive=ROOT/ledger['archive_path'];assert digest(archive)==ledger['archive_sha256']
 with gzip.open(archive,'rb') as stream:assert stream_hash(stream)==ledger['decompressed_tar_sha256']
 link=r['raw_link_command'];expected_paths={str(Path(link[2]).relative_to(ROOT)):r['llvm_sha256'],str(Path(link[link.index('-o')+1]).relative_to(ROOT)):r['executable_sha256']}
 assert {row['path']:row['sha256'] for row in ledger['members']}==expected_paths
 with tarfile.open(archive,'r:gz') as tar:
  entries=tar.getmembers();assert [m.name for m in entries]==[m['path'] for m in ledger['members']]
  for entry,expected in zip(entries,ledger['members']):
   assert entry.isfile() and entry.size==expected['bytes'] and stat.S_IMODE(entry.mode)==expected['mode']
   stream=tar.extractfile(entry);assert stream is not None and stream_hash(stream)==expected['sha256']
 if suite!='owner':qualified_rows+=288
 receipts[suite]={'runtime_receipt':path,'runtime_receipt_sha256':digest(ROOT/path),'source_receipt_sha256':r['declaration_receipt_sha256'],'fixture_sha256':r['fixture_sha256'],'recovery_index':ledger_path,'recovery_index_sha256':digest(ROOT/ledger_path),'archive_sha256':ledger['archive_sha256'],'tested_output_bytes':r['artifact_bytes']}
reference=ROOT/'seed/tests/fixtures/json-codec-error-context-vectors.json';oracle=json.loads(reference.read_text())
assert digest(reference)=='c4525fda0a20808f082b657575f89240e7b543f48fad18ed58507dabce27f2d7'
assert {key:len(rows) for key,rows in oracle.items()}=={'method_errors':288,'tagged':1152,'updates':54}
assert sum('text_panic' in row['error'] for rows in oracle.values() for row in rows)==12
assert qualified_rows==1152 and processes==10
cases=load('compiler/evidence/stdlib/stdlib-json-error-context-cases.json');assert cases['reference_sha256']==digest(reference)
for suite,row in cases['consumers'].items():assert row['rows']==288 and row['source_sha256']==receipts[suite]['fixture_sha256']
pending=ROOT/'seed/tests/fixtures/json-codec-addressability-pending.json';assert digest(pending)==cases['retained_pending_source_sha256']
assert len(json.loads(pending.read_text())['rows'])==1
plan=load('compiler/evidence/stdlib/stdlib-json-capability-plan.json');assert plan['reference_declarations']==92 and len(plan['declarations'])==92 and plan['status'].startswith('partial-')
doc_path=ROOT/'compiler/evidence/stdlib/stdlib-json-quoted-docs-rust-v1.json';docs=None;unchanged_docs={};canonical_total=None
if doc_path.exists():
 docs=json.loads(doc_path.read_text());assert docs['status']=='passed' and docs['compiler_sha256']==RUST
 for name,row in docs['modules'].items():
  assert row['source_sha256']==common['Text.Json.'+name]
  assert digest(ROOT/f'build/json-error-docs-rust-v1/{name}.json')==row['docs_sha256']
 def walk(rows):
  for row in rows:
   yield row;yield from walk(row.get('children',[]))
 for name in ['Dynamic','Binary32','Legacy']:
  folder='json-method-protocol-docs-v1' if name=='Dynamic' else 'json-dynamic-docs'
  artifact=ROOT/'build'/folder/(name+'.json');data=json.loads(artifact.read_text())
  assert data['sourceSha256']==digest(ROOT/f'seed/prelude/Text/Json/{name}.cinch') and not data['orphans'] and not data['diagnostics']
  rows=list(walk(data['declarations']));assert all(row.get('doc') and not row['doc']['legacy'] for row in rows)
  unchanged_docs[name]={'source_sha256':data['sourceSha256'],'canonical_declarations':len(rows),'docs_sha256':digest(artifact)}
 canonical_total=sum(row['canonical_declarations'] for row in docs['modules'].values())+sum(row['canonical_declarations'] for row in unchanged_docs.values())
 assert canonical_total==822
working={name:digest(ROOT/'seed/prelude'/(name.replace('.','/')+'.cinch'))==expected for name,expected in common.items()}
report={'status':'passed-rust-quoted-stage-only','description':'Rust855/c4 only: all1152 quoted-field outcomes plus original retained descriptor refusal and4 owner/error-family groups; no Native/family/package promotion','compiler_sha256':RUST,'runtime_sha256':RUNTIME,'sources':common,'working_sources_match':working,'source_and_runtime_receipts':receipts,'qualified_quoted_reference_rows':1152,'retained_original_descriptor_rows':1,'additional_ownership_error_family_groups':4,'bounded_memory_processes':processes,'independent_oracle_rows_retained':1494,'sdk_error_render_panic_outcomes_retained':12,'canonical_docs':docs,'unchanged_canonical_docs':unchanged_docs,'canonical_json_declarations':canonical_total,'canonical_docs_status':'passed' if docs else 'pending-resource-guard','remaining_existing_oracle_rows':{'method_errors':288,'updates':54},'native_executed':False,'production_promoted':False,'full92_complete':False,'performance_claim':None,'remaining':['Method JSON/text error identity and context; all12 SDK panic observations remain explicit','All54 read/type/record/state outcomes and missing error metadata capabilities','Value-sensitive nullable/interface/key semantics','Reporting-disabled malformed-syntax partial updates','Bounded Effect.Json and actual public pipeline evidence','Native qualification and final92 reconciliation']}
output=ROOT/'compiler/evidence/stdlib/stdlib-json-quoted-rust-stage-checkpoint.json';output.write_text(json.dumps(report,indent=2)+'\n')
print(f'PASS:1152quoted+retained1+owner4,10 memory processes; docs={report["canonical_docs_status"]}; Rust-only, no promotion.')
