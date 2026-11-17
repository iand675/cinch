#!/usr/bin/env python3
"""Build sealed complete public color pipelines and check exact independent output/owner hashes."""
from __future__ import annotations
import argparse
import json
import os
from pathlib import Path
import re
import resource
import shutil
import subprocess
import time
from bench_stdlib import source_closure
from check_compare import digest


#: Repository-owned source, driver and reference inputs; no production file is changed.
ROOT = Path(__file__).resolve().parents[1]


def scalar_source(source: str) -> str:
    """Derive only four vector kernels as lane-by-lane operations with identical rounding/wrapping."""
    replacements = {}

    def lane(word: str, index: int, prefix: str) -> list[str]:
        """Extract one bounded logical channel without memory or vector operations."""
        return [f'%{prefix}shift = lshr i64 %{word}, {16*index}',
                f'%{prefix}short = trunc i64 %{prefix}shift to i16',
                f'%{prefix} = zext i16 %{prefix}short to i32']

    def packed(names: list[str]) -> list[str]:
        """Pack narrowed mathematical results while retaining modulo-sixteen-bit behavior."""
        body = []
        for index, name in enumerate(names):
            body += [f'%outshort{index} = trunc i32 %{name} to i16',
                     f'%outwide{index} = zext i16 %outshort{index} to i64',
                     f'%outlane{index} = shl i64 %outwide{index}, {16*index}']
        body += ['%sum1 = or i64 %outlane0, %outlane1',
                 '%sum2 = or i64 %sum1, %outlane2', '%result = or i64 %sum2, %outlane3']
        return body

    body = []
    for index in range(4):
        body += [f'%shift{index} = lshr i64 %0, {8*index}', f'%byte{index} = trunc i64 %shift{index} to i8',
                 f'%wide{index} = zext i8 %byte{index} to i32', f'%c{index} = mul i32 %wide{index}, 257']
    replacements['expand8'] = body + packed(['c0','c1','c2','c3']) + ['ret i64 %result']
    for name in ['premultiply','unpremultiply']:
        body = []
        for index in range(4):
            body += lane('0', index, 'c'+str(index))
        if name == 'unpremultiply':
            body += ['%zero = icmp eq i32 %c3, 0', '%divisor = select i1 %zero, i32 1, i32 %c3']
        for index in range(3):
            body += [f'%product{index} = mul i32 %c{index}, '+('%c3' if name == 'premultiply' else '65535'),
                     f'%scaled{index} = udiv i32 %product{index}, '+('65535' if name == 'premultiply' else '%divisor')]
        body += packed(['scaled0','scaled1','scaled2','c3'])
        body += ['%selected = select i1 %zero, i64 %0, i64 %result', 'ret i64 %selected'] if name == 'unpremultiply' else ['ret i64 %result']
        replacements[name] = body
    body = []
    for index in range(4):
        body += lane('0', index, 'left'+str(index)) + lane('1', index, 'right'+str(index))
        body += [f'%difference{index} = sub i32 %left{index}, %right{index}',
                 f'%square{index} = mul i32 %difference{index}, %difference{index}',
                 f'%scaled{index} = lshr i32 %square{index}, 2']
    replacements['distance4'] = body + ['%sum1 = add i32 %scaled0, %scaled1',
                                      '%sum2 = add i32 %scaled2, %scaled3',
                                      '%result = add i32 %sum1, %sum2', 'ret i32 %result']
    for name, body in replacements.items():
        pattern = r'(  prim '+re.escape(name)+r' :: [^\n]+ where\s+llvm \{).*?(\n    \})'
        source, count = re.subn(pattern, lambda match: match.group(1)+'\n'+'\n'.join('      '+line for line in body)+match.group(2), source, count=1, flags=re.S)
        if count != 1:
            raise RuntimeError('missing scalar derivation basis: '+name)
    return source


