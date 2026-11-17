#!/usr/bin/env python3
"""Consolidate only an explicitly approved identical closed-success source/CIF inventory."""
from __future__ import annotations
import argparse
import hashlib
import json
import os
from pathlib import Path
import stat


def digest(path: Path) -> str:
    """Read the complete original bytes without rewriting any evidence path."""
    return hashlib.sha256(path.read_bytes()).hexdigest()


def fsync_directory(path: Path) -> None:
    """Persist an atomic directory-entry change before another group can start."""
    descriptor = os.open(path, os.O_RDONLY | os.O_DIRECTORY)
    try:
        os.fsync(descriptor)
    finally:
        os.close(descriptor)


def original_path(row: dict, group: dict) -> None:
    """Require exact reviewed identity, bytes, metadata and unshared original inode."""
    path = Path(row['path'])
    if path.is_symlink() or not path.is_file():
        raise RuntimeError('nonregular approved original: '+str(path))
    value = path.stat()
    expected = (row['dev'], row['inode'], row['nlink'], group['bytes'], group['mode'],
                group['uid'], group['gid'], row['mtime_ns'], row['ctime_ns'])
    actual = (value.st_dev, value.st_ino, value.st_nlink, value.st_size, stat.S_IMODE(value.st_mode),
              value.st_uid, value.st_gid, value.st_mtime_ns, value.st_ctime_ns)
    if actual != expected or value.st_nlink != 1 or digest(path) != group['sha256']:
        raise RuntimeError('changed reviewed original: '+str(path))


def active_inputs(paths: set[str]) -> None:
    """Refuse selected paths currently opened or mapped by any visible other process."""
    own = str(os.getpid())
    for process in Path('/proc').iterdir():
        if not process.name.isdecimal() or process.name == own:
            continue
        try:
            for descriptor in (process / 'fd').iterdir():
                try:
                    target = os.readlink(descriptor).removesuffix(' (deleted)')
                    if target in paths:
                        raise RuntimeError('approved input is active: '+target)
                except FileNotFoundError:
                    continue
            mappings = (process / 'maps').read_text()
            if any(line.split(maxsplit=5)[-1] in paths for line in mappings.splitlines() if len(line.split(maxsplit=5)) == 6):
                raise RuntimeError('approved input is mapped: '+process.name)
        except (PermissionError, FileNotFoundError, ProcessLookupError):
            continue


def verify_anchors(plan: dict, include_archives: bool) -> None:
    """Bind every approved group to unchanged successful reports and prior archive receipts."""
    checked = set()
    for anchor in plan['anchors']:
        report = Path(anchor['report'])
        if digest(report) != anchor['report_sha256']:
            raise RuntimeError('changed successful original report: '+str(report))
        for link in anchor['archive_success_links']:
            for name, key in [('ledger', 'ledger_sha256'), ('source_report', 'source_report_sha256')]:
                if link.get(name) and link[name] not in checked:
                    if digest(Path(link[name])) != link[key]:
                        raise RuntimeError('changed successful archive binding: '+link[name])
                    checked.add(link[name])
            if include_archives and link.get('archive') and link['archive'] not in checked:
                if digest(Path(link['archive'])) != link['archive_sha256']:
                    raise RuntimeError('changed prior verified archive: '+link['archive'])
                checked.add(link['archive'])


