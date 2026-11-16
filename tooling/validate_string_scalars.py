#!/usr/bin/env python3
"""Validate import-free String observations and sealed-valid SIMD counts with exact source receipts."""
from __future__ import annotations
import argparse
import json
import os
from pathlib import Path
import shutil
from bench_stdlib import source_closure
from validate_string_construction import digest,literal,run,verify_receipt

ROOT=Path(__file__).resolve().parents[1]


def endpoint(value:int)->int:
    """Represent absence with the first value beyond the Unicode scalar range."""
    return value if value>=0 else 1114112


def checks(group:str,rows:list[dict])->list[str]:
    """Render independent wire identities and the deliberately unchanged raw prefix policy."""
    out=[]
    for r in rows:
        if group=='raw':out.append(f'raw-length-case "{r["hex"]}" {r["start"]} {r["initial"]} {r["result"]}');continue
        head=f'aggregate-case {r["low"]} {r["high"]}' if group=='aggregates' else f'scalar-case {literal(r["text"])} {r["count"]}'
        out.append(head+f' {r["bytes"]} {r["scalars"]} {endpoint(r["first"])} {r["firstWidth"]} {endpoint(r["last"])} {r["lastWidth"]} "{r["sha256"]}"')
    return out


def consumer(expressions:list[str])->str:
    """Keep ordinary full-corpus runs and one-assertion poison selections in the same executable."""
    source=(ROOT/'compiler/tests/fixtures/string-construction-consumer.cinchpart').read_text().replace('  import Data.String\n','  import Data.String\n  import Data.Char\n  import Effect.Process\n')+'\n'+(ROOT/'compiler/tests/fixtures/string-scalar-consumer.cinchpart').read_text()
    groups=[]
    for first in range(0,len(expressions),64):
        group=first//64;name=f'checks{group}';groups.append(name+' ()');batch=expressions[first:first+64]
        source+=f'\n  -- | Observe complete independent scalar fixture batch {group}.\n  {name} :: Unit -> Bool\n  {name} unit = all [\n'+',\n'.join(f'    ({value})' for value in batch)+'\n    ]\n'
        dispatch='False'
        for index in reversed(range(len(batch))):dispatch=f'if item == "{index}" then ({batch[index]}) else '+dispatch
        source+=f'\n  -- | Select one bounded independent poisoned observation in batch {group}.\n  selected{group} :: String -> Bool\n  selected{group} item = '+dispatch+'\n'
    dispatch='False'
    for group in reversed(range(len(groups))):dispatch=f'if group == "{group}" then selected{group} item else '+dispatch
    source+='\n  -- | Select one independent process observation without a deep global branch chain.\n  selected :: String -> String -> Bool\n  selected group item = '+dispatch+'\n'
    return source+'\n  -- | Run the ordinary corpus together or one independently poisoned assertion.\n  main :: Unit ->{IO r} Int\n  main unit = with (io = newIO ()) in case process-args () of\n    Ok (Cons group (Cons item Nil)) -> if (if group == "all" then '+(' && '.join(groups) or 'True')+' else selected group item) then 0 else 1\n    other -> 2\n'


def main()->None:
    """Keep compiler stack, compiled-program limits and source epochs independently receipted."""
    p=argparse.ArgumentParser(description=__doc__);p.add_argument('--compiler',type=Path,required=True);p.add_argument('--module-source',type=Path,default=ROOT/'compiler/prelude/Data/String.cinch');p.add_argument('--artifacts',type=Path,required=True);p.add_argument('--output',type=Path,required=True);p.add_argument('--opt-level',type=int,choices=[0,2],default=0);p.add_argument('--groups',nargs='+',choices=['texts','edges','aggregates','raw','owners'],default=['texts','edges','aggregates','raw','owners']);p.add_argument('--first',type=int,default=0);p.add_argument('--limit',type=int);args=p.parse_args()
    fixture_path=ROOT/'compiler/tests/fixtures/string-scalar-vectors.json';fixture=json.loads(fixture_path.read_text());report={'compiler_sha256':digest(args.compiler),'source_sha256':digest(args.module_source),'fixture_sha256':digest(fixture_path),'opt_level':args.opt_level,'epoch':'explicit sealed String source override','fresh_embedded_validation_claimed':False,'fresh_native_provider_validation_claimed':False,'results':[]}
    for group in args.groups:
        rows=fixture.get(group,[None])[args.first:]
        if args.limit is not None:rows=rows[:args.limit]
        expressions=checks(group,rows) if group!='owners' else ['scalar-owners-case ()']
        project=args.artifacts/group;project.mkdir(parents=True,exist_ok=True);text=consumer(expressions);entry=project/'Main.cinch';entry.write_text(text)
        for name,path in source_closure(text).items():shutil.copyfile(path,project/(name+'.cinch'))
        selected=project/'Data.String.cinch';selected.write_text('-- Explicit sealed String scalar source-override receipt v1.\n'+args.module_source.read_text());exe=(project/'consumer').resolve();run([str(args.compiler.resolve()),'build',str(entry.resolve()),'-o',str(exe),'--opt-level',str(args.opt_level),'--rc-debug']);receipt=verify_receipt(project,selected);ownership=[]
        for debug in [False,True]:
          cases=range(len(expressions)) if debug else [-1]
          for case in cases:
           for repetition in range(10 if group=='owners' else 1):
            arguments=['all','0'] if case<0 else [str(case//64),str(case%64)]
            result=run([str(exe),*arguments],env={**os.environ,'CINCH_RC_DEBUG':'1' if debug else '0','CINCH_HEAP_LIMIT':'256M' if debug else '32M','CINCH_STACK_SIZE':'8M','CINCH_LEAK_CHECK':'1'});assert 'leaked=0' in result.stderr,result.stderr;ownership.append({'debug':debug,'case':case,'repetition':repetition,'zero_leaks':True})
        report['results'].append({'group':group,'cases':len(rows),'epoch_receipt':receipt,'entry_sha256':digest(entry),'executable_sha256':digest(exe),'llvm_sha256':digest(exe.with_suffix('.ll')),'ownership':ownership});print(f'{group}: {len(rows)} cases, exact source and both memory modes passed',flush=True)
    args.output.parent.mkdir(parents=True,exist_ok=True);args.output.write_text(json.dumps(report,indent=2)+'\n')


if __name__=='__main__':main()
