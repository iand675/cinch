from pathlib import Path
import os,json,hashlib,subprocess,resource,re
ROOT=Path('/workspace/dasl-stdlib-json');MIB=1024*1024
h=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
def free():s=os.statvfs(ROOT);return s.f_bavail*s.f_frsize
assert free()>=300*MIB
compiler=Path('/workspace/dasl-stdlib-io/build/boxed-nat-memory-safe-frozen/cinch');assert h(compiler)=='855df83617f2ea84e7f0e3cf90b9755685bc23226594d81cd43ccd6168071b4e'
expected={'Value':'21a53224006fad8dea23f64b7fd1ffee7961eaf5ccf7f4dc6c7b9e1e5a3a9eb0','Codec':'75c3031b7c0b3f359c8d00ec01afdc97276f949cb7268ca65ab46271209f4022','Dynamic':'19b7ea13c5bc726c4d3ece8b12913a427d0c9ffcdd17d6fa971b9f94149b6591'}
folder=ROOT/'build/json-error-syntax-feature-docs-v1';receipt=ROOT/'bootstrap/stdlib-json-syntax-feature-docs-v1.json';assert not folder.exists() and not receipt.exists();folder.mkdir();os.sched_setaffinity(0,{2})
r={'status':'extracting','scope':'Three bounded parse-only canonical doc extractions; no typechecking/emission/runtime/SDK execution or compiler-family claim','compiler_sha256':h(compiler),'start_guard_bytes':300*MIB,'reserve_bytes':250*MIB,'output_limit_bytes':5*MIB,'budget_exception':'Root explicit tiny-doc-only300MiB initial/250MiB reserve,5MiB total; normal source/runtime350 unchanged','source_hashes_before':expected,'modules':{}}
def save():receipt.write_text(json.dumps(r,indent=2)+'\n')
def walk(rows):
 for row in rows:yield row;yield from walk(row.get('children',[]))
save()
try:
 for name,sha in expected.items():
  p=ROOT/f'compiler/prelude/Text/Json/{name}.cinch';assert h(p)==sha
  out=folder/(name+'.json');argv=[str(compiler),'doc','extract',str(p),'--out',str(out)]
  room=5*MIB-sum(x.stat().st_size for x in folder.rglob('*') if x.is_file())-16384
  def limit():resource.setrlimit(resource.RLIMIT_FSIZE,(room,room))
  v=subprocess.run(argv,capture_output=True,text=True,timeout=15,preexec_fn=limit);(folder/(name+'.log')).write_text(v.stdout+v.stderr);assert v.returncode==0,v.stdout+v.stderr
  data=json.loads(out.read_text());assert data['sourceSha256']==sha and not data['diagnostics'] and not data['orphans']
  rows=list(walk(data['declarations']))
  for row in rows:
   assert row.get('doc') and not row['doc']['legacy'],row['id']
   assert not re.search(r'\b(?:Go|Golang|Rust|Python)\b|C\+\+',row['doc']['markdown']),row['id']
  r['modules'][name]={'source_sha256':sha,'canonical_declarations':len(rows),'canonical_source_ids':[row['id'] for row in rows],'diagnostics':data['diagnostics'],'orphans':data['orphans'],'docs_sha256':h(out),'command':argv};save()
  assert free()>=250*MIB and sum(x.stat().st_size for x in folder.rglob('*') if x.is_file())<=5*MIB
 r['source_hashes_after']={name:h(ROOT/f'compiler/prelude/Text/Json/{name}.cinch') for name in expected};assert r['source_hashes_after']==expected
 r['identity_context']={'runtime_sha256':h(compiler.parent/'libcinchruntime.a'),'runtime_executed':False};assert r['identity_context']['runtime_sha256']=='c4b1097b4b8e26541400228bdbbdcba1376101014823aeb2f5af619fbede57ac'
 r['status']='passed';save();print('Canonical changed-module docs PASS',sum(x['canonical_declarations'] for x in r['modules'].values()),flush=True)
except BaseException as error:r['status']='failed-first-observation';r['failure']=repr(error);save();raise
