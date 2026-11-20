#!/usr/bin/env python3
"""Validate fresh imported String/Bytes construction through an explicitly admitted Native provider."""
from __future__ import annotations

import argparse
import json
import os
from pathlib import Path
import shutil
import resource

import bootstrap
import bootstrap_objects

from bench_stdlib import source_closure
from validate_buffer_construction import checks as buffer_checks
from validate_string_join import checks as join_checks
from validate_string_scalars import checks as scalar_checks, consumer as scalar_consumer
from validate_string_construction import digest, run, source_hash

ROOT = Path(__file__).resolve().parents[1]


def compiler_stack() -> None:
    """Set the compiler main-thread limit independently of each consumer's bounded stack."""
    _, hard = resource.getrlimit(resource.RLIMIT_STACK)
    amount = 64 * 1024 * 1024
    resource.setrlimit(resource.RLIMIT_STACK, (amount if hard == resource.RLIM_INFINITY else min(amount, hard), hard))


def consumer(expressions: list[str], extra: str) -> str:
    """Select one assertion per process to bound checked-RC quarantine at the required heap cap."""
    source = (ROOT / 'seed/tests/fixtures/string-construction-consumer.cinchpart').read_text()
    source = source.replace('  import Data.String\n', '  import Data.String\n  import Effect.Process\n') + '\n' + extra
    dispatch = 'False'
    for i in reversed(range(len(expressions))):
        source += f'\n  -- | Observe one independent fresh-provider assertion {i}.\n  check{i} :: Unit -> Bool\n  check{i} unit = {expressions[i]}\n'
        dispatch = f'if batch == "{i}" then check{i} () else ' + dispatch
    source += '\n  -- | Select exactly one complete observation.\n  selected :: String -> Bool\n  selected batch = ' + dispatch
    return source + '\n  -- | Observe imported fresh-provider construction without source shadows.\n  main :: Unit ->{IO r} Int\n  main unit = with (io = newIO ()) in case process-args () of\n    Ok (Cons batch Nil) -> if selected batch then 0 else 1\n    other -> 2\n'


def receipts(interfaces: Path, provider: Path) -> dict:
    """Require both exact source hashes from admitted interface files and the provider's source bytes."""
    selected = {}
    for name, iface_hash, path in json.loads(interfaces.read_text())['interfaces']:
        if name not in ['Data.String', 'Data.Bytes']:
            continue
        path = Path(path)
        data = json.loads(path.read_text())
        module = provider / 'base' / Path(*name.split('.')).with_suffix('.cinch')
        assert data['module'] == name and data['source_hash'] == source_hash(module.read_bytes())
        selected[name] = {'source_hash': data['source_hash'], 'source_sha256': digest(module),
                          'admitted_interface_hash': iface_hash, 'interface_path': str(path),
                          'interface_sha256': digest(path)}
    assert selected['Data.String']['source_hash'] == '416560347689830784'
    assert selected['Data.Bytes']['source_hash'] == '12579928885703464370'
    return selected


