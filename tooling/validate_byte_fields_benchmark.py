#!/usr/bin/env python3
"""Validate complete public field-pipeline outputs and retained inputs before any performance comparison."""
from __future__ import annotations
import argparse
import json
import os
from pathlib import Path
import shutil

from bench_stdlib import source_closure
from reference_byte_fields_benchmark import expected
from validate_byte_view import digest, run

ROOT = Path(__file__).resolve().parents[1]
KINDS = ['eager', 'producer', 'visit', 'predicate', 'effect']
PATTERNS = ['space', 'text', 'dense', 'fringe', 'unicode']
SIZES = [0, 1, 15, 16, 17, 31, 32, 33, 63, 64, 65, 4096, 65536]


def scalar_source() -> Path:
    """Require the scalar control to change only the one documented private ASCII skipping helper."""
    original = (ROOT/'compiler/prelude/Data/Bytes/Fields.cinch').read_text()
    old = '''  -- | End one current ASCII class run through the sealed SIMD observation.
  ascii-run-end :: ByteView -> Nat -> Bool -> Nat
  ascii-run-end input position spaces = case byte-view-ascii-space-run-end input position spaces of
    None -> position
    Some end -> end'''
    new = '''  -- | Preserve scalar scanning as an isolated benchmark control with the same field and callback semantics.
  ascii-run-end :: ByteView -> Nat -> Bool -> Nat
  ascii-run-end input position spaces = position'''
    path = ROOT/'benchmarks/stdlib/byte-fields-scalar.cinch'
    assert original.count(old) == 1 and path.read_text() == original.replace(old, new)
    return path


