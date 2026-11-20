#!/usr/bin/env python3
"""Compare exact packed byte scans with scalar machine-loop equivalents in observable public pipelines."""
from __future__ import annotations
import argparse
import hashlib
import json
import os
from pathlib import Path
import statistics
import sys
from bench_stdlib import source_closure
from validate_byte_search import digest,run

ROOT=Path(__file__).resolve().parents[1]
SOURCE=r'''-- | Observe complete public byte queries after runtime initialization and checked mutation.
module Main where
  import Data.Bytes.Search as S
  import Data.Bytes
  import Data.Bool
  import Data.String
  import Data.Char
  import Data.List
  import Data.Machine
  import Data.Nat
  import Data.Option
  import Data.Result
  import Effect.Clock
  import Effect.Process
  import Text.Hex
  -- | Parse bounded validated harness numbers before timing.
  decimal :: String -> Nat
  decimal text = decimal-from (unpack text) 0
  -- | Accumulate exact decimal digits without recursive return frames.
  decimal-from :: List Char -> Nat -> Nat
  decimal-from chars value = case chars of
    Nil -> value
    Cons first rest -> case digit-value first of
      None -> 0
      Some digit -> let next = value * 10 + digit in decimal-from rest next
  -- | Produce a range-checked initialization byte before the timed interval.
  pattern-byte :: Bool -> Nat -> InRange 0 255 Nat
  pattern-byte dense at = if dense then nat-rem at 2 else 0
  -- | Produce an exact bounded marker for ordinary checked timed stores.
  marker-byte :: Bool -> Nat -> InRange 0 255 Nat
  marker-byte dense i = let parity = nat-rem i 2 in
    let value = ((if dense then parity else 254 + parity) :: Nat) in
    let byte = nat-rem value 256 in byte
  -- | Initialize exact dense or sparse query input outside timing.
  fill :: Bytes -> Bool -> Nat -> Unit
  fill raw dense at = if at < bytes-len raw then
    let byte = pattern-byte dense at in
    let written = bytes-set raw at byte in let next = at + 1 in fill raw dense next
    else ()
  -- | Represent missing query offsets with an independent length-plus-one sentinel.
  offset :: Bytes -> Option Nat -> Nat
  offset raw value = case value of None -> bytes-len raw + 1; Some at -> at
  -- | Mutate visible checked bytes so every measured query observes changed runtime data.
  -- Sparse inputs have one alternating marker; dense inputs retain all other lanes.
  operation :: String -> Bytes -> Bool -> Nat -> Nat
  operation kind raw dense i =
    let middle = nat-div (bytes-len raw) 2 in let following = middle + 1 in
    if following < bytes-len raw && middle < bytes-len raw then
      let zeroed = bytes-set raw middle 0 in let reset = bytes-set raw following 0 in
      let parity = nat-rem i 2 in let at = middle + parity in
      if at < bytes-len raw then
        let needle-value = marker-byte dense i in
        let written = bytes-set raw at needle-value in
        let needle = wrapping-i64-to-u8 (nat-to-i64 needle-value) in
        if kind == "index" then offset raw (S.bytes-index-byte raw needle)
        else if kind == "last" then offset raw (S.bytes-last-index-byte raw needle)
        else if kind == "count" then S.bytes-count-byte raw needle
        else let first = offset raw (S.bytes-index-byte raw needle) in
          let last = offset raw (S.bytes-last-index-byte raw needle) in
          let count = S.bytes-count-byte raw needle in
          first + last * 257 + count * 65537 + (if S.bytes-contains-byte raw needle then 16777217 else 0)
      else 0
    else 0
  -- | Sequence complete public query observations in an ordinary tail loop.
  work :: String -> Bytes -> Bool -> Nat -> Nat -> Nat -> Nat
  work kind raw dense left i total = if left == 0 then total else
    let value = operation kind raw dense i in let next-total = nat-rem (total + value) 1000000007 in
    let remaining = left - 1 in let next = i + 1 in work kind raw dense remaining next next-total
  -- | Keep the input owner observable beyond the ending clock and release it afterward.
  retained :: Bytes -> Nat
  retained raw = let size = bytes-len raw in if size > 0 then let last = size - 1 in
    if last < bytes-len raw then size + bytes-get raw 0 * 257 + bytes-get raw last * 65537 else 0
    else 0
  -- | Measure query work after setup, with output digests and retained inputs outside timing.
  measure :: forall r. using (io :: Io r) => String -> Bool -> Nat -> Nat ->{IO r} Int
  measure kind dense size iterations =
    let raw = bytes-new size in let initialized = fill raw dense 0 in
    let original = hex-encode (sha256-bytes raw) in
    let start = clock-monotonic-ns () in let checksum = work kind raw dense iterations 0 0 in
    let end = clock-monotonic-ns () in let kept = retained raw in
    let output = string-concat-all [nat-to-string (saturating-sub end start)," ",nat-to-string checksum," ",original," ",nat-to-string kept,"\n"] in
    let printed = ioWrite (string-to-bytes output) in 0
  -- | Resolve runtime workload arguments before all measured operations.
  main :: Unit ->{IO r} Int
  main unit = with (io = newIO ()) in case process-args () of
    Ok (Cons kind (Cons pattern (Cons size (Cons iterations Nil)))) -> measure kind (pattern == "dense") (decimal size) (decimal iterations)
    other -> 2
'''


