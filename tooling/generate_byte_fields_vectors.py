#!/usr/bin/env python3
"""Crosscheck complete bounded fields and lazy callback traces independently of the library."""
from __future__ import annotations
import argparse
import json
import os
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[1]
FIXTURE = ROOT/'seed/tests/fixtures/byte-fields-vectors.json'
SPACE = frozenset([*range(9,14),32,133,160,5760,*range(8192,8203),8232,8233,8239,8287,12288])


def recipes() -> list[dict]:
    """Keep a small first corpus with all bytes and adversarial scalar/view boundaries."""
    inputs = [b'',b' ',b' a  b\t',b'\0a\0',b'a,b,,c,',b'\x85\xa0',b'\xff\xc0\xaf',
              b'\xed\xa0\x80',b'\xf0\x9f\x99', '\u0085x\u00a0'.encode(),
              '\ufffdx🙂é'.encode(), 'a\u2003b\u3000c'.encode()]
    inputs += [chr(c).encode()+b'x'+chr(c).encode() for c in sorted(SPACE | {28,29,30,31,6158,8203,65279})]
    inputs += [bytes([c]) for c in range(256)]
    inputs += [(b'a '*32)[:n] for n in [7,8,9,15,16,17,31,32,33,63,64]]
    inputs += [value*n for n in [15,16,17,31,32,33,63,64,65] for value in [b' ',b'z']]
    rows=[]
    for number,raw in enumerate(inputs):
        modes=['default','space','comma','error','nonascii','even'] if number<12 else ['default','error'] if len(raw)==1 else ['default','space']
        for mode in modes:
            rows.append(dict(input=raw.hex(),start=0,end=len(raw),mode=mode,take=-1,replays=2))
        if number<12:
            for take in [0,1,2]:
                rows.append(dict(input=raw.hex(),start=0,end=len(raw),mode='space',take=take,replays=2))
    for raw in ['🙂é\u2003x'.encode(),b'\xf0\x9f\x99\x82\xed\xa0\x80']:
        for start in range(len(raw)+1):
            for end in range(start,len(raw)+1):
                for mode in ['default','error']:
                    rows.append(dict(input=raw.hex(),start=start,end=end,mode=mode,take=1,replays=1))
    for value in [b' ',b'z']:
        raw=value*49
        for start in range(16):
            rows.append(dict(input=raw.hex(),start=start,end=start+33,mode='default',take=-1,replays=2))
    for value in range(256):
        raw=bytes([value])*33
        rows.append(dict(input=raw.hex(),start=0,end=len(raw),mode='default',take=-1,replays=2))
    for lane in range(16):
        for stop in [32,133]:
            raw=bytearray(b'z'*33)
            raw[lane]=stop
            rows.append(dict(input=raw.hex(),start=0,end=len(raw),mode='default',take=-1,replays=2))
    return rows


def decode(raw: bytes, at: int) -> tuple[int,int]:
    """Use the independent strict decoder with one-byte replacement progress on every invalid prefix."""
    first=raw[at]
    width=1 if first<128 else 2 if 194<=first<=223 else 3 if 224<=first<=239 else 4 if 240<=first<=244 else 0
    if width and at+width<=len(raw):
        try:
            value=raw[at:at+width].decode('utf-8','strict')
            return ord(value),width
        except UnicodeDecodeError:
            pass
    return 65533,1


def selected(mode: str, cp: int) -> bool:
    """Apply an explicit scalar set; Python's broader whitespace policy is deliberately unused."""
    return cp in SPACE if mode in ['default','space'] else cp==44 if mode=='comma' else cp==65533 if mode=='error' else cp>=128 if mode=='nonascii' else cp%2==0


def traverse(raw: bytes, mode: str, take: int) -> tuple[list[str],list[dict]]:
    """Record decisions and yielded raw spans in execution order, including the terminating scalar."""
    if take==0:
        return [],[]
    out,events=[],[]
    start=None
    at=0
    while at<len(raw):
        cp,width=decode(raw,at)
        if mode!='default':
            events.append({'kind':'predicate','rune':cp})
        if selected(mode,cp):
            if start is not None:
                value=raw[start:at].hex()
                out.append(value)
                events.append({'kind':'yield','rune':0,'value':value})
                if take>0 and len(out)>=take:
                    return out,events
                start=None
        elif start is None:
            start=at
        at+=width
    if start is not None:
        value=raw[start:].hex()
        out.append(value)
        events.append({'kind':'yield','rune':0,'value':value})
    return out,events


