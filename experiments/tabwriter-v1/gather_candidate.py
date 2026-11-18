#!/usr/bin/env python3
"""Gather only actual source/runtime receipts into a compact SDK-free partial tabwriter checkpoint."""
from pathlib import Path
import argparse
import json
import shutil
import check


def main():
    """Preserve all549 pure cases and the currently completed streaming batches without inventing remaining gates."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--protocol-batches', type=int, required=True)
    args = parser.parse_args()
    assert not args.output.exists(), 'never overwrite a checkpoint evidence epoch'
    assert 1 <= args.protocol_batches <= 17
    output = args.output.resolve()
    output.mkdir(parents=True)
    proof = output / 'proof'
    proof.mkdir()
    pure = check.ROOT / 'build/tabwriter-format-full-330-v1'
    consumers = check.ROOT / 'build/tabwriter-format-consumers-v3'
    manifest = json.loads((consumers / 'manifest.json').read_text())
    assert manifest['selected_count'] == 549 and len(manifest['batches']) == 23
    report = {'scope': __doc__, 'compiler_sha256': check.COMPILER, 'runtime_sha256': check.RUNTIME,
              'compiler_source': check.SOURCE, 'pure_cases': 549, 'ordinary_protocol_cases': args.protocol_batches * 3,
              'ordinary_protocol_total': 51, 'native_adaptations_total': 12, 'pure': [], 'protocol': [],
              'sources': {}, 'files': {}, 'native_qualified': False, 'portable_qualified': False,
              'full_owner_controls_qualified': False, 'performance_reviewed': False, 'production_adopted': False}

    def preserve(path, target):
        """Keep exact reports/indexes/consumer manifests plain and bind every copied byte in the checkpoint."""
        destination = output / target
        destination.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(path, destination)
        report['files'][target] = check.digest(destination)
        return target

    def unit(source, runtime, prefix):
        """Accept only source-positive paired zero-leak runs with completed verified success-only archives."""
        source_report = json.loads((source / 'report.json').read_text())
        runtime_report = json.loads((runtime / 'report.json').read_text())
        assert source_report['source_gate_passed'] and runtime_report['all_passed']
        assert source_report['compiler_sha256'] == runtime_report['compiler_sha256'] == check.COMPILER
        assert source_report['runtime_sha256'] == runtime_report['runtime_sha256'] == check.RUNTIME
        assert runtime_report['source_report_sha256'] == check.digest(source / 'report.json')
        assert len(runtime_report['runs']) == 2
        assert all(row['returncode'] == 0 and row['stdout'] == '' and 'live=0 immortal=0 leaked=0' in row['stderr'] for row in runtime_report['runs'])
        index = json.loads((runtime / 'closed-success-index.json').read_text())
        assert index['originals_removed'] and index['report_sha256'] == check.digest(runtime / 'report.json')
        for name, identity in source_report['sources'].items():
            if name.startswith(('Text.Tabwriter', 'Effect.Tabwriter', 'Fixture.Tabwriter')):
                assert check.digest(check.HERE / 'sources' / (name + '.cinch')) == identity['sha256']
                if name in report['sources']:
                    assert report['sources'][name] == identity['sha256']
                report['sources'][name] = identity['sha256']
        return {'source': preserve(source / 'report.json', f'proof/{prefix}-source.json'),
                'runtime': preserve(runtime / 'report.json', f'proof/{prefix}-runtime.json'),
                'archive_index': preserve(runtime / 'closed-success-index.json', f'proof/{prefix}-archive.json')}

    preserve(consumers / 'manifest.json', 'pure-consumers.json')
    for index, batch in enumerate(manifest['batches']):
        source = pure / f'tabwriter-format-{index:03d}-source'
        runtime = pure / f'tabwriter-format-{index:03d}-runtime'
        row = unit(source, runtime, f'pure-{index:03d}')
        row['cases'] = batch['cases']
        report['pure'].append(row)
    protocol = check.ROOT / 'build/tabwriter-protocol-consumers-v1'
    protocol_manifest = json.loads((protocol / 'manifest.json').read_text())
    preserve(protocol / 'manifest.json', 'protocol-consumers.json')
    for index in range(args.protocol_batches):
        if index < 2:
            source = check.ROOT / f'build/tabwriter-protocol-{index:03d}-source-v1'
            runtime = check.ROOT / f'build/tabwriter-protocol-{index:03d}-runtime-v1'
        else:
            source = check.ROOT / f'build/tabwriter-protocol-full-330-v1/tabwriter-protocol-{index:03d}-source'
            runtime = check.ROOT / f'build/tabwriter-protocol-full-330-v1/tabwriter-protocol-{index:03d}-runtime'
        row = unit(source, runtime, f'protocol-{index:03d}')
        row['cases'] = protocol_manifest['batches'][index]['cases']
        report['protocol'].append(row)
    interface_root = check.ROOT / 'build/tabwriter-writer-source-v1/build'
    for name in ('Effect.Tabwriter', 'Effect.Tabwriter.Internal.Cell'):
        preserve(interface_root / (name + '.cinchif'), 'proof/' + name + '.cinchif')
    report['sources_present_but_not_qualified_here'] = [p.name for p in (check.HERE / 'sources').glob('*.cinch') if p.name[:-6] not in report['sources']]
    (output / 'gates.json').write_text(json.dumps(report, indent=2) + '\n')
    print(f'Gathered549 pure cases +{args.protocol_batches * 3} ordinary streaming cases; remaining gates explicitly open')


if __name__ == '__main__':
    main()
