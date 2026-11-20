#!/usr/bin/env python3
"""Build reproducible public CSV pipeline artifacts and retain independent timing/checksum samples."""
from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import platform
import re
import shutil
import statistics
import subprocess

from bench_stdlib import source_closure, source_digest

ROOT = Path(__file__).resolve().parents[1]
WORKLOADS = ["read-plain", "read-quoted", "write-plain", "write-quoted", "stream-read", "stream-write", "copy", "sha256-runtime"]
SOURCE = r'''-- | Public CSV pipeline driver; output is elapsed nanoseconds and an observable checksum.
module Main where
  import Text.Csv.Record
  import Effect.Csv
  import Effect.Stream
  import Effect.Clock
  import Effect.Process
  import Data.Char
  import Data.Machine
  -- | Parse validated decimal harness arguments.
  decimal :: String -> Nat
  decimal text = decimal-from (unpack text) 0
  -- | Accumulate decimal digits without accepting a malformed argument as a partial number.
  decimal-from :: List Char -> Nat -> Nat
  decimal-from chars count = case chars of
    Nil -> count
    Cons first rest -> case digit-value first of None -> 0; Some digit -> decimal-from rest (count * 10 + digit)
  -- | Resolve workload names once before timing.
  mode-code :: String -> U32
  mode-code name = __RESOLVER__
  -- | Initialize a mutable ordinary ASCII payload before timing.
  fill :: Bytes -> Nat -> Unit
  fill out at = if at < bytes-len out then let written = bytes-set out at 97 in fill out (at + 1) else ()
  -- | Observe one output byte only after checking its complete interval.
  sample :: Bytes -> Nat -> Nat
  sample out at = if at < bytes-len out then bytes-get out at else 0
  -- | Retain length, prefix, middle and final bytes as observable output.
  fingerprint :: Bytes -> Nat
  fingerprint out = let n = bytes-len out in n + sample out 0 + sample out 1 + sample out 2 + sample out 3 + sample out (saturating-sub n 1) + sample out (nat-div n 2)
  -- | Observe every returned field without imposing an unrelated byte-wise hash pipeline.
  fields-code :: List Bytes -> Nat -> Nat
  fields-code fields total = case fields of Nil -> total; Cons first rest -> fields-code rest (total + 1 + fingerprint first)
  -- | Copy a prepared literal tail only after checking its complete source and destination range.
  copy-tail :: Bytes -> Nat -> Bytes -> Unit
  copy-tail out start tail = if start + bytes-len tail <= bytes-len out then bytes-copy-range out tail start 0 (bytes-len tail) else ()
  -- | Construct prepared quoted/unquoted input or a mutable writer field outside the timer.
  prepare :: U32 -> Nat -> Bytes
  prepare mode size =
    let body = bytes-new size in let initialized = fill body 0 in
    let quoted = eq-u32 mode 1 || eq-u32 mode 3 || eq-u32 mode 4 || eq-u32 mode 5 in
    let leading = if quoted && size > 0 then bytes-set body 0 32 else () in
    if eq-u32 mode 0 then
      let out = bytes-new (size + 6) in let a = bytes-copy-range out body 0 0 size in
      let tail = string-to-bytes ",tail\n" in let b = copy-tail out size tail in out
    else if eq-u32 mode 1 || eq-u32 mode 4 then
      let out = bytes-new (size + 8) in let a = bytes-set out 0 34 in
      let b = bytes-copy-range out body 1 0 size in let tail = string-to-bytes "\",tail\n" in
      let c = copy-tail out (size + 1) tail in out
    else body
  -- | Consume exactly one public parse/render/stream/control pipeline.
  operation :: forall r. using (io :: Io r) => U32 -> Bytes -> Bytes ->{IO r} Nat
  operation mode input tail =
    if eq-u32 mode 0 || eq-u32 mode 1 then case csv-byte-reader-read (csv-byte-reader csv-default-options input) of
      (CsvReadByteRecord row None, next) -> fields-code (csv-byte-record-fields row) 0
      other -> 1000000000
    else if eq-u32 mode 2 || eq-u32 mode 3 then fingerprint (csv-render-byte-record csv-default-options False [input, tail])
    else if eq-u32 mode 4 then case csv-byte-stream-read (csv-byte-stream-reader csv-default-options (reader-from-bytes input)) of
      (Ok (CsvReadByteRecord row None), next) -> fields-code (csv-byte-record-fields row) 0
      other -> 1000000000
    else if eq-u32 mode 5 then case csv-byte-stream-write (csv-stream-writer csv-default-options False writer-buffer) [input, tail] of
      (Ok unit, next) -> case writer-bytes (csv-stream-writer-sink next) of Some out -> fingerprint out; None -> 1000000000
      other -> 1000000000
    else if eq-u32 mode 6 then
      let out = bytes-new (bytes-len input) in let copied = bytes-copy-range out input 0 0 (bytes-len input) in fingerprint out
    else fingerprint (sha256-bytes input)
  -- | Mutate a bounded payload byte between samples so its observed output remains live.
  repeat :: forall r. using (io :: Io r) => U32 -> Bytes -> Bytes -> Nat -> Nat -> Nat -> Nat ->{IO r} Nat
  repeat mode input tail change count at checksum =
    if at < count then
      let changed = if change < bytes-len input then bytes-set input change (65 + nat-rem at 26) else () in
      repeat mode input tail change count (at + 1) (nat-rem (checksum + operation mode input tail) 1000000007)
    else checksum
  -- | Choose a changing field byte while preserving quoted-mode leading whitespace when possible.
  change-index :: U32 -> Nat -> Bytes -> Nat
  change-index mode size input =
    if size == 0 then bytes-len input else
      let offset = if eq-u32 mode 1 || eq-u32 mode 4 then (1 :: Nat) else 0 in
      let quoted = eq-u32 mode 1 || eq-u32 mode 3 || eq-u32 mode 4 || eq-u32 mode 5 in
      let extra = if quoted && size > 1 then (1 :: Nat) else 0 in offset + extra
  -- | Time repeated operations after all input preparation and workload resolution.
  measure :: forall r. using (io :: Io r) => String -> Nat -> Nat ->{IO r} Int
  measure name size count =
    let mode = mode-code name in let input = prepare mode size in let tail = string-to-bytes "tail" in
    let change = change-index mode size input in
    let start = clock-monotonic-ns () in let checksum = repeat mode input tail change count 0 0 in
    let elapsed = saturating-sub (clock-monotonic-ns ()) start in
    let printed = ioWrite (string-to-bytes (string-concat (nat-to-string elapsed) (string-concat " " (string-concat (nat-to-string checksum) "\n")))) in 0
  -- | Accept exactly the validated mode, size and iteration arguments from the host harness.
  main :: Unit ->{IO r} Int
  main unit = with (io = newIO ()) in case process-args () of
    Err error -> 2
    Ok args -> case args of Cons mode (Cons size (Cons count Nil)) -> measure mode (decimal size) (decimal count); other -> 2
'''
SOURCE = SOURCE.replace("__RESOLVER__", "\n    " + "\n    ".join(f'{"if" if index == 0 else "else if"} name == "{name}" then {index}' for index, name in enumerate(WORKLOADS)) + "\n    else 4294967295")


