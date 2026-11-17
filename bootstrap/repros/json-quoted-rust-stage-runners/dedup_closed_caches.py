#!/usr/bin/env python3
"""Share only byte/mode-identical CIF inputs of completed successful, now sealed runtime epochs."""
from pathlib import Path
import hashlib,json,os,stat,time
from pipeline_common import ROOT,digest,pause_guard,fsync_dir

def dedup_closed():
 closed=[]
 for suite in ['pointer','owner','bool','string','integer']:
  receipt=ROOT/f'bootstrap/stdlib-json-error-{suite}-rust-v1-results.json'
  if not receipt.exists():continue
  r=json.loads(receipt.read_text())
  if r.get('status')!='passed' or len(r.get('runs',[]))!=2:continue
  for row in r['runs']:assert row['exit_code']==0 and row['stats']['live']==0 and row['leaked']==0
  closed.append(Path(r['frozen_inputs']))
 files=[p for folder in closed for p in sorted((folder/'build').rglob('*.cinchif')) if p.is_file()]
 unique={};before={};relinks=[]
 for path in files:
  s=path.stat();before[(s.st_dev,s.st_ino)]=s.st_size
  identity=(digest(path),s.st_size,stat.S_IMODE(s.st_mode))
  if identity not in unique:unique[identity]=path;continue
  canonical=unique[identity]
  if path.stat().st_ino==canonical.stat().st_ino:continue
  temporary=path.with_name(path.name+'.sealed-link-tmp');assert not temporary.exists()
  os.link(canonical,temporary)
  assert (digest(temporary),temporary.stat().st_size,stat.S_IMODE(temporary.stat().st_mode))==identity
  os.replace(temporary,path);fsync_dir(path.parent)
  assert (digest(path),path.stat().st_size,stat.S_IMODE(path.stat().st_mode))==identity
  relinks.append({'path':str(path.relative_to(ROOT)),'canonical':str(canonical.relative_to(ROOT)),'sha256':identity[0],'bytes':identity[1],'mode':identity[2]})
 after={}
 for path in files:
  s=path.stat();after[(s.st_dev,s.st_ino)]=s.st_size
 if relinks:
  ledger=ROOT/f'bootstrap/stdlib-json-error-closed-cif-dedup-{len(closed)}-epochs.json';assert not ledger.exists()
  report={'description':'Only completed successful runtime input caches; every path stays plain; byte/size/mode identical files share inodes. These epochs are sealed: never compile into or mutate them again. Replays use saved ELF/IR or fresh copies.','sealed_input_directories':[str(p.relative_to(ROOT)) for p in closed],'before_unique_bytes':sum(before.values()),'after_unique_bytes':sum(after.values()),'relinks':relinks}
  with ledger.open('x') as out:out.write(json.dumps(report,indent=2)+'\n');out.flush();os.fsync(out.fileno())
  fsync_dir(ledger.parent)
 pause_guard()
 return {'closed_runtime_epochs':len(closed),'relinked_cifs':len(relinks),'bytes_recovered':sum(before.values())-sum(after.values())}
if __name__=='__main__':print(json.dumps(dedup_closed()))
