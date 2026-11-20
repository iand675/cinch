#!/usr/bin/env python3
"""Qualify only the exact source-positive pointer289 snapshot with its matched runtime."""
from pathlib import Path
import argparse
from pipeline_v3 import phase_guard,pause_guard,unarchived_bytes,UNARCHIVED,START
import hashlib,json,os,re,resource,subprocess,time
root=Path('/workspace/dasl-stdlib-json')
parser=argparse.ArgumentParser();parser.add_argument('suite',choices=['methods','updates']);parser.add_argument('--attempt',type=int,default=1);args=parser.parse_args()
phase_guard()
unit_limit=min(20*1024*1024,UNARCHIVED-unarchived_bytes()-128*1024)
assert unit_limit>=12*1024*1024,'archive completed successful ELF/IR before this runtime phase'
inputs=root/f'build/json-error-{args.suite}-probe-v{args.attempt}'
directory=root/f'build/json-error-{args.suite}-rust-v{args.attempt}'
receipt=root/f'compiler/stdlib-json-error-{args.suite}-rust-v{args.attempt}-results.json'
probe=json.loads((inputs/'report.json').read_text())
compiler=Path('/workspace/dasl-stdlib-io/build/boxed-nat-memory-safe-frozen/cinch')
runtime=compiler.parent/'libcinchruntime.a'
clang=Path('/workspace/.tools/llvm/usr/lib/llvm-20/bin/clang')
def digest(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def free():
 s=os.statvfs(root);return s.f_bavail*s.f_frsize
assert probe['exit_code']==0 and probe['diagnostic']==''
assert digest(compiler)==probe['compiler_sha256']=='855df83617f2ea84e7f0e3cf90b9755685bc23226594d81cd43ccd6168071b4e'
assert digest(runtime)==probe['provenance']['runtime_sha256']=='c4b1097b4b8e26541400228bdbbdcba1376101014823aeb2f5af619fbede57ac'
for name,expected in probe['sources'].items():assert digest(inputs/(name+'.cinch'))==expected,name
assert digest(inputs/'Main.cinch')==probe['fixture_sha256']
reference=root/'seed/tests/fixtures/json-codec-error-context-vectors.json'
assert digest(reference)=='c4525fda0a20808f082b657575f89240e7b543f48fad18ed58507dabce27f2d7'
assert free()>=START, f'start guard refused: {free()}'
assert not directory.exists() and not receipt.exists(),'refuse repeated qualification'
directory.mkdir();(directory/'runner.py').write_bytes(Path(__file__).read_bytes())
os.sched_setaffinity(0,{2})
_,hard=resource.getrlimit(resource.RLIMIT_STACK);resource.setrlimit(resource.RLIMIT_STACK,(65536*1024,hard))
env={**os.environ,'LD_LIBRARY_PATH':'/workspace/.tools/llvm/usr/lib/x86_64-linux-gnu'+(':'+os.environ['LD_LIBRARY_PATH'] if os.environ.get('LD_LIBRARY_PATH') else '')}
ir=directory/'consumer.ll';output=directory/'consumer'
emit=[str(compiler),'emit-ir',str(inputs/'Main.cinch'),'-o',str(ir),'--rc-debug']
link=[str(clang),'-O2',str(ir),str(runtime),'-ldl','-lpthread','-lm','-o',str(output)]
report={'description':'Exact frozen '+args.suite+' specialization only; no Native/full1494 claim','status':'prepared','frontend':'rust','compiler_sha256':digest(compiler),'provenance':probe['provenance'],'observed_runtime_archive_sha256':digest(runtime),'sources':probe['sources'],'fixture_sha256':probe['fixture_sha256'],'declaration_receipt_sha256':digest(inputs/'report.json'),'reference_sha256':digest(reference),'independent_rows':288 if args.suite=='methods' else 54,'ownership_control_groups':None,'sdk_render_panic_outcomes_retained':12,'runner_sha256':digest(Path(__file__)),'frozen_inputs':str(inputs),'raw_compile_command':emit,'raw_link_command':link,'clang_sha256':digest(clang),'opt_level':2,'cpu_affinity':[2],'compiler_host_stack_kib':65536,'consumer_stack_limit':'8M','free_bytes_before':free(),'new_output_limit_bytes':unit_limit,'runs':[]}
def budget():
 size=sum(p.stat().st_size for p in directory.rglob('*') if p.is_file())+(receipt.stat().st_size if receipt.exists() else 0)
 assert size<=unit_limit,f'output budget exceeded: {size} > {unit_limit}'
 pause_guard()
 return size
def save():
 receipt.write_text(json.dumps(report,indent=2)+'\n')
started=time.monotonic()
save()
try:
 report['status']='emitting';save();budget()
 emitted=subprocess.run(emit,env=env,cwd=inputs,text=True,capture_output=True,timeout=60)
 (directory/'emit.log').write_text(emitted.stdout+emitted.stderr)
 report['emit_exit_code']=emitted.returncode;report['emit_seconds']=time.monotonic()-started
 if ir.exists():report['llvm_sha256']=digest(ir)
 save();budget()
 assert emitted.returncode==0,emitted.stdout+emitted.stderr
 report['status']='linking';save();stage=time.monotonic()
 linked=subprocess.run(link,env=env,cwd=inputs,text=True,capture_output=True,timeout=30)
 (directory/'link.log').write_text(linked.stdout+linked.stderr)
 report['link_exit_code']=linked.returncode;report['link_seconds']=time.monotonic()-stage
 save();budget()
 assert linked.returncode==0,linked.stdout+linked.stderr
 report['executable_sha256']=digest(output);report['build_seconds']=time.monotonic()-started
 for poisoned in [False,True]:
  budget();report['status']='running-poisoned' if poisoned else 'running-normal';save()
  run_env={**env,'CINCH_HEAP_LIMIT':'256M' if poisoned else '32M','CINCH_STACK_SIZE':'8M','CINCH_LEAK_CHECK':'1','CINCH_RC_DEBUG':'1' if poisoned else '0','CINCH_ALLOC_STATS':'1'}
  run=subprocess.run([str(output)],env=run_env,cwd=directory,text=True,capture_output=True,timeout=30)
  (directory/('poisoned.log' if poisoned else 'ordinary.log')).write_text(run.stdout+run.stderr)
  observed={'poisoned_free_validation':poisoned,'heap_limit':run_env['CINCH_HEAP_LIMIT'],'exit_code':run.returncode,'stdout':run.stdout,'stderr':run.stderr}
  report['runs'].append(observed);save()
  assert run.returncode==0,f'exit {run.returncode}: {run.stdout}\n{run.stderr}'
  assert run.stdout=='' and 'leaked=0' in run.stderr,run.stdout+run.stderr
  line=next(line for line in run.stderr.splitlines() if line.startswith('cinch-alloc:'))
  observed['stats']={key:int(value) for key,value in re.findall(r'([a-z-]+)=(\d+)',line)}
  assert observed['stats']['live']==0,observed['stats']
  observed['leaked']=0;save()
 for name,expected in probe['sources'].items():assert digest(inputs/(name+'.cinch'))==expected,name
 assert digest(inputs/'Main.cinch')==probe['fixture_sha256'] and digest(runtime)==probe['provenance']['runtime_sha256']
 report['artifact_bytes']=budget();report['status']='passed';save()
 print(f'PASS: exact855/c4 {args.suite}, normal32M/poison256M stack8M zero live/leaks.',flush=True)
except BaseException as error:
 report['status']='failed-first-observation';report['failure']=repr(error);report['elapsed_seconds']=time.monotonic()-started
 save();print(report['failure'],flush=True);raise
