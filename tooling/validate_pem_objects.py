#!/usr/bin/env python3
"""Compile actual PEM read-only kernel/scanner bodies into portable target objects without Cargo."""
from __future__ import annotations
import argparse
import json
import os
from pathlib import Path
import subprocess
from render_pem_kernels import rendered
from validate_pem import ARTIFACT,digest,host_stack


def main() -> None:
    """Retain exact source/IR/object bytes for each supported frozen-CLI foreign target."""
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--compiler',type=Path,default=ARTIFACT/'cinch')
    parser.add_argument('--cpu',type=int,default=2)
    parser.add_argument('--artifacts',type=Path,required=True)
    parser.add_argument('--output',type=Path,required=True)
    args=parser.parse_args();os.sched_setaffinity(0,{args.cpu})
    artifacts=args.artifacts.resolve();artifacts.mkdir(parents=True,exist_ok=True)
    source=artifacts/'Main.cinch';source.write_text(rendered(True));reports=[]
    for target in ['wasm32-unknown-unknown','aarch64-unknown-linux-gnu']:
        ir=artifacts/(target+'.ll');obj=ir.with_suffix('.o')
        command=[str(args.compiler.resolve()),'emit-ir',str(source),'--target',target,'-o',str(ir)]
        result=subprocess.run(command,capture_output=True,text=True,preexec_fn=host_stack)
        (artifacts/(target+'.compile.stdout')).write_text(result.stdout);(artifacts/(target+'.compile.stderr')).write_text(result.stderr)
        if result.returncode:raise RuntimeError(result.stdout+'\n'+result.stderr)
        link=['clang','-target',target,'-O2','-c','-x','ir',str(ir),'-o',str(obj)]
        result=subprocess.run(link,capture_output=True,text=True)
        (artifacts/(target+'.object.stderr')).write_text(result.stderr)
        if result.returncode:raise RuntimeError(result.stderr)
        reports.append({'target':target,'compile_command':command,'object_command':link,'llvm_sha256':digest(ir),'object_sha256':digest(obj),'bytes':obj.stat().st_size})
    args.output.parent.mkdir(parents=True,exist_ok=True);args.output.write_text(json.dumps({'compiler_sha256':digest(args.compiler),'source_sha256':digest(source),'maintained_source_sha256':digest(Path(__file__).resolve().parents[1]/'compiler/prelude/Text/Pem.cinch'),'actual_primitive_count':10,'targets':reports,'all_passed':True,'big_endian':'PowerPC profile supplied in parent Cargo target lib_pem; not claimed by this CLI gate'},indent=2)+'\n')
    print('PEM actual kernels compiled for wasm and ARM',flush=True)


if __name__=='__main__':main()
