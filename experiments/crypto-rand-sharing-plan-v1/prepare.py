"""Audit closed source-check outputs and prepare metadata-only sharing proposals."""
from pathlib import Path
import collections
import hashlib
import json
import os
import stat

BASE = Path(__file__).resolve().parent
LANE = BASE.parent.parent
EVIDENCE = LANE / 'experiments/crypto-rand-source-v1'
SEALED_MANIFEST_SHA = '9813c82cf26422b8c52b5d09beb4b4d740052ca9fa931dbdbef75b932be07bd3'
MIB = 1024 * 1024


def sha(data):
    """Hash exact payload bytes independently of the compiler identity markers."""
    return hashlib.sha256(data).hexdigest()


def canonical(value):
    """Match the verified compact sorted-key interface JSON representation."""
    return json.dumps(value, ensure_ascii=False, sort_keys=True, separators=(',', ':')).encode()


def fnv(data):
    """Reproduce the documented FNV-1a artifact marker, never a security hash."""
    value = 0xcbf29ce484222325
    for byte in data:
        value = ((value ^ byte) * 0x100000001b3) & 0xffffffffffffffff
    return str(value)


def metadata(path, info):
    """Preserve full path and original nanosecond metadata before any proposal."""
    return {'path': str(path), 'bytes': info.st_size, 'mode': oct(stat.S_IMODE(info.st_mode)),
            'uid': info.st_uid, 'gid': info.st_gid, 'device': info.st_dev, 'inode': info.st_ino,
            'nlink': info.st_nlink, 'blocks_512': info.st_blocks,
            'mtime_ns': info.st_mtime_ns, 'ctime_ns': info.st_ctime_ns, 'atime_ns_before_read': info.st_atime_ns}


def free_bytes():
    """Check the shared-filesystem read-only plan resource guard."""
    info = os.statvfs(BASE)
    return info.f_bavail * info.f_frsize