def digest(path: Path) -> str:
    """Identify immutable source and executable bytes used by this report."""
    return hashlib.sha256(path.read_bytes()).hexdigest()


def run(command: list[str], cwd: Path, env: dict[str, str] | None = None) -> subprocess.CompletedProcess[str]:
    """Run one checked compilation or workload and retain useful diagnostics on failure."""
    result = subprocess.run(command, cwd=cwd, env=env, capture_output=True, text=True, timeout=900)
    if result.returncode:
        raise RuntimeError(f"command failed ({result.returncode}): {command}\n{result.stdout}\n{result.stderr}")
    return result


def measure(report: dict, args: argparse.Namespace, artifacts: Path, reference_env: dict[str, str]) -> None:
    """Measure retained executables after checking independent checksums on the requested CPU."""
    report["results"] = []
    report["cpu_affinity"] = [args.cpu]
    report["iterations"] = args.iterations
    environment = {**os.environ, "CINCH_RC_DEBUG": "0", "CINCH_ALLOC_STATS": "0", "CINCH_HEAP_LIMIT": "32M", "CINCH_STACK_SIZE": "8M", "CINCH_LEAK_CHECK": "1"}
    for mode in WORKLOADS:
        for size in args.sizes:
            samples = []; reference_samples = []; checksums = set()
            for index in range(args.samples + 1):
                elapsed, checksum = map(int, run([str(artifacts / "bench"), mode, str(size), str(args.iterations)], artifacts, environment).stdout.split())
                reference_elapsed, reference_checksum = map(int, run([str(artifacts / "reference"), mode, str(size), str(args.iterations)], artifacts, reference_env).stdout.split())
                if checksum != reference_checksum: raise RuntimeError(f"independent checksum mismatch: {mode}/{size}: {checksum} != {reference_checksum}")
                checksums.add(checksum)
                if index: samples.append(elapsed); reference_samples.append(reference_elapsed)
            if len(checksums) != 1: raise RuntimeError("nondeterministic observable checksum")
            report["results"].append({"workload": mode, "field_bytes": size, "raw_input_bytes": size, "checksum": checksums.pop(), "samples_ns": samples, "reference_samples_ns": reference_samples, "median_ns_per_operation": statistics.median(samples) / args.iterations, "reference_median_ns_per_operation": statistics.median(reference_samples) / args.iterations})
            print(mode, size, report["results"][-1]["median_ns_per_operation"], flush=True)


