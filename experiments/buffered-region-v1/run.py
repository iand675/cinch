#!/usr/bin/env python3
"""Run one exact source-admitted Region consumer with matched tools and explicit retained-cycle failure reporting."""
from pathlib import Path
import argparse
import json
import os
import re
import resource
import shutil
import subprocess
import time
import check


def main():
    """Keep source admission, runtime identity, disk budget and the two fixed consumer profiles independent."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--checked', type=Path, required=True)
    parser.add_argument('--work', type=Path, required=True)
    parser.add_argument('--deadline', type=int, default=90)
    parser.add_argument('--retained-cycle-control', action='store_true')
    parser.add_argument('--start-free-mib', type=int, choices=(320, 350), default=350)
    parser.add_argument('--resource-policy-note', help='Explicit Root token/scope for the temporary320MiB exception; defaults stay unchanged')
    args = parser.parse_args()
    os.sched_setaffinity(0, {2})
    checked = args.checked.resolve()
    receipt_path = checked / 'report.json'
    admitted = json.loads(receipt_path.read_text())
    assert admitted['source_gate_passed'] and admitted['returncode'] == 0
    assert admitted['proposal_sha256'] == check.digest(check.HERE / 'proposal.json')
    assert admitted['compiler_sha256'] == check.COMPILER and admitted['runtime_sha256'] == check.RUNTIME
    assert check.digest(checked / 'Main.cinch') == admitted['main_sha256']
    for name, row in admitted['source_selection'].items():
        assert check.digest(checked / (name + '.cinch')) == row['sha256'], name
    compiler, runtime = check.SNAPSHOT / 'cinch', check.SNAPSHOT / 'libcinchruntime.a'
    assert check.digest(compiler) == check.COMPILER and check.digest(runtime) == check.RUNTIME
    clang = Path(shutil.which('clang')).resolve()
    version = subprocess.run([str(clang), '--version'], capture_output=True, text=True, check=True).stdout
    assert 'clang version 20.' in version, version
    if args.start_free_mib != 350:
        assert args.resource_policy_note, 'temporary resource exception requires its explicit token/scope note'
    free_at_start = shutil.disk_usage(check.ROOT).free
    assert free_at_start >= args.start_free_mib * 1024 * 1024, 'fresh-start disk guard'
    work = args.work.resolve()
    assert not work.exists(), 'refuse to overwrite a runtime epoch'
    project = work / 'project'
    project.mkdir(parents=True)
    for path in checked.glob('*.cinch'):
        shutil.copyfile(path, project / path.name)
    ir, executable = work / 'consumer.ll', work / 'consumer'
    report = {'scope': __doc__, 'source_receipt_sha256': check.digest(receipt_path),
              'compiler_sha256': check.COMPILER, 'runtime_sha256': check.RUNTIME,
              'compiler_source': check.SOURCE, 'clang': str(clang), 'clang_version': version,
              'source_sha256': check.digest(project / 'Main.cinch'),
              'sources': {p.name: check.digest(p) for p in project.glob('*.cinch')},
              'production_adopted': False, 'retained_cycle_control': args.retained_cycle_control,
              'resource_policy': {'start_free_mib': args.start_free_mib, 'reserve_mib': 250,
                                  'free_at_start_bytes': free_at_start, 'explicit_scope': args.resource_policy_note},
              'normal_runtime_collection_added': False, 'steps': [], 'runs': []}

    def save():
        """Retain every actual diagnostic and partial outcome before considering another phase."""
        (work / 'report.json').write_text(json.dumps(report, indent=2) + '\n')

    def guard():
        """Reserve shared free space and cap this unit's complete source, IR, cache, ELF and log outputs."""
        assert shutil.disk_usage(work).free >= 250 * 1024 * 1024, '250MiB pause guard'
        used = sum(p.stat().st_size for p in work.rglob('*') if p.is_file())
        assert used < 20 * 1024 * 1024, '20MiB complete runtime-unit output cap'
        return used

    def invoke(command, name, environment):
        """Bound each external phase, preserve first failure and keep the source-admission epoch immutable."""
        used = guard()
        allowance = 20 * 1024 * 1024 - used - 128 * 1024

        def limits():
            """Use a 64MiB compiler host stack and a strict remaining per-file output ceiling."""
            resource.setrlimit(resource.RLIMIT_STACK, (67108864, resource.getrlimit(resource.RLIMIT_STACK)[1]))
            resource.setrlimit(resource.RLIMIT_FSIZE, (allowance, resource.getrlimit(resource.RLIMIT_FSIZE)[1]))

        before = time.monotonic()
        timed_out = False
        try:
            result = subprocess.run(command, cwd=project, capture_output=True, text=True,
                                    preexec_fn=limits, timeout=args.deadline, env=environment)
            code, stdout, stderr = result.returncode, result.stdout, result.stderr
        except subprocess.TimeoutExpired as error:
            timed_out = True
            code = None
            stdout, stderr = error.stdout or b'', error.stderr or b''
            stdout = stdout.decode() if isinstance(stdout, bytes) else stdout
            stderr = stderr.decode() if isinstance(stderr, bytes) else stderr
        (work / (name + '.stdout')).write_text(stdout)
        (work / (name + '.stderr')).write_text(stderr)
        report['steps'].append({'name': name, 'command': command, 'returncode': code,
                                'stdout': stdout, 'stderr': stderr, 'seconds': time.monotonic() - before,
                                'timeout_without_verdict': timed_out})
        save()
        guard()
        assert not timed_out and code == 0, 'first ' + name + ' failure or timeout retained'

    host = {**os.environ, 'CINCH_HEAP_LIMIT': '2G', 'CINCH_RC_DEBUG': '0', 'CINCH_LEAK_CHECK': '1'}
    invoke([str(compiler), 'emit-ir', str(project / 'Main.cinch'), '-o', str(ir), '--rc-debug'], 'compile', host)
    invoke([str(clang), '-O2', str(ir), str(runtime), '-ldl', '-lpthread', '-lm', '-o', str(executable)], 'link', os.environ)
    report.update(llvm_sha256=check.digest(ir), executable_sha256=check.digest(executable))
    save()
    for poisoned in [False, True]:
        guard()
        env = {**os.environ, 'CINCH_HEAP_LIMIT': '256M' if poisoned else '32M',
               'CINCH_STACK_SIZE': '8M', 'CINCH_RC_DEBUG': '1' if poisoned else '0',
               'CINCH_LEAK_CHECK': '1', 'CINCH_ALLOC_STATS': '1'}
        result = subprocess.run([str(executable)], capture_output=True, text=True, timeout=30, env=env)
        row = {'poisoned': poisoned, 'heap': env['CINCH_HEAP_LIMIT'], 'stack': '8M',
               'returncode': result.returncode, 'stdout': result.stdout, 'stderr': result.stderr}
        report['runs'].append(row)
        save()
        if args.retained_cycle_control:
            leaks = re.search(r'live=(\d+) immortal=(\d+) leaked=(\d+)', result.stderr)
            assert result.stdout == 'retained-cycle-ready\n', 'cycle control did not confirm both exact transfers and EOF'
            assert result.returncode == 74 and leaks and int(leaks[3]) > 0, 'expected retained-cycle failure changed; review actual result'
            row['known_cycle_failure_observed'] = True
        else:
            assert result.returncode == 0 and 'live=0 immortal=0 leaked=0' in result.stderr, 'first runtime failure retained'
        save()
    report['artifact_bytes'] = guard()
    report['all_passed'] = not args.retained_cycle_control
    report['known_cycle_failure_reproduced'] = args.retained_cycle_control
    save()
    print('Retained-cycle failure observed; package promotion remains blocked' if args.retained_cycle_control
          else 'Current Region consumer passed both fixed profiles with zero live/leaks', flush=True)


if __name__ == '__main__':
    main()
