from pathlib import Path
import sys,json,hashlib,tarfile,gzip,stat,os
ROOT=Path('/workspace/dasl-stdlib-json');sys.path.insert(0,str(ROOT/'build/json-next-error-context'));from pipeline_common import digest,free,fsync_dir,MIB
unit=sys.argv[1];assert unit in ['lexer'];receipt=ROOT/f'bootstrap/stdlib-json-syntax-{unit}-rust-v2-results.json';result=json.loads(receipt.read_text());assert result['status']=='passed' and len(result['runs'])==2
for row in result['runs']:assert row['exit_code']==0 and 'live=0' in row['stderr'] and 'leaked=0' in row['stderr']
folder=ROOT/f'build/json-error-syntax-{unit}-v2/runtime';files=[folder/'consumer.ll',folder/'consumer'];assert digest(files[0])==result['llvm_sha256'] and digest(files[1])==result['executable_sha256'];assert free()>=250*MIB
archive=ROOT/f'build/json-error-functional-archives/syntax-{unit}-v2.tar.gz';index=ROOT/f'bootstrap/stdlib-json-syntax-{unit}-v2-functional-archive.json';assert not archive.exists() and not index.exists()
members=[{'path':str(p.relative_to(ROOT)),'sha256':digest(p),'bytes':p.stat().st_size,'mode':stat.S_IMODE(p.stat().st_mode)} for p in files]
class Limited:
 def __init__(self,f):self.f=f
 def write(self,b):assert self.f.tell()+len(b)<=10*MIB;return self.f.write(b)
 def __getattr__(self,n):return getattr(self.f,n)
with archive.open('xb') as f:
 with gzip.GzipFile(fileobj=Limited(f),mode='wb',mtime=0,filename='',compresslevel=1) as z:
  with tarfile.open(fileobj=z,mode='w') as t:
   for p in files:t.add(p,arcname=str(p.relative_to(ROOT)),recursive=False)
 f.flush();os.fsync(f.fileno())
fsync_dir(archive.parent)
h=hashlib.sha256()
with gzip.open(archive,'rb') as s:
 while b:=s.read(MIB):h.update(b)
with tarfile.open(archive,'r:gz') as t:
 entries=t.getmembers();assert [p.name for p in entries]==[p['path'] for p in members]
 for a,e in zip(entries,members):assert a.isfile() and a.size==e['bytes'] and stat.S_IMODE(a.mode)==e['mode'] and hashlib.sha256(t.extractfile(a).read()).hexdigest()==e['sha256']
r={'status':'verified-closed-success','archive_path':str(archive.relative_to(ROOT)),'archive_sha256':digest(archive),'archive_bytes':archive.stat().st_size,'decompressed_tar_sha256':h.hexdigest(),'members':members,'execution_receipt':str(receipt.relative_to(ROOT)),'execution_receipt_sha256':digest(receipt),'verified_before_unlink':True,'ordering':'Archive+directory fsync and full CRC/decompressed/member SHA/size/mode verification; recovery index+parent fsync before unlink; original identity recheck; output parent fsync after. All failed artifacts/sources/frozen/benchmark inputs remain plain.','restore_command':['tar','-xzf',str(archive),'-C',str(ROOT)]}
with index.open('x') as f:f.write(json.dumps(r,indent=2)+'\n');f.flush();os.fsync(f.fileno())
fsync_dir(index.parent)
for p,e in zip(files,members):assert digest(p)==e['sha256'] and p.stat().st_size==e['bytes'] and stat.S_IMODE(p.stat().st_mode)==e['mode']
for p in files:p.unlink()
fsync_dir(folder);print('Archived',unit,'net',sum(p['bytes'] for p in members)-archive.stat().st_size,'free',free(),flush=True)
