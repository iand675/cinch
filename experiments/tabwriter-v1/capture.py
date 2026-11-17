#!/usr/bin/env python3
"""Capture one bounded pinned-SDK tabwriter epoch with exact request/source/tool identities and actual protocol outcomes."""
from pathlib import Path
import argparse
import hashlib
import json
import os
import resource
import signal
import shutil
import subprocess
import time
from tabwriter_requests import requests

HERE = Path(__file__).resolve().parent
MIB = 1024 * 1024


def digest(path):
    """Hash exact source, executable, request and observation bytes in bounded memory."""
    result = hashlib.sha256()
    with path.open('rb') as stream:
        for chunk in iter(lambda: stream.read(MIB), b''):
            result.update(chunk)
    return result.hexdigest()


def bytes_used(path):
    """Measure actual output/cache bytes without deleting or relabeling any existing input."""
    return sum(item.stat().st_size for item in path.rglob('*') if item.is_file()) if path.exists() else 0


def main():
    """Build and execute only after fresh guards, preserving the first failure and original SDK panics as observations."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--work', type=Path, required=True)
    parser.add_argument('--full', action='store_true')
    parser.add_argument('--cache', type=Path, default=Path('/tmp/cinch-compare-go-cache'))
    parser.add_argument('--deadline', type=int, default=60)
    parser.add_argument('--built-from', type=Path, help='Reuse one exact preserved SDK executable without rebuilding its populated cache')
    args = parser.parse_args()
    os.sched_setaffinity(0, {2})
    work = args.work.resolve()
    assert shutil.disk_usage(HERE).free >= 350 * MIB, '350MiB fresh-start guard; no reference unit launched'
    assert not work.exists(), 'never overwrite a captured reference epoch'
    provenance = json.loads((HERE / 'reference-provenance.json').read_text())
    go = Path(provenance['executable'])
    assert digest(go) == provenance['executable_sha256']
    cases = requests(args.full)
    built_from = args.built_from.resolve() if args.built_from else None
    previous = None
    if built_from:
        previous = json.loads((built_from / 'report.json').read_text())
        assert previous['sdk_executable_sha256'] == digest(go)
        assert previous['steps'][0]['name'] == 'build' and previous['steps'][0]['returncode'] == 0
        assert digest(built_from / 'reference.go') == previous['observer_source_sha256']
        assert digest(built_from / 'requests.json') == previous['requests_sha256']
        assert previous['full'] == args.full and previous['request_count'] == len(cases)
    cache_before = bytes_used(args.cache)
    work.mkdir(parents=True)
    shutil.copyfile((built_from if built_from else HERE) / 'reference.go', work / 'reference.go')
    inputs = work / 'requests.json'
    inputs.write_text(json.dumps(cases, separators=(',', ':')) + '\n')
    report = {'status': 'independent SDK observations only; no Cinch implementation or completion claim',
              'sdk_executable_sha256': digest(go), 'observer_source_sha256': digest(work / 'reference.go'),
              'requests_sha256': digest(inputs), 'request_count': len(cases), 'full': args.full,
              'cache': str(args.cache), 'cache_bytes_before': cache_before,
              'artifact_and_observed_cache_growth_cap_bytes': 30 * MIB,
              'start_guard_bytes': 350 * MIB, 'reserve_bytes': 250 * MIB, 'cpu': 2, 'steps': []}
    if built_from:
        assert digest(inputs) == previous['requests_sha256'], 'requests changed before reused-executable capture'
        report['reused_build'] = {'path': str(built_from), 'report_sha256': digest(built_from / 'report.json'),
                                'executable_sha256': digest(built_from / 'reference'),
                                'prior_observed_cache_growth_bytes': previous.get('observed_cache_growth_bytes'),
                                'prior_capture_complete': previous.get('capture_complete', False),
                                'note': 'Prior build completed but exceeded its new-cache budget; no prior observations ran.'}

    def save():
        """Keep exact phase results and first resource or program failure durable in the new epoch."""
        (work / 'report.json').write_text(json.dumps(report, indent=2) + '\n')

    def guard():
        """Preserve the shared reserve and bound this unit's new visible outputs."""
        assert shutil.disk_usage(work).free >= 250 * MIB, '250MiB pause guard'
        assert bytes_used(work) <= 30 * MIB, '30MiB output cap'
        growth = max(0, bytes_used(args.cache) - cache_before)
        assert bytes_used(work) + growth <= 30 * MIB, '30MiB artifact/cache growth cap'

    def invoke(command, name, environment):
        """Use one bounded process, recording stdout/stderr without rendering captured data as instructions."""
        guard()
        before = time.monotonic()
        out_path, err_path = work / (name + '.stdout'), work / (name + '.stderr')

        def limits():
            """Cap a single emitted file and keep the tool host stack independent from later consumer limits."""
            resource.setrlimit(resource.RLIMIT_FSIZE, (30 * MIB, resource.getrlimit(resource.RLIMIT_FSIZE)[1]))
            resource.setrlimit(resource.RLIMIT_STACK, (64 * MIB, resource.getrlimit(resource.RLIMIT_STACK)[1]))

        reason = None
        with out_path.open('wb') as stdout, err_path.open('wb') as stderr:
            child = subprocess.Popen(command, cwd=work, env=environment, stdout=stdout, stderr=stderr,
                                     preexec_fn=limits, start_new_session=True)
            while child.poll() is None:
                if time.monotonic() - before > args.deadline:
                    reason = 'deadline without completed verdict'
                elif shutil.disk_usage(work).free < 250 * MIB:
                    reason = 'global250MiB reserve reached'
                elif bytes_used(work) + max(0, bytes_used(args.cache) - cache_before) > 30 * MIB:
                    reason = 'artifact/cache growth cap reached'
                if reason:
                    os.killpg(child.pid, signal.SIGTERM)
                    try:
                        child.wait(timeout=3)
                    except subprocess.TimeoutExpired:
                        os.killpg(child.pid, signal.SIGKILL)
                        child.wait()
                    break
                time.sleep(0.1)
            code = child.wait()
        report['steps'].append({'name': name, 'command': command, 'returncode': code,
                                'seconds': time.monotonic() - before, 'interrupted_reason': reason,
                                'stdout_sha256': digest(out_path), 'stderr': err_path.read_text()})
        save()
        guard()
        assert code == 0 and reason is None, 'first phase failure preserved: ' + name
        growth = max(0, bytes_used(args.cache) - cache_before)
        report['observed_cache_growth_bytes'] = growth
        save()
        assert bytes_used(work) + growth <= 30 * MIB, 'new artifact/cache output budget exceeded'

    environment = {**os.environ, 'GOCACHE': str(args.cache), 'GOMAXPROCS': '1', 'GOMEMLIMIT': '256MiB',
                   'GOTELEMETRY': 'off', 'GOTOOLCHAIN': 'local', 'GOPROXY': 'off', 'GOSUMDB': 'off',
                   'GOFLAGS': '-p=1', 'CGO_ENABLED': '0', 'GO111MODULE': 'off', 'GOWORK': 'off'}
    executable = (built_from if built_from else work) / 'reference'
    save()
    if not built_from:
        invoke([str(go), 'build', '-trimpath', '-o', str(executable), str(work / 'reference.go')], 'build', environment)
    report['observer_executable_sha256'] = digest(executable)
    save()
    invoke([str(executable), '-requests', str(inputs)], 'observe', environment)
    output_path = work / 'observe.stdout'
    observed = json.loads(output_path.read_text())
    assert observed['sdk'] == provenance['version_file'].splitlines()[0]
    assert observed['integer_bits'] == 64, 'separate machine interval policy required for another SDK target'
    assert [row['id'] for row in observed['cases']] == [row['id'] for row in cases]
    actions = [step for row in observed['cases'] for step in row['actions'] or []]
    offers = [offer for step in actions for offer in step['offers']]
    report.update(sdk=observed['sdk'], architecture=observed['architecture'], integer_bits=observed['integer_bits'],
                  observations_sha256=digest(output_path), cases=len(observed['cases']), actions=len(actions),
                  underlying_offers=len(offers), empty_offers=sum(row['data'] == '' for row in offers),
                  action_panics=sum(bool(row.get('panic')) for row in actions),
                  constructor_panics=sum(bool(row.get('constructor_panic')) for row in observed['cases']),
                  returned_errors=sum(bool(row.get('error')) for row in actions),
                  observer_limit_errors=sum((row.get('error') or {}).get('code') == 'observer-limit' for row in actions),
                  artifact_bytes=bytes_used(work), capture_complete=True, production_adopted=False)
    save()
    print(json.dumps({key: report[key] for key in ['cases', 'actions', 'underlying_offers', 'empty_offers', 'action_panics', 'constructor_panics', 'returned_errors', 'artifact_bytes']}), flush=True)


if __name__ == '__main__':
    main()
