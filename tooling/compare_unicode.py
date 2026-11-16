#!/usr/bin/env python3
"""Measure paired/interleaved Unicode scalar public pipelines from exact frozen artifacts."""
from __future__ import annotations
import argparse
import json
import os
from pathlib import Path
import statistics
import subprocess
import time
from bench_unicode import MODES
from validate_unicode import digest


def main() -> None:
    """Rotate candidates and reference order for each sample while retaining controls and every raw value."""
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--artifacts',type=Path,nargs='+',required=True)
    parser.add_argument('--cpu',type=int,required=True)
    parser.add_argument('--output',type=Path,required=True)
    parser.add_argument('--iterations',type=int,default=32768)
    parser.add_argument('--samples',type=int,default=5)
    parser.add_argument('--workloads',nargs='+',choices=MODES,default=MODES)
    parser.add_argument('--patterns',nargs='+',type=int,default=[0,1,2,3])
    args=parser.parse_args();os.sched_setaffinity(0,{args.cpu})
    artifacts=[p.resolve() for p in args.artifacts];identities={}
    for folder in artifacts:
        identity=json.loads((folder/'identity.json').read_text())
        for key,name in [('compiler_sha256','compiler'),('executable_sha256','bench'),('llvm_sha256','bench.ll'),('assembly_sha256','bench.s'),('reference_executable_sha256','reference'),('input_sha256','inputs.json')]:
            if digest(folder/name)!=identity[key]:raise ValueError('retained artifact changed: '+str(folder/name))
        for name,value in identity['sources'].items():
            if digest(folder/'project'/name)!=value:raise ValueError('retained source changed: '+name)
        identities[str(folder)]=identity
    env={**os.environ,'GOMAXPROCS':'1'}
    report={'cpu':args.cpu,'iterations':args.iterations,'samples':args.samples,'warmup_samples':1,'runner_sha256':digest(Path(__file__)),'identities':identities,'order':'Each workload/pattern pairs all candidates. Candidate order rotates per sample; reference/Cinch order alternates.','harness':'One workload dispatch per sample; fixed per-workload U32 loops, direct scalar cases/fixed arrays, U64 wraparound result checksum. No per-iteration natural counter, checked remainder or optional harness allocation.','results':[]}
    started=time.monotonic()
    for mode in args.workloads:
        for pattern in args.patterns:
            samples={str(p):{'cinch':[],'reference':[]} for p in artifacts};checksums=set();raw=[]
            for sample in range(args.samples+1):
                rotation=sample%len(artifacts);order=artifacts[rotation:]+artifacts[:rotation]
                if sample%2:order=list(reversed(order))
                for folder in order:
                    for label in (['cinch','reference'] if sample%2==0 else ['reference','cinch']):
                        command=[str(folder/'bench'),mode,str(pattern),str(args.iterations)] if label=='cinch' else [str(folder/'reference'),str(folder/'inputs.json'),mode,str(pattern),str(args.iterations)]
                        result=subprocess.run(command,capture_output=True,text=True,env=env)
                        if result.returncode:raise RuntimeError(f'{command}: {result.stdout}\n{result.stderr}')
                        elapsed,checksum=map(int,result.stdout.split());checksums.add(checksum)
                        if sample:samples[str(folder)][label].append(elapsed)
                        raw.append({'artifact':str(folder),'label':label,'sample':sample,'warmup':sample==0,'nanoseconds':elapsed,'checksum':checksum})
            if len(checksums)!=1:raise ValueError(f'public checksum mismatch {mode}/{pattern}: {checksums}')
            row={'workload':mode,'pattern':pattern,'checksum':checksums.pop(),'raw_samples':raw,'samples_ns':samples,'median_ns_per_operation':{name:{label:statistics.median(values)/args.iterations for label,values in measured.items()} for name,measured in samples.items()}}
            report['results'].append(row);report['elapsed_seconds']=time.monotonic()-started
            args.output.parent.mkdir(parents=True,exist_ok=True);args.output.write_text(json.dumps(report,indent=2)+'\n')
            print(mode,pattern,{Path(name).name:{key:round(value,2) for key,value in times.items()} for name,times in row['median_ns_per_operation'].items()},flush=True)
    report['complete']=True;report['elapsed_seconds']=time.monotonic()-started;args.output.write_text(json.dumps(report,indent=2)+'\n')
    print('Unicode paired comparison complete',round(report['elapsed_seconds'],3),flush=True)


if __name__=='__main__':main()
