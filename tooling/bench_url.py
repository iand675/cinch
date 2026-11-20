#!/usr/bin/env python3
"""Compile or measure public URL pipelines with independent observable results.

Compilation runs on its assigned functional CPU. Measurement is a separate
invocation on an explicitly coordinated CPU, preserving frozen executables,
raw samples, source identities and public pipeline conversion costs.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import platform
import shutil
import statistics
import subprocess

from validate_url import ROOT, BUILD_ROOT, source_closure, toolchain_identity

MODES = ("query-escape", "path-escape", "query-unescape", "path-unescape", "query-values", "parse-render", "resolve", "join", "escape-length", "bytes-copy")


def digest(path: Path) -> str:
    """Identify exact compiler, source and retained executable bytes."""
    return hashlib.sha256(path.read_bytes()).hexdigest()


def source_digest(paths: list[Path]) -> str:
    """Hash sorted relative source names and contents, including uncommitted implementation changes."""
    result = hashlib.sha256()
    for path in sorted(paths):
        result.update(str(path.relative_to(ROOT)).encode() + b"\0")
        result.update(path.read_bytes())
        result.update(b"\0")
    return result.hexdigest()


def run(command: list[str], **kwargs) -> subprocess.CompletedProcess:
    """Execute one bounded build or sample and preserve useful failure output."""
    result = subprocess.run(command, text=True, capture_output=True, **kwargs)
    if result.returncode != 0:
        raise RuntimeError(f"command failed: {command}\n{result.stdout}\n{result.stderr}")
    return result


def main() -> None:
    """Build frozen artifacts or measure them on the explicitly selected CPU."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("action", choices=["build", "measure"])
    parser.add_argument("--compiler", type=Path, default=Path("/workspace/dasl-stdlib-io/build/prim-wide-frozen/cinch"))
    parser.add_argument("--go", type=Path, default=Path("/tmp/cinch-go-current/go/bin/go"))
    parser.add_argument("--cpu", type=int, required=True)
    parser.add_argument("--artifacts", type=Path, required=True)
    parser.add_argument("--output", type=Path)
    parser.add_argument("--sizes", nargs="+", type=int, default=[32, 1024, 16384, 65536])
    parser.add_argument("--patterns", nargs="+", type=int, default=[0, 1, 2])
    parser.add_argument("--iterations", type=int, default=64)
    parser.add_argument("--samples", type=int, default=5)
    parser.add_argument("--workloads", nargs="+", choices=MODES, default=list(MODES))
    parser.add_argument("--source-override", type=Path, help="Use a retained URL source directory for an isolated implementation comparison")
    args = parser.parse_args()
    os.sched_setaffinity(0, {args.cpu})
    artifacts = args.artifacts.resolve()
    identity = artifacts / "identity.json"
    environment = {**os.environ, "GOCACHE": "/tmp/cinch-go-url-benchmark-cache", "GOTELEMETRY": "off", "GOMAXPROCS": "1"}
    if args.action == "build":
        project = artifacts / "project"
        project.mkdir(parents=True, exist_ok=True)
        driver = project / "Main.cinch"
        driver.write_bytes((ROOT / "tooling/data/url-benchmark.cinch").read_bytes())
        inputs = source_closure(project, driver) + [driver]
        if args.source_override:
            for original in args.source_override.glob("*.cinch"):
                (project / ("Text.Url." + original.stem + ".cinch")).write_bytes(original.read_bytes())
        compiler = artifacts / "compiler"
        shutil.copy2(args.compiler, compiler)
        run([str(compiler), "build", str(driver), "-o", str(artifacts / "bench"), "--opt-level", "2"], cwd=project)
        reference = ROOT / "tooling/bench_url_reference.go"
        run([str(args.go), "build", "-o", str(artifacts / "reference"), str(reference)], env=environment)
        run([os.environ.get("CINCH_CLANG", "clang"), "-O2", "-S", "-x", "ir", str(artifacts / "bench.ll"), "-o", str(artifacts / "bench.s")])
        report = {"compiler_sha256": digest(compiler), "executable_sha256": digest(artifacts / "bench"), "llvm_sha256": digest(artifacts / "bench.ll"), "assembly_sha256": digest(artifacts / "bench.s"), "sources": {path.name: digest(path) for path in inputs}, "reference_source_sha256": digest(reference), "reference_executable_sha256": digest(artifacts / "reference"), "reference_sdk": run([str(args.go), "version"], env=environment).stdout.strip(), "opt_level": 2, "artifacts": str(artifacts), "runner_sha256": digest(Path(__file__)), "driver_sha256": digest(driver), "compiler_source_sha256": source_digest(list((ROOT / "seed/src").rglob("*.rs")) + [ROOT / "seed/Cargo.toml", ROOT / "seed/Cargo.lock", ROOT / "seed/build.rs"]), "runtime_source_sha256": source_digest(list((ROOT / "seed/runtime/src").rglob("*.rs")) + [ROOT / "seed/runtime/Cargo.toml", ROOT / "seed/runtime/Cargo.lock"]), "cpu": next(line.split(":", 1)[1].strip() for line in Path("/proc/cpuinfo").read_text().splitlines() if line.startswith("model name"))}
        report["toolchain"] = toolchain_identity(BUILD_ROOT, "rust", args.compiler.resolve())
        report["compiler_source_sha256_scope"] = "Worktree context; exact frozen compiler source snapshot is recorded in toolchain.compiler_snapshot"
        report["source_override"] = str(args.source_override.resolve()) if args.source_override else None
        identity.write_text(json.dumps(report, indent=2) + "\n")
        print("Built frozen URL benchmark artifacts", flush=True)
        return
    if args.output is None:
        parser.error("measurement requires --output")
    report = json.loads(identity.read_text())
    for key, path in [("executable_sha256", "bench"), ("llvm_sha256", "bench.ll"), ("reference_executable_sha256", "reference")]:
        if digest(artifacts / path) != report[key]:
            raise ValueError(f"retained artifact changed: {path}")
    report.update(cpu_affinity=[args.cpu], platform=platform.platform(), cpu=next(line.split(":", 1)[1].strip() for line in Path("/proc/cpuinfo").read_text().splitlines() if line.startswith("model name")), iterations=args.iterations, samples=args.samples, warmup_processes=1, timing="Monotonic clock around public operation loop; startup/input preparation excluded; first-byte input mutation and observable result included", pipeline="Public byte pipelines include required allocations and byte preservation. Byte reference modes use the SDK string API; Cinch byte outputs always copy. The length reference computes len(QueryEscape), which performs different work from a counting API.", results=[])
    for mode in args.workloads:
        for pattern in args.patterns:
            for size in args.sizes:
                samples = {"cinch": [], "reference": []}
                checksums = set()
                for index in range(args.samples + 1):
                    order = ["cinch", "reference"] if index % 2 == 0 else ["reference", "cinch"]
                    for label in order:
                        executable = artifacts / ("bench" if label == "cinch" else "reference")
                        result = run([str(executable), mode, str(size), str(args.iterations), str(pattern)], env=environment)
                        elapsed, checksum = map(int, result.stdout.split())
                        checksums.add(checksum)
                        if index:
                            samples[label].append(elapsed)
                if len(checksums) != 1:
                    raise ValueError(f"reference mismatch for {mode}/{size}/{pattern}: {checksums}")
                medians = {label: statistics.median(values) / args.iterations for label, values in samples.items()}
                report["results"].append({"workload": mode, "raw_input_bytes": size, "pattern": pattern, "samples_ns": samples, "median_ns_per_operation": medians, "checksum": checksums.pop()})
                print(mode, size, pattern, medians, flush=True)
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(report, indent=2) + "\n")


if __name__ == "__main__":
    main()
