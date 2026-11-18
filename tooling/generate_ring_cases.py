#!/usr/bin/env python3
"""Project independent circular-sequence traces into documented standalone consumers."""
from __future__ import annotations

import argparse
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
FIXTURES = ROOT / 'compiler/tests/fixtures'


def source(index: int, mode: str = 'trace') -> str:
    """Render offline observations without consulting implementation output or the runtime SDK."""
    if mode == 'kernel':
        from generate_ring_kernel_cases import source as kernel_source
        return kernel_source()
    if mode == 'control':
        return (FIXTURES / 'ring-control.cinch').read_text()
    row = json.loads((FIXTURES / 'ring-vectors.json').read_text())[index]
    consumer = (FIXTURES / 'ring-consumer.cinch').read_text()
    if mode == 'window':
        consumer = window_consumer(consumer)
    steps = 'Nil'
    for operation in reversed(row['operations']):
        values = ' '.join(str(operation[name]) if operation[name] >= 0 else f'({operation[name]})'
                          for name in ['kind', 'source', 'other', 'result', 'distance', 'count',
                                       'value', 'returned', 'visit', 'state'])
        steps = f'(Cons (Step {values}) {steps})'
    if mode == 'window':
        return consumer + window_main(row['initial'], len(row['operations']), steps)
    return consumer + (
        '\n  -- | Verify one complete independently recorded mutation trace.\n'
        '  main :: Unit ->{Mut r} Int\n'
        f'  main unit = if trace-case {row["initial"]} {steps} then 0 else 1\n'
    )


def window_consumer(consumer: str) -> str:
    """Replay every action while bounding only exhaustive state observations for poisoned quarantine."""
    replacements = {
        '  import Data.Result\n': '  import Data.Result\n  import Data.String (unpack)\n  import Effect.Process (process-args)\n',
        '  -- | Require every exact result, callback observation and post-action state in the original sequence.':
            '  -- | Check every replayed result and callback, and complete post-states only in the selected observation window.',
        'steps-match :: forall r. World r -> List Step ->{Mut r} Bool':
            'steps-match :: forall r. World r -> Nat -> Nat -> List Step ->{Mut r} Bool',
        'steps-match world steps = case steps of\n    Nil -> True':
            'steps-match world exempt remaining steps = if remaining == 0 then True else case steps of\n    Nil -> False',
        'eq-u64 (state-hash world) expected-state && steps-match world rest':
            '(if exempt == 0 then eq-u64 (state-hash world) expected-state else True) && steps-match world (if exempt == 0 then 0 else exempt - 1) (remaining - 1) rest',
        'trace-case :: forall r. Nat -> List Step ->{Mut r} Bool':
            'trace-case :: forall r. Nat -> List Step -> Nat -> Nat ->{Mut r} Bool',
        'trace-case count steps =': 'trace-case count steps exempt remaining =',
        '&& steps-match world steps': '&& steps-match world exempt remaining steps',
    }
    for before, after in replacements.items():
        if consumer.count(before) != 1:
            raise ValueError(f'window projection requires one exact fixture fragment: {before!r}')
        consumer = consumer.replace(before, after)
    return consumer


def window_main(initial: int, count: int, steps: str) -> str:
    """Select one bounded observation window at runtime so all windows share one frozen executable."""
    return (
        '\n  -- | Decode bounded decimal fixture arguments before replaying the independent trace.\n'
        '  window-number :: List Char -> Nat -> Nat\n'
        '  window-number chars value = case chars of Nil -> value; Cons first rest -> case digit-value first of None -> 0; Some digit -> window-number rest (value * 10 + digit)\n'
        '\n  -- | Replay the complete prefix and observe only the requested half-open action window.\n'
        '  main :: Unit ->{IO r, Mut r} Int\n'
        '  main unit = with (io = newIO ()) in case process-args () of\n'
        '    Ok (Cons first (Cons last Nil)) ->\n'
        '      let start = window-number (unpack first) 0 in\n'
        '      let end = window-number (unpack last) 0 in\n'
        f'      if start < end && end <= {count} then (if trace-case {initial} {steps} start end then 0 else 1) else 2\n'
        '    other -> 2\n'
    )


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--case', type=int, default=0)
    parser.add_argument('--mode', choices=['trace', 'control', 'kernel', 'window'], default='trace')
    args = parser.parse_args()
    print(source(args.case, args.mode), end='')
