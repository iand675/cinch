#!/usr/bin/env python3
"""Verify current-source noncycle observations and reversible consumer changes without relabeling historical epochs."""
from pathlib import Path
import argparse
import hashlib
import json
import re
import check
import verify
from verify_runtime import validate_archive


def original_bytes(current, preparation):
    """Undo only the exact recorded declaration/import changes and recover every original observation/body byte."""
    text = current.decode()
    imports = preparation['added_imports']
    if imports:
        before = 'module Main where\n' + ''.join(imports)
        assert text.count(before) == 1
        text = text.replace(before, 'module Main where\n', 1)
    rows = text.splitlines(keepends=True)
    for change in preparation['changes']:
        at = change['line'] - 1
        assert rows[at] == change['new']
        rows[at] = change['old']
    return ''.join(rows).encode()


def main():
    """Require literal observation counts, actual source admission, exact runtime reports and streamed success artifacts."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--artifacts', type=Path, default=check.ROOT / 'build')
    args = parser.parse_args()
    verify.main()
    gates = json.loads((check.HERE / 'corpus-gates.json').read_text())
    assert gates['compiler_sha256'] == check.COMPILER and gates['runtime_sha256'] == check.RUNTIME
    assert not gates['production_adopted']
    for suite, row in gates['cases'].items():
        fixture = check.HERE / row['fixture']
        data = fixture.read_bytes()
        assert check.digest(fixture) == row['fixture_sha256'] == row['preparation']['output_sha256']
        assert hashlib.sha256(original_bytes(data, row['preparation'])).hexdigest() == row['preparation']['original_sha256']
        text = data.decode()
        count = (sum(map(int, re.findall(r'\(ScanCheck\s+(\d+)\s', text))) if suite == 'scanner'
                 else len(re.findall(r'\((?:Check|SplitCheck)\s', text)))
        assert count == row['observations'], (suite, count)
        source = args.artifacts / row['source_work']
        work = args.artifacts / row['runtime_work']
        assert check.digest(source / 'Main.cinch') == row['fixture_sha256']
        assert check.digest(source / 'report.json') == row['source_report_sha256']
        admitted = json.loads((source / 'report.json').read_text())
        assert admitted['source_gate_passed'] and admitted['returncode'] == 0 and not admitted['timeout_without_verdict']
        assert len(admitted['public_contracts']) == 9
        assert check.digest(work / 'report.json') == row['runtime_report_sha256']
        report = json.loads((work / 'report.json').read_text())
        assert report == row['runtime'] and report['source_receipt_sha256'] == row['source_report_sha256']
        assert report['all_passed'] and not report['retained_cycle_control']
        assert report['compiler_sha256'] == check.COMPILER and report['runtime_sha256'] == check.RUNTIME
        assert len(report['runs']) == 2 and {run['poisoned'] for run in report['runs']} == {False, True}
        for run in report['runs']:
            assert run['returncode'] == 0 and run['stdout'] == ''
            assert run['heap'] == ('256M' if run['poisoned'] else '32M') and run['stack'] == '8M'
            assert 'live=0 immortal=0 leaked=0' in run['stderr']
        validate_archive(work, row)
    print('Verified current noncycle observations:', ', '.join(name + '=' + str(row['observations']) for name, row in gates['cases'].items()))


if __name__ == '__main__':
    main()
