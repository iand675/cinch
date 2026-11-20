#!/usr/bin/env python3
"""Archive exactly two approved successful context-test outputs with byte and complete file-metadata checks."""
from __future__ import annotations
from decimal import Decimal
import hashlib
import json
import os
from pathlib import Path
import resource
import shutil
import stat
import tarfile
from archive_uuid_unit import save, sync_directory
from recover_uuid_headroom import protected_inputs
from validate_uuid import ROOT, digest


def current_users(paths: set[Path]) -> list[str]:
    """Refuse an executable in use or a process working on these exact output files."""
    users = []
    for process in Path('/proc').glob('[0-9]*'):
        try:
            if (process / 'exe').resolve() in paths:
                users.append(process.name)
            cwd = (process / 'cwd').resolve()
            arguments = (process / 'cmdline').read_bytes().split(b'\0')
            for argument in arguments:
                if argument:
                    path = Path(os.fsdecode(argument))
                    if (path if path.is_absolute() else cwd / path).resolve() in paths:
                        users.append(process.name)
        except (OSError, PermissionError):
            continue
    return users


def main() -> None:
    """Persist archive and private index before unlink; preserve every source, failure and frozen input."""
    os.sched_setaffinity(0, {4})
    resource.setrlimit(resource.RLIMIT_FSIZE, (10 * 1024 * 1024, 10 * 1024 * 1024))
    old_root = Path('/workspace/dasl-stdlib-ring-positivity')
    receipt = old_root / 'bootstrap/evidence/other/sealed-array-positivity-context-validation-results.json'
    assert digest(receipt) == '59e5b7944da19d0b8b847ff2aed8bef75065673af5e3d8ee4eaf885ff2230c06'
    description = json.loads(receipt.read_text())
    variants = description['variants']
    matching = [row for row in variants.values() if row.get('executable_sha256') == 'b72533d2add169db28340050fdf9434c44cea4ee05b616eeeeab1aedc856767e']
    assert len(matching) == 1
    row = matching[0]
    assert len(row['results']) == 2 and all(run['exit'] == 0 and 'leaked=0' in run['diagnostics'] for run in row['results'])
    directory = ROOT / 'build/uuid-headroom-recovery-v2'
    assert not directory.exists(), 'Inspect any partial recovery before another attempt'
    directory.mkdir(parents=True)
    protected = protected_inputs()
    protected.update({str(path): digest(path) for path in (ROOT / 'experiments/uuid-packed-parser-v1').rglob('*') if path.is_file()})
    index = {'kind': 'exact-two-successful-context-consumer-recovery', 'receipt': str(receipt),
             'receipt_sha256': digest(receipt), 'free_before': shutil.disk_usage(ROOT).free,
             'protected': protected, 'entries': [], 'root_ledger_touched': False}
    index_path = directory / 'index.json'
    save(index_path, index)
    paths = [old_root / 'build/array-payload-context-after' / name for name in ('consumer', 'consumer.ll')]
    assert not current_users(set(paths))
    for original, key in zip(paths, ('executable_sha256', 'llvm_sha256')):
        assert original.is_file() and not original.is_symlink() and digest(original) == row[key]
        metadata = original.stat()
        assert metadata.st_size <= 15 * 1024 * 1024
        assert shutil.disk_usage(ROOT).free >= 260 * 1024 * 1024
        mode = stat.S_IMODE(metadata.st_mode)
        seconds, nanos = divmod(metadata.st_mtime_ns, 1000000000)
        exact_mtime = f'{seconds}.{nanos:09d}'
        archive = directory / (hashlib.sha256(str(original).encode()).hexdigest()[:16] + '.tar.gz')
        temporary = archive.with_suffix('.gz.tmp')
        member_name = str(original.relative_to('/workspace'))
        with temporary.open('xb') as stream:
            with tarfile.open(fileobj=stream, mode='w:gz', compresslevel=1, format=tarfile.PAX_FORMAT) as packed:
                member = packed.gettarinfo(str(original), arcname=member_name)
                member.pax_headers['mtime'] = exact_mtime
                with original.open('rb') as source:
                    packed.addfile(member, source)
            stream.flush(); os.fsync(stream.fileno())
        assert temporary.stat().st_size <= 10 * 1024 * 1024
        assert shutil.disk_usage(ROOT).free >= 250 * 1024 * 1024
        with tarfile.open(temporary, 'r:gz') as packed:
            members = packed.getmembers(); assert len(members) == 1
            member = members[0]
            assert member.name == member_name and member.isfile()
            assert (member.size, member.mode, member.uid, member.gid) == (metadata.st_size, mode, metadata.st_uid, metadata.st_gid)
            assert Decimal(member.pax_headers['mtime']) * 1000000000 == metadata.st_mtime_ns
            checksum, size = hashlib.sha256(), 0
            stream = packed.extractfile(member)
            while block := stream.read(1048576):
                checksum.update(block); size += len(block)
            assert (checksum.hexdigest(), size) == (row[key], metadata.st_size)
        os.replace(temporary, archive); sync_directory(directory)
        entry = {'original': str(original), 'original_sha256': row[key], 'original_bytes': metadata.st_size,
                 'mode': mode, 'uid': metadata.st_uid, 'gid': metadata.st_gid, 'mtime_ns': metadata.st_mtime_ns,
                 'archive': str(archive.relative_to(ROOT)), 'archive_sha256': digest(archive),
                 'archive_bytes': archive.stat().st_size, 'member': member_name, 'original_removed': False}
        index['entries'].append(entry); save(index_path, index)
        assert not current_users({original})
        after = original.stat()
        assert (after.st_uid, after.st_gid, after.st_mtime_ns, stat.S_IMODE(after.st_mode)) == (metadata.st_uid, metadata.st_gid, metadata.st_mtime_ns, mode)
        assert digest(original) == row[key]
        original.unlink(); sync_directory(original.parent)
        entry['original_removed'] = True; save(index_path, index)
    for path, sha in protected.items():
        assert digest(Path(path)) == sha
    assert digest(receipt) == index['receipt_sha256']
    index.update(status='completed', original_bytes=sum(item['original_bytes'] for item in index['entries']),
                 compressed_bytes=sum(item['archive_bytes'] for item in index['entries']),
                 protected_rechecks=len(protected), free_after=shutil.disk_usage(ROOT).free)
    index['net_original_minus_archive_bytes'] = index['original_bytes'] - index['compressed_bytes']
    save(index_path, index)
    print(json.dumps({key: index[key] for key in ('status', 'original_bytes', 'compressed_bytes', 'net_original_minus_archive_bytes', 'protected_rechecks', 'free_after')}, indent=2))


if __name__ == '__main__':
    main()
