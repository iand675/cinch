#!/usr/bin/env python3
"""Freeze map sources and validate independent operations with explicit frontend/runtime identities."""
from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import resource
import shutil
import subprocess
import time

from generate_maps_cases import edge_source, float_source, source


ROOT = Path(__file__).resolve().parents[1]
MODULES = (
    "Data.Mutable.Map", "Data.Mutable.Map.Optional", "Data.Mutable.Map.Float",
    "Data.Mutable.HashTable", "Data.Mutable.LinkedList", "Data.Mutable.Var",
)


def digest(path: Path) -> str:
    """Identify exact compiler, provider, runtime and source artifacts."""
    return hashlib.sha256(path.read_bytes()).hexdigest()


def compiler_stack() -> None:
    """Give only the compiler host a 64 MiB stack; runtime processes use 8 MiB."""
    soft, hard = resource.getrlimit(resource.RLIMIT_STACK)
    desired = 64 * 1024 * 1024
    resource.setrlimit(resource.RLIMIT_STACK, (desired if hard == resource.RLIM_INFINITY else min(desired, hard), hard))


def execute(command: list[str], directory: Path, environment: dict, host: bool = False) -> dict:
    """Retain every command's verdict and diagnostics without conflating check and runtime gates."""
    started = time.monotonic()
    result = subprocess.run(command, cwd=directory, env=environment, capture_output=True, text=True,
                            preexec_fn=compiler_stack if host else None, timeout=180)
    return {"command": command, "exit": result.returncode, "seconds": time.monotonic() - started,
            "stdout": result.stdout, "stderr": result.stderr}


