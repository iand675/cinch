#!/usr/bin/env python3
"""Validate owner-sealed exact-byte queries on complete bounded spans and actual portable objects."""
from __future__ import annotations
import argparse
import hashlib
import json
import os
import re
from pathlib import Path
import shutil

from bench_stdlib import source_closure
from validate_byte_view import digest, input_bytes, run

ROOT = Path(__file__).resolve().parents[1]
FIXTURE = ROOT / 'compiler/tests/fixtures/byte-view-index-byte-vectors.json'
PROTO = ROOT / 'benchmarks/stdlib/byte-view-index-byte-prototype.cinch'


def definition(ir: str, symbol: str) -> str:
    """Inspect an actual emitted function body rather than an unrelated instruction spelling."""
    marker = '@"' + symbol + '"('
    start = next(match.start() for match in re.finditer(r'^define .*',ir,re.M) if marker in match.group())
    return ir[start:ir.index('\n}',start)+2]


def call_path(ir: str, start: str, wanted: str) -> list[str] | None:
    """Follow actual direct view calls, including intermediaries retained by newer compilers."""
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
                for callee in re.findall(r'@"(Data\.Bytes\.View\.[^"]+)"', line):
                    pending.append([*path, callee])
    return None


def assert_actual_query(ir: str) -> None:
    """Require real adapter-to-block/tail and worker paths, allowing intermediary inlining."""
    for caller, callee in [
        ('byte-view-index-byte', 'index-forward-blocks'),
        ('byte-view-index-byte', 'index-forward-tail'),
        ('index-forward-blocks', 'load-word.llvm'),
        ('index-forward-blocks', 'matching-lanes.llvm'),
        ('index-forward-tail', 'load-byte.llvm'),
    ]:
        assert call_path(ir, 'Data.Bytes.View.'+caller, 'Data.Bytes.View.'+callee), (caller, callee)
    if re.search(r'^define .*@"Data\.Bytes\.View\.index-forward"\(', ir, re.M):
        assert call_path(ir, 'Data.Bytes.View.byte-view-index-byte', 'Data.Bytes.View.index-forward')
        for worker in ['index-forward-blocks', 'index-forward-tail']:
            assert call_path(ir, 'Data.Bytes.View.index-forward', 'Data.Bytes.View.'+worker)
    word = definition(ir,'Data.Bytes.View.load-word.llvm')
    byte = definition(ir,'Data.Bytes.View.load-byte.llvm')
    masks = definition(ir,'Data.Bytes.View.matching-lanes.llvm')
    assert 'load i64, ptr %at, align 1' in word
    assert 'load i8, ptr %at, align 1' in byte
    assert '9187201950435737471' in masks and '-9187201950435737472' in masks


