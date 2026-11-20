"""Durable success-only functional archival and opt-in sealed-input consolidation."""
from pathlib import Path
from decimal import Decimal
import gzip, hashlib, json, os, shutil, stat, tarfile
MIB=1024*1024

def digest(p):
 """Hash every byte without interpreting input contents."""
 h=hashlib.sha256()
 with Path(p).open('rb') as f:
  while b:=f.read(MIB):h.update(b)
 return h.hexdigest()

def fsync_dir(p):
 """Durably record directory changes before the next recovery step."""
 fd=os.open(p,os.O_RDONLY|os.O_DIRECTORY)
 try:os.fsync(fd)
 finally:os.close(fd)

def durable_json(p,value):
 """Create and fsync an immutable evidence record and its parent."""
 p=Path(p);p.parent.mkdir(parents=True,exist_ok=True)
 with p.open('x') as f:json.dump(value,f,indent=2);f.write('\n');f.flush();os.fsync(f.fileno())
 fsync_dir(p.parent)

def identity(p):
 """Measure byte and recoverable storage identity of an ordinary file."""
 p=Path(p);s=p.lstat();assert stat.S_ISREG(s.st_mode) and not p.is_symlink(),p
 return {'path':str(p.resolve()),'sha256':digest(p),'bytes':s.st_size,'mode':stat.S_IMODE(s.st_mode),'uid':s.st_uid,'gid':s.st_gid,'mtime_ns':s.st_mtime_ns}

def matches(p,w,mtime=True):
 """Compare full byte, owner and mode identity with optional exact mtime."""
 got=identity(p)
 return all(got[k]==w[k] for k in ['sha256','bytes','mode','uid','gid']+(['mtime_ns'] if mtime else []))

def archive_project(project,marker_path,destination,test_receipt):
 """Called only after the complete --exact test returned success and every marker passed."""
 project=Path(project);destination=Path(destination);destination.mkdir(parents=True,exist_ok=True)
 assert shutil.disk_usage(project).free>=250*MIB
 paths=[project/'consumer',project/'consumer.ll'];members=[identity(p) for p in paths]
 index=destination/(project.name+'.index.json');archive=destination/(project.name+'.tar.gz')
 assert not archive.exists() and not index.exists()
 class Limited:
  """Bound compressed scratch while forwarding the underlying stream."""
  def __init__(self,f):
   """Retain the open bounded archive destination."""
   self.f=f
  def write(self,b):
   """Refuse compressed scratch growth beyond ten MiB before writing."""
   assert self.f.tell()+len(b)<=10*MIB;return self.f.write(b)
  def __getattr__(self,n):
   """Forward stream operations while retaining the bounded write method."""
   return getattr(self.f,n)
 with archive.open('xb') as raw:
  with gzip.GzipFile(fileobj=Limited(raw),mode='wb',filename='',mtime=0,compresslevel=1) as z:
   with tarfile.open(fileobj=z,mode='w',format=tarfile.PAX_FORMAT) as t:
    for p,row in zip(paths,members):
     info=t.gettarinfo(str(p),arcname=p.name)
     info.pax_headers['mtime']=f"{row['mtime_ns']//1_000_000_000}.{row['mtime_ns']%1_000_000_000:09d}"
     with p.open('rb') as f:t.addfile(info,f)
  raw.flush();os.fsync(raw.fileno())
 fsync_dir(destination)
 h=hashlib.sha256()
 with gzip.open(archive,'rb') as f:
  while b:=f.read(MIB):h.update(b)
 with tarfile.open(archive,'r:gz') as t:
  got=t.getmembers();assert len(got)==2
  for item,want,p in zip(got,members,paths):
   assert item.isfile() and item.name==p.name and item.size==want['bytes']
   assert stat.S_IMODE(item.mode)==want['mode'] and item.uid==want['uid'] and item.gid==want['gid']
   assert int(Decimal(item.pax_headers['mtime'])*1_000_000_000)==want['mtime_ns']
   assert hashlib.sha256(t.extractfile(item).read()).hexdigest()==want['sha256']
 assert shutil.disk_usage(project).free>=250*MIB
 record={'status':'verified-closed-test-and-both-profile-success','archive':identity(archive),'decompressed_tar_sha256':h.hexdigest(),'members':members,'marker':str(marker_path),'marker_sha256':digest(marker_path),'successful_test_receipt':str(test_receipt),'successful_test_receipt_sha256':digest(test_receipt),'verified_before_unlink':True,'ordering':'Archive and directory fsync; gzip/decompressed/member SHA/size/mode/uid/gid/nsmtime verified; durable index plus parent fsync BEFORE unlink; original identity recheck; project parent fsync after unlink.','restore':'Extract each member only to a missing original path; verify recorded SHA/size/mode/uid/gid and restore exact recorded mtime_ns with os.utime(ns=...). Never overwrite an existing path.'}
 durable_json(index,record)
 for p,row in zip(paths,members):assert matches(p,row)
 for p in paths:p.unlink()
 fsync_dir(project)
 return {'index':str(index),'index_sha256':digest(index),'net_recovered_bytes':sum(x['bytes'] for x in members)-archive.stat().st_size}

