#!/usr/bin/env python3
"""Check one independently generated specialization using sealed source-positive modules."""
from pathlib import Path
import argparse,json,os,resource,stat,subprocess,time
from pipeline_methods_v7 import ROOT,FROZEN,digest,free,phase_guard,pause_guard,unarchived_bytes
OVERRIDES={'Text.Json.Value': 'a330ddfa8c6b0ee57f842efc1f73d3a8c46c9173314704121f644cedba7a3758', 'Text.Json.Codec': '8ea76ca75af1cf37304ef924d06462d0efbde16340eee99a676f57f1d02558f3'}
p=argparse.ArgumentParser();p.add_argument('suite',choices=['methods']);p.add_argument('--attempt',type=int,default=7);args=p.parse_args()
phase_guard()
seed=json.loads((FROZEN/'report.json').read_text());assert seed['exit_code']==0 and seed['diagnostic']==''
compiler=Path('/workspace/dasl-stdlib-io/build/boxed-nat-memory-safe-frozen/cinch')
assert digest(compiler)==seed['compiler_sha256']=='855df83617f2ea84e7f0e3cf90b9755685bc23226594d81cd43ccd6168071b4e'
for name,expected in seed['sources'].items():assert digest(FROZEN/(name+'.cinch'))==expected,name
fixture=ROOT/f'seed/tests/fixtures/json-codec-error-{args.suite}.cinch'
directory=ROOT/f'build/json-error-{args.suite}-probe-v{args.attempt}'
assert not directory.exists(),'refuse repeated source epoch';directory.mkdir()
for name,expected in seed['sources'].items():
 source=FROZEN/(name+'.cinch');target=directory/source.name
 if name in OVERRIDES:
  source=ROOT/'seed/prelude'/(name.replace('.','/')+'.cinch')
  expected=OVERRIDES[name]
  target.write_bytes(source.read_bytes())
 else: os.link(source,target)
 seed['sources'][name]=expected
 assert digest(target)==expected and stat.S_IMODE(target.stat().st_mode)==stat.S_IMODE(source.stat().st_mode)
(directory/'Main.cinch').write_bytes(fixture.read_bytes())
os.sched_setaffinity(0,{2});_,hard=resource.getrlimit(resource.RLIMIT_STACK);resource.setrlimit(resource.RLIMIT_STACK,(65536*1024,hard))
command=[str(compiler),'check',str(directory/'Main.cinch')]
report={'description':'Source-only independent '+args.suite+' specialization; no runtime/promotion claim','compiler_sha256':digest(compiler),'provenance':seed['provenance'],'sources':seed['sources'],'fixture_sha256':digest(directory/'Main.cinch'),'command':command,'fixture_source':str(fixture),'source_attempt':args.attempt,'cpu_affinity':[2],'free_bytes_before':free(),'status':'checking','control_groups':None,'independent_rows':288 if args.suite=='methods' else 54,'reference_sha256':digest(ROOT/'seed/tests/fixtures/json-codec-error-context-vectors.json')}
started=time.monotonic()
try:
 run=subprocess.run(command,cwd=directory,capture_output=True,text=True,timeout=15)
 diagnostic=run.stdout+run.stderr;(directory/'check.log').write_text(diagnostic)
 report.update(exit_code=run.returncode,diagnostic=diagnostic,elapsed_seconds=time.monotonic()-started,status='passed' if run.returncode==0 else 'failed-source')
 (directory/'report.json').write_text(json.dumps(report,indent=2)+'\n')
 assert run.returncode==0,diagnostic
 pause_guard()
 assert sum(x.stat().st_size for x in directory.rglob("*") if x.is_file())<5*1024*1024
 print(f'Source PASS {args.suite}: {report["elapsed_seconds"]:.3f}s, unarchived={unarchived_bytes()}B',flush=True)
except BaseException as error:
 report.update(failure=repr(error),elapsed_seconds=time.monotonic()-started)
 if report['status']=='checking':report['status']='failed-source-attempt'
 (directory/'report.json').write_text(json.dumps(report,indent=2)+'\n');raise
