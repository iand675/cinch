#!/usr/bin/env python3
"""Recheck real callback/storage contracts against an explicitly frozen source gate.

Positive clients preserve complete IO/Mut ownership and old-factory widening.
Four negative clients must refuse mutation erasure; their failed check artifacts
stay plain. This is a Rust source gate, not a Native-family admission claim.
"""
from __future__ import annotations
import argparse
import json
import os
from pathlib import Path
import resource
import shutil
import subprocess
from check_compare import digest
from multipart_qualification_support import archive_cache


#: Maintained historical contract fixtures; exact copies are frozen for each new epoch.
ROOT = Path(__file__).resolve().parents[1]


def main() -> None:
    """Require fresh exact source identity and preserve every first genuine refusal."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--source-work', type=Path, required=True)
    parser.add_argument('--work', type=Path, required=True)
    parser.add_argument('--compiler', type=Path, required=True)
    parser.add_argument('--cpu', type=int, required=True)
    args = parser.parse_args()
    os.sched_setaffinity(0, {args.cpu})
    resource.setrlimit(resource.RLIMIT_STACK, (67108864, resource.getrlimit(resource.RLIMIT_STACK)[1]))
    source = args.source_work.resolve(); gate = json.loads((source / 'report.json').read_text())
    compiler = args.compiler.resolve(); work = args.work.resolve(); work.mkdir(parents=True, exist_ok=True)
    if not any(row['returncode'] == 0 and row['compiler_sha256'] == digest(compiler) for row in gate['checks']):
        raise RuntimeError('no fresh exact source admission')
    if shutil.disk_usage(work).free < 350 * 1024 * 1024:
        raise RuntimeError('resource pause before source checks')
    report = {'scope': __doc__, 'compiler_sha256': digest(compiler), 'source_gate_sha256': digest(source / 'report.json'),
              'sources': gate['sources'], 'checks': [], 'production_selected': False}
    for client in sorted((ROOT / 'experiments/multipart-both-effects-v1/clients').glob('*.cinch')):
        unit = work / client.stem; unit.mkdir(exist_ok=True)
        for name, expected in gate['sources'].items():
            path = source / (name + '.cinch')
            if digest(path) != expected:
                raise RuntimeError('changed frozen source')
            shutil.copyfile(path, unit / path.name)
        shutil.copyfile(client, unit / 'Main.cinch')
        if shutil.disk_usage(work).free < 250 * 1024 * 1024 or sum(path.stat().st_size for path in work.rglob('*') if path.is_file()) > 30 * 1024 * 1024:
            raise RuntimeError('resource pause before next source check')
        command = [str(compiler), 'check', str(unit / 'Main.cinch')]
        result = subprocess.run(command, cwd=unit, capture_output=True, text=True, timeout=60,
                                env={**os.environ, 'CINCH_STACK_SIZE': '64M', 'RUST_MIN_STACK': '67108864'})
        expected = client.stem == 'positive'
        row = {'client': client.stem, 'client_sha256': digest(client), 'command': command,
               'expected_acceptance': expected, 'returncode': result.returncode,
               'stdout': result.stdout, 'stderr': result.stderr}
        report['checks'].append(row)
        if result.returncode == 0:
            archive_cache(unit, 'accepted-row-contract', row)
        row['plain_failure_artifacts_retained'] = result.returncode != 0
        row['matched'] = (result.returncode == 0) == expected and (expected or 'effect' in result.stderr.lower())
        (work / 'report.json').write_text(json.dumps(report, indent=2) + '\n')
        if not row['matched']:
            raise RuntimeError('unexpected callback row outcome: ' + client.stem + '\n' + result.stderr)
    print('positive full contracts and four intended IO-only refusals passed', flush=True)


if __name__ == '__main__':
    main()
