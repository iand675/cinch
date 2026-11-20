#!/usr/bin/env python3
"""Measure configurable byte, stream and preserved text Base32 public pipelines."""
from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import statistics
import subprocess
import tempfile

import bench_stdlib as shared

ROOT = Path(__file__).resolve().parents[1]
WORKLOADS = ("encode-bytes", "decode-bytes", "custom-encode", "custom-decode", "legacy-encode", "legacy-decode", "stream-encode64", "stream-decode64", "append-encode", "append-decode", "into-encode", "into-decode")
SOURCE = r'''-- | Timed public Base32 pipelines; startup and input preparation are outside the measured loop.
module Main where
  import Text.Base32 as E
  import Effect.Base32
  import Effect.Stream
  import Effect.Clock
  import Effect.Process
  import Data.Bytes
  import Data.Nat
  import Data.Option
  import Data.Result
  import Data.String
  -- | Parse the host harness's validated decimal arguments.
  decimal :: String -> Nat
  decimal text = decimal-from (unpack text) 0
  -- | Accumulate nonnegative decimal digits; malformed arguments select zero.
  decimal-from :: List Char -> Nat -> Nat
  decimal-from chars n = case chars of
    Nil -> n
    Cons c rest -> case digit-value c of
      Some digit -> decimal-from rest (n * 10 + digit)
      None -> 0
  -- | Initialize deterministic full-byte input outside the measured interval.
  fill :: Bytes -> Nat -> Unit
  fill input at = if at < bytes-len input then let written = bytes-set input at (nat-rem (at * 37 + 11) 256) in fill input (at + 1) else ()
  -- | Consume the result length and boundary bytes so every operation remains observable.
  fingerprint :: Bytes -> Nat
  fingerprint input = if 0 < bytes-len input then bytes-len input + bytes-get input 0 + bytes-get input (bytes-len input - 1) else 0
  -- | Observe a selected existing byte without reading outside a result interval.
  sample :: Bytes -> Nat -> Nat
  sample input at = if at < bytes-len input then bytes-get input at else 0
  -- | Consume an appended result's input-dependent first output byte after its fixed prefix.
  appended-fingerprint :: Bytes -> Nat
  appended-fingerprint input = fingerprint input + sample input 4
  -- | Observe only the written interval of an offset destination operation.
  interval-fingerprint :: Bytes -> Nat -> Nat -> Nat
  interval-fingerprint input start n = if n > 0 && start + n <= bytes-len input then n + bytes-get input start + bytes-get input (start + n - 1) else 0
  -- | Snapshot an exact interval used by the immutable streaming API.
  part :: Bytes -> Nat -> Nat -> Bytes
  part input at n = if at + n <= bytes-len input then let out = bytes-new n in let copied = bytes-copy-range out input 0 at n in out else bytes-new 0
  -- | Select a stable reversed byte alphabet or the standard alphabet before timing begins.
  encoding :: Nat -> E.Base32Encoding
  encoding mode = if mode == 2 || mode == 3 then
    case E.base32-encoding (string-to-bytes "765432ZYXWVUTSRQPONMLKJIHGFEDCBA") of Ok selected -> selected; Err error -> E.base32-standard-encoding
    else E.base32-standard-encoding
  -- | Recognize pipelines whose prepared input is already encoded.
  decoding :: Nat -> Bool
  decoding mode = mode == 1 || mode == 3 || mode == 5 || mode == 7 || mode == 9 || mode == 11
  -- | Mutate one valid input byte on every iteration to prevent a constant-result measurement.
  change :: Nat -> Bytes -> Bytes -> Nat -> Unit
  change mode alphabet input i = if 0 < bytes-len input then
    if decoding mode then let at = nat-rem i 32 in if at < bytes-len alphabet then bytes-set input 0 (bytes-get alphabet at) else ()
    else bytes-set input 0 (nat-rem i 256)
    else ()
  -- | Feed bounded immutable chunks and explicitly finish the final partial encoding quantum.
  write-stream :: forall r. using (io :: Io r) => Bytes -> Nat -> Base32Writer r ->{IO r} Nat
  write-stream input at writer = if at < bytes-len input then
    let left = bytes-len input - at in let take = if left < 64 then left else 64 in
    case base32-write writer (part input at take) of
      (Ok accepted, next) -> write-stream input (at + take) next
      other -> 1000000000
    else case base32-finish writer of
      (Ok done, sink) -> case writer-bytes sink of Some bytes -> fingerprint bytes; None -> 1000000000
      other -> 1000000000
  -- | Execute one public byte, stream or preserved text operation, including owned output buffers.
  operation :: forall r. using (io :: Io r) => Nat -> E.Base32Encoding -> Bytes ->{IO r} Nat
  operation mode selected input =
    if mode == 0 || mode == 2 then fingerprint (E.base32-encode-bytes selected input)
    else if mode == 1 || mode == 3 then case E.base32-decode-bytes selected input of Ok bytes -> fingerprint bytes; Err error -> 1000000000
    else if mode == 4 then fingerprint (string-to-bytes (E.base32-encode E.Base32Standard input))
    else if mode == 5 then case string-from-bytes input of
      Some text -> case E.base32-decode E.Base32Standard text of Ok bytes -> fingerprint bytes; Err error -> 1000000000
      None -> 1000000000
    else if mode == 6 then write-stream input 0 (base32-writer selected writer-buffer)
    else if mode == 7 then case stream-read-all 64 (base32-reader selected (reader-from-bytes input)) of (Ok bytes, next) -> fingerprint bytes; other -> 1000000000
    else if mode == 8 then appended-fingerprint (E.base32-encode-append selected (bytes-from-u8s [1,2,3,4]) input)
    else if mode == 9 then case E.base32-decode-append selected (bytes-from-u8s [1,2,3,4]) input of (bytes, None) -> appended-fingerprint bytes; other -> 1000000000
    else if mode == 10 then
      let out = bytes-new (E.base32-encoded-length selected (bytes-len input) + 2) in
      case E.base32-encode-into selected out 2 input of Ok n -> interval-fingerprint out 2 n; Err error -> 1000000000
    else
      let out = bytes-new (E.base32-decoded-length selected (bytes-len input) + 2) in
      case E.base32-decode-into selected out 2 input of (n, None) -> interval-fingerprint out 2 n; other -> 1000000000
  -- | Repeat observable operations without timing argument decoding or input construction.
  repeat :: forall r. using (io :: Io r) => Nat -> E.Base32Encoding -> Bytes -> Bytes -> Nat -> Nat -> Nat ->{IO r} Nat
  repeat mode selected alphabet input iterations i total = if i < iterations then
    let changed = change mode alphabet input i in
    repeat mode selected alphabet input iterations (i + 1) (nat-rem (total + operation mode selected input) 1000000007)
    else total
  -- | Time only the mutable operation loop and print elapsed nanoseconds with its checksum.
  measure :: forall r. using (io :: Io r) => Nat -> Nat -> Nat ->{IO r} Int
  measure mode size iterations =
    let raw = bytes-new size in let filled = fill raw 0 in let selected = encoding mode in
    let input = if decoding mode then E.base32-encode-bytes selected raw else raw in
    let alphabet = E.base32-encoding-alphabet selected in
    let start = clock-monotonic-ns () in let checksum = repeat mode selected alphabet input iterations 0 0 in
    let end = clock-monotonic-ns () in
    let printed = ioWrite (string-to-bytes (string-concat (nat-to-string (saturating-sub end start)) (string-concat " " (string-concat (nat-to-string checksum) "\n")))) in 0
  -- | Resolve validated host arguments outside the measured interval.
  main :: Unit ->{IO r} Int
  main unit = with (io = newIO ()) in case process-args () of
    Ok (Cons mode (Cons size (Cons iterations Nil))) -> measure (decimal mode) (decimal size) (decimal iterations)
    other -> 2
'''


