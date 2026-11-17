#!/usr/bin/env python3
"""Qualify unchanged baseline and separate entry candidate one bounded frozen unit at a time."""
from __future__ import annotations

import argparse
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys

from archive_map_functional_elfs import ROOT, digest, save_index

COMPILER = "855df83617f2ea84e7f0e3cf90b9755685bc23226594d81cd43ccd6168071b4e"
RUNTIME = "c4b1097b4b8e26541400228bdbbdcba1376101014823aeb2f5af619fbede57ac"
SOURCES = {"baseline": "0fc095f0bdcbe26f9ba3f26e5db8cb76891698784b744bb76e2e420dc14eb7c6",
           "entry": "c839d9b0868aaddfc9914fe515eeb613ba23bd934cc07dc4fa58e44463c00078"}


def main() -> None:
    """Resume completed exact units, stop at the first refusal, and archive closed outputs before progressing."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--compiler", type=Path, required=True)
    parser.add_argument("--runtime", type=Path, required=True)
    parser.add_argument("--clang", type=Path, required=True)
    parser.add_argument("--cpu", type=int, required=True)
    parser.add_argument("--representations", nargs="+", choices=SOURCES, default=list(SOURCES))
    parser.add_argument("--maximum-units", type=int, default=1, help="Stop at a natural caller-selected handoff boundary")
    parser.add_argument("--retry-failed", action="store_true", help="Retry only after the retained semantic failure has been diagnosed")
    args = parser.parse_args()
    assert args.maximum_units > 0 and digest(args.compiler) == COMPILER and digest(args.runtime) == RUNTIME
    os.sched_setaffinity(0, {args.cpu})
    environment = {**os.environ, "CINCH_CLANG": str(args.clang.resolve()), "RUST_MIN_STACK": "67108864"}
    candidate = ROOT / "build/maps-entry-candidate/sources/Data.Mutable.Map.cinch"
    assert digest(ROOT / "compiler/prelude/Data/Mutable/Map.cinch") == SOURCES["baseline"]
    assert digest(candidate) == SOURCES["entry"]
    ledger_path = ROOT / "build/maps-855-qualification-progress.json"
    ledger = json.loads(ledger_path.read_text()) if ledger_path.exists() else {
        "kind": "partial-bounded-map-qualification", "compiler_sha256": COMPILER, "runtime_sha256": RUNTIME,
        "candidate_selected": False, "package_complete": False, "units": {},
    }
    assert ledger["compiler_sha256"] == COMPILER and ledger["runtime_sha256"] == RUNTIME
    jobs = [("edge", ["--modes", "edge"], True),
            ("region", ["--modes", "region"], False),
            ("negative", ["--modes", "negative"], False),
            ("float-32", ["--modes", "float", "--float-widths", "32"], True),
            ("float-64", ["--modes", "float", "--float-widths", "64"], True),
            ("effect", ["--modes", "effect"], True)]
    jobs += [(f"trace-{index}", ["--modes", "trace", "--cases", str(index)], True) for index in range(8)]
    completed = 0
    for label, options, runtime in jobs:
        for representation in args.representations:
            key = representation + ":" + label
            if ledger["units"].get(key, {}).get("status") == "passed":
                previous = ledger["units"][key]
                assert digest(ROOT / previous["report"]) == previous["report_sha256"], "completed report changed"
                continue
            if ledger["units"].get(key, {}).get("status") == "failed" and not args.retry_failed:
                raise RuntimeError(f"Retained failure at {key} needs diagnosis before an explicit retry")
            if shutil.disk_usage(ROOT).free < 250 * 1024 * 1024:
                ledger["pause"] = {"reason": "disk start below250MiB", "next_unit": key, "free_bytes": shutil.disk_usage(ROOT).free}
                save_index(ledger_path, ledger)
                print(f"Resource pause before {key}; no consumer compilation started", flush=True)
                return
            attempt = 1
            while (ROOT / f"build/maps-855-{representation}-{label}-a{attempt}-results.json").exists():
                attempt += 1
            directory = ROOT / f"build/maps-855-{representation}-{label}-a{attempt}"
            report_path = ROOT / (str(directory.relative_to(ROOT)) + "-results.json")
            command = [sys.executable, str(ROOT / "tooling/validate_maps.py"), "--frontend", "rust",
                       "--compiler", str(args.compiler.resolve()), "--runtime", str(args.runtime.resolve()),
                       "--cpu", str(args.cpu), "--minimum-free-mib", "250", "--minimum-reserve-mib", "200",
                       "--maximum-unit-mib", "20", "--artifacts", str(directory), "--output", str(report_path)] + options
            if representation == "entry":
                command += ["--source-override", "Data.Mutable.Map=" + str(candidate)]
            result = subprocess.run(command, cwd=ROOT, env=environment, timeout=180)
            unit = {"report": str(report_path.relative_to(ROOT)), "exit": result.returncode,
                    "source_sha256": SOURCES[representation], "status": "failed"}
            ledger["units"][key] = unit
            if result.returncode != 0:
                if report_path.exists():
                    unit["report_sha256"] = digest(report_path)
                save_index(ledger_path, ledger)
                raise RuntimeError(f"Stopped at {key}; exact failed input/report retained")
            report = json.loads(report_path.read_text())
            assert report["status"] == "passed" and report["sources"]["Data.Mutable.Map"] == SOURCES[representation]
            if runtime:
                subprocess.run([sys.executable, str(ROOT / "tooling/archive_map_runtime_unit.py"), "--cpu", str(args.cpu),
                                "--report", str(report_path)], cwd=ROOT, env=environment, check=True, timeout=30)
            unit.update(status="passed", report_sha256=digest(report_path))
            ledger.pop("pause", None)
            save_index(ledger_path, ledger)
            completed += 1
            if completed >= args.maximum_units:
                print(f"Natural boundary after {completed} completed units", flush=True)
                return


if __name__ == "__main__":
    main()
