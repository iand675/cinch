#!/usr/bin/env python3
"""Compare matched current-compiler UUID baseline/packed pipelines with exact memory proofs and quota telemetry."""
from __future__ import annotations
import argparse
import json
import os
from pathlib import Path
import re
import statistics
import subprocess
import time
from profile_uuid_benchmarks import checked_identity
from validate_uuid import digest


def quota() -> dict:
    """Capture raw quota and throttle counters around the complete protocol without claiming absent data."""
    result = {}
    for name in ('cpu.stat', 'cpu.max'):
        path = Path('/sys/fs/cgroup') / name
        result[name] = path.read_text() if path.is_file() else None
    return result


def save(path: Path, report: dict) -> None:
    """Keep partial verdicts and failures alongside immutable executable/source snapshots."""
    temporary = path.with_name(path.name + '.tmp')
    temporary.write_text(json.dumps(report, indent=2) + '\n')
    temporary.replace(path)


def run(executable: Path, arguments: list[str], measure: bool, poison: bool, reference: bool, deadline: float) -> dict:
    """Normalize process arguments and check output independently of elapsed observations."""
    environment = {**{k: v for k, v in os.environ.items() if not k.startswith('CINCH_')}, 'GOMAXPROCS': '1',
                   'CINCH_HEAP_LIMIT': '256M' if poison else '32M', 'CINCH_STACK_SIZE': '8M',
                   'CINCH_RC_DEBUG': '1' if poison else '0', 'CINCH_LEAK_CHECK': '0' if measure else '1',
                   'CINCH_ALLOC_STATS': '0' if measure or poison else '1'}
    remaining = deadline - time.monotonic()
    assert remaining > 0, 'Protocol deadline reached'
    result = subprocess.run(['uuid-bench', *arguments], executable=str(executable), env=environment,
                            capture_output=True, text=True, timeout=remaining)
    assert result.returncode == 0, (executable, arguments, result.stdout, result.stderr)
    elapsed, checksum = map(int, result.stdout.split())
    assert elapsed >= 0
    row = {'fingerprint': checksum}
    if measure:
        row['elapsed_ns'] = elapsed
    else:
        row['diagnostics'] = result.stderr
        if not reference:
            assert 'cinch-leak: live=0 immortal=0 leaked=0' in result.stderr
            if not poison:
                match = re.search(r'^cinch-alloc: (.*)$', result.stderr, re.M)
                assert match
                row['allocations'] = {k: int(v) for k, v in re.findall(r'([\w-]+)=(\d+)', match.group(1))}
    return row


def main() -> None:
    """Require matching-count normal/poison fingerprints before any separately granted bounded timing run."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('action', choices=('preflight', 'measure'))
    parser.add_argument('--baseline', type=Path, required=True)
    parser.add_argument('--candidate', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--preflight', type=Path)
    parser.add_argument('--cpu', type=int, required=True)
    parser.add_argument('--iterations', type=int, default=4096)
    parser.add_argument('--samples', type=int, default=5)
    parser.add_argument('--budget-seconds', type=int, choices=range(1, 31), default=30)
    parser.add_argument('--background', required=True)
    args = parser.parse_args()
    os.sched_setaffinity(0, {args.cpu})
    assert args.iterations > 0 and args.samples > 0
    baseline, candidate = args.baseline.resolve(), args.candidate.resolve()
    one, two = checked_identity(baseline), checked_identity(candidate)
    assert set(one['variants']) == set(two['variants']) == {'simd'}
    assert (one['compiler_sha256'], one['runtime_sha256']) == (two['compiler_sha256'], two['runtime_sha256'])
    assert one['variants']['simd']['sources']['Main.cinch'] == two['variants']['simd']['sources']['Main.cinch']
    assert one['reference_executable_sha256'] == two['reference_executable_sha256'] == digest(baseline / 'reference')
    for name, sha in one['variants']['simd']['sources'].items():
        if name != 'Data.Uuid.cinch':
            assert two['variants']['simd']['sources'][name] == sha, f'Unmatched closure input: {name}'
    signature = {'baseline_identity_sha256': digest(baseline / 'identity.json'),
                 'candidate_identity_sha256': digest(candidate / 'identity.json'), 'iterations': args.iterations,
                 'runner_sha256': digest(Path(__file__))}
    measuring = args.action == 'measure'
    if measuring:
        assert args.preflight
        proof = json.loads(args.preflight.read_text())
        assert proof['status'] == 'passed' and proof['signature'] == signature and len(proof['rows']) == 10
    args.output.parent.mkdir(parents=True, exist_ok=True)
    assert not args.output.exists()
    report = {'kind': 'matched-current-compiler-UUID-parser-pair', 'status': 'running', 'action': args.action,
              'signature': signature, 'compiler_sha256': one['compiler_sha256'], 'runtime_sha256': one['runtime_sha256'],
              'background': args.background, 'cpu_affinity': [args.cpu], 'argv0': 'uuid-bench', 'GOMAXPROCS': '1',
              'timing_claim': False, 'candidate_selected': False, 'package_complete': False, 'rows': []}
    variants = {'baseline': baseline / 'simd/bench', 'packed': candidate / 'simd/bench', 'reference': baseline / 'reference'}
    conditions = [('parse', 0, p) for p in range(5)] + [('format', 0, 0)] + [('append', size, 0) for size in (0, 64, 4096)] + [('compare', 0, 0)]
    report['quota_before'] = quota()
    started = time.monotonic(); deadline = started + args.budget_seconds
    save(args.output, report)
    try:
        for mode, size, pattern in conditions:
            counts = [args.iterations] if measuring else [0, 1, 20, args.iterations]
            observations, samples = [], {name: [] for name in variants}
            for count in counts:
                checksums = set()
                for sample in range(args.samples + 1 if measuring else 1):
                    names = list(variants); order = names[sample % len(names):] + names[:sample % len(names)]
                    for name in order:
                        for poison in ((False,) if measuring or name == 'reference' else (False, True)):
                            row = run(variants[name], [mode, str(size), str(count), str(pattern)], measuring, poison, name == 'reference', deadline)
                            checksums.add(row['fingerprint'])
                            if measuring and sample:
                                samples[name].append(row['elapsed_ns'])
                            if not measuring:
                                row.update(variant=name, poison=poison, iterations=count); observations.append(row)
                assert len(checksums) == 1, (mode, size, pattern, count, checksums)
            result = {'workload': mode, 'size': size, 'pattern': pattern, 'iterations': args.iterations}
            if measuring:
                result.update(samples_ns=samples, median_ns={name: statistics.median(values) for name, values in samples.items()})
            else:
                result['runs'] = observations
            report['rows'].append(result); save(args.output, report)
        report.update(status='passed', timing_claim=measuring)
    except BaseException as error:
        report.update(status='failed', failure=repr(error)); raise
    finally:
        report.update(quota_after=quota(), elapsed_protocol_seconds=time.monotonic() - started)
        save(args.output, report)
    print(f'{args.action}: all10 matched current-compiler pipelines passed; timing_claim={measuring}')


if __name__ == '__main__':
    main()
