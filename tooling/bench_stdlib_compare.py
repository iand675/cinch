#!/usr/bin/env python3
"""Alternate retained public-library executables while checking matched compilation inputs.

This measures local pipeline changes without a timing threshold. Compiler,
optimization level, driver and runtime source identities must match; differences
in library sources remain explicit. Independent SDK results stay in the input
reports rather than being attributed to the changed Cinch library.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import statistics
import subprocess


def digest(path: Path) -> str:
    """Identify executable and LLVM bytes rather than trusting retained filenames."""
    return hashlib.sha256(path.read_bytes()).hexdigest()


def sample(executable: Path, workload: str, size: int, iterations: int) -> tuple[int, int]:
    """Run a bounded timed workload and preserve its observable output checksum."""
    result = subprocess.run([str(executable), workload, str(size), str(iterations)],
                            check=True, capture_output=True, text=True, timeout=120)
    elapsed, checksum = map(int, result.stdout.split())
    return elapsed, checksum


def main() -> None:
    """Verify both artifacts, alternate their order and retain every measured sample."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--before", type=Path, required=True)
    parser.add_argument("--after", type=Path, required=True)
    parser.add_argument("--workloads", nargs="+", required=True)
    parser.add_argument("--sizes", nargs="+", type=int, default=[32, 1024, 16384])
    parser.add_argument("--iterations", type=int, default=256)
    parser.add_argument("--samples", type=int, default=9)
    parser.add_argument("--cpu", type=int, default=0)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    if min(args.sizes) < 0 or min(args.iterations, args.samples) <= 0:
        parser.error("sizes must be nonnegative; iterations and samples must be positive")
    if args.cpu not in os.sched_getaffinity(0):
        parser.error("--cpu must name an allowed CPU")
    os.sched_setaffinity(0, {args.cpu})
    reports = {"before": json.loads(args.before.read_text()), "after": json.loads(args.after.read_text())}
    matched = ("compiler_sha256", "compiler_source_sha256", "runtime_source_sha256", "driver_sha256", "opt_level")
    for key in matched:
        if reports["before"][key] != reports["after"][key]:
            raise ValueError(f"unmatched compilation input: {key}")
    report = {"format": 1, "description": "Alternating retained executables with matched compiler, driver, optimization and runtime sources",
              **{key: reports["before"][key] for key in matched}, "iterations": args.iterations,
              "cpu": reports["before"]["cpu"], "cpu_affinity": [args.cpu], "warmup_processes_per_case": 1,
              "inputs": {}, "results": []}
    executables = {}
    for label, entry in reports.items():
        artifacts = Path(entry["artifacts"])
        executable = artifacts / "bench"
        if digest(executable) != entry["executable_sha256"] or digest(artifacts / "bench.ll") != entry["llvm_sha256"]:
            raise ValueError(f"retained {label} executable or LLVM no longer matches its input report")
        executables[label] = executable
        report["inputs"][label] = {"report": str(args.before if label == "before" else args.after),
                                  "sources": entry["sources"], "executable_sha256": digest(executable),
                                  "llvm_sha256": digest(artifacts / "bench.ll")}
    for workload in args.workloads:
        for size in args.sizes:
            for exe in executables.values():
                sample(exe, workload, size, args.iterations)
            samples = {"before": [], "after": []}
            checksums = set()
            for index in range(args.samples):
                order = ["before", "after"] if index % 2 == 0 else ["after", "before"]
                for label in order:
                    elapsed, checksum = sample(executables[label], workload, size, args.iterations)
                    samples[label].append(elapsed)
                    checksums.add(checksum)
            if len(checksums) != 1:
                raise ValueError(f"output mismatch for {workload}/{size}: {checksums}")
            medians = {label: statistics.median(values) / args.iterations for label, values in samples.items()}
            item = {"workload": workload, "raw_input_bytes": size, "checksum": checksums.pop(),
                    "samples_ns": samples, "median_ns_per_operation": medians,
                    "before_over_after": medians["before"] / medians["after"]}
            report["results"].append(item)
            print(f"{workload} {size}: {item['before_over_after']:.3f}x", flush=True)
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(report, indent=2) + "\n")


if __name__ == "__main__":
    main()
