#!/usr/bin/env python3
"""Test actual checked color LLVM bodies against independent scalar controls and portable objects."""
from __future__ import annotations
import argparse
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import time
from check_compare import digest


#: Exact private arithmetic bodies required by the complete selected source.
KERNELS = {'expand8', 'premultiply', 'unpremultiply', 'gray-sum', 'ycbcr16',
           'rgb-ycbcr', 'rgb-cmyk', 'cmyk16', 'distance4'}


def main() -> None:
    """Extract exact source bodies, stop at first failure and retain all object/assembly evidence."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--source', type=Path, required=True)
    parser.add_argument('--fixture', type=Path, required=True)
    parser.add_argument('--work', type=Path, required=True)
    parser.add_argument('--cpu', type=int, required=True)
    args = parser.parse_args()
    work = args.work.resolve()
    work.mkdir(parents=True, exist_ok=False)
    initial = shutil.disk_usage(work).free
    if initial < 350 * 1024 * 1024:
        raise RuntimeError('350 MiB initial kernel-unit guard; no compiler started')
    os.sched_setaffinity(0, {args.cpu})
    source = args.source.read_text()
    matches = re.findall(r'^  prim ([\w-]+) :: ([^\n]+) where\s+llvm \{(.*?)\n    \}', source, re.M | re.S)
    if {name for name, _, _ in matches} != KERNELS or len(matches) != len(KERNELS):
        raise RuntimeError('missing or duplicate actual numeric kernel')
    ir = ''
    for name, signature, body in matches:
        types = signature.split(' -> ')
        if any(value not in ('U64', 'U32') for value in types):
            raise RuntimeError('unexpected arithmetic signature: ' + signature)
        llvm_types = ['i64' if value == 'U64' else 'i32' for value in types]
        arguments = ', '.join(value + ' %' + str(index) for index, value in enumerate(llvm_types[:-1]))
        ir += 'define ' + llvm_types[-1] + ' @color_' + name.replace('-', '_') + '(' + arguments + ') {\n' + body + '\n}\n'
    (work / 'kernels.ll').write_text(ir)
    shutil.copyfile(args.fixture, work / 'fixture.c')
    clang = os.environ.get('CINCH_CLANG', 'clang')
    report = {'scope': __doc__, 'source_sha256': digest(args.source),
              'fixture_sha256': digest(args.fixture), 'actual_ir_sha256': digest(work / 'kernels.ll'),
              'runner_sha256': digest(Path(__file__)), 'cpu_affinity': [args.cpu],
              'resource_policy': {'initial_mib': 350, 'reserve_mib': 250, 'unit_mib': 30},
              'initial_free_bytes': initial, 'commands': [], 'kernel_names': sorted(KERNELS),
              'limits': 'Native scalar C numeric controls only; not a Cinch ownership/heap or foreign execution claim.'}

    def execute(command: list[str]) -> dict:
        """Keep exact diagnostics and fail closed on resource, compiler or numeric discrepancies."""
        if shutil.disk_usage(work).free < 250*1024*1024 or sum(p.stat().st_size for p in work.rglob('*') if p.is_file()) > 30*1024*1024:
            raise RuntimeError('250 MiB reserve /30 MiB output guard')
        row = {'command': command}
        report['commands'].append(row)
        started = time.monotonic()
        try:
            result = subprocess.run(command, cwd=work, text=True, capture_output=True, timeout=30)
            row.update(returncode=result.returncode, stdout=result.stdout, stderr=result.stderr)
        except subprocess.TimeoutExpired as error:
            row.update(timeout_seconds=30, failure=repr(error))
            raise
        finally:
            row['seconds'] = time.monotonic()-started
            (work / 'report.json').write_text(json.dumps(report, indent=2)+'\n')
        if row['returncode']:
            raise RuntimeError('first actual kernel failure: ' + repr(row))
        return row

    try:
        execute([clang, '-O2', '-c', str(work / 'kernels.ll'), '-o', str(work / 'kernels.o')])
        execute([clang, '-O2', '-fno-vectorize', '-fno-slp-vectorize', str(work / 'fixture.c'),
                 str(work / 'kernels.o'), '-o', str(work / 'kernel-check')])
        checked = execute([str(work / 'kernel-check')])
        if checked['stdout'].strip() != 'color-kernels packed=131072 chroma-triples=16777216 ink-cube=65536 passed':
            raise RuntimeError('missing complete scalar control counts')
        execute([clang, '-O2', '-S', str(work / 'kernels.ll'), '-o', str(work / 'kernels.s')])
        report['portable_objects'] = []
        for triple, magic, endian, machine in [
                ('aarch64-unknown-linux-gnu', b'\x7fELF', 'little', 183),
                ('wasm32-unknown-unknown', b'\x00asm', 'little', None),
                ('powerpc64-unknown-linux-gnu', b'\x7fELF', 'big', 21)]:
            output = work / (triple + '.o')
            execute([clang, '-O2', '--target=' + triple, '-c', str(work / 'kernels.ll'), '-o', str(output)])
            header = output.read_bytes()[:64]
            if header[:4] != magic:
                raise RuntimeError('wrong actual portable object header')
            if machine is not None and (header[4] != 2 or header[5] != (1 if endian == 'little' else 2) or int.from_bytes(header[18:20], endian) != machine):
                raise RuntimeError('wrong target/endian object identity')
            if machine is None and header[4:8] != b'\x01\x00\x00\x00':
                raise RuntimeError('wrong actual wasm object version')
            report['portable_objects'].append({'target': triple, 'sha256': digest(output),
                                               'header_verified': True, 'execution_claim': False})
        report.update(status='passed', packed_inputs=131072, complete_byte_triples=16777216,
                      ink_cube_inputs=65536, scalar_exact_comparisons=51052544,
                      optimized_assembly_sha256=digest(work / 'kernels.s'))
    except BaseException as error:
        report.update(status='failed-or-resource-paused', failure=repr(error), failed_artifacts_retained_plain=True)
        raise
    finally:
        report.update(final_free_bytes=shutil.disk_usage(work).free,
                      output_bytes=sum(path.stat().st_size for path in work.rglob('*') if path.is_file()))
        (work / 'report.json').write_text(json.dumps(report, indent=2)+'\n')
    print(report['status'], report['scalar_exact_comparisons'], 'scalar comparisons and3 portable objects')


if __name__ == '__main__':
    main()
