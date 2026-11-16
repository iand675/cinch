#!/usr/bin/env python3
"""Measure complete circular-sequence pipelines with independent output checks."""
from __future__ import annotations

import argparse
import datetime
import hashlib
import json
import os
from pathlib import Path
import shutil
import statistics
import subprocess
import tempfile

import bench_stdlib as shared
from validate_rings import compiler_stack

ROOT = Path(__file__).resolve().parents[1]
WORKLOADS = ("dense-construct-walk", "growing-join", "balanced-join-aliases", "short-split-rejoin", "detached-copy")
SOURCE = (ROOT / "tooling/ring-benchmark.cinch").read_text()


def command(args: list[str], **kwargs) -> subprocess.CompletedProcess:
    """Run one bounded process and preserve a useful diagnostic on failure."""
    result = subprocess.run(args, capture_output=True, text=True, **kwargs)
    if result.returncode:
        raise RuntimeError(f"{args}: {result.returncode}\n{result.stdout}\n{result.stderr}")
    return result


def main() -> None:
    """Build frozen artifacts once or measure only their recorded immutable identities."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--compiler", type=Path)
    parser.add_argument("--frontend", choices=["rust", "native"], default="rust")
    parser.add_argument("--interfaces", type=Path)
    parser.add_argument("--objects", type=Path)
    parser.add_argument("--runtime", type=Path)
    parser.add_argument("--verify-only", action="store_true")
    parser.add_argument("--generated-rc-debug", action="store_true", help="Use Native generated count checks for diagnostic artifacts; ordinary generated RC is the default")
    parser.add_argument("--reference-go", type=Path, default=Path("/tmp/cinch-go-current/go/bin/go"))
    parser.add_argument("--artifacts", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--build-only", action="store_true")
    parser.add_argument("--measure-only", action="store_true")
    parser.add_argument("--source-override", action="append", default=[], metavar="MODULE=PATH", help="Compile a retained source-distributed baseline without changing the working library")
    parser.add_argument("--cpu", type=int, default=0)
    parser.add_argument("--sizes", nargs="+", type=int, default=[32, 1024, 16384])
    parser.add_argument("--iterations", type=int, default=64)
    parser.add_argument("--samples", type=int, default=5)
    parser.add_argument("--workloads", nargs="+", choices=WORKLOADS, default=WORKLOADS)
    args = parser.parse_args()
    if args.build_only and args.measure_only:
        parser.error("select at most one build/measure mode")
    if args.measure_only and args.source_override:
        parser.error("source overrides apply only when compiling new artifacts")
    if min(args.sizes) < 0 or min(args.iterations, args.samples) <= 0:
        parser.error("sizes must be nonnegative; samples and iterations must be positive")
    os.sched_setaffinity(0, {args.cpu})
    artifacts = args.artifacts.resolve()
    metadata = artifacts / "metadata.json"
    environment = {**os.environ, "GOCACHE": "/tmp/cinch-go-surface-cache", "GOTELEMETRY": "off", "GOMAXPROCS": "1", "GOTOOLCHAIN": "local", "CINCH_RC_DEBUG": "0", "CINCH_STACK_SIZE": "8M", "CINCH_HEAP_LIMIT": "32M", "CINCH_LEAK_CHECK": "1"}
    if not args.measure_only:
        if not args.compiler:
            parser.error("--compiler is required to build")
        compiler = args.compiler.resolve()
        overrides = {}
        for entry in args.source_override:
            name, separator, value = entry.partition("=")
            if not separator or name in overrides:
                parser.error("source overrides must be unique MODULE=PATH pairs")
            overrides[name] = Path(value).resolve()
        closure = shared.source_closure(SOURCE, overrides)
        artifacts.mkdir(parents=True, exist_ok=True)
        with tempfile.TemporaryDirectory(prefix="cinch-ring-bench-") as directory:
            work = Path(directory)
            frozen_sources = artifacts / "sources"
            frozen_sources.mkdir(exist_ok=True)
            for name, path in closure.items():
                contents = path.read_bytes()
                (work / f"{name}.cinch").write_bytes(contents)
                (frozen_sources / f"{name}.cinch").write_bytes(contents)
            (work / "Main.cinch").write_text(SOURCE)
            if args.frontend == "rust":
                if not args.runtime:
                    parser.error("Rust builds require the explicit matched --runtime archive")
                command([str(compiler), "build", str(work / "Main.cinch"), "-o", str(artifacts / "bench"), "--opt-level", "2", "--runtime-archive", str(args.runtime.resolve())], cwd=work, timeout=600, preexec_fn=compiler_stack)
            else:
                if not all([args.interfaces, args.objects, args.runtime]):
                    parser.error("native builds require --interfaces, --objects and --runtime")
                bundle = work / "Bundle.cinch"
                bundle.write_text("\n\n".join(path.read_text() for path in frozen_sources.glob("*.cinch")) + "\n\n" + SOURCE)
                configuration = json.loads(args.objects.read_text())
                compiler_environment = {**os.environ, "CINCH_HEAP_LIMIT": "256M", "CINCH_STACK_SIZE": "64M", "CINCH_RC_DEBUG": "0", "CINCH_LEAK_CHECK": "1"}
                command([str(compiler), "--emit-ir", *(["--rc-debug"] if args.generated_rc_debug else []), "--interfaces", str(args.interfaces.resolve()), "--objects", str(args.objects.resolve()), str(bundle), str(artifacts / "bench.ll")], cwd=work, env=compiler_environment, timeout=600, preexec_fn=compiler_stack)
                command([os.environ.get("CINCH_CLANG", "clang"), "-O2", str(artifacts / "bench.ll"), configuration["object"], str(args.runtime.resolve()), "-o", str(artifacts / "bench"), "-ldl", "-lpthread", "-lm"], timeout=120)
        (artifacts / "Main.cinch").write_text(SOURCE)
        reference = ROOT / "tooling/bench_ring_reference.go"
        command([str(args.reference_go), "build", "-o", str(artifacts / "reference"), str(reference)], env=environment, timeout=120)
        command([os.environ.get("CINCH_CLANG", "clang"), "-O2", "-S", "-x", "ir", str(artifacts / "bench.ll"), "-o", str(artifacts / "bench.s")], timeout=120)
        report = {"description": "Complete circular-sequence pipelines with exact ordered output checksums; ownership, snapshots and allocation representations differ", "compiler_sha256": shared.digest(compiler), "checkout_compiler_source_sha256_not_binary_provenance": shared.source_digest(list((ROOT / "compiler/src").rglob("*.rs")) + [ROOT / "compiler/Cargo.toml", ROOT / "compiler/Cargo.lock", ROOT / "compiler/build.rs"]), "runtime_source_sha256": shared.source_digest(list((ROOT / "compiler/runtime/src").rglob("*.rs"))), "driver_sha256": hashlib.sha256(SOURCE.encode()).hexdigest(), "sources": {name: shared.digest(path) for name, path in closure.items()}, "opt_level": 2, "generated_rc_debug_checks": args.frontend == "native" and args.generated_rc_debug, "runtime_poisoned_free_checks": True, "executable_sha256": shared.digest(artifacts / "bench"), "llvm_sha256": shared.digest(artifacts / "bench.ll"), "assembly_sha256": shared.digest(artifacts / "bench.s"), "reference_source_sha256": shared.digest(reference), "reference_executable_sha256": shared.digest(artifacts / "reference"), "reference_sdk": command([str(args.reference_go), "version"]).stdout.strip(), "artifacts": str(artifacts), "results": []}
        report["frontend"] = args.frontend
        report["compiler_host_stack"] = "64M"
        report["runtime_sha256"] = shared.digest(args.runtime)
        provenance = compiler.parent / "provenance.json"
        if provenance.exists():
            frozen = json.loads(provenance.read_text())
            if frozen["compiler_sha256"] != shared.digest(compiler) or frozen["runtime_sha256"] != shared.digest(args.runtime):
                raise ValueError("frozen compiler/runtime identity mismatch")
            report["frozen_compiler_provenance"] = frozen
        if args.frontend == "native":
            report["native_provenance"] = {"compiler_sha256": shared.digest(compiler), "interfaces_sha256": shared.digest(args.interfaces), "objects_sha256": shared.digest(args.objects), "runtime_sha256": shared.digest(args.runtime), "compiler_snapshot_sources": {path.name: shared.digest(path) for path in sorted((compiler.parent / "sources").glob("*.cinch"))}}
        report["source_snapshots"] = str(frozen_sources)
        compiler_snapshot = compiler.parent / "snapshot.json"
        if compiler_snapshot.exists():
            provenance = json.loads(compiler_snapshot.read_text())
            if provenance.get("compiler_sha256") != shared.digest(compiler):
                raise ValueError("frozen compiler snapshot hash does not match the selected executable")
            report["frozen_compiler_provenance"] = provenance
        report["clang"] = command([os.environ.get("CINCH_CLANG", "clang"), "--version"]).stdout.splitlines()[0]
        metadata.write_text(json.dumps(report, indent=2) + "\n")
    else:
        report = json.loads(metadata.read_text())
    if args.build_only:
        print("Built frozen benchmark artifacts", flush=True)
        return
    for name, key in [("bench", "executable_sha256"), ("reference", "reference_executable_sha256"), ("bench.ll", "llvm_sha256")]:
        if shared.digest(artifacts / name) != report[key]:
            raise ValueError(f"artifact changed: {name}")
    for name, expected in report["sources"].items():
        if shared.digest(Path(report["source_snapshots"]) / f"{name}.cinch") != expected:
            raise ValueError(f"source snapshot changed: {name}")
    if args.verify_only:
        verified = []
        for workload in args.workloads:
            for size in args.sizes:
                observations = {}
                for label in ["cinch", "reference"]:
                    executable = artifacts / ("bench" if label == "cinch" else "reference")
                    result = command([str(executable), str(WORKLOADS.index(workload)), str(size), str(args.iterations)], env=environment, timeout=180)
                    if label == "cinch" and "leaked=0" not in result.stderr:
                        raise ValueError("missing zero-leak functional evidence")
                    observations[label] = int(result.stdout.split()[1])
                if observations["cinch"] != observations["reference"]:
                    raise ValueError(f"functional checksum mismatch: {workload}/{size}: {observations}")
                if size <= 32:
                    poisoned = {**environment, "CINCH_HEAP_LIMIT": "256M", "CINCH_RC_DEBUG": "1"}
                    result = command([str(artifacts / "bench"), str(WORKLOADS.index(workload)), str(size), str(args.iterations)], env=poisoned, timeout=180)
                    if "leaked=0" not in result.stderr or int(result.stdout.split()[1]) != observations["cinch"]:
                        raise ValueError("poisoned functional verification failed")
                verified.append({"workload": workload, "elements": size, "iterations": args.iterations, "checksum": observations["cinch"], "leaked": 0})
                print(f"Verified {workload}/{size}", flush=True)
        (artifacts / "verification.json").write_text(json.dumps({"frontend": report["frontend"], "executable_sha256": report["executable_sha256"], "checks": verified}, indent=2) + "\n")
        return
    report["timing_started_utc"] = datetime.datetime.now(datetime.timezone.utc).isoformat()
    report["background_note"] = "Only the selected CPU is reserved for this timing window. Compiler/validation work may overlap on other assigned CPUs; shared memory and caches are not isolated."
    process_snapshot = command(["ps", "-eo", "pid,psr,pcpu,comm"]).stdout.splitlines()
    report["background_process_snapshot"] = [line for line in process_snapshot if any(name in line for name in ["rustc", "cargo", "clang", "cinch", "python", "go"]) and str(os.getpid()) not in line.split()]
    report.update(cpu_affinity=[args.cpu], iterations=args.iterations, samples=args.samples, timing="runtime monotonic clock around operation loop; startup, configuration and preparation excluded", results=[])
    for workload in args.workloads:
        for size in args.sizes:
            samples = {"cinch": [], "reference": []}
            checksums = set()
            for index in range(args.samples + 1):
                for label in (["cinch", "reference"] if index % 2 == 0 else ["reference", "cinch"]):
                    executable = artifacts / ("bench" if label == "cinch" else "reference")
                    result = command([str(executable), str(WORKLOADS.index(workload)), str(size), str(args.iterations)], env=environment, timeout=180)
                    if label == "cinch" and "leaked=0" not in result.stderr:
                        raise ValueError("missing zero-leak evidence from measured process")
                    elapsed, checksum = map(int, result.stdout.split())
                    checksums.add(checksum)
                    if index:
                        samples[label].append(elapsed)
            if len(checksums) != 1:
                raise ValueError(f"checksum mismatch: {workload}/{size}: {checksums}")
            row = {"workload": workload, "elements": size, "checksum": checksums.pop(), "samples_ns": samples, "median_ns_per_pipeline": {label: statistics.median(values) / args.iterations for label, values in samples.items()}}
            report["results"].append(row)
            print(workload, size, row["median_ns_per_pipeline"], flush=True)
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(report, indent=2) + "\n")


if __name__ == "__main__":
    main()
