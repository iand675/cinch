#!/usr/bin/env python3
"""Verify paired scanner timing, preserve every regression/control sample and separate observations from causal claims."""
from pathlib import Path
import argparse
import json
import statistics
import check

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]


def counters(text):
    """Parse the cgroup evidence without adjusting measured samples using scheduler counters."""
    return {name: int(value) for name, value in (line.split() for line in text.splitlines())}


def main():
    """Require the complete same-epoch paired matrix and retain noisy controls and the small raw-byte word regression."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--input', type=Path, default=ROOT/'build/buffered-scan-builtins-pair-cpu2-v1.json')
    parser.add_argument('--output', type=Path, default=HERE/'paired-gates.json')
    parser.add_argument('--conditions', type=int, default=54)
    parser.add_argument('--samples', type=int, default=3)
    args = parser.parse_args()
    args.input = args.input.resolve()
    raw = json.loads(args.input.read_text())
    assert len(raw['results']) == args.conditions and raw['samples'] == args.samples and raw['cpu'] == 2
    assert raw['preflight_gates_sha256'] == check.digest(HERE/'preflight-gates.json')
    identities = raw['identities']
    assert identities['baseline']['compiler_sha256'] == identities['candidate']['compiler_sha256']
    assert identities['baseline']['runtime_sha256'] == identities['candidate']['runtime_sha256']
    assert all(not item['rc_debug_instrumentation'] and item['ordinary_reclaim'] for item in identities.values())
    rows, runs = [], 0
    seen = set()
    for row in raw['results']:
        key = (row['workload'], row['size'], row['pattern'])
        assert key not in seen
        seen.add(key)
        assert len(row['pairs']) == args.samples+1 and [pair['warmup'] for pair in row['pairs']] == [True]+[False]*args.samples
        baseline_first = sum(pair['order'][0] == 'baseline' for pair in row['pairs'] if not pair['warmup'])
        assert abs(baseline_first-(args.samples-baseline_first)) <= 1
        ratios = []
        for pair in row['pairs']:
            assert [run['variant'] for run in pair['runs']] == pair['order'] and set(pair['order']) == {'baseline', 'candidate'}
            times = {}
            for run in pair['runs']:
                assert run['returncode'] == 0 and 'live=0 immortal=0 leaked=0' in run['stderr']
                elapsed, checksum = map(int, run['stdout'].split())
                assert elapsed >= 0 and checksum == row['checksum']
                times[run['variant']] = elapsed
                runs += 1
            if not pair['warmup']:
                ratios.append(times['candidate']/times['baseline'])
        rows.append({**row, 'paired_candidate_over_baseline': ratios, 'median_paired_ratio': statistics.median(ratios),
                     'paired_ratio_range': [min(ratios), max(ratios)]})
    assert runs == args.conditions*(args.samples+1)*2
    before, after = counters(raw['cpu_stat_start']), counters(raw['cpu_stat_end'])
    delta = {key: after[key]-before[key] for key in before}
    regressions = [row for row in rows if row['workload'].startswith('scan-') and row['median_paired_ratio'] > 1]
    report = {'scope': 'Observed paired historical same-epoch scanner timings; no production selection or causal uncontended-speedup claim.',
              'input_sha256': check.digest(args.input), 'input': str(args.input.relative_to(ROOT)),
              'runner_sha256': raw['runner_sha256'], 'cpu': raw['cpu'], 'wall_seconds': raw['wall_seconds'],
              'compiler_sha256': identities['candidate']['compiler_sha256'], 'runtime_sha256': identities['candidate']['runtime_sha256'],
              'preflight_gates_sha256': raw['preflight_gates_sha256'], 'conditions': args.conditions, 'zero_leak_cinch_runs': runs,
              'independent_reference_checksum_conditions': args.conditions, 'background': raw['background'], 'cpu_stat_delta': delta,
              'results': rows, 'scanner_regressions': regressions,
              'limits': ['All samples per condition retain their full raw ranges. Median-of-paired ratios and ratio-of-medians are both preserved.',
                         'Each condition has balanced nonwarmup order counts.' if args.samples%2==0 else 'Nonwarmup order counts differ by one; use the balanced focused confirmation when evaluating regression/noise.',
                         'Unmodified controls drift, including mixed64KiB rune and fresh-copy cases. Unchanged cgroup throttling counters do not prove absent host/cache/frequency interference.',
                         'Historical7e0d boxed-Nat memory-attribute limitation is not repaired by this bounded comparison.',
                         'Complete1771/840 independent corpus runtime gates, real live adapter execution/disk-close and callback-cycle promotion hold remain open.']}
    args.output.write_text(json.dumps(report, indent=2)+'\n')
    print('Verified '+str(args.conditions)+' paired conditions/'+str(runs)+' zero-leak runs; retained all raw controls and scanner ratios')


if __name__ == '__main__':
    main()
