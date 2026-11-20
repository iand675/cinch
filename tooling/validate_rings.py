#!/usr/bin/env python3
"""Validate complete circular-sequence traces with independent inputs and bounded normal/poison runs."""
from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import resource
import subprocess

from generate_ring_cases import source

ROOT = Path(__file__).resolve().parents[1]
MODULES = {'Data.Mutable.Ring': ROOT / 'seed/prelude/Data/Mutable/Ring.cinch',
           'Data.Mutable.Vec': ROOT / 'seed/prelude/Data/Mutable/Vec.cinch',
           'Data.Mutable.Var': ROOT / 'seed/prelude/Data/Mutable/Var.cinch',
           'Data.Array': ROOT / 'seed/prelude/Data/Array.cinch'}


def digest(path: Path) -> str:
    """Record exact consumed artifacts rather than a checkout revision alone."""
    return hashlib.sha256(path.read_bytes()).hexdigest()


def compiler_stack() -> None:
    """Give only the compiler host enough stack for the deliberately large source observation fixture."""
    soft, hard = resource.getrlimit(resource.RLIMIT_STACK)
    resource.setrlimit(resource.RLIMIT_STACK, (min(64 * 1024 * 1024, hard) if hard != resource.RLIM_INFINITY else 64 * 1024 * 1024, hard))


