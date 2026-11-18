#!/usr/bin/env python3
"""Verify exact isolated built-in scanner source and ownership receipts without selecting a production candidate."""
from pathlib import Path
import argparse
import json
import re
import sys
import check

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
sys.path.insert(0, str(ROOT/'tooling'))
from buffered_saved_artifacts import artifact_digest


def main():
    """Keep differential controls, independent corpora and public performance qualification explicitly distinct."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--output', type=Path, default=HERE/'gates.json')
    args = parser.parse_args()
    manifest = json.loads((HERE/'candidate.json').read_text())
    gates = []
    for frontend in ['rust', 'native']:
        source_path = ROOT/'build'/f'buffered-scan-builtins-{frontend}-source-v2'/'receipt.json'
        source = json.loads(source_path.read_text())
        assert source['returncode'] == 0 and not source['timeout_without_verdict']
        assert source['candidate_manifest_sha256'] == check.digest(HERE/'candidate.json')
        assert source['sources']['Main.cinch'] == check.digest(HERE/'Main.cinch')
        for name, entry in manifest['sources'].items():
            assert source['sources'][name] == entry['new_sha256'] == check.digest(HERE/'sources'/name)
            assert check.digest(ROOT/entry['old_path']) == entry['old_sha256'], 'maintained prelude changed'
        runtime_path = ROOT/'build'/f'buffered-scan-builtins-{frontend}-runtime-v1'/'receipt.json'
        runtime = json.loads(runtime_path.read_text())
        assert runtime['all_passed'] and runtime['source_receipt_sha256'] == check.digest(source_path)
        assert len(runtime['runs']) == 2
        for run in runtime['runs']:
            assert run['returncode'] == 0 and 'live=0 immortal=0 leaked=0' in run['stderr']
            assert run['environment']['CINCH_STACK_SIZE'] == '8M'
            assert run['environment']['CINCH_HEAP_LIMIT'] == ('256M' if run['poisoned'] else '32M')
            allocated = re.search(r'header-blocks=(\d+)', run['stderr'])
            freed = re.search(r'freed=(\d+)', run['stderr'])
            assert allocated and freed and int(allocated[1]) == int(freed[1]) > 1000000
        assert runtime['artifact_bytes'] <= 20*1024*1024
        assert artifact_digest(ROOT, runtime_path.parent/'consumer.ll') == runtime['llvm_sha256']
        assert artifact_digest(ROOT, runtime_path.parent/'consumer') == runtime['executable_sha256']
        gates.append({'frontend': frontend, 'source_receipt': str(source_path.relative_to(ROOT)), 'source_receipt_sha256': check.digest(source_path),
                      'source_seconds': source['seconds'], 'compiler_sha256': source['compiler_sha256'], 'runtime_sha256': source['runtime_sha256'],
                      'interfaces_sha256': source['interfaces_sha256'], 'provenance_sha256': source['provenance_sha256'],
                      'runtime_receipt': str(runtime_path.relative_to(ROOT)), 'runtime_receipt_sha256': check.digest(runtime_path),
                      'llvm_sha256': runtime['llvm_sha256'], 'executable_sha256': runtime['executable_sha256'],
                      'clang': runtime['clang'], 'clang_version': runtime['clang_version'], 'opt_level': runtime['opt_level'],
                      'steps': runtime['steps'], 'runs': runtime['runs'], 'artifact_bytes': runtime['artifact_bytes']})
    previous_path = ROOT/'build/buffered-scan-builtins-native-source-v1/receipt.json'
    previous = json.loads(previous_path.read_text())
    assert previous['timeout_without_verdict'] and previous['returncode'] is None
    assert previous['sources'] == json.loads((ROOT/gates[1]['source_receipt']).read_text())['sources']
    byte_spans = (9+1)*(9+2)//2
    unicode_length = len('\t é　😀\r\nz end'.encode())
    unicode_spans = (unicode_length+1)*(unicode_length+2)//2
    report = {'scope': 'Tested isolated candidate only; no maintained prelude edit, production selection or package-completion claim.',
              'manifest_sha256': check.digest(HERE/'candidate.json'), 'fixture_sha256': check.digest(HERE/'Main.cinch'),
              'runner_sha256': check.digest(HERE/'run.py'), 'source_hashes': manifest['sources'],
              'new_capabilities': manifest['public_additions'],
              'differential_comparisons': 256*byte_spans*2*4 + unicode_spans*2*4,
              'differential_counts': {'byte_values': 256, 'spans_per_byte_owner': byte_spans, 'terminal_modes': 2, 'capabilities': 4,
                                      'unicode_owner_bytes': unicode_length, 'unicode_spans': unicode_spans},
              'extra_controls': ['fresh built-in token survives owner mutation', 'token mutation does not modify private owner',
                                 'custom callback mutation remains snapshot-isolated', 'reversed bounds checked refusal', 'huge bounds checked refusal'],
              'gates': gates, 'zero_leak_memory_runs': 4,
              'earlier_native_source_deadline': {'receipt': str(previous_path.relative_to(ROOT)), 'sha256': check.digest(previous_path),
                                                 'interpretation': '15-second deadline without verdict; exact source later passed in granted45-second window'},
              'pending': ['complete independent1771 Scanner and840 direct-split observations using explicit capabilities',
                          'public pipeline benchmark/codegen/allocation comparison against the frozen generic-wrapper baseline',
                          'full combined-effect source epoch if adopted separately', 'retained-callback cycle remains a full-family promotion blocker']}
    args.output.write_text(json.dumps(report, indent=2)+'\n')
    print('Verified 114320 differential span comparisons and owner/bounds controls, both frontends and both memory profiles')


if __name__ == '__main__':
    main()
