#!/usr/bin/env python3
"""One authorized source-first current-source quoted consumer, archived before returning its boundary."""
from pathlib import Path
import argparse,json,os,subprocess,sys
from pipeline_common import ROOT,digest,free,scope_files,unarchived_bytes,START,PAUSE,UNARCHIVED
p=argparse.ArgumentParser();p.add_argument('suite',choices=['pointer','owner','bool','string','integer']);args=p.parse_args()
assert free()>=START,f'start guard refused: {free()}'
baseline=ROOT/f'compiler/stdlib-json-error-{args.suite}-current-v2-baseline.json';assert not baseline.exists()
ledger={'authorization':'Root requests current-source quoted1152+retained1+owner4 affected refresh after shared descriptor/error changes. Each independent source/runtime/archive unit standard350/250 fresh30MiB/archive scratch10MiB. Historical passed/failure epochs untouched; no oracle regeneration or normalization.','prior_unique_bytes':unarchived_bytes(),'existing_files':{},'immutable_failure_files':json.loads((ROOT/'compiler/evidence/stdlib/stdlib-json-method-v5-baseline.json').read_text())['immutable_failure_files'],'start_guard_bytes':START,'pause_guard_bytes':PAUSE,'new_output_cap_bytes':UNARCHIVED}
for f in scope_files():
 s=f.stat();ledger['existing_files'][str(f.relative_to(ROOT))]={'bytes':s.st_size,'sha256':digest(f),'mode':s.st_mode&0o777}
baseline.write_text(json.dumps(ledger,indent=2)+'\n')
env={**os.environ,'JSON_CURRENT_QUOTED_SUITE':args.suite}
for script in ['probe_current_quoted.py','qualify_current_quoted.py','archive_current_quoted.py']:
 command=[sys.executable,str(ROOT/'build/json-next-error-context'/script),args.suite]
 if script!='archive_current_quoted.py':command+=['--attempt','2']
 subprocess.run(command,cwd=ROOT,env=env,check=True)
print(f'RELEASE natural {args.suite} unit: free={free()}B',flush=True)
