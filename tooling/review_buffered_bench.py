#!/usr/bin/env python3
"""Verify frozen public-pipeline observations and preserve allocation, timing and hot-code evidence."""
from __future__ import annotations
import argparse
from collections import Counter
import hashlib
import json
from pathlib import Path
import re


def digest(path: Path) -> str:
    """Identify the exact observed artifact without copying its executable or compiler into a receipt."""
    return hashlib.sha256(path.read_bytes()).hexdigest()


def allocation(stderr: str) -> dict[str, int]:
    """Extract the actual runtime allocation counters, retaining their distinction from live memory."""
    line = next(line for line in stderr.splitlines() if line.startswith('cinch-alloc:'))
    return {name: int(value) for name, value in re.findall(r'([\w-]+)=(\d+)', line)}


def function_evidence(ir: str, assembly: str, name: str) -> dict:
    """Count static emitted callsites and assembly lines without presenting them as dynamic call frequencies."""
    body = re.search(r'^define[^\n]*@"' + re.escape(name) + r'"[^\n]*\n(.*?)^}', ir, re.M | re.S)
    machine = re.search(r'^"' + re.escape(name) + r'":.*?^\s*\.size\s+"' + re.escape(name) + r'"[^\n]*', assembly, re.M | re.S)
    if body is None or machine is None:
        raise ValueError('missing frozen public hot function: ' + name)
    calls = Counter(a or b for a, b in re.findall(r'(?:call|invoke) [^\n]*?@(?:"([^"]+)"|([^ (]+))', body.group(1)))
    return {'ir_lines': len(body.group(1).splitlines()), 'static_ir_callsites': dict(sorted(calls.items())),
            'optimized_assembly_lines': len(machine.group(0).splitlines()),
            'optimized_assembly_callsites': dict(sorted(Counter(re.findall(r'\bcallq\s+([^\n]+)', machine.group(0))).items()))}


def main() -> None:
    """Require complete checksum/memory receipts before writing a bounded, reproducible performance review."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--artifacts', type=Path, required=True)
    parser.add_argument('--preflights', type=Path, nargs='+', required=True)
    parser.add_argument('--timing', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    identity_path = args.artifacts / 'identity.json'
    identity = json.loads(identity_path.read_text())
    for key, name in [('executable_sha256', 'benchmark'), ('llvm_sha256', 'benchmark.ll'), ('assembly_sha256', 'benchmark.s'),
                      ('reference_executable_sha256', 'reference'), ('reference_source_sha256', 'reference.go')]:
        assert digest(args.artifacts / name) == identity[key], name
    assert identity['rc_debug_instrumentation'] is False and identity['ordinary_reclaim'] is True
    observations = {}
    memory_runs = 0
    for path in args.preflights:
        report = json.loads(path.read_text())
        assert report['action'] == 'preflight' and report['executable_sha256'] == identity['executable_sha256']
        for row in report['results']:
            key = tuple(row[name] for name in ['workload', 'pattern', 'size', 'iterations'])
            assert key not in observations, 'overlapping preflight condition'
            checksums = set()
            for run in row['runs']:
                assert run['exit_code'] == 0
                checksums.add(int(run['stdout'].split()[1]))
                if run['label'] == 'cinch':
                    assert 'live=0 immortal=0 leaked=0' in run['stderr']
                    memory_runs += 1
            assert checksums == {row['checksum']}
            assert sorted(run['poisoned'] for run in row['runs'] if run['label'] == 'cinch') == [False, True]
            observations[key] = row
    assert len(observations) == 729 and memory_runs == 1458
    timing = json.loads(args.timing.read_text())
    assert timing['action'] == 'measure' and timing['executable_sha256'] == identity['executable_sha256']
    assert len(timing['results']) == 54
    rows = []
    allocations = []
    for row in timing['results']:
        assert all(len(samples) == 3 and min(samples) >= 0 for samples in row['samples_ns'].values())
        rows.append({**row, 'cinch_over_reference': row['median_ns_per_operation']['cinch'] / row['median_ns_per_operation']['reference']})
    for mode in ['byte', 'rune', 'scan-rune']:
        before = observations[(mode, 0, 4096, 0)]
        after = observations[(mode, 0, 4096, 3)]
        base_run = next(run for run in before['runs'] if run['label'] == 'cinch' and not run['poisoned'])
        full_run = next(run for run in after['runs'] if run['label'] == 'cinch' and not run['poisoned'])
        base, full = allocation(base_run['stderr']), allocation(full_run['stderr'])
        allocations.append({'workload': mode, 'pattern': 'ASCII', 'bytes_per_iteration': 4096, 'iterations': 3,
                            'setup_only': base, 'full': full,
                            'incremental_headers_per_observation': (full['header-blocks'] - base['header-blocks']) / (4096 * 3)})
    ir = (args.artifacts / 'benchmark.ll').read_text()
    assembly = (args.artifacts / 'benchmark.s').read_text()
    functions = ['Main.read-bytes', 'Main.read-runes', 'Effect.Buffered.Reader.read-byte-from#cc',
                 'Effect.Scanner.scanner-next#cc', 'Effect.Scanner.Split.scan-runes#cc']
    result = {'scope': 'Observed old-source ordinary-RC public pipelines; no package completion, API promotion or compiler adoption claim.',
              'identity_sha256': digest(identity_path), 'identity': identity,
              'inputs': {str(path): digest(path) for path in args.preflights + [args.timing]},
              'preflight_conditions': len(observations), 'zero_leak_memory_runs': memory_runs,
              'timing_conditions': rows, 'cpu_stat_start': timing['cpu_stat_start'], 'cpu_stat_end': timing['cpu_stat_end'],
              'background': 'Root runtime build on CPU3; JSON Native unit on CPU2; Collections potentially CPU1. Compare/Path held CPU0/4; no own CPU2 process.',
              'allocation_evidence': allocations, 'hot_functions': {name: function_evidence(ir, assembly, name) for name in functions},
              'limitations': ['Three paired samples preserve raw noise; these are one admitted host/compiler epoch.',
                             'Byte fingerprints observe length and selected positions; independent full functional corpora provide separate byte-exact validation.',
                             'Scalar public reader/scanner paths remain allocation-heavy. Bulk write/transfer and fresh-copy control do not show the same slowdown.',
                             'No source mutation is selected from these measurements. Compare the exact source under an admitted borrow/reuse compiler before adopting a compiler optimization.',
                             'Native public benchmark and remaining complete-family runtime gates are pending.',
                             'The separate retained-callback ownership cycle still blocks full buffered-family promotion.']}
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(result, indent=2) + '\n')
    print('Verified 729 conditions / 1458 zero-leak memory runs / 54 paired timing conditions')


if __name__ == '__main__':
    main()
