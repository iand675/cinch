#!/usr/bin/env python3
"""Prepare frozen slash-path executables, then measure alternating public pipelines.

Preparation is separate from measurement so a coordinated CPU window contains
only retained executable sampling. Independent checksums are required for every
row; compiled-pattern timing has no corresponding reference precompile API.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil
import statistics
import subprocess

import bench_stdlib as shared

ROOT = Path(__file__).resolve().parents[1]
WORKLOADS = ["clean-component", "clean-segments", "base", "ext", "compiled-match", "match", "join", "absolute-control"]
SOURCE = r'''-- | Timed portable slash-path pipelines with observable output.
module Main where
  import Text.Path
  import Effect.Clock
  import Effect.Process
  -- | Parse a validated decimal argument outside the timed interval.
  decimal :: String -> Nat
  decimal text = digits (unpack text) 0
  -- | Accumulate decimal values supplied by the host harness.
  digits :: List Char -> Nat -> Nat
  digits chars n = case chars of Nil -> n; Cons char rest -> case digit-value char of None -> 0; Some value -> digits rest (n * 10 + value)
  -- | Resolve a workload name before timing.
  code :: String -> U8
  code name = if name == "clean-component" then 0 else if name == "clean-segments" then 1 else if name == "base" then 2 else if name == "ext" then 3 else if name == "compiled-match" then 4 else if name == "match" then 5 else if name == "join" then 6 else 7
  -- | Fill dynamic ASCII input before the measured loop.
  fill :: Bytes -> Bytes -> U8 -> Nat -> Unit
  fill out pieces mode at = if at < bytes-len out then
    let value = if eq-u8 mode 1 && 9 <= bytes-len pieces then bytes-get pieces (nat-rem at 9) else 120 in
    let written = bytes-set out at value in fill out pieces mode (at + 1)
    else ()
  -- | Convert exact driver ASCII bytes to independent text.
  text :: Bytes -> String
  text input = case string-from-bytes input of Some result -> result; None -> empty-string
  -- | Observe output bytes and length without timing an additional full output scan.
  fingerprint :: String -> Nat
  fingerprint result = let bytes = string-to-bytes result in if bytes-len bytes > 0 then bytes-len bytes + bytes-get bytes 0 + bytes-get bytes (bytes-len bytes - 1) else 0
  -- | Exercise one public pipeline, with pattern preparation excluded only for the compiled API.
  operation :: U8 -> Bytes -> String -> PathPattern -> Nat
  operation mode input pattern compiled =
    let value = text input in
    if eq-u8 mode 0 then fingerprint (path-clean (string-concat value "/../z"))
    else if eq-u8 mode 1 then fingerprint (path-clean value)
    else if eq-u8 mode 2 then fingerprint (path-base (string-concat "root/" value))
    else if eq-u8 mode 3 then fingerprint (path-ext value)
    else if eq-u8 mode 4 then (if path-pattern-match compiled value then 1 else 0)
    else if eq-u8 mode 5 then (case path-match pattern value of Err error -> 1000000000; Ok matched -> if matched then 1 else 0)
    else if eq-u8 mode 6 then fingerprint (path-join [value,"..","z"])
    else if path-is-absolute value then 1 else 0
  -- | Mutate dynamic input and retain an observable rolling checksum through the timed tail loop.
  repeat :: U8 -> Bytes -> String -> PathPattern -> Nat -> Nat -> Nat -> Nat
  repeat mode input pattern compiled iterations i checksum =
    if i < iterations then
      let changed = if bytes-len input > 0 then bytes-set input (bytes-len input - 1) (u8-to-nat (wrapping-i64-to-u8 (nat-to-i64 (120 + nat-rem i 2)))) else () in
      repeat mode input pattern compiled iterations (i + 1) (nat-rem (checksum + operation mode input pattern compiled) 1000000007)
    else checksum
  -- | Time only repeated operations after dynamic input and optional compiled pattern preparation.
  measure :: forall r. using (io :: Io r) => String -> Nat -> Nat ->{IO r} Int
  measure name size iterations =
    let mode = code name in let input = bytes-new size in
    let initialized = fill input (string-to-bytes "a/./b/../") mode 0 in
    let pattern = text input in
    case path-pattern pattern of
      Err error -> 3
      Ok compiled ->
        let start = clock-monotonic-ns () in let checksum = repeat mode input pattern compiled iterations 0 0 in
        let end = clock-monotonic-ns () in
        let printed = ioWrite (string-to-bytes (string-concat (nat-to-string (saturating-sub end start)) (string-concat " " (string-concat (nat-to-string checksum) "\n")))) in 0
  -- | Read the validated host arguments and select one actual pipeline sample.
  main :: Unit ->{IO r} Int
  main unit = with (io = newIO ()) in case process-args () of
    Err error -> 2
    Ok args -> case args of Cons name (Cons size (Cons iterations Nil)) -> measure name (decimal size) (decimal iterations); other -> 2
'''


def digest(path: Path) -> str:
    """Identify exact source and executable bytes."""
    return hashlib.sha256(path.read_bytes()).hexdigest()


def run(command: list[str], **kwargs) -> subprocess.CompletedProcess:
    """Capture complete failures without accepting partial benchmark output."""
    result = subprocess.run(command, capture_output=True, text=True, timeout=300, **kwargs)
    if result.returncode:
        raise RuntimeError(f"command failed: {command}\n{result.stderr}")
    return result


def prepare(args: argparse.Namespace) -> None:
    """Build matched baseline/final drivers and the independent reference before CPU timing."""
    args.artifacts.mkdir(parents=True, exist_ok=True)
    report = {"compiler_sha256": digest(args.compiler), "driver_sha256": hashlib.sha256(SOURCE.encode()).hexdigest(), "opt_level": 2, "inputs": {}}
    for label, source in [("before", args.before_source), ("after", ROOT / "seed/prelude/Text/Path.cinch")]:
        work = (args.artifacts / label).resolve()
        work.mkdir(parents=True, exist_ok=True)
        closure = shared.source_closure(SOURCE, {"Text.Path": source.resolve()})
        for name, path in closure.items():
            shutil.copyfile(path, work / f"{name}.cinch")
        (work / "Main.cinch").write_text(SOURCE)
        exe = work / "bench"
        run([str(args.compiler.resolve()), "build", str(work / "Main.cinch"), "-o", str(exe), "--opt-level", "2"])
        run([os.environ.get("CINCH_CLANG", "clang"), "-O2", "-S", "-x", "ir", str(exe.with_suffix('.ll')), "-o", str(exe.with_suffix('.s'))])
        report["inputs"][label] = {"executable": str(exe), "executable_sha256": digest(exe), "llvm_sha256": digest(exe.with_suffix('.ll')), "assembly_sha256": digest(exe.with_suffix('.s')), "sources": {name: digest(path) for name, path in closure.items()}}
    env = {**os.environ, "GOCACHE": "/tmp/cinch-go-path-benchmark-cache", "GOTELEMETRY": "off", "GOMAXPROCS": "1"}
    reference = (args.artifacts / "reference").resolve()
    run([str(args.reference_go.resolve()), "build", "-o", str(reference), str(ROOT / "tooling/bench_path_reference.go")], env=env)
    report["reference"] = {"executable": str(reference), "executable_sha256": digest(reference), "source_sha256": digest(ROOT / "tooling/bench_path_reference.go"), "sdk": run([str(args.reference_go.resolve()), "version"], env=env).stdout.strip(), "GOMAXPROCS": "1", "compiled_match_scope": "Reference Match supplies output checksums; its timing includes parsing because no public precompile API exists"}
    (args.artifacts / "snapshot.json").write_text(json.dumps(report, indent=2) + "\n")


def sample(exe: str, name: str, size: int, iterations: int) -> tuple[int, int]:
    """Read elapsed nanoseconds and observable result from a bounded retained executable."""
    result = run([exe, name, str(size), str(iterations)], env={**os.environ, "CINCH_RC_DEBUG": "0", "CINCH_HEAP_LIMIT": "32M", "CINCH_STACK_SIZE": "8M", "GOMAXPROCS": "1"})
    return tuple(map(int, result.stdout.split()))


def measure(args: argparse.Namespace) -> None:
    """Alternate verified artifacts on one coordinated CPU and retain every sample and checksum."""
    if args.cpu not in os.sched_getaffinity(0):
        raise ValueError("requested CPU is unavailable")
    os.sched_setaffinity(0, {args.cpu})
    report = json.loads((args.artifacts / "snapshot.json").read_text())
    for entry in [*report["inputs"].values(), report["reference"]]:
        assert digest(Path(entry["executable"])) == entry["executable_sha256"]
    report.update(cpu_affinity=[args.cpu], results=[], cpu=next(line.split(":", 1)[1].strip() for line in Path('/proc/cpuinfo').read_text().splitlines() if line.startswith('model name')), timing="monotonic clock around public operation loop; input/pattern setup and process startup excluded", iterations_policy="min(requested, max(1, 524288/input_bytes))", space_policy="normal reclamation; no poisoned-free quarantine")
    for name in WORKLOADS:
        for size in args.sizes:
            iterations = min(args.iterations, max(1, 524288 // max(1, size)))
            values = {label: [] for label in ["before", "after", "reference"]}
            checksums = set()
            for entry in [*report["inputs"].values(), report["reference"]]:
                sample(entry["executable"], name, size, iterations)
            for index in range(args.samples):
                for label in (["before", "after"] if index % 2 == 0 else ["after", "before"]):
                    elapsed, checksum = sample(report["inputs"][label]["executable"], name, size, iterations)
                    values[label].append(elapsed)
                    checksums.add(checksum)
                elapsed, checksum = sample(report["reference"]["executable"], name, size, iterations)
                values["reference"].append(elapsed)
                checksums.add(checksum)
            assert len(checksums) == 1, (name, size, checksums)
            medians = {label: statistics.median(samples) / iterations for label, samples in values.items()}
            item = {"workload": name, "input_bytes": size, "iterations": iterations, "samples_ns": values, "median_ns_per_operation": medians, "checksum": checksums.pop(), "before_over_after": medians["before"] / medians["after"]}
            report["results"].append(item)
            print(f"{name}/{size}: {item['before_over_after']:.3f}x", flush=True)
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(report, indent=2) + "\n")


def main() -> None:
    """Prepare immutable artifacts or sample them during a separately coordinated CPU window."""
    parser = argparse.ArgumentParser(description=__doc__)
    actions = parser.add_subparsers(dest="action", required=True)
    build = actions.add_parser("prepare")
    build.add_argument("--compiler", type=Path, required=True)
    build.add_argument("--before-source", type=Path, required=True)
    build.add_argument("--reference-go", type=Path, required=True)
    build.add_argument("--artifacts", type=Path, required=True)
    timing = actions.add_parser("measure")
    timing.add_argument("--artifacts", type=Path, required=True)
    timing.add_argument("--sizes", nargs="+", type=int, default=[32, 1024, 16384, 65536])
    timing.add_argument("--iterations", type=int, default=128)
    timing.add_argument("--samples", type=int, default=9)
    timing.add_argument("--cpu", type=int, default=0)
    timing.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    if args.action == "prepare":
        prepare(args)
    else:
        if min(args.sizes) < 0 or min(args.iterations, args.samples) <= 0:
            parser.error("sizes must be nonnegative; iterations/samples must be positive")
        measure(args)


if __name__ == "__main__":
    main()
