#!/usr/bin/env python3
"""Verify frozen public UTF8 candidate pipelines without retaining any functional-run timing data."""
from __future__ import annotations
import argparse
import json
import os
from pathlib import Path
import subprocess
from bench_utf8 import digest, MODES


def main() -> None:
    """Check exact artifact identities, boundary sizes and observable checksums against the SDK consumer."""
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--artifacts',type=Path,nargs='+',required=True)
    parser.add_argument('--cpu',type=int,default=2)
    parser.add_argument('--output',type=Path,required=True)
    args=parser.parse_args();os.sched_setaffinity(0,{args.cpu})
    cases=[];identities={}
    env={**os.environ,'CINCH_RC_DEBUG':'0','CINCH_LEAK_CHECK':'1','CINCH_HEAP_LIMIT':'32M','CINCH_STACK_SIZE':'8M','GOMAXPROCS':'1'}
    sizes=[0,1,2,3,4,7,8,9,15,16,17,31,32,33,64,65536]
    for folder in args.artifacts:
        folder=folder.resolve();identity=json.loads((folder/'identity.json').read_text())
        for key,name in [('compiler_sha256','compiler'),('executable_sha256','bench'),('llvm_sha256','bench.ll'),('assembly_sha256','bench.s'),('reference_executable_sha256','reference')]:
            if digest(folder/name)!=identity[key]:raise ValueError('changed retained artifact: '+str(folder/name))
        for name,expected in identity['sources'].items():
            if digest(folder/'project'/name)!=expected:raise ValueError('changed candidate source: '+name)
        identities[str(folder)]=identity
        for mode in MODES:
            for pattern in range(3):
                for size in sizes:
                    observed=[]
                    for name in ['bench','reference']:
                        result=subprocess.run([str(folder/name),mode,str(size),'2',str(pattern)],env=env,capture_output=True,text=True)
                        if result.returncode:raise RuntimeError(f'{folder}/{name} {mode}/{size}/{pattern}: {result.stdout}\n{result.stderr}')
                        fields=result.stdout.split()
                        if len(fields)!=2:raise ValueError('unexpected checksum protocol: '+result.stdout)
                        observed.append(int(fields[1]))
                        if name=='bench' and 'leaked=0' not in result.stderr:raise ValueError('missing zero-leak result: '+result.stderr)
                    if observed[0]!=observed[1]:raise ValueError(f'candidate mismatch {folder} {mode}/{size}/{pattern}: {observed}')
                    cases.append({'artifact':str(folder),'workload':mode,'pattern':pattern,'size':size,'iterations':2,'checksum':observed[0]})
    report={'cpu':args.cpu,'functional_timing_discarded':True,'identities':identities,'normal_heap':'32M','stack':'8M','all_cinch_processes_zero_leaks':True,'cases':cases,'all_passed':True}
    args.output.parent.mkdir(parents=True,exist_ok=True);args.output.write_text(json.dumps(report,indent=2)+'\n')
    print(f'UTF8 candidate preflight passed {len(cases)} independent checksum cases',flush=True)


if __name__=='__main__':main()