def main() -> None:
    """Compile once per corpus, link O0/O2 explicitly and keep every memory-process receipt separate."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--compiler', type=Path, required=True)
    parser.add_argument('--interfaces', type=Path, required=True)
    parser.add_argument('--objects', type=Path, required=True)
    parser.add_argument('--provider', type=Path, required=True)
    parser.add_argument('--artifacts', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--groups', nargs='+', choices=['packed', 'buffers', 'regression', 'buffer-owners', 'joins', 'join-owners', 'texts', 'edges', 'aggregates', 'raw', 'scalar-owners'],
                        default=['packed', 'buffers', 'regression', 'buffer-owners', 'joins', 'join-owners', 'texts', 'edges', 'aggregates', 'raw', 'scalar-owners'])
    parser.add_argument('--compiler-stack-size', choices=['64M', '1G'], default='1G',
                        help='Native compiler program-thread stack; consumer runs retain their fixed eight-MiB stack')
    args = parser.parse_args()
    fixture_path = ROOT / 'seed/tests/fixtures/buffer-construction-vectors.json'
    fixture = json.loads(fixture_path.read_text())
    join_path = ROOT / 'seed/tests/fixtures/string-join-vectors.json'
    join_fixture = json.loads(join_path.read_text())
    admitted = bootstrap_objects.artifact_provider(bootstrap, args.provider / 'provider.json')
    configuration = json.loads(args.objects.read_text())
    if configuration != admitted['configuration'] or json.loads(args.interfaces.read_text()) != admitted['interfaces']:
        raise bootstrap.BootstrapError('client manifests differ from the admitted provider')
    obj = Path(configuration['object'])
    assert digest(obj) == configuration['object_sha256']
    runtime = args.provider / 'libcinchruntime.a'
    report = {'epoch': 'fresh-imported-provider', 'source_shadows_used': False,
              'native_compiler_sha256': digest(args.compiler), 'provider_producer_sha256': digest(args.provider / 'cinch'),
              'provider_receipt_sha256': digest(args.provider / 'provider.json'),
              'runtime_sha256': digest(runtime), 'library_object_sha256': digest(obj),
              'interfaces_sha256': digest(args.interfaces), 'objects_sha256': digest(args.objects),
              'epoch_receipts': receipts(args.interfaces, args.provider),
              'constructor_fixture_sha256': digest(fixture_path), 'join_fixture_sha256': digest(join_path),
              'compiler_main_thread_stack_bytes': 67108864, 'compiler_runtime_stack_bytes': 67108864 if args.compiler_stack_size == '64M' else 1073741824, 'program_stack_bytes': 8388608, 'results': []}
    scalar_path = ROOT / 'seed/tests/fixtures/string-scalar-vectors.json'
    scalar_fixture = json.loads(scalar_path.read_text())
    report['scalar_fixture_sha256'] = digest(scalar_path)
    extra_buffer = (ROOT / 'seed/tests/fixtures/buffer-construction-consumer.cinchpart').read_text()
    extra_join = (ROOT / 'seed/tests/fixtures/string-join-consumer.cinchpart').read_text()
    for group in args.groups:
        if group in ['packed', 'buffers']:
            expressions = buffer_checks(group, fixture[group])
        elif group == 'regression':
            packed = next(row for row in fixture['packed'] if row['text'] == 'é' and row['count'] == 65536)
            buffer = next(row for row in fixture['buffers'] if row['mode'] == 2 and row['count'] == 65536)
            expressions = buffer_checks('packed', [packed]) + buffer_checks('buffers', [buffer])
        elif group == 'buffer-owners':
            expressions = ['buffer-owners-case ()']
        elif group == 'joins':
            expressions = join_checks(join_fixture['joins'])
        elif group == 'join-owners':
            expressions = ['join-owners-case ()']
        elif group == 'scalar-owners':
            expressions = ['scalar-owners-case ()']
        else:
            expressions = scalar_checks(group, scalar_fixture[group])
        work = args.artifacts / group
        work.mkdir(parents=True, exist_ok=True)
        is_scalar = group in ['texts', 'edges', 'aggregates', 'raw', 'scalar-owners']
        text = scalar_consumer(expressions) if is_scalar else consumer(expressions, extra_join if group in ['joins', 'join-owners'] else extra_buffer)
        entry = work / 'Main.cinch'
        entry.write_text(text)
        dependencies = source_closure(text)
        assert not {'Data.String', 'Data.Bytes'}.intersection(dependencies)
        for name, path in dependencies.items():
            shutil.copyfile(path, work / (name + '.cinch'))
        assert not (work / 'Data.String.cinch').exists() and not (work / 'Data.Bytes.cinch').exists()
        bundle = work / 'Bundle.cinch'
        bundle.write_text('\n'.join(p.read_text() for p in reversed(list(dependencies.values()))) + '\n' + text)
        ir = (work / 'consumer.ll').resolve()
        command = [str(args.compiler.resolve()), '--emit-ir', '--rc-debug', '--interfaces', str(args.interfaces.resolve()),
                   '--objects', str(args.objects.resolve()), str(bundle.resolve()), str(ir)]
        try:
            build = run(command, env={**os.environ, 'CINCH_STACK_SIZE': args.compiler_stack_size}, preexec_fn=compiler_stack)
        except RuntimeError as error:
            diagnostics = ir.with_suffix('.ll.diagnostics')
            raise RuntimeError(str(error) + '\n' + (diagnostics.read_text() if diagnostics.exists() else '')) from error
        (work / 'compile.log').write_text(build.stdout + build.stderr)
        result = {'group': group, 'cases': len(expressions), 'entry_sha256': digest(entry), 'bundle_sha256': digest(bundle),
                  'llvm_sha256': digest(ir), 'compile_command': command,
                  'source_dependencies': {n: digest(p) for n, p in dependencies.items()}, 'links': []}
        for opt in [0, 2]:
            exe = (work / f'consumer-o{opt}').resolve()
            link = [os.environ.get('CINCH_CLANG', 'clang'), str(ir), str(obj), str(runtime), f'-O{opt}',
                    '-o', str(exe), '-ldl', '-lpthread', '-lm']
            linked = run(link)
            (work / f'link-o{opt}.log').write_text(linked.stdout + linked.stderr)
            ownership = []
            for debug in [False, True]:
                for batch in range(len(expressions)):
                    for repetition in range(10 if group.endswith('owners') else 1):
                        arguments = [str(batch // 64), str(batch % 64)] if is_scalar else [str(batch)]
                        observation = run([str(exe), *arguments], env={**os.environ, 'CINCH_RC_DEBUG': str(int(debug)),
                                          'CINCH_HEAP_LIMIT': '256M' if debug else '32M', 'CINCH_STACK_SIZE': '8M',
                                          'CINCH_LEAK_CHECK': '1'})
                        assert 'leaked=0' in observation.stderr, observation.stderr
                        ownership.append({'debug': debug, 'assertion': batch, 'repetition': repetition, 'zero_leaks': True})
            result['links'].append({'optimization': opt, 'command': link, 'executable_sha256': digest(exe), 'ownership': ownership})
            print(f'{group}: {len(expressions)} independent assertions, Native O{opt}, normal+poison zero leaks', flush=True)
        report['results'].append(result)
        args.output.parent.mkdir(parents=True, exist_ok=True)
        args.output.write_text(json.dumps(report, indent=2) + '\n')
    bootstrap_objects.artifact_provider(bootstrap, args.provider / 'provider.json')
    report['status'] = 'passed'
    args.output.write_text(json.dumps(report, indent=2) + '\n')


if __name__ == '__main__':
    main()
