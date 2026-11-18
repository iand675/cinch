#!/usr/bin/env python3
"""Generate compact buffered-reader history consumers from independently observed byte-preserving traces."""
from __future__ import annotations
import argparse
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
CODES = {'none':0, 'end':1, 'buffer-full':2, 'negative-count':3, 'invalid-unread-byte':4, 'invalid-unread-rune':5, 'no-progress':6,'invalid-read-count':7}
OPS = {'size':0,'buffered':1,'peek':2,'discard':3,'read':4,'byte':5,'rune':6,'unread-byte':7,'unread-rune':8,'slice':9,'bytes':10,'string':11,'line':12,'reset-self':13,'reset':14,'write-to':15}

HEADER = '''-- | Independent buffered-reader histories check every byte, status, cursor and actual source request.
module Main where
  import Effect.Buffered.Reader
  import Effect.Buffered.Core
  import Effect.Buffered.Internal.Cell as Cell
  import Effect.Io
  import Data.Bytes
  import Data.Bytes.Build
  import Data.Bool
  import Data.Char
  import Data.Int
  import Data.List
  import Data.Machine
  import Data.Nat
  import Data.Option
  import Data.Result
  import Data.String
  import Text.Show

  -- | An operation and its exact independently observed post-operation state.
  data Check where
    -- | Operation, argument, reset input, expected bytes/count/scalar/width/prefix/error/buffered and callback widths.
    Check :: Nat -> Int -> Bytes -> Bytes -> Nat -> Nat -> Nat -> Bool -> Nat -> Nat -> List Int -> Check

  -- | One normalized result keeps raw bytes independent of Unicode scalar observations.
  data Observation where
    -- | Bytes, count, scalar code, width, prefix status and error family.
    Observed :: Bytes -> Nat -> Nat -> Nat -> Bool -> Nat -> Observation

  -- | Compare arbitrary bytes without applying a text conversion.
  same :: Bytes -> Bytes -> Bool
  same first second = eq-i64 (bytes-compare first second) 0

  -- | Decode one generator-owned lowercase hexadecimal digit.
  hex-digit :: Nat -> Nat
  hex-digit value = if value >= 97 then value - 87 else if value >= 48 then value - 48 else 0

  -- | Fill exact byte pairs from an ASCII hexadecimal fixture, with both intervals checked.
  unhex-from :: Bytes -> Bytes -> Nat -> Unit
  unhex-from input output at = if at < bytes-len output && at * 2 + 1 < bytes-len input then
    let high = hex-digit (bytes-get input (at * 2)) in let low = hex-digit (bytes-get input (at * 2 + 1)) in
    let value = high * 16 + low in
    if value <= 255 then let written = bytes-set output at value in unhex-from input output (at + 1) else ()
    else ()

  -- | Reconstruct an arbitrary byte fixture without a deeply nested numeric constructor list.
  unhex :: String -> Bytes
  unhex text = let input = string-to-bytes text in let output = bytes-new (string-byte-length text / 2) in
    let written = unhex-from input output 0 in output

  -- | Copy one checked raw source interval into a fresh event.
  part :: Bytes -> Nat -> Nat -> Bytes
  part input at count = if at + count <= bytes-len input then
    let output = bytes-new count in let copied = bytes-copy-range output input 0 at count in output
    else bytes-new 0

  -- | Normalize only structural error families; presentation strings remain native diagnostics.
  error-code :: Option BufferError -> Nat
  error-code status = case status of
    None -> 0
    Some BufferEnd -> 1
    Some (BufferFull capacity requested) -> 2
    Some (BufferNegativeCount count) -> 3
    Some BufferInvalidUnreadByte -> 4
    Some BufferInvalidUnreadRune -> 5
    Some BufferNoProgress -> 6
    Some (BufferInvalidReadCount wanted actual supplied) -> 7
    other -> 99

  -- | Preserve actual source request widths in initialized private I/O storage.
  source :: forall r. using (io :: Io r) => Bytes -> Nat -> Nat -> Bool -> Nat -> Cell.IoCell r (List Int) -> Bool -> Bool -> BufferSource r
  source input at chunk together empties calls logging fast = let plain = buffer-source (\\wanted -> source-read input at chunk together empties calls logging fast wanted) in
    if fast then buffer-source-with-transfer plain (\\write -> source-transfer input at chunk together empties calls logging write) else plain

  -- | Advance the immutable transport cursor and distinguish transient emptiness from end-of-input.
  source-read :: forall r. using (io :: Io r) => Bytes -> Nat -> Nat -> Bool -> Nat -> Cell.IoCell r (List Int) -> Bool -> Bool -> Nat ->{IO r} (ReadEvent, BufferSource r)
  source-read input at chunk together empties calls logging fast wanted =
    let logged = if logging then Cell.cell-write calls (Cons (nat-to-int wanted) (Cell.cell-read calls)) else () in
    if empties > 0 then (ReadEvent 0 (bytes-new 0) None, source input at chunk together (empties - 1) calls logging fast)
    else if wanted == 0 then (ReadEvent 0 (bytes-new 0) None, source input at chunk together 0 calls logging fast)
    else if at < bytes-len input then
      let remaining = bytes-len input - at in let limited = if remaining < wanted then remaining else wanted in
      let count = if chunk > 0 && chunk < limited then chunk else limited in
      let status = if together && at + count == bytes-len input then Some BufferEnd else None in
      (ReadEvent (nat-to-int count) (part input at count) status, source input (at + count) chunk together 0 calls logging fast)
    else (ReadEvent 0 (bytes-new 0) (Some BufferEnd), source input at chunk together 0 calls logging fast)

  -- | Expose a direct source transfer whose request log remains distinguishable from ordinary reads.
  source-transfer :: forall r. using (io :: Io r) => Bytes -> Nat -> Nat -> Bool -> Nat -> Cell.IoCell r (List Int) -> Bool -> (Bytes ->{IO r} WriteEvent) ->{IO r} (TransferEvent, BufferSource r)
  source-transfer input at chunk together empties calls logging write =
    let logged = if logging then Cell.cell-write calls (Cons (-1) (Cell.cell-read calls)) else () in
    let remaining = if at <= bytes-len input then bytes-len input - at else 0 in
    let payload = part input at remaining in
    case write payload of
      event -> case buffer-write-accepted remaining event of
        Err failure -> (TransferEvent 0 (Some failure), source input at chunk together empties calls logging True)
        Ok accepted -> case event of
          WriteEvent actual status -> if accepted <= 9223372036854775807 then
            (TransferEvent (nat-to-i64 accepted) status, source input (at + accepted) chunk together empties calls logging True)
            else (TransferEvent 0 (Some (BufferInvalidWriteCount remaining actual)), source input at chunk together empties calls logging True)

  -- | Produce an independently configured invalid source count without inventing initialized payload bytes.
  fault-source :: forall r. using (io :: Io r) => Int -> Cell.IoCell r (List Int) -> BufferSource r
  fault-source fault calls = buffer-source (\\wanted -> let logged = Cell.cell-write calls (Cons (nat-to-int wanted) (Cell.cell-read calls)) in
    let actual = if fault < 0 then (-1) else nat-to-int wanted + 1 in
    (ReadEvent actual (bytes-new 0) None, fault-source fault calls))

  -- | Compare complete callback histories rather than only the final byte result.
  same-calls :: List Int -> List Int -> Bool
  same-calls first second = case first of
    Nil -> case second of Nil -> True; other -> False
    Cons value rest -> case second of
      Cons expected tail -> value == expected && same-calls rest tail
      Nil -> False

  -- | Normalize one raw read event without losing its simultaneous terminal status.
  read-observation :: ReadEvent -> Observation
  read-observation event = case event of
    ReadEvent count payload status -> if 0 <= count then Observed payload (nat-from-int count) 0 0 False (error-code status)
      else Observed payload 0 0 0 False 99

  -- | Preserve an unread result's exact structural failure family.
  unread-observation :: Result BufferError Unit -> Observation
  unread-observation result = case result of
    Ok unit -> Observed (bytes-new 0) 0 0 0 False 0
    Err error -> Observed (bytes-new 0) 0 0 0 False (error-code (Some error))

  -- | Widen an exact conversion result to its ordinary nonnegative carrier.
  natural :: Nat -> Nat
  natural value = value

  -- | Convert fixture delimiter arguments through a checked signed-machine boundary.
  delimiter :: Int -> U8
  delimiter argument = case checked-int-to-i64 argument of
    Some value -> wrapping-i64-to-u8 value
    None -> 0

  -- | Snapshot every sink offer into independent fixture state for transfer-result comparison.
  sink :: forall r. using (io :: Io r) => Cell.IoCell r (List Bytes) -> BufferSink r
  sink output = buffer-sink (\\payload -> let prior = Cell.cell-read output in
    let stored = Cell.cell-write output (Cons (part payload 0 (bytes-len payload)) prior) in
    (WriteEvent (nat-to-int (bytes-len payload)) None, sink output))

  -- | Run one public reader operation, retaining every authoritative advanced handle.
  action :: forall r. using (io :: Io r) => BufferedReader r -> Nat -> Int -> Bytes -> Cell.IoCell r (List Int) ->{IO r} (Observation, BufferedReader r)
  action reader op argument reset calls = if op <= 1 then (Observed (bytes-new 0) 0 0 0 False 0, reader)
    else if op == 2 then case buffered-peek reader argument of (event, next) -> (read-observation event, next)
    else if op == 3 then case buffered-discard reader argument of
      (CountEvent count status, next) -> (Observed (bytes-new 0) count 0 0 False (error-code status), next)
    else if op == 4 then
      let wanted = if 0 <= argument then natural (nat-from-int argument) else 0 in
      case buffered-read reader wanted of (event, next) -> (read-observation event, next)
    else if op == 5 then case buffered-read-byte reader of
      (Ok value, next) -> let payload = bytes-new 1 in let written = bytes-set payload 0 (u8-to-nat value) in (Observed payload 1 0 0 False 0, next)
      (Err error, next) -> (Observed (bytes-new 1) 0 0 0 False (error-code (Some error)), next)
    else if op == 6 then case buffered-read-rune reader of
      (Ok (value, width), next) -> (Observed (bytes-new 0) width (char-code value) width False 0, next)
      (Err error, next) -> (Observed (bytes-new 0) 0 0 0 False (error-code (Some error)), next)
    else if op == 7 then case buffered-unread-byte reader of (event, next) -> (unread-observation event, next)
    else if op == 8 then case buffered-unread-rune reader of (event, next) -> (unread-observation event, next)
    else if op == 9 then case buffered-read-slice reader (delimiter argument) of (event, next) -> (read-observation event, next)
    else if op == 10 then case buffered-read-bytes reader (delimiter argument) of (event, next) -> (read-observation event, next)
    else if op == 11 then case buffered-read-string reader (delimiter argument) of
      (TextEvent payload text status, next) -> (Observed payload (bytes-len payload) 0 0 False (error-code status), next)
    else if op == 12 then case buffered-read-line reader of
      (LineEvent payload prefix status, next) -> (Observed payload (bytes-len payload) 0 0 prefix (error-code status), next)
    else if op == 13 then (Observed (bytes-new 0) 0 0 0 False 0, buffered-reader-reset-self reader)
    else if op == 14 then (Observed (bytes-new 0) 0 0 0 False 0, buffered-reader-reset reader (source reset 0 0 False 0 calls False False))
    else if op == 15 then let output = Cell.cell-new Nil in
      case buffered-write-to reader (sink output) of
        (TransferEvent count status, next, destination) ->
          let payload = bytes-concat (reverse (Cell.cell-read output)) in
          if 0 <= count then (Observed payload (nat-from-i64 count) 0 0 False (error-code status), next)
          else (Observed payload 0 0 0 False 99, next)
    else (Observed (bytes-new 0) 0 0 0 False 99, reader)

  -- | Check complete operation results and source call traces, reporting the first absolute operation index.
  checks :: forall r. using (io :: Io r) => BufferedReader r -> Cell.IoCell r (List Int) -> List Check -> Nat ->{IO r} Int
  checks reader calls rows index = case rows of
    Nil -> 0
    Cons row rest -> case row of
      Check op argument reset expected count scalar width prefix error buffered requests ->
        let cleared = Cell.cell-write calls Nil in
        case action reader op argument reset calls of
          (observed, next) -> case observed of
            Observed payload actual actual-scalar actual-width actual-prefix actual-error ->
              if same payload expected && actual == count && actual-scalar == scalar && actual-width == width
                && actual-prefix == prefix && actual-error == error && buffered-reader-buffered next == buffered
                && same-calls (reverse (Cell.cell-read calls)) requests then checks next calls rest (index + 1)
              else nat-to-int index

  -- | Print the full observation index and normalize every failure to a nonzero process status without byte truncation.
  report-failure :: forall r. using (io :: Io r) => Int ->{IO r} Int
  report-failure index = let printed = ioWrite (string-to-bytes (string-concat (int-to-string index) "\\n")) in 1
'''


