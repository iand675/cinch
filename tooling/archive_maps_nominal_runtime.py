#!/usr/bin/env python3
"""Verify and durably archive one successful Role+Slots map unit; preserve sources and historical interfaces."""
from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import stat
import tarfile

from archive_map_functional_elfs import ROOT, digest, save_index, sync_directory
from qualify_maps_nominal_source import COMPILER_SHA, RUNTIME_SHA


def main() -> None:
    """Archive only closed successful functional outputs and generated disposable interface caches."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--report', type=Path, required=True)
    parser.add_argument('--cpu', type=int, required=True)
    args = parser.parse_args()
    os.sched_setaffinity(0, {args.cpu})
    report = json.loads(args.report.read_text())
    assert report['status'] == 'passed' and report['compiler_sha256'] == COMPILER_SHA and report['runtime_sha256'] == RUNTIME_SHA
    assert len(report['runs']) == 2 and [r['poisoned_free'] for r in report['runs']] == [False, True]
    for run in report['runs']:
        assert run['exit'] == 0 and run['stdout'] == '' and run['stack'] == '8M'
        assert run['heap'] == ('256M' if run['poisoned_free'] else '32M')
        assert 'cinch-leak: live=0 immortal=0 leaked=0' in run['stderr']
    executable = Path(report['runs'][0]['command'][0]).resolve()
    assert executable.parent.parent == ROOT / 'build' and executable.parent.name.startswith('maps-role-slots-matrix-')
    assert executable.name == 'consumer' and digest(executable) == report['executable_sha256']
    llvm = executable.with_suffix('.ll')
    assert digest(llvm) == report['llvm_sha256']
    assert report['sources']['Data.Mutable.Map']['sha256'] == '7085aebe4c10dcd0cf27be6ffc0348a239398b07d54f60cb098722802b244c65'
    files = [executable, llvm] + sorted(p for p in (executable.parent / 'build').rglob('*') if p.is_file())
    assert all(not p.is_symlink() and p.is_relative_to(executable.parent) for p in files)
    members = {str(p.relative_to(ROOT)): {'sha256': digest(p), 'bytes': p.stat().st_size,
                'mode': stat.S_IMODE(p.stat().st_mode)} for p in files}
    directory = ROOT / 'build/maps-role-slots-closed-unit-archive'
    directory.mkdir(parents=True, exist_ok=True)
    key = hashlib.sha256(str(executable).encode()).hexdigest()[:16] + '-' + report['unit']
    archive, snapshot = directory / (key + '.tar.gz'), directory / (key + '.result.json')
    assert not archive.exists() and not snapshot.exists()
    with snapshot.open('xb') as output:
        output.write(args.report.read_bytes()); output.flush(); os.fsync(output.fileno())
    temporary = archive.with_suffix('.gz.tmp')
    with temporary.open('xb') as output:
        with tarfile.open(fileobj=output, mode='w:gz', compresslevel=1) as packed:
            for p in files:
                packed.add(p, arcname=str(p.relative_to(ROOT)), recursive=False)
        output.flush(); os.fsync(output.fileno())
    with tarfile.open(temporary, 'r:gz') as packed:
        assert len(packed.getmembers()) == len(members)
        for member in packed.getmembers():
            expected = members[member.name]
            assert member.isfile() and member.mode == expected['mode']
            checksum, size = hashlib.sha256(), 0
            stream = packed.extractfile(member)
            while block := stream.read(1048576):
                checksum.update(block); size += len(block)
            assert (checksum.hexdigest(), size) == (expected['sha256'], expected['bytes'])
    os.replace(temporary, archive); sync_directory(directory)
    index_path = directory / 'index.json'
    index = json.loads(index_path.read_text()) if index_path.exists() else {'kind': 'closed-Role-Slots-map-runtime-units', 'entries': []}
    entry = {'report': str(args.report.resolve()), 'result_snapshot': str(snapshot.relative_to(ROOT)),
             'result_snapshot_sha256': digest(snapshot), 'archive': str(archive.relative_to(ROOT)),
             'archive_sha256': digest(archive), 'archive_bytes': archive.stat().st_size,
             'members': members, 'originals_removed': False}
    index['entries'].append(entry); save_index(index_path, index)
    report['closed_archive'] = {'index': str(index_path.relative_to(ROOT)), **entry}; save_index(args.report.resolve(), report)
    for p in files:
        assert digest(p) == members[str(p.relative_to(ROOT))]['sha256']
        p.unlink()
    for parent in {p.parent for p in files}:
        sync_directory(parent)
    entry['originals_removed'] = True; save_index(index_path, index)
    report['closed_archive']['originals_removed'] = True; save_index(args.report.resolve(), report)
    print(f"Archived {report['unit']}: exact functional/cache bytes recoverable; sources/logs retained")


if __name__ == '__main__':
    main()
