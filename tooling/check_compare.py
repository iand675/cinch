#!/usr/bin/env python3
"""Prepare and check independent comparison fixtures with explicitly supplied frozen frontends.

No compiler is built here. An optional ordinary build initializes its host runtime.
Compile fixture-only C, emit LLVM through each public
frontend, link against its exact runtime/provider, and run ordinary and poisoned
allocation profiles separately. All commands, hashes, failures and timing samples
are retained in the report rather than normalized away.
"""
from __future__ import annotations
import argparse
import fcntl
import hashlib
import json
import os
from pathlib import Path
import re
import resource
import shutil
import subprocess
import time

ROOT=Path(__file__).resolve().parents[1]


def digest(path: Path) -> str:
    """Identify the bytes actually used for source, compiler and object artifacts."""
    return hashlib.sha256(path.read_bytes()).hexdigest()


def initialize_runtime(compiler: Path, runtime: Path, work: Path) -> tuple[Path,dict]:
    """Use the normal build path to refresh the exact host archive and retain a locked snapshot."""
    source=work/'RuntimeInit.cinch'
    source.write_text('''-- | Initialize the current Rust compiler's normal host runtime build.
module Main where
  -- | Return successfully without depending on any comparison fixture.
  main :: Int
  main = 0
''')
    executable=work/'runtime-initializer'
    command=[str(compiler),'build',str(source),'-o',str(executable)]
    built=run(command,work,600)
    env={**os.environ,'CINCH_HEAP_LIMIT':'32M','CINCH_STACK_SIZE':'8M','CINCH_LEAK_CHECK':'1','CINCH_RC_DEBUG':'0'}
    initialized=run([str(executable)],work,60,env)
    if 'leaked=0' not in initialized.stderr: raise RuntimeError('runtime initializer omitted zero-leak evidence')
    if not runtime.is_file(): raise RuntimeError(f'normal Rust build did not produce the expected host archive: {runtime}')
    snapshot=work/'host-runtime.a'
    with (runtime.parent.parent/'.cinch-runtime.lock').open('a+b') as lock:
        fcntl.flock(lock,fcntl.LOCK_SH)
        shutil.copyfile(runtime,snapshot)
        archive_sha256=digest(runtime)
        if digest(snapshot)!=archive_sha256: raise RuntimeError('host runtime changed while being captured')
    return snapshot,{'command':command,'archive_path':str(runtime),'archive_sha256':archive_sha256,'snapshot_sha256':digest(snapshot),'initializer_source_sha256':digest(source),'initializer_executable_sha256':digest(executable),'build_stdout':built.stdout,'build_stderr':built.stderr,'run_stdout':initialized.stdout,'run_stderr':initialized.stderr}


def run(argv: list[str], cwd: Path, timeout: int = 180, env: dict | None = None) -> subprocess.CompletedProcess:
    """Run one bounded command, retaining diagnostics on failure."""
    result=subprocess.run(list(map(str,argv)),cwd=cwd,text=True,capture_output=True,timeout=timeout,env=env)
    if result.returncode:
        diagnostics='\n'.join(path.read_text() for path in cwd.glob('*.diagnostics'))
        raise RuntimeError(f'{argv}: exit {result.returncode}\n{result.stdout}\n{result.stderr}\n{diagnostics}')
    return result


def list_of(values: list[str]) -> str:
    """Build a Cinch list without relying on frontend collection desugaring."""
    out='Nil'
    for value in reversed(values): out=f'(Cons {value} {out})'
    return out


def text(value: str) -> str:
    """Spell supported literal escapes while retaining Unicode scalar values."""
    return json.dumps(value,ensure_ascii=False).replace('\\u0000','\\u{0}')


def header() -> str:
    """Declare public imports and the independently linked payload fixture API."""
    return '''module Main where
  import Data.Compare
  import Data.Machine
  import Data.Ordering
  import Data.Core
  import Data.String

  -- | Construct an exact binary32 fixture through an actual foreign object.
  foreign import ccall "compare_from_f32" from-f32 :: U32 ->{Global} F32
  -- | Read exact binary32 result bits through an actual foreign object.
  foreign import ccall "compare_bits_f32" bits-f32 :: F32 ->{Global} U32
  -- | Construct an exact binary64 fixture through an actual foreign object.
  foreign import ccall "compare_from_f64" from-f64 :: U64 ->{Global} F64
  -- | Read exact binary64 result bits through an actual foreign object.
  foreign import ccall "compare_bits_f64" bits-f64 :: F64 ->{Global} U64

  -- | Project a comparison to the independent reference's numeric result.
  code :: Ordering -> I32
  code x = case x of
    LT -> -1
    EQ -> 0
    GT -> 1
'''


