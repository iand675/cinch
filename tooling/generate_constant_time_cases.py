"""Build bounded public consumers from independent scalar and complete-buffer observations."""
import argparse,json
from pathlib import Path

# Every helper is part of the test consumer; all production calls remain public.
HELPERS=r'''-- | Observe public constant-time primitives with independently captured results.
module Main where
  import Crypto.ConstantTime
  import Data.Bytes
  import Data.Machine
  import Data.List
  import Data.Bool
  import Data.Nat
  import Effect.Process
  -- | Combine complete observations while keeping each group bounded.
  all :: List Bool -> Bool
  all values = case values of Nil -> True; Cons first rest -> first && all rest
  -- | Fill one prepared input from its public size and deterministic salt.
  fill :: Bytes -> Nat -> Nat -> Unit
  fill buffer position salt = if position < bytes-len buffer then
    let value = (position * 73 + salt * 37 + position / 7) % 256 in
    let written = if value <= 255 then bytes-set buffer position value else () in fill buffer (position + 1) salt
    else ()
  -- | Allocate reproducible input without using an operation under test.
  pattern :: Nat -> Nat -> Bytes
  pattern size salt = let buffer = bytes-new size in let filled = fill buffer 0 salt in buffer
  -- | Hash every output byte in order, including the unchanged destination tail.
  fingerprint-from :: Bytes -> Nat -> U64 -> U64
  fingerprint-from buffer position hash = if position < bytes-len buffer then
    let byte = wrapping-i64-to-u64 (nat-to-i64 (bytes-get buffer position)) in
    fingerprint-from buffer (position + 1) (wrapping-mul-u64 (bit-xor-u64 hash byte) 1099511628211)
    else hash
  -- | Observe the complete output with a fixed independent rolling hash.
  fingerprint :: Bytes -> U64
  fingerprint buffer = fingerprint-from buffer 0 14695981039346656037
  -- | Keep mismatch construction outside the comparison under test.
  compare-case :: Nat -> Nat -> Nat -> Bool -> Bool
  compare-case size other changed expected =
    let left = pattern size 1 in let right = pattern other 1 in
    let modified = (if changed < bytes-len right then
      let old = wrapping-i64-to-u8 (nat-to-i64 (bytes-get right changed)) in
      bytes-set right changed (u8-to-nat (bit-xor-u8 old 128))
      else ()) in
    constant-bytes-equal left right == expected
  -- | Conditional copying reports exact public lengths before mutating a mismatched destination.
  copy-case :: Nat -> Nat -> Bool -> Bool -> Bool -> U64 -> Bool
  copy-case size count selected alias failed expected =
    let destination = pattern size (if alias then 1 else 2) in
    let source = if alias then destination else pattern count 1 in
    let outcome = constant-copy selected destination source in
    let status = (case outcome of
      Ok unit -> not failed
      Err (ConstantTimeLengthMismatch got needed) -> failed && got == size && needed == count
      other -> False) in
    status && fingerprint destination == expected
  -- | XOR handles either exact source alias and preserves short-destination atomicity.
  xor-case :: Nat -> Nat -> Nat -> Nat -> Bool -> Nat -> U64 -> Bool
  xor-case size left-size right-size alias failed count expected =
    let left = pattern left-size 1 in let right = pattern right-size 2 in
    let destination = if alias == 1 then left else if alias == 2 then right else pattern size 3 in
    let outcome = xor-bytes destination left right in
    let status = (case outcome of
      Ok written -> not failed && written == count
      Err (ConstantTimeShortDestination needed got) -> failed && got == size && needed == (if left-size <= right-size then left-size else right-size)
      other -> False) in
    status && fingerprint destination == expected
  -- | Exhaust every byte pair through the actual public primitive.
  byte-equalities :: U32 -> U32 -> U64 -> U64
  byte-equalities left right hash = if left < 256 then
    if right < 256 then
      let same = constant-byte-equal (wrapping-u32-to-u8 left) (wrapping-u32-to-u8 right) in
      let observed = ((if same then 1 else 0) :: U64) in
      byte-equalities left (wrapping-add-u32 right 1) (wrapping-mul-u64 (bit-xor-u64 hash observed) 1099511628211)
    else byte-equalities (wrapping-add-u32 left 1) 0 hash
    else hash
'''


def emit(vectors, width):
    """Create small runtime-selected groups without deeply nested fixture constructors."""
    checks=[f'byte-equalities 0 0 14695981039346656037 == ({vectors["byte_equality_hash"]} :: U64)']
    for left,right,result in vectors['scalars']['i32']:
        checks.append(f'constant-i32-equal ({left} :: I32) ({right} :: I32) == {str(bool(result))}')
    for flag,left,right,result in vectors['scalars']['select']:
        checks.append(f'constant-select {str(bool(flag))} ({left} :: I64) ({right} :: I64) == ({result} :: I64)')
    for left,right,result in vectors['scalars']['less_or_equal']:
        checks.append(f'constant-less-or-equal {left} {right} == {str(bool(result))}')
    for row in vectors['rows']:
        sizes=row['lengths'];failed=str(row['panicked']);alias=row['alias']
        if row['operation']=='compare':
            changed=row['changed'] if row['changed']>=0 else max(sizes)+1
            checks.append(f'compare-case {sizes[0]} {sizes[1]} {changed} {str(bool(row["result"]))}')
        elif row['operation']=='copy':checks.append(f'copy-case {sizes[0]} {sizes[1]} {str(bool(row["flag"]))} {str(alias==1)} {failed} {row["output_hash"]}')
        else:checks.append(f'xor-case {sizes[0]} {sizes[1]} {sizes[2]} {alias} {failed} {row["result"]} {row["output_hash"]}')
    text=HELPERS
    groups=[checks[i:i+width] for i in range(0,len(checks),width)]
    for index,group in enumerate(groups):
        text+=f'\n  -- | Observe complete independent result group {index}.\n  group{index} :: Unit -> Bool\n  group{index} unit = all [\n'+',\n'.join('    ('+check+')' for check in group)+'\n    ]\n'
    dispatch='False'
    for index in reversed(range(len(groups))):dispatch=f'if index == "{index}" then group{index} () else {dispatch}'
    text+='\n  -- | Select one bounded complete observation group.\n  selected :: String -> Bool\n  selected index = '+dispatch+'\n'
    text+='\n  -- | Run each group in an independent bounded process.\n  main :: Unit ->{IO r} Int\n  main unit = with (io = newIO ()) in case process-args () of\n    Ok (Cons index Nil) -> if selected index then 0 else 1\n    other -> 2\n'
    return text,{'groups':len(groups),'checks':len(checks),'scalar_byte_pairs':65536}


def main():
    """Write the actual source and its independent group inventory."""
    parser=argparse.ArgumentParser();parser.add_argument('vectors',type=Path);parser.add_argument('output',type=Path);parser.add_argument('--width',type=int,default=16);args=parser.parse_args()
    source,groups=emit(json.loads(args.vectors.read_text()),args.width);args.output.write_text(source);args.output.with_suffix('.groups.json').write_text(json.dumps(groups,indent=2)+'\n')

if __name__=='__main__':main()
