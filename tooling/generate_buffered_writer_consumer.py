#!/usr/bin/env python3
"""Generate independent buffered-writer history consumers with callback, sticky-error and partial-output observations."""
from __future__ import annotations
import argparse
import json
from pathlib import Path
from generate_buffered_reader_consumer import HEADER as READER_HEADER, literal, runner

ROOT=Path(__file__).resolve().parents[1]
CODES={'none':0,'end':1,'short-write':7,'custom':8,'invalid-write-count':9}
OPS={'size':0,'buffered':1,'available':2,'write':3,'string':4,'byte':5,'rune':6,'flush':7,'space':8,'read-from':9,'reset-self':10,'reset':11}


def helper(first: str, following: str) -> str:
    """Reuse exact owned fixture helpers rather than derive any expectation from the implementation."""
    return READER_HEADER[READER_HEADER.index(first):READER_HEADER.index(following)]

HEADER='''-- | Independent buffered-writer histories retain exact partial counts, sticky errors, bytes and callback widths.
module Main where
  import Effect.Buffered.Writer
  import Effect.Buffered.Core
  import Effect.Buffered.Internal.Cell as Cell
  import Effect.Io
  import Data.Bytes
  import Data.Bytes.Build
  import Data.Bool
  import Data.Int
  import Data.List
  import Data.Machine
  import Data.Nat
  import Data.Option
  import Data.Result
  import Data.String
  import Text.Show

  -- | A writer operation and its independently observed complete state.
  data Check where
    -- | Operation, argument, input, accepted count, error, buffered, available, reservation capacity, output and calls.
    Check :: Nat -> Int -> Bytes -> Nat -> Nat -> Nat -> Nat -> Nat -> Bytes -> List Int -> Check

  -- | Operation-local progress, failure and initial reservation capacity.
  data Observation where
    -- | Accepted bytes, error family and advertised reservation capacity.
    Observed :: Nat -> Nat -> Nat -> Observation

'''
HEADER+=helper('  -- | Compare arbitrary bytes','  -- | Normalize only structural error')
HEADER+=helper('  -- | Preserve actual source request widths','  -- | Normalize one raw read event')
HEADER+=helper('  -- | Widen an exact conversion result','  -- | Convert fixture delimiter arguments')
HEADER+='''
  -- | Normalize only relevant structured sink errors; custom diagnostic text is retained by the production value.
  error-code :: Option BufferError -> Nat
  error-code status = case status of
    None -> 0
    Some BufferEnd -> 1
    Some (BufferShortWrite wanted actual) -> 7
    Some (BufferCustomError message) -> 8
    Some (BufferInvalidWriteCount wanted actual) -> 9
    other -> 99

  -- | Construct a sink over explicit mutable transport state, so reset can reuse the same underlying transport.
  sink :: forall r. using (io :: Io r) => Cell.IoCell r (List Bytes) -> Cell.IoCell r (List Int) -> Cell.IoCell r Int -> Int -> BufferSink r
  sink output calls budget limit = buffer-sink (\\input -> sink-write output calls budget limit input)

  -- | Accept a bounded prefix with an independently recorded callback width and optional simultaneous budget failure.
  sink-write :: forall r. using (io :: Io r) => Cell.IoCell r (List Bytes) -> Cell.IoCell r (List Int) -> Cell.IoCell r Int -> Int -> Bytes ->{IO r} (WriteEvent, BufferSink r)
  sink-write output calls budget limit input =
    let logged = Cell.cell-write calls (Cons (nat-to-int (bytes-len input)) (Cell.cell-read calls)) in
    let remaining = Cell.cell-read budget in let size = bytes-len input in
    let limited = if 0 <= limit && limit < nat-to-int size then natural (nat-from-int limit) else size in
    let count = if 0 <= remaining && remaining < nat-to-int limited then natural (nat-from-int remaining) else limited in
    let saved = Cell.cell-write output (Cons (part input 0 count) (Cell.cell-read output)) in
    let spent = if 0 <= remaining then Cell.cell-write budget (remaining - nat-to-int count) else () in
    let status = if 0 <= remaining && count < size then Some (BufferCustomError "fixture-sink") else None in
    (WriteEvent (nat-to-int count) status, sink output calls budget limit)

  -- | Report a deliberately invalid signed count without mutating any output bytes.
  fault-sink :: forall r. using (io :: Io r) => Int -> Cell.IoCell r (List Int) -> BufferSink r
  fault-sink fault calls = buffer-sink (\\input -> let logged = Cell.cell-write calls (Cons (nat-to-int (bytes-len input)) (Cell.cell-read calls)) in
    let actual = if fault < 0 then (-1) else nat-to-int (bytes-len input) + 1 in
    (WriteEvent actual None, fault-sink fault calls))

  -- | Retain a reported write count independently of the error accompanying that accepted prefix.
  write-observation :: WriteEvent -> Observation
  write-observation event = case event of
    WriteEvent actual status -> if 0 <= actual then Observed (nat-from-int actual) (error-code status) 0 else Observed 0 99 0

  -- | Convert a fixture's signed rune into its explicit32-bit numeric input without requiring scalar validity.
  rune-code :: Int -> I32
  rune-code argument = case checked-int-to-i64 argument of Some value -> wrapping-i64-to-i32 value; None -> (-1)

  -- | Run an available-buffer append with explicit reservation growth before ordinary checked commit.
  write-space :: forall r. using (io :: Io r) => BufferedWriter r -> Bytes ->{IO r} (Observation, BufferedWriter r)
  write-space writer input = let initial = buffered-writer-space writer in let capacity = buffer-space-capacity initial in
    let space = buffer-space-reserve initial (bytes-len input) in
    case buffer-space-append space input of
      Err failure -> (Observed 0 99 capacity, writer)
      Ok appended -> case buffered-write-space writer appended of
        (event, next) -> case write-observation event of Observed count error ignored -> (Observed count error capacity, next)

  -- | Invoke one public writer operation and preserve its authoritative advanced handle.
  action :: forall r. using (io :: Io r) => BufferedWriter r -> Nat -> Int -> Bytes -> BufferSink r -> Cell.IoCell r (List Int) ->{IO r} (Observation, BufferedWriter r)
  action writer op argument input original calls = if op <= 2 then (Observed 0 0 0, writer)
    else if op == 3 then case buffered-write writer input of (event, next) -> (write-observation event, next)
    else if op == 4 then case buffered-write-text-bytes writer input of (event, next) -> (write-observation event, next)
    else if op == 5 then
      let value = wrapping-i32-to-u8 (rune-code argument) in
      case buffered-write-byte writer value of
        (None, next) -> (Observed 1 0 0, next)
        (status, next) -> (Observed 0 (error-code status) 0, next)
    else if op == 6 then case buffered-write-rune writer (rune-code argument) of (event, next) -> (write-observation event, next)
    else if op == 7 then case buffered-flush writer of (status, next) -> (Observed 0 (error-code status) 0, next)
    else if op == 8 then write-space writer input
    else if op == 9 then
      case buffered-read-from writer (source input 0 0 False 0 calls False False) of
        (TransferEvent count status, next, advanced) -> if 0 <= count then (Observed (nat-from-i64 count) (error-code status) 0, next) else (Observed 0 99 0, next)
    else if op == 10 then (Observed 0 0 0, buffered-writer-reset-self writer)
    else if op == 11 then (Observed 0 0 0, buffered-writer-reset writer original)
    else (Observed 0 99 0, writer)

  -- | Compare each operation's progress, buffered suffix, output bytes and complete callback widths.
  checks :: forall r. using (io :: Io r) => BufferedWriter r -> BufferSink r -> Cell.IoCell r (List Bytes) -> Cell.IoCell r (List Int) -> List Check -> Nat ->{IO r} Int
  checks writer original output calls rows index = case rows of
    Nil -> 0
    Cons row rest -> case row of
      Check op argument input count error buffered available capacity expected requests ->
        let cleared = Cell.cell-write calls Nil in
        case action writer op argument input original calls of
          (Observed actual actual-error actual-capacity, next) ->
            if actual == count && actual-error == error && actual-capacity == capacity
              && buffered-writer-buffered next == buffered && buffered-writer-available next == available
              && same (bytes-concat (reverse (Cell.cell-read output))) expected
              && same-calls (reverse (Cell.cell-read calls)) requests then checks next original output calls rest (index + 1)
            else nat-to-int index

  -- | Print the full observation index and normalize every failure to a nonzero process status without byte truncation.
  report-failure :: forall r. using (io :: Io r) => Int ->{IO r} Int
  report-failure index = let printed = ioWrite (string-to-bytes (string-concat (int-to-string index) "\\n")) in 1
'''


