#!/usr/bin/env python3
"""Inspect actual optimized multipart code and the public benchmark clock boundary."""
from __future__ import annotations
import argparse
import json
from pathlib import Path
import re
from check_compare import digest


def decoded(symbol: str) -> str:
    """Decode LLVM byte escapes without rewriting source-level symbol identities."""
    return re.sub(r'\\([0-9A-Fa-f]{2})', lambda match: chr(int(match.group(1), 16)), symbol)


def definitions(ir: str) -> dict[str, str]:
    """Index complete emitted bodies while ignoring metadata and external declarations."""
    return {decoded(match.group(1)): ir[match.start():ir.index('\n}', match.end()) + 2]
            for match in re.finditer(r'^define [^\n]*@"([^\"]+)"[^\n]*\{\n', ir, re.M)}


def assembly_body(assembly: str, symbol: str) -> str:
    """Read one actual target assembly function, accepting quoted or plain labels."""
    marker = re.search(r'^(?:"' + re.escape(symbol) + r'"|' + re.escape(symbol) + r'):[^\n]*\n', assembly, re.M)
    if marker is None:
        raise RuntimeError('missing optimized assembly body: ' + symbol)
    return assembly[marker.start():assembly.index('.Lfunc_end', marker.end())]


def inspect(folder: Path) -> dict:
    """Require exact frozen code and post-clock input/context observations in LLVM and assembly."""
    report = json.loads((folder / 'report.json').read_text())
    ir, assembly = folder / 'bench.ll', folder / 'bench.s'
    if digest(ir) != report['llvm_sha256'] or digest(assembly) != report['assembly_sha256']:
        raise RuntimeError('changed benchmark code')
    bodies = definitions(ir.read_text())
    symbol = next(name for name in bodies if name.endswith('Main.measure'))
    body = decoded(bodies[symbol])
    calls = [line.strip() for line in body.splitlines() if 'call ' in line]
    clocks = [i for i, line in enumerate(calls) if 'clock-monotonic-ns' in line]
    repeat = next(i for i, line in enumerate(calls) if 'Main.repeat' in line)
    checksum = next(i for i, line in enumerate(calls) if 'Main.checksum' in line)
    retained = next(i for i, line in enumerate(calls) if 'Main.retained' in line)
    if not (len(clocks) == 2 and clocks[0] < repeat < clocks[1] < checksum < retained):
        raise RuntimeError('incorrect retained-input/context clock boundary')
    if any('drop' in line for line in calls[repeat + 1:clocks[1]]):
        raise RuntimeError('teardown before end clock')
    asm = assembly_body(assembly.read_text(), symbol)
    actual_calls = [line.strip() for line in asm.splitlines() if 'call' in line]
    asm_clocks = [i for i, line in enumerate(actual_calls) if 'clock' in line and 'monotonic' in line]
    asm_repeat = next(i for i, line in enumerate(actual_calls) if 'Main.repeat' in line)
    asm_checksum = next(i for i, line in enumerate(actual_calls) if 'Main.checksum' in line)
    if not (len(asm_clocks) == 2 and asm_clocks[0] < asm_repeat < asm_clocks[1] < asm_checksum):
        raise RuntimeError('optimized assembly moved retained traversal into interval')
    if any('drop' in line for line in actual_calls[asm_repeat + 1:asm_clocks[1]]):
        raise RuntimeError('optimized assembly performs teardown before end clock')
    matcher = bodies.get('Text.Multipart.equal-sixteen.llvm')
    vector = None
    if matcher is not None:
        machine_symbol = 'Text.Multipart.equal-sixteen.llvm'
        try:
            machine = assembly_body(assembly.read_text(), machine_symbol)
        except RuntimeError:
            # Native's always-inline matcher legitimately disappears as a named
            # machine function; require its real instructions in the escape loop.
            machine_symbol = next(name for name in bodies if name.endswith('Text$2eMultipart.escape$2dend'))
            machine = assembly_body(assembly.read_text(), machine_symbol)
        if 'icmp eq <16 x i8>' not in matcher or 'pcmpeqb' not in machine or 'pmovmskb' not in machine:
            raise RuntimeError('missing actual x86 vector matching instructions')
        vector = {'llvm_body': matcher, 'optimized_assembly_symbol': machine_symbol, 'optimized_assembly': machine}
    if not report['scalar_snapshot'] and vector is None:
        raise RuntimeError('SIMD snapshot lost its actual matcher')
    return {'benchmark_report': str(folder / 'report.json'), 'benchmark_report_sha256': digest(folder / 'report.json'),
            'compiler_sha256': report['compiler_sha256'], 'llvm_sha256': digest(ir), 'assembly_sha256': digest(assembly),
            'scalar_snapshot': report['scalar_snapshot'], 'measurement_symbol': symbol,
            'llvm_calls': calls, 'optimized_assembly_calls': actual_calls,
            'end_clock_before_retained_input_context_and_last_teardown': True, 'actual_matcher': vector,
            'limits': 'Only bounded complete windows use matching SIMD; scalar fringes and public parser/allocation/callback costs remain. Scalar snapshot changes byte/escape/prefix classification while retaining bulk-copy and runtime policies.'}


def main() -> None:
    """Retain reproducible actual-code receipts without compiling or timing another program."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--bench', type=Path, nargs='+', required=True)
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    rows = [inspect(folder.resolve()) for folder in args.bench]
    args.output.write_text(json.dumps({'scope': __doc__, 'runs': rows}, indent=2) + '\n')
    print(len(rows), 'actual LLVM/optimized assembly boundaries passed')


if __name__ == '__main__':
    main()
