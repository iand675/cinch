#!/usr/bin/env python3
"""Validate exact String counting kernels independently of the pending embedded provider epoch."""
from __future__ import annotations
import argparse
import json
import os
from pathlib import Path
import shutil
import sys
from bench_stdlib import source_closure
from validate_byte_search import digest,run

ROOT=Path(__file__).resolve().parents[1]


def consumer(expressions:list[str],module:Path)->str:
    """Extract actual production kernels and keep input ownership around their private traversal."""
    text=module.read_text();start=text.index('  -- | Add bounded private machine counters')
    source='-- | Isolated sealed String count kernels; raw model tests classify byte starts only.\nmodule Main where\n  import Data.Bytes\n  import Data.Bytes.Build as B\n  import Data.Bool\n  import Data.List\n  import Data.Machine\n  import Data.Nat\n  import Data.Result\n  import Text.Hex\n'+text[start:]+'\n'+(ROOT/'compiler/tests/fixtures/string-scalar-kernel-consumer.cinchpart').read_text()
    groups=[]
    for first in range(0,len(expressions),64):
        name=f'checks{first//64}';groups.append(name+' ()');source+=f'\n  -- | Observe independent kernel fixture batch {first//64}.\n  {name} :: Unit -> Bool\n  {name} unit = all [\n'+',\n'.join(f'    ({value})' for value in expressions[first:first+64])+'\n    ]\n'
    return source+'\n  -- | Observe all selected complete production-kernel results.\n  main :: Int\n  main = if '+(' && '.join(groups) or 'True')+' then 0 else 1\n'


def main()->None:
    """Keep isolated kernel evidence distinct from full immutable String/fresh-provider validation."""
    p=argparse.ArgumentParser(description=__doc__);p.add_argument('--compiler',type=Path,required=True);p.add_argument('--native-interfaces',type=Path);p.add_argument('--module-source',type=Path,default=ROOT/'compiler/prelude/Data/String.cinch');p.add_argument('--artifacts',type=Path,required=True);p.add_argument('--output',type=Path,required=True);p.add_argument('--groups',nargs='+',choices=['texts','edges','masks','bytes'],default=['texts','edges','masks','bytes']);p.add_argument('--limit',type=int);args=p.parse_args()
    fixture=json.loads((ROOT/'compiler/tests/fixtures/string-scalar-vectors.json').read_text());compiler=args.compiler.parent/'cinch' if args.native_interfaces else args.compiler;report={'compiler_sha256':digest(compiler),'frontend_sha256':digest(args.compiler),'source_sha256':digest(args.module_source),'isolated_actual_kernels':True,'full_native_String_validation_claimed':False,'native_interfaces_sha256':digest(args.native_interfaces) if args.native_interfaces else None,'results':[]}
    for group in args.groups:
        rows=fixture.get(group,[None])
        if args.limit is not None:rows=rows[:args.limit]
        expressions=['mask-model-loop 0'] if group=='masks' else ['byte-model-loop 0'] if group=='bytes' else [f'kernel-case "{r["text"].encode().hex()}" {r["count"]} {r["scalars"]}' for r in rows]
        project=args.artifacts/group;project.mkdir(parents=True,exist_ok=True);text=consumer(expressions,args.module_source);entry=project/'Main.cinch';entry.write_text(text);dependencies=source_closure(text)
        for name,path in dependencies.items():shutil.copyfile(path,project/(name+'.cinch'))
        exe=(project/'consumer').resolve()
        if args.native_interfaces:
            bundle=project/'Bundle.cinch';bundle.write_text('\n'.join(path.read_text() for path in reversed(list(dependencies.values())))+'\n'+text);command=[sys.executable,str(args.compiler.resolve()),'--rc-debug','--compile-only','--interfaces',str(args.native_interfaces.resolve()),str(bundle.resolve()),str(exe.with_suffix('.ll')),str(exe),str(exe.with_suffix('.link.txt'))]
        else:command=[str(args.compiler.resolve()),'build',str(entry.resolve()),'-o',str(exe),'--opt-level','2','--rc-debug']
        try:run(command)
        except RuntimeError as error:
            path=exe.with_suffix('.ll.diagnostics');raise RuntimeError(str(error)+'\n'+(path.read_text() if path.exists() else '')) from error
        ownership=[]
        for debug in [False,True]:
            result=run([str(exe)],env={**os.environ,'CINCH_RC_DEBUG':'1' if debug else '0','CINCH_HEAP_LIMIT':'256M' if debug else '32M','CINCH_STACK_SIZE':'8M','CINCH_LEAK_CHECK':'1'});assert 'leaked=0' in result.stderr,result.stderr;ownership.append({'debug':debug,'zero_leaks':True})
        report['results'].append({'group':group,'cases':65536 if group=='masks' else 256*34 if group=='bytes' else len(rows),'entry_sha256':digest(entry),'executable_sha256':digest(exe),'ir_sha256':digest(exe.with_suffix('.ll')),'ownership':ownership});print(group,'actual kernels pass both memory profiles',flush=True)
    args.output.parent.mkdir(parents=True,exist_ok=True);args.output.write_text(json.dumps(report,indent=2)+'\n')


if __name__=='__main__':main()
