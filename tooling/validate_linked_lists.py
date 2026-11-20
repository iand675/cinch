#!/usr/bin/env python3
"""Validate complete linked-list traces with independent inputs and bounded normal/poison runs."""
from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import resource
import subprocess

from generate_linked_list_cases import source

ROOT = Path(__file__).resolve().parents[1]
MODULES = {'Data.Mutable.LinkedList': ROOT / 'seed/prelude/Data/Mutable/LinkedList.cinch',
           'Data.Mutable.Vec': ROOT / 'seed/prelude/Data/Mutable/Vec.cinch'}


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
    parser.add_argument('--runtime', type=Path, help='Explicit matched Rust runtime archive')
    parser.add_argument('--artifacts', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--cpu', type=int, default=4)
    parser.add_argument('--cases', type=int, nargs='+', default=list(range(8)))
    parser.add_argument('--modes', choices=['trace', 'control', 'capacity', 'growth'], nargs='+', default=['trace', 'control', 'capacity', 'growth'])
    args = parser.parse_args()
    os.sched_setaffinity(0, {args.cpu})
    args.compiler = args.compiler.resolve()
    if args.frontend == 'native' and not args.interfaces:
        parser.error('native consumers require the exact frozen interface manifest')
    if args.frontend == 'rust' and not args.runtime:
        parser.error('Rust consumers require an explicit matched --runtime archive')
    if args.frontend == 'native' and args.runtime:
        parser.error('Native runtime selection belongs to its exact frontend wrapper')
    frozen = {name: path.read_bytes() for name, path in MODULES.items()}
    jobs = {f'trace-{index}': source(index) for index in args.cases if 'trace' in args.modes}
    if 'control' in args.modes:
        jobs['control'] = source(0, 'control')
    for mode in ['capacity', 'growth']:
        if mode in args.modes:
            jobs[mode] = source(0, mode)
    report = {'description': __doc__, 'frontend': args.frontend, 'compiler_sha256': digest(args.compiler),
              'runner_sha256': digest(Path(__file__)), 'fixture_sha256': digest(ROOT / 'seed/tests/fixtures/linked-list-vectors.json'),
              'sources': {name: hashlib.sha256(contents).hexdigest() for name, contents in frozen.items()},
              'compiler_host_stack': '64M', 'cpu_affinity': [args.cpu], 'suites': []}
    if args.frontend == 'native':
        report.update(native_compiler_sha256=digest(args.compiler.parent / 'cinch'),
                      interfaces_sha256=digest(args.interfaces.resolve()))
    if args.runtime:
        args.runtime = args.runtime.resolve()
        report['runtime_sha256'] = digest(args.runtime)
        provenance = args.compiler.parent / 'provenance.json'
        if provenance.exists():
            frozen_producer = json.loads(provenance.read_text())
            assert frozen_producer['compiler_sha256'] == digest(args.compiler)
            assert frozen_producer['runtime_sha256'] == digest(args.runtime)
            report['frozen_compiler_provenance'] = frozen_producer
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.artifacts.mkdir(parents=True, exist_ok=True)
    (args.artifacts / 'runner.py').write_bytes(Path(__file__).read_bytes())
    for label, consumer in jobs.items():
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
        for poisoned in [False, True]:
            environment = {**os.environ, 'CINCH_HEAP_LIMIT': '256M' if poisoned else '32M', 'CINCH_STACK_SIZE': '8M',
                           'CINCH_RC_DEBUG': '1' if poisoned else '0', 'CINCH_LEAK_CHECK': '1', 'CINCH_ALLOC_STATS': '1'}
            result = subprocess.run([str(executable)], cwd=directory, env=environment, capture_output=True, text=True)
            (directory / ('poisoned.log' if poisoned else 'ordinary.log')).write_text(result.stdout + result.stderr)
            assert result.returncode == 0, (label, poisoned, result.returncode, result.stdout, result.stderr)
            assert 'cinch-leak: live=0 immortal=0 leaked=0' in result.stderr, (label, poisoned, result.stderr)
            entry['runs'].append({'poisoned_free': poisoned, 'heap': environment['CINCH_HEAP_LIMIT'], 'stack': '8M',
                                  'exit': result.returncode, 'stdout': result.stdout, 'diagnostics': result.stderr, 'live': 0, 'leaked': 0})
        report['suites'].append(entry)
        args.output.write_text(json.dumps(report, indent=2) + '\n')
        print(f'{args.frontend} {label}: normal and poisoned runs passed with zero leaks', flush=True)


if __name__ == '__main__':
    main()
