#!/usr/bin/env python3
"""Qualify bounded ordinary streaming tabwriter batches serially, preserving the first failure and every independent case identity."""
from pathlib import Path
import argparse
import json
import os
import shutil
import subprocess
import sys
import check


def main():
    """Freeze current streaming source identities, observe ordinary runtime profiles, and archive only closed successes."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--consumers', type=Path, required=True)
    parser.add_argument('--work', type=Path, required=True)
    parser.add_argument('--start', type=int, default=0)
    parser.add_argument('--stop', type=int)
    parser.add_argument('--pause-file', type=Path, default=check.ROOT / 'build/TABWRITER-HOLD')
    args = parser.parse_args()
    os.sched_setaffinity(0, {2})
    manifest_path = args.consumers / 'manifest.json'
    manifest = json.loads(manifest_path.read_text())
    rows = manifest['batches']
    stop = args.stop if args.stop is not None else len(rows)
    assert 0 <= args.start < stop <= len(rows)
    assert shutil.disk_usage(check.ROOT).free >= 350 * check.MIB
    work = args.work.resolve()
    work.mkdir(parents=True, exist_ok=True)
    report_path = work / 'report.json'
    text_sources = {p.name: check.digest(p) for p in (check.HERE / 'sources').glob('*.cinch')}
    if report_path.exists():
        report = json.loads(report_path.read_text())
        assert report['source_identities'] == text_sources and report['manifest_sha256'] == check.digest(manifest_path)
    else:
        report = {'scope': __doc__, 'manifest_sha256': check.digest(manifest_path), 'source_identities': text_sources,
                  'compiler_sha256': check.COMPILER, 'runtime_sha256': check.RUNTIME, 'batches': [],
                  'production_adopted': False, 'native_qualified': False, 'all51_qualified': False}

    def save():
        """Publish exact per-phase receipts and original local-to-global failure mappings at each natural boundary."""
        report_path.write_text(json.dumps(report, indent=2) + '\n')

    def boundary():
        """Honor peer timing/output holds before starting another child and preserve the shared free-space reserve."""
        if args.pause_file.exists() or shutil.disk_usage(check.ROOT).free < 350 * check.MIB:
            report['paused_without_next_child'] = {'pause_file_present': args.pause_file.exists(),
                                                  'fresh_free_bytes': shutil.disk_usage(check.ROOT).free}
            save()
            print('Paused at natural boundary without launching the next child; completed phase receipts retained', flush=True)
            raise SystemExit(0)
        assert {p.name: check.digest(p) for p in (check.HERE / 'sources').glob('*.cinch')} == text_sources

    def invoke(command, row, phase):
        """Run one source, runtime or recovery child sequentially; a nonzero result ends the entire batch driver."""
        boundary()
        result = subprocess.run(command, cwd=check.ROOT, capture_output=True, text=True)
        row[phase + '_driver'] = {'command': command, 'returncode': result.returncode,
                                  'stdout': result.stdout, 'stderr': result.stderr}
        save()
        assert result.returncode == 0, 'first ' + phase + ' result retained; no later batch launched'

    save()
    for index in range(args.start, stop):
        batch = rows[index]
        entry = (args.consumers / batch['source']).resolve()
        assert check.digest(entry) == batch['sha256']
        source = (check.ROOT / 'build' / f'tabwriter-protocol-{index:03d}-source-v1') if index < 2 else work / f'tabwriter-protocol-{index:03d}-source'
        runtime = (check.ROOT / 'build' / f'tabwriter-protocol-{index:03d}-runtime-v1') if index < 2 else work / f'tabwriter-protocol-{index:03d}-runtime'
        existing = [row for row in report['batches'] if row['index'] == index]
        assert len(existing) <= 1
        row = existing[0] if existing else {'index': index, 'cases': batch['cases'], 'source': str(source), 'runtime': str(runtime)}
        if not existing:
            report['batches'].append(row)
        if not source.exists():
            invoke([sys.executable, '-B', str(check.HERE / 'check.py'), '--main', str(entry), '--work', str(source)], row, 'source')
        source_receipt = source / 'report.json'
        admitted = json.loads(source_receipt.read_text())
        assert admitted['source_gate_passed'] and admitted['main_sha256'] == batch['sha256']
        row['source_report_sha256'] = check.digest(source_receipt)
        if not runtime.exists():
            invoke([sys.executable, '-B', str(check.HERE / 'run.py'), '--checked', str(source), '--work', str(runtime)], row, 'runtime')
        runtime_receipt = runtime / 'report.json'
        actual = json.loads(runtime_receipt.read_text())
        assert actual['all_passed'] and len(actual['runs']) == 2
        row['runtime_report_sha256'] = check.digest(runtime_receipt)
        if not (runtime / 'closed-success-index.json').exists():
            invoke([sys.executable, '-B', str(check.HERE / 'archive_success.py'), '--work', str(runtime)], row, 'archive')
        row['archive_index_sha256'] = check.digest(runtime / 'closed-success-index.json')
        row['all_passed'] = True
        save()
        print(f'Batch{index:03d}: {len(batch["cases"])} exact streaming traces PASS normal+poison; success-only archive durable', flush=True)
    passed = [row for row in report['batches'] if row.get('all_passed')]
    report['all51_qualified'] = len(passed) == len(rows) and sum(len(row['cases']) for row in passed) == 51
    save()


if __name__ == '__main__':
    main()
