#!/usr/bin/env python3
"""Consolidate only the exact reviewed closed-success input duplicates as plain hardlinks, without changing payload bytes."""
from pathlib import Path
import hashlib
import json
import os
import stat
import shutil
import check


def sync_directory(path):
    """Make each plan publication and atomic path replacement durable in its actual parent directory."""
    descriptor = os.open(path, os.O_RDONLY | os.O_DIRECTORY)
    try:
        os.fsync(descriptor)
    finally:
        os.close(descriptor)


def durable_json(path, value):
    """Publish a complete fsynced recovery mapping before any old independent inode can be replaced."""
    temporary = path.with_suffix('.json.tmp')
    with temporary.open('w') as stream:
        json.dump(value, stream, indent=2)
        stream.write('\n')
        stream.flush()
        os.fsync(stream.fileno())
    os.replace(temporary, path)
    sync_directory(path.parent)


def identity(path):
    """Record exact payload and original filesystem metadata without restamping a source or interface."""
    information = path.stat()
    assert not path.is_symlink() and stat.S_ISREG(information.st_mode)
    return {'path': str(path), 'sha256': check.digest(path), 'bytes': information.st_size,
            'mode': stat.S_IMODE(information.st_mode), 'uid': information.st_uid, 'gid': information.st_gid,
            'mtime_ns': information.st_mtime_ns, 'device': information.st_dev, 'inode': information.st_ino,
            'nlink': information.st_nlink}


def main():
    """Require the exact reviewed3691/94,825,901-byte candidate set and reverify every resulting plain path."""
    os.sched_setaffinity(0, {2})
    root = check.ROOT / 'build/tabwriter-format-full-330-v1'
    work = check.ROOT / 'build/tabwriter-closed-input-consolidation-v1'
    assert not work.exists(), 'never overwrite an input-recovery epoch'
    assert shutil.disk_usage(root).free >= 350 * check.MIB and shutil.disk_usage(root).free >= 250 * check.MIB + 5 * check.MIB
    paths, receipts = [], []
    units = sorted(root.glob('tabwriter-format-*-runtime'))
    assert len(units) == 23
    for runtime in units:
        source = Path(str(runtime).replace('-runtime', '-source'))
        actual = json.loads((runtime / 'report.json').read_text())
        admitted = json.loads((source / 'report.json').read_text())
        assert actual['all_passed'] and admitted['source_gate_passed']
        assert actual['source_report_sha256'] == check.digest(source / 'report.json')
        assert all(row['returncode'] == 0 and 'live=0 immortal=0 leaked=0' in row['stderr'] for row in actual['runs'])
        assert (runtime / 'closed-success-index.json').exists()
        receipts.append({str(folder): check.digest(folder / 'report.json') for folder in (source, runtime)})
        for folder in (source, runtime / 'project'):
            paths.extend(identity(path) for path in sorted(folder.rglob('*'))
                         if path.is_file() and not path.is_symlink() and path.suffix in ('.cinch', '.cinchif', '.cinchpkg'))
    assert len(paths) == 3818 and sum(row['bytes'] for row in paths) == 97655370
    assert all(row['uid'] == row['gid'] == 1000 and row['nlink'] == 1 for row in paths)
    groups = {}
    for row in paths:
        groups.setdefault((row['sha256'], row['bytes'], row['mode']), []).append(row)
    replacements, recovery = [], 0
    for members in groups.values():
        canonical = members[0]
        for row in members[1:]:
            replacements.append({'canonical': canonical['path'], 'original': row})
            recovery += row['bytes']
    assert len(replacements) == 3691 and recovery == 94825901
    work.mkdir()
    plan = {'scope': __doc__, 'authorization': 'Root exact3691 duplicate inode consolidations from23 CLOSED successful input epochs',
            'paths': paths, 'replacements': replacements, 'passed_receipts': receipts,
            'all_paths_remain_plain': True, 'no_future_compiler_writes_to_these_epochs': True,
            'shared_timestamp_policy': 'Aliases share their canonical inode timestamp. Original per-path mtimes are recorded; no metadata is restamped.',
            'failures_frozen_providers_runtimes_benchmarks_untouched': True,
            'free_at_start_bytes': shutil.disk_usage(root).free, 'expected_recovered_payload_bytes': recovery}
    plan_path = work / 'plan.json'
    durable_json(plan_path, plan)
    synced = set()
    for index, row in enumerate(replacements):
        assert shutil.disk_usage(root).free >= 250 * check.MIB, 'reserve; remaining original paths retained'
        original, canonical = row['original'], Path(row['canonical'])
        target = Path(original['path'])
        actual = identity(target)
        assert actual == original
        keeper = identity(canonical)
        assert (keeper['sha256'], keeper['bytes'], keeper['mode'], keeper['uid'], keeper['gid']) == (
            original['sha256'], original['bytes'], original['mode'], original['uid'], original['gid'])
        if canonical not in synced:
            with canonical.open('rb') as stream:
                os.fsync(stream.fileno())
            sync_directory(canonical.parent)
            synced.add(canonical)
        temporary = target.with_name(target.name + '.sealed-link-tmp')
        assert not temporary.exists()
        os.link(canonical, temporary)
        os.replace(temporary, target)
        sync_directory(target.parent)
    for row in paths:
        actual = identity(Path(row['path']))
        assert all(actual[key] == row[key] for key in ('sha256', 'bytes', 'mode', 'uid', 'gid'))
    for row in replacements:
        canonical, target = Path(row['canonical']).stat(), Path(row['original']['path']).stat()
        assert (canonical.st_dev, canonical.st_ino) == (target.st_dev, target.st_ino)
    for receipt in receipts:
        for folder, expected in receipt.items():
            assert check.digest(Path(folder) / 'report.json') == expected
    result = {'plan_sha256': check.digest(plan_path), 'replacements': len(replacements),
              'recovered_payload_bytes': recovery, 'all_plain_paths_reverified': len(paths),
              'free_after_bytes': shutil.disk_usage(root).free, 'no_compiler_runtime_oracle_run': True}
    durable_json(work / 'report.json', result)
    print(json.dumps(result), flush=True)


if __name__ == '__main__':
    main()
