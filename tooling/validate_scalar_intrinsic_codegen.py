#!/usr/bin/env python3
"""Retain paired scalar-loop LLVM, assembly, and independently checked runtime observations."""
from __future__ import annotations

import argparse
import json
import os
from pathlib import Path
import re
import subprocess

import bootstrap
import bootstrap_objects


def checksum(count: int) -> int:
    """Evaluate the recurrence as mathematical arithmetic reduced to an unsigned finite word."""
    word = 1
    for _ in range(count):
        added = (word * 6364136223846793005 + 1442695040888963407) % (1 << 64)
        word = added ^ (added >> 29)
    return word


def decode_globals(text: str) -> str:
    """Make LLVM's escaped object names readable without altering any compiled source or metadata."""
    return re.sub(r'\\([0-9A-Fa-f]{2})', lambda match: chr(int(match[1], 16)), text)


def main() -> None:
    """Build paired clients with identical source/provider and require scalar semantics before any timing claim."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--baseline', type=Path, required=True)
    parser.add_argument('--compiler', type=Path, required=True)
    parser.add_argument('--library-artifact', type=Path, required=True)
    parser.add_argument('--artifacts', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    args.artifacts = args.artifacts.resolve()
    args.artifacts.mkdir(parents=True, exist_ok=False)
    provider = bootstrap_objects.artifact_provider(bootstrap, args.library_artifact)
    manifest, objects = provider['interface_package'], args.artifacts / 'objects.json'
    bootstrap.write_json(objects, provider['configuration'])
    source = bootstrap.ROOT / 'compiler/programs/Run.ScalarIntrinsicLoop.cinch'
    fixture = bootstrap.ROOT / 'tooling/scalar_intrinsic_benchmark.c'
    fixture_object = args.artifacts / 'fixture.o'
    subprocess.run(['clang', '-O2', '-c', str(fixture), '-o', str(fixture_object)], check=True)
    report = {'source_sha256': bootstrap.file_hash(source), 'foreign_fixture_sha256': bootstrap.file_hash(fixture),
              'provider_receipt_sha256': provider['identity']['receipt_sha256'], 'object_sha256': bootstrap.file_hash(provider['object']),
              'runtime_sha256': bootstrap.file_hash(provider['runtime']), 'status': 'running', 'results': [],
              'timing_claim': False, 'smoke_iterations': 32, 'expected_checksum': checksum(32)}
    bootstrap.write_json(args.output, report)
    for name, compiler in [('baseline', args.baseline), ('experimental', args.compiler)]:
        compiler = compiler.resolve()
        directory = args.artifacts / name
        directory.mkdir()
        ir, executable = directory / 'consumer.ll', directory / 'consumer'
        command = [str(compiler), '--emit-ir', '--rc-debug', '--interfaces', str(manifest), '--objects', str(objects), str(source), str(ir)]
        compiled = subprocess.run(command, capture_output=True, text=True, env=bootstrap.environment())
        (directory / 'compile.log').write_text(compiled.stdout + compiled.stderr)
        compiled.check_returncode()
        link = ['clang', '-O2', str(ir), str(fixture_object), str(provider['object']), str(provider['runtime']),
                '-o', str(executable), '-ldl', '-lpthread', '-lm']
        linked = subprocess.run(link, capture_output=True, text=True, env=bootstrap.environment())
        (directory / 'link.log').write_text(linked.stdout + linked.stderr)
        linked.check_returncode()
        readable = decode_globals(ir.read_text())
        (directory / 'readable.ll').write_text(readable)
        assembly = '\n'.join(subprocess.check_output(['objdump', '-d', '--disassemble=' + symbol, str(executable)], text=True)
                             for symbol in ['cinch.typed.m.Main.recurrence', 'cinch.typed.m.Main.recurrence#tail'])
        if '<cinch.typed.m.Main.recurrence>:' not in assembly:
            raise bootstrap.BootstrapError('the public recurrence machine-code body was not found')
        (directory / 'recurrence.asm').write_text(assembly)
        row = {'name': name, 'executable': str(executable), 'compiler_sha256': bootstrap.file_hash(compiler), 'llvm_sha256': bootstrap.file_hash(ir),
               'executable_sha256': bootstrap.file_hash(executable), 'assembly_sha256': bootstrap.file_hash(directory / 'recurrence.asm'),
               'compile_command': command, 'link_command': link, 'runs': []}
        report['results'].append(row)
        loop = re.search(r'^define[^\n]*@"cinch\.typed\.m\.Main\.recurrence#tail"[^\n]*\{\n(.*?)^}', readable, re.M | re.S)
        if not loop:
            raise bootstrap.BootstrapError('the recurrence tail worker was not found')
        (directory / 'recurrence.ll').write_text(loop[0])
        row['loop_object_calls'] = re.findall(r'call[^\n]*@"(Data\.Machine\.[^"]+)"', loop[1])
        if name == 'baseline' and not row['loop_object_calls']:
            raise bootstrap.BootstrapError('baseline does not demonstrate the imported-object call cost')
        if name == 'experimental' and row['loop_object_calls']:
            raise bootstrap.BootstrapError('experimental loop still calls imported scalar object wrappers')
        entry = re.search(r'^define[^\n]*@"cinch\.typed\.main"[^\n]*\{\n(.*?)^}', readable, re.M | re.S)
        if not entry:
            raise bootstrap.BootstrapError('the clocked entry worker was not found')
        clocks = [match.start() for match in re.finditer(r'call i64 @"scalar_clock"', entry[1])]
        recurrence = entry[1].find('call i64 @"cinch.typed.m.Main.recurrence"')
        observe = entry[1].find('call i64 @"scalar_report"')
        if len(clocks) != 2 or not clocks[0] < recurrence < clocks[1] < observe:
            raise bootstrap.BootstrapError('LLVM clock boundaries do not enclose the complete recurrence')
        entry_assembly = subprocess.check_output(['objdump', '-d', '--disassemble=cinch.typed.main', str(executable)], text=True)
        (directory / 'entry.asm').write_text(entry_assembly)
        clock_calls = [match.start() for match in re.finditer(r'call[^\n]*<scalar_clock>', entry_assembly)]
        report_call = entry_assembly.find('<scalar_report>')
        if len(clock_calls) != 2 or not clock_calls[1] < report_call:
            raise bootstrap.BootstrapError('machine clock calls or post-clock checksum observation moved')
        interval = entry_assembly[clock_calls[0]:clock_calls[1]]
        if not ('recurrence' in interval or ('imul' in interval and 'shr' in interval and 'xor' in interval)):
            raise bootstrap.BootstrapError('the recurrence machine code is outside its measured interval')
        row['clock_boundary_proof'] = {'llvm_start_before_recurrence_before_end_before_observation': True,
                                       'machine_recurrence_inside_clock_calls': True,
                                       'entry_assembly_sha256': bootstrap.file_hash(directory / 'entry.asm')}
        row['machine_call_instructions'] = [line for line in assembly.splitlines() if re.search(r'\bcall[q]?\b', line)]
        if name == 'experimental' and row['machine_call_instructions']:
            raise bootstrap.BootstrapError('experimental recurrence still contains machine calls')
        for debug in [False, True]:
            ran = subprocess.run([str(executable)], capture_output=True, text=True, timeout=180,
                env={**os.environ, 'CINCH_BENCH_ITERATIONS': '32', 'CINCH_STACK_SIZE': '8M',
                     'CINCH_HEAP_LIMIT': '256M' if debug else '32M', 'CINCH_RC_DEBUG': '1' if debug else '0', 'CINCH_LEAK_CHECK': '1'})
            observation = json.loads(ran.stdout) if ran.stdout else {}
            row['runs'].append({'debug': debug, 'exit': ran.returncode, 'stderr': ran.stderr, 'observation': observation})
            bootstrap.write_json(args.output, report)
            if ran.returncode or 'leaked=0' not in ran.stderr or observation.get('checksum') != report['expected_checksum']:
                raise bootstrap.BootstrapError('paired recurrence semantic smoke failed')
        bootstrap.write_json(args.output, report)
    report['status'] = 'passed'
    bootstrap.write_json(args.output, report)
    print('Paired recurrence semantics and direct scalar machine-code checks passed.', flush=True)


if __name__ == '__main__':
    main()
