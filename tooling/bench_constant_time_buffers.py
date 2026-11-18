"""Preflight complete copy/XOR outputs, then compare byte and packed public pipelines."""
from pathlib import Path
import argparse
import hashlib
import json
import os
import resource
import statistics
import subprocess


def digest(path):
    """Identify each exact source, compiler, runtime, and observed executable."""
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def pattern(size, salt):
    """Construct independent host input bytes without using a tested implementation."""
    return bytearray((i * 73 + salt * 37 + i // 7) % 256 for i in range(size))


def expected(size, count, mode):
    """Observe every final byte using independent alias and selection semantics."""
    destination = pattern(size, 3)
    first, alternate, right = pattern(size, 1), pattern(size, 4), pattern(size, 2)
    if count:
        if mode == 1:
            destination = alternate
        elif mode == 4:
            destination = bytearray(a ^ b for a, b in zip(alternate, right))
        elif mode == 5 and count % 2:
            destination = bytearray(a ^ b for a, b in zip(destination, right))
        elif mode == 6:
            for data, occurrences in [(first, count // 2), (alternate, (count + 1) // 2)]:
                if occurrences % 2:
                    destination = bytearray(a ^ b for a, b in zip(destination, data))
        elif mode == 7:
            destination = bytearray(size)
    value = 14695981039346656037
    for byte in destination:
        value = ((value ^ byte) * 1099511628211) & ((1 << 64) - 1)
    return value


def scalar_source(source):
    """Create a benchmark-only byte baseline by removing the two packed fast paths."""
    for name in ['copy-from selected destination source position =',
                 'xor-from destination left right count position =']:
        start = source.index('    if position + 8', source.index(name))
        end = source.index('    else if position <', start)
        source = source[:start] + '    if position <' + source[end + len('    else if position <'):]
    return source.replace('Visit each public-size word and byte', 'Visit each public-size byte')


def main():
    """Keep preflight profiles distinct from paired ordinary-RC timing samples."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--compiler', type=Path, required=True)
    parser.add_argument('--runtime', type=Path, required=True)
    parser.add_argument('--clang', required=True)
    parser.add_argument('--samples', type=int, default=7)
    parser.add_argument('--work-bytes', type=int, default=1024 * 1024)
    parser.add_argument('--artifact-root', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--preflight-only', action='store_true')
    args = parser.parse_args()
    if args.samples < 1 or args.work_bytes < 1:
        parser.error('samples and work bytes must be positive')
    if args.artifact_root.exists() and any(args.artifact_root.iterdir()):
        parser.error('fresh emission requires an empty artifact directory; select another epoch')
    if args.output.exists():
        parser.error('select a new output path to preserve earlier measurements')
    soft, hard = resource.getrlimit(resource.RLIMIT_STACK)
    resource.setrlimit(resource.RLIMIT_STACK, (max(soft, 64 * 1024 * 1024), hard))
    root = Path(__file__).resolve().parents[1]
    work = args.artifact_root.resolve()
    work.mkdir(parents=True, exist_ok=True)
    args.output.parent.mkdir(parents=True, exist_ok=True)
    library = (root / 'compiler/prelude/Crypto/ConstantTime.cinch').read_text()
    support = root / 'benchmarks/stdlib/constant-time-equality.c'
    report = {'compiler_sha256': digest(args.compiler), 'runtime_sha256': digest(args.runtime),
              'support_sha256': digest(support), 'observer_sha256': digest(__file__),
              'baseline': 'benchmark-only scalar-byte implementation; not selected library source',
              'compiled_rc_debug': False, 'artifacts': {}, 'preflight': [], 'timings': [],
              'modes': ['control', 'copy-true', 'copy-false', 'copy-self', 'xor-disjoint',
                        'xor-left', 'xor-right', 'xor-all-same'],
              'quota': Path('/sys/fs/cgroup/cpu.max').read_text(),
              'cpu_stat_before': Path('/sys/fs/cgroup/cpu.stat').read_text()}

    def save():
        """Retain completed observations before any dependent assertion or build."""
        args.output.write_text(json.dumps(report, indent=2) + '\n')

    executables = {}
    for variant in ['scalar', 'packed']:
        directory = work / variant
        directory.mkdir(exist_ok=True)
        (directory / 'Crypto.ConstantTime.cinch').write_text(
            scalar_source(library) if variant == 'scalar' else library)
        for name, source in [('Data.Bytes.Word.cinch', root / 'compiler/prelude/Data/Bytes/Word.cinch'),
                             ('Main.cinch', root / 'benchmarks/stdlib/constant-time-buffers.cinch')]:
            (directory / name).write_bytes(source.read_bytes())
        ir, executable = directory / 'consumer.ll', directory / 'consumer'
        for label, command in [
            ('emit', [args.compiler.resolve(), 'emit-ir', directory / 'Main.cinch', '-o', ir]),
            ('link', [args.clang, '-O2', ir, support, args.runtime.resolve(), '-lpthread', '-ldl', '-lm', '-o', executable]),
        ]:
            result = subprocess.run(command, capture_output=True, text=True, timeout=90)
            (directory / (label + '.log')).write_text(result.stdout + result.stderr)
            assert result.returncode == 0, result.stderr
        executables[variant] = executable
        report['artifacts'][variant] = {
            'source_sha256': {p.name: digest(p) for p in directory.glob('*.cinch')},
            'llvm_sha256': digest(ir), 'executable_sha256': digest(executable)}
    save()

    def run(variant, size, count, mode, poisoned):
        """Check all output bytes and progress in each fixed public workload."""
        result = subprocess.run(['constant-time-buffers', str(size), str(count), str(mode)],
                                executable=str(executables[variant]), capture_output=True, text=True,
                                env={**os.environ, 'CINCH_RC_DEBUG': '1' if poisoned else '0',
                                     'CINCH_HEAP_LIMIT': '256M' if poisoned else '32M',
                                     'CINCH_STACK_SIZE': '8M', 'CINCH_LEAK_CHECK': '1',
                                     'CINCH_ALLOC_STATS': '0'}, timeout=45)
        assert result.returncode == 0, (variant, size, count, mode, result.stderr)
        assert 'cinch-leak: live=0 immortal=0 leaked=0' in result.stderr, result.stderr
        elapsed, consumed, observed = map(int, result.stdout.split())
        assert consumed == size * count and observed == expected(size, count, mode), (variant, size, count, mode, result.stdout)
        return {'variant': variant, 'size': size, 'iterations': count, 'mode': mode,
                'poisoned': poisoned, 'nanoseconds': elapsed, 'consumed': consumed,
                'complete_output_hash': observed}

    for variant in executables:
        for size in [0, 1, 7, 8, 9, 15, 16, 17, 256, 4096, 65536]:
            for mode in range(8):
                for poisoned in [False, True]:
                    report['preflight'].append(run(variant, size, 3, mode, poisoned))
    save()
    if args.preflight_only:
        report['cpu_stat_after'] = Path('/sys/fs/cgroup/cpu.stat').read_text()
        report['status'] = 'preflight-passed'
        save()
        print(json.dumps({'preflight': len(report['preflight']), 'timings': 0}))
        return
    for sample in range(args.samples):
        for size in [16, 256, 4096, 65536]:
            count = max(16, args.work_bytes // size)
            for mode in range(8):
                order = ['scalar', 'packed'] if sample % 2 == 0 else ['packed', 'scalar']
                for variant in order:
                    if sample == 0:
                        run(variant, size, max(2, count // 10), mode, False)
                    row = run(variant, size, count, mode, False)
                    row['sample'] = sample
                    report['timings'].append(row)
        save()
    report['medians'] = []
    for size in [16, 256, 4096, 65536]:
        for mode in range(8):
            medians = {v: statistics.median(r['nanoseconds'] for r in report['timings']
                        if r['variant'] == v and r['size'] == size and r['mode'] == mode)
                       for v in executables}
            report['medians'].append({'size': size, 'mode': mode, **medians,
                                      'speedup': medians['scalar'] / medians['packed']})
    report['cpu_stat_after'] = Path('/sys/fs/cgroup/cpu.stat').read_text()
    report['status'] = 'passed'
    save()
    print(json.dumps({'preflight': len(report['preflight']), 'timings': len(report['timings']),
                      'medians': report['medians']}))


if __name__ == '__main__':
    main()