def compiler_snapshot(report: dict, compiler: Path) -> None:
    """Attach a verified frozen compiler build receipt while preserving the checkout source digest."""
    path = compiler.parent / "snapshot.json"
    if path.exists():
        snapshot = json.loads(path.read_text())
        if snapshot.get("compiler_sha256") != report["compiler_sha256"]:
            raise RuntimeError("frozen compiler snapshot does not identify this executable")
        report["compiler_build_snapshot"] = snapshot
        report["compiler_source_identity_note"] = "compiler_source_sha256 identifies the benchmark checkout; compiler_build_snapshot identifies sources compiled into the frozen executable."


def allocations(report: dict, args: argparse.Namespace, artifacts: Path) -> None:
    """Observe ordinary allocation work at an identical executable path for both retained candidates."""
    probe = ROOT / "build/csv-record-allocation-probe/bench"
    probe.parent.mkdir(parents=True, exist_ok=True)
    shutil.copyfile(artifacts / "bench", probe)
    probe.chmod(0o700)
    environment = {**os.environ, "CINCH_RC_DEBUG": "0", "CINCH_ALLOC_STATS": "1", "CINCH_HEAP_LIMIT": "32M", "CINCH_STACK_SIZE": "8M", "CINCH_LEAK_CHECK": "1"}
    report["allocation_probe"] = {"executable_path": str(probe), "cpu_affinity": [args.cpu], "iterations": args.iterations, "runner_sha256": digest(Path(__file__)), "results": []}
    for mode in WORKLOADS:
        for size in args.sizes:
            result = run([str(probe), mode, str(size), str(args.iterations)], artifacts, environment)
            line = next(line for line in result.stderr.splitlines() if line.startswith("cinch-alloc:"))
            stats = {name: int(value) for name, value in re.findall(r"([a-z-]+)=(\d+)", line)}
            if stats["live"] or "leaked=0" not in result.stderr:
                raise RuntimeError(f"allocation probe retained live bytes: {result.stderr}")
            elapsed, checksum = map(int, result.stdout.split())
            report["allocation_probe"]["results"].append({"workload": mode, "field_bytes": size, "checksum": checksum, "stats": stats, "leaked": 0})


