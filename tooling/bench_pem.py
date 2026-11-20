#!/usr/bin/env python3
"""Build/preflight paired public PEM pipelines with exact seed/provider identities and bounded counters."""
from __future__ import annotations
import argparse
import json
import os
from pathlib import Path
import shutil
import statistics
import subprocess
from validate_pem import ROOT,ARTIFACT,STAGE3,CONFIG,NATIVE,digest,host_stack,source_closure

MODES=['encode','decode','preamble','recovery','writer','copy']


def run(command: list[str], **kwargs) -> subprocess.CompletedProcess:
    """Run one bounded build/sample while retaining failures and avoiding unverified timings."""
    result=subprocess.run(command,capture_output=True,text=True,**kwargs)
    if result.returncode:raise RuntimeError(str(command)+'\n'+result.stdout+'\n'+result.stderr)
    return result


def main() -> None:
    """Separate functional preflights on CPU2 from exclusive granted CPU0 sampling."""
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('action',choices=['build','preflight','measure'])
    parser.add_argument('--frontend',choices=['rust','native'],default='rust')
    parser.add_argument('--compiler',type=Path)
    parser.add_argument('--runtime',type=Path,default=STAGE3/'libcinchruntime.a')
    parser.add_argument('--interfaces',type=Path,default=CONFIG/'base-interfaces.json')
    parser.add_argument('--objects',type=Path,default=CONFIG/'objects.json')
    parser.add_argument('--provider',type=Path,default=STAGE3/'provider/provider.json')
    parser.add_argument('--module-override',type=Path)
    parser.add_argument('--go',type=Path,default=Path('/tmp/cinch-go-current/go/bin/go'))
    parser.add_argument('--cpu',type=int,required=True)
    parser.add_argument('--artifacts',type=Path,required=True)
    parser.add_argument('--output',type=Path)
    parser.add_argument('--iterations',type=int,default=16)
    parser.add_argument('--samples',type=int,default=5)
    parser.add_argument('--sizes',nargs='+',type=int,default=[0,1,2,3,15,16,17,47,48,49,63,64,65,767,768,769,4096,65536])
    parser.add_argument('--patterns',nargs='+',type=int,default=[0,1,2])
    parser.add_argument('--workloads',nargs='+',choices=MODES,default=MODES)
    args=parser.parse_args();os.sched_setaffinity(0,{args.cpu})
    artifacts=args.artifacts.resolve();artifacts.mkdir(parents=True,exist_ok=True)
    env={**os.environ,'GOCACHE':'/tmp/cinch-go-pem-benchmark-cache','GOTELEMETRY':'off','GODEBUG':'','GOMAXPROCS':'1'}
    identity_path=artifacts/'identity.json'
    if args.action=='build':
        project=artifacts/'project';project.mkdir(parents=True,exist_ok=True)
        source=project/'Main.cinch';source.write_bytes((ROOT/'tooling/data/pem-benchmark.cinch').read_bytes())
        interfaces=json.loads(args.interfaces.read_text())['interfaces'];paths=source_closure(project,source,{row[0] for row in interfaces})+[source]
        if args.module_override:(project/'Text.Pem.cinch').write_bytes(args.module_override.read_bytes())
        actual=args.compiler or (ARTIFACT/'cinch' if args.frontend=='rust' else NATIVE)
        compiler=artifacts/'compiler';shutil.copy2(actual,compiler)
        objects=json.loads(args.objects.read_text());provider=json.loads(args.provider.read_text())
        if digest(args.runtime)!=provider['runtime']['sha256']:raise ValueError('Runtime/provider mismatch')
        if args.frontend=='rust' and digest(compiler)!=provider['producer']['compiler_sha256']:raise ValueError('Rust embedded producer mismatch')
        for key in ['object','metadata']:
            if digest(Path(objects[key]))!=objects[key+'_sha256']:raise ValueError('object/metadata changed')
        ir=artifacts/'bench.ll'
        if args.frontend=='rust':command=[str(compiler),'emit-ir',str(source),'-o',str(ir)]
        else:
            bundle=artifacts/'bundle.cinch';bundle.write_text('\n\n'.join(path.read_text() for path in sorted(paths)))
            command=[str(compiler),'--emit-ir','--interfaces',str(args.interfaces.resolve()),'--objects',str(args.objects.resolve()),str(bundle),str(ir)]
        result=subprocess.run(command,cwd=project,capture_output=True,text=True,preexec_fn=host_stack)
        (artifacts/'compile.stdout').write_text(result.stdout);(artifacts/'compile.stderr').write_text(result.stderr)
        if result.returncode:
            path=ir.with_name(ir.name+'.diagnostics');raise RuntimeError((path.read_text() if path.exists() else result.stdout)+'\n'+result.stderr)
        link=['clang','-O2',str(ir)]
        if args.frontend=='native':link.append(objects['object'])
        link += [str(args.runtime.resolve()),'-ldl','-lpthread','-lm','-o',str(artifacts/'bench')];run(link)
        run(['clang','-O2','-S','-x','ir',str(ir),'-o',str(artifacts/'bench.s')])
        reference=ROOT/'tooling/bench_pem_reference.go';shutil.copy2(reference,artifacts/'reference.go');run([str(args.go),'build','-o',str(artifacts/'reference'),str(reference)],env=env)
        report={'frontend':args.frontend,'compiler':str(actual),'compiler_sha256':digest(compiler),'runtime_sha256':digest(args.runtime),'provider_sha256':digest(args.provider),'interfaces_sha256':digest(args.interfaces),'objects_sha256':digest(args.objects),'object_sha256':objects['object_sha256'],'sources':{path.name:digest(path) for path in paths},'executable_sha256':digest(artifacts/'bench'),'llvm_sha256':digest(ir),'assembly_sha256':digest(artifacts/'bench.s'),'reference_executable_sha256':digest(artifacts/'reference'),'reference_source_sha256':digest(reference),'reference_sdk':run([str(args.go),'version'],env=env).stdout.strip(),'compile_command':command,'link_command':link,'opt_level':2,'rc_debug_instrumentation':False,'module_override':str(args.module_override.resolve()) if args.module_override else None,'producer':provider['producer'],'host':dict(zip(['sysname','nodename','release','version','machine'],os.uname())),'clang':run(['clang','--version']).stdout,'cpu_model':next((line.split(':',1)[1].strip() for line in Path('/proc/cpuinfo').read_text().splitlines() if line.startswith('model name')),'unknown')}
        identity_path.write_text(json.dumps(report,indent=2)+'\n');print('PEM '+args.frontend+' public driver built',flush=True);return
    if args.output is None:parser.error('preflight/measure requires --output')
    report=json.loads(identity_path.read_text())
    for key,name in [('compiler_sha256','compiler'),('executable_sha256','bench'),('llvm_sha256','bench.ll'),('assembly_sha256','bench.s'),('reference_executable_sha256','reference')]:
        if digest(artifacts/name)!=report[key]:raise ValueError('frozen artifact changed: '+name)
    for name,value in report['sources'].items():
        if digest(artifacts/'project'/name)!=value:raise ValueError('frozen source changed: '+name)
    report.update(action=args.action,cpu=args.cpu,runner_sha256=digest(Path(__file__)),results=[],samples=args.samples,iterations=args.iterations,harness='Two prepared immutable block/input variants; bounded U32 loop/phase and U64 wrapping checksum. Preparation is outside clock. Selected length/boundary/body fingerprints retain complete public allocations without an unrelated full byte-hash traversal. Decode includes fresh mutable field snapshots; reference explicitly copies decoded payload/label to match that observation contract. Writer targets an inspectable memory sink; copy is a separate control. No per-iteration Nat remainder/Option counter.')
    for mode in args.workloads:
        for pattern in args.patterns:
            for size in args.sizes:
                counts=[0,1,3] if args.action=='preflight' else [args.iterations]
                for count in counts:
                    values={'cinch':[],'reference':[]};checksums=set();runs=[]
                    for sample in range(1 if args.action=='preflight' else args.samples+1):
                        for label in (['cinch','reference'] if sample%2==0 else ['reference','cinch']):
                            command=[str(artifacts/('bench' if label=='cinch' else 'reference')),mode,str(size),str(count),str(pattern)]
                            current={**env,'CINCH_RC_DEBUG':'0','CINCH_STACK_SIZE':'8M','CINCH_HEAP_LIMIT':'32M','CINCH_LEAK_CHECK':'1'} if args.action=='preflight' and label=='cinch' else env
                            result=run(command,env=current);elapsed,checksum=map(int,result.stdout.split());checksums.add(checksum)
                            if args.action=='preflight' and label=='cinch' and 'leaked=0' not in result.stderr:raise ValueError('missing zero-leak receipt')
                            if sample:values[label].append(elapsed)
                            if args.action=='preflight':runs.append({'label':label,'stderr':result.stderr})
                    if len(checksums)!=1:raise ValueError(f'public checksum mismatch {mode}/{pattern}/{size}/{count}: {checksums}')
                    row={'workload':mode,'pattern':pattern,'size':size,'iterations':count,'checksum':checksums.pop()}
                    if args.action=='measure':row.update(samples_ns=values,median_ns_per_operation={key:statistics.median(data)/count for key,data in values.items()})
                    else:row['runs']=runs
                    report['results'].append(row);args.output.parent.mkdir(parents=True,exist_ok=True);args.output.write_text(json.dumps(report,indent=2)+'\n')
                print(mode,pattern,size,'passed' if args.action=='preflight' else row['median_ns_per_operation'],flush=True)
    report['all_passed']=True;args.output.write_text(json.dumps(report,indent=2)+'\n')


if __name__=='__main__':main()
