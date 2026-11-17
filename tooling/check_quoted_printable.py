#!/usr/bin/env python3
"""Check committed independent quoted-printable value, stream and ownership fixtures.

Explicit compiler/runtime pairs emit through the public CLI. Each consumer links
an independent sink fixture and the actual selected SIMD classifier object, then
runs separate normal and poisoned-free allocation profiles. CI needs no SDK.
"""
from __future__ import annotations
import argparse
import json
import os
from pathlib import Path
import re
import resource
import shutil
import time
from bench_stdlib import source_closure
from multipart_qualification_support import archive_completed
from check_compare import digest, initialize_runtime, list_of, run

ROOT = Path(__file__).resolve().parents[1]


def literal(value: str) -> str:
    """Encode scalar text using the escapes accepted by both frontends."""
    out = '"'
    for char in value:
        code = ord(char)
        out += {'"': '\\"', '\\': '\\\\', '\n': '\\n', '\r': '\\r', '\t': '\\t'}.get(char, f'\\u{{{code:x}}}' if code < 32 or code == 127 else char)
    return out + '"'


def byte_literal(value: str) -> str:
    """Prefer compact scalar literals and retain arbitrary invalid UTF-8 bytes exactly."""
    data = bytes.fromhex(value)
    try:
        return '(string-to-bytes ' + literal(data.decode('utf-8')) + ')'
    except UnicodeDecodeError:
        return '(bytes-from-list ' + list_of([str(byte) for byte in data]) + ')'


