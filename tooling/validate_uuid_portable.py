#!/usr/bin/env python3
"""Emit actual immutable UUID adapters and compile supported target objects without a runtime link."""
from __future__ import annotations
import argparse
import json
import os
from pathlib import Path
import shutil
from validate_uuid import ROOT,digest,execute


def main() -> None:
    """Preserve supported object evidence and any unsupported target refusal as separate observations."""
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--compiler',type=Path,required=True)
    parser.add_argument('--clang',type=Path,required=True)
    parser.add_argument('--candidate-sources',type=Path,help='Optional isolated immutable UUID source override')
    parser.add_argument('--cpu',type=int,required=True)
    parser.add_argument('--artifacts',type=Path,required=True)
    parser.add_argument('--output',type=Path,required=True)
    args=parser.parse_args();os.sched_setaffinity(0,{args.cpu});assert shutil.disk_usage(ROOT).free>=350*1024*1024
    assert not args.artifacts.exists();args.artifacts.mkdir(parents=True)
    report={'kind':'actual-full-immutable-UUID-portable-adapter-objects','compiler_sha256':digest(args.compiler),'clang_sha256':digest(args.clang),'cpu_affinity':[args.cpu],'runtime_linked':False,'candidate_selected':False,'package_complete':False,'targets':[]}
    environment={**os.environ,'CINCH_CLANG':str(args.clang.resolve()),'RUST_MIN_STACK':'67108864','CINCH_TEST_API_EMIT_INTERFACES':'0'}
    for triple in ('wasm32-unknown-unknown','aarch64-unknown-linux-gnu','powerpc64-unknown-linux-gnu'):
        directory=args.artifacts/triple;directory.mkdir()
        modules=('Data.Uuid','Text.Hex','Data.Bytes.Word')
        for module in modules:
            source=ROOT/'compiler/prelude'/(module.replace('.','/')+'.cinch')
            if args.candidate_sources and module=='Data.Uuid':
                candidate=args.candidate_sources.resolve()/(module+'.cinch')
                if candidate.is_file():source=candidate
            (directory/(module+'.cinch')).write_bytes(source.read_bytes())
        entry=directory/'Main.cinch';entry.write_bytes((ROOT/'compiler/tests/fixtures/uuid-portable.cinch').read_bytes())
        ir=directory/'consumer.ll';command=[str(args.compiler.resolve()),'emit-ir',str(entry.resolve()),'--target',triple,'-o',str(ir.resolve())]
        result=execute(command,directory,environment,True);(directory/'emit.log').write_text(result['stdout']+result['stderr'])
        row={'target':triple,'emit':result,'sources':{p.name:digest(p) for p in directory.glob('*.cinch')}};report['targets'].append(row)
        if result['exit']:
            row['status']='refused-before-object';row['llvm_created']=ir.exists()
            if triple!='powerpc64-unknown-linux-gnu':raise RuntimeError(result)
            continue
        text=ir.read_text();assert '@llvm.bswap.i64' in text and '<8 x i8>' in text and 'icmp ult i64' in text,'Actual endian/SIMD/packed bodies must be reachable'
        obj=directory/'consumer.o';compile=execute([str(args.clang.resolve()),'-target',triple,'-O2','-c','-x','ir',str(ir.resolve()),'-o',str(obj.resolve())],directory,environment)
        (directory/'object.log').write_text(compile['stdout']+compile['stderr']);row.update(object=compile,llvm_sha256=digest(ir));assert compile['exit']==0,compile
        row.update(status='actual-adapters-object-passed',object_sha256=digest(obj),object_bytes=obj.stat().st_size)
        assert sum(p.stat().st_size for p in args.artifacts.rglob('*') if p.is_file())<=20*1024*1024
    report['scope']='Supported actual CLI targets only; unsupported PPC remains a maintained custom-profile API gate, never a retargeted little-endian claim.'
    args.output.parent.mkdir(parents=True,exist_ok=True);args.output.write_text(json.dumps(report,indent=2)+'\n')
    print([(r['target'],r['status']) for r in report['targets']])

if __name__=='__main__':main()
