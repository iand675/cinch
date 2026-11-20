#!/usr/bin/env python3
"""Validate fresh owned-byte construction against independent raw-byte observations."""
from __future__ import annotations
import argparse
import json
import os
from pathlib import Path
import shutil
import sys
from bench_stdlib import source_closure
from validate_byte_search import digest, run

ROOT = Path(__file__).resolve().parents[1]


def consumer(expressions:list[str])->str:
    """Dispatch independent process batches with a fixed bounded poison quarantine."""
    source=(ROOT/'seed/tests/fixtures/byte-build-consumer.cinchpart').read_text()
    dispatch='False'
    for first in reversed(range(0,len(expressions),8)):
        number=first//8;name=f'checks{number}'
        source+=f'\n  -- | Observe independent public construction fixture batch {number}.\n  {name} :: Unit -> Bool\n  {name} unit = all [\n'+',\n'.join(f'    ({value})' for value in expressions[first:first+8])+'\n    ]\n'
        dispatch=f'if batch == "{number}" then {name} () else '+dispatch
    return source+'\n  -- | Execute only the selected bounded poison-quarantine batch.\n  selected :: String -> Bool\n  selected batch = '+dispatch+'\n  -- | Observe one independent process batch.\n  main :: Unit ->{IO r} Int\n  main unit = with (io = newIO ()) in case process-args () of\n    Ok (Cons batch Nil) -> if selected batch then 0 else 1\n    other -> 2\n'


def checks(group: str, rows: list[dict]) -> list[str]:
    """Render exact binary inputs and complete output digests without using text conversion."""
    if group == 'repeats':
        return [f'repeat-case "{r["input"]}" {r["count"]} {r["bytes"]} "{r["hex"]}" "{r["sha256"]}"' for r in rows]
    return [f'join-case [{",".join(json.dumps(value) for value in r["pattern"])}] {r["cycles"]} "{r["separator"]}" {r["bytes"]} "{r["hex"]}" "{r["sha256"]}"' for r in rows]


def main() -> None:
    """Compile the exact source-only module and check ordinary and poisoned reclamation."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--compiler', type=Path, required=True)
    parser.add_argument('--native-interfaces', type=Path)
    parser.add_argument('--artifacts', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--groups', nargs='+', choices=['repeats', 'joins', 'owners'], default=['repeats', 'joins', 'owners'])
    parser.add_argument('--opt-level', type=int, choices=[0, 2], default=2)
    parser.add_argument('--first', type=int, default=0)
    parser.add_argument('--limit', type=int)
    args = parser.parse_args()
    fixture_path = ROOT / 'seed/tests/fixtures/byte-build-vectors.json'
    fixture = json.loads(fixture_path.read_text())
    compiler = args.compiler.parent / 'cinch' if args.native_interfaces else args.compiler
    report = {'compiler_sha256': digest(compiler), 'frontend_sha256': digest(args.compiler), 'source_sha256': digest(ROOT / 'seed/prelude/Data/Bytes/Build.cinch'), 'fixture_sha256': digest(fixture_path), 'opt_level': None if args.native_interfaces else args.opt_level, 'source_distributed_module': True, 'changed_embedded_constructor_validation_claimed': False, 'native_base_interfaces_sha256': digest(args.native_interfaces) if args.native_interfaces else None, 'results': []}
    for group in args.groups:
        project = args.artifacts / group
        project.mkdir(parents=True, exist_ok=True)
        rows = fixture.get(group, [None])[args.first:]
        if args.limit is not None:
            rows = rows[:args.limit]
        expressions = ['owners-case ()'] if group == 'owners' else checks(group, rows)
        text = consumer(expressions)
        entry = project / 'Main.cinch'
        entry.write_text(text)
        deps = source_closure(text)
        for name, path in deps.items():
            shutil.copyfile(path, project / (name + '.cinch'))
        exe = (project / 'consumer').resolve()
        if args.native_interfaces:
            bundle = project / 'Bundle.cinch'
            bundle.write_text('\n'.join(path.read_text() for path in reversed(list(deps.values()))) + '\n' + text)
            command = [sys.executable, str(args.compiler.resolve()), '--rc-debug', '--compile-only', '--interfaces', str(args.native_interfaces.resolve()), str(bundle.resolve()), str(exe.with_suffix('.ll')), str(exe), str(exe.with_suffix('.link.txt'))]
        else:
            command = [str(args.compiler.resolve()), 'build', str(entry.resolve()), '-o', str(exe), '--opt-level', str(args.opt_level), '--rc-debug']
        try:
            run(command)
        except RuntimeError as error:
            diagnostics = exe.with_suffix('.ll.diagnostics')
            raise RuntimeError(str(error) + '\n' + (diagnostics.read_text() if diagnostics.exists() else '')) from error
        ownership = []
        for debug in [False, True]:
          for batch in range((len(expressions) + 7) // 8):
            result = run([str(exe), str(batch)], env={**os.environ, 'CINCH_RC_DEBUG': '1' if debug else '0', 'CINCH_LEAK_CHECK': '1', 'CINCH_HEAP_LIMIT': '256M' if debug else '32M', 'CINCH_STACK_SIZE': '8M'})
            assert 'leaked=0' in result.stderr, result.stderr
            ownership.append({'debug': debug, 'batch': batch, 'zero_leaks': True})
        report['results'].append({'group': group, 'cases': len(rows), 'entry_sha256': digest(entry), 'executable_sha256': digest(exe), 'llvm_sha256': digest(exe.with_suffix('.ll')), 'ownership': ownership})
        print(f'{group}: {len(rows)} cases, both ownership modes passed', flush=True)
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(report, indent=2) + '\n')


if __name__ == '__main__':
    main()
