#!/usr/bin/env python3
"""Preflight matched UUID workloads, then measure sealed pipelines only in a separately granted exclusive window."""
from __future__ import annotations
import argparse
import json
import os
from pathlib import Path
import statistics
import subprocess
import time
from profile_uuid_benchmarks import checked_identity
from validate_uuid import digest


def invoke(executable: Path, arguments: list[str], generation: bool, reference: bool,
           measure: bool, poison: bool, deadline: float) -> tuple[int, int, str]:
    """Normalize argv0 and observe every result while keeping entropy outputs independent."""
    remaining = deadline - time.monotonic()
    assert remaining > 0, 'Bounded protocol deadline reached'
    environment = {**{key: value for key, value in os.environ.items() if not key.startswith('CINCH_')},
                   'CINCH_HEAP_LIMIT': '256M' if poison else '32M', 'CINCH_STACK_SIZE': '8M',
                   'CINCH_RC_DEBUG': '1' if poison else '0', 'CINCH_ALLOC_STATS': '0',
                   'CINCH_LEAK_CHECK': '0' if measure else '1'}
    output = subprocess.run(['uuid-bench', *arguments], executable=str(executable), env=environment,
                            capture_output=True, text=True, timeout=remaining)
    assert output.returncode == 0, (executable, arguments, output.stdout, output.stderr)
    fields = list(map(int, output.stdout.split()))
    assert len(fields) == (3 if generation else 2) and fields[0] >= 0
    if generation:
        assert fields[1] == int(arguments[-1]), output.stdout
    if not reference and not measure:
        assert 'cinch-leak: live=0 immortal=0 leaked=0' in output.stderr, output.stderr
    return fields[0], fields[-1], output.stderr


def save(path: Path, report: dict) -> None:
    """Preserve incremental protocol progress and the first failure without touching sealed inputs."""
    temporary = path.with_name(path.name + '.tmp')
    temporary.write_text(json.dumps(report, indent=2) + '\n')
    temporary.replace(path)


