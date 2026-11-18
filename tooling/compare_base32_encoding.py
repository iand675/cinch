#!/usr/bin/env python3
"""Alternate frozen Base32 pipeline executables while enforcing matched compilation inputs."""
from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import statistics
import subprocess

from bench_base32_encoding import WORKLOADS


def digest(path: Path) -> str:
    """Verify exact retained bytes rather than trusting executable filenames."""
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main() -> None:
    """Check inputs, alternate execution order and preserve every sample without a timing threshold."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--before", type=Path, required=True, help="Baseline artifact directory containing metadata.json")
    parser.add_argument("--after", type=Path, required=True, help="Final artifact directory containing metadata.json")
    parser.add_argument("--allowed-source", nargs="+", default=["Effect.Base32"])
    parser.add_argument("--workloads", nargs="+", choices=WORKLOADS, default=["stream-decode64", "encode-bytes", "decode-bytes"])
    parser.add_argument("--sizes", nargs="+", type=int, default=[32, 16384, 65536])
    parser.add_argument("--iterations", type=int, default=64)
    parser.add_argument("--samples", type=int, default=7)
    parser.add_argument("--cpu", type=int, default=0)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    if min(args.sizes) < 0 or min(args.iterations, args.samples) <= 0:
        parser.error("sizes must be nonnegative; iterations and samples must be positive")
    os.sched_setaffinity(0, {args.cpu})
    paths = {"before": args.before.resolve(), "after": args.after.resolve()}
    metadata = {label: json.loads((path / "metadata.json").read_text()) for label, path in paths.items()}
    for key in ["compiler_sha256", "compiler_source_sha256", "runtime_source_sha256", "driver_sha256", "opt_level", "rc_debug_capability"]:
        if metadata["before"][key] != metadata["after"][key]:
            raise ValueError(f"unmatched compilation input: {key}")
    if set(metadata["before"]["sources"]) != set(metadata["after"]["sources"]):
        raise ValueError("source dependency closures differ")
    changed = [name for name in metadata["before"]["sources"] if metadata["before"]["sources"][name] != metadata["after"]["sources"][name]]
    if sorted(changed) != sorted(args.allowed_source):
        raise ValueError(f"unexpected source changes: {changed}")
    for label, path in paths.items():
        for filename, key in [("bench", "executable_sha256"), ("bench.ll", "llvm_sha256"), ("bench.s", "assembly_sha256")]:
            if digest(path / filename) != metadata[label][key]:
                raise ValueError(f"changed retained artifact: {label}/{filename}")
        for name, expected in metadata[label]["sources"].items():
            if digest(Path(metadata[label]["source_snapshots"]) / f"{name}.cinch") != expected:
                raise ValueError(f"changed frozen source: {label}/{name}")
    report = {"description": "Alternating retained public pipelines; compiler, driver, runtime and all unrelated library sources match", "inputs": metadata, "changed_sources": changed, "cpu_affinity": [args.cpu], "iterations": args.iterations, "samples": args.samples, "warmup_processes_per_case": 1, "results": []}
    environment = {**os.environ, "CINCH_RC_DEBUG": "0", "CINCH_STACK_SIZE": "8M", "CINCH_HEAP_LIMIT": "32M", "CINCH_LEAK_CHECK": "1"}
    for workload in args.workloads:
        for size in args.sizes:
            samples = {label: [] for label in paths}
            checksums = set()
            for index in range(args.samples + 1):
                for label in (["before", "after"] if index % 2 == 0 else ["after", "before"]):
                    result = subprocess.run([str(paths[label] / "bench"), str(WORKLOADS.index(workload)), str(size), str(args.iterations)], capture_output=True, text=True, env=environment, timeout=180)
                    if result.returncode or "leaked=0" not in result.stderr:
                        raise RuntimeError(f"{label}/{workload}/{size}: {result.returncode}\n{result.stderr}")
                    elapsed, checksum = map(int, result.stdout.split())
                    checksums.add(checksum)
                    if index:
                        samples[label].append(elapsed)
            if len(checksums) != 1:
                raise ValueError(f"observable mismatch: {workload}/{size}: {checksums}")
            medians = {label: statistics.median(values) / args.iterations for label, values in samples.items()}
            report["results"].append({"workload": workload, "input_bytes": size, "checksum": checksums.pop(), "samples_ns": samples, "median_ns_per_operation": medians, "before_over_after": medians["before"] / medians["after"]})
            print(workload, size, report["results"][-1]["before_over_after"], flush=True)
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(report, indent=2) + "\n")


if __name__ == "__main__":
    main()
