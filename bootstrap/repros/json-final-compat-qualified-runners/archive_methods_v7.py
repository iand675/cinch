#!/usr/bin/env python3
"""Durably archive only completed successful ELF/IR, retaining all plain evidence and inputs."""
from pathlib import Path
import argparse,gzip,hashlib,json,os,stat,tarfile,time
from pipeline_methods_v7 import ROOT,digest,free,pause_guard,unarchived_bytes
from pipeline_common import fsync_dir
# Continue the single Root-authorized source/runtime/archive unit under its reserve.
pause_guard()
receipt=ROOT/f'bootstrap/stdlib-json-error-methods-rust-v7-results.json';result=json.loads(receipt.read_text())
assert result['status']=='passed' and len(result['runs'])==2
for row in result['runs']:assert row['exit_code']==0 and row['stdout']=='' and row['stats']['live']==0 and row['leaked']==0
folder=ROOT/f'build/json-error-methods-rust-v7'
files=[folder/'consumer.ll',folder/'consumer']
assert digest(files[0])==result['llvm_sha256'] and digest(files[1])==result['executable_sha256']
archives=ROOT/'build/json-error-functional-archives';archives.mkdir(exist_ok=True)
archive=archives/f'methods-rust-v7.tar.gz';index=ROOT/f'bootstrap/stdlib-json-error-methods-rust-v7-functional-archive.json'
assert not archive.exists() and not index.exists(),'refuse repeated archival'
members=[{'path':str(path.relative_to(ROOT)),'bytes':path.stat().st_size,'mode':stat.S_IMODE(path.stat().st_mode),'sha256':digest(path)} for path in files]
start=time.monotonic();before=free()
class LimitedWriter:
 def __init__(self,target):self.target=target
 def write(self,data):
  assert self.target.tell()+len(data)<=10*1024*1024,'Root archive scratch cap'
  return self.target.write(data)
 def __getattr__(self,name):return getattr(self.target,name)
with archive.open('xb') as target:
 with gzip.GzipFile(fileobj=LimitedWriter(target),mode='wb',filename='',mtime=0,compresslevel=1) as zipped:
  with tarfile.open(fileobj=zipped,mode='w') as tar:
   for path in files:tar.add(path,arcname=str(path.relative_to(ROOT)),recursive=False)
 target.flush();os.fsync(target.fileno())
fsync_dir(archives)
pause_guard()
raw_hash=hashlib.sha256()
with gzip.open(archive,'rb') as raw:
 while chunk:=raw.read(1024*1024):raw_hash.update(chunk)
with tarfile.open(archive,'r:gz') as tar:
 observed=tar.getmembers();assert [row.name for row in observed]==[row['path'] for row in members]
 for metadata,expected in zip(observed,members):
  assert metadata.isfile() and metadata.size==expected['bytes'] and stat.S_IMODE(metadata.mode)==expected['mode']
  stream=tar.extractfile(metadata);assert stream is not None
  hashed=hashlib.sha256()
  while chunk:=stream.read(1024*1024):hashed.update(chunk)
  assert hashed.hexdigest()==expected['sha256']
report={'description':'Verified successful closed functional ELF/IR archive only; all sources/frozen/compiler/provider/runtime/benchmark inputs/failure diagnostics remain plain','archive_path':str(archive.relative_to(ROOT)),'archive_sha256':digest(archive),'archive_bytes':archive.stat().st_size,'decompressed_tar_sha256':raw_hash.hexdigest(),'members':members,'execution_receipt':str(receipt.relative_to(ROOT)),'execution_receipt_sha256':digest(receipt),'ordering':'archive fsync+directory fsync; CRC/decompressed tar+all member SHA/size/mode verification; recovery index fsync+parent fsync BEFORE any unlink; original file identity recheck; output-parent fsync AFTER unlink','free_bytes_before':before,'verified_before_unlink':True,'recovery_exception':False,'compressed_output_limit_bytes':10*1024*1024,'recovery_command':['tar','-xzf',str(archive),'-C',str(ROOT)]}
with index.open('x') as target:
 target.write(json.dumps(report,indent=2)+'\n');target.flush();os.fsync(target.fileno())
fsync_dir(index.parent)
for path,expected in zip(files,members):assert path.stat().st_size==expected['bytes'] and stat.S_IMODE(path.stat().st_mode)==expected['mode'] and digest(path)==expected['sha256']
for path in files:path.unlink()
fsync_dir(folder)
# The durable recovery index already exists; a separate post-action receipt cannot invalidate it.
completion={**report,'free_bytes_after':free(),'net_payload_bytes_recovered':sum(row['bytes'] for row in members)-archive.stat().st_size,'elapsed_seconds':time.monotonic()-start,'unarchived_bytes_after':unarchived_bytes(),'all_unlinks_complete':True}
post=folder/'archive-complete.json'
with post.open('x') as target:target.write(json.dumps(completion,indent=2)+'\n');target.flush();os.fsync(target.fileno())
fsync_dir(folder)
print(json.dumps({k:completion[k] for k in ['net_payload_bytes_recovered','archive_bytes','unarchived_bytes_after','free_bytes_after','elapsed_seconds']}),flush=True)
