#!/usr/bin/env python3
"""Publish a checked Stage 3 as a native binary, independent of the Nix store."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile


def checked(command, **kwargs):
    return subprocess.run(command, check=True, text=True, capture_output=True, **kwargs).stdout


def linux_dependencies(binary):
    headers = checked(['readelf', '--program-headers', str(binary)])
    dynamic = checked(['readelf', '--dynamic', str(binary)])
    return 'INTERP' in headers or '(NEEDED)' in dynamic


def verify_frontend(reference, binary):
    # Neither run has Nix's tools on PATH. Compare real IR output after
    # relinking and require a named refusal as well as a working entry point.
    with tempfile.TemporaryDirectory(prefix='cinch-nightly-') as scratch:
        root = Path(scratch)
        source = root / 'Main.cinch'
        source.write_text('module Main where\n  main :: Int\n  main = 2 + 3 * 4\n')
        outputs = []
        for index, compiler in enumerate((reference, binary)):
            output = root / f'program-{index}.ll'
            checked([str(compiler), '--emit-ir-only', str(source), str(output)],
                    cwd=root, env={'PATH': os.defpath}, timeout=120)
            outputs.append(output.read_bytes())
        if not outputs[0] or outputs[0] != outputs[1]:
            raise RuntimeError('published binary does not emit the checked Stage 3 IR')
        source.write_text('module Main where\n  main :: Int\n  main = unknown-name\n')
        diagnostics = root / 'refusal'
        refused = subprocess.run([str(binary), '--check', str(source), str(diagnostics)],
                                 cwd=root, env={'PATH': os.defpath}, timeout=120)
        if refused.returncode != 4 or 'unknown-name' not in diagnostics.with_suffix('.diagnostics').read_text():
            raise RuntimeError('published binary did not produce the expected named refusal')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--ladder', type=Path, required=True)
    parser.add_argument('--system', choices=('x86_64-linux', 'aarch64-linux', 'aarch64-darwin'), required=True)
    parser.add_argument('--revision', required=True)
    parser.add_argument('--out', type=Path, default=Path('dist'))
    args = parser.parse_args()
    stage = args.ladder.resolve() / 'stage3'
    reference = stage / 'cinch'
    args.out.mkdir(parents=True, exist_ok=True)
    binary = (args.out / f'cinch-{args.system}').resolve()
    shutil.copy2(reference, binary)
    packaging = 'checked-stage3'
    if args.system.endswith('-linux'):
        if linux_dependencies(binary):
            # The Nix toolchain may have installed its own ELF interpreter
            # even though the release must be usable outside /nix/store.
            # Keep Stage 3's compiled objects and link them statically with
            # the runner's native toolchain; do not recompile the frontend.
            objects = sorted(stage.glob('compiler.shard*.o'))
            if not objects:
                raise RuntimeError('the checked stage has no native objects to relink')
            provider = Path(json.loads((stage / 'base-objects.json').read_text())['object'])
            runtime = provider.with_name('libcinchruntime.a')
            checked(['/usr/bin/gcc', '-static', '-no-pie', *map(str, objects), str(provider),
                     str(runtime), '-ldl', '-lpthread', '-lm', '-o', str(binary)])
            packaging = 'checked-stage3-static-link'
        if linux_dependencies(binary):
            raise RuntimeError('Linux binary still has a dynamic loader or shared-library dependency')
    else:
        dependencies = checked(['otool', '-L', str(binary)]).splitlines()[1:]
        if any(not line.strip().startswith(('/usr/lib/', '/System/Library/')) for line in dependencies):
            raise RuntimeError('macOS binary depends on a non-system library')
    binary.chmod(0o755)
    verify_frontend(reference, binary)
    hasher = hashlib.sha256()
    with binary.open('rb') as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b''):
            hasher.update(chunk)
    digest = hasher.hexdigest()
    binary.with_name(binary.name + '.sha256').write_text(f'{digest}  {binary.name}\n')
    receipt = json.loads((stage / 'stage.json').read_text())
    (args.out / f'build-{args.system}.json').write_text(json.dumps({
        'revision': args.revision, 'system': args.system, 'stage': 3,
        'sha256': digest, 'packaging': packaging,
        'compiler_source_sha256': receipt['inputs']['source_sha256'],
    }, indent=2) + '\n')
    print(f'Verified native Stage 3: {binary.name} ({digest})')


if __name__ == '__main__':
    main()
