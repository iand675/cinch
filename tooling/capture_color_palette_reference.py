#!/usr/bin/env python3
"""Capture exact installed fixed-palette tables and public operations in a bounded standalone SDK unit."""
from __future__ import annotations
import argparse
import json
import os
from pathlib import Path
import shutil
import subprocess
from check_compare import digest


def main() -> None:
    """Retain the actual source, binary, commands and first failure without claiming Cinch qualification."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--go', type=Path, required=True)
    parser.add_argument('--source', type=Path, required=True)
    parser.add_argument('--work', type=Path, required=True)
    parser.add_argument('--cpu', type=int, default=1)
    args = parser.parse_args()
    work = args.work.resolve()
    work.mkdir(parents=True, exist_ok=False)
    initial = shutil.disk_usage(work).free
    if initial < 350*1024*1024:
        raise RuntimeError('350MiB initial SDK-unit guard')
    os.sched_setaffinity(0, {args.cpu})
    sdk = args.go.resolve()
    root = sdk.parent.parent
    shutil.copyfile(args.source, work / 'reference.go')
    report = {'scope': __doc__, 'runner_sha256': digest(Path(__file__)),
              'source_sha256': digest(work / 'reference.go'), 'sdk': str(sdk),
              'sdk_sha256': digest(sdk), 'cpu_affinity': [args.cpu], 'initial_free_bytes': initial,
              'sdk_sources': {name: digest(root / ('src/'+name)) for name in
                              ['image/color/palette/palette.go', 'image/color/color.go']},
              'commands': [], 'cinch_runtime_claim': False, 'elapsed_claim': False}
    env = {**os.environ, 'GOMAXPROCS': '1', 'GOTELEMETRY': 'off', 'GOTOOLCHAIN': 'local',
           'GOCACHE': '/tmp/cinch-mime-go-cache'}

    def save() -> None:
        """Preserve complete phase output and resource facts before another command can fail."""
        report.update(final_free_bytes=shutil.disk_usage(work).free,
                      output_bytes=sum(path.stat().st_size for path in work.rglob('*') if path.is_file()))
        (work / 'report.json').write_text(json.dumps(report, indent=2)+'\n')

    try:
        command = [str(sdk), 'build', '-trimpath', '-p=1', '-o', str(work / 'reference'), str(work / 'reference.go')]
        built = subprocess.run(command, cwd=work, text=True, capture_output=True, env=env, timeout=30)
        report['commands'].append({'command': command, 'returncode': built.returncode,
                                   'stdout': built.stdout, 'stderr': built.stderr})
        save()
        if built.returncode:
            raise RuntimeError('first SDK build failure')
        if report['output_bytes'] > 30*1024*1024 or report['final_free_bytes'] < 250*1024*1024:
            raise RuntimeError('SDK-unit output/reserve guard')
        command = [str(work / 'reference')]
        with (work / 'vectors.json').open('w') as handle:
            ran = subprocess.run(command, cwd=work, stdout=handle, stderr=subprocess.PIPE,
                                 text=True, env=env, timeout=15)
        report['commands'].append({'command': command, 'returncode': ran.returncode, 'stderr': ran.stderr})
        if ran.returncode:
            raise RuntimeError('first actual SDK reference failure')
        vectors = json.loads((work / 'vectors.json').read_text())
        if [(row['name'], len(row['entries']), len(row['queries'])) for row in vectors['palettes']] != [('Plan9', 256, 512), ('WebSafe', 216, 512)]:
            raise RuntimeError('unexpected actual table/query domain')
        report.update(status='passed', vectors_sha256=digest(work / 'vectors.json'),
                      executable_sha256=digest(work / 'reference'), sdk_version=vectors['sdk'],
                      exact_table_entries=472, palette_use_queries=1024)
    except BaseException as error:
        report.update(status='failed-or-resource-paused', failure=repr(error))
        raise
    finally:
        save()
    if report['output_bytes'] > 30*1024*1024 or report['final_free_bytes'] < 250*1024*1024:
        raise RuntimeError('SDK-unit output/reserve guard')
    print('472 actual ordered colors and1024 public palette queries captured; no Cinch claim')


if __name__ == '__main__':
    main()
