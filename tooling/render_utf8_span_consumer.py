#!/usr/bin/env python3
"""Render bounded UTF-8 span fingerprints without large refined constructor lists."""
from __future__ import annotations
import json
from pathlib import Path
from generate_unicode_tables import render_words

ROOT = Path(__file__).resolve().parents[1]


def rendered() -> str:
    """Retain direct independent hashes and a compact hexadecimal owner-buffer stream."""
    fixture = json.loads((ROOT / 'seed/tests/fixtures/utf8-span-vectors.json').read_text())
    fields, offsets = [], []
    size = 0
    for index, row in enumerate(fixture['rows']):
        if index % 8 == 0: offsets.append(size)
        field = row['input'] + '\0'; fields.append(field); size += len(field)
    words = render_words('span-hashes', 64, [row['hash'] for row in fixture['rows']]) + '\n' + render_words('span-offsets', 32, offsets)
    inputs = json.dumps(''.join(fields)).replace('\\u0000', '\\u{0}')
    source = (ROOT / 'seed/tests/fixtures/utf8-span-consumer.cinch.in').read_text()
    return source.replace('__ROWS_COUNT__', str(len(fixture['rows']))).replace('__GROUPS__', str(len(offsets))).replace('__INPUTS__', inputs).replace('__WORDS__', words)


if __name__ == '__main__':
    print(rendered(), end='')
