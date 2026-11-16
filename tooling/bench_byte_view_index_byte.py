#!/usr/bin/env python3
"""Compare current one-byte pattern construction and exact-byte queries in the same public artifact."""
from __future__ import annotations
import argparse
import hashlib
import json
import os
from pathlib import Path
import statistics

from bench_stdlib import source_closure
from validate_byte_view import digest,run
from validate_byte_view_index_byte import PROTO

ROOT=Path(__file__).resolve().parents[1]


def expected(operation: str, name: str, size: int, iterations: int) -> tuple[int,str,str,str]:
    """Model every changed input, delimiter and nonoverlapping query result independently."""
    raw=bytearray(b'a'*size)
    if name=='dense':
        for at in range(size):
            if at%64==31:raw[at]=44
            elif at%64==63:raw[at]=59
    elif name=='sparse' and size:
        raw[-1]=44
    original=hashlib.sha256(raw).hexdigest()
    mask=(1<<64)-1
    def mix(total:int,value:int)->int:
        return ((total*65599)^(value&mask))&mask
    total=0
    key=44
    for index in range(iterations):
        key=44 if index%2==0 else 59
        if size:raw[0]=121 if index%2==0 else 122
        if operation=='first':
            value=mix(0,raw.find(bytes([key])))
        else:
            value,position=0,0
            while True:
                at=raw.find(bytes([key]),position)
                if at<0:
                    value=mix(value,size-position)
                    break
                value=mix(value,at-position)
                position=at+1
        total=(total+mix(value,key))&mask
    return total,original,hashlib.sha256(raw).hexdigest(),bytes([key]).hex()