def command(args: list[str], **kwargs) -> subprocess.CompletedProcess:
    """Run one bounded process and preserve a useful diagnostic on failure."""
    result = subprocess.run(args, capture_output=True, text=True, **kwargs)
    if result.returncode:
        raise RuntimeError(f"{args}: {result.returncode}\n{result.stdout}\n{result.stderr}")
    return result


def frozen_closure(entry: str, source_root: Path, overrides: dict[str, Path]) -> dict[str, tuple[Path, bytes]]:
    """Read each reachable source once so hashing and compilation share the same immutable bytes."""
    embedded = {module["name"] for module in json.loads((source_root / "cinch-base.cinchpkg").read_text())["modules"]}
    found: dict[str, tuple[Path, bytes]] = {}

    def visit(text: str) -> None:
        """Resolve only source-distributed imports without changing any source checkout."""
        for name in re.findall(r"^\s*import\s+([A-Z][\w.]*)", text, re.M):
            if name in embedded or name in found:
                continue
            path = overrides.get(name, source_root / (name.replace(".", "/") + ".cinch"))
            content = path.read_bytes()
            found[name] = (path.resolve(), content)
            visit(content.decode())

    visit(entry)
    if unused := set(overrides) - set(found):
        raise ValueError(f"source overrides must name reachable source-distributed modules: {sorted(unused)}")
    return found


