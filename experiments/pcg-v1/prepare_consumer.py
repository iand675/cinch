#!/usr/bin/env python3
"""Generate an SDK-free consumer from independently frozen output and snapshot observations."""
from pathlib import Path
import argparse,json


def cons(values):
    """Spell explicit persistent lists without relying on collection-literal inference."""
    result='Nil'
    for value in reversed(values):result=f'(Cons {value} {result})'
    return result


def byte_literal(value):
    """Materialize the exact octets of an independently captured snapshot."""
    return '(bytes-from-list '+cons([str(x) for x in bytes.fromhex(value)])+')'


def main():
    """Keep every expected integer and byte unchanged while preparing the public caller."""
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('vectors',type=Path);parser.add_argument('output',type=Path)
    args=parser.parse_args();rows=json.loads(args.vectors.read_text())
    cases=[]
    for row in rows:
        cases.append(f'(Sample {row["high"]} {row["low"]} {cons([str(x) for x in row["values"]])} {byte_literal(row["before"])} {byte_literal(row["after"])} {byte_literal(row["appended"])})')
    source='''-- | Check frozen independent generator outputs, snapshots, aliases and exact refusals.
module Main where
  import Data.Random.Pcg
  import Data.Bytes
  import Data.Machine
  import Data.Nat
  import Data.Result

  -- | One independent seed, output sequence and exact wire-state observation.
  data Sample where
    -- | Preserve independent snapshot bytes separately from candidate encoding.
    Sample :: U64 -> U64 -> List U64 -> Bytes -> Bytes -> Bytes -> Sample

  -- | Require all successive machine outputs and the complete terminal state.
  check-values :: List U64 -> PcgState -> Bytes -> Bool
  check-values values state expected = case values of
    Nil -> pcg-encode state == expected
    Cons wanted rest -> case pcg-next state of
      (observed, next) -> eq-u64 observed wanted && check-values rest next expected

  -- | A malformed snapshot is a named refusal, not a replacement state.
  refused :: Bytes -> Bool
  refused bytes = case pcg-decode bytes of
    Err InvalidPcgEncoding -> True
    Ok state -> False

  -- | Preserve immutable replay and decoded ownership across buffer mutations.
  owner-round :: PcgState -> Bool
  owner-round state =
    let encoded = pcg-encode state in
    case pcg-decode encoded of
      Err error -> False
      Ok restored ->
        let changed = if 4 < bytes-len encoded then bytes-set encoded 4 255 else () in
        case pcg-next state of
          (first, next) -> case pcg-next restored of
            (same, restored-next) -> case pcg-next state of
              (replayed, replay-next) -> eq-u64 first same && eq-u64 first replayed
                && pcg-encode next == pcg-encode restored-next
                && pcg-encode next == pcg-encode replay-next
                && pcg-encode state == pcg-encode restored

  -- | Exercise each prefix refusal without allowing input mutation to change a state.
  bad-prefixes :: PcgState -> Nat -> Bool
  bad-prefixes state index = if index < 4 then
    let bytes = pcg-encode state in
    if index < bytes-len bytes then
      let changed = bytes-set bytes index 0 in refused bytes && bad-prefixes state (index + 1)
    else False
    else True

  -- | Check exact encoding, append ownership, immutable replay and resumed output.
  check-sample :: Sample -> Bool
  check-sample sample = case sample of
    Sample high low values before after appended ->
      let state = pcg-seeded high low in
      let prefix = bytes-from-list (Cons 0 (Cons 255 (Cons 128 (Cons 1 Nil)))) in
      pcg-encode state == before && owner-round state && bad-prefixes state 0
        && check-values values state after
        && (case pcg-decode before of Err error -> False; Ok decoded -> check-values values decoded after)
        && (case walk values state of final -> pcg-append prefix final == appended)
        && prefix == bytes-from-list (Cons 0 (Cons 255 (Cons 128 (Cons 1 Nil))))

  -- | Reach the final replay point without consulting the expected snapshot.
  walk :: List U64 -> PcgState -> PcgState
  walk values state = case values of
    Nil -> state
    Cons wanted rest -> case pcg-next state of
      (observed, next) -> walk rest next

  -- | Return a stable one-based case number on an output, state or ownership mismatch.
  check :: List Sample -> Int -> Int
  check samples index = case samples of
    Nil -> if refused (bytes-new 0) && refused (bytes-new 19) && refused (bytes-new 21) then 0 else 99
    Cons sample rest -> if check-sample sample then check rest (index + 1) else index

  -- | Run every independent sample through the public state API.
  main :: Int
  main = check samples 1

  -- | Frozen reference observations; expected outputs are never computed by the candidate.
  samples :: List Sample
  samples = '''+cons(cases)+'\n'
    args.output.write_text(source)


if __name__=='__main__':main()
