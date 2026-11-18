#!/usr/bin/env python3
"""Validate actual byte-splitting outputs, persistent states and shared pull cursors."""
from __future__ import annotations
import argparse
import json
import os
from pathlib import Path
import shutil

from bench_stdlib import source_closure
from generate_byte_split_vectors import crosscheck
from validate_byte_view import digest, run

ROOT = Path(__file__).resolve().parents[1]
FIXTURE = ROOT / 'compiler/tests/fixtures/byte-split-vectors.json'


def expression(row: dict) -> str:
    """Encode unlimited as zero, and a concrete nonnegative maximum as maximum plus one."""
    return (f'split-case "{row["input"]}" "{row["separator"]}" {row["start"]} {row["end"]} '
            f'{max(0, row["limit"] + 1)} {str(row["after"])} {str(row["lines"])} {row["hash"]} {row["count"]}')


def consumer(checks: list[str], width: int) -> str:
    """Choose bounded memory processes without changing the runtime's fixed heap limits."""
    text = (ROOT / 'compiler/tests/fixtures/byte-split-consumer.cinchpart').read_text()
    dispatch = 'False'
    for first in reversed(range(0, len(checks), width)):
        number = first // width
        text += f'\n  -- | Observe complete independent splitting group {number}.\n  group{number} :: Unit -> Bool\n  group{number} unit = all [\n'
        text += ',\n'.join(f'    ({value})' for value in checks[first:first + width]) + '\n    ]\n'
        dispatch = f'if index == "{number}" then group{number} () else ' + dispatch
    return text + '\n  -- | Select a bounded complete observation.\n  selected :: String -> Bool\n  selected index = ' + dispatch + '\n  -- | Run the selected complete output and lifecycle group with runtime deep-input sizes.\n  main :: Unit ->{IO r} Int\n  main unit = with (io = newIO ()) in case process-args () of\n    Ok (Cons index (Cons size Nil)) -> if index == "deep" then (if deep-case (decimal size) then 0 else 1) else 2\n    Ok (Cons index Nil) -> if selected index then 0 else 1\n    other -> 2\n'


