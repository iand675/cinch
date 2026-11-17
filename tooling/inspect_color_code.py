#!/usr/bin/env python3
"""Link actual public color call paths to the exact independently tested arithmetic bodies."""
from __future__ import annotations
import argparse
import json
from pathlib import Path
import re
from check_compare import digest


def decoded(symbol: str) -> str:
    """Decode LLVM byte escapes while retaining the original qualified symbol identity."""
    return re.sub(r'\\([0-9A-Fa-f]{2})', lambda match: chr(int(match.group(1), 16)), symbol)


def definitions(ir: str) -> dict[str, str]:
    """Read complete actual definitions, excluding declarations and data globals."""
    return {decoded(match.group(1)): ir[match.start():ir.index('\n}', match.end())+2]
            for match in re.finditer(r'^define [^\n]*@"([^\"]+)"[^\n]*\{\n', ir, re.M)}


def assembly_body(assembly: str, symbol: str) -> str:
    """Read one actual host function through its final body instruction."""
    marker = re.search(r'^(?:"'+re.escape(symbol)+r'"|'+re.escape(symbol)+r'):[^\n]*\n', assembly, re.M)
    if marker is None:
        raise RuntimeError('missing actual optimized body: '+symbol)
    return assembly[marker.start():assembly.index('.Lfunc_end', marker.end())]


def path_to(bodies: dict[str, str], wanted: str) -> list[str] | None:
    """Follow only direct callees, excluding data constants or callback arguments to runtime calls."""
    pending = [['Main.main']]
    visited = set()
    while pending:
        path = pending.pop(0)
        name = path[-1]
        if name == wanted:
            return path
        if name in visited:
            continue
        visited.add(name)
        body = bodies.get(name, '')
        callees = [decoded(match) for match in re.findall(r'\bcall\s+[^@\n]*@"([^"]+)"\(', body)]
        pending.extend(path+[callee] for callee in callees if callee in bodies)
    return None


def main() -> None:
    """Require exact source/runtime/kernel receipts before describing actual optimized instructions."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--source', type=Path, required=True)
    parser.add_argument('--runtime-work', type=Path, required=True)
    parser.add_argument('--kernel-work', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    runtime_path = args.runtime_work / 'report.json'
    kernel_path = args.kernel_work / 'report.json'
    runtime = json.loads(runtime_path.read_text())
    kernel = json.loads(kernel_path.read_text())
    source_hash = digest(args.source)
    if runtime['status'] != 'passed' or kernel['status'] != 'passed' or runtime['source_sha256'] != source_hash or kernel['source_sha256'] != source_hash:
        raise RuntimeError('missing exact qualified color source/actual-kernel pair')
    ir = args.runtime_work / 'consumer.ll'
    assembly = args.kernel_work / 'kernels.s'
    if digest(ir) != runtime['ir_sha256'] or digest(assembly) != kernel['optimized_assembly_sha256']:
        raise RuntimeError('changed actual code evidence')
    bodies = definitions(ir.read_text())
    rows = []
    for name, body in re.findall(r'^  prim ([\w-]+) :: [^\n]+ where\s+llvm \{(.*?)\n    \}', args.source.read_text(), re.M | re.S):
        symbol = 'Image.Color.'+name+'.llvm'
        emitted = bodies[symbol].split('\n', 1)[1].removesuffix('\n}')
        expected_lines = [line.strip() for line in body.splitlines() if line.strip()]
        actual_lines = [line.strip() for line in emitted.splitlines() if line.strip() and line.strip() != 'entry:']
        if expected_lines != actual_lines:
            raise RuntimeError('emitted body differs from tested source: '+name)
        path = path_to(bodies, symbol)
        if path is None:
            raise RuntimeError('no actual direct public path to selected kernel: '+name)
        machine = assembly_body(assembly.read_text(), 'color_'+name.replace('-', '_'))
        rows.append({'kernel': name, 'actual_symbol': symbol, 'public_entrypoint_path': path,
                     'emitted_body_equals_independently_tested_source': True,
                     'optimized_host_body': machine,
                     'vector_integer_instructions': sorted(set(re.findall(r'\b(?:pmul\w*|padd\w*|psub\w*|psrl\w*|punpck\w*)\b', machine)))})
    report = {'scope': __doc__, 'source_sha256': source_hash,
              'runtime_report_sha256': digest(runtime_path), 'kernel_report_sha256': digest(kernel_path),
              'actual_public_ir_sha256': digest(ir), 'optimized_kernel_assembly_sha256': digest(assembly),
              'runner_sha256': digest(Path(__file__)), 'kernels': rows,
              'limits': 'Actual vector instructions and complete public call reachability are demonstrated; elapsed performance and foreign execution are not.'}
    args.output.write_text(json.dumps(report, indent=2)+'\n')
    print(len(rows), 'exact public-to-kernel paths and actual optimized instruction bodies verified')


if __name__ == '__main__':
    main()