def literal(value: int) -> str:
    """Keep signed fixture numbers unambiguous inside constructor applications."""
    return f'({value})' if value < 0 else str(value)


def runner(functions: list[str]) -> str:
    """Keep every function below the language's let-depth limit while preserving the first absolute failure index."""
    result=''
    groups=[]
    for begin in range(0,len(functions),16):
        name='run-group'+str(begin//16);groups.append(name)
        result+=f'\n  -- | Run independent observations {begin+1} through {min(begin+16,len(functions))} without a deep generated let spine.\n  {name} :: forall r. using (io :: Io r) => Unit ->{{IO r}} Int\n  {name} unit =\n'
        for i,function in enumerate(functions[begin:begin+16]):
            result+='    '+('let' if i==0 else 'else let')+f' result{i} = {function} () in if not (result{i} == 0) then result{i}\n'
        result+='    else 0\n'
    result+='\n  -- | Print the full first mismatch and normalize every failing process exit.\n  main :: Unit ->{IO r} Int\n  main unit = with (io = newIO ()) in\n'
    for i,function in enumerate(groups):
        result+='    '+('let' if i==0 else 'else let')+f' result{i} = {function} () in if not (result{i} == 0) then report-failure result{i}\n'
    return result+'    else 0\n'


def main() -> None:
    """Select bounded complete trace prefixes and emit no implementation-derived expectations."""
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--vectors',type=Path,default=ROOT/'build/buffered-vectors-v3.json')
    p.add_argument('--output',type=Path,required=True)
    p.add_argument('--start',type=int,default=0);p.add_argument('--count',type=int,default=8)
    p.add_argument('--indices',help='Comma-separated original trace positions for a diverse bounded smoke')
    p.add_argument('--all',action='store_true',help='Include every ordinary and explicitly adapted invalid-contract history')
    a=p.parse_args();all_rows=json.loads(a.vectors.read_text())['readers']
    positions=list(range(len(all_rows))) if a.all else [int(i) for i in a.indices.split(',')] if a.indices else list(range(len(all_rows)))[a.start:a.start+a.count]
    rows=[all_rows[i] for i in positions]
    text=HEADER
    for number,row in enumerate(rows):
        steps=[]
        for op,expected in zip(row['steps'],row['expected']):
            calls='['+','.join(literal(-1 if c=='write-to' else int(c.split(':')[1])) for c in expected['calls'])+']'
            if row['fault']:
                expected={**expected,'error':'invalid-read-count','data':'','count':0,'rune':0,'width':0,'prefix':False,'buffered':0}
            elif expected['panic']:raise ValueError('unreviewed reference reader panic')
            steps.append(f'Check {OPS[op["name"]]} {literal(op.get("n",0))} (unhex "{op.get("data","")}") (unhex "{expected["data"]}") {expected["count"]} {expected["rune"]} {expected["width"]} {str(expected["prefix"])} {CODES[expected["error"]]} {expected["buffered"]} {calls}')
        raw=f'fault-source {literal(row["fault"])} calls' if row['fault'] else f'source (unhex "{row["input"]}") 0 {row["chunk"]} {str(row["together"])} {row["empties"]} calls True {row["fast"]}'
        text+=f'\n  -- | Independently observed complete reader trace {positions[number]}, including transfer and reset continuations.\n  tracecase{number} :: forall r. using (io :: Io r) => Unit ->{{IO r}} Int\n  tracecase{number} unit = let calls = Cell.cell-new Nil in\n    let reader = buffered-reader-sized {literal(row["size"])} ({raw}) in\n    if buffered-reader-size reader == {row["expected"][0]["size"]} then checks reader calls [\n'
        text+=',\n'.join('      ('+s+')' for s in steps)+f'\n    ] {1+(a.start+number)*100} else {99+(a.start+number)*100}\n'
    text+=runner([f'tracecase{i}' for i in range(len(rows))]);a.output.parent.mkdir(parents=True,exist_ok=True);a.output.write_text(text)
    print(f'Generated {len(rows)} independent reader histories')

if __name__=='__main__':main()