def main() -> None:
    """Compile one retained snapshot, then measure only when a coordinated CPU is explicitly supplied."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--compiler", type=Path, required=True)
    parser.add_argument("--reference-go", type=Path, required=True)
    parser.add_argument("--artifacts", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--source-override", action="append", default=[])
    parser.add_argument("--build-only", action="store_true")
    parser.add_argument("--allocation-only", action="store_true", help="Observe ordinary allocation statistics on retained executables at one identical probe path")
    parser.add_argument("--reuse", action="store_true", help="Measure an existing report/artifact pair without recompiling")
    parser.add_argument("--cpu", type=int, required=True)
    parser.add_argument("--sizes", nargs="+", type=int, default=[32, 1024, 16384, 65536])
    parser.add_argument("--iterations", type=int, default=128)
    parser.add_argument("--samples", type=int, default=5)
    args = parser.parse_args()
    if args.cpu not in os.sched_getaffinity(0): parser.error("--cpu must be allowed")
    if args.allocation_only and not args.reuse: parser.error("--allocation-only requires --reuse")
    if not args.sizes or min(args.sizes) < 0 or min(args.iterations, args.samples) <= 0:
        parser.error("sizes must be nonnegative; iterations and samples must be positive")
    os.sched_setaffinity(0, {args.cpu})
    if args.reuse:
        report = json.loads(args.output.read_text())
        artifacts = args.artifacts.resolve()
        if report["driver_sha256"] != hashlib.sha256(SOURCE.encode()).hexdigest():
            raise RuntimeError("retained driver source differs from this runner")
        for path, key in [("bench", "executable_sha256"), ("bench.ll", "llvm_sha256")]:
            if digest(artifacts / path) != report[key]: raise RuntimeError(f"retained {path} differs from report")
        if digest(artifacts / "reference") != report["reference"]["executable_sha256"]:
            raise RuntimeError("retained reference differs from report")
        reference_env = {**os.environ, "GOTELEMETRY": "off", "GOMAXPROCS": "1"}
        compiler_snapshot(report, args.compiler.resolve())
        if args.allocation_only:
            allocations(report, args, artifacts)
        else:
            report["measurement_runner_sha256"] = digest(Path(__file__))
            measure(report, args, artifacts, reference_env)
        args.output.write_text(json.dumps(report, indent=2) + "\n")
        return
    overrides = {name: Path(value).resolve() for name, value in (item.split("=", 1) for item in args.source_override)}
    closure = source_closure(SOURCE, overrides)
    artifacts = args.artifacts.resolve(); artifacts.mkdir(parents=True, exist_ok=True)
    frozen_sources = {name: path.read_bytes() for name, path in closure.items()}
    for name, contents in frozen_sources.items(): (artifacts / f"{name}.cinch").write_bytes(contents)
    (artifacts / "Main.cinch").write_text(SOURCE)
    compiler = args.compiler.resolve(); compiler_hash = digest(compiler)
    run([str(compiler), "build", str(artifacts / "Main.cinch"), "-o", str(artifacts / "bench"), "--opt-level", "2"], artifacts)
    if digest(compiler) != compiler_hash: raise RuntimeError("compiler changed during compilation")
    reference_env = {**os.environ, "GOCACHE": "/tmp/cinch-go-csv-benchmark-cache", "GOTELEMETRY": "off", "GOMAXPROCS": "1"}
    reference = ROOT / "tooling/bench_csv_records_reference.go"
    run([str(args.reference_go.resolve()), "build", "-o", str(artifacts / "reference"), str(reference)], ROOT, reference_env)
    run([os.environ.get("CINCH_CLANG", "clang"), "-O2", "-S", "-x", "ir", str(artifacts / "bench.ll"), "-o", str(artifacts / "bench.s")], artifacts)
    report = {
        "format": 1, "description": "Public CSV parse/render/owned-stream pipelines with prepared mutable input, normal reclamation and observable byte samples",
        "opt_level": 2, "compiler_sha256": compiler_hash,
        "compiler_source_sha256": source_digest(list((ROOT / "seed/src").rglob("*.rs")) + [ROOT / "seed/Cargo.toml", ROOT / "seed/Cargo.lock", ROOT / "seed/build.rs"]),
        "runtime_source_sha256": source_digest(list((ROOT / "seed/runtime/src").rglob("*.*")) + [ROOT / "seed/runtime/Cargo.toml"]),
        "driver_sha256": hashlib.sha256(SOURCE.encode()).hexdigest(), "runner_sha256": digest(Path(__file__)),
        "sources": {name: hashlib.sha256(contents).hexdigest() for name, contents in frozen_sources.items()},
        "source_overrides": {name: str(path) for name, path in overrides.items()},
        "platform": platform.platform(), "cpu": next(line.split(":", 1)[1].strip() for line in Path("/proc/cpuinfo").read_text().splitlines() if line.startswith("model name")),
        "cpu_affinity": [args.cpu], "iterations": args.iterations, "warmup_processes": 1,
        "executable_sha256": digest(artifacts / "bench"), "llvm_sha256": digest(artifacts / "bench.ll"), "assembly_sha256": digest(artifacts / "bench.s"),
        "artifacts": str(artifacts), "results": [],
        "reference": {"sdk": run([str(args.reference_go.resolve()), "version"], ROOT, reference_env).stdout.strip(), "source_sha256": digest(reference), "executable_sha256": digest(artifacts / "reference"), "GOMAXPROCS": "1"},
        "scope": "Pipeline timing includes copied input/field snapshots, returned output and owned stream chunks; it does not assert equal allocation complexity",
        "size_argument": "field byte length; prepared read input adds six unquoted or eight quoted delimiter/tail bytes",
    }
    compiler_snapshot(report, compiler)
    if not args.build_only:
        measure(report, args, artifacts, reference_env)
    args.output.parent.mkdir(parents=True, exist_ok=True); args.output.write_text(json.dumps(report, indent=2) + "\n")


if __name__ == "__main__":
    main()
