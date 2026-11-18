#!/usr/bin/env python3
"""Confirm three control pipelines using existing sealed programs and a fresh report."""
from __future__ import annotations
import argparse
import json
import os
from pathlib import Path
import shutil
import statistics
import subprocess
import time
from check_compare import digest
from inspect_multipart_controls import sealed_inputs


#: Unchanged reader/writer controls and the targeted disposition anchor, all at 4 KiB.
MODES = ('read', 'write-body', 'disposition-dense')


def save(path: Path, report: dict) -> None:
    """Durably preserve partial observations and diagnostics without changing input receipts."""
    with path.open('w') as handle:
        handle.write(json.dumps(report, indent=2) + '\n')
        handle.flush()
        os.fsync(handle.fileno())


def main() -> None:
    """Bound the executable-only confirmation by wall time, disk reserve and exact identities."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--baseline', type=Path, required=True)
    parser.add_argument('--candidate', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--context', required=True)
    parser.add_argument('--cpu', type=int, default=0)
    parser.add_argument('--iterations', type=int, default=200)
    parser.add_argument('--samples', type=int, default=7)
    args = parser.parse_args()
    if args.iterations <= 0 or args.samples <= 0 or args.output.exists():
        parser.error('positive counts and a fresh output path are required')
    folders = [args.baseline.resolve(), args.candidate.resolve()]
    reports = [sealed_inputs(folder) for folder in folders]
    for key in ('compiler_sha256', 'runtime_sha256', 'driver_sha256', 'endpoint_object_sha256',
                'reference_executable_sha256', 'sdk_executable_sha256'):
        if reports[0][key] != reports[1][key]:
            raise RuntimeError('unmatched control identity: ' + key)
    initial_free = shutil.disk_usage(args.output.parent).free
    if initial_free < 300 * 1024 * 1024:
        raise RuntimeError('timing-only 300 MiB initial guard')
    os.sched_setaffinity(0, {args.cpu})
    env = {**os.environ, 'GOMAXPROCS': '1', 'GOTELEMETRY': 'off',
           'CINCH_HEAP_LIMIT': '32M', 'CINCH_STACK_SIZE': '8M',
           'CINCH_LEAK_CHECK': '1', 'CINCH_RC_DEBUG': '0', 'CINCH_ALLOC_STATS': '0'}
    programs = [('baseline', folders[0] / 'bench'), ('candidate', folders[1] / 'bench'),
                ('reference', folders[0] / 'reference')]
    report = {'scope': __doc__, 'runner_sha256': digest(Path(__file__)),
              'cpu_affinity': [args.cpu], 'background_context': args.context,
              'iterations': args.iterations, 'samples': args.samples,
              'resource_policy': {'scope': 'Explicit Root timing-only exception; no build guard changes',
                                  'start_mib': 300, 'reserve_mib': 250, 'new_report_cap_mib': 5,
                                  'maximum_measurement_wall_seconds': 20},
              'initial_free_bytes': initial_free, 'allocation_counters': False,
              'profile': {'heap': '32M', 'stack': '8M', 'poison': False, 'leak_check': True},
              'sealed_inputs': [{'folder': str(folder), 'report_sha256': digest(folder / 'report.json'),
                                 'executable_sha256': receipt['executable_sha256'],
                                 'reference_executable_sha256': receipt['reference_executable_sha256'],
                                 'source_sha256': receipt['source_sha256'],
                                 'compiler_sha256': receipt['compiler_sha256'],
                                 'runtime_sha256': receipt['runtime_sha256']}
                                for folder, receipt in zip(folders, reports)],
              'results': [], 'observations': [], 'quota_before': Path('/sys/fs/cgroup/cpu.stat').read_text()}
    started = time.monotonic()
    deadline = started + 20
    try:
        for mode_index, mode in enumerate(MODES):
            elapsed = {label: [] for label, _ in programs}
            checksums = set()
            for index in range(-1, args.samples):
                rotation = (mode_index + max(index, 0)) % len(programs)
                ordered = programs[rotation:] + programs[:rotation]
                if index >= 0 and index % 2:
                    ordered = list(reversed(ordered))
                for label, executable in ordered:
                    if shutil.disk_usage(args.output.parent).free < 250 * 1024 * 1024:
                        raise RuntimeError('250 MiB reserve reached')
                    remaining = deadline - time.monotonic()
                    if remaining <= 0:
                        raise RuntimeError('20-second timing window exhausted')
                    command = [str(executable), mode, '4096', str(args.iterations)]
                    completed = subprocess.run(command, cwd=args.output.parent, env=env,
                                               text=True, capture_output=True, timeout=remaining)
                    report['observations'].append({'label': label, 'sample_index': index,
                                                   'command': command, 'stdout': completed.stdout,
                                                   'stderr': completed.stderr,
                                                   'returncode': completed.returncode})
                    if completed.returncode or (label != 'reference' and 'leaked=0' not in completed.stderr):
                        raise RuntimeError('failed control observation: ' + label + '/' + mode)
                    nanos, output, retained = map(int, completed.stdout.split())
                    checksums.add((output, retained))
                    if index >= 0:
                        elapsed[label].append(nanos)
            if len(checksums) != 1:
                raise RuntimeError('independent control checksums differ: ' + mode)
            medians = {label: statistics.median(values) / args.iterations
                       for label, values in elapsed.items()}
            report['results'].append({'mode': mode, 'input_bytes': 4096,
                                      'output_and_retained_input_checksums': list(checksums.pop()),
                                      'raw_samples_ns': elapsed, 'median_ns_per_iteration': medians,
                                      'baseline_over_candidate': medians['baseline'] / medians['candidate']})
            save(args.output, report)
        report['status'] = 'passed'
    except BaseException as error:
        report['status'] = 'failed'
        report['failure'] = repr(error)
        raise
    finally:
        report['quota_after'] = Path('/sys/fs/cgroup/cpu.stat').read_text()
        report['measurement_wall_seconds'] = time.monotonic() - started
        report['final_free_bytes'] = shutil.disk_usage(args.output.parent).free
        report['input_receipts_unchanged'] = all(digest(folder / 'report.json') == row['report_sha256']
                                                for folder, row in zip(folders, report['sealed_inputs']))
        save(args.output, report)
        if args.output.stat().st_size > 5 * 1024 * 1024:
            raise RuntimeError('5 MiB new-report cap exceeded')
    print(json.dumps(report['results'], indent=2))


if __name__ == '__main__':
    main()
