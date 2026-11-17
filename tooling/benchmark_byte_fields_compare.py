#!/usr/bin/env python3
"""Prepare or run a bounded paired field-pipeline comparison using previously qualified unchanged artifacts."""
from __future__ import annotations
import argparse
import json
import os
from pathlib import Path
import statistics
import subprocess
import time

from reference_byte_fields_benchmark import expected
from validate_byte_view import digest

ROOT = Path(__file__).resolve().parents[1]
CONDITIONS = [
    ('eager', 'text', 4096), ('eager', 'dense', 4096), ('eager', 'unicode', 4096),
    ('eager', 'text', 65536), ('eager', 'space', 65536), ('eager', 'dense', 65536),
    ('eager', 'unicode', 65536), ('producer', 'text', 65536), ('producer', 'dense', 65536),
    ('visit', 'text', 65536), ('visit', 'dense', 65536),
    ('predicate', 'text', 65536), ('effect', 'unicode', 65536),
]


def quota() -> dict:
    """Capture the actual cgroup quota, throttling counters and inherited CPU affinity."""
    result = dict(affinity=sorted(os.sched_getaffinity(0)))
    for file in ['cpu.max', 'cpu.stat', 'cpuset.cpus.effective']:
        p = Path('/sys/fs/cgroup')/file
        result[file] = p.read_text() if p.is_file() else None
    return result


def prepare(reassociated: bool = False, custom_preflights: tuple[Path, Path] | None = None) -> dict:
    """Freeze exact already-green binaries and independent observations before obtaining a timing lease."""
    variants = []
    names = (['rust706-simd', 'rust706-reassociated'] if reassociated else
             ['rust706-simd', 'rust706-scalar', 'native-v4-simd', 'native-v4-scalar', 'reference'])
    selections = [(name, ROOT/'build'/('byte-fields-bench-'+name+'.json')) for name in names]
    if custom_preflights:
        assert reassociated
        selections = list(zip(['rust-projector-baseline', 'rust-projector-reassociated'],
                              [path.resolve() for path in custom_preflights]))
    for name, receipt in selections:
        directory = receipt.with_suffix('')
        r = json.loads(receipt.read_text())
        assert r['status'] == 'passed'
        executable = directory/('reference' if name == 'reference' else 'consumer')
        assert digest(executable) == r['executable_sha256']
        if name != 'reference':
            assert r['compiled_rc_debug'] is False and len(r['profiles']) == 650
            assert all(p['zero_leaks'] for p in r['profiles'])
        else:
            assert len(r['profiles']) == 325
        variants.append(dict(name=name, executable=str(executable), executable_sha256=digest(executable),
                             preflight_receipt=str(receipt), preflight_sha256=digest(receipt),
                             compiler_sha256=r.get('compiler_sha256'), runtime_sha256=r.get('runtime_sha256'),
                             entry_sha256=r.get('entry_sha256'), llvm_sha256=r.get('llvm_sha256'),
                             dependencies=r.get('dependencies')))
    if reassociated:
        baseline, candidate = variants
        for key in ['compiler_sha256', 'runtime_sha256', 'entry_sha256']:
            assert baseline[key] == candidate[key], key
        old = baseline['dependencies']
        new = candidate['dependencies']
        assert old.keys() == new.keys()
        assert [name for name in old if old[name] != new[name]] == ['Data.Bytes.Utf8']
        source = (ROOT/'compiler/prelude/Data/Bytes/Utf8.cinch').read_text()
        experimental = ROOT/'benchmarks/stdlib/byte-fields-utf8-reassociated.cinch'
        before = 'hold-source source (packed-step (decode-raw base size offset))'
        after = 'packed-step (hold-source source (decode-raw base size offset))'
        assert source.count(before) == 1 and experimental.read_text() == source.replace(before, after)
        assert digest(ROOT/'compiler/prelude/Data/Bytes/Utf8.cinch') == old['Data.Bytes.Utf8']
        assert digest(experimental) == new['Data.Bytes.Utf8']
    conditions = [dict(kind=kind, pattern=pattern, size=size, iterations=4,
                       expected=expected(kind, pattern, size, 4)) for kind,pattern,size in CONDITIONS]
    return dict(scope=('Paired public fields baseline/reassociated UTF8 comparison; no production candidate selection'
                       if reassociated else 'Paired public fields SIMD/scalar/reference comparison; no production candidate selection'),
                variants=variants, conditions=conditions, samples=3, excluded_warmups=1,
                normal_heap='32M', program_stack='8M', compiled_rc_debug=False,
                reference_memory_model=(None if reassociated else 'standard tracing GC; no Cinch RC assertion'),
                timed_pipeline='Changed input, complete field consumption and ordinary reclamation; input preparation/retention hashing excluded',
                neutral_controls=[] if reassociated else ['predicate', 'effect'],
                generic_callback_controls=['predicate', 'effect'] if reassociated else [],
                generator_sha256=digest(Path(__file__)),
                raw_variation_must_be_retained=True)


