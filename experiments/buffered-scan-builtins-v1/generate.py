#!/usr/bin/env python3
"""Prepare an isolated span-native built-in scanner candidate while preserving frozen custom callback semantics."""
from pathlib import Path
import hashlib
import json

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]


def digest(data):
    """Record exact original and candidate source bytes independently of later qualification artifacts."""
    return hashlib.sha256(data).hexdigest()


def main():
    """Generate only private experiment copies; never edit the maintained prelude or frozen benchmark inputs."""
    output = HERE/'sources'
    output.mkdir(exist_ok=True)
    split_path = ROOT/'compiler/prelude/Effect/Scanner/Split.cinch'
    scanner_path = ROOT/'compiler/prelude/Effect/Scanner.cinch'
    assert digest(split_path.read_bytes()) == '5bcdaea29e3d7bcc9a7f77aaaad39e5fcb8af0504862e891a4d853f2e8e2c48e'
    assert digest(scanner_path.read_bytes()) == 'dd6d190f2b53e1fb80b10e48d64a29363b82041895660675d2b383c637be06aa'
    split, scanner = split_path.read_text(), scanner_path.read_text()
    assert 'case scan-apply-between split owner start end terminal of' in scanner
    assert 'case split of\n      Splitter callback' in split
    split = split.replace('  export scan-bytes, scan-runes, scan-lines, scan-words',
                          '  export scan-bytes, scan-runes, scan-lines, scan-words\n  export scan-byte-split, scan-rune-split, scan-line-split, scan-word-split', 1)
    split = split.replace('    Splitter :: (Bytes -> Bool -> ScanStep) -> ScanSplit', '''    Splitter :: (Bytes -> Bool -> ScanStep) -> ScanSplit
    -- | Private byte tokenization reads only the checked owner span and returns fresh tokens.
    ByteSplitter :: ScanSplit
    -- | Private scalar tokenization retains exact incomplete and malformed-byte semantics.
    RuneSplitter :: ScanSplit
    -- | Private line tokenization scans the bounded owner without callback input snapshots.
    LineSplitter :: ScanSplit
    -- | Private Unicode whitespace tokenization preserves arbitrary non-whitespace bytes.
    WordSplitter :: ScanSplit''', 1)
    split = split.replace('  -- | Invoke a split capability with a fresh input snapshot and snapshot any mutable returned token.',
                          '  -- | Invoke a built-in on a checked span, or isolate a custom callback with fresh input and returned-token snapshots.', 1)
    split = split.replace('  -- | Invoke a splitter over a checked owner span with exactly one callback-input snapshot and one retained-token snapshot.',
                          '  -- | Invoke a splitter over a checked owner span; built-ins return fresh tokens without copying the remaining input.\n  -- Custom callbacks retain exactly one input snapshot and one returned-token snapshot.', 1)
    split = split.replace('    case split of\n      Splitter callback', '''    case split of
      ByteSplitter -> scan-byte-between input start end
      RuneSplitter -> scan-rune-between input start end terminal
      LineSplitter -> scan-line-between input start end terminal
      WordSplitter -> scan-word-between input start end terminal
      Splitter callback''', 1)
    split += '''

  -- | A first-class byte splitter reads the bounded scanner span and returns one independently owned byte token.
  scan-byte-split :: ScanSplit
  scan-byte-split = ByteSplitter

  -- | A first-class scalar splitter preserves incomplete-input requests and per-byte malformed replacement without copying the unread span.
  scan-rune-split :: ScanSplit
  scan-rune-split = RuneSplitter

  -- | A first-class line splitter uses packed delimiter scans and returns fresh CR-trimmed tokens.
  scan-line-split :: ScanSplit
  scan-line-split = LineSplitter

  -- | A first-class word splitter uses Unicode whitespace and packed ASCII runs while retaining arbitrary token bytes.
  scan-word-split :: ScanSplit
  scan-word-split = WordSplitter

  -- | Return one fresh byte without exposing or copying the rest of the checked unread owner span.
  scan-byte-between :: Bytes -> Nat -> Nat -> ScanStep
  scan-byte-between input start end = if start < end && end <= bytes-len input then
    ScanStep 1 (Some (part input start 1)) None
    else ScanStep 0 None None

  -- | Split a scalar inside absolute bounds; inspect at most four copied fringe bytes only for nonterminal non-ASCII fullness.
  scan-rune-between :: Bytes -> Nat -> Nat -> Bool -> ScanStep
  scan-rune-between input start end terminal = if start < end && end <= bytes-len input then
    if bytes-get input start < 128 then ScanStep 1 (Some (part input start 1)) None
    else
      let remaining = end - start in let fringe = if remaining < 4 then remaining else 4 in
      if not terminal && not (Utf8.utf8-full-rune (part input start fringe)) then ScanStep 0 None None
      else let step = Utf8.utf8-decode-between input start end start in let width = Utf8.utf8-step-width step in
        case Utf8.utf8-step-status step of
          Utf8.Utf8Invalid -> ScanStep 1 (Some (Utf8.utf8-encode-rune 65533)) None
          other -> if width > 0 && start + width <= end then ScanStep (nat-to-int width) (Some (part input start width)) None
            else ScanStep 0 None (Some (ScanCustomError "invalid private scalar width"))
    else ScanStep 0 None None

  -- | Copy a complete bounded line token once, excluding one optional final carriage return.
  line-token-between :: Bytes -> Nat -> Nat -> Bytes
  line-token-between input start end = if start <= end && end <= bytes-len input then
    if start < end then let last = end - 1 in
      if last < bytes-len input && bytes-get input last == 13 && start <= last then part input start (last - start)
      else part input start (end - start)
    else bytes-new 0
    else bytes-new 0

  -- | Find LF with packed bounded lanes and translate the absolute endpoint into a relative consumed count.
  scan-line-between :: Bytes -> Nat -> Nat -> Bool -> ScanStep
  scan-line-between input start end terminal = if start <= end && end <= bytes-len input then
    case Scan.find-byte input start end 10 of
      Some at -> if start <= at then ScanStep (nat-to-int (at + 1 - start)) (Some (line-token-between input start at)) None
        else ScanStep 0 None (Some (ScanCustomError "invalid private line offset"))
      None -> if terminal && start < end then ScanStep (nat-to-int (end - start)) (Some (line-token-between input start end)) None
        else ScanStep 0 None None
    else ScanStep 0 None (Some (ScanCustomError "invalid private line interval"))

  -- | Skip only complete Unicode whitespace within the logical endpoint, with packed ASCII runs and scalar fringes.
  span-skip-space :: Bytes -> Nat -> Nat -> Nat
  span-skip-space input at end = let next = Scan.ascii-spaces input at end in
    if next < end && end <= bytes-len input then
      let step = Utf8.utf8-decode-between input at end next in let width = Utf8.utf8-step-width step in
      if unicode-is-space (Utf8.utf8-step-char step) && width > 0 && next + width <= end then span-skip-space input (next + width) end else next
    else next

  -- | Find a bounded word ending while reporting advances relative to the originally supplied span.
  span-word-end :: Bytes -> Nat -> Nat -> Nat -> Nat -> Bool -> ScanStep
  span-word-end input origin start at end terminal = let next = Scan.ascii-word input at end in
    if origin <= start && start <= next && next <= end && end <= bytes-len input then
      if next < end then
        let step = Utf8.utf8-decode-between input origin end next in let width = Utf8.utf8-step-width step in
        if width > 0 && next + width <= end then
          if unicode-is-space (Utf8.utf8-step-char step) then
            ScanStep (nat-to-int (next + width - origin)) (Some (part input start (next - start))) None
          else span-word-end input origin start (next + width) end terminal
        else ScanStep (nat-to-int (start - origin)) None (Some (ScanCustomError "invalid private word width"))
      else if terminal && start < end then ScanStep (nat-to-int (end - origin)) (Some (part input start (end - start))) None
      else ScanStep (nat-to-int (start - origin)) None None
    else ScanStep 0 None (Some (ScanCustomError "invalid private word interval"))

  -- | Split a nonempty word without copying skipped whitespace or the unread suffix, preserving raw malformed token bytes.
  scan-word-between :: Bytes -> Nat -> Nat -> Bool -> ScanStep
  scan-word-between input start end terminal = if start <= end && end <= bytes-len input then
    let first = span-skip-space input start end in span-word-end input start first first end terminal
    else ScanStep 0 None (Some (ScanCustomError "invalid private word interval"))
'''
    scanner = scanner.replace('Scanning source (scan-split scan-lines) 65536', 'Scanning source scan-line-split 65536', 1)
    files = {'Effect.Scanner.Split.cinch': (split_path, split), 'Effect.Scanner.cinch': (scanner_path, scanner)}
    manifest = {'status': 'Static isolated candidate only; no source/runtime/performance qualification yet.',
                'ownership': 'Custom callback input and returned-token snapshots remain unchanged; private built-ins read bounded owner spans and publish fresh tokens only.',
                'public_additions': ['scan-byte-split', 'scan-rune-split', 'scan-line-split', 'scan-word-split'], 'sources': {}}
    for name, (original, candidate) in files.items():
        (output/name).write_text(candidate)
        manifest['sources'][name] = {'old_path': str(original.relative_to(ROOT)), 'old_sha256': digest(original.read_bytes()), 'new_sha256': digest(candidate.encode())}
    (HERE/'candidate.json').write_text(json.dumps(manifest, indent=2)+'\n')
    print('Prepared isolated trusted built-in scanner copies; maintained prelude and frozen measurements unchanged')


if __name__ == '__main__':
    main()
