#!/usr/bin/env python3
"""Require a concrete immutable-string map enumeration from an admitted Native provider."""
from __future__ import annotations

import argparse
import os
from pathlib import Path
import subprocess

import bootstrap
import bootstrap_objects


def main() -> None:
    """Check the actual specialization and independent retained-map results at both link profiles."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--compiler', type=Path, required=True)
    parser.add_argument('--library-artifact', type=Path, required=True)
    parser.add_argument('--artifacts', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    provider = bootstrap_objects.artifact_provider(bootstrap, args.library_artifact)
    args.artifacts = args.artifacts.resolve()
    args.artifacts.mkdir(parents=True, exist_ok=False)
    manifest, objects = args.artifacts / 'interfaces.json', args.artifacts / 'objects.json'
    bootstrap.write_json(manifest, provider['interfaces'])
    bootstrap.write_json(objects, provider['configuration'])
    metadata = bootstrap.read_json(Path(provider['configuration']['metadata']))
    emitted = [unit for unit in metadata['units'] if unit['symbol'] == 'Data.HashMap.hash-map-to-list@OO']
    if len(emitted) != 1:
        raise bootstrap.BootstrapError('the actual object must contain exactly one requested OO enumeration unit')
    fixture = bootstrap.ROOT / 'seed/tests/fixtures/base-provider-hashmap-oo.cinch'
    source = args.artifacts / 'Main.cinch'
    source.write_bytes(fixture.read_bytes())
    ir = args.artifacts / 'consumer.ll'
    command = [str(args.compiler.resolve()), '--emit-ir', '--rc-debug', '--interfaces', str(manifest),
               '--objects', str(objects), str(source), str(ir)]
    report = {'status': 'building', 'compiler_sha256': bootstrap.file_hash(args.compiler),
              'provider': provider['identity'], 'source_sha256': bootstrap.file_hash(source),
              'source_shadow_used': False, 'emitted_unit': emitted[0], 'compile_command': command, 'links': []}
    bootstrap.write_json(args.output, report)
    compiled = subprocess.run(command, env=bootstrap.environment(), capture_output=True, text=True)
    (args.artifacts / 'compile.log').write_text(compiled.stdout + compiled.stderr)
    report['compile_exit'] = compiled.returncode
    bootstrap.write_json(args.output, report)
    if compiled.returncode:
        raise bootstrap.BootstrapError('concrete map enumeration compilation failed; see retained diagnostics')
    report['llvm_sha256'] = bootstrap.file_hash(ir)
    for opt in [0, 2]:
        executable = args.artifacts / f'consumer-o{opt}'
        link = [os.environ.get('CINCH_CLANG', 'clang'), str(ir), str(provider['object']), str(provider['runtime']),
                f'-O{opt}', '-o', str(executable)]
        if bootstrap.HOST['system'] == 'Linux':
            link.extend(['-ldl', '-lpthread', '-lm'])
        linked = subprocess.run(link, env=bootstrap.environment(), capture_output=True, text=True)
        (args.artifacts / f'link-o{opt}.log').write_text(linked.stdout + linked.stderr)
        if linked.returncode:
            raise bootstrap.BootstrapError('concrete map enumeration link failed')
        row = {'optimization': opt, 'command': link, 'executable_sha256': bootstrap.file_hash(executable), 'runs': []}
        for debug in [False, True]:
            run = subprocess.run([str(executable)], capture_output=True, text=True, timeout=180,
                env={**os.environ, 'CINCH_STACK_SIZE': '8M', 'CINCH_HEAP_LIMIT': '256M' if debug else '32M',
                     'CINCH_RC_DEBUG': str(int(debug)), 'CINCH_LEAK_CHECK': '1'})
            row['runs'].append({'debug': debug, 'exit': run.returncode, 'stderr': run.stderr})
            if run.returncode or 'leaked=0' not in run.stderr:
                raise bootstrap.BootstrapError('concrete map enumeration runtime observation failed')
        report['links'].append(row)
        bootstrap.write_json(args.output, report)
    bootstrap_objects.artifact_provider(bootstrap, args.library_artifact)
    report['status'] = 'passed'
    bootstrap.write_json(args.output, report)
    print('Concrete OO map enumeration passed O0/O2, normal and poisoned ownership.', flush=True)


if __name__ == '__main__':
    main()