def fixture_name(index: int) -> str:
    """Use alphabetic suffixes because numeric hyphen suffixes parse as arithmetic."""
    return 'fixture-' + chr(97 + index // 26) + chr(97 + index % 26)


def generated_cases(data: dict) -> dict[str, tuple[str, int]]:
    """Construct bounded consumers of independent expected values and trace observations."""
    header = (ROOT / 'tooling/data/quoted-printable-consumer.cinchpart').read_text()
    cases = {}
    for family, width in [('values', 64), ('readers', 8), ('writers', 8)]:
        for offset in range(0, len(data[family]), width):
            rows = data[family][offset:offset + width]
            expressions = []
            for row in rows:
                if family == 'values':
                    expressions.append('value-is ' + ' '.join(byte_literal(row[k]) for k in ('input', 'text', 'binary', 'decoded')) + ' ' + literal(row['error']))
                elif family == 'readers':
                    steps = [f'({s["request"]}, ({byte_literal(s["output"])}, ({literal(s["error"])}, {s["source"]})))' for s in row['steps']]
                    expressions.append(f'reader-is {byte_literal(row["input"])} {row["chunk"]} ({row["fail_at"]}) {list_of(steps)}')
                else:
                    steps = [f'({byte_literal(s["input"])}, ({s["binary"]}, ({s["accepted"]}, ({literal(s["error"])}, {byte_literal(s["output"])}))))' for s in row['steps']]
                    finish = not any(s['error'] for s in row['steps'])
                    expressions.append(f'writer-is ({row["limit"]}) {list_of(steps)} {finish} {literal(row["close_error"])} {byte_literal(row["output"])}')
            source = header
            for index, expression in enumerate(expressions):
                source += f'\n  -- | Validate independent {family} fixture {offset + index}.\n  {fixture_name(index)} :: forall r. using (io :: Io r) => Unit ->{{IO r}} Bool\n  {fixture_name(index)} unit = {expression}\n'
            result = '0'
            for index in reversed(range(len(expressions))):
                result = f'if {fixture_name(index)} () then {result} else {index + 1}'
            source += '\n  -- | Run this bounded fixture group and the linked independent kernel oracle.\n  main :: Unit ->{IO r} Int\n  main unit = with (io = newIO ()) in\n    if eq-i32 (raw-kernel-check 0) 0 then ' + result + ' else 250\n'
            cases[f'{family}-{offset:04d}'] = (source, len(rows))
    cases['ownership'] = (header + '\n' + (ROOT / 'tooling/data/quoted-printable-ownership.cinchpart').read_text(), 6)
    return cases


def prepare(work: Path, vectors: Path, stream_source: Path | None = None, text_source: Path | None = None) -> tuple[dict[str, int], list[str]]:
    """Snapshot source dependencies in postorder for both public frontend import paths."""
    cases = generated_cases(json.loads(vectors.read_text()))
    entry = next(iter(cases.values()))[0]
    overrides = {'Effect.Stream': stream_source} if stream_source else {}
    if text_source is not None:
        overrides['Text.QuotedPrintable'] = text_source
    closure = source_closure(entry, overrides)
    ordered = []
    visited = set()
    def visit(name: str) -> None:
        """Append each source module after its reachable source dependencies."""
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
    """Compile a scalar reference fixture against the actual checked SIMD body."""
    source = (work / 'Text.QuotedPrintable.cinch').read_text()
    match = re.search(r'prim raw16 .*?llvm\s*\{(.*?)\}\s*when endian little', source, re.S)
    if not match:
        raise RuntimeError('missing selected little-endian classifier body')
    kernel = work / 'classifier.ll'
    kernel.write_text('define i32 @qp_fixture_classify16(i64 %0, i64 %1, i8 %2) {\n' + match.group(1) + '\n}\n')
    fixture = ROOT / 'tooling/data/quoted-printable-fixture.c'
    for src, obj in [(kernel, work / 'classifier.o'), (fixture, work / 'fixture.o')]:
        run([clang, '-O2', '-c', str(src), '-o', str(obj)], work)
    return {'kernel_cases_per_invocation': 32768, 'kernel_ir_sha256': digest(kernel), 'kernel_object_sha256': digest(work / 'classifier.o'), 'fixture_source_sha256': digest(fixture), 'fixture_object_sha256': digest(work / 'fixture.o')}


def main() -> None:
    """Emit explicitly selected frontend consumers and retain exact bounded profile evidence."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--work', type=Path, required=True)
    parser.add_argument('--stream-source', type=Path, help='Explicit separately gated stable-API stream helper source snapshot')
    parser.add_argument('--text-source', type=Path, help='Explicit isolated shared-codec source candidate')
    parser.add_argument('--archive-completed', action='store_true')
    parser.add_argument('--start-free-mib', type=int, default=0)
    parser.add_argument('--pause-free-mib', type=int, default=0)
    parser.add_argument('--unarchived-cap-mib', type=int, default=0)
    parser.add_argument('--vectors', type=Path, default=ROOT / 'compiler/tests/fixtures/quoted-printable-vectors.json')
    parser.add_argument('--prepare-only', action='store_true')
    parser.add_argument('--resume', action='store_true')
    parser.add_argument('--rust', type=Path)
    parser.add_argument('--rust-runtime', type=Path)
    parser.add_argument('--initialize-rust-runtime', action='store_true')
    parser.add_argument('--native', type=Path)
    parser.add_argument('--only', nargs='+')
    parser.add_argument('--cpu', type=int)
    args = parser.parse_args()
    if not args.prepare_only and not args.rust and not args.native:
        parser.error('select at least one explicit frontend/runtime pair')
    if args.rust and not args.rust_runtime:
        parser.error('--rust requires --rust-runtime')
    if args.initialize_rust_runtime and (not args.rust or args.native or args.resume):
        parser.error('runtime initialization requires an explicit Rust-only invocation')
    work = args.work.resolve()
    work.mkdir(parents=True, exist_ok=True)
    if shutil.disk_usage(work).free < args.start_free_mib * 1024 * 1024:
        raise RuntimeError('resource pause before preparation')
    counts, modules = prepare(work, args.vectors, args.stream_source, args.text_source)
    if args.prepare_only:
        print(json.dumps(counts))
        return
    if args.cpu is not None:
        os.sched_setaffinity(0, {args.cpu})
    resource.setrlimit(resource.RLIMIT_STACK, (64 * 1024 * 1024, resource.getrlimit(resource.RLIMIT_STACK)[1]))
    clang = os.environ.get('CINCH_CLANG', 'clang')
    report = {'format': 1, 'runner_sha256': digest(Path(__file__)), 'vectors_sha256': digest(args.vectors), 'compiler_host_stack_bytes': 64 * 1024 * 1024, 'counts': counts, 'consumer_sha256': {name: digest(work / (name + '.source')) for name in counts}, 'reference_source_sha256': digest(ROOT / 'tooling/data/quoted-printable-reference.go'), 'sources': {name: digest(work / (name + '.cinch')) for name in modules}, 'frontends': {}, 'runs': [], **build_fixture(work, clang)}
    if args.resume:
        previous = json.loads((work / 'report.json').read_text())
        for key in ('runner_sha256', 'consumer_sha256', 'vectors_sha256', 'sources', 'kernel_ir_sha256', 'kernel_object_sha256', 'fixture_source_sha256', 'fixture_object_sha256'):
            if previous[key] != report[key]:
                raise RuntimeError('cannot resume changed source/object identity: ' + key)
        report = previous
    def execute(argv: list[str], timeout: int = 180, env: dict | None = None):
        """Retain failed command diagnostics in the same immutable-source epoch report."""
        free = shutil.disk_usage(work).free
        plain = sum(path.stat().st_size for path in work.rglob('*') if path.is_file() and not path.name.endswith('.gz'))
        report.setdefault('resource_boundaries', []).append({'free_bytes': free, 'plain_bytes': plain, 'command': list(map(str, argv))})
        (work / 'report.json').write_text(json.dumps(report, indent=2) + '\n')
        if free < args.pause_free_mib * 1024 * 1024 or (args.unarchived_cap_mib and plain > args.unarchived_cap_mib * 1024 * 1024):
            raise RuntimeError('resource pause before process')
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
        snap = args.native.resolve()
        frontends.append(('native', snap / 'cinch', snap / 'libcinchruntime.a'))
    for label, compiler, runtime in frontends:
        identity = {'compiler': str(compiler), 'compiler_sha256': digest(compiler), 'runtime': str(runtime), 'runtime_sha256': digest(runtime), 'compile_rc_debug': label == 'native'}
        if label == 'native':
            config = json.loads((snap / 'objects.json').read_text())
            provider = Path(config['object'])
            identity.update({name: digest(snap / name) for name in ('objects.json', 'base-interfaces.json')})
            identity['provider_sha256'] = digest(provider)
        if label in report['frontends'] and report['frontends'][label] != identity:
            raise RuntimeError('cannot resume changed frontend identity: ' + label)
        report['frontends'][label] = identity
        for name in args.only or counts:
            if any(row['frontend'] == label and row['case'] == name and len(row['profiles']) == 2 and row['source_sha256'] == digest(work / (name + '.source')) for row in report['runs']):
                continue
            shutil.copyfile(work / (name + '.source'), work / 'Main.cinch')
            ir, exe = work / f'{label}-{name}.ll', work / f'{label}-{name}'
            (work / 'bundle.source').write_text('\n'.join((work / (module + '.cinch')).read_text() for module in modules) + '\n' + (work / 'Main.cinch').read_text())
            command = [str(compiler), 'emit-ir', str(work / 'Main.cinch'), '-o', str(ir)] if label == 'rust' else [str(compiler), '--emit-ir', '--rc-debug', '--interfaces', str(snap / 'base-interfaces.json'), '--objects', str(snap / 'objects.json'), str(work / 'bundle.source'), str(ir)]
            start = time.monotonic()
            result = execute(command, 300)
            target = re.search(r'target triple = "([^"]+)"', ir.read_text())
            triple = target.group(1) if target else config['target'] if label == 'native' else 'unspecified'
            entry = {'frontend': label, 'case': name, 'count': counts[name], 'command': command, 'compile_seconds': time.monotonic() - start, 'stdout': result.stdout, 'stderr': result.stderr, 'target': triple, 'source_sha256': digest(work / 'Main.cinch'), 'bundle_sha256': digest(work / 'bundle.source'), 'ir_sha256': digest(ir), 'profiles': []}
            link = [clang, '-O2', '--target=' + triple, str(ir), str(work / 'fixture.o'), str(work / 'classifier.o')]
            if label == 'native':
                link.append(str(provider))
            execute(link + [str(runtime), '-ldl', '-lpthread', '-lm', '-o', str(exe)])
            entry['executable_sha256'] = digest(exe)
            report['runs'].append(entry)
            for poison, heap in [('0', '32M'), ('1', '256M')]:
                env = {**os.environ, 'CINCH_LEAK_CHECK': '1', 'CINCH_RC_DEBUG': poison, 'CINCH_HEAP_LIMIT': heap, 'CINCH_STACK_SIZE': '8M'}
                result = execute([str(exe)], 60, env)
                if 'leaked=0' not in result.stderr:
                    raise RuntimeError('missing zero-leak evidence: ' + result.stderr)
                entry['profiles'].append({'poison': poison, 'heap': heap, 'stack': '8M', 'stdout': result.stdout, 'stderr': result.stderr})
            (work / 'report.json').write_text(json.dumps(report, indent=2) + '\n')
            if args.archive_completed:
                archive_completed(work, label, name, report)
            print(label, name, counts[name], flush=True)
    (work / 'report.json').write_text(json.dumps(report, indent=2) + '\n')


if __name__ == '__main__':
    main()