def main() -> None:
    """Retain exact source/compiler/ABI profiles and complete independent output checks without summarizing timings."""
    parser = argparse.ArgumentParser(description=__doc__)
    program = parser.add_mutually_exclusive_group(required=True)
    program.add_argument('--compiler', type=Path)
    program.add_argument('--reference-program', type=Path)
    parser.add_argument('--runtime-archive', type=Path)
    parser.add_argument('--native-interfaces', type=Path)
    parser.add_argument('--native-objects', type=Path)
    parser.add_argument('--native-rc-debug', action='store_true')
    parser.add_argument('--scalar-control', action='store_true')
    parser.add_argument('--utf8-source', type=Path)
    parser.add_argument('--artifacts', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--sizes', type=int, nargs='+', default=SIZES)
    parser.add_argument('--iterations', type=int, default=2)
    parser.add_argument('--check-only', action='store_true')
    args = parser.parse_args()
    assert args.iterations >= 0 and all(value >= 0 for value in args.sizes)
    work = args.artifacts.resolve()
    work.mkdir(parents=True, exist_ok=True)
    report = dict(scope='functional public-pipeline preflight only; elapsed fields are ignored',
                  variant='scalar' if args.scalar_control else 'simd', profiles=[], status='prepared',
                  timing_summary_produced=False, compiler_main_stack_bytes=67108864,
                  program_stack_bytes=8388608, iterations=args.iterations,
                  sizes=args.sizes, generator_sha256=digest(Path(__file__)))
    def save() -> None:
        """Keep each completed compile or memory observation durable before attempting the next stage."""
        args.output.parent.mkdir(parents=True, exist_ok=True)
        args.output.write_text(json.dumps(report, indent=2)+'\n')
    executable = args.reference_program.resolve() if args.reference_program else work/'consumer'
    if args.reference_program:
        assert not args.check_only and not args.scalar_control and not args.native_interfaces and not args.utf8_source
        report.update(variant='independent-pinned-reference', compiled_rc_debug=None,
                      reference_source_sha256=digest(ROOT/'tooling/go_byte_fields_benchmark.go'))
    else:
        source = (ROOT/'benchmarks/stdlib/byte-fields-driver.cinch').read_text()
        entry = work/'Main.cinch'
        entry.write_text(source)
        overrides = {'Data.Bytes.Fields':scalar_source()} if args.scalar_control else {}
        if args.utf8_source:
            overrides['Data.Bytes.Utf8'] = args.utf8_source
        closure = source_closure(source, overrides)
        for name, path in closure.items():
            shutil.copyfile(path, work/(name+'.cinch'))
        report.update(compiler_sha256=digest(args.compiler), entry_sha256=digest(entry),
                      dependencies={name:digest(path) for name,path in closure.items()},
                      compiled_rc_debug=bool(args.native_rc_debug) if not args.check_only else None)
        if args.check_only:
            assert not args.native_interfaces
            command = [str(args.compiler.resolve()), 'check', str(entry)]
        elif args.native_interfaces:
            assert args.native_objects and args.runtime_archive
            bundle = work/'Bundle.cinch'
            bundle.write_text('\n'.join(path.read_text() for path in reversed(list(closure.values())))+'\n'+source)
            command = [str(args.compiler.resolve()), '--emit-ir'] + (['--rc-debug'] if args.native_rc_debug else [])
            command += ['--interfaces', str(args.native_interfaces.resolve()), '--objects', str(args.native_objects.resolve()),
                        str(bundle), str(executable.with_suffix('.ll'))]
            report.update(interfaces_sha256=digest(args.native_interfaces), objects_sha256=digest(args.native_objects))
        else:
            assert args.runtime_archive and not args.native_rc_debug
            command = [str(args.compiler.resolve()), 'build', str(entry), '-o', str(executable), '--opt-level', '2',
                       '--runtime-archive', str(args.runtime_archive.resolve())]
        report['command'] = command
        save()
        try:
            result = run(command)
            (work/'compile.log').write_text(result.stdout+result.stderr)
            if args.check_only:
                report['status'] = 'source-checked-only'
                save()
                return
            report['runtime_sha256'] = digest(args.runtime_archive)
            if args.native_interfaces:
                configuration = json.loads(args.native_objects.read_text())
                assert digest(Path(configuration['object'])) == configuration['object_sha256']
                command = [os.environ.get('CINCH_CLANG', 'clang'), '-O2', str(executable.with_suffix('.ll')),
                           configuration['object'], str(args.runtime_archive.resolve()), '-o', str(executable), '-ldl', '-lpthread', '-lm']
                report.update(link_command=command, native_object_sha256=configuration['object_sha256'])
                save()
                result = run(command)
                (work/'link.log').write_text(result.stdout+result.stderr)
            report['llvm_sha256'] = digest(executable.with_suffix('.ll'))
        except Exception as error:
            diagnostic = executable.with_suffix('.ll.diagnostics')
            detail = str(error) + ('\n'+diagnostic.read_text() if diagnostic.exists() else '')
            (work/'failure.log').write_text(detail)
            report.update(status='build-failed', diagnostics=detail)
            save()
            raise
    report.update(executable_sha256=digest(executable), status='running')
    save()
    for poison in ([False] if args.reference_program else [False, True]):
        for kind in KINDS:
            for pattern in PATTERNS:
                for size in args.sizes:
                    arguments = [str(executable), kind, pattern, str(size), str(args.iterations)]
                    model = expected(kind, pattern, size, args.iterations)
                    try:
                        env = {**os.environ, 'GOMAXPROCS':'1'}
                        if not args.reference_program:
                            env.update(CINCH_RC_DEBUG=str(int(poison)), CINCH_HEAP_LIMIT='256M' if poison else '32M',
                                       CINCH_STACK_SIZE='8M', CINCH_LEAK_CHECK='1')
                        result = run(arguments, env=env)
                        fields = result.stdout.split()
                        assert len(fields) == 5 and int(fields[0]) >= 0
                        actual = dict(checksum=int(fields[1]), original=fields[2], retained=fields[3], kept_size=int(fields[4]))
                        assert actual == model, (actual, model)
                        assert args.reference_program or 'leaked=0' in result.stderr
                    except Exception as error:
                        (work/f'failure-{poison}-{kind}-{pattern}-{size}.log').write_text(str(error))
                        report.update(status='preflight-failed', failed_profile=dict(debug=poison, kind=kind, pattern=pattern, size=size),
                                      diagnostics=str(error))
                        save()
                        raise
                    report['profiles'].append(dict(debug=poison, kind=kind, pattern=pattern, size=size,
                                                   observed=actual, zero_leaks=None if args.reference_program else True))
                    save()
    report['status'] = 'passed'
    save()


if __name__ == '__main__':
    main()
