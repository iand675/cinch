#!/usr/bin/env python3
"""Emit a current base artifact with an explicitly qualified immutable Rust producer.

The selected snapshot is a trusted prior compiler build, not a new compilation.
Its committed source envelope, embedded base sources, compiler and runtime are
verified before use. This workflow never changes pins or builds a Cargo cache.
"""
from __future__ import annotations

import argparse
import io
import json
from pathlib import Path
import shutil
import subprocess
import tarfile

import bootstrap
import bootstrap_objects
from bootstrap_interfaces import content_hash


def committed_sources(repository: Path, checkpoint: str) -> tuple[str, dict[str, bytes]]:
    """Read exactly the seed-source envelope from an immutable commit without consulting working files."""
    revision = subprocess.check_output(
        ['git', '-C', str(repository), 'rev-parse', '--verify', checkpoint + '^{commit}'], text=True).strip()
    paths = ['compiler/src', 'compiler/prelude', 'compiler/runtime/src', 'compiler/Cargo.toml',
             'compiler/Cargo.lock', 'compiler/.cargo/config.toml', 'compiler/runtime/Cargo.toml',
             'compiler/runtime/Cargo.lock', bootstrap.NATIVE_BUILD_SCRIPT]
    paths.extend(bootstrap.native_grammar_inputs(repository, revision))
    archive = subprocess.check_output(['git', '-C', str(repository), 'archive', '--format=tar', revision, *paths])
    files = {}
    with tarfile.open(fileobj=io.BytesIO(archive)) as tree:
        for item in tree:
            if item.isdir():
                continue
            if not item.isfile() or item.name.startswith('/') or '..' in Path(item.name).parts:
                raise bootstrap.BootstrapError('producer source envelope contains a nonregular or unsafe entry')
            stream = tree.extractfile(item)
            if stream is None:
                raise bootstrap.BootstrapError('producer source entry cannot be read: ' + item.name)
            files[item.name] = stream.read()
    return revision, files


def verify_source_claims(provenance: dict, files: dict[str, bytes]) -> dict:
    """Verify every legacy/current claim map against committed bytes without changing the trusted receipt."""
    merged: dict[str, str] = {}
    fields = []
    for key in ('source_sha256', 'rust_source_sha256'):
        if key not in provenance:
            continue
        claims = provenance[key]
        if not isinstance(claims, dict) or not claims:
            raise bootstrap.BootstrapError('qualified snapshot has an empty or malformed source claim map: ' + key)
        fields.append(key)
        for name, expected in claims.items():
            if not isinstance(name, str) or not name or not isinstance(expected, str) or len(expected) != 64:
                raise bootstrap.BootstrapError('qualified snapshot has a malformed source claim: ' + key)
            if name in merged and merged[name] != expected:
                raise bootstrap.BootstrapError('conflicting qualified snapshot source claims: ' + name)
            merged[name] = expected
    if not fields:
        raise bootstrap.BootstrapError('qualified snapshot has no committed source claims')
    for name, expected in merged.items():
        if name not in files or bootstrap.digest(files[name]) != expected:
            raise bootstrap.BootstrapError('qualified snapshot source claim mismatch: ' + name)
    manifest_verified = False
    if 'rust_source_manifest_sha256' in provenance:
        if 'rust_source_sha256' not in provenance:
            raise bootstrap.BootstrapError('qualified Rust manifest has no corresponding source claim map')
        manifest = json.dumps(provenance['rust_source_sha256'], sort_keys=True, separators=(',', ':')).encode()
        if bootstrap.digest(manifest) != provenance['rust_source_manifest_sha256']:
            raise bootstrap.BootstrapError('qualified Rust source manifest digest mismatch')
        manifest_verified = True
    library_count = 0
    if 'library_source_sha256' in provenance:
        claims = provenance['library_source_sha256']
        if not isinstance(claims, dict) or not claims:
            raise bootstrap.BootstrapError('qualified snapshot has an empty or malformed library source claim map')
        for module, expected in claims.items():
            if (not isinstance(module, str) or not module or '/' in module or '\\' in module
                    or any(not part for part in module.split('.')) or not isinstance(expected, str) or len(expected) != 64):
                raise bootstrap.BootstrapError('qualified snapshot has a malformed library source claim')
            name = 'compiler/prelude/' + module.replace('.', '/') + '.cinch'
            if name not in files or bootstrap.digest(files[name]) != expected:
                raise bootstrap.BootstrapError('qualified library source claim mismatch: ' + module)
        library_count = len(claims)
    return {'source_claim_fields': fields, 'source_claim_count': len(merged),
            'rust_manifest_verified': manifest_verified, 'library_source_claim_count': library_count}


