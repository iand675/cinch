#!/usr/bin/env python3
"""Build and measure full MIME public pipelines against a pinned reference.

All samples, allocation-policy differences and regressions remain visible. The
scope includes fresh output allocation and an observable scalar output checksum;
input generation and decode-fixture encoding occur before the measured interval.
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


def sample(program: Path, mode: str, size: int, count: int, work: Path) -> tuple[int, int, int]:
    """Retain one pipeline's elapsed time and independently comparable result checksum."""
    env = {**os.environ, 'GOMAXPROCS': '1', 'GOTELEMETRY': 'off', 'CINCH_HEAP_LIMIT': '32M', 'CINCH_STACK_SIZE': '8M', 'CINCH_LEAK_CHECK': '1', 'CINCH_RC_DEBUG': '0'}
    result = run([str(program), mode, str(size), str(count)], work, 60, env)
    if program.name == 'bench' and 'leaked=0' not in result.stderr:
        raise RuntimeError('missing zero-leak benchmark evidence: ' + result.stderr)
    return tuple(map(int, result.stdout.split()))


def functional_smoke(exe: Path, reference: Path, sizes: list[int], count: int, work: Path) -> None:
    """Check independent pipeline checksums and both heap profiles without claiming timing samples."""
    rows = []
    for mode in ('parse', 'format', 'b-encode', 'q-encode', 'header', 'header-q', 'registry'):
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
    parser.add_argument('--scalar', action='store_true', help='Prepare a source snapshot replacing vector scanning and packed lowercase with the same scalar predicates')
    parser.add_argument('--run-only', action='store_true')
    parser.add_argument('--smoke-only', action='store_true', help='Check both heap profiles and independent checksums without paired timing')
    parser.add_argument('--sizes', nargs='+', type=int, default=[64, 4096])
    parser.add_argument('--iterations', type=int, default=50)
    parser.add_argument('--samples', type=int, default=7)
    parser.add_argument('--context', default='', help='Observed shared-machine background during the exclusive timing slot')
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
    driver = ROOT / 'tooling/data/mime-benchmark.cinch'
    reference_source = ROOT / 'tooling/data/mime-benchmark.go'
    exe, reference = work / 'bench', work / 'reference'
    report_path = work / 'report.json'
    if not args.run_only:
        closure = source_closure(driver.read_text())
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
            source = work / 'Text.Mime.cinch'
            contents = source.read_text()
            begin = contents.index('  scan-end kind input at limit =')
            end = contents.index('\n  -- | Lower one byte', begin)
            contents = contents[:begin] + """  scan-end kind input at limit =
    if at < limit then
      if class-byte kind (bytes-get input at) then scan-end kind input (at + 1) limit else at
    else at
""" + contents[end:]
            begin = contents.index('  lower-ascii-copy input start count out at =')
            end = contents.index('\n  -- | ', begin)
            contents = contents[:begin] + """  lower-ascii-copy input start count out at =
    if at < count && start + at < bytes-len input && at < bytes-len out then
      let stored = bytes-set out at (lower-ascii-byte (bytes-get input (start + at))) in
      lower-ascii-copy input start count out (at + 1)
    else ()
""" + contents[end:]
            source.write_text(contents)
        shutil.copyfile(driver, work / 'Main.cinch')
        report = {'format': 1, 'scalar_snapshot': args.scalar, 'compile_rc_debug': False, 'runner_sha256': digest(Path(__file__)), 'compiler': str(compiler), 'compiler_sha256': digest(compiler), 'runtime': str(runtime), 'runtime_sha256': digest(runtime), 'driver_sha256': digest(driver), 'reference_source_sha256': digest(reference_source), 'source_sha256': {name: digest(work / (name + '.cinch')) for name in ordered}, 'compiler_host_stack_bytes': 64 * 1024 * 1024, 'compiler_environment': {'RUST_MIN_STACK': '67108864', 'CINCH_STACK_SIZE': '64M'}, 'allocation_policy': 'Cinch returns fresh mutable byte outputs; the reference uses immutable strings and may reuse an unchanged input.', 'registry_context_scope': 'Cinch owns one custom association in an explicit empty context; the public reference retains its ambient host registry plus the same custom association. Queries and returned bytes match, but total index populations differ.', 'profile_scope': 'Cinch executable only; the public reference retains its normal GC, heap and goroutine-stack policies.', 'profile': {'heap': '32M', 'stack': '8M', 'poison': False, 'leak_check': True}, 'scope': __doc__, 'results': []}
        ir = work / 'bench.ll'
        if args.native:
            snap = args.native.resolve()
            config = json.loads((snap / 'objects.json').read_text())
            provider = Path(config['object'])
            (work / 'bundle.source').write_text('\n'.join((work / (name + '.cinch')).read_text() for name in ordered) + '\n' + driver.read_text())
            command = [str(compiler), '--emit-ir', '--interfaces', str(snap / 'base-interfaces.json'), '--objects', str(snap / 'objects.json'), str(work / 'bundle.source'), str(ir)]
            report['native'] = {name: digest(snap / name) for name in ('base-interfaces.json', 'objects.json')}
            report['native']['provider_sha256'] = digest(provider)
        else:
            command = [str(compiler), 'emit-ir', str(work / 'Main.cinch'), '-o', str(ir)]
        result = run(command, work, 300, {**os.environ, 'RUST_MIN_STACK': '67108864', 'CINCH_STACK_SIZE': '64M'})
        report['compile'] = {'command': command, 'stdout': result.stdout, 'stderr': result.stderr}
        target = re.search(r'target triple = "([^"]+)"', ir.read_text())
        triple = target.group(1) if target else config['target']
        link = [clang, '-O2', '--target=' + triple, str(ir)]
        if args.native:
            link.append(str(provider))
        run(link + [str(runtime), '-ldl', '-lpthread', '-lm', '-o', str(exe)], work)
        run([clang, '-O2', '-S', str(ir), '-o', str(work / 'bench.s')], work)
        env = {**os.environ, 'GOMAXPROCS': '1', 'GOTELEMETRY': 'off', 'GOCACHE': '/tmp/cinch-mime-go-cache'}
        run([str(args.go), 'build', '-o', str(reference), str(reference_source)], work, 180, env)
        report.update(executable_sha256=digest(exe), reference_executable_sha256=digest(reference), llvm_sha256=digest(ir), assembly_sha256=digest(work / 'bench.s'), target_triple=triple, sdk=run([str(args.go), 'version'], work, env=env).stdout.strip(), platform=platform.platform(), cpu=next(line.split(':', 1)[1].strip() for line in Path('/proc/cpuinfo').read_text().splitlines() if line.startswith('model name')), clang=run([clang, '--version'], work).stdout.splitlines()[0])
        report_path.write_text(json.dumps(report, indent=2) + '\n')
    else:
        report = json.loads(report_path.read_text())
        if report['scalar_snapshot'] != args.scalar:
            raise RuntimeError('changed scalar/SIMD benchmark selection')
        for key, path in [('compiler_sha256', compiler), ('runtime_sha256', runtime), ('driver_sha256', driver), ('reference_source_sha256', reference_source), ('executable_sha256', exe), ('reference_executable_sha256', reference)]:
            if report[key] != digest(path):
                raise RuntimeError('changed benchmark identity: ' + key)
    if args.build_only:
        return
    if args.smoke_only:
        functional_smoke(exe, reference, args.sizes, args.iterations, work)
        return
    report.update(cpu_affinity=[args.cpu], samples=args.samples, iterations=args.iterations, timing_runner_sha256=digest(Path(__file__)), background_context=args.context)
    for mode in ('parse', 'format', 'b-encode', 'q-encode', 'header', 'header-q', 'registry'):
        for size in args.sizes:
            elapsed = {'cinch': [], 'reference': []}
            checksums = set()
            for label, program in [('cinch', exe), ('reference', reference)]:
                checksums.add(sample(program, mode, size, args.iterations, work)[1:])
            for index in range(args.samples):
                order = [('cinch', exe), ('reference', reference)]
                for label, program in order if index % 2 == 0 else reversed(order):
                    nanos, checksum, retained = sample(program, mode, size, args.iterations, work)
                    elapsed[label].append(nanos)
                    checksums.add((checksum, retained))
            if len(checksums) != 1:
                raise RuntimeError(f'{mode}/{size}: independent checksums differ: {checksums}')
            medians = {label: statistics.median(values) / args.iterations for label, values in elapsed.items()}
            report['results'].append({'mode': mode, 'input_bytes': size, 'iterations': args.iterations, 'output_and_retained_input_checksums': checksums.pop(), 'raw_samples_ns': elapsed, 'median_ns_per_iteration': medians, 'cinch_over_reference': medians['cinch'] / medians['reference']})
            report_path.write_text(json.dumps(report, indent=2) + '\n')
            print(mode, size, medians, flush=True)


if __name__ == '__main__':
    main()
