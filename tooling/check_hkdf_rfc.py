#!/usr/bin/env python3
"""Execute one SDK-free fixed-vector HKDF unit against an exact Rust archive.

The unchanged fixed expectations and actual independent reference validation
must already be recorded. Source checks precede LLVM emission and execution.
"""
from __future__ import annotations
import argparse
import json
import os
from pathlib import Path
import re
import resource
import shutil
import signal
import subprocess
import time
from bench_stdlib import source_closure
from check_hkdf_source import digest, inventory

#: Owned candidate and fixture location; no production source is selected.
ROOT = Path(__file__).resolve().parents[1]


def child_stack() -> None:
    """Apply the independent eight-MiB consumer host stack in addition to runtime configuration."""
    resource.setrlimit(resource.RLIMIT_STACK, (8 * 1024**2, resource.getrlimit(resource.RLIMIT_STACK)[1]))


def main() -> None:
    """Run a single fresh bounded functional epoch and preserve every first failure plainly."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--compiler', type=Path, required=True)
    parser.add_argument('--runtime', type=Path, required=True)
    parser.add_argument('--reference-report', type=Path, required=True)
    parser.add_argument('--work', type=Path, required=True)
    parser.add_argument('--cpu', type=int, default=1)
    parser.add_argument('--opt', choices=('0','2'), default='2')
    args = parser.parse_args()
    work, compiler, runtime = args.work.resolve(), args.compiler.resolve(), args.runtime.resolve()
    assert not work.exists(), 'never overwrite an actual source/functional epoch'
    assert digest(compiler) == 'ffb5d39c42d0446a912b6027ebbe88bd825f5c6d91065aeb8a2048526c4999b9'
    assert digest(runtime) == 'bdf524d891ad27d2f36676ef3133b22f10b196d40ea97c08cc88787b07cc1cf3'
    source = ROOT / 'experiments/hkdf-v1/sources/Crypto.Hkdf.cinch'
    assert digest(source) == '401c215a903b74574c0d88605f1259e5e1236621422d548ebb658c3f2148d213'
    rfc = ROOT / 'experiments/hkdf-v1/rfc5869-vectors.json'
    assert digest(rfc) == 'b08c9ad3925c3e5757d3b3d6bd86942a091274ca133b2812c5e51a79fd9a14bb'
    reference = json.loads(args.reference_report.read_text())
    assert reference['status'] == 'passed' and reference['checked_operations'] == 381
    if shutil.disk_usage(ROOT).free < 350 * 1024**2:
        raise SystemExit('350MiB runtime-unit start guard; no compiler launched')
    os.sched_setaffinity(0, {args.cpu})
    resource.setrlimit(resource.RLIMIT_STACK, (67108864, resource.getrlimit(resource.RLIMIT_STACK)[1]))
    work.mkdir(parents=True)
    consumer = ROOT / 'experiments/hkdf-v1/rfc-consumer.cinch'
    sources = source_closure(consumer.read_text(), {'Crypto.Hkdf': source})
    assert 'Data.Bytes' not in sources
    for name, path in sources.items():
        shutil.copyfile(path, work / (name + '.cinch'))
    shutil.copyfile(consumer, work / 'Main.cinch')
    report = {'scope': __doc__, 'source_sha256': digest(source), 'compiler_sha256': digest(compiler),
              'runtime_sha256': digest(runtime), 'rfc_sha256': digest(rfc),
              'consumer_sha256': digest(consumer), 'runner_sha256': digest(Path(__file__)),
              'reference_report': str(args.reference_report.resolve()), 'reference_report_sha256': digest(args.reference_report),
              'source_closure': {name: {'path': str(path), 'sha256': digest(path)} for name, path in sources.items()},
              'cpu': args.cpu, 'compiler_host_stack_bytes': 67108864, 'consumer_host_stack_bytes': 8388608,
              'optimization': args.opt, 'fixed_rfc_rows': 8, 'complete_output_observations': 60,
              'sdk_required_by_unit': False, 'elapsed_performance_claim': False,
              'commands': [], 'profiles': [], 'runtime_claim': False}
    host = {**os.environ, 'RUST_MIN_STACK': '67108864', 'CINCH_STACK_SIZE': '64M'}

    def save() -> None:
        """Persist exact phase results, all original diagnostics and resource accounting."""
        report.update(free_bytes=shutil.disk_usage(work).free,
                      unarchived_output_bytes=sum(path.stat().st_size for path in work.rglob('*') if path.is_file()))
        (work / 'report.json').write_text(json.dumps(report, indent=2) + '\n')

    def execute(command: list[str], label: str, env: dict, seconds: int, runtime_stage: bool = False) -> dict:
        """Enforce the reserve and output cap throughout one bounded serial command."""
        save()
        if report['free_bytes'] < 250*1024**2 or report['unarchived_output_bytes'] >= 30*1024**2:
            raise RuntimeError('pre-phase250MiB reserve/30MiB output cap')
        row = {'command': command, 'stdout': label+'.stdout', 'stderr': label+'.stderr'}
        report['commands'].append(row)
        started, reason = time.monotonic(), None
        with (work / row['stdout']).open('wb') as stdout, (work / row['stderr']).open('wb') as stderr:
            process = subprocess.Popen(command, cwd=work, stdout=stdout, stderr=stderr, env=env,
                                       start_new_session=True, preexec_fn=child_stack if runtime_stage else None)
            while process.poll() is None:
                save()
                if time.monotonic() - started >= seconds:
                    reason = 'phase deadline'
                elif report['free_bytes'] < 250*1024**2:
                    reason = '250MiB free reserve'
                elif report['unarchived_output_bytes'] >= 30*1024**2:
                    reason = '30MiB fresh-output cap'
                if reason:
                    os.killpg(process.pid, signal.SIGTERM)
                    try:
                        process.wait(timeout=1)
                    except subprocess.TimeoutExpired:
                        os.killpg(process.pid, signal.SIGKILL)
                        process.wait()
                    break
                time.sleep(0.05)
            row.update(returncode=process.wait(), resource_stop=reason)
        save()
        if row['returncode'] or reason:
            raise RuntimeError('first genuine source/functional outcome: ' + label)
        return row

    try:
        execute([str(compiler), 'check', str(work / 'Main.cinch')], 'source', host, 15)
        execute([str(compiler), 'emit-ir', str(work / 'Main.cinch'), '-o', str(work / 'consumer.ll')], 'emit', host, 30)
        report['ir_sha256'] = digest(work / 'consumer.ll')
        triple = re.search(r'^target triple = "([^"]+)"', (work / 'consumer.ll').read_text(), re.M)
        if not triple:
            raise RuntimeError('missing actual target triple')
        clang = os.environ.get('CINCH_CLANG', 'clang')
        execute([clang, '-O'+args.opt, '--target='+triple.group(1), str(work / 'consumer.ll'),
                 str(runtime), '-ldl', '-lpthread', '-lm', '-o', str(work / 'consumer')], 'link', host, 30)
        report['executable_sha256'] = digest(work / 'consumer')
        for label, poison, heap in [('normal','0','32M'), ('poison','1','256M')]:
            env = {**os.environ, 'CINCH_HEAP_LIMIT': heap, 'CINCH_STACK_SIZE': '8M',
                   'CINCH_LEAK_CHECK':'1', 'CINCH_RC_DEBUG':poison, 'CINCH_ALLOC_STATS':'0'}
            row = execute([str(work / 'consumer')], label, env, 15, True)
            diagnostics = (work / row['stderr']).read_text()
            if 'leaked=0' not in diagnostics:
                raise RuntimeError('missing zero-leak receipt')
            report['profiles'].append({'label':label,'heap':heap,'poison':poison,'stack':'8M',
                                       'returncode':row['returncode'],'stderr':diagnostics})
        report.update(status='passed', runtime_claim=True)
    except BaseException as error:
        report.update(status='failed-or-resource-stopped', failure=repr(error), all_failure_artifacts_plain=True)
        raise
    finally:
        report['files'] = inventory(work)
        save()
    print('8 fixed RFC rows /60 complete outputs pass both memory profiles with zero leaks')


if __name__ == '__main__':
    main()