def scalar_source(kind: str, rows: list[dict]) -> str:
    """Walk frozen expected results instead of generating a large unrolled condition tree."""
    f=kind in ('F32','F64'); string=kind=='String'
    stored=f'U{kind[1:]}' if f else kind
    q=kind.lower()
    values=[]
    for row in rows:
        a=text(row['x']) if string else str(row['x'])
        b=text(row['y']) if string else str(row['y'])
        values.append(f'({a}, ({b}, ({row["compare"]}, {str(row["less"])})))')
    a=f'from-{q} x' if f else 'x'; b=f'from-{q} y' if f else 'y'
    extra=f' && eq-i32 (code (compare-{q} a b)) expected && less-{q} a b == expectedLess' if f else ''
    return header()+f'''
  -- | Frozen operands and independent expected comparison/precedence results.
  fixtures :: List ({stored}, ({stored}, (I32, Bool)))
  fixtures = {list_of(values)}

  -- | Validate every frozen row, returning its one-based index on failure.
  check :: List ({stored}, ({stored}, (I32, Bool))) -> Int ->{{Global}} Int
  check rows index = case rows of
    Nil -> 0
    Cons row rest -> case row of
      (x, pair) -> case pair of
        (y, expectedPair) -> case expectedPair of
          (expected, expectedLess) ->
            let a = {a} in
            let b = {b} in
            if eq-i32 (code (compare a b)) expected && less a b == expectedLess{extra}
            then check rest (index + 1) else index

  -- | Run the entire {kind} comparison corpus.
  main :: Unit ->{{Global}} Int
  main unit = check fixtures 1
'''


def defaults_source(data: dict) -> tuple[str,int]:
    """Exercise zero/default policy, returned identity, short circuiting and custom capabilities."""
    checks=[]
    for t in ('I8','I16','I32','I64','Isize','U8','U16','U32','U64','Usize','Int','Nat'):
        checks += [f'is-default (default-value :: {t})',f'is-default (first-non-default (Nil :: List {t}))',f'is-default (first-non-default (Cons (0 :: {t}) (Cons 0 Nil)))',f'first-non-default (Cons (0 :: {t}) (Cons 7 (Cons 8 Nil))) == (7 :: {t})']
    checks += [
     'first-non-default (Cons "" (Cons "雪" (Cons "later" Nil))) == "雪"',
     'is-default (first-non-default (Nil :: List String))',
     'first-non-default (Cons False (Cons True Nil))',
     "first-non-default (Cons '\\u{0}' (Cons '雪' Nil)) == '雪'",
     'is-default (first-non-default (Nil :: List Unit))',
     'eq-i32 (first-non-default-by 99 (\\x -> eq-i32 x 0) Nil) 99',
     'eq-i32 (first-non-default-by 99 (\\x -> eq-i32 x 0) (Cons 0 (Cons 42 Nil))) 42',
     'eq-i32 (first-non-default-by 99 (\\x -> eq-i32 x 0) (Cons 0 (Cons 0 Nil))) 99',
     'case first-non-default (Cons Blank (Cons (Value 0) (Cons (Value 7) Nil))) of Blank -> False; Value x -> eq-i32 x 0',
     'eq-i32 (code (compare (Value 1) (Value 2))) -1',
     'eq-i32 (code (compare (123456789012345678901234567890 :: Int) 0)) 1',
     'eq-i32 (code (compare (123456789012345678901234567890 :: Nat) 0)) 1',
     "eq-i32 (code (compare 'é' '雪')) -1",
    ]
    for width,negative,nan in [(32,0x80000000,0xffc01234),(64,0x8000000000000000,0xfff8123412345678)]:
        q=f'f{width}'; u=f'u{width}'
        checks += [f'eq-{u} (bits-{q} (first-non-default (Nil :: List F{width}))) 0',
         f'is-default (from-{q} {negative})',f'eq-{u} (bits-{q} (first-non-default (Cons (from-{q} {negative}) Nil))) 0',
         f'eq-{u} (bits-{q} (first-non-default (Cons (from-{q} {negative}) (Cons (from-{q} {nan}) Nil)))) {nan}',
         f'not (is-default (from-{q} {nan}))']
        checks.append(f'eq-{u} (bits-{q} (first-non-default-by (from-{q} {negative}) (\\x -> feq-{q} x (i32-to-{q} 0)) Nil)) {negative}')
        for bits in sorted({row['x'] for row in data[q]}-{0,negative}):
            checks.append(f'eq-{u} (bits-{q} (first-non-default (Cons (i32-to-{q} 0) (Cons (from-{q} {bits}) (Cons (i32-to-{q} 1) Nil))))) {bits}')
    src=header()+'''
  -- | A custom record-like value with an explicit distinguished default.
  data Choice where
    -- | No candidate has been supplied.
    Blank :: Choice
    -- | A present payload, including zero.
    Value :: I32 -> Choice
  -- | A custom default proves first-non-default needs no global Eq instance.
  implicit instance Default_Choice :: Default Choice where
    default-value = Blank
    is-default x = case x of Blank -> True; Value _ -> False
  -- | Order absence before present values, then compare their payloads.
  implicit instance Compare_Choice :: Compare Choice where
    compare x y = case x of
      Blank -> case y of Blank -> EQ; Value _ -> LT
      Value a -> case y of Blank -> GT; Value b -> compare a b
'''
    for i,expr in enumerate(checks):
        src+=f'\n  -- | Check default/selection policy case {i+1}.\n  step{i} :: Unit ->{{Global}} Int\n  step{i} unit = if ({expr}) then step{i+1} unit else {i+1}\n'
    src+=f'\n  -- | Successful termination of the policy corpus.\n  step{len(checks)} :: Unit ->{{Global}} Int\n  step{len(checks)} unit = 0\n\n  -- | Run every default/selection policy assertion.\n  main :: Unit ->{{Global}} Int\n  main unit = step0 unit\n'
    return src,len(checks)


