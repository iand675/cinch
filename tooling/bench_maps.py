#!/usr/bin/env python3
"""Build, independently preflight, or measure complete public integer-map workloads."""
from __future__ import annotations

import argparse
import datetime
import json
import os
from pathlib import Path
import re
import statistics
import subprocess

from generate_map_benchmark import WORKLOADS, expected, source
from validate_maps import ROOT, digest, execute

MODULES = ("Data.Mutable.Map", "Data.Mutable.HashTable", "Data.Mutable.LinkedList", "Data.Mutable.Var")
VARIANTS = ("hash-table", "stable-map")


def cpu_budget() -> dict[str, str]:
    """Record shared cgroup limits/throttling without treating them as process-local measurements."""
    root = Path("/sys/fs/cgroup")
    return {name: (root / name).read_text() for name in ("cpu.max", "cpu.stat") if (root / name).exists()}


def main() -> None:
    """Use one caller-granted CPU token and preserve exact source/compiler/runtime artifacts."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--action", choices=("build", "preflight", "measure"), required=True)
    parser.add_argument("--frontend", choices=("rust", "native"), default="rust")
    parser.add_argument("--compiler", type=Path)
    parser.add_argument("--runtime", type=Path)
    parser.add_argument("--interfaces", type=Path)
    parser.add_argument("--objects", type=Path)
    parser.add_argument("--variants", nargs="+", choices=VARIANTS, default=VARIANTS)
    parser.add_argument("--sizes", nargs="+", type=int, default=(0, 1, 31, 256, 4096))
    parser.add_argument("--iterations", type=int, default=8)
    parser.add_argument("--samples", type=int, default=5)
    parser.add_argument("--cpu", type=int, default=4)
    parser.add_argument("--artifacts", type=Path, required=True)
    args = parser.parse_args()
    if min(args.sizes) < 0 or max(args.sizes) > 65536 or not 0 < args.iterations <= 256 or args.samples < 1:
        parser.error("bounded inputs require 0..65536 elements, 1..256 iterations and positive samples")
    if args.action == "build" and not (args.compiler and args.runtime):
        parser.error("building requires explicit compiler and matched runtime")
    if args.action == "build" and args.frontend == "native" and not (args.interfaces and args.objects):
        parser.error("Native building requires explicit matched interface/object manifests")
    os.sched_setaffinity(0, {args.cpu})
    artifacts = args.artifacts.resolve()
    artifacts.mkdir(parents=True, exist_ok=True)
    environment = {**os.environ, "RUST_MIN_STACK": "67108864", "CINCH_TEST_API_EMIT_INTERFACES": "0"}
    if args.action == "build":
        for variant in args.variants:
            directory = artifacts / variant
            directory.mkdir(parents=True, exist_ok=True)
            entry = directory / "Main.cinch"
            entry.write_text(source(variant))
            modules = {}
            for name in MODULES:
                module = directory / (name + ".cinch")
                module.write_bytes((ROOT / "compiler/prelude" / (name.replace(".", "/") + ".cinch")).read_bytes())
                modules[name] = module
            executable, llvm = directory / "consumer", directory / "consumer.ll"
            report = {"kind": "public-map-benchmark-build", "variant": variant, "frontend": args.frontend,
                      "compiler_sha256": digest(args.compiler), "runtime_sha256": digest(args.runtime),
                      "consumer_sha256": digest(entry), "sources": {name: digest(path) for name, path in modules.items()},
                      "runner_sha256": digest(Path(__file__)), "opt_level": 2, "generated_rc_debug": False,
                      "status": "running", "commands": []}
            output = directory / "build.json"
            try:
                if args.frontend == "rust":
                    command = [str(args.compiler.resolve()), "build", str(entry), "-o", str(executable), "--opt-level", "2", "--runtime-archive", str(args.runtime.resolve())]
                else:
                    bundle = directory / "Bundle.cinch"
                    bundle.write_text("\n\n".join(path.read_text() for path in modules.values()) + "\n\n" + entry.read_text())
                    command = ["python3", str(args.compiler.resolve()), "--emit-ir", "--interfaces", str(args.interfaces.resolve()), str(bundle), str(llvm)]
                    provider_description = json.loads(args.objects.read_text())
                    provider = Path(provider_description["object"])
                    assert digest(provider) == provider_description["object_sha256"], "provider object differs from its retained manifest"
                    if "metadata" in provider_description:
                        assert digest(Path(provider_description["metadata"])) == provider_description["metadata_sha256"], "provider metadata differs from its retained manifest"
                    report.update(raw_compiler_sha256=digest(args.compiler.parent / "cinch"), interfaces_sha256=digest(args.interfaces), objects_sha256=digest(args.objects), provider_sha256=digest(provider))
                result = execute(command, directory, environment, host=True)
                report["commands"].append(result)
                assert result["exit"] == 0, result
                if args.frontend == "native":
                    result = execute([environment.get("CINCH_CLANG", "clang"), "-O2", str(llvm), str(provider), str(args.runtime.resolve()), "-o", str(executable), "-ldl", "-lpthread", "-lm"], directory, environment)
                    report["commands"].append(result)
                    assert result["exit"] == 0, result
                assembly = directory / "consumer.s"
                result = execute([environment.get("CINCH_CLANG", "clang"), "-O2", "-S", "-x", "ir", str(llvm), "-o", str(assembly)], directory, environment)
                report["commands"].append(result)
                assert result["exit"] == 0, result
                report.update(status="passed", executable_sha256=digest(executable), llvm_sha256=digest(llvm), assembly_sha256=digest(assembly))
            except BaseException as error:
                report.update(status="failed", failure=str(error))
                raise
            finally:
                output.write_text(json.dumps(report, indent=2) + "\n")
            print(f"{variant}: immutable O2 benchmark artifacts built", flush=True)
        return

    builds = {variant: json.loads((artifacts / variant / "build.json").read_text()) for variant in args.variants}
    for variant, build in builds.items():
        assert build["status"] == "passed"
        for filename, field in (("consumer", "executable_sha256"), ("consumer.ll", "llvm_sha256"), ("consumer.s", "assembly_sha256")):
            assert digest(artifacts / variant / filename) == build[field], (variant, filename)
        assert digest(artifacts / variant / "Main.cinch") == build["consumer_sha256"]
        for name, identity in build["sources"].items():
            assert digest(artifacts / variant / (name + ".cinch")) == identity, (variant, name)
    if args.action == "measure":
        preflight_path = artifacts / "preflight.json"
        preflight = json.loads(preflight_path.read_text())
        assert preflight["status"] == "passed" and preflight["iterations"] == args.iterations
        assert set(args.sizes) <= set(preflight["sizes"]), "every measured size needs its own successful preflight"
        for variant, build in builds.items():
            assert preflight["builds"][variant] == build, "preflight belongs to different artifacts"
    report = {"kind": "public-map-benchmark-" + args.action, "builds": builds, "cpu_affinity": [args.cpu],
              "runner_sha256": digest(Path(__file__)),
              "workloads": WORKLOADS, "iterations": args.iterations, "sizes": args.sizes, "results": [],
              "started_utc": datetime.datetime.now(datetime.timezone.utc).isoformat(),
              "contract": "Identical Nat inputs without callback mutation; HashTable traversal materializes its public value snapshot, MutableMap captures stable identities. No equivalence under callback mutation is claimed.",
              "timing": "Runtime monotonic interval includes complete pipeline; prepared map construction excluded for scan workloads; process teardown excluded. Normal generated RC only. Allocation counters run only in preflight and include startup/preparation/teardown.",
              "status": "running"}
    if args.action == "measure":
        report["preflight_sha256"] = digest(preflight_path)
        report["shared_cgroup_before"] = cpu_budget()
        report["background_process_snapshot"] = subprocess.run(["ps", "-eo", "pid,psr,pcpu,comm"], capture_output=True, text=True, check=True).stdout
    output = artifacts / (args.action + ".json")
    try:
        for mode, workload in enumerate(WORKLOADS):
            for size in args.sizes:
                observation = {"workload": workload, "size": size, "expected": expected(mode, size, args.iterations), "variants": {name: [] for name in args.variants}}
                report["results"].append(observation)
                repetitions = 2 if args.action == "preflight" else args.samples + 1
                for repetition in range(repetitions):
                    for variant in args.variants if repetition % 2 == 0 else reversed(args.variants):
                        poisoned = args.action == "preflight" and repetition == 1
                        runtime_environment = {**environment, "CINCH_HEAP_LIMIT": "256M" if poisoned else "32M", "CINCH_STACK_SIZE": "8M", "CINCH_RC_DEBUG": "1" if poisoned else "0", "CINCH_LEAK_CHECK": "1", "CINCH_ALLOC_STATS": "1" if args.action == "preflight" else "0"}
                        executable = artifacts / variant / "consumer"
                        # Identical argv[0] keeps captured process arguments from changing allocation profiles.
                        result = subprocess.run(["map-benchmark", str(mode), str(size), str(args.iterations)], executable=executable, cwd=artifacts / variant, env=runtime_environment, text=True, capture_output=True, timeout=90)
                        fields = result.stdout.split()
                        record = {"exit": result.returncode, "stdout": result.stdout, "stderr": result.stderr, "poisoned": poisoned, "warmup": args.action == "measure" and repetition == 0}
                        observation["variants"][variant].append(record)
                        assert result.returncode == 0 and len(fields) == 2, (variant, workload, size, record)
                        record.update(nanoseconds=int(fields[0]), checksum=int(fields[1]))
                        assert record["checksum"] == observation["expected"], (variant, workload, size, record, observation["expected"])
                        assert "cinch-leak: live=0 immortal=0 leaked=0" in result.stderr, record
                        if args.action == "preflight":
                            allocation = re.search(r"cinch-alloc: blocks=(\d+) bytes=(\d+) peak=(\d+) header-blocks=(\d+)", result.stderr)
                            assert allocation, record
                            record["allocation"] = dict(zip(("blocks", "bytes", "peak", "header_blocks"), map(int, allocation.groups())))
                if args.action == "measure":
                    observation["median_ns"] = {variant: statistics.median(row["nanoseconds"] for row in rows if not row["warmup"]) for variant, rows in observation["variants"].items()}
                output.write_text(json.dumps(report, indent=2) + "\n")
                print(f"{workload}/{size}: independent fingerprint and zero-leak profiles passed", flush=True)
        report["status"] = "passed"
    except BaseException as error:
        report.update(status="failed", failure=str(error))
        raise
    finally:
        if args.action == "measure":
            report["shared_cgroup_after"] = cpu_budget()
        output.write_text(json.dumps(report, indent=2) + "\n")


if __name__ == "__main__":
    main()
