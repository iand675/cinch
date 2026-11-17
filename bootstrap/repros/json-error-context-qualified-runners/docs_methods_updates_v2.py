#!/usr/bin/env python3
"""Extract canonical docs for the exact Rust-qualified error/output source without adopting a compiler."""
from pathlib import Path
import json,os,re,subprocess,resource
from pipeline_methods_v6 import ROOT,FROZEN,digest,free,pause_guard
assert free()>=350*1024*1024,f'tiny docs start guard: {free()}'
compiler=Path('/workspace/dasl-stdlib-io/build/boxed-nat-memory-safe-frozen/cinch')
assert digest(compiler)=='855df83617f2ea84e7f0e3cf90b9755685bc23226594d81cd43ccd6168071b4e'
source=json.loads((ROOT/'build/json-error-updates-probe-v1/report.json').read_text())
folder=ROOT/'build/json-error-docs-rust-v2';assert not folder.exists();folder.mkdir()
os.sched_setaffinity(0,{2});report={'status':'extracting','compiler_sha256':digest(compiler),'start_guard_bytes':350*1024*1024,'reserve_bytes':250*1024*1024,'output_limit_bytes':5*1024*1024,'scope':'two parse-only doc extractions; no typecheck/emission/runtime/reference','modules':{}}
def walk(rows):
 for row in rows:
  yield row;yield from walk(row.get('children',[]))
for name in ['Value','Codec']:
 p=ROOT/f'compiler/prelude/Text/Json/{name}.cinch';assert digest(p)==source['sources']['Text.Json.'+name]
 output=folder/(name+'.json');command=[str(compiler),'doc','extract',str(p),'--out',str(output)]
 remaining=5*1024*1024-sum(x.stat().st_size for x in folder.rglob('*') if x.is_file())-16384
 def child_limit():resource.setrlimit(resource.RLIMIT_FSIZE,(remaining,remaining))
 result=subprocess.run(command,capture_output=True,text=True,timeout=15,preexec_fn=child_limit);(folder/(name+'.log')).write_text(result.stdout+result.stderr)
 assert result.returncode==0,result.stdout+result.stderr
 data=json.loads(output.read_text());assert data['sourceSha256']==digest(p) and not data['diagnostics'] and not data['orphans']
 rows=list(walk(data['declarations']))
 for row in rows:
  assert row.get('doc') and not row['doc']['legacy'],row['id']
  assert not re.search(r'\b(?:Go|Golang|Rust|Python)\b|C\+\+',row['doc']['markdown']),row['id']
 report['modules'][name]={'source_sha256':digest(p),'canonical_declarations':len(rows),'docs_sha256':digest(output),'command':command}
 assert sum(x.stat().st_size for x in folder.rglob('*') if x.is_file())<5*1024*1024
 pause_guard()
report['status']='passed';(ROOT/'bootstrap/stdlib-json-error-docs-rust-v2.json').write_text(json.dumps(report,indent=2)+'\n')
print(json.dumps(report,indent=2),flush=True)
