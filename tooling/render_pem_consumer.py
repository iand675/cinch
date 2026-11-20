#!/usr/bin/env python3
"""Render compact independent PEM byte fixtures and shallow readonly page offsets."""
from __future__ import annotations
import json
from pathlib import Path
from generate_unicode_tables import render_words

ROOT = Path(__file__).resolve().parents[1]


def block_fields(block: dict) -> list[str]:
    """Retain exact label/header/payload bytes, including malformed UTF-8 metadata."""
    fields = [block['label'], len(block['headers']).to_bytes(4, 'little').hex()]
    for header in block['headers']: fields.extend([header['key'], header['value']])
    return fields + [block['data']]


def rendered() -> str:
    """Use one byte stream per operation so the compiler never sees a giant refined constructor list."""
    data = json.loads((ROOT / 'seed/tests/fixtures/pem-vectors.json').read_text())
    values, words = {}, []
    for kind in ['encode', 'decode']:
        fields, offsets, size = [], [], 0
        for index, row in enumerate(data[kind + 's']):
            if index % 8 == 0: offsets.append(size)
            row_fields = block_fields(row['block']) + ['01' if row['valid'] else '00', row['output']] if kind == 'encode' else [row['input'], '01' if row['found'] else '00', *block_fields(row['block']), row['rest']]
            for field in row_fields: fields.append(field + '\0'); size += len(field) + 1
        values['__' + kind.upper() + 'S__'] = json.dumps(''.join(fields)).replace('\\u0000', '\\u{0}')
        values['__' + kind.upper() + '_GROUPS__'] = str(len(offsets))
        values['__' + kind.upper() + '_ROWS__'] = str(len(data[kind + 's']))
        words.append(render_words('pem-' + kind + '-offsets', 32, offsets))
    source = (ROOT / 'seed/tests/fixtures/pem-consumer.cinch.in').read_text()
    for marker, value in values.items(): source = source.replace(marker, value)
    return source.replace('__WORDS__', '\n'.join(words))


if __name__ == '__main__':
    print(rendered(), end='')
