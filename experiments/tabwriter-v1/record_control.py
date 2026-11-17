#!/usr/bin/env python3
"""Capture compact completed-success tabwriter control receipts without executing or modifying any source epoch."""
from pathlib import Path
import argparse
import hashlib
import json

HERE = Path(__file__).resolve().parent


def digest(path):
    """Bind complete original content rather than infer source or tool identity from a directory name."""
    return hashlib.sha256(path.read_bytes()).hexdigest()


def retain(source, target):
    """Preserve exact receipt bytes and refuse to overwrite any independently recorded differing epoch."""
    body = source.read_bytes()
    if target.exists():
        assert target.read_bytes() == body, target
    else:
        target.write_bytes(body)


def main():
    """Record one verified closed success under an aggregate whose other pending controls remain explicitly open."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--name', required=True)
    parser.add_argument('--entry', type=Path, required=True)
    parser.add_argument('--source', type=Path, required=True)
    parser.add_argument('--runtime', type=Path, required=True)
    parser.add_argument('--groups', type=int, required=True)
    args = parser.parse_args()
    assert args.name in ['error-owner', 'offer-owner', 'layers', 'stream', 'kernel']
    source_report = args.source / 'report.json'
    runtime_report = args.runtime / 'report.json'
    source = json.loads(source_report.read_text())
    runtime = json.loads(runtime_report.read_text())
    index_path = args.runtime / 'closed-success-index.json'
    archive = json.loads(index_path.read_text())
    assert source['source_gate_passed'] and source['main_sha256'] == digest(args.entry)
    assert runtime['all_passed'] and runtime['source_report_sha256'] == digest(source_report)
    assert archive['originals_removed'] and archive['report_sha256'] == digest(runtime_report)
    assert runtime['runner_sha256'] == digest(HERE / 'run.py')
    assert args.groups > 0
    evidence = HERE / 'controls-v1'
    source_copy = evidence / (args.name + '-source-v1')
    runtime_copy = evidence / (args.name + '-runtime-v1')
    source_copy.mkdir(exist_ok=True)
    runtime_copy.mkdir(exist_ok=True)
    retain(args.source / 'Main.cinch', source_copy / 'Main.cinch')
    retain(source_report, source_copy / 'report.json')
    retain(runtime_report, runtime_copy / 'report.json')
    retain(index_path, runtime_copy / 'closed-success-index.json')
    retain(HERE / 'run.py', runtime_copy / 'runner.py')
    gate_path = evidence / 'runtime-gates.json'
    if gate_path.exists():
        gates = json.loads(gate_path.read_text())
    else:
        gates = {'scope': __doc__, 'controls': [], 'native_backend_qualified': False,
                 'whole_package_qualified': False, 'production_adopted': False,
                 'runtime_cycle_reclamation_claim': False}
    assert all(row['name'] != args.name for row in gates['controls']), 'never replace a prior qualified control'
    files = [source_copy / 'Main.cinch', source_copy / 'report.json', runtime_copy / 'report.json',
             runtime_copy / 'closed-success-index.json', runtime_copy / 'runner.py']
    gates['controls'].append({'name': args.name, 'groups': args.groups,
        'entry': str(args.entry.resolve().relative_to(HERE)), 'entry_sha256': digest(args.entry),
        'source': str((source_copy / 'report.json').relative_to(evidence)),
        'runtime': str((runtime_copy / 'report.json').relative_to(evidence)),
        'archive': str((runtime_copy / 'closed-success-index.json').relative_to(evidence)),
        'runner': str((runtime_copy / 'runner.py').relative_to(evidence)),
        'files': {str(path.relative_to(evidence)): digest(path) for path in files},
        'candidate_source_identities': {name: row['sha256'] for name, row in source['sources'].items()
             if (HERE / 'sources' / (name + '.cinch')).exists()}})
    gate_path.write_text(json.dumps(gates, indent=2) + '\n')
    print('Recorded compact closed-success control without source/compiler/runtime execution:', args.name)


if __name__ == '__main__':
    main()
