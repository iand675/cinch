#!/usr/bin/env python3
"""Build one pinned offline reference helper with monitored cache and temporary output bounds."""
from __future__ import annotations
import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import signal
import subprocess
import time

ROOT = Path(__file__).resolve().parent
GO = Path("/tmp/cinch-go-current/go/bin/go")
CACHE = Path("/tmp/cinch-go-surface-cache")


def size(path: Path) -> int:
    """Count actual retained file sizes without following protected external inputs."""
    return sum(p.stat().st_size for p in path.rglob("*") if p.is_file())


def digest(path: Path) -> str:
    """Record byte identities for the executed tool, source and complete observations."""
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main() -> None:
    """Enforce one worker, offline SDK selection and the explicit 30-MiB output allowance."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--artifacts", type=Path, required=True)
    args = parser.parse_args()
    PROJECT = args.artifacts.resolve()
    os.sched_setaffinity(0, {2})
    assert shutil.disk_usage(ROOT).free >= 350 * 1024**2
    assert not PROJECT.exists(), "Preserve every previous helper attempt"
    PROJECT.mkdir(parents=True)
    (PROJECT / "tmp").mkdir()
    source = PROJECT / "blocks.go"
    source.write_bytes((ROOT / "reference/blocks.go").read_bytes())
    before = size(CACHE)
    env = {**os.environ, "GOCACHE": str(CACHE), "GOTMPDIR": str(PROJECT / "tmp"),
           "GOTOOLCHAIN": "local", "GOMAXPROCS": "1", "GOFLAGS": "-p=1",
           "GOPROXY": "off", "GOSUMDB": "off", "GOTELEMETRY": "off"}
    report = {"status": "started", "go_sha256": digest(GO), "source_sha256": digest(source),
              "cpu": 2, "cache_before": before, "free_before": shutil.disk_usage(ROOT).free}
    try:
        version = subprocess.run([str(GO), "version"], capture_output=True, text=True, env=env, timeout=5)
        report["version"] = version.stdout.strip()
        assert version.returncode == 0 and " go1.27.4 " in version.stdout
        executable = PROJECT / "reference"
        command = [str(GO), "build", "-o", str(executable), str(source)]
        plan = subprocess.run([str(GO), "build", "-n", "-o", str(executable), str(source)],
                              cwd=PROJECT, env=env, capture_output=True, text=True, timeout=15)
        (PROJECT / "build-plan.log").write_text(plan.stdout + plan.stderr)
        assert plan.returncode == 0
        report["uncached_compiles"] = [line for line in (plan.stdout + plan.stderr).splitlines() if re.search(r"/compile(?:\s|$)", line)]
        started = time.monotonic()
        with (PROJECT / "build.log").open("w") as log:
            child = subprocess.Popen(command, cwd=PROJECT, env=env, stdout=log, stderr=log, start_new_session=True)
            try:
                while child.poll() is None:
                    growth = size(PROJECT) + max(0, size(CACHE) - before)
                    if growth > 29 * 1024**2 or shutil.disk_usage(ROOT).free < 250 * 1024**2 or time.monotonic() - started > 60:
                        raise RuntimeError("Bounded reference build resource/deadline guard")
                    time.sleep(.1)
            finally:
                if child.poll() is None:
                    os.killpg(child.pid, signal.SIGTERM)
                    child.wait()
        report["build"] = {"command": command, "exit": child.returncode, "seconds": time.monotonic() - started}
        assert child.returncode == 0
        report["executable_sha256"] = digest(executable)
        with (PROJECT / "vectors.json").open("w") as output:
            observed = subprocess.run([str(executable)], cwd=PROJECT, env=env, stdout=output,
                                      stderr=subprocess.PIPE, text=True, timeout=5)
        report["observations"] = {"exit": observed.returncode, "stderr": observed.stderr,
                                  "sha256": digest(PROJECT / "vectors.json")}
        assert observed.returncode == 0
        vectors = json.loads((PROJECT / "vectors.json").read_text())
        report["cases"] = len(vectors["Cases"])
        report["encoding_cases"] = len(vectors.get("Encodings", []))
        report["new_cache_bytes"] = max(0, size(CACHE) - before)
        report["output_bytes"] = size(PROJECT)
        assert report["new_cache_bytes"] + report["output_bytes"] <= 30 * 1024**2
        assert shutil.disk_usage(ROOT).free >= 250 * 1024**2
        report["status"] = "public-sdk-observations-recorded"
    finally:
        report["free_after"] = shutil.disk_usage(ROOT).free
        (PROJECT / "report.json").write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps({key: report[key] for key in ("status", "cases", "output_bytes", "new_cache_bytes", "free_after")}))


if __name__ == "__main__":
    main()
