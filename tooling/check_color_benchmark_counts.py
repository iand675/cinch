#!/usr/bin/env python3
"""Check sealed color benchmark allocation counts or longer-count fingerprints without timing claims."""
from __future__ import annotations
import argparse
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
from check_compare import digest
from measure_color import sealed


def main() -> None:
    """Execute only existing inputs, preserving full observations and the first failure."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--simd', type=Path, required=True)
    parser.add_argument('--scalar', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--modes', nargs='+', required=True)
    parser.add_argument('--counts', type=int, nargs='+', default=[0, 1, 20])
    parser.add_argument('--palette-count', type=int)
    parser.add_argument('--size', type=int, default=256)
    parser.add_argument('--cpu', type=int, default=1)
    parser.add_argument('--allocations', action='store_true')
    args = parser.parse_args()
    if args.output.exists() or any(count < 0 for count in args.counts):
        parser.error('fresh output and nonnegative counts required')
    folders = [args.simd.resolve(), args.scalar.resolve()]
    receipts = [sealed(folder) for folder in folders]
    for key in ['compiler_sha256', 'runtime_sha256', 'driver_sha256', 'reference_source_sha256']:
        if receipts[0][key] != receipts[1][key]:
            raise RuntimeError('unmatched source/compiler pair: '+key)
    initial = shutil.disk_usage(args.output.parent).free
    if initial < 350*1024*1024:
        raise RuntimeError('350MiB initial guard')
    os.sched_setaffinity(0, {args.cpu})
    report = {'scope': __doc__, 'runner_sha256': digest(Path(__file__)),
              'allocation_counters': args.allocations, 'elapsed_values_are_not_measurements': True,
              'cpu_affinity': [args.cpu], 'initial_free_bytes': initial,
              'resource_policy': {'start_mib': 350, 'reserve_mib': 250, 'new_report_cap_mib': 5},
              'input_receipts': [{'path': str(folder / 'report.json'), 'sha256': digest(folder / 'report.json'),
                                  'compiler_sha256': receipt['compiler_sha256'],
                                  'runtime_sha256': receipt['runtime_sha256'],
                                  'executable_sha256': receipt['executable_sha256']}
                                 for folder, receipt in zip(folders, receipts)],
              'observations': [], 'summary': []}

    def save() -> None:
        """Fsync completed observations and partial failure details before returning."""
        with args.output.open('w') as handle:
            handle.write(json.dumps(report, indent=2)+'\n')
            handle.flush()
            os.fsync(handle.fileno())
        if args.output.stat().st_size > 5*1024*1024:
            raise RuntimeError('5MiB report cap')

    try:
        for mode in args.modes:
            counts = [args.palette_count] if args.palette_count is not None and mode.startswith('palette-') else args.counts
            for count in counts:
                checksums = set()
                programs = [('reference', folders[0] / 'reference', False),
                            ('simd', folders[0] / 'bench', False), ('scalar', folders[1] / 'bench', False)]
                if not args.allocations:
                    programs += [('simd', folders[0] / 'bench', True), ('scalar', folders[1] / 'bench', True)]
                for label, program, poison in programs:
                    if shutil.disk_usage(args.output.parent).free < 250*1024*1024:
                        raise RuntimeError('250MiB reserve')
                    env = {**os.environ, 'CINCH_HEAP_LIMIT': '256M' if poison else '32M',
                           'CINCH_STACK_SIZE': '8M', 'CINCH_LEAK_CHECK': '1',
                           'CINCH_RC_DEBUG': '1' if poison else '0',
                           'CINCH_ALLOC_STATS': '1' if args.allocations else '0',
                           'CINCH_ALLOC_KINDS': '1' if args.allocations else '0',
                           'GOMAXPROCS': '1', 'GOTELEMETRY': 'off'}
                    command = [str(program), mode, str(args.size), str(count)]
                    result = subprocess.run(command, text=True, capture_output=True, env=env, timeout=15)
                    row = {'mode': mode, 'input_pixels': args.size, 'iterations': count,
                           'program': label, 'poison': poison, 'command': command,
                           'stdout': result.stdout, 'stderr': result.stderr, 'returncode': result.returncode}
                    report['observations'].append(row)
                    if result.returncode or (label != 'reference' and 'leaked=0' not in result.stderr):
                        raise RuntimeError('failed count control: '+label+'/'+mode)
                    unused_elapsed, output, retained = map(int, result.stdout.split())
                    checksums.add((output, retained))
                    if args.allocations and label != 'reference':
                        stats = re.search(r'^cinch-alloc: (.+)$', result.stderr, re.M)
                        if stats is None:
                            raise RuntimeError('missing allocation statistics')
                        row['allocation_stats'] = {key: int(value) for key, value in re.findall(r'([\w-]+)=(\d+)', stats.group(1))}
                        row['allocation_kinds'] = [{key: (int(value) if value.isdecimal() else value)
                                                   for key, value in re.findall(r'([\w-]+)=([^\s]+)', line)}
                                                  for line in result.stderr.splitlines() if line.startswith('cinch-alloc-kind: ')]
                    save()
                if len(checksums) != 1:
                    raise RuntimeError('complete output/retained fingerprint mismatch: '+mode)
                report['summary'].append({'mode': mode, 'input_pixels': args.size, 'iterations': count,
                                          'output_and_retained_checksums': list(checksums.pop())})
        report['status'] = 'passed'
    except BaseException as error:
        report.update(status='failed-or-resource-paused', failure=repr(error))
        raise
    finally:
        report['final_free_bytes'] = shutil.disk_usage(args.output.parent).free
        report['input_receipts_unchanged'] = all(digest(folder / 'report.json') == row['sha256']
                                                for folder, row in zip(folders, report['input_receipts']))
        save()
    print(len(report['observations']), 'complete fingerprint/leak observations passed; no elapsed claim')


if __name__ == '__main__':
    main()
