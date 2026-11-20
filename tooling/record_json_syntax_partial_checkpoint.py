#!/usr/bin/env python3
"""Verify exact current-source syntax-partial observations without executing compilers or reference providers."""
from pathlib import Path
import json,hashlib,tarfile,gzip,stat
ROOT=Path(__file__).resolve().parents[1]
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def read(rel):return json.loads((ROOT/rel).read_text())
def verify_archive(path,receipt):
 p=ROOT/path;a=json.loads(p.read_text());assert a['status']=='verified-closed-success' and a['verified_before_unlink']
 rp=ROOT/a['execution_receipt'];assert sha(rp)==a['execution_receipt_sha256'] and json.loads(rp.read_text())==receipt
 archive=ROOT/a['archive_path'];assert sha(archive)==a['archive_sha256'] and archive.stat().st_size==a['archive_bytes']
 h=hashlib.sha256()
 with gzip.open(archive,'rb') as f:
  while b:=f.read(1024*1024):h.update(b)
 assert h.hexdigest()==a['decompressed_tar_sha256']
 with tarfile.open(archive,'r:gz') as f:
  rows=f.getmembers();assert len(rows)==len(a['members'])
  for got,want in zip(rows,a['members']):
   assert got.isfile() and got.name==want['path'] and got.size==want['bytes'] and stat.S_IMODE(got.mode)==want['mode']
   assert hashlib.sha256(f.extractfile(got).read()).hexdigest()==want['sha256']
 return {'path':path,'sha256':sha(p),'archive_sha256':a['archive_sha256']}
def verify_runtime(rel,expected_sources,fixture):
 p=ROOT/rel;r=json.loads(p.read_text());assert r['status']=='passed' and r['sources']==expected_sources
 assert r['compiler_sha256']=='855df83617f2ea84e7f0e3cf90b9755685bc23226594d81cd43ccd6168071b4e' and r['runtime_sha256']=='c4b1097b4b8e26541400228bdbbdcba1376101014823aeb2f5af619fbede57ac'
 assert r['check_exit_code']==r['emit_exit_code']==r['link_exit_code']==0
 assert len(r['runs'])==2 and {x['poisoned'] for x in r['runs']}=={False,True}
 for x in r['runs']:assert x['exit_code']==0 and x['stdout']=='' and 'live=0' in x['stderr'] and 'leaked=0' in x['stderr']
 assert r['fixture_sha256']==sha(ROOT/fixture)
 # Execution snapshots are immutable inputs, never recompiled during this verification.
 folder=Path(r['emit_command'][2]).parent
 for name,h in expected_sources.items():assert sha(folder/(name+'.cinch'))==h
 assert sha(folder/'Main.cinch')==r['fixture_sha256']
 return r
oracle=ROOT/'compiler/tests/fixtures/json-codec-syntax-partial-vectors.json';assert sha(oracle)=='7257d12bb9947b25b8fc3afcc29d4a365ec94c585f12b19a9bcf657eb34c2d8d'
observations=json.loads(oracle.read_text());assert len(observations)==902
manifest_path='bootstrap/evidence/stdlib/stdlib-json-syntax-partial-consumer-manifest-v2.json';manifest=read(manifest_path);assert manifest['rows']==902 and manifest['oracle_sha256']==sha(oracle)
sources=read('bootstrap/evidence/stdlib/stdlib-json-syntax-observations-slice-rust-v4-results.json')['sources']
for name,h in sources.items():assert sha(ROOT/'compiler/prelude'/(name.replace('.','/')+'.cinch'))==h
units={};indices=[]
for entry in manifest['consumers']:
 model=entry['model'];fixture=entry['path'];assert sha(ROOT/fixture)==entry['sha256']
 version='v5' if model in ['record','pointer-record'] else 'v4'
 rel=f'bootstrap/evidence/stdlib/stdlib-json-syntax-observations-{model}-rust-{version}-results.json';r=verify_runtime(rel,sources,fixture)
 assert r['independent_rows']==entry['rows'] and r['original_row_indices']==entry['indices'];indices+=r['original_row_indices']
 for index in entry['indices']:assert observations[index]['model']==model
 arc=verify_archive(f'bootstrap/evidence/stdlib/stdlib-json-syntax-observations-{model}-{version}-functional-archive.json',r)
 units[model]={'receipt':rel,'receipt_sha256':sha(ROOT/rel),'rows':entry['rows'],'fixture':fixture,'fixture_sha256':entry['sha256'],'archive':arc,'memory_processes':2}
