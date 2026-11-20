#!/usr/bin/env python3
"""Run independent scalar observations against one explicit Native frontend and base artifact."""
from __future__ import annotations

import argparse
import json
import os
from pathlib import Path
import subprocess

import bootstrap
import bootstrap_objects
from generate_scalar_intrinsic_vectors import consumer


def main() -> None:
    """Retain exact source, compiler, object and runtime identities for every bounded consumer."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--compiler", type=Path, required=True)
    parser.add_argument("--library-artifact", type=Path, required=True)
    parser.add_argument("--artifacts", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--opt-level", type=int, choices=[0, 2], required=True)
    parser.add_argument("--first", type=int, default=0)
    parser.add_argument("--limit", type=int)
    parser.add_argument("--batch-size", type=int, default=64)
    args = parser.parse_args()
    args.compiler = args.compiler.resolve()
    args.artifacts = args.artifacts.resolve()
    args.artifacts.mkdir(parents=True, exist_ok=False)
    provider = bootstrap_objects.artifact_provider(bootstrap, args.library_artifact)
    manifest, objects = args.artifacts / "interfaces.json", args.artifacts / "objects.json"
    bootstrap.write_json(manifest, provider["interfaces"])
    bootstrap.write_json(objects, provider["configuration"])
    fixture = bootstrap.ROOT / "seed/tests/fixtures/native-scalar-intrinsic-vectors.json"
    rows = json.loads(fixture.read_text())[args.first:]
    if args.limit is not None:
        rows = rows[:args.limit]
    report = {"compiler_sha256": bootstrap.file_hash(args.compiler), "fixture_sha256": bootstrap.file_hash(fixture),
              "provider_receipt_sha256": provider["identity"]["receipt_sha256"],
              "object_sha256": bootstrap.file_hash(provider["object"]), "runtime_sha256": bootstrap.file_hash(provider["runtime"]),
              "opt_level": args.opt_level, "first": args.first, "cases": len(rows), "status": "running", "results": []}
    bootstrap.write_json(args.output, report)
    for first in range(0, len(rows), args.batch_size):
        selected = rows[first:first + args.batch_size]
        directory = args.artifacts / f"batch-{first:04}"
        directory.mkdir()
        source, ir, executable = directory / "Main.cinch", directory / "consumer.ll", directory / "consumer"
        source.write_text(consumer(selected))
        command = [str(args.compiler), "--emit-ir", "--rc-debug", "--interfaces", str(manifest),
                   "--objects", str(objects), str(source), str(ir)]
        compiled = subprocess.run(command, env=bootstrap.environment(), capture_output=True, text=True)
        (directory / "compile.log").write_text(compiled.stdout + compiled.stderr)
        diagnostics = Path(str(ir) + ".diagnostics")
        if compiled.returncode:
            report.update(status="failed", failure={"batch": first, "exit": compiled.returncode,
                          "diagnostics": diagnostics.read_text() if diagnostics.exists() else compiled.stderr})
            bootstrap.write_json(args.output, report)
            raise bootstrap.BootstrapError("scalar consumer compilation failed: " + str(directory))
        link = [os.environ.get("CINCH_CLANG", "clang"), f"-O{args.opt_level}", str(ir), str(provider["object"]),
                str(provider["runtime"]), "-o", str(executable)]
        if bootstrap.HOST["system"] == "Linux":
            link.extend(["-ldl", "-lpthread", "-lm"])
        linked = subprocess.run(link, env=bootstrap.environment(), capture_output=True, text=True)
        (directory / "link.log").write_text(linked.stdout + linked.stderr)
        linked.check_returncode()
        runs = []
        for debug in [False, True]:
            result = subprocess.run([str(executable)], capture_output=True, text=True, timeout=180,
                env={**os.environ, "CINCH_STACK_SIZE": "8M", "CINCH_HEAP_LIMIT": "256M" if debug else "32M",
                     "CINCH_LEAK_CHECK": "1", "CINCH_RC_DEBUG": "1" if debug else "0"})
            runs.append({"debug": debug, "exit": result.returncode, "stderr": result.stderr})
            if result.returncode or "leaked=0" not in result.stderr:
                report.update(status="failed", failure={"batch": first, "runs": runs})
                bootstrap.write_json(args.output, report)
                raise bootstrap.BootstrapError("scalar observation failed: " + str(directory) + ": " + result.stderr)
        report["results"].append({"first": first + args.first, "cases": len(selected), "names": [row["name"] for row in selected],
                                  "source_sha256": bootstrap.file_hash(source), "llvm_sha256": bootstrap.file_hash(ir),
                                  "executable_sha256": bootstrap.file_hash(executable), "compile_command": command,
                                  "link_command": link, "runs": runs})
        bootstrap.write_json(args.output, report)
        print(f"scalar batch {first + args.first}: {len(selected)} cases passed both ownership modes", flush=True)
    report["status"] = "passed"
    bootstrap.write_json(args.output, report)


if __name__ == "__main__":
    main()