def verify_snapshot(snapshot: Path, provenance_path: Path, repository: Path, checkpoint: str) -> tuple[dict, dict[str, bytes]]:
    """Require certified binary/runtime identities and every embedded module to match the committed epoch."""
    provenance = bootstrap.read_json(provenance_path)
    revision, files = committed_sources(repository, checkpoint)
    certified = subprocess.check_output(['git', '-C', str(repository), 'rev-parse', '--verify',
                                        provenance['source_checkpoint'] + '^{commit}'], text=True).strip()
    if revision != certified:
        raise bootstrap.BootstrapError('snapshot provenance names a different source checkpoint')
    for name, key in [('cinch', 'compiler_sha256'), ('libcinchruntime.a', 'runtime_sha256')]:
        if bootstrap.file_hash(snapshot / name) != provenance[key]:
            raise bootstrap.BootstrapError('qualified snapshot digest mismatch: ' + name)
    claim_verification = verify_source_claims(provenance, files)
    package_name = 'compiler/prelude/cinch-base.cinchpkg'
    package_path = bootstrap.ROOT / package_name
    if files[package_name] != package_path.read_bytes():
        raise bootstrap.BootstrapError('qualified snapshot base package differs from this lane')
    package = bootstrap.read_json(package_path)
    if len({row['name'] for row in package['modules']}) != len(package['modules']):
        raise bootstrap.BootstrapError('qualified snapshot base package has duplicate modules')
    modules = {}
    for row in package['modules']:
        name = 'compiler/prelude/' + row['path']
        if files[name] != (bootstrap.ROOT / name).read_bytes():
            raise bootstrap.BootstrapError('qualified embedded module differs from this lane: ' + row['name'])
        modules[row['name']] = {'path': row['path'], 'sha256': bootstrap.digest(files[name])}
    identity = {'kind': 'qualified-producer-snapshot-reuse', 'source_checkpoint': revision,
                'source_sha256': bootstrap.source_hash(files),
                'compiler_sha256': bootstrap.file_hash(snapshot / 'cinch'),
                'runtime_sha256': bootstrap.file_hash(snapshot / 'libcinchruntime.a'),
                'runtime_source_sha256': bootstrap.source_hash({name: data for name, data in files.items()
                                                               if name.startswith('compiler/runtime/')}),
                'provenance_sha256': bootstrap.file_hash(provenance_path), 'provenance': provenance,
                'claim_verification': claim_verification,
                'embedded_sources': modules, 'new_compiler_build_performed': False,
                'native_family_admission': False,
                'source_files': {name: bootstrap.digest(data) for name, data in sorted(files.items())}}
    return identity, files