def crosscheck(rows: list[dict]) -> None:
    """Check every returned byte, capacity bound, scalar callback and early-stop/replay observation."""
    for row in rows:
        raw=bytes.fromhex(row['input'])[row['start']:row['end']]
        eager,events=traverse(raw,row['mode'],-1)
        assert eager==row['eager'],row
        assert row['capacities']==[len(bytes.fromhex(x)) for x in eager],row
        assert row['eager_events']==[x for x in events if x['kind']=='predicate'],row
        assert row['construction_events']==[],row
        values,events=traverse(raw,row['mode'],row['take'])
        assert len(row['invocations'])==row['replays'],row
        for run in row['invocations']:
            assert run=={'values':values,'events':events},row


def effect_control(row: dict) -> dict:
    """Model mutable aliases with saved pre-callback widths and fresh per-invocation positions."""
    raw=bytearray.fromhex(row['input'])
    name,mode,take=row['name'],row['mode'],row['take']
    mutated=False
    runs=[]
    for replay in range(2):
        at,start,events,held=0,None,[],[]
        def emit(begin: int,end: int) -> bool:
            """Save the alias bounds before performing the yield callback's input mutation."""
            held.append((begin,end))
            events.append({'kind':'yield','value':raw[begin:end].hex(),'input':raw.hex()})
            if len(held)==1:
                if name in ['yield-terminator-width','default-terminator-width']:
                    raw[17:20]=b' xy'
                elif name=='yield-future-invalid':
                    raw[2:4]=b'\xff '
                elif name=='yield-alias':
                    raw[begin]=90
                elif name=='stop-after-first':
                    raw[-1]=122
                else:
                    return take<0 or len(held)<take
                events.append({'kind':'mutation','input':raw.hex()})
            return take<0 or len(held)<take
        stopped=False
        while at<len(raw):
            cp,width=decode(raw,at)
            if mode!='default':
                events.append({'kind':'predicate','rune':cp,'input':raw.hex()})
                if not mutated and (name in ['predicate-future','predicate-current-width'] or (name=='predicate-past' and cp==98)):
                    if name=='predicate-future':
                        raw[2]=120
                    elif name=='predicate-current-width':
                        raw[:2]=b'a '
                    else:
                        raw[0]=90
                    mutated=True
                    events.append({'kind':'mutation','input':raw.hex()})
            if selected(mode,cp):
                if start is not None:
                    if not emit(start,at):
                        stopped=True
                        break
                    start=None
            elif start is None:
                start=at
            at+=width
        if not stopped and start is not None:
            emit(start,len(raw))
        runs.append({'events':events,'held_after':[raw[a:b].hex() for a,b in held],'input_after':raw.hex()})
    return {'construction_events':[],'invocations':runs}


def main() -> None:
    """Generate from the actual pinned SDK, independently crosscheck, and retain a bounded fixture."""
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--go',type=Path)
    parser.add_argument('--output',type=Path,default=FIXTURE)
    parser.add_argument('--crosscheck-only',action='store_true')
    args=parser.parse_args()
    if args.crosscheck_only:
        result=json.loads(args.output.read_text())
    else:
        assert args.go
        run=subprocess.run([str(args.go),'run',str(ROOT/'tooling/go_byte_fields_vectors.go')],input=json.dumps(recipes()),text=True,capture_output=True,
                           env={**os.environ,'GOCACHE':'/tmp/cinch-go-surface-cache','GOTELEMETRY':'off','GOMAXPROCS':'1'},check=True)
        result=json.loads(run.stdout)
    crosscheck(result['rows'])
    for row in result.get('effect_controls',[]):
        expected=effect_control(row)
        assert row['construction_events']==expected['construction_events'],row['name']
        assert row['invocations']==expected['invocations'],row['name']
    if not args.crosscheck_only:
        args.output.parent.mkdir(parents=True,exist_ok=True)
        payload = ('{\n  "sdk_version": '+json.dumps(result['sdk_version'])
                   +',\n  "rows": [\n'
                   +',\n'.join('    '+json.dumps(row,separators=(',',':')) for row in result['rows'])
                   +'\n  ],\n  "effect_controls": '+json.dumps(result.get('effect_controls',[]),indent=2)+'\n}\n')
        assert len(payload.encode()) < 1_000_000, 'bounded first fixture exceeds size budget'
        args.output.write_text(payload)
    print('Independent complete field/trace rows',len(result['rows']))


if __name__=='__main__':
    main()
