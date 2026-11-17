#!/usr/bin/env python3
"""Measure already preflighted scanner baseline/candidate executables on an explicitly granted core, preserving all pairs and controls."""
from pathlib import Path
import argparse
import json
import os
import statistics
import subprocess
import time
import check

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
MODES = ['byte', 'rune', 'line', 'write', 'transfer', 'scan-line', 'scan-rune', 'scan-word', 'copy']


def main():
    """Keep tools, source, owners and counter mode frozen; never compile, relink or change a provider during timing."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--cpu', type=int, required=True)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--iterations', type=int, default=8)
    parser.add_argument('--samples', type=int, default=3)
    parser.add_argument('--workloads', choices=MODES, nargs='+', default=MODES)
    parser.add_argument('--sizes', type=int, nargs='+', default=[4096, 65536])
    parser.add_argument('--patterns', type=int, nargs='+', default=[0, 1, 2])
    args = parser.parse_args()
    assert 1 <= args.iterations <= 4294967295 and args.samples > 0
    assert all(0 <= size <= 65536 for size in args.sizes) and all(pattern in [0,1,2] for pattern in args.patterns)
    os.sched_setaffinity(0, {args.cpu})
    directories = {'baseline': ROOT/'build/buffered-benchmark-rust-v1', 'candidate': ROOT/'build/buffered-benchmark-scan-candidate-7e-v1'}
    identities = {}
    for name, directory in directories.items():
        identity = json.loads((directory/'identity.json').read_text())
        for field, filename in [('executable_sha256', 'benchmark'), ('llvm_sha256', 'benchmark.ll'), ('assembly_sha256', 'benchmark.s'),
                                ('reference_executable_sha256', 'reference'), ('reference_source_sha256', 'reference.go')]:
            assert check.digest(directory/filename) == identity[field]
        for filename, digest in identity['sources'].items():
            assert check.digest(directory/'project'/filename) == digest
        assert check.digest(Path(identity['compiler'])) == identity['compiler_sha256']
        assert check.digest(Path(identity['runtime'])) == identity['runtime_sha256']
        assert not identity['rc_debug_instrumentation'] and identity['ordinary_reclaim']
        identities[name] = identity
    assert identities['baseline']['compiler_sha256'] == identities['candidate']['compiler_sha256'] == '7e0d98d14cb0213d4deeb277e1ad65fce07784309716085e70e4b9db2c259102'
    assert identities['baseline']['runtime_sha256'] == identities['candidate']['runtime_sha256']
    preflight_path = HERE/'preflight-gates.json'
    preflight = json.loads(preflight_path.read_text())
    assert preflight['conditions'] == 729 and preflight['zero_leak_cinch_memory_runs'] == 1458
    assert check.digest(directories['candidate']/'identity.json') == preflight['candidate_identity_sha256']
    environment = {**os.environ, 'CINCH_HEAP_LIMIT': '32M', 'CINCH_STACK_SIZE': '8M', 'CINCH_RC_DEBUG': '0', 'CINCH_LEAK_CHECK': '1', 'CINCH_ALLOC_STATS': '0',
                   'GOMAXPROCS': '1', 'GOMEMLIMIT': '256MiB', 'GOTELEMETRY': 'off'}
    report = {'scope': 'Explicitly granted prebuilt same-epoch ordinary-RC paired timing; no production selection or compiler repair.',
              'cpu': args.cpu, 'iterations': args.iterations, 'samples': args.samples, 'identities': identities,
              'preflight_gates_sha256': check.digest(preflight_path), 'runner_sha256': check.digest(Path(__file__)),
              'background': 'Root documentation onCPU3; Region recovery archive onCPU1; Path CPU4 held; JSON CPU2 source probe completed/released before START.',
              'cpu_stat_start': Path('/sys/fs/cgroup/cpu.stat').read_text(), 'results': []}
    started = time.monotonic()
    pair = 0
    for mode in args.workloads:
        for size in args.sizes:
            for pattern in args.patterns:
                metadata = [mode, str(size), str(args.iterations), str(pattern)]
                reference_command = [str(directories['baseline']/'reference'), *metadata]
                oracle = subprocess.run(reference_command, capture_output=True, text=True, env=environment)
                assert oracle.returncode == 0
                expected = int(oracle.stdout.split()[1])
                row = {'workload': mode, 'size': size, 'pattern': pattern, 'checksum': expected, 'pairs': [],
                       'reference': {'command': reference_command, 'returncode': oracle.returncode, 'stdout': oracle.stdout, 'stderr': oracle.stderr}}
                for sample in range(args.samples+1):
                    order = ['baseline', 'candidate'] if pair % 2 == 0 else ['candidate', 'baseline']
                    observation = {'warmup': sample == 0, 'order': order, 'runs': []}
                    for name in order:
                        command = [str(directories[name]/'benchmark'), *metadata]
                        result = subprocess.run(command, capture_output=True, text=True, env=environment)
                        run = {'variant': name, 'command': command, 'returncode': result.returncode, 'stdout': result.stdout, 'stderr': result.stderr}
                        observation['runs'].append(run)
                        if result.returncode or 'live=0 immortal=0 leaked=0' not in result.stderr or int(result.stdout.split()[1]) != expected:
                            args.output.with_suffix('.failure.json').write_text(json.dumps({'row': row, 'pair': observation}, indent=2)+'\n')
                            raise RuntimeError('First timing/checksum/leak failure preserved; no sample discarded')
                    row['pairs'].append(observation)
                    pair += 1
                samples = {name: [int(run['stdout'].split()[0])/args.iterations for item in row['pairs'] if not item['warmup'] for run in item['runs'] if run['variant'] == name] for name in directories}
                row['samples_ns_per_operation'] = samples
                row['median_ns_per_operation'] = {name: statistics.median(values) for name, values in samples.items()}
                row['candidate_over_baseline'] = row['median_ns_per_operation']['candidate']/row['median_ns_per_operation']['baseline']
                report['results'].append(row)
                report['cpu_stat_end'] = Path('/sys/fs/cgroup/cpu.stat').read_text()
                report['wall_seconds'] = time.monotonic()-started
                args.output.parent.mkdir(parents=True, exist_ok=True)
                args.output.write_text(json.dumps(report, indent=2)+'\n')
                assert args.output.stat().st_size < 5*1024*1024
    print('Completed '+str(len(report['results']))+' same-epoch paired conditions with independent checksums, controls and zero leaks')


if __name__ == '__main__':
    main()