def main() -> None:
    """Persist the original plan first, then reverify and atomically replace only exact approved aliases."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--plan', type=Path, required=True)
    parser.add_argument('--plan-sha256', required=True)
    parser.add_argument('--ledger', type=Path, required=True)
    parser.add_argument('--cpu', type=int, default=1)
    args = parser.parse_args()
    if args.ledger.exists() or digest(args.plan) != args.plan_sha256 or args.plan.stat().st_size > 5*1024*1024:
        raise RuntimeError('fresh ledger/exact approved plan required')
    os.sched_setaffinity(0, {args.cpu})
    plan = json.loads(args.plan.read_text())
    with args.plan.open('rb') as handle:
        os.fsync(handle.fileno())
    fsync_directory(args.plan.parent)
    paths = {row['path'] for group in plan['groups'] for row in [group['keeper']]+group['duplicates']}
    ledger = {'scope': __doc__, 'runner_sha256': digest(Path(__file__)),
              'original_durable_plan': str(args.plan.resolve()), 'original_plan_sha256': args.plan_sha256,
              'original_per_path_metadata': 'The durable approved plan retains group SHA/size/mode/uid/gid plus each original dev/inode/nlink/allocated bytes/mtime/ctime.',
              'timestamp_policy': 'All aliases share the keeper timestamp after consolidation; original per-path timestamps remain recorded. No restamping is performed.',
              'cpu_affinity': [args.cpu], 'initial_free_bytes': os.statvfs('/workspace').f_bavail*os.statvfs('/workspace').f_frsize,
              'completed': [], 'gross_recovered_allocated_bytes': 0}

    def save() -> None:
        """Fsync an atomic recovery ledger before any further original directory entry changes."""
        encoded = json.dumps(ledger, indent=2)+'\n'
        if len(encoded.encode()) > 5*1024*1024:
            raise RuntimeError('5MiB recovery ledger cap')
        temporary = args.ledger.with_name(args.ledger.name+'.writing')
        with temporary.open('w') as handle:
            handle.write(encoded)
            handle.flush()
            os.fsync(handle.fileno())
        os.replace(temporary, args.ledger)
        fsync_directory(args.ledger.parent)

    try:
        verify_anchors(plan, True)
        active_inputs(paths)
        # Complete preflight occurs before the first original is replaced.
        for group in plan['groups']:
            for row in [group['keeper']]+group['duplicates']:
                original_path(row, group)
        ledger['status'] = 'preflight-passed'
        save()
        for index, group in enumerate(plan['groups']):
            keeper = Path(group['keeper']['path'])
            if digest(keeper) != group['sha256']:
                raise RuntimeError('keeper changed before group')
            for row in group['duplicates']:
                original_path(row, group)
                target = Path(row['path'])
                temporary = target.with_name(target.name+'.approved-link-'+str(os.getpid()))
                if temporary.exists():
                    raise RuntimeError('unexpected replacement scratch path')
                ledger['pending'] = {'group': index, 'target': str(target), 'keeper': str(keeper), 'temporary': str(temporary)}
                save()
                os.link(keeper, temporary)
                os.replace(temporary, target)
                fsync_directory(target.parent)
                if target.stat().st_ino != keeper.stat().st_ino or digest(target) != group['sha256']:
                    raise RuntimeError('atomic alias verification failed')
                ledger['gross_recovered_allocated_bytes'] += row['allocated_bytes']
                ledger['completed'].append({'group': index, 'path': str(target), 'keeper': str(keeper), 'shared_inode': keeper.stat().st_ino,
                                             'shared_mtime_ns': keeper.stat().st_mtime_ns})
                del ledger['pending']
                save()
        for group in plan['groups']:
            keeper = Path(group['keeper']['path'])
            for row in [group['keeper']]+group['duplicates']:
                path = Path(row['path']); value = path.stat()
                if path.is_symlink() or digest(path) != group['sha256'] or (value.st_dev, value.st_ino) != (keeper.stat().st_dev, keeper.stat().st_ino):
                    raise RuntimeError('final alias identity/bytes mismatch')
                if (value.st_size, stat.S_IMODE(value.st_mode), value.st_uid, value.st_gid) != (group['bytes'], group['mode'], group['uid'], group['gid']):
                    raise RuntimeError('final alias size/mode/ownership mismatch')
        verify_anchors(plan, False)
        ledger['status'] = 'passed'
    except BaseException as error:
        ledger.update(status='stopped-first-mismatch', failure=repr(error))
        raise
    finally:
        ledger['final_free_bytes'] = os.statvfs('/workspace').f_bavail*os.statvfs('/workspace').f_frsize
        save()
    print(len(ledger['completed']), 'approved aliases verified;', ledger['gross_recovered_allocated_bytes'], 'gross allocated bytes recovered')


if __name__ == '__main__':
    main()
