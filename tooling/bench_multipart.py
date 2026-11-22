#!/usr/bin/env python3
"""Build and measure full multipart public pipelines against a pinned reference.

All samples, allocation-policy differences and regressions remain visible. The
scope includes fresh output allocation and an observable scalar output checksum;
input generation, reader-wire encoding and reusable writer initialization occur before the measured interval.
"""
from __future__ import annotations
import argparse
import json
import os
from pathlib import Path
import platform
import re
import resource
import shutil
import statistics
from bench_stdlib import source_closure
from check_compare import ROOT, digest, run


#: Independently observed public pipelines, including adverse escaping distributions.
MODES = ('read', 'read-header', 'read-qp', 'write-body', 'write-sparse', 'write-dense', 'disposition-sparse', 'disposition-dense')


def sample(program: Path, mode: str, size: int, count: int, work: Path, observations: list | None = None) -> tuple[int, int, int]:
    """Retain one pipeline's elapsed time and independently comparable result checksum."""
    env = {**os.environ, 'GOMAXPROCS': '1', 'GOTELEMETRY': 'off', 'CINCH_HEAP_LIMIT': '32M', 'CINCH_STACK_SIZE': '8M', 'CINCH_LEAK_CHECK': '1', 'CINCH_RC_DEBUG': '0'}
    result = run([str(program), mode, str(size), str(count)], work, 60, env)
    if program.name == 'bench' and 'leaked=0' not in result.stderr:
        raise RuntimeError('missing zero-leak benchmark evidence: ' + result.stderr)
    if observations is not None:
        observations.append({'program': str(program), 'mode': mode, 'size': size, 'iterations': count,
                             'stdout': result.stdout, 'stderr': result.stderr})
    return tuple(map(int, result.stdout.split()))


def functional_smoke(exe: Path, reference: Path, sizes: list[int], count: int, work: Path) -> None:
    """Check independent pipeline checksums and both heap profiles without claiming timing samples."""
    rows = []
    for mode in MODES:
        for size in sizes:
            expected = sample(reference, mode, size, count, work)[1:]
            for poison, heap in [('0', '32M'), ('1', '256M')]:
                env = {**os.environ, 'CINCH_HEAP_LIMIT': heap, 'CINCH_STACK_SIZE': '8M', 'CINCH_LEAK_CHECK': '1', 'CINCH_RC_DEBUG': poison}
                result = run([str(exe), mode, str(size), str(count)], work, 60, env)
                fields = tuple(map(int, result.stdout.split()))
                if fields[1:] != expected or 'leaked=0' not in result.stderr:
                    raise RuntimeError(f'pipeline smoke mismatch: {mode}/{size}/{poison}: {result.stdout} {result.stderr}')
                rows.append({'mode': mode, 'size': size, 'iterations': count, 'poison': poison, 'heap': heap, 'stack': '8M', 'stdout': result.stdout, 'stderr': result.stderr, 'reference_checksums': expected})
    (work / 'functional-smoke.json').write_text(json.dumps({'purpose': 'functional checks only; elapsed fields are not timing samples', 'runner_sha256': digest(Path(__file__)), 'executable_sha256': digest(exe), 'reference_sha256': digest(reference), 'runs': rows}, indent=2) + '\n')


