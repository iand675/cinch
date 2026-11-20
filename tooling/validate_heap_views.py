#!/usr/bin/env python3
"""Compile independent heap traces with explicit frontends and bounded zero-leak execution."""
from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess

from generate_heap_view_cases import consumer

ROOT = Path(__file__).resolve().parents[1]
MODULES = {
    "Text.Hex": ROOT / "seed/prelude/Text/Hex.cinch",
    "Data.Bytes.Word": ROOT / "seed/prelude/Data/Bytes/Word.cinch",
    "Data.Mutable.Vec": ROOT / "seed/prelude/Data/Mutable/Vec.cinch",
    "Data.Mutable.Var": ROOT / "seed/prelude/Data/Mutable/Var.cinch",
    "Data.Mutable.Heap": ROOT / "seed/prelude/Data/Mutable/Heap.cinch",
    "Data.Mutable.HeapView": ROOT / "seed/prelude/Data/Mutable/HeapView.cinch",
}


def digest(path: Path) -> str:
    """Identify exact source, compiler or emitted bytes rather than trusting a filename."""
    return hashlib.sha256(path.read_bytes()).hexdigest()


def execute(command: list[str], cwd: Path, env: dict[str, str] | None = None) -> subprocess.CompletedProcess[str]:
    """Run one bounded build or consumer, preserving diagnostics on any failure."""
    result = subprocess.run(command, cwd=cwd, env=env, capture_output=True, text=True, timeout=900)
    if result.returncode:
        raise RuntimeError(f"command failed ({result.returncode}): {command}\nstdout:\n{result.stdout}\nstderr:\n{result.stderr}")
    return result


def main() -> None:
    """Validate selected independent heap consumers using an explicitly selected frozen frontend."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--frontend", choices=["rust", "native"], required=True)
    parser.add_argument("--compiler", type=Path, required=True)
    parser.add_argument("--interfaces", type=Path)
    parser.add_argument("--artifacts", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--cases", nargs="+", type=int, default=list(range(16)))
    parser.add_argument("--modes", nargs="+", choices=["trace", "owned", "control"], default=["trace", "owned"])
    parser.add_argument("--cpu", type=int, default=0)
    args = parser.parse_args()
    if args.cpu not in os.sched_getaffinity(0):
        parser.error("--cpu must be allowed by the current execution environment")
    os.sched_setaffinity(0, {args.cpu})
    if args.frontend == "native" and not args.interfaces:
        parser.error("the native wrapper requires its frozen --interfaces manifest")
    compiler = args.compiler.resolve()
    frozen_modules = {name: path.read_bytes() for name, path in MODULES.items()}
    frozen_consumers = {f"{mode}-{index}": consumer(index, mode).encode() for mode in args.modes for index in args.cases}
    report = {
        "description": "Independent generic and owned heap consumers; normal reclamation and separate poisoned-free validation",
        "frontend": args.frontend, "runner_sha256": digest(Path(__file__)),
        "compiler_sha256": digest(compiler),
        "sources": {name: hashlib.sha256(contents).hexdigest() for name, contents in frozen_modules.items()},
        "fixture_sha256": digest(ROOT / "seed/tests/fixtures/heap-view-vectors.json"),
        "cpu_affinity": [args.cpu], "suites": [],
    }
    if args.frontend == "native":
        report["native_compiler_sha256"] = digest(compiler.parent / "cinch")
        report["interfaces_sha256"] = digest(args.interfaces.resolve())
    args.output.parent.mkdir(parents=True, exist_ok=True)
    for suite in frozen_consumers:
        directory = args.artifacts.resolve() / suite
        directory.mkdir(parents=True, exist_ok=True)
        for name, contents in frozen_modules.items():
            (directory / f"{name}.cinch").write_bytes(contents)
        (directory / "Main.cinch").write_bytes(frozen_consumers[suite])
        output = directory / "consumer"
        if args.frontend == "rust":
            command = [str(compiler), "build", str(directory / "Main.cinch"), "-o", str(output), "--opt-level", "2", "--rc-debug"]
        else:
            bundle = directory / "Bundle.cinch"
            bundle.write_text("\n\n".join(contents.decode() for contents in frozen_modules.values()) + "\n\n" + frozen_consumers[suite].decode())
            command = ["python3", str(compiler), "--rc-debug", "--compile-only", "--interfaces", str(args.interfaces.resolve()), str(bundle), str(output.with_suffix(".ll")), str(output), str(directory / "link-report.txt")]
        try:
            built = execute(command, directory)
        except RuntimeError as error:
            diagnostics = output.with_suffix(".ll.diagnostics")
            if diagnostics.exists():
                raise RuntimeError(f"{error}\ncompiler diagnostics:\n{diagnostics.read_text()}") from error
            raise
        (directory / "build.log").write_text(built.stdout + built.stderr)
        item = {"suite": suite, "consumer_source_sha256": hashlib.sha256(frozen_consumers[suite]).hexdigest(), "llvm_sha256": digest(output.with_suffix(".ll")), "runs": []}
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
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(report, indent=2) + "\n")


if __name__ == "__main__":
    main()
