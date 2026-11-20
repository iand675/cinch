#!/usr/bin/env python3
"""Build and measure actual changed-input split pipelines with scalar-search controls."""
from __future__ import annotations
import argparse
import hashlib
import json
import os
from pathlib import Path
import statistics

from bench_stdlib import source_closure
from bench_byte_view import scalar_source
from generate_byte_split_vectors import split
from validate_byte_view import digest, run

ROOT = Path(__file__).resolve().parents[1]


def checked_hash(text: str) -> str:
    """Match the checked compiler interface's complete source identity."""
    value = 14695981039346656037
    for byte in text.encode():
        value = ((value ^ byte) * 1099511628211) & ((1 << 64) - 1)
    return str(value)


def expected(name: str, size: int, iterations: int) -> tuple[int, str, str, str]:
    """Model every changed output boundary, sampled endpoint byte and retained owner independently."""
    if name == 'dense':
        raw = bytearray((b'a,' * ((size + 1)//2))[:size])
    elif name in ['sparse', 'prefix']:
        raw = bytearray(b'a' * (size-1) + (b',' if name == 'sparse' else b'b'))
    else:
        seed = b'a\r\nb\n' if name == 'lines' else bytes.fromhex('61f09f9982ffc3a9')
        raw = bytearray((seed * ((size+len(seed)-1)//len(seed)))[:size])
    delimiter = b'' if name == 'utf8' else b'a'*256 + b'b' if name == 'prefix' else b','
    original = hashlib.sha256(raw).hexdigest()
    total = 0
    for index in range(iterations):
        raw[0] = 121 if index % 2 == 0 else 122
        value = 0
        for chunk in split(bytes(raw), delimiter, -1, False, name == 'lines'):
            value = (value*65599 + len(chunk)*257 + (chunk[0]*17+chunk[-1] if chunk else 0) + 1) & ((1 << 64) - 1)
        total = (total + value) & ((1 << 64) - 1)
    return total, original, hashlib.sha256(raw).hexdigest(), hashlib.sha256(delimiter).hexdigest()


def main() -> None:
    """Separate bounded seed/ownership preparation from explicitly granted exclusive timing."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--compiler', type=Path, required=True)
    parser.add_argument('--driver-source', type=Path, default=ROOT / 'benchmarks/stdlib/byte-split-driver.cinch')
    parser.add_argument('--split-source', type=Path, default=ROOT / 'seed/prelude/Data/Bytes/Split.cinch')
    parser.add_argument('--before-split-source', type=Path)
    parser.add_argument('--view-source', type=Path, default=ROOT / 'seed/prelude/Data/Bytes/View.cinch')
    parser.add_argument('--runtime-archive', type=Path, required=True)
    parser.add_argument('--native-interfaces', type=Path)
    parser.add_argument('--native-objects', type=Path)
    parser.add_argument('--artifacts', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--build-only', action='store_true')
    parser.add_argument('--measure-only', action='store_true')
    parser.add_argument('--build-receipt', type=Path)
    parser.add_argument('--preflight-only', action='store_true')
    parser.add_argument('--samples', type=int, default=3)
    parser.add_argument('--iteration-scale', type=int, default=1)
    parser.add_argument('--cpu', type=int, required=True)
    args = parser.parse_args()
    assert args.iteration_scale > 0
    driver = args.driver_source.read_text()
    view = args.view_source.read_text()
    selected_split = args.split_source.read_text()
    variants = ([('initial', view, args.before_split_source.read_text()), ('candidate', view, selected_split)] if args.before_split_source
                else [('scalar', scalar_source(view), selected_split), ('packed', view, selected_split)])
    report = {'driver_sha256':hashlib.sha256(driver.encode()).hexdigest(), 'compiler_sha256':digest(args.compiler),
              'runtime_sha256':digest(args.runtime_archive), 'cpu':args.cpu, 'source_sha256':digest(args.split_source),
              'scope':'complete changed-input public split/list/fold/cursor pipelines including normal per-operation output cleanup; retained input/setup/digests outside clocks',
              'compiled_rc_debug':bool(args.native_interfaces),
              'rust_cli_profile_note':'Default rc_debug=false; dynamic normal/poison profiles are separate',
              'program_stack_bytes':8388608,'compiler_stack_bytes':67108864,'executables':{},'dependencies':{},'checked_sources':{},'results':[]}
    report['iteration_scale'] = args.iteration_scale
    report['samples_per_variant'] = 1 if args.preflight_only else args.samples
    if args.native_interfaces:
        assert args.native_objects
        report['interfaces_sha256'] = digest(args.native_interfaces)
        report['objects_sha256'] = digest(args.native_objects)
    previous = None
    if args.measure_only:
        assert args.build_receipt, '--measure-only requires the original build receipt'
        previous = json.loads(args.build_receipt.read_text())
        for key in ['driver_sha256','compiler_sha256','runtime_sha256','source_sha256']:
            assert previous[key] == report[key], key
        if args.native_interfaces:
            assert previous['interfaces_sha256'] == report['interfaces_sha256']
            assert previous['objects_sha256'] == report['objects_sha256']
    for variant, source, split_source in variants:
        work = args.artifacts / variant
        work.mkdir(parents=True, exist_ok=True)
        entry = work / 'Main.cinch'
        if args.measure_only:
            assert entry.read_text() == driver
        else:
            entry.write_text(driver)
        dependencies = source_closure(driver)
        hashes = {}
        checked = {}
        for name, path in dependencies.items():
            text = source if name == 'Data.Bytes.View' else split_source if name == 'Data.Bytes.Split' else path.read_text()
            retained = work / (name + '.cinch')
            if args.measure_only:
                assert retained.read_text() == text, name
            else:
                retained.write_text(text)
            hashes[name] = hashlib.sha256(text.encode()).hexdigest()
        exe = (work / 'consumer').resolve()
        if not args.measure_only:
            if args.native_interfaces:
                bundle = work / 'Bundle.cinch'
                bundle.write_text('\n'.join(source if name == 'Data.Bytes.View' else split_source if name == 'Data.Bytes.Split' else path.read_text()
                                             for name,path in reversed(list(dependencies.items()))) + '\n' + driver)
                try:
                    run([str(args.compiler.resolve()), '--emit-ir', '--rc-debug', '--interfaces', str(args.native_interfaces.resolve()),
                         '--objects', str(args.native_objects.resolve()), str(bundle.resolve()), str(exe.with_suffix('.ll'))])
                except RuntimeError as error:
                    raise RuntimeError(str(error) + '\n' + exe.with_suffix('.ll.diagnostics').read_text()) from error
                configuration = json.loads(args.native_objects.read_text())
                assert digest(Path(configuration['object'])) == configuration['object_sha256']
                run([os.environ.get('CINCH_CLANG', 'clang'), '-O2', str(exe.with_suffix('.ll')), configuration['object'],
                     str(args.runtime_archive.resolve()), '-o', str(exe), '-ldl', '-lpthread', '-lm'])
            else:
                run([str(args.compiler.resolve()), 'build', str(entry.resolve()), '-o', str(exe), '--opt-level', '2',
                     '--runtime-archive', str(args.runtime_archive.resolve())])
        report['executables'][variant] = {'executable_sha256':digest(exe),'llvm_sha256':digest(exe.with_suffix('.ll')),
                                          'view_sha256':hashlib.sha256(source.encode()).hexdigest(),
                                          'split_sha256':hashlib.sha256(split_source.encode()).hexdigest()}
        report['dependencies'][variant] = hashes
        if not args.native_interfaces:
            for name in ['Main', *dependencies]:
                text = (entry if name == 'Main' else work / (name + '.cinch')).read_text()
                identity = json.loads((work / 'build' / (name + '.cinchif')).read_text())['source_hash']
                assert identity == checked_hash(text), name
                checked[name] = identity
            report['checked_sources'][variant] = checked
        if previous:
            assert all(report['executables'][variant][key] == value for key,value in previous['executables'][variant].items()), variant
            if 'dependencies' in previous:
                assert previous['dependencies'][variant] == hashes
            else:
                assert not args.native_interfaces and checked, 'legacy receipts require actual checked source interfaces'
    if not args.build_only:
        for kind in ['list', 'producer', 'cursor', 'stable']:
            for name in ['dense', 'sparse', 'prefix', 'utf8', 'lines']:
                if name == 'lines' and kind != 'producer':
                    continue
                for size, iterations in [(4096, 2), (65536, 1)]:
                    iterations *= args.iteration_scale
                    wanted = expected(name, size, iterations)
                    row = {'pipeline':kind,'pattern':name,'bytes':size,'iterations':iterations,'checksum':wanted[0],
                           'input_sha256':wanted[1],'retained_sha256':wanted[2],'pattern_sha256':wanted[3],
                           'samples':{variant:[] for variant,_,_ in variants},'excluded_warmup':{variant:[] for variant,_,_ in variants}}
                    for sample in range(0 if args.preflight_only else -1, 1 if args.preflight_only else args.samples):
                        for variant, source, split_source in (variants if sample % 2 == 0 else list(reversed(variants))):
                            for debug in ([False, True] if args.preflight_only else [False]):
                                exe = args.artifacts / variant / 'consumer'
                                result = run(['taskset','-c',str(args.cpu),str(exe.resolve()),kind,name,str(size),str(iterations)],
                                             env={**os.environ,'CINCH_RC_DEBUG':str(int(debug)),'CINCH_HEAP_LIMIT':'256M' if debug else '32M',
                                                  'CINCH_STACK_SIZE':'8M','CINCH_LEAK_CHECK':'1'})
                                assert 'leaked=0' in result.stderr
                                elapsed, checksum, before, after, stable = result.stdout.split()
                                assert (int(checksum),before,after,stable) == wanted, (row,result.stdout)
                                row['samples' if sample >= 0 else 'excluded_warmup'][variant].append(int(elapsed))
                    row['median_ns'] = {variant:statistics.median(values) for variant,values in row['samples'].items()}
                    key = 'initial_over_candidate' if args.before_split_source else 'scalar_over_packed'
                    row[key] = row['median_ns'][variants[0][0]] / row['median_ns'][variants[1][0]]
                    report['results'].append(row)
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(report, indent=2) + '\n')


if __name__ == '__main__':
    main()
