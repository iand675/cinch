#!/usr/bin/env python3
"""Measure public compiled substring pipelines with changed inputs and independent retained-owner observations."""
from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import statistics
import sys

from bench_stdlib import source_closure
from validate_byte_view import digest, run

ROOT = Path(__file__).resolve().parents[1]
DRIVER = r'''
  -- | Parse validated benchmark sizes before timing.
  decimal :: String -> Nat
  decimal text = decimal-from (unpack text) 0
  -- | Traverse decimal digits without recursive return frames.
  decimal-from :: List Char -> Nat -> Nat
  decimal-from chars value = case chars of
    Nil -> value
    Cons first rest -> case digit-value first of
      None -> 0
      Some digit -> let next = value * 10 + digit in decimal-from rest next
  -- | Select reproducible raw inputs outside the measured interval.
  bench-input :: String -> Nat -> Bytes
  bench-input name size = input (if name == "sparse" then 0 else if name == "dense" then 3 else if name == "prefix" then 4 else 6) size
  -- | Construct one exact independent compiled needle before timing.
  bench-needle :: String -> Bytes
  bench-needle name = if name == "prefix" then input 4 257
    else if name == "dense" then input 3 17 else if name == "periodic" then input 6 17
    else let raw = bytes-new 17 in let written = if 0 < bytes-len raw then bytes-set raw 0 254 else () in raw
  -- | Represent an absent match with an independent length-plus-one sentinel.
  offset :: ByteView -> Option Nat -> Nat
  offset view result = case result of None -> byte-view-length view + 1; Some at -> at
  -- | Mutate one checked input byte before every public query so repeated work cannot be hoisted.
  mutate :: String -> Bytes -> Nat -> Unit
  mutate name raw i = let middle = nat-div (bytes-len raw) 2 in
    if middle < bytes-len raw then
      let odd = nat-rem i 2 == 1 in
      let byte = ((if name == "sparse" then (if odd then 255 else 254)
        else if name == "prefix" then (if odd then 97 else 98)
        else if odd then 255 else fixture-byte (if name == "dense" then 3 else 6) (bytes-len raw) middle) :: InRange 0 255 Nat) in
      bytes-set raw middle byte
    else ()
  -- | Observe complete first, overlapping-last, nonoverlapping-count and retained cut projections.
  operation :: String -> ByteView -> BytePattern -> Nat
  operation kind view pattern = if kind == "index" then offset view (byte-view-index-pattern view pattern)
    else if kind == "last" then offset view (byte-view-last-index-pattern view pattern)
    else if kind == "count" then byte-view-count-pattern view pattern
    else let first = offset view (byte-view-index-pattern view pattern) in
      let last = offset view (byte-view-last-index-pattern view pattern) in
      let count = byte-view-count-pattern view pattern in
      let cut = case byte-view-cut-last-pattern view pattern of None -> 0; Some (before, after) -> byte-view-length before * 17 + byte-view-length after * 31 in
      first + last * 257 + count * 65537 + cut
        + (if byte-view-has-prefix-pattern view pattern then 1048576 else 0)
        + (if byte-view-has-suffix-pattern view pattern then 16777216 else 0)
  -- | Sequence checked mutations and public observations in a tail loop with retained owners.
  work :: String -> String -> Bytes -> ByteView -> BytePattern -> Nat -> Nat -> Nat -> Nat
  work kind name raw view pattern left i total = if left == 0 then total else
    let changed = mutate name raw i in let value = operation kind view pattern in
    let accumulated = nat-rem (total + value) 1000000007 in
    let remaining = left - 1 in let following = i + 1 in work kind name raw view pattern remaining following accumulated
  -- | Measure only changed public queries; all input setup, digests and retained-owner teardown are outside timing.
  measure :: forall r. using (io :: Io r) => String -> String -> Nat -> Nat ->{IO r} Int
  measure kind name size iterations = let raw = bench-input name size in let view = bytes-view raw in
    let needle = bench-needle name in let pattern = bytes-compile-pattern needle in
    let original = hex-encode (sha256-bytes raw) in
    let start = clock-monotonic-ns () in let checksum = work kind name raw view pattern iterations 0 0 in
    let end = clock-monotonic-ns () in let retained = hex-encode (sha256-bytes raw) in
    let stable = hex-encode (sha256-bytes (byte-pattern-bytes pattern)) in
    let out = string-concat-all [nat-to-string (saturating-sub end start)," ",nat-to-string checksum," ",original," ",retained," ",stable,"\n"] in
    let printed = ioWrite (string-to-bytes out) in 0
  -- | Resolve runtime workload arguments before all clock readings.
  main :: Unit ->{IO r} Int
  main unit = with (io = newIO ()) in case process-args () of
    Ok (Cons kind (Cons name (Cons size (Cons iterations Nil)))) -> measure kind name (decimal size) (decimal iterations)
    other -> 2
'''


