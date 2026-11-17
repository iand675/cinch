#!/usr/bin/env python3
"""Compare sealed pipeline control code without compiling or measuring programs."""
from __future__ import annotations
import argparse
import hashlib
import json
from pathlib import Path
import re
from check_compare import digest
from inspect_multipart_bench import assembly_body, definitions


#: Public pipeline workers and clock/dispatch functions relevant to both controls.
CONTROLS = (
    'Main.measure', 'Main.repeat', 'Main.operation', 'Main.body', 'Main.parts',
    'Main.checksum', 'Effect.Multipart.read-raw@R#cc',
    'Effect.Multipart.multipart-part-read@R#cc',
    'Effect.Multipart.multipart-output-write@R#cc',
    'Effect.Multipart.multipart-write-field@R',
    'Effect.Multipart.multipart-create-field@R',
    'Text.Multipart.complete-delimiter', 'Text.Multipart.find-byte',
    'Text.Multipart.multipart-posix-filename',
)


def normalized_ir(body: str) -> str:
    """Remove only emitted metadata and attribute reference numbering."""
    return re.sub(r'#\d+', '#ATTR', re.sub(r'!\d+', '!MD', body))


def normalized_assembly(body: str) -> str:
    """Remove comments and function-local label IDs while retaining instructions."""
    body = re.sub(r'\s+#(?:\s|$).*', '', body)
    body = re.sub(r'\.LBB\d+_(\d+)', r'.LBB_\1', body)
    body = re.sub(r'\.LCPI\d+_(\d+)', r'.LCPI_\1', body)
    return '\n'.join(line.strip() for line in body.splitlines()
                     if line.strip() and not line.strip().startswith('#'))


def text_digest(value: str) -> str:
    """Hash the exact normalized comparison text independently of input files."""
    return hashlib.sha256(value.encode()).hexdigest()


def sealed_inputs(folder: Path) -> dict:
    """Verify the complete retained source/code identities against their own receipt."""
    report = json.loads((folder / 'report.json').read_text())
    files = {'bench': 'executable_sha256', 'bench.ll': 'llvm_sha256',
             'bench.s': 'assembly_sha256', 'reference': 'reference_executable_sha256',
             'reference.go': 'reference_source_sha256', 'endpoint.c': 'endpoint_source_sha256',
             'endpoint.o': 'endpoint_object_sha256'}
    for name, key in files.items():
        if digest(folder / name) != report[key]:
            raise RuntimeError('changed sealed input: ' + str(folder / name))
    for name, expected in report['source_sha256'].items():
        if digest(folder / (name + '.cinch')) != expected:
            raise RuntimeError('changed sealed source: ' + name)
    return report


def main() -> None:
    """Retain exact code identities, existing counters and unfiltered control samples."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--baseline', type=Path, required=True)
    parser.add_argument('--candidate', type=Path, required=True)
    parser.add_argument('--allocations', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    folders = [args.baseline.resolve(), args.candidate.resolve()]
    reports = [sealed_inputs(folder) for folder in folders]
    for key in ('compiler_sha256', 'runtime_sha256', 'driver_sha256', 'endpoint_object_sha256'):
        if reports[0][key] != reports[1][key]:
            raise RuntimeError('unmatched control identity: ' + key)
    irs = [definitions((folder / 'bench.ll').read_text()) for folder in folders]
    assemblies = [(folder / 'bench.s').read_text() for folder in folders]
    common = sorted(set(irs[0]) & set(irs[1]))
    changed = [name for name in common
               if normalized_ir(irs[0][name]) != normalized_ir(irs[1][name])]
    rows = []
    for name in CONTROLS:
        ir = [normalized_ir(bodies[name]) for bodies in irs]
        actual = [assembly_body(assembly, name) for assembly in assemblies]
        machine = [normalized_assembly(body) for body in actual]
        rows.append({'symbol': name, 'normalized_llvm_equal': ir[0] == ir[1],
                     'normalized_assembly_equal': machine[0] == machine[1],
                     'normalized_llvm_sha256': list(map(text_digest, ir)),
                     'normalized_assembly_sha256': list(map(text_digest, machine)),
                     'stack_subtractions_bytes': [re.findall(r'subq\s+\$(\d+), %rsp', body)
                                                  for body in actual],
                     'static_call_instruction_count': [len(re.findall(r'\bcallq\b', body))
                                                       for body in actual]})
    allocation = json.loads(args.allocations.read_text())
    output = {'scope': __doc__, 'runner_sha256': digest(Path(__file__)),
              'inputs': [{'folder': str(folder), 'report_sha256': digest(folder / 'report.json'),
                          'executable_sha256': report['executable_sha256'],
                          'llvm_sha256': report['llvm_sha256'], 'assembly_sha256': report['assembly_sha256'],
                          'source_sha256': report['source_sha256']}
                         for folder, report in zip(folders, reports)],
              'common_llvm_functions': len(common), 'changed_common_llvm_functions': changed,
              'normalization': 'LLVM metadata/attribute reference IDs only; assembly comments and local BB/CPI function IDs only. Registers, instructions, constants, symbols and CFI retained.',
              'control_functions': rows,
              'allocation_report': str(args.allocations.resolve()),
              'allocation_report_sha256': digest(args.allocations),
              'existing_counter_controls': [row for row in allocation['summary']
                                             if row['mode'] in ('read', 'write-body')],
              'existing_timing_controls': [row for row in reports[1]['results']
                                           if row['mode'] in ('read', 'write-body')],
              'limits': 'Instruction identity does not prove equal elapsed cost across independently laid-out executables. The shared dispatcher differs; its frame increase is observed, not a demonstrated cause. No program was executed.'}
    args.output.write_text(json.dumps(output, indent=2) + '\n')
    print(len(common), 'common functions;', len(changed), 'changed;', len(rows), 'controls inspected')


if __name__ == '__main__':
    main()