def main() -> None:
    """Emit only independently terminating ordinary traces; invalid-contract traces need explicit refusal controls."""
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--vectors',type=Path,default=ROOT/'build/buffered-vectors-v3.json')
    p.add_argument('--output',type=Path,required=True)
    p.add_argument('--indices',default='0,1,2,10,40,60,80,100,120,140')
    p.add_argument('--all',action='store_true',help='Include every ordinary and explicitly adapted invalid-contract history')
    a=p.parse_args();all_rows=json.loads(a.vectors.read_text())['writers'];positions=list(range(len(all_rows))) if a.all else [int(i) for i in a.indices.split(',')];rows=[all_rows[i] for i in positions]
    text=HEADER
    for number,row in enumerate(rows):
        steps=[]
        for op,e in zip(row['steps'],row['expected']):
            requests=e['calls']
            if e['panic']:
                if row['fault']:e={**e,'error':'invalid-write-count','count':0}
                elif 'fixture zero-progress writer watchdog' in bytes.fromhex(e['panic']).decode():
                    e={**e,'error':'short-write','count':0};requests=requests[:1]
                else:raise ValueError('unreviewed reference writer panic')
            calls='['+','.join(literal(int(c.split(':')[1])) for c in requests)+']'
            steps.append(f'Check {OPS[op["name"]]} {literal(op.get("n",0))} (unhex "{op.get("data","")}") {e["count"]} {CODES[e["error"]]} {e["buffered"]} {e["available"]} {e["capacity"]} (unhex "{e["output"]}") {calls}')
        size=row['expected'][0]['size']
        original=f'fault-sink {literal(row["fault"])} calls' if row['fault'] else f'sink output calls budget {literal(row["limit"])}'
        text+=f'\n  -- | Independent full writer history {positions[number]} with its own transport budget and output.\n  tracecase{number} :: forall r. using (io :: Io r) => Unit ->{{IO r}} Int\n  tracecase{number} unit = let output = Cell.cell-new Nil in let calls = Cell.cell-new Nil in let budget = Cell.cell-new ({literal(row["budget"])} :: Int) in\n    let original = {original} in let writer = buffered-writer-sized {literal(row["size"])} original in\n    if buffered-writer-size writer == {size} then checks writer original output calls [\n'
        text+=',\n'.join('      ('+s+')' for s in steps)+f'\n    ] {1+number*100} else {99+number*100}\n'
    text+=runner([f'tracecase{i}' for i in range(len(rows))]);a.output.parent.mkdir(parents=True,exist_ok=True);a.output.write_text(text)
    print(f'Generated {len(rows)} complete independent writer histories')

if __name__=='__main__':main()
