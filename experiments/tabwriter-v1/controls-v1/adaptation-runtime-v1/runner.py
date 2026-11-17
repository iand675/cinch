#!/usr/bin/env python3
"""Execute one source-positive frozen tabwriter consumer with matched tools and fixed ownership profiles."""
from pathlib import Path
import argparse
import json
import os
import resource
import shutil
import signal
import subprocess
import time
import check


def main():
    """Charge source and runtime outputs together, stop at the first failure, and never infer package promotion."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--checked', type=Path, required=True)
    parser.add_argument('--work', type=Path, required=True)
    parser.add_argument('--deadline', type=int, default=60)
    parser.add_argument('--c-support', type=Path, help='Optional independent fixture support, copied and hashed in this new epoch')
    args = parser.parse_args()
    os.sched_setaffinity(0, {2})
    checked = args.checked.resolve()
    receipt = checked / 'report.json'
    source = json.loads(receipt.read_text())
    assert source['source_gate_passed'] and source['compiler_sha256'] == check.COMPILER
    assert source['runtime_sha256'] == check.RUNTIME and source['compiler_source'] == check.SOURCE
    assert check.digest(checked / 'Main.cinch') == source['main_sha256']
    for name, row in source['sources'].items():
        assert check.digest(checked / (name + '.cinch')) == row['sha256']
    assert shutil.disk_usage(check.ROOT).free >= 350 * check.MIB, '350MiB fresh-start guard'
    work = args.work.resolve()
    assert not work.exists(), 'never overwrite a runtime epoch'
    project = work / 'project'
    project.mkdir(parents=True)
    for path in checked.glob('*.cinch'):
        shutil.copyfile(path, project / path.name)
    support = None
    if args.c_support:
        support = work / 'fixture.c'
        shutil.copyfile(args.c_support.resolve(), support)
    shutil.copyfile(checked / 'embedded-family.cinchpkg', project / 'embedded-family.cinchpkg')
    compiler, runtime = check.SNAPSHOT / 'cinch', check.SNAPSHOT / 'libcinchruntime.a'
    assert check.digest(compiler) == check.COMPILER and check.digest(runtime) == check.RUNTIME
    clang = Path(shutil.which('clang')).resolve()
    version = subprocess.run([str(clang), '--version'], capture_output=True, text=True, check=True).stdout
    assert 'clang version 20.' in version
    source_bytes = sum(p.stat().st_size for p in checked.rglob('*') if p.is_file())
    report = {'source_report_sha256': check.digest(receipt), 'compiler_sha256': check.COMPILER,
              'runtime_sha256': check.RUNTIME, 'compiler_source': check.SOURCE,
              'provenance_sha256': check.digest(check.SNAPSHOT / 'provenance.json'),
              'clang': str(clang), 'clang_version': version, 'source_artifact_bytes_charged': source_bytes,
              'sources': {p.name: check.digest(p) for p in project.glob('*.cinch')},
              'cpu': 2, 'steps': [], 'runs': [], 'production_adopted': False,
              'cap_bytes': 20 * check.MIB, 'start_guard_bytes': 350 * check.MIB, 'reserve_bytes': 250 * check.MIB}
    if support:
        report['fixture_c_sha256'] = check.digest(support)

    def save():
        """Record exact tool outcomes and memory profiles before checking a possibly failed result."""
        (work / 'report.json').write_text(json.dumps(report, indent=2) + '\n')

    def used():
        """Count this unit's source, cache, IR, executable and diagnostic artifacts together."""
        return source_bytes + sum(p.stat().st_size for p in work.rglob('*') if p.is_file())

    def phase(command, name, environment, runtime_profile=False):
        """Run one process group under reserve/output/deadline checks, retaining every first failure."""
        assert shutil.disk_usage(work).free >= 250 * check.MIB, '250MiB pause guard'
        assert used() < 20 * check.MIB, '20MiB complete unit output cap'
        allowance = 20 * check.MIB - used() - 65536

        def limits():
            """Separate the compiler host stack from the fixed consumer-managed stack and bound each output file."""
            resource.setrlimit(resource.RLIMIT_STACK, (64 * check.MIB, resource.getrlimit(resource.RLIMIT_STACK)[1]))
            resource.setrlimit(resource.RLIMIT_FSIZE, (allowance, resource.getrlimit(resource.RLIMIT_FSIZE)[1]))

        started, reason = time.monotonic(), None
        out_path, err_path = work / (name + '.stdout'), work / (name + '.stderr')
        with out_path.open('wb') as out, err_path.open('wb') as err:
            child = subprocess.Popen(command, cwd=project, env=environment, stdout=out, stderr=err,
                                     preexec_fn=limits, start_new_session=True)
            while child.poll() is None:
                if time.monotonic() - started > args.deadline:
                    reason = 'deadline without completed verdict'
                elif shutil.disk_usage(work).free < 250 * check.MIB or used() >= 20 * check.MIB:
                    reason = 'reserve or complete unit output cap reached'
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
        row = {'name': name, 'command': command, 'returncode': code, 'interrupted_reason': reason,
               'seconds': time.monotonic() - started, 'stdout': out_path.read_text(), 'stderr': err_path.read_text()}
        (report['runs'] if runtime_profile else report['steps']).append(row)
        save()
        assert code == 0 and reason is None, 'first failure retained: ' + name
        if runtime_profile:
            assert row['stdout'] == '' and 'live=0 immortal=0 leaked=0' in row['stderr'], 'ownership or observation failure: ' + name
        assert used() <= 20 * check.MIB and shutil.disk_usage(work).free >= 250 * check.MIB

    ir, executable = work / 'consumer.ll', work / 'consumer'
    save()
    host = {**os.environ, 'CINCH_HEAP_LIMIT': '2G', 'CINCH_RC_DEBUG': '0', 'CINCH_LEAK_CHECK': '1'}
    phase([str(compiler), 'emit-ir', str(project / 'Main.cinch'), '-o', str(ir), '--rc-debug'], 'compile', host)
    support_args = [str(support)] if support else []
    phase([str(clang), '-O2', str(ir)] + support_args + [str(runtime), '-ldl', '-lpthread', '-lm', '-o', str(executable)], 'link', os.environ)
    report.update(llvm_sha256=check.digest(ir), executable_sha256=check.digest(executable))
    for poison in (False, True):
        environment = {**os.environ, 'CINCH_HEAP_LIMIT': '256M' if poison else '32M', 'CINCH_STACK_SIZE': '8M',
                       'CINCH_RC_DEBUG': '1' if poison else '0', 'CINCH_LEAK_CHECK': '1', 'CINCH_ALLOC_STATS': '1'}
        phase([str(executable)], 'poison' if poison else 'normal', environment, True)
    report.update(all_passed=True, complete_artifact_bytes=used())
    save()
    print('Exact source-positive consumer passed both fixed ownership profiles', flush=True)


if __name__ == '__main__':
    main()
