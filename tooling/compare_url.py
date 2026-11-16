#!/usr/bin/env python3
"""Alternate retained URL executables with explicit corpus patterns and matched compilation inputs."""
from __future__ import annotations

import argparse
import json
import os
from pathlib import Path
import statistics

from bench_url import MODES, digest, run


def sample(executable: Path, mode: str, size: int, iterations: int, pattern: int) -> tuple[int, int]:
    """Retain one complete public pipeline's elapsed time and observable result."""
    result = run([str(executable), mode, str(size), str(iterations), str(pattern)])
    return tuple(map(int, result.stdout.split()))


def main() -> None:
    """Check identities, alternate process order and preserve every sample without timing thresholds."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--before", type=Path, required=True)
    parser.add_argument("--after", type=Path, required=True)
    parser.add_argument("--cpu", type=int, required=True)
    parser.add_argument("--iterations", type=int, default=128)
    parser.add_argument("--samples", type=int, default=9)
    parser.add_argument("--sizes", nargs="+", type=int, default=[32, 65536])
    parser.add_argument("--patterns", nargs="+", type=int, default=[0, 1, 2])
    parser.add_argument("--workloads", nargs="+", choices=MODES, default=list(MODES))
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    os.sched_setaffinity(0, {args.cpu})
    inputs = {"before": json.loads(args.before.read_text()), "after": json.loads(args.after.read_text())}
    matched = ["compiler_sha256", "compiler_source_sha256", "runtime_source_sha256", "driver_sha256", "opt_level"]
    for key in matched:
        if inputs["before"][key] != inputs["after"][key]:
            raise ValueError(f"unmatched compilation input: {key}")
    executables = {}
    report = {"description": "Alternating URL public pipelines with explicit corpus patterns; matched compiler, runtime, driver and optimization", **{key: inputs["before"][key] for key in matched}, "cpu": inputs["before"]["cpu"], "cpu_affinity": [args.cpu], "iterations": args.iterations, "warmup_processes_per_case": 1, "inputs": {}, "results": []}
    for label, source in inputs.items():
        artifacts = Path(source["artifacts"])
        exe = artifacts / "bench"
        if digest(exe) != source["executable_sha256"] or digest(artifacts / "bench.ll") != source["llvm_sha256"]:
            raise ValueError(f"retained executable or LLVM changed: {label}")
        executables[label] = exe
        report["inputs"][label] = {"report": str(args.before if label == "before" else args.after), "sources": source["sources"], "executable_sha256": source["executable_sha256"], "llvm_sha256": source["llvm_sha256"]}
    for mode in args.workloads:
        for pattern in args.patterns:
            for size in args.sizes:
                for exe in executables.values():
                    sample(exe, mode, size, args.iterations, pattern)
                samples = {"before": [], "after": []}
                checksums = set()
                for index in range(args.samples):
                    order = ["before", "after"] if index % 2 == 0 else ["after", "before"]
                    for label in order:
                        elapsed, checksum = sample(executables[label], mode, size, args.iterations, pattern)
                        checksums.add(checksum)
                        samples[label].append(elapsed)
                if len(checksums) != 1:
                    raise ValueError(f"observable mismatch: {mode}/{size}/{pattern}: {checksums}")
                medians = {key: statistics.median(values) / args.iterations for key, values in samples.items()}
                ratio = medians["before"] / medians["after"]
                report["results"].append({"workload": mode, "raw_input_bytes": size, "pattern": pattern, "checksum": checksums.pop(), "samples_ns": samples, "median_ns_per_operation": medians, "before_over_after": ratio})
                print(mode, size, pattern, f"{ratio:.3f}x", flush=True)
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(report, indent=2) + "\n")


if __name__ == "__main__":
    main()
