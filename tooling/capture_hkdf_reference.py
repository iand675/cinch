#!/usr/bin/env python3
"""Capture actual pinned SDK HKDF outcomes before any candidate execution.

The reference and independent scalar validator are copied byte-for-byte. The
existing cache is reusable optimization state; fresh cache growth is included
in the unit's explicit thirty-MiB output budget.
"""
from __future__ import annotations
import argparse
import json
import os
from pathlib import Path
import shutil
import signal
import subprocess
import time
from check_hkdf_source import digest

#: Owned static reference and unchanged fixed expectations.
ROOT = Path(__file__).resolve().parents[1]


def tree_bytes(root: Path) -> int:
    """Count all regular output files, including transient compiler work and cache entries."""
    total = 0
    for directory, children, names in os.walk(root):
        for name in names:
            try:
                total += (Path(directory) / name).stat().st_size
            except FileNotFoundError:
                pass
    return total


def main() -> None:
    """Run one offline one-worker reference unit and preserve its first failure plainly."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--work', type=Path, required=True)
    parser.add_argument('--go', type=Path, required=True)
    parser.add_argument('--cache', type=Path, required=True)
    parser.add_argument('--cpu', type=int, default=1)
    args = parser.parse_args()
    work, sdk, cache = args.work.resolve(), args.go.resolve(), args.cache.resolve()
    assert not work.exists(), 'never overwrite a reference epoch'
    if shutil.disk_usage(ROOT).free < 350 * 1024**2:
        raise SystemExit('350MiB SDK-unit start guard; no SDK process started')
    os.sched_setaffinity(0, {args.cpu})
    work.mkdir(parents=True)
    for name in ['reference.go', 'rfc5869-vectors.json']:
        shutil.copyfile(ROOT / 'experiments/hkdf-v1' / name, work / name)
    shutil.copyfile(ROOT / 'tooling/validate_hkdf_reference.py', work / 'validate.py')
    (work / 'tmp').mkdir()
    baseline = tree_bytes(cache)
    report = {'scope': __doc__, 'candidate_executed': False, 'elapsed_performance_claim': False,
              'sdk_sha256': digest(sdk), 'sdk_version_file': (sdk.parent.parent / 'VERSION').read_text(),
              'reference_sha256': digest(work / 'reference.go'), 'rfc_sha256': digest(work / 'rfc5869-vectors.json'),
              'validator_sha256': digest(work / 'validate.py'), 'runner_sha256': digest(Path(__file__)),
              'cpu': args.cpu, 'initial_free_bytes': shutil.disk_usage(work).free,
              'cache': str(cache), 'cache_baseline_bytes': baseline,
              'output_cap_bytes_including_cache_growth': 30 * 1024**2, 'commands': []}
    env = {**os.environ, 'GOMAXPROCS': '1', 'GOTELEMETRY': 'off', 'GOTOOLCHAIN': 'local',
           'GOPROXY': 'off', 'GOSUMDB': 'off', 'GOWORK': 'off', 'GO111MODULE': 'off',
           'CGO_ENABLED': '0', 'GOCACHE': str(cache), 'GOTMPDIR': str(work / 'tmp')}

    def save() -> None:
        """Keep the complete command/resource ledger even on an unexpected reference outcome."""
        report.update(free_bytes=shutil.disk_usage(work).free, work_bytes=tree_bytes(work),
                      cache_growth_bytes=max(0, tree_bytes(cache) - baseline))
        (work / 'report.json').write_text(json.dumps(report, indent=2) + '\n')

    def execute(command: list[str], label: str, seconds: int, extra: dict | None = None) -> None:
        """Stop one subprocess at its deadline, reserve or total fresh-byte boundary."""
        save()
        row = {'command': command, 'environment_override': extra or {}, 'stdout': label + '.stdout', 'stderr': label + '.stderr'}
        report['commands'].append(row)
        started, reason = time.monotonic(), None
        with (work / row['stdout']).open('wb') as stdout, (work / row['stderr']).open('wb') as stderr:
            process = subprocess.Popen(command, cwd=work, env={**env, **(extra or {})}, stdout=stdout,
                                       stderr=stderr, start_new_session=True)
            while process.poll() is None:
                save()
                if time.monotonic() - started >= seconds:
                    reason = 'unit subprocess deadline'
                elif report['free_bytes'] < 250 * 1024**2:
                    reason = '250MiB free reserve'
                elif report['work_bytes'] + report['cache_growth_bytes'] >= 30 * 1024**2:
                    reason = '30MiB fresh-byte cap including cache growth'
                if reason:
                    os.killpg(process.pid, signal.SIGTERM)
                    try:
                        process.wait(timeout=1)
                    except subprocess.TimeoutExpired:
                        os.killpg(process.pid, signal.SIGKILL)
                        process.wait()
                    break
                time.sleep(0.1)
            row.update(returncode=process.wait(), resource_stop=reason)
        save()
        if row['returncode'] or reason:
            raise RuntimeError('first SDK/reference failure: ' + label)

    try:
        execute([str(sdk), 'build', '-trimpath', '-p=1', '-o', str(work / 'reference'), str(work / 'reference.go')], 'build', 30)
        report['executable_sha256'] = digest(work / 'reference')
        execute([str(work / 'reference'), '-family', 'all', '-rfc', str(work / 'rfc5869-vectors.json')], 'all', 15,
                {'GODEBUG': 'fips140=off'})
        execute(['python3', str(work / 'validate.py'), '--observations', str(work / 'all.stdout'),
                 '--rfc', str(work / 'rfc5869-vectors.json'), '--output', str(work / 'all-validation.json')], 'validate-all', 15)
        execute([str(work / 'reference'), '-family', 'policy', '-rfc', str(work / 'rfc5869-vectors.json')], 'policy', 15,
                {'GODEBUG': 'fips140=only'})
        execute(['python3', str(work / 'validate.py'), '--observations', str(work / 'policy.stdout'),
                 '--rfc', str(work / 'rfc5869-vectors.json'), '--output', str(work / 'policy-validation.json'), '--fips-only'], 'validate-policy', 15)
        normal = json.loads((work / 'all.stdout').read_text())
        policy = json.loads((work / 'policy.stdout').read_text())
        if len(normal['observations']) != 107 or len(policy['observations']) != 20:
            raise RuntimeError('unexpected actual reference row matrix')
        report.update(status='passed', normal_rows=107, policy_rows=20, checked_operations=381,
                      normal_observations_sha256=digest(work / 'all.stdout'),
                      policy_observations_sha256=digest(work / 'policy.stdout'))
    except BaseException as error:
        report.update(status='failed-or-resource-stopped', failure=repr(error), all_failure_artifacts_plain=True)
        raise
    finally:
        save()
    print('127 actual reference rows /381 operations agree with independent scalar/fixed RFC expectations')


if __name__ == '__main__':
    main()
