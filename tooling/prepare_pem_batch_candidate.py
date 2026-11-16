#!/usr/bin/env python3
"""Prepare isolated bounded PEM payload-batch candidates without editing the maintained source."""
from __future__ import annotations
import argparse
import hashlib
import json
from pathlib import Path

ROOT=Path(__file__).resolve().parents[1]


def candidate(source: str, size: int) -> str:
    """Batch Base64 work across complete 64-column lines while retaining checked mutations and sink chunks."""
    if size==48:return source
    if size<48 or size>3072 or size%48:raise ValueError('batch must be a bounded multiple of48')
    marker='  -- | Encode bounded 48-byte source windows into canonical 64-column output lines.\n'
    helper='''  -- | Copy complete encoded lines through checked bulk writes, retaining only a bounded batch.
  put-lines :: Bytes -> Nat -> Bytes -> Nat -> Nat
  put-lines encoded at out cursor = if at < bytes-len encoded then
    let remaining = bytes-len encoded - at in let count = if remaining < 64 then remaining else 64 in
    if at + count <= bytes-len encoded && cursor + count <= bytes-len out then
      let copied = bytes-copy-range out encoded cursor at count in
      let next = byte out (cursor + count) 10 in put-lines encoded (at + count) out next
    else cursor
    else cursor

  -- | Encode bounded payload batches while preserving canonical 64-column output lines.
'''
    if marker not in source:raise ValueError('expected validated48-byte baseline')
    source=source.replace(marker,helper,1)
    source=source.replace('let size = if remaining < 48 then remaining else 48 in','let size = if remaining < '+str(size)+' then remaining else '+str(size)+' in')
    old='''    let next = put out cursor encoded in let after = byte out next 10 in
    put-body source (at + size) out after
'''
    new='''    let next = put-lines encoded 0 out cursor in put-body source (at + size) out next
'''
    if old not in source:raise ValueError('memory body changed')
    source=source.replace(old,new,1)
    old='''    let line = bytes-new (bytes-len encoded + 1) in let cursor = put line 0 encoded in
    let after = byte line cursor 10 in case send sink line of
'''
    new='''    let count = bytes-len encoded in let line = bytes-new (count + nat-div (count + 63) 64) in
    let after = put-lines encoded 0 line 0 in case send sink line of
'''
    if old not in source:raise ValueError('writer body changed')
    return source.replace(old,new,1)


def main() -> None:
    """Keep source hashes and the exact bounded batch parameter with every reproducible candidate."""
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--baseline',type=Path,default=ROOT/'compiler/prelude/Text/Pem.cinch')
    parser.add_argument('--batch',type=int,default=768)
    parser.add_argument('--output',type=Path,required=True)
    args=parser.parse_args();original=args.baseline.read_bytes();rendered=candidate(original.decode(),args.batch).encode()
    args.output.parent.mkdir(parents=True,exist_ok=True);args.output.write_bytes(rendered)
    args.output.with_suffix('.json').write_text(json.dumps({'baseline_sha256':hashlib.sha256(original).hexdigest(),'candidate_sha256':hashlib.sha256(rendered).hexdigest(),'batch':args.batch,'mutations':'checked language bulk copies/byte writes only; no primitive changes','scope':'payload batching only; byte output/header/parser/error contracts unchanged'},indent=2)+'\n')


if __name__=='__main__':main()
