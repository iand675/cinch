#!/usr/bin/env python3
"""Generate independent direct-split and complete scanner consumers with explicit checked adaptations for invalid protocols."""
from __future__ import annotations
import argparse
import json
from pathlib import Path
from generate_buffered_reader_consumer import HEADER as READER_HEADER, literal, runner

ROOT=Path(__file__).resolve().parents[1]
KINDS={'bytes':0,'lines':1,'runes':2,'words':3,'negative':4,'overadvance':5,'final-nil':6,'final-empty':7,'empty':8}
CODES={'none':0,'too-long':1,'negative-advance':2,'advance-too-far':3,'bad-read-count':4,'no-progress':5}


def helper(first: str, following: str) -> str:
    """Share byte reconstruction and transport fixture code without sharing any implementation expectations."""
    return READER_HEADER[READER_HEADER.index(first):READER_HEADER.index(following)]

HEADER='''-- | Independent split/scanner observations preserve malformed bytes and absent versus empty tokens.
module Main where
  import Effect.Scanner
  import Effect.Scanner.Split
  import Effect.Buffered.Core
  import Effect.Buffered.Internal.Cell as Cell
  import Effect.Io
  import Data.Bool
  import Data.Bytes
  import Data.Int
  import Data.List
  import Data.Machine
  import Data.Nat
  import Data.Option
  import Data.Result
  import Data.String
  import Text.Show

'''
HEADER+=helper('  -- | Compare arbitrary bytes','  -- | Normalize only structural error')
HEADER+=helper('  -- | Preserve actual source request widths','  -- | Normalize one raw read event')
HEADER+='''
  -- | Normalize every failure to exit1 after printing its full independent observation index.
  report-failure :: forall r. using (io :: Io r) => Int ->{IO r} Int
  report-failure index = let printed = ioWrite (string-to-bytes (string-concat (int-to-string index) "\\n")) in 1
'''
HEADER+='''
  -- | Keep byte reconstruction opaque to compile-time evaluation so public splitter calls execute at runtime.
  prim runtime-size :: U32 -> U32 where
    llvm {
      %value = or i32 %0, 0
      ret i32 %value
    }

  -- | Normalize structural scanner failures independently of their native presentation strings.
  error-code :: Option ScanError -> Nat
  error-code error = case error of
    None -> 0
    Some ScanTooLong -> 1
    Some (ScanNegativeAdvance count) -> 2
    Some (ScanAdvanceTooFar count available) -> 3
    Some (ScanBadReadCount wanted actual supplied) -> 4
    Some ScanNoProgress -> 5
    Some (ScanEmptyTokens count) -> 6
    Some (ScanEmptyTokensAfter count prior) -> 6
    other -> 99

  -- | Compare optional tokens, preserving Some empty independently of None.
  same-token :: Option Bytes -> Option Bytes -> Bool
  same-token actual expected = case actual of
    None -> case expected of None -> True; other -> False
    Some payload -> case expected of None -> False; Some wanted -> same payload wanted

  -- | Select actual built-in functions or explicit adversarial pure split callbacks.
  splitter :: Nat -> ScanSplit
  splitter kind = if kind == 0 then scan-split scan-bytes
    else if kind == 1 then scan-split scan-lines
    else if kind == 2 then scan-split scan-runes
    else if kind == 3 then scan-split scan-words
    else if kind == 4 then scan-split (\\input terminal -> ScanStep (-1) None None)
    else if kind == 5 then scan-split (\\input terminal -> ScanStep (nat-to-int (bytes-len input + 1)) None None)
    else if kind == 6 then scan-split (\\input terminal -> ScanStep 0 None (Some ScanFinalToken))
    else if kind == 7 then scan-split (\\input terminal -> ScanStep 0 (Some (bytes-new 0)) (Some ScanFinalToken))
    else scan-split (\\input terminal -> ScanStep 0 (Some (bytes-new 0)) None)
'''
HEADER=HEADER.replace('bytes-new (string-byte-length text / 2)', 'bytes-new (u32-to-nat (runtime-size (wrapping-i64-to-u32 (nat-to-i64 (bytes-len input)))) / 2)')
DIRECT='''
  -- | An independently observed direct split, including optional token identity and exact signed advance.
  data SplitCheck where
    -- | Function, input, terminal flag, expected advance, optional token and error family.
    SplitCheck :: Nat -> Bytes -> Bool -> Int -> Option Bytes -> Nat -> SplitCheck

  -- | Execute every supplied public split pipeline at runtime and return the first absolute mismatch.
  direct-checks :: List SplitCheck -> Nat -> Int
  direct-checks rows index = case rows of
    Nil -> 0
    Cons row rest -> case row of
      SplitCheck kind input terminal advance token error ->
        case scan-apply (splitter kind) input terminal of
          ScanStep actual payload status ->
            if actual == advance && same-token payload token && error-code status == error then direct-checks rest (index + 1)
            else nat-to-int index
'''
SCANNER='''
  -- | One scanner observation may repeat, keeping a long nonprogress sequence shallow in the source AST.
  data ScanCheck where
    -- | Repetitions, more-token result, optional raw bytes, error family and UTF-8 text validity.
    ScanCheck :: Nat -> Bool -> Option Bytes -> Nat -> Bool -> ScanCheck

  -- | Compare scanner state through public observations after each authoritative advanced scan.
  scan-checks :: forall r. using (io :: Io r) => Scanner r -> List ScanCheck -> Nat ->{IO r} Int
  scan-checks current rows index = case rows of
    Nil -> 0
    Cons row rest -> case row of
      ScanCheck repeated more token error valid-text -> if repeated > 0 then
        case scanner-next current of
          (actual, next) -> let text-valid = case scanner-text next of None -> False; Some text -> True in
            if actual == more && same-token (scanner-token next) token && error-code (scanner-error next) == error && text-valid == valid-text then
              if repeated > 1 then scan-checks next (Cons (ScanCheck (repeated - 1) more token error valid-text) rest) (index + 1)
              else scan-checks next rest (index + 1)
            else nat-to-int index
        else scan-checks current rest index
'''


