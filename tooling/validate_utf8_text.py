#!/usr/bin/env python3
"""Validate immutable-text UTF-8 adapters against an explicit combined String artifact."""
from __future__ import annotations
import argparse
import json
import os
from pathlib import Path
import re
import resource
import subprocess
import time
from render_utf8_text_consumer import rendered, groups
from validate_url import digest

ROOT = Path(__file__).resolve().parents[1]
STRING_SHA256 = 'e5b79f9ddb1266a75a198096921b62e4fd6db172fc9111496f37bf7fa9cd04d6'
STRING_SOURCE_HASH = '416560347689830784'


def compiler_stack() -> None:
    """Bound the compiler host stack explicitly without changing the consumer's eight-MiB limit."""
    resource.setrlimit(resource.RLIMIT_STACK, (64 * 1024 * 1024, resource.getrlimit(resource.RLIMIT_STACK)[1]))


def source_closure(project: Path, main: Path, facade: Path, embedded: set[str]) -> list[Path]:
    """Copy source-distributed imports while refusing any embedded String source shadow."""
    pending = re.findall(r'^\s*import\s+([A-Za-z0-9_.]+)', main.read_text(), re.M)
    found, paths = set(), []
    while pending:
        name = pending.pop()
        if name in embedded or name in found: continue
        found.add(name)
        original = facade if name == 'Text.Utf8' else ROOT / 'seed/prelude' / (name.replace('.', '/') + '.cinch')
        target = project / (name + '.cinch'); target.write_bytes(original.read_bytes()); paths.append(target)
        pending.extend(re.findall(r'^\s*import\s+([A-Za-z0-9_.]+)', target.read_text(), re.M))
    if (project / 'Data.String.cinch').exists(): raise ValueError('Refusing embedded String source shadow')
    return paths


