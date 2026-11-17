"""Apply only the exact Root-approved closed SOURCE/CIF sharing plan with a durable journal."""
from pathlib import Path
import collections
import hashlib
import importlib.util
import json
import os
import stat
import sys

BASE = Path(__file__).resolve().parent
PLAN_SHA = 'd7d024c027d3be73f970a56d90730f89dc830c8c62ac7c18385ad44b3b1bf60d'
LEDGER_SHA = 'fbfaef44cb57b85c0a65c0104ead5adc865b9611af6cc22374591194efa5f87e'
MIB = 1024 * 1024
sys.dont_write_bytecode = True
spec = importlib.util.spec_from_file_location('approved_planner', BASE / 'prepare.py')
planner = importlib.util.module_from_spec(spec)
spec.loader.exec_module(planner)


def digest(data):
    """Hash exact bytes for immutable approval and full-path checks."""
    return hashlib.sha256(data).hexdigest()


def compact(value):
    """Use bounded JSONL/JSON payloads with full paths supplied by the durable ledger."""
    return json.dumps(value, sort_keys=True, separators=(',', ':')).encode()


def free_bytes():
    """Observe shared-filesystem reserve without attributing unrelated agent changes."""
    info = os.statvfs(BASE)
    return info.f_bavail * info.f_frsize


def payload_bytes():
    """Account for every retained plan, script, ledger, journal and final-index payload."""
    return sum(path.stat().st_size for path in BASE.rglob('*') if path.is_file())


def allocated_bytes():
    """Count this plan's actual allocated file and directory blocks once per inode."""
    paths = [BASE] + list(BASE.rglob('*'))
    inodes = {(path.stat().st_dev, path.stat().st_ino): path.stat().st_blocks * 512 for path in paths}
    return sum(inodes.values())


def durable_file(path):
    """Flush an existing qualification/metadata payload without changing its bytes or mode."""
    descriptor = os.open(path, os.O_RDONLY)
    try:
        os.fsync(descriptor)
    finally:
        os.close(descriptor)


def durable_directory(path):
    """Flush the directory entry update after an atomic path replacement."""
    descriptor = os.open(path, os.O_RDONLY | os.O_DIRECTORY)
    try:
        os.fsync(descriptor)
    finally:
        os.close(descriptor)


def validate_original(row, require_inode=True):
    """Recheck exact approved regular-file bytes, ownership, mode and original ns metadata."""
    path = Path(row['path'])
    info = path.lstat()
    assert stat.S_ISREG(info.st_mode) and not path.is_symlink(), str(path)
    assert info.st_size == row['bytes'] and stat.S_IMODE(info.st_mode) == int(row['mode'], 8)
    assert (info.st_uid, info.st_gid, info.st_mtime_ns) == (row['uid'], row['gid'], row['mtime_ns'])
    if require_inode:
        assert (info.st_dev, info.st_ino, info.st_nlink, info.st_ctime_ns) == (row['device'], row['inode'], row['nlink'], row['ctime_ns']), str(path)
    assert digest(path.read_bytes()) == row['sha256'], str(path)
    return info


def verify_bindings(plan, ledger):
    """Reverify whole source/doc process evidence and exact SOURCE/CIF identity markers."""
    assert digest((planner.EVIDENCE / 'manifest-source-v2.json').read_bytes()) == plan['source_manifest_sha256']
    for closure in plan['whole_source_doc_qualification_bindings']:
        directory = planner.EVIDENCE / closure['epoch']
        receipt = json.loads((directory / 'receipt.json').read_bytes())
        docs = json.loads((directory / 'docs.json').read_bytes())
        inputs = json.loads((directory / 'inputs.json').read_bytes())
        assert digest((directory / 'receipt.json').read_bytes()) == closure['receipt_sha256']
        assert digest((directory / 'docs.json').read_bytes()) == closure['docs_sha256']
        assert digest((directory / 'inputs.json').read_bytes()) == closure['inputs_sha256']
        assert receipt['source_passed'] and receipt['guard_violation'] is None
        assert [command['exit_code'] for command in receipt['commands']] == [0, 0]
        assert receipt['commands'][0] == closure['canonical_doc_process'] and receipt['commands'][1] == closure['source_check']
        assert docs['diagnostics'] == [] and docs['orphans'] == []
        assert docs['sourceSha256'] == closure['entry_source_sha256']
        assert inputs['builtin_Data_Bytes_authority'] and inputs['builtin_Data_Core_authority']
        for name in ['build_marker', 'store_marker']:
            marker = closure[name]
            assert digest(Path(marker['path']).read_bytes()) == marker['sha256']
    cache = {}
    for row in ledger['eligible_paths']:
        data = Path(row['path']).read_bytes()
        assert digest(data) == row['sha256']
        qualification = row['qualification']
        if row['class'] == 'SOURCE':
            assert digest(data) == qualification['source_input_sha256']
            assert planner.fnv(data) == qualification['source_fnv_marker']
            witness_data = Path(qualification['CIF_witness_path']).read_bytes()
            assert digest(witness_data) == qualification['CIF_witness_sha256']
            assert json.loads(witness_data)['source_hash'] == qualification['source_fnv_marker']
        else:
            fingerprint = row['sha256']
            if fingerprint not in cache:
                interface = json.loads(data)
                assert planner.canonical(interface) == data
                contract = dict(interface)
                contract['source_hash'] = ''
                contract['compiler'] = ''
                contract['units'] = []
                cache[fingerprint] = (interface, planner.fnv(planner.canonical(contract)))
            interface, contract_hash = cache[fingerprint]
            assert interface['source_hash'] == qualification['source_fnv_marker']
            assert contract_hash == qualification['contract_fnv_marker']
            marker = json.loads(Path(qualification['marker_path']).read_bytes())
            modules = marker['modules']
            assert next(module['iface_hash'] for module in modules if module['module'] == interface['module']) == contract_hash


