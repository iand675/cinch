#!/usr/bin/env python3
"""Build or measure bounded public Unicode scalar pipelines with explicit source/compiler/provider epochs."""
from __future__ import annotations
import argparse
import json
import os
from pathlib import Path
import shutil
import statistics
import subprocess
from validate_unicode import ROOT,RUST,NATIVE,digest
from validate_url import source_closure

MODES=['letter','space','classify','case','fold','table-letter','table-surrogates','table-greek','table-space','table-custom','table-union','catalog','special-turkish','special-custom','case-observe','harness']


def run(command: list[str], **kwargs) -> subprocess.CompletedProcess:
    """Execute one bounded build or checked observable operation sample."""
    result=subprocess.run(command,capture_output=True,text=True,**kwargs)
    if result.returncode:raise RuntimeError(f'{command}: {result.stdout}\n{result.stderr}')
    return result


def main() -> None:
    """Keep functional build/preflight work separate from explicitly coordinated timing runs."""
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('action',choices=['build','preflight','measure'])
    parser.add_argument('--frontend',choices=['rust','native'],default='rust')
    parser.add_argument('--native-snapshot',type=Path,default=NATIVE)
    parser.add_argument('--compiler',type=Path,default=RUST)
    parser.add_argument('--go',type=Path,default=Path('/tmp/cinch-go-current/go/bin/go'))
    parser.add_argument('--cpu',type=int,required=True)
    parser.add_argument('--artifacts',type=Path,required=True)
    parser.add_argument('--module-overrides',type=Path)
    parser.add_argument('--output',type=Path)
    parser.add_argument('--iterations',type=int,default=8192)
    parser.add_argument('--samples',type=int,default=3)
    parser.add_argument('--patterns',nargs='+',type=int,default=[0,1,2,3])
    parser.add_argument('--workloads',nargs='+',choices=MODES,default=MODES)
    args=parser.parse_args();os.sched_setaffinity(0,{args.cpu})
    artifacts=args.artifacts.resolve();artifacts.mkdir(parents=True,exist_ok=True);identity=artifacts/'identity.json'
    env={**os.environ,'GOCACHE':'/tmp/cinch-go-unicode-benchmark-cache','GOTELEMETRY':'off','GODEBUG':'','GOMAXPROCS':'1'}
    if args.action=='build':
        project=artifacts/'project';project.mkdir(parents=True,exist_ok=True)
        driver=project/'Main.cinch';driver.write_bytes((ROOT/'tooling/data/unicode-benchmark.cinch').read_bytes())
        inputs=source_closure(project,driver)+[driver]
        if args.module_overrides:
            for source in args.module_overrides.glob('Data.Unicode*.cinch'):
                destination=project/source.name
                if not destination.exists():raise ValueError('override is outside the exact source closure: '+source.name)
                destination.write_bytes(source.read_bytes())
        actual=args.compiler if args.frontend=='rust' else args.native_snapshot/'cinch';compiler=artifacts/'compiler'
        compiler.unlink(missing_ok=True);shutil.copy2(actual,compiler)
        provider={}
        if args.frontend=='rust':
            command=[str(compiler),'build',str(driver),'-o',str(artifacts/'bench'),'--opt-level','2']
            run(command,cwd=project)
        else:
            bundle=artifacts/'bundle.cinch';bundle.write_text('\n\n'.join(source.read_text() for source in sorted(inputs)))
            command=['python3',str(args.native_snapshot/'frontend.py'),'--compile-only','--interfaces',str(args.native_snapshot/'base-interfaces.json'),str(bundle),str(artifacts/'bench.ll'),str(artifacts/'bench-o0'),str(artifacts/'link-report.txt')]
            try:run(command,cwd=project)
            except RuntimeError:
                path=artifacts/'bench.ll.diagnostics'
                if path.exists():print(path.read_text(),flush=True)
                raise
            link=[os.environ.get('CINCH_CLANG','clang'),'-O2',str(artifacts/'bench.ll'),str(args.native_snapshot/'library.o'),str(args.native_snapshot/'libcinchruntime.a'),'-ldl','-lpthread','-lm','-o',str(artifacts/'bench')]
            run(link)
            provider={name:digest(args.native_snapshot/name) for name in ['frontend.py','base-interfaces.json','objects.json','library.o','library.abi.json','libcinchruntime.a','provenance.json']}
        reference=ROOT/'tooling/bench_unicode_reference.go';shutil.copyfile(reference,artifacts/'reference.go');run([str(args.go),'build','-o',str(artifacts/'reference'),str(reference)],env=env)
        snapshot=ROOT/'tooling/data/unicode-benchmark-inputs.json';shutil.copyfile(snapshot,artifacts/'inputs.json')
        run([os.environ.get('CINCH_CLANG','clang'),'-O2','-S','-x','ir',str(artifacts/'bench.ll'),'-o',str(artifacts/'bench.s')])
        provenance=actual.parent/('snapshot.json' if args.frontend=='rust' else 'provenance.json')
        report={'frontend':args.frontend,'compiler_sha256':digest(compiler),'runtime_sha256':digest(Path('/workspace/dasl-stdlib-io/compiler/runtime/target/release/libcinchruntime.a') if args.frontend=='rust' else args.native_snapshot/'libcinchruntime.a'),'provider_sha256':provider,'compiler_provenance':json.loads(provenance.read_text()) if provenance.exists() else None,'sources':{p.name:digest(p) for p in inputs},'executable_sha256':digest(artifacts/'bench'),'llvm_sha256':digest(artifacts/'bench.ll'),'assembly_sha256':digest(artifacts/'bench.s'),'reference_executable_sha256':digest(artifacts/'reference'),'reference_source_sha256':digest(reference),'input_sha256':digest(artifacts/'inputs.json'),'reference_sdk':run([str(args.go),'version'],env=env).stdout.strip(),'compile_command':command,'optimized_link_command':link if args.frontend=='native' else None,'opt_level':2,'module_overrides':str(args.module_overrides) if args.module_overrides else None}
        identity.write_text(json.dumps(report,indent=2)+'\n');print(f'Unicode {args.frontend} bounded scalar driver built',flush=True);return
    if args.output is None:parser.error('preflight/measure requires --output')
    report=json.loads(identity.read_text())
    for key,name in [('compiler_sha256','compiler'),('executable_sha256','bench'),('llvm_sha256','bench.ll'),('assembly_sha256','bench.s'),('reference_executable_sha256','reference'),('input_sha256','inputs.json')]:
        if digest(artifacts/name)!=report[key]:raise ValueError('retained artifact changed: '+name)
    for name,value in report['sources'].items():
        if digest(artifacts/'project'/name)!=value:raise ValueError('retained source changed: '+name)
    report.update(cpu=args.cpu,runner_sha256=digest(Path(__file__)),iterations=args.iterations,samples=args.samples,results=[],harness='Bounded U32 counter/phase, direct scalar case dispatch and U64 wraparound checksum; no per-iteration natural counter/remainder/Option allocation. Setup, table validation, name resolution for membership contexts and input JSON parsing are outside the timer. Catalog lookups and returned case observations deliberately occur inside it.')
    sizes=[0,1,15,16,17,127] if args.action=='preflight' else [args.iterations]
    for mode in args.workloads:
        for pattern in args.patterns:
            for iterations in sizes:
                values={'cinch':[],'reference':[]};checksums=set();runs=[]
                samples=1 if args.action=='preflight' else args.samples+1
                for index in range(samples):
                    for label in (['cinch','reference'] if index%2==0 else ['reference','cinch']):
                        command=[str(artifacts/'bench'),mode,str(pattern),str(iterations)] if label=='cinch' else [str(artifacts/'reference'),str(artifacts/'inputs.json'),mode,str(pattern),str(iterations)]
                        current=env if args.action=='measure' or label=='reference' else {**env,'CINCH_RC_DEBUG':'0','CINCH_LEAK_CHECK':'1','CINCH_HEAP_LIMIT':'32M','CINCH_STACK_SIZE':'8M'}
                        result=run(command,env=current);elapsed,checksum=map(int,result.stdout.split());checksums.add(checksum)
                        if args.action=='preflight' and label=='cinch' and 'leaked=0' not in result.stderr:raise ValueError('missing zero-leak result: '+result.stderr)
                        if args.action=='measure' and index:values[label].append(elapsed)
                        if args.action=='preflight':runs.append({'label':label,'stderr':result.stderr})
                if len(checksums)!=1:raise ValueError(f'public pipeline checksum mismatch {mode}/{pattern}/{iterations}: {checksums}')
                row={'workload':mode,'pattern':pattern,'iterations':iterations,'checksum':checksums.pop()}
                if args.action=='measure':row.update(samples_ns=values,median_ns_per_operation={k:statistics.median(v)/iterations for k,v in values.items()})
                else:row['runs']=runs
                report['results'].append(row);args.output.parent.mkdir(parents=True,exist_ok=True);args.output.write_text(json.dumps(report,indent=2)+'\n')
                print(mode,pattern,iterations,'passed' if args.action=='preflight' else row['median_ns_per_operation'],flush=True)
    print('Unicode '+args.action+' complete',flush=True)


if __name__=='__main__':main()