def prepare(work: Path, vectors: Path) -> dict[str,int]:
    """Materialize sources before requesting any compile resources."""
    data=json.loads(vectors.read_text()); counts={}
    shutil.copyfile(ROOT/'compiler/prelude/Data/Compare.cinch',work/'Data.Compare.cinch')
    for kind,key in [('F32','f32'),('F64','f64'),('String','strings')]:
        rows=data[key]; (work/f'{kind}.source').write_text(scalar_source(kind,rows)); counts[kind]=len(rows)
    for kind in sorted({row['type'] for row in data['integers']}):
        rows=[row for row in data['integers'] if row['type']==kind]
        (work/f'{kind}.source').write_text(scalar_source(kind,rows)); counts[kind]=len(rows)
    source,count=defaults_source(data); (work/'Default.source').write_text(source); counts['Default']=count
    return counts


def main() -> None:
    """Prepare fixtures or execute exact frozen frontend/runtime pairs under bounded profiles."""
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--work',type=Path,required=True)
    parser.add_argument('--vectors',type=Path,default=ROOT/'compiler/tests/fixtures/compare-vectors.json')
    parser.add_argument('--prepare-only',action='store_true')
    parser.add_argument('--resume',action='store_true',help='Retain verified completed cases with identical source and frontend identities')
    parser.add_argument('--rust',type=Path)
    parser.add_argument('--rust-runtime',type=Path)
    parser.add_argument('--initialize-rust-runtime',action='store_true',help='Initialize/refresh the current Rust compiler host runtime through an ordinary build, then capture its exact archive')
    parser.add_argument('--native-rc-debug',action='store_true',help='Emit Native count operations through checked out-of-line runtime entries; Rust CLI does not expose this option')
    parser.add_argument('--native',type=Path,help='Frozen snapshot directory with cinch, manifests, runtime and provider')
    parser.add_argument('--only',nargs='+')
    parser.add_argument('--cpu',type=int)
    args=parser.parse_args(); work=args.work.resolve(); work.mkdir(parents=True,exist_ok=True)
    if args.rust and not args.rust_runtime: parser.error('--rust requires its exact --rust-runtime archive')
    if args.initialize_rust_runtime and (not args.rust or args.native or args.resume): parser.error('--initialize-rust-runtime requires a fresh explicit Rust-only invocation')
    if args.native_rc_debug and not args.native: parser.error('--native-rc-debug requires --native')
    counts=prepare(work,args.vectors)
    if args.prepare_only: print(json.dumps(counts)); return
    if args.cpu is not None: os.sched_setaffinity(0,{args.cpu})
    resource.setrlimit(resource.RLIMIT_STACK,(64*1024*1024,resource.getrlimit(resource.RLIMIT_STACK)[1]))
    clang=os.environ.get('CINCH_CLANG','clang')
    run([clang,'-O2','-c',str(ROOT/'tooling/data/compare-bits.c'),'-o',str(work/'bits.o')],work)
    report={'format':1,'compiler_host_stack_bytes':64*1024*1024,'runner_sha256':digest(Path(__file__)),'counts':counts,'vectors_sha256':digest(args.vectors),'library_sha256':digest(work/'Data.Compare.cinch'),'foreign_source_sha256':digest(ROOT/'tooling/data/compare-bits.c'),'foreign_object_sha256':digest(work/'bits.o'),'frontends':{},'runs':[]}
    if args.resume:
        previous=json.loads((work/'report.json').read_text())
        for key in ('vectors_sha256','library_sha256','foreign_source_sha256','foreign_object_sha256'):
            if previous[key]!=report[key]: raise RuntimeError(f'cannot resume changed input {key}')
        report=previous
    frontends=[]
    if args.rust:
        runtime=args.rust_runtime.resolve()
        if args.initialize_rust_runtime:
            runtime,initialization=initialize_runtime(args.rust.resolve(),runtime,work)
            report['runtime_initialization']=initialization
        frontends.append(('rust',args.rust.resolve(),runtime))
    if args.native: frontends.append(('native',args.native.resolve()/'cinch',args.native.resolve()/'libcinchruntime.a'))
    for label,compiler,runtime in frontends:
        if label in report['frontends'] and (report['frontends'][label]['compiler_sha256']!=digest(compiler) or report['frontends'][label]['runtime_sha256']!=digest(runtime)):
            raise RuntimeError(f'cannot resume changed frontend {label}')
        if label in report['frontends'] and report['frontends'][label].get('compile_rc_debug',False)!=(label=='native' and args.native_rc_debug):
            raise RuntimeError(f'cannot resume changed count instrumentation for {label}')
        report['frontends'][label]={'compiler':str(compiler),'compiler_sha256':digest(compiler),'runtime':str(runtime),'runtime_sha256':digest(runtime),'compile_rc_debug':label=='native' and args.native_rc_debug}
        if label=='native':
            snap=args.native.resolve(); cfg=json.loads((snap/'objects.json').read_text()); provider=Path(cfg['object'])
            report['frontends'][label].update({key:digest(snap/key) for key in ('base-interfaces.json','objects.json')}); report['frontends'][label]['provider_sha256']=digest(provider)
        for name in args.only or counts:
            if any(entry['frontend']==label and entry['case']==name and len(entry.get('profiles',[]))==2 for entry in report['runs']): continue
            shutil.copyfile(work/f'{name}.source',work/'Main.cinch')
            ir=work/f'{label}-{name}.ll'; exe=work/f'{label}-{name}'
            (work/'bundle.source').write_text((work/'Data.Compare.cinch').read_text()+'\n'+(work/'Main.cinch').read_text())
            command=[str(compiler),'emit-ir',str(work/'Main.cinch'),'-o',str(ir)] if label=='rust' else [str(compiler),'--emit-ir','--interfaces',str(snap/'base-interfaces.json'),'--objects',str(snap/'objects.json'),str(work/'bundle.source'),str(ir)]
            if label=='native' and args.native_rc_debug: command.insert(2,'--rc-debug')
            start=time.monotonic(); result=run(command,work,300)
            target=re.search(r'target triple = "([^"]+)"',ir.read_text())
            triple=target.group(1) if target else cfg['target'] if label=='native' else 'unspecified'
            native_bits=json.loads(args.vectors.read_text())['native_word_bits']
            if name in ('Isize','Usize') and not (native_bits==64 and triple.startswith(('x86_64','aarch64'))):
                raise RuntimeError(f'native-width fixture mismatch: reference {native_bits}, LLVM target {triple}')
            report['runs'].append({'target_triple':triple,'target_source':'LLVM' if target else 'matched provider configuration','reference_native_word_bits':native_bits,'frontend':label,'case':name,'compile_seconds':time.monotonic()-start,'command':command,'stdout':result.stdout,'stderr':result.stderr,'ir_sha256':digest(ir)})
            report['runs'][-1].update(consumer_source_sha256=digest(work/'Main.cinch'),consumer_source_path=str(work/f'{name}.source'))
            if label=='native': report['runs'][-1]['source_bundle_sha256']=digest(work/'bundle.source')
            link=[clang,'-O2','--target='+triple,str(ir),str(work/'bits.o')]
            if label=='native': link.append(str(provider))
            link.extend([str(runtime),'-ldl','-lpthread','-lm','-o',str(exe)])
            run(link,work)
            run([clang,'-O2','-S',str(ir),'-o',str(exe)+'.s'],work)
            entry=report['runs'][-1]; entry['executable_sha256']=digest(exe); entry['profiles']=[]
            for poison,heap in [('0','32M'),('1','256M')]:
                env={**os.environ,'CINCH_LEAK_CHECK':'1','CINCH_RC_DEBUG':poison,'CINCH_HEAP_LIMIT':heap,'CINCH_STACK_SIZE':'8M'}
                result=run([str(exe)],work,60,env)
                if 'leaked=0' not in result.stderr:
                    raise RuntimeError(f'missing zero-leak evidence: {result.stderr}')
                entry['profiles'].append({'poison':poison,'heap':heap,'stack':'8M','stdout':result.stdout,'stderr':result.stderr})
            (work/'report.json').write_text(json.dumps(report,indent=2)+'\n')
            print(label,name,counts[name],f'{entry["compile_seconds"]:.2f}s',flush=True)
    (work/'report.json').write_text(json.dumps(report,indent=2)+'\n')


if __name__=='__main__': main()
