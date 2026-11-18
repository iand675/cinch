#!/usr/bin/env python3
"""Relink exact accepted fixed-palette IR at another backend optimization without re-emission."""
from __future__ import annotations
import argparse
import json
import os
from pathlib import Path
import shutil
import subprocess
import time
from check_compare import digest


def main() -> None:
    """Verify all original identities, retain failed outputs and execute both independent profiles."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--accepted-report', type=Path, required=True)
    parser.add_argument('--work', type=Path, required=True)
    parser.add_argument('--cpu', type=int, required=True)
    parser.add_argument('--opt', type=int, choices=(0, 2), required=True)
    args = parser.parse_args()
    original = args.accepted_report.resolve()
    accepted = json.loads(original.read_text())
    if accepted.get('status') != 'passed':
        raise RuntimeError('successful accepted runtime report required')
    base = original.parent
    inputs = [(name+'.cinch', checksum) for name, checksum in accepted['source_closure_sha256'].items()] + [('Main.cinch', accepted['consumer_sha256']),
              ('vectors.json', accepted['vectors_sha256']), ('consumer.ll', accepted['ir_sha256']),
              ('fixture.c', accepted['foreign_fixture']['fixture_sha256']), ('fixture.o', accepted['fixture_object_sha256'])]
    for name, expected in inputs:
        if digest(base / name) != expected:
            raise RuntimeError('changed accepted source/code/object: ' + name)
    runtime, compiler = Path(accepted['runtime']), Path(accepted['compiler'])
    if digest(runtime) != accepted['runtime_sha256'] or digest(compiler) != accepted['compiler_sha256']:
        raise RuntimeError('changed frozen compiler/runtime epoch')
    work = args.work.resolve()
    work.mkdir(parents=True, exist_ok=False)
    initial = shutil.disk_usage(work).free
    if initial < 350*1024*1024:
        raise RuntimeError('350 MiB initial relink guard; no compiler started')
    os.sched_setaffinity(0, {args.cpu})
    report = {'scope': __doc__, 'accepted_report': str(original), 'accepted_report_sha256': digest(original),
              'compiler': str(compiler), 'compiler_sha256': accepted['compiler_sha256'],
              'runtime': str(runtime), 'runtime_sha256': accepted['runtime_sha256'],
              'source_sha256': accepted['source_sha256'], 'source_closure_sha256': accepted['source_closure_sha256'],
              'consumer_sha256': accepted['consumer_sha256'],
              'vectors_sha256': accepted['vectors_sha256'], 'checked_ir_sha256': accepted['ir_sha256'],
              'fixture_object_sha256': accepted['fixture_object_sha256'], 'opt': args.opt,
              'compiler_reemission': False, 'cpu_affinity': [args.cpu],
              'compiler_host_stack_bytes': accepted['compiler_host_stack_bytes'],
              'runner_sha256': digest(Path(__file__)), 'initial_free_bytes': initial,
              'resource_policy': {'initial_mib': 350, 'reserve_mib': 250, 'unit_mib': 30},
              'commands': [], 'profiles': []}

    def execute(command: list[str], env: dict) -> dict:
        """Preserve the actual first failing process and all diagnostics before stopping."""
        if shutil.disk_usage(work).free < 250*1024*1024 or sum(p.stat().st_size for p in work.rglob('*') if p.is_file()) > 30*1024*1024:
            raise RuntimeError('250 MiB reserve /30 MiB output guard')
        row = {'command': command}
        report['commands'].append(row)
        started = time.monotonic()
        try:
            result = subprocess.run(command, cwd=work, text=True, capture_output=True, timeout=30, env=env)
            row.update(returncode=result.returncode, stdout=result.stdout, stderr=result.stderr)
        except subprocess.TimeoutExpired as error:
            row.update(timeout_seconds=30, failure=repr(error))
            raise
        finally:
            row['seconds'] = time.monotonic()-started
            (work / 'report.json').write_text(json.dumps(report, indent=2)+'\n')
        if row['returncode']:
            raise RuntimeError('first relink/runtime failure: ' + repr(row))
        return row

    try:
        executable = work / 'consumer'
        clang = os.environ.get('CINCH_CLANG', 'clang')
        execute([clang, '-O'+str(args.opt), '--target='+accepted['target_triple'], str(base/'consumer.ll'),
                 str(base/'fixture.o'), str(runtime), '-ldl', '-lpthread', '-lm', '-o', str(executable)], dict(os.environ))
        report['executable_sha256'] = digest(executable)
        for poison, heap in [('0','32M'), ('1','256M')]:
            row = execute([str(executable)], {**os.environ, 'CINCH_HEAP_LIMIT': heap, 'CINCH_STACK_SIZE': '8M',
                          'CINCH_RC_DEBUG': poison, 'CINCH_LEAK_CHECK': '1', 'CINCH_ALLOC_STATS': '0'})
            report['profiles'].append({'poison': poison, 'heap': heap, 'stack': '8M',
                                       'stdout': row['stdout'], 'stderr': row['stderr']})
            if 'leaked=0' not in row['stderr']:
                raise RuntimeError('missing zero-leak relink evidence')
        report['status'] = 'passed'
    except BaseException as error:
        report.update(status='failed-or-resource-paused', failure=repr(error), failed_artifacts_retained_plain=True)
        raise
    finally:
        report.update(final_free_bytes=shutil.disk_usage(work).free,
                      output_bytes=sum(path.stat().st_size for path in work.rglob('*') if path.is_file()))
        (work / 'report.json').write_text(json.dumps(report, indent=2)+'\n')
    print(report['status'], 'exact accepted IR atO'+str(args.opt), report['output_bytes'])


if __name__ == '__main__':
    main()
