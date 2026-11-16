#!/usr/bin/env python3
"""Validate scalar packing and typed byte constructors with receipts for both embedded-source overrides."""
from __future__ import annotations
import argparse
import json
import os
from pathlib import Path
import shutil
from bench_stdlib import source_closure
from validate_string_construction import digest,literal,run,verify_receipt

ROOT=Path(__file__).resolve().parents[1]



def consumer(expressions:list[str],extra:str)->str:
    """Dispatch small independent process batches so poisoned frees fit the fixed debug heap."""
    source=(ROOT/'compiler/tests/fixtures/string-construction-consumer.cinchpart').read_text().replace('  import Data.String\n','  import Data.String\n  import Effect.Process\n')+'\n'+extra
    dispatch='False'
    for first in reversed(range(0,len(expressions),8)):
        number=first//8;name=f'checks{number}'
        source+=f'\n  -- | Observe independent constructor fixture batch {number}.\n  {name} :: Unit -> Bool\n  {name} unit = all [\n'+',\n'.join(f'    ({value})' for value in expressions[first:first+8])+'\n    ]\n'
        dispatch=f'if batch == "{number}" then {name} () else '+dispatch
    return source+'\n  -- | Execute only the selected bounded poison-quarantine batch.\n  selected :: String -> Bool\n  selected batch = '+dispatch+'\n  -- | Observe one independent process batch.\n  main :: Unit ->{IO r} Int\n  main unit = with (io = newIO ()) in case process-args () of\n    Ok (Cons batch Nil) -> if selected batch then 0 else 1\n    other -> 2\n'

def checks(group:str,rows:list[dict])->list[str]:
    """Render exact independent scalar or raw-byte observations into actual public operations."""
    out=[]
    for row in rows:
        if group=='packed':out.append(f'packed-case {literal(row["text"])} {row["count"]} {row["bytes"]} {row["scalars"]} "{row["hex"]}" "{row["sha256"]}"')
        else:out.append(f'buffer-case {row["mode"]} {row["count"]} {row["bytes"]} "{row["hex"]}" "{row["sha256"]}"')
    return out


def main()->None:
    """Compile exact selected String/Bytes epochs and run normal plus poisoned ownership checks."""
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--compiler',type=Path,required=True)
    parser.add_argument('--string-source',type=Path,default=ROOT/'compiler/prelude/Data/String.cinch')
    parser.add_argument('--bytes-source',type=Path,default=ROOT/'compiler/prelude/Data/Bytes.cinch')
    parser.add_argument('--epoch',choices=['source-override','fresh-embedded'],default='source-override')
    parser.add_argument('--artifacts',type=Path,required=True)
    parser.add_argument('--output',type=Path,required=True)
    parser.add_argument('--opt-level',type=int,choices=[0,2],default=0)
    parser.add_argument('--groups',nargs='+',choices=['packed','buffers','regression','owners'],default=['packed','buffers','regression','owners'])
    parser.add_argument('--first',type=int,default=0)
    parser.add_argument('--limit',type=int)
    args=parser.parse_args()
    fixture_path=ROOT/'compiler/tests/fixtures/buffer-construction-vectors.json';fixture=json.loads(fixture_path.read_text())
    report={'compiler_sha256':digest(args.compiler),'epoch':args.epoch,'selected_sources':{'Data.String':digest(args.string_source),'Data.Bytes':digest(args.bytes_source)},'fixture_sha256':digest(fixture_path),'opt_level':args.opt_level,'native_base_validation_claimed':False,'results':[]}
    extra=(ROOT/'compiler/tests/fixtures/buffer-construction-consumer.cinchpart').read_text()
    for group in args.groups:
        project=args.artifacts/group;project.mkdir(parents=True,exist_ok=True)
        rows=fixture.get(group,[None])[args.first:]
        if args.limit is not None:rows=rows[:args.limit]
        if group=='owners':expressions=['buffer-owners-case ()']
        elif group=='regression':
            packed=next(row for row in fixture['packed'] if row['text']=='é' and row['count']==65536)
            buffer=next(row for row in fixture['buffers'] if row['mode']==2 and row['count']==65536)
            expressions=checks('packed',[packed])+checks('buffers',[buffer]);rows=[packed,buffer]
        else:expressions=checks(group,rows)
        source=consumer(expressions,extra);entry=project/'Main.cinch';entry.write_text(source)
        for name,path in source_closure(source).items():shutil.copyfile(path,project/(name+'.cinch'))
        selected={}
        for name,path in [('Data.String',args.string_source),('Data.Bytes',args.bytes_source)]:
            override=project/(name+'.cinch')
            if args.epoch=='source-override':
                override.write_text('-- Explicit buffer-constructor source-override receipt v1.\n'+path.read_text());selected[name]=override
            else:
                if override.exists():override.unlink()
                selected[name]=path
        exe=(project/'consumer').resolve()
        run([str(args.compiler.resolve()),'build',str(entry.resolve()),'-o',str(exe),'--opt-level',str(args.opt_level),'--rc-debug'])
        receipts={name:verify_receipt(project,path,name) for name,path in selected.items()}
        ownership=[]
        for debug in [False,True]:
          for batch in range((len(expressions)+7)//8):
            result=run([str(exe),str(batch)],env={**os.environ,'CINCH_RC_DEBUG':'1' if debug else '0','CINCH_LEAK_CHECK':'1','CINCH_HEAP_LIMIT':'256M' if debug else '32M','CINCH_STACK_SIZE':'8M'})
            assert 'leaked=0' in result.stderr,result.stderr
            ownership.append({'debug':debug,'batch':batch,'zero_leaks':True})
        report['results'].append({'group':group,'cases':len(rows),'epoch_receipts':receipts,'entry_sha256':digest(entry),'executable_sha256':digest(exe),'llvm_sha256':digest(exe.with_suffix('.ll')),'ownership':ownership})
        print(f'{group}: {len(rows)} cases, both exact source epochs and ownership modes passed',flush=True)
    args.output.parent.mkdir(parents=True,exist_ok=True);args.output.write_text(json.dumps(report,indent=2)+'\n')

if __name__=='__main__':main()
