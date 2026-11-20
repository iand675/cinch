#!/usr/bin/env python3
"""Measure retained public comparison pipelines against an independent SDK reference.

Inputs are exact frozen frontends and their matching runtime/provider archives.
Every elapsed sample and output checksum is retained, including regressions and
noise. Results cover the complete scalar loop, dispatch and selection containers.
"""
from __future__ import annotations
import argparse
import json
import os
from pathlib import Path
import platform
import re
import resource
import shutil
import statistics

from check_compare import ROOT, digest, run


def sample(exe: Path, mode: str, iterations: int, work: Path) -> tuple[int,int]:
    """Read one raw timed sample plus its independently checked observable checksum."""
    env={**os.environ,'GOMAXPROCS':'1','GOTELEMETRY':'off','CINCH_HEAP_LIMIT':'32M','CINCH_STACK_SIZE':'8M','CINCH_LEAK_CHECK':'1','CINCH_RC_DEBUG':'0'}
    result=run([str(exe),mode,str(iterations)],work,60,env)
    if exe.name=='bench' and 'leaked=0' not in result.stderr: raise RuntimeError(f'missing zero-leak sample evidence: {result.stderr}')
    fields=result.stdout.split()
    return tuple(map(int,fields))


def main() -> None:
    """Build both public pipelines with explicit identities and alternate their sample order."""
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--work',type=Path,required=True)
    parser.add_argument('--compiler',type=Path,required=True)
    parser.add_argument('--runtime',type=Path,required=True)
    parser.add_argument('--native',type=Path,help='Matching frozen Native manifest/provider directory')
    parser.add_argument('--go',type=Path,required=True)
    parser.add_argument('--cpu',type=int,required=True)
    parser.add_argument('--iterations',nargs='+',type=int,default=[1000,10000,100000])
    parser.add_argument('--samples',type=int,default=7)
    parser.add_argument('--progression',action='store_true',help='Advance scalar operand phases without checked remainder allocation')
    args=parser.parse_args(); os.sched_setaffinity(0,{args.cpu})
    resource.setrlimit(resource.RLIMIT_STACK,(64*1024*1024,resource.getrlimit(resource.RLIMIT_STACK)[1]))
    work=args.work.resolve(); work.mkdir(parents=True,exist_ok=True)
    compiler=args.compiler.resolve(); runtime=args.runtime.resolve(); clang=os.environ.get('CINCH_CLANG','clang')
    driver=ROOT/('tooling/data/compare-benchmark-progression.cinch' if args.progression else 'tooling/data/compare-benchmark.cinch')
    reference_source=ROOT/('tooling/data/compare-reference-progression.go' if args.progression else 'tooling/data/compare-reference.go')
    for source,target in [(ROOT/'seed/prelude/Data/Compare.cinch',work/'Data.Compare.cinch'),(driver,work/'Main.cinch')]: shutil.copyfile(source,target)
    ir=work/'bench.ll'; exe=work/'bench'; reference=work/'reference'
    report={'format':1,'compiler':str(compiler),'compiler_sha256':digest(compiler),'runtime_sha256':digest(runtime),'library_sha256':digest(work/'Data.Compare.cinch'),'driver_sha256':digest(work/'Main.cinch'),'reference_source_sha256':digest(reference_source),'cpu_affinity':[args.cpu],'opt_level':2,'iterations':args.iterations,'samples':args.samples,'profile':{'heap':'32M','stack':'8M','poison':False,'leak_check':True},'scope':'Complete public scalar comparison/less or first-non-default pipeline; includes loop, dispatch, arithmetic and selection container construction. No timing threshold. Different allocation and compiler policies remain visible.','results':[]}
    report.update(driver_path=str(driver),reference_source_path=str(reference_source),runner_sha256=digest(Path(__file__)),progression=args.progression)
    if args.native:
        snap=args.native.resolve(); cfg=json.loads((snap/'objects.json').read_text()); provider=Path(cfg['object'])
        (work/'bundle.source').write_text((work/'Data.Compare.cinch').read_text()+'\n'+(work/'Main.cinch').read_text())
        command=[str(compiler),'--emit-ir','--interfaces',str(snap/'base-interfaces.json'),'--objects',str(snap/'objects.json'),str(work/'bundle.source'),str(ir)]
        report['native']={key:digest(snap/key) for key in ('base-interfaces.json','objects.json')};report['native']['provider_sha256']=digest(provider)
    else: command=[str(compiler),'emit-ir',str(work/'Main.cinch'),'-o',str(ir)]
    result=run(command,work,300); report['compile']={'command':command,'stdout':result.stdout,'stderr':result.stderr}
    target=re.search(r'target triple = "([^"]+)"',ir.read_text())
    triple=target.group(1) if target else cfg['target']
    report.update(target_triple=triple,platform=platform.platform(),machine=platform.machine(),clang=run([clang,'--version'],work).stdout.splitlines()[0],reference_GOMAXPROCS=1,cpu=next(line.split(':',1)[1].strip() for line in Path('/proc/cpuinfo').read_text().splitlines() if line.startswith('model name')))
    link=[clang,'-O2','--target='+triple,str(ir)]
    if args.native: link.append(str(provider))
    run([*link,str(runtime),'-ldl','-lpthread','-lm','-o',str(exe)],work)
    run([clang,'-O2','-S',str(ir),'-o',str(work/'bench.s')],work)
    env={**os.environ,'GOMAXPROCS':'1','GOTELEMETRY':'off','GOCACHE':'/tmp/cinch-compare-go-cache'}
    run([str(args.go),'build','-o',str(reference),str(reference_source)],work,180,env)
    report.update(executable_sha256=digest(exe),llvm_sha256=digest(ir),assembly_sha256=digest(work/'bench.s'),reference_executable_sha256=digest(reference),sdk=run([str(args.go),'version'],work,env=env).stdout.strip())
    for mode in ('integer','float','choice','string'):
        for n in args.iterations:
            sample(exe,mode,n,work); sample(reference,mode,n,work)
            elapsed={'cinch':[],'reference':[]}; checksums=set()
            for i in range(args.samples):
                for label,program in ([('cinch',exe),('reference',reference)] if i%2==0 else [('reference',reference),('cinch',exe)]):
                    nanos,checksum=sample(program,mode,n,work);elapsed[label].append(nanos);checksums.add(checksum)
            if len(checksums)!=1: raise RuntimeError(f'{mode}/{n}: checksums differ {checksums}')
            medians={label:statistics.median(values)/n for label,values in elapsed.items()}
            report['results'].append({'mode':mode,'iterations':n,'checksum':checksums.pop(),'raw_samples_ns':elapsed,'median_ns_per_iteration':medians,'cinch_over_reference':medians['cinch']/medians['reference']})
            (work/'report.json').write_text(json.dumps(report,indent=2)+'\n')
            print(mode,n,medians,flush=True)
    (work/'report.json').write_text(json.dumps(report,indent=2)+'\n')


if __name__=='__main__': main()
