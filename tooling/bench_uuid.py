#!/usr/bin/env python3
"""Build and independently preflight complete UUID pipelines; measurements require a separate exclusive timing grant."""
from __future__ import annotations
import argparse
import hashlib
import json
import os
import re
from pathlib import Path
import shutil
import statistics
import subprocess
from validate_uuid import ROOT, MODULES, digest, execute


def scalar_hex(source: str) -> str:
    """Disable only the private vector block dispatch in a separate benchmark source copy."""
    start=source.index('  encode-buffer-from b i out oi =')
    end=source.index('\n  -- | Decode SIMD blocks',start)
    replacement='''  encode-buffer-from b i out oi =
    if i < bytes-len b && oi + 2 <= bytes-len out then
      let v = bytes-get b i in
      let hi = bytes-set out oi (hex-digit (nat-div v 16)) in
      let lo = bytes-set out (oi + 1) (hex-digit (nat-rem v 16)) in
      encode-buffer-from b (i + 1) out (oi + 2)
    else ()
'''
    source=source[:start]+replacement+source[end:]
    source=source.replace('-- | Fill complete four-byte SIMD blocks, then the exact scalar tail.', '-- | Scalar-only encoding control with unchanged public buffer ownership.')
    start=source.index('  decode-buffer-from input i out oi =')
    end=source.index('\n  -- | Store a valid pair',start)
    result=source[:start]+'  decode-buffer-from input i out oi = decode-scalar input i out oi\n'+source[end:]
    return result.replace('-- | Decode SIMD blocks directly into the output, then identify a scalar error or exact tail.\n  -- Each decoded block is stored once; no per-byte list nodes are allocated.', '-- | Scalar-only pair decoding with unchanged public error ordering and ownership.')


def cache_bytes(directory: Path) -> int:
    """Account for new shared-cache payload growth in the bounded output budget."""
    return sum(path.stat().st_size for path in directory.rglob('*') if path.is_file()) if directory.exists() else 0