def active_writer_audit(entries):
    """Check readable process entries and retain every denied surface explicitly."""
    selected_inodes = {(row['device'], row['inode']) for row in entries}
    selected_paths = {row['path'] for row in entries}
    writers = []
    denied = {}
    inspected = 0
    for process in Path('/proc').iterdir():
        if not process.name.isdigit():
            continue
        try:
            try:
                descriptors = list((process / 'fd').iterdir())
                inspected += 1
            except PermissionError:
                denied.setdefault(process.name, []).append('fd-directory')
                descriptors = []
            for descriptor in descriptors:
                try:
                    info = descriptor.stat()
                    if (info.st_dev, info.st_ino) not in selected_inodes:
                        continue
                    details = (process / 'fdinfo' / descriptor.name).read_text()
                    flags = int(next(line.split(':', 1)[1].strip() for line in details.splitlines() if line.startswith('flags:')), 8)
                    if flags & os.O_ACCMODE in (os.O_WRONLY, os.O_RDWR):
                        writers.append({'pid': process.name, 'fd': descriptor.name, 'path': os.readlink(descriptor)})
                except (FileNotFoundError, ProcessLookupError):
                    pass
                except PermissionError:
                    denied.setdefault(process.name, []).append('fd-' + descriptor.name)
            maps = process / 'maps'
            try:
                lines = maps.read_text().splitlines()
            except PermissionError:
                denied.setdefault(process.name, []).append('maps')
                lines = []
            for line in lines:
                fields = line.split(None, 5)
                if len(fields) == 6 and fields[1][1:4] == 'w-s' and fields[5] in selected_paths:
                    writers.append({'pid': process.name, 'shared_mapping': fields[5]})
        except (FileNotFoundError, ProcessLookupError):
            continue
        except PermissionError:
            denied.setdefault(process.name, []).append('other-process-entry')
    assert not writers, writers
    return {'readable_fd_processes': inspected, 'readable_writers_found': writers,
            'denied_process_surfaces': denied,
            'limitation': 'Unreadable host processes were not inspected. No universal /proc visibility claim.',
            'ownership_handoff': 'Root explicitly confirms the seven closed lane-owned units are sealed, no other team task may write/compile them, own compiler/SDK/runtime jobs are stopped, and Root active JSON tests use separate projects.'}


def unique_allocation(entries):
    """Count actual data allocation by unique inode, allowing shared links once."""
    inodes = {}
    for row in entries:
        info = Path(row['path']).stat()
        inodes[(info.st_dev, info.st_ino)] = info.st_blocks * 512
    return sum(inodes.values())


