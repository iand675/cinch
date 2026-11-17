#!/usr/bin/env python3
"""Share a fully passed four-unit suffix batch with older qualified plain source/CIF canonicals through a new immutable ledger."""
from __future__ import annotations
import argparse
import json
import os
from pathlib import Path
import shutil
import time
from share_success_inputs import ROOT, FIELDS, VISIBLE, active_writer_check, digest, journal, metadata, qualified_path, sync_directory, verify_original, write_new_durable


def entry(path: Path, report: Path) -> dict:
    """Bind one regular file's complete original identity to its passed consumer report."""
    return dict(path=str(path.relative_to(ROOT)), **metadata(path), sha256=digest(path),
                report=str(report.relative_to(ROOT)), report_sha256=digest(report))


def key(item: dict) -> tuple:
    """Permit sharing only when complete bytes, size, access mode, ownership and device all match."""
    return tuple(item[field] for field in ['sha256', 'size', 'mode', 'uid', 'gid', 'device'])


def passed(report: Path) -> dict:
    """Exclude failed, active, provider-producing or incomplete memory-profile epochs."""
    proof = json.loads(report.read_text())
    assert proof['status'] == 'passed' and len(proof['profiles']) == 2 * proof['count']
    assert all(item['zero_leaks'] for item in proof['profiles'])
    assert {item['poison'] for item in proof['profiles']} == {False, True}
    assert proof['command'][1] == 'build' and not any(flag in proof['command'] for flag in ['--emit-library', '--interfaces', '--objects', '--provider'])
    return proof


