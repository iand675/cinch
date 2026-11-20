#!/usr/bin/env python3
"""Render direct immutable-text UTF-8 observations with shallow fixture groups."""
from __future__ import annotations

import json
from pathlib import Path
from generate_unicode_tables import suffix

ROOT = Path(__file__).resolve().parents[1]


def literal(encoded: str) -> str:
    """Encode valid fixture scalars canonically, avoiding byte-list proof ASTs and host surrogates."""
    text = bytes.fromhex(encoded).decode('utf-8')
    return '"' + ''.join('\\u{' + format(ord(value), 'X') + '}' for value in text) + '"'


def arguments(row: dict) -> str:
    """Keep expected scalar, width and count values independent of the implementation under test."""
    values = [row[key] for key in ['first', 'first_width', 'last', 'last_width', 'count', 'full', 'valid']]
    return ' '.join('True' if value is True else 'False' if value is False else str(value) for value in values)


def rendered() -> str:
    """Check direct SDK rows, copied-input alias isolation, large runtime text and empty status."""
    fixture = json.loads((ROOT / 'seed/tests/fixtures/utf8-text-vectors.json').read_text())
    source = '''-- | Independent immutable-text scalar observations, sealed validity and copy isolation.
module Main where
  import Text.Utf8
  import Data.Bytes
  import Data.Bool
  import Data.Char
  import Data.List
  import Data.Nat
  import Data.Option
  import Data.Result
  import Data.String hiding (utf8-encode-into)
  import Effect.Process

  -- | Check one exact scalar, consumed width and empty-versus-valid status independently.
  step :: Utf8Step -> Nat -> Nat -> Bool
  step value scalar width = char-code (utf8-step-char value) == scalar
    && utf8-step-width value == width
    && (if width == 0 then (case utf8-step-status value of Utf8End -> True; other -> False)
        else (case utf8-step-status value of Utf8Valid -> True; other -> False))

  -- | Check the String helper's own tuple metadata independently of the safe step factory.
  helper-step :: Option (Char, Nat) -> Nat -> Nat -> Bool
  helper-step observed scalar width = case observed of
    None -> scalar == 65533 && width == 0
    Some pair -> case pair of (cp, actual) -> char-code cp == scalar && actual == width

  -- | Observe all five public text operations without converting the immutable backing to Bytes.
  observe :: String -> Nat -> Nat -> Nat -> Nat -> Nat -> Bool -> Bool -> Bool
  observe text first first-width last last-width count full valid =
    helper-step (string-utf8-first text) first first-width
      && helper-step (string-utf8-last text) last last-width
      && step (utf8-first-text text) first first-width
      && step (utf8-last-text text) last last-width
      && utf8-count-text text == count
      && (if full then utf8-full-rune-text text else not (utf8-full-rune-text text))
      && (if valid then utf8-valid-text text else not (utf8-valid-text text))

  -- | Mutate a caller-owned buffer, leaving empty buffers unchanged.
  change :: Bytes -> Unit
  change bytes = if bytes-len bytes > 0 then bytes-set bytes 0 255 else ()

  -- | Mutating both conversion copies preserves the sealed snapshot and every scalar observation.
  check :: String -> Nat -> Nat -> Nat -> Nat -> Nat -> Bool -> Bool -> Bool
  check input first first-width last last-width count full valid =
    let bytes = string-to-bytes input in case string-from-bytes bytes of
      None -> False
      Some text ->
        let initial = observe text first first-width last last-width count full valid in
        let copy = string-to-bytes text in let a = change bytes in let b = change copy in
        initial && observe text first first-width last last-width count full valid
          && string-compare input text == 0

  -- | Fold only shallow fixture batches, keeping the compiler host stack bounded.
  all-checks :: List Bool -> Bool
  all-checks values = case values of Nil -> True; Cons value rest -> value && all-checks rest

'''
    groups = []
    checks = ['check ' + literal(row['input']) + ' ' + arguments(row) for row in fixture['rows']]
    for index, start in enumerate(range(0, len(checks), 16)):
        name = 'rows-' + suffix(index)
        source += '\n  -- | Check independent short-text rows and both conversion aliases.\n  ' + name + ' :: Unit -> Bool\n  ' + name + ' unit = all-checks [' + ', '.join(checks[start:start + 16]) + ']\n'
        groups.append(name)
    for index, row in enumerate(fixture.get('long', [])):
        name = 'long-' + suffix(index)
        source += '\n  -- | Build large valid text at runtime and check its exact independent observations.\n  ' + name + ' :: Unit -> Bool\n  ' + name + ' unit = let text = string-repeat ' + str(row['repeats']) + ' ' + literal(row['pattern']) + ' in check text ' + arguments(row['observation']) + '\n'
        groups.append(name)
    source += '\n  -- | Select one poisoned-memory fixture group, preserving the same public pipeline.\n  selected :: String -> Bool\n  selected name =\n'
    for index, name in enumerate(groups):
        source += '    ' + ('if' if index == 0 else 'else if') + ' name == "' + name + '" then ' + name + ' ()\n'
    source += '    else False\n'
    block_calls = []
    for index, start in enumerate(range(0, len(groups), 8)):
        name = 'block-' + suffix(index)
        source += '\n  -- | Check a shallow bounded batch of independent fixture groups.\n  ' + name + ' :: Unit -> Bool\n  ' + name + ' unit = all-checks [' + ', '.join(group + ' ()' for group in groups[start:start + 8]) + ']\n'
        block_calls.append(name + ' ()')
    source += '\n  -- | Check all rows normally or one exact group with poisoned frees.\n  main :: Unit ->{IO r} Int\n  main unit = with (io = newIO ()) in case process-args () of\n    Err error -> 2\n    Ok args -> case args of\n      Nil -> if all-checks [' + ', '.join(block_calls) + '] then 0 else 1\n      Cons name Nil -> if selected name then 0 else 1\n      other -> 2\n'
    return source


def groups() -> list[str]:
    """List the exact normal/poison selection names emitted by the current reference fixture."""
    fixture = json.loads((ROOT / 'seed/tests/fixtures/utf8-text-vectors.json').read_text())
    return ['rows-' + suffix(index) for index in range((len(fixture['rows']) + 15) // 16)] + ['long-' + suffix(index) for index in range(len(fixture.get('long', [])))]


if __name__ == '__main__':
    print(rendered(), end='')
