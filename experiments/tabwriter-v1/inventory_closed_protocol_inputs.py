#!/usr/bin/env python3
"""Inventory only protocol006–016 closed successful source/CIF copies against previously sealed success inputs."""
from pathlib import Path
import json
import os
import shutil
from consolidate_success_inputs import identity, durable_json
import check


def key(row):
    """Match complete content, size, permissions and current ownership before sharing any inode."""
    return row['sha256'], row['bytes'], row['mode'], row['uid'], row['gid']


def closed_unit(source, runtime):
    """Require genuine source/runtime/archive success bindings without touching the sealed epoch."""
    admitted = json.loads((source / 'report.json').read_text())
    actual = json.loads((runtime / 'report.json').read_text())
    archive = json.loads((runtime / 'closed-success-index.json').read_text())
    assert admitted['source_gate_passed'] and actual['all_passed'] and archive['originals_removed']
    assert actual['source_report_sha256'] == check.digest(source / 'report.json')
    assert archive['report_sha256'] == check.digest(runtime / 'report.json')
    assert len(actual['runs']) == 2 and all(row['returncode'] == 0 and row['stdout'] == '' and
                                          'live=0 immortal=0 leaked=0' in row['stderr'] for row in actual['runs'])
    assert admitted['compiler_sha256'] == actual['compiler_sha256'] == check.COMPILER
    assert admitted['runtime_sha256'] == actual['runtime_sha256'] == check.RUNTIME
    return {'source': str(source), 'runtime': str(runtime),
            'source_report_sha256': check.digest(source / 'report.json'),
            'runtime_report_sha256': check.digest(runtime / 'report.json'),
            'archive_index_sha256': check.digest(runtime / 'closed-success-index.json')}


def input_paths(source, runtime):
    """Keep only plain source/CIF files, excluding Main authorities, manifests, binaries and provider artifacts."""
    return [path for folder in (source, runtime / 'project') for path in sorted(folder.rglob('*'))
            if path.is_file() and not path.is_symlink() and path.suffix in ('.cinch', '.cinchif')
            and path.name not in ('Main.cinch', 'Main.cinchif')]


def protocol_unit(index):
    """Resolve the original independently created success epochs without copying or relabeling them."""
    if index < 2:
        return (check.ROOT / f'build/tabwriter-protocol-{index:03d}-source-v1',
                check.ROOT / f'build/tabwriter-protocol-{index:03d}-runtime-v1')
    root = check.ROOT / 'build/tabwriter-protocol-full-330-v1'
    return root / f'tabwriter-protocol-{index:03d}-source', root / f'tabwriter-protocol-{index:03d}-runtime'


def main():
    """Produce a durable review plan and estimate only; never link, unlink or restamp an input during inventory."""
    os.sched_setaffinity(0, {2})
    assert shutil.disk_usage(check.ROOT).free >= 300 * check.MIB
    work = check.ROOT / 'build/tabwriter-protocol-input-inventory-v2'
    assert not work.exists(), 'never overwrite a reviewed recovery plan'
    existing, previous_units = {}, []
    prior = [protocol_unit(index) for index in range(6)]
    pure_root = check.ROOT / 'build/tabwriter-format-full-330-v1'
    prior += [(pure_root / f'tabwriter-format-{index:03d}-source',
               pure_root / f'tabwriter-format-{index:03d}-runtime') for index in range(23)]
    for source, runtime in prior:
        previous_units.append(closed_unit(source, runtime))
        for path in input_paths(source, runtime):
            row = identity(path)
            existing.setdefault(key(row), row)
    paths, units = [], []
    for index in range(6, 17):
        source, runtime = protocol_unit(index)
        units.append(closed_unit(source, runtime))
        admitted = json.loads((source / 'report.json').read_text())
        for name, entry in admitted['sources'].items():
            if name.startswith(('Text.Tabwriter', 'Effect.Tabwriter')):
                assert check.digest(check.HERE / 'sources' / (name + '.cinch')) == entry['sha256']
        paths.extend(identity(path) for path in input_paths(source, runtime))
    assert all(row['nlink'] == 1 for row in paths), 'only new independent closed-success copies are candidates'
    canonicals, canonical_ids, replacements = [], {}, []
    for index, row in enumerate(paths):
        signature = key(row)
        if signature not in existing:
            existing[signature] = row
            continue
        canonical = existing[signature]
        if canonical['path'] not in canonical_ids:
            canonical_ids[canonical['path']] = len(canonicals)
            canonicals.append(canonical)
        replacements.append({'target': index, 'canonical': canonical_ids[canonical['path']]})
    logical_recovery = sum(paths[row['target']]['bytes'] for row in replacements)
    allocated_recovery = sum(Path(paths[row['target']]['path']).stat().st_blocks * 512 for row in replacements)
    plan = {'scope': __doc__, 'target_protocol_indices': list(range(6, 17)),
            'units': units, 'prior_closed_units': previous_units, 'paths': paths, 'canonicals': canonicals,
            'replacements': replacements, 'logical_input_bytes': sum(row['bytes'] for row in paths),
            'logical_payload_recovery_estimate': logical_recovery, 'allocated_recovery_estimate': allocated_recovery,
            'no_input_mutation_started': True, 'all_paths_remain_plain': True,
            'sealed_against_future_compiler_writes': True,
            'main_active_failure_frozen_provider_runtime_benchmark_excluded': True,
            'shared_timestamp_policy': 'Original per-path nsmtime is recorded; linked aliases share canonical timestamps; no restamping.',
            'free_at_inventory_bytes': shutil.disk_usage(check.ROOT).free}
    work.mkdir()
    durable_json(work / 'plan.json', plan)
    assert (work / 'plan.json').stat().st_size < 5 * check.MIB
    print(json.dumps({'plan': str(work / 'plan.json'), 'sha256': check.digest(work / 'plan.json'),
                      'candidate_paths': len(paths), 'canonical_paths': len(canonicals), 'replacements': len(replacements),
                      'logical_input_bytes': plan['logical_input_bytes'], 'logical_payload_recovery_estimate': logical_recovery,
                      'allocated_recovery_estimate': allocated_recovery, 'ledger_bytes': (work / 'plan.json').stat().st_size}), flush=True)


if __name__ == '__main__':
    main()
