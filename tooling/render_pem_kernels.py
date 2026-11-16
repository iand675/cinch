#!/usr/bin/env python3
"""Isolate actual PEM read-only primitives for exhaustive host checks and portable object gates."""
from __future__ import annotations
from pathlib import Path
import re

ROOT=Path(__file__).resolve().parents[1]
NAMES=['before','plus','minus','address','has-one','has-sixteen','begin-token','end-token','blank-sixteen','blank-one']


def declaration(source: str, name: str) -> str:
    """Retain exact owned implementations, including their explicit unsafe read-only LLVM annotation."""
    match=re.search(r'^  (?:prim )?'+re.escape(name)+r' ::',source,re.M)
    if match is None: raise ValueError('missing maintained kernel '+name)
    end=source.find('\n  -- |',match.start())
    return source[match.start():end if end>=0 else len(source)].rstrip()+'\n'


def prefix() -> str:
    """Copy actual private code without exporting pointers from the maintained package."""
    source=(ROOT/'compiler/prelude/Text/Pem.cinch').read_text()
    output='-- | Actual PEM byte kernels with checked caller-owned test buffers.\nmodule Main where\n  import Data.Bytes\n  import Data.Bool\n  import Data.Machine\n  import Data.Nat\n  import Data.String\n\n'
    for name in NAMES+['hold','natural','find-machine','blank-machine']:
        output+='  -- | Actual maintained private arithmetic, exact source read or bounded scanner.\n'+declaration(source,name)+'\n'
    return output


def rendered(foreign: bool=False) -> str:
    """Retain observable unknown-input reads in portable objects; exhaust byte/lane mutations on hosts."""
    if foreign:
        return prefix()+'''  -- | Give each independent predicate a distinct observable output bit.
  flag :: Bool -> U64 -> U64
  flag value mask = if value then mask else 0

  -- | Retain both scanners and both marker predicates without an impossible boolean conjunction.
  main :: Int
  main = let source = bytes-new 32 in let base = bytes-data source in
    let found = hold source (find-machine base 0 32 45) in
    let blank = hold source (blank-machine base 0 32) in
    let begin = hold source (begin-token base) in let end = hold source (end-token base) in
    let a = flag begin 1 in let b = flag end 2 in let flags = bit-or-u64 a b in
    let count = plus found blank in let score = plus count flags in nat-to-int (natural score)
'''
    return prefix()+'''  -- | Check every byte substitution at every byte position of a complete marker.
  marker-values :: Bytes -> Nat -> Nat -> Nat -> Bool -> Bool
  marker-values source at original value begin = if value < 256 then
    if at < bytes-len source && (if begin then bytes-len source == 11 else bytes-len source == 10) then let changed = bytes-set source at value in let base = bytes-data source in
      let actual = hold source (if begin then begin-token base else end-token base) in
      (if value == original then actual else not actual) && marker-values source at original (value + 1) begin
    else False
    else True

  -- | Restore each marker position before moving to the next independent mutation lane.
  marker-lanes :: Bytes -> Nat -> Bool -> Bool
  marker-lanes source at begin = if at < bytes-len source then
    let original = bytes-get source at in let checked = marker-values source at original 0 begin in
    let restored = bytes-set source at original in checked && marker-lanes source (at + 1) begin
    else True

  -- | Keep wanted-byte parameters dynamic, including both high-bit boundary values.
  wanted-byte :: U32 -> U8
  wanted-byte index = case index of 0 -> 0; 1 -> 9; 2 -> 10; 3 -> 32; 4 -> 45; 5 -> 58; 6 -> 128; other -> 255

  -- | Verify one-byte and sixteen-byte matches against independent known buffer contents.
  wanted-values :: Bytes -> Nat -> U8 -> Nat -> Bool
  wanted-values source lane wanted value = if value < 256 then
    if bytes-len source >= 16 && lane < bytes-len source then let changed = bytes-set source lane value in
      let base = bytes-data source in let wide = nat-to-i64 lane in let offset = wrapping-i64-to-u64 wide in
      let pointer = address base offset in let single = hold source (has-one pointer wanted) in
      let whole = hold source (has-sixteen base wanted) in let expected = u8-to-nat wanted in
      (if value == expected then single else not single)
        && (if expected == 0 || value == expected then whole else not whole)
        && wanted-values source lane wanted (value + 1)
    else False
    else True

  -- | Reset every mutated source lane before advancing across complete SIMD alignments.
  wanted-lanes :: Bytes -> Nat -> U8 -> Bool
  wanted-lanes source lane wanted = if lane < 16 then
    let checked = wanted-values source lane wanted 0 in
    if lane < bytes-len source then let restored = bytes-set source lane 0 in checked && wanted-lanes source (lane + 1) wanted else False
    else True

  -- | Check all lane/byte combinations for space/tab filtering independently of encoder behavior.
  blank-values :: Bytes -> Nat -> Nat -> Bool
  blank-values source lane value = if value < 256 then
    if bytes-len source >= 16 && lane < bytes-len source then let changed = bytes-set source lane value in
      let base = bytes-data source in let wide = nat-to-i64 lane in let offset = wrapping-i64-to-u64 wide in
      let pointer = address base offset in let single = hold source (blank-one pointer) in
      let whole = hold source (blank-sixteen base) in
      (if value == 9 || value == 32 then single && whole else not single && not whole)
        && blank-values source lane (value + 1)
    else False
    else True

  -- | Restore every filtered lane and retain all sixteen vector positions.
  blank-lanes :: Bytes -> Nat -> Bool
  blank-lanes source lane = if lane < 16 then
    let checked = blank-values source lane 0 in
    if lane < bytes-len source then let restored = bytes-set source lane 0 in checked && blank-lanes source (lane + 1) else False
    else True

  -- | Traverse every requested byte class without a giant refined literal fixture.
  wanted-all :: Bytes -> U32 -> Bool
  wanted-all source index = if lt-u32 index 8 then let wanted = wanted-byte index in
    wanted-lanes source 0 wanted && wanted-all source (wrapping-add-u32 index 1)
    else True

  -- | Check unsigned arithmetic and all exhaustive read-only marker/vector predicates.
  main :: Int
  main = let source = bytes-new 16 in
    let sum = plus 18446744073709551615 1 in let difference = minus 18446744073709551615 18446744073709551615 in
    let pointer = address 7 9 in
    if eq-u64 sum 0 && eq-u64 difference 0 && eq-i64 pointer 16
      && before 0 18446744073709551615 && not (before 18446744073709551615 0)
      && marker-lanes (string-to-bytes "-----BEGIN ") 0 True
      && marker-lanes (string-to-bytes "\\n-----END ") 0 False
      && wanted-all source 0 && blank-lanes source 0 then 0 else 1
'''


if __name__=='__main__':print(rendered(),end='')