def main() -> None:
    """Build frozen artifacts once or measure only their recorded immutable identities."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--compiler", type=Path)
    parser.add_argument("--compiler-snapshot", type=Path, help="Retain a frozen compiler provenance JSON and verify its binary identity")
    parser.add_argument("--source-root", type=Path, default=ROOT / "seed/prelude", help="Read-only prelude source root containing cinch-base.cinchpkg")
    parser.add_argument("--source-override", action="append", default=[], metavar="MODULE=PATH", help="Read a retained baseline module without changing a library checkout")
    parser.add_argument("--reference-go", type=Path, default=Path("/tmp/cinch-go-current/go/bin/go"))
    parser.add_argument("--artifacts", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--build-only", action="store_true")
    parser.add_argument("--measure-only", action="store_true")
    parser.add_argument("--cpu", type=int, default=0)
    parser.add_argument("--sizes", nargs="+", type=int, default=[32, 1024, 16384])
    parser.add_argument("--iterations", type=int, default=64)
    parser.add_argument("--samples", type=int, default=5)
    parser.add_argument("--workloads", nargs="+", choices=WORKLOADS, default=WORKLOADS)
    args = parser.parse_args()
    if args.build_only and args.measure_only:
        parser.error("select at most one build/measure mode")
    if args.measure_only and (args.source_override or args.compiler_snapshot):
        parser.error("source/compiler provenance options apply when compiling new frozen artifacts")
    if min(args.sizes) < 0 or min(args.iterations, args.samples) <= 0:
        parser.error("sizes must be nonnegative; samples and iterations must be positive")
    os.sched_setaffinity(0, {args.cpu})
    artifacts = args.artifacts.resolve()
    metadata = artifacts / "metadata.json"
    environment = {**os.environ, "GOCACHE": "/tmp/cinch-go-base32-bench-cache", "GOTELEMETRY": "off", "GOMAXPROCS": "1", "CINCH_RC_DEBUG": "0", "CINCH_STACK_SIZE": "8M", "CINCH_HEAP_LIMIT": "256M"}
    if not args.measure_only:
        if not args.compiler:
            parser.error("--compiler is required to build")
        compiler = args.compiler.resolve()
        source_root = args.source_root.resolve()
        overrides = {}
        for value in args.source_override:
            name, separator, path = value.partition("=")
            if not separator or name in overrides:
                parser.error("source overrides must be unique MODULE=PATH pairs")
            overrides[name] = Path(path).resolve()
        closure = frozen_closure(SOURCE, source_root, overrides)
        artifacts.mkdir(parents=True, exist_ok=True)
        snapshots = artifacts / "sources"
        snapshots.mkdir(exist_ok=True)
        compiler_hash = shared.digest(compiler)
        provenance = None
        if args.compiler_snapshot:
            content = args.compiler_snapshot.read_bytes()
            provenance = json.loads(content)
            if provenance.get("compiler_sha256") != compiler_hash:
                raise ValueError("compiler snapshot does not identify the selected compiler binary")
            (artifacts / "compiler-snapshot.json").write_bytes(content)
        with tempfile.TemporaryDirectory(prefix="cinch-base32-bench-") as directory:
            work = Path(directory)
            frozen_compiler = work / "compiler"
            shutil.copyfile(compiler, frozen_compiler)
            frozen_compiler.chmod(0o755)
            if shared.digest(frozen_compiler) != compiler_hash:
                raise ValueError("compiler binary changed while freezing it")
            for name, (_, content) in closure.items():
                (work / f"{name}.cinch").write_bytes(content)
                (snapshots / f"{name}.cinch").write_bytes(content)
            (work / "Main.cinch").write_text(SOURCE)
            command([str(frozen_compiler), "build", str(work / "Main.cinch"), "-o", str(artifacts / "bench"), "--opt-level", "2", "--rc-debug"], cwd=work, timeout=600)
        (artifacts / "Main.cinch").write_text(SOURCE)
        reference = ROOT / "tooling/bench_base32_encoding_reference.go"
        command([str(args.reference_go), "build", "-o", str(artifacts / "reference"), str(reference)], env=environment, timeout=120)
        command([os.environ.get("CINCH_CLANG", "clang"), "-O2", "-S", "-x", "ir", str(artifacts / "bench.ll"), "-o", str(artifacts / "bench.s")], timeout=120)
        report = {"description": "Public Base32 pipelines with matched length/boundary checksums; immutable buffers and mutable reference allocations differ", "compiler_sha256": compiler_hash, "compiler_snapshot": provenance, "compiler_source_sha256": shared.source_digest(list((ROOT / "seed/src").rglob("*.rs")) + [ROOT / "seed/Cargo.toml", ROOT / "seed/Cargo.lock", ROOT / "seed/build.rs"]), "runtime_source_sha256": shared.source_digest(list((ROOT / "seed/runtime/src").rglob("*.rs"))), "driver_sha256": hashlib.sha256(SOURCE.encode()).hexdigest(), "sources": {name: hashlib.sha256(content).hexdigest() for name, (_, content) in closure.items()}, "source_root": str(source_root), "source_paths": {name: str(path) for name, (path, _) in closure.items()}, "source_overrides": {name: str(path) for name, path in overrides.items()}, "source_snapshots": str(snapshots), "runner_sha256": shared.digest(Path(__file__)), "opt_level": 2, "rc_debug_capability": True, "executable_sha256": shared.digest(artifacts / "bench"), "llvm_sha256": shared.digest(artifacts / "bench.ll"), "assembly_sha256": shared.digest(artifacts / "bench.s"), "reference_source_sha256": shared.digest(reference), "reference_executable_sha256": shared.digest(artifacts / "reference"), "reference_sdk": command([str(args.reference_go), "version"]).stdout.strip(), "artifacts": str(artifacts), "results": []}
        metadata.write_text(json.dumps(report, indent=2) + "\n")
    else:
        report = json.loads(metadata.read_text())
    if args.build_only:
        print("Built frozen benchmark artifacts", flush=True)
        return
    for name, key in [("bench", "executable_sha256"), ("reference", "reference_executable_sha256"), ("bench.ll", "llvm_sha256")]:
        if shared.digest(artifacts / name) != report[key]:
            raise ValueError(f"artifact changed: {name}")
    for name, digest in report["sources"].items():
        if shared.digest(Path(report["source_snapshots"]) / f"{name}.cinch") != digest:
            raise ValueError(f"frozen source changed: {name}")
    report.update(cpu_affinity=[args.cpu], iterations=args.iterations, samples=args.samples, timing="runtime monotonic clock around operation loop; startup, configuration and preparation excluded", results=[])
    for workload in args.workloads:
        for size in args.sizes:
            samples = {"cinch": [], "reference": []}
            checksums = set()
            for index in range(args.samples + 1):
                for label in (["cinch", "reference"] if index % 2 == 0 else ["reference", "cinch"]):
                    executable = artifacts / ("bench" if label == "cinch" else "reference")
                    result = command([str(executable), str(WORKLOADS.index(workload)), str(size), str(args.iterations)], env=environment, timeout=180)
                    elapsed, checksum = map(int, result.stdout.split())
                    checksums.add(checksum)
                    if index:
                        samples[label].append(elapsed)
            if len(checksums) != 1:
                raise ValueError(f"checksum mismatch: {workload}/{size}: {checksums}")
            row = {"workload": workload, "raw_input_bytes": size, "checksum": checksums.pop(), "samples_ns": samples, "median_ns_per_operation": {label: statistics.median(values) / args.iterations for label, values in samples.items()}}
            report["results"].append(row)
            print(workload, size, row["median_ns_per_operation"], flush=True)
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(report, indent=2) + "\n")


if __name__ == "__main__":
    main()