def token_literal(raw: str, absent: bool) -> str:
    """Render token presence separately from its complete arbitrary byte payload."""
    return 'None' if absent else f'(Some (unhex "{raw}"))'


def is_utf8(raw: str) -> bool:
    """Validate independent expected bytes without consulting the implementation's decoder."""
    try:bytes.fromhex(raw).decode('utf8');return True
    except UnicodeDecodeError:return False


def main() -> None:
    """Emit full independent direct observations or bounded scanner trace groups with documented panic adaptations."""
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--vectors',type=Path,default=ROOT/'build/buffered-vectors-v3.json')
    p.add_argument('--output',type=Path,required=True)
    p.add_argument('--mode',choices=['direct','scanner'],required=True)
    p.add_argument('--indices',help='Comma-separated original scanner trace positions')
    a=p.parse_args();vectors=json.loads(a.vectors.read_text());text=HEADER+(DIRECT if a.mode=='direct' else SCANNER);functions=[]
    if a.mode=='direct':
        rows=vectors['splits']
        for begin in range(0,len(rows),16):
            number=begin//16;functions.append(f'batchcase{number}');entries=[]
            for row in rows[begin:begin+16]:
                if row['panic']:
                    if row['input'] or row['eof'] or row['kind'] not in ['bytes','runes']:raise ValueError('unreviewed reference protocol panic')
                    advance,token,absent,error=0,'',True,0
                else:advance,token,absent,error=row['advance'],row['token'],row['nil'],CODES[row['error']]
                entries.append(f'SplitCheck {KINDS[row["kind"]]} (unhex "{row["input"]}") {row["eof"]} {literal(advance)} {token_literal(token,absent)} {error}')
            text+=f'\n  -- | Independent direct observations {begin+1} through {begin+len(entries)}; invalid empty nonterminal calls safely request more data.\n  batchcase{number} :: Unit -> Int\n  batchcase{number} unit = direct-checks [\n'+',\n'.join('    ('+e+')' for e in entries)+f'\n  ] {begin+1}\n'
    else:
        positions=[int(i) for i in a.indices.split(',')] if a.indices else list(range(len(vectors['scanners'])))
        for number,position in enumerate(positions):
            row=vectors['scanners'][position];functions.append(f'batchcase{number}');entries=[]
            for e in row['expected']:
                if e['name']=='panic':
                    if row['kind']!='empty':raise ValueError('unreviewed scanner panic')
                    value=(False,'',False,6,True)
                else:value=(e['more'],e['data'],e['nil'],CODES[e['error']],is_utf8(e['data']))
                if entries and entries[-1][1]==value:entries[-1]=(entries[-1][0]+1,value)
                else:entries.append((1,value))
            rendered=[f'ScanCheck {n} {v[0]} {token_literal(v[1],v[2])} {v[3]} {v[4]}' for n,v in entries]
            text+=f'\n  -- | Independent full scanner trace {position} with explicit terminal-empty-token refusal adaptation.\n  batchcase{number} :: forall r. using (io :: Io r) => Unit ->{{IO r}} Int\n  batchcase{number} unit = let calls = Cell.cell-new Nil in\n    let raw = scanner (source (unhex "{row["input"]}") 0 {row["chunk"]} {row["together"]} 0 calls False False) in\n    case scanner-buffer-size raw {row["capacity"]} {literal(row["maximum"])} of\n      Err failure -> {position*1000+998}\n      Ok buffered -> case scanner-split buffered (splitter {KINDS[row["kind"]]}) of\n        Err failure -> {position*1000+999}\n        Ok configured -> scan-checks configured [\n'+',\n'.join('          ('+e+')' for e in rendered)+f'\n        ] {position*1000+1}\n'
    text+=runner(functions);a.output.parent.mkdir(parents=True,exist_ok=True);a.output.write_text(text)
    print(f'Generated {a.mode}: {len(functions)} shallow groups')

if __name__=='__main__':main()