def eligible_inputs(project,marker):
 # Only project-owned source and interfaces for those SAME on-disk modules.
 # Embedded-only provider interfaces, ABI/runtime/compiler/benchmark/frozen inputs are excluded.
 """Yield only passed project-owned module sources and their interfaces."""
 sources={Path(x['path']).stem for x in marker['files'] if Path(x['path']).suffix=='.cinch'}
 for row in marker['files']:
  rel=Path(row['path']);name=rel.name.lower()
  if rel.is_absolute() or '..' in rel.parts:raise AssertionError(rel)
  if any(x in name for x in ['abi','runtime','provider','compiler','benchmark','frozen']):continue
  if rel.suffix not in ['.cinch','.cinchif','.cif']:continue
  if rel.suffix!='.cinch' and rel.stem not in sources:continue
  p=Path(project)/rel
  if p.is_file() and not p.is_symlink():yield p

def dedup_passed(project,marker,canonicals,ledger_dir,apply=False):
 """Default dry plan; explicit reviewed flag authorizes exact passed-input replacements."""
 ledger_dir=Path(ledger_dir);ledger_dir.mkdir(parents=True,exist_ok=True)
 rows=[];pending=[]
 for p in eligible_inputs(project,marker):
  own=identity(p);key=tuple(own[k] for k in ['sha256','bytes','mode','uid','gid'])
  candidate=canonicals.get(key)
  if candidate:
   old=identity(candidate);assert all(own[k]==old[k] for k in ['sha256','bytes','mode','uid','gid'])
   # Full byte comparison supplements independent full hashes; no semantic interface comparison.
   with p.open('rb') as a,Path(candidate).open('rb') as b:
    while block:=a.read(MIB):assert block==b.read(len(block))
    assert b.read(1)==b''
   if p.stat().st_ino!=Path(candidate).stat().st_ino:
    rows.append({'original':own,'canonical':old,'shared_visible_mtime_ns_after_apply':old['mtime_ns'],'restore_original_mtime_ns':own['mtime_ns']})
  else:pending.append((key,p))
 plan={'status':'reviewed-policy-exact-plan' if apply else 'dry-plan-only-no-links','project':str(project),'rows':rows,'all_sources_and_interfaces_remain_plain':True,'sealed_success_only':True,'never_compile_any_listed_project_again':True,'restore':'Copy canonical bytes to a new separate temporary inode; verify full SHA/size, restore original mode/uid/gid and exact mtime_ns from original row, fsync before replace, then parent fsync. Original uid/gid are measured, never inferred.'}
 path=ledger_dir/(Path(project).name+'.plan.json')
 encoded=(json.dumps(plan,indent=2)+'\n').encode()
 assert sum(p.stat().st_size for p in ledger_dir.glob('*') if p.is_file())+len(encoded)+4096<=5*MIB,'dedup metadata cap'
 durable_json(path,plan)
 if apply:
  for n,row in enumerate(rows):
   p=Path(row['original']['path']);canonical=Path(row['canonical']['path'])
   assert matches(p,row['original']) and matches(canonical,row['canonical'],mtime=False)
   tmp=p.with_name(p.name+f'.passed-link-{n}');assert not tmp.exists()
   os.link(canonical,tmp);os.replace(tmp,p);fsync_dir(p.parent)
   assert matches(p,row['original'],mtime=False) and p.stat().st_ino==canonical.stat().st_ino
  durable_json(ledger_dir/(Path(project).name+'.applied.json'),{'status':'applied-and-reverified','plan':str(path),'plan_sha256':digest(path),'replacements':len(rows),'original_metadata_preserved_in_plan':True})
 for key,p in pending:canonicals[key]=p
 return {'plan':str(path),'plan_sha256':digest(path),'replacements':len(rows),'applied':apply}