def scalar_source(text:str)->str:
    """Retain identical owners and machine counters while routing complete blocks to exact scalar tails."""
    replacements=[('  -- | Find a matching first byte','  -- | Read only remaining scalar bytes', '''  -- | Search all bytes through the equivalent checked-length scalar worker.
  index-forward :: I64 -> I64 -> I64 -> U64 -> I64
  index-forward base at end repeated = let byte = word-byte repeated in index-forward-tail base at end byte

'''),('  -- | Find a matching final byte','  -- | Read only the leading scalar fringe', '''  -- | Search all suffix bytes through the equivalent checked-length scalar worker.
  index-backward :: I64 -> I64 -> U64 -> I64
  index-backward base end repeated = let byte = word-byte repeated in index-backward-tail base end byte

'''),('  -- | Count matching complete-word lanes','  -- | Count only available scalar-fringe bytes', '''  -- | Count all bytes through the equivalent checked-length scalar worker.
  count-forward :: I64 -> I64 -> I64 -> U64 -> I64 -> I64
  count-forward base at end repeated total = let byte = word-byte repeated in count-tail base at end byte total

''')]
    for begin,end,replacement in replacements:
        first=text.index(begin);last=text.index(end,first);text=text[:first]+replacement+text[last:]
    return text


def expected(kind:str,dense:bool,size:int,iterations:int)->tuple[int,str,int]:
    """Compute changed query outputs from independent exact mutable byte arrays."""
    raw=bytearray((i%2 if dense else 0) for i in range(size));original=hashlib.sha256(raw).hexdigest();total=0
    for i in range(iterations):
        middle=size//2;raw[middle]=raw[middle+1]=0;needle=(i%2) if dense else 254+i%2;raw[middle+i%2]=needle
        first=raw.find(bytes([needle]));last=raw.rfind(bytes([needle]));count=raw.count(needle)
        first=first if first>=0 else size+1;last=last if last>=0 else size+1
        value=first if kind=='index' else last if kind=='last' else count if kind=='count' else first+last*257+count*65537+(16777217 if count else 0)
        total=(total+value)%1000000007
    return total,original,size+raw[0]*257+raw[-1]*65537


