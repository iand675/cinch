"""Compare exact check artifacts with driver diagnostics disabled and enabled."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import resource
import shutil
import subprocess


def digest(path):
    """Identify an exact compiler, source or emitted artifact."""
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    """Keep every epoch plain and refuse any diagnostic-dependent artifact change."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--compiler', type=Path, required=True)
    parser.add_argument('--baseline', type=Path, required=True)
    parser.add_argument('--work', type=Path, required=True)
    parser.add_argument('--cpu', type=int, default=3)
    args = parser.parse_args()
    os.sched_setaffinity(0, {args.cpu})
    resource.setrlimit(resource.RLIMIT_STACK, (67108864, resource.getrlimit(resource.RLIMIT_STACK)[1]))
    compiler, baseline, work = args.compiler.resolve(), args.baseline.resolve(), args.work.resolve()
    work.mkdir(parents=True, exist_ok=False)
    for path in (Path(__file__).parent / 'input').glob('*.cinch'):
        shutil.copy2(path, work / path.name)
    report = {'compiler_sha256': digest(compiler), 'baseline_sha256': digest(baseline), 'epochs': []}
    for label, binary, enabled in [('baseline', baseline, False), ('disabled', compiler, False), ('enabled', compiler, True)]:
        env = {**os.environ, 'CINCH_STORE': str(work / 'store')}
        env.pop('CINCH_TRACE_DRIVER_PHASES', None)
        if enabled:
            env['CINCH_TRACE_DRIVER_PHASES'] = '1'
        result = subprocess.run([str(binary), 'check', str(work / 'Main.cinch')], cwd=work,
                                env=env, capture_output=True, text=True, timeout=60)
        (work / (label + '.stdout')).write_text(result.stdout)
        (work / (label + '.stderr')).write_text(result.stderr)
        row = {'label': label, 'returncode': result.returncode, 'stderr': result.stderr}
        report['epochs'].append(row)
        (work / 'report.json').write_text(json.dumps(report, indent=2) + '\n')
        assert result.returncode == 0, result.stderr
        output = work / label
        (work / 'build').rename(output)
        row['artifacts'] = {str(path.relative_to(output)): digest(path) for path in sorted(output.rglob('*')) if path.is_file()}
        if label != 'baseline':
            assert row['artifacts'] == report['epochs'][0]['artifacts'], label
        phases = re.findall(r'^cinch\.driver\.phase (\S+) elapsed_seconds=(\d+\.\d+)$', result.stderr, re.M)
        if enabled:
            assert [name for name, _ in phases] == ['driver.begin', 'check.begin', 'check.end', 'contracts.begin', 'contracts.end', 'interface_bodies.begin', 'interface_bodies.end', 'optimization.begin', 'optimization.end', 'artifacts.begin', 'artifacts.end', 'check.return']
            times = [float(value) for _, value in phases]
            assert times == sorted(times)
        else:
            assert not phases
        (work / 'report.json').write_text(json.dumps(report, indent=2) + '\n')
    print('Identical artifacts across baseline, diagnostics disabled and enabled:', len(report['epochs'][0]['artifacts']))


if __name__ == '__main__':
    main()
