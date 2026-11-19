#!/usr/bin/env python3
"""Run the isolated combined candidate's source, metadata and live regressions on the selected compiler.

The selected compiler and runtime are explicit inputs. CI initializes its host
archive through an ordinary compiler build; frozen standalone gates retain
their exact immutable archive instead. No SDK, Native fallback or production
source substitution is used.
"""
from __future__ import annotations
import argparse
import json
import os
from pathlib import Path
import resource
import subprocess
import sys
from check_compare import digest, initialize_runtime, run
from check_multipart_live_effects import prepare
from inspect_multipart_carriers import inspect, observation_source
from multipart_qualification_support import build_fixture


#: Own repository root; all candidate and fixture inputs remain source controlled.
ROOT = Path(__file__).resolve().parents[1]


def qualify_native(snapshot: Path, work: Path, candidate: Path) -> None:
    """Compile the exact live consumer once, then inspect and execute its defining artifacts."""
    compiler = snapshot / 'cinch'
    interfaces, objects = snapshot / 'base-interfaces.json', snapshot / 'objects.json'
    configuration = json.loads(objects.read_text())
    provider = Path(configuration['object'])
    runtime = snapshot / 'libcinchruntime.a'
    identity = {'compiler_sha256': digest(compiler), 'runtime_sha256': digest(runtime),
                'provider_sha256': digest(provider), 'base-interfaces.json': digest(interfaces),
                'objects.json': digest(objects)}
    if identity['provider_sha256'] != configuration['object_sha256']:
        raise ValueError('multipart provider disagrees with its object manifest')

    project, observation = work / 'source', work / 'metadata'
    project.mkdir()
    observation.mkdir()
    sources = candidate / 'sources'
    gate = prepare(project, sources / 'Effect.Multipart.cinch', sources / 'Text.Multipart.cinch')
    manifest = json.loads((candidate / 'manifest.json').read_text())
    for name in ('Effect.Multipart', 'Text.Multipart'):
        if gate['sources'][name] != manifest['sources'][name + '.cinch']:
            raise ValueError(f'{name} differs from the candidate manifest')
    ir = project / 'consumer.ll'
    command = [str(compiler), '--emit-ir-with-interfaces', '--interfaces', str(interfaces),
               '--objects', str(objects), str(project / 'bundle.source'), str(ir)]
    run(command, project, timeout=600)
    source = (project / 'Effect.Multipart.cinch').read_text()
    changes = manifest['changes']
    public = inspect(json.loads((project / 'consumer.ll.Effect.Multipart.cinchif').read_bytes()),
                     changes, source, True)
    observed, witnesses = observation_source(source, changes)
    # The private observers typecheck against the exact sources used by the live fixture.
    check = observation / 'Main.cinch'
    modules = [observed if name == 'Effect.Multipart' else (project / (name + '.cinch')).read_text()
               for name in gate['sources']]
    check.write_text('\n'.join([*modules, 'module Main where\n  import Effect.Multipart\n  main :: Int\n  main = 0\n']))
    run([compiler, '--check', '--interfaces', interfaces, check, observation / 'checked'], observation, timeout=600)
    private = inspect(json.loads((observation / 'build/Effect.Multipart.cinchif').read_bytes()),
                      changes, source, False)
    metadata = {'public_contracts': public, 'private_contracts': private, 'witnesses': witnesses}
    (observation / 'report.json').write_text(json.dumps(metadata, indent=2))

    clang = os.environ.get('CINCH_CLANG', 'clang')
    fixture = build_fixture(project, clang)
    executable = project / 'consumer'
    run([clang, '-O2', '--target=' + configuration['target'], ir, project / 'fixture.o', project / 'kernels.o',
         provider, runtime, '-ldl', '-lpthread', '-lm', '-o', executable], project, timeout=300)
    profiles = []
    for poison, heap in [('0', '32M'), ('1', '256M')]:
        result = run([executable], project, timeout=120, env={**os.environ, 'CINCH_RC_DEBUG': poison,
                     'CINCH_HEAP_LIMIT': heap, 'CINCH_STACK_SIZE': '8M', 'CINCH_LEAK_CHECK': '1'})
        if 'leaked=0' not in result.stderr.split():
            raise RuntimeError('multipart runtime did not report zero leaks: ' + result.stderr)
        profiles.append({'poison': poison, 'heap': heap, 'stack': '8M',
                         'stdout': result.stdout, 'stderr': result.stderr})
    row = {'frontend': 'native', 'command': command, 'ir_sha256': digest(ir), 'profiles': profiles}
    report = {'compiler_host_stack_bytes': 67108864, **identity, 'frontends': {'native': identity},
              'phases': ['source', 'metadata', 'live-runtime'], 'fixture': fixture, 'runs': [row]}
    (work / 'report.json').write_text(json.dumps(report, indent=2))


