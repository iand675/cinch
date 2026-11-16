#!/usr/bin/env python3
"""Measure interleaved complete MIME pipelines for SIMD, scalar and independent reference builds.

This tool never compiles. Every raw sample, checksum, allocation-policy difference
and observed shared-machine context is retained without filtering regressions.
"""
from __future__ import annotations
import argparse
import json
import os
from pathlib import Path
import statistics
from bench_mime import sample
from check_compare import digest


# All public workload families are measured, including both header transfer encodings.
MODES = ('parse', 'format', 'b-encode', 'q-encode', 'header', 'header-q', 'registry')


def main() -> None:
    """Validate exact prebuilt identities, rotate measurement order and retain every observation."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--simd-work', type=Path, required=True)
    parser.add_argument('--scalar-work', type=Path, required=True)
    parser.add_argument('--out', type=Path, required=True)
    parser.add_argument('--cpu', type=int, required=True)
    parser.add_argument('--samples', type=int, default=5)
    parser.add_argument('--iterations', type=int, default=10)
    parser.add_argument('--sizes', nargs='+', type=int, default=[64, 4096])
    parser.add_argument('--context', required=True)
    args = parser.parse_args()
    if args.samples <= 0 or args.iterations <= 0 or any(size < 0 for size in args.sizes):
        parser.error('positive sample/iteration counts and nonnegative byte sizes are required')
    os.sched_setaffinity(0, {args.cpu})
    simd, scalar = args.simd_work.resolve(), args.scalar_work.resolve()
    records = {name: json.loads((work / 'report.json').read_text()) for name, work in [('simd', simd), ('scalar', scalar)]}
    if records['simd']['scalar_snapshot'] or not records['scalar']['scalar_snapshot']:
        raise RuntimeError('mislabeled SIMD/scalar snapshots')
    for field in ('compiler_sha256', 'runtime_sha256', 'driver_sha256', 'reference_source_sha256', 'reference_executable_sha256', 'target_triple'):
        if records['simd'][field] != records['scalar'][field]:
            raise RuntimeError('different paired pipeline identity: ' + field)
    for name, work in [('simd', simd), ('scalar', scalar)]:
        record = records[name]
        if digest(work / 'bench') != record['executable_sha256'] or digest(work / 'reference') != record['reference_executable_sha256']:
            raise RuntimeError('prebuilt executable changed: ' + name)
        for module, expected in record['source_sha256'].items():
            if digest(work / (module + '.cinch')) != expected:
                raise RuntimeError('retained benchmark source changed: ' + module)
        if not (work / 'functional-smoke.json').is_file():
            raise RuntimeError('missing independent normal/poison preflight: ' + name)
    report = {'format': 1, 'runner_sha256': digest(Path(__file__)), 'cpu_affinity': sorted(os.sched_getaffinity(0)), 'iterations': args.iterations, 'samples': args.samples, 'background_context': args.context, 'build_reports': {name: {'path': str(work / 'report.json'), 'sha256': digest(work / 'report.json')} for name, work in [('simd', simd), ('scalar', scalar)]}, 'programs': {'simd': records['simd']['executable_sha256'], 'scalar': records['scalar']['executable_sha256'], 'reference': records['simd']['reference_executable_sha256']}, 'scope': records['simd']['scope'], 'allocation_policy': records['simd']['allocation_policy'], 'registry_context_scope': records['simd']['registry_context_scope'], 'results': []}
    args.out.parent.mkdir(parents=True, exist_ok=True)
    programs = [('simd', simd / 'bench'), ('scalar', scalar / 'bench'), ('reference', simd / 'reference')]
    for mode in MODES:
        for size in args.sizes:
            checksums, warmups = set(), {}
            for name, program in programs:
                fields = sample(program, mode, size, args.iterations, simd)
                warmups[name] = list(fields); checksums.add(fields[1:])
            elapsed = {name: [] for name, _ in programs}
            orders = []
            for index in range(args.samples):
                order = programs[index % 3:] + programs[:index % 3]
                if index % 2:
                    order = list(reversed(order))
                orders.append([name for name, _ in order])
                for name, program in order:
                    nanos, output, retained = sample(program, mode, size, args.iterations, simd)
                    elapsed[name].append(nanos); checksums.add((output, retained))
            if len(checksums) != 1:
                raise RuntimeError('independent complete-pipeline checksum mismatch: ' + mode)
            medians = {name: statistics.median(values) / args.iterations for name, values in elapsed.items()}
            report['results'].append({'mode': mode, 'input_bytes': size, 'checksums': list(checksums.pop()), 'warmups_ns_and_checksums': warmups, 'measurement_orders': orders, 'raw_samples_ns': elapsed, 'median_ns_per_iteration': medians, 'simd_over_scalar': medians['simd'] / medians['scalar'], 'simd_over_reference': medians['simd'] / medians['reference']})
            args.out.write_text(json.dumps(report, indent=2) + '\n')
            print(mode, size, medians, flush=True)


if __name__ == '__main__':
    main()
