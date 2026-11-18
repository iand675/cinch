#!/usr/bin/env python3
"""Compile the complete public palette closure and supplied foreign seed into real target objects."""
from __future__ import annotations
import argparse
import json
import os
from pathlib import Path
import re
import resource
import shutil
import subprocess
from bench_stdlib import source_closure
from check_compare import digest
from inspect_color_code import definitions, path_to, assembly_body


def object_header(path: Path, triple: str) -> dict:
    """Require a real relocatable object with the requested target class, endianness and machine."""
    header = path.read_bytes()[:64]
    if triple == 'wasm32-unknown-unknown':
        if header[:8] != b'\x00asm\x01\x00\x00\x00':
            raise RuntimeError('wrong actual wasm object header')
        return {'format': 'wasm', 'version': 1}
    if header[:4] != b'\x7fELF' or header[4:6] != b'\x02\x01':
        raise RuntimeError('wrong actual ELF64/little-endian object header')
    machine = int.from_bytes(header[18:20], 'little')
    expected = {'x86_64-unknown-linux-gnu': 62, 'aarch64-unknown-linux-gnu': 183}[triple]
    if machine != expected or int.from_bytes(header[16:18], 'little') != 1:
        raise RuntimeError('wrong actual target/relocatable object identity')
    return {'format': 'ELF64', 'endian': 'little', 'machine': machine, 'type': 'relocatable'}


