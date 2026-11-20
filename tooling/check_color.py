#!/usr/bin/env python3
"""Execute committed public color/model/palette fixtures with an explicit compiler and its matching runtime."""
from __future__ import annotations
import argparse
import json
import os
from pathlib import Path
import re
import resource
import shutil
import subprocess
import time
from check_compare import corpus_emit_mode, digest
import prepare_color_fixtures
import prepare_color_palette_fixtures


#: Repository root; every fixture input is committed and needs no installed SDK.
ROOT = Path(__file__).resolve().parents[1]


def main() -> None:
    """Run source first, retain first failures and distinguish O0/O2 and both heap profiles."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--source', type=Path, required=True)
    parser.add_argument('--palette', type=Path, help='Include the fixed-palette module and its table/query fixtures')
    parser.add_argument('--consumer', type=Path)
    parser.add_argument('--vectors', type=Path)
    frontend = parser.add_mutually_exclusive_group(required=True)
    frontend.add_argument('--compiler', type=Path)
    frontend.add_argument('--native', type=Path, help='Standalone stage compiler and matching provider manifests')
    parser.add_argument('--runtime', type=Path)
    parser.add_argument('--work', type=Path, required=True)
    parser.add_argument('--cpu', type=int, default=min(os.sched_getaffinity(0)))
    parser.add_argument('--opt', type=int, choices=(0, 2), default=2)
    parser.add_argument('--check-only', action='store_true')
    args = parser.parse_args()
    if bool(args.compiler) != bool(args.runtime):
        parser.error('--compiler requires a matching --runtime')
    experiment = ROOT / 'experiments' / ('image-color-palette-v1' if args.palette else 'image-color-v1')
    args.consumer = args.consumer or experiment / 'consumer.cinch'
    args.vectors = args.vectors or experiment / 'vectors.json'
    fixtures = prepare_color_palette_fixtures if args.palette else prepare_color_fixtures
    work = args.work.resolve()
    work.mkdir(parents=True, exist_ok=False)
    initial = shutil.disk_usage(work).free
    if initial < 350 * 1024 * 1024:
        (work / 'resource-pause.json').write_text(json.dumps({'phase': 'before any compiler/output preparation',
            'free_bytes': initial, 'required_initial_bytes': 350*1024*1024, 'compiler_started': False}) + '\n')
        raise RuntimeError('350 MiB initial guard')
    os.sched_setaffinity(0, {args.cpu})
    resource.setrlimit(resource.RLIMIT_STACK, (67108864, resource.getrlimit(resource.RLIMIT_STACK)[1]))
    snapshot = args.native.resolve() if args.native else None
    compiler = snapshot / 'cinch' if snapshot else args.compiler.resolve()
    runtime = snapshot / 'libcinchruntime.a' if snapshot else args.runtime.resolve()
    report = {'scope': __doc__, 'compiler': str(compiler), 'compiler_sha256': digest(compiler),
              'runtime': str(runtime), 'runtime_sha256': digest(runtime), 'opt': args.opt,
              'compiler_host_stack_bytes': 67108864, 'cpu_affinity': [args.cpu],
              'runner_sha256': digest(Path(__file__)), 'fixture_generator_sha256': digest(Path(fixtures.__file__)),
              'source_sha256': digest(args.source), 'consumer_sha256': digest(args.consumer),
              'vectors_sha256': digest(args.vectors), 'initial_free_bytes': initial,
              'resource_policy': {'initial_mib': 350, 'reserve_mib': 250, 'unit_mib': 30},
              'compile_rc_debug': False, 'runtime_claim': False, 'commands': [], 'profiles': []}

    def save() -> None:
        """Retain exact partial commands, outputs and identities on every outcome boundary."""
        report.update(current_free_bytes=shutil.disk_usage(work).free,
                      unarchived_output_bytes=sum(path.stat().st_size for path in work.rglob('*') if path.is_file()))
        (work / 'report.json').write_text(json.dumps(report, indent=2) + '\n')

    def execute(command: list[str], env: dict[str, str], timeout: int = 30) -> dict:
        """Record failed diagnostics before stopping; no archive or artifact deletion occurs here."""
        save()
        if report['current_free_bytes'] < 250 * 1024 * 1024 or report['unarchived_output_bytes'] > 30 * 1024 * 1024:
            raise RuntimeError('250 MiB reserve /30 MiB output guard')
        row = {'command': command}
        report['commands'].append(row)
        started = time.monotonic()
        try:
            completed = subprocess.run(command, cwd=work, text=True, capture_output=True, timeout=timeout, env=env)
            row.update(returncode=completed.returncode, stdout=completed.stdout, stderr=completed.stderr)
            if completed.returncode and snapshot and command[0] == str(compiler):
                diagnostics = work / 'consumer.ll.diagnostics'
                if diagnostics.is_file():
                    row['diagnostics'] = diagnostics.read_text()
        except subprocess.TimeoutExpired as error:
            row.update(timeout_seconds=timeout, failure=repr(error))
            raise
        finally:
            row['seconds'] = time.monotonic() - started
            save()
        if row['returncode'] != 0:
            raise RuntimeError('first failed color command: ' + repr(row))
        if report['current_free_bytes'] < 250 * 1024 * 1024 or report['unarchived_output_bytes'] > 30 * 1024 * 1024:
            raise RuntimeError('post-command250 MiB reserve /30 MiB output guard')
        return row

    try:
        shutil.copyfile(args.source, work / 'Image.Color.cinch')
        modules = [work / 'Image.Color.cinch']
        if args.palette:
            palette = work / 'Image.Color.Palette.cinch'
            shutil.copyfile(args.palette, palette)
            report['palette_sha256'] = digest(palette)
            modules.append(palette)
        shutil.copyfile(args.consumer, work / 'Main.cinch')
        shutil.copyfile(args.vectors, work / 'vectors.json')
        host_env = {**os.environ, 'RUST_MIN_STACK': '67108864', 'CINCH_STACK_SIZE': '64M'}
        link_objects = []
        if snapshot:
            objects = snapshot / 'objects.json'
            interfaces = snapshot / 'base-interfaces.json'
            configuration = json.loads(objects.read_text())
            provider = Path(configuration['object'])
            identity = {'compiler_sha256': report['compiler_sha256'], 'runtime_sha256': report['runtime_sha256'],
                        'provider_sha256': digest(provider), 'base-interfaces.json': digest(interfaces),
                        'objects.json': digest(objects)}
            if identity['provider_sha256'] != configuration['object_sha256']:
                raise RuntimeError('color provider hash disagrees with its object manifest')
            report['frontends'] = {'native': identity}
            bundle = work / 'bundle.source'
            bundle.write_bytes(b'\n'.join(path.read_bytes() for path in [*modules, work / 'Main.cinch']))
            if args.check_only:
                command = [str(compiler), '--check', '--interfaces', str(interfaces)]
            else:
                mode = '--emit-ir-with-interfaces' if args.palette else corpus_emit_mode(compiler, [])
                command = [str(compiler), mode, '--interfaces', str(interfaces), '--objects', str(objects)]
            command.extend([str(bundle), str(work / 'consumer.ll')])
            row = execute(command, host_env, timeout=300)
            if not args.check_only:
                report['runs'] = [{'frontend': 'native', 'command': command, 'seconds': row['seconds'],
                                   'ir_sha256': digest(work / 'consumer.ll')}]
            if args.palette and not args.check_only:
                report['defining_interfaces'] = {path.stem: digest(work / f'consumer.ll.{path.stem}.cinchif')
                                                 for path in modules}
            link_objects.append(str(provider))
        else:
            execute([str(compiler), 'check', str(work / 'Main.cinch')], host_env)
            report['defining_interfaces'] = {path.name: digest(path) for path in sorted((work / 'build').glob('*.cinchif'))}
        if args.check_only:
            report['status'] = 'source-check-passed'
            return
        report['foreign_fixture'] = fixtures.prepare(args.vectors, work / 'fixture.c')
        clang = os.environ.get('CINCH_CLANG', 'clang')
        execute([clang, '-O2', '-fno-vectorize', '-fno-slp-vectorize', '-c', str(work / 'fixture.c'), '-o', str(work / 'fixture.o')], host_env)
        if not snapshot:
            execute([str(compiler), 'emit-ir', str(work / 'Main.cinch'), '-o', str(work / 'consumer.ll')], host_env)
        report.update(ir_sha256=digest(work / 'consumer.ll'), fixture_object_sha256=digest(work / 'fixture.o'))
        triple = re.search(r'^target triple = "([^"]+)"', (work / 'consumer.ll').read_text(), re.M)
        if triple:
            report['target_triple'] = triple.group(1)
        elif snapshot:
            report['target_triple'] = configuration['target']
        else:
            raise RuntimeError('missing actual emitted target triple')
        execute([clang, '-O' + str(args.opt), '--target=' + report['target_triple'], str(work / 'consumer.ll'), str(work / 'fixture.o'),
                 *link_objects, str(runtime), '-ldl', '-lpthread', '-lm', '-o', str(work / 'consumer')], host_env)
        report['executable_sha256'] = digest(work / 'consumer')
        for poison, heap in [('0', '32M'), ('1', '256M')]:
            env = {**os.environ, 'CINCH_HEAP_LIMIT': heap, 'CINCH_STACK_SIZE': '8M',
                   'CINCH_RC_DEBUG': poison, 'CINCH_LEAK_CHECK': '1', 'CINCH_ALLOC_STATS': '0'}
            row = execute([str(work / 'consumer')], env)
            profile = {'poison': poison, 'heap': heap, 'stack': '8M', 'stdout': row['stdout'], 'stderr': row['stderr']}
            report['profiles'].append(profile)
            if 'leaked=0' not in row['stderr']:
                raise RuntimeError('missing zero-leak evidence')
        report.update(runtime_claim=True, status='passed')
    except BaseException as error:
        report.update(status='failed-or-resource-paused', failure=repr(error), failed_artifacts_retained_plain=True)
        raise
    finally:
        save()
    print(report['status'], report['foreign_fixture'], report['unarchived_output_bytes'])


if __name__ == '__main__':
    main()
