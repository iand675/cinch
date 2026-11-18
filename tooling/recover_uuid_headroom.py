#!/usr/bin/env python3
"""Recover only the ten approved closed functional outputs through verified durable lossless archives."""
from __future__ import annotations
import hashlib
import json
import os
from pathlib import Path
import resource
import shutil
import stat
import tarfile
from validate_uuid import ROOT, digest
from archive_uuid_unit import save, sync_directory


def protected_inputs() -> dict[str, str]:
    """Protect candidate, vector, frozen compiler/runtime/provider and every sealed benchmark input."""
    manifest = json.loads((ROOT / 'bootstrap/stdlib-uuid-integration-manifest.json').read_text())
    paths = {ROOT / row['path'] for row in manifest['files']}
    for name in ('uuid-public-bench-a1', 'uuid-public-bench-direct-a1', 'uuid-generation-bench-a1'):
        paths.update(path for path in (ROOT / 'build' / name).rglob('*') if path.is_file())
    frozen = Path('/workspace/dasl-stdlib-io/build/stdlib-nominal-region-slots-frozen')
    paths.update(frozen / name for name in ('cinch', 'libcinchruntime.a', 'provenance.json'))
    native = Path('/workspace/dasl-stdlib-string-combined/build/combined-stage3-frozen')
    paths.update(native / name for name in ('cinch', 'frontend.py', 'libcinchruntime.a', 'base-interfaces.json', 'objects.json'))
    objects = json.loads((native / 'objects.json').read_text())
    paths.update(Path(objects[key]) for key in ('object', 'metadata'))
    return {str(path): digest(path) for path in sorted(paths)}


def main() -> None:
    """Verify stored report linkage, preserve archive/index first, and remove only approved exact bytes."""
    os.sched_setaffinity(0, {4})
    resource.setrlimit(resource.RLIMIT_FSIZE, (10 * 1024 * 1024, 10 * 1024 * 1024))
    proposal = ROOT / 'bootstrap/stdlib-uuid-headroom-recovery-proposal.json'
    plan = json.loads(proposal.read_text())
    assert len(plan['files']) == 10 and plan['original_bytes'] == 43152855
    directory = ROOT / 'build/uuid-headroom-recovery-v1'
    directory.mkdir(parents=True, exist_ok=True)
    index_path = directory / 'index.json'
    index = json.loads(index_path.read_text()) if index_path.exists() else {
        'kind': 'verified-approved-own-cold-functional-recovery', 'proposal_sha256': digest(proposal),
        'protected': protected_inputs(), 'entries': [], 'root_ledger_touched': False,
        'free_before': shutil.disk_usage(ROOT).free}
    assert index['proposal_sha256'] == digest(proposal)
    for path, sha in index['protected'].items():
        assert digest(Path(path)) == sha
    save(index_path, index)
    for number, row in enumerate(plan['files']):
        previous = next((entry for entry in index['entries'] if entry['original'] == row['path']), None)
        if previous and previous['original_removed']:
            assert digest(ROOT / previous['archive']) == previous['archive_sha256']
            continue
        assert previous is None, 'Review a crash between durable index and original removal before resuming'
        original, receipt = Path(row['path']), Path(row['receipt'])
        assert digest(receipt) == row['receipt_sha256']
        assert not original.is_symlink() and original.is_file()
        assert (digest(original), original.stat().st_size, stat.S_IMODE(original.stat().st_mode)) == (row['sha256'], row['bytes'], row['mode'])
        assert shutil.disk_usage(ROOT).free >= 260 * 1024 * 1024
        key = hashlib.sha256(str(original).encode()).hexdigest()[:16]
        archive = directory / (key + '.tar.gz')
        temporary = archive.with_suffix('.gz.tmp')
        snapshot = directory / (key + '.receipt.json')
        assert not archive.exists() and not temporary.exists() and not snapshot.exists()
        with snapshot.open('xb') as stream:
            stream.write(receipt.read_bytes()); stream.flush(); os.fsync(stream.fileno())
        member_name = str(original.relative_to('/workspace'))
        with temporary.open('xb') as stream:
            with tarfile.open(fileobj=stream, mode='w:gz', compresslevel=1) as packed:
                packed.add(original, arcname=member_name, recursive=False)
            stream.flush(); os.fsync(stream.fileno())
        assert temporary.stat().st_size <= 10 * 1024 * 1024
        assert shutil.disk_usage(ROOT).free >= 250 * 1024 * 1024
        with tarfile.open(temporary, 'r:gz') as packed:
            members = packed.getmembers()
            assert len(members) == 1
            member = members[0]
            assert member.isfile() and member.name == member_name and member.mode == row['mode'] and member.size == row['bytes']
            checksum, size = hashlib.sha256(), 0
            stream = packed.extractfile(member)
            while block := stream.read(1048576):
                checksum.update(block); size += len(block)
            assert (checksum.hexdigest(), size) == (row['sha256'], row['bytes'])
        os.replace(temporary, archive)
        sync_directory(directory)
        entry = {'original': str(original), 'original_bytes': row['bytes'], 'original_sha256': row['sha256'],
                 'original_mode': row['mode'], 'member': member_name, 'receipt': str(receipt),
                 'receipt_sha256': row['receipt_sha256'], 'snapshot': str(snapshot.relative_to(ROOT)),
                 'snapshot_sha256': digest(snapshot), 'archive': str(archive.relative_to(ROOT)),
                 'archive_sha256': digest(archive), 'archive_bytes': archive.stat().st_size,
                 'original_removed': False}
        index['entries'].append(entry)
        save(index_path, index)
        assert digest(original) == row['sha256']
        original.unlink(); sync_directory(original.parent)
        entry['original_removed'] = True
        save(index_path, index)
        if number == 0:
            print(f'First verified recovery net {row["bytes"] - entry["archive_bytes"]} bytes', flush=True)
    for path, sha in index['protected'].items():
        assert digest(Path(path)) == sha
    original_bytes = sum(entry['original_bytes'] for entry in index['entries'])
    archive_bytes = sum(entry['archive_bytes'] for entry in index['entries'])
    index.update(status='completed', original_bytes=original_bytes, compressed_bytes=archive_bytes,
                 net_original_minus_archive_bytes=original_bytes - archive_bytes,
                 protected_rechecks=len(index['protected']), free_after=shutil.disk_usage(ROOT).free)
    save(index_path, index)
    print(json.dumps({key: index[key] for key in ('status', 'original_bytes', 'compressed_bytes', 'net_original_minus_archive_bytes', 'protected_rechecks', 'free_after')}, indent=2))


if __name__ == '__main__':
    main()