def main() -> None:
    """Require a prepared immutable plan; retain every sample and stop before another process exceeds the lease."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--plan', type=Path, required=True)
    parser.add_argument('--prepare', action='store_true')
    parser.add_argument('--reassociated-control', action='store_true')
    parser.add_argument('--baseline-preflight', type=Path)
    parser.add_argument('--candidate-preflight', type=Path)
    parser.add_argument('--output', type=Path)
    parser.add_argument('--budget-seconds', type=float, default=30)
    args = parser.parse_args()
    assert bool(args.baseline_preflight) == bool(args.candidate_preflight)
    custom = ((args.baseline_preflight, args.candidate_preflight) if args.baseline_preflight else None)
    if args.prepare:
        args.plan.write_text(json.dumps(prepare(args.reassociated_control, custom), indent=2)+'\n')
        print(args.plan)
        return
    assert args.output and os.sched_getaffinity(0) == {0}, 'timed processes require the granted exclusive CPU0 affinity'
    plan = json.loads(args.plan.read_text())
    for variant in plan['variants']:
        assert digest(Path(variant['executable'])) == variant['executable_sha256']
        assert digest(Path(variant['preflight_receipt'])) == variant['preflight_sha256']
    result = dict(plan=plan, plan_sha256=digest(args.plan), before=quota(), rows=[], status='running',
                  background='Record the actual coordinated background work in the companion session receipt')
    def save() -> None:
        """Keep every raw paired observation before launching another candidate process."""
        args.output.write_text(json.dumps(result, indent=2)+'\n')
    save()
    start = time.monotonic()
    try:
        for number, condition in enumerate(plan['conditions']):
            for sample in range(-1, plan['samples']):
                variants = plan['variants']
                rotate = (number+sample+1) % len(variants)
                order = variants[rotate:]+variants[:rotate]
                if sample % 2:
                    order = list(reversed(order))
                for variant in order:
                    remaining = args.budget_seconds-(time.monotonic()-start)
                    if remaining < 0.5:
                        result['status'] = 'bounded-incomplete'
                        raise TimeoutError('stopped at sample boundary before exhausting the timing lease')
                    command = [variant['executable'], condition['kind'], condition['pattern'],
                               str(condition['size']), str(condition['iterations'])]
                    env = {**os.environ, 'GOMAXPROCS':'1', 'CINCH_RC_DEBUG':'0', 'CINCH_HEAP_LIMIT':'32M',
                           'CINCH_STACK_SIZE':'8M', 'CINCH_LEAK_CHECK':'1'}
                    run = subprocess.run(command, env=env, capture_output=True, text=True, timeout=remaining)
                    fields = run.stdout.split()
                    observed = (dict(checksum=int(fields[1]), original=fields[2], retained=fields[3], kept_size=int(fields[4]))
                                if len(fields) == 5 else None)
                    row = dict(condition=number, sample=sample, variant=variant['name'], command=command,
                               exit_code=run.returncode, stdout=run.stdout, stderr=run.stderr,
                               observed=observed, elapsed_ns=int(fields[0]) if len(fields) == 5 else None)
                    result['rows'].append(row)
                    save()
                    assert run.returncode == 0 and observed == condition['expected'], row
                    assert variant['name'] == 'reference' or 'leaked=0' in run.stderr
        result['status'] = 'passed'
    except Exception as error:
        if result['status'] != 'bounded-incomplete':
            result['status'] = 'failed'
        result['diagnostics'] = str(error)
    result.update(after=quota(), wall_seconds=time.monotonic()-start)
    summaries = []
    for number, condition in enumerate(plan['conditions']):
        medians = {}
        raw = {}
        for variant in plan['variants']:
            samples = [r['elapsed_ns']/condition['iterations'] for r in result['rows']
                       if r['condition'] == number and r['variant'] == variant['name'] and r['sample'] >= 0 and r['exit_code'] == 0]
            raw[variant['name']] = samples
            if len(samples) == plan['samples']:
                medians[variant['name']] = statistics.median(samples)
        ratios = {family:medians[family+'-scalar']/medians[family+'-simd'] for family in ['rust706','native-v4']
                  if family+'-scalar' in medians and family+'-simd' in medians and medians[family+'-simd'] > 0}
        summaries.append(dict(condition=condition, median_ns_per_operation=medians,
                              scalar_over_simd=ratios, raw_ns_per_operation=raw))
        for baseline, candidate in [('rust706-simd', 'rust706-reassociated'),
                                    ('rust-projector-baseline', 'rust-projector-reassociated')]:
            if baseline in medians and medians.get(candidate, 0) > 0:
                summaries[-1]['baseline_over_reassociated'] = medians[baseline]/medians[candidate]
    result['summaries'] = summaries
    save()
    print(json.dumps(dict(status=result['status'], wall_seconds=result['wall_seconds'], samples=len(result['rows']))))
    assert result['status'] == 'passed', result.get('diagnostics')


if __name__ == '__main__':
    main()