def main() -> None:
    """Separate bounded builds, full result/leak preflights and explicitly scheduled timing samples."""
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('action',choices=('build','preflight','measure'))
    parser.add_argument('--compiler',type=Path,required=True)
    parser.add_argument('--runtime',type=Path,required=True)
    parser.add_argument('--clang',type=Path,required=True)
    parser.add_argument('--go',type=Path,default=Path('/tmp/cinch-go-current/go/bin/go'))
    parser.add_argument('--reference-artifacts',type=Path,help='Reuse an immutable reference executable instead of building another SDK program')
    parser.add_argument('--cpu',type=int,required=True)
    parser.add_argument('--artifacts',type=Path,required=True)
    parser.add_argument('--output',type=Path)
    parser.add_argument('--samples',type=int,default=5)
    parser.add_argument('--iterations',type=int,default=4096)
    parser.add_argument('--workloads',nargs='+',choices=('parse','format','append','compare'),default=['parse','format','append','compare'])
    parser.add_argument('--patterns',nargs='+',type=int,default=[0,1,2,3,4])
    parser.add_argument('--sizes',nargs='+',type=int,default=[0,64,4096])
    args=parser.parse_args();os.sched_setaffinity(0,{args.cpu})
    directory=args.artifacts.resolve();identity=directory/'identity.json'
    environment={**os.environ,'CINCH_CLANG':str(args.clang.resolve()),'GOCACHE':'/tmp/cinch-go-surface-cache',
                 'GOTELEMETRY':'off','GOTOOLCHAIN':'local','GOMAXPROCS':'1','GOFLAGS':'-p=1','RUST_MIN_STACK':'67108864'}
    if args.action=='build':
        assert shutil.disk_usage(ROOT).free>=350*1024*1024
        assert not directory.exists(),'Preserve every benchmark snapshot and failed build'
        directory.mkdir(parents=True)
        cache_before=cache_bytes(Path(environment['GOCACHE']))
        report={'status':'building','compiler_sha256':digest(args.compiler),'runtime_sha256':digest(args.runtime),
                'clang_sha256':digest(args.clang),'variants':{},'cache_bytes_before':cache_before,'scope':'Separate SIMD/scalar Hex copies, identical public UUID pipeline and ownership semantics; packed comparison is unchanged as a control.'}
        identity.write_text(json.dumps(report,indent=2)+'\n')
        source=directory/'reference.go';source.write_bytes((ROOT/'tooling/bench_uuid_reference.go').read_bytes())
        reference=directory/'reference'
        if args.reference_artifacts:
            previous=args.reference_artifacts.resolve();record=json.loads((previous/'identity.json').read_text())
            assert digest(previous/'reference')==record['reference_executable_sha256']
            assert digest(previous/'reference.go')==record['reference_source_sha256']==digest(source)
            shutil.copy2(previous/'reference',reference)
            report.update(reference_reused_from=str(previous),reference_source_sha256=digest(source),
                          go_sha256=record['go_sha256'],go_version=record['go_version'])
        else:
            plan=execute([str(args.go),'build','-n','-o',str(reference),str(source)],directory,environment)
            (directory/'reference-plan.log').write_text(plan['stdout']+plan['stderr'])
            report.update(reference_plan=plan,reference_source_sha256=digest(source),go_sha256=digest(args.go),go_version=execute([str(args.go),'version'],directory,environment))
            identity.write_text(json.dumps(report,indent=2)+'\n')
            assert plan['exit']==0,plan
            compiles=[line for line in (plan['stdout']+plan['stderr']).splitlines() if re.search(r'/compile(?:\s|$)',line)]
            assert all(' -p main ' in line for line in compiles),'Stop before rebuilding uncached SDK dependencies; retain dry-run plan'
        for variant in ('simd','scalar'):
            project=directory/variant;project.mkdir()
            for module in MODULES:
                path=ROOT/'experiments/uuid-candidate-v1/sources'/(module+'.cinch') if module in ('Data.Uuid','Effect.Uuid') else ROOT/'compiler/prelude'/(module.replace('.','/')+'.cinch')
                text=path.read_text()
                if module=='Text.Hex' and variant=='scalar':text=scalar_hex(text)
                (project/(module+'.cinch')).write_text(text)
            entry=project/'Main.cinch';entry.write_bytes((ROOT/'tooling/data/uuid-benchmark.cinch').read_bytes())
            executable=project/'bench';command=[str(args.compiler.resolve()),'build',str(entry),'--opt-level','2','--runtime-archive',str(args.runtime.resolve()),'-o',str(executable)]
            result=execute(command,project,environment,True)
            (project/'build.log').write_text(result['stdout']+result['stderr'])
            report['current_build']=result;identity.write_text(json.dumps(report,indent=2)+'\n')
            assert result['exit']==0,result
            ir=project/'bench.ll';assembly=project/'bench.s'
            subprocess.run([str(args.clang),'-O2','-S','-x','ir',str(ir),'-o',str(assembly)],check=True,capture_output=True,text=True)
            report['variants'][variant]={'command':result,'executable_sha256':digest(executable),'llvm_sha256':digest(ir),'assembly_sha256':digest(assembly),
                                        'sources':{p.name:digest(p) for p in project.glob('*.cinch')}}
            assert cache_bytes(directory)+max(0,cache_bytes(Path(environment['GOCACHE']))-cache_before)<=30*1024*1024
            assert shutil.disk_usage(ROOT).free>=250*1024*1024
        if args.reference_artifacts:
            result={'reused':True,'source':str(args.reference_artifacts.resolve()),'exit':0}
        else:
            result=execute([str(args.go),'build','-o',str(reference),str(source)],directory,environment)
            (directory/'reference-build.log').write_text(result['stdout']+result['stderr'])
        report['reference_build']=result;identity.write_text(json.dumps(report,indent=2)+'\n')
        assert result['exit']==0,result
        assert cache_bytes(directory)+max(0,cache_bytes(Path(environment['GOCACHE']))-cache_before)<=30*1024*1024
        assert shutil.disk_usage(ROOT).free>=250*1024*1024
        report.update(status='built-not-preflighted',new_cache_bytes=max(0,cache_bytes(Path(environment['GOCACHE']))-cache_before),output_bytes=cache_bytes(directory),reference_source_sha256=digest(source),reference_executable_sha256=digest(reference),reference_build=result)
        identity.write_text(json.dumps(report,indent=2)+'\n');print('Built immutable UUID benchmark snapshots; no timing/adoption claim');return
    assert args.output is not None
    report=json.loads(identity.read_text());assert report['compiler_sha256']==digest(args.compiler) and report['runtime_sha256']==digest(args.runtime)
    for variant,record in report['variants'].items():
        for key,name in [('executable_sha256','bench'),('llvm_sha256','bench.ll'),('assembly_sha256','bench.s')]:assert digest(directory/variant/name)==record[key]
        for name,value in record['sources'].items():assert digest(directory/variant/name)==value
    assert digest(directory/'reference')==report['reference_executable_sha256']
    if args.action=='measure':
        assert report.get('preflight_passed'),'Use only exact snapshots that completed both heap profiles and reference fingerprints'
        covered={tuple(row) for row in report.get('preflight_conditions',[])}
        assert all((mode,size,pattern) in covered for mode in args.workloads for size in args.sizes for pattern in args.patterns),'Every measured condition must have exact normal/poison/reference preflights'
    rows=[]
    for mode in args.workloads:
        for size in args.sizes:
            for pattern in args.patterns:
                counts=[0,1,17] if args.action=='preflight' else [args.iterations]
                for count in counts:
                    checksums=set();samples={k:[] for k in ('simd','scalar','reference')};runs=[]
                    for sample in range(1 if args.action=='preflight' else args.samples+1):
                        variants=('simd','scalar','reference') if sample%2==0 else ('reference','scalar','simd')
                        for variant in variants:
                            executable=directory/'reference' if variant=='reference' else directory/variant/'bench'
                            profiles=(False,True) if args.action=='preflight' and variant!='reference' else (False,)
                            for poison in profiles:
                                env={**environment,'CINCH_HEAP_LIMIT':'256M' if poison else '32M','CINCH_STACK_SIZE':'8M',
                                     'CINCH_RC_DEBUG':'1' if poison else '0','CINCH_LEAK_CHECK':'1'} if args.action=='preflight' else {**{k:v for k,v in environment.items() if not k.startswith('CINCH_')},'CINCH_RC_DEBUG':'0','CINCH_ALLOC_STATS':'0','CINCH_LEAK_CHECK':'0','CINCH_HEAP_LIMIT':'32M','CINCH_STACK_SIZE':'8M'}
                                result=execute([str(executable),mode,str(size),str(count),str(pattern)],directory,env)
                                assert result['exit']==0,result
                                elapsed,checksum=map(int,result['stdout'].split());checksums.add(checksum)
                                if args.action=='preflight' and variant!='reference':assert 'cinch-leak: live=0 immortal=0 leaked=0' in result['stderr']
                                if sample:samples[variant].append(elapsed)
                                if args.action=='preflight':runs.append({'variant':variant,'poison':poison,'diagnostics':result['stderr']})
                    assert len(checksums)==1,(mode,size,pattern,count,checksums)
                    row={'workload':mode,'size':size,'pattern':pattern,'iterations':count,'checksum':checksums.pop()}
                    if args.action=='preflight':row['runs']=runs
                    else:row.update(samples_ns=samples,median_ns={k:statistics.median(v) for k,v in samples.items()})
                    rows.append(row)
    result={'action':args.action,'identity_sha256':digest(identity),'rows':rows,'timing_claim':args.action=='measure','cpu_affinity':[args.cpu]}
    args.output.parent.mkdir(parents=True,exist_ok=True);args.output.write_text(json.dumps(result,indent=2)+'\n')
    if args.action=='preflight':report.update(status='preflight-passed-not-measured',preflight_passed=True,preflight_sha256=digest(args.output),preflight_conditions=[[mode,size,pattern] for mode in args.workloads for size in args.sizes for pattern in args.patterns]);identity.write_text(json.dumps(report,indent=2)+'\n')

if __name__=='__main__':main()
