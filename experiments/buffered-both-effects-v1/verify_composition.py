#!/usr/bin/env python3
"""Verify the concrete live-source composition and refusal receipts without rerunning a compiler."""
from pathlib import Path
import argparse
import json
import check

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]


def main():
    """Tie positive and deliberate negative source gates to the same exact experimental signature manifest."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--build', type=Path, default=ROOT/'build')
    parser.add_argument('--output', type=Path, default=HERE/'composition-gates.json')
    args = parser.parse_args()
    entries = []
    for frontend in ['rust', 'native']:
        for name in ['legacy-source', 'reader-io-only', 'reader-from-io-only', 'part-io-only']:
            entries.append((f'buffered-refusal-{name}-{frontend}-v1', True, HERE/'refusals'/(name+'.cinch')))
        entries.append((f'buffered-multipart-adapter-{frontend}-source-v1', False, HERE/'AdapterCheck.cinch'))
        priority = 'buffered-transfer-priority-rust-source-v1' if frontend == 'rust' else 'buffered-transfer-priority-native-v1'
        entries.append((priority, False, HERE/'TransferPriority.cinch'))
    gates = []
    manifest = check.digest(HERE/'signature-impact.json')
    for name, refusal, source in entries:
        path = args.build/name/'receipt.json'
        receipt = json.loads(path.read_text())
        assert receipt['signature_manifest_sha256'] == manifest
        assert receipt['sources']['Main.cinch'] == check.digest(source)
        assert receipt['expected_effect_refusal'] == refusal
        for module in (HERE/'sources').glob('*.cinch'):
            if module.name in receipt['sources']:
                assert receipt['sources'][module.name] == check.digest(module)
        if refusal:
            assert receipt['returncode'] != 0 and check.effect_refusal(receipt['diagnostic'])
        else:
            assert receipt['returncode'] == 0
        gates.append({'name': name, 'source': str(source.relative_to(ROOT)), 'source_sha256': check.digest(source),
                      'frontend': receipt['frontend'], 'outcome': 'intended effect-row refusal' if refusal else 'source/type/proof pass',
                      'receipt': str(path.relative_to(ROOT)), 'receipt_sha256': check.digest(path),
                      'command': receipt['command'], 'compiler_sha256': receipt['compiler_sha256'],
                      'runtime_sha256': receipt['runtime_sha256'], 'interfaces_sha256': receipt['interfaces_sha256'],
                      'provenance_sha256': receipt['provenance_sha256'], 'sources': receipt['sources'],
                      'compiler_returncode': receipt['returncode'], 'seconds': receipt['seconds'],
                      'diagnostic': receipt['diagnostic']})
    report = {'scope': 'Review-only source/type/proof and deliberate effect-refusal gates; no client execution or API promotion.',
              'signature_manifest_sha256': manifest, 'existing_public_signature_changes': 18, 'new_factories': ['buffer-source-with-mutation'],
              'factory_source_sha256': check.digest(HERE/'sources/Effect.Buffered.Core.cinch'),
              'adapter_source_sha256': check.digest(HERE/'MultipartAdapter.cinch'),
              'gates': gates,
              'notes': ['The Native legacy-source compiler verdict was correct; its original harness rejected the diagnostic wording. The original receipt/log are retained and this verifier checks the exact IO-versus-IO+Mut mismatch without a compiler rerun.',
                        'Native multipart dependencies use the admitted 2f/V2 tuple, including interfaces229ee and object manifest36d5; this is separate from the old 1d22 source-only family gate.',
                        'ReaderFrom priority, full/empty-buffer delegation and advanced capability-drop assertions are typechecked fixture bodies only; their runtime outcomes remain pending.',
                        'The live adapters retain the actual native part/file, preserve prefix-plus-status and caller-owned close responsibility; no eager snapshot or effect erasure is introduced.',
                        'Complete effect/owner/runtime/corpus gates and the separately recorded retained-callback cycle must clear before promotion.']}
    args.output.write_text(json.dumps(report, indent=2)+'\n')
    print('Verified twelve exact source gates: four positives and eight intended effect refusals')


if __name__ == '__main__':
    main()
