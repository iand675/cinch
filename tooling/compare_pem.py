#!/usr/bin/env python3
"""Compare checksum-equal public PEM baseline/candidate/reference pipelines in interleaved samples."""
from __future__ import annotations
import argparse
from datetime import datetime, timezone
import json
import os
from pathlib import Path
import statistics
from bench_pem import MODES, run
from validate_pem import digest


def quota_state() -> dict:
    """Observe cgroup capacity/throttling without correcting or discarding measured samples."""
    result={}
    for name in ['cpu.max','cpu.stat','cpuset.cpus.effective']:
        path=Path('/sys/fs/cgroup')/name
        if path.exists():result[name]=path.read_text().strip()
    return result


def frozen(artifacts: Path, preflight: Path) -> dict:
    """Require the exact built bytes and successful public checksum/memory preflight before timing."""
    identity=json.loads((artifacts/'identity.json').read_text())
    checked=json.loads(preflight.read_text())
    if not checked.get('all_passed'):raise ValueError('preflight incomplete')
    for key,name in [('compiler_sha256','compiler'),('executable_sha256','bench'),('llvm_sha256','bench.ll'),('assembly_sha256','bench.s'),('reference_executable_sha256','reference')]:
        if digest(artifacts/name)!=identity[key] or checked[key]!=identity[key]:raise ValueError('artifact/preflight changed: '+name)
    for name,value in identity['sources'].items():
        if digest(artifacts/'project'/name)!=value:raise ValueError('frozen source changed: '+name)
    identity['preflight_sha256']=digest(preflight)
    identity['checked_conditions']={(row['workload'],row['pattern'],row['size'],row['iterations']):row['checksum'] for row in checked['results']}
    return identity


def main() -> None:
    """Retain every warmup, order, sample and checksum, including regressions and background load."""
    parser=argparse.ArgumentParser(description=__doc__)
    for name in ['baseline','candidate','baseline-preflight','candidate-preflight','output']:
        parser.add_argument('--'+name,type=Path,required=True)
    parser.add_argument('--cpu',type=int,required=True)
    parser.add_argument('--samples',type=int,default=5)
    parser.add_argument('--iterations',type=int,default=8)
    parser.add_argument('--sizes',type=int,nargs='+',default=[49,4096,65536])
    parser.add_argument('--patterns',type=int,nargs='+',default=[0,1])
    parser.add_argument('--workloads',choices=MODES,nargs='+',default=['encode','decode','preamble','writer','copy'])
    parser.add_argument('--background',required=True)
    args=parser.parse_args()
    if not 0<args.iterations<=4294967295 or args.samples<3:raise ValueError('bounded positive iterations and at least3 samples required')
    os.sched_setaffinity(0,{args.cpu})
    baseline=frozen(args.baseline,args.baseline_preflight);candidate=frozen(args.candidate,args.candidate_preflight)
    if baseline['reference_executable_sha256']!=candidate['reference_executable_sha256']:raise ValueError('reference epochs differ')
    report={'baseline':{key:value for key,value in baseline.items() if key!='checked_conditions'},'candidate':{key:value for key,value in candidate.items() if key!='checked_conditions'},'cpu':args.cpu,'iterations':args.iterations,'samples':args.samples,'background':args.background,'started_at_utc':datetime.now(timezone.utc).isoformat(),'quota_before':quota_state(),'runner_sha256':digest(Path(__file__)),'method':'One warmup and rotating baseline/candidate/reference order per condition. All elapsed/checksum outputs retained. Separate public copy control; pipeline includes documented fresh decoded field observations.','results':[]}
    env={**os.environ,'GOMAXPROCS':'1','GODEBUG':'','CINCH_RC_DEBUG':'0','CINCH_LEAK_CHECK':'1','CINCH_HEAP_LIMIT':'32M','CINCH_STACK_SIZE':'8M'}
    paths={'baseline':args.baseline/'bench','candidate':args.candidate/'bench','reference':args.baseline/'reference'}
    labels=list(paths)
    for mode in args.workloads:
        for pattern in args.patterns:
            for size in args.sizes:
                for count in [0,1,3]:
                    key=(mode,pattern,size,count)
                    if key not in baseline['checked_conditions'] or baseline['checked_conditions'][key]!=candidate['checked_conditions'].get(key):raise ValueError('missing checksum-equal preflight '+repr(key))
                values={key:[] for key in labels};runs=[];checksums=set()
                for sample in range(args.samples+1):
                    order=labels[sample%3:]+labels[:sample%3]
                    for label in order:
                        result=run([str(paths[label].resolve()),mode,str(size),str(args.iterations),str(pattern)],env=env)
                        elapsed,checksum=map(int,result.stdout.split());checksums.add(checksum)
                        if label!='reference' and 'leaked=0' not in result.stderr:raise ValueError('memory receipt missing')
                        if sample:values[label].append(elapsed/args.iterations)
                        runs.append({'sample':sample,'order':order,'label':label,'elapsed_ns':elapsed,'checksum':checksum,'stderr':result.stderr})
                if len(checksums)!=1:raise ValueError('timed checksum mismatch')
                medians={key:statistics.median(value) for key,value in values.items()}
                row={'workload':mode,'pattern':pattern,'size':size,'checksum':checksums.pop(),'runs':runs,'samples_ns_per_operation':values,'median_ns_per_operation':medians,'candidate_over_baseline':medians['candidate']/medians['baseline'],'candidate_over_reference':medians['candidate']/medians['reference']}
                report['results'].append(row);args.output.parent.mkdir(parents=True,exist_ok=True);args.output.write_text(json.dumps(report,indent=2)+'\n')
                print(mode,pattern,size,medians,flush=True)
    report['quota_after']=quota_state();report['finished_at_utc']=datetime.now(timezone.utc).isoformat()
    report['all_passed']=True;args.output.write_text(json.dumps(report,indent=2)+'\n')


if __name__=='__main__':main()
