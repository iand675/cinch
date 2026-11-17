#!/usr/bin/env python3
"""Verify exact twelve-policy runtime evidence without an installed reference SDK or compiler execution."""
from pathlib import Path
import hashlib
import json

HERE = Path(__file__).resolve().parent
COMPILER = '330adb4d77f8bcbfc26f246269280f391c3a10b6d3a40771b5f6243f6a8969b2'
RUNTIME = 'd91ce012395a99e1148232d20ec316979cd7d0cb13647a3283ba4820192c0abf'


def digest(path):
    """Bind maintained source and each original receipt without relabeling a tool or dependency epoch."""
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    """Require all literal adaptation identities, both fixed ownership runs and a durable verified success archive."""
    evidence = HERE / 'controls-v1'
    gates = json.loads((evidence / 'adaptation-gates.json').read_text())
    assert gates['compiler_sha256'] == COMPILER and gates['runtime_sha256'] == RUNTIME
    for name, expected in gates['files'].items():
        assert digest(evidence / name) == expected, name
    assert digest(HERE / 'AdaptMain.cinch') == gates['entry_sha256']
    for name, expected in gates['source_identities'].items():
        assert digest(HERE / 'sources' / (name + '.cinch')) == expected, name
    source_path = evidence / 'adaptation-source-v4/report.json'
    runtime_path = evidence / 'adaptation-runtime-v1/report.json'
    source = json.loads(source_path.read_text())
    runtime = json.loads(runtime_path.read_text())
    archive = json.loads((evidence / 'adaptation-runtime-v1/closed-success-index.json').read_text())
    assert source['source_gate_passed'] and source['original_main_and_library_sources_unchanged']
    assert source['main_sha256'] == gates['entry_sha256']
    assert source['source_only'] and not source['runtime_claim']
    for key in ['compiler_sha256', 'runtime_sha256', 'compiler_source']:
        assert source[key] == runtime[key] == gates[key]
    assert runtime['source_report_sha256'] == digest(source_path)
    assert runtime['all_passed'] and runtime['complete_artifact_bytes'] <= 20 * 1024 * 1024
    assert 'clang version 20.' in runtime['clang_version']
    assert [run['name'] for run in runtime['runs']] == ['normal', 'poison']
    assert all(run['returncode'] == 0 and run['interrupted_reason'] is None and run['stdout'] == ''
               and 'live=0 immortal=0 leaked=0' in run['stderr'] for run in runtime['runs'])
    assert all(step['returncode'] == 0 and step['interrupted_reason'] is None for step in runtime['steps'])
    assert archive['report_sha256'] == digest(runtime_path) and archive['originals_removed']
    assert archive['compressed_bytes'] <= 10 * 1024 * 1024
    members = {row['member']: row for row in archive['entries']}
    assert set(members) == {'consumer', 'consumer.ll'}
    assert members['consumer']['sha256'] == runtime['executable_sha256']
    assert members['consumer.ll']['sha256'] == runtime['llvm_sha256']
    plan = json.loads((HERE / 'adaptation-plan.json').read_text())
    ids = [row['reference_id'] for row in gates['cases']]
    assert len(ids) == len(set(ids)) == 12
    assert [row['failure_exit'] for row in gates['cases']] == list(range(1, 13))
    assert set(ids) == {row['reference_case'] for row in plan['records']}
    assert all(row['qualified'] for row in plan['records']) and gates['all12_runtime_qualified']
    assert not any(gates[key] for key in ['native_backend_qualified', 'whole_package_qualified',
                                       'production_adopted', 'original_reference_modified', 'panic_return_count_invented'])
    assert not plan['full_package_qualified'] and not plan['native_backend_qualified']
    print('Verified all12 explicit adaptation policies on exact330/d91, both zero-leak profiles; remaining gates stay open')


if __name__ == '__main__':
    main()