def source() -> str:
    """Reuse the fully checked runtime recipes without copying production private representations."""
    helpers = (ROOT / 'compiler/tests/fixtures/byte-view-consumer.cinchpart').read_text()
    helpers = helpers[:helpers.index('  -- | Compare an optional relative byte offset')]
    return helpers.replace('  import Data.String\n', '  import Data.String\n  import Data.Char\n  import Effect.Clock\n  import Text.Hex\n') + DRIVER


def scalar_source(text: str) -> str:
    """Keep the same linear algorithm and owners while routing match scans through scalar bytes."""
    first = text.index('  -- | Find a matching first byte')
    last = text.index('  -- | Read only remaining scalar bytes', first)
    text = text[:first] + '''  -- | Route first-byte scans through the equivalent exact scalar loop.
  index-forward :: I64 -> I64 -> I64 -> U64 -> I64
  index-forward base at end repeated = let byte = word-byte repeated in index-forward-tail base at end byte

''' + text[last:]
    conditions = ['offset-at-least left 8 && offset-at-least needed 8 && same-eight base at needle matched',
                  'block-before matched size && block-before at end && same-eight base at needle matched',
                  'block-before matched size && (let left = offset-sub end at in offset-at-least left 8) && same-eight base at needle matched']
    assert sum(text.count(condition) for condition in conditions) == 3
    for condition in conditions:
        text = text.replace(condition, 'False')
    marker = '  -- | Filter only the initial start region, then retain the complete linear failure traversal.'
    if marker in text:
        first = text.index(marker)
        last = text.index('  -- | Extend a matched prefix', first)
        text = text[:first] + '''  -- | Route initial starts directly through the equivalent scalar linear traversal.
  search-first-candidates :: I64 -> I64 -> I64 -> I64 -> I64 -> I64 -> U64 -> I64
  search-first-candidates base begin end needle size table repeated = search-first base begin end needle size table repeated 0

''' + text[last:]
    return text


def expected(kind: str, name: str, size: int, iterations: int) -> tuple[int, str, str, str]:
    """Model exact changed raw input, full public results and both retained owners independently."""
    from validate_byte_view import input_bytes
    mode = {'sparse': 0, 'dense': 3, 'prefix': 4, 'periodic': 6}[name]
    raw = bytearray(input_bytes(mode, size))
    needle = input_bytes(4, 257) if name == 'prefix' else input_bytes(mode, 17)
    if name == 'sparse':
        needle = b'\xfe' + bytes(16)
    original = hashlib.sha256(raw).hexdigest()
    total = 0
    for i in range(iterations):
        middle = size // 2
        raw[middle] = (255 if i % 2 else 254) if name == 'sparse' else (97 if i % 2 else 98) if name == 'prefix' else 255 if i % 2 else input_bytes(mode, middle + 1)[middle]
        first, last, count = raw.find(needle), raw.rfind(needle), raw.count(needle)
        cut = last * 17 + (size - last - len(needle)) * 31 if last >= 0 else 0
        first = first if first >= 0 else size + 1
        last = last if last >= 0 else size + 1
        value = first if kind == 'index' else last if kind == 'last' else count if kind == 'count' else first + last * 257 + count * 65537 + cut + (1048576 if raw.startswith(needle) else 0) + (16777216 if raw.endswith(needle) else 0)
        total = (total + value) % 1000000007
    return total, original, hashlib.sha256(raw).hexdigest(), hashlib.sha256(needle).hexdigest()


