#!/usr/bin/env python3
"""Verify the exact four-group transfer-priority runtime receipts without rebuilding their frozen clients."""
from pathlib import Path
import argparse
import json
import sys
import check

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
sys.path.insert(0, str(ROOT/'tooling'))
from buffered_saved_artifacts import artifact_digest


def main():
    """Keep source-only composition, executed transfer assertions and the unresolved retention contract separate."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--output', type=Path, default=HERE/'runtime-gates.json')
    args = parser.parse_args()
    gates = []
    for name in ['buffered-transfer-priority-rust-runtime-v2', 'buffered-transfer-priority-native-runtime-v1']:
        path = ROOT/'build'/name/'receipt.json'
        report = json.loads(path.read_text())
        assert report['all_passed'] and report['opt_level'] == 2
        source = report['source_receipt']
        assert source['signature_manifest_sha256'] == check.digest(HERE/'signature-impact.json')
        assert source['sources']['Main.cinch'] == check.digest(HERE/'TransferPriority.cinch')
        assert len(report['runs']) == 2 and sorted(run['poisoned'] for run in report['runs']) == [False, True]
        for run in report['runs']:
            assert run['returncode'] == 0 and 'live=0 immortal=0 leaked=0' in run['stderr']
            assert run['environment']['CINCH_STACK_SIZE'] == '8M'
            assert run['environment']['CINCH_HEAP_LIMIT'] == ('256M' if run['poisoned'] else '32M')
        assert all(step['returncode'] == 0 for step in report['steps'])
        assert artifact_digest(ROOT, path.parent/'consumer.ll') == report['llvm_sha256']
        assert artifact_digest(ROOT, path.parent/'consumer') == report['executable_sha256']
        assert report['artifact_bytes'] <= 20*1024*1024
        gates.append({'frontend': source['frontend'], 'receipt': str(path.relative_to(ROOT)), 'receipt_sha256': check.digest(path),
                      'compiler_sha256': source['compiler_sha256'], 'runtime_sha256': source['runtime_sha256'],
                      'provenance_sha256': source['provenance_sha256'], 'interfaces_sha256': source['interfaces_sha256'],
                      'source_receipt_sha256': report['source_receipt_sha256'], 'llvm_sha256': report['llvm_sha256'],
                      'executable_sha256': report['executable_sha256'], 'clang': report['clang'], 'clang_version': report['clang_version'],
                      'steps': report['steps'], 'runs': report['runs'], 'artifact_bytes': report['artifact_bytes']})
    result = {'scope': 'Four exact transfer-priority and advanced-writer groups executed on both frozen frontends; no complete-library or API promotion.',
              'runner_sha256': check.digest(HERE/'run.py'), 'fixture_sha256': check.digest(HERE/'TransferPriority.cinch'),
              'signature_manifest_sha256': check.digest(HERE/'signature-impact.json'),
              'groups': ['source WriterTo precedes sink ReaderFrom', 'sink ReaderFrom fallback when source WriterTo is absent',
                         'empty writer delegates and retains the returned sink/capability drop', 'pending prefix fills/flushes before delegation at the advanced sink; count excludes old prefix'],
              'gates': gates, 'zero_leak_memory_runs': 4,
              'remaining': ['actual live Multipart Part/File shared aliases, prefix/error and caller-owned close runtime cases',
                            'full combined-effect family functional corpus and owner/retained callback gates',
                            'the original retained callback cross-cycle remains a promotion blocker; no lifetime or collection policy changes']}
    args.output.write_text(json.dumps(result, indent=2)+'\n')
    print('Verified four groups on two frontends / four fixed-memory runs with zero live/leaks')


if __name__ == '__main__':
    main()
