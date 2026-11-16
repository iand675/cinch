#!/usr/bin/env python3
"""Cross-check independent byte-span fixtures and exercise both source-distributed frontends."""
from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys

from bench_stdlib import source_closure

ROOT = Path(__file__).resolve().parents[1]
FIXTURE = ROOT / 'compiler/tests/fixtures/byte-view-vectors.json'


def digest(path: Path) -> str:
    """Identify every exact compiler, dependency, consumer and fixture epoch."""
    return hashlib.sha256(path.read_bytes()).hexdigest()


def input_bytes(mode: int, size: int) -> bytes:
    """Build the independent recipes without calling the compiled library."""
    if mode == 0:
        return bytes(size)
    if mode == 1:
        return b'\xff' * size
    if mode == 2:
        return bytes((i * 37 + 11) % 256 for i in range(size))
    if mode == 3:
        return bytes(i % 2 for i in range(size))
    if mode == 4:
        return b'a' * max(0, size - 1) + (b'b' if size else b'')
    if mode == 5:
        pattern = bytes.fromhex('f09f9880eda080c2a98000ff')
    elif mode == 6:
        pattern = b'ababcaba'
    else:
        return bytes(i % 251 for i in range(size))
    return (pattern * ((size + len(pattern) - 1) // len(pattern)))[:size]


def crosscheck(rows: list[dict]) -> None:
    """Compare all SDK answers against a second independent byte/Unicode implementation."""
    for i, row in enumerate(rows):
        raw = input_bytes(row['mode'], row['size'])
        assert hashlib.sha256(raw).hexdigest() == row['raw_sha256'], i
        width = row['needle_size']
        at = row['needle_source']
        needle = raw[at:at + width] if at >= 0 else input_bytes(row['needle_mode'], width)
        assert needle.hex() == row['needle_hex'], i
        view = raw[row['start']:row['end']]
        # Surrogate escape preserves each malformed byte as one independent scalar step.
        count = view.count(needle) if needle else len(view.decode('utf-8', errors='surrogateescape')) + 1
        expected = {'first': view.find(needle), 'last': view.rfind(needle), 'count': count,
                    'prefix': view.startswith(needle), 'suffix': view.endswith(needle),
                    'compare': (view > needle) - (view < needle), 'equal': view == needle}
        assert all(row[key] == value for key, value in expected.items()), (i, row, expected)


def expression(row: dict) -> str:
    """Render concrete typed public observations without embedding expected input byte literals."""
    fields = ['mode', 'size', 'start', 'end', 'needle_mode', 'needle_size', 'needle_source',
              'first', 'last', 'count', 'prefix', 'suffix', 'compare', 'equal']
    def literal(value: object) -> str:
        if isinstance(value, bool):
            return 'True' if value else 'False'
        return f'({value})' if int(value) < 0 else str(value)
    return 'view-case ' + ' '.join(literal(row[name]) for name in fields)


def consumer(expressions: list[str], width: int) -> str:
    """Dispatch bounded process groups so checked-RC quarantine is never a cumulative test artifact."""
    source = (ROOT / 'compiler/tests/fixtures/byte-view-consumer.cinchpart').read_text()
    dispatch = 'False'
    for first in reversed(range(0, len(expressions), width)):
        number = first // width
        source += f'\n  -- | Observe independent byte-view batch {number}.\n  checks{number} :: Unit -> Bool\n  checks{number} unit = all [\n'
        source += ',\n'.join(f'    ({value})' for value in expressions[first:first + width]) + '\n    ]\n'
        dispatch = f'if batch == "{number}" then checks{number} () else ' + dispatch
    source += '\n  -- | Select one bounded independent ownership process.\n  selected :: String -> Bool\n  selected batch = ' + dispatch
    return source + '\n  -- | Run an exact independently selected fixture batch.\n  main :: Unit ->{IO r} Int\n  main unit = with (io = newIO ()) in case process-args () of\n    Ok (Cons batch Nil) -> if selected batch then 0 else 1\n    other -> 2\n'


def run(command: list[str], **kwargs) -> subprocess.CompletedProcess:
    """Keep exact diagnostic text whenever a dependent build or observation fails."""
    result = subprocess.run(command, capture_output=True, text=True, **kwargs)
    if result.returncode:
        raise RuntimeError(f'{command}\nexit={result.returncode}\n{result.stdout}\n{result.stderr}')
    return result


def main() -> None:
    """Compile the actual module closure and observe exact query, mutation and owner results."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--crosscheck-only', action='store_true')
    parser.add_argument('--compiler', type=Path)
    parser.add_argument('--native-interfaces', type=Path)
    parser.add_argument('--artifacts', type=Path)
    parser.add_argument('--output', type=Path)
    parser.add_argument('--opt-level', default='2', choices=['0', '2'])
    parser.add_argument('--groups', nargs='+', default=['rows', 'long', 'owners'], choices=['rows', 'long', 'owners'])
    parser.add_argument('--first', type=int, default=0)
    parser.add_argument('--limit', type=int)
    parser.add_argument('--batch-size', type=int, default=32)
    parser.add_argument('--reuse-artifacts', type=Path)
    parser.add_argument('--reuse-receipt', type=Path)
    args = parser.parse_args()
    rows = json.loads(FIXTURE.read_text())['rows']
    crosscheck(rows)
    print(f'Independent SDK/Python agreement: {len(rows)} rows', flush=True)
    if args.crosscheck_only:
        return
    assert args.compiler and args.artifacts and args.output
    report = {'frontend_sha256': digest(args.compiler), 'fixture_sha256': digest(FIXTURE),
              'source_distributed_only': True, 'embedded_base_change_claimed': False,
              'compiler_main_thread_stack_bytes': 67108864, 'program_stack_bytes': 8388608,
              'opt_level': args.opt_level, 'results': []}
    if args.native_interfaces:
        report['native_interfaces_sha256'] = digest(args.native_interfaces)
        report['native_compiler_sha256'] = digest(args.compiler.parent / 'cinch')
    for group in args.groups:
        selected = [row for row in rows if (row['size'] > 129) == (group == 'long')][args.first:]
        if args.limit is not None:
            selected = selected[:args.limit]
        expressions = [expression(row) for row in selected] if group != 'owners' else ['owners-case ()'] * 10
        width = args.batch_size if group == 'rows' else 1
        work = args.artifacts / group
        work.mkdir(parents=True, exist_ok=True)
        entry = work / 'Main.cinch'
        text = consumer(expressions, width)
        entry.write_text(text)
        deps = source_closure(text)
        for name, path in deps.items():
            shutil.copyfile(path, work / (name + '.cinch'))
        exe = (work / 'consumer').resolve()
        if args.native_interfaces:
            bundle = work / 'Bundle.cinch'
            bundle.write_text('\n'.join(p.read_text() for p in reversed(list(deps.values()))) + '\n' + text)
            command = [sys.executable, str(args.compiler.resolve()), '--rc-debug', '--compile-only',
                       '--interfaces', str(args.native_interfaces.resolve()), str(bundle.resolve()),
                       str(exe.with_suffix('.ll')), str(exe), str(exe.with_suffix('.link.txt'))]
        else:
            command = [str(args.compiler.resolve()), 'build', str(entry.resolve()), '-o', str(exe),
                       '--opt-level', args.opt_level, '--rc-debug']
        if args.reuse_artifacts or args.reuse_receipt:
            assert args.native_interfaces and args.reuse_artifacts and args.reuse_receipt and args.opt_level == '2'
            previous = json.loads(args.reuse_receipt.read_text())
            assert previous['frontend_sha256'] == report['frontend_sha256']
            assert previous['native_interfaces_sha256'] == report['native_interfaces_sha256']
            before = next(result for result in previous['results'] if result['group'] == group)
            assert before['entry_sha256'] == digest(entry)
            assert before['dependencies'] == {name: digest(path) for name, path in deps.items()}
            original = args.reuse_artifacts / group / 'consumer.ll'
            assert digest(original) == before['llvm_sha256']
            shutil.copyfile(original, exe.with_suffix('.ll'))
            report['reused_native_build_receipt_sha256'] = digest(args.reuse_receipt)
        else:
            try:
                build = run(command)
            except RuntimeError as error:
                diagnostics = exe.with_suffix('.ll.diagnostics')
                raise RuntimeError(str(error) + '\n' + (diagnostics.read_text() if diagnostics.exists() else '')) from error
            (work / 'compile.log').write_text(build.stdout + build.stderr)
        if args.native_interfaces and args.opt_level == '2':
            configuration = json.loads((args.compiler.parent / 'objects.json').read_text())
            runtime = Path(configuration['metadata']).with_name('libcinchruntime.a')
            link = [os.environ.get('CINCH_CLANG', 'clang'), '-O2', str(exe.with_suffix('.ll')),
                    configuration['object'], str(runtime), '-o', str(exe), '-ldl', '-lpthread', '-lm']
            linked = run(link)
            (work / 'optimized-link.log').write_text(linked.stdout + linked.stderr)
        observations = []
        for debug in [False, True]:
            for batch in range((len(expressions) + width - 1) // width):
                observed = run([str(exe), str(batch)], env={**os.environ, 'CINCH_RC_DEBUG': str(int(debug)),
                               'CINCH_LEAK_CHECK': '1', 'CINCH_HEAP_LIMIT': '256M' if debug else '32M',
                               'CINCH_STACK_SIZE': '8M'})
                assert 'leaked=0' in observed.stderr, observed.stderr
                observations.append({'debug': debug, 'batch': batch, 'zero_leaks': True})
        report['results'].append({'group': group, 'cases': len(expressions), 'batch_width': width,
                                  'entry_sha256': digest(entry), 'executable_sha256': digest(exe),
                                  'llvm_sha256': digest(exe.with_suffix('.ll')),
                                  'dependencies': {name: digest(path) for name, path in deps.items()},
                                  'ownership': observations})
        print(f'{group}: {len(expressions)} observations, normal+poison zero leaks', flush=True)
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(report, indent=2) + '\n')


if __name__ == '__main__':
    main()