def main():
    """Read and bind closed evidence; write a plan and ledger without making links."""
    initial = free_bytes()
    assert initial >= 300 * MIB, ('300 MiB start guard', initial)
    raw_manifest = (EVIDENCE / 'manifest-source-v2.json').read_bytes()
    assert sha(raw_manifest) == SEALED_MANIFEST_SHA
    manifest = json.loads(raw_manifest)
    successful = [unit for unit in manifest['units'] if unit['source_passed']]
    assert len(successful) == 7
    entries = []
    closures = []
    excluded = []
    contract_cache = {}
    minimum = initial
    for unit in successful:
        directory = EVIDENCE / unit['epoch']
        receipt_data = (directory / 'receipt.json').read_bytes()
        input_data = (directory / 'inputs.json').read_bytes()
        doc_data = (directory / 'docs.json').read_bytes()
        assert sha(receipt_data) == unit['receipt_sha256']
        assert sha(input_data) == unit['inputs_sha256']
        assert sha(doc_data) == unit['docs_sha256']
        receipt = json.loads(receipt_data)
        inputs = json.loads(input_data)
        docs = json.loads(doc_data)
        assert receipt['source_passed'] and receipt['guard_violation'] is None
        assert len(receipt['commands']) == 2 and all(command['exit_code'] == 0 for command in receipt['commands'])
        assert receipt['commands'][0]['argv'][4:6] == ['doc', 'extract']
        assert receipt['commands'][1]['argv'][4] == 'check'
        assert docs['diagnostics'] == [] and docs['orphans'] == []
        assert inputs['builtin_Data_Bytes_authority'] and inputs['builtin_Data_Core_authority']
        by_relative = {row['path']: row for row in unit['artifacts']}
        loaded = {}
        for artifact in unit['artifacts']:
            path = EVIDENCE / artifact['path']
            info = path.lstat()
            assert stat.S_ISREG(info.st_mode) and not path.is_symlink()
            data = path.read_bytes()
            assert len(data) == artifact['bytes'] and sha(data) == artifact['sha256'], str(path)
            loaded[path] = (data, info)
        build_marker_path = directory / 'project/build' / (unit['module'] + '.manifest.json')
        build_marker = json.loads(loaded[build_marker_path][0])
        build_hashes = {row['module']: row['iface_hash'] for row in build_marker['modules']}
        store_paths = [path for path in loaded if path.name == 'store.json']
        assert len(store_paths) == 1
        store_marker_path = store_paths[0]
        store_marker = json.loads(loaded[store_marker_path][0])
        store_hashes = {row['module']: row['iface_hash'] for row in store_marker['modules']}
        closure = {'epoch': unit['epoch'], 'source_check': receipt['commands'][1], 'canonical_doc_process': receipt['commands'][0],
                   'receipt_sha256': sha(receipt_data), 'inputs_sha256': sha(input_data), 'docs_sha256': sha(doc_data),
                   'entry_source_sha256': docs['sourceSha256'], 'compiler_sha256': receipt['compiler_sha256_after'],
                   'source_qualified_only': True, 'runtime_qualified': False,
                   'build_marker': {'path': str(build_marker_path), 'sha256': sha(loaded[build_marker_path][0])},
                   'store_marker': {'path': str(store_marker_path), 'sha256': sha(loaded[store_marker_path][0])}}
        assert closure['compiler_sha256'] == manifest['compiler_sha256']
        closures.append(closure)
        source_rows = {row['module']: row for row in inputs['sources']}
        entry_path = directory / 'project' / (unit['module'] + '.cinch')
        assert sha(loaded[entry_path][0]) == docs['sourceSha256']
        for path, (data, info) in sorted(loaded.items()):
            relative = path.relative_to(directory)
            is_source = path.suffix == '.cinch' and relative.parts[0] == 'project' and 'build' not in relative.parts
            is_cif = path.suffix == '.cinchif' and ('build' in relative.parts or relative.parts[0] == 'store')
            if not (is_source or is_cif):
                protected = metadata(path, info)
                protected.update({'reason': 'not SOURCE/CIF; original qualification metadata stays separate', 'sha256': sha(data)})
                excluded.append(protected)
                continue
            row = metadata(path, info)
            row.update({'id': len(entries), 'epoch': unit['epoch'], 'class': 'SOURCE' if is_source else 'CIF', 'sha256': sha(data)})
            if is_source:
                module = path.stem
                witness_path = directory / 'project/build' / (module + '.cinchif')
                witness = json.loads(loaded[witness_path][0])
                assert sha(data) == source_rows[module]['sha256']
                assert fnv(data) == witness['source_hash']
                row['qualification'] = {'module': module, 'source_input_sha256': source_rows[module]['sha256'],
                                        'source_fnv_marker': witness['source_hash'], 'CIF_witness_path': str(witness_path),
                                        'CIF_witness_sha256': sha(loaded[witness_path][0])}
            else:
                fingerprint = sha(data)
                if fingerprint not in contract_cache:
                    interface = json.loads(data)
                    assert canonical(interface) == data
                    contract = dict(interface)
                    contract['source_hash'] = ''
                    contract['compiler'] = ''
                    contract['units'] = []
                    contract_cache[fingerprint] = (interface, fnv(canonical(contract)))
                interface, contract_hash = contract_cache[fingerprint]
                module = interface['module']
                expected = store_hashes[module] if relative.parts[0] == 'store' else build_hashes[module]
                assert contract_hash == expected, str(path)
                row['qualification'] = {'module': module, 'source_fnv_marker': interface['source_hash'],
                                        'contract_fnv_marker': contract_hash,
                                        'marker_path': str(store_marker_path if relative.parts[0] == 'store' else build_marker_path),
                                        'origin': 'qualified compiler builtin consumer output' if module not in source_rows else 'qualified explicit source output'}
            current = path.stat()
            assert (current.st_dev, current.st_ino, current.st_size, current.st_mtime_ns, current.st_ctime_ns) == (info.st_dev, info.st_ino, info.st_size, info.st_mtime_ns, info.st_ctime_ns)
            row['atime_ns_after_read'] = current.st_atime_ns
            entries.append(row)
        minimum = min(minimum, free_bytes())
        assert minimum >= 250 * MIB, ('250 MiB reserve', minimum)
    selected_paths = {row['path'] for row in entries}
    known_paths = {str(EVIDENCE / artifact['path']) for unit in manifest['units'] for artifact in unit['artifacts']}
    for path in sorted(EVIDENCE.rglob('*')):
        if path.is_file() and str(path) not in selected_paths and not any(row['path'] == str(path) for row in excluded):
            info = path.lstat()
            protected = metadata(path, info)
            protected.update({'reason': 'failure/launch/support/unknown; excluded', 'known_sealed_artifact': str(path) in known_paths,
                              'sha256': sha(path.read_bytes())})
            excluded.append(protected)
    inode_counts = collections.Counter((row['device'], row['inode']) for row in entries)
    by_content = collections.defaultdict(list)
    for row in entries:
        if row['nlink'] != inode_counts[(row['device'], row['inode'])]:
            row['sharing_refusal'] = 'inode has a link outside the closed eligible set'
            continue
        by_content[(row['sha256'], row['bytes'], row['mode'], row['uid'], row['gid'], row['device'])].append(row)
    groups = []
    for key, members in sorted(by_content.items()):
        unique = {(row['device'], row['inode']): row for row in members}
        if len(unique) < 2:
            continue
        members.sort(key=lambda row: row['path'])
        canonical_row = members[0]
        other_inodes = {inode: row for inode, row in unique.items() if inode != (canonical_row['device'], canonical_row['inode'])}
        recovered_blocks = sum(row['blocks_512'] * 512 for row in other_inodes.values())
        if recovered_blocks <= 0:
            continue
        groups.append({'id': len(groups), 'sha256': key[0], 'bytes_per_path': key[1], 'mode': key[2], 'uid': key[3], 'gid': key[4],
                       'canonical_id': canonical_row['id'], 'member_ids': [row['id'] for row in members],
                       'distinct_inodes_before': len(unique), 'logical_duplicate_bytes': key[1] * (len(unique) - 1),
                       'estimated_recoverable_allocation_bytes': recovered_blocks,
                       'visible_metadata_after_sharing': {'mtime_ns': canonical_row['mtime_ns'], 'mode': canonical_row['mode'],
                                                         'uid': canonical_row['uid'], 'gid': canonical_row['gid'],
                                                         'atime': 'canonical inode access time; reads can update it',
                                                         'ctime': 'kernel changes during linking; exact future ns value cannot be predicted',
                                                         'nlink_expected': len(members)},
                       'original_per_path_mtime_ns_preserved_in': 'ledger.json, member IDs'})
    protected_prefixes = [str(LANE / 'experiments/random-provider-v1'), str(LANE / 'experiments/crypto-rand-surface-v1'),
                          str(EVIDENCE / 'token-exact-6fea'), str(EVIDENCE / 'prime-candidate-exact-6fea'),
                          '/workspace/dasl-stdlib-io', str(LANE / 'compiler'), str(LANE / 'build')]
    ledger = {'schema': 1, 'status': 'read-only-original-per-path-ledger', 'source_checkpoint_sha256': SEALED_MANIFEST_SHA,
              'eligible_paths': entries, 'protected_leaf_paths': excluded, 'protected_prefixes': protected_prefixes}
    plan = {'schema': 1, 'status': 'PROPOSED-NO-LINKS-MADE', 'source_checkpoint': 'f95f4320ffe32f402777f6d828eb802a0584bbff',
            'planner_sha256': sha(Path(__file__).read_bytes()),
            'source_manifest_sha256': SEALED_MANIFEST_SHA, 'whole_source_doc_qualification_bindings': closures,
            'source_unit_policy': 'All seven closed units are frozen. Never compile them again or write through their source/CIF paths. Future work must copy into a new epoch.',
            'allowed_classes': ['SOURCE', 'CIF'], 'runtime_claim': False,
            'unknown_paths_policy': 'excluded by default', 'external_genuine_provider_baselines': 'fully excluded; no candidate/canonical path is outside these seven units',
            'protected_prefixes': protected_prefixes, 'groups': groups,
            'eligible_paths': len(entries), 'sharing_groups': len(groups), 'shared_member_paths': sum(len(group['member_ids']) for group in groups),
            'logical_duplicate_bytes': sum(group['logical_duplicate_bytes'] for group in groups),
            'estimated_gross_allocation_recovery_bytes': sum(group['estimated_recoverable_allocation_bytes'] for group in groups),
            'metadata_policy': 'Grouping requires exact bytes/size/mode/uid/gid/device. Hardlinks cannot retain distinct per-path mtimes; each original mtime_ns is sealed in ledger and future visible metadata is explicitly canonical. No chmod/chown/utime is proposed.',
            'revalidation_required_before_any_link': ['Root reviews exact plan/ledger hashes, net estimate and protected set', 'reverify every qualified receipt/doc/hash marker binding', 'reverify selected byte hash/size/mode/uid/gid/inode/nlink and original mtime_ns', 'stop on any changed or unknown input', 'verify all paths/hashes after sharing and measure actual filesystem recovery'],
            'resource_observations': {'initial_free_bytes': initial, 'minimum_free_bytes': minimum, 'start_guard_bytes': 300*MIB, 'reserve_bytes': 250*MIB, 'metadata_cap_bytes': 3*MIB}}
    ledger_data = (json.dumps(ledger, indent=2)+'\n').encode()
    plan['ledger_sha256'] = sha(ledger_data)
    for unused in range(5):
        plan_data = (json.dumps(plan, indent=2)+'\n').encode()
        payload = len(ledger_data) + len(plan_data) + Path(__file__).stat().st_size
        allocation = ((len(ledger_data)+4095)//4096 + (len(plan_data)+4095)//4096 + (Path(__file__).stat().st_size+4095)//4096)*4096
        plan['retained_plan_metadata_bytes'] = payload
        plan['estimated_plan_metadata_allocation_bytes'] = allocation
        plan['estimated_net_allocation_recovery_bytes'] = plan['estimated_gross_allocation_recovery_bytes'] - allocation
    plan_data = (json.dumps(plan, indent=2)+'\n').encode()
    assert len(ledger_data)+len(plan_data)+Path(__file__).stat().st_size <= 3*MIB
    (BASE/'ledger.json').write_bytes(ledger_data)
    (BASE/'plan.json').write_bytes(plan_data)
    assert free_bytes() >= 250*MIB
    print(json.dumps({'status':plan['status'],'plan_sha256':sha(plan_data),'ledger_sha256':sha(ledger_data),
                      'groups':len(groups),'eligible_paths':len(entries),'shared_members':plan['shared_member_paths'],
                      'gross_estimated_recovery_bytes':plan['estimated_gross_allocation_recovery_bytes'],
                      'net_estimated_recovery_bytes':plan['estimated_net_allocation_recovery_bytes'],
                      'metadata_bytes':len(ledger_data)+len(plan_data)+Path(__file__).stat().st_size,'actual_links_made':0},indent=2))


if __name__ == '__main__':
    main()