def main() -> None:
    """Retain exact producer/client/provider identities and full normal versus bounded poison coverage."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--frontend', choices=['rust', 'native'], required=True)
    parser.add_argument('--compiler', type=Path, required=True)
    parser.add_argument('--runtime', type=Path, required=True)
    parser.add_argument('--provider', type=Path, required=True)
    parser.add_argument('--interfaces', type=Path, required=True)
    parser.add_argument('--objects', type=Path, required=True)
    parser.add_argument('--facade-source', type=Path, default=ROOT / 'seed/prelude/Text/Utf8.cinch')
    parser.add_argument('--consumer-source', type=Path, help='A standalone public control consumer instead of the reference corpus')
    parser.add_argument('--provisional', action='store_true', help='This consumer is part of the still-running combined artifact gate')
    parser.add_argument('--cpu', type=int, default=2)
    parser.add_argument('--opt-level', choices=['0', '2'], default='2')
    parser.add_argument('--artifacts', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--group', choices=groups(), help='One bounded diagnostic group instead of full coverage')
    args = parser.parse_args(); os.sched_setaffinity(0, {args.cpu})
    producer = json.loads(args.provider.read_text())
    if producer['sources']['Data.String']['sha256'] != STRING_SHA256:
        raise ValueError('Provider String source differs from the reviewed import-free helper epoch')
    string_source = args.provider.parent / producer['sources']['Data.String']['path']
    if digest(string_source) != STRING_SHA256: raise ValueError('Provider String source bytes changed')
    if digest(args.runtime) != producer['runtime']['sha256']: raise ValueError('Runtime differs from the admitted producer artifact')
    if args.frontend == 'rust' and digest(args.compiler) != producer['producer']['compiler_sha256']:
        raise ValueError('Rust compiler differs from the combined embedded producer')
    interfaces = json.loads(args.interfaces.read_text())['interfaces']
    matched = [row for row in interfaces if row[0] == 'Data.String']
    if len(matched) != 1:
        raise ValueError('Admitted interfaces do not contain exactly one String interface')
    string_interface = Path(matched[0][2])
    if str(json.loads(string_interface.read_text())['source_hash']) != STRING_SOURCE_HASH:
        raise ValueError('Admitted String interface source hash differs from the reviewed helper epoch')
    objects = json.loads(args.objects.read_text())
    for key in ['object', 'metadata']:
        if digest(Path(objects[key])) != objects[key + '_sha256']: raise ValueError('Admitted object/metadata bytes changed')
    artifacts = args.artifacts.resolve(); project = artifacts / 'project'; project.mkdir(parents=True, exist_ok=True)
    if args.consumer_source is not None and args.group is not None:
        parser.error('--consumer-source cannot be combined with a corpus group')
    source = project / 'Main.cinch'; source.write_text(args.consumer_source.read_text() if args.consumer_source else rendered())
    sources = source_closure(project, source, args.facade_source, {row[0] for row in interfaces}) + [source]
    executable = artifacts / 'consumer'; ir = artifacts / 'consumer.ll'
    if args.frontend == 'rust':
        command = [str(args.compiler.resolve()), 'emit-ir', str(source), '-o', str(ir), '--rc-debug']
    else:
        bundle = artifacts / 'bundle.cinch'; bundle.write_text('\n\n'.join(path.read_text() for path in sorted(sources)))
        command = [str(args.compiler.resolve()), '--emit-ir', '--rc-debug', '--interfaces', str(args.interfaces.resolve()), '--objects', str(args.objects.resolve()), str(bundle), str(ir)]
    started = time.monotonic(); result = subprocess.run(command, capture_output=True, text=True, cwd=project, preexec_fn=compiler_stack)
    (artifacts / 'compile.stdout').write_text(result.stdout); (artifacts / 'compile.stderr').write_text(result.stderr)
    if result.returncode:
        diagnostics = ir.with_name(ir.name + '.diagnostics')
        raise RuntimeError((diagnostics.read_text() if diagnostics.exists() else result.stdout) + '\n' + result.stderr)
    compile_seconds = time.monotonic() - started
    link = ['clang', '-O' + args.opt_level, str(ir)]
    if args.frontend == 'native': link.append(objects['object'])
    link += [str(args.runtime.resolve()), '-ldl', '-lpthread', '-lm', '-o', str(executable)]
    result = subprocess.run(link, capture_output=True, text=True)
    (artifacts / 'link.stdout').write_text(result.stdout); (artifacts / 'link.stderr').write_text(result.stderr)
    if result.returncode: raise RuntimeError(result.stderr)
    selections = [(False, [] if args.group is None else [args.group])]
    selections += [(True, [])] if args.consumer_source else [(True, [name]) for name in (groups() if args.group is None else [args.group])]
    runs = []
    for poisoned, selection in selections:
        env = {**os.environ, 'CINCH_RC_DEBUG': '1' if poisoned else '0', 'CINCH_LEAK_CHECK': '1', 'CINCH_HEAP_LIMIT': '256M' if poisoned else '32M', 'CINCH_STACK_SIZE': '8M'}
        started = time.monotonic(); result = subprocess.run([str(executable), *selection], capture_output=True, text=True, env=env)
        if result.returncode or 'leaked=0' not in result.stderr:
            raise RuntimeError(f'group={selection}, poisoned={poisoned}, exit={result.returncode}: {result.stdout}\n{result.stderr}')
        runs.append({'group': selection, 'poisoned': poisoned, 'heap': env['CINCH_HEAP_LIMIT'], 'stack': '8M', 'seconds': time.monotonic() - started, 'stderr': result.stderr})
    report = {'frontend': args.frontend, 'provisional_combined_gate': args.provisional, 'source_shadow_used': False, 'opt_level': args.opt_level, 'compiler': str(args.compiler.resolve()), 'compiler_sha256': digest(args.compiler), 'producer': producer['producer'], 'runtime': str(args.runtime.resolve()), 'runtime_sha256': digest(args.runtime), 'provider_sha256': digest(args.provider), 'interfaces_sha256': digest(args.interfaces), 'objects_sha256': digest(args.objects), 'object_sha256': objects['object_sha256'], 'metadata_sha256': objects['metadata_sha256'], 'string_source_sha256': digest(string_source), 'string_source_hash': STRING_SOURCE_HASH, 'string_interface_hash': str(matched[0][1]), 'string_interface_sha256': digest(string_interface), 'sources': {path.name: digest(path) for path in sources}, 'fixture_sha256': digest(args.consumer_source if args.consumer_source else ROOT / 'seed/tests/fixtures/utf8-text-vectors.json'), 'llvm_sha256': digest(ir), 'executable_sha256': digest(executable), 'compile_command': command, 'link_command': link, 'compiler_host_stack_mib': 64, 'compile_seconds': compile_seconds, 'cpu': args.cpu, 'complete_domain': args.group is None and args.consumer_source is None, 'consumer_kind': 'controls' if args.consumer_source else '804 SDK observations', 'runs': runs, 'all_passed': True}
    args.output.parent.mkdir(parents=True, exist_ok=True); args.output.write_text(json.dumps(report, indent=2) + '\n')
    print(f'{args.frontend} O{args.opt_level} text adapters passed {len(runs)} bounded-memory runs', flush=True)


if __name__ == '__main__':
    main()
