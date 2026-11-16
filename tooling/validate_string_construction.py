#!/usr/bin/env python3
"""Validate immutable-string construction with an explicit embedded-source epoch receipt.

Source-override mode copies the exact Data.String implementation beside an
explicit consumer import and requires its emitted interface source hash. Fresh
embedded mode requires the same hash without placing an override file. A frozen
compiler that silently uses its older String implementation cannot pass either
receipt. Native base refresh and full bootstrap gates are separate integration
steps; this runner currently invokes the Rust CLI protocol only.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import subprocess

from bench_stdlib import source_closure

ROOT = Path(__file__).resolve().parents[1]


def digest(path: Path) -> str:
    """Identify the exact compiler, implementation, fixture and executable bytes."""
    return hashlib.sha256(path.read_bytes()).hexdigest()


def source_hash(source: bytes) -> str:
    """Match the interface format's independently computed source identity."""
    value = 0xCBF29CE484222325
    for byte in source:
        value = ((value ^ byte) * 0x100000001B3) & ((1 << 64) - 1)
    return str(value)


def literal(text: str) -> str:
    """Preserve Unicode scalars and controls using the language's accepted escapes."""
    out = '"'
    for char in text:
        if char == '"': out += '\\"'
        elif char == '\\': out += '\\\\'
        elif char == '\n': out += '\\n'
        elif char == '\r': out += '\\r'
        elif char == '\t': out += '\\t'
        elif ord(char) < 32 or ord(char) == 127: out += f'\\u{{{ord(char):x}}}'
        else: out += char
    return out + '"'


def checks(group: str, rows: list[dict]) -> list[str]:
    """Render exact independent result observations rather than reconstructing expected output with String APIs."""
    out = []
    for row in rows:
        expected = f'{row["bytes"]} {row["scalars"]} "{row["hex"]}" "{row["sha256"]}"'
        if group == 'repeats': out.append(f'repeat-case {literal(row["text"])} {row["count"]} {expected}')
        elif group == 'concats': out.append('concat-case [' + ','.join(map(literal,row['pieces'])) + f'] {row["cycles"]} {expected}')
    return out


def consumer(expressions: list[str]) -> str:
    """Keep source batches bounded and every large construction count opaque at runtime."""
    source = (ROOT/'compiler/tests/fixtures/string-construction-consumer.cinchpart').read_text()
    names = []
    for first in range(0,len(expressions),64):
        name = f'checks{first//64}'; names.append(name+' ()')
        source += f'\n  -- | Observe independent fixture batch {first//64}.\n  {name} :: Unit -> Bool\n  {name} unit = all [\n' + ',\n'.join(f'    ({x})' for x in expressions[first:first+64]) + '\n    ]\n'
    return source + '\n  -- | Observe the complete executable fixture result.\n  main :: Unit ->{IO r} Int\n  main unit = with (io = newIO ()) in if ' + (' && '.join(names) or 'True') + ' then 0 else 1\n'


def run(command: list[str], **kwargs) -> subprocess.CompletedProcess:
    """Stop dependent operations on failure and retain the original compiler diagnostics."""
    result = subprocess.run(command,capture_output=True,text=True,**kwargs)
    if result.returncode: raise RuntimeError(f'{command}\n{result.stdout}\n{result.stderr}')
    return result


def verify_receipt(project: Path, module: Path) -> dict:
    """Require the exact selected String source identity in the emitted consumer interface."""
    expected = source_hash(module.read_bytes())
    path = project/'build/Data.String.cinchif'
    interface = json.loads(path.read_text())
    assert interface['module']=='Data.String', interface.get('module')
    assert interface['source_hash']==expected, (interface['source_hash'],expected)
    return {'interface_sha256':digest(path),'source_hash':expected,'module_source_sha256':digest(module)}