def main() -> None:
    """Build or preflight one explicit candidate; elapsed timing requires a separately coordinated tool."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--source', type=Path, required=True)
    parser.add_argument('--compiler', type=Path, required=True)
    parser.add_argument('--runtime', type=Path, required=True)
    parser.add_argument('--go', type=Path, required=True)
    parser.add_argument('--work', type=Path, required=True)
    parser.add_argument('--cpu', type=int, required=True)
    parser.add_argument('--scalar', action='store_true')
    parser.add_argument('--smoke-only', action='store_true')
    parser.add_argument('--sizes', type=int, nargs='+', default=[0,1,16,64])
    args = parser.parse_args()
    work = args.work.resolve()
    work.mkdir(parents=True, exist_ok=args.smoke_only)
    initial = shutil.disk_usage(work).free
    if initial < 350*1024*1024:
        raise RuntimeError('350 MiB initial benchmark functional-unit guard')
    os.sched_setaffinity(0, {args.cpu})
    resource.setrlimit(resource.RLIMIT_STACK, (67108864, resource.getrlimit(resource.RLIMIT_STACK)[1]))
    compiler, runtime, sdk = args.compiler.resolve(), args.runtime.resolve(), args.go.resolve()
    experiment = ROOT / 'experiments/image-color-v1'
    driver = experiment / 'benchmark.cinch'
    reference = experiment / 'benchmark.go'
    host_env = {**os.environ, 'RUST_MIN_STACK': '67108864', 'CINCH_STACK_SIZE': '64M',
                'GOMAXPROCS': '1', 'GOTELEMETRY': 'off', 'GOCACHE': '/tmp/cinch-mime-go-cache'}
    report = json.loads((work/'report.json').read_text()) if args.smoke_only else {
        'scope': __doc__, 'scalar': args.scalar, 'source_basis': str(args.source.resolve()),
        'source_basis_sha256': digest(args.source), 'compiler': str(compiler), 'compiler_sha256': digest(compiler),
        'runtime': str(runtime), 'runtime_sha256': digest(runtime), 'driver_sha256': digest(driver),
        'reference_source_sha256': digest(reference), 'sdk_sha256': digest(sdk),
        'compiler_host_stack_bytes': 67108864, 'runner_sha256': digest(Path(__file__)),
        'initial_free_bytes': initial, 'cpu_affinity': [args.cpu], 'commands': [],
        'resource_policy': {'initial_mib':350,'reserve_mib':250,'unit_mib':30},
        'timing_claim': False, 'allocation_policy': 'Full immutable color/model outputs and source/palette owner traversal; Cinch List/owned values and SDK slices/interfaces/GC retain their normal policies.'}

    def save() -> None:
        """Preserve exact source/command/output facts before any later phase can fail."""
        report.update(final_free_bytes=shutil.disk_usage(work).free,
                      output_bytes=sum(p.stat().st_size for p in work.rglob('*') if p.is_file()))
        (work/'report.json').write_text(json.dumps(report,indent=2)+'\n')

    def execute(command: list[str], env: dict) -> dict:
        """Stop at the first actual source/compiler/reference/output discrepancy and retain it plain."""
        save()
        if report['final_free_bytes'] < 250*1024*1024 or report['output_bytes'] > 30*1024*1024:
            raise RuntimeError('250 MiB reserve /30 MiB output guard')
        row = {'command': command}
        report['commands'].append(row)
        started = time.monotonic()
        try:
            result = subprocess.run(command,cwd=work,text=True,capture_output=True,timeout=45,env=env)
            row.update(returncode=result.returncode,stdout=result.stdout,stderr=result.stderr)
        except subprocess.TimeoutExpired as error:
            row.update(timeout_seconds=45,failure=repr(error))
            raise
        finally:
            row['seconds']=time.monotonic()-started
            save()
        if row['returncode']:
            raise RuntimeError('first benchmark preparation failure: '+repr(row))
        return row

    try:
        if not args.smoke_only:
            text = args.source.read_text()
            (work/'Image.Color.cinch').write_text(scalar_source(text) if args.scalar else text)
            shutil.copyfile(driver,work/'Main.cinch')
            shutil.copyfile(reference,work/'reference.go')
            closure = source_closure(driver.read_text(), {'Image.Color':work/'Image.Color.cinch'})
            for name,path in closure.items():
                destination=work/(name+'.cinch')
                if path.resolve()!=destination.resolve():
                    shutil.copyfile(path,destination)
            report['source_closure_sha256']={name:digest(work/(name+'.cinch')) for name in closure}
            execute([str(compiler),'check',str(work/'Main.cinch')],host_env)
            execute([str(compiler),'emit-ir',str(work/'Main.cinch'),'-o',str(work/'bench.ll')],host_env)
            triple=re.search(r'^target triple = "([^"]+)"',(work/'bench.ll').read_text(),re.M)
            if triple is None:
                raise RuntimeError('missing actual benchmark target')
            clang=os.environ.get('CINCH_CLANG','clang')
            execute([clang,'-O2','--target='+triple.group(1),str(work/'bench.ll'),str(runtime),'-ldl','-lpthread','-lm','-o',str(work/'bench')],host_env)
            execute([clang,'-O2','-S',str(work/'bench.ll'),'-o',str(work/'bench.s')],host_env)
            execute([str(sdk),'build','-trimpath','-o',str(work/'reference'),str(work/'reference.go')],host_env)
            version=execute([str(sdk),'version'],host_env)['stdout'].strip()
            sdkroot=Path(execute([str(sdk),'env','GOROOT'],host_env)['stdout'].strip())
            report.update(status='built',sdk=version,target_triple=triple.group(1),executable_sha256=digest(work/'bench'),
                          reference_executable_sha256=digest(work/'reference'),llvm_sha256=digest(work/'bench.ll'),
                          assembly_sha256=digest(work/'bench.s'),sdk_sources={str(p.relative_to(sdkroot)):digest(p) for p in sorted((sdkroot/'src/image/color').glob('*.go')) if not p.name.endswith('_test.go')})
        else:
            for key,path in [('source_basis_sha256',args.source),('compiler_sha256',compiler),('runtime_sha256',runtime),
                             ('driver_sha256',driver),('reference_source_sha256',reference),('sdk_sha256',sdk),
                             ('executable_sha256',work/'bench'),('reference_executable_sha256',work/'reference'),
                             ('llvm_sha256',work/'bench.ll'),('assembly_sha256',work/'bench.s')]:
                if report[key]!=digest(path):
                    raise RuntimeError('changed sealed benchmark input: '+key)
            if report['scalar']!=args.scalar:
                raise RuntimeError('changed scalar/SIMD selection')
            rows=list(report.get('functional_preflights', []))
            for mode in json.loads((experiment/'benchmark-modes.json').read_text())['modes']:
                for size in args.sizes:
                    expected=execute([str(work/'reference'),mode,str(size),'3'],{**os.environ,'GOMAXPROCS':'1','GOTELEMETRY':'off'})
                    sums=list(map(int,expected['stdout'].split()))[1:]
                    for poison,heap in [('0','32M'),('1','256M')]:
                        actual=execute([str(work/'bench'),mode,str(size),'3'],{**os.environ,'CINCH_HEAP_LIMIT':heap,'CINCH_STACK_SIZE':'8M','CINCH_LEAK_CHECK':'1','CINCH_RC_DEBUG':poison,'CINCH_ALLOC_STATS':'0'})
                        if list(map(int,actual['stdout'].split()))[1:]!=sums or 'leaked=0' not in actual['stderr']:
                            raise RuntimeError('independent full-output/retained-owner mismatch: '+mode+'/'+str(size)+'/'+poison)
                        rows.append({'mode':mode,'size':size,'iterations':3,'poison':poison,'heap':heap,'stack':'8M',
                                     'stdout':actual['stdout'],'stderr':actual['stderr'],'reference_stdout':expected['stdout']})
            report.update(status='functional-preflights-passed',functional_preflights=rows,
                          elapsed_fields_are_not_timing_samples=True, smoke_runner_sha256=digest(Path(__file__)))
    except BaseException as error:
        report.update(status='failed-or-resource-paused',failure=repr(error),failed_artifacts_retained_plain=True)
        raise
    finally:
        save()
    print(report['status'],report['output_bytes'])


if __name__ == '__main__':
    main()
