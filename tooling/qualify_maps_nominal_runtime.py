#!/usr/bin/env python3
"""Qualify one bounded annotated-entry map runtime unit against the exact Role+Slots tuple."""
from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil

from generate_maps_cases import edge_source, float_source, source
from qualify_maps_nominal_source import COMPILER_SHA, FROZEN, ROOT, RUNTIME_SHA, digest
from validate_maps import execute


ENTRY = '''-- | Run the independent edge, IEEE64 and mixed-effect fixtures in one scoped runtime unit.
module Main where
  import Data.Core
  import Data.Nat
  import Relation.Eq ((==))
  import MapProbe.Edge as Edge
  import MapProbe.IEEE64 as IEEE
  import MapProbe.Mixed as Mixed

  -- | Preserve each original failure index in a distinct group without changing its assertions.
  main :: forall (r :: Region). Unit ->{Mut r, IO r} Int
  main unit =
    let edge = Edge.main () in
    if edge == 0 then
      let ieee = IEEE.main () in
      if ieee == 0 then
        let mixed = Mixed.main () in
        if mixed == 0 then 0 else 300 + mixed
      else 200 + ieee
    else 100 + edge
'''


def export_probe(source: str, module: str) -> str:
    """Expose an unchanged consumer entry under a unique test module identity."""
    assert source.count('module Main where\n') == 1
    result = source.replace('module Main where\n', f'module {module} where\n  export main\n', 1)
    assert result.replace(f'module {module} where\n  export main\n', 'module Main where\n', 1) == source
    return result


def save(path: Path, report: dict) -> None:
    """Preserve the first genuine source, link or runtime failure and all exact input identities."""
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(report, indent=2) + '\n')


def output_bytes(directory: Path) -> int:
    """Count every new functional artifact before beginning another phase."""
    return sum(path.stat().st_size for path in directory.rglob('*') if path.is_file())