def main():
    """Atomically replace only approved duplicate paths after durable original metadata."""
    initial_free = free_bytes()
    assert initial_free >= 300 * MIB, ('300 MiB start guard', initial_free)
    plan_data = (BASE / 'plan.json').read_bytes()
    ledger_data = (BASE / 'ledger.json').read_bytes()
    assert digest(plan_data) == PLAN_SHA and digest(ledger_data) == LEDGER_SHA
    plan = json.loads(plan_data)
    ledger = json.loads(ledger_data)
    assert digest((BASE / 'prepare.py').read_bytes()) == plan['planner_sha256']
    assert len(plan['groups']) == 82 and plan['shared_member_paths'] == 982
    assert len(ledger['eligible_paths']) == 990
    entries = ledger['eligible_paths']
    assert all(row['nlink'] == 1 for row in entries)
    by_id = {row['id']: row for row in entries}
    successful_dirs = {planner.EVIDENCE / row['epoch'] for row in plan['whole_source_doc_qualification_bindings']}
    assert len(successful_dirs) == 7
    for row in entries:
        path = Path(row['path'])
        assert any(directory in path.parents for directory in successful_dirs)
        assert row['class'] in ['SOURCE', 'CIF']
        validate_original(row)
    for row in ledger['protected_leaf_paths']:
        validate_original(row)
    verify_bindings(plan, ledger)
    writer_audit = active_writer_audit(entries)
    before_allocation = unique_allocation(entries)
    parent_directories = {Path(row['path']).parent for row in entries}
    directory_before = {str(path): {'blocks_512': path.stat().st_blocks, 'mtime_ns': path.stat().st_mtime_ns} for path in parent_directories}
    final_reserve = len(compact(entries)) + 32768
    journal_reserve = 2048 + 512 * (plan['shared_member_paths'] - plan['sharing_groups'])
    assert payload_bytes() + final_reserve + journal_reserve + 16384 < 3 * MIB, 'prospective metadata cap'
    assert not (BASE / 'journal.jsonl').exists()
    assert not (BASE / 'final-index.json').exists()
    durable_file(BASE / 'plan.json')
    durable_file(BASE / 'ledger.json')
    durable_file(BASE / 'prepare.py')
    durable_file(BASE / 'apply.py')
    for closure in plan['whole_source_doc_qualification_bindings']:
        directory = planner.EVIDENCE / closure['epoch']
        for name in ['receipt.json', 'inputs.json', 'docs.json']:
            durable_file(directory / name)
        durable_file(Path(closure['build_marker']['path']))
        durable_file(Path(closure['store_marker']['path']))
    durable_directory(BASE)
    journal = (BASE / 'journal.jsonl').open('xb')
    actions = 0
    minimum_free = initial_free

    def event(value):
        """Persist one write-ahead/completion record before the following mutation."""
        nonlocal minimum_free
        data = compact(value) + b'\n'
        assert payload_bytes() + len(data) + final_reserve + 16384 <= 3*MIB, 'metadata cap before journal append'
        minimum_free = min(minimum_free, free_bytes())
        assert minimum_free >= 250*MIB, ('250 MiB reserve', minimum_free)
        journal.write(data)
        journal.flush()
        os.fsync(journal.fileno())

    try:
        event({'phase': 'durable-preflight', 'plan_sha256': PLAN_SHA, 'ledger_sha256': LEDGER_SHA,
               'writer_audit': writer_audit, 'all_original_metadata_durable': True,
               'canonical_visible_metadata_map_durable': True, 'qualified_bindings_reverified': True})
        durable_directory(BASE)
        for group in plan['groups']:
            canonical_row = by_id[group['canonical_id']]
            canonical_path = Path(canonical_row['path'])
            durable_file(canonical_path)
            canonical_info = canonical_path.stat()
            for identifier in group['member_ids']:
                if identifier == group['canonical_id']:
                    continue
                target_row = by_id[identifier]
                target = Path(target_row['path'])
                current = validate_original(target_row)
                if (current.st_dev, current.st_ino) == (canonical_info.st_dev, canonical_info.st_ino):
                    continue
                temporary = target.with_name('.entropy-sharing-' + str(os.getpid()) + '-' + str(identifier) + '.tmp')
                assert not temporary.exists()
                event({'phase': 'intent', 'group': group['id'], 'target_id': identifier,
                       'canonical_id': group['canonical_id'], 'original_inode': current.st_ino,
                       'canonical_inode': canonical_info.st_ino, 'temporary_name': temporary.name})
                validate_original(target_row)
                validate_original(canonical_row, require_inode=False)
                assert (canonical_path.stat().st_dev, canonical_path.stat().st_ino) == (canonical_info.st_dev, canonical_info.st_ino)
                os.link(canonical_path, temporary, follow_symlinks=False)
                durable_directory(target.parent)
                os.replace(temporary, target)
                durable_directory(target.parent)
                info = target.stat()
                assert (info.st_dev, info.st_ino) == (canonical_info.st_dev, canonical_info.st_ino)
                assert info.st_mtime_ns == canonical_row['mtime_ns']
                actions += 1
                event({'phase': 'done', 'group': group['id'], 'target_id': identifier,
                       'inode': info.st_ino, 'mtime_ns': info.st_mtime_ns})
        final_entries = []
        for row in entries:
            path = Path(row['path'])
            info = path.lstat()
            assert stat.S_ISREG(info.st_mode) and not path.is_symlink()
            assert digest(path.read_bytes()) == row['sha256'] and info.st_size == row['bytes']
            assert (stat.S_IMODE(info.st_mode), info.st_uid, info.st_gid) == (int(row['mode'], 8), row['uid'], row['gid'])
            final_entries.append({'id': row['id'], 'path': row['path'], 'sha256': row['sha256'], 'bytes': info.st_size,
                                  'device': info.st_dev, 'inode': info.st_ino, 'nlink': info.st_nlink,
                                  'mode': row['mode'], 'uid': info.st_uid, 'gid': info.st_gid,
                                  'mtime_ns': info.st_mtime_ns, 'ctime_ns': info.st_ctime_ns})
        for group in plan['groups']:
            canonical_row = by_id[group['canonical_id']]
            canonical_info = Path(canonical_row['path']).stat()
            for identifier in group['member_ids']:
                info = Path(by_id[identifier]['path']).stat()
                assert (info.st_dev, info.st_ino) == (canonical_info.st_dev, canonical_info.st_ino)
                assert info.st_mtime_ns == canonical_row['mtime_ns']
            assert canonical_info.st_nlink == len(group['member_ids'])
        grouped_ids = {identifier for group in plan['groups'] for identifier in group['member_ids']}
        for row in entries:
            if row['id'] not in grouped_ids:
                validate_original(row)
        for row in ledger['protected_leaf_paths']:
            validate_original(row)
        verify_bindings(plan, ledger)
        assert not [path for directory in parent_directories for path in directory.glob('.entropy-sharing-*.tmp')]
        final_data = compact({'plan_sha256': PLAN_SHA, 'ledger_sha256': LEDGER_SHA, 'eligible_paths': final_entries}) + b'\n'
        assert payload_bytes() + len(final_data) + 16384 <= 3*MIB
        with (BASE / 'final-index.json').open('xb') as output:
            output.write(final_data)
            output.flush()
            os.fsync(output.fileno())
        durable_directory(BASE)
        after_allocation = unique_allocation(entries)
        directory_growth = sum(max(0, path.stat().st_blocks - directory_before[str(path)]['blocks_512']) * 512 for path in parent_directories)
        event({'phase': 'complete', 'actions': actions, 'groups': len(plan['groups']), 'all_990_paths_plain': True,
               'all_bytes_and_qualification_markers_unchanged': True, 'all_group_inodes_verified': True,
               'protected_leaf_files_unchanged': len(ledger['protected_leaf_paths']),
               'final_index_sha256': digest(final_data)})
    finally:
        journal.flush()
        os.fsync(journal.fileno())
        journal.close()
        durable_directory(BASE)
    result = {'status': 'completed-approved-hardlink-sharing', 'plan_sha256': PLAN_SHA, 'ledger_sha256': LEDGER_SHA,
              'actual_replacements': actions, 'groups': len(plan['groups']), 'grouped_paths': plan['shared_member_paths'],
              'all_eligible_paths_verified_plain': len(entries), 'protected_leaf_paths_verified_unchanged': len(ledger['protected_leaf_paths']),
              'actual_selected_allocation_before_bytes': before_allocation, 'actual_selected_allocation_after_bytes': after_allocation,
              'actual_gross_selected_allocation_recovered_bytes': before_allocation - after_allocation,
              'actual_selected_parent_directory_allocation_growth_bytes': directory_growth,
              'initial_free_bytes': initial_free, 'minimum_free_bytes': minimum_free,
              'final_index_sha256': digest((BASE / 'final-index.json').read_bytes()),
              'journal_sha256': digest((BASE / 'journal.jsonl').read_bytes()),
              'source_units_frozen_no_future_compilation': True, 'runtime_or_compiler_executed': False,
              'scope': 'source/doc-qualified SOURCE/CIF only; no runtime qualification claim',
              'allocation_scope': 'Actual selected inode blocks minus all retained plan metadata and selected parent-directory growth; shared Git bookkeeping excluded.',
              'directory_metadata_before': directory_before}
    for unused in range(4):
        result['retained_metadata_payload_bytes'] = payload_bytes()
        result['actual_plan_metadata_allocated_bytes'] = allocated_bytes()
        result['actual_net_allocated_recovery_bytes'] = before_allocation - after_allocation - allocated_bytes() - directory_growth
        result['final_free_bytes_observed_unattributed'] = free_bytes()
        data = compact(result) + b'\n'
        assert payload_bytes() + len(data) <= 3*MIB
        with (BASE / 'result.json').open('wb') as output:
            output.write(data)
            output.flush()
            os.fsync(output.fileno())
        durable_directory(BASE)
    assert payload_bytes() <= 3*MIB and free_bytes() >= 250*MIB
    print(json.dumps({key: result[key] for key in ['status', 'actual_replacements', 'groups', 'all_eligible_paths_verified_plain',
                       'actual_gross_selected_allocation_recovered_bytes', 'actual_plan_metadata_allocated_bytes',
                       'actual_net_allocated_recovery_bytes', 'final_index_sha256', 'journal_sha256', 'retained_metadata_payload_bytes']}, indent=2))


if __name__ == '__main__':
    main()