def main()->None:
    """Compile on an assigned functional CPU and sample only in an exclusive timing window."""
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--compiler',type=Path,required=True)
    parser.add_argument('--native-interfaces',type=Path)
    parser.add_argument('--before-source',type=Path)
    parser.add_argument('--kinds',nargs='+',choices=['index','last','count','pipeline'],default=['index','last','count','pipeline'])
    parser.add_argument('--patterns',nargs='+',choices=['sparse','dense'],default=['sparse','dense'])
    parser.add_argument('--sizes',nargs='+',type=int,choices=[17,4097,65537],default=[17,4097,65537])
    parser.add_argument('--module-source',type=Path,default=ROOT/'seed/prelude/Data/Bytes/Search.cinch')
    parser.add_argument('--artifacts',type=Path,required=True)
    parser.add_argument('--output',type=Path,required=True)
    parser.add_argument('--build-only',action='store_true')
    parser.add_argument('--measure-only',action='store_true')
    parser.add_argument('--samples',type=int,default=5)
    parser.add_argument('--cpu',type=int,default=0)
    args=parser.parse_args()
    packed=args.module_source.read_text();baseline=scalar_source(packed)
    actual_compiler=args.compiler.parent/'cinch' if args.native_interfaces else args.compiler
    report={'compiler_sha256':digest(actual_compiler),'frontend_sha256':digest(args.compiler),'native_interfaces_sha256':digest(args.native_interfaces) if args.native_interfaces else None,'source_sha256':hashlib.sha256(packed.encode()).hexdigest(),'baseline_sha256':hashlib.sha256(baseline.encode()).hexdigest(),'driver_sha256':hashlib.sha256(SOURCE.encode()).hexdigest(),'runner_sha256':digest(Path(__file__)),'cpu':args.cpu,'opt_level':2,'measurement_scope':'Checked runtime input mutation and complete public queries; input setup/digest before start, owner use and teardown after end.','executables':{},'results':[]}
    executables={}
    variants=[('scalar',baseline)]+([('initial',args.before_source.read_text())] if args.before_source else [])+[('packed',packed)]
    for version,module in variants:
        project=args.artifacts/version;project.mkdir(parents=True,exist_ok=True);entry=project/'Main.cinch';entry.write_text(SOURCE)
        for name,path in source_closure(SOURCE).items():(project/(name+'.cinch')).write_bytes(path.read_bytes())
        (project/'Data.Bytes.Search.cinch').write_text(module);exe=(project/'consumer').resolve();executables[version]=exe
        if not args.measure_only:
            if args.native_interfaces:
                dependencies=source_closure(SOURCE)
                bundle=project/'Bundle.cinch'
                bundle.write_text('\n'.join(module if name=='Data.Bytes.Search' else path.read_text() for name,path in reversed(list(dependencies.items())))+'\n'+SOURCE)
                command=[sys.executable,str(args.compiler.resolve()),'--rc-debug','--compile-only','--interfaces',str(args.native_interfaces.resolve()),str(bundle.resolve()),str(exe.with_suffix('.ll')),str(exe),str(exe.with_suffix('.link.txt'))]
                try:run(command)
                except RuntimeError as error:
                    diagnostics=exe.with_suffix('.ll.diagnostics');raise RuntimeError(str(error)+'\n'+(diagnostics.read_text() if diagnostics.exists() else '')) from error
                objects=json.loads((args.compiler.parent/'objects.json').read_text())
                runtime='/workspace/dasl-stdlib-io/build/bootstrap/library-provider/4d80b86fac573a2dfefdb6a7a9c0970a979a28b31ad1b77553b8ff7dbbb897ea/libcinchruntime.a'
                run([os.environ.get('CINCH_CLANG','clang'),'-O2',str(exe.with_suffix('.ll')),objects['object'],runtime,'-o',str(exe),'-ldl','-lpthread','-lm'])
            else:run([str(args.compiler.resolve()),'build',str(entry.resolve()),'-o',str(exe),'--opt-level','2','--rc-debug'])
        report['executables'][version]={'sha256':digest(exe),'source_sha256':digest(project/'Data.Bytes.Search.cinch'),'ir_sha256':digest(exe.with_suffix('.ll'))}
    args.output.parent.mkdir(parents=True,exist_ok=True)
    if args.build_only:args.output.write_text(json.dumps(report,indent=2)+'\n');return
    env={**os.environ,'CINCH_RC_DEBUG':'0','CINCH_LEAK_CHECK':'1','CINCH_HEAP_LIMIT':'32M','CINCH_STACK_SIZE':'8M'}
    for kind in args.kinds:
      for pattern in args.patterns:
       for size in args.sizes:
        iterations=2000 if pattern=="dense" and kind in ["index","last"] else {17:2000,4097:100,65537:8}[size]
        checksum,wire,kept=expected(kind,pattern=='dense',size,iterations);row={'kind':kind,'pattern':pattern,'bytes':size,'iterations':iterations,'checksum':checksum,'input_sha256':wire,'retained':kept,'samples':{version:[] for version in executables}}
        for sample in range(args.samples):
          for version in (list(executables) if sample%2==0 else list(reversed(executables))):
            result=run(['taskset','-c',str(args.cpu),str(executables[version]),kind,pattern,str(size),str(iterations)],env=env)
            assert 'leaked=0' in result.stderr,result.stderr
            elapsed,observed,digest_value,retained=result.stdout.split();assert (int(observed),digest_value,int(retained))==(checksum,wire,kept),(row,result.stdout)
            row['samples'][version].append(int(elapsed))
        row['median_ns']={version:statistics.median(values) for version,values in row['samples'].items()};row['speedup']=row['median_ns']['scalar']/row['median_ns']['packed']
        if 'initial' in row['median_ns']:row['speedup_initial']=row['median_ns']['initial']/row['median_ns']['packed']
        report['results'].append(row)
        print(f'{kind} {pattern} {size}: {row["speedup"]:.3f}x, independent observations pass',flush=True)
    args.output.write_text(json.dumps(report,indent=2)+'\n')


if __name__=='__main__':main()
