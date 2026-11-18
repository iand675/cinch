#!/usr/bin/env python3
"""Compile complete retained split pipelines and their actual packed/fringe worker graph."""
from __future__ import annotations
import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import shutil

from bench_stdlib import source_closure
from validate_byte_view import run

ROOT = Path(__file__).resolve().parents[1]


def digest(path: Path) -> str:
    """Identify each exact input, emitted complete program and actual object."""
    return hashlib.sha256(path.read_bytes()).hexdigest()


def definition(ir: str, symbol: str) -> str:
    """Read one complete emitted definition without matching unrelated header instructions."""
    marker = '@"' + symbol + '"('
    start = next(m.start() for m in re.finditer(r'^define .*', ir, re.M) if marker in m.group())
    return ir[start:ir.index('\n}', start)+2]


def call_path(ir: str, start: str, wanted: str) -> list[str] | None:
    """Follow actual direct calls through retained or inlined split/search intermediaries."""
    pending, seen = [[start]], set()
    while pending:
        path = pending.pop()
        symbol = path[-1]
        if symbol == wanted:
            return path
        if symbol in seen:
            continue
        seen.add(symbol)
        for line in definition(ir, symbol).splitlines():
            if 'call ' in line:
                for callee in re.findall(r'@"((?:Main|Data\.Bytes\.(?:Split|View))\.[^"]+)"\(', line):
                    pending.append([*path, callee])
    return None


def actual_workers(ir: str) -> dict[str, list[str]]:
    """Require public splitting to reach complete blocks, scalar fringes and exact load/mask kernels."""
    paths = {}
    for worker in ['Data.Bytes.Split.split-step', *['Data.Bytes.View.'+name for name in ['index-forward-blocks', 'index-forward-tail', 'load-word.llvm', 'load-byte.llvm', 'matching-lanes.llvm']]]:
        path = call_path(ir, 'Main.main', worker)
        assert path, worker
        paths[worker] = path
    for start, end in [('index-forward-blocks', 'load-word.llvm'), ('index-forward-blocks', 'matching-lanes.llvm'), ('index-forward-tail', 'load-byte.llvm')]:
        assert call_path(ir, 'Data.Bytes.View.'+start, 'Data.Bytes.View.'+end), (start, end)
    assert 'load i64, ptr %at, align 1' in definition(ir, 'Data.Bytes.View.load-word.llvm')
    assert 'load i8, ptr %at, align 1' in definition(ir, 'Data.Bytes.View.load-byte.llvm')
    mask = definition(ir, 'Data.Bytes.View.matching-lanes.llvm')
    assert '9187201950435737471' in mask and '-9187201950435737472' in mask
    return paths


def main() -> None:
    """Keep target-specific complete sources separate and require unchanged positive heap/stack profiles."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--compiler', type=Path, required=True)
    parser.add_argument('--runtime-archive', type=Path, required=True)
    parser.add_argument('--module-source', type=Path, default=ROOT/'compiler/prelude/Data/Bytes/Split.cinch')
    parser.add_argument('--view-source', type=Path, default=ROOT/'compiler/prelude/Data/Bytes/View.cinch')
    parser.add_argument('--artifacts', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    source = (ROOT/'compiler/tests/fixtures/byte-split-portable.cinch').read_text()
    deps = source_closure(source, {'Data.Bytes.Split':args.module_source, 'Data.Bytes.View':args.view_source})
    report = {'compiler_sha256':digest(args.compiler),'runtime_sha256':digest(args.runtime_archive),
              'entry_sha256':hashlib.sha256(source.encode()).hexdigest(),
              'dependencies':{name:digest(path) for name,path in deps.items()},
              'compiler_stack_bytes':67108864,'program_stack_bytes':8388608,
              'compiled_rc_debug':False,'runtime_profile_note':'Dynamic normal/poison runs do not enable compiled per-operation RC assertions',
              'status':'prepared','results':[],'ppc_scope':'maintained Rust API gate; unsupported by frozen CLI'}
    def save() -> None:
        """Preserve the current exact portability boundary before assertions or object emission."""
        args.output.parent.mkdir(parents=True, exist_ok=True)
        args.output.write_text(json.dumps(report, indent=2)+'\n')
    save()
    for target in ['wasm32-unknown-unknown','aarch64-unknown-linux-gnu','host-o0','host-o2']:
        work = (args.artifacts/target).resolve()
        work.mkdir(parents=True, exist_ok=True)
        entry = work/'Main.cinch'
        entry.write_text(source)
        for name,path in deps.items():
            shutil.copyfile(path,work/(name+'.cinch'))
        report['status']='preparing-'+target
        save()
        if target.startswith('host-'):
            opt = target[-1]
            exe = work/'consumer'
            result = run([str(args.compiler.resolve()),'build',str(entry),'-o',str(exe),'--opt-level',opt,
                          '--runtime-archive',str(args.runtime_archive.resolve())])
            (work/'compile.log').write_text(result.stdout+result.stderr)
            profiles=[]
            for debug in [0,1]:
                result=run([str(exe)],env={**os.environ,'CINCH_RC_DEBUG':str(debug),'CINCH_HEAP_LIMIT':'256M' if debug else '32M',
                                           'CINCH_STACK_SIZE':'8M','CINCH_LEAK_CHECK':'1'})
                (work/f'run-{debug}.log').write_text(result.stdout+result.stderr)
                assert 'leaked=0' in result.stderr
                profiles.append({'debug':debug,'zero_leaks':True})
            report['results'].append({'target':target,'llvm_sha256':digest(exe.with_suffix('.ll')),'exe_sha256':digest(exe),'profiles':profiles})
        else:
            file = work/'consumer.ll'
            result=run([str(args.compiler.resolve()),'emit-ir',str(entry),'--target',target,'-o',str(file)])
            (work/'compile.log').write_text(result.stdout+result.stderr)
            paths=actual_workers(file.read_text())
            object_file=work/'consumer.o'
            result=run([os.environ.get('CINCH_CLANG','clang'),'-target',target,'-O2','-c',str(file),'-o',str(object_file)])
            (work/'object.log').write_text(result.stdout+result.stderr)
            report['results'].append({'target':target,'llvm_sha256':digest(file),'object_sha256':digest(object_file),'actual_worker_paths':paths})
        save()
    report['status']='passed'
    save()
    print('Complete public split: wasm/ARM objects and four host O0/O2 ownership profiles pass')


if __name__ == '__main__':
    main()
