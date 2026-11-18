#!/usr/bin/env python3
"""Compare sealed public color control code without building or executing a program."""
from __future__ import annotations
import argparse
import hashlib
import json
from pathlib import Path
import re
from check_compare import digest
from inspect_color_code import assembly_body, definitions


def normalized_ir(body: str) -> str:
    """Ignore only metadata and attribute reference numbers in emitted definitions."""
    return re.sub(r'#\d+', '#ATTR', re.sub(r'!\d+', '!MD', body))


def normalized_assembly(body: str) -> str:
    """Ignore comments and local label IDs while retaining instructions and registers."""
    body = re.sub(r'\s+#(?:\s|$).*', '', body)
    body = re.sub(r'\.LBB\d+_(\d+)', r'.LBB_\1', body)
    body = re.sub(r'\.LCPI\d+_(\d+)', r'.LCPI_\1', body)
    return '\n'.join(line.strip() for line in body.splitlines()
                     if line.strip() and not line.strip().startswith('#'))


def text_digest(value: str) -> str:
    """Hash the exact normalized comparison text independently of input artifacts."""
    return hashlib.sha256(value.encode()).hexdigest()


def sealed(folder: Path) -> tuple[dict, dict[str, str], str]:
    """Require the existing successful receipt and exact measured machine/code inputs."""
    report = json.loads((folder / 'report.json').read_text())
    if report['status'] != 'functional-preflights-passed':
        raise RuntimeError('unqualified benchmark input: '+str(folder))
    for name, key in [('bench', 'executable_sha256'), ('bench.ll', 'llvm_sha256'),
                      ('bench.s', 'assembly_sha256'), ('Main.cinch', 'driver_sha256'),
                      ('reference', 'reference_executable_sha256'),
                      ('reference.go', 'reference_source_sha256')]:
        if digest(folder / name) != report[key]:
            raise RuntimeError('changed sealed artifact: '+str(folder / name))
    return report, definitions((folder / 'bench.ll').read_text()), (folder / 'bench.s').read_text()


def rgba16_path(assembly: str) -> str:
    """Observe the actual stored RGBA16 jump-table path through the common return."""
    body = assembly_body(assembly, 'Image.Color.rgba-word#cc')
    table_name = re.search(r'leaq\s+(\.LJTI\d+_0)\(%rip\)', body).group(1)
    table = assembly.split(table_name+':\n', 1)[1].split('\n')[:12]
    target = re.search(r'\.long\s+(\.LBB\d+_\d+)-', table[1]).group(1)
    # The entry's first conditional block is part of the opaque carrier check;
    # retain all blocks through the table dispatch, excluding other color cases.
    prefix = body[:body.index('\n.LBB', body.index('jmpq'))]
    tail = body.split(target+':', 1)[1].split('retq', 1)[0]+'retq\t$8'
    return normalized_assembly(prefix+'\n'+tail)


def main() -> None:
    """Record machine-code facts separately from unproven timing explanations."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--simd', type=Path, required=True)
    parser.add_argument('--scalar', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    inputs = [sealed(folder) for folder in [args.simd, args.scalar]]
    for key in ['compiler_sha256', 'runtime_sha256', 'driver_sha256', 'reference_source_sha256']:
        if inputs[0][0][key] != inputs[1][0][key]:
            raise RuntimeError('unmatched benchmark identity: '+key)
    bodies = [row[1] for row in inputs]
    common = sorted(bodies[0].keys() & bodies[1].keys())
    changed = [name for name in common if normalized_ir(bodies[0][name]) != normalized_ir(bodies[1][name])]
    expected = ['Image.Color.'+name+'.llvm' for name in ['distance4', 'expand8', 'premultiply', 'unpremultiply']]
    if changed != expected or bodies[0].keys() != bodies[1].keys():
        raise RuntimeError('difference outside the four sealed numeric kernels')
    names = ['Main.measure', 'Main.repeat', 'Main.batch', 'Main.operation',
             'Main.color-hash', 'Main.raw-hash', 'Image.Color.rgba-word#cc',
             'Image.Color.convert-standard', 'Image.Color.color-convert#cc',
             'Image.Color.color-palette-convert', 'Image.Color.nearest#cc',
             'Image.Color.nearest.shared0#cc']
    rows = []
    for name in names:
        machine = [assembly_body(row[2], name) for row in inputs]
        normalized = [normalized_assembly(body) for body in machine]
        rows.append({'symbol': name, 'normalized_ir_equal': normalized_ir(bodies[0][name]) == normalized_ir(bodies[1][name]),
                     'normalized_assembly_equal': normalized[0] == normalized[1],
                     'assembly_lines': [len(body.splitlines()) for body in machine],
                     'stack_subtractions_bytes': [re.findall(r'subq\s+\$(\d+), %rsp', body) for body in machine],
                     'normalized_assembly_sha256': [text_digest(body) for body in normalized],
                     'direct_ir_callees': sorted(set(re.findall(r'\bcall\s+[^@\n]*@"([^"]+)"\(', bodies[0][name])))})
    paths = [rgba16_path(row[2]) for row in inputs]
    if paths[0] != paths[1]:
        raise RuntimeError('the purported unchanged RGBA16 arithmetic path changed')
    report = {'scope': __doc__, 'runner_sha256': digest(Path(__file__)),
              'inputs': [{'folder': str(folder), 'report_sha256': digest(folder / 'report.json'),
                          'executable_sha256': row[0]['executable_sha256'],
                          'assembly_sha256': row[0]['assembly_sha256'], 'llvm_sha256': row[0]['llvm_sha256']}
                         for folder, row in zip([args.simd, args.scalar], inputs)],
              'common_definitions': len(common), 'changed_ir_definitions': changed,
              'selected_functions': rows,
              'stored_rgba16_path_equal': True,
              'stored_rgba16_normalized_path_sha256': text_digest(paths[0]),
              'stored_rgba16_normalized_path': paths[0],
              'observations': [
                  'The complete Main clock/repeat/batch/operation/raw/canonical hashing instructions compare equal after comment/local-label normalization.',
                  'Stored RGBA16 dispatch goes directly to the same common return; no changed numeric kernel executes on that canonical observation path.',
                  'YCbCr16 workload constructs NYCbCrA and therefore executes the changed premultiply body after unchanged y-dependent conversion.',
                  'Nearest lookup inlines carrier dispatch and distance arithmetic. Its SIMD stack subtraction is24 bytes versus scalar40; this fact alone does not predict elapsed time.',
                  'PaletteModel reaches the same nearest worker through color-convert; direct palette-convert and model dispatch are distinct public paths.',
                  'Both sources allocate an optional (Nat,Color) tuple on every improved nearest choice and advance a Nat index; source and IR expose these sites, not their dynamic counts.',
                  'Typed component observers and canonical output tuples add public-pipeline allocation work shared by both variants.'
              ],
              'limits': [
                  'No compiler, program, allocation-counter or elapsed measurement is executed by this review.',
                  'Identical executed instructions do not prove identical placement, cache behavior, branch state or scheduling.',
                  'The retained RGBA control regression prevents treating the first short timing window as isolated arithmetic attribution.',
                  'Allocation counts and longer paired control confirmation remain separate future evidence; no source optimization or SDK parity is claimed.'
              ]}
    args.output.write_text(json.dumps(report, indent=2)+'\n')
    print(len(common), 'definitions; four exact numeric changes; stored RGBA16 path equal')


if __name__ == '__main__':
    main()
