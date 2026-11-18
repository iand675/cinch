#!/usr/bin/env python3
"""Run committed independent MIME fixtures through explicit public frontend pipelines.

No SDK is required during fixture execution. Actual selected vector/copy kernel
objects, callback trace objects, compiler/runtime/provider identities and separate
normal/poison allocation profiles are retained in one source-epoch report.
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
import time
from bench_stdlib import source_closure
from check_compare import corpus_emit_mode, digest, initialize_runtime, list_of, run
from check_quoted_printable import byte_literal as direct_byte_literal, literal

ROOT = Path(__file__).resolve().parents[1]


def byte_literal(value: str) -> str:
    """Bound only large malformed-UTF-8 source lists; preserve every original byte exactly."""
    data = bytes.fromhex(value)
    if len(data) > 256:
        try:
            data.decode('utf-8')
        except UnicodeDecodeError:
            pieces = [direct_byte_literal(data[at:at + 64].hex()) for at in range(0, len(data), 64)]
            return '(fixture-wire ' + list_of(pieces) + ')'
    return direct_byte_literal(value)


def fixture_name(index: int) -> str:
    """Avoid numeric hyphen suffixes, which are parsed as arithmetic."""
    return 'fixture-' + chr(97 + index // 26) + chr(97 + index % 26)


def trace_digest(events: list[dict]) -> int:
    """Freeze exact callback/read/copy observations with an independent byte-order digest."""
    value = 14695981039346656037
    def mix(data: bytes) -> None:
        """Hash exact observed bytes using wrapping unsigned arithmetic."""
        nonlocal value
        for byte in data:
            value = ((value ^ byte) * 1099511628211) & ((1 << 64) - 1)
    def word(number: int) -> None:
        """Length-prefix observations in explicit little-endian order."""
        mix(number.to_bytes(8, 'little'))
    for identity, event in enumerate(events):
        label, data = bytes.fromhex(event['charset']), bytes.fromhex(event['input'])
        mix(b'\x01'); word(identity); word(len(label)); mix(label); word(len(data)); mix(data)
        for row in event['reads']:
            out = bytes.fromhex(row['output'])
            status = 1 if row['error'] == 'EOF' else 2 if row['error'] else 0
            mix(b'\x02'); word(identity); word(row['request']); word(len(out)); word(status); mix(out)
        for row in event.get('copies', []):
            out = bytes.fromhex(row['output']); status = 2 if row['error'] else 0
            mix(b'\x03'); word(identity); word(len(out)); word(status); mix(out)
    return value


def pair_list(pairs: list[dict] | None) -> str:
    """Preserve original arbitrary-byte key spelling and exact value bytes."""
    return list_of([f'({byte_literal(p["key"])}, {byte_literal(p["value"])})' for p in pairs or []])


def decode_expression(row: dict) -> str:
    """Retain all bytes, typed diagnostics and independent callback trace observations."""
    return f'decode-is {row["header"]} {byte_literal(row["input"])} {literal(row["callback"])} {byte_literal(row["output"])} {literal(row["error"])} {trace_digest(row["events"])}'


def generated_cases(data: dict, width: int = 32) -> dict[str, tuple[str, int]]:
    """Generate bounded exact-case consumers without depending on SDK access in CI."""
    header = (ROOT / 'tooling/data/mime-consumer.cinchpart').read_text()
    result = {}
    for family in ('media', 'formats', 'encoded', 'decoded', 'writes'):
        for offset in range(0, len(data.get(family, [])), width):
            expressions = []
            rows = data[family][offset:offset + width]
            for row in rows:
                if family == 'media':
                    expression = f'media-is {byte_literal(row["input"])} {byte_literal(row["base"])} {row["parameters"] is None} {pair_list(row["parameters"])} {literal(row["error"])}'
                elif family == 'formats':
                    expression = f'format-is {byte_literal(row["base"])} {pair_list(row["parameters"])} {byte_literal(row["output"])}'
                elif family == 'encoded':
                    expression = f'encode-is {row["encoding"]} {byte_literal(row["charset"])} {byte_literal(row["input"])} {byte_literal(row["output"])}'
                elif family == 'decoded':
                    expression = decode_expression(row)
                else:
                    if row['header']:
                        expression = f'header-write-is {byte_literal(row["input"])} ({row["budget"]}) {byte_literal(row["output"])} {literal(row["error"])}'
                    else:
                        expression = f'write-is {row["encoding"]} {byte_literal(row["charset"])} {byte_literal(row["input"])} ({row["budget"]}) {byte_literal(row["output"])} {literal(row["error"])}'
                expressions.append(expression)
            source = header
            if any('fixture-wire ' in expression for expression in expressions):
                source += '\n' + (ROOT / 'tooling/data/mime-wire-literal.cinchpart').read_text()
            for index, expression in enumerate(expressions):
                source += f'\n  -- | Validate independent {family} case {offset + index}.\n  {fixture_name(index)} :: forall r. using (io :: Io r) => Unit ->{{IO r}} Bool\n  {fixture_name(index)} unit = {expression}\n'
            answer = '0'
            for index in reversed(range(len(rows))):
                answer = f'if {fixture_name(index)} () then {answer} else {index + 1}'
            source += '\n  -- | Run exact fixture observations and the actual foreign vector/copy oracle.\n  main :: Unit ->{IO r} Int\n  main unit = with (io = newIO ()) in\n    if eq-i32 (raw-kernel-check 0) 0 then ' + answer + ' else 250\n'
            result[f'{family}-{offset:04d}'] = source, len(rows)
    selected, policies = [], set()
    for row in data['decoded']:
        if row['events'] and row['callback'] not in policies:
            selected.append(row)
            policies.add(row['callback'])
    source = header
    for index, row in enumerate(selected):
        source += f'\n  -- | Validate representative callback capability {row["callback"]}.\n  {fixture_name(index)} :: forall r. using (io :: Io r) => Unit ->{{IO r}} Bool\n  {fixture_name(index)} unit = {decode_expression(row)}\n'
    answer = '0'
    for index in reversed(range(len(selected))):
        answer = f'if {fixture_name(index)} () then {answer} else {index + 1}'
    source += '\n  -- | Exercise every callback capability with the actual selected foreign kernel objects.\n  main :: Unit ->{IO r} Int\n  main unit = with (io = newIO ()) in\n    if eq-i32 (raw-kernel-check 0) 0 then ' + answer + ' else 250\n'
    result['decoded-callbacks'] = source, len(selected)
    registry = header
    registry_rows = data['registry']
    for index, row in enumerate(registry_rows):
        name = fixture_name(index)
        following = fixture_name(index + 1) + ' next' if index + 1 < len(registry_rows) else 'True'
        same = fixture_name(index + 1) + ' registry' if index + 1 < len(registry_rows) else 'True'
        registry += f'\n  -- | Validate independent explicit association step {index}.\n  {name} :: MimeRegistry -> Bool\n'
        if row['operation'] == 'add':
            registry += f'  {name} registry = case registry-add-is {byte_literal(row["extension"])} {byte_literal(row["type"])} {literal(row["error"])} registry of (matched, next) -> matched && {following}\n'
        elif row['operation'] == 'type':
            registry += f'  {name} registry = same (mime-registry-type {byte_literal(row["extension"])} registry) {byte_literal(row["output"])} && {same}\n'
        else:
            extensions = list_of([byte_literal(value) for value in row['extensions'] or []])
            registry += f'  {name} registry = registry-extensions-is {byte_literal(row["type"])} {extensions} {literal(row["error"])} registry && {same}\n'
    registry += '\n  -- | Run the complete historical association trace from an explicit empty context.\n  main :: Int\n  main = if fixture-aa mime-registry-empty then 0 else 1\n'
    result['registry'] = registry, len(registry_rows)
    result['ownership'] = (header + '\n' + (ROOT / 'tooling/data/mime-ownership.cinchpart').read_text() + '\n' + (ROOT / 'tooling/data/mime-writer-continuation.cinchpart').read_text() + '\n' + (ROOT / 'tooling/data/mime-reader-reuse.cinchpart').read_text(), 9)
    return result


def prepare(work: Path, vectors: Path, width: int = 32) -> tuple[dict[str, int], list[str]]:
    """Retain postordered source modules and generated consumer source bytes."""
    cases = generated_cases(json.loads(vectors.read_text()), width)
    closure = source_closure(next(iter(cases.values()))[0])
    ordered, visited = [], set()
    def visit(name: str) -> None:
        """Place imported source modules before their dependent module."""
        if name in visited:
            return
        visited.add(name)
        for dep in re.findall(r'^\s*import\s+([A-Z][\w.]*)', closure[name].read_text(), re.M):
            if dep in closure:
                visit(dep)
        ordered.append(name)
    for name in closure:
        visit(name)
    for name, path in closure.items():
        shutil.copyfile(path, work / (name + '.cinch'))
    for name, (source, _) in cases.items():
        (work / (name + '.source')).write_text(source)
    return {name: count for name, (_, count) in cases.items()}, ordered


def build_fixture(work: Path, clang: str) -> dict:
    """Compile the selected unmodified class/word-transform bodies and an independent scalar oracle."""
    source = (work / 'Text.Mime.cinch').read_text()
    classifier = re.search(r'prim classify-sixteen .*?llvm\s*\{(.*?)\}\s*when endian little', source, re.S)
    copy = re.search(r'prim lower-eight .*?llvm\s*\{(.*?)\}', source, re.S)
    if not classifier or not copy:
        raise RuntimeError('missing actual selected classifier/copy bodies')
    kernel = work / 'kernels.ll'
    kernel.write_text('define i32 @mime_fixture_classify16(i64 %0, i64 %1, i8 %2) {\n' + classifier.group(1) + '\n}\ndefine i64 @mime_fixture_lower8(i64 %0) {\n' + copy.group(1) + '\n}\n')
    fixture = ROOT / 'tooling/data/mime-fixture.c'
    run([clang, '-O2', '-c', str(kernel), '-o', str(work / 'kernels.o')], work)
    run([clang, '-O2', '-fno-vectorize', '-fno-slp-vectorize', '-c', str(fixture), '-o', str(work / 'fixture.o')], work)
    return {'kernel_ir_sha256': digest(kernel), 'kernel_object_sha256': digest(work / 'kernels.o'), 'fixture_source_sha256': digest(fixture), 'fixture_object_sha256': digest(work / 'fixture.o'), 'scalar_oracle_vectorization': False, 'classifier_cases_per_invocation': 24576, 'lowercase_word_cases_per_invocation': 2048}


def main() -> None:
    """Execute explicit frozen/current frontend identities under separately bounded profiles."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--work', type=Path, required=True)
    parser.add_argument('--vectors', type=Path, default=ROOT / 'compiler/tests/fixtures/mime-vectors.json')
    parser.add_argument('--prepare-only', action='store_true')
    parser.add_argument('--group-size', type=int, choices=(32, 64, 128), default=32, help='Bound each generated exact-case consumer; larger final groups amortize source checking')
    parser.add_argument('--resume', action='store_true')
    parser.add_argument('--rust', type=Path)
    parser.add_argument('--rust-runtime', type=Path)
    parser.add_argument('--initialize-rust-runtime', action='store_true')
    parser.add_argument('--native', type=Path)
    parser.add_argument('--only', nargs='+')
    parser.add_argument('--cpu', type=int)
    args = parser.parse_args()
    if args.rust and not args.rust_runtime:
        parser.error('--rust requires its exact --rust-runtime')
    if args.initialize_rust_runtime and (not args.rust or args.native or args.resume):
        parser.error('runtime initialization requires a fresh explicit Rust-only invocation')
    if not args.prepare_only and not args.rust and not args.native:
        parser.error('select at least one explicit frontend/runtime pair')
    work = args.work.resolve(); work.mkdir(parents=True, exist_ok=True)
    counts, modules = prepare(work, args.vectors, args.group_size)
    if args.prepare_only:
        print(json.dumps(counts)); return
    if args.cpu is not None:
        os.sched_setaffinity(0, {args.cpu})
    resource.setrlimit(resource.RLIMIT_STACK, (67108864, resource.getrlimit(resource.RLIMIT_STACK)[1]))
    clang = os.environ.get('CINCH_CLANG', 'clang')
    report = {'format': 1, 'group_size': args.group_size, 'runner_sha256': digest(Path(__file__)), 'vectors_sha256': digest(args.vectors), 'compiler_host_stack_bytes': 67108864, 'compiler_environment': {'RUST_MIN_STACK': '67108864', 'CINCH_STACK_SIZE': '64M'}, 'counts': counts, 'sources': {m: digest(work / (m + '.cinch')) for m in modules}, 'consumers': {name: digest(work / (name + '.source')) for name in counts}, 'frontends': {}, 'runs': [], **build_fixture(work, clang)}
    if args.resume:
        previous = json.loads((work / 'report.json').read_text())
        for key in ('runner_sha256', 'vectors_sha256', 'sources', 'consumers', 'kernel_ir_sha256', 'kernel_object_sha256', 'fixture_source_sha256', 'fixture_object_sha256'):
            if previous[key] != report[key]:
                raise RuntimeError('cannot resume changed source/object identity: ' + key)
        report = previous
    def execute(argv: list[str], timeout: int = 300, env: dict | None = None):
        """Preserve complete failed command diagnostics in the same exact source epoch."""
        try:
            return run(argv, work, timeout, env)
        except Exception as failure:
            report.setdefault('failures', []).append({'command': list(map(str, argv)), 'diagnostic': str(failure)})
            (work / 'report.json').write_text(json.dumps(report, indent=2) + '\n')
            raise
    frontends = []
    if args.rust:
        runtime = args.rust_runtime.resolve()
        if args.initialize_rust_runtime:
            runtime, report['runtime_initialization'] = initialize_runtime(args.rust.resolve(), runtime, work)
        frontends.append(('rust', args.rust.resolve(), runtime))
    if args.native:
        snap = args.native.resolve(); frontends.append(('native', snap / 'cinch', snap / 'libcinchruntime.a'))
    for label, compiler, runtime in frontends:
        identity = {'compiler': str(compiler), 'compiler_sha256': digest(compiler), 'runtime': str(runtime), 'runtime_sha256': digest(runtime), 'compile_rc_debug': label == 'native'}
        if label == 'native':
            mode = corpus_emit_mode(compiler, report['runs'])
            config = json.loads((snap / 'objects.json').read_text()); provider = Path(config['object'])
            identity.update({name: digest(snap / name) for name in ('objects.json', 'base-interfaces.json')}); identity['provider_sha256'] = digest(provider)
        if label in report['frontends'] and report['frontends'][label] != identity:
            raise RuntimeError('cannot resume changed frontend identity: ' + label)
        report['frontends'][label] = identity
        for name in args.only or counts:
            if any(row['frontend'] == label and row['case'] == name and len(row['profiles']) == 2 for row in report['runs']):
                continue
            shutil.copyfile(work / (name + '.source'), work / 'Main.cinch')
            (work / 'bundle.source').write_text('\n'.join((work / (m + '.cinch')).read_text() for m in modules) + '\n' + (work / 'Main.cinch').read_text())
            ir, exe = work / f'{label}-{name}.ll', work / f'{label}-{name}'
            command = [str(compiler), 'emit-ir', str(work / 'Main.cinch'), '-o', str(ir)] if label == 'rust' else [str(compiler), mode, '--rc-debug', '--interfaces', str(snap / 'base-interfaces.json'), '--objects', str(snap / 'objects.json'), str(work / 'bundle.source'), str(ir)]
            start = time.monotonic(); result = execute(command, env={**os.environ, 'RUST_MIN_STACK': '67108864', 'CINCH_STACK_SIZE': '64M'})
            target = re.search(r'target triple = "([^"]+)"', ir.read_text()); triple = target.group(1) if target else config['target']
            entry = {'frontend': label, 'case': name, 'count': counts[name], 'command': command, 'compile_seconds': time.monotonic() - start, 'stdout': result.stdout, 'stderr': result.stderr, 'target': triple, 'source_sha256': digest(work / 'Main.cinch'), 'ir_sha256': digest(ir), 'profiles': []}
            report['runs'].append(entry)
            link = [clang, '-O2', '--target=' + triple, str(ir), str(work / 'fixture.o'), str(work / 'kernels.o')]
            if label == 'native':
                link.append(str(provider))
            execute(link + [str(runtime), '-ldl', '-lpthread', '-lm', '-o', str(exe)])
            entry['executable_sha256'] = digest(exe)
            for poison, heap in [('0', '32M'), ('1', '256M')]:
                env = {**os.environ, 'CINCH_LEAK_CHECK': '1', 'CINCH_RC_DEBUG': poison, 'CINCH_HEAP_LIMIT': heap, 'CINCH_STACK_SIZE': '8M'}
                result = execute([str(exe)], 60, env)
                if 'leaked=0' not in result.stderr:
                    raise RuntimeError('missing zero-leak evidence: ' + result.stderr)
                entry['profiles'].append({'poison': poison, 'heap': heap, 'stack': '8M', 'stdout': result.stdout, 'stderr': result.stderr})
            (work / 'report.json').write_text(json.dumps(report, indent=2) + '\n')
            print(label, name, counts[name], flush=True)
    (work / 'report.json').write_text(json.dumps(report, indent=2) + '\n')


if __name__ == '__main__':
    main()
