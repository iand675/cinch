#!/usr/bin/env python3
"""Verify source-bound literal tabwriter qualification without requiring an installed reference SDK."""
from pathlib import Path
import argparse
import hashlib
import json

HERE = Path(__file__).resolve().parent


def digest(path):
    """Identify maintained source and exact copied evidence without relabeling a compiler, provider or runtime."""
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    """Require complete pure coverage, the declared streaming subset and genuine Region/private-storage contracts."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--evidence', type=Path, required=True)
    args = parser.parse_args()
    evidence = args.evidence
    gates = json.loads((evidence / 'gates.json').read_text())
    for name, expected in gates['files'].items():
        assert digest(evidence / name) == expected, name
    for name, expected in gates['sources'].items():
        assert digest(HERE / 'sources' / (name + '.cinch')) == expected, name
    compiler, runtime = gates['compiler_sha256'], gates['runtime_sha256']
    pure_ids, protocol_ids = [], []
    for kind in ('pure', 'protocol'):
        for row in gates[kind]:
            source = json.loads((evidence / row['source']).read_text())
            actual = json.loads((evidence / row['runtime']).read_text())
            index = json.loads((evidence / row['archive_index']).read_text())
            assert source['source_gate_passed'] and actual['all_passed']
            assert source['compiler_sha256'] == actual['compiler_sha256'] == compiler
            assert source['runtime_sha256'] == actual['runtime_sha256'] == runtime
            assert actual['source_report_sha256'] == digest(evidence / row['source'])
            assert index['report_sha256'] == digest(evidence / row['runtime']) and index['originals_removed']
            assert len(actual['runs']) == 2 and [run['name'] for run in actual['runs']] == ['normal', 'poison']
            assert all(run['returncode'] == 0 and run['stdout'] == '' and 'live=0 immortal=0 leaked=0' in run['stderr'] for run in actual['runs'])
            assert actual['complete_artifact_bytes'] <= 20 * 1024 * 1024
            if kind == 'pure':
                pure_ids.extend(item['reference_id'] for item in row['cases'])
            else:
                protocol_ids.extend(row['cases'])
    assert len(pure_ids) == len(set(pure_ids)) == gates['pure_cases'] == 549
    assert len(protocol_ids) == len(set(protocol_ids)) == gates['ordinary_protocol_cases']
    assert not set(pure_ids).intersection(protocol_ids)
    manifest = json.loads((evidence / 'protocol-consumers.json').read_text())
    assert len(manifest['native_adaptation_cases']) == gates['native_adaptations_total'] == 12
    assert len(pure_ids) + manifest['ordinary_streaming_count'] + manifest['native_adaptation_count'] == 612
    public = json.loads((evidence / 'proof/Effect.Tabwriter.cinchif').read_text())
    exported = {row['name']: row for row in public['exports']['types']}
    for name in ('TabOutput', 'TabWriter'):
        assert exported[name]['parameters'] == [{'implicit': False, 'kind': 'Region', 'name': 'r'},
                                                 {'implicit': False, 'kind': 'Type', 'name': 'e'}]
    values = {row['name']: row for row in public['exports']['values']}
    assert 'Writer' not in values and 'WriterState' not in values and 'Output' not in values and 'MutableOutput' not in values
    for name in ('tab-writer-write', 'tab-writer-flush', 'tab-output-write'):
        assert '{IO r, Mut r}' in values[name]['scheme']
    assert '{IO r}' in values['tab-writer-init']['scheme'] and '{IO r, Mut r}' not in values['tab-writer-init']['scheme']
    cell = json.loads((evidence / 'proof/Effect.Tabwriter.Internal.Cell.cinchif').read_text())
    assert {row['name'] for row in cell['exports']['values']} == {'cell-new', 'cell-read', 'cell-write'}
    assert not any(gates[key] for key in ('native_qualified', 'portable_qualified', 'full_owner_controls_qualified', 'performance_reviewed', 'production_adopted'))
    print(f'Verified549 literal complete-input cases +{len(protocol_ids)} streaming cases, genuine Region contracts and all remaining gate exclusions')


if __name__ == '__main__':
    main()
