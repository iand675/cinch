"""Shared strict resource and exact-source guards for the authorized partial JSON pipeline."""
from pathlib import Path
import hashlib,os
ROOT=Path('/workspace/dasl-stdlib-json')
MIB=1024*1024
START=350*MIB
PAUSE=250*MIB
UNARCHIVED=30*MIB
FROZEN=ROOT/'build/json-error-pointer-probe-v5'
def digest(path):return hashlib.sha256(path.read_bytes()).hexdigest()
def free():
 s=os.statvfs(ROOT);return s.f_bavail*s.f_frsize
def scope_files():
 roots=[*sorted((ROOT/'build').glob('json-error-*')),ROOT/'build/json-next-error-context',*sorted((ROOT/'bootstrap/repros').glob('json-error-context-*'))]
 files=[]
 for p in roots:
  files.extend(p.rglob('*') if p.is_dir() else [p])
 files += list((ROOT/'bootstrap').glob('stdlib-json-error-*'))
 files += list((ROOT/'compiler/tests/fixtures').glob('json-codec-error-*'))
 files += [ROOT/'tooling/generate_json_error_context_cases.py',ROOT/'tooling/go_json_error_context_vectors.go']
 return [p for p in files if p.is_file() and not p.name.endswith('.tar.gz')]
def unarchived_bytes():
 unique={}
 for p in scope_files():
  s=p.stat();unique[(s.st_dev,s.st_ino)]=s.st_size
 return sum(unique.values())
def phase_guard():
 assert free()>=START,f'phase start guard refused: {free()} < {START}'
 assert unarchived_bytes()<=UNARCHIVED,f'unarchived scope guard refused: {unarchived_bytes()}'
def pause_guard():
 assert free()>=PAUSE,f'phase pause guard: {free()} < {PAUSE}'
 assert unarchived_bytes()<=UNARCHIVED,f'unarchived scope guard: {unarchived_bytes()}'
def fsync_dir(path):
 fd=os.open(path,os.O_RDONLY|os.O_DIRECTORY)
 try:os.fsync(fd)
 finally:os.close(fd)
