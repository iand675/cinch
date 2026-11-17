#!/usr/bin/env python3
"""Check one explicit suffix-index source epoch and its public documentation without linking or running it."""
from __future__ import annotations
import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import time

HERE=Path(__file__).resolve().parent
ROOT=HERE.parents[1]
sys.path.insert(0,str(ROOT/'tooling'))
from bench_stdlib import source_closure


def digest(path: Path) -> str:
    """Record exact complete source/compiler identities before any frontend invocation."""
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main() -> None:
    """Preserve the first diagnostic with its flat source closure before trying any later phase."""
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--compiler',type=Path,required=True)
    parser.add_argument('--artifacts',type=Path,required=True)
    args=parser.parse_args();work=args.artifacts.resolve();report=work.with_suffix('.json')
    assert not work.exists() and not report.exists()
    before=shutil.disk_usage(ROOT).free;assert before>=350*1024*1024
    assert digest(args.compiler)=='c2ae24e64f7999060dce8610618f316710f6300ca29e89b9335056936344a86e'
    work.mkdir();entry=work/'Main.cinch';entry.write_bytes((HERE/'consumer-source.cinch').read_bytes())
    dependencies=source_closure(entry.read_text(),{'Data.Index.SuffixArray':HERE/'src/Data.Index.SuffixArray.cinch'})
    for name,path in dependencies.items():shutil.copyfile(path,work/(name+'.cinch'))
    r=dict(scope='FrozenC2 public source-first consumer and documentation only; no linked runtime',status='checking',compiler_sha256=digest(args.compiler),entry_sha256=digest(entry),dependencies={n:digest(p) for n,p in dependencies.items()},before_free_bytes=before,steps=[])
    start=time.monotonic()
    commands=[[str(args.compiler),'check',str(entry)],[str(args.compiler),'doc','extract',str(work/'Data.Index.SuffixArray.cinch'),'--iface',str(work/'build/Data.Index.SuffixArray.cinchif'),'--out',str(work/'documentation.json')]]
    for number,command in enumerate(commands):
        result=subprocess.run(command,capture_output=True,text=True)
        (work/f'step-{number}.log').write_text(result.stdout+result.stderr)
        r['steps'].append(dict(command=command,exit_code=result.returncode,diagnostics=result.stdout+result.stderr))
        if result.returncode:
            r['status']='source-failed' if number==0 else 'documentation-failed';break
    else:r['status']='passed'
    r.update(seconds=time.monotonic()-start,after_free_bytes=shutil.disk_usage(ROOT).free,own_output_bytes=sum(p.stat().st_size for p in work.rglob('*') if p.is_file()))
    report.write_text(json.dumps(r,indent=2)+'\n');print(json.dumps(r))
    assert r['own_output_bytes']<=30*1024*1024 and r['after_free_bytes']>=250*1024*1024
    if r['status']!='passed':sys.exit(1)


if __name__=='__main__':main()
