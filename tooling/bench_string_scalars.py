#!/usr/bin/env python3
"""Compare complete public String scalar-count pipelines under exact before/after source overrides."""
from __future__ import annotations
import argparse
import hashlib
import json
import os
from pathlib import Path
import statistics
from bench_stdlib import source_closure
from validate_string_construction import digest,run,verify_receipt

ROOT=Path(__file__).resolve().parents[1]
SOURCE=r'''-- | Complete immutable scalar counts with runtime inputs and postclock backing observations.
module Main where
  import Data.String
  import Data.Bytes
  import Data.Char
  import Data.List
  import Data.Nat
  import Data.Option
  import Data.Result
  import Effect.Clock
  import Effect.Process
  import Text.Hex
  -- | Parse validated nonnegative harness arguments outside timing.
  decimal :: String -> Nat
  decimal text = decimal-from (unpack text) 0
  -- | Accumulate one exact decimal value without recursive return frames.
  decimal-from :: List Char -> Nat -> Nat
  decimal-from chars value = case chars of
    Nil -> value
    Cons first rest -> case digit-value first of
      None -> 0
      Some digit -> let next = value * 10 + digit in decimal-from rest next
  -- | Select valid UTF-8 patterns, including embedded NUL, outside timing.
  pattern :: String -> String
  pattern name = if name == "ascii" then "x" else if name == "two" then "é"
    else if name == "four" then "🙂" else if name == "nul" then "a\u{0}é🙂" else "é界🙂"
  -- | Consume public counts of alternating complete immutable inputs on every iteration.
  work :: String -> String -> Nat -> Nat -> Nat -> Nat
  work first second left i total = if left == 0 then total else
    let count = string-length (if nat-rem i 2 == 0 then first else second) in
    let next-total = nat-rem (total + count) 1000000007 in
    let remaining = left - 1 in let next = i + 1 in work first second remaining next next-total
  -- | Keep actual backing reads and copied digest observations after the ending clock.
  retained :: String -> String
  retained text = hex-encode (sha256-bytes (string-to-bytes text))
  -- | Time public counting after materialization and before final owner reads and teardown.
  measure :: forall r. using (io :: Io r) => String -> Nat -> Nat ->{IO r} Int
  measure unit count iterations =
    let first = string-repeat count unit in let second = string-concat first unit in
    let original = hex-encode (sha256-bytes (string-to-bytes first)) in
    let start = clock-monotonic-ns () in let checksum = work first second iterations 0 0 in
    let end = clock-monotonic-ns () in let kept = retained second in
    let output = string-concat-all [nat-to-string (saturating-sub end start)," ",nat-to-string checksum," ",original," ",kept,"\n"] in
    let printed = ioWrite (string-to-bytes output) in 0
  -- | Resolve runtime materialization sizes and patterns before all clock readings.
  main :: Unit ->{IO r} Int
  main unit = with (io = newIO ()) in case process-args () of
    Ok (Cons name (Cons count (Cons iterations Nil))) -> measure (pattern name) (decimal count) (decimal iterations)
    other -> 2
'''
PATTERNS={'ascii':'x','two':'é','four':'🙂','mixed':'é界🙂','nul':'a\0é🙂'}


def expected(name:str,count:int,iterations:int)->tuple[int,str,str]:
    """Compute complete alternating scalar counts and both independent wire digests."""
    unit=PATTERNS[name];checksum=sum((count+i%2)*len(unit) for i in range(iterations))%1000000007
    return checksum,hashlib.sha256((unit*count).encode()).hexdigest(),hashlib.sha256((unit*(count+1)).encode()).hexdigest()


def main()->None:
    """Compile on a granted functional CPU, then measure only in a separate exclusive timing window."""
    p=argparse.ArgumentParser(description=__doc__);p.add_argument('--compiler',type=Path,required=True);p.add_argument('--module-source',type=Path,required=True);p.add_argument('--baseline-source',type=Path,required=True);p.add_argument('--artifacts',type=Path,required=True);p.add_argument('--output',type=Path,required=True);p.add_argument('--build-only',action='store_true');p.add_argument('--measure-only',action='store_true');p.add_argument('--samples',type=int,default=5);p.add_argument('--cpu',type=int,default=0);args=p.parse_args()
    report={'compiler_sha256':digest(args.compiler),'source_sha256':digest(args.module_source),'baseline_sha256':digest(args.baseline_source),'driver_sha256':hashlib.sha256(SOURCE.encode()).hexdigest(),'runner_sha256':digest(Path(__file__)),'opt_level':2,'cpu':args.cpu,'epoch':'exact source overrides in finalcad70; no fresh embedded/Native claim','measurement_scope':'Alternating complete public scalar counts; materialization/input digest before clock, backing-copy digest and input release after ending clock.','executables':{},'results':[]};executables={}
    for version,module in [('before',args.baseline_source),('after',args.module_source)]:
        project=args.artifacts/version;project.mkdir(parents=True,exist_ok=True);entry=project/'Main.cinch';entry.write_text(SOURCE)
        for name,path in source_closure(SOURCE).items(): (project/(name+'.cinch')).write_bytes(path.read_bytes())
        selected=project/'Data.String.cinch';selected.write_text('-- Explicit sealed String scalar benchmark receipt v1.\n'+module.read_text());exe=(project/'consumer').resolve();executables[version]=exe
        if not args.measure_only:run([str(args.compiler.resolve()),'build',str(entry.resolve()),'-o',str(exe),'--opt-level','2','--rc-debug'])
        report['executables'][version]={'sha256':digest(exe),'llvm_sha256':digest(exe.with_suffix('.ll')),'epoch_receipt':verify_receipt(project,selected)}
    args.output.parent.mkdir(parents=True,exist_ok=True)
    if args.build_only:args.output.write_text(json.dumps(report,indent=2)+'\n');return
    env={**os.environ,'CINCH_RC_DEBUG':'0','CINCH_HEAP_LIMIT':'32M','CINCH_STACK_SIZE':'8M','CINCH_LEAK_CHECK':'1'}
    for name in PATTERNS:
      for count,iterations in [(17,1000),(4097,20),(65537,2)]:
        checksum,first,second=expected(name,count,iterations);row={'pattern':name,'count':count,'iterations':iterations,'checksum':checksum,'input_sha256':first,'retained_sha256':second,'samples':{'before':[],'after':[]}}
        for sample in range(args.samples):
          for version in (['before','after'] if sample%2==0 else ['after','before']):
            result=run(['taskset','-c',str(args.cpu),str(executables[version]),name,str(count),str(iterations)],env=env);assert 'leaked=0' in result.stderr,result.stderr
            elapsed,observed,a,b=result.stdout.split();assert(int(observed),a,b)==(checksum,first,second),(row,result.stdout);row['samples'][version].append(int(elapsed))
        row['median_ns']={version:statistics.median(values) for version,values in row['samples'].items()};row['speedup']=row['median_ns']['before']/row['median_ns']['after'];report['results'].append(row);print(name,count,f'{row["speedup"]:.3f}x independent full-wire observations pass',flush=True)
    args.output.write_text(json.dumps(report,indent=2)+'\n')


if __name__=='__main__':main()
