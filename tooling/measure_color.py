#!/usr/bin/env python3
"""Measure sealed complete color pipelines only after an explicit exclusive timing grant."""
from __future__ import annotations
import argparse
import json
import os
from pathlib import Path
import shutil
import statistics
import subprocess
import time
from check_compare import digest


def sealed(folder: Path) -> dict:
    """Verify original source/code/reference bytes without compiling or rewriting their receipts."""
    report=json.loads((folder/'report.json').read_text())
    if report['status']!='functional-preflights-passed':
        raise RuntimeError('complete successful functional preflight required')
    for name,key in [('bench','executable_sha256'),('bench.ll','llvm_sha256'),('bench.s','assembly_sha256'),
                     ('Main.cinch','driver_sha256'),('reference','reference_executable_sha256'),('reference.go','reference_source_sha256')]:
        if digest(folder/name)!=report[key]:
            raise RuntimeError('changed sealed input: '+str(folder/name))
    for name,expected in report['source_closure_sha256'].items():
        if digest(folder/(name+'.cinch'))!=expected:
            raise RuntimeError('changed sealed source: '+name)
    if digest(Path(report['compiler']))!=report['compiler_sha256'] or digest(Path(report['runtime']))!=report['runtime_sha256']:
        raise RuntimeError('changed frozen compiler/runtime identity')
    return report