def main() -> None:
    """Freeze source inputs, compile each consumer and distinguish reclamation from poisoned quarantine."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--frontend', choices=['rust', 'native'], required=True)
    parser.add_argument('--compiler', type=Path, required=True)
    parser.add_argument('--interfaces', type=Path)
    parser.add_argument('--runtime', type=Path, help='Exact matching Rust runtime archive, selected explicitly at link time')
    parser.add_argument('--artifacts', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--source-override', action='append', default=[], metavar='MODULE=PATH', help='Freeze a separate candidate without modifying the selected library source')
    parser.add_argument('--extra-control', type=Path, help='Add a compiler-owned recursive-array runtime control by exact source path')
    parser.add_argument('--cpu', type=int, default=4)
    parser.add_argument('--cases', type=int, nargs='+', default=list(range(8)))
    parser.add_argument('--modes', choices=['trace', 'control', 'kernel', 'window', 'extra'], nargs='+', default=['trace', 'control', 'kernel'])
    parser.add_argument('--window-ranges', nargs='+', metavar='START:END', help='Select bounded poisoned observation windows; defaults to four actions each')
    args = parser.parse_args()
    if 'extra' in args.modes and not args.extra_control:
        parser.error('extra mode requires --extra-control')
    os.sched_setaffinity(0, {args.cpu})
    args.compiler = args.compiler.resolve()
    if args.frontend == 'native' and not args.interfaces:
        parser.error('native consumers require the exact frozen interface manifest')
    if args.frontend == 'rust' and not args.runtime:
        parser.error('Rust consumers require an explicit matched --runtime archive')
    if args.frontend == 'native' and args.runtime:
        parser.error('Native runtime selection belongs to the exact selected frontend wrapper')
    modules = dict(MODULES)
    overrides = {}
    for entry in args.source_override:
        name, separator, value = entry.partition('=')
        if not separator or name not in modules or name in overrides:
            parser.error('source overrides require unique known MODULE=PATH pairs')
        overrides[name] = Path(value).resolve()
        modules[name] = overrides[name]
    frozen = {name: path.read_bytes() for name, path in modules.items()}
    windows = [tuple(map(int, item.split(':'))) for item in args.window_ranges] if args.window_ranges else [(start, start + 4) for start in range(0, 96, 4)]
    if any(len(window) != 2 or not 0 <= window[0] < window[1] <= 96 for window in windows):
        parser.error('observation windows must satisfy 0 <= START < END <= 96')
    jobs = {}
    for index in args.cases:
        if 'trace' in args.modes:
            jobs[f'trace-{index}'] = (source(index), [(False, [])] if index == 7 else [(False, []), (True, [])])
        if 'window' in args.modes or ('trace' in args.modes and index == 7):
            jobs[f'window-{index}'] = (source(index, 'window'), [(True, [str(start), str(end)]) for start, end in windows])
    if 'control' in args.modes:
        jobs['control'] = (source(0, 'control'), [(False, []), (True, [])])
    if 'kernel' in args.modes:
        jobs['offset-kernel'] = (source(0, 'kernel'), [(False, []), (True, [])])
    if args.extra_control:
        jobs['nested-owned-array'] = (args.extra_control.read_text(), [(False, []), (True, [])])
    report = {'description': __doc__, 'frontend': args.frontend, 'compiler_sha256': digest(args.compiler),
              'runner_sha256': digest(Path(__file__)), 'fixture_sha256': digest(ROOT / 'seed/tests/fixtures/ring-vectors.json'),
              'sources': {name: hashlib.sha256(contents).hexdigest() for name, contents in frozen.items()},
              'compiler_host_stack': '64M', 'cpu_affinity': [args.cpu], 'source_overrides': {name: str(path) for name, path in overrides.items()}, 'suites': []}
    if args.frontend == 'native':
        report.update(native_compiler_sha256=digest(args.compiler.parent / 'cinch'),
                      interfaces_sha256=digest(args.interfaces.resolve()))
    if args.runtime:
        args.runtime = args.runtime.resolve()
        report['runtime_sha256'] = digest(args.runtime)
        provenance = args.compiler.parent / 'provenance.json'
        if provenance.exists():
            preserved = json.loads(provenance.read_text())
            assert preserved['compiler_sha256'] == digest(args.compiler), 'frozen compiler mismatch'
            assert preserved['runtime_sha256'] == digest(args.runtime), 'frozen runtime mismatch'
            report['frozen_compiler_provenance'] = preserved
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.artifacts.mkdir(parents=True, exist_ok=True)
    (args.artifacts / 'runner.py').write_bytes(Path(__file__).read_bytes())
    for label, (consumer, profiles) in jobs.items():
        directory = args.artifacts.resolve() / label
        directory.mkdir(parents=True, exist_ok=True)
        for name, contents in frozen.items():
            (directory / f'{name}.cinch').write_bytes(contents)
        (directory / 'Main.cinch').write_text(consumer)
        executable = directory / 'consumer'
        if args.frontend == 'rust':
            command = [str(args.compiler), 'build', str(directory / 'Main.cinch'), '-o', str(executable), '--opt-level', '2', '--runtime-archive', str(args.runtime)]
        else:
            bundle = directory / 'Bundle.cinch'
            bundle.write_text('\n\n'.join(contents.decode() for contents in frozen.values()) + '\n\n' + consumer)
            command = ['python3', str(args.compiler), '--rc-debug', '--compile-only', '--interfaces', str(args.interfaces.resolve()), str(bundle), str(executable.with_suffix('.ll')), str(executable), str(directory / 'link.txt')]
        built = subprocess.run(command, cwd=directory, capture_output=True, text=True, preexec_fn=compiler_stack)
        (directory / 'build.log').write_text(built.stdout + built.stderr)
        diagnostics = executable.with_suffix('.ll.diagnostics')
        assert built.returncode == 0, (label, built.stdout, built.stderr, diagnostics.read_text() if diagnostics.exists() else '')
        entry = {'suite': label, 'consumer_sha256': digest(directory / 'Main.cinch'),
                 'executable_sha256': digest(executable), 'llvm_sha256': digest(executable.with_suffix('.ll')), 'runs': []}
        for poisoned, arguments in profiles:
            environment = {**os.environ, 'CINCH_HEAP_LIMIT': '256M' if poisoned else '32M', 'CINCH_STACK_SIZE': '8M',
                           'CINCH_RC_DEBUG': '1' if poisoned else '0', 'CINCH_LEAK_CHECK': '1', 'CINCH_ALLOC_STATS': '1'}
            result = subprocess.run([str(executable), *arguments], cwd=directory, env=environment, capture_output=True, text=True)
            (directory / (('poisoned' if poisoned else 'ordinary') + ('-' + '-'.join(arguments) if arguments else '') + '.log')).write_text(result.stdout + result.stderr)
            assert result.returncode == 0, (label, poisoned, result.returncode, result.stdout, result.stderr)
            assert 'cinch-leak: live=0 immortal=0 leaked=0' in result.stderr, (label, poisoned, result.stderr)
            entry['runs'].append({'poisoned_free': poisoned, 'heap': environment['CINCH_HEAP_LIMIT'], 'stack': '8M',
                                  'arguments': arguments, 'exhaustive_observation_window': list(map(int, arguments)) if arguments else None, 'exit': result.returncode, 'stdout': result.stdout, 'diagnostics': result.stderr, 'live': 0, 'leaked': 0})
        report['suites'].append(entry)
        args.output.write_text(json.dumps(report, indent=2) + '\n')
        print(f'{args.frontend} {label}: {len(profiles)} selected memory/observation profiles passed with zero leaks', flush=True)


if __name__ == '__main__':
    main()