def main() -> None:
    """Build/preflight on a functional CPU and sample only during a separately coordinated timing window."""
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument('--compiler', type=Path, required=True)
    p.add_argument('--module-source', type=Path, default=ROOT / 'compiler/prelude/Data/Bytes/View.cinch')
    p.add_argument('--baseline-source', type=Path)
    p.add_argument('--before-source', type=Path)
    p.add_argument('--native-interfaces', type=Path)
    p.add_argument('--artifacts', type=Path, required=True)
    p.add_argument('--output', type=Path, required=True)
    p.add_argument('--build-only', action='store_true')
    p.add_argument('--measure-only', action='store_true')
    p.add_argument('--preflight-only', action='store_true')
    p.add_argument('--samples', type=int, default=5)
    p.add_argument('--cpu', type=int, default=0)
    args = p.parse_args()
    driver = source()
    module = args.module_source.read_text()
    baseline = args.baseline_source.read_text() if args.baseline_source else module
    report = {'source_sha256': hashlib.sha256(module.encode()).hexdigest(),
              'driver_sha256': hashlib.sha256(driver.encode()).hexdigest(),
              'scalar_sha256': hashlib.sha256(scalar_source(baseline).encode()).hexdigest(),
              'compiler_sha256': digest(args.compiler.parent / 'cinch' if args.native_interfaces else args.compiler),
              'frontend_sha256': digest(args.compiler), 'cpu': args.cpu, 'opt_level': 2,
              'scope': 'checked changed-input queries with a reused stable compiled pattern; setup/digests/retained-owner cleanup outside clocks',
              'executables': {}, 'results': []}
    executables = {}
    variants = [('scalar', scalar_source(baseline))] + ([('initial', args.before_source.read_text())] if args.before_source else []) + [('packed', module)]
    for version, selected in variants:
        work = args.artifacts / version
        work.mkdir(parents=True, exist_ok=True)
        entry = work / 'Main.cinch'
        entry.write_text(driver)
        dependencies = source_closure(driver)
        for name, path in dependencies.items():
            (work / (name + '.cinch')).write_text(selected if name == 'Data.Bytes.View' else path.read_text())
        exe = (work / 'consumer').resolve()
        executables[version] = exe
        if not args.measure_only:
            if args.native_interfaces:
                bundle = work / 'Bundle.cinch'
                bundle.write_text('\n'.join(selected if name == 'Data.Bytes.View' else path.read_text()
                                             for name, path in reversed(list(dependencies.items()))) + '\n' + driver)
                configuration = json.loads((args.compiler.parent / 'objects.json').read_text())
                try:
                    run([sys.executable, str(args.compiler.resolve()), '--rc-debug', '--compile-only', '--interfaces',
                         str(args.native_interfaces.resolve()), str(bundle.resolve()), str(exe.with_suffix('.ll')), str(exe), str(exe.with_suffix('.link.txt'))])
                except RuntimeError as error:
                    raise RuntimeError(str(error) + '\n' + exe.with_suffix('.ll.diagnostics').read_text()) from error
                runtime = Path(configuration['metadata']).with_name('libcinchruntime.a')
                run([os.environ.get('CINCH_CLANG', 'clang'), '-O2', str(exe.with_suffix('.ll')), configuration['object'],
                     str(runtime), '-o', str(exe), '-ldl', '-lpthread', '-lm'])
            else:
                run([str(args.compiler.resolve()), 'build', str(entry.resolve()), '-o', str(exe), '--opt-level', '2', '--rc-debug'])
        report['executables'][version] = {'sha256': digest(exe), 'llvm_sha256': digest(exe.with_suffix('.ll')),
                                          'source_sha256': hashlib.sha256(selected.encode()).hexdigest()}
    args.output.parent.mkdir(parents=True, exist_ok=True)
    if args.build_only:
        args.output.write_text(json.dumps(report, indent=2) + '\n')
        return
    for kind in ['index', 'last', 'count', 'pipeline']:
        for name in ['sparse', 'dense', 'prefix', 'periodic']:
            for size, iterations in [(4097, 30), (65537, 4)]:
                if kind == 'index' and name in ['dense', 'periodic']:
                    iterations = 1000
                wanted = expected(kind, name, size, iterations)
                row = {'kind': kind, 'pattern': name, 'bytes': size, 'iterations': iterations,
                       'checksum': wanted[0], 'input_sha256': wanted[1], 'retained_sha256': wanted[2],
                       'stable_pattern_sha256': wanted[3], 'samples': {v: [] for v in executables},
                       'excluded_warmup': {v: [] for v in executables}}
                samples = 1 if args.preflight_only else args.samples
                for sample in range(0 if args.preflight_only else -1, samples):
                    for version in (list(executables) if sample % 2 == 0 else list(reversed(executables))):
                        for debug in ([False, True] if args.preflight_only else [False]):
                            result = run(['taskset', '-c', str(args.cpu), str(executables[version]), kind, name, str(size), str(iterations)],
                                         env={**os.environ, 'CINCH_RC_DEBUG': str(int(debug)), 'CINCH_HEAP_LIMIT': '256M' if debug else '32M',
                                              'CINCH_STACK_SIZE': '8M', 'CINCH_LEAK_CHECK': '1'})
                            assert 'leaked=0' in result.stderr, result.stderr
                            elapsed, checksum, before, after, stable = result.stdout.split()
                            assert (int(checksum), before, after, stable) == wanted, (row, result.stdout)
                            row['samples' if sample >= 0 else 'excluded_warmup'][version].append(int(elapsed))
                row['median_ns'] = {v: statistics.median(values) for v, values in row['samples'].items()}
                row['scalar_over_packed'] = row['median_ns']['scalar'] / row['median_ns']['packed']
                if 'initial' in row['median_ns']:
                    row['initial_over_packed'] = row['median_ns']['initial'] / row['median_ns']['packed']
                report['results'].append(row)
                print(f'{kind}/{name}/{size}: independent observations pass', flush=True)
    report['preflight_only'] = args.preflight_only
    args.output.write_text(json.dumps(report, indent=2) + '\n')


if __name__ == '__main__':
    main()
