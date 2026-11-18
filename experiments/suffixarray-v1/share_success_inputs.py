#!/usr/bin/env python3
"""Execute only the reviewed closed-success source/interface sharing plan with a durable full-metadata recovery ledger."""
from __future__ import annotations
import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil
import stat
import time

ROOT = Path(__file__).resolve().parents[2]
APPROVED_PLAN = '1453d96c9727b2ebcbd67d2752786d632f2a7bcbe28e40298d9fadd9f50b9c65'
FIELDS = ['size', 'mode', 'uid', 'gid', 'mtime_ns', 'device', 'inode', 'nlink', 'blocks_512']
VISIBLE = ['size', 'mode', 'uid', 'gid', 'mtime_ns']


def digest(path: Path) -> str:
    """Identify complete bytes without retaining a large interface in memory."""
    result = hashlib.sha256()
    with path.open('rb') as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b''):
            result.update(block)
    return result.hexdigest()


def metadata(path: Path) -> dict:
    """Capture original ownership, access mode, timestamps and identity without following symbolic links."""
    value = path.lstat()
    assert stat.S_ISREG(value.st_mode)
    return dict(size=value.st_size, mode=stat.S_IMODE(value.st_mode), uid=value.st_uid,
                gid=value.st_gid, mtime_ns=value.st_mtime_ns, device=value.st_dev,
                inode=value.st_ino, nlink=value.st_nlink, blocks_512=value.st_blocks)


def sync_directory(path: Path) -> None:
    """Make each ledger creation and path replacement durable in its parent directory."""
    fd = os.open(path, os.O_DIRECTORY)
    try:
        os.fsync(fd)
    finally:
        os.close(fd)


def qualified_path(relative: str, directories: list[str]) -> Path:
    """Keep every mutation inside the exact passed-directory allowlist and retain regular plain paths."""
    path = ROOT / relative
    assert path.is_relative_to(ROOT / 'build') and path.resolve() == path
    assert any(path.is_relative_to(ROOT / directory) for directory in directories)
    assert path.suffix in ['.cinch', '.cinchif'] and not path.is_symlink()
    return path


def verify_original(entry: dict) -> None:
    """Refuse any unexpected content or metadata change before replacing an original inode."""
    path = ROOT / entry['path']
    assert metadata(path) == {key: entry[key] for key in FIELDS}, entry['path']
    assert digest(path) == entry['sha256'], entry['path']


def active_writer_check(directories: list[str]) -> dict:
    """Reject observed writers, writable file mappings or a process using a sealed working directory."""
    prefixes = [str(ROOT / directory) + '/' for directory in directories]
    closed = {str(ROOT / directory) for directory in directories}
    writers, inaccessible = [], []
    inspected = 0
    for proc in Path('/proc').iterdir():
        if not proc.name.isdecimal() or int(proc.name) == os.getpid():
            continue
        inspected += 1
        try:
            cwd = os.readlink(proc / 'cwd')
            if cwd in closed or any(cwd.startswith(prefix) for prefix in prefixes):
                writers.append(dict(pid=int(proc.name), reason='closed working directory', path=cwd))
            for fd in (proc / 'fd').iterdir():
                try:
                    target = os.readlink(fd)
                    if not any(target.startswith(prefix) for prefix in prefixes):
                        continue
                    info = (proc / 'fdinfo' / fd.name).read_text()
                    flags = int(next(line.split()[1] for line in info.splitlines() if line.startswith('flags:')), 8)
                    if flags & os.O_ACCMODE in [os.O_WRONLY, os.O_RDWR]:
                        writers.append(dict(pid=int(proc.name), reason='writable descriptor', path=target))
                except FileNotFoundError:
                    pass
            for line in (proc / 'maps').read_text().splitlines():
                parts = line.split(maxsplit=5)
                if len(parts) == 6 and 'w' in parts[1] and any(parts[5].startswith(prefix) for prefix in prefixes):
                    writers.append(dict(pid=int(proc.name), reason='writable mapped file', path=parts[5]))
        except FileNotFoundError:
            pass
        except PermissionError:
            inaccessible.append(int(proc.name))
    assert not writers, writers
    return dict(inspected_processes=inspected, observed_writers=writers,
                inaccessible_processes=inaccessible,
                scope='Linux descriptor/mapping/cwd observation plus integration-owner closed-epoch guarantee')


