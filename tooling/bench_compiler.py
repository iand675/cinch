#!/usr/bin/env python3
"""Measure compiler latency and optionally check it against a baseline.

Empty-store samples start with a fresh Cinch package store. Warm-store
samples reuse it. Neither measurement evicts the operating system's cache.
All linked samples produce a fresh executable and run it after timing.
"""
from __future__ import annotations

import argparse
import json
import os
from pathlib import Path
import resource
import shutil
import statistics
import subprocess
import sys
import tempfile
import time

import profile_build as profile

HELLO = """module Main where
  import Data.Path (path-from-string)
  import Effect.Fs (fs-write)
  main :: Unit ->{IO r} Int
  main unit = with (io = newIO ()) in
    case fs-write (path-from-string "/dev/stdout") (string-to-bytes "Hello, world!\\n") of
      Ok _ -> 0
      Err error -> 1
"""


def measure(command, directory, environment):
    before = resource.getrusage(resource.RUSAGE_CHILDREN)
    start = time.perf_counter()
    result = subprocess.run(command, cwd=directory, env=environment, capture_output=True)
    wall = time.perf_counter() - start
    after = resource.getrusage(resource.RUSAGE_CHILDREN)
    cpu = after.ru_utime + after.ru_stime - before.ru_utime - before.ru_stime
    if result.returncode:
        detail = (result.stdout + result.stderr).decode(errors="replace")
        raise RuntimeError(f"{command[0]} {command[1]} exited {result.returncode}\n{detail}")
    return {"wall_ms": wall * 1000, "cpu_ms": cpu * 1000}


def summarize(samples):
    return {
        metric: round(statistics.median(sample[metric] for sample in samples), 3)
        for metric in samples[0]
    }


def measure_hello(compiler, cli, runtime, directory, env, samples):
    source, output = directory / "Hello.cinch", directory / "hello"
    ir = output.with_suffix(".ll")
    source.write_text(HELLO)
    commands = profile.check_commands(compiler, cli, source, ir, directory / "hello.diag")
    timings = {}
    clang = env.get("CINCH_CLANG") or shutil.which("clang") or "clang"
    objects = []
    if cli == "seed":
        commands["linked"] = [compiler, "build", source, "--runtime-archive", runtime, "-o", output]
    else:
        config = json.loads((compiler.parent / "base-objects.json").read_bytes())
        objects.append(config["object"])
    system_libraries = ["-ldl", "-lpthread", "-lm"] if sys.platform.startswith("linux") else []
    link = [clang, ir, *objects, runtime, "-o", output, *system_libraries]

    for mode in ("check", "emit-ir", "linked"):
        empty, warm = [], []
        for index in range(samples):
            sample_env = {**env, "CINCH_STORE": str(directory / f"{mode}-{index}.store")}
            for batch in (empty, warm):
                output.unlink(missing_ok=True)
                ir.unlink(missing_ok=True)
                # A prior project output can short-circuit work independently
                # of the package-store state being measured.
                shutil.rmtree(directory / "build", ignore_errors=True)
                if mode != "linked" or cli == "seed":
                    timing = measure(commands[mode], directory, sample_env)
                else:
                    front = measure(commands["emit-ir"], directory, sample_env)
                    back = measure(link, directory, sample_env)
                    timing = {key: front[key] + back[key] for key in front}
                if mode == "linked":
                    if subprocess.check_output([output], cwd=directory, env=sample_env) != b"Hello, world!\n":
                        raise RuntimeError("compiled Hello World printed unexpected output")
                batch.append(timing)
        timings[mode] = {"empty_store": summarize(empty), "warm_store": summarize(warm)}
    obj = directory / "hello.o"
    for name, command in (
        ("clang_combined", link),
        ("clang_assemble", [clang, "-c", ir, "-o", obj]),
        ("clang_link_object", [clang, obj, *objects, runtime, "-o", output, *system_libraries]),
    ):
        timings[name] = summarize([measure(command, directory, env) for _ in range(samples)])
    return timings


def fixture_case(size):
    return (
        "module Main where\n  dispatch :: Sym -> Int\n  dispatch s = case s of\n"
        + "".join(f"    'symbol{i:05d} -> {i}\n" for i in range(size))
        + "    _ -> -1\n  main :: Int\n  main = dispatch 'symbol00000\n"
    )


