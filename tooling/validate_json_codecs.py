#!/usr/bin/env python3
"""Compile real JSON consumer fixtures with either frontend and verify bounded zero-leak execution."""
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
MODULES = {
    "Data.Array": ROOT / "compiler/prelude/Data/Array.cinch",
    "Data.Bytes.Word": ROOT / "compiler/prelude/Data/Bytes/Word.cinch",
    "Data.Unicode.Tables": ROOT / "compiler/prelude/Data/Unicode/Tables.cinch",
    "Data.Unicode.Kernel": ROOT / "compiler/prelude/Data/Unicode/Kernel.cinch",
    "Text.Json.Value": ROOT / "compiler/prelude/Text/Json/Value.cinch",
}



def digest(path: Path) -> str:
    """Identify exact source, compiler or emitted bytes rather than trusting a filename."""
    return hashlib.sha256(path.read_bytes()).hexdigest()


def execute(command: list[str], cwd: Path, env: dict[str, str] | None = None, host_stack_bytes: int | None = None) -> subprocess.CompletedProcess[str]:
    """Run one bounded build or consumer, preserving diagnostics on any failure."""
    def prepare_compiler_stack() -> None:
        """Apply the explicit compiler-host stack limit without changing consumer runtime limits."""
        _, hard = resource.getrlimit(resource.RLIMIT_STACK)
        resource.setrlimit(resource.RLIMIT_STACK, (host_stack_bytes, hard))

    result = subprocess.run(command, cwd=cwd, env=env, capture_output=True, text=True, timeout=900,
                            preexec_fn=prepare_compiler_stack if host_stack_bytes else None)
    if result.returncode:
        raise RuntimeError(f"command failed ({result.returncode}): {command}\nstdout:\n{result.stdout}\nstderr:\n{result.stderr}")
    return result


