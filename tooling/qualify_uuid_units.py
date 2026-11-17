#!/usr/bin/env python3
"""Resume bounded UUID reference/runtime units and archive every closed success before progressing."""
from __future__ import annotations
import argparse
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
from archive_uuid_unit import save
from validate_uuid import ROOT, digest
COMPILER=Path('/workspace/dasl-stdlib-io/build/stdlib-nominal-region-slots-frozen/cinch')
RUNTIME=COMPILER.parent/'libcinchruntime.a'
CLANG=Path('/workspace/.tools/llvm/usr/lib/llvm-20/bin/clang')


def main() -> None:
    """Resume exact successful reports, preserve first failures, and stop at resource/CPU handoff boundaries."""
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--cpu',type=int,required=True)
    parser.add_argument('--modes',nargs='+',choices=('parse','append','compare','generated','v7','control','io'),default=['parse'])
    parser.add_argument('--maximum-units',type=int,default=1)
    parser.add_argument('--retry-failed',action='store_true',help='Use only after the preserved failure has a grounded diagnosis')
    args=parser.parse_args();os.sched_setaffinity(0,{args.cpu});assert args.maximum_units>0
    assert digest(COMPILER)=='330adb4d77f8bcbfc26f246269280f391c3a10b6d3a40771b5f6243f6a8969b2'
    assert digest(RUNTIME)=='d91ce012395a99e1148232d20ec316979cd7d0cb13647a3283ba4820192c0abf'
    fixture=ROOT/'compiler/tests/fixtures/uuid-vectors.json';assert digest(fixture)=='af3900f51bd4ed04fc660d988bfa4a36391cac5c58eb9cc3cde2c638df90a44e'
    vectors=json.loads(fixture.read_text());path=ROOT/'build/uuid-qualification-progress.json'
    ledger=json.loads(path.read_text()) if path.exists() else {'kind':'partial-UUID-exact-tuple-qualification','compiler_sha256':digest(COMPILER),'runtime_sha256':digest(RUNTIME),'fixture_sha256':digest(fixture),'candidate_selected':False,'package_complete':False,'units':{}}
    sources={name:digest(ROOT/'compiler/prelude'/(name.replace('.','/')+'.cinch')) for name in ('Data.Uuid','Effect.Uuid')}
    if 'source_sha256' in ledger:assert ledger['source_sha256']==sources,'A source change needs its own qualification epoch'
    ledger['source_sha256']=sources
    first=ROOT/'build/uuid-parse-0-a1-results.json'
    if first.exists() and 'parse-0000' not in ledger['units']:
        r=json.loads(first.read_text());assert r['status']=='passed' and r['closed_archive']['originals_removed']
        ledger['units']['parse-0000']={'status':'passed','report':str(first.relative_to(ROOT)),'report_sha256':digest(first),'observations':128}
    completed=0
    for mode in args.modes:
        count=len(vectors[mode]) if mode in vectors else 1
        starts=range(0,count,128) if mode not in ('v7','control','io') else (0,)
        for start in starts:
            key=f'{mode}-{start:04d}'
            previous=ledger['units'].get(key)
            if previous and previous['status']=='passed':
                assert digest(ROOT/previous['report'])==previous['report_sha256']
                continue
            if previous and previous['status']=='failed' and not args.retry_failed:raise RuntimeError(f'Preserved failure at{key}; diagnose before retry')
            free=shutil.disk_usage(ROOT).free
            if free<350*1024*1024:
                ledger['pause']={'reason':'Fresh350MiB guard','next_unit':key,'free_bytes':free};save(path,ledger);print(f'Resource pause before{key}; no compiler invoked');return
            attempt=1
            while (ROOT/f'build/uuid-{key}-a{attempt}-results.json').exists():attempt+=1
            artifacts=ROOT/f'build/uuid-{key}-a{attempt}';report=artifacts.with_name(artifacts.name+'-results.json')
            command=[sys.executable,str(ROOT/'tooling/validate_uuid.py'),'--compiler',str(COMPILER),'--runtime',str(RUNTIME),'--clang',str(CLANG),'--cpu',str(args.cpu),'--mode',mode,'--start',str(start),'--count','128','--artifacts',str(artifacts),'--output',str(report)]
            result=subprocess.run(command,cwd=ROOT,timeout=180)
            unit={'status':'failed','report':str(report.relative_to(ROOT)),'exit':result.returncode};ledger['units'][key]=unit
            if result.returncode:
                if report.exists():unit['report_sha256']=digest(report)
                save(path,ledger);raise RuntimeError(f'Stopped at first genuine failure or resource refusal:{key}')
            value=json.loads(report.read_text());assert value['status']=='passed' and len(value['runs'])==2
            subprocess.run([sys.executable,str(ROOT/'tooling/archive_uuid_unit.py'),'--report',str(report),'--cpu',str(args.cpu)],cwd=ROOT,check=True,timeout=30)
            unit.update(status='passed',report_sha256=digest(report),observations=min(128,count-start) if mode not in ('control','io') else None)
            ledger.pop('pause',None);save(path,ledger);completed+=1
            if completed>=args.maximum_units:print(f'Natural handoff after{completed} bounded UUID units');return

if __name__=='__main__':main()
