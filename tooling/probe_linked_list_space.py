#!/usr/bin/env python3
"""Record ordinary process allocation/peak evidence for complete linked-list pipelines."""
from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess

from bench_linked_lists import WORKLOADS


def main() -> None:
    """Keep normal reclamation evidence separate from poisoned quarantine and timed measurements."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--artifacts', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--sizes', nargs='+', type=int, default=[32, 1024, 16384])
    parser.add_argument('--workloads', nargs='+', choices=WORKLOADS, default=WORKLOADS)
    parser.add_argument('--cpu', type=int, default=4)
    args = parser.parse_args()
    os.sched_setaffinity(0, {args.cpu})
    metadata = json.loads((args.artifacts / 'metadata.json').read_text())
    executable = args.artifacts.resolve() / 'bench'
    assert hashlib.sha256(executable.read_bytes()).hexdigest() == metadata['executable_sha256']
    environment = {**os.environ, 'CINCH_HEAP_LIMIT': '32M', 'CINCH_STACK_SIZE': '8M',
                   'CINCH_RC_DEBUG': '0', 'CINCH_LEAK_CHECK': '1', 'CINCH_ALLOC_STATS': '1'}
    report = {'description': __doc__, 'frontend': metadata['frontend'],
              'executable_sha256': metadata['executable_sha256'], 'sources': metadata['sources'],
              'cpu_affinity': [args.cpu], 'heap': '32M', 'stack': '8M', 'poisoned_free': False,
              'scope': 'Whole ordinary process: startup/configuration, prepared immutable input/list/handles, and one complete pipeline iteration. Prepared-owner teardown occurs after the final clock.',
              'rows': []}
    for workload in args.workloads:
        for size in args.sizes:
            result = subprocess.run([str(executable), str(WORKLOADS.index(workload)), str(size), '1'],
                                    env=environment, capture_output=True, text=True, check=True)
            assert 'cinch-leak: live=0' in result.stderr and 'leaked=0' in result.stderr, result.stderr
            line = next(line for line in result.stderr.splitlines() if line.startswith('cinch-alloc: '))
            stats = {name: int(value) for name, value in re.findall(r'([a-z-]+)=(\d+)', line)}
            assert stats['live'] == 0, stats
            report['rows'].append({'workload': workload, 'elements': size, 'stats': stats, 'leaked': 0,
                                   'checksum': int(result.stdout.split()[1])})
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(report, indent=2) + '\n')
    print(f'Recorded {len(report["rows"])} ordinary allocation probes with zero leaks.', flush=True)


if __name__ == '__main__':
    main()
