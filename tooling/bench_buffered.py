#!/usr/bin/env python3
"""Stage ordinary-RC public buffered pipeline checks, functional preflights and explicitly granted paired measurements."""
from __future__ import annotations
import argparse
import json
import os
from pathlib import Path
import resource
import shutil
import statistics
import subprocess
from validate_buffered import ROOT,NATIVE,closure,digest,host_stack

RUST=Path('/workspace/dasl-stdlib-io/build/string-word-safe-frozen')
MODES=['byte','rune','line','write','transfer','scan-line','scan-rune','scan-word','copy']


def invoke(command:list[str],**kwargs)->subprocess.CompletedProcess:
    """Return complete bounded command observations and preserve useful failures in the caller's artifact scope."""
    result=subprocess.run(command,capture_output=True,text=True,**kwargs)
    if result.returncode:raise RuntimeError(str(command)+'\n'+result.stdout+result.stderr)
    return result


def main()->None:
    """Keep build/check and memory preflight on functional cores; measurement requires the caller's timing-core grant."""
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('action',choices=['check','build','preflight','measure'])
    parser.add_argument('--frontend',choices=['rust','native'],default='rust')
    parser.add_argument('--rust-snapshot',type=Path,default=RUST)
    parser.add_argument('--native-snapshot',type=Path,default=NATIVE)
    parser.add_argument('--source',type=Path,default=ROOT/'tooling/data/buffered-benchmark.cinch')
    parser.add_argument('--source-overrides',type=Path,help='Explicit experiment-only module copies; embedded base-family overrides are refused')
    parser.add_argument('--go',type=Path,default=Path('/tmp/cinch-go-current/go/bin/go'))
    parser.add_argument('--artifacts',type=Path,required=True)
    parser.add_argument('--output',type=Path)
    parser.add_argument('--cpu',type=int,required=True)
    parser.add_argument('--workloads',nargs='+',choices=MODES,default=MODES)
    parser.add_argument('--sizes',nargs='+',type=int,default=[0,1,7,8,15,16,17,4096,65536])
    parser.add_argument('--patterns',nargs='+',type=int,default=[0,1,2])
    parser.add_argument('--iterations',type=int,default=8)
    parser.add_argument('--samples',type=int,default=5)
    args=parser.parse_args();os.sched_setaffinity(0,{args.cpu})
    if not 1 <= args.iterations <= 4294967295:parser.error('iterations must be in 1..4294967295')
    if args.samples < 1:parser.error('samples must be positive')
    if any(size < 0 or size > 65536 for size in args.sizes):parser.error('sizes must be in 0..65536')
    if any(pattern not in [0,1,2] for pattern in args.patterns):parser.error('patterns must be 0, 1 or 2')
    artifacts=args.artifacts.resolve();artifacts.mkdir(parents=True,exist_ok=True)
    if args.action in ['build','preflight'] and shutil.disk_usage(artifacts).free < 250*1024*1024:
        raise RuntimeError('standing benchmark start guard requires 250 MiB free')
    def used_bytes()->int:
        """Keep the explicitly granted persistent artifact budget separate from shared frozen tools and warmed SDK caches."""
        return sum(path.stat().st_size for path in artifacts.rglob('*') if path.is_file())
    def build_run(command:list[str],name:str,maximum:int,environment:dict|None=None)->subprocess.CompletedProcess:
        """Bound each heavy output file by the remaining25 MiB allowance and retain every diagnostic before failure."""
        available=25*1024*1024-used_bytes()-256*1024
        if available<=0:raise RuntimeError('buffered benchmark artifact allowance exhausted')
        cap=min(maximum,available)
        if shutil.disk_usage(artifacts).free < 200*1024*1024+available:
            raise RuntimeError('standing benchmark reserve requires 200 MiB plus remaining outputs')
        def limits()->None:
            """Keep compiler-host stack explicit and prevent one IR/object/link output from consuming the shared disk."""
            host_stack();resource.setrlimit(resource.RLIMIT_FSIZE,(cap,resource.getrlimit(resource.RLIMIT_FSIZE)[1]))
        result=subprocess.run(command,capture_output=True,text=True,cwd=artifacts,env=environment,preexec_fn=limits)
        (artifacts/(name+'.stdout')).write_text(result.stdout);(artifacts/(name+'.stderr')).write_text(result.stderr)
        if used_bytes()>25*1024*1024:raise RuntimeError('buffered benchmark artifact allowance exceeded')
        if result.returncode:raise RuntimeError(str(command)+'\n'+result.stdout+result.stderr)
        return result
    native=args.native_snapshot.resolve();rust=args.rust_snapshot.resolve()
    interfaces=native/'base-interfaces.json';objects_path=native/'objects.json';objects=json.loads(objects_path.read_text())
    env={**os.environ,'GOCACHE':'/tmp/cinch-go-surface-cache','GOMAXPROCS':'1','GOMEMLIMIT':'256MiB','GOTELEMETRY':'off','GOFLAGS':'-p=1'}
    identity_path=artifacts/'identity.json'
    if args.action in ['check','build']:
        snapshot=rust if args.frontend=='rust' else native
        compiler=snapshot/'cinch';runtime=snapshot/'libcinchruntime.a'
        provenance_path=snapshot/('provenance.json' if args.frontend=='rust' else 'snapshot.json');provenance=json.loads(provenance_path.read_text())
        if digest(compiler)!=provenance['compiler_sha256'] or digest(runtime)!=provenance['runtime_sha256']:raise ValueError('snapshot seed/runtime mismatch')
        if args.frontend=='native':
            provider_path=native/'provider/provider.json';provider=json.loads(provider_path.read_text())
            if digest(runtime)!=provider['runtime']['sha256']:raise ValueError('native provider/runtime mismatch')
            for key in ['object','metadata']:
                if digest(Path(objects[key]))!=objects[key+'_sha256']:raise ValueError('native provider payload mismatch')
        project=artifacts/'project';project.mkdir(exist_ok=True)
        source=project/'Main.cinch';source.write_bytes(args.source.read_bytes())
        paths=closure(project,source,{row[0] for row in json.loads(interfaces.read_text())['interfaces']},args.source_overrides)
        ir=artifacts/'benchmark.ll';executable=artifacts/'benchmark';assembly=artifacts/'benchmark.s'
        if args.frontend=='rust':
            command=[str(compiler),'check',str(source)] if args.action=='check' else [str(compiler),'emit-ir',str(source),'-o',str(ir)]
        else:
            bundle=artifacts/'bundle.cinch';bundle.write_text('\n\n'.join(path.read_text() for path in sorted(paths)))
            command=[str(compiler),'--emit-ir','--interfaces',str(interfaces),'--objects',str(objects_path),str(bundle),str(ir)]
        result=build_run(command,'compile',12*1024*1024)
        (artifacts/'compile.stdout').write_text(result.stdout);(artifacts/'compile.stderr').write_text(result.stderr)
        if result.returncode:
            diagnostic=ir.with_name(ir.name+'.diagnostics');raise RuntimeError((diagnostic.read_text() if diagnostic.exists() else result.stdout)+result.stderr)
        identity={'frontend':args.frontend,'compiler':str(compiler),'compiler_sha256':digest(compiler),'runtime':str(runtime),'runtime_sha256':digest(runtime),'snapshot_sha256':digest(provenance_path),'snapshot':provenance,'interfaces_sha256':digest(interfaces),'objects_sha256':digest(objects_path),'sources':{path.name:digest(path) for path in paths},'compile_command':command,'rc_debug_instrumentation':False,'ordinary_reclaim':True,'source_shadow_used':False,'scope':'Complete public reader/scanner operations and owned writer/transfer pipelines; fresh source-event and mutable-output ownership included. No seed/provider copies or unrelated optional loop counters.'}
        identity['source_overrides']=str(args.source_overrides) if args.source_overrides else None
        if args.frontend=='native':identity['provider']={'sha256':digest(provider_path),'object_sha256':objects['object_sha256'],'metadata_sha256':objects['metadata_sha256']}
        if args.action=='check':
            identity['phase']='source/type/proof check only, no link or execution'
            identity_path.write_text(json.dumps(identity,indent=2)+'\n');print('Buffered public benchmark source checked');return
        link=['clang','-O2',str(ir)]+([objects['object']] if args.frontend=='native' else [])+[str(runtime),'-ldl','-lpthread','-lm','-o',str(executable)]
        result=build_run(link,'link',12*1024*1024)
        build_run(['clang','-O2','-S','-x','ir',str(ir),'-o',str(assembly)],'assembly',8*1024*1024)
        reference=artifacts/'reference.go';reference.write_bytes((ROOT/'tooling/bench_buffered_reference.go').read_bytes())
        version=invoke([str(args.go),'version'],env=env).stdout.strip()
        if version.split()[2]!='go1.27.4':raise ValueError('unexpected reference SDK')
        build_run([str(args.go),'build','-o',str(artifacts/'reference'),str(reference)],'reference-build',4*1024*1024,env)
        identity.update(phase='ordinary-RC O2 public pipeline build',link_command=link,executable_sha256=digest(executable),llvm_sha256=digest(ir),assembly_sha256=digest(assembly),reference_executable_sha256=digest(artifacts/'reference'),reference_source_sha256=digest(reference),reference_sdk=version,clang=invoke(['clang','--version']).stdout)
        identity_path.write_text(json.dumps(identity,indent=2)+'\n');print('Buffered ordinary-RC public driver/reference built');return
    if args.output is None:parser.error('preflight/measure requires --output')
    identity=json.loads(identity_path.read_text())
    for key,name in [('executable_sha256','benchmark'),('llvm_sha256','benchmark.ll'),('assembly_sha256','benchmark.s'),('reference_executable_sha256','reference'),('reference_source_sha256','reference.go')]:
        if digest(artifacts/name)!=identity[key]:raise ValueError('frozen benchmark artifact changed: '+name)
    for name,value in identity['sources'].items():
        if digest(artifacts/'project'/name)!=value:raise ValueError('frozen benchmark source changed: '+name)
    if digest(Path(identity['compiler']))!=identity['compiler_sha256'] or digest(Path(identity['runtime']))!=identity['runtime_sha256']:raise ValueError('frozen seed/runtime changed')
    if args.action=='measure' and args.cpu!=0:raise ValueError('measure requires the explicitly granted timing CPU0')
    report={**identity,'action':args.action,'cpu':args.cpu,'runner_sha256':digest(Path(__file__)),'results':[],'cpu_stat_start':Path('/sys/fs/cgroup/cpu.stat').read_text()}
    for mode in args.workloads:
        for pattern in args.patterns:
            for size in args.sizes:
                for count in ([0,1,3] if args.action=='preflight' else [args.iterations]):
                    if shutil.disk_usage(artifacts).free < 200*1024*1024:
                        raise RuntimeError('standing benchmark reserve fell below 200 MiB')
                    checksums=set();samples={'cinch':[],'reference':[]};runs=[]
                    for sample in range(1 if args.action=='preflight' else args.samples+1):
                        labels=['cinch','reference'] if sample%2==0 else ['reference','cinch']
                        for label in labels:
                            profiles=[False,True] if args.action=='preflight' and label=='cinch' else [False]
                            for poisoned in profiles:
                                command=[str(artifacts/('benchmark' if label=='cinch' else 'reference')),mode,str(size),str(count),str(pattern)]
                                current={**env,'CINCH_RC_DEBUG':'1' if poisoned else '0','CINCH_LEAK_CHECK':'1','CINCH_ALLOC_STATS':'1' if args.action=='preflight' else '0','CINCH_HEAP_LIMIT':'256M' if poisoned else '32M','CINCH_STACK_SIZE':'8M'} if label=='cinch' else env
                                result=subprocess.run(command,capture_output=True,text=True,env=current)
                                observation={'label':label,'command':command,'poisoned':poisoned,'exit_code':result.returncode,'stdout':result.stdout,'stderr':result.stderr}
                                (artifacts/'last-run.json').write_text(json.dumps(observation,indent=2)+'\n')
                                if result.returncode:raise RuntimeError(json.dumps(observation))
                                elapsed,checksum=map(int,result.stdout.split());checksums.add(checksum)
                                if label=='cinch' and 'live=0 immortal=0 leaked=0' not in result.stderr:raise ValueError('missing zero-leak pipeline receipt')
                                if sample:samples[label].append(elapsed)
                                if args.action=='preflight':runs.append(observation)
                    if len(checksums)!=1:
                        (artifacts/'checksum-failure.json').write_text(json.dumps({'workload':mode,'pattern':pattern,'size':size,'iterations':count,'checksums':sorted(checksums),'runs':runs},indent=2)+'\n')
                        raise ValueError(f'public pipeline checksum mismatch {mode}/{pattern}/{size}/{count}: {checksums}')
                    row={'workload':mode,'pattern':pattern,'size':size,'iterations':count,'checksum':checksums.pop()}
                    if args.action=='preflight':row['runs']=runs
                    else:row.update(samples_ns=samples,median_ns_per_operation={key:statistics.median(values)/count for key,values in samples.items()})
                    report['results'].append(row);report['cpu_stat_end']=Path('/sys/fs/cgroup/cpu.stat').read_text()
                    args.output.parent.mkdir(parents=True,exist_ok=True);args.output.write_text(json.dumps(report,indent=2)+'\n')
    print('Buffered public pipeline '+args.action+' completed')


if __name__=='__main__':main()