def main() -> None:
    """Keep raw paired order, all regressions, exact checksums and actual before/after quota facts."""
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--simd',type=Path,required=True)
    parser.add_argument('--scalar',type=Path,required=True)
    parser.add_argument('--output',type=Path,required=True)
    parser.add_argument('--context',required=True)
    parser.add_argument('--modes',nargs='+',required=True)
    parser.add_argument('--sizes',type=int,nargs='+',default=[16,256])
    parser.add_argument('--iterations',type=int,default=20)
    parser.add_argument('--palette-iterations',type=int,default=1)
    parser.add_argument('--samples',type=int,default=5)
    parser.add_argument('--cpu',type=int,default=0)
    parser.add_argument('--max-wall-seconds',type=int,default=20)
    parser.add_argument('--start-free-mib',type=int,choices=(300,350),default=350)
    args=parser.parse_args()
    if args.output.exists() or args.samples<=0 or args.iterations<=0 or args.palette_iterations<=0 or any(size<0 for size in args.sizes):
        parser.error('fresh report and positive samples/counts are required')
    folders=[args.simd.resolve(),args.scalar.resolve()]
    receipts=[sealed(folder) for folder in folders]
    for key in ['compiler_sha256','runtime_sha256','source_basis_sha256','driver_sha256','reference_source_sha256','reference_executable_sha256','sdk_sha256','sdk_sources']:
        if receipts[0][key]!=receipts[1][key]:
            raise RuntimeError('unmatched paired input: '+key)
    if receipts[0]['scalar'] or not receipts[1]['scalar']:
        raise RuntimeError('incorrect paired source selection')
    known=json.loads((Path(__file__).resolve().parents[1]/'experiments/image-color-v1/benchmark-modes.json').read_text())['modes']
    if any(mode not in known for mode in args.modes):
        raise RuntimeError('unrecognized public mode')
    initial=shutil.disk_usage(args.output.parent).free
    if initial<args.start_free_mib*1024*1024:
        raise RuntimeError('explicit timing initial guard')
    os.sched_setaffinity(0,{args.cpu})
    programs=[('simd',folders[0]/'bench'),('scalar',folders[1]/'bench'),('reference',folders[0]/'reference')]
    report={'scope':__doc__,'runner_sha256':digest(Path(__file__)),'cpu_affinity':[args.cpu],
            'background_context':args.context,'samples':args.samples,'allocation_counters':False,
            'resource_policy':{'scope':'existing sealed inputs only; build350MiB guard unchanged',
                               'start_mib':args.start_free_mib,'reserve_mib':250,'new_report_cap_mib':5,
                               'wall_seconds':args.max_wall_seconds},'initial_free_bytes':initial,
            'input_receipts':[{'path':str(folder/'report.json'),'sha256':digest(folder/'report.json'),
                               'compiler_sha256':receipt['compiler_sha256'],'runtime_sha256':receipt['runtime_sha256'],
                               'executable_sha256':receipt['executable_sha256'],'source_basis_sha256':receipt['source_basis_sha256'],
                               'source_closure_sha256':receipt['source_closure_sha256'],
                               'reference_executable_sha256':receipt['reference_executable_sha256']}
                              for folder,receipt in zip(folders,receipts)],
            'profile':{'heap':'32M','stack':'8M','poison':False,'leak_check':True},'results':[],
            'quota_before':Path('/sys/fs/cgroup/cpu.stat').read_text()}
    env={**os.environ,'GOMAXPROCS':'1','GOTELEMETRY':'off','CINCH_HEAP_LIMIT':'32M','CINCH_STACK_SIZE':'8M',
         'CINCH_RC_DEBUG':'0','CINCH_LEAK_CHECK':'1','CINCH_ALLOC_STATS':'0'}
    started=time.monotonic()
    deadline=started+args.max_wall_seconds

    def save() -> None:
        """Durably retain each completed condition and any partial failing observation."""
        with args.output.open('w') as handle:
            handle.write(json.dumps(report,indent=2)+'\n');handle.flush();os.fsync(handle.fileno())

    try:
        condition=0
        for mode in args.modes:
            for size in args.sizes:
                count=args.palette_iterations if mode.startswith('palette-') else args.iterations
                row={'mode':mode,'input_pixels':size,'iterations':count,'raw_samples_ns':{label:[] for label,_ in programs},'observations':[]}
                report['results'].append(row)
                checksums=set()
                for index in range(-1,args.samples):
                    rotation=(condition+max(index,0))%len(programs)
                    order=programs[rotation:]+programs[:rotation]
                    if index>=0 and index%2: order=list(reversed(order))
                    for label,program in order:
                        remaining=deadline-time.monotonic()
                        if remaining<=0 or shutil.disk_usage(args.output.parent).free<250*1024*1024:
                            raise RuntimeError('timing wall/reserve boundary')
                        command=[str(program),mode,str(size),str(count)]
                        completed=subprocess.run(command,cwd=args.output.parent,text=True,capture_output=True,timeout=remaining,env=env)
                        row['observations'].append({'label':label,'sample_index':index,'command':command,
                                                    'stdout':completed.stdout,'stderr':completed.stderr,'returncode':completed.returncode})
                        if completed.returncode or (label!='reference' and 'leaked=0' not in completed.stderr):
                            raise RuntimeError('failed timed pipeline: '+label+'/'+mode)
                        nanos,output,retained=map(int,completed.stdout.split())
                        checksums.add((output,retained))
                        if index>=0: row['raw_samples_ns'][label].append(nanos)
                if len(checksums)!=1:
                    raise RuntimeError('independent complete output/owner hashes differ: '+mode+'/'+str(size))
                row['output_and_retained_checksums']=list(checksums.pop())
                row['median_ns_per_batch']={label:statistics.median(values)/count for label,values in row['raw_samples_ns'].items()}
                row['scalar_over_simd']=row['median_ns_per_batch']['scalar']/row['median_ns_per_batch']['simd']
                row['simd_over_reference']=row['median_ns_per_batch']['simd']/row['median_ns_per_batch']['reference']
                save();condition+=1
        report['status']='passed'
    except BaseException as error:
        report.update(status='failed-or-resource-paused',failure=repr(error))
        raise
    finally:
        report.update(quota_after=Path('/sys/fs/cgroup/cpu.stat').read_text(),wall_seconds=time.monotonic()-started,
                      final_free_bytes=shutil.disk_usage(args.output.parent).free,
                      input_receipts_unchanged=all(digest(folder/'report.json')==row['sha256'] for folder,row in zip(folders,report['input_receipts'])))
        save()
        if args.output.stat().st_size>5*1024*1024:
            raise RuntimeError('5MiB new-report cap exceeded')
    print(len(report['results']),'complete interleaved public conditions passed')


if __name__=='__main__':
    main()
