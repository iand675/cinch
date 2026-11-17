#!/usr/bin/env python3
"""Inspect actual accepted fixed-palette call paths and machine instructions without building or running code."""
from __future__ import annotations
import argparse
import json
from pathlib import Path
import re
import subprocess
from check_compare import digest
from inspect_color_code import definitions, path_to


def main() -> None:
    """Bind factory/nearest/owner reachability to exact qualified source, IR and existing ELF bytes."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--accepted-report', type=Path, required=True)
    parser.add_argument('--color-source', type=Path, required=True)
    parser.add_argument('--kernel-receipt', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    accepted = json.loads(args.accepted_report.read_text())
    if accepted['status'] != 'passed':
        raise RuntimeError('successful full runtime receipt required')
    work = args.accepted_report.parent
    for name, key in [('consumer.ll', 'ir_sha256'), ('consumer', 'executable_sha256')]:
        if digest(work / name) != accepted[key]:
            raise RuntimeError('changed accepted code: '+name)
    for name, checksum in accepted['source_closure_sha256'].items():
        if digest(work / (name+'.cinch')) != checksum:
            raise RuntimeError('changed accepted source: '+name)
    if digest(args.color_source) != accepted['color_sha256']:
        raise RuntimeError('unmatched original color kernel source')
    kernel = json.loads(args.kernel_receipt.read_text())
    if kernel['status'] != 'passed' or kernel['source_sha256'] != accepted['color_sha256']:
        raise RuntimeError('unmatched independent actual-body scalar proof')
    bodies = definitions((work / 'consumer.ll').read_text())
    paths = {}
    for name in ['Image.Color.Palette.plan9-colors', 'Image.Color.Palette.web-safe-colors',
                 'Image.Color.nearest#cc', 'Image.Color.distance4.llvm', 'Image.Color.expand8.llvm', 'Main.owners-match']:
        path = path_to(bodies, name)
        if path is None:
            raise RuntimeError('missing actual entrypoint path: '+name)
        paths[name] = path
    source = args.color_source.read_text()
    exact = []
    for name in ['distance4', 'expand8']:
        original = re.search(r'  prim '+name+r' :: [^\n]+ where\s+llvm \{(.*?)\n    \}', source, re.S).group(1)
        emitted = bodies['Image.Color.'+name+'.llvm'].split('\n', 1)[1].removesuffix('\n}')
        expected_lines = [line.strip() for line in original.splitlines() if line.strip()]
        actual_lines = [line.strip() for line in emitted.splitlines() if line.strip() and line.strip() != 'entry:']
        if expected_lines != actual_lines:
            raise RuntimeError('changed independently tested body: '+name)
        exact.append(name)
    machine = []
    for symbol in ['Image.Color.Palette.plan9-colors', 'Image.Color.Palette.web-safe-colors', 'Image.Color.nearest#cc']:
        command = ['objdump', '-d', '--no-show-raw-insn', '--disassemble='+symbol, str((work / 'consumer').resolve())]
        result = subprocess.run(command, text=True, capture_output=True, check=True, timeout=10)
        if '<'+symbol+'>:' not in result.stdout:
            raise RuntimeError('missing actual machine symbol: '+symbol)
        vectors = sorted(set(re.findall(r'\b(?:pmul\w*|padd\w*|psub\w*|psrl\w*|punpck\w*)\b', result.stdout)))
        if symbol.endswith('nearest#cc') and not vectors:
            raise RuntimeError('actual nearest body has no expected integer vector instructions')
        machine.append({'symbol': symbol, 'command': command, 'stdout': result.stdout, 'stderr': result.stderr,
                        'vector_integer_instructions': vectors})
    report = {'scope': __doc__, 'runner_sha256': digest(Path(__file__)),
              'accepted_report_sha256': digest(args.accepted_report), 'source_closure_sha256': accepted['source_closure_sha256'],
              'actual_ir_sha256': accepted['ir_sha256'], 'actual_executable_sha256': accepted['executable_sha256'],
              'actual_public_paths': paths, 'kernel_bodies_equal_independently_tested_source': exact,
              'independent_scalar_kernel_receipt': str(args.kernel_receipt),
              'independent_scalar_kernel_receipt_sha256': digest(args.kernel_receipt), 'actual_host_disassembly': machine,
              'limits': 'Public Unit factory wrappers are inlined into the actual entry; the reachable table constructors are checked. No new compilation, runtime, elapsed claim or foreign execution occurs. Complete target object composition remains separate.'}
    args.output.write_text(json.dumps(report, indent=2)+'\n')
    print('both actual factories, nearest SIMD and escaped-owner paths verified in accepted code')


if __name__ == '__main__':
    main()
