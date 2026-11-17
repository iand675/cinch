#!/usr/bin/env python3
"""Verify exact current-source transfer successes separately from the plain preserved retained-cycle failure."""
from pathlib import Path
import argparse
import hashlib
import json
import tarfile
import check
import verify


def validate_archive(work, row):
    """Stream the verified closed-success members without restoring them or accepting a failed artifact archive."""
    index_path = work / 'closed-success-index.json'
    assert check.digest(index_path) == row['closed_success_index_sha256']
    index = json.loads(index_path.read_text())
    assert index == row['closed_success_archive_index'] and index['originals_removed']
    assert index['report_sha256'] == row['runtime_report_sha256']
    archive = Path(index['archive'])
    assert check.digest(archive) == index['compressed_sha256']
    assert index['compressed_bytes'] <= 10 * 1024 * 1024
    expected = {entry['member']: entry for entry in index['entries']}
    seen = set()
    with tarfile.open(archive, 'r:gz') as source:
        for member in source:
            assert member.name in expected and member.name not in seen and member.isfile()
            entry = expected[member.name]
            assert (member.mode, member.size, member.uid, member.gid) == (entry['mode'], entry['bytes'], entry['uid'], entry['gid'])
            assert member.pax_headers['CINCH.mtime_ns'] == str(entry['mtime_ns'])
            digest = hashlib.sha256()
            with source.extractfile(member) as stream:
                for chunk in iter(lambda: stream.read(1024 * 1024), b''):
                    digest.update(chunk)
            assert digest.hexdigest() == entry['sha256'] == row['runtime'][entry['report_hash_field']]
            assert not Path(entry['path']).exists()
            seen.add(member.name)
    assert seen == set(expected) == {'consumer', 'consumer.ll'}


def main():
    """Require source and artifact bindings, both strict profiles, and the unchanged explicit promotion hold."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--artifacts', type=Path, default=check.ROOT / 'build')
    args = parser.parse_args()
    verify.main()
    gates = json.loads((check.HERE / 'runtime-gates.json').read_text())
    manifest_path = check.HERE / 'runtime-fixtures/manifest.json'
    assert check.digest(manifest_path) == gates['fixture_manifest_sha256']
    assert check.digest(check.HERE / 'proposal.json') == gates['proposal_sha256']
    assert gates['compiler_sha256'] == check.COMPILER and gates['runtime_sha256'] == check.RUNTIME
    assert not gates['production_adopted'] and not gates['full80_complete']
    fixtures = json.loads(manifest_path.read_text())
    for name, row in gates['cases'].items():
        source = args.artifacts / row['source_work']
        work = args.artifacts / row['runtime_work']
        source_report_path = source / 'report.json'
        runtime_report_path = work / 'report.json'
        assert check.digest(source_report_path) == row['source_report_sha256']
        assert check.digest(runtime_report_path) == row['runtime_report_sha256']
        admitted = json.loads(source_report_path.read_text())
        report = json.loads(runtime_report_path.read_text())
        assert report == row['runtime']
        assert admitted['source_gate_passed'] and admitted['returncode'] == 0 and not admitted['timeout_without_verdict']
        assert report['source_receipt_sha256'] == row['source_report_sha256']
        assert report['compiler_sha256'] == check.COMPILER and report['runtime_sha256'] == check.RUNTIME
        assert report['source_sha256'] == row['fixture_sha256'] == fixtures[name]['source_sha256']
        assert check.digest(check.HERE / fixtures[name]['source']) == row['fixture_sha256']
        for module, selection in row['source_selection'].items():
            assert check.digest(source / (module + '.cinch')) == selection['sha256']
        assert len(admitted['public_contracts']) == 9
        assert len(report['runs']) == 2 and {run['poisoned'] for run in report['runs']} == {False, True}
        assert all(step['returncode'] == 0 for step in report['steps'])
        assert '-O2' in report['steps'][1]['command'] and 'clang version 20.' in report['clang_version']
        for run in report['runs']:
            assert run['heap'] == ('256M' if run['poisoned'] else '32M') and run['stack'] == '8M'
            if name == 'transfer_priority':
                assert run['returncode'] == 0 and run['stdout'] == ''
                assert 'live=0 immortal=0 leaked=0' in run['stderr']
            else:
                assert run['returncode'] == 74 and run['stdout'] == 'retained-cycle-ready\n'
                assert 'live=22 immortal=0 leaked=22' in run['stderr']
                assert run['known_cycle_failure_observed']
        if name == 'transfer_priority':
            assert report['all_passed'] and not report['known_cycle_failure_reproduced']
            validate_archive(work, row)
        else:
            assert not report['all_passed'] and report['known_cycle_failure_reproduced']
            assert not (work / 'closed-success-index.json').exists()
            assert check.digest(work / 'consumer') == report['executable_sha256']
            assert check.digest(work / 'consumer.ll') == report['llvm_sha256']
    for name in ('RetainedCycle-marker-v1', 'RetainedCycle-capability-drop-v2'):
        repro = json.loads((check.HERE / 'repros' / (name + '.json')).read_text())
        assert check.digest(check.HERE / 'repros' / (name + '.cinch')) == repro['source_sha256']
        assert repro['compiler_sha256'] == check.COMPILER and repro['runtime_sha256'] == check.RUNTIME
    prior = args.artifacts / 'buffered-region-330-cycle-runtime-v1'
    failure = json.loads((check.HERE / 'repros/RetainedCycle-capability-drop-v2.json').read_text())
    assert check.digest(prior / 'report.json') == failure['report_sha256']
    assert check.digest(prior / 'consumer') == failure['executable_sha256']
    assert check.digest(prior / 'consumer.ll') == failure['llvm_sha256']
    assert failure['runs'][0]['returncode'] == 2 and 'live=0 immortal=0 leaked=0' in failure['runs'][0]['stderr']
    print('Verified current4-group zero-leak controls and separate observed22-block cycle failure; promotion remains held')


if __name__ == '__main__':
    main()
