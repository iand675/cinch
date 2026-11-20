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
    """Check original/current receipt schemas and named identity, source and embedded-byte refusals."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--snapshot', type=Path, required=True)
    parser.add_argument('--provenance', type=Path, required=True)
    parser.add_argument('--repository', type=Path, required=True)
    parser.add_argument('--checkpoint', required=True)
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    args.output.parent.mkdir(parents=True, exist_ok=True)
    identity, _ = verify_snapshot(args.snapshot, args.provenance, args.repository, args.checkpoint)
    report = {'status': 'running', 'producer': identity,
              'observations': [{'name': 'certified-snapshot', 'accepted': True}]}
    original = bootstrap.read_json(args.provenance)
    source_keys = [key for key in ('source_sha256', 'rust_source_sha256') if key in original]
    with tempfile.TemporaryDirectory(prefix='snapshot-contract-', dir=args.output.parent) as temporary:
        directory = Path(temporary)

        def observation(name: str, document: dict, expected: str | None) -> None:
            """Require the exact refusal category or an explicitly expected successful claim combination."""
            path = directory / (name + '.json')
            bootstrap.write_json(path, document)
            try:
                actual, _ = verify_snapshot(args.snapshot, path, args.repository, args.checkpoint)
            except bootstrap.BootstrapError as error:
                if expected is None or expected not in str(error):
                    raise bootstrap.BootstrapError(f'{name}: unexpected refusal: {error}') from error
                report['observations'].append({'name': name, 'accepted': False, 'diagnostic': str(error)})
            else:
                if expected is not None:
                    raise bootstrap.BootstrapError('contradictory snapshot provenance was accepted: ' + name)
                report['observations'].append({'name': name, 'accepted': True,
                                               'claim_verification': actual['claim_verification']})

        for key in ('compiler_sha256', 'runtime_sha256', 'source_checkpoint'):
            wrong = copy.deepcopy(original)
            wrong[key] = identity['source_checkpoint'] + '^' if key == 'source_checkpoint' else '0' * 64
            expected = 'different source checkpoint' if key == 'source_checkpoint' else 'snapshot digest mismatch'
            observation(key + '-mismatch', wrong, expected)
        for key in source_keys:
            wrong = copy.deepcopy(original)
            selected = 'seed/src/codegen.rs' if 'seed/src/codegen.rs' in wrong[key] else next(iter(wrong[key]))
            wrong[key][selected] = '0' * 64
            observation(key + '-mismatch', wrong, 'source claim mismatch')
        selected = source_keys[0]
        for name, value in [('empty-source-map', {}), ('malformed-source-map', 'not-a-map')]:
            wrong = copy.deepcopy(original)
            wrong[selected] = value
            observation(name, wrong, 'empty or malformed source claim map')
        wrong = copy.deepcopy(original)
        for key in source_keys:
            del wrong[key]
        observation('missing-source-claims', wrong, 'no committed source claims')
        agreeing = copy.deepcopy(original)
        claims = copy.deepcopy(original.get('rust_source_sha256', original.get('source_sha256')))
        agreeing['source_sha256'] = copy.deepcopy(claims)
        agreeing['rust_source_sha256'] = copy.deepcopy(claims)
        observation('both-source-maps-agree', agreeing, None)
        wrong = copy.deepcopy(agreeing)
        first = next(iter(claims))
        wrong['source_sha256'][first] = '0' * 64
        observation('source-maps-conflict', wrong, 'conflicting qualified snapshot source claims')
        wrong = copy.deepcopy(original)
        wrong[selected]['seed/src/absent-snapshot-file.rs'] = '0' * 64
        observation('source-claim-outside-commit', wrong, 'source claim mismatch')
        if 'rust_source_manifest_sha256' in original:
            wrong = copy.deepcopy(original)
            wrong['rust_source_manifest_sha256'] = '0' * 64
            observation('rust-manifest-mismatch', wrong, 'Rust source manifest digest mismatch')
            wrong = copy.deepcopy(original)
            wrong['source_sha256'] = copy.deepcopy(wrong['rust_source_sha256'])
            del wrong['rust_source_sha256']
            observation('rust-manifest-without-map', wrong, 'manifest has no corresponding source claim map')
        if 'library_source_sha256' in original:
            wrong = copy.deepcopy(original)
            wrong['library_source_sha256']['Data.String'] = '0' * 64
            observation('library-source-mismatch', wrong, 'library source claim mismatch: Data.String')
            for name, value in [('empty-library-map', {}), ('malformed-library-map', [])]:
                wrong = copy.deepcopy(original)
                wrong['library_source_sha256'] = value
                observation(name, wrong, 'empty or malformed library source claim map')

        retained_root = bootstrap.ROOT
        local = directory / 'lane'
        prelude = local / 'seed/prelude'
        prelude.mkdir(parents=True)
        package_path = retained_root / 'seed/prelude/cinch-base.cinchpkg'
        shutil.copy2(package_path, prelude / package_path.name)
        for module in bootstrap.read_json(package_path)['modules']:
            destination = prelude / module['path']
            destination.parent.mkdir(parents=True, exist_ok=True)
            shutil.copy2(retained_root / 'seed/prelude' / module['path'], destination)
        bootstrap.ROOT = local
        try:
            for name in ('cinch-base.cinchpkg', 'Data/String.cinch'):
                path = prelude / name
                before = path.read_bytes()
                path.write_bytes(before + b'\n')
                try:
                    verify_snapshot(args.snapshot, args.provenance, args.repository, args.checkpoint)
                except bootstrap.BootstrapError as error:
                    expected = 'base package differs' if name.endswith('.cinchpkg') else 'embedded module differs'
                    if expected not in str(error):
                        raise bootstrap.BootstrapError(f'{name}: unexpected source refusal: {error}') from error
                    report['observations'].append({'name': name + '-mismatch', 'accepted': False, 'diagnostic': str(error)})
                else:
                    raise bootstrap.BootstrapError('different embedded source was accepted: ' + name)
                finally:
                    path.write_bytes(before)
        finally:
            bootstrap.ROOT = retained_root
        report['physical_fixture_bytes'] = sum(path.stat().st_size for path in directory.rglob('*') if path.is_file())
        if report['physical_fixture_bytes'] > 5 * 1024 * 1024:
            raise bootstrap.BootstrapError('snapshot verification fixtures exceeded the 5MiB budget')
    report['status'] = 'passed'
    bootstrap.write_json(args.output, report)
    print(f'{len(report["observations"])} qualified snapshot source/provenance observations passed.')


if __name__ == '__main__':
    main()
