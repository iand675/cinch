#!/usr/bin/env python3
"""Freeze a shared-stream consumer and verify bounded real copy work with both frontends."""
from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import resource
import subprocess
import time

ROOT = Path(__file__).resolve().parents[1]


def digest(path: Path) -> str:
    """Identify exact source, compiler, runtime and instrumented linker inputs."""
    return hashlib.sha256(path.read_bytes()).hexdigest()


def run(command: list[str], cwd: Path, environment: dict[str, str] | None = None, host_stack: bool = False) -> subprocess.CompletedProcess[str]:
    """Run one coordinated build or consumer without altering its runtime memory limits."""
    def prepare_host() -> None:
        """Give only the compiler host the established main-stack allowance."""
        _, hard = resource.getrlimit(resource.RLIMIT_STACK)
        resource.setrlimit(resource.RLIMIT_STACK, (64 * 1024 * 1024, hard))
    return subprocess.run(command, cwd=cwd, env=environment, capture_output=True, text=True,
                          preexec_fn=prepare_host if host_stack else None, timeout=180)


def checked(command: list[str], cwd: Path, environment: dict[str, str] | None = None, host_stack: bool = False) -> subprocess.CompletedProcess[str]:
    """Preserve complete diagnostics when a frontend or linker refuses the frozen source."""
    result = run(command, cwd, environment, host_stack)
    if result.returncode:
        raise RuntimeError(f"command failed ({result.returncode}): {command}\n{result.stdout}\n{result.stderr}")
    return result


