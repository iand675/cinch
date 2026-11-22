#!/usr/bin/env python3
"""Exercise finite recursive array payloads and escaping leaf owners through a matched Native snapshot."""
from __future__ import annotations

import argparse
import os
from pathlib import Path
import subprocess

import bootstrap
import bootstrap_objects


def main() -> None:
    """Require persistent structure and shared mutable leaf observations in both ownership modes."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--compiler', type=Path, required=True)
    parser.add_argument('--library-artifact', type=Path, required=True)
    parser.add_argument('--artifacts', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    provider = bootstrap_objects.artifact_provider(bootstrap, args.library_artifact)
    args.artifacts = args.artifacts.resolve()
    args.artifacts.mkdir(parents=True, exist_ok=False)
    manifest, objects = provider['interface_package'], args.artifacts / 'objects.json'
    bootstrap.write_json(objects, provider['configuration'])
    array = bootstrap.ROOT / 'seed/prelude/Data/Array.cinch'
    fixture = bootstrap.ROOT / 'seed/tests/fixtures/nested-array-runtime-control.cinch'
    bundle, ir = args.artifacts / 'Bundle.cinch', args.artifacts / 'consumer.ll'
    bundle.write_bytes(array.read_bytes() + b'\n\n' + fixture.read_bytes())
    command = [str(args.compiler.resolve()), '--emit-ir', '--rc-debug', '--interfaces', str(manifest),
               '--objects', str(objects), str(bundle), str(ir)]
    report = {'status': 'building', 'compiler_sha256': bootstrap.file_hash(args.compiler),
              'provider': provider['identity'], 'array_source_sha256': bootstrap.file_hash(array),
              'fixture_sha256': bootstrap.file_hash(fixture), 'bundle_sha256': bootstrap.file_hash(bundle),
              'compile_command': command, 'compiler_runtime_stack': '64M', 'links': []}
    bootstrap.write_json(args.output, report)
    result = subprocess.run(command, capture_output=True, text=True,
                            env={**bootstrap.environment(), 'CINCH_STACK_SIZE': '64M'})
    (args.artifacts / 'compile.log').write_text(result.stdout + result.stderr)
    report['compile_exit'] = result.returncode
    bootstrap.write_json(args.output, report)
    if result.returncode:
        raise bootstrap.BootstrapError('recursive array runtime fixture did not compile')
    report['llvm_sha256'] = bootstrap.file_hash(ir)
    for opt in [0, 2]:
        exe = args.artifacts / f'consumer-o{opt}'
        command = [os.environ.get('CINCH_CLANG', 'clang'), str(ir), str(provider['object']), str(provider['runtime']),
                   f'-O{opt}', '-o', str(exe), '-ldl', '-lpthread', '-lm']
        linked = subprocess.run(command, capture_output=True, text=True, env=bootstrap.environment())
        (args.artifacts / f'link-o{opt}.log').write_text(linked.stdout + linked.stderr)
        if linked.returncode:
            raise bootstrap.BootstrapError('recursive array runtime fixture did not link')
        row = {'optimization': opt, 'command': command, 'executable_sha256': bootstrap.file_hash(exe), 'runs': []}
        for debug in [False, True]:
            result = subprocess.run([str(exe)], capture_output=True, text=True, timeout=180,
                env={**os.environ, 'CINCH_STACK_SIZE': '8M', 'CINCH_HEAP_LIMIT': '256M' if debug else '32M',
                     'CINCH_RC_DEBUG': str(int(debug)), 'CINCH_LEAK_CHECK': '1'})
            row['runs'].append({'debug': debug, 'exit': result.returncode, 'stderr': result.stderr})
            if result.returncode or 'leaked=0' not in result.stderr:
                raise bootstrap.BootstrapError('recursive array ownership observation failed')
        report['links'].append(row)
        bootstrap.write_json(args.output, report)
    bootstrap_objects.artifact_provider(bootstrap, args.library_artifact)
    report['status'] = 'passed'
    bootstrap.write_json(args.output, report)
    print('Recursive array escaping owners passed O0/O2, normal and poisoned ownership.', flush=True)


if __name__ == '__main__':
    main()
