#!/usr/bin/env python3
"""Validate exact MIME coverage across retained, explicitly identified fixture epochs.

Completed observations are never synthesized. Reports may share unchanged
library code while a bounded fixture constructor replaces excessive source
nesting. Documentation-only normalization is recorded separately and checked
against the retained source, rather than silently relabeling an earlier run.
"""
from __future__ import annotations
import argparse
import gzip
import hashlib
import json
from pathlib import Path
import re
from bench_stdlib import source_closure
from check_mime import ROOT, generated_cases


def digest(path: Path) -> str:
    """Identify retained bytes before accepting an observation as evidence."""
    return hashlib.sha256(path.read_bytes()).hexdigest()


def artifact_digest(path: Path) -> str:
    """Verify executed bytes directly or through their verified lossless cold-client archive."""
    if path.is_file():
        return digest(path)
    value = hashlib.sha256()
    with gzip.open(Path(str(path) + '.gz'), 'rb') as source:
        while piece := source.read(1048576):
            value.update(piece)
    return value.hexdigest()


def code_lines(path: Path) -> list[str]:
    """Permit only changes to complete leading-comment lines, preserving every code line."""
    return [line for line in path.read_text().splitlines() if not line.lstrip().startswith('--')]


def main() -> None:
    """Check source, profiles, artifacts and every primary SDK observation per frontend pair."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--reports', nargs='+', type=Path, required=True)
    parser.add_argument('--vectors', type=Path, default=ROOT / 'seed/tests/fixtures/mime-vectors.json')
    parser.add_argument('--out', type=Path, required=True)
    parser.add_argument('--allow-incomplete', action='store_true')
    args = parser.parse_args()
    data = json.loads(args.vectors.read_text())
    families = ('media', 'formats', 'encoded', 'decoded', 'writes', 'registry')
    expected = {name: len(data[name]) for name in families}
    expected_controls = {name: count for name, (_, count) in generated_cases(data).items() if name in ('decoded-callbacks', 'ownership')}
    closure = source_closure((ROOT / 'tooling/data/mime-consumer.cinchpart').read_text())
    summary = {'format': 1, 'status': 'incomplete', 'vectors_sha256': digest(args.vectors), 'expected_primary_observations': expected, 'reports': [], 'frontends': {}, 'historical_failures': []}
    coverage = {}
    for path in args.reports:
        report = json.loads(path.read_text())
        if report['vectors_sha256'] != summary['vectors_sha256']:
            raise RuntimeError('different independent fixture corpus: ' + str(path))
        changes = []
        for module, recorded in report['sources'].items():
            retained = path.parent / (module + '.cinch')
            if digest(retained) != recorded:
                raise RuntimeError('retained source no longer matches report: ' + str(retained))
            current = closure[module]
            if digest(current) != recorded:
                if code_lines(retained) != code_lines(current):
                    raise RuntimeError('functional library source changed: ' + module)
                changes.append({'module': module, 'retained_sha256': recorded, 'current_sha256': digest(current), 'change': 'complete leading-comment lines only'})
        item = {'path': str(path.resolve()), 'sha256': digest(path), 'comment_normalization': changes, 'completed_runs': 0}
        summary['reports'].append(item)
        summary['historical_failures'].extend({'report': str(path), **failure} for failure in report.get('failures', []))
        for row in report['runs']:
            profiles = row['profiles']
            if len(profiles) != 2:
                continue
            for observed, (poison, heap) in zip(profiles, [('0', '32M'), ('1', '256M')]):
                if (observed['poison'], observed['heap'], observed['stack']) != (poison, heap, '8M') or 'leaked=0' not in observed['stderr']:
                    raise RuntimeError('invalid allocation profile: ' + row['case'])
            if report['compiler_host_stack_bytes'] != 67108864:
                raise RuntimeError('compiler host stack profile changed')
            label = row['frontend']; identity = report['frontends'][label]
            if label == 'native' and report.get('compiler_environment', {}).get('CINCH_STACK_SIZE') != '64M':
                raise RuntimeError('Native compiler program-stack bound was not explicit')
            pair = {key: identity[key] for key in ('compiler_sha256', 'runtime_sha256', 'provider_sha256', 'base.cpk', 'base-objects.json') if key in identity}
            if label in summary['frontends'] and summary['frontends'][label]['identity'] != pair:
                raise RuntimeError('multiple qualification pairs for frontend: ' + label)
            if label not in coverage:
                coverage[label] = {name: set() for name in families}
                summary['frontends'][label] = {'identity': pair, 'qualified_runs': [], 'controls': {}}
            source = path.parent / (row['case'] + '.source')
            if digest(source) != row['source_sha256']:
                raise RuntimeError('retained consumer does not match executed bytes: ' + str(source))
            for suffix, field in [('.ll', 'ir_sha256'), ('', 'executable_sha256')]:
                artifact = path.parent / (label + '-' + row['case'] + suffix)
                if artifact_digest(artifact) != row[field]:
                    raise RuntimeError('executed artifact changed: ' + str(artifact))
            qualified = {'case': row['case'], 'count': row['count'], 'report': str(path.resolve()), 'consumer_sha256': row['source_sha256'], 'ir_sha256': row['ir_sha256'], 'executable_sha256': row['executable_sha256']}
            summary['frontends'][label]['qualified_runs'].append(qualified)
            item['completed_runs'] += 1
            match = re.fullmatch(r'(media|formats|encoded|decoded|writes)-(\d+)', row['case'])
            if match:
                family, start = match.group(1), int(match.group(2))
                if start + row['count'] > expected[family]:
                    raise RuntimeError('out-of-range fixture interval')
                coverage[label][family].update(range(start, start + row['count']))
            elif row['case'] == 'registry':
                if row['count'] != expected['registry']:
                    raise RuntimeError('partial ordered registry trace')
                coverage[label]['registry'].update(range(row['count']))
            else:
                summary['frontends'][label]['controls'][row['case']] = row['count']
    complete = set(coverage) == {'rust', 'native'}
    for label, seen in coverage.items():
        counts = {name: len(rows) for name, rows in seen.items()}
        summary['frontends'][label]['covered_primary_observations'] = counts
        summary['frontends'][label]['missing'] = {name: sorted(set(range(expected[name])) - seen[name]) for name in families if len(seen[name]) != expected[name]}
        complete = complete and counts == expected and all(summary['frontends'][label]['controls'].get(name, 0) >= count for name, count in expected_controls.items())
    summary['status'] = 'complete' if complete else 'incomplete'
    args.out.parent.mkdir(parents=True, exist_ok=True)
    args.out.write_text(json.dumps(summary, indent=2) + '\n')
    print(json.dumps({'status': summary['status'], 'coverage': {label: value['covered_primary_observations'] for label, value in summary['frontends'].items()}}))
    if not complete and not args.allow_incomplete:
        raise SystemExit('MIME qualification coverage is incomplete')


if __name__ == '__main__':
    main()
