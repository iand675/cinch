#!/usr/bin/env python3
"""Project offline linked-list traces into a standalone documented consumer."""
from __future__ import annotations

import argparse
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
FIXTURES = ROOT / 'compiler/tests/fixtures'


def source(index: int, mode: str = 'trace') -> str:
    """Keep fixture generation independent from compilation and runtime SDK availability."""
    if mode == 'control':
        return (FIXTURES / 'linked-list-control.cinch').read_text()
    row = json.loads((FIXTURES / 'linked-list-vectors.json').read_text())[index]
    steps = 'Nil'
    for op in reversed(row['operations']):
        values = ' '.join(str(op[name]) for name in
                          ['kind', 'list', 'handle', 'marker', 'value', 'code', 'identity', 'returned', 'state'])
        steps = f'(Cons (Step {values}) {steps})'
    return (FIXTURES / 'linked-list-consumer.cinch').read_text() + (
        '\n  -- | Verify one complete independently recorded mutation trace.\n'
        '  main :: Unit ->{Mut r} Int\n'
        f'  main unit = if trace-case {row["initial"]} {steps} then 0 else 1\n'
    )


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--case', type=int, default=0)
    parser.add_argument('--mode', choices=['trace', 'control'], default='trace')
    args = parser.parse_args()
    print(source(args.case, args.mode), end='')
