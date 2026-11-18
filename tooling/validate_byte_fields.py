#!/usr/bin/env python3
"""Retain complete byte-field outputs and effect traces under explicit compiler/runtime epochs."""
from __future__ import annotations
import argparse
import json
import os
from pathlib import Path
import shutil

from bench_stdlib import source_closure
from generate_byte_fields_vectors import crosscheck, effect_control
from validate_byte_view import digest, run

ROOT = Path(__file__).resolve().parents[1]
FIXTURE = ROOT / 'compiler/tests/fixtures/byte-fields-vectors.json'
MODES = {'default': 0, 'space': 1, 'comma': 2, 'error': 3, 'nonascii': 4, 'even': 5}


def strings(values: list[str]) -> str:
    """Encode literal full hexadecimal fragments without a lossy digest."""
    return '[' + ','.join(json.dumps(value) for value in values) + ']'


def observations(events: list[dict], mutable: bool = False) -> str:
    """Encode exact callback order, decoded scalars and complete callback-time bytes."""
    values = []
    for event in events:
        kind = event['kind']
        if mutable:
            number = {"predicate":0,"yield":1,"mutation":2}[kind]
            values.append(f'(Callback {number} '
                          f'{event.get("rune",0)} {json.dumps(event.get("value",""))} {json.dumps(event["input"])})')
        elif kind == 'predicate':
            values.append(f'(Predicate {event["rune"]})')
        else:
            values.append(f'(Output {json.dumps(event["value"])})')
    return '[' + ','.join(values) + ']'


def expression(row: dict) -> str:
    """Observe eager and persistent outputs plus both fresh invocations of the same factory."""
    return (f'fields-case {json.dumps(row["input"])} {row["start"]} {row["end"]} {MODES[row["mode"]]} '
            f'{max(0,row["take"])} {str(row["take"] != 0)} {strings(row["eager"])} '
            f'{observations(row["eager_events"])} {observations(row["invocations"][0]["events"])}')


def effect_expression(row: dict, control: int) -> str:
    """Retain alias contents after callbacks, not only the bytes observed when initially yielded."""
    result = f'effect-case {json.dumps(row["input"])} {control} {MODES[row["mode"]]} {max(0,row["take"])}'
    for invocation in row['invocations']:
        result += (f' {observations(invocation["events"],True)} {strings(invocation["held_after"])} '
                   f'{json.dumps(invocation["input_after"])}')
    return result


def consumer(checks: list[str], width: int) -> str:
    """Select bounded quarantine processes while preserving genuine mutation effects in every call."""
    text = (ROOT / 'compiler/tests/fixtures/byte-fields-consumer.cinchpart').read_text()
    dispatch = 'False'
    for first in reversed(range(0,len(checks),width)):
        group = first // width
        body = '\n'.join(f'    let check{index} = {value} in' for index,value in enumerate(checks[first:first+width]))
        verdict = ' && '.join(f'check{index}' for index in range(len(checks[first:first+width])))
        text += (f'\n  -- | Run complete output/effect group {group}.\n  group{group} :: forall (r :: Region). Unit ->{{Mut r}} Bool\n'
                 f'  group{group} unit =\n{body}\n    {verdict}\n')
        dispatch = f'if index == "{group}" then group{group} () else {dispatch}'
    return text + ('\n  -- | Select one bounded memory profile without weakening callback effects.\n'
                   '  main :: Unit ->{IO r, Mut s} Int\n'
                   '  main unit = with (io = newIO ()) in case process-args () of\n'
                   '    Ok (Cons index (Cons size Nil)) -> if index == "deep" then (let accepted = deep-case (decimal size) in if accepted then 0 else 1) else 2\n'
                   '    Ok (Cons index Nil) -> let accepted = ' + dispatch + ' in if accepted then 0 else 1\n'
                   '    other -> 2\n')


