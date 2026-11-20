#!/usr/bin/env python3
"""Compile real sealed ASCII adapter consumers and verify reachable SIMD/scalar code before foreign object generation."""
from __future__ import annotations
import argparse
import json
import os
from pathlib import Path
import re
import shutil

from bench_stdlib import source_closure
from validate_byte_view import digest, run

ROOT = Path(__file__).resolve().parents[1]
FIXTURE = ROOT/'seed/tests/fixtures/byte-view-ascii-run-portable.cinch'


def definitions(text: str) -> dict[str,str]:
    """Read complete actual definitions rather than accepting unrelated runtime-header instructions."""
    result = {}
    position = 0
    for line in text.splitlines(keepends=True):
        if line.startswith('define '):
            match = re.search(r'@"([^"]+)"\(',line)
            if match:
                end = position+text[position:].index('\n}')+2
                result[match[1]] = text[position:end]
        position += len(line)
    return result


def path_to(functions: dict[str,str], start: str, wanted: str) -> list[str] | None:
    """Follow actual direct callee operands through retained or inlined public/private adapters."""
    pending = [[start]]
    seen = set()
    while pending:
        path = pending.pop()
        symbol = path[-1]
        if symbol == wanted:
            return path
        if symbol in seen:
            continue
        seen.add(symbol)
        for line in functions[symbol].splitlines():
            if 'call ' in line:
                for target in re.findall(r'@"([^"]+)"\(',line):
                    if target in functions:
                        pending.append(path+[target])
    return None


def inspect(text: str) -> dict:
    """Require the real entrypoint to reach both packed and scalar loads/classifiers with no hidden kernel stores."""
    functions = definitions(text)
    paths = {}
    for worker in ['ascii-space-sixteen.llvm','ascii-space-byte.llvm','load-byte.llvm']:
        name = 'Data.Bytes.View.'+worker
        path = path_to(functions,'Main.main',name)
        assert path is not None, f'actual public consumer does not reach {name}'
        paths[worker] = path
    vector = functions['Data.Bytes.View.ascii-space-sixteen.llvm']
    for instruction in ['load <16 x i8>','align 1','icmp ule <16 x i8>','icmp ult <16 x i8>','icmp eq <16 x i8>','bitcast <16 x i1>']:
        assert instruction in vector,instruction
    assert 'store ' not in vector
    scalar = functions['Data.Bytes.View.ascii-space-byte.llvm']
    assert 'icmp ule i64' in scalar and 'icmp ult i64' in scalar
    assert 'load i8,' in functions['Data.Bytes.View.load-byte.llvm']
    return dict(actual_paths=paths,vector_load_bytes=16,vector_alignment=1,kernel_stores=False)


def main() -> None:
    """Keep exact source/target/compiler identities and every failure artifact before any reachability or object assertion."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--compiler',type=Path,required=True)
    parser.add_argument('--runtime-archive',type=Path)
    parser.add_argument('--target',choices=['host','wasm32-unknown-unknown','aarch64-unknown-linux-gnu'],required=True)
    parser.add_argument('--artifacts',type=Path,required=True)
    parser.add_argument('--output',type=Path,required=True)
    args = parser.parse_args()
    work = args.artifacts.resolve()
    work.mkdir(parents=True,exist_ok=True)
    source = FIXTURE.read_text()
    entry = work/'Main.cinch'
    entry.write_text(source)
    dependencies = source_closure(source)
    for name,path in dependencies.items():
        shutil.copyfile(path,work/(name+'.cinch'))
    ir = work/'consumer.ll'
    executable = work/'consumer'
    report = dict(scope='actual complete sealed adapter consumer',target=args.target,compiler_sha256=digest(args.compiler),
                  entry_sha256=digest(entry),dependencies={name:digest(path) for name,path in dependencies.items()},
                  compiler_main_stack_bytes=67108864,program_stack_bytes=8388608,compiled_rc_debug=False,profiles=[],status='prepared')
    def save() -> None:
        """Persist exact stage boundaries and failures while leaving source/IR/object evidence plain."""
        args.output.parent.mkdir(parents=True,exist_ok=True)
        args.output.write_text(json.dumps(report,indent=2)+'\n')
    if args.target == 'host':
        assert args.runtime_archive
        report['runtime_sha256'] = digest(args.runtime_archive)
        command = [str(args.compiler.resolve()),'build',str(entry),'-o',str(executable),'--opt-level','2','--runtime-archive',str(args.runtime_archive.resolve())]
    else:
        command = [str(args.compiler.resolve()),'emit-ir',str(entry),'-o',str(ir),'--target',args.target]
    report['command'] = command
    save()
    try:
        compiled = run(command)
        (work/'compile.log').write_text(compiled.stdout+compiled.stderr)
        report.update(llvm_sha256=digest(ir),status='emitted-before-graph-check')
        save()
        text = ir.read_text()
        if args.target!='host':
            assert f'target triple = "{args.target}"' in text
        report['graph'] = inspect(text)
        save()
        if args.target == 'host':
            report['executable_sha256'] = digest(executable)
            for debug in [False,True]:
                observed = run([str(executable)],env={**os.environ,'CINCH_RC_DEBUG':str(int(debug)),
                               'CINCH_HEAP_LIMIT':'256M' if debug else '32M','CINCH_STACK_SIZE':'8M','CINCH_LEAK_CHECK':'1'})
                (work/f'run-{debug}.log').write_text(observed.stdout+observed.stderr)
                assert 'leaked=0' in observed.stderr
                report['profiles'].append(dict(debug=debug,zero_leaks=True))
                save()
        else:
            object_file = work/'consumer.o'
            object_command = [os.environ.get('CINCH_CLANG','clang'),'-target',args.target,'-O2','-c','-x','ir',str(ir),'-o',str(object_file)]
            report['object_command'] = object_command
            save()
            linked = run(object_command)
            (work/'object.log').write_text(linked.stdout+linked.stderr)
            report['object_sha256'] = digest(object_file)
            header = object_file.read_bytes()[:20]
            if args.target=='wasm32-unknown-unknown':
                assert header[:4]==b'\0asm'
            else:
                assert header[:6]==b'\x7fELF\x02\x01' and int.from_bytes(header[18:20],'little')==183
            report['object_header_verified'] = True
            report['foreign_runtime_executed'] = False
        report['status'] = 'passed'
        save()
    except Exception as error:
        report.update(status='failed',diagnostics=str(error))
        (work/'failure.log').write_text(str(error))
        save()
        raise


if __name__ == '__main__':
    main()