def main() -> None:
    """Keep old ledgers immutable and write every new original metadata record before any new alias replacement."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--batch-report', type=Path, required=True)
    parser.add_argument('--artifacts', type=Path, required=True)
    args = parser.parse_args()
    before = shutil.disk_usage(ROOT).free
    assert before >= 300 * 1024 * 1024
    batch_path = args.batch_report.resolve()
    batch = json.loads(batch_path.read_text())
    assert batch['status'] == 'passed-and-archived' and len(batch['units']) == 4
    work = args.artifacts.resolve()
    assert work.parent == ROOT / 'build' and work.name.startswith('suffixarray-v1-source-cif-extension-') and not work.exists()
    new_reports = [ROOT / item['path'] for item in batch['units']]
    new_directories = [str(path.with_suffix('').relative_to(ROOT)) for path in new_reports]
    archive_index = json.loads((ROOT / 'build/suffixarray-v1-functional-recovery/index.json').read_text())
    archives = {item['path']: item for item in archive_index['files']}
    for item, report in zip(batch['units'], new_reports):
        assert digest(report) == item['sha256']
        proof = passed(report)
        prior = ROOT / item['o0_report']
        assert digest(prior) == item['o0_report_sha256']
        assert proof['entry_sha256'] == passed(prior)['entry_sha256']
        for name, field in [('consumer', 'executable_sha256'), ('consumer.ll', 'llvm_sha256')]:
            path = report.with_suffix('') / name
            archived = archives[str(path.relative_to(ROOT))]
            assert not path.exists() and archived['original_unlinked'] and archived['sha256'] == proof[field]
            assert archived['report_sha256'] == digest(report) and digest(ROOT / archived['archive']) == archived['compressed_sha256']
    canonicals = {}
    canonical_receipts = {}
    for report in sorted((ROOT / 'build').glob('suffixarray-v1-runtime-literals-*.json')):
        if report in new_reports:
            continue
        proof = json.loads(report.read_text())
        if proof['status'] != 'passed':
            continue
        passed(report)
        canonical_receipts[str(report.relative_to(ROOT))] = digest(report)
        for path in sorted(report.with_suffix('').rglob('*')):
            if path.is_file() and path.suffix in ['.cinch', '.cinchif']:
                original = entry(path, report)
                canonicals.setdefault(key(original), original)
    groups, protected, unshared = {}, [], []
    for report in new_reports:
        for path in sorted(report.with_suffix('').rglob('*')):
            if not path.is_file():
                continue
            original = entry(path, report)
            if path.suffix not in ['.cinch', '.cinchif']:
                protected.append(original)
            elif key(original) not in canonicals:
                unshared.append(original)
            else:
                assert original['nlink'] == 1
                canonical = canonicals[key(original)]
                group = groups.setdefault(key(original), dict(canonical=canonical, aliases=[],
                    visible={field: canonical[field] for field in VISIBLE}))
                group['aliases'].append(original)
    assert groups
    for group in groups.values():
        verify_original(group['canonical'])
        for original in group['aliases']:
            qualified_path(original['path'], new_directories)
            verify_original(original)
    writer_check = active_writer_check(new_directories + [str((ROOT / item['canonical']['path']).parent.relative_to(ROOT)) for item in groups.values()])
    ledger = dict(schema=1, scope='Standing approved extension for four closed fully passed consumer units only',
        batch_report=str(batch_path.relative_to(ROOT)), batch_report_sha256=digest(batch_path),
        execution_tool_sha256=digest(Path(__file__)), old_ledgers_immutable=True,
        directories=new_directories, qualification_receipts={str(path.relative_to(ROOT)): digest(path) for path in new_reports},
        canonical_qualification_receipts=canonical_receipts, groups=list(groups.values()), protected=protected, unshared=unshared,
        original_metadata='Every path fullSHA/size/mode/uid/gid/nsmtime recorded before replacement',
        shared_metadata='Explicit canonical visible metadata; restore independent original metadata through a new temporary inode, never modify a shared inode',
        excluded='All paths outside the four approved closed directories are excluded from alias replacement; older closed canonical links may gain link count only. All failures/unknown/provider/compiler/runtime/benchmark inputs remain protected.',
        writer_check=writer_check)
    work.mkdir()
    sync_directory(work.parent)
    index = work / 'index.json'
    write_new_durable(index, ledger)
    completed = 0
    minimum = shutil.disk_usage(ROOT).free
    with (work / 'journal.jsonl').open('x') as log:
        journal(log, dict(event='complete-original-ledger-durable', index_sha256=digest(index), time_ns=time.time_ns()))
        sync_directory(work)
        for group in groups.values():
            canonical = ROOT / group['canonical']['path']
            for original in group['aliases']:
                minimum = min(minimum, shutil.disk_usage(ROOT).free)
                assert minimum >= 250 * 1024 * 1024
                assert sum(path.stat().st_size for path in work.iterdir() if path.is_file()) < 5 * 1024 * 1024
                alias = qualified_path(original['path'], new_directories)
                verify_original(original)
                assert digest(canonical) == original['sha256']
                assert {field: metadata(canonical)[field] for field in VISIBLE} == group['visible']
                temporary = alias.with_name(alias.name + '.extension-sharing-partial')
                assert not temporary.exists()
                journal(log, dict(event='intent', path=original['path'], canonical=group['canonical']['path']))
                os.link(canonical, temporary, follow_symlinks=False)
                assert digest(temporary) == original['sha256'] and metadata(temporary)['inode'] == metadata(canonical)['inode']
                with temporary.open('rb') as source:
                    os.fsync(source.fileno())
                sync_directory(alias.parent)
                os.replace(temporary, alias)
                sync_directory(alias.parent)
                journal(log, dict(event='shared', path=original['path'], visible_mtime_ns=group['visible']['mtime_ns']))
                completed += 1
        for group in groups.values():
            canonical = ROOT / group['canonical']['path']
            assert metadata(canonical)['nlink'] == group['canonical']['nlink'] + len(group['aliases'])
            for original in group['aliases']:
                path = ROOT / original['path']
                assert digest(path) == original['sha256'] and metadata(path)['inode'] == metadata(canonical)['inode']
                assert {field: metadata(path)[field] for field in VISIBLE} == group['visible']
        for original in protected + unshared:
            verify_original(original)
        journal(log, dict(event='complete-and-verified', aliases=completed, protected=len(protected), unshared=len(unshared)))
    sync_directory(work)
    report = dict(scope='Approved four-unit closed-success sharing extension only', status='passed',
        index=str(index.relative_to(ROOT)), index_sha256=digest(index), journal_sha256=digest(work / 'journal.jsonl'),
        batch_report=str(batch_path.relative_to(ROOT)), groups=len(groups), aliases=completed,
        protected_files_unchanged=len(protected), unshared_files_unchanged=len(unshared), all_paths_plain=True,
        logical_redundant_bytes=sum(original['size'] for group in groups.values() for original in group['aliases']),
        allocated_blocks_recovered_bytes=sum(original['blocks_512'] * 512 for group in groups.values() for original in group['aliases']),
        before_free_bytes=before, minimum_observed_free_bytes=minimum, after_free_bytes=shutil.disk_usage(ROOT).free,
        own_metadata_bytes=sum(path.stat().st_size for path in work.iterdir() if path.is_file()))
    write_new_durable(work / 'completion.json', report)
    print(json.dumps(report))


if __name__ == '__main__':
    main()