def main() -> None:
    """Keep source, compiler, provider, compile-profile and dynamic runtime profiles separately identifiable."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--compiler',type=Path,required=True)
    parser.add_argument('--artifacts',type=Path,required=True)
    parser.add_argument('--output',type=Path,required=True)
    parser.add_argument('--runtime-archive',type=Path)
    parser.add_argument('--native-interfaces',type=Path)
    parser.add_argument('--native-objects',type=Path)
    parser.add_argument('--reuse-native-report',type=Path)
    parser.add_argument('--utf8-source',type=Path)
    parser.add_argument('--opt-level',choices=['0','2'],default='0')
    parser.add_argument('--group',choices=['rows','effects','owners','deep','controls','mixed'],default='rows')
    parser.add_argument('--size',type=int,default=65536)
    parser.add_argument('--first',type=int,default=0)
    parser.add_argument('--limit',type=int)
    parser.add_argument('--batch-size',type=int,default=8)
    parser.add_argument('--check-only',action='store_true')
    args = parser.parse_args()
    fixture = json.loads(FIXTURE.read_text())
    crosscheck(fixture['rows'])
    for row in fixture['effect_controls']:
        expected = effect_control(row)
        assert row['invocations'] == expected['invocations']
    checks = (['True'] if args.group in ['deep','mixed'] else [expression(row) for row in fixture['rows']] if args.group == 'rows' else
              ['owner-case ()','stateful-case ()'] if args.group == 'owners' else
              [effect_expression(row,index) for index,row in enumerate(fixture['effect_controls'])])
    if args.group == 'controls':
        checks += ['owner-case ()','stateful-case ()']
    checks = checks[args.first:]
    if args.limit is not None:
        checks = checks[:args.limit]
    assert checks
    work = args.artifacts.resolve()
    work.mkdir(parents=True,exist_ok=True)
    text = ((ROOT/'compiler/tests/fixtures/byte-fields-mixed-runtime.cinch').read_text()
            if args.group == 'mixed' else consumer(checks,args.batch_size))
    entry = work/'Main.cinch'
    entry.write_text(text)
    dependencies = source_closure(text, {'Data.Bytes.Utf8':args.utf8_source} if args.utf8_source else None)
    for name,path in dependencies.items():
        shutil.copyfile(path,work/(name+'.cinch'))
    exe = work/'consumer'
    report = dict(group=args.group,cases=len(checks),fixture_sha256=digest(FIXTURE),compiler_sha256=digest(args.compiler),
                  entry_sha256=digest(entry),dependencies={name:digest(path) for name,path in dependencies.items()},
                  compiled_rc_debug=None if args.check_only else bool(args.native_interfaces),compiler_stack_bytes=67108864,
                  program_stack_bytes=8388608,profiles=[],status='prepared')
    def save() -> None:
        """Keep a durable result before compilation and after every memory observation."""
        args.output.parent.mkdir(parents=True,exist_ok=True)
        args.output.write_text(json.dumps(report,indent=2)+'\n')
    save()
    if args.check_only:
        assert not args.native_interfaces, 'native source qualification has a separate explicit tuple'
        command = [str(args.compiler.resolve()),'check',str(entry)]
    elif args.native_interfaces:
        assert args.native_objects and args.runtime_archive
        report.update(interfaces_sha256=digest(args.native_interfaces),objects_sha256=digest(args.native_objects))
        bundle = work/'Bundle.cinch'
        bundle.write_text('\n'.join(path.read_text() for path in reversed(list(dependencies.values())))+'\n'+text)
        command = [str(args.compiler.resolve()),'--emit-ir','--rc-debug','--interfaces',str(args.native_interfaces.resolve()),
                   '--objects',str(args.native_objects.resolve()),str(bundle),str(exe.with_suffix('.ll'))]
    else:
        assert args.runtime_archive
        command = [str(args.compiler.resolve()),'build',str(entry),'-o',str(exe),'--opt-level',args.opt_level,
                   '--runtime-archive',str(args.runtime_archive.resolve())]
    report['command'] = command
    report['opt_level'] = None if args.check_only else args.opt_level
    report['runtime_size'] = args.size if args.group in ['deep','controls'] else None
    report['expected_stdout'] = 'a b a b ' if args.group == 'mixed' else ''
    report['selection'] = dict(first=args.first,limit=args.limit,batch_size=args.batch_size)
    report['generator_sha256'] = digest(Path(__file__))
    save()
    if args.reuse_native_report:
        assert args.native_interfaces and not args.check_only and args.runtime_archive
        previous = json.loads(args.reuse_native_report.read_text())
        assert previous['status'] == 'passed' and previous['compiled_rc_debug'] is True
        assert previous['profiles'] and all(value['zero_leaks'] for value in previous['profiles'])
        for key in ['compiler_sha256','entry_sha256','dependencies','interfaces_sha256','objects_sha256']:
            assert previous[key] == report[key], f'Native emission provenance changed: {key}'
        assert previous['runtime_sha256'] == digest(args.runtime_archive)
        original = args.reuse_native_report.with_suffix('')/'consumer.ll'
        assert digest(original) == previous['llvm_sha256']
        report['reused_native_emission'] = dict(report=str(args.reuse_native_report.resolve()),
            report_sha256=digest(args.reuse_native_report),llvm_sha256=previous['llvm_sha256'],
            original_emission_command=previous['command'])
        report['emission_command_not_executed'] = command
        report['command'] = ['copy-verified-native-ir',str(original.resolve()),str(exe.with_suffix('.ll'))]
        shutil.copyfile(original,exe.with_suffix('.ll'))
        (work/'compile.log').write_text('Reused verified exact Native IR; compiler was not executed for this link profile.\n')
        save()
    else:
        try:
            compiled = run(command)
            (work/'compile.log').write_text(compiled.stdout+compiled.stderr)
        except RuntimeError as error:
            diagnostics = exe.with_suffix('.ll.diagnostics')
            detail = str(error) + ('\n'+diagnostics.read_text() if diagnostics.exists() else '')
            (work/'compile.log').write_text(detail)
            report.update(status='compile-failed',diagnostics=detail)
            save()
            raise
    if args.check_only:
        report['status'] = 'source-checked-only'
        save()
        return
    report['runtime_sha256'] = digest(args.runtime_archive)
    if args.native_interfaces:
        configuration = json.loads(args.native_objects.read_text())
        assert digest(Path(configuration['object'])) == configuration['object_sha256']
        try:
            linked = run([os.environ.get('CINCH_CLANG','clang'),'-O'+args.opt_level,str(exe.with_suffix('.ll')),
                          configuration['object'],str(args.runtime_archive.resolve()),'-o',str(exe),'-ldl','-lpthread','-lm'])
        except RuntimeError as error:
            (work/'link.log').write_text(str(error))
            report.update(status='link-failed',diagnostics=str(error))
            save()
            raise
        (work/'link.log').write_text(linked.stdout+linked.stderr)
        report['native_object_sha256'] = configuration['object_sha256']
    report.update(llvm_sha256=digest(exe.with_suffix('.ll')),executable_sha256=digest(exe),status='running')
    save()
    for debug in [False,True]:
        batches = list(range((len(checks)+args.batch_size-1)//args.batch_size))
        if args.group == 'controls':
            batches.append('deep')
        for batch in batches:
            try:
                arguments = [str(exe),'deep',str(args.size)] if args.group == 'deep' or batch == 'deep' else [str(exe),str(batch)]
                result = run(arguments,env={**os.environ,'CINCH_RC_DEBUG':str(int(debug)),
                             'CINCH_HEAP_LIMIT':'256M' if debug else '32M','CINCH_STACK_SIZE':'8M','CINCH_LEAK_CHECK':'1'})
                if 'leaked=0' not in result.stderr or result.stdout != report['expected_stdout']:
                    raise RuntimeError(f'exact stdout/leak observation failed: {result.stdout!r}\n{result.stderr}')
            except RuntimeError as error:
                (work/f'run-{debug}-{batch}.log').write_text(str(error))
                report.update(status='runtime-failed',failed_profile=dict(debug=debug,batch=batch),diagnostics=str(error))
                save()
                raise
            report['profiles'].append(dict(debug=debug,batch=batch,stdout=result.stdout,zero_leaks=True))
            save()
    report['status'] = 'passed'
    save()


if __name__ == '__main__':
    main()
