#!/usr/bin/env python3
"""Prepare real UUID generation pipelines and validate invariant counts without comparing entropy."""
from __future__ import annotations
import argparse
import json
import os
import re
from pathlib import Path
import shutil
import statistics
from bench_uuid import cache_bytes
from validate_uuid import ROOT, MODULES, digest, execute


def main() -> None:
    """Separate bounded builds, memory checks and externally scheduled timing samples."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('action', choices=('build', 'preflight', 'measure'))
    parser.add_argument('--compiler', type=Path, required=True)
    parser.add_argument('--runtime', type=Path, required=True)
    parser.add_argument('--clang', type=Path, required=True)
    parser.add_argument('--reference-artifacts', type=Path, required=True)
    parser.add_argument('--artifacts', type=Path, required=True)
    parser.add_argument('--cpu', type=int, required=True)
    parser.add_argument('--output', type=Path)
    parser.add_argument('--iterations', type=int, default=4096)
    parser.add_argument('--samples', type=int, default=5)
    args = parser.parse_args()
    os.sched_setaffinity(0, {args.cpu})
    directory = args.artifacts.resolve()
    reference_directory = args.reference_artifacts.resolve()
    reference = reference_directory / 'reference'
    reference_identity = json.loads((reference_directory / 'identity.json').read_text())
    assert digest(reference) == reference_identity['reference_executable_sha256']
    assert digest(reference_directory / 'reference.go') == reference_identity['reference_source_sha256']
    assert digest(args.compiler) == reference_identity['compiler_sha256']
    assert digest(args.runtime) == reference_identity['runtime_sha256']
    environment = {**os.environ, 'CINCH_CLANG': str(args.clang.resolve()), 'RUST_MIN_STACK': '67108864'}
    identity = directory / 'identity.json'
    if args.action == 'build':
        assert shutil.disk_usage(ROOT).free >= 350 * 1024 * 1024
        assert not directory.exists(), 'Preserve every earlier snapshot and failure'
        directory.mkdir(parents=True)
        report = {'status': 'building', 'compiler_sha256': digest(args.compiler),
                  'runtime_sha256': digest(args.runtime), 'clang_sha256': digest(args.clang),
                  'reference_executable_sha256': digest(reference),
                  'reference_source_sha256': reference_identity['reference_source_sha256'],
                  'scope': 'Actual entropy, wall clock and serialized V7 state; invariant equality only; payload fingerprints may differ.'}
        identity.write_text(json.dumps(report, indent=2) + '\n')
        for module in MODULES:
            path = ROOT / 'experiments/uuid-candidate-v1/sources' / (module + '.cinch') if module in ('Data.Uuid', 'Effect.Uuid') else ROOT / 'compiler/prelude' / (module.replace('.', '/') + '.cinch')
            (directory / (module + '.cinch')).write_bytes(path.read_bytes())
        entry = directory / 'Main.cinch'
        entry.write_bytes((ROOT / 'tooling/data/uuid-generation-benchmark.cinch').read_bytes())
        executable = directory / 'bench'
        result = execute([str(args.compiler.resolve()), 'build', str(entry), '--opt-level', '2',
                          '--runtime-archive', str(args.runtime.resolve()), '-o', str(executable)], directory, environment, True)
        (directory / 'build.log').write_text(result['stdout'] + result['stderr'])
        report['build'] = result
        identity.write_text(json.dumps(report, indent=2) + '\n')
        assert result['exit'] == 0, result
        llvm, assembly = directory / 'bench.ll', directory / 'bench.s'
        result = execute([str(args.clang.resolve()), '-O2', '-S', '-x', 'ir', str(llvm), '-o', str(assembly)], directory, environment)
        (directory / 'assembly.log').write_text(result['stdout'] + result['stderr'])
        report['assembly'] = result
        identity.write_text(json.dumps(report, indent=2) + '\n')
        assert result['exit'] == 0, result
        for symbol in ('@cinch_random_bytes', '@cinch_clock_wall_ns', '@cinch_clock_monotonic_ns'):
            assert re.search(r'\bcall\b[^\n]*' + r'@"?' + re.escape(symbol[1:]) + r'"?\(', llvm.read_text()), f'Missing real provider call: {symbol}'
        report.update(status='built-not-preflighted', executable_sha256=digest(executable),
                      llvm_sha256=digest(llvm), assembly_sha256=digest(assembly),
                      sources={p.name: digest(p) for p in directory.glob('*.cinch')},
                      output_bytes=cache_bytes(directory))
        assert report['output_bytes'] <= 20 * 1024 * 1024
        assert shutil.disk_usage(ROOT).free >= 250 * 1024 * 1024
        identity.write_text(json.dumps(report, indent=2) + '\n')
        print('Built real-generation pipeline; no timing or entropy-equality claim')
        return
    assert args.output is not None
    report = json.loads(identity.read_text())
    assert report['reference_executable_sha256'] == digest(reference)
    assert report['reference_source_sha256'] == reference_identity['reference_source_sha256']
    assert report['compiler_sha256'] == digest(args.compiler) and report['runtime_sha256'] == digest(args.runtime)
    for key, name in [('executable_sha256', 'bench'), ('llvm_sha256', 'bench.ll'), ('assembly_sha256', 'bench.s')]:
        assert report[key] == digest(directory / name)
    for name, sha in report['sources'].items():
        assert digest(directory / name) == sha
    if args.action == 'measure':
        assert report.get('preflight_passed'), 'Use only exact snapshots that passed both memory profiles'
    rows = []
    for mode in ('default', 'v4', 'v7'):
        for count in ([0, 1, 17] if args.action == 'preflight' else [args.iterations]):
            runs, samples = [], {'candidate': [], 'reference': []}
            for sample in range(1 if args.action == 'preflight' else args.samples + 1):
                for variant in (('candidate', 'reference') if sample % 2 == 0 else ('reference', 'candidate')):
                    profiles = (False, True) if args.action == 'preflight' and variant == 'candidate' else (False,)
                    for poison in profiles:
                        env = {**{k: v for k, v in environment.items() if not k.startswith('CINCH_')},
                               'CINCH_HEAP_LIMIT': '256M' if poison else '32M', 'CINCH_STACK_SIZE': '8M',
                               'CINCH_RC_DEBUG': '1' if poison else '0', 'CINCH_LEAK_CHECK': '1' if args.action == 'preflight' else '0',
                               'CINCH_ALLOC_STATS': '1' if args.action == 'preflight' else '0'}
                        command = [str(directory / 'bench'), mode, str(count)] if variant == 'candidate' else [str(reference), 'generate', mode, str(count)]
                        result = execute(command, directory, env)
                        assert result['exit'] == 0, result
                        elapsed, valid, fingerprint = map(int, result['stdout'].split())
                        assert valid == count and elapsed >= 0 and 0 <= fingerprint < 2 ** 64, result
                        if count == 0:
                            assert fingerprint == 14695981039346656037
                        if args.action == 'preflight' and variant == 'candidate':
                            assert 'cinch-leak: live=0 immortal=0 leaked=0' in result['stderr'], result
                        if sample:
                            samples[variant].append(elapsed)
                        if args.action == 'preflight':
                            runs.append({'variant': variant, 'poison': poison, 'valid': valid,
                                         'variable_fingerprint': fingerprint, 'diagnostics': result['stderr']})
            row = {'mode': mode, 'iterations': count}
            if args.action == 'preflight':
                row['runs'] = runs
            else:
                row.update(samples_ns=samples, median_ns={key: statistics.median(value) for key, value in samples.items()})
            rows.append(row)
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps({'action': args.action, 'rows': rows,
                                      'timing_claim': args.action == 'measure', 'cpu_affinity': [args.cpu],
                                      'compares_random_payloads': False}, indent=2) + '\n')
    if args.action == 'preflight':
        report.update(status='preflight-passed-not-measured', preflight_passed=True, preflight_sha256=digest(args.output))
        identity.write_text(json.dumps(report, indent=2) + '\n')


if __name__ == '__main__':
    main()