def main() -> None:
    """Validate selected independent consumers without rebuilding or sharing a mutable compiler."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--frontend", choices=["rust", "native"], required=True)
    parser.add_argument("--compiler", type=Path, required=True)
    parser.add_argument("--interfaces", type=Path)
    parser.add_argument("--runtime-receipt", type=Path)
    parser.add_argument("--artifacts", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--suites", nargs="+", default=["syntax", "quote", "number", "indent", "control"])
    parser.add_argument("--opt-level", type=int, choices=[0, 1, 2, 3], default=None)
    parser.add_argument("--cpu", type=int, default=2)
    parser.add_argument("--compiler-host-stack-kib", type=int, default=65536)
    args = parser.parse_args()
    if args.cpu not in os.sched_getaffinity(0):
        parser.error("--cpu must be allowed by the current execution environment")
    os.sched_setaffinity(0, {args.cpu})
    if args.frontend == "native" and not args.interfaces:
        parser.error("the native wrapper requires its frozen --interfaces manifest")
    if args.frontend == "native" and args.opt_level is not None:
        parser.error("--opt-level configures Rust only; the frozen Native functional wrapper links without an optimization flag")
    if args.frontend == "rust" and args.opt_level is None:
        args.opt_level = 2
    compiler = args.compiler.resolve()
    modules = dict(MODULES)
    if any(suite in {"typed", "codec-control", "float", "field", "integer", "binary", "binary-control", "map", "dynamic"} for suite in args.suites):
        modules["Text.Base64.Encoding"] = ROOT / "compiler/prelude/Text/Base64/Encoding.cinch"
        modules["Text.Json.Codec"] = ROOT / "compiler/prelude/Text/Json/Codec.cinch"
    frozen_modules = {name: path.read_bytes() for name, path in modules.items()}
    frozen_consumers = {suite: (ROOT / f"compiler/tests/fixtures/json-codec-{suite}.cinch").read_bytes() for suite in args.suites}
    reference_paths = {suite: ROOT / (f"compiler/tests/fixtures/json-codec-{suite}-vectors.json" if suite in {"float", "field", "integer", "binary"} else "compiler/tests/fixtures/json-codec-vectors.json") for suite in args.suites}
    report = {
        "description": "Independent configurable JSON consumers; normal reclamation and separate poisoned-free validation",
        "frontend": args.frontend, "opt_level": args.opt_level if args.frontend == "rust" else 0,
        "native_link_policy": None if args.frontend == "rust" else "frozen-wrapper clang default; no -O flag", "runner_sha256": digest(Path(__file__)),
        "compiler_sha256": digest(compiler),
        "sources": {name: hashlib.sha256(contents).hexdigest() for name, contents in frozen_modules.items()},
        "fixture_sha256": digest(ROOT / "compiler/tests/fixtures/json-codec-vectors.json"),
        "reference_fixtures": {suite: digest(path) for suite, path in reference_paths.items()},
        "cpu_affinity": [args.cpu], "compiler_host_stack_kib": args.compiler_host_stack_kib, "consumer_stack_limit": "8M", "suites": [],
    }
    if args.runtime_receipt:
        report["observed_runtime_archive_sha256"] = digest(args.runtime_receipt.resolve())
    if args.frontend == "native":
        report["native_compiler_sha256"] = digest(compiler.parent / "cinch")
        report["interfaces_sha256"] = digest(args.interfaces.resolve())
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.artifacts.mkdir(parents=True, exist_ok=True)
    (args.artifacts / "validator.py").write_bytes(Path(__file__).read_bytes())
    (args.artifacts / "reference-vectors.json").write_bytes((ROOT / "compiler/tests/fixtures/json-codec-vectors.json").read_bytes())
    for suite in args.suites:
        directory = args.artifacts.resolve() / suite
        directory.mkdir(parents=True, exist_ok=True)
        for name, contents in frozen_modules.items():
            (directory / f"{name}.cinch").write_bytes(contents)
        (directory / "Main.cinch").write_bytes(frozen_consumers[suite])
        (directory / "reference-vectors.json").write_bytes(reference_paths[suite].read_bytes())
        output = directory / "consumer"
        if args.frontend == "rust":
            command = [str(compiler), "build", str(directory / "Main.cinch"), "-o", str(output), "--opt-level", str(args.opt_level), "--rc-debug"]
        else:
            bundle = directory / "Bundle.cinch"
            bundle.write_text("\n\n".join(contents.decode() for contents in frozen_modules.values()) + "\n\n" + frozen_consumers[suite].decode())
            command = ["python3", str(compiler), "--rc-debug", "--compile-only", "--interfaces", str(args.interfaces.resolve()), str(bundle), str(output.with_suffix(".ll")), str(output), str(directory / "link-report.txt")]
        started = time.monotonic()
        try:
            built = execute(command, directory, host_stack_bytes=args.compiler_host_stack_kib * 1024)
        except RuntimeError as error:
            diagnostics = output.with_suffix(".ll.diagnostics")
            if diagnostics.exists():
                raise RuntimeError(f"{error}\ncompiler diagnostics:\n{diagnostics.read_text()}") from error
            raise
        (directory / "build.log").write_text(built.stdout + built.stderr)
        item = {"suite": suite, "consumer_source_sha256": hashlib.sha256(frozen_consumers[suite]).hexdigest(), "llvm_sha256": digest(output.with_suffix(".ll")), "build_seconds": time.monotonic() - started, "runs": []}
        for debug in [False, True]:
            environment = {**os.environ, "CINCH_HEAP_LIMIT": "256M" if debug else "32M", "CINCH_STACK_SIZE": "8M", "CINCH_LEAK_CHECK": "1", "CINCH_RC_DEBUG": "1" if debug else "0", "CINCH_ALLOC_STATS": "1"}
            result = execute([str(output)], directory, environment)
            if "leaked=0" not in result.stderr:
                raise RuntimeError(f"{suite}: missing zero-leak observation\n{result.stderr}")
            stats_line = next(line for line in result.stderr.splitlines() if line.startswith("cinch-alloc:"))
            stats = {name: int(value) for name, value in re.findall(r"([a-z-]+)=(\d+)", stats_line)}
            if stats["live"] != 0:
                raise RuntimeError(f"{suite}: residual live allocation bytes: {stats}")
            item["runs"].append({"poisoned_free_validation": debug, "heap_limit": environment["CINCH_HEAP_LIMIT"], "stdout": result.stdout, "stats": stats, "leaked": 0})
            (directory / ("poisoned.log" if debug else "ordinary.log")).write_text(result.stdout + result.stderr)
        report["suites"].append(item)
        args.output.write_text(json.dumps(report, indent=2) + "\n")
        print(f"{args.frontend} {suite}: normal and poisoned runs passed with zero leaks", flush=True)
    if args.runtime_receipt and report["observed_runtime_archive_sha256"] != digest(args.runtime_receipt.resolve()):
        raise RuntimeError("observed runtime archive changed during validation")
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.artifacts.mkdir(parents=True, exist_ok=True)
    (args.artifacts / "validator.py").write_bytes(Path(__file__).read_bytes())
    (args.artifacts / "reference-vectors.json").write_bytes((ROOT / "compiler/tests/fixtures/json-codec-vectors.json").read_bytes())
    args.output.write_text(json.dumps(report, indent=2) + "\n")


if __name__ == "__main__":
    main()