def emit(output: Path, snapshot: Path, provenance_path: Path, repository: Path, checkpoint: str) -> Path:
    """Emit actual rooted units and admit their source/interface/object closure without a fallback producer."""
    identity, files = verify_snapshot(snapshot, provenance_path, repository, checkpoint)
    output.mkdir(parents=True, exist_ok=False)
    bootstrap.write_json(output / 'producer-snapshot-verification.json', identity)
    shutil.copy2(provenance_path, output / 'producer-provenance.json')
    for name, key in [('cinch', 'compiler_sha256'), ('libcinchruntime.a', 'runtime_sha256')]:
        shutil.copy2(snapshot / name, output / name)
        if bootstrap.file_hash(output / name) != identity[key]:
            raise bootstrap.BootstrapError('copied qualified snapshot changed: ' + name)
    package_path = bootstrap.ROOT / 'compiler/prelude/cinch-base.cinchpkg'
    package = bootstrap.read_json(package_path)
    sources = {}
    for module in package['modules']:
        relative = 'base/' + module['path']
        destination = output / relative
        destination.parent.mkdir(parents=True, exist_ok=True)
        destination.write_bytes(files['compiler/prelude/' + module['path']])
        sources[module['name']] = {'path': relative, 'sha256': bootstrap.file_hash(destination)}
    shutil.copy2(package_path, output / 'base/cinch-base.cinchpkg')
    shutil.copy2(bootstrap.ROOT / 'bootstrap/pins/library-object-roots.json', output / 'roots.json')
    library = output / 'library'
    library.mkdir()
    entry = library / 'CinchBaseObject.cinch'
    entry.write_bytes(bootstrap_objects.ARTIFACT_ENTRY)
    command = [str(output / 'cinch'), 'emit-object', str(entry), '--object-roots', str(output / 'roots.json'),
               '--opt-level', '2', '-o', str(output / 'library.o')]
    with (output / 'object-build.log').open('w') as log:
        subprocess.run(command, cwd=library, env=bootstrap.environment(), stdout=log,
                       stderr=subprocess.STDOUT, check=True)
    verified_after, _ = verify_snapshot(snapshot, provenance_path, repository, checkpoint)
    if verified_after != identity:
        raise bootstrap.BootstrapError('qualified producer inputs changed during object emission')
    metadata = bootstrap.read_json(output / 'library.abi.json')
    bootstrap.write_json(output / 'interfaces.json', bootstrap_objects.admitted_interfaces(bootstrap, metadata, package, output))
    retained = {'cinch', 'libcinchruntime.a', 'library.o', 'library.ll', 'library.abi.json', 'interfaces.json',
                'base/cinch-base.cinchpkg', 'roots.json', 'library/CinchBaseObject.cinch', 'object-build.log',
                'producer-provenance.json', 'producer-snapshot-verification.json'}
    retained.update(row['path'] for row in sources.values())
    retained.update(str(path.relative_to(output)) for path in (library / 'build').glob('*.cinchif'))
    artifacts = {name: bootstrap.file_hash(output / name) for name in sorted(retained)}
    receipt = {'schema': 1, 'kind': 'current-base-artifact', 'host': bootstrap.HOST,
               'target': metadata['target'], 'runtime_abi': metadata['runtime_abi'],
               'producer': {'mode': 'fresh-embedded', 'compiler_sha256': artifacts['cinch'],
                            'source_sha256': identity['source_sha256'], 'snapshot_reuse': identity,
                            'commands': [{'command': command, 'cwd': str(library)}],
                            'builder_script_sha256': bootstrap.file_hash(Path(__file__))},
               'runtime': {'runtime_abi': metadata['runtime_abi'], 'sha256': artifacts['libcinchruntime.a'],
                           'source_sha256': identity['runtime_source_sha256']},
               'sources': sources, 'artifacts': artifacts}
    path = output / 'provider.json'
    bootstrap.write_json(path, receipt)
    bootstrap_objects.artifact_provider(bootstrap, path)
    bootstrap.write_json(output / 'source-proof.json', {
        'producer_snapshot_reused': True, 'native_family_admission': False, 'source_shadow_used': False,
        'source_checkpoint': identity['source_checkpoint'], 'source_sha256': identity['source_sha256'],
        'string_sha256': sources['Data.String']['sha256'],
        'string_source_hash': content_hash((output / sources['Data.String']['path']).read_bytes()),
        'provider_receipt_sha256': bootstrap.file_hash(path)})
    return path


def main() -> None:
    """Require an explicit snapshot and committed epoch for both preflight and real artifact emission."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--snapshot', type=Path, required=True, help='qualified directory containing cinch and its matched runtime')
    parser.add_argument('--provenance', type=Path, required=True, help='trusted prior build receipt with binary/runtime/source identities')
    parser.add_argument('--repository', type=Path, required=True, help='repository holding the certified committed source epoch')
    parser.add_argument('--checkpoint', required=True, help='exact committed producer source revision; working files are never used')
    parser.add_argument('--output', type=Path, required=True, help='new artifact directory, or verification JSON with --verify-only')
    parser.add_argument('--verify-only', action='store_true', help='check provenance and all embedded modules without emitting objects')
    args = parser.parse_args()
    if args.verify_only:
        identity, _ = verify_snapshot(args.snapshot.resolve(), args.provenance.resolve(), args.repository.resolve(), args.checkpoint)
        if args.output.exists():
            raise bootstrap.BootstrapError('verification output already exists')
        bootstrap.write_json(args.output, identity)
        print(args.output)
    else:
        print(emit(args.output.resolve(), args.snapshot.resolve(), args.provenance.resolve(), args.repository.resolve(), args.checkpoint))


if __name__ == '__main__':
    main()
