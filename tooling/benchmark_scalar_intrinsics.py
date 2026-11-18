#!/usr/bin/env python3
"""Measure verified scalar recurrences in alternating pairs on one explicitly selected CPU."""
from __future__ import annotations

import argparse
import json
import os
from pathlib import Path
import statistics
import subprocess

import bootstrap
from validate_scalar_intrinsic_codegen import checksum


def main() -> None:
    """Require immutable verified executables and exact checksums for every timed observation."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--codegen-report', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--cpu', type=int, required=True)
    parser.add_argument('--iterations', type=int, default=5_000_000)
    parser.add_argument('--samples', type=int, default=9)
    parser.add_argument('--background-context', default='')
    args = parser.parse_args()
    if not 1 <= args.iterations <= 100_000_000 or not 3 <= args.samples <= 31:
        parser.error('iterations must be 1..100000000 and samples 3..31')
    gate = json.loads(args.codegen_report.read_text())
    if gate['status'] != 'passed' or len(gate['results']) != 2:
        parser.error('a complete paired codegen and semantic gate is required')
    rows = {row['name']: row for row in gate['results']}
    if set(rows) != {'baseline', 'experimental'}:
        parser.error('the verified gate must contain baseline and experimental clients')
    for row in rows.values():
        if bootstrap.file_hash(Path(row['executable'])) != row['executable_sha256'] or not row.get('clock_boundary_proof'):
            parser.error('executable identity or independently inspected clock boundaries disagree')
    background = subprocess.run(['ps', '-C', 'cinch,clang,rustc', '-o', 'pid=,psr=,comm=,args='], capture_output=True, text=True).stdout
    expected = checksum(args.iterations)
    report = {'codegen_report_sha256': bootstrap.file_hash(args.codegen_report),
              'source_sha256': gate['source_sha256'], 'provider_receipt_sha256': gate['provider_receipt_sha256'],
              'object_sha256': gate['object_sha256'], 'runtime_sha256': gate['runtime_sha256'],
              'cpu_affinity': [args.cpu], 'driver_cpu_affinity': sorted(os.sched_getaffinity(0)),
              'background_context': args.background_context, 'compiler_activity_before': background,
              'iterations': args.iterations, 'expected_checksum': expected,
              'description': 'Constructor-free scalar recurrence only; input acquisition, checksum observation, startup and teardown remain outside both clocks.',
              'profile': {'heap': '32M', 'stack': '8M', 'poison': False, 'checked_rc_compilation': True},
              'status': 'running', 'warmups': [], 'pairs': [], 'summary': {}}
    bootstrap.write_json(args.output, report)

    def observe(name: str) -> dict:
        """Keep measured duration only when the runtime produces the exact independently computed result."""
        row = rows[name]
        command = ['taskset', '-c', str(args.cpu), row['executable']]
        run = subprocess.run(command, capture_output=True, text=True, timeout=180,
            env={**os.environ, 'CINCH_BENCH_ITERATIONS': str(args.iterations), 'CINCH_STACK_SIZE': '8M',
                 'CINCH_HEAP_LIMIT': '32M', 'CINCH_RC_DEBUG': '0', 'CINCH_LEAK_CHECK': '1'})
        observed = json.loads(run.stdout)
        if run.returncode or observed['checksum'] != expected or 'leaked=0' not in run.stderr:
            raise bootstrap.BootstrapError('timed recurrence failed semantic or ownership validation')
        return {'name': name, 'compiler_sha256': row['compiler_sha256'], 'executable_sha256': row['executable_sha256'],
                'command': command, 'exit': run.returncode, 'stderr': run.stderr, **observed}

    for name in ['baseline', 'experimental']:
        report['warmups'].append(observe(name))
    for index in range(args.samples):
        order = ['baseline', 'experimental'] if index % 2 == 0 else ['experimental', 'baseline']
        report['pairs'].append([observe(name) for name in order])
        bootstrap.write_json(args.output, report)
    medians = {}
    for name in ['baseline', 'experimental']:
        values = [row['elapsed_ns'] for pair in report['pairs'] for row in pair if row['name'] == name]
        medians[name] = statistics.median(values)
        report['summary'][name] = {'median_ns': medians[name], 'min_ns': min(values), 'max_ns': max(values),
                                    'ns_per_step': medians[name] / args.iterations}
    report['summary']['median_speedup'] = medians['baseline'] / medians['experimental']
    report['compiler_activity_after'] = subprocess.run(['ps', '-C', 'cinch,clang,rustc', '-o', 'pid=,psr=,comm=,args='], capture_output=True, text=True).stdout
    report['status'] = 'passed'
    bootstrap.write_json(args.output, report)
    print(json.dumps(report['summary'], indent=2), flush=True)


if __name__ == '__main__':
    main()
