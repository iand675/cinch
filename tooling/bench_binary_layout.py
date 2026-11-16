#!/usr/bin/env python3
"""Build and measure paired public typed-array pipelines against a retained scalar source baseline."""
from __future__ import annotations
import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil
import statistics
import subprocess
from bench_stdlib import source_closure

ROOT=Path(__file__).resolve().parents[1]
SOURCE=r'''-- | Public fixed-array encode and decode pipeline timing with observable exact bits.
module Main where
  import Text.Binary.Layout as B
  import Text.Hex
  import Data.Bool
  import Data.Bytes
  import Data.Char
  import Data.List
  import Data.Machine
  import Data.Nat
  import Data.Option
  import Data.Result
  import Data.String
  import Effect.Clock
  import Effect.Process
  -- | Parse validated harness arguments outside the measured interval.
  decimal :: String -> Nat
  decimal text = decimal-from (unpack text) 0
  -- | Accumulate one nonnegative decimal field.
  decimal-from :: List Char -> Nat -> Nat
  decimal-from chars n = case chars of Nil -> n; Cons first rest -> case digit-value first of None -> 0; Some digit -> let next = n * 10 + digit in decimal-from rest next
  -- | Convert the bounded benchmark position into its exact low word.
  index-word :: Nat -> U32
  index-word i = if i <= 4294967295 then wrapping-i64-to-u32 (nat-to-i64 i) else 0
  -- | Construct independent arithmetic patterns outside the measured interval.
  values :: Nat -> Nat -> List U32 -> List U32
  values left i reversed = if left == 0 then reverse reversed else
    let bits = wrapping-mul-u32 (index-word i) 2654435761 in
    let out = Cons bits reversed in let remaining = left - 1 in let next = i + 1 in values remaining next out
  -- | Consume all decoded payload bits with a bounded word accumulator.
  sum-values :: List U32 -> U64 -> U64
  sum-values input acc = case input of Nil -> acc; Cons first rest -> let next = wrapping-add-u64 acc (wrapping-u32-to-u64 first) in sum-values rest next
  -- | Preserve changed runtime values across repeated operations.
  replace-head :: List U32 -> U32 -> List U32
  replace-head input salt = case input of Nil -> Nil; Cons first rest -> Cons (bit-xor-u32 first salt) rest
  -- | Exercise the actual public encoder and optional complete public decoder.
  operation :: Bool -> B.BinaryLayout (List U32) -> List U32 -> Nat -> Nat
  operation roundtrip layout input i =
    let salt = index-word i in let changed = replace-head input salt in
    case B.binary-encode layout B.BinaryBigEndian changed of
      Err error -> 1000000000
      Ok encoded -> if roundtrip then
        (case B.binary-decode layout B.BinaryBigEndian encoded of
          Err error -> 1000000000
          Ok (decoded, used) -> u32-to-nat (wrapping-u64-to-u32 (sum-values decoded 0)) + used)
        else if 4 <= bytes-len encoded then
          case B.binary-read-u32 B.BinaryBigEndian encoded 0 of Ok value -> u32-to-nat value + bytes-len encoded; Err error -> 1000000000
        else bytes-len encoded
  -- | Sequence named observations before the measured tail call.
  repeat :: Bool -> B.BinaryLayout (List U32) -> List U32 -> Nat -> Nat -> Nat -> Nat
  repeat roundtrip layout input remaining i total = if remaining == 0 then total else
    let observed = operation roundtrip layout input i in let next-total = nat-rem (total + observed) 1000000007 in
    let left = remaining - 1 in let next = i + 1 in repeat roundtrip layout input left next next-total
  -- | Initialize inputs before timing and print exact observable output bytes plus duration and checksum.
  measure :: forall r. using (io :: Io r) => Bool -> Nat -> Nat ->{IO r} Int
  measure roundtrip count iterations =
    let input = values count 0 Nil in let layout = B.binary-array count B.binary-u32 in
    let digest = case B.binary-encode layout B.BinaryBigEndian input of Ok out -> hex-encode (sha256-bytes out); Err error -> "" in
    let started = clock-monotonic-ns () in let checksum = repeat roundtrip layout input iterations 0 0 in
    let ended = clock-monotonic-ns () in
    let printed = ioWrite (string-to-bytes (string-concat (nat-to-string (saturating-sub ended started)) (string-concat " " (string-concat (nat-to-string checksum) (string-concat " " (string-concat digest "\n")))))) in
    if string-length digest == 64 then 0 else 1
  -- | Resolve validated workload and numeric arguments before the clock boundary.
  main :: Unit ->{IO r} Int
  main unit = with (io = newIO ()) in case process-args () of
    Ok (Cons mode (Cons count (Cons iterations Nil))) -> measure (mode == "roundtrip") (decimal count) (decimal iterations)
    other -> 2
'''