def main() -> None:
    """Check actual bulk-copy calls, offer telemetry, continuation epochs and zero-leak memory modes."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--frontend", choices=["rust", "native"], required=True)
    parser.add_argument("--compiler", type=Path, required=True)
    parser.add_argument("--interfaces", type=Path)
    parser.add_argument("--runtime-archive", type=Path, required=True)
    parser.add_argument("--source", type=Path, default=ROOT / "compiler/prelude/Effect/Stream.cinch")
    parser.add_argument("--artifacts", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--opt-level", type=int, choices=[0, 2], default=2)
    parser.add_argument("--cpu", type=int, default=2)
    parser.add_argument("--expect-failure", action="store_true")
    arguments = parser.parse_args()
    os.sched_setaffinity(0, {arguments.cpu})
    compiler = arguments.compiler.resolve()
    runtime = arguments.runtime_archive.resolve()
    artifacts = arguments.artifacts.resolve()
    artifacts.mkdir(parents=True, exist_ok=True)
    sources = {
        "Data.Bytes.Word.cinch": ROOT / "compiler/prelude/Data/Bytes/Word.cinch",
        "Effect.Stream.cinch": arguments.source.resolve(),
        "Main.cinch": ROOT / "compiler/tests/fixtures/stream-write-all-controls.cinch",
        "counter.c": ROOT / "tooling/stream_copy_counter.c",
        "validator.py": Path(__file__),
    }
    for name, path in sources.items():
        (artifacts / name).write_bytes(path.read_bytes())
    report = {
        "description": "Independent sink schedules, retained prior offers, exact prefix/error/continuation checks and real runtime bulk-copy counting",
        "frontend": arguments.frontend, "opt_level": arguments.opt_level,
        "compiler_sha256": digest(compiler), "runtime_archive_sha256": digest(runtime),
        "sources": {name: digest(artifacts / name) for name in sources},
        "compiler_host_stack_bytes": 64 * 1024 * 1024,
        "cpu_affinity": [arguments.cpu], "expected_regression": arguments.expect_failure, "runs": [],
    }
    ir = artifacts / "consumer.ll"
    started = time.monotonic()
    objects: list[str] = []
    if arguments.frontend == "rust":
        command = [str(compiler), "build", str(artifacts / "Main.cinch"), "-o", str(artifacts / "consumer"), "--opt-level", str(arguments.opt_level), "--rc-debug"]
    else:
        if not arguments.interfaces:
            parser.error("Native requires its exact matched --interfaces manifest")
        bundle = artifacts / "Bundle.cinch"
        bundle.write_text("\n\n".join((artifacts / name).read_text() for name in ["Data.Bytes.Word.cinch", "Effect.Stream.cinch", "Main.cinch"]))
        command = ["python3", str(compiler), "--rc-debug", "--compile-only", "--interfaces", str(arguments.interfaces.resolve()), str(bundle), str(ir), str(artifacts / "wrapper-default"), str(artifacts / "link-report.txt")]
        manifest = compiler.parent / "objects.json"
        configuration = json.loads(manifest.read_text())
        objects = [configuration["object"]]
        report.update(native_compiler_sha256=digest(compiler.parent / "cinch"), interfaces_sha256=digest(arguments.interfaces.resolve()), objects_manifest_sha256=digest(manifest), provider_object_sha256=digest(Path(objects[0])))
    compiled = checked(command, artifacts, host_stack=True)
    (artifacts / "build.log").write_text(compiled.stdout + compiled.stderr)
    clang = os.environ.get("CINCH_CLANG", "clang")
    counter = artifacts / "counter.o"
    checked([clang, "-O2", "-c", str(artifacts / "counter.c"), "-o", str(counter)], artifacts)
    executable = artifacts / "consumer-counted"
    link = [clang, f"-O{arguments.opt_level}", str(ir), *objects, str(counter), str(runtime), "-Wl,--wrap=cinch_bytes_copy", "-ldl", "-lpthread", "-lm", "-o", str(executable)]
    linked = checked(link, artifacts)
    (artifacts / "counted-link.log").write_text(linked.stdout + linked.stderr)
    report.update(build_seconds=time.monotonic() - started, llvm_sha256=digest(ir), counter_object_sha256=digest(counter), executable_sha256=digest(executable), link_command=link)
    for poison in [False, True]:
        environment = {**os.environ, "CINCH_HEAP_LIMIT": "256M" if poison else "32M", "CINCH_STACK_SIZE": "8M", "CINCH_RC_DEBUG": "1" if poison else "0", "CINCH_LEAK_CHECK": "1", "CINCH_ALLOC_STATS": "1"}
        result = run([str(executable)], artifacts, environment)
        (artifacts / ("poisoned.log" if poison else "ordinary.log")).write_text(result.stdout + result.stderr)
        traces = [{key: int(value) for key, value in re.findall(r"([a-z]+)=(\d+)", row)} for row in result.stdout.splitlines() if row.startswith("stream-trace:")]
        copied = re.search(r"stream-copy: bytes=(\d+) calls=(\d+)", result.stderr)
        if not copied or len(traces) != 7:
            raise RuntimeError(f"missing independent copy/offer trace:\n{result.stdout}\n{result.stderr}")
        total_input = sum(row["size"] for row in traces)
        copy_bound = total_input * 5 + 16384
        passed = result.returncode == 0 and result.stdout.splitlines()[-1] == "0"
        copy_passed = int(copied[1]) <= copy_bound
        if not arguments.expect_failure and not (passed and copy_passed):
            raise RuntimeError(f"functional/copy bound failed: actual={copied[1]} bound={copy_bound}\n{result.stdout}\n{result.stderr}")
        if arguments.expect_failure and passed:
            raise RuntimeError("the preserved original unexpectedly passed all regression controls")
        if "live=0" not in result.stderr or "leaked=0" not in result.stderr:
            raise RuntimeError(f"missing zero-live/leak evidence:\n{result.stderr}")
        report["runs"].append({"poisoned": poison, "heap_limit": environment["CINCH_HEAP_LIMIT"], "stack_limit": "8M", "exit_code": result.returncode, "controls_passed": passed, "traces": traces, "actual_bulk_copy_bytes": int(copied[1]), "actual_bulk_copy_calls": int(copied[2]), "copy_bound_including_fixture": copy_bound, "copy_bound_passed": copy_passed, "stdout": result.stdout, "stderr": result.stderr})
    if digest(runtime) != report["runtime_archive_sha256"] or digest(compiler) != report["compiler_sha256"]:
        raise RuntimeError("immutable compiler or matched runtime changed during validation")
    arguments.output.parent.mkdir(parents=True, exist_ok=True)
    arguments.output.write_text(json.dumps(report, indent=2) + "\n")
    print(f"{arguments.frontend} O{arguments.opt_level}: normal and poisoned controls complete; copied bytes bounded={all(row['copy_bound_passed'] for row in report['runs'])}")


if __name__ == "__main__":
    main()