def tail_receipt(path: Path) -> dict:
    """Retain ordinary recursive-call observations without confusing missing inlined workers with a tail-call proof."""
    text = path.read_text(); out = {}
    for name in ['concat-size','concat-copy','repeat-double']:
        matches = re.findall(r'define[^\n]*@"[^"\n]*Data\.String\.'+re.escape(name)+r'[^"\n]*"[^\n]*\{\n(.*?)\n\}',text,re.S)
        recursive = [line.strip() for body in matches for line in body.splitlines() if 'call ' in line and 'Data.String.'+name in line]
        out[name] = {'emitted_workers':len(matches),'recursive_calls':recursive,'all_observed_recursive_calls_tail':all('musttail call' in line or 'tail call' in line for line in recursive)}
    return out


def main() -> None:
    """Compile explicit source epochs and run bounded normal and poisoned ownership observations."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--compiler',type=Path,required=True)
    parser.add_argument('--module-source',type=Path,default=ROOT/'compiler/prelude/Data/String.cinch')
    parser.add_argument('--epoch',choices=['source-override','fresh-embedded'],default='source-override')
    parser.add_argument('--artifacts',type=Path,required=True)
    parser.add_argument('--output',type=Path,required=True)
    parser.add_argument('--opt-level',type=int,choices=[0,2],default=0)
    parser.add_argument('--groups',nargs='+',choices=['repeats','concats','regression','owners'],default=['repeats','concats','regression','owners'])
    parser.add_argument('--first',type=int,default=0)
    parser.add_argument('--limit',type=int)
    args = parser.parse_args()
    fixture_path=ROOT/'compiler/tests/fixtures/string-construction-vectors.json'
    fixture=json.loads(fixture_path.read_text())
    report={'compiler_sha256':digest(args.compiler),'epoch':args.epoch,'string_source_sha256':digest(args.module_source),'fixture_sha256':digest(fixture_path),'opt_level':args.opt_level,'native_base_validation_claimed':False,'results':[]}
    for group in args.groups:
        project=args.artifacts/group;project.mkdir(parents=True,exist_ok=True)
        rows=fixture.get(group,[None])[args.first:]
        if args.limit is not None: rows=rows[:args.limit]
        if group=='owners': expressions=['owners-case ()']
        elif group=='regression':
            repeat=next(r for r in fixture['repeats'] if r['text']=='x' and r['count']==65536)
            concat=next(r for r in fixture['concats'] if r['pieces']==['x'] and r['cycles']==65536)
            expressions=checks('repeats',[repeat])+checks('concats',[concat]);rows=[repeat,concat]
        else: expressions=checks(group,rows)
        source=consumer(expressions);entry=project/'Main.cinch';entry.write_text(source)
        for name,path in source_closure(source).items(): shutil.copyfile(path,project/(name+'.cinch'))
        override=project/'Data.String.cinch'
        if args.epoch=='source-override': override.write_text('-- Explicit constructor source-override receipt v1.\n'+args.module_source.read_text())
        elif override.exists(): override.unlink()
        exe=(project/'consumer').resolve()
        run([str(args.compiler.resolve()),'build',str(entry.resolve()),'-o',str(exe),'--opt-level',str(args.opt_level),'--rc-debug'])
        receipt=verify_receipt(project,override if args.epoch=='source-override' else args.module_source)
        if args.epoch=='source-override': assert receipt['source_hash']!=source_hash(args.module_source.read_bytes())
        ownership=[]
        for debug in [False,True]:
            result=run([str(exe)],env={**os.environ,'CINCH_RC_DEBUG':'1' if debug else '0','CINCH_LEAK_CHECK':'1','CINCH_HEAP_LIMIT':'256M' if debug else '32M','CINCH_STACK_SIZE':'8M'})
            assert 'leaked=0' in result.stderr,result.stderr
            ownership.append({'debug':debug,'zero_leaks':True})
        report['results'].append({'group':group,'cases':len(rows),'epoch_receipt':receipt,'entry_sha256':digest(entry),'executable_sha256':digest(exe),'llvm_sha256':digest(exe.with_suffix('.ll')),'tail_receipt':tail_receipt(exe.with_suffix('.ll')),'ownership':ownership})
        print(f'{group}: {len(rows)} cases, exact source epoch and both ownership modes passed',flush=True)
    args.output.parent.mkdir(parents=True,exist_ok=True)
    args.output.write_text(json.dumps(report,indent=2)+'\n')

if __name__=='__main__': main()
