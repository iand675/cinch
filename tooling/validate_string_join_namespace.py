#!/usr/bin/env python3
"""Check explicit scanning-join imports against an identified embedded compiler family."""
from __future__ import annotations

import argparse
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
    args = parser.parse_args()
    project = args.artifacts.resolve()
    project.mkdir(parents=True, exist_ok=False)
    fixture = ROOT / 'compiler/tests/fixtures/string-join-namespace-consumer.cinch'
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
    run(command)
    assert not (project / 'Data.String.cinch').exists(), 'embedded String source shadow'
    string_interface = project / 'build/Data.String.cinchif'
    string_contract = json.loads(string_interface.read_text())
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
              'fixture_sha256': digest(fixture), 'command': command,
              'source_sha256': {name: digest(source) for name, source in sources.items()},
              'executable_sha256': digest(executable),
              'llvm_sha256': digest(executable.with_suffix('.ll')),
              'string_interface_sha256': digest(string_interface),
              'string_source_hash': string_contract['source_hash'],
              'profiles': profiles, 'source_shadow_used': False}
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(report, indent=2) + '\n')
    print(f'{args.label}: all five importer namespaces and both ownership modes passed')


if __name__ == '__main__':
    main()
