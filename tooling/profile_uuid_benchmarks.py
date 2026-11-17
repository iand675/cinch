#!/usr/bin/env python3
"""Compare ordinary-RC allocation counters from sealed UUID binaries without retaining timing observations."""
from __future__ import annotations
import argparse
import json
import os
from pathlib import Path
import re
import subprocess
from validate_uuid import digest


def checked_identity(directory: Path, generation: bool = False) -> dict:
    """Verify existing executable, IR, assembly and sources without rebuilding any input."""
    identity = json.loads((directory / 'identity.json').read_text())
    assert identity['preflight_passed']
    variants = {'candidate': identity} if generation else identity['variants']
    for variant, record in variants.items():
        project = directory if generation else directory / variant
        for key, name in [('executable_sha256', 'bench'), ('llvm_sha256', 'bench.ll'), ('assembly_sha256', 'bench.s')]:
            assert digest(project / name) == record[key]
        for name, sha in record['sources'].items():
            assert digest(project / name) == sha
    return identity


def observe(executable: Path, arguments: list[str], generation: bool, reference: bool) -> dict:
    """Discard elapsed time, normalize argv0, and retain exact result and ordinary memory diagnostics."""
    environment = {**os.environ, 'CINCH_RC_DEBUG': '0', 'CINCH_LEAK_CHECK': '1',
                   'CINCH_ALLOC_STATS': '1', 'CINCH_HEAP_LIMIT': '32M', 'CINCH_STACK_SIZE': '8M'}
    output = subprocess.run(['uuid-bench', *arguments], executable=str(executable), env=environment,
                            capture_output=True, text=True, timeout=30)
    assert output.returncode == 0, (executable, arguments, output.stdout, output.stderr)
    fields = list(map(int, output.stdout.split()))
    assert len(fields) == (3 if generation else 2)
    assert fields[0] >= 0
    row = {'executable': str(executable), 'arguments': arguments, 'argv0': 'uuid-bench',
           'fingerprint': fields[-1], 'diagnostics': output.stderr}
    if generation:
        row['valid'] = fields[1]
        assert fields[1] == int(arguments[-1])
    if not reference:
        assert 'cinch-leak: live=0 immortal=0 leaked=0' in output.stderr
        match = re.search(r'^cinch-alloc: (.*)$', output.stderr, re.M)
        assert match, output.stderr
        row['allocations'] = {key: int(value) for key, value in re.findall(r'([\w-]+)=(\d+)', match.group(1))}
        assert row['allocations']['live'] == 0
    return row


def differences(rows: list[dict]) -> dict:
    """Expose raw baseline differences; reporting/argument work prevents an exact per-operation interpretation."""
    samples = {row['iterations']: row['allocations'] for row in rows}
    metrics = ('blocks', 'bytes', 'header-blocks', 'header-bytes', 'freed')
    return {metric: {'one_minus_zero': samples[1][metric] - samples[0][metric],
                     'twenty_minus_zero': samples[20][metric] - samples[0][metric],
                     'twenty_minus_one': samples[20][metric] - samples[1][metric],
                     'twenty_minus_one_over_nineteen': (samples[20][metric] - samples[1][metric]) / 19}
            for metric in metrics}


def main() -> None:
    """Collect zero/one/twenty profiles with result checks and no performance-measurement claim."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--original', type=Path, required=True)
    parser.add_argument('--direct', type=Path, required=True)
    parser.add_argument('--generation', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--cpu', type=int, required=True)
    args = parser.parse_args()
    os.sched_setaffinity(0, {args.cpu})
    original, direct, generation = args.original.resolve(), args.direct.resolve(), args.generation.resolve()
    one, two, generated = checked_identity(original), checked_identity(direct), checked_identity(generation, True)
    assert (one['compiler_sha256'], one['runtime_sha256']) == (two['compiler_sha256'], two['runtime_sha256']) == (generated['compiler_sha256'], generated['runtime_sha256'])
    reference = original / 'reference'
    assert digest(reference) == one['reference_executable_sha256'] == two['reference_executable_sha256'] == generated['reference_executable_sha256']
    groups, all_runs = [], []
    for workload in ('parse', 'format', 'append', 'compare'):
        for size in (0, 64, 4096):
            cases = {}
            for count in (0, 1, 20):
                fingerprints = set()
                for family, directory in (('original', original), ('direct', direct)):
                    for variant in ('simd', 'scalar'):
                        label = family + '-' + variant
                        row = observe(directory / variant / 'bench', [workload, str(size), str(count), '0'], False, False)
                        row.update(variant=label, workload=workload, size=size, iterations=count)
                        cases.setdefault(label, []).append(row)
                        fingerprints.add(row['fingerprint'])
                        all_runs.append(row)
                row = observe(reference, [workload, str(size), str(count), '0'], False, True)
                row.update(variant='reference', workload=workload, size=size, iterations=count)
                fingerprints.add(row['fingerprint'])
                all_runs.append(row)
                assert len(fingerprints) == 1, (workload, size, count, fingerprints)
            groups.append({'workload': workload, 'size': size,
                           'deltas': {variant: differences(rows) for variant, rows in cases.items()}})
    for mode in ('default', 'v4', 'v7'):
        rows = []
        for count in (0, 1, 20):
            row = observe(generation / 'bench', [mode, str(count)], True, False)
            row.update(variant='generation', workload=mode, iterations=count)
            rows.append(row)
            all_runs.append(row)
            reference_row = observe(reference, ['generate', mode, str(count)], True, True)
            reference_row.update(variant='reference', workload=mode, iterations=count)
            all_runs.append(reference_row)
        groups.append({'workload': mode, 'deltas': {'generation': differences(rows)}})
    report = {'kind': 'UUID-ordinary-RC-allocation-diagnostics', 'timing_claim': False,
              'elapsed_values_discarded': True, 'compares_random_payloads': False,
              'compiler_sha256': one['compiler_sha256'], 'runtime_sha256': one['runtime_sha256'],
              'identity_sha256': {str(p): digest(p / 'identity.json') for p in (original, direct, generation)},
              'cpu_affinity': [args.cpu], 'argv0': 'uuid-bench', 'memory': 'normal32MiB/8MiB,RCdebug=0',
              'caveat': 'Raw counts include reporting and argument conversion. Baseline differences are diagnostic estimates, not exact dynamic operation allocation counts. Peak/live/immortal counters are not subtracted.',
              'runs': all_runs, 'groups': groups, 'status': 'passed'}
    args.output.parent.mkdir(parents=True, exist_ok=True)
    assert not args.output.exists(), 'Preserve every prior diagnostic report'
    args.output.write_text(json.dumps(report, indent=2) + '\n')
    print(f'{len(all_runs)} existing-binary allocation observations passed; elapsed values discarded')


if __name__ == '__main__':
    main()
