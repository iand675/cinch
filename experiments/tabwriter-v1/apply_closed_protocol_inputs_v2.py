#!/usr/bin/env python3
"""Apply only the reviewed protocol006–016 source/CIF hardlink plan while preserving every plain input path."""
from pathlib import Path
import json
import os
import shutil
import check
from consolidate_success_inputs import identity, sync_directory, durable_json
from apply_protocol_input_plan import allocated

PLAN_SHA = '79d2334253f670b4127e1167c961539cf1326eb55e17ee58a5b2572f64dabbe4'


def verify_units(units):
    """Require unchanged source/runtime/archive receipts for every closed candidate or canonical epoch."""
    for unit in units:
        source, runtime = Path(unit['source']), Path(unit['runtime'])
        assert check.digest(source / 'report.json') == unit['source_report_sha256']
        assert check.digest(runtime / 'report.json') == unit['runtime_report_sha256']
        assert check.digest(runtime / 'closed-success-index.json') == unit['archive_index_sha256']


def main():
    """Reverify the exact 1,848 targets and 84 canonicals, publish a durable index, then replace only reviewed duplicate inodes."""
    os.sched_setaffinity(0, {2})
    work = check.ROOT / 'build/tabwriter-protocol-input-inventory-v2'
    plan_path, result_path = work / 'plan.json', work / 'recovery.json'
    assert check.digest(plan_path) == PLAN_SHA and not result_path.exists()
    assert shutil.disk_usage(check.ROOT).free >= 300 * check.MIB, 'reviewed metadata-only recovery start guard'
    plan = json.loads(plan_path.read_text())
    assert plan['target_protocol_indices'] == list(range(6, 17))
    assert len(plan['paths']) == len(plan['replacements']) == 1848 and len(plan['canonicals']) == 84
    assert plan['logical_payload_recovery_estimate'] == 46510420 and plan['allocated_recovery_estimate'] == 50552832
    assert plan['sealed_against_future_compiler_writes'] and plan['main_active_failure_frozen_provider_runtime_benchmark_excluded']
    all_units = plan['units'] + plan['prior_closed_units']
    verify_units(all_units)
    for row in plan['paths'] + plan['canonicals']:
        path = Path(row['path'])
        assert path.is_relative_to(check.ROOT / 'build') and path.suffix in ('.cinch', '.cinchif')
        assert path.name not in ('Main.cinch', 'Main.cinchif')
        assert identity(path) == row
    targets = {row['path'] for row in plan['paths']}
    assert not targets.intersection(row['path'] for row in plan['canonicals'])
    assert len({row['target'] for row in plan['replacements']}) == 1848
    scope_paths = [Path(row['path']) for row in plan['paths'] + plan['canonicals']]
    before_allocated, before_free = allocated(scope_paths), shutil.disk_usage(check.ROOT).free
    with plan_path.open('rb') as stream:
        os.fsync(stream.fileno())
    sync_directory(work)
    durable_json(work / 'execution-index.json', {'plan_sha256': PLAN_SHA,
                 'scope': 'Root reviewed exact protocol006–016 source/CIF targets and prior sealed canonicals',
                 'no_future_compiler_writes_to_linked_epochs': True, 'no_metadata_restamping': True,
                 'original_per_path_metadata_in_plan': True, 'allocated_scope_before_bytes': before_allocated,
                 'free_before_bytes': before_free, 'metadata_only_start_mib': 300, 'reserve_mib': 250,
                 'ledger_cap_bytes': 5 * check.MIB})
    synced = set()
    for replacement in plan['replacements']:
        assert shutil.disk_usage(check.ROOT).free >= 250 * check.MIB
        assert sum(path.stat().st_size for path in work.glob('*.json')) < 5 * check.MIB
        original, keeper = plan['paths'][replacement['target']], plan['canonicals'][replacement['canonical']]
        target, canonical = Path(original['path']), Path(keeper['path'])
        assert identity(target) == original
        actual = identity(canonical)
        assert all(actual[key] == keeper[key] for key in ('sha256', 'bytes', 'mode', 'uid', 'gid', 'mtime_ns', 'device', 'inode'))
        assert all(actual[key] == original[key] for key in ('sha256', 'bytes', 'mode', 'uid', 'gid'))
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
    for row in plan['paths'] + plan['canonicals']:
        actual = identity(Path(row['path']))
        assert all(actual[key] == row[key] for key in ('sha256', 'bytes', 'mode', 'uid', 'gid'))
    for replacement in plan['replacements']:
        target = Path(plan['paths'][replacement['target']]['path']).stat()
        canonical = Path(plan['canonicals'][replacement['canonical']]['path']).stat()
        assert (target.st_dev, target.st_ino) == (canonical.st_dev, canonical.st_ino)
    verify_units(all_units)
    after_allocated = allocated(scope_paths)
    result = {'plan_sha256': PLAN_SHA, 'replacements': 1848, 'verified_target_paths': 1848,
              'verified_canonical_paths': 84, 'logical_payload_recovered_bytes': 46510420,
              'allocated_scope_before_bytes': before_allocated, 'allocated_scope_after_bytes': after_allocated,
              'actual_allocated_recovered_bytes': before_allocated - after_allocated,
              'free_before_bytes': before_free, 'free_after_bytes': shutil.disk_usage(check.ROOT).free,
              'all_inputs_plain_and_sealed': True, 'normal_functional_guards_changed': False}
    durable_json(result_path, result)
    assert sum(path.stat().st_size for path in work.glob('*.json')) < 5 * check.MIB
    print(json.dumps(result), flush=True)


if __name__ == '__main__':
    main()
