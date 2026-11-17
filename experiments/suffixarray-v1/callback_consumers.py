"""Generate exact concrete callback consumers from recorded ordinary leftmost results; this supplies no general regex adapter."""
from __future__ import annotations

HEADER = '''-- | Compare concrete empty, optional-byte and greedy-byte callback results with independently recorded byte offsets.
module Main where
  import Data.Index.SuffixArray
  import Data.Bool
  import Data.Bytes
  import Data.List
  import Data.Machine
  import Data.Nat
  import Data.Option
  import Data.Result
  import Data.String
  import Effect.Process

  -- | Compare every public endpoint without a fingerprint or access to private range fields.
  same-ranges :: List SuffixMatch -> List (Nat, Nat) -> Bool
  same-ranges found expected = case found of
    Nil -> case expected of Nil -> True; other -> False
    Cons value more -> case expected of
      Nil -> False
      Cons pair rest -> case pair of
        (start, end) -> if suffix-match-start value == start && suffix-match-end value == end then same-ranges more rest else False

  -- | Compare one complete typed search result without a nested case inside a generated let block.
  accepted-ranges :: Result (SuffixScanError Unit) (List SuffixMatch) -> List (Nat, Nat) -> Bool
  accepted-ranges result expected = case result of
    Err failure -> False
    Ok found -> same-ranges found expected

  -- | Build a range through the same checked public constructor available to any callback.
  found-range :: SuffixIndex -> Nat -> Nat -> Result Unit (Option SuffixMatch)
  found-range index first end = case suffix-match index first end of
    Err failure -> Err ()
    Ok found -> Ok (Some found)

  -- | Observe only one sealed scalar byte; no query creates a mutable backing alias.
  is-a :: SuffixIndex -> Nat -> Bool
  is-a index at = case suffix-index-byte-at index at of
    None -> False
    Some byte -> u8-to-nat byte == 97

  -- | Match an empty range at the requested cursor, including the logical end.
  empty-search :: SuffixIndex -> Nat -> Result Unit (Option SuffixMatch)
  empty-search index cursor = found-range index cursor cursor

  -- | Match one optional ASCII byte greedily, otherwise an empty range at the cursor.
  optional-search :: SuffixIndex -> Nat -> Result Unit (Option SuffixMatch)
  optional-search index cursor = found-range index cursor (if is-a index cursor then cursor + 1 else cursor)

  -- | Finish one maximal run using tail iteration over safe scalar observations.
  run-end :: SuffixIndex -> Nat -> Nat
  run-end index cursor = if is-a index cursor then run-end index (cursor + 1) else cursor

  -- | Match the maximal ASCII run beginning at the cursor, permitting an empty range.
  greedy-search :: SuffixIndex -> Nat -> Result Unit (Option SuffixMatch)
  greedy-search index cursor = found-range index cursor (run-end index cursor)
'''


def consumer(rows: list[dict]) -> str:
    """Construct one subject per input and compare all fifteen exact callback results in separate memory profiles."""
    source = HEADER
    names = {'': 'empty-search', 'a?': 'optional-search', 'a*': 'greedy-search'}
    for number, row in enumerate(rows):
        data = '[' + ','.join(map(str, bytes.fromhex(row['data']))) + ']'
        source += f'\n  -- | Check every selected callback limit for corpus input {number}.\n  case{number} :: Unit -> Int\n  case{number} unit = case suffix-index (bytes-from-list {data}) of\n    Err failure -> 100\n    Ok index ->\n'
        queries = [q for q in row['regex'] if q['pattern'] in names]
        for at, query in enumerate(queries):
            limit = 'AllSuffixMatches' if query['limit'] < 0 else f'(AtMostSuffixMatches {query["limit"]})'
            spans = '[' + ','.join(f'({start},{end})' for start, end in (query['leftmost'] or [])) + ']'
            source += f'      let q{at} = accepted-ranges (suffix-index-find-all index {names[query["pattern"]]} {limit}) {spans} in\n'
        source += '      ' + ''.join(f'if not q{at} then {at + 1} else ' for at in range(len(queries))) + '0\n'
    dispatch = '101'
    for number in reversed(range(len(rows))):
        dispatch = f'if number == "{number}" then case{number} () else {dispatch}'
    source += f'''\n  -- | Select one bounded input and keep every poison quarantine independent.
  main :: Unit ->{{IO r}} Int
  main unit = with (io = newIO ()) in case process-args () of
    Ok (Cons number Nil) -> {dispatch}
    other -> 102
'''
    return source
