#!/usr/bin/env python3
"""Record exact source verdicts for sealed-array positivity, distinguishing baseline observations from gates."""
from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import resource
import subprocess

ROOT = Path(__file__).resolve().parents[1]
FIXTURE = ROOT / 'compiler/tests/fixtures/nested-array-positivity-vectors.json'


def digest(path: Path) -> str:
    """Identify exact compiler, corpus and source artifact bytes."""
    return hashlib.sha256(path.read_bytes()).hexdigest()


def compiler_stack() -> None:
    """Reserve sufficient host stack independently from an executed program's runtime stack."""
    soft, hard = resource.getrlimit(resource.RLIMIT_STACK)
    amount = 64 * 1024 * 1024
    resource.setrlimit(resource.RLIMIT_STACK, (amount if hard == resource.RLIM_INFINITY else min(amount, hard), hard))


def main() -> None:
    """Require precise new verdicts or retain the older compiler's complete unmodified observations."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--frontend', choices=['rust', 'native'], required=True)
    parser.add_argument('--compiler', type=Path, required=True)
    parser.add_argument('--interfaces', type=Path)
    parser.add_argument('--artifacts', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--baseline', action='store_true')
    parser.add_argument('--cpu', type=int, default=4)
    args = parser.parse_args()
    args.compiler = args.compiler.resolve()
    args.artifacts = args.artifacts.resolve()
    args.artifacts.mkdir(parents=True, exist_ok=True)
    if args.frontend == 'native' and not args.interfaces:
        parser.error('Native checks require an exact preserved base-interface manifest')
    rows = json.loads(FIXTURE.read_text())
    report = {'description': __doc__, 'frontend': args.frontend, 'baseline_only': args.baseline,
              'compiler_sha256': digest(args.compiler), 'fixture_sha256': digest(FIXTURE),
              'runner_sha256': digest(Path(__file__)), 'cpu_affinity': [args.cpu],
              'compiler_host_stack': '64M', 'compiler_heap': '256M', 'results': [], 'status': 'running'}
    if args.interfaces:
        args.interfaces = args.interfaces.resolve()
        report['interfaces_sha256'] = digest(args.interfaces)
    environment = {**os.environ, 'CINCH_HEAP_LIMIT': '256M', 'CINCH_STACK_SIZE': '8M',
                   'CINCH_LEAK_CHECK': '1', 'CINCH_RC_DEBUG': '0'}
    args.output.parent.mkdir(parents=True, exist_ok=True)
    for row in rows:
        directory = args.artifacts / row['name']
        directory.mkdir(exist_ok=True)
        source = directory / 'Main.cinch'
        source.write_text(row['source'])
        prefix = directory / 'checked'
        diagnostics = Path(str(prefix) + '.diagnostics')
        diagnostics.unlink(missing_ok=True)
        arguments = (['check', str(source)] if args.frontend == 'rust' else
                     ['--check', '--interfaces', str(args.interfaces), str(source), str(prefix)])
        command = ['taskset', '-c', str(args.cpu), str(args.compiler), *arguments]
        result = subprocess.run(command, env=environment, capture_output=True, text=True, preexec_fn=compiler_stack)
        (directory / 'check.log').write_text(result.stdout + result.stderr)
        detail = diagnostics.read_text() if diagnostics.exists() else result.stdout + result.stderr
        accepted = result.returncode == 0
        valid = accepted == row['accepted'] and (accepted or row['diagnostic'] in detail)
        if args.frontend == 'native':
            assert 'leaked=0' in result.stderr, (row['name'], result.returncode, result.stderr)
            assert result.returncode in [0, 4], (row['name'], result.returncode, detail)
        else:
            assert result.returncode in [0, 1], (row['name'], result.returncode, detail)
        entry = {'name': row['name'], 'source_sha256': digest(source), 'accepted': accepted,
                 'exit': result.returncode, 'diagnostic': detail, 'command': command,
                 'expected_new_acceptance': row['accepted'], 'expected_new_diagnostic': row['diagnostic'],
                 'matches_new_gate': valid}
        report['results'].append(entry)
        report['status'] = 'observing-baseline' if args.baseline else ('running' if valid else 'failed')
        args.output.write_text(json.dumps(report, indent=2) + '\n')
        if not args.baseline:
            assert valid, (row['name'], result.returncode, detail)
    report['status'] = 'baseline-observed' if args.baseline else 'passed'
    args.output.write_text(json.dumps(report, indent=2) + '\n')
    print(f'{args.frontend}: {len(rows)} exact source verdicts recorded; {report["status"]}.', flush=True)


if __name__ == '__main__':
    main()
