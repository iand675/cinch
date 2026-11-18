#!/usr/bin/env python3
"""Compile the actual private binary bitcast, packed-lane and proved-load kernels for portable targets."""
from __future__ import annotations
import argparse
import hashlib
import json
from pathlib import Path
import subprocess

ROOT=Path(__file__).resolve().parents[1]


def source()->str:
    """Extract exact implementation fragments rather than reimplementing their behavior in tests."""
    text=(ROOT/'compiler/prelude/Text/Binary/Layout.cinch').read_text()
    out='-- | Isolated binary-layout packed, bitcast and exact-load kernels.\nmodule Main where\n  import Data.Machine\n  import Data.Bool\n  import Data.Bytes\n  import Data.Nat\n'
    for begin,end in [
        ('  -- | Select target endianness at compilation','  -- | Resolve explicit or target-native'),
        ('  -- | Retain a complete byte owner','  -- | Reverse one complete unsigned sixteen'),
        ('  -- | Observe all binary32','  -- | One bit-preserving binary32'),
        ('  -- | Pack four low byte lanes','  -- | Resolve the packed little-endian')]:
        start=text.index(begin);finish=text.index(end,start);out+=text[start:finish]
    return out+(ROOT/'compiler/tests/fixtures/binary-layout-kernel-entry.cinchpart').read_text()


def run(command:list[str])->None:
    """Stop on either frontend or target object-emission failure."""
    result=subprocess.run(command,capture_output=True,text=True)
    if result.returncode:raise RuntimeError(f'{command}\n{result.stdout}\n{result.stderr}')


def main()->None:
    """Check actual target profiles available to the frozen CLI; integration also covers big-endian profiles."""
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--compiler',type=Path,required=True)
    parser.add_argument('--clang',default='clang')
    parser.add_argument('--artifacts',type=Path,required=True)
    parser.add_argument('--output',type=Path,required=True)
    args=parser.parse_args();args.artifacts.mkdir(parents=True,exist_ok=True)
    text=source();entry=args.artifacts/'Main.cinch';entry.write_text(text)
    report={'compiler_sha256':hashlib.sha256(args.compiler.read_bytes()).hexdigest(),'kernel_source_sha256':hashlib.sha256(text.encode()).hexdigest(),'foreign_execution':False,'objects':[]}
    for triple in ['wasm32-unknown-unknown','aarch64-unknown-linux-gnu']:
        ir=args.artifacts/(triple+'.ll');obj=ir.with_suffix('.o')
        run([str(args.compiler.resolve()),'emit-ir',str(entry.resolve()),'--target',triple,'-o',str(ir.resolve())])
        assert '<4 x i32>' in ir.read_text()
        run([args.clang,'-target',triple,'-O2','-c','-x','ir',str(ir),'-o',str(obj)])
        report['objects'].append({'target':triple,'ir_sha256':hashlib.sha256(ir.read_bytes()).hexdigest(),'object_sha256':hashlib.sha256(obj.read_bytes()).hexdigest()})
        print(triple+': actual kernels compiled to object',flush=True)
    args.output.write_text(json.dumps(report,indent=2)+'\n')

if __name__=='__main__':main()
