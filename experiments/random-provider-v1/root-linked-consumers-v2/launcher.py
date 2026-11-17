"""Link qualified public consumer objects against the new integrated runtime only."""
from pathlib import Path
import subprocess,json,os,hashlib,shutil,time
root=Path('/workspace/dasl-stdlib-io');out=root/'experiments/random-provider-v1/root-linked-consumers-v2';archive=root/'build/stdlib-entropy-runtime-frozen/libcinchruntime.a';prefix=root/'experiments/random-provider-v1/staticlib-pair-v1';clang=Path('/workspace/.tools/llvm/usr/lib/llvm-20/bin/clang');env={**os.environ,'LD_LIBRARY_PATH':'/workspace/.tools/llvm/usr/lib/x86_64-linux-gnu:'+os.environ.get('LD_LIBRARY_PATH','')}
def sha(p):
 """Verify the exact previously qualified object or new archive."""
 return hashlib.sha256(p.read_bytes()).hexdigest()
def output_size():
 """Bound all new links and reports together."""
 return sum(p.stat().st_size for p in out.rglob('*') if p.is_file())
assert shutil.disk_usage(root).free>=350*1024*1024
record={'scope':'FreshRoot archive linked to unchanged previouslyqualified C/publicCinch objects, normal andpoison; no recompiled consumer source or new timing.','archive_sha256':sha(archive),'clang_sha256':sha(clang),'commands':[],'cases':[]}
cases=[('c-abi',prefix/'c-abi-v1/run-1/receipt.json','consumer_object_sha256')]+[(name,prefix/'cinch-consumers-v1'/name/'run-1/receipt.json','common_object_sha256') for name in ['random-lengths','random-bounds','random-errors','random-hex','uuid-control','uuid-io']]
for name,path,key in cases:
 data=json.loads(path.read_text());link=next(x['command'] for x in data['commands'] if any('libcinchruntime.a' in arg for arg in x['command']));obj=Path(link[4]);assert obj.suffix=='.o' and sha(obj)==data[key],(name,obj)
 target=out/name;cmd=['taskset','-c','3',str(clang),str(obj),str(archive),'-ldl','-lpthread','-lm','-o',str(target)];start=time.monotonic();r=subprocess.run(cmd,env=env,text=True,capture_output=True,timeout=30);record['commands'].append({'command':cmd,'exit_code':r.returncode,'stdout':r.stdout,'stderr':r.stderr,'wall_seconds_bookkeeping':time.monotonic()-start});assert r.returncode==0,r.stderr
 row={'case':name,'object_path':str(obj),'object_sha256':sha(obj),'source_receipt_sha256':sha(path),'executable':str(target),'executable_sha256':sha(target),'profiles':[]}
 for label,poison in [('normal','0'),('poison','1')]:
  assert shutil.disk_usage(root).free>=250*1024*1024 and output_size()<=30*1024*1024
  e={**env,'CINCH_HEAP_LIMIT':'256M' if poison=='1' else '32M','CINCH_STACK_SIZE':'8M','CINCH_LEAK_CHECK':'1','CINCH_RC_DEBUG':poison,'CINCH_ALLOC_STATS':'1'};r=subprocess.run(['taskset','-c','3',str(target)],env=e,text=True,capture_output=True,timeout=30);row['profiles'].append({'name':label,'exit_code':r.returncode,'stdout':r.stdout,'stderr':r.stderr});assert r.returncode==0 and 'leaked=0' in r.stderr,(name,label,r.stdout,r.stderr)
 record['cases'].append(row);(out/'receipt.json').write_text(json.dumps(record,indent=2)+'\n')
record.update(status='passed',processes=14,output_bytes=output_size(),end_free_bytes=shutil.disk_usage(root).free,archive_sha256_after=sha(archive));assert record['archive_sha256_after']==record['archive_sha256'];(out/'receipt.json').write_text(json.dumps(record,indent=2)+'\n');print(json.dumps({k:v for k,v in record.items() if k not in ['commands','cases']}))
