#!/usr/bin/env python3
"""Recover the exact Root-authorized six closed published765 functional pairs, keeping every input plain."""
from pathlib import Path
import gzip,hashlib,json,os,stat,tarfile,time
from pipeline_common import ROOT,digest,free,fsync_dir,unarchived_bytes
assert free()>=50*1024*1024,f'recovery start refused: {free()}'
sets=[];files=[]
for stage in ['method-protocol','method-owner','addressability-trace']:
 for frontend in ['rust','native']:
  receipt=ROOT/f'bootstrap/stdlib-json-{stage}-{frontend}-v1-results.json';r=json.loads(receipt.read_text())
  assert r['status']=='passed' and len(r['runs'])==2
  for run in r['runs']:assert run['stdout']=='' and run['stats']['live']==0 and run['leaked']==0
  command=r['raw_link_command'];ir=Path(command[2]);exe=Path(command[command.index('-o')+1])
  assert ir.is_relative_to(ROOT/'build') and exe.is_relative_to(ROOT/'build')
  assert ir.name=='consumer.ll' and exe.name=='consumer' and ir.parent==exe.parent
  assert digest(ir)==r['llvm_sha256']
  if 'executable_sha256' in r:assert digest(exe)==r['executable_sha256']
  manifest={'stage':stage,'frontend':frontend,'receipt':str(receipt.relative_to(ROOT)),'receipt_sha256':digest(receipt),'llvm_sha256_from_passed_receipt':r['llvm_sha256'],'executable_link_path_from_passed_receipt':str(exe.relative_to(ROOT)),'executable_prior_hash_available':'executable_sha256' in r,'compiler_sha256':r['compiler_sha256'],'runtime_sha256':r['observed_runtime_archive_sha256']}
  sets.append(manifest);files.extend([ir,exe])
assert len(set(files))==12
members=[{'path':str(p.relative_to(ROOT)),'bytes':p.stat().st_size,'mode':stat.S_IMODE(p.stat().st_mode),'sha256':digest(p)} for p in files]
assert sum(m['bytes'] for m in members)==96419608
folder=ROOT/'build/json-error-functional-archives';folder.mkdir(exist_ok=True)
archive=folder/'closed765-six-functional-pairs-v1.tar.gz'
index=ROOT/'bootstrap/stdlib-json-error-closed765-functional-recovery-v1.json'
assert not archive.exists() and not index.exists(),'refuse repeated recovery'
class Limited:
 def __init__(self,target):self.target=target
 def write(self,data):
  assert self.target.tell()+len(data)<=30*1024*1024,'temporary compressed output cap30MiB'
  return self.target.write(data)
 def __getattr__(self,key):return getattr(self.target,key)
started=time.monotonic();before=free()
with archive.open('xb') as target:
 with gzip.GzipFile(fileobj=Limited(target),mode='wb',filename='',mtime=0,compresslevel=1) as stream:
  with tarfile.open(fileobj=stream,mode='w') as tar:
   for p in files:tar.add(p,arcname=str(p.relative_to(ROOT)),recursive=False)
 target.flush();os.fsync(target.fileno())
fsync_dir(folder)
raw_hash=hashlib.sha256()
with gzip.open(archive,'rb') as stream:
 while chunk:=stream.read(1024*1024):raw_hash.update(chunk)
with tarfile.open(archive,'r:gz') as tar:
 observed=tar.getmembers();assert [m.name for m in observed]==[m['path'] for m in members]
 for actual,expected in zip(observed,members):
  assert actual.isfile() and actual.size==expected['bytes'] and stat.S_IMODE(actual.mode)==expected['mode']
  extracted=tar.extractfile(actual);assert extracted is not None
  h=hashlib.sha256()
  while chunk:=extracted.read(1024*1024):h.update(chunk)
  assert h.hexdigest()==expected['sha256']
report={'description':'Root-authorized exact six closed765 functional ELF/IR pairs only; no source/log/receipt/frozen/compiler/provider/runtime/benchmark input archived','original_payload_bytes':96419608,'archive_path':str(archive.relative_to(ROOT)),'archive_bytes':archive.stat().st_size,'archive_sha256':digest(archive),'decompressed_tar_sha256':raw_hash.hexdigest(),'members':members,'passed_execution_linkage':sets,'executable_identity_note':'Historical receipts record successful execution and exact executable link paths but no executable hashes; original ELF hashes are captured now, not retroactively claimed as execution-time hashes. LLVM hashes are checked against their original passed receipts.','recovery_command':['tar','-xzf',str(archive),'-C',str(ROOT)],'verified_before_unlink':True,'durability':'archive+archive-directory fsync; compressed/decompressed/member hashes+size+mode verification; index+index-parent fsync BEFORE any unlink; original file identities and passed-receipt hashes rechecked; each output parent fsynced AFTER unlink','free_bytes_before':before,'start_guard_bytes':50*1024*1024,'temporary_compressed_limit_bytes':30*1024*1024}
with index.open('x') as target:target.write(json.dumps(report,indent=2)+'\n');target.flush();os.fsync(target.fileno())
fsync_dir(index.parent)
for p,expected in zip(files,members):assert p.stat().st_size==expected['bytes'] and stat.S_IMODE(p.stat().st_mode)==expected['mode'] and digest(p)==expected['sha256']
for link in sets:assert digest(ROOT/link['receipt'])==link['receipt_sha256']
for p in files:p.unlink()
for parent in {p.parent for p in files}:fsync_dir(parent)
completion={**report,'all_unlinks_complete':True,'net_payload_bytes_recovered':96419608-archive.stat().st_size,'free_bytes_after':free(),'unarchived_bytes_after':unarchived_bytes(),'elapsed_seconds':time.monotonic()-started}
post=ROOT/'build/json-next-error-context/closed765-recovery-complete.json'
with post.open('x') as target:target.write(json.dumps(completion,indent=2)+'\n');target.flush();os.fsync(target.fileno())
fsync_dir(post.parent)
print(json.dumps({k:completion[k] for k in ['original_payload_bytes','archive_bytes','net_payload_bytes_recovered','free_bytes_after','unarchived_bytes_after','elapsed_seconds']}),flush=True)