def write_new_durable(path: Path, value: dict) -> None:
    """Persist complete recovery information before any original path can be replaced."""
    temporary = path.with_suffix('.initial-partial')
    assert not path.exists() and not temporary.exists()
    raw = (json.dumps(value, indent=2) + '\n').encode()
    assert len(raw) < 3 * 1024 * 1024
    with temporary.open('xb') as stream:
        stream.write(raw)
        stream.flush()
        os.fsync(stream.fileno())
    os.replace(temporary, path)
    sync_directory(path.parent)


def journal(stream, record: dict) -> None:
    """Fsync each intention before replacement and each completed replacement afterward."""
    stream.write(json.dumps(record, separators=(',', ':')) + '\n')
    stream.flush()
    os.fsync(stream.fileno())


def restore(index: Path, requested: list[str]) -> None:
    """Recover one independent inode per requested path without altering any other shared alias."""
    ledger = json.loads(index.read_text())
    assert ledger['approved_plan_sha256'] == APPROVED_PLAN
    plan = ledger['plan']
    entries = {entry['path']: entry for group in plan['groups'] for entry in [group['canonical'], *group['aliases']]}
    for relative in requested:
        assert shutil.disk_usage(ROOT).free >= 300 * 1024 * 1024
        original = qualified_path(relative, plan['allowed_directories'])
        entry = entries[relative]
        assert digest(original) == entry['sha256']
        assert shutil.disk_usage(ROOT).free - entry['size'] >= 250 * 1024 * 1024
        temporary = original.with_name(original.name + '.metadata-restore-partial')
        assert not temporary.exists()
        with temporary.open('xb') as target:
            with original.open('rb') as source:
                shutil.copyfileobj(source, target, 1024 * 1024)
            target.flush()
            os.fchown(target.fileno(), entry['uid'], entry['gid'])
            os.fchmod(target.fileno(), entry['mode'])
            os.fsync(target.fileno())
        os.utime(temporary, ns=(entry['mtime_ns'], entry['mtime_ns']))
        assert digest(temporary) == entry['sha256']
        assert {key: metadata(temporary)[key] for key in VISIBLE} == {key: entry[key] for key in VISIBLE}
        with temporary.open('rb') as source:
            os.fsync(source.fileno())
        with index.with_name('restore-journal.jsonl').open('a') as log:
            journal(log, dict(event='restore-intent', path=relative, sha256=entry['sha256']))
            sync_directory(index.parent)
            os.replace(temporary, original)
            sync_directory(original.parent)
            journal(log, dict(event='restored-independent-metadata', path=relative, sha256=entry['sha256']))


