#!/usr/bin/env python3
"""Qualify complete retained-field consumers and their real portable SIMD/scalar/effect call graph."""
from __future__ import annotations
import argparse
import json
import os
from pathlib import Path
import re
import shutil

from bench_stdlib import source_closure
from validate_byte_ascii_portable import definitions, path_to
from validate_byte_view import digest, run

ROOT = Path(__file__).resolve().parents[1]


def inspect(text: str) -> dict:
    """Require reachable complete collectors, effectful traversal, packed loads and Unicode decoding."""
    functions = definitions(text)
    paths = {}
    for symbol in [
        'Data.Bytes.Fields.collect-space', 'Data.Bytes.Fields.collect-with',
        'Data.Bytes.Fields.visit-with', 'Data.Bytes.Fields.scan-space',
        'Data.Bytes.Fields.scan-with', 'Data.Bytes.View.ascii-space-sixteen.llvm',
        'Data.Bytes.View.ascii-space-byte.llvm', 'Data.Bytes.View.load-byte.llvm',
        'Data.Bytes.Utf8.utf8-decode-between', 'Data.Unicode.Kernel.unicode-is-space-code',
    ]:
        # The fixed pure predicate creates the compiler's actual collector specialization.
        # Every accepted body still needs a real direct-call path from the complete consumer.
        candidates = ([name for name in functions if name == symbol or
                       re.fullmatch(re.escape(symbol)+r'\.sp[0-9]+', name)]
                      if symbol == 'Data.Bytes.Fields.collect-with' else [symbol])
        path = next((found for name in candidates
                     if (found := path_to(functions, 'Main.main', name)) is not None), None)
        assert path is not None, f'complete public Fields consumer cannot reach {symbol}'
        paths[symbol] = path
    vector = functions['Data.Bytes.View.ascii-space-sixteen.llvm']
    for instruction in ['load <16 x i8>', 'align 1', 'icmp ule <16 x i8>', 'icmp ult <16 x i8>']:
        assert instruction in vector, instruction
    assert 'store ' not in vector
    return dict(actual_paths=paths, vector_load_bytes=16, vector_alignment=1, hidden_kernel_stores=False)


def main() -> None:
    """Retain source, actual full IR, objects and failure logs before asserting target or graph coverage."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--compiler', type=Path, required=True)
    parser.add_argument('--runtime-archive', type=Path)
    parser.add_argument('--target', choices=['host', 'wasm32-unknown-unknown', 'aarch64-unknown-linux-gnu'], required=True)
    parser.add_argument('--artifacts', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    work = args.artifacts.resolve()
    work.mkdir(parents=True, exist_ok=True)
    source = (ROOT/'compiler/tests/fixtures/byte-fields-portable.cinch').read_text()
    entry = work/'Main.cinch'
    entry.write_text(source)
    closure = source_closure(source)
    for name, path in closure.items():
        shutil.copyfile(path, work/(name+'.cinch'))
    ir, executable = work/'consumer.ll', work/'consumer'
    report = dict(scope='complete Fields collector and effectful predicate/yield consumer', target=args.target,
                  compiler_sha256=digest(args.compiler), entry_sha256=digest(entry),
                  dependencies={name:digest(path) for name,path in closure.items()}, profiles=[],
                  compiler_main_stack_bytes=67108864, program_stack_bytes=8388608, compiled_rc_debug=False, status='prepared')
    def save() -> None:
        """Persist every stage before running the next assertion or external command."""
        args.output.parent.mkdir(parents=True, exist_ok=True)
        args.output.write_text(json.dumps(report, indent=2)+'\n')
    if args.target == 'host':
        assert args.runtime_archive
        report['runtime_sha256'] = digest(args.runtime_archive)
        command = [str(args.compiler.resolve()), 'build', str(entry), '-o', str(executable), '--opt-level', '2',
                   '--runtime-archive', str(args.runtime_archive.resolve())]
    else:
        command = [str(args.compiler.resolve()), 'emit-ir', str(entry), '-o', str(ir), '--target', args.target]
    report['command'] = command
    save()
    try:
        compiled = run(command)
        (work/'compile.log').write_text(compiled.stdout+compiled.stderr)
        report.update(status='emitted-before-graph-check', llvm_sha256=digest(ir))
        save()
        text = ir.read_text()
        if args.target != 'host':
            assert f'target triple = "{args.target}"' in text
        report['graph'] = inspect(text)
        save()
        if args.target == 'host':
            report['executable_sha256'] = digest(executable)
            for poison in [False, True]:
                observed = run([str(executable)], env={**os.environ, 'CINCH_RC_DEBUG':str(int(poison)),
                               'CINCH_HEAP_LIMIT':'256M' if poison else '32M', 'CINCH_STACK_SIZE':'8M', 'CINCH_LEAK_CHECK':'1'})
                (work/f'run-{poison}.log').write_text(observed.stdout+observed.stderr)
                assert 'leaked=0' in observed.stderr
                report['profiles'].append(dict(debug=poison, zero_leaks=True))
                save()
        else:
            object_file = work/'consumer.o'
            command = [os.environ.get('CINCH_CLANG', 'clang'), '-target', args.target, '-O2', '-c', '-x', 'ir', str(ir), '-o', str(object_file)]
            report['object_command'] = command
            save()
            compiled = run(command)
            (work/'object.log').write_text(compiled.stdout+compiled.stderr)
            header = object_file.read_bytes()[:20]
            if args.target == 'wasm32-unknown-unknown':
                assert header[:4] == b'\0asm'
            else:
                assert header[:6] == b'\x7fELF\x02\x01' and int.from_bytes(header[18:20], 'little') == 183
            report.update(object_sha256=digest(object_file), object_header_verified=True, foreign_runtime_executed=False)
        report['status'] = 'passed'
        save()
    except Exception as error:
        report.update(status='failed', diagnostics=str(error))
        (work/'failure.log').write_text(str(error))
        save()
        raise


if __name__ == '__main__':
    main()
