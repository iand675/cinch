#!/usr/bin/env python3
"""Verify sealed public color benchmark clock boundaries and actual vector/scalar instruction paths."""
from __future__ import annotations
import argparse
import json
from pathlib import Path
import re
from check_compare import digest
from inspect_color_code import assembly_body, definitions


def inspect(folder: Path) -> dict:
    """Validate exact sealed source/code receipts before interpreting clock or numeric bodies."""
    report = json.loads((folder/'report.json').read_text())
    for name,key in [('bench','executable_sha256'),('bench.ll','llvm_sha256'),('bench.s','assembly_sha256'),
                     ('reference','reference_executable_sha256'),('reference.go','reference_source_sha256')]:
        if digest(folder/name)!=report[key]:
            raise RuntimeError('changed benchmark evidence: '+name)
    for name,expected in report['source_closure_sha256'].items():
        if digest(folder/(name+'.cinch'))!=expected:
            raise RuntimeError('changed benchmark source closure: '+name)
    bodies=definitions((folder/'bench.ll').read_text())
    calls=[line.strip() for line in bodies['Main.measure'].splitlines() if 'call ' in line]
    clocks=[i for i,line in enumerate(calls) if 'Effect.Clock.raw-monotonic-ns' in line]
    repeat=next(i for i,line in enumerate(calls) if 'Main.repeat' in line)
    retained=[i for i,line in enumerate(calls) if 'Main.retained' in line]
    model=next(i for i,line in enumerate(calls) if 'Main.option-hash' in line)
    if len(clocks)!=2 or not clocks[0]<repeat<clocks[1]<min(retained) or model<=clocks[1]:
        raise RuntimeError('incorrect exact public clock/owner boundary')
    if any('drop' in line for line in calls[repeat+1:clocks[1]]):
        raise RuntimeError('teardown before end clock')
    assembly=(folder/'bench.s').read_text()
    measure=assembly_body(assembly,'Main.measure')
    machine_calls=[line.strip() for line in measure.splitlines() if 'callq' in line]
    machine_clocks=[i for i,line in enumerate(machine_calls) if 'cinch_clock_monotonic_ns' in line]
    machine_repeat=next(i for i,line in enumerate(machine_calls) if 'Main.repeat' in line)
    machine_retained=[i for i,line in enumerate(machine_calls) if 'Main.retained' in line]
    if len(machine_clocks)!=2 or not machine_clocks[0]<machine_repeat<machine_clocks[1]<min(machine_retained):
        raise RuntimeError('optimized public clock boundary moved')
    if any('drop' in line for line in machine_calls[machine_repeat+1:machine_clocks[1]]):
        raise RuntimeError('optimized teardown before end clock')
    kernels=[]
    for name in ['expand8','premultiply','unpremultiply','distance4']:
        symbol='Image.Color.'+name+'.llvm'
        body=assembly_body(assembly,symbol)
        vector=sorted(set(re.findall(r'\b(?:pmul\w*|padd\w*|psub\w*|psrl\w*|punpck\w*)\b',body)))
        kernels.append({'symbol':symbol,'optimized_body':body,'actual_vector_integer_instructions':vector})
    return {'report_path':str(folder/'report.json'),'report_sha256':digest(folder/'report.json'),
            'compiler_sha256':report['compiler_sha256'],'runtime_sha256':report['runtime_sha256'],
            'driver_sha256':report['driver_sha256'],'reference_source_sha256':report['reference_source_sha256'],
            'scalar_source_derivation':report['scalar'],'backend_auto_vectorization_enabled_for_both':True,
            'llvm_sha256':report['llvm_sha256'],'assembly_sha256':report['assembly_sha256'],
            'all_postclock_input_palette_and_model_observations_after_end':True,
            'llvm_calls':calls,'optimized_measurement_calls':machine_calls,'actual_numeric_kernels':kernels}


def main() -> None:
    """Inspect both matched builds without invoking a timed program or changing measured inputs."""
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--simd',type=Path,required=True)
    parser.add_argument('--scalar',type=Path,required=True)
    parser.add_argument('--output',type=Path,required=True)
    args=parser.parse_args()
    rows=[inspect(folder.resolve()) for folder in [args.simd,args.scalar]]
    for key in ['compiler_sha256','runtime_sha256','driver_sha256','reference_source_sha256']:
        if rows[0][key]!=rows[1][key]:
            raise RuntimeError('unmatched public pipeline identity: '+key)
    if rows[0]['scalar_source_derivation'] or not rows[1]['scalar_source_derivation']:
        raise RuntimeError('wrong paired source selection')
    if not rows[0]['actual_numeric_kernels'][-1]['actual_vector_integer_instructions']:
        raise RuntimeError('missing actual vector palette distance')
    if any(row['actual_vector_integer_instructions'] for row in rows[1]['actual_numeric_kernels']):
        raise RuntimeError('lane-by-lane source was revectorized; update scope before claiming scalar control')
    args.output.write_text(json.dumps({'scope':__doc__,'runner_sha256':digest(Path(__file__)),
        'runs':rows,'limits':'Actual host code only; no elapsed performance or foreign execution claim.'},indent=2)+'\n')
    print('2 matched public clock boundaries; explicit vector and actual scalar numeric kernels verified')


if __name__=='__main__':
    main()
