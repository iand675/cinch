#!/usr/bin/env python3
"""Validate exact String joining and cloning through a receipted embedded-source override."""
from __future__ import annotations
import argparse
import json
import os
from pathlib import Path
import shutil
from bench_stdlib import source_closure
from validate_buffer_construction import consumer
from validate_string_construction import digest,literal,run,verify_receipt

ROOT=Path(__file__).resolve().parents[1]


def checks(rows:list[dict])->list[str]:
    """Render independent valid scalar patterns and complete result observations."""
    return ['join-case ['+','.join(literal(part) for part in row['pattern'])+'] '+str(row['cycles'])+' '+literal(row['separator'])+f' {row["bytes"]} {row["scalars"]} "{row["hex"]}" "{row["sha256"]}"' for row in rows]


def main()->None:
    """Keep String source epochs explicit and each poison quarantine inside its fixed heap cap."""
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--compiler',type=Path,required=True)
    parser.add_argument('--module-source',type=Path,default=ROOT/'seed/prelude/Data/String.cinch')
    parser.add_argument('--epoch',choices=['source-override','fresh-embedded'],default='source-override')
    parser.add_argument('--artifacts',type=Path,required=True)
    parser.add_argument('--output',type=Path,required=True)
    parser.add_argument('--opt-level',type=int,choices=[0,2],default=0)
    parser.add_argument('--groups',nargs='+',choices=['joins','owners'],default=['joins','owners'])
    parser.add_argument('--first',type=int,default=0)
    parser.add_argument('--limit',type=int)
    args=parser.parse_args();fixture_path=ROOT/'seed/tests/fixtures/string-join-vectors.json';fixture=json.loads(fixture_path.read_text())
    report={'compiler_sha256':digest(args.compiler),'source_sha256':digest(args.module_source),'fixture_sha256':digest(fixture_path),'opt_level':args.opt_level,'epoch':'source-override','fresh_embedded_validation_claimed':False,'native_base_validation_claimed':False,'results':[]}
    report.update(epoch=args.epoch,fresh_embedded_validation_claimed=args.epoch=='fresh-embedded',source_shadow_used=args.epoch=='source-override',compile_rc_debug_flag=False)
    for group in args.groups:
        project=args.artifacts/group;project.mkdir(parents=True,exist_ok=True);rows=fixture.get(group,[None])[args.first:]
        if args.limit is not None:rows=rows[:args.limit]
        expressions=checks(rows) if group=='joins' else ['join-owners-case ()']
        text=consumer(expressions,(ROOT/'seed/tests/fixtures/string-join-consumer.cinchpart').read_text());entry=project/'Main.cinch';entry.write_text(text)
        for name,path in source_closure(text).items():shutil.copyfile(path,project/(name+'.cinch'))
        selected=args.module_source.resolve()
        shadow=project/'Data.String.cinch'
        if args.epoch=='source-override':
            shadow.write_text('-- Explicit String join/clone source-override receipt v1.\n'+args.module_source.read_text());selected=shadow
        else:assert not shadow.exists(),'fresh embedded consumer must not shadow String'
        exe=(project/'consumer').resolve()
        run([str(args.compiler.resolve()),'build',str(entry.resolve()),'-o',str(exe),'--opt-level',str(args.opt_level)]);receipt=verify_receipt(project,selected);ownership=[]
        for debug in [False,True]:
          for batch in range((len(expressions)+7)//8):
            repeats=10 if group=='owners' else 1
            for repetition in range(repeats):
                result=run([str(exe),str(batch)],env={**os.environ,'CINCH_RC_DEBUG':'1' if debug else '0','CINCH_LEAK_CHECK':'1','CINCH_HEAP_LIMIT':'256M' if debug else '32M','CINCH_STACK_SIZE':'8M'})
                assert 'leaked=0' in result.stderr,result.stderr
                ownership.append({'debug':debug,'batch':batch,'repetition':repetition,'zero_leaks':True})
        report['results'].append({'group':group,'cases':len(rows),'epoch_receipt':receipt,'entry_sha256':digest(entry),'executable_sha256':digest(exe),'llvm_sha256':digest(exe.with_suffix('.ll')),'ownership':ownership});print(f'{group}: {len(rows)} cases, exact source and both ownership modes passed',flush=True)
    args.output.parent.mkdir(parents=True,exist_ok=True);args.output.write_text(json.dumps(report,indent=2)+'\n')


if __name__=='__main__':main()
