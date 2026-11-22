#!/usr/bin/env python3
"""Verify frozen compiler sources with private build caches and serial gates."""
from __future__ import annotations

import argparse
import datetime
import fcntl
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys

import bootstrap

PARTS = ("compiler", "bootstrap", "tooling", "examples", "specifications", "design", "experiments")
TOP = ("README.md", "overview.md", "manifest.json", "SHA256SUMS")
SKIP = {"target", "__pycache__", "node_modules", ".DS_Store", ".generated"}


def inputs(root: Path) -> dict[str, str]:
    root = root.resolve()
    found = {}
    for part in PARTS:
        for parent, dirs, files in os.walk(root / part, followlinks=True):
            directory = Path(parent)
            if not directory.resolve().is_relative_to(root):
                raise ValueError(f"source directory link leaves the source tree: {directory}")
            dirs[:] = sorted(name for name in dirs if name not in SKIP
                             and not (directory == root / "design" and name == "brand")
                             and not (directory == root / "experiments/malleable-syntax/editor"
                                      and name.startswith("tree-sitter-")))
            ancestors = {directory.resolve()}
            for ancestor in directory.parents:
                ancestors.add(ancestor.resolve())
                if ancestor == root:
                    break
            for name in dirs:
                path = directory / name
                target = path.resolve()
                if not target.is_relative_to(root):
                    raise ValueError(f"source directory link leaves the source tree: {path}")
                if target in ancestors:
                    raise ValueError(f"source directory link forms a cycle: {path}")
            for name in sorted(files):
                if name not in SKIP:
                    path = directory / name
                    if not path.resolve().is_relative_to(root):
                        raise ValueError(f"source file link leaves the source tree: {path}")
                    found[str(path.relative_to(root))] = hashlib.sha256(path.read_bytes()).hexdigest()
    for name in TOP:
        if (root / name).is_file():
            found[name] = hashlib.sha256((root / name).read_bytes()).hexdigest()
    return found


def capture(source: Path, evidence: Path) -> tuple[Path, dict]:
    before = inputs(source)
    identity = hashlib.sha256(json.dumps(before, sort_keys=True).encode()).hexdigest()
    snapshot = evidence / "source"
    snapshot.mkdir(parents=True, exist_ok=False)
    for part in PARTS:
        (snapshot / part).mkdir()
    for relative in before:
        destination = snapshot / relative
        destination.parent.mkdir(parents=True, exist_ok=True)
        # Copy file contents rather than links to the developing checkout.
        shutil.copy2(source / relative, destination, follow_symlinks=True)
    if inputs(snapshot) != before or inputs(source) != before:
        raise ValueError("source inputs changed while capturing the snapshot")
    manifest = {"source": str(source), "path": str(snapshot), "sha256": identity, "files": before}
    bootstrap.write_json(evidence / "source-snapshot.json", manifest)
    return snapshot, manifest


def audit(snapshot: Path, manifest: dict) -> None:
    actual = inputs(snapshot)
    if actual != manifest["files"]:
        changed = sorted(name for name in actual.keys() | manifest["files"].keys()
                         if actual.get(name) != manifest["files"].get(name))
        raise ValueError("frozen inputs changed: " + ", ".join(changed))