def benchmark(args):
    compiler = args.compiler.resolve()
    cli = profile.compiler_cli(compiler)
    runtime = args.runtime
    if runtime is None and cli == "pipeline":
        objects = json.loads((compiler.parent / "base-objects.json").read_bytes())
        runtime = Path(objects["object"]).with_name("libcinchruntime.a")
    if "hello" in args.workloads and (runtime is None or not runtime.is_file()):
        raise ValueError("the Hello World workload needs the matching runtime archive")
    env = os.environ.copy()
    profile.SCRATCH.mkdir(parents=True, exist_ok=True)
    with tempfile.TemporaryDirectory(prefix="compiler-bench-", dir=profile.SCRATCH) as scratch:
        directory = Path(scratch)
        env["CINCH_STORE"] = str(directory / "store")
        timings = {}
        if "hello" in args.workloads:
            timings["hello"] = measure_hello(compiler, cli, runtime.resolve(), directory, env, args.samples)
        sources = {}
        if "graph" in args.workloads:
            sources["graph"] = (profile.workload("wide-graph", directory), "emit-ir")
        if "symbols" in args.workloads:
            for size in args.sizes:
                source = directory / f"Symbols{size}.cinch"
                source.write_text(fixture_case(size))
                sources[f"symbols_{size}"] = (source, "check")
        if "self" in args.workloads:
            if args.bundle is None:
                raise ValueError("the self workload needs --bundle")
            sources["self"] = (args.bundle.resolve(), "emit-ir")

        for name, (source, mode) in sources.items():
            if source.parent != directory:
                source = Path(shutil.copyfile(source, directory / f"{name}.cinch"))
            output, diag = directory / f"{name}.ll", directory / f"{name}.diag"
            command = profile.check_commands(compiler, cli, source, output, diag)[mode]
            measure(command, directory, env)
            timings[name] = summarize([measure(command, directory, env) for _ in range(args.samples)])
    sizes = sorted(set(args.sizes)) if "symbols" in args.workloads else []
    scaling = {}
    for small, large in zip(sizes, sizes[1:]):
        duration = timings[f"symbols_{large}"]["cpu_ms"] - timings[f"symbols_{small}"]["cpu_ms"]
        scaling[f"{small}_to_{large}"] = round(duration * 1000 / (large - small), 3)
    host = profile.host()
    if "hello" in args.workloads and env.get("CINCH_CLANG"):
        clang_version = subprocess.check_output([env["CINCH_CLANG"], "--version"], text=True)
        host["clang"] = clang_version.splitlines()[0]
    return {"schema": 1, "compiler": str(compiler), "cli": cli, "host": host,
            "samples": args.samples, "timings": timings, "symbol_scaling_us_per_arm": scaling}


def cpu_regressions(previous, current, tolerance):
    def compare(old, now, path):
        if "cpu_ms" in now:
            limit = old["cpu_ms"] * (1 + tolerance)
            if now["cpu_ms"] > limit:
                yield f"{path}: {now['cpu_ms']:.2f} ms CPU exceeds {limit:.2f} ms"
        else:
            for key in sorted(old.keys() | now.keys()):
                if key not in old or key not in now:
                    yield f"{path}.{key}: reports cover different workloads"
                else:
                    yield from compare(old[key], now[key], f"{path}.{key}")

    for key in ("schema", "cli", "host"):
        if previous[key] != current[key]:
            raise ValueError(f"baseline and current report differ in {key}")
    return list(compare(previous["timings"], current["timings"], "timings"))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--compiler", required=True, type=Path)
    parser.add_argument("--runtime", type=Path)
    parser.add_argument("--bundle", type=Path, help="bundled Cinch compiler source")
    parser.add_argument("--samples", type=int, default=3)
    parser.add_argument("--sizes", type=int, nargs="+", default=(128, 512, 2048))
    parser.add_argument("--workloads", nargs="+", choices=("hello", "graph", "symbols", "self"),
                        default=("hello", "graph", "symbols"))
    parser.add_argument("--baseline", type=Path, help="prior report from this host and CLI")
    parser.add_argument("--tolerance", type=float, default=0.15, help="allowed fractional CPU increase")
    args = parser.parse_args()
    if args.samples < 1 or min(args.sizes) < 1 or args.tolerance < 0:
        parser.error("samples and sizes must be positive, and tolerance must be nonnegative")
    try:
        report = benchmark(args)
        failures = cpu_regressions(json.loads(args.baseline.read_bytes()), report, args.tolerance) if args.baseline else []
    except (OSError, ValueError, RuntimeError, KeyError) as error:
        print(f"compiler benchmark: {error}", file=sys.stderr)
        return 2
    print(json.dumps(report, indent=2))
    for failure in failures:
        print(failure, file=sys.stderr)
    return int(bool(failures))


if __name__ == "__main__":
    raise SystemExit(main())