def main() -> None:
    """Execute each independently recorded boundary before permitting the next one."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--work', type=Path, required=True)
    selection = parser.add_mutually_exclusive_group(required=True)
    selection.add_argument('--compiler', '--rust', type=Path)
    selection.add_argument('--native', type=Path)
    parser.add_argument('--runtime', '--rust-runtime', type=Path)
    parser.add_argument('--initialize-rust-runtime', action='store_true')
    parser.add_argument('--cpu', type=int)
    args = parser.parse_args()
    if bool(args.compiler) != bool(args.runtime) or (args.native and args.initialize_rust_runtime):
        parser.error('the Rust compiler requires its runtime archive')
    cpu = args.cpu if args.cpu is not None else min(os.sched_getaffinity(0))
    os.sched_setaffinity(0, {cpu})
    resource.setrlimit(resource.RLIMIT_STACK, (67108864, resource.getrlimit(resource.RLIMIT_STACK)[1]))
    work = args.work.resolve(); work.mkdir(parents=True, exist_ok=True)
    candidate = ROOT / 'experiments/multipart-promotion-v1'
    if args.native:
        qualify_native(args.native.resolve(), work, candidate)
        return
    compiler, runtime = args.compiler.resolve(), args.runtime.resolve()
    report = {'scope': __doc__, 'compiler_sha256': digest(compiler), 'production_adopted': False,
              'compiler_host_stack_bytes': 67108864, 'cpu_affinity': [cpu], 'phases': []}
    if args.initialize_rust_runtime:
        initialization = work / 'runtime-initialization'; initialization.mkdir(exist_ok=True)
        runtime, report['runtime_initialization'] = initialize_runtime(compiler, runtime, initialization)
    report['runtime_sha256'] = digest(runtime)
    source_work, metadata_work = work / 'source', work / 'metadata'
    commands = [
        ('source', [sys.executable, str(ROOT / 'tooling/check_multipart_live_effects.py'),
                    '--work', str(source_work), '--rust', str(compiler), '--rust-runtime', str(runtime),
                    '--effect-source', str(candidate / 'sources/Effect.Multipart.cinch'),
                    '--text-source', str(candidate / 'sources/Text.Multipart.cinch'),
                    '--retain-interfaces', '--source-cap-mib', '6', '--cpu', str(cpu)]),
        ('metadata', [sys.executable, str(ROOT / 'tooling/inspect_multipart_carriers.py'),
                      '--source-work', str(source_work), '--candidate', str(candidate),
                      '--compiler', str(compiler), '--work', str(metadata_work), '--cpu', str(cpu)]),
        ('live-runtime', [sys.executable, str(ROOT / 'tooling/run_multipart_live_effects.py'),
                          '--source-work', str(source_work), '--compiler', str(compiler),
                          '--runtime', str(runtime), '--cpu', str(cpu)]),
    ]
    for name, command in commands:
        result = subprocess.run(command, cwd=ROOT, capture_output=True, text=True, timeout=900,
                                env={**os.environ, 'RUST_MIN_STACK': '67108864', 'CINCH_STACK_SIZE': '64M'})
        report['phases'].append({'name': name, 'command': command, 'returncode': result.returncode,
                                 'stdout': result.stdout, 'stderr': result.stderr})
        (work / 'report.json').write_text(json.dumps(report, indent=2) + '\n')
        if result.returncode:
            raise RuntimeError(name + ' failed:\n' + result.stdout + result.stderr)
    print('isolated multipart source, defining metadata and live runtime passed', flush=True)


if __name__ == '__main__':
    main()
