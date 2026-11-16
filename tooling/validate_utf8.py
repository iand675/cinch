#!/usr/bin/env python3
"""Build independent raw UTF-8 corpora with explicit frozen compiler epochs and bounded ownership runs."""
from __future__ import annotations
import argparse
import hashlib
import json
import os
from pathlib import Path
import subprocess
import time
import tempfile
from render_utf8_consumer import rendered
from validate_url import source_closure

ROOT = Path(__file__).resolve().parents[1]
RUST = Path('/workspace/dasl-stdlib-io/build/prim-wide-frozen/cinch')
NATIVE = Path('/workspace/dasl-stdlib-char-contract/build/character-contract-stage3-frozen')


def digest(path: Path) -> str:
    """Identify exact compiler, corpus, runtime and executable bytes."""
    return hashlib.sha256(path.read_bytes()).hexdigest()


def consumer() -> str:
    """Render shallow expected-word pages without a deep constant constructor list."""
    return rendered()


def main() -> None:
    """Compile once and keep complete normal coverage separate from bounded poisoned quarantine."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--frontend', choices=['rust', 'native'], required=True)
    parser.add_argument('--native-snapshot', type=Path, default=NATIVE)
    parser.add_argument('--cpu', type=int, default=2)
    parser.add_argument('--artifacts', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--chunk', type=int, help='Run one diagnostic chunk instead of complete coverage')
    parser.add_argument('--compile-only', action='store_true')
    parser.add_argument('--reuse', action='store_true')
    parser.add_argument('--source-override', type=Path, help='Retain an isolated candidate implementation without editing the maintained library')
    args = parser.parse_args()
    os.sched_setaffinity(0, {args.cpu})
    artifacts = args.artifacts.resolve()
    artifacts.mkdir(parents=True, exist_ok=True)
    temporary = tempfile.TemporaryDirectory(prefix='reuse-source-check-', dir=artifacts) if args.reuse else None
    project = Path(temporary.name) if temporary is not None else artifacts / 'project'
    project.mkdir(parents=True, exist_ok=True)
    source = project / 'Main.cinch'
    source.write_text(consumer())
    inputs = source_closure(project, source) + [source]
    if args.source_override:
        (project / 'Data.Bytes.Utf8.cinch').write_bytes(args.source_override.read_bytes())
    exe = artifacts / 'consumer'
    compiler = RUST if args.frontend == 'rust' else args.native_snapshot / 'cinch'
    if args.frontend == 'rust':
        command = [str(compiler), 'build', str(source), '-o', str(exe), '--opt-level', '2', '--rc-debug']
    else:
        bundle = artifacts / 'bundle.cinch'
        if not args.reuse:
            bundle.write_text('\n\n'.join(path.read_text() for path in sorted(inputs)))
        command = ['python3', str(args.native_snapshot / 'frontend.py'), '--compile-only', '--interfaces', str(args.native_snapshot / 'base-interfaces.json'), '--rc-debug', str(bundle), str(artifacts / 'consumer.ll'), str(exe), str(artifacts / 'link-report.txt')]
    runtime = (Path('/workspace/dasl-stdlib-io/compiler/runtime/target/release/libcinchruntime.a')
               if args.frontend == 'rust' else args.native_snapshot / 'libcinchruntime.a')
    provider = {} if args.frontend == 'rust' else {
        name: digest(args.native_snapshot / name)
        for name in ['frontend.py', 'base-interfaces.json', 'objects.json', 'library.o', 'library.abi.json', 'provenance.json']
    }
    current_sources = {path.name: digest(path) for path in inputs}
    identity_path = artifacts / 'build-identity.json'
    started = time.monotonic()
    if args.reuse:
        prior = json.loads(identity_path.read_text()) if identity_path.exists() else json.loads(args.output.read_text())
        if prior['sources'] != current_sources or prior['compiler_sha256'] != digest(compiler) or prior['fixture_sha256'] != digest(ROOT / 'compiler/tests/fixtures/utf8-vectors.json') or prior['executable_sha256'] != digest(exe) or prior.get('runtime_sha256') != digest(runtime) or prior.get('provider_sha256') != provider:
            raise ValueError('Refusing executable reuse after compiler, provider, runtime, fixture, source or executable identity changed')
        command = prior.get('compile_command')
    if not args.reuse:
        result = subprocess.run(command, capture_output=True, text=True, cwd=project)
        (artifacts / 'compile.stdout').write_text(result.stdout)
        (artifacts / 'compile.stderr').write_text(result.stderr)
        if result.returncode:
            diagnostics = artifacts / 'consumer.ll.diagnostics'
            raise RuntimeError((diagnostics.read_text() if diagnostics.exists() else result.stdout) + '\n' + result.stderr)
    compile_seconds = time.monotonic() - started
    identity = {'sources': current_sources, 'compiler_sha256': digest(compiler), 'fixture_sha256': digest(ROOT / 'compiler/tests/fixtures/utf8-vectors.json'), 'executable_sha256': digest(exe), 'runtime_sha256': digest(runtime), 'provider_sha256': provider, 'compile_command': command}
    identity_path.write_text(json.dumps(identity, indent=2) + '\n')
    if args.compile_only:
        print(f'{args.frontend} UTF8 corpus compiled', flush=True)
        return
    fixture = json.loads((ROOT / 'compiler/tests/fixtures/utf8-vectors.json').read_text())
    counts = [len(fixture['scalar_chunks']), len(fixture['pair_chunks']), (len(fixture['rows']) + 63) // 64]
    runs = []
    groups = [[str(mode), str(index)] for mode, count in enumerate(counts) for index in (range(count) if args.chunk is None else [args.chunk]) if index < count]
    selected = [(False, [])] if args.chunk is None else [(False, group) for group in groups]
    selected += [(True, group) for group in groups]
    for debug, group in selected:
        env = {**os.environ, 'CINCH_RC_DEBUG': '1' if debug else '0', 'CINCH_LEAK_CHECK': '1', 'CINCH_HEAP_LIMIT': '256M' if debug else '32M', 'CINCH_STACK_SIZE': '8M'}
        started = time.monotonic()
        result = subprocess.run([str(exe), *group], capture_output=True, text=True, env=env)
        if result.returncode or 'leaked=0' not in result.stderr:
            raise RuntimeError(f'{group}, poisoned={debug}, exit={result.returncode}: {result.stdout}\n{result.stderr}')
        runs.append({'group': group, 'poisoned': debug, 'heap': env['CINCH_HEAP_LIMIT'], 'stack': '8M', 'seconds': time.monotonic() - started, 'stderr': result.stderr})
    report = {'frontend': args.frontend, 'compiler': str(compiler), 'compiler_sha256': digest(compiler), 'native_snapshot': str(args.native_snapshot) if args.frontend == 'native' else None, 'sources': {path.name: digest(path) for path in inputs}, 'fixture_sha256': digest(ROOT / 'compiler/tests/fixtures/utf8-vectors.json'), 'executable_sha256': digest(exe), 'runtime': str(runtime), 'runtime_sha256': digest(runtime), 'provider_sha256': provider, 'compile_command': command, 'reused_existing_executable': args.reuse, 'source_override': str(args.source_override.resolve()) if args.source_override else None, 'compile_seconds': compile_seconds, 'cpu': args.cpu, 'complete_domain': args.chunk is None, 'runs': runs, 'all_passed': True}
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(report, indent=2) + '\n')
    print(f'{args.frontend} UTF8 corpus passed {len(runs)} memory runs', flush=True)


if __name__ == '__main__':
    main()
