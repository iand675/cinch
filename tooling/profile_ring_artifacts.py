#!/usr/bin/env python3
"""Inspect frozen ring code and optionally collect whole-process allocation observations."""
from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess

WORKLOADS = ('dense-construct-walk', 'growing-join', 'balanced-join-aliases', 'short-split-rejoin', 'detached-copy')
HOT_FUNCTIONS = ('offset-links', 'count-cycle', 'walk', 'splice', 'resolve', 'canonical')


def digest(path: Path) -> str:
    """Verify actual frozen bytes before examining or executing an artifact."""
    return hashlib.sha256(path.read_bytes()).hexdigest()


def assembly_functions(text: str) -> dict[str, str]:
    """Extract function bodies using LLVM assembly boundaries, retaining original names."""
    functions = {}
    for start in re.finditer(r'^.*# -- Begin function (.+)$', text, re.M):
        end = text.find('# -- End function', start.end())
        if end != -1:
            functions[start.group(1)] = text[start.start():end]
    return functions


def main() -> None:
    """Keep disassembly facts separate from allocation observations and timing claims."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--variant', action='append', required=True, metavar='NAME=ARTIFACT_DIRECTORY')
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--allocations', action='store_true', help='Run one complete operation per condition; requires a granted functional CPU slot')
    parser.add_argument('--cpu', type=int, default=4)
    parser.add_argument('--sizes', nargs='+', type=int, default=[1024, 16384])
    args = parser.parse_args()
    if args.allocations:
        os.sched_setaffinity(0, {args.cpu})
    report = {'description': __doc__, 'runner_sha256': digest(Path(__file__)), 'allocation_scope': 'Whole process including startup, preparation, operation and teardown; stdout clocks are discarded, not timing evidence', 'variants': {}}
    checksums = {}
    for entry in args.variant:
        name, separator, directory = entry.partition('=')
        if not separator or name in report['variants']:
            parser.error('variants require unique NAME=ARTIFACT_DIRECTORY entries')
        directory = Path(directory).resolve()
        metadata = json.loads((directory / 'metadata.json').read_text())
        for filename, key in [('bench', 'executable_sha256'), ('bench.ll', 'llvm_sha256'), ('bench.s', 'assembly_sha256')]:
            assert digest(directory / filename) == metadata[key], (name, filename)
        for module, expected in metadata['sources'].items():
            assert digest(Path(metadata['source_snapshots']) / f'{module}.cinch') == expected, (name, module)
        functions = assembly_functions((directory / 'bench.s').read_text())
        selected = {}
        for symbol, body in functions.items():
            normalized = symbol.replace('$2e', '.').replace('$2d', '-')
            if 'Ring.' not in normalized or not any(function in normalized for function in HOT_FUNCTIONS):
                continue
            selected[symbol] = {'assembly_bytes': len(body.encode()), 'calls': len(re.findall(r'\bcallq?\b', body)),
                                'owner_calls': len(re.findall(r'\bcallq?\s+[^\n]*(?:cinch_dup|cinch_drop|cinch\.typed\.rc)', body)),
                                'wide_provider_calls': len(re.findall(r'\bcallq?\s+[^\n]*Data\.Wide', body)),
                                'spills': body.count('Spill'), 'reloads': body.count('Reload'),
                                'stack_reservations': re.findall(r'subq\s+\$(\d+), %rsp', body), 'packed_lane_adds': body.count('paddq')}
        row = {'metadata_sha256': digest(directory / 'metadata.json'), 'provenance': metadata, 'assembly': selected, 'allocations': []}
        if args.allocations:
            environment = {**os.environ, 'CINCH_HEAP_LIMIT': '32M', 'CINCH_STACK_SIZE': '8M', 'CINCH_LEAK_CHECK': '1', 'CINCH_RC_DEBUG': '0', 'CINCH_ALLOC_STATS': '1'}
            for mode, workload in enumerate(WORKLOADS):
                for size in args.sizes:
                    result = subprocess.run(['ring-benchmark', str(mode), str(size), '1'], executable=str(directory / 'bench'), env=environment, capture_output=True, text=True, timeout=30)
                    assert result.returncode == 0 and 'leaked=0' in result.stderr, (name, workload, size, result.stderr)
                    match = re.search(r'cinch-alloc: ([^\n]+)', result.stderr)
                    assert match, (name, workload, size, result.stderr)
                    statistics = {key: int(value) for key, value in re.findall(r'([\w-]+)=(\d+)', match.group(1))}
                    checksum = int(result.stdout.split()[1])
                    previous = checksums.setdefault((workload, size), checksum)
                    assert checksum == previous, (name, workload, size, checksum, previous)
                    row['allocations'].append({'workload': workload, 'elements': size, 'checksum': checksum, 'statistics': statistics, 'leaked': 0})
        report['variants'][name] = row
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(report, indent=2) + '\n')
    print(f'Inspected {len(report["variants"])} frozen ring artifact(s); allocation execution={args.allocations}', flush=True)


if __name__ == '__main__':
    main()
