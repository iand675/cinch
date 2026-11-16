#!/usr/bin/env python3
"""Build and measure explicit-source before/after immutable string construction pipelines."""
from __future__ import annotations
import argparse
import hashlib
import json
import os
from pathlib import Path
import statistics
import subprocess
from bench_stdlib import source_closure
from validate_string_construction import verify_receipt

ROOT=Path(__file__).resolve().parents[1]
SOURCE=r'''-- | Public immutable-string materialization with opaque runtime inputs and observable bytes.
module Main where
  import Data.String
  import Data.Bytes
  import Data.List
  import Data.Char
  import Data.Nat
  import Data.Option
  import Data.Result
  import Effect.Clock
  import Effect.Process
  import Text.Hex
  -- | Parse validated nonnegative harness fields before timing.
  decimal :: String -> Nat
  decimal text = decimal-from (unpack text) 0
  -- | Accumulate one exact decimal natural.
  decimal-from :: List Char -> Nat -> Nat
  decimal-from text n = case text of Nil -> n; Cons c rest -> case digit-value c of None -> 0; Some digit -> let next = n * 10 + digit in decimal-from rest next
  -- | Build a deep piece list outside the measured interval without another construction helper.
  pieces :: Nat -> String -> List String -> List String
  pieces left text reversed = if left == 0 then reverse reversed else
    let values = Cons text reversed in let remaining = left - 1 in pieces remaining text values
  -- | Observe bytes from all parts of a fully materialized result rather than only its known length.
  fingerprint :: Bytes -> Nat
  fingerprint bytes = let n = bytes-len bytes in if n > 0 then
    let last = n - 1 in let middle = nat-div n 2 in
    if last < bytes-len bytes && middle < bytes-len bytes then
      n + bytes-get bytes 0 * 257 + bytes-get bytes middle * 65537 + bytes-get bytes last * 16777217
    else 0
    else 0
  -- | Select one changed complete public construction without hiding a loop in a primitive.
  construct :: Bool -> String -> String -> List String -> Nat -> Nat -> String
  construct repeat text doubled values count salt = if repeat then string-repeat (count + salt) text
    else case values of Nil -> empty-string; Cons first rest -> string-concat-all (Cons (if salt == 0 then text else doubled) rest)
  -- | Materialize a changed public result on each iteration and consume its copied bytes.
  operation :: Bool -> String -> String -> List String -> Nat -> Nat -> Nat
  operation repeat text doubled values count i =
    let salt = nat-rem i 2 in let result = construct repeat text doubled values count salt in
    fingerprint (string-to-bytes result)
  -- | Sequence primitive observations before the ordinary measured tail loop.
  measure-loop :: Bool -> String -> String -> List String -> Nat -> Nat -> Nat -> Nat -> Nat
  measure-loop repeat text doubled values count left i total = if left == 0 then total else
    let value = operation repeat text doubled values count i in let next-total = nat-rem (total + value) 1000000007 in
    let remaining = left - 1 in let next = i + 1 in measure-loop repeat text doubled values count remaining next next-total
  -- | Observe every retained input piece after the clock so last-owner teardown stays outside timing.
  retained-size :: List String -> Nat -> Nat
  retained-size values total = case values of
    Nil -> total
    Cons first rest -> let next = total + string-byte-length first in retained-size rest next
  -- | Time complete public construction and output-copy pipelines after all inputs are prepared.
  measure :: forall r. using (io :: Io r) => Bool -> String -> Nat -> Nat ->{IO r} Int
  measure repeat text count iterations =
    let doubled = string-concat text text in let values = if repeat then Nil else pieces count text Nil in
    let original = if repeat then string-repeat count text else string-concat-all values in
    let digest = hex-encode (sha256-bytes (string-to-bytes original)) in
    let start = clock-monotonic-ns () in let checksum = measure-loop repeat text doubled values count iterations 0 0 in
    let end = clock-monotonic-ns () in
    let retained = retained-size values 0 in
    let valid = retained == (if repeat then 0 else count * string-byte-length text) && string-byte-length doubled == 2 * string-byte-length text in
    let printed = ioWrite (string-to-bytes (string-concat (nat-to-string (saturating-sub end start)) (string-concat " " (string-concat (nat-to-string checksum) (string-concat " " (string-concat digest "\n")))))) in if valid then 0 else 1
  -- | Resolve validated harness arguments before the clock boundary.
  main :: Unit ->{IO r} Int
  main unit = with (io = newIO ()) in case process-args () of
    Ok (Cons mode (Cons text (Cons count (Cons iterations Nil)))) -> measure (mode == "repeat") text (decimal count) (decimal iterations)
    other -> 2
'''


