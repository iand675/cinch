#!/usr/bin/env python3
"""Validate sealed ASCII run endpoints independently of the Unicode field splitter."""
from __future__ import annotations
import argparse
import json
import os
from pathlib import Path
import shutil

from bench_stdlib import source_closure
from validate_byte_view import digest, run

ROOT = Path(__file__).resolve().parents[1]


def checks() -> list[str]:
    """Cover all byte values, sixteen alignments, every position and each mismatch lane at both logical fringes."""
    result = []
    for value in range(256):
        white = value in [9,10,11,12,13,32]
        result.append(f'uniform-case "{bytes([value]*49).hex()}" {white} {value < 128 and not white}')
    for fill,spaces in [(32,True),(122,False)]:
        for stop in [9,32,122,128,133,255]:
            if stop < 128 and ((stop in [9,10,11,12,13,32]) == spaces):
                continue
            for size in [15,16,17,31,32,33,63,64,65]:
                for lane in range(min(size,17)):
                    raw = bytearray([fill]*(size+2))
                    raw[0] = raw[-1] = 255
                    raw[lane+1] = stop
                    result.append(f'mismatch-case "{raw.hex()}" {lane} {spaces} 1 {size+1}')
    result.append('ownership-case ()')
    return result


def consumer(expressions: list[str], width: int) -> str:
    """Choose bounded memory groups without changing the observer or its heap limits."""
    source = (ROOT/'seed/tests/fixtures/byte-view-ascii-run-consumer.cinchpart').read_text()
    dispatch = 'False'
    for first in reversed(range(0,len(expressions),width)):
        number = first//width
        source += (f'\n  -- | Check complete ASCII-run group {number}.\n  group{number} :: Unit -> Bool\n'
                   f'  group{number} unit = all [\n' + ',\n'.join('    ('+value+')' for value in expressions[first:first+width])+'\n    ]\n')
        dispatch = f'if index == "{number}" then group{number} () else {dispatch}'
    return source + ('\n  -- | Select one independently bounded endpoint and ownership group.\n'
                     '  main :: Unit ->{IO r} Int\n'
                     '  main unit = with (io = newIO ()) in case process-args () of\n'
                     '    Ok (Cons index Nil) -> let accepted = '+dispatch+' in if accepted then 0 else 1\n    other -> 2\n')


def main() -> None:
    """Preserve exact seed/source/runtime identities and all failed diagnostics before asserting results."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--compiler',type=Path,required=True)
    parser.add_argument('--runtime-archive',type=Path)
    parser.add_argument('--native-interfaces',type=Path)
    parser.add_argument('--native-objects',type=Path)
    parser.add_argument('--artifacts',type=Path,required=True)
    parser.add_argument('--output',type=Path,required=True)
    parser.add_argument('--first',type=int,default=0)
    parser.add_argument('--limit',type=int)
    parser.add_argument('--case-indices',type=int,nargs='+')
    parser.add_argument('--batch-size',type=int,default=8)
    parser.add_argument('--opt-level',choices=['0','2'],default='0')
    parser.add_argument('--check-only',action='store_true')
    args = parser.parse_args()
    independent = checks()
    if args.case_indices is not None:
        assert args.first == 0 and args.limit is None
        expressions = [independent[index] for index in args.case_indices]
    else:
        expressions = independent[args.first:]
    if args.limit is not None:
        expressions = expressions[:args.limit]
    assert expressions
    work = args.artifacts.resolve()
    work.mkdir(parents=True,exist_ok=True)
    source = consumer(expressions,args.batch_size)
    entry = work/'Main.cinch'
    entry.write_text(source)
    dependencies = source_closure(source)
    for name,path in dependencies.items():
        shutil.copyfile(path,work/(name+'.cinch'))
    executable = work/'consumer'
    report = dict(compiler_sha256=digest(args.compiler),entry_sha256=digest(entry),cases=len(expressions),
                  dependencies={name:digest(path) for name,path in dependencies.items()},compiled_rc_debug=None if args.check_only else bool(args.native_interfaces),
                  compiler_stack_bytes=67108864,program_stack_bytes=8388608,profiles=[],status='prepared')
    def save() -> None:
        """Retain each exact boundary, including failed compiler diagnostics."""
        args.output.parent.mkdir(parents=True,exist_ok=True)
        args.output.write_text(json.dumps(report,indent=2)+'\n')
    if args.check_only:
        assert not args.native_interfaces
        command = [str(args.compiler.resolve()),'check',str(entry)]
    elif args.native_interfaces:
        assert args.native_objects and args.runtime_archive
        report.update(interfaces_sha256=digest(args.native_interfaces),objects_sha256=digest(args.native_objects))
        bundle = work/'Bundle.cinch'
        bundle.write_text('\n'.join(path.read_text() for path in reversed(list(dependencies.values())))+'\n'+source)
        command = [str(args.compiler.resolve()),'--emit-ir','--rc-debug','--interfaces',str(args.native_interfaces.resolve()),
                   '--objects',str(args.native_objects.resolve()),str(bundle),str(executable.with_suffix('.ll'))]
    else:
        assert args.runtime_archive
        command = [str(args.compiler.resolve()),'build',str(entry),'-o',str(executable),'--opt-level',args.opt_level,
                   '--runtime-archive',str(args.runtime_archive.resolve())]
    report['command'] = command
    report['opt_level'] = None if args.check_only else args.opt_level
    report['case_indices'] = args.case_indices
    report['selection'] = dict(first=args.first,limit=args.limit,batch_size=args.batch_size)
    report['generator_sha256'] = digest(Path(__file__))
    save()
    try:
        compiled = run(command)
        (work/'compile.log').write_text(compiled.stdout+compiled.stderr)
    except RuntimeError as error:
        diagnostics = executable.with_suffix('.ll.diagnostics')
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
            linked = run([os.environ.get('CINCH_CLANG','clang'),'-O'+args.opt_level,str(executable.with_suffix('.ll')),
                          configuration['object'],str(args.runtime_archive.resolve()),'-o',str(executable),'-ldl','-lpthread','-lm'])
        except RuntimeError as error:
            (work/'link.log').write_text(str(error))
            report.update(status='link-failed',diagnostics=str(error))
            save()
            raise
        (work/'link.log').write_text(linked.stdout+linked.stderr)
        report['native_object_sha256'] = configuration['object_sha256']
    report.update(llvm_sha256=digest(executable.with_suffix('.ll')),executable_sha256=digest(executable),status='running')
    save()
    for debug in [False,True]:
        for batch in range((len(expressions)+args.batch_size-1)//args.batch_size):
            try:
                observed = run([str(executable),str(batch)],env={**os.environ,'CINCH_RC_DEBUG':str(int(debug)),
                               'CINCH_HEAP_LIMIT':'256M' if debug else '32M','CINCH_STACK_SIZE':'8M','CINCH_LEAK_CHECK':'1'})
            except RuntimeError as error:
                (work/f'run-{debug}-{batch}.log').write_text(str(error))
                report.update(status='runtime-failed',failed_profile=dict(debug=debug,batch=batch),diagnostics=str(error))
                save()
                raise
            assert 'leaked=0' in observed.stderr,observed.stderr
            report['profiles'].append(dict(debug=debug,batch=batch,zero_leaks=True))
            save()
    report['status'] = 'passed'
    save()


if __name__ == '__main__':
    main()
