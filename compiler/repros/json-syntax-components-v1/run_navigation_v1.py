#!/usr/bin/env python3
"""One source-first focused compatibility unit with exact immutable evidence and bounded successful-output recovery."""
from pathlib import Path
import argparse,sys,json,os,subprocess,resource,hashlib,stat,time,gzip,tarfile,re
ROOT=Path('/workspace/dasl-stdlib-json');sys.path.insert(0,str(ROOT/'build/json-next-error-context'))
from pipeline_common import digest,free,scope_files,fsync_dir,MIB
p=argparse.ArgumentParser();p.add_argument('unit',choices=['navigation']);a=p.parse_args()
assert free()>=350*MIB
folder=ROOT/f'build/json-error-syntax-{a.unit}-v1';assert not folder.exists();folder.mkdir()
source=folder/'source';source.mkdir();output=folder/'runtime';output.mkdir()
fixture=ROOT/'seed/tests/fixtures'/'json-syntax-partial-navigation.cinch'
seed=ROOT/'build/json-error-methods-probe-v7';seed_report=json.loads((seed/'report.json').read_text())
expected={'Text.Json.Value':'2f95c9abc08a221a12fd2b5d810faeeffff726a0408f6323af01f67f9ca19931','Text.Json.Codec':'8ea76ca75af1cf37304ef924d06462d0efbde16340eee99a676f57f1d02558f3'}
inputs={}
for name,original_hash in seed_report['sources'].items():
 src=ROOT/'seed/prelude'/(name.replace('.','/')+'.cinch') if name in expected else seed/(name+'.cinch')
 assert digest(src)==expected.get(name,original_hash),name
 target=source/(name+'.cinch');target.write_bytes(src.read_bytes());inputs[name]=digest(target)
(source/'Main.cinch').write_bytes(fixture.read_bytes())
compiler=Path('/workspace/dasl-stdlib-io/build/boxed-nat-memory-safe-frozen/cinch');runtime=compiler.parent/'libcinchruntime.a';clang=Path('/workspace/.tools/llvm/usr/lib/llvm-20/bin/clang')
assert digest(compiler)=='855df83617f2ea84e7f0e3cf90b9755685bc23226594d81cd43ccd6168071b4e' and digest(runtime)=='c4b1097b4b8e26541400228bdbbdcba1376101014823aeb2f5af619fbede57ac'
os.sched_setaffinity(0,{2});_,hard=resource.getrlimit(resource.RLIMIT_STACK);resource.setrlimit(resource.RLIMIT_STACK,(65536*1024,hard))
env={**os.environ,'LD_LIBRARY_PATH':'/workspace/.tools/llvm/usr/lib/x86_64-linux-gnu'+(':'+os.environ['LD_LIBRARY_PATH'] if os.environ.get('LD_LIBRARY_PATH') else '')}
receipt=ROOT/f'compiler/stdlib-json-syntax-{a.unit}-rust-v1-results.json';assert not receipt.exists()
r={'status':'prepared','scope':'One-pass partial-navigation component only; 902rows reference-only, selected1494 proof immutable; typed partial-mutation remains unimplemented','unit':a.unit,'control_groups':10,'compiler_sha256':digest(compiler),'runtime_sha256':digest(runtime),'sources':inputs,'fixture_path':str(fixture),'fixture_sha256':digest(fixture),'free_bytes_before':free(),'start_guard_bytes':350*MIB,'reserve_bytes':250*MIB,'output_limit_bytes':30*MIB,'archive_scratch_limit_bytes':10*MIB,'SDK_evidence_sha256':digest(ROOT/'seed/tests/fixtures/json-codec-syntax-partial-vectors.json'),'runs':[]}
def save():receipt.write_text(json.dumps(r,indent=2)+'\n')
def budget():
 total=sum(x.stat().st_size for x in folder.rglob('*') if x.is_file() and not x.name.endswith('.tar.gz'))
 assert total<=30*MIB and free()>=250*MIB,(total,free())
 return total
def command(argv,label,seconds):
 r[label+'_command']=argv;save();start=time.monotonic();v=subprocess.run(argv,cwd=source,env=env,capture_output=True,text=True,timeout=seconds);(output/(label+'.log')).write_text(v.stdout+v.stderr);r[label+'_seconds']=time.monotonic()-start;r[label+'_exit_code']=v.returncode;save();assert v.returncode==0,v.stdout+v.stderr;budget();return v
save()
try:
 command([str(compiler),'check',str(source/'Main.cinch')],'check',15);assert sum(x.stat().st_size for x in source.rglob('*') if x.is_file())<=5*MIB
 print(a.unit+' source PASS',flush=True)
 ir=output/'consumer.ll';exe=output/'consumer'
 command([str(compiler),'emit-ir',str(source/'Main.cinch'),'-o',str(ir),'--rc-debug'],'emit',60)
 command([str(clang),'-O2',str(ir),str(runtime),'-ldl','-lpthread','-lm','-o',str(exe)],'link',30)
 assert ir.stat().st_size+exe.stat().st_size<=20*MIB
 r['llvm_sha256']=digest(ir);r['executable_sha256']=digest(exe);save()
 for poison in [False,True]:
  budget();e={**env,'CINCH_HEAP_LIMIT':'256M' if poison else '32M','CINCH_STACK_SIZE':'8M','CINCH_LEAK_CHECK':'1','CINCH_RC_DEBUG':'1' if poison else '0','CINCH_ALLOC_STATS':'1'}
  v=subprocess.run([str(exe)],cwd=output,env=e,capture_output=True,text=True,timeout=30);(output/('poison.log' if poison else 'normal.log')).write_text(v.stdout+v.stderr)
  row={'poisoned':poison,'exit_code':v.returncode,'stdout':v.stdout,'stderr':v.stderr};r['runs'].append(row);save()
  assert v.returncode==0 and v.stdout=='',row
  assert 'live=0' in v.stderr and 'leaked=0' in v.stderr,row
 r['status']='passed';r['artifact_bytes']=budget();save()
 print(a.unit+' both runtime profiles PASS',flush=True)
except BaseException as error:
 r['status']='failed-first-observation';r['failure']=repr(error);save();raise
