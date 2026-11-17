#!/usr/bin/env python3
"""Execute the public fixed-palette corpus and retained-owner controls with an exact Rust runtime archive."""
from __future__ import annotations
import argparse
import json
import os
from pathlib import Path
import re
import resource
import shutil
import subprocess
from bench_stdlib import source_closure
from check_compare import digest
from prepare_color_palette_fixtures import prepare


def main() -> None:
    """Check source first, then execute independent fixture observations in both memory profiles."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--source', type=Path, required=True)
    parser.add_argument('--color', type=Path, required=True)
    parser.add_argument('--consumer', type=Path, required=True)
    parser.add_argument('--vectors', type=Path, required=True)
    parser.add_argument('--compiler', type=Path, required=True)
    parser.add_argument('--runtime', type=Path, required=True)
    parser.add_argument('--work', type=Path, required=True)
    parser.add_argument('--cpu', type=int, default=1)
    parser.add_argument('--opt', type=int, choices=[0, 2], default=2)
    parser.add_argument('--check-only', action='store_true')
    args = parser.parse_args()
    work = args.work.resolve()
    work.mkdir(parents=True, exist_ok=False)
    initial = shutil.disk_usage(work).free
    if initial < 350*1024*1024:
        raise RuntimeError('350MiB initial functional-unit guard')
    os.sched_setaffinity(0, {args.cpu})
    resource.setrlimit(resource.RLIMIT_STACK, (67108864, resource.getrlimit(resource.RLIMIT_STACK)[1]))
    compiler, runtime = args.compiler.resolve(), args.runtime.resolve()
    clang = os.environ.get('CINCH_CLANG', 'clang')
    report = {'scope': __doc__, 'runner_sha256': digest(Path(__file__)), 'compiler': str(compiler),
              'compiler_sha256': digest(compiler), 'runtime': str(runtime), 'runtime_sha256': digest(runtime),
              'source_sha256': digest(args.source), 'color_sha256': digest(args.color),
              'consumer_sha256': digest(args.consumer), 'vectors_sha256': digest(args.vectors),
              'cpu_affinity': [args.cpu], 'compiler_host_stack_bytes': 67108864,
              'consumer_optimization': args.opt, 'initial_free_bytes': initial,
              'profiles': [], 'commands': [], 'runtime_claim': False}

    def save() -> None:
        """Retain every completed command and any first failure in its unchanged source project."""
        report.update(final_free_bytes=shutil.disk_usage(work).free,
                      unarchived_output_bytes=sum(path.stat().st_size for path in work.rglob('*') if path.is_file()))
        (work / 'report.json').write_text(json.dumps(report, indent=2)+'\n')

    def execute(command: list[str], env: dict) -> dict:
        """Enforce the reserve/output cap at each phase and preserve exact diagnostics."""
        save()
        if report['final_free_bytes'] < 250*1024*1024 or report['unarchived_output_bytes'] > 30*1024*1024:
            raise RuntimeError('250MiB reserve/30MiB output cap')
        row = {'command': command}
        report['commands'].append(row)
        try:
            result = subprocess.run(command, cwd=work, env=env, text=True, capture_output=True, timeout=45)
            row.update(returncode=result.returncode, stdout=result.stdout, stderr=result.stderr)
        except BaseException as error:
            row['failure'] = repr(error)
            raise
        finally:
            save()
        if row['returncode']:
            raise RuntimeError('first source/fixture/consumer failure')
        if report['final_free_bytes'] < 250*1024*1024 or report['unarchived_output_bytes'] > 30*1024*1024:
            raise RuntimeError('post-phase250MiB reserve/30MiB output cap')
        return row

    try:
        caller = args.consumer.read_text()
        sources = source_closure(caller, {'Image.Color': args.color, 'Image.Color.Palette': args.source})
        for name, source in sources.items():
            shutil.copyfile(source, work / (name+'.cinch'))
        shutil.copyfile(args.consumer, work / 'Main.cinch')
        shutil.copyfile(args.vectors, work / 'vectors.json')
        report['source_closure_sha256'] = {name: digest(work / (name+'.cinch')) for name in sources}
        host = {**os.environ, 'RUST_MIN_STACK': '67108864', 'CINCH_STACK_SIZE': '64M'}
        execute([str(compiler), 'check', str(work / 'Main.cinch')], host)
        report['defining_interfaces'] = {path.name: digest(path) for path in (work / 'build').glob('*.cinchif')}
        if args.check_only:
            report['status'] = 'source-check-passed'
            return
        report['foreign_fixture'] = prepare(args.vectors, work / 'fixture.c')
        execute([str(compiler), 'emit-ir', str(work / 'Main.cinch'), '-o', str(work / 'consumer.ll')], host)
        report['ir_sha256'] = digest(work / 'consumer.ll')
        target = re.search(r'^target triple = "([^"]+)"', (work / 'consumer.ll').read_text(), re.M)
        if target is None:
            raise RuntimeError('missing actual emitted target triple')
        triple = target.group(1)
        report['target_triple'] = triple
        execute([clang, '-O2', '--target='+triple, '-fno-vectorize', '-fno-slp-vectorize', '-c',
                 str(work / 'fixture.c'), '-o', str(work / 'fixture.o')], host)
        report['fixture_object_sha256'] = digest(work / 'fixture.o')
        execute([clang, '-O'+str(args.opt), '--target='+triple, str(work / 'consumer.ll'), str(work / 'fixture.o'),
                 str(runtime), '-ldl', '-lpthread', '-lm', '-o', str(work / 'consumer')], host)
        report['executable_sha256'] = digest(work / 'consumer')
        for poison, heap in [('0', '32M'), ('1', '256M')]:
            env = {**os.environ, 'CINCH_HEAP_LIMIT': heap, 'CINCH_STACK_SIZE': '8M',
                   'CINCH_LEAK_CHECK': '1', 'CINCH_RC_DEBUG': poison, 'CINCH_ALLOC_STATS': '0'}
            row = execute([str(work / 'consumer')], env)
            report['profiles'].append({'poison': poison, 'heap': heap, 'stack': '8M', 'stdout': row['stdout'], 'stderr': row['stderr']})
            if 'leaked=0' not in row['stderr']:
                raise RuntimeError('missing zero-leak evidence')
        report.update(status='passed', runtime_claim=True)
    except BaseException as error:
        report.update(status='failed-or-resource-paused', failure=repr(error), failed_artifacts_retained_plain=True)
        raise
    finally:
        save()
    print('all472 entries/1024 queries and escaped factory owners pass both memory profiles')


if __name__ == '__main__':
    main()