def spans(size: int):
    """Keep every short span and ordered larger-owner alignment/fringe observations."""
    if size <= 33:
        for start in range(size + 1):
            for end in range(start, size + 1):
                yield start, end
    else:
        starts = [*range(32), size//2, size-33, size-17, size-9, size-8, size-1, size]
        for start in starts:
            for width in [0,1,7,8,9,15,16,17,23,24,25,31,32,33]:
                if start+width <= size:
                    yield start, start+width
            yield start, size


def crosscheck(rows: list[dict]) -> None:
    """Check every public SDK search with an independent byte implementation and ordered fingerprint."""
    mask, prime = (1 << 64)-1, 1099511628211
    for row in rows:
        raw = input_bytes(row['mode'], row['size'])
        assert hashlib.sha256(raw).hexdigest() == row['raw_sha256']
        keys = range(256) if len(raw) <= 257 else [0,1,2,10,44,97,98,127,128,129,194,237,240,253,254,255]
        current, count, queries = 14695981039346656037, 0, 0
        for start, end in spans(len(raw)):
            count += 1
            current = ((current ^ start)*prime) & mask
            current = ((current ^ end)*prime) & mask
            view = raw[start:end]
            for key in keys:
                queries += 1
                at = view.find(bytes([key])) & 0xFFFFFFFF
                current = ((current ^ key)*prime) & mask
                current = ((current ^ at)*prime) & mask
        assert (current, count, queries) == (row['hash'], row['spans'], row['queries']), row


def consumer(checks: list[str], width: int) -> str:
    """Use public views and bounded process groups without mirroring the production search worker."""
    prefix = (ROOT / 'compiler/tests/fixtures/byte-view-consumer.cinchpart').read_text()
    prefix = prefix[:prefix.index('  -- | Compare an optional relative byte offset')]
    text = prefix.replace('  import Data.String\n', '  import Data.String\n  import Text.Hex\n')
    text += (ROOT / 'compiler/tests/fixtures/byte-view-index-byte-consumer.cinchpart').read_text()
    dispatch = 'False'
    for first in reversed(range(0, len(checks), width)):
        number = first//width
        text += f'\n  -- | Observe complete exact-byte group {number}.\n  group{number} :: Unit -> Bool\n  group{number} unit = all [\n'
        text += ',\n'.join(f'    ({value})' for value in checks[first:first+width]) + '\n    ]\n'
        dispatch = f'if index == "{number}" then group{number} () else ' + dispatch
    return text + '\n  -- | Select one bounded independent byte-query group.\n  selected :: String -> Bool\n  selected index = ' + dispatch + '\n  -- | Observe complete fingerprints and retained-owner controls.\n  main :: Unit ->{IO r} Int\n  main unit = with (io = newIO ()) in case process-args () of\n    Ok (Cons index Nil) -> if selected index then 0 else 1\n    other -> 2\n'


def prepare(text: str, work: Path, module: Path) -> tuple[Path, dict[str, Path]]:
    """Retain the exact reachable full-source closure, including the sealed adapter module."""
    work.mkdir(parents=True, exist_ok=True)
    entry = work / 'Main.cinch'
    entry.write_text(text)
    dependencies = source_closure(text, {'Data.Bytes.View':module})
    for name,path in dependencies.items():
        shutil.copyfile(path, work/(name+'.cinch'))
    return entry, dependencies


def main() -> None:
    """Keep full public algorithms, compiler epochs, matched objects and runtime ownership evidence explicit."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--compiler', type=Path)
    parser.add_argument('--runtime-archive', type=Path)
    parser.add_argument('--native-interfaces', type=Path)
    parser.add_argument('--native-objects', type=Path)
    parser.add_argument('--module-source', type=Path, default=PROTO)
    parser.add_argument('--go', type=Path)
    parser.add_argument('--generate', action='store_true')
    parser.add_argument('--crosscheck-only', action='store_true')
    parser.add_argument('--skip-crosscheck', action='store_true')
    parser.add_argument('--artifacts', type=Path)
    parser.add_argument('--output', type=Path)
    parser.add_argument('--opt-level', choices=['0','2'], default='0')
    parser.add_argument('--groups', nargs='+', choices=['rows','owners','portable'], default=['rows','owners'])
    parser.add_argument('--first', type=int, default=0)
    parser.add_argument('--limit', type=int)
    parser.add_argument('--reuse-native-artifacts', type=Path)
    parser.add_argument('--reuse-native-receipt', type=Path)
    args = parser.parse_args()
    if args.generate:
        assert args.go
        result = run([str(args.go), 'run', str(ROOT/'tooling/go_byte_view_index_vectors.go')],
                     env={**os.environ,'GOCACHE':'/tmp/cinch-go-surface-cache','GOTELEMETRY':'off','GOMAXPROCS':'1'})
        fixture = json.loads(result.stdout)
        crosscheck(fixture['rows'])
        FIXTURE.write_text(json.dumps(fixture,indent=2)+'\n')
    fixture = json.loads(FIXTURE.read_text())
    if not args.skip_crosscheck and not args.generate:
        crosscheck(fixture['rows'])
    print(f"Reference rows {len(fixture['rows'])}, spans {sum(r['spans'] for r in fixture['rows'])}, queries {sum(r['queries'] for r in fixture['rows'])}",flush=True)
    if args.crosscheck_only:
        return
    assert args.compiler and args.runtime_archive and args.artifacts and args.output
    report = {'compiler_sha256':digest(args.compiler),'runtime_sha256':digest(args.runtime_archive),
              'source_sha256':digest(args.module_source),'fixture_sha256':digest(FIXTURE),'opt_level':args.opt_level,
              'source_distributed_only':True,'compiler_stack_bytes':67108864,'program_stack_bytes':8388608,'results':[]}
    if args.native_interfaces:
        assert args.native_objects
        report['interfaces_sha256'] = digest(args.native_interfaces)
        report['objects_sha256'] = digest(args.native_objects)
    for group in args.groups:
        if group == 'portable':
            assert not args.native_interfaces, 'full foreign-target object emission requires the typed Rust frontend'
            text = (ROOT/'compiler/tests/fixtures/byte-view-index-byte-portable.cinch').read_text()
            work = args.artifacts/group
            entry,deps = prepare(text,work,args.module_source)
            objects = []
            for triple in ['wasm32-unknown-unknown','aarch64-unknown-linux-gnu']:
                ir = work/(triple+'.ll')
                run([str(args.compiler.resolve()),'emit-ir',str(entry.resolve()),'-o',str(ir.resolve()),'--target',triple,'--rc-debug'])
                body = ir.read_text()
                assert_actual_query(body)
                obj = ir.with_suffix('.o')
                run([os.environ.get('CINCH_CLANG','clang'),'-target',triple,'-O2','-c','-x','ir',str(ir),'-o',str(obj)])
                assert obj.stat().st_size > 0
                objects.append({'triple':triple,'llvm_sha256':digest(ir),'object_sha256':digest(obj)})
            report['results'].append({'group':group,'entry_sha256':digest(entry),'full_public_adapter':True,'objects':objects})
            continue
        rows = fixture['rows'][args.first:]
        if args.limit is not None:
            rows = rows[:args.limit]
        checks = [f'index-case {r["mode"]} {r["size"]} "{r["raw_sha256"]}" {r["hash"]}' for r in rows] if group=='rows' else ['index-owner-case ()']*10
        width = 1
        work = args.artifacts/group
        text = consumer(checks,width)
        entry,deps = prepare(text,work,args.module_source)
        exe = (work/'consumer').resolve()
        if args.native_interfaces:
            bundle = work/'Bundle.cinch'
            bundle.write_text('\n'.join(p.read_text() for p in reversed(list(deps.values())))+'\n'+text)
            command = [str(args.compiler.resolve()),'--emit-ir','--rc-debug','--interfaces',str(args.native_interfaces.resolve()),
                       '--objects',str(args.native_objects.resolve()),str(bundle.resolve()),str(exe.with_suffix('.ll'))]
        else:
            command = [str(args.compiler.resolve()),'build',str(entry.resolve()),'-o',str(exe),'--opt-level',args.opt_level,
                       '--rc-debug','--runtime-archive',str(args.runtime_archive.resolve())]
        if args.reuse_native_artifacts or args.reuse_native_receipt:
            assert args.native_interfaces and args.reuse_native_artifacts and args.reuse_native_receipt and args.opt_level=='2'
            previous = json.loads(args.reuse_native_receipt.read_text())
            for key in ['compiler_sha256','runtime_sha256','source_sha256','fixture_sha256','interfaces_sha256','objects_sha256']:
                assert previous[key]==report[key],key
            before = next(value for value in previous['results'] if value['group']==group)
            assert before['entry_sha256']==digest(entry)
            assert before['dependencies']=={name:digest(p) for name,p in deps.items()}
            original = args.reuse_native_artifacts/group/'consumer.ll'
            assert before['llvm_sha256']==digest(original)
            shutil.copyfile(original,exe.with_suffix('.ll'))
            report['reused_native_receipt_sha256'] = digest(args.reuse_native_receipt)
        else:
            try:
                compiled = run(command)
            except RuntimeError as error:
                diagnostics = exe.with_suffix('.ll.diagnostics')
                raise RuntimeError(str(error)+'\n'+(diagnostics.read_text() if diagnostics.exists() else '')) from error
            (work/'compile.log').write_text(compiled.stdout+compiled.stderr)
        if args.native_interfaces:
            configuration = json.loads(args.native_objects.read_text())
            assert digest(Path(configuration['object'])) == configuration['object_sha256']
            linked = run([os.environ.get('CINCH_CLANG','clang'),'-O'+args.opt_level,str(exe.with_suffix('.ll')),configuration['object'],
                          str(args.runtime_archive.resolve()),'-o',str(exe),'-ldl','-lpthread','-lm'])
            (work/'link.log').write_text(linked.stdout+linked.stderr)
            report['native_object_sha256'] = configuration['object_sha256']
        profiles=[]
        for debug in [False,True]:
            for index in range(len(checks)):
                result = run([str(exe),str(index)],env={**os.environ,'CINCH_RC_DEBUG':str(int(debug)),
                             'CINCH_HEAP_LIMIT':'256M' if debug else '32M','CINCH_STACK_SIZE':'8M','CINCH_LEAK_CHECK':'1'})
                assert 'leaked=0' in result.stderr
                profiles.append({'debug':debug,'index':index,'zero_leaks':True})
        report['results'].append({'group':group,'cases':len(checks),'entry_sha256':digest(entry),'llvm_sha256':digest(exe.with_suffix('.ll')),
                                  'executable_sha256':digest(exe),'dependencies':{name:digest(p) for name,p in deps.items()},'profiles':profiles})
        print(f'{group}: {len(checks)} complete observations, normal+poison zero leaks',flush=True)
    args.output.parent.mkdir(parents=True,exist_ok=True)
    args.output.write_text(json.dumps(report,indent=2)+'\n')


if __name__ == '__main__':
    main()
