#!/usr/bin/env python3
"""Prepare and qualify one live both-effect fixture without changing production.

Source checks precede any separately requested runtime stage. Every stage uses
explicit compiler identities and freezes the selected declaration headers.
"""
from __future__ import annotations
import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import resource
import shutil
import subprocess
from bench_stdlib import source_closure
from check_compare import digest
from check_multipart_effect_experiment import archive_cache


#: Production-independent reviewed effect candidate and maintained fixture support.
ROOT = Path(__file__).resolve().parents[1]


def word(hash_value: int, number: int) -> int:
    """Extend the independent scalar trace with one explicit little-endian word."""
    for byte in number.to_bytes(8, 'little'):
        hash_value = ((hash_value ^ byte) * 1099511628211) & ((1 << 64) - 1)
    return hash_value


def expected_sink() -> dict:
    """Compute actual offered/accepted/error traces independently of the language implementation."""
    header = b'--outer\r\nContent-Disposition: form-data; name="f"\r\n\r\n'
    trace = output = 14695981039346656037
    size = 0
    for offer, count, error in [(header, len(header), 0), (b'abcd', 2, 1), (b'xyz', 1, 0), (b'', 0, 4)]:
        trace = word(trace, len(offer))
        for byte in offer:
            trace = ((trace ^ byte) * 1099511628211) & ((1 << 64) - 1)
        trace = word(word(trace, count), error)
        for byte in offer[:count]:
            output = ((output ^ byte) * 1099511628211) & ((1 << 64) - 1)
        size += count
    return {'trace': trace, 'output': output, 'size': size, 'calls': 4}


def prepare(work: Path, effect_source: Path | None = None, text_source: Path | None = None) -> dict:
    """Freeze the candidate, all actual source dependencies and one complete executable fixture."""
    experiment = ROOT / 'experiments/multipart-both-effects-v1/sources'
    source = (ROOT / 'tooling/data/multipart-consumer.cinchpart').read_text().replace(
        '  import Data.Bytes\n', '  import Data.Bytes\n  import Data.Mutable.Var\n', 1)
    for name in ('multipart-storage', 'multipart-managed-storage', 'multipart-live-effects'):
        source += (ROOT / 'tooling/data' / (name + '.cinchpart')).read_text()
    oracle = expected_sink()
    source += f'''
  -- | Execute all three live factories and real independent kernel/resource observations.
  main :: Unit ->{{IO r, Mut r}} Int
  main unit = with (io = newIO ()) in
    let kernels = raw-kernels 0 in
    let part = live-part-control () in
    let sink = live-sink-control {oracle['trace']} {oracle['output']} {oracle['size']} in
    let file = live-file-control () in
    if eq-i32 kernels 0 then (if part then (if sink then (if file then 0 else 3) else 2) else 1) else 250
'''
    overrides = {'Text.Multipart': text_source or experiment / 'Text.Multipart.cinch',
                 'Effect.Multipart': effect_source or experiment / 'Effect.Multipart.cinch'}
    closure = source_closure(source, overrides)
    ordered, seen = [], set()

    def visit(name: str) -> None:
        """Keep the source-distributed frontend's real dependency ordering explicit."""
        if name in seen:
            return
        seen.add(name)
        for dependency in re.findall(r'^\s*import\s+([A-Z][\w.]*)', closure[name].read_text(), re.M):
            if dependency in closure:
                visit(dependency)
        ordered.append(name)

    for name, path in closure.items():
        visit(name)
        shutil.copyfile(path, work / (name + '.cinch'))
    (work / 'Main.cinch').write_text(source)
    bundle = '\n'.join((work / (name + '.cinch')).read_text() for name in ordered) + '\n' + source
    (work / 'bundle.source').write_text(bundle)
    return {'scope': __doc__, 'production_adopted': False, 'runtime_claim': False,
            'compiler_host_stack_bytes': 67108864, 'consumer_stack': '8M', 'source_sha256': digest(work / 'Main.cinch'),
            'source_selection': {name: {'path': str(path.resolve()), 'sha256': digest(path)} for name, path in overrides.items()},
            'bundle_sha256': digest(work / 'bundle.source'), 'sources': {name: digest(work / (name + '.cinch')) for name in ordered},
            'independent_sink_oracle': oracle, 'checks': []}


