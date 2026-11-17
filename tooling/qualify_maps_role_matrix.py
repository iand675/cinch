#!/usr/bin/env python3
"""Resume independent annotated-entry map controls and800 trace observations one bounded unit at a time."""
from __future__ import annotations

import argparse
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys

from archive_map_functional_elfs import ROOT, digest, save_index
from qualify_maps_nominal_source import COMPILER_SHA, FROZEN, RUNTIME_SHA


def main() -> None:
    """Stop on the first genuine failure or resource boundary; durably archive each completed success."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--cpu', type=int, required=True)
    parser.add_argument('--maximum-units', type=int, default=1)
    parser.add_argument('--retry-failed', action='store_true', help='Use only after diagnosing a preserved failure')
    args = parser.parse_args()
    assert args.maximum_units > 0
    os.sched_setaffinity(0, {args.cpu})
    assert digest(FROZEN / 'cinch') == COMPILER_SHA and digest(FROZEN / 'libcinchruntime.a') == RUNTIME_SHA
    path = ROOT / 'build/maps-role-slots-matrix-progress.json'
    ledger = json.loads(path.read_text()) if path.exists() else {
        'kind': 'partial-annotated-C839-Role-Slots-runtime-matrix', 'compiler_sha256': COMPILER_SHA,
        'runtime_sha256': RUNTIME_SHA, 'source_sha256': '7085aebe4c10dcd0cf27be6ffc0348a239398b07d54f60cb098722802b244c65',
        'candidate_selected': False, 'package_complete': False, 'units': {}}
    assert ledger['compiler_sha256'] == COMPILER_SHA and ledger['runtime_sha256'] == RUNTIME_SHA
    jobs = [('control', ['--unit', 'control'])] + [(f'trace-{i}', ['--unit', 'trace', '--case', str(i)]) for i in range(8)]
    completed = 0
    for label, options in jobs:
        previous = ledger['units'].get(label)
        if previous and previous['status'] == 'passed':
            assert digest(ROOT / previous['report']) == previous['report_sha256']
            report = json.loads((ROOT / previous['report']).read_text())
            assert report['closed_archive']['originals_removed']
            assert digest(ROOT / report['closed_archive']['archive']) == report['closed_archive']['archive_sha256']
            continue
        if previous and previous['status'] == 'failed' and not args.retry_failed:
            raise RuntimeError(f'Preserved failure at {label} must be diagnosed before retry')
        free = shutil.disk_usage(ROOT).free
        if free < 350 * 1024 * 1024:
            ledger['pause'] = {'reason': 'Fresh350MiB runtime guard', 'next_unit': label, 'free_bytes': free}
            save_index(path, ledger)
            print(f'Resource pause before {label}; no compiler invoked')
            return
        attempt = 1
        while (ROOT / f'build/maps-role-slots-matrix-{label}-a{attempt}-results.json').exists():
            attempt += 1
        artifacts = ROOT / f'build/maps-role-slots-matrix-{label}-a{attempt}'
        report_path = artifacts.with_name(artifacts.name + '-results.json')
        command = [sys.executable, str(ROOT / 'tooling/qualify_maps_nominal_runtime.py'), '--cpu', str(args.cpu),
                   '--maximum-new-output-mib', '30', '--artifacts', str(artifacts), '--output', str(report_path)] + options
        result = subprocess.run(command, cwd=ROOT, timeout=180)
        record = {'status': 'failed', 'exit': result.returncode, 'report': str(report_path.relative_to(ROOT))}
        ledger['units'][label] = record
        if result.returncode:
            if report_path.exists():
                record['report_sha256'] = digest(report_path)
            save_index(path, ledger)
            raise RuntimeError(f'First failure at {label}; original attempt retained')
        report = json.loads(report_path.read_text())
        assert report['status'] == 'passed' and report['unit'] == label and len(report['runs']) == 2
        subprocess.run([sys.executable, str(ROOT / 'tooling/archive_maps_nominal_runtime.py'),
                        '--report', str(report_path), '--cpu', str(args.cpu)], cwd=ROOT, check=True, timeout=30)
        record.update(status='passed', report_sha256=digest(report_path),
                      trace_observations=100 if label.startswith('trace-') else 0,
                      ownership_control_groups=12 if label == 'control' else 0)
        ledger.pop('pause', None)
        save_index(path, ledger)
        completed += 1
        if completed >= args.maximum_units:
            print(f'Natural CPU handoff after {completed} successful bounded units')
            return


if __name__ == '__main__':
    main()
