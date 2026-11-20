"""Separate newly created correction-unit output from immutable prior qualification evidence."""
from pathlib import Path
import json,os
from pipeline_common import ROOT,MIB,START,PAUSE,UNARCHIVED,FROZEN,digest,free,scope_files
# Root's scoped single-unit override: only this correction unit, <=30MiB fresh output.
START=350*MIB
BASELINE=ROOT/'compiler/evidence/stdlib/stdlib-json-method-v6-baseline.json'
def baseline():return json.loads(BASELINE.read_text())
def verify_failed_evidence():
 for name,expected in baseline()['immutable_failure_files'].items():
  p=ROOT/name
  assert p.stat().st_size==expected['bytes'] and p.stat().st_mode&0o777==expected['mode'],name
  assert digest(p)==expected['sha256'],name

def unarchived_bytes():
 prior=baseline()['existing_files'];new={}
 for p in scope_files():
  s=p.stat();name=str(p.relative_to(ROOT));old=prior.get(name)
  if old and s.st_size==old['bytes'] and digest(p)==old['sha256']:continue
  new[(s.st_dev,s.st_ino)]=s.st_size
 return sum(new.values())
def phase_guard():
 verify_failed_evidence()
 assert free()>=START,f'phase start guard refused: {free()} < {START}'
 assert unarchived_bytes()<=UNARCHIVED,f'update v1 output guard refused: {unarchived_bytes()}'
def pause_guard():
 verify_failed_evidence()
 assert free()>=PAUSE,f'phase pause guard refused: {free()} < {PAUSE}'
 assert unarchived_bytes()<=UNARCHIVED,f'update v1 output guard refused: {unarchived_bytes()}'