def digest(path:Path)->str:
    """Identify the exact compiler, selected implementation and executable bytes."""
    return hashlib.sha256(path.read_bytes()).hexdigest()


def run(command:list[str],**kwargs)->subprocess.CompletedProcess:
    """Preserve failures and stop dependent sampling on compile, ownership or execution error."""
    result=subprocess.run(command,capture_output=True,text=True,**kwargs)
    if result.returncode:raise RuntimeError(f'{command}\n{result.stdout}\n{result.stderr}')
    return result


def fingerprint(raw:bytes)->int:
    """Compute the independent public output observation using exact wire bytes."""
    if not raw:return 0
    return len(raw)+raw[0]*257+raw[len(raw)//2]*65537+raw[-1]*16777217


def main()->None:
    """Prepare artifacts on a functional CPU and sample only during a granted benchmark window."""
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--compiler',type=Path,required=True)
    parser.add_argument('--module-source',type=Path,required=True)
    parser.add_argument('--baseline-source',type=Path,required=True)
    parser.add_argument('--artifacts',type=Path,required=True)
    parser.add_argument('--output',type=Path,required=True)
    parser.add_argument('--build-only',action='store_true')
    parser.add_argument('--measure-only',action='store_true')
    parser.add_argument('--samples',type=int,default=5)
    parser.add_argument('--cpu',type=int,default=0)
    args=parser.parse_args()
    report={'compiler_sha256':digest(args.compiler),'source_sha256':digest(args.module_source),'baseline_sha256':digest(args.baseline_source),'driver_sha256':hashlib.sha256(SOURCE.encode()).hexdigest(),'runner_sha256':digest(Path(__file__)),'epoch':'explicit Data.String source override in both versions','fresh_native_base_validation_claimed':False,'optimization':2,'cpu':args.cpu,'measurement_scope':'Constructor plus output copy; input setup before start and observable input traversal/teardown after end clock','executables':{},'results':[]}
    executables={}
    for version,module in [('before',args.baseline_source),('after',args.module_source)]:
        project=args.artifacts/version;project.mkdir(parents=True,exist_ok=True)
        entry=project/'Main.cinch';entry.write_text(SOURCE)
        for name,path in source_closure(SOURCE).items():(project/(name+'.cinch')).write_bytes(path.read_bytes())
        selected=project/'Data.String.cinch';selected.write_text('-- Explicit constructor source-override receipt v1.\n'+module.read_text())
        exe=(project/'consumer').resolve();executables[version]=exe
        if not args.measure_only:run([str(args.compiler.resolve()),'build',str(entry.resolve()),'-o',str(exe),'--opt-level','2','--rc-debug'])
        report['executables'][version]={'sha256':digest(exe),'epoch_receipt':verify_receipt(project,selected)}
    args.output.parent.mkdir(parents=True,exist_ok=True)
    if args.build_only:args.output.write_text(json.dumps(report,indent=2)+'\n');return
    env={**os.environ,'CINCH_RC_DEBUG':'0','CINCH_LEAK_CHECK':'1','CINCH_HEAP_LIMIT':'32M','CINCH_STACK_SIZE':'8M'}
    for mode in ['repeat','concat']:
        for text in ['x','éλ']:
            for count,iterations in [(17,2000),(4097,100),(65536,8)]:
                row={'workload':mode,'unit':text,'count':count,'iterations':iterations,'samples':{'before':[],'after':[]}}
                raw=text.encode();expected_digest=hashlib.sha256(raw*count).hexdigest()
                expected=sum(fingerprint(raw*(count+(i%2))) for i in range(iterations))%1000000007
                for sample in range(args.samples):
                    for version in (['before','after'] if sample%2==0 else ['after','before']):
                        result=run(['taskset','-c',str(args.cpu),str(executables[version]),mode,text,str(count),str(iterations)],env=env)
                        assert 'leaked=0' in result.stderr,result.stderr
                        ns,checksum,observed_digest=result.stdout.split()
                        assert int(checksum)==expected,(row,int(checksum),expected)
                        assert observed_digest==expected_digest,(row,observed_digest,expected_digest)
                        row['samples'][version].append(int(ns))
                row['checksum']=expected;row['wire_sha256']=expected_digest
                row['median_ns']={version:statistics.median(values) for version,values in row['samples'].items()}
                row['speedup']=row['median_ns']['before']/row['median_ns']['after'];report['results'].append(row)
                print(f'{mode} {text!r} {count}: {row["speedup"]:.3f}x, independent digest/checksum passed',flush=True)
    args.output.write_text(json.dumps(report,indent=2)+'\n')

if __name__=='__main__':main()
