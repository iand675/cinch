#!/usr/bin/env python3
"""Verify compact exact-epoch tabwriter runtime control receipts without an installed SDK or compiler run."""
from pathlib import Path
import hashlib
import json

HERE = Path(__file__).resolve().parent
COMPILER = '330adb4d77f8bcbfc26f246269280f391c3a10b6d3a40771b5f6243f6a8969b2'
RUNTIME = 'd91ce012395a99e1148232d20ec316979cd7d0cb13647a3283ba4820192c0abf'


def digest(path):
    """Keep each maintained entry, original source report and executed runner independently identified."""
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    """Require actual fixed memory profiles, output caps and verified archive bindings for every recorded success."""
    evidence = HERE / 'controls-v1'
    gates = json.loads((evidence / 'runtime-gates.json').read_text())
    assert gates['controls'] and len({row['name'] for row in gates['controls']}) == len(gates['controls'])
    count = 0
    for row in gates['controls']:
        for name, expected in row['files'].items():
            assert digest(evidence / name) == expected, name
        assert digest(HERE / row['entry']) == row['entry_sha256']
        for name, expected in row['candidate_source_identities'].items():
            assert digest(HERE / 'sources' / (name + '.cinch')) == expected, name
        source = json.loads((evidence / row['source']).read_text())
        runtime = json.loads((evidence / row['runtime']).read_text())
        archive = json.loads((evidence / row['archive']).read_text())
        assert source['source_gate_passed'] and source['original_main_and_library_sources_unchanged']
        assert source['main_sha256'] == row['entry_sha256']
        assert source['compiler_sha256'] == runtime['compiler_sha256'] == COMPILER
        assert source['runtime_sha256'] == runtime['runtime_sha256'] == RUNTIME
        assert source['compiler_source'] == runtime['compiler_source']
        assert runtime['source_report_sha256'] == digest(evidence / row['source'])
        assert runtime['runner_sha256'] == digest(evidence / row['runner'])
        assert runtime['all_passed'] and runtime['complete_artifact_bytes'] <= 20 * 1024 * 1024
        assert runtime['free_at_start_bytes'] >= 350 * 1024 * 1024
        assert runtime['minimum_sampled_free_bytes'] >= 250 * 1024 * 1024
        assert 'clang version 20.' in runtime['clang_version']
        assert all(step['returncode'] == 0 and step['interrupted_reason'] is None for step in runtime['steps'])
        assert '-O2' in next(step['command'] for step in runtime['steps'] if step['name'] == 'link')
        assert [run['name'] for run in runtime['runs']] == ['normal', 'poison']
        for run in runtime['runs']:
            assert run['returncode'] == 0 and run['interrupted_reason'] is None and run['stdout'] == ''
            assert 'live=0 immortal=0 leaked=0' in run['stderr']
            env = run['managed_environment']
            assert env['CINCH_HEAP_LIMIT'] == ('32M' if run['name'] == 'normal' else '256M')
            assert env['CINCH_RC_DEBUG'] == ('0' if run['name'] == 'normal' else '1')
            assert env['CINCH_STACK_SIZE'] == '8M' and env['CINCH_LEAK_CHECK'] == '1'
        assert archive['originals_removed'] and archive['report_sha256'] == digest(evidence / row['runtime'])
        assert archive['compressed_bytes'] <= 10 * 1024 * 1024
        entries = {entry['member']: entry for entry in archive['entries']}
        assert set(entries) == {'consumer', 'consumer.ll'}
        assert entries['consumer']['sha256'] == runtime['executable_sha256']
        assert entries['consumer.ll']['sha256'] == runtime['llvm_sha256']
        assert row['groups'] > 0
        count += row['groups']
    assert not any(gates[key] for key in ['native_backend_qualified', 'whole_package_qualified',
                'production_adopted', 'runtime_cycle_reclamation_claim'])
    print(f'Verified{len(gates["controls"])} exact330/d91 runtime controls/{count} groups, two zero-leak profiles each; remaining gates stay open')


if __name__ == '__main__':
    main()
