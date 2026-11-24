#!/usr/bin/env python3
"""Check the exact private ring vector kernel against independent full-width lane arithmetic."""
from __future__ import annotations

from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def source() -> str:
    """Extract the consumed kernel and compute each lane expectation using unbounded integers."""
    ring = (ROOT / 'seed/prelude/Data/Mutable/Ring.cinch').read_text()
    start = ring.index('  -- | Offset independent previous/next links')
    end = ring.index('  -- | Adjust both links', start)
    values = [0, 1, (1 << 32) - 1, 1 << 32, (1 << 63) - 1, 1 << 63, (1 << 64) - 1]
    checks = []
    for left_index, left in enumerate(values):
        for right_index, right in enumerate(values):
            offset = values[(left_index + 2 * right_index) % len(values)]
            low = (left + offset) % (1 << 64)
            high = (right + offset) % (1 << 64)
            checks.append(f'(eq-u128 (offset-pair {left} {right} {offset}) {low + (high << 64)})')
    return ('-- | Verify independent packed lane offsets at every scalar boundary.\nmodule Main where\n'
            '  import Data.Wide\n' + ring[start:end]
            + '\n  -- | Require every independently computed full-width lane result.\n'
            + '  main :: Int\n  main = if ' + ' && '.join(checks) + ' then 0 else 1\n')


if __name__ == '__main__':
    print(source(), end='')
