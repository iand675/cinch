#!/usr/bin/env python3
"""Compile and execute the complete independent HTML corpus with a selected frozen frontend.

This lane-local validator avoids rebuilding the compiler. It uses the same
consumer template and fixtures as the Rust regression target, keeps all source
identities, and checks ordinary reclamation separately from poisoned frees.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess
import time

ROOT = Path(__file__).resolve().parents[1]
DEFAULT_HOME = Path("/workspace/dasl-stdlib-io/build")


def digest(path: Path) -> str:
    """Identify exact source, executable and fixture bytes."""
    return hashlib.sha256(path.read_bytes()).hexdigest()


def consumer() -> str:
    """Render independent expected bytes into a compact NUL-delimited runtime fixture stream."""
    fixture = json.loads((ROOT / "seed/tests/fixtures/html-vectors.json").read_text())
    fields = "".join(row["input"] + "\0" + row["escaped"] + "\0" + row["unescaped"] + "\0" for row in fixture["rows"])
    literal = json.dumps(fields).replace("\\u0000", "\\u{0}")
    checks = []
    for row in fixture["long"]:
        checks.append("long-case " + " ".join(str(row[key]) for key in ("size", "mode", "input_hash", "escaped_size", "escaped_hash", "unescaped_size", "unescaped_hash")))
    template = (ROOT / "seed/tests/fixtures/html-consumer.cinch.in").read_text()
    return template.replace("__ROWS__", literal).replace("__LONG__", ",\n      ".join(checks))


def source_closure(project: Path) -> list[Path]:
    """Copy non-embedded imports by declared module name beside the consumer."""
    manifest = json.loads((ROOT / "website/src/data/library-sources.json").read_text())
    embedded = {row["module"] for row in manifest["modules"] if row["embedded"]}
    pending = ["Text.Html", "Text.Hex", "Text.Fnv"]
    found = set()
    result = []
    while pending:
        name = pending.pop()
        if name in found or name in embedded:
            continue
        found.add(name)
        path = ROOT / "seed/prelude" / (name.replace(".", "/") + ".cinch")
        source = path.read_text()
        destination = project / f"{name}.cinch"
        destination.write_text(source)
        result.append(destination)
        pending.extend(re.findall(r"^\s*import\s+([A-Za-z0-9_.]+)", source, re.MULTILINE))
    return result


def main() -> None:
    """Build the selected corpus once, run both memory modes and retain exact evidence."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--frontend", choices=["rust", "native"], required=True)
    parser.add_argument("--cpu", type=int, default=2)
    parser.add_argument("--build-root", type=Path, default=DEFAULT_HOME)
    parser.add_argument("--artifacts", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    os.sched_setaffinity(0, {args.cpu})
    artifacts = args.artifacts.resolve()
    project = artifacts / "project"
    project.mkdir(parents=True, exist_ok=True)
    source = project / "Main.cinch"
    source.write_text(consumer())
    inputs = source_closure(project) + [source]
    executable = artifacts / "consumer"
    compiler = args.build_root / ("prim-wide-frozen/cinch" if args.frontend == "rust" else "stdlib-qualified-tail-stage3-frozen/cinch")
    if args.frontend == "rust":
        command = [str(compiler), "build", str(source), "-o", str(executable), "--opt-level", "2", "--rc-debug"]
    else:
        bundle = artifacts / "bundle.cinch"
        bundle.write_text("\n\n".join(path.read_text() for path in sorted(inputs)))
        wrapper = args.build_root / "stdlib-qualified-tail-stage3-frozen/frontend.py"
        manifest = wrapper.with_name("base-interfaces.json")
        command = ["python3", str(wrapper), "--compile-only", "--interfaces", str(manifest), "--rc-debug", str(bundle), str(artifacts / "consumer.ll"), str(executable), str(artifacts / "link-report.txt")]
    started = time.monotonic()
    compiled = subprocess.run(command, cwd=project, text=True, capture_output=True)
    (artifacts / "compile.stdout").write_text(compiled.stdout)
    (artifacts / "compile.stderr").write_text(compiled.stderr)
    if compiled.returncode != 0:
        diagnostics = artifacts / "consumer.ll.diagnostics"
        detail = diagnostics.read_text() if diagnostics.exists() else compiled.stdout
        raise RuntimeError(f"{args.frontend} compiler failed: {detail[-8000:]}\n{compiled.stderr[-8000:]}")
    elapsed = time.monotonic() - started
    runs = []
    for debug in [False, True]:
        environment = {**os.environ, "CINCH_RC_DEBUG": "1" if debug else "0", "CINCH_LEAK_CHECK": "1", "CINCH_STACK_SIZE": "8M", "CINCH_HEAP_LIMIT": "256M" if debug else "32M"}
        started = time.monotonic()
        result = subprocess.run([str(executable)], capture_output=True, text=True, env=environment)
        if result.returncode != 0 or "leaked=0" not in result.stderr:
            raise RuntimeError(f"debug={debug}, exit={result.returncode}: {result.stdout}\n{result.stderr}")
        runs.append({"debug_poisoning": debug, "heap_limit": environment["CINCH_HEAP_LIMIT"], "stack_limit": "8M", "elapsed_seconds": time.monotonic() - started, "stdout": result.stdout, "stderr": result.stderr})
    fixture = json.loads((ROOT / "seed/tests/fixtures/html-vectors.json").read_text())
    report = {"frontend": args.frontend, "cpu_affinity": [args.cpu], "compiler_sha256": digest(compiler), "compiler_command": command, "compile_seconds": elapsed, "sources": {path.name: digest(path) for path in inputs}, "fixture_sha256": digest(ROOT / "seed/tests/fixtures/html-vectors.json"), "fixture_rows": len(fixture["rows"]), "large_pipeline_rows": len(fixture["long"]), "intentional_reference_differences": sum(row["full_html5_expansion"] for row in fixture["rows"]), "executable_sha256": digest(executable), "runs": runs, "all_passed": True}
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(report, indent=2) + "\n")
    print(f"{args.frontend}: {len(fixture['rows'])} exact rows and {len(fixture['long'])} large pipelines passed both memory modes", flush=True)


if __name__ == "__main__":
    main()