def main() -> None:
    """Build exact seed/runtime pairs and alternate complete public pipeline timings."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--work', type=Path, required=True)
    parser.add_argument('--compiler', type=Path, required=True)
    parser.add_argument('--runtime', type=Path, required=True)
    parser.add_argument('--native', type=Path)
    parser.add_argument('--go', type=Path, required=True)
    parser.add_argument('--cpu', type=int, required=True)
    parser.add_argument('--build-only', action='store_true')
    parser.add_argument('--scalar', action='store_true', help='Prepare an exact-source snapshot replacing SIMD byte/escape scanning and SWAR prefix comparisons with scalar predicates')
    parser.add_argument('--run-only', action='store_true')
    parser.add_argument('--smoke-only', action='store_true', help='Check both heap profiles and independent checksums without paired timing')
    parser.add_argument('--sizes', nargs='+', type=int, default=[64, 4096])
    parser.add_argument('--iterations', type=int, default=50)
    parser.add_argument('--samples', type=int, default=7)
    parser.add_argument('--context', default='', help='Observed shared-machine background during the exclusive timing slot')
    parser.add_argument('--compare', type=Path, help='Interleave the opposite scalar/SIMD snapshot and the same independent reference')
    parser.add_argument('--baseline', type=Path, help='Also interleave a separately sealed same-compiler baseline without changing its report or inputs')
    parser.add_argument('--effect-source', type=Path, help='Explicit selected Effect.Multipart candidate, preserved in both scalar and SIMD sets')
    parser.add_argument('--text-source', type=Path, help='Explicit selected Text.Multipart basis before any documented scalar derivation')
    parser.add_argument('--qp-source', type=Path, help='Explicit current shared codec source candidate')
    parser.add_argument('--start-free-mib', type=int, default=0)
    parser.add_argument('--pause-free-mib', type=int, default=0)
    parser.add_argument('--unarchived-cap-mib', type=int, default=0)
    args = parser.parse_args()
    if args.build_only and args.run_only:
        parser.error('--build-only and --run-only are mutually exclusive')
    if args.iterations <= 0 or args.samples <= 0 or any(size < 0 for size in args.sizes):
        parser.error('iterations/samples must be positive and byte sizes nonnegative')
    os.sched_setaffinity(0, {args.cpu})
    resource.setrlimit(resource.RLIMIT_STACK, (64 * 1024 * 1024, resource.getrlimit(resource.RLIMIT_STACK)[1]))
    work = args.work.resolve()
    work.mkdir(parents=True, exist_ok=True)
    compiler, runtime = args.compiler.resolve(), args.runtime.resolve()
    clang = os.environ.get('CINCH_CLANG', 'clang')
    driver = ROOT / 'tooling/data/multipart-benchmark.cinch'
    reference_source = ROOT / 'tooling/data/multipart-benchmark.go'
    exe, reference = work / 'bench', work / 'reference'
    report_path = work / 'report.json'
    selection = {name: {'path': str(path.resolve()), 'sha256': digest(path)} for name, path in
                 (('Effect.Multipart', args.effect_source), ('Text.Multipart', args.text_source), ('Text.QuotedPrintable', args.qp_source)) if path is not None}
    policy = {'start_free_mib': args.start_free_mib, 'pause_free_mib': args.pause_free_mib,
              'unarchived_cap_mib': args.unarchived_cap_mib}

    def guard(phase: str, minimum: int) -> None:
        """Keep measured inputs plain while bounding each functional build/preflight epoch."""
        free = shutil.disk_usage(work).free
        plain = sum(path.stat().st_size for path in work.rglob('*') if path.is_file())
        if free < minimum or (args.unarchived_cap_mib and plain > args.unarchived_cap_mib * 1024 * 1024):
            raise RuntimeError('resource pause before ' + phase + '; preserve all current inputs')

    guard('start', args.start_free_mib * 1024 * 1024)
    if not args.run_only:
        if report_path.exists():
            raise RuntimeError('preserve existing benchmark epoch; select --run-only or a fresh work directory')
        frozen_reference = work / 'reference.go'
        frozen_endpoint = work / 'endpoint.c'
        shutil.copyfile(reference_source, frozen_reference)
        shutil.copyfile(ROOT / 'tooling/data/multipart-benchmark.c', frozen_endpoint)
        overrides = {name: Path(row['path']) for name, row in selection.items()}
        closure = source_closure(driver.read_text(), overrides)
        ordered, visited = [], set()
        def visit(name: str) -> None:
            """Preserve dependency order for source-distributed Native modules."""
            if name in visited:
                return
            visited.add(name)
            for dep in re.findall(r'^\s*import\s+([A-Z][\w.]*)', closure[name].read_text(), re.M):
                if dep in closure:
                    visit(dep)
            ordered.append(name)
        for name in closure:
            visit(name)
            shutil.copyfile(closure[name], work / (name + '.cinch'))
        if args.scalar:
            source = work / 'Text.Multipart.cinch'
            contents = source.read_text()
            replacements = [
                ('  escape-end input at =', '\n  -- | Copy ordinary disposition runs', """  escape-end input at = if at < bytes-len input then
    let byte = bytes-get input at in
    if byte == 13 || byte == 10 || byte == 34 || byte == 92 then at else escape-end input (at + 1)
    else at
"""),
                ('  find-byte input at needle =', '\n  -- | Return the first target byte', """  find-byte input at needle = if at < bytes-len input then
    if bytes-get input at == u8-to-nat needle then at else find-byte input (at + 1) needle
    else at
