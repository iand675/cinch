#!/usr/bin/env python3
"""Render bounded UUID consumers from actual public-operation vectors without recomputing expectations."""
from __future__ import annotations
import argparse
import json
from pathlib import Path
ROOT = Path(__file__).resolve().parents[1]

def quote(text: str) -> str:
    """Preserve string contents as a literal without interpreting identifier text."""
    return json.dumps(text, ensure_ascii=True)

def source(mode: str, start: int = 0, count: int = 128) -> str:
    """Embed one bounded group of independently recorded operation results."""
    vectors = json.loads((ROOT / 'compiler/tests/fixtures/uuid-vectors.json').read_text())
    rows = vectors[mode] if mode == 'v7' else vectors[mode][start:start + count]
    assert rows
    values = []
    for row in rows:
        if mode == 'parse':
            values.append('ParseCase ' + ' '.join((quote(row['input_hex']), str(row['ok']), quote(row['canonical']), quote(row['receiver']), str(row['must_panic']))))
        elif mode == 'compare':
            values.append('CompareCase ' + ' '.join((quote(row['first']), quote(row['second']), '('+str(row['order'])+')')))
        elif mode == 'append':
            values.append('AppendCase ' + ' '.join((quote(row['prefix']), quote(row['value']), quote(row['output']))))
        elif mode == 'v7':
            values.append('ClockCase ' + ' '.join((str(row['unix_nanos']), str(row['repeat']), quote(row['entropy']), quote(row['canonical']))))
        else:
            raise ValueError(mode)
    template = (ROOT / f'compiler/tests/fixtures/uuid-{mode}.cinch.in').read_text()
    assert template.count('@CASES@') == 1
    return template.replace('@CASES@', '[' + ',\n    '.join(values) + ']')

def main() -> None:
    """Write exactly one bounded consumer selected by the caller's runtime budget."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--mode', choices=('parse','compare','append','v7'), required=True)
    parser.add_argument('--start', type=int, default=0)
    parser.add_argument('--count', type=int, default=128)
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(source(args.mode,args.start,args.count))

if __name__ == '__main__':
    main()
