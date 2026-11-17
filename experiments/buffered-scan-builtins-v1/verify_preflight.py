#!/usr/bin/env python3
"""Verify the candidate's complete bounded public pipeline preflight under the exact frozen baseline compiler epoch."""
from pathlib import Path
import argparse
import json
import check

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]


def rows(path):
    """Require complete reference agreement and zero-leak normal/poison outcomes for each distinct public condition."""
    report = json.loads(path.read_text())
    found = {}
    for row in report['results']:
        key = tuple(row[name] for name in ['workload', 'pattern', 'size', 'iterations'])
        assert key not in found
        assert sorted(run['poisoned'] for run in row['runs'] if run['label'] == 'cinch') == [False, True]
        for run in row['runs']:
            assert run['exit_code'] == 0 and int(run['stdout'].split()[1]) == row['checksum']
            if run['label'] == 'cinch':
                assert 'live=0 immortal=0 leaked=0' in run['stderr']
        found[key] = row['checksum']
    return report, found


def main():
    """Keep the historical bounded epoch, exact source differences and unfinished selection gates reviewable."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--output', type=Path, default=HERE/'preflight-gates.json')
    args = parser.parse_args()
    baseline_dir = ROOT/'build/buffered-benchmark-rust-v1'
    candidate_dir = ROOT/'build/buffered-benchmark-scan-candidate-7e-v1'
    baseline = json.loads((baseline_dir/'identity.json').read_text())
    candidate = json.loads((candidate_dir/'identity.json').read_text())
    for identity, directory in [(baseline, baseline_dir), (candidate, candidate_dir)]:
        for field, name in [('executable_sha256', 'benchmark'), ('llvm_sha256', 'benchmark.ll'), ('assembly_sha256', 'benchmark.s'),
                            ('reference_executable_sha256', 'reference'), ('reference_source_sha256', 'reference.go')]:
            assert check.digest(directory/name) == identity[field]
        for name, digest in identity['sources'].items():
            assert check.digest(directory/'project'/name) == digest
        assert identity['rc_debug_instrumentation'] is False and identity['ordinary_reclaim'] is True
        assert identity['reference_sdk'] == 'go version go1.27.4 linux/amd64'
    assert baseline['compiler_sha256'] == candidate['compiler_sha256'] == '7e0d98d14cb0213d4deeb277e1ad65fce07784309716085e70e4b9db2c259102'
    assert baseline['runtime_sha256'] == candidate['runtime_sha256'] == 'c4b1097b4b8e26541400228bdbbdcba1376101014823aeb2f5af619fbede57ac'
    assert baseline['reference_source_sha256'] == candidate['reference_source_sha256']
    changed = sorted(name for name, digest in candidate['sources'].items() if baseline['sources'].get(name) != digest)
    assert changed == ['Effect.Scanner.Split.cinch', 'Effect.Scanner.cinch', 'Main.cinch']
    expected_main = (baseline_dir/'project/Main.cinch').read_text()
    for name, capability in [('runes', 'rune'), ('words', 'word')]:
        expected_main = expected_main.replace('scan-split scan-'+name, 'scan-'+capability+'-split')
    assert (candidate_dir/'project/Main.cinch').read_text() == expected_main
    manifest = json.loads((HERE/'candidate.json').read_text())
    for name, entry in manifest['sources'].items():
        assert baseline['sources'][name] == entry['old_sha256'] and candidate['sources'][name] == entry['new_sha256']
    previous = {}
    input_paths = [ROOT/'build/buffered-benchmark-preflight-v1.json', ROOT/'build/buffered-benchmark-preflight-fringes-v1.json']
    for path in input_paths:
        report, observed = rows(path)
        assert report['executable_sha256'] == baseline['executable_sha256'] and not (previous.keys() & observed.keys())
        previous.update(observed)
    preflight_path = ROOT/'build/buffered-benchmark-scan-candidate-preflight-v1.json'
    preflight, observed = rows(preflight_path)
    assert preflight['executable_sha256'] == candidate['executable_sha256']
    assert len(previous) == len(observed) == 729 and observed == previous
    artifact_bytes = sum(path.stat().st_size for path in candidate_dir.rglob('*') if path.is_file()) + preflight_path.stat().st_size
    assert artifact_bytes <= 25*1024*1024
    result = {'scope': 'Complete bounded public pipeline preflight only; no timing, optimization selection or general compiler repair claim.',
              'baseline_identity_sha256': check.digest(baseline_dir/'identity.json'), 'candidate_identity_sha256': check.digest(candidate_dir/'identity.json'),
              'compiler_sha256': candidate['compiler_sha256'], 'runtime_sha256': candidate['runtime_sha256'],
              'candidate_identity': candidate, 'changed_modules': changed,
              'inputs': {str(path.relative_to(ROOT)): check.digest(path) for path in input_paths + [preflight_path]},
              'conditions': 729, 'zero_leak_cinch_memory_runs': 1458, 'independent_reference_runs': 729,
              'candidate_artifact_and_preflight_bytes': artifact_bytes,
              'data_scope': 'The same nine public pipelines, three byte patterns, nine sizes0..65536 and iteration counts0/1/3; two alternating prepared owners and U32 counters.',
              'known_epoch_limitation': 'Historical7e0d has a known boxed-Nat comparison memory-attribute bug. This explicit bounded comparison does not claim a general boxed-Nat boundary repair;855/2f functional evidence is separate.',
              'pending': ['complete1771/840 independent scanner/direct-split corpus runtime gates', 'explicitly granted paired baseline/candidate timing and codegen/allocation review',
                          'actual live Multipart owner/error/close controls', 'callback cycle still blocks full-family promotion']}
    args.output.write_text(json.dumps(result, indent=2)+'\n')
    print('Verified same-epoch candidate729 conditions/1458 zero-leak memory runs and exact three-module source difference')


if __name__ == '__main__':
    main()
