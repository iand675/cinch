#!/usr/bin/env python3
"""One source-first focused compatibility unit with exact immutable evidence and bounded successful-output recovery."""
from pathlib import Path
import argparse,sys,json,os,subprocess,resource,hashlib,stat,time,gzip,tarfile,re
ROOT=Path('/workspace/dasl-stdlib-json');sys.path.insert(0,str(ROOT/'build/json-next-error-context'))
from pipeline_common import digest,free,scope_files,fsync_dir,MIB
p=argparse.ArgumentParser();p.add_argument('unit',choices=['observations-int', 'observations-record', 'observations-pointer-record', 'observations-slice', 'observations-array', 'observations-map', 'observations-any', 'observations-raw', 'observations-json-hook', 'observations-text-hook']);a=p.parse_args()
assert free()>=350*MIB
folder=ROOT/f'build/json-error-syntax-{a.unit}-v1';assert not folder.exists();folder.mkdir()
source=folder/'source';source.mkdir();output=folder/'runtime';output.mkdir()
model=a.unit.removeprefix('observations-')
manifest_path=ROOT/'bootstrap/evidence/stdlib/stdlib-json-syntax-partial-consumer-manifest-v1.json';manifest=json.loads(manifest_path.read_text());consumer=next(x for x in manifest['consumers'] if x['model']==model)
fixture=ROOT/consumer['path'];assert digest(fixture)==consumer['sha256']
(folder/'consumer-manifest.json').write_bytes(manifest_path.read_bytes())
(folder/'executed-runner.py').write_bytes(Path(__file__).read_bytes())
(folder/'generator.py').write_bytes((ROOT/'tooling/generate_json_syntax_partial_cases.py').read_bytes())
seed=ROOT/'build/json-error-methods-probe-v7';seed_report=json.loads((seed/'report.json').read_text())
expected={'Text.Json.Value':'1b31989157537d41aa7efc5071737db4180f26fa3281a6ab2d71b8f3fa8bb358','Text.Json.Codec':'17b75fa73b491bb5cf08ca3638ae4ff10f800793abf1edd735021f291c560182'}
inputs={}
for name,original_hash in seed_report['sources'].items():
 src=ROOT/'compiler/prelude'/(name.replace('.','/')+'.cinch') if name in expected else seed/(name+'.cinch')
 assert digest(src)==expected.get(name,original_hash),name
 target=source/(name+'.cinch');target.write_bytes(src.read_bytes());inputs[name]=digest(target)
for name in ['Text.Json.Dynamic','Data.Bytes.Utf8']:
 src=ROOT/'compiler/prelude'/(name.replace('.','/')+'.cinch');target=source/(name+'.cinch')
 assert digest(src)=={'Text.Json.Dynamic':'d8149a32f3328af47877b48db9c928ab844c34564a2f9fb9fae88d446c68beb6','Data.Bytes.Utf8':'f1e87cec4f82029b165271e0da2315d2b647ca04aeb4648c1c0b5907db033187'}[name]
 target.write_bytes(src.read_bytes());inputs[name]=digest(target)
(source/'Main.cinch').write_bytes(fixture.read_bytes())
compiler=Path('/workspace/dasl-stdlib-io/build/boxed-nat-memory-safe-frozen/cinch');runtime=compiler.parent/'libcinchruntime.a';clang=Path('/workspace/.tools/llvm/usr/lib/llvm-20/bin/clang')
assert digest(compiler)=='855df83617f2ea84e7f0e3cf90b9755685bc23226594d81cd43ccd6168071b4e' and digest(runtime)=='c4b1097b4b8e26541400228bdbbdcba1376101014823aeb2f5af619fbede57ac'
os.sched_setaffinity(0,{2});_,hard=resource.getrlimit(resource.RLIMIT_STACK);resource.setrlimit(resource.RLIMIT_STACK,(65536*1024,hard))
env={**os.environ,'LD_LIBRARY_PATH':'/workspace/.tools/llvm/usr/lib/x86_64-linux-gnu'+(':'+os.environ['LD_LIBRARY_PATH'] if os.environ.get('LD_LIBRARY_PATH') else '')}
receipt=ROOT/f'bootstrap/stdlib-json-syntax-{a.unit}-rust-v1-results.json';assert not receipt.exists()
r={'status':'prepared','scope':'One existing-oracle model only; all902/full92 and current1494 regression remain separately pending; exact full application state, Result/error family/offset/pointer/type/kind/reported bytes and callback counts/arguments checked','unit':a.unit,'independent_rows':consumer['rows'],'original_row_indices':consumer['indices'],'generator_sha256':digest(folder/'generator.py'),'consumer_manifest_sha256':digest(manifest_path),'compiler_sha256':digest(compiler),'runtime_sha256':digest(runtime),'sources':inputs,'fixture_path':str(fixture),'fixture_sha256':digest(fixture),'free_bytes_before':free(),'start_guard_bytes':350*MIB,'reserve_bytes':250*MIB,'output_limit_bytes':30*MIB,'archive_scratch_limit_bytes':10*MIB,'SDK_evidence_sha256':digest(ROOT/'compiler/tests/fixtures/json-codec-syntax-partial-vectors.json'),'runs':[]}
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