"""),
                ('  prefix-at input at prefix index =', '\n  -- | Test a complete raw prefix', """  prefix-at input at prefix index = if index < bytes-len prefix then
    if at + index < bytes-len input then bytes-get input (at + index) == bytes-get prefix index && prefix-at input at prefix (index + 1) else False
    else True
""")]
            for begin, end, replacement in replacements:
                first = contents.index(begin)
                following = re.search(r'\n  -- \|', contents[first:])
                if following is None:
                    raise RuntimeError('missing canonical next declaration for scalar derivation')
                last = first + following.start()
                contents = contents[:first] + replacement + contents[last:]
            source.write_text(contents)
        shutil.copyfile(driver, work / 'Main.cinch')
        report = {'format': 1, 'scalar_snapshot': args.scalar, 'compile_rc_debug': False, 'runner_sha256': digest(Path(__file__)), 'compiler': str(compiler), 'compiler_sha256': digest(compiler), 'runtime': str(runtime), 'runtime_sha256': digest(runtime), 'driver_sha256': digest(driver), 'reference_source_sha256': digest(reference_source), 'source_sha256': {name: digest(work / (name + '.cinch')) for name in ordered}, 'compiler_host_stack_bytes': 64 * 1024 * 1024, 'compiler_environment': {'RUST_MIN_STACK': '67108864', 'CINCH_STACK_SIZE': '64M'}, 'allocation_policy': 'Cinch returns fresh mutable byte/header snapshots; the reference uses immutable strings and may keep local state or buffers on its stack. Both consume every output byte inside the clock and traverse retained inputs afterward.', 'writer_context_scope': 'Both prepare one validated-boundary writer outside the clock, then benchmark complete WriteField plus repeatable Close operations on the reused context; entropy and root-context initialization are excluded on both sides.', 'transport_scope': 'Cinch uses a borrowed C source with fresh language-owned read buffers and a scalar C observing sink; the reference uses bytes.Reader, one reusable body buffer and an equivalent scalar observing writer.', 'profile_scope': 'Cinch executable only; the public reference retains its normal GC, heap and goroutine-stack policies.', 'profile': {'heap': '32M', 'stack': '8M', 'poison': False, 'leak_check': True}, 'scope': __doc__, 'results': []}
        report.update(source_selection=selection, resource_policy=policy,
                      sdk_executable_sha256=digest(args.go), sdk_executable=str(args.go.resolve()))
        report_path.write_text(json.dumps(report, indent=2) + '\n')

        def execute(command: list, timeout: int = 300, env: dict | None = None):
            """Preserve the exact first failed build command and its plain source/diagnostic artifacts."""
            guard('process', args.pause_free_mib * 1024 * 1024)
            try:
                return run(command, work, timeout, env)
            except Exception as error:
                report.setdefault('failures', []).append({'command': list(map(str, command)), 'diagnostic': str(error)})
                report_path.write_text(json.dumps(report, indent=2) + '\n')
                raise
        ir = work / 'bench.ll'
        if args.native:
            snap = args.native.resolve()
            config = json.loads((snap / 'base-objects.json').read_text())
            provider = Path(config['object'])
            (work / 'bundle.source').write_text('\n'.join((work / (name + '.cinch')).read_text() for name in ordered) + '\n' + driver.read_text())
            command = [str(compiler), '--emit-ir', '--interfaces', str(snap / 'base.cpk'), '--objects', str(snap / 'base-objects.json'), str(work / 'bundle.source'), str(ir)]
            report['native'] = {name: digest(snap / name) for name in ('base.cpk', 'base-objects.json')}
            report['native']['provider_sha256'] = digest(provider)
        else:
            command = [str(compiler), 'emit-ir', str(work / 'Main.cinch'), '-o', str(ir)]
        endpoint = frozen_endpoint
        endpoint_object = work / 'endpoint.o'
        if not args.native:
            checked = execute([str(compiler), 'check', str(work / 'Main.cinch')], env={**os.environ, 'RUST_MIN_STACK': '67108864', 'CINCH_STACK_SIZE': '64M'})
            report['source_check'] = {'stdout': checked.stdout, 'stderr': checked.stderr, 'accepted': True}
        execute([clang, '-O2', '-fno-vectorize', '-fno-slp-vectorize', '-c', str(endpoint), '-o', str(endpoint_object)])
        report['endpoint_source_sha256'] = digest(endpoint)
        report['endpoint_object_sha256'] = digest(endpoint_object)
        report['endpoint_vectorization'] = False
        result = execute(command, env={**os.environ, 'RUST_MIN_STACK': '67108864', 'CINCH_STACK_SIZE': '64M'})
        report['compile'] = {'command': command, 'stdout': result.stdout, 'stderr': result.stderr}
        target = re.search(r'target triple = "([^"]+)"', ir.read_text())
        triple = target.group(1) if target else config['target']
        link = [clang, '-O2', '--target=' + triple, str(ir), str(endpoint_object)]
        if args.native:
            link.append(str(provider))
        execute(link + [str(runtime), '-ldl', '-lpthread', '-lm', '-o', str(exe)])
        execute([clang, '-O2', '-S', str(ir), '-o', str(work / 'bench.s')])
        env = {**os.environ, 'GOMAXPROCS': '1', 'GOTELEMETRY': 'off', 'GOCACHE': '/tmp/cinch-mime-go-cache'}
        reference_command = [str(args.go), 'build', '-trimpath', '-o', str(reference), str(frozen_reference)]
        execute(reference_command, 180, env)
        report['reference_command'] = reference_command
        report.update(executable_sha256=digest(exe), reference_executable_sha256=digest(reference), llvm_sha256=digest(ir), assembly_sha256=digest(work / 'bench.s'), target_triple=triple, sdk=run([str(args.go), 'version'], work, env=env).stdout.strip(), platform=platform.platform(), cpu=next(line.split(':', 1)[1].strip() for line in Path('/proc/cpuinfo').read_text().splitlines() if line.startswith('model name')), clang=run([clang, '--version'], work).stdout.splitlines()[0])
        sdk_root = Path(run([str(args.go), 'env', 'GOROOT'], work, env=env).stdout.strip())
        report['sdk_package_sources'] = {str(path.relative_to(sdk_root)): digest(path)
            for package in ('mime', 'mime/multipart', 'mime/quotedprintable')
            for path in sorted((sdk_root / 'src' / package).glob('*.go')) if not path.name.endswith('_test.go')}
        guard('completed build', args.pause_free_mib * 1024 * 1024)
        report_path.write_text(json.dumps(report, indent=2) + '\n')
    else:
        report = json.loads(report_path.read_text())
        if report.get('source_selection', {}) != selection:
            raise RuntimeError('changed explicit current-candidate selection')
        if report.get('sdk_executable_sha256', digest(args.go)) != digest(args.go):
            raise RuntimeError('changed pinned SDK executable')
        if report['scalar_snapshot'] != args.scalar:
            raise RuntimeError('changed scalar/SIMD benchmark selection')
        for key, path in [('compiler_sha256', compiler), ('runtime_sha256', runtime), ('driver_sha256', driver), ('reference_source_sha256', reference_source), ('endpoint_source_sha256', ROOT / 'tooling/data/multipart-benchmark.c'), ('endpoint_object_sha256', work / 'endpoint.o'), ('executable_sha256', exe), ('reference_executable_sha256', reference)]:
            if report[key] != digest(path):
                raise RuntimeError('changed benchmark identity: ' + key)
        for name, expected in report['source_sha256'].items():
            if digest(work / (name + '.cinch')) != expected:
                raise RuntimeError('changed frozen benchmark source: ' + name)
        for name, key in [('bench.ll', 'llvm_sha256'), ('bench.s', 'assembly_sha256')]:
            if digest(work / name) != report[key]:
                raise RuntimeError('changed benchmark code evidence: ' + name)
        for name, key in [('reference.go', 'reference_source_sha256'), ('endpoint.c', 'endpoint_source_sha256')]:
            if digest(work / name) != report[key]:
                raise RuntimeError('changed frozen benchmark helper: ' + name)
    if args.build_only:
        return
    if args.smoke_only:
        guard('functional preflight', args.pause_free_mib * 1024 * 1024)
        functional_smoke(exe, reference, args.sizes, args.iterations, work)
        return
    report['quota_before'] = Path('/sys/fs/cgroup/cpu.stat').read_text()
    report.update(cpu_affinity=[args.cpu], samples=args.samples, iterations=args.iterations, timing_runner_sha256=digest(Path(__file__)), background_context=args.context)
    programs = [('scalar' if args.scalar else 'simd', exe), ('reference', reference)]
    if args.compare:
        companion = args.compare.resolve()
        other = json.loads((companion / 'report.json').read_text())
        for key in ('compiler_sha256', 'runtime_sha256', 'driver_sha256', 'reference_source_sha256', 'endpoint_source_sha256', 'sdk'):
            if report[key] != other[key]:
                raise RuntimeError('changed companion benchmark identity: ' + key)
        if report.get('source_selection', {}) != other.get('source_selection', {}):
            raise RuntimeError('companion changes the selected candidate basis')
        if other['scalar_snapshot'] == args.scalar:
            raise RuntimeError('companion must be the opposite scalar/SIMD implementation')
        if digest(companion / 'bench') != other['executable_sha256']:
            raise RuntimeError('changed companion executable')
        for name, expected in other['source_sha256'].items():
            if digest(companion / (name + '.cinch')) != expected:
                raise RuntimeError('changed frozen companion source: ' + name)
        programs.insert(1, ('simd' if args.scalar else 'scalar', companion / 'bench'))
        report['companion'] = {'report_path': str(companion / 'report.json'), 'report_sha256': digest(companion / 'report.json'),
                               'executable_sha256': other['executable_sha256'], 'llvm_sha256': other['llvm_sha256'],
                               'source_sha256': other['source_sha256']}
    if args.baseline:
        baseline = args.baseline.resolve()
        old = json.loads((baseline / 'report.json').read_text())
        for key in ('compiler_sha256', 'runtime_sha256', 'driver_sha256', 'reference_source_sha256', 'endpoint_source_sha256', 'sdk', 'sdk_executable_sha256'):
            if report.get(key) != old.get(key):
                raise RuntimeError('baseline changes a matched seed/runtime/driver/reference identity: ' + key)
        for filename, key in [('bench', 'executable_sha256'), ('bench.ll', 'llvm_sha256'), ('bench.s', 'assembly_sha256')]:
            if digest(baseline / filename) != old[key]:
                raise RuntimeError('changed sealed baseline code: ' + filename)
        for name, expected in old['source_sha256'].items():
            if digest(baseline / (name + '.cinch')) != expected:
                raise RuntimeError('changed sealed baseline source: ' + name)
        programs.insert(1, ('baseline-scalar' if old['scalar_snapshot'] else 'baseline-simd', baseline / 'bench'))
        report['baseline'] = {'report_path': str(baseline / 'report.json'), 'report_sha256': digest(baseline / 'report.json'),
                              'source_selection': old.get('source_selection', {}), 'source_sha256': old['source_sha256'],
                              'executable_sha256': old['executable_sha256'], 'llvm_sha256': old['llvm_sha256'],
                              'scope': 'Separate immutable same-compiler source baseline. Its files/report are never rewritten.'}
    for mode in MODES:
        for size in args.sizes:
            elapsed = {label: [] for label, _ in programs}
            observations = []
            checksums = set()
            for label, program in programs:
                checksums.add(sample(program, mode, size, args.iterations, work, observations)[1:])
            for index in range(args.samples):
                rotation = index % len(programs)
                order = programs[rotation:] + programs[:rotation]
                for label, program in order if index % 2 == 0 else reversed(order):
                    nanos, checksum, retained = sample(program, mode, size, args.iterations, work, observations)
                    elapsed[label].append(nanos)
                    checksums.add((checksum, retained))
            if len(checksums) != 1:
                raise RuntimeError(f'{mode}/{size}: independent checksums differ: {checksums}')
            medians = {label: statistics.median(values) / args.iterations for label, values in elapsed.items()}
            report['results'].append({'mode': mode, 'input_bytes': size, 'iterations': args.iterations, 'output_and_retained_input_checksums': checksums.pop(), 'raw_samples_ns': elapsed, 'warmup_and_sample_observations': observations, 'median_ns_per_iteration': medians, 'relative_to_reference': {label: value / medians['reference'] for label, value in medians.items() if label != 'reference'}})
            report_path.write_text(json.dumps(report, indent=2) + '\n')
            print(mode, size, medians, flush=True)
    report['quota_after'] = Path('/sys/fs/cgroup/cpu.stat').read_text()
    report_path.write_text(json.dumps(report, indent=2) + '\n')


if __name__ == '__main__':
    main()
