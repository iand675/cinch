#!/usr/bin/env python3
"""Render wrong-codepoint, selector, locale, rule-observation and checked-refusal Unicode fixtures."""
from __future__ import annotations
import json
from pathlib import Path
from generate_unicode_tables import suffix

ROOT = Path(__file__).resolve().parents[1]


def rendered() -> str:
    """Generate shallow bounded public-API checks from independent SDK records."""
    fixture = json.loads((ROOT / 'seed/tests/fixtures/unicode-vectors.json').read_text())
    scalar = (ROOT / 'seed/tests/fixtures/unicode-kernel-consumer.cinch.in').read_text()
    flag_start = scalar.index('  -- | Turn one public predicate')
    flag_end = scalar.index('  -- | Observe every public char classification')
    hash_start = scalar.index('  -- | Observe every byte of a U32')
    hash_end = scalar.index('  -- | Observe every byte of a complete packed')
    source = '''-- | Independent numeric boundaries, exact case observations and custom rule validation.
module Main where
  import Data.Unicode.Kernel
  import Data.Bool
  import Data.Char
  import Data.List
  import Data.Machine
  import Data.Nat
  import Data.Option
  import Data.Result

  -- | Keep reference inputs in the executable rather than constant-evaluating the corpus.
  prim runtime-input :: U32 -> U32 where
    llvm {
      %value = or i32 %0, 0
      ret i32 %value
    }

''' + '''  -- | Fold a bounded fixture batch without a polymorphic container predicate.
  all-checks :: List Bool -> Bool
  all-checks values = case values of Nil -> True; Cons value rest -> value && all-checks rest

''' + scalar[flag_start:flag_end]+scalar[hash_start:hash_end]+'''
  -- | Check all public numeric observations for negative, surrogate and out-of-domain inputs.
  numeric-edge :: U32 -> U32 -> U32 -> U32 -> U32 -> U32 -> Bool
  numeric-edge input flags upper lower title fold =
    let raw = runtime-input input in let cp = wrapping-u32-to-i32 raw in
    eq-u32 (flags-code cp) flags && eq-u32 (wrapping-i32-to-u32 (unicode-to-upper-code cp)) upper
      && eq-u32 (wrapping-i32-to-u32 (unicode-to-lower-code cp)) lower
      && eq-u32 (wrapping-i32-to-u32 (unicode-to-title-code cp)) title
      && eq-u32 (wrapping-i32-to-u32 (unicode-simple-fold-code cp)) fold

  -- | Check signed selector policy independently of numeric code-point validity.
  selector-edge :: U32 -> U32 -> U32 -> Bool
  selector-edge which input expected =
    let selector = wrapping-u32-to-i32 (runtime-input which) in
    let cp = wrapping-u32-to-i32 (runtime-input input) in
    eq-u32 (wrapping-i32-to-u32 (unicode-to-code selector cp)) expected

  -- | Construct the exact small independent override sets used by the reference corpus.
  special :: U32 -> Result UnicodeCaseError UnicodeSpecialCase
  special which =
    if eq-u32 which 0 then Ok unicode-turkish-case
    else if eq-u32 which 1 then Ok unicode-azeri-case
    else if eq-u32 which 2 then unicode-special-case Nil
    else if eq-u32 which 3 then unicode-special-case [MkUnicodeCaseRange 65 90 0 0 0]
    else if eq-u32 which 4 then unicode-special-case [MkUnicodeCaseRange 256 257 1114112 1114112 1114112]
    else unicode-special-case [MkUnicodeCaseRange 233 233 (-145) (-145) (-145)]

  -- | Observe actual opaque override methods, including explicit identity and absent-rule fallback.
  special-edge :: U32 -> U32 -> U32 -> U32 -> U32 -> Bool
  special-edge which input upper lower title =
    let cp = wrapping-u32-to-i32 (runtime-input input) in case special which of
      Err error -> False
      Ok rules -> eq-u32 (wrapping-i32-to-u32 (unicode-special-to-upper-code rules cp)) upper
        && eq-u32 (wrapping-i32-to-u32 (unicode-special-to-lower-code rules cp)) lower
        && eq-u32 (wrapping-i32-to-u32 (unicode-special-to-title-code rules cp)) title

  -- | Hash every exact standard case bound and signed delta through its public observation getters.
  rule-hash :: Nat -> U64 -> U64
  rule-hash index hash = if index < unicode-case-range-count then (case unicode-case-range-at index of
    None -> 0
    Some rule ->
      let a = hash-word (unicode-case-range-low rule) 0 hash in
      let b = hash-word (unicode-case-range-high rule) 0 a in
      let c = hash-word (wrapping-i32-to-u32 (unicode-case-range-upper rule)) 0 b in
      let d = hash-word (wrapping-i32-to-u32 (unicode-case-range-lower rule)) 0 c in
      let e = hash-word (wrapping-i32-to-u32 (unicode-case-range-title rule)) 0 d in
      rule-hash (index + 1) e)
    else hash

  -- | Check named refusal variants without relying on formatted diagnostic wording.
  error-code :: UnicodeCaseError -> U32
  error-code error = case error of UnicodeCaseInvalidBounds index -> 1; UnicodeCaseInvalidOrder index -> 2; UnicodeCaseInvalidMapping index -> 3; UnicodeCaseInvalidAlternating index -> 4; UnicodeCaseTooManyRules index -> 5

  -- | Validate complete custom sets before mutation or raw pointer traversal becomes possible.
  rejected :: List UnicodeCaseRange -> U32 -> Nat -> Bool
  rejected rules kind index = case unicode-special-case rules of Ok value -> False; Err error -> eq-u32 (error-code error) kind && unicode-case-error-index error == index

  -- | Exercise explicit checked refusals, identity priority, indexed boundaries and ASCII fast-path capability.
  contracts :: Unit -> Bool
  contracts unit =
    rejected [MkUnicodeCaseRange 90 65 0 0 0] 1 0
      && rejected [MkUnicodeCaseRange 55296 55296 0 0 0] 1 0
      && rejected [MkUnicodeCaseRange 65 66 0 0 0, MkUnicodeCaseRange 66 67 0 0 0] 2 1
      && rejected [MkUnicodeCaseRange 0 0 (-1) 0 0] 3 0
      && rejected [MkUnicodeCaseRange 55295 55295 1 1 1] 3 0
      && rejected [MkUnicodeCaseRange 1114111 1114111 1 1 1] 3 0
      && rejected [MkUnicodeCaseRange 256 257 1114112 0 1114112] 4 0
      && rejected [MkUnicodeCaseRange 55295 55295 1114112 1114112 1114112] 4 0
      && (case unicode-case-range-at unicode-case-range-count of None -> True; Some rule -> False)
      && (case unicode-case-range-at 18446744073709551616 of None -> True; Some rule -> False)
      && unicode-special-case-count unicode-turkish-case == 4
      && unicode-special-affects-ascii unicode-turkish-case
      && (case special 2 of Err error -> False; Ok rules -> unicode-special-case-count rules == 0 && not (unicode-special-affects-ascii rules))
      && (case special 4 of Err error -> False; Ok rules -> unicode-special-case-count rules == 1 && not (unicode-special-affects-ascii rules) && (case unicode-special-case-range-at rules 1 of None -> True; Some rule -> False))
      && unicode-equal-fold 'K' '\\u{212A}' && unicode-equal-fold '\\u{3A3}' '\\u{3C2}'
      && not (unicode-equal-fold 'I' '\\u{131}') && not (unicode-equal-fold 'i' '\\u{130}')
      && unicode-equal-fold-code (-1) (-1) && not (unicode-equal-fold-code (-1) 65533)
'''
    checks = []
    for row in fixture['numeric_edges']:
        raw = row['input'] & 0xFFFFFFFF
        checks.append('numeric-edge ' + ' '.join(str(value) for value in [raw, *row['words']]))
        checks.extend('selector-edge ' + ' '.join(str(value & 0xFFFFFFFF) for value in [which, row['input'], result]) for which, result in row['selectors'])
    kinds = {'turkish': 0, 'azeri': 1, 'empty': 2, 'identity': 3, 'alternating': 4, 'custom': 5}
    for row in fixture['special']:
        checks.append('special-edge ' + ' '.join(str(value & 0xFFFFFFFF) for value in [kinds[row['name']], row['input'], row['upper'], row['lower'], row['title']]))
    calls = []
    for index, start in enumerate(range(0, len(checks), 16)):
        name = 'edge-batch-' + suffix(index)
        source += f'\n  -- | Check one shallow batch of independent public numeric/locale observations.\n  {name} :: Unit -> Bool\n  {name} unit = all-checks [' + ', '.join(checks[start:start+16]) + ']\n'
        calls.append(name+' ()')
    digest = 14695981039346656037
    for row in fixture['case_ranges']:
        for value in row:
            for byte in (value & 0xFFFFFFFF).to_bytes(4, 'little'):
                digest = ((digest ^ byte) * 1099511628211) & ((1<<64)-1)
    source += '\n  -- | Verify all independent boundary observations and checked refusal contracts.\n  main :: Int\n  main = if contracts () && eq-u64 (rule-hash 0 14695981039346656037) '+str(digest)+' && all-checks ['+', '.join(calls)+'] then 0 else 1\n'
    return source


if __name__ == '__main__':
    print(rendered(), end='')
