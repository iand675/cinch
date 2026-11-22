#!/usr/bin/env python3
"""Check explicit scanning-join imports against an identified embedded compiler family."""
from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil

from bench_stdlib import source_closure
from validate_string_construction import digest, run

ROOT = Path(__file__).resolve().parents[1]


def main() -> None:
    """Compile the unchanged consumer closure and retain exact source and ownership receipts."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--compiler', type=Path, required=True)
    parser.add_argument('--artifacts', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--label', required=True)
    parser.add_argument('--runtime', type=Path, help='Link and record an explicit runtime archive')
    args = parser.parse_args()
    project = args.artifacts.resolve()
    project.mkdir(parents=True, exist_ok=False)
    fixture = ROOT / 'seed/tests/fixtures/string-join-namespace-consumer.cinch'
    text = fixture.read_text()
    sources = source_closure(text)
    assert 'Data.String' not in sources, 'namespace clients must use the compiler embedded String'
    for name, source in sources.items():
        shutil.copyfile(source, project / (name + '.cinch'))
    entry = project / 'Main.cinch'
    entry.write_text(text)
    executable = project / 'consumer'
    command = [str(args.compiler.resolve()), 'build', str(entry), '-o',
               str(executable), '--opt-level', '2']
    if args.runtime:
        command += ['--runtime-archive', str(args.runtime.resolve())]
    run(command)
    assert not (project / 'Data.String.cinch').exists(), 'embedded String source shadow'
    from pack_interfaces import PackedInterfaces, canonical
    package = project / "build/Main.cpk"
    [string_contract] = [doc for name, _, doc in PackedInterfaces(package.read_bytes()).documents if name == "Data.String"]
    profiles = []
    for poisoned in [False, True]:
        environment = {**os.environ, 'CINCH_HEAP_LIMIT': '256M' if poisoned else '32M',
                       'CINCH_STACK_SIZE': '8M', 'CINCH_LEAK_CHECK': '1',
                       'CINCH_RC_DEBUG': '1' if poisoned else '0'}
        result = run([str(executable)], env=environment)
        assert 'leaked=0' in result.stderr, result.stderr
        profiles.append({'poisoned': poisoned, 'zero_leaks': True,
                         'stderr': result.stderr})
    report = {'label': args.label, 'compiler_sha256': digest(args.compiler),
              'runtime_sha256': digest(args.runtime) if args.runtime else None,
              'fixture_sha256': digest(fixture), 'command': command,
              'source_sha256': {name: digest(source) for name, source in sources.items()},
              'executable_sha256': digest(executable),
              'llvm_sha256': digest(executable.with_suffix('.ll')),
              'string_interface_sha256': hashlib.sha256(canonical(string_contract)).hexdigest(),
              'interface_package_sha256': digest(package),
              'string_source_hash': string_contract['source_hash'],
              'profiles': profiles, 'source_shadow_used': False}
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(report, indent=2) + '\n')
    print(f'{args.label}: all five importer namespaces and both ownership modes passed')


if __name__ == '__main__':
    main()
