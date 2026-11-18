#!/usr/bin/env python3
"""Recover only a completed zero-leak control's ELF/IR after exact durable archive verification."""
from pathlib import Path
import argparse
import gzip
import hashlib
import json
import os
import resource
import stat
import tarfile
import check


def sync_dir(path):
    """Persist archive/index publication and source-parent removals in their actual directories."""
    descriptor = os.open(path, os.O_RDONLY | os.O_DIRECTORY)
    try:
        os.fsync(descriptor)
    finally:
        os.close(descriptor)


def file_digest(path):
    """Hash an ELF, LLVM source or compressed archive with bounded host memory."""
    result = hashlib.sha256()
    with path.open('rb') as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b''):
            result.update(chunk)
    return result.hexdigest()


def main():
    """Keep every failure, input, log and genuine interface plain; archive only two explicitly bound success artifacts."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--work', type=Path, required=True)
    args = parser.parse_args()
    os.sched_setaffinity(0, {2})
    work = args.work.resolve()
    assert work.is_relative_to(check.ROOT / 'build') and work.name.startswith('buffered-region-330-')
    report_path = work / 'report.json'
    report = json.loads(report_path.read_text())
    assert report['compiler_sha256'] == check.COMPILER and report['runtime_sha256'] == check.RUNTIME
    report_sha = check.digest(report_path)
    assert report['all_passed'] and not report['retained_cycle_control']
    assert len(report['runs']) == 2 and all(row['returncode'] == 0 and 'live=0 immortal=0 leaked=0' in row['stderr'] for row in report['runs'])
    archive = work / 'closed-success.tar.gz'
    index_path = work / 'closed-success-index.json'
    temporary = work / 'closed-success.tar.gz.tmp'
    assert not archive.exists() and not index_path.exists() and not temporary.exists()
    resource.setrlimit(resource.RLIMIT_FSIZE, (10 * 1024 * 1024, resource.getrlimit(resource.RLIMIT_FSIZE)[1]))
    entries = []
    for name, key in [('consumer', 'executable_sha256'), ('consumer.ll', 'llvm_sha256')]:
        path = work / name
        information = path.stat()
        assert stat.S_ISREG(information.st_mode) and information.st_nlink == 1
        assert file_digest(path) == report[key]
        entries.append({'path': str(path), 'member': name, 'report_hash_field': key,
                        'sha256': report[key], 'bytes': information.st_size,
                        'mode': stat.S_IMODE(information.st_mode), 'uid': information.st_uid,
                        'gid': information.st_gid, 'mtime_ns': information.st_mtime_ns,
                        'device': information.st_dev, 'inode': information.st_ino})
    with tarfile.open(temporary, 'w:gz', compresslevel=6) as output:
        for entry in entries:
            path = Path(entry['path'])
            info = output.gettarinfo(str(path), arcname=entry['member'])
            info.pax_headers['CINCH.mtime_ns'] = str(entry['mtime_ns'])
            with path.open('rb') as stream:
                output.addfile(info, stream)
    with temporary.open('rb') as stream:
        os.fsync(stream.fileno())
    os.replace(temporary, archive)
    sync_dir(work)
    decoded_hash = hashlib.sha256()
    decoded_bytes = 0
    with gzip.open(archive, 'rb') as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b''):
            decoded_hash.update(chunk)
            decoded_bytes += len(chunk)
    expected = {row['member']: row for row in entries}
    seen = set()
    with tarfile.open(archive, 'r:gz') as source:
        for member in source:
            assert member.name in expected and member.name not in seen and member.isfile()
            entry = expected[member.name]
            assert (member.size, member.mode, member.uid, member.gid) == (entry['bytes'], entry['mode'], entry['uid'], entry['gid'])
            assert member.pax_headers['CINCH.mtime_ns'] == str(entry['mtime_ns'])
            body = hashlib.sha256()
            with source.extractfile(member) as stream:
                for chunk in iter(lambda: stream.read(1024 * 1024), b''):
                    body.update(chunk)
            assert body.hexdigest() == entry['sha256']
            seen.add(member.name)
    assert seen == set(expected)
    index = {'scope': __doc__, 'report': str(report_path), 'report_sha256': report_sha,
             'archive': str(archive), 'compressed_sha256': file_digest(archive),
             'compressed_bytes': archive.stat().st_size, 'decompressed_tar_sha256': decoded_hash.hexdigest(),
             'decompressed_tar_bytes': decoded_bytes, 'entries': entries, 'originals_removed': False}

    def save():
        """Make the complete recovery mapping durable before unlinking either successful artifact."""
        pending = index_path.with_suffix('.json.tmp')
        with pending.open('w') as stream:
            json.dump(index, stream, indent=2)
            stream.write('\n')
            stream.flush()
            os.fsync(stream.fileno())
        os.replace(pending, index_path)
        sync_dir(work)

    save()
    assert check.digest(report_path) == report_sha
    for entry in entries:
        path = Path(entry['path'])
        information = path.stat()
        assert (information.st_dev, information.st_ino, information.st_size, information.st_mtime_ns,
                stat.S_IMODE(information.st_mode), information.st_uid, information.st_gid, information.st_nlink) == (
                    entry['device'], entry['inode'], entry['bytes'], entry['mtime_ns'], entry['mode'], entry['uid'], entry['gid'], 1)
        assert file_digest(path) == entry['sha256']
    for entry in entries:
        Path(entry['path']).unlink()
    sync_dir(work)
    index['originals_removed'] = True
    index['net_logical_recovered_bytes'] = sum(row['bytes'] for row in entries) - index['compressed_bytes']
    save()
    print(json.dumps({key: index[key] for key in ['compressed_bytes', 'net_logical_recovered_bytes', 'originals_removed']}), flush=True)


if __name__ == '__main__':
    main()