def run(command: list[str], **kwargs) -> subprocess.CompletedProcess:
    """Preserve complete compilation failures and halt dependent measurements."""
    result=subprocess.run(command,capture_output=True,text=True,**kwargs)
    if result.returncode:raise RuntimeError(f'{command}\n{result.stdout}\n{result.stderr}')
    return result


def digest(path:Path)->str:
    """Identify exact compiler, source and executable bytes for reproducible paired results."""
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main()->None:
    """Prepare paired artifacts on a functional CPU; measure only with a granted benchmark CPU."""
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--compiler',type=Path,required=True)
    parser.add_argument('--artifacts',type=Path,required=True)
    parser.add_argument('--build-only',action='store_true')
    parser.add_argument('--measure-only',action='store_true')
    parser.add_argument('--samples',type=int,default=5)
    parser.add_argument('--cpu',type=int,default=0)
    parser.add_argument('--output',type=Path,required=True)
    args=parser.parse_args()
    source=ROOT/'compiler/prelude/Text/Binary/Layout.cinch'
    scalar=source.read_text().replace('ScalarAtom width pack unpack -> put-atom-elements width pack order output start values','ScalarAtom width pack unpack -> put-elements size put order output start values').replace('ScalarAtom width pack unpack -> get-atom-elements count width unpack order input start Nil','ScalarAtom width pack unpack -> get-elements count size get order input start Nil')
    report={'compiler_sha256':digest(args.compiler),'driver_sha256':hashlib.sha256(SOURCE.encode()).hexdigest(),'source_sha256':digest(source),'baseline_sha256':hashlib.sha256(scalar.encode()).hexdigest(),'optimization':2,'cpu':args.cpu,'results':[],'executables':{}}
    executables={}
    for version in ['scalar','packed']:
        work=args.artifacts/version;work.mkdir(parents=True,exist_ok=True)
        (work/'Main.cinch').write_text(SOURCE)
        deps=source_closure(SOURCE)
        for name,path in deps.items():
            (work/(name+'.cinch')).write_text(scalar if name=='Text.Binary.Layout' and version=='scalar' else path.read_text())
        exe=(work/'consumer').resolve();executables[version]=exe
        if not args.measure_only:run([str(args.compiler.resolve()),'build',str((work/'Main.cinch').resolve()),'-o',str(exe),'--opt-level','2','--rc-debug'])
        report['executables'][version]=digest(exe)
    if args.build_only:
        args.output.write_text(json.dumps(report,indent=2)+'\n');return
    env={**os.environ,'CINCH_RC_DEBUG':'0','CINCH_LEAK_CHECK':'1','CINCH_HEAP_LIMIT':'32M','CINCH_STACK_SIZE':'8M'}
    for mode in ['encode','roundtrip']:
        for count,iterations in [(17,10000),(1024,400),(16385,24)]:
            observed={'workload':mode,'count':count,'iterations':iterations,'samples':{'scalar':[],'packed':[]}}
            def sample(version:str)->int:
                """Validate each complete sample against independent bytes and retain its raw duration."""
                result=run(['taskset','-c',str(args.cpu),str(executables[version]),mode,str(count),str(iterations)],env=env)
                assert 'leaked=0' in result.stderr,result.stderr
                ns,checksum,wire_digest=result.stdout.split();ns=int(ns);checksum=int(checksum)
                expected_digest=hashlib.sha256(b''.join(((i*2654435761)&0xffffffff).to_bytes(4,'big') for i in range(count))).hexdigest()
                assert wire_digest==expected_digest,(wire_digest,expected_digest)
                observed['wire_sha256']=wire_digest
                observed['samples'][version].append(ns);return checksum
            expected=[]
            for i in range(args.samples):
                for version in (['scalar','packed'] if i%2==0 else ['packed','scalar']):expected.append(sample(version))
            assert len(set(expected))==1,expected
            # Independent integer-only expected observation of each actual repeated input.
            modulus=1<<32
            total=sum((i*2654435761)%modulus for i in range(count))
            reference=sum((((total if mode=='roundtrip' else 0)+i)%modulus+count*4) for i in range(iterations))%1000000007
            assert expected[0]==reference,(expected[0],reference)
            observed['checksum']=reference
            observed['median_ns']={k:statistics.median(v) for k,v in observed['samples'].items()}
            observed['speedup']=observed['median_ns']['scalar']/observed['median_ns']['packed']
            report['results'].append(observed)
            print(f'{mode} {count}: {observed["speedup"]:.3f}x, independent checksum {reference}',flush=True)
    args.output.write_text(json.dumps(report,indent=2)+'\n')

if __name__=='__main__':main()
