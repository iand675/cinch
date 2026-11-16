#!/usr/bin/env python3
"""Compare frozen complete ring pipelines with rotating variant order and exact outputs."""
from __future__ import annotations

import argparse
import datetime
import hashlib
import json
import os
from pathlib import Path
import statistics
import subprocess

WORKLOADS = ("dense-construct-walk", "growing-join", "balanced-join-aliases", "short-split-rejoin", "detached-copy")


def digest(path: Path) -> str:
    """Identify the actual consumed executable, IR, source or metadata bytes."""
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main() -> None:
    """Measure only previously checked immutable artifacts, keeping profile differences explicit."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--variant', action='append', required=True, metavar='NAME=ARTIFACT_DIRECTORY')
    parser.add_argument('--reference', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--cpu', type=int, default=0)
    parser.add_argument('--iterations', type=int, default=8)
    parser.add_argument('--samples', type=int, default=5)
    parser.add_argument('--sizes', type=int, nargs='+', default=[32, 1024, 16384])
    parser.add_argument('--workloads', nargs='+', choices=WORKLOADS, default=WORKLOADS)
    args = parser.parse_args()
    if min(args.iterations, args.samples) <= 0 or min(args.sizes) < 0:
        parser.error('positive iteration/sample counts and nonnegative sizes required')
    os.sched_setaffinity(0, {args.cpu})
    variants = {}
    provenance = {}
    for entry in args.variant:
        name, separator, path = entry.partition('=')
        if not separator or name in variants:
            parser.error('variants require unique NAME=ARTIFACT_DIRECTORY entries')
        directory = Path(path).resolve()
        metadata = json.loads((directory / 'metadata.json').read_text())
        for filename, key in [('bench', 'executable_sha256'), ('bench.ll', 'llvm_sha256'), ('bench.s', 'assembly_sha256')]:
            assert digest(directory / filename) == metadata[key], (name, filename)
        for module, expected in metadata['sources'].items():
            assert digest(Path(metadata['source_snapshots']) / f'{module}.cinch') == expected, (name, module)
        assert (directory / 'verification.json').exists(), name
        variants[name] = directory / 'bench'
        provenance[name] = metadata
    variants['reference'] = args.reference.resolve()
    assert len({row['driver_sha256'] for row in provenance.values()}) == 1
    assert all(digest(args.reference) == row['reference_executable_sha256'] for row in provenance.values())
    report = {
        'description': __doc__, 'status': 'running', 'cpu_affinity': [args.cpu],
        'started_utc': datetime.datetime.now(datetime.timezone.utc).isoformat(),
        'runner_sha256': digest(Path(__file__)), 'iterations': args.iterations, 'samples': args.samples,
        'timing': 'Internal monotonic clock around complete operation loop; preparation, startup and prepared-owner teardown excluded',
        'memory': 'Ordinary reclamation, 32M heap, 8M runtime stack; separate functional poison profiles already passed',
        'profile_limit': 'Generated RC modes are retained per artifact; compare matched Native profiles. Crossfrontend/reference ratios also include different allocation and ownership representations.',
        'argv0': 'ring-benchmark (fixed across variants to keep captured startup arguments identical)',
        'native_limit': 'Rust-produced experimental Native645; no self-built distribution claim',
        'background_note': 'Only this CPU is reserved. Parent Cargo CPU3, Codegen foreign-alias CPU1 and MIME CPU4 may overlap; shared caches/memory are not isolated.',
        'background_process_snapshot': subprocess.check_output(['ps', '-eo', 'pid,psr,pcpu,comm'], text=True).splitlines(),
        'variants': provenance, 'reference_sha256': digest(args.reference), 'rows': [],
    }
    environment = {**os.environ, 'CINCH_HEAP_LIMIT': '32M', 'CINCH_STACK_SIZE': '8M',
                   'CINCH_LEAK_CHECK': '1', 'CINCH_RC_DEBUG': '0', 'GOMAXPROCS': '1', 'GOTELEMETRY': 'off'}
    names = list(variants)
    args.output.parent.mkdir(parents=True, exist_ok=True)
    for workload in args.workloads:
        workload_index = WORKLOADS.index(workload)
        for size in args.sizes:
            values = {name: [] for name in names}
            checksums = set()
            for round_index in range(args.samples + 1):
                offset = round_index % len(names)
                for name in names[offset:] + names[:offset]:
                    output = subprocess.run(['ring-benchmark', str(workload_index), str(size), str(args.iterations)],
                                            executable=str(variants[name]), env=environment, capture_output=True, text=True, timeout=30)
                    assert output.returncode == 0, (name, workload, size, output.stderr)
                    if name != 'reference':
                        assert 'leaked=0' in output.stderr, (name, workload, size, output.stderr)
                    elapsed, checksum = map(int, output.stdout.split())
                    checksums.add(checksum)
                    if round_index:
                        values[name].append(elapsed)
            assert len(checksums) == 1, (workload, size, checksums)
            row = {'workload': workload, 'elements': size, 'checksum': checksums.pop(), 'samples_ns': values,
                   'median_ns_per_pipeline': {name: statistics.median(samples) / args.iterations for name, samples in values.items()}}
            report['rows'].append(row)
            args.output.write_text(json.dumps(report, indent=2) + '\n')
            print(workload, size, row['median_ns_per_pipeline'], flush=True)
    report['status'] = 'passed'
    report['finished_utc'] = datetime.datetime.now(datetime.timezone.utc).isoformat()
    args.output.write_text(json.dumps(report, indent=2) + '\n')


if __name__ == '__main__':
    main()