def main() -> None:
    """Require exact matching-count memory preflights before a bounded paired timing protocol."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('action', choices=('preflight', 'measure'))
    parser.add_argument('--original', type=Path, required=True)
    parser.add_argument('--direct', type=Path, required=True)
    parser.add_argument('--generation', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--preflight', type=Path)
    parser.add_argument('--cpu', type=int, required=True)
    parser.add_argument('--iterations', type=int, default=4096)
    parser.add_argument('--generation-iterations', type=int, default=256)
    parser.add_argument('--samples', type=int, default=5)
    parser.add_argument('--budget-seconds', type=int, choices=range(1, 31), default=30)
    parser.add_argument('--background', default='Record the parent-coordinated workload at execution time.')
    args = parser.parse_args()
    assert args.iterations > 0 and args.generation_iterations > 0 and args.samples > 0
    os.sched_setaffinity(0, {args.cpu})
    original, direct, generation = args.original.resolve(), args.direct.resolve(), args.generation.resolve()
    identities = [checked_identity(original), checked_identity(direct), checked_identity(generation, True)]
    assert len({(item['compiler_sha256'], item['runtime_sha256']) for item in identities}) == 1
    reference = original / 'reference'
    assert len({digest(reference), *(item['reference_executable_sha256'] for item in identities)}) == 1
    signature = {'original_identity_sha256': digest(original / 'identity.json'),
                 'direct_identity_sha256': digest(direct / 'identity.json'),
                 'generation_identity_sha256': digest(generation / 'identity.json'),
                 'iterations': args.iterations, 'generation_iterations': args.generation_iterations}
    measuring = args.action == 'measure'
    if measuring:
        assert args.preflight is not None
        proof = json.loads(args.preflight.read_text())
        assert proof['status'] == 'passed' and proof['signature'] == signature
        assert not proof['timing_claim'] and len(proof['rows']) == 13
    args.output.parent.mkdir(parents=True, exist_ok=True)
    assert not args.output.exists(), 'Preserve previous protocols and failures'
    report = {'kind': 'UUID-paired-public-pipelines', 'action': args.action, 'status': 'running',
              'signature': signature, 'compiler_sha256': identities[0]['compiler_sha256'],
              'runtime_sha256': identities[0]['runtime_sha256'], 'timing_claim': False,
              'compares_random_payloads': False, 'argv0': 'uuid-bench', 'cpu_affinity': [args.cpu],
              'memory': '32MiB/8MiB,RCdebug=0 for timing; additionally poison256MiB for preflights',
              'background': args.background, 'rows': [],
              'original_and_direct_preserved': True, 'candidate_selected': False, 'package_complete': False}
    save(args.output, report)
    started = time.monotonic()
    deadline = started + args.budget_seconds
    pure = [('parse', 0, pattern) for pattern in range(5)] + [('format', 0, 0)] + [('append', size, 0) for size in (0, 64, 4096)] + [('compare', 0, 0)]
    try:
        for mode, size, pattern in pure:
            variants = {family + '-' + variant: directory / variant / 'bench'
                        for family, directory in (('original', original), ('direct', direct))
                        for variant in ('simd', 'scalar')}
            variants['reference'] = reference
            names = list(variants)
            samples = {name: [] for name in names}
            checksums = set()
            runs = []
            for sample in range(args.samples + 1 if measuring else 1):
                order = names[sample % len(names):] + names[:sample % len(names)]
                for name in order:
                    for poison in ((False,) if measuring or name == 'reference' else (False, True)):
                        elapsed, checksum, diagnostics = invoke(variants[name], [mode, str(size), str(args.iterations), str(pattern)],
                                                                False, name == 'reference', measuring, poison, deadline)
                        checksums.add(checksum)
                        if measuring and sample:
                            samples[name].append(elapsed)
                        if not measuring:
                            runs.append({'variant': name, 'poison': poison, 'fingerprint': checksum, 'diagnostics': diagnostics})
            assert len(checksums) == 1, (mode, size, pattern, checksums)
            row = {'workload': mode, 'size': size, 'pattern': pattern,
                   'iterations': args.iterations, 'fingerprint': checksums.pop()}
            if measuring:
                row.update(samples_ns=samples, median_ns={key: statistics.median(value) for key, value in samples.items()})
            else:
                row['runs'] = runs
            report['rows'].append(row)
            save(args.output, report)
        for mode in ('default', 'v4', 'v7'):
            variants = {'candidate': generation / 'bench', 'reference': reference}
            samples = {name: [] for name in variants}
            runs = []
            for sample in range(args.samples + 1 if measuring else 1):
                order = ('candidate', 'reference') if sample % 2 == 0 else ('reference', 'candidate')
                for name in order:
                    arguments = [mode, str(args.generation_iterations)]
                    if name == 'reference':
                        arguments.insert(0, 'generate')
                    for poison in ((False,) if measuring or name == 'reference' else (False, True)):
                        elapsed, checksum, diagnostics = invoke(variants[name], arguments, True, name == 'reference', measuring, poison, deadline)
                        if measuring and sample:
                            samples[name].append(elapsed)
                        if not measuring:
                            runs.append({'variant': name, 'poison': poison, 'variable_fingerprint': checksum,
                                         'valid': args.generation_iterations, 'diagnostics': diagnostics})
            row = {'workload': 'generate-' + mode, 'iterations': args.generation_iterations}
            if measuring:
                row.update(samples_ns=samples, median_ns={key: statistics.median(value) for key, value in samples.items()})
            else:
                row['runs'] = runs
            report['rows'].append(row)
            save(args.output, report)
        report.update(status='passed', timing_claim=measuring, elapsed_protocol_seconds=time.monotonic() - started)
    except BaseException as error:
        report.update(status='failed', failure=repr(error), elapsed_protocol_seconds=time.monotonic() - started)
        raise
    finally:
        save(args.output, report)
    print(f'{args.action} passed all13 public workloads; timing_claim={measuring}')


if __name__ == '__main__':
    main()