def main() -> None:
    """Identify exact compiler/provider/runtime epochs and retain every failed command's diagnostics."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--compiler', type=Path, required=True)
    parser.add_argument('--module-source', type=Path, default=ROOT / 'compiler/prelude/Data/Bytes/Split.cinch')
    parser.add_argument('--view-source', type=Path, default=ROOT / 'compiler/prelude/Data/Bytes/View.cinch')
    parser.add_argument('--runtime-archive', type=Path, required=True)
    parser.add_argument('--native-interfaces', type=Path)
    parser.add_argument('--native-objects', type=Path)
    parser.add_argument('--artifacts', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--opt-level', choices=['0', '2'], default='0')
    parser.add_argument('--groups', nargs='+', choices=['rows', 'owners', 'deep', 'progress'], default=['rows', 'owners', 'deep'])
    parser.add_argument('--first', type=int, default=0)
    parser.add_argument('--limit', type=int)
    parser.add_argument('--batch-size', type=int, default=16)
    parser.add_argument('--build-only', action='store_true')
    parser.add_argument('--reuse-receipt', type=Path)
    args = parser.parse_args()
    fixture = json.loads(FIXTURE.read_text())
    crosscheck(fixture['rows'])
    rows = fixture['rows'][args.first:]
    if args.limit is not None:
        rows = rows[:args.limit]
    report = {'frontend_sha256':digest(args.compiler), 'runtime_sha256':digest(args.runtime_archive),
              'fixture_sha256':digest(FIXTURE), 'source_sha256':digest(args.module_source),
              'source_distributed_only':True, 'compiler_stack_bytes':67108864, 'program_stack_bytes':8388608,
              'opt_level':args.opt_level, 'results':[], 'view_sha256':digest(args.view_source), 'status':'prepared',
              'compiled_rc_debug':bool(args.native_interfaces),
              'rust_cli_profile_note':'Default rc_debug=false; dynamic normal/poison profiles are separate'}
    def save() -> None:
        """Retain the exact current gate boundary before expensive work and after each observation."""
        args.output.parent.mkdir(parents=True, exist_ok=True)
        args.output.write_text(json.dumps(report, indent=2) + '\n')
    if args.native_interfaces:
        assert args.native_objects
        report['interfaces_sha256'] = digest(args.native_interfaces)
        report['objects_sha256'] = digest(args.native_objects)
    for group in args.groups:
        checks = ([expression(row) for row in rows] if group == 'rows' else ['True'] if group == 'deep' else ['byte-split-test-progress ()'] if group == 'progress' else [name + ' ()' for name in
            ['lifecycle-case', 'mutation-case', 'multibyte-mutation-case', 'aliased-separator-case', 'yielded-separator-case', 'empty-mutation-case', 'lines-mutation-case', 'temporary-owner-case', 'counter-boundary-case']])
        width = args.batch_size if group == 'rows' else 1
        work = args.artifacts / group
        work.mkdir(parents=True, exist_ok=True)
        text = consumer(checks, width)
        entry = work / 'Main.cinch'
        entry.write_text(text)
        module_source = args.module_source
        if group == 'progress':
            module_source = work / 'private-source' / 'Data.Bytes.Split.cinch'
            module_source.parent.mkdir(parents=True, exist_ok=True)
            hook = ROOT / 'compiler/tests/fixtures/byte-split-private-progress.cinchpart'
            module_source.write_text(args.module_source.read_text() + hook.read_text())
            report['test_only_private_progress_hook_sha256'] = digest(hook)
        deps = source_closure(text, {'Data.Bytes.Split':module_source, 'Data.Bytes.View':args.view_source})
        for name, path in deps.items():
            shutil.copyfile(path, work / (name + '.cinch'))
        exe = (work / 'consumer').resolve()
        if args.native_interfaces:
            bundle = work / 'Bundle.cinch'
            bundle.write_text('\n'.join(path.read_text() for path in reversed(list(deps.values()))) + '\n' + text)
            command = [str(args.compiler.resolve()), '--emit-ir', '--rc-debug', '--interfaces', str(args.native_interfaces.resolve()),
                       '--objects', str(args.native_objects.resolve()), str(bundle.resolve()), str(exe.with_suffix('.ll'))]
        else:
            command = [str(args.compiler.resolve()), 'build', str(entry.resolve()), '-o', str(exe), '--opt-level', args.opt_level,
                       '--runtime-archive', str(args.runtime_archive.resolve())]
        report['status'] = 'preparing-' + group
        report['pending_group'] = {'group':group,'entry_sha256':digest(entry),
                                   'dependencies':{name:digest(path) for name,path in deps.items()},'command':command}
        save()
        if args.reuse_receipt:
            previous = json.loads(args.reuse_receipt.read_text())
            for key in ['frontend_sha256', 'runtime_sha256', 'fixture_sha256', 'source_sha256', 'opt_level']:
                assert previous[key] == report[key], key
            before = next(value for value in previous['results'] if value['group'] == group)
            assert before['entry_sha256'] == digest(entry)
            assert before['dependencies'] == {name:digest(path) for name,path in deps.items()}
            assert before['executable_sha256'] == digest(exe)
            assert before['llvm_sha256'] == digest(exe.with_suffix('.ll'))
            if args.native_interfaces:
                assert previous['interfaces_sha256'] == report['interfaces_sha256']
                assert previous['objects_sha256'] == report['objects_sha256']
        else:
            try:
                compiled = run(command)
            except RuntimeError as error:
                diagnostics = exe.with_suffix('.ll.diagnostics')
                report['status'] = 'failed-compile-' + group
                report['diagnostics'] = diagnostics.read_text() if diagnostics.exists() else str(error)
                save()
                raise RuntimeError(str(error) + '\n' + (diagnostics.read_text() if diagnostics.exists() else '')) from error
            (work / 'compile.log').write_text(compiled.stdout + compiled.stderr)
        if args.native_interfaces and not args.reuse_receipt:
            configuration = json.loads(args.native_objects.read_text())
            assert digest(Path(configuration['object'])) == configuration['object_sha256']
            linked = run([os.environ.get('CINCH_CLANG', 'clang'), f'-O{args.opt_level}', str(exe.with_suffix('.ll')),
                          configuration['object'], str(args.runtime_archive.resolve()), '-o', str(exe), '-ldl', '-lpthread', '-lm'])
            (work / 'link.log').write_text(linked.stdout + linked.stderr)
            report['native_object_sha256'] = digest(Path(configuration['object']))
        profiles = []
        current = {'group':group,'cases':len(checks),'entry_sha256':digest(entry),
                   'llvm_sha256':digest(exe.with_suffix('.ll')),'executable_sha256':digest(exe),
                   'dependencies':{name:digest(path) for name,path in deps.items()},'profiles':profiles}
        report['results'].append(current)
        report['status'] = 'running-' + group
        save()
        for debug in ([] if args.build_only else [False, True]):
            for batch in range((len(checks) + width - 1) // width):
                arguments = [str(exe), 'deep', '16384'] if group == 'deep' else [str(exe), str(batch)]
                result = run(arguments, env={**os.environ,'CINCH_RC_DEBUG':str(int(debug)),
                             'CINCH_HEAP_LIMIT':'256M' if debug else '32M','CINCH_STACK_SIZE':'8M','CINCH_LEAK_CHECK':'1'})
                assert 'leaked=0' in result.stderr, result.stderr
                profiles.append({'debug':debug,'batch':batch,'zero_leaks':True})
                save()
        print(f'{group}: {len(checks)} complete output/control cases, ' + ('build only' if args.build_only else 'normal+poison zero leaks'), flush=True)
    report['build_only'] = args.build_only
    report['status'] = 'passed'
    report.pop('pending_group', None)
    save()


if __name__ == '__main__':
    main()