assert len(indices)==len(set(indices))==902 and sorted(indices)==list(range(902))
focused_rel='bootstrap/evidence/stdlib/stdlib-json-syntax-grounded-controls-rust-v1-results.json';focus=verify_runtime(focused_rel,sources,'compiler/tests/fixtures/json-syntax-grounded-controls-v1.cinch');assert focus['control_groups']==29
focused_archive=verify_archive('bootstrap/evidence/stdlib/stdlib-json-syntax-grounded-controls-v1-functional-archive.json',focus)
docpath='bootstrap/evidence/stdlib/stdlib-json-syntax-feature-docs-v1.json';docs=read(docpath);assert docs['status']=='passed' and docs['source_hashes_before']==docs['source_hashes_after']
for name,row in docs['modules'].items():
 assert row['source_sha256']==sources['Text.Json.'+name] and not row['diagnostics'] and not row['orphans']
 assert len(row['canonical_source_ids'])==row['canonical_declarations']
 p=ROOT/'build/json-error-syntax-feature-docs-v1'/(name+'.json');assert sha(p)==row['docs_sha256']
# Historical failures retain their actual source epochs and remain plain.
failures={}
for model,version in [('int','v1'),('record','v2'),('slice','v3')]:
 rel=f'bootstrap/evidence/stdlib/stdlib-json-syntax-observations-{model}-rust-{version}-results.json';r=read(rel);assert r['status']=='failed-first-observation' and len(r['runs'])==1 and r['runs'][0]['exit_code']!=0
 folder=ROOT/f'build/json-error-syntax-observations-{model}-{version}/runtime'
 assert sha(folder/'consumer.ll')==r['llvm_sha256'] and sha(folder/'consumer')==r['executable_sha256']
 failures[model]={'receipt':rel,'receipt_sha256':sha(ROOT/rel),'source_hashes':r['sources'],'original_failure_stdout':r['runs'][0]['stdout'],'plain_failed_ELF_IR_preserved':True}
out={'status':'passed-current-source-rust-syntax-partial-stage-only','compiler_sha256':focus['compiler_sha256'],'runtime_sha256':focus['runtime_sha256'],'sources':sources,'oracle_sha256':sha(oracle),'oracle_rows':902,'units':units,'model_memory_processes':20,'focused_controls':29,'focused_receipt':focused_rel,'focused_receipt_sha256':sha(ROOT/focused_rel),'focused_archive':focused_archive,'focused_memory_processes':2,'canonical_changed_module_docs':sum(x['canonical_declarations'] for x in docs['modules'].values()),'docs_receipt':docpath,'docs_receipt_sha256':sha(ROOT/docpath),'canonical_doc_diagnostics':0,'canonical_doc_orphans':0,'preserved_first_failure_epochs':failures,'historical_selected1494_oracle_sha256':sha(ROOT/'compiler/tests/fixtures/json-codec-error-context-vectors.json') if (ROOT/'compiler/tests/fixtures/json-codec-error-context-vectors.json').exists() else 'See selected final-compat manifest; no regeneration','current1494_affected_regression':'pending-separate-current-source-gate','scope_remaining':['Additional partial Unicode and delimiter/method/interface/key corners beyond902','Affected1494 plus retained/owner source-matched regressions before production promotion','Full92 Effect.Json public IO pipelines, exact original IO causes and advanced handles','Native qualification and paired public pipeline performance gates'],'no_family_native_performance_or_package_completion_claim':True}
path=ROOT/'bootstrap/evidence/stdlib/stdlib-json-syntax-partial-stage-checkpoint.json';path.write_text(json.dumps(out,indent=2)+'\n');print('PASS current902 +focused29,22 bounded memory processes; full92 remains open')