def main() -> int:
    """Run a single qualified runtime fixture after the unmodified350MiB start guard passes."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--cpu', type=int, required=True)
    parser.add_argument('--unit', choices=('combined', 'control', 'trace'), default='combined')
    parser.add_argument('--case', type=int, choices=range(8), default=0)
    parser.add_argument('--maximum-new-output-mib', type=int, choices=(20, 30), default=20)
    parser.add_argument('--artifacts', type=Path, default=ROOT / 'build/maps-role-slots-runtime-a1')
    parser.add_argument('--output', type=Path, default=ROOT / 'build/maps-role-slots-runtime-a1-results.json')
    args = parser.parse_args()
    os.sched_setaffinity(0, {args.cpu})
    compiler, runtime = FROZEN / 'cinch', FROZEN / 'libcinchruntime.a'
    clang = Path('/workspace/.tools/llvm/usr/lib/llvm-20/bin/clang')
    assert digest(compiler) == COMPILER_SHA and digest(runtime) == RUNTIME_SHA
    assert os.environ.get('CINCH_CLANG') == str(clang)
    source_receipt = ROOT / 'bootstrap/evidence/stdlib/stdlib-maps-role-slots-source-qualification-results.json'
    previous = json.loads(source_receipt.read_text())
    assert previous['status'] == 'source-and-public-defining-headers-passed'
    assert previous['compiler_sha256'] == COMPILER_SHA and previous['runtime_sha256'] == RUNTIME_SHA
    review_path = ROOT / 'bootstrap/evidence/stdlib/stdlib-maps-nominal-region-static-results.json'
    review = json.loads(review_path.read_text())
    for row in review['proposals']:
        assert digest(ROOT / row['original']) == row['original_sha256']
        assert digest(ROOT / row['proposal']) == row['proposal_sha256']
    for row in review['actual_historical_headers']:
        assert digest(ROOT / row['path']) == row['sha256']
    report = {'kind': 'one-bounded-annotated-C839-Region-runtime-unit', 'status': 'prepared',
              'compiler_sha256': COMPILER_SHA, 'runtime_sha256': RUNTIME_SHA,
              'provenance_sha256': digest(FROZEN / 'provenance.json'), 'source_receipt_sha256': digest(source_receipt),
              'cpu_affinity': [args.cpu], 'candidate_selected': False, 'production_promoted': False,
              'native_family_relabelled': False, 'historical_interfaces_modified': False,
              'fresh_start_minimum_bytes': 350 * 1024 * 1024, 'minimum_reserve_bytes': 250 * 1024 * 1024,
              'maximum_new_output_bytes': args.maximum_new_output_mib * 1024 * 1024, 'runs': [],
              'unit': f'trace-{args.case}' if args.unit == 'trace' else args.unit,
              'failure_indices': {'101-103': 'independent value-NaN and callback-mutation edge cases',
                                  '201-207': 'independent full-bit IEEE64 multiset observations',
                                  '301-303': 'mixed IO+Mut source, value and EqualBy callback contracts'}}
    free = shutil.disk_usage(ROOT).free
    report['free_bytes_before'] = free
    if free < report['fresh_start_minimum_bytes']:
        report.update(status='resource-paused-before-compiler', compiler_invoked=False)
        save(args.output, report)
        print(f'Resource pause: {free} bytes free; no compiler invoked')
        return 2
    assert not args.artifacts.exists(), 'Preserve every previous attempt and choose a fresh directory'
    args.artifacts.mkdir(parents=True)
    modules = {
        'Data.Mutable.Map': ROOT / 'build/maps-nominal-region-static/candidate/Data.Mutable.Map.cinch',
        'Data.Mutable.HashTable': ROOT / 'build/maps-nominal-region-static/common/Data.Mutable.HashTable.cinch',
        'Data.Mutable.Var': ROOT / 'build/maps-nominal-region-static/common/Data.Mutable.Var.cinch',
        'Data.Mutable.Map.Optional': ROOT / 'compiler/prelude/Data/Mutable/Map/Optional.cinch',
        'Data.Mutable.Map.Float': ROOT / 'compiler/prelude/Data/Mutable/Map/Float.cinch',
    }
    report['sources'] = {}
    for module, path in modules.items():
        target = args.artifacts / (module + '.cinch')
        target.write_bytes(path.read_bytes())
        report['sources'][module] = {'original': str(path), 'sha256': digest(target)}
    report['fixtures'] = {}
    if args.unit == 'combined':
        probes = {'MapProbe.Edge': edge_source(), 'MapProbe.IEEE64': float_source(64),
                  'MapProbe.Mixed': (ROOT / 'compiler/tests/fixtures/mutable-map-effect-control.cinch').read_text()}
        for module, text in probes.items():
            target = args.artifacts / (module + '.cinch')
            target.write_text(export_probe(text, module))
            report['fixtures'][module] = {'original_consumer_sha256': hashlib.sha256(text.encode()).hexdigest(),
                                         'selected_module_sha256': digest(target),
                                         'only_transformation': 'Rename Main module and export its existing main entry; every assertion and helper is unchanged.'}
        consumer = ENTRY
    else:
        consumer = source(args.case) if args.unit == 'trace' else (ROOT / 'compiler/tests/fixtures/mutable-map-control.cinch').read_text()
        report['failure_indices'] = {'index': 'The original independent trace step or ownership/control group; no transformation.'}
        report['fixtures']['Main'] = {'original_consumer_sha256': hashlib.sha256(consumer.encode()).hexdigest(),
                                      'only_transformation': 'None; exact original generated trace or ownership/control consumer.'}
    entry = args.artifacts / 'Main.cinch'
    entry.write_text(consumer)
    report['entry_sha256'] = digest(entry)
    for name in ('maps-vectors.json', 'maps-edge-vectors.json'):
        report[name] = digest(ROOT / 'compiler/tests/fixtures' / name)
    executable, llvm = args.artifacts / 'consumer', args.artifacts / 'consumer.ll'
    environment = {**os.environ, 'RUST_MIN_STACK': '67108864', 'CINCH_TEST_API_EMIT_INTERFACES': '0'}
    command = [str(compiler), 'build', str(entry.resolve()), '-o', str(executable.resolve()),
               '--opt-level', '2', '--runtime-archive', str(runtime)]
    report['compiler_invoked'] = True
    try:
        report['compile'] = execute(command, args.artifacts, environment, host=True)
        (args.artifacts / 'build.log').write_text(report['compile']['stdout'] + report['compile']['stderr'])
        assert report['compile']['exit'] == 0, report['compile']
        report['executable_sha256'], report['llvm_sha256'] = digest(executable), digest(llvm)
        report['output_bytes_after_compile'] = output_bytes(args.artifacts)
        assert report['output_bytes_after_compile'] <= report['maximum_new_output_bytes']
        assert shutil.disk_usage(ROOT).free >= report['minimum_reserve_bytes']
        for poisoned in (False, True):
            run_environment = {**environment, 'CINCH_HEAP_LIMIT': '256M' if poisoned else '32M',
                               'CINCH_STACK_SIZE': '8M', 'CINCH_RC_DEBUG': '1' if poisoned else '0',
                               'CINCH_LEAK_CHECK': '1', 'CINCH_ALLOC_STATS': '1'}
            run = execute([str(executable.resolve())], args.artifacts, run_environment)
            run.update(poisoned_free=poisoned, heap=run_environment['CINCH_HEAP_LIMIT'], stack='8M')
            report['runs'].append(run)
            (args.artifacts / ('poisoned.log' if poisoned else 'normal.log')).write_text(run['stdout'] + run['stderr'])
            assert run['exit'] == 0, run
            assert run['stdout'] == '', run
            assert 'cinch-leak: live=0 immortal=0 leaked=0' in run['stderr'], run
        for row in review['actual_historical_headers']:
            assert digest(ROOT / row['path']) == row['sha256']
        report.update(status='passed', output_bytes=output_bytes(args.artifacts), free_bytes_after=shutil.disk_usage(ROOT).free)
        assert report['output_bytes'] <= report['maximum_new_output_bytes']
        assert report['free_bytes_after'] >= report['minimum_reserve_bytes']
        print(f"{report['unit']} fixture passed normal+poison with zero leaks")
        return 0
    except BaseException as error:
        report.update(status='failed', failure=repr(error), output_bytes=output_bytes(args.artifacts))
        raise
    finally:
        save(args.output, report)


if __name__ == '__main__':
    raise SystemExit(main())