def main() -> None:
    """Apply only the exact approved 82-group plan and retain an immutable original-metadata index."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--plan', type=Path)
    parser.add_argument('--restore-index', type=Path)
    parser.add_argument('--restore-path', action='append', default=[])
    args = parser.parse_args()
    if args.restore_index:
        assert args.restore_path and args.plan is None
        restore(args.restore_index.resolve(), args.restore_path)
        return
    assert args.plan and not args.restore_path
    assert digest(args.plan) == APPROVED_PLAN
    plan = json.loads(args.plan.read_text())
    assert plan['summary']['duplicate_groups'] == 82 and plan['summary']['duplicate_alias_paths'] == 1710
    assert not plan['uid_gid_conflicts'] and len(plan['eligible_receipts']) == 24
    before = shutil.disk_usage(ROOT).free
    assert before >= 300 * 1024 * 1024
    for receipt in plan['eligible_receipts']:
        path = ROOT / receipt['report']
        assert digest(path) == receipt['sha256']
        proof = json.loads(path.read_text())
        assert proof['status'] == 'passed' and len(proof['profiles']) == 2 * proof['count']
        assert all(profile['zero_leaks'] for profile in proof['profiles'])
        assert {profile['poison'] for profile in proof['profiles']} == {False, True}
        assert proof['source_sha256'] == receipt['source_sha256'] and proof['compiler_sha256'] == receipt['compiler_sha256'] and proof['runtime_sha256'] == receipt['runtime_sha256']
    for group in plan['groups']:
        for entry in [group['canonical'], *group['aliases']]:
            qualified_path(entry['path'], plan['allowed_directories'])
            verify_original(entry)
        assert len({(entry['uid'], entry['gid']) for entry in [group['canonical'], *group['aliases']]}) == 1
    for entry in plan['protected_inside_allowed_directories'] + plan['unique_source_interface_files_unchanged']:
        verify_original(entry)
    writer_check = active_writer_check(plan['allowed_directories'])
    index = ROOT / plan['recovery_index']
    assert not index.parent.exists()
    index.parent.mkdir()
    sync_directory(index.parent.parent)
    ledger = dict(schema=1, scope='Approved closed successful suffix source/CIF sharing; complete original metadata',
                  approved_plan_sha256=APPROVED_PLAN, execution_tool_sha256=digest(Path(__file__)),
                  writer_check=writer_check, plan=plan,
                  original_metadata_semantics='Every original path bytes/size/mode/uid/gid/nsmtime recorded before any mutation; inode/ctime identities are not promised restorable',
                  shared_metadata_semantics='Canonical visible mtime/owner/mode intentionally shared; original path metadata remains recoverable through independent temporary inodes',
                  restore_command=['python3', 'experiments/suffixarray-v1/share_success_inputs.py', '--restore-index', str(index.relative_to(ROOT)), '--restore-path', '<recorded relative path>'],
                  progress_journal='journal.jsonl')
    write_new_durable(index, ledger)
    completed = 0
    journal_path = index.with_name('journal.jsonl')
    minimum = shutil.disk_usage(ROOT).free
    with journal_path.open('x') as log:
        journal(log, dict(event='complete-original-ledger-durable', index_sha256=digest(index), time_ns=time.time_ns()))
        sync_directory(index.parent)
        for group in plan['groups']:
            canonical = ROOT / group['canonical']['path']
            for entry in group['aliases']:
                minimum = min(minimum, shutil.disk_usage(ROOT).free)
                assert minimum >= 250 * 1024 * 1024
                assert sum(p.stat().st_size for p in index.parent.iterdir() if p.is_file()) < 5 * 1024 * 1024
                alias = qualified_path(entry['path'], plan['allowed_directories'])
                verify_original(entry)
                assert digest(canonical) == group['sha256']
                assert {key: metadata(canonical)[key] for key in VISIBLE} == group['shared_visible_metadata']
                temporary = alias.with_name(alias.name + '.source-cif-sharing-partial')
                assert not temporary.exists()
                journal(log, dict(event='intent', path=entry['path'], canonical=group['canonical']['path'], sha256=group['sha256']))
                os.link(canonical, temporary, follow_symlinks=False)
                assert digest(temporary) == group['sha256'] and metadata(temporary)['inode'] == metadata(canonical)['inode']
                with temporary.open('rb') as source:
                    os.fsync(source.fileno())
                sync_directory(temporary.parent)
                os.replace(temporary, alias)
                sync_directory(alias.parent)
                journal(log, dict(event='shared', path=entry['path'], canonical=group['canonical']['path'], visible_mtime_ns=group['shared_visible_metadata']['mtime_ns']))
                completed += 1
        for group in plan['groups']:
            canonical = ROOT / group['canonical']['path']
            expected_links = 1 + len(group['aliases'])
            assert metadata(canonical)['nlink'] == expected_links
            for entry in [group['canonical'], *group['aliases']]:
                path = ROOT / entry['path']
                actual = metadata(path)
                assert actual['inode'] == metadata(canonical)['inode'] and actual['nlink'] == expected_links
                assert {key: actual[key] for key in VISIBLE} == group['shared_visible_metadata']
                assert digest(path) == entry['sha256']
        for entry in plan['protected_inside_allowed_directories'] + plan['unique_source_interface_files_unchanged']:
            verify_original(entry)
        journal(log, dict(event='all-shared-and-protected-verified', aliases=completed, protected=len(plan['protected_inside_allowed_directories'])))
    sync_directory(index.parent)
    after = shutil.disk_usage(ROOT).free
    report = dict(scope='Exact approved sealed-success source/CIF metadata sharing only', status='passed',
                  approved_plan_sha256=APPROVED_PLAN, index=str(index.relative_to(ROOT)), index_sha256=digest(index),
                  journal=str(journal_path.relative_to(ROOT)), journal_sha256=digest(journal_path), writer_check=writer_check,
                  shared_groups=82, shared_aliases=completed, all_paths_plain=True,
                  protected_files_unchanged=812, unique_source_interface_files_unchanged=len(plan['unique_source_interface_files_unchanged']),
                  uid_gid_conflicts=0, logical_redundant_bytes=plan['summary']['redundant_logical_bytes'],
                  allocated_blocks_recovered_bytes=plan['summary']['estimated_redundant_allocated_bytes'],
                  before_free_bytes=before, minimum_observed_free_bytes=minimum, after_free_bytes=after,
                  observed_global_free_change_bytes=after-before,
                  own_metadata_bytes=sum(p.stat().st_size for p in index.parent.iterdir() if p.is_file()))
    write_new_durable(index.with_name('completion.json'), report)
    print(json.dumps(report))


if __name__ == '__main__':
    main()
