#!/usr/bin/env python3
"""Observe native producer-row inference independently of map source and runtime code."""
from __future__ import annotations

import argparse
import json
import os
from pathlib import Path

from validate_maps import ROOT, digest, execute


def main() -> None:
    """Check immutable tiny sources with an explicit native compiler/interface tuple and no linking."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--compiler", type=Path, required=True)
    parser.add_argument("--interfaces", type=Path, required=True)
    parser.add_argument("--cpu", type=int, required=True)
    parser.add_argument("--artifacts", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    if args.output.exists():
        parser.error("retain previous observations by choosing a fresh report path")
    os.sched_setaffinity(0, {args.cpu})
    args.artifacts.mkdir(parents=True, exist_ok=True)
    args.output.parent.mkdir(parents=True, exist_ok=True)
    compiler, interfaces = args.compiler.resolve(), args.interfaces.resolve()
    report = {"kind": "source-only-native-producer-row-observations", "compiler_sha256": digest(compiler),
              "interfaces_sha256": digest(interfaces), "runner_sha256": digest(Path(__file__)),
              "runtime_consumer_executed": False, "linked": False, "cpu_affinity": [args.cpu],
              "cases": [], "status": "running"}
    variable_module = ROOT / "compiler/prelude/Data/Mutable/Var.cinch"
    report["support_source_sha256"] = {"Data.Mutable.Var": digest(variable_module)}
    environment = {**os.environ, "CINCH_LEAK_CHECK": "1", "CINCH_RC_DEBUG": "0"}
    try:
        for name, fixture in (
            ("generic-producer", "maps-native-source-row-probe.cinch"),
            ("explicit-client-row", "maps-native-source-row-closed-probe.cinch"),
        ):
            directory = (args.artifacts / name).resolve()
            directory.mkdir(parents=True, exist_ok=True)
            source = directory / "Main.cinch"
            source.write_bytes((ROOT / "compiler/tests/fixtures" / fixture).read_bytes())
            bundle = directory / "Bundle.cinch"
            bundle.write_bytes(variable_module.read_bytes() + b"\n\n" + source.read_bytes())
            prefix = directory / "checked"
            result = execute([str(compiler), "--check", "--interfaces", str(interfaces), str(bundle), str(prefix)], directory, environment, host=True)
            diagnostics = prefix.with_suffix(".diagnostics")
            record = {"case": name, "source_sha256": digest(source), "bundle_sha256": digest(bundle), "command_result": result,
                      "diagnostics": diagnostics.read_text() if diagnostics.exists() else "",
                      "admitted": result["exit"] == 0}
            report["cases"].append(record)
            (directory / "process.log").write_text(result["stdout"] + result["stderr"])
            assert "cinch-leak: live=0 immortal=0 leaked=0" in result["stderr"], record
            # The native CLI creates an empty requested output even on a refused check.
            assert not prefix.with_suffix(".ll").exists(), "check-only must not emit LLVM code"
            if prefix.exists():
                record.update(check_output_bytes=prefix.stat().st_size, check_output_sha256=digest(prefix))
            args.output.write_text(json.dumps(report, indent=2) + "\n")
            print(f"{name}: {'admitted' if record['admitted'] else 'refused'} (exit {result['exit']})", flush=True)
        report["status"] = "observed"
    except BaseException as error:
        report.update(status="failed", failure=str(error))
        raise
    finally:
        args.output.write_text(json.dumps(report, indent=2) + "\n")


if __name__ == "__main__":
    main()
