#!/usr/bin/env python3
"""Exercise qualified producer reuse against real snapshots and contradictory provenance/source inputs."""
from __future__ import annotations

import argparse
import copy
from pathlib import Path
import shutil
import tempfile

import bootstrap
from build_base_artifact_from_snapshot import verify_snapshot


def main() -> None:
    """Check trusted reuse and independent digest, commit, source-claim and embedded-source refusals."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--snapshot', type=Path, required=True)
    parser.add_argument('--provenance', type=Path, required=True)
    parser.add_argument('--repository', type=Path, required=True)
    parser.add_argument('--checkpoint', required=True)
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    identity, _ = verify_snapshot(args.snapshot, args.provenance, args.repository, args.checkpoint)
    report = {'status': 'running', 'producer': identity, 'observations': [{'name': 'certified-snapshot', 'accepted': True}]}
    original = bootstrap.read_json(args.provenance)
    with tempfile.TemporaryDirectory(prefix='cinch-snapshot-contract-') as temporary:
        directory = Path(temporary)
        for key in ['compiler_sha256', 'runtime_sha256', 'source_checkpoint', 'source_sha256']:
            wrong = copy.deepcopy(original)
            if key == 'source_checkpoint':
                wrong[key] = identity['source_checkpoint'] + '^'
            elif key == 'source_sha256':
                wrong[key]['compiler/src/codegen.rs'] = '0' * 64
            else:
                wrong[key] = '0' * 64
            path = directory / (key + '.json')
            bootstrap.write_json(path, wrong)
            try:
                verify_snapshot(args.snapshot, path, args.repository, args.checkpoint)
            except bootstrap.BootstrapError as error:
                report['observations'].append({'name': key + '-mismatch', 'accepted': False, 'diagnostic': str(error)})
            else:
                raise bootstrap.BootstrapError('contradictory snapshot provenance was accepted: ' + key)
        retained_root = bootstrap.ROOT
        local = directory / 'lane'
        shutil.copytree(retained_root / 'compiler/prelude', local / 'compiler/prelude')
        bootstrap.ROOT = local
        try:
            for name in ['cinch-base.cinchpkg', 'Data/String.cinch']:
                path = local / 'compiler/prelude' / name
                before = path.read_bytes()
                path.write_bytes(before + b'\n')
                try:
                    verify_snapshot(args.snapshot, args.provenance, args.repository, args.checkpoint)
                except bootstrap.BootstrapError as error:
                    report['observations'].append({'name': name + '-mismatch', 'accepted': False, 'diagnostic': str(error)})
                else:
                    raise bootstrap.BootstrapError('different embedded source was accepted: ' + name)
                finally:
                    path.write_bytes(before)
        finally:
            bootstrap.ROOT = retained_root
    report['status'] = 'passed'
    bootstrap.write_json(args.output, report)
    print(f'{len(report["observations"])} qualified snapshot source/provenance observations passed.')


if __name__ == '__main__':
    main()
