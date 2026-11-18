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
    if mode == 'capacity':
        return (FIXTURES / 'linked-list-capacity-control.cinch').read_text()
    if mode == 'growth':
        return growth_source()
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


def growth_checks(bits: int, suffix: str = "") -> str:
    """Calculate independent allocation-layout and near-overflow expectations using unbounded host integers."""
    maximum = ((1 << (bits - 1)) - 1 - 32) // 8
    half = maximum // 2
    rows = [(0, 1, 4), (1, 2, 4), (3, 4, 4), (4, 5, 8), (5, 6, 10),
            (half, half + 1, half * 2), (half, maximum, maximum),
            (half + 1, half + 2, maximum), (maximum - 1, maximum, maximum),
            (maximum, maximum, maximum)]
    return ' && '.join([f'eq-u64 maximum-slots{suffix} {maximum}'] +
                       [f'eq-u64 (grown-capacity{suffix} {capacity} {needed}) {expected}' for capacity, needed, expected in rows])



def growth_source() -> str:
    """Project actual private pure helpers so boundary growth can be tested without enormous allocations."""
    module = (ROOT / 'compiler/prelude/Data/Mutable/LinkedList.cinch').read_text()
    maximum_start = module.rfind('  -- |', 0, module.index('  maximum-slots :: U64'))
    maximum_end = module.index('  -- | Convert a bounded nonnegative slot count', maximum_start)
    growth_start = module.index('  -- | Grow geometrically without overflowing')
    growth_end = module.index('  -- | Install enough arena space', growth_start)
    actual = module[maximum_start:maximum_end] + module[growth_start:growth_end]
    modeled = []
    primitive = '(wrapping-usize-to-u64 max-bound-usize)'
    for bits in [32, 64]:
        suffix = f'-model{bits}'
        copy = actual.replace('maximum-slots', f'maximum-slots{suffix}').replace('grown-capacity', f'grown-capacity{suffix}')
        if copy.count(primitive) != 1:
            raise ValueError('capacity model requires one exact target primitive input')
        copy = copy.replace(primitive, f'(model-input {(1 << bits) - 1})')
        modeled.append(copy)
    opaque = ('  -- | Supply the target primitive value opaquely to source folding while retaining ordinary LLVM optimization.\n'
              '  prim model-input :: U64 -> U64 where\n'
              '    llvm {\n      ret i64 %0\n    }\n\n')
    return ('-- | Check actual private capacity arithmetic without allocating its synthetic boundary sizes.\n'
            'module Main where\n  import Data.Machine\n\n' + opaque + actual + ''.join(modeled)
            + '  -- | Check both modeled target inputs and the actual native target against independent constants.\n'
            + '  main :: Int\n  main = '
            + f'if {growth_checks(32, "-model32")} then (if {growth_checks(64, "-model64")} then '
            + '(if eq-u64 (wrapping-usize-to-u64 max-bound-usize) 4294967295 then '
            + f'(if {growth_checks(32)} then 0 else 3) else (if {growth_checks(64)} then 0 else 4)) else 2) else 1\n')



if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--case', type=int, default=0)
    parser.add_argument('--mode', choices=['trace', 'control', 'capacity', 'growth'], default='trace')
    args = parser.parse_args()
    print(source(args.case, args.mode), end='')