def main()->None:
    """Separate functional preparation from granted same-binary paired sampling and retain exact epochs."""
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--compiler',type=Path,required=True)
    parser.add_argument('--runtime-archive',type=Path,required=True)
    parser.add_argument('--native-interfaces',type=Path)
    parser.add_argument('--native-objects',type=Path)
    parser.add_argument('--module-source',type=Path,default=PROTO)
    parser.add_argument('--artifacts',type=Path,required=True)
    parser.add_argument('--output',type=Path,required=True)
    parser.add_argument('--build-only',action='store_true')
    parser.add_argument('--measure-only',action='store_true')
    parser.add_argument('--build-receipt',type=Path)
    parser.add_argument('--preflight-only',action='store_true')
    parser.add_argument('--samples',type=int,default=5)
    parser.add_argument('--cpu',type=int,required=True)
    args=parser.parse_args()
    driver=(ROOT/'benchmarks/stdlib/byte-view-index-byte-driver.cinch').read_text()
    args.artifacts.mkdir(parents=True,exist_ok=True)
    entry=args.artifacts/'Main.cinch'
    dependencies=source_closure(driver,{'Data.Bytes.View':args.module_source})
    exe=(args.artifacts/'consumer').resolve()
    report={'compiler_sha256':digest(args.compiler),'runtime_sha256':digest(args.runtime_archive),
            'driver_sha256':hashlib.sha256(driver.encode()).hexdigest(),'view_sha256':digest(args.module_source),
            'dependencies':{name:digest(p) for name,p in dependencies.items()},'same_artifact_both_methods':True,
            'scope':'changed current one-byte delimiter, public first-query or complete nonoverlapping-query pipeline; all per-query allocations and normal cleanup included, setup/digests/retained owners outside clocks',
            'compiler_stack_bytes':67108864,'program_stack_bytes':8388608,'cpu':args.cpu,'results':[]}
    if args.native_interfaces:
        assert args.native_objects
        report['interfaces_sha256']=digest(args.native_interfaces);report['objects_sha256']=digest(args.native_objects)
    if args.measure_only:
        assert args.build_receipt
        previous=json.loads(args.build_receipt.read_text())
        for key in ['compiler_sha256','runtime_sha256','driver_sha256','view_sha256','dependencies']:
            assert previous[key]==report[key],key
        assert entry.read_text()==driver
        for name,path in dependencies.items():assert (args.artifacts/(name+'.cinch')).read_bytes()==path.read_bytes(),name
        assert previous['executable_sha256']==digest(exe)
        assert previous['llvm_sha256']==digest(exe.with_suffix('.ll'))
        if args.native_interfaces:
            assert previous['interfaces_sha256']==report['interfaces_sha256']
            assert previous['objects_sha256']==report['objects_sha256']
    else:
        entry.write_text(driver)
        for name,path in dependencies.items():(args.artifacts/(name+'.cinch')).write_bytes(path.read_bytes())
        if args.native_interfaces:
            bundle=args.artifacts/'Bundle.cinch'
            bundle.write_text('\n'.join(p.read_text() for p in reversed(list(dependencies.values())))+'\n'+driver)
            try:
                run([str(args.compiler.resolve()),'--emit-ir','--rc-debug','--interfaces',str(args.native_interfaces.resolve()),
                     '--objects',str(args.native_objects.resolve()),str(bundle.resolve()),str(exe.with_suffix('.ll'))])
            except RuntimeError as error:
                raise RuntimeError(str(error)+'\n'+exe.with_suffix('.ll.diagnostics').read_text()) from error
            configuration=json.loads(args.native_objects.read_text())
            assert digest(Path(configuration['object']))==configuration['object_sha256']
            run([os.environ.get('CINCH_CLANG','clang'),'-O2',str(exe.with_suffix('.ll')),configuration['object'],
                 str(args.runtime_archive.resolve()),'-o',str(exe),'-ldl','-lpthread','-lm'])
        else:
            run([str(args.compiler.resolve()),'build',str(entry.resolve()),'-o',str(exe),'--opt-level','2','--rc-debug',
                 '--runtime-archive',str(args.runtime_archive.resolve())])
    report['executable_sha256']=digest(exe);report['llvm_sha256']=digest(exe.with_suffix('.ll'))
    if not args.build_only:
        for operation in ['first','pipeline']:
            for name in ['dense','sparse','missing']:
                sizes=[0,1,7,8,9,15,16,17,23,24,25,31,32,33,128,4096,65536] if args.preflight_only else [128,4096,65536]
                for size in sizes:
                    iterations=(4 if size<=128 else 2) if args.preflight_only else ({128:4000,4096:512,65536:64}[size] if operation=='first' else {128:512,4096:32,65536:4}[size])
                    wanted=expected(operation,name,size,iterations)
                    row={'operation':operation,'pattern':name,'bytes':size,'iterations':iterations,'checksum':wanted[0],
                         'input_sha256':wanted[1],'retained_sha256':wanted[2],'separator_hex':wanted[3],
                         'samples':{'pattern':[],'byte':[]},'excluded_warmup':{'pattern':[],'byte':[]}}
                    for sample in range(0 if args.preflight_only else -1,1 if args.preflight_only else args.samples):
                        for method in (['pattern','byte'] if sample%2==0 else ['byte','pattern']):
                            for debug in ([False,True] if args.preflight_only else [False]):
                                result=run(['taskset','-c',str(args.cpu),str(exe),method,operation,name,str(size),str(iterations)],
                                           env={**os.environ,'CINCH_RC_DEBUG':str(int(debug)),'CINCH_HEAP_LIMIT':'256M' if debug else '32M','CINCH_STACK_SIZE':'8M','CINCH_LEAK_CHECK':'1'})
                                assert 'leaked=0' in result.stderr
                                elapsed,checksum,before,after,key=result.stdout.split()
                                assert (int(checksum),before,after,key)==wanted,(row,result.stdout)
                                row['samples' if sample>=0 else 'excluded_warmup'][method].append(int(elapsed))
                    row['median_ns']={key:statistics.median(values) for key,values in row['samples'].items()}
                    row['pattern_over_byte']=row['median_ns']['pattern']/row['median_ns']['byte']
                    report['results'].append(row)
    args.output.parent.mkdir(parents=True,exist_ok=True)
    args.output.write_text(json.dumps(report,indent=2)+'\n')


if __name__=='__main__':main()