def main() -> None:
    """Check source first and preserve the first genuine source/code/object/runtime failure."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--compiler', type=Path, required=True)
    parser.add_argument('--runtime', type=Path, required=True)
    parser.add_argument('--source', type=Path, required=True)
    parser.add_argument('--color', type=Path, required=True)
    parser.add_argument('--probe', type=Path, required=True)
    parser.add_argument('--seed', type=Path, required=True)
    parser.add_argument('--work', type=Path, required=True)
    parser.add_argument('--cpu', type=int, default=1)
    args = parser.parse_args()
    work = args.work.resolve()
    work.mkdir(parents=True, exist_ok=False)
    initial = shutil.disk_usage(work).free
    if initial < 350*1024*1024:
        raise RuntimeError('350MiB initial portable-unit guard')
    os.sched_setaffinity(0, {args.cpu})
    resource.setrlimit(resource.RLIMIT_STACK, (67108864, resource.getrlimit(resource.RLIMIT_STACK)[1]))
    compiler, runtime = args.compiler.resolve(), args.runtime.resolve()
    clang = os.environ.get('CINCH_CLANG', 'clang')
    report = {'scope': __doc__, 'runner_sha256': digest(Path(__file__)), 'compiler': str(compiler),
              'compiler_sha256': digest(compiler), 'host_runtime': str(runtime), 'host_runtime_sha256': digest(runtime),
              'source_sha256': digest(args.source), 'color_sha256': digest(args.color),
              'probe_sha256': digest(args.probe), 'seed_sha256': digest(args.seed),
              'compiler_host_stack_bytes': 67108864, 'cpu_affinity': [args.cpu],
              'initial_free_bytes': initial, 'targets': [], 'commands': [], 'profiles': [],
              'foreign_execution_claim': False}

    def save() -> None:
        """Persist exact commands and diagnostics, retaining all source and defining artifacts plain."""
        report.update(final_free_bytes=shutil.disk_usage(work).free,
                      output_bytes=sum(path.stat().st_size for path in work.rglob('*') if path.is_file()))
        (work / 'report.json').write_text(json.dumps(report, indent=2)+'\n')

    def execute(command: list[str], folder: Path, env: dict) -> dict:
        """Stop on the first command or reserve/output failure without substituting another frontend."""
        save()
        if report['final_free_bytes'] < 250*1024*1024 or report['output_bytes'] > 30*1024*1024:
            raise RuntimeError('250MiB reserve/30MiB portable-unit cap')
        row = {'command': command, 'cwd': str(folder)}
        report['commands'].append(row)
        try:
            result = subprocess.run(command, cwd=folder, text=True, capture_output=True, env=env, timeout=30)
            row.update(returncode=result.returncode, stdout=result.stdout, stderr=result.stderr)
        except BaseException as error:
            row['failure'] = repr(error)
            raise
        finally:
            save()
        if row['returncode']:
            raise RuntimeError('first portable source/object/runtime failure')
        if report['final_free_bytes'] < 250*1024*1024 or report['output_bytes'] > 30*1024*1024:
            raise RuntimeError('post-phase portable reserve/output cap')
        return row

    try:
        caller = args.probe.read_text()
        closure = source_closure(caller, {'Image.Color': args.color, 'Image.Color.Palette': args.source})
        report['source_closure_sha256'] = {name: digest(path) for name, path in closure.items()}
        host = {**os.environ, 'RUST_MIN_STACK': '67108864', 'CINCH_STACK_SIZE': '64M'}
        for label, triple in [('host', 'x86_64-unknown-linux-gnu'), ('aarch64', 'aarch64-unknown-linux-gnu'), ('wasm32', 'wasm32-unknown-unknown')]:
            folder = work / label
            folder.mkdir()
            for name, source in closure.items():
                shutil.copyfile(source, folder / (name+'.cinch'))
            shutil.copyfile(args.probe, folder / 'Main.cinch')
            shutil.copyfile(args.seed, folder / 'seed.c')
            entry = folder / 'Main.cinch'
            execute([str(compiler), 'check', str(entry), '--target', triple], folder, host)
            execute([str(compiler), 'emit-ir', str(entry), '--target', triple, '-o', str(folder / 'probe.ll')], folder, host)
            bodies = definitions((folder / 'probe.ll').read_text())
            paths = {}
            for symbol in ['Image.Color.Palette.plan9-colors', 'Image.Color.Palette.web-safe-colors', 'Image.Color.distance4.llvm', 'Image.Color.expand8.llvm']:
                path = path_to(bodies, symbol)
                if path is None:
                    raise RuntimeError('missing actual portable entrypoint path: '+symbol)
                paths[symbol] = path
            for name in ['distance4', 'expand8']:
                expected = re.search(r'  prim '+name+r' :: [^\n]+ where\s+llvm \{(.*?)\n    \}', args.color.read_text(), re.S).group(1)
                actual = bodies['Image.Color.'+name+'.llvm'].split('\n', 1)[1].removesuffix('\n}')
                expected_lines = [line.strip() for line in expected.splitlines() if line.strip()]
                actual_lines = [line.strip() for line in actual.splitlines() if line.strip() and line.strip() != 'entry:']
                if expected_lines != actual_lines:
                    raise RuntimeError('changed actual portable kernel body: '+name)
            execute([clang, '-O2', '--target='+triple, '-c', str(folder / 'probe.ll'), '-o', str(folder / 'probe.o')], folder, host)
            execute([clang, '-O2', '--target='+triple, '-ffreestanding', '-fno-builtin', '-c', str(folder / 'seed.c'), '-o', str(folder / 'seed.o')], folder, host)
            target = {'target': triple, 'ir_sha256': digest(folder / 'probe.ll'),
                      'object_sha256': digest(folder / 'probe.o'), 'seed_object_sha256': digest(folder / 'seed.o'),
                      'object_header': object_header(folder / 'probe.o', triple),
                      'seed_object_header': object_header(folder / 'seed.o', triple), 'actual_public_paths': paths}
            report['targets'].append(target)
            if label == 'host':
                execute([clang, '-O2', '--target='+triple, '-S', str(folder / 'probe.ll'), '-o', str(folder / 'probe.s')], folder, host)
                machine = assembly_body((folder / 'probe.s').read_text(), 'Image.Color.nearest#cc')
                target['nearest_host_body'] = machine
                target['actual_integer_vector_instructions'] = sorted(set(re.findall(r'\b(?:pmul\w*|padd\w*|psub\w*|psrl\w*|punpck\w*)\b', machine)))
                if not target['actual_integer_vector_instructions']:
                    raise RuntimeError('no actual nearest integer vector instructions')
                for optimization in [0, 2]:
                    binary = folder / ('probe-O'+str(optimization))
                    execute([clang, '-O'+str(optimization), '--target='+triple, str(folder / 'probe.ll'), str(folder / 'seed.o'),
                             str(runtime), '-ldl', '-lpthread', '-lm', '-o', str(binary)], folder, host)
                    for poison, heap in [('0', '32M'), ('1', '256M')]:
                        result = execute([str(binary)], folder, {**os.environ, 'CINCH_HEAP_LIMIT': heap,
                            'CINCH_STACK_SIZE': '8M', 'CINCH_LEAK_CHECK': '1', 'CINCH_RC_DEBUG': poison, 'CINCH_ALLOC_STATS': '0'})
                        if 'leaked=0' not in result['stderr']:
                            raise RuntimeError('missing zero-leak host portable control')
                        report['profiles'].append({'opt': optimization, 'poison': poison, 'heap': heap, 'stack': '8M',
                                                   'stdout': result['stdout'], 'stderr': result['stderr']})
        report['status'] = 'passed'
    except BaseException as error:
        report.update(status='failed-or-resource-paused', failure=repr(error), failed_artifacts_retained_plain=True)
        raise
    finally:
        save()
    print('complete public palette host/ARM/wasm objects and four host profiles passed')


if __name__ == '__main__':
    main()
