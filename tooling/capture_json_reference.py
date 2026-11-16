#!/usr/bin/env python3
"""Capture the exact active JSON reference SDK files independently from a cumulative declaration audit."""
from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import subprocess


def sdk_output(executable: Path, arguments: list[str]) -> str:
    """Run one reference metadata query with bounded tool parallelism and telemetry disabled."""
    environment = {**os.environ, "GOMAXPROCS": "1", "GOTELEMETRY": "off"}
    return subprocess.check_output([str(executable), *arguments], text=True, env=environment).strip()


def main() -> None:
    """Record selected source hashes and default experiment evidence without inferring active build tags."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--go", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    arguments = parser.parse_args()
    executable = arguments.go.resolve()
    environment = json.loads(sdk_output(executable, ["env", "-json", "GOROOT", "GOEXPERIMENT", "GOOS", "GOARCH", "GOTOOLCHAIN"]))
    selected = json.loads(sdk_output(executable, ["list", "-json", "encoding/json"]))
    directory = Path(selected["Dir"])
    build_config = Path(environment["GOROOT"]) / "src/internal/buildcfg/exp.go"
    report = {
        "package": "encoding/json",
        "version": sdk_output(executable, ["version"]),
        "sdk_executable_sha256": hashlib.sha256(executable.read_bytes()).hexdigest(),
        "environment": environment,
        "selected_files": {name: hashlib.sha256((directory / name).read_bytes()).hexdigest() for name in selected["GoFiles"]},
        "build_configuration_sha256": hashlib.sha256(build_config.read_bytes()).hexdigest(),
        "default_json_experiment_evidence": [line.strip() for line in build_config.read_text().splitlines() if "JSONv2:" in line],
        "scope": "The cumulative declaration records include historical forms. These selected source files define the active reference behavior, including internally enabled default experiments.",
    }
    arguments.output.parent.mkdir(parents=True, exist_ok=True)
    arguments.output.write_text(json.dumps(report, indent=2) + "\n")


if __name__ == "__main__":
    main()
