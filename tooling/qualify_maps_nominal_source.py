#!/usr/bin/env python3
"""Check only the isolated nominal Region proposal with the qualified Role+Slots tuple and a fresh disk guard."""
from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[1]
FROZEN = Path('/workspace/dasl-stdlib-io/build/stdlib-nominal-region-slots-frozen')
COMPILER_SHA = '330adb4d77f8bcbfc26f246269280f391c3a10b6d3a40771b5f6243f6a8969b2'
RUNTIME_SHA = 'd91ce012395a99e1148232d20ec316979cd7d0cb13647a3283ba4820192c0abf'


def digest(path: Path) -> str:
    """Identify immutable tuple inputs and actual output bytes without changing metadata."""
    return hashlib.sha256(path.read_bytes()).hexdigest()


def save(path: Path, value: dict) -> None:
    """Preserve a complete source-only observation with explicit qualification limits."""
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(value, indent=2) + '\n')


def main() -> int:
    """Stop before compiler launch when the fresh guard fails; otherwise inspect actual defining headers."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--cpu', type=int, required=True)
    parser.add_argument('--minimum-free-mib', type=int, choices=(200, 350), default=350,
                        help='Use200 only for the explicitly authorized bounded source-only exception')
    parser.add_argument('--minimum-reserve-mib', type=int, default=180,
                        help='Retain the source-only minimum disk reserve after checking')
    parser.add_argument('--artifacts', type=Path, default=ROOT / 'build/maps-role-slots-source')
    parser.add_argument('--output', type=Path, default=ROOT / 'compiler/evidence/stdlib/stdlib-maps-role-slots-source-results.json')
    args = parser.parse_args()
    if not 180 <= args.minimum_reserve_mib <= args.minimum_free_mib:
        parser.error('source reserve must remain at least180MiB and at most the fresh-start guard')
    os.sched_setaffinity(0, {args.cpu})
    compiler, runtime = FROZEN / 'cinch', FROZEN / 'libcinchruntime.a'
    assert digest(compiler) == COMPILER_SHA and digest(runtime) == RUNTIME_SHA
    review_path = ROOT / 'compiler/evidence/stdlib/stdlib-maps-nominal-region-static-results.json'
    review = json.loads(review_path.read_text())
    for row in review['proposals']:
        assert digest(ROOT / row['original']) == row['original_sha256']
        assert digest(ROOT / row['proposal']) == row['proposal_sha256']
    for row in review['actual_historical_headers']:
        assert digest(ROOT / row['path']) == row['sha256']
    report = {'kind': 'isolated-C839-explicit-nominal-Region-source-qualification',
              'compiler_sha256': COMPILER_SHA, 'runtime_sha256': RUNTIME_SHA,
              'provenance_sha256': digest(FROZEN / 'provenance.json'), 'cpu_affinity': [args.cpu],
              'review_sha256': digest(review_path), 'runtime_executed': False,
              'native_family_relabelled': False, 'candidate_selected': False, 'package_complete': False,
              'production_sources_modified': False, 'historical_interfaces_modified': False,
              'fresh_start_minimum_bytes': args.minimum_free_mib * 1024 * 1024,
              'minimum_reserve_bytes': args.minimum_reserve_mib * 1024 * 1024,
              'maximum_source_output_bytes': 5 * 1024 * 1024,
              'scope_of_resource_exception': 'Source-only; runtime and emission guards remain unchanged.'}
    free = shutil.disk_usage(ROOT).free
    report['free_bytes_before'] = free
    if free < report['fresh_start_minimum_bytes']:
        report.update(status='resource-paused-before-compiler', compiler_invoked=False)
        save(args.output, report)
        print(f'Resource pause: {free} bytes free, less than fresh{args.minimum_free_mib}MiB guard; no compiler invoked')
        return 2
    assert not args.artifacts.exists(), 'Preserve any previous attempt; choose a fresh --artifacts and --output pair'
    overrides = {name: ROOT / path for name, path in (
        ('Data.Mutable.Map', 'build/maps-nominal-region-static/candidate/Data.Mutable.Map.cinch'),
        ('Data.Mutable.HashTable', 'build/maps-nominal-region-static/common/Data.Mutable.HashTable.cinch'),
        ('Data.Mutable.Var', 'build/maps-nominal-region-static/common/Data.Mutable.Var.cinch'))}
    runner_report = args.artifacts.with_name(args.artifacts.name + '-runner-results.json')
    assert not runner_report.exists()
    command = [sys.executable, str(ROOT / 'tooling/validate_maps.py'), '--frontend', 'rust',
               '--compiler', str(compiler), '--runtime', str(runtime), '--cpu', str(args.cpu),
               '--modes', 'source', '--minimum-free-mib', str(args.minimum_free_mib),
               '--minimum-reserve-mib', str(args.minimum_reserve_mib), '--maximum-unit-mib', '5',
               '--artifacts', str(args.artifacts), '--output', str(runner_report)]
    for name, path in overrides.items():
        command += ['--source-override', name + '=' + str(path)]
    report.update(compiler_invoked=True, command=command)
    result = subprocess.run(command, cwd=ROOT, timeout=60)
    report.update(exit=result.returncode, runner_report=str(runner_report), runner_report_sha256=digest(runner_report))
    if result.returncode:
        report['status'] = 'source-refused'
        save(args.output, report)
        return result.returncode
    actual = json.loads(runner_report.read_text())
    assert actual['status'] == 'passed' and actual['compiler_sha256'] == COMPILER_SHA
    assert actual['sources']['Data.Mutable.Map'] == review['proposals'][0]['proposal_sha256']
    headers = []
    for module, name in [('Data.Mutable.Map', 'MutableMap'), ('Data.Mutable.HashTable', 'HashTable'), ('Data.Mutable.Var', 'Var')]:
        path = args.artifacts / 'source/build' / (module + '.cinchif')
        data = json.loads(path.read_text())
        header = next(row for row in data['exports']['types'] if row['name'] == name)
        assert header['identity'] == module + '.' + name
        assert header['parameters'][0] == {'implicit': False, 'kind': 'Region', 'name': 'r'}, header
        assert all(row['kind'] == 'Type' for row in header['parameters'][1:])
        headers.append({'path': str(path), 'sha256': digest(path), 'actual_header': header})
    map_data = json.loads((args.artifacts / 'source/build/Data.Mutable.Map.cinchif').read_text())
    assert not any(row['name'] == 'MapEntry' for row in map_data['exports']['types'])
    private_source = overrides['Data.Mutable.Map'].read_text()
    assert 'data MapEntry (r :: Region) k v where' in private_source
    report['private_MapEntry'] = {'explicit_defining_declaration_source_checked': True, 'public_header_emitted': False,
                                  'source_sha256': digest(overrides['Data.Mutable.Map']),
                                  'claim': 'The checked source declaration is explicitly Region; no public private-type header is claimed.'}
    for row in review['actual_historical_headers']:
        assert digest(ROOT / row['path']) == row['sha256']
    size = sum(path.stat().st_size for path in args.artifacts.rglob('*') if path.is_file()) + runner_report.stat().st_size
    assert size <= report['maximum_source_output_bytes'], size
    report.update(status='source-and-public-defining-headers-passed', actual_headers=headers, output_bytes=size)
    save(args.output, report)
    print('Source-only proposal and actual public Region headers passed; no runtime or interface-consumer claim')
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
