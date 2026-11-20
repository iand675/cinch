#!/usr/bin/env python3
"""Resume bounded pure UUID observations on one historical Native tuple without adopting its IO provider."""
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


def main() -> None:
    """Preserve every verdict and stop at a failure, resource guard or explicit unit boundary."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--cpu', type=int, required=True)
    parser.add_argument('--maximum-units', type=int, default=1)
    parser.add_argument('--modes', nargs='+', choices=('parse', 'append', 'compare', 'generated', 'v7', 'control'),
                        default=['parse', 'append', 'compare', 'generated', 'v7', 'control'])
    args = parser.parse_args()
    assert args.maximum_units > 0
    os.sched_setaffinity(0, {args.cpu})
    fixture = ROOT / 'seed/tests/fixtures/uuid-vectors.json'
    assert digest(fixture) == 'af3900f51bd4ed04fc660d988bfa4a36391cac5c58eb9cc3cde2c638df90a44e'
    vectors = json.loads(fixture.read_text())
    path = ROOT / 'build/uuid-native-qualification-progress.json'
    ledger = json.loads(path.read_text()) if path.exists() else {
        'kind': 'partial-historical-Native-pure-UUID-matrix',
        'pair_review_sha256': digest(ROOT / 'compiler/evidence/stdlib/stdlib-uuid-native-pair-review.json'),
        'fixture_sha256': digest(fixture),
        'source_sha256': digest(ROOT / 'experiments/uuid-candidate-v1/sources/Data.Uuid.cinch'),
        'candidate_selected': False, 'package_complete': False,
        'new66_family_relabelled': False, 'io_or_mvar_requested': False, 'units': {}}
    assert ledger['pair_review_sha256'] == digest(ROOT / 'compiler/evidence/stdlib/stdlib-uuid-native-pair-review.json')
    assert ledger['source_sha256'] == digest(ROOT / 'experiments/uuid-candidate-v1/sources/Data.Uuid.cinch')
    assert ledger['source_sha256'] == digest(ROOT / 'seed/prelude/Data/Uuid.cinch')
    first = ROOT / 'build/uuid-native-parse-0-a1-results.json'
    if first.exists() and 'parse-0000' not in ledger['units']:
        report = json.loads(first.read_text())
        assert report['status'] == 'passed' and report['closed_archive']['originals_removed']
        assert report['pair_review_sha256'] == ledger['pair_review_sha256']
        assert report['sources']['Data.Uuid.cinch'] == ledger['source_sha256']
        ledger['units']['parse-0000'] = {'status': 'passed', 'report': str(first.relative_to(ROOT)),
                                       'report_sha256': digest(first), 'observations': 128}
    completed = 0
    for mode in args.modes:
        count = len(vectors[mode]) if mode in vectors else 1
        for start in (range(0, count, 128) if mode not in ('control', 'v7') else (0,)):
            key = f'{mode}-{start:04d}'
            previous = ledger['units'].get(key)
            if previous:
                assert previous['status'] == 'passed', f'Preserved failure at {key}; diagnose before retrying'
                assert digest(ROOT / previous['report']) == previous['report_sha256']
                continue
            free = shutil.disk_usage(ROOT).free
            if free < 350 * 1024 * 1024:
                ledger['pause'] = {'next_unit': key, 'reason': 'Fresh350MiB guard', 'free_bytes': free}
                save(path, ledger)
                print(f'Resource pause before {key}; no compiler invoked')
                return
            artifacts = ROOT / f'build/uuid-native-{key}-a1'
            report = artifacts.with_name(artifacts.name + '-results.json')
            assert not report.exists() and not artifacts.exists(), 'Preserve every previous attempt'
            result = subprocess.run([sys.executable, str(ROOT / 'tooling/validate_uuid_native.py'),
                                     '--mode', mode, '--start', str(start), '--count', '128',
                                     '--cpu', str(args.cpu), '--clang', '/workspace/.tools/llvm/usr/lib/llvm-20/bin/clang',
                                     '--artifacts', str(artifacts), '--output', str(report)], cwd=ROOT, timeout=180)
            unit = {'status': 'failed', 'report': str(report.relative_to(ROOT)), 'exit': result.returncode}
            ledger['units'][key] = unit
            if result.returncode:
                if report.exists():
                    unit['report_sha256'] = digest(report)
                save(path, ledger)
                raise RuntimeError(f'Stopped at first failure: {key}')
            value = json.loads(report.read_text())
            assert value['status'] == 'passed' and len(value['runs']) == 2
            subprocess.run([sys.executable, str(ROOT / 'tooling/archive_uuid_unit.py'),
                            '--report', str(report), '--cpu', str(args.cpu)], cwd=ROOT, check=True, timeout=30)
            unit.update(status='passed', report_sha256=digest(report),
                        observations=min(128, count - start) if mode != 'control' else None)
            ledger.pop('pause', None)
            save(path, ledger)
            completed += 1
            if completed >= args.maximum_units:
                print(f'Natural handoff after {completed} bounded historical Native units')
                return
    save(path, ledger)
    print('All requested historical pure units passed; no IO/new-family adoption')


if __name__ == '__main__':
    main()
