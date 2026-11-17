#!/usr/bin/env python3
"""Check one bounded archive component with explicit copied dependencies and retained failures."""
from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import resource
import shutil
import subprocess
import time


HERE = Path(__file__).resolve().parent
COMPILER_SHA = "ffb5d39c42d0446a912b6027ebbe88bd825f5c6d91065aeb8a2048526c4999b9"


def digest(path: Path) -> str:
    """Identify the actual byte inputs rather than inferring a compiler epoch from paths."""
    return hashlib.sha256(path.read_bytes()).hexdigest()


def stack_limit() -> None:
    """Give the source checker its usual host stack independently of runtime test profiles."""
    soft, hard = resource.getrlimit(resource.RLIMIT_STACK)
    resource.setrlimit(resource.RLIMIT_STACK, (67108864 if hard == resource.RLIM_INFINITY else min(67108864, hard), hard))


def closure(prelude: Path, entry: str) -> dict[str, Path]:
    """Resolve and copy each imported source explicitly; no missing dependency silently falls back."""
    found: dict[str, Path] = {}
    pending = [entry]
    while pending:
        name = pending.pop()
        if name in found:
            continue
        relative = Path(name.replace(".", "/") + ".cinch")
        path = HERE / "sources" / relative
        if not path.is_file():
            path = prelude / relative
        assert path.is_file(), f"Missing explicit dependency {name}: {path}"
        found[name] = path.resolve()
        pending.extend(re.findall(r"^\s*import\s+([A-Za-z][\w.]*)", path.read_text(), re.M))
    return found


def main() -> None:
    """Enforce source-only CPU, disk and timeout bounds and retain every attempt's actual bytes."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--compiler", type=Path, required=True)
    parser.add_argument("--prelude", type=Path, required=True)
    parser.add_argument("--artifacts", type=Path, required=True)
    parser.add_argument("--consumer", type=Path, help="Optional explicit source-only component consumer")
    parser.add_argument("--cpu", type=int, default=2)
    parser.add_argument("--dependencies", choices=("all-source", "frozen-builtins"), default="all-source",
                        help="Select explicit full source closure or producer-embedded core homes plus copied extension modules")
    args = parser.parse_args()
    compiler = args.compiler.resolve()
    assert digest(compiler) == COMPILER_SHA, "The granted compiler identity changed"
    assert shutil.disk_usage(HERE).free >= 350 * 1024**2, "Fresh source-unit guard failed"
    assert not args.artifacts.exists(), "Preserve prior attempts and choose a fresh directory"
    directory = args.artifacts.resolve()
    directory.mkdir(parents=True)
    os.sched_setaffinity(0, {args.cpu})
    modules = closure(args.prelude.resolve(), "Text.Tar.Block")
    sources = {}
    for name, path in modules.items():
        if args.dependencies == "frozen-builtins" and name not in ("Data.Tar", "Text.Tar.Block", "Data.Bytes.Word"):
            continue
        (directory / (name + ".cinch")).write_bytes(path.read_bytes())
        sources[name] = {"path": str(path), "sha256": digest(path)}
    entry = directory / "Text.Tar.Block.cinch"
    if args.consumer:
        entry = directory / "Main.cinch"
        entry.write_bytes(args.consumer.read_bytes())
    report = {"status": "started", "compiler": str(compiler), "compiler_sha256": digest(compiler),
              "sources": sources, "cpu": args.cpu, "runtime_executed": False,
              "scope": "metadata and physical block source only; stream framing excluded",
              "dependency_selection": args.dependencies,
              "embedded_builtin_authority": COMPILER_SHA if args.dependencies == "frozen-builtins" else None,
              "consumer_sha256": digest(entry) if args.consumer else None,
              "runner_sha256": digest(Path(__file__)),
              "free_before": shutil.disk_usage(HERE).free, "commands": []}
    commands = [[str(compiler), "check", str(entry)]]
    commands.extend([str(compiler), "doc", "extract", str(directory / (name + ".cinch")), "--out", str(directory / (name + ".docs.json"))]
                    for name in ("Data.Tar", "Text.Tar.Block"))
    try:
        for index, command in enumerate(commands):
            started = time.monotonic()
            try:
                result = subprocess.run(command, cwd=directory, capture_output=True, text=True,
                                        timeout=15, preexec_fn=stack_limit,
                                        env={**os.environ, "RUST_MIN_STACK": "67108864"})
                item = {"command": command, "exit": result.returncode, "stdout": result.stdout,
                        "stderr": result.stderr, "seconds": time.monotonic() - started}
            except subprocess.TimeoutExpired as error:
                item = {"command": command, "timeout": True, "seconds": time.monotonic() - started,
                        "stdout": (error.stdout or b"").decode() if isinstance(error.stdout, bytes) else error.stdout or "",
                        "stderr": (error.stderr or b"").decode() if isinstance(error.stderr, bytes) else error.stderr or ""}
            report["commands"].append(item)
            (directory / f"command-{index}.log").write_text(item["stdout"] + item["stderr"])
            if item.get("exit") != 0:
                report["status"] = "source-refused-or-timeout"
                break
        else:
            report["status"] = "source-and-doc-parser-passed"
        report["output_bytes"] = sum(p.stat().st_size for p in directory.rglob("*") if p.is_file())
        report["free_after"] = shutil.disk_usage(HERE).free
        assert report["output_bytes"] <= 5 * 1024**2, report["output_bytes"]
        assert report["free_after"] >= 250 * 1024**2, report["free_after"]
    finally:
        (directory / "report.json").write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps({key: report.get(key) for key in ("status", "output_bytes", "free_after")}))
    raise SystemExit(0 if report["status"] == "source-and-doc-parser-passed" else 1)


if __name__ == "__main__":
    main()
