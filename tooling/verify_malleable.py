#!/usr/bin/env python3
"""Run syntax experiments outside the frozen source tree, using its compiler."""
from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import sys

import bootstrap
import verify_bootstrap


def summary(output: str, code: int) -> dict:
    count = re.search(r"^Ran (\d+) tests? in ", output, re.MULTILINE)
    skipped = re.search(r"skipped=(\d+)", output)
    tests = int(count.group(1)) if count else 0
    skips = int(skipped.group(1)) if skipped else 0
    return {"status": "passed" if code == 0 and tests > 0 and skips == 0 else "failed",
            "tests": tests, "skipped": skips, "exit_code": code}


def copy_inputs(source: Path, workspace: Path) -> None:
    def ignore(directory, names):
        return [name for name in names if name in verify_bootstrap.SKIP
                or (Path(directory).name == "editor" and name.startswith("tree-sitter-"))]
    for name in ("experiments/malleable-syntax", "tooling/tree-sitter-cinch", "tooling/editors"):
        destination = workspace / name
        destination.parent.mkdir(parents=True, exist_ok=True)
        shutil.copytree(source / name, destination, ignore=ignore)


def verify(source: Path, evidence: Path, compiler: Path) -> int:
    source, evidence, compiler = source.resolve(), evidence.resolve(), compiler.resolve()
    if source == evidence or source in evidence.parents:
        raise ValueError("experiment evidence must be outside the source tree")
    if evidence.exists() and any(evidence.iterdir()):
        raise ValueError("experiment evidence already exists")
    if not compiler.is_file() or not os.access(compiler, os.X_OK):
        raise ValueError("build the frozen source compiler before running syntax experiments")
    before = verify_bootstrap.inputs(source)
    compiler_hash = bootstrap.file_hash(compiler)
    evidence.mkdir(parents=True, exist_ok=True)
    workspace = evidence / "work"
    workspace.mkdir()
    copy_inputs(source, workspace)
    suite = workspace / "experiments/malleable-syntax"
    env = bootstrap.environment()
    env.update(CINCH_EXPERIMENT_COMPILER=str(compiler), PYTHONDONTWRITEBYTECODE="1")
    command = [sys.executable, "-m", "unittest", "discover", "-s", str(suite / "tests"), "-v"]
    result = subprocess.run(command, cwd=workspace, env=env, stdout=subprocess.PIPE,
                            stderr=subprocess.STDOUT, text=True)
    (evidence / "tests.log").write_text(result.stdout)
    report = summary(result.stdout, result.returncode)
    unchanged = verify_bootstrap.inputs(source) == before and bootstrap.file_hash(compiler) == compiler_hash
    if not unchanged:
        report["status"] = "failed"
    experiment_inputs = {name: digest for name, digest in before.items()
                         if name.startswith("experiments/malleable-syntax/")}
    report.update(source_sha256=hashlib.sha256(json.dumps(before, sort_keys=True).encode()).hexdigest(),
                  compiler_sha256=compiler_hash, compiler=str(compiler), source_unchanged=unchanged,
                  experiment_inputs=experiment_inputs,
                  positive_programs=len(list((suite / "examples").glob("*.cinch"))),
                  negative_programs=len(list((suite / "examples/negative").glob("*.cinch"))),
                  syntax_libraries=len(list((suite / "syntax").glob("*.json"))),
                  command=command, bootstrap_achieved=False)
    bootstrap.write_json(evidence / "results.json", report)
    print(result.stdout, end="")
    print(f"syntax experiments: {report['status']}; {report['tests']} tests, {report['skipped']} skipped", flush=True)
    return 0 if report["status"] == "passed" else 1


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--source", required=True, type=Path)
    parser.add_argument("--evidence", required=True, type=Path)
    parser.add_argument("--compiler", required=True, type=Path)
    args = parser.parse_args()
    return verify(args.source, args.evidence, args.compiler)


if __name__ == "__main__":
    sys.exit(main())
