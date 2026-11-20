#!/usr/bin/env python3
"""Validate exact no-copy UTF-8 span queries with explicit frozen frontend identities."""
from __future__ import annotations

import argparse
import json
import os
from pathlib import Path
import subprocess
import time
import tempfile
from render_utf8_span_consumer import rendered
from validate_url import source_closure, digest
from validate_utf8 import ROOT, RUST, NATIVE


def main() -> None:
    """Compile once, run all spans normally, and bound poisoned quarantine by eight owner buffers."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--frontend', choices=['rust', 'native'], required=True)
    parser.add_argument('--native-snapshot', type=Path, default=NATIVE)
    parser.add_argument('--cpu', type=int, default=2)
    parser.add_argument('--artifacts', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--group', type=int, help='Run one diagnostic group instead of complete coverage')
    parser.add_argument('--compile-only', action='store_true')
    parser.add_argument('--reuse', action='store_true')
    args = parser.parse_args(); os.sched_setaffinity(0, {args.cpu})
    fixture_path = ROOT / 'seed/tests/fixtures/utf8-span-vectors.json'
    fixture = json.loads(fixture_path.read_text()); count = (len(fixture['rows']) + 7) // 8
    if args.group is not None and not 0 <= args.group < count:
        parser.error('group is outside the current fixture')
    artifacts = args.artifacts.resolve(); artifacts.mkdir(parents=True, exist_ok=True)
    temporary = tempfile.TemporaryDirectory(prefix='reuse-source-check-', dir=artifacts) if args.reuse else None
    project = Path(temporary.name) if temporary is not None else artifacts / 'project'
    project.mkdir(parents=True, exist_ok=True)
    source = project / 'Main.cinch'; source.write_text(rendered())
    sources = source_closure(project, source) + [source]
    executable = artifacts / 'consumer'
    compiler = RUST if args.frontend == 'rust' else args.native_snapshot / 'cinch'
    runtime = Path('/workspace/dasl-stdlib-io/compiler/runtime/target/release/libcinchruntime.a') if args.frontend == 'rust' else args.native_snapshot / 'libcinchruntime.a'
    provider = {} if args.frontend == 'rust' else {name: digest(args.native_snapshot / name) for name in ['frontend.py', 'base-interfaces.json', 'objects.json', 'library.o', 'library.abi.json', 'provenance.json']}
    identity = {'sources': {path.name: digest(path) for path in sources}, 'compiler_sha256': digest(compiler), 'runtime_sha256': digest(runtime), 'provider_sha256': provider, 'fixture_sha256': digest(fixture_path)}
    identity_path = artifacts / 'build-identity.json'
    started = time.monotonic()
    if args.reuse:
        prior = json.loads(identity_path.read_text())
        if any(prior[key] != value for key, value in identity.items()) or prior['executable_sha256'] != digest(executable):
            raise ValueError('Refusing executable reuse after an exact source/compiler/provider/runtime/fixture change')
        command = prior['compile_command']
    else:
        if args.frontend == 'rust': command = [str(compiler), 'build', str(source), '-o', str(executable), '--opt-level', '2', '--rc-debug']
        else:
            bundle = artifacts / 'bundle.cinch'; bundle.write_text('\n\n'.join(path.read_text() for path in sorted(sources)))
            command = ['python3', str(args.native_snapshot / 'frontend.py'), '--compile-only', '--interfaces', str(args.native_snapshot / 'base-interfaces.json'), '--rc-debug', str(bundle), str(artifacts / 'consumer.ll'), str(executable), str(artifacts / 'link-report.txt')]
        result = subprocess.run(command, capture_output=True, text=True, cwd=project)
        (artifacts / 'compile.stdout').write_text(result.stdout); (artifacts / 'compile.stderr').write_text(result.stderr)
        if result.returncode:
            diagnostics = artifacts / 'consumer.ll.diagnostics'
            raise RuntimeError((diagnostics.read_text() if diagnostics.exists() else result.stdout) + '\n' + result.stderr)
    compile_seconds = time.monotonic() - started
    identity.update({'executable_sha256': digest(executable), 'compile_command': command})
    identity_path.write_text(json.dumps(identity, indent=2) + '\n')
    if args.compile_only:
        print(f'{args.frontend} span corpus compiled', flush=True); return
    selections = [(False, [])] if args.group is None else [(False, [str(args.group)])]
    selections += [(True, [str(index)]) for index in (range(count) if args.group is None else [args.group])]
    runs = []
    for poisoned, group in selections:
        env = {**os.environ, 'CINCH_RC_DEBUG': '1' if poisoned else '0', 'CINCH_LEAK_CHECK': '1', 'CINCH_HEAP_LIMIT': '256M' if poisoned else '32M', 'CINCH_STACK_SIZE': '8M'}
        started = time.monotonic(); result = subprocess.run([str(executable), *group], capture_output=True, text=True, env=env)
        if result.returncode or 'leaked=0' not in result.stderr:
            raise RuntimeError(f'group={group}, poisoned={poisoned}, exit={result.returncode}: {result.stdout}\n{result.stderr}')
        runs.append({'group': group, 'poisoned': poisoned, 'heap': env['CINCH_HEAP_LIMIT'], 'stack': '8M', 'seconds': time.monotonic() - started, 'stderr': result.stderr})
    report = {**identity, 'frontend': args.frontend, 'native_link_optimization': 'wrapper default O0' if args.frontend == 'native' else None, 'compiler': str(compiler), 'runtime': str(runtime), 'compile_seconds': compile_seconds, 'cpu': args.cpu, 'owner_buffers': len(fixture['rows']), 'spans': fixture['spans'], 'decodes': fixture['decodes'], 'complete_domain': args.group is None, 'runs': runs, 'all_passed': True}
    args.output.parent.mkdir(parents=True, exist_ok=True); args.output.write_text(json.dumps(report, indent=2) + '\n')
    print(f'{args.frontend} span corpus passed {len(runs)} bounded-memory runs', flush=True)


if __name__ == '__main__':
    main()