def gates(snapshot: Path, evidence: Path) -> list[tuple[str, Path, list[str]]]:
    return [
        ("source-paths", snapshot / "compiler", ["cargo", "test", "--target-dir", str(evidence / "cargo/compiler"), "--test", "bootstrap_source_paths", "--", "--include-ignored"]),
        ("compiler", snapshot / "compiler", ["cargo", "test", "--target-dir", str(evidence / "cargo/compiler"), "--no-fail-fast", "--", "--include-ignored"]),
        ("compiler-bin-build", snapshot / "compiler", ["cargo", "build", "--target-dir", str(evidence / "cargo/compiler"), "--bin", "cinch"]),
        ("malleable-syntax", snapshot, [sys.executable, "tooling/verify_malleable.py", "--source", str(snapshot), "--evidence", str(evidence / "malleable-syntax"), "--compiler", str(evidence / "cargo/compiler/debug/cinch")]),
        ("runtime", snapshot / "compiler/runtime", ["cargo", "test", "--target-dir", str(evidence / "cargo/runtime"), "--no-fail-fast", "--", "--include-ignored"]),
        ("tooling", snapshot, [sys.executable, "-m", "unittest", "discover", "-s", "tooling/tests", "-v"]),
        ("stage0-smoke", snapshot, [sys.executable, "tooling/bootstrap.py", "smoke", "--stage", "0"]),
        ("stage1-smoke", snapshot, [sys.executable, "tooling/bootstrap.py", "smoke", "--stage", "1"]),
        ("self-hosting", snapshot, [sys.executable, "tooling/bootstrap.py", "check"]),
    ]


def now() -> str:
    return datetime.datetime.now(datetime.timezone.utc).isoformat()


def verify(source: Path, evidence: Path) -> int:
    if evidence.exists() and any(evidence.iterdir()):
        raise ValueError("evidence already exists; choose a new directory")
    for part in PARTS:
        if evidence == source / part or source / part in evidence.parents:
            raise ValueError("evidence must be outside the source input directories")
    evidence.mkdir(parents=True, exist_ok=True)
    snapshot, manifest = capture(source, evidence)
    env = bootstrap.environment()
    env["GIT_DIR"] = subprocess.check_output(
        ["git", "-c", "core.fsmonitor=false", "rev-parse", "--absolute-git-dir"],
        cwd=source, text=True).strip()
    # Never inherit a Cargo cache shared with the live checkout. Embedded
    # env!(CARGO_MANIFEST_DIR) paths survive Cargo's content reuse otherwise.
    env.pop("CARGO_TARGET_DIR", None)
    results = []
    for name, cwd, command in gates(snapshot, evidence):
        audit(snapshot, manifest)
        log = evidence / f"{name}.log"
        row = {"name": name, "command": command, "log": str(log), "status": "running", "started": now()}
        results.append(row)
        bootstrap.write_json(evidence / "results.json", results)
        print(f"{name}: started; log {log}", flush=True)
        gate_env = env.copy()
        gate_env["CINCH_TEST_SOURCE_ROOT"] = str(cwd)
        with log.open("wb") as output:
            code = subprocess.run(command, cwd=cwd, env=gate_env,
                                  stdout=output, stderr=subprocess.STDOUT).returncode
        row.update(exit_code=code, status="passed" if code == 0 else "failed", finished=now())
        bootstrap.write_json(evidence / "results.json", results)
        print(f"{name}: {row['status']} (exit {code})", flush=True)
        if name == "source-paths" and code != 0:
            # Do not spend a full run on tests whose input paths are wrong.
            audit(snapshot, manifest)
            return 1
    audit(snapshot, manifest)
    bootstrap.write_json(evidence / "source-audit.json", {"status": "unchanged", "sha256": manifest["sha256"], "files": len(manifest["files"]), "finished": now()})
    return 0 if all(row["status"] == "passed" for row in results) else 1


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--source", type=Path, default=bootstrap.ROOT)
    parser.add_argument("--evidence", type=Path)
    args = parser.parse_args()
    source = args.source.resolve()
    evidence = (args.evidence or source / "build/verification" / datetime.datetime.now().strftime("%Y%m%d-%H%M%S-%f")).resolve()
    priority = os.getpriority(os.PRIO_PROCESS, 0)
    if priority < 10:
        os.setpriority(os.PRIO_PROCESS, 0, 10)
    (source / "build").mkdir(exist_ok=True)
    with (source / "build/verification.lock").open("a") as lock:
        fcntl.flock(lock, fcntl.LOCK_EX)
        return verify(source, evidence)


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except (ValueError, OSError, subprocess.CalledProcessError, bootstrap.BootstrapError) as error:
        print(f"verification: {error}", file=sys.stderr)
        raise SystemExit(2)