def main() -> None:
    """Stop at the first genuine source failure and keep runtime an explicit later stage."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--work', type=Path, required=True)
    parser.add_argument('--rust', type=Path, required=True)
    parser.add_argument('--rust-runtime', type=Path, help='Exact future host link archive; source checks do not initialize or link it')
    parser.add_argument('--native', type=Path)
    parser.add_argument('--cpu', type=int, required=True)
    parser.add_argument('--prepare-only', action='store_true')
    parser.add_argument('--effect-source', type=Path, help='Explicit isolated candidate, without changing the historical default')
    parser.add_argument('--text-source', type=Path, help='Explicit isolated pure-module candidate')
    parser.add_argument('--retain-interfaces', action='store_true', help='Keep the complete genuine defining closure and original manifest plain')
    parser.add_argument('--source-cap-mib', type=int, choices=(5, 6), default=5, help='Explicit authorized source-only epoch bound; historical default is unchanged')
    args = parser.parse_args()
    os.sched_setaffinity(0, {args.cpu})
    resource.setrlimit(resource.RLIMIT_STACK, (67108864, resource.getrlimit(resource.RLIMIT_STACK)[1]))
    work = args.work.resolve(); work.mkdir(parents=True, exist_ok=True)
    report = prepare(work, args.effect_source, args.text_source)
    report['source_cap_bytes'] = args.source_cap_mib * 1024 * 1024
    if args.rust_runtime:
        report['future_rust_runtime'] = {'path': str(args.rust_runtime.resolve()), 'sha256': digest(args.rust_runtime)}
    if args.prepare_only:
        (work / 'report.json').write_text(json.dumps(report, indent=2) + '\n')
        return
    frontends = [('rust', args.rust.resolve())]
    if args.native:
        frontends.append(('native', args.native.resolve() / 'cinch'))
    for frontend, compiler in frontends:
        if frontend == 'rust':
            command = [str(compiler), 'check', str(work / 'Main.cinch')]
        else:
            command = [str(compiler), '--check', '--interfaces', str(args.native.resolve() / 'base-interfaces.json'),
                       str(work / 'bundle.source'), str(work / 'check.output')]
        result = subprocess.run(command, cwd=work, text=True, capture_output=True, timeout=60,
                                env={**os.environ, 'RUST_MIN_STACK': '67108864', 'CINCH_STACK_SIZE': '64M'})
        diagnostic = work / 'check.output.diagnostics'
        row = {'frontend': frontend, 'compiler_sha256': digest(compiler), 'command': command,
               'returncode': result.returncode, 'stdout': result.stdout, 'stderr': result.stderr,
               'diagnostics': diagnostic.read_text() if diagnostic.exists() else ''}
        report['checks'].append(row)
        if result.returncode == 0 and frontend == 'rust' and args.retain_interfaces:
            artifacts = work / 'defining-interfaces'
            artifacts.mkdir(exist_ok=True)
            retained = {}
            for path in sorted((work / 'build').glob('*.cinchif')) + [work / 'build/Main.manifest.json']:
                target = artifacts / path.name
                shutil.copyfile(path, target)
                retained[path.name] = digest(target)
            row['genuine_defining_interfaces'] = retained
        archive_cache(work, frontend + '-live-runtime-source', row)
        report['artifact_bytes'] = sum(path.stat().st_size for path in work.rglob('*') if path.is_file())
        (work / 'report.json').write_text(json.dumps(report, indent=2) + '\n')
        print(frontend, 'source accepted' if result.returncode == 0 else 'source refused', flush=True)
        if result.returncode:
            raise RuntimeError(result.stderr + row['diagnostics'])
        if report['artifact_bytes'] > report['source_cap_bytes']:
            raise RuntimeError('source-only artifact cap reached')


if __name__ == '__main__':
    main()
