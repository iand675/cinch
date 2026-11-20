#!/usr/bin/env python3
"""Resume bounded runtime units for an explicitly separate UUID source candidate and preserve every epoch."""
from __future__ import annotations
import argparse
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
from archive_uuid_unit import save
from validate_uuid import ROOT, digest

COMPILER = Path('/workspace/dasl-stdlib-io/build/stdlib-nominal-region-slots-frozen/cinch')
RUNTIME = COMPILER.with_name('libcinchruntime.a')
CLANG = Path('/workspace/.tools/llvm/usr/lib/llvm-20/bin/clang')


def main() -> None:
    """Keep candidate hashes, independent observations and archive mappings separate from the baseline."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--cpu', type=int, required=True)
    parser.add_argument('--candidate-sources', type=Path, required=True)
    parser.add_argument('--prefix', default='uuid-packed')
    parser.add_argument('--maximum-units', type=int, default=1)
    parser.add_argument('--modes', nargs='+', choices=('parse', 'append', 'compare', 'generated', 'v7', 'control', 'io', 'custom'), default=['parse'])
    parser.add_argument('--consumer', type=Path)
    parser.add_argument('--custom-observations', type=int)
    args = parser.parse_args()
    os.sched_setaffinity(0, {args.cpu})
    assert args.maximum_units > 0 and args.prefix.startswith('uuid-') and '/' not in args.prefix
    assert digest(COMPILER) == '330adb4d77f8bcbfc26f246269280f391c3a10b6d3a40771b5f6243f6a8969b2'
    assert digest(RUNTIME) == 'd91ce012395a99e1148232d20ec316979cd7d0cb13647a3283ba4820192c0abf'
    fixture = ROOT / 'seed/tests/fixtures/uuid-vectors.json'
    assert digest(fixture) == 'af3900f51bd4ed04fc660d988bfa4a36391cac5c58eb9cc3cde2c638df90a44e'
    sources = {}
    for module in ('Data.Uuid', 'Effect.Uuid'):
        candidate = args.candidate_sources.resolve() / (module + '.cinch')
        path = candidate if candidate.is_file() else ROOT / 'seed/prelude' / (module.replace('.', '/') + '.cinch')
        sources[module] = digest(path)
    vectors = json.loads(fixture.read_text())
    progress = ROOT / 'build' / (args.prefix + '-qualification-progress.json')
    ledger = json.loads(progress.read_text()) if progress.exists() else {
        'kind': 'separate-UUID-candidate-runtime-matrix', 'compiler_sha256': digest(COMPILER),
        'runtime_sha256': digest(RUNTIME), 'fixture_sha256': digest(fixture), 'source_sha256': sources,
        'candidate_selected': False, 'package_complete': False, 'units': {}}
    assert ledger['source_sha256'] == sources and ledger['fixture_sha256'] == digest(fixture)
    first = ROOT / 'build' / (args.prefix + '-parse-0-a1-results.json')
    if first.exists() and 'parse-0000' not in ledger['units']:
        value = json.loads(first.read_text())
        assert value['status'] == 'passed' and value['closed_archive']['originals_removed']
        assert all(value['sources'][module] == sha for module, sha in sources.items())
        ledger['units']['parse-0000'] = {'status': 'passed', 'report': str(first.relative_to(ROOT)),
                                       'report_sha256': digest(first), 'independent_observations': 128}
    completed = 0
    for mode in args.modes:
        count = len(vectors[mode]) if mode in vectors else 1
        for start in (range(0, count, 128) if mode not in ('v7', 'control', 'io', 'custom') else (0,)):
            key = f'{mode}-{start:04d}'
            previous = ledger['units'].get(key)
            if previous:
                assert previous['status'] == 'passed', f'Preserved failure at {key}; diagnose before another attempt'
                assert digest(ROOT / previous['report']) == previous['report_sha256']
                if mode == 'custom':
                    assert args.consumer and previous['consumer_sha256'] == digest(args.consumer)
                continue
            if shutil.disk_usage(ROOT).free < 350 * 1024 * 1024:
                ledger['pause'] = {'reason': 'Fresh350MiB guard', 'next_unit': key}
                save(progress, ledger)
                print('Resource pause before compiler')
                return
            attempt = 1
            while (ROOT / 'build' / f'{args.prefix}-{key}-a{attempt}-results.json').exists():
                attempt += 1
            directory = ROOT / 'build' / f'{args.prefix}-{key}-a{attempt}'
            report = directory.with_name(directory.name + '-results.json')
            command = [sys.executable, str(ROOT / 'tooling/validate_uuid.py'), '--compiler', str(COMPILER),
                       '--runtime', str(RUNTIME), '--clang', str(CLANG), '--cpu', str(args.cpu), '--mode', mode,
                       '--start', str(start), '--count', '128', '--candidate-sources', str(args.candidate_sources.resolve()),
                       '--artifacts', str(directory), '--output', str(report)]
            if mode == 'custom':
                assert args.consumer is not None and args.custom_observations and args.custom_observations > 0
                command += ['--consumer', str(args.consumer.resolve())]
            result = subprocess.run(command, cwd=ROOT, timeout=180)
            value = json.loads(report.read_text()) if report.exists() else {}
            if result.returncode == 2 and value.get('status', '').startswith('resource-paused'):
                ledger['pause'] = {'reason': 'Child fresh guard', 'next_unit': key, 'report': str(report.relative_to(ROOT))}
                save(progress, ledger)
                return
            unit = {'status': 'failed', 'report': str(report.relative_to(ROOT)), 'exit': result.returncode}
            ledger['units'][key] = unit
            if result.returncode:
                if report.exists():
                    unit['report_sha256'] = digest(report)
                save(progress, ledger)
                raise RuntimeError(f'Stopped at first candidate failure: {key}')
            assert value['status'] == 'passed' and all(value['sources'][module] == sha for module, sha in sources.items())
            subprocess.run([sys.executable, str(ROOT / 'tooling/archive_uuid_unit.py'), '--report', str(report), '--cpu', str(args.cpu)], cwd=ROOT, check=True, timeout=30)
            unit.update(status='passed', report_sha256=digest(report))
            if mode == 'custom':
                unit.update(consumer_sha256=digest(args.consumer), derived_metamorphic_observations=args.custom_observations,
                            oracle_kind='positional law using actual256SDKbyte classifications; no new reference execution')
            else:
                unit['independent_observations'] = min(128, count - start) if mode in vectors else None
            ledger.pop('pause', None)
            save(progress, ledger)
            completed += 1
            if completed >= args.maximum_units:
                print(f'Natural handoff after {completed} candidate units')
                return
    save(progress, ledger)
    print('Requested candidate units passed; no selection')


if __name__ == '__main__':
    main()