def main() -> None:
    """Run only requested suites under a caller-granted CPU lease, one compiler at a time."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--frontend", choices=("rust", "native"), required=True)
    parser.add_argument("--compiler", type=Path, required=True)
    parser.add_argument("--runtime", type=Path, required=True)
    parser.add_argument("--interfaces", type=Path)
    parser.add_argument("--objects", type=Path)
    parser.add_argument("--source-override", action="append", default=[], metavar="MODULE=PATH", help="Select a separate candidate without changing the production library")
    parser.add_argument("--minimum-free-mib", type=int, default=0, help="Require this free disk budget before each consumer")
    parser.add_argument("--minimum-reserve-mib", type=int, default=0, help="Preserve this free disk reserve after each consumer")
    parser.add_argument("--maximum-unit-mib", type=int, default=0, help="Bound each consumer's new output directory")
    parser.add_argument("--cpu", type=int, default=0)
    parser.add_argument("--opt-level", choices=("0", "2"), default="2")
    parser.add_argument("--modes", nargs="+", choices=("source", "region", "negative", "control", "edge", "effect", "trace", "float"), default=["negative", "control", "edge", "effect", "trace", "float"])
    parser.add_argument("--cases", nargs="+", type=int, choices=range(8), default=list(range(8)))
    parser.add_argument("--float-widths", nargs="+", type=int, choices=(32, 64), default=[32, 64], help="Select one bounded floating-key consumer when disk units must be serialized")
    parser.add_argument("--artifacts", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    if args.frontend == "native" and (not args.interfaces or not args.objects):
        parser.error("Native gates require explicit matched interface and object manifests")
    if args.frontend == "rust" and (args.interfaces or args.objects):
        parser.error("Rust whole-source gates do not link a Native provider object")
    os.sched_setaffinity(0, {args.cpu})
    compiler, runtime = args.compiler.resolve(), args.runtime.resolve()
    environment = {**os.environ, "RUST_MIN_STACK": "67108864", "CINCH_TEST_API_EMIT_INTERFACES": "0"}
    overrides = {}
    for entry in args.source_override:
        name, separator, value = entry.partition("=")
        if not separator or name not in MODULES or name in overrides:
            parser.error("source overrides must name each selected module at most once as MODULE=PATH")
        overrides[name] = Path(value).resolve()
    modules = {name: overrides.get(name, ROOT / "seed/prelude" / (name.replace(".", "/") + ".cinch")).read_bytes() for name in MODULES}
    report = {"format": 1, "frontend": args.frontend, "compiler_sha256": digest(compiler), "runtime_sha256": digest(runtime),
              "runner_sha256": digest(Path(__file__)), "cpu_affinity": [args.cpu], "opt_level": args.opt_level,
              "sources": {name: hashlib.sha256(data).hexdigest() for name, data in modules.items()}, "suites": [], "status": "running"}
    report["source_overrides"] = {name: str(path) for name, path in overrides.items()}
    clang = Path(environment.get("CINCH_CLANG", shutil.which("clang") or "clang")).resolve()
    if clang.is_file():
        report["linker"] = {"path": str(clang), "sha256": digest(clang)}
    report["disk_budget_mib"] = {"minimum_free": args.minimum_free_mib, "minimum_reserve": args.minimum_reserve_mib, "maximum_unit": args.maximum_unit_mib}
    provider = None
    if args.frontend == "native":
        provider = Path(json.loads(args.objects.read_text())["object"])
        report.update(raw_compiler_sha256=digest(compiler.parent / "cinch"), interfaces_sha256=digest(args.interfaces),
                      objects_sha256=digest(args.objects), provider_object_sha256=digest(provider))
    provenance = compiler.parent / "provenance.json"
    if provenance.exists():
        preserved = json.loads(provenance.read_text())
        assert preserved["compiler_sha256"] == digest(compiler)
        assert preserved["runtime_sha256"] == digest(runtime)
        report["frozen_provenance"] = preserved
    jobs = {}
    if "source" in args.modes:
        jobs["source"] = "module Main where\n  import Data.Mutable.Map\n  import Data.Mutable.Map.Optional\n  import Data.Mutable.Map.Float\n  main :: Int\n  main = 0\n"
    if "region" in args.modes:
        jobs["region"] = (ROOT / "seed/tests/fixtures/maps-region-pair.cinch").read_text()
        jobs["region-source"] = (ROOT / "seed/tests/fixtures/maps-region-source.cinch").read_text()
    if "negative" in args.modes:
        jobs["negative-pure-read"] = (ROOT / "seed/tests/fixtures/mutable-map-negative-pure-read.cinch").read_text()
    if "control" in args.modes:
        jobs["control"] = (ROOT / "seed/tests/fixtures/mutable-map-control.cinch").read_text()
    if "edge" in args.modes:
        jobs["edge"] = edge_source()
    if "effect" in args.modes:
        jobs["effect"] = (ROOT / "seed/tests/fixtures/mutable-map-effect-control.cinch").read_text()
    if "trace" in args.modes:
        jobs.update({f"trace-{index}": source(index) for index in args.cases})
    if "float" in args.modes:
        jobs.update({f"float-{width}": float_source(width) for width in args.float_widths})
    args.artifacts.mkdir(parents=True, exist_ok=True)
    args.output.parent.mkdir(parents=True, exist_ok=True)
    (args.artifacts / "runner.py").write_bytes(Path(__file__).read_bytes())
    try:
        for label, consumer in jobs.items():
            free_before = shutil.disk_usage(args.artifacts).free
            assert free_before >= args.minimum_free_mib * 1024 * 1024, (label, "disk start budget", free_before)
            directory = (args.artifacts / label).resolve()
            directory.mkdir(parents=True, exist_ok=True)
            for name, data in modules.items():
                (directory / (name + ".cinch")).write_bytes(data)
            entry = directory / "Main.cinch"
            entry.write_text(consumer)
            executable, llvm = directory / "consumer", directory / "consumer.ll"
            suite = {"name": label, "consumer_sha256": digest(entry), "runs": [], "free_bytes_before": free_before}
            report["suites"].append(suite)
            check_only = label in ("source", "region", "region-source", "negative-pure-read")
            if args.frontend == "rust":
                command = [str(compiler), "check" if check_only else "build", str(entry)]
                if not check_only:
                    command += ["-o", str(executable), "--opt-level", args.opt_level, "--runtime-archive", str(runtime)]
            else:
                bundle = directory / "Bundle.cinch"
                bundle.write_text("\n\n".join(data.decode() for data in modules.values()) + "\n\n" + consumer)
                command = ["python3", str(compiler), "--check" if check_only else "--emit-ir", "--interfaces", str(args.interfaces.resolve()), str(bundle), str(llvm)]
                if not check_only:
                    command.insert(2, "--rc-debug")
            suite["compile"] = execute(command, directory, environment, host=True)
            (directory / "build.log").write_text(suite["compile"]["stdout"] + suite["compile"]["stderr"])
            if label == "negative-pure-read":
                diagnostic_file = llvm.with_suffix(".ll.diagnostics")
                diagnostics = suite["compile"]["stderr"] + (diagnostic_file.read_text() if diagnostic_file.exists() else "")
                suite["diagnostics"] = diagnostics
                assert suite["compile"]["exit"] != 0 and "effect" in diagnostics.lower(), (label, suite)
                assert not executable.exists() and not llvm.exists(), "a refused consumer must not produce executable code"
                suite.update(status="passed", expected_refusal=True)
                args.output.write_text(json.dumps(report, indent=2) + "\n")
                print(f"{args.frontend} {label}: required effect refusal passed", flush=True)
                continue
            assert suite["compile"]["exit"] == 0, (label, suite["compile"], llvm.with_suffix(".ll.diagnostics").read_text() if llvm.with_suffix(".ll.diagnostics").exists() else "")
            if not check_only:
                if args.frontend == "native":
                    command = [environment.get("CINCH_CLANG", "clang"), "-O" + args.opt_level, str(llvm), str(provider), str(runtime), "-o", str(executable), "-ldl", "-lpthread", "-lm"]
                    suite["link"] = execute(command, directory, environment)
                    assert suite["link"]["exit"] == 0, (label, suite["link"])
                suite.update(executable_sha256=digest(executable), llvm_sha256=digest(llvm))
                for poisoned in (False, True):
                    runtime_environment = {**environment, "CINCH_HEAP_LIMIT": "256M" if poisoned else "32M", "CINCH_STACK_SIZE": "8M",
                                           "CINCH_RC_DEBUG": "1" if poisoned else "0", "CINCH_LEAK_CHECK": "1", "CINCH_ALLOC_STATS": "1"}
                    run = execute([str(executable)], directory, runtime_environment)
                    run.update(poisoned_free=poisoned, heap=runtime_environment["CINCH_HEAP_LIMIT"], stack="8M")
                    suite["runs"].append(run)
                    (directory / ("poisoned.log" if poisoned else "normal.log")).write_text(run["stdout"] + run["stderr"])
                    assert run["exit"] == 0, (label, poisoned, run)
                    assert "cinch-leak: live=0 immortal=0 leaked=0" in run["stderr"], (label, poisoned, run)
            suite["status"] = "passed"
            suite["output_bytes"] = sum(path.stat().st_size for path in directory.rglob("*") if path.is_file())
            suite["free_bytes_after"] = shutil.disk_usage(directory).free
            assert not args.maximum_unit_mib or suite["output_bytes"] <= args.maximum_unit_mib * 1024 * 1024, (label, "unit output budget", suite["output_bytes"])
            assert suite["free_bytes_after"] >= args.minimum_reserve_mib * 1024 * 1024, (label, "disk reserve", suite["free_bytes_after"])
            args.output.write_text(json.dumps(report, indent=2) + "\n")
            print(f"{args.frontend} {label}: {'source check' if check_only else 'normal/poison zero-leak runs'} passed", flush=True)
        report["status"] = "passed"
    except BaseException as error:
        report.update(status="failed", failure=str(error))
        raise
    finally:
        args.output.write_text(json.dumps(report, indent=2) + "\n")


if __name__ == "__main__":
    main()
