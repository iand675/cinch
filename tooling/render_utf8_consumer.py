#!/usr/bin/env python3
"""Render bounded executable UTF-8 corpus pages and a raw-byte hexadecimal fixture stream."""
from __future__ import annotations
import json
from pathlib import Path
from generate_unicode_tables import render_words

ROOT=Path(__file__).resolve().parents[1]


def rendered() -> str:
    """Keep input data independent while sharing only shallow constant-page formatting."""
    fixture=json.loads((ROOT/'compiler/tests/fixtures/utf8-vectors.json').read_text())
    fields=[]
    offsets=[]
    size=0
    for index,row in enumerate(fixture['rows']):
        if index%64==0: offsets.append(size)
        for value in [row['input'],row['replacement'],row['repair'],row['each'],row['hash'].to_bytes(8,'little').hex()]:
            fields.append(value+'\0');size+=len(value)+1
    literal=json.dumps(''.join(fields)).replace('\\u0000','\\u{0}')
    words='\n'.join([render_words('utf8-scalars',64,[row['hash'] for row in fixture['scalar_chunks']]),render_words('utf8-pairs',64,[row['hash'] for row in fixture['pair_chunks']]),render_words('utf8-offsets',32,offsets)])
    return (ROOT/'compiler/tests/fixtures/utf8-consumer.cinch.in').read_text().replace('__ROW_GROUPS__',str(len(offsets))).replace('__ROWS__',literal).replace('__EXPECTED__',words)


if __name__=='__main__':
    print(rendered(),end='')
