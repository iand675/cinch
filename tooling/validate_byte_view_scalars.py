#!/usr/bin/env python3
"""Validate backing-safe relative scalar observations with independent bounded-slice fingerprints."""
from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil
import sys

from bench_stdlib import source_closure
from validate_byte_view import digest, run

ROOT = Path(__file__).resolve().parents[1]
FIXTURE = ROOT / 'seed/tests/fixtures/byte-view-scalar-vectors.json'


def hash_word(value: int, current: int) -> int:
    """Hash an independent complete observation in the reference's fixed wire order."""
    for byte in value.to_bytes(4, 'little'):
        current = ((current ^ byte) * 1099511628211) & ((1 << 64) - 1)
    return current


def crosscheck(rows: list[dict]) -> None:
    """Use a second decoder to verify every scalar, consumed width and boundary status."""
    for row in rows:
        raw = bytes.fromhex(row['input'])
        current = 14695981039346656037
        spans = decodes = 0
        for start in range(len(raw) + 1):
            for end in range(start, len(raw) + 1):
                spans += 1
                current = hash_word(start, current)
                current = hash_word(end, current)
                size = end - start
                for at in [0, 1, size // 2, size, size + 1, len(raw) + 1]:
                    decodes += 1
                    cp, width, status = 65533, 0, 3
                    if at == size:
                        status = 0
                    elif at < size:
                        first = raw[start + at:end].decode('utf-8', 'surrogateescape')[0]
                        if 0xDC80 <= ord(first) <= 0xDCFF:
                            width, status = 1, 2
                        else:
                            cp, width, status = ord(first), len(first.encode('utf-8')), 1
                    for value in [at, cp, width, status]:
                        current = hash_word(value, current)
        assert (current, spans, decodes) == (row['hash'], row['spans'], row['decodes']), row['input']


def consumer(checks: list[str], width: int) -> str:
    """Select bounded groups so each poisoned quarantine fits the unchanged fixed heap cap."""
    text = (ROOT / 'seed/tests/fixtures/byte-view-scalar-consumer.cinchpart').read_text()
    dispatch = 'False'
    for first in reversed(range(0, len(checks), width)):
        i = first // width
        text += f'\n  -- | Observe independent relative scalar group {i}.\n  group{i} :: Unit -> Bool\n  group{i} unit = all [\n'
        text += ',\n'.join(f'    ({value})' for value in checks[first:first + width]) + '\n    ]\n'
        dispatch = f'if index == "{i}" then group{i} () else ' + dispatch
    return text + '\n  -- | Select one bounded complete scalar group.\n  selected :: String -> Bool\n  selected index = ' + dispatch + '\n  -- | Observe the selected independent output and owner group.\n  main :: Unit ->{IO r} Int\n  main unit = with (io = newIO ()) in case process-args () of\n    Ok (Cons index Nil) -> if selected index then 0 else 1\n    other -> 2\n'


def main() -> None:
    """Keep exact source/compiler/runtime epochs and ordinary versus poisoned observations separate."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--compiler', type=Path, required=True)
    parser.add_argument('--runtime-archive', type=Path)
    parser.add_argument('--native-interfaces', type=Path)
    parser.add_argument('--native-objects', type=Path)
    parser.add_argument('--artifacts', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--opt-level', choices=['0', '2'], default='0')
    parser.add_argument('--groups', nargs='+', choices=['rows', 'owners'], default=['rows', 'owners'])
    parser.add_argument('--limit', type=int)
    parser.add_argument('--skip-crosscheck', action='store_true')
    args = parser.parse_args()
    fixture = json.loads(FIXTURE.read_text())
    rows = fixture['rows'][:args.limit]
    if not args.skip_crosscheck:
        crosscheck(rows)
    report = {'frontend_sha256': digest(args.compiler), 'source_sha256': digest(ROOT / 'seed/prelude/Data/Bytes/View.cinch'),
              'fixture_sha256': digest(FIXTURE), 'independent_rows': len(rows), 'source_distributed_only': True,
              'opt_level': args.opt_level, 'compiler_stack_bytes': 67108864, 'program_stack_bytes': 8388608,
              'runtime_sha256': digest(args.runtime_archive) if args.runtime_archive else None, 'results': []}
    if args.native_interfaces:
        assert args.native_objects and args.runtime_archive
        report['native_interfaces_sha256'] = digest(args.native_interfaces)
        report['native_objects_sha256'] = digest(args.native_objects)
        report['native_compiler_sha256'] = digest(args.compiler)
    for group in args.groups:
        checks = [f'scalar-case "{row["input"]}" {row["hash"]}' for row in rows] if group == 'rows' else ['owners-case ()'] * 10
        width = 8 if group == 'rows' else 1
        work = args.artifacts / group
        work.mkdir(parents=True, exist_ok=True)
        text = consumer(checks, width)
        entry = work / 'Main.cinch'
        entry.write_text(text)
        deps = source_closure(text)
        for name, path in deps.items():
            shutil.copyfile(path, work / (name + '.cinch'))
        exe = (work / 'consumer').resolve()
        if args.native_interfaces:
            bundle = work / 'Bundle.cinch'
            bundle.write_text('\n'.join(path.read_text() for path in reversed(list(deps.values()))) + '\n' + text)
            command = [str(args.compiler.resolve()), '--emit-ir', '--rc-debug', '--interfaces', str(args.native_interfaces.resolve()),
                       '--objects', str(args.native_objects.resolve()), str(bundle.resolve()), str(exe.with_suffix('.ll'))]
        else:
            command = [str(args.compiler.resolve()), 'build', str(entry.resolve()), '-o', str(exe), '--opt-level', args.opt_level, '--rc-debug']
            if args.runtime_archive:
                command += ['--runtime-archive', str(args.runtime_archive.resolve())]
        try:
            compiled = run(command)
        except RuntimeError as error:
            diagnostics = exe.with_suffix('.ll.diagnostics')
            raise RuntimeError(str(error) + '\n' + (diagnostics.read_text() if diagnostics.exists() else '')) from error
        (work / 'compile.log').write_text(compiled.stdout + compiled.stderr)
        if args.native_interfaces:
            configuration = json.loads(args.native_objects.read_text())
            assert digest(Path(configuration['object'])) == configuration['object_sha256']
            command = [os.environ.get('CINCH_CLANG', 'clang'), f'-O{args.opt_level}', str(exe.with_suffix('.ll')),
                       configuration['object'], str(args.runtime_archive.resolve()), '-o', str(exe), '-ldl', '-lpthread', '-lm']
            linked = run(command)
            (work / 'link.log').write_text(linked.stdout + linked.stderr)
            report['native_library_object_sha256'] = digest(Path(configuration['object']))
        ownership = []
        for debug in [False, True]:
            for batch in range((len(checks) + width - 1) // width):
                output = run([str(exe), str(batch)], env={**os.environ, 'CINCH_RC_DEBUG': str(int(debug)),
                             'CINCH_HEAP_LIMIT': '256M' if debug else '32M', 'CINCH_STACK_SIZE': '8M', 'CINCH_LEAK_CHECK': '1'})
                assert 'leaked=0' in output.stderr, output.stderr
                ownership.append({'debug': debug, 'batch': batch, 'zero_leaks': True})
        report['results'].append({'group': group, 'cases': len(checks), 'entry_sha256': digest(entry),
                                  'llvm_sha256': digest(exe.with_suffix('.ll')), 'executable_sha256': digest(exe),
                                  'dependencies': {name: digest(path) for name, path in deps.items()}, 'ownership': ownership})
        print(f'{group}: {len(checks)} complete scalar groups, normal+poison zero leaks', flush=True)
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(report, indent=2) + '\n')


if __name__ == '__main__':
    main()
