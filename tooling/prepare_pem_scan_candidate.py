#!/usr/bin/env python3
"""Prepare isolated machine-counter PEM marker scans without changing the maintained source."""
from __future__ import annotations
import argparse
import hashlib
import json
from pathlib import Path

ROOT=Path(__file__).resolve().parents[1]


def candidate(source: str, end_prefilter: str='newline') -> str:
    """Keep exact marker reads and recovery offsets while removing boxed per-candidate arithmetic."""
    start=source.index('  -- | Match a complete BEGIN prefix only inside the selected bounded interval.\n')
    end=source.index('  -- | Strip only trailing space and tab bytes, preserving all other line bytes exactly.\n',start)
    replacement='''  -- | Locate a newline-anchored END with bounded machine positions and exact marker fringes.
  find-end-machine :: I64 -> U64 -> U64 -> U64
  find-end-machine base at end = if before at end then
    let found = find-machine base at end 10 in let remaining = minus end found in
    if before remaining 10 then end
    else let pointer = address base found in
      if end-token pointer then found else let next = plus found 1 in find-end-machine base next end
    else end

  -- | Locate the first END while holding its immutable owner through every read-only scan.
  find-end :: Bytes -> Nat -> Option Nat
  find-end source at = let end = bytes-len source in
    if at <= end then
      let base = bytes-data source in let position = wrapping-i64-to-u64 (nat-to-i64 at) in
      let start = if before 0 position then minus position 1 else 0 in
      let limit = wrapping-i64-to-u64 (nat-to-i64 end) in
      let found = hold source (find-end-machine base start limit) in
      if before found limit then Some (natural found) else None
    else None

  -- | Retain the final raw BEGIN before an END without allocating a result per dash candidate.
  last-begin-machine :: I64 -> U64 -> U64 -> U64 -> U64
  last-begin-machine base at end best = if before at end then
    let found = find-machine base at end 45 in let remaining = minus end found in
    if before remaining 11 then best
    else let pointer = address base found in
      let chosen = if begin-token pointer then found else best in
      let next = plus found 1 in last-begin-machine base next end chosen
    else best

  -- | Preserve the final raw BEGIN, including an unanchored candidate, inside checked owner bounds.
  last-begin :: Bytes -> Nat -> Nat -> Option Nat -> Option Nat
  last-begin source at end best = if at <= end && end <= bytes-len source then
    let base = bytes-data source in let start = wrapping-i64-to-u64 (nat-to-i64 at) in
    let limit = wrapping-i64-to-u64 (nat-to-i64 end) in
    let prior = case best of None -> limit; Some position -> if position <= end then wrapping-i64-to-u64 (nat-to-i64 position) else limit in
    let found = hold source (last-begin-machine base start limit prior) in
    if before found limit then Some (natural found) else best
    else best

'''
    if end_prefilter=='dash':
        old=replacement[:replacement.index('  -- | Locate the first END while holding')]
        new='''  -- | Locate a newline-anchored END using dash prefilters and bounded machine positions.
  find-end-machine :: I64 -> U64 -> U64 -> U64
  find-end-machine base at end = if before at end then
    let found = find-machine base at end 45 in
    if before found end then
      if before 0 found then
        let remaining = minus end found in
        if before remaining 9 then end
        else let line = minus found 1 in let pointer = address base line in
          if end-token pointer then line else let next = plus found 1 in find-end-machine base next end
      else let next = plus found 1 in find-end-machine base next end
    else end
    else end

'''
        replacement=replacement.replace(old,new,1).replace('let start = if before 0 position then minus position 1 else 0 in','let start = position in',1)
    elif end_prefilter!='newline':raise ValueError('unknown END prefilter')
    return source[:start]+replacement+source[end:]


def priority_candidate(source: str) -> str:
    """Replace repeated temporary comparison buffers with an exact bounded read-only token test."""
    old='''  -- | Recognize the exact case-sensitive header that must precede other encoded metadata.
  proc-type :: PemHeader -> Bool
  proc-type header = case header of
    OwnedHeader key value -> let expected = string-to-bytes "Proc-Type" in
      let ordering = bytes-compare key expected in eq-i64 ordering 0
'''
    new='''  -- | Match the nine-byte priority key without constructing a temporary comparison owner.
  prim proc-token :: I64 -> Bool where
    unsafe llvm {
      %pointer = inttoptr i64 %0 to ptr
      %bytes = load <8 x i8>, ptr %pointer, align 1
      %equal = icmp eq <8 x i8> %bytes, <i8 80, i8 114, i8 111, i8 99, i8 45, i8 84, i8 121, i8 112>
      %mask = bitcast <8 x i1> %equal to i8
      %head = icmp eq i8 %mask, -1
      %lastptr = getelementptr i8, ptr %pointer, i64 8
      %last = load i8, ptr %lastptr, align 1
      %suffix = icmp eq i8 %last, 101
      %yes = and i1 %head, %suffix
      %result = zext i1 %yes to i8
      ret i8 %result
    }

  -- | Recognize the exact case-sensitive priority header with a checked nine-byte owner bound.
  proc-type :: PemHeader -> Bool
  proc-type header = case header of
    OwnedHeader key value -> if bytes-len key == 9 then
      let pointer = bytes-data key in hold key (proc-token pointer)
      else False
'''
    if old not in source:raise ValueError('validated priority comparison changed')
    return source.replace(old,new,1)


def main() -> None:
    """Record exact baseline and derived bytes so experimental scan variants remain reviewable."""
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--baseline',type=Path,default=ROOT/'seed/prelude/Text/Pem.cinch')
    parser.add_argument('--priority-token',action='store_true')
    parser.add_argument('--end-prefilter',choices=['newline','dash'],default='newline')
    parser.add_argument('--output',type=Path,required=True)
    args=parser.parse_args();original=args.baseline.read_bytes();text=candidate(original.decode(),args.end_prefilter)
    if args.priority_token:text=priority_candidate(text)
    rendered=text.encode()
    args.output.parent.mkdir(parents=True,exist_ok=True);args.output.write_bytes(rendered)
    args.output.with_suffix('.json').write_text(json.dumps({'baseline_sha256':hashlib.sha256(original).hexdigest(),'candidate_sha256':hashlib.sha256(rendered).hexdigest(),'scope':'Bounded machine-position marker loops; optional exact9 priority-token read; no LLVM mutation','end_prefilter':args.end_prefilter,'priority_token':args.priority_token,'ownership':'whole scan remains under the checked owner hold'},indent=2)+'\n')


if __name__=='__main__':main()
