#!/usr/bin/env python3
"""Compile and run independent URL codec/query/parser suites with frozen frontends.

The consumer templates are shared with Rust regression targets. Source-distributed
dependencies are copied by declared module name, and execution checks ordinary
live space separately from poisoned-free quarantine. No compiler rebuild is needed.
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
BUILD_ROOT = Path("/workspace/dasl-stdlib-io/build")


def digest(path: Path) -> str:
    """Identify exact source, fixture, executable and compiler bytes."""
    return hashlib.sha256(path.read_bytes()).hexdigest()


def literal(text: str) -> str:
    """Encode an ASCII metadata stream using canonical embedded NUL escapes."""
    return json.dumps(text).replace("\\u0000", "\\u{0}")


def consumer(suite: str) -> str:
    """Render compact exact rows and shallow batches of large public-pipeline checks."""
    fixture = json.loads((ROOT / "compiler/tests/fixtures/url-vectors.json").read_text())
    if suite == "url":
        return url_consumer()
    if suite != "codec":
        raise ValueError(f"URL suite not yet implemented: {suite}")
    source = (ROOT / "compiler/tests/fixtures/url-codec-consumer.cinch.in").read_text()
    encoded = "".join(str(row["mode"]) + "\0" + row["input"] + "\0" + row["output"] + "\0" for row in fixture["encoded"])
    decoded = "".join(str(row["mode"]) + "\0" + row["input"] + "\0" + row["output"] + "\0" + row["error"] + "\0" + str(row["offset"]) + "\0" for row in fixture["decoded"])
    checks = ["long-case " + " ".join(str(row[key]) for key in ("mode", "size", "pattern", "input_hash", "output_size", "output_hash")) for row in fixture["long"]]
    declarations = []
    calls = []
    selection = []
    for index, start in enumerate(range(0, len(checks), 16)):
        suffix = chr(97 + index) if index < 26 else "a" + chr(97 + index - 26)
        name = f"long-batch-{suffix}"
        declarations.append(f"  -- | Check independent large-input rows {start} through {min(start + 15, len(checks) - 1)}.\n  {name} :: Unit -> Bool\n  {name} unit = all [" + ",\n    ".join(checks[start:start + 16]) + "]")
        calls.append(name + " ()")
        selection.append(f'    else if name == "{name}" then {name} ()')
    return source.replace("__ENCODED__", literal(encoded)).replace("__DECODED__", literal(decoded)).replace("__LONG_DECLS__", "\n\n".join(declarations)).replace("__LONG_CALLS__", ", ".join(calls)).replace("__LONG_SELECT__", "\n".join(selection))


def url_consumer() -> str:
    """Render shallow independent public-capability groups with exact byte metadata."""
    fixture = json.loads((ROOT / "compiler/tests/fixtures/url-vectors.json").read_text())
    source = (ROOT / "compiler/tests/fixtures/url-consumer.cinch.in").read_text()
    declarations, selection, calls = [], ['    else if name == "ownership" then ownership-cases ()'], ["ownership-cases"]
    for section, runner in [("parsing", "parsing-from"), ("queries", "query-from"), ("resolution", "resolution-from"), ("manual", "manual-from"), ("joining", "joining-from")]:
        for index, start in enumerate(range(0, len(fixture[section]), 64)):
            fields = []
            for row in fixture[section][start:start + 64]:
                if section == "manual": fields.extend(row)
                elif section == "parsing": fields.extend([row["input"], "1" if row["request"] else "0", row["error"], row["context"], *row["fields"]])
                elif section == "resolution": fields.extend([row["base"], row["reference"], row["error"], row["context"], *row["fields"]])
                elif section == "queries": fields.extend([row["input"], row["output"], row["error"], str(row["size"])])
                else: fields.extend([row["base"], str(len(row["elements"])), *row["elements"], row["output"], row["error"], *row["fields"]])
            name = section + "-batch-" + (chr(97 + index) if index < 26 else "a" + chr(97 + index - 26))
            stream = "".join(value + "\0" for value in fields)
            declarations.append(f"  -- | Check independent {section} rows in one bounded metadata group.\n  {name} :: Unit -> Nat\n  {name} unit = {runner} (string-to-bytes {literal(stream)}) (u32-to-nat (runtime-size 0))")
            selection.append(f'    else if name == "{name}" then {name} ()')
            calls.append(name)
    for index, row in enumerate(fixture["query_long"]):
        name = "stress-batch-" + (chr(97 + index) if index < 26 else "a" + chr(97 + index - 26))
        arguments = " ".join(str(row[key]) for key in ["count", "mode", "input_size", "input_hash", "output_size", "output_hash", "size"]) + " " + literal(row["error"])
        declarations.append(f"  -- | Check independent query stress and exact default-limit boundaries.\n  {name} :: Unit -> Nat\n  {name} unit = if query-stress-case {arguments} then 0 else 1")
        selection.append(f'    else if name == "{name}" then {name} ()')
        calls.append(name)
    blocks = []
    for block, start in enumerate(range(0, len(calls), 8)):
        name = "check-block-" + chr(97 + block)
        block_source = f"  -- | Check a shallow batch of independent capability groups.\n  {name} :: Unit -> Nat\n  {name} unit =\n"
        for index, call in enumerate(calls[start:start + 8], start):
            block_source += f"    let result = {call} () in if result /= 0 then {index + 1} * 1000000 + result else\n"
        declarations.append(block_source + "    0")
        blocks.append(name)
    all_source = "  -- | Return the first failing group number and byte offset.\n  all-groups :: Unit -> Nat\n  all-groups unit =\n"
    for name in blocks:
        all_source += f"    let result = {name} () in if result /= 0 then result else\n"
    declarations.append(all_source + "    0")
    return source.replace("__GROUP_DECLS__", "\n\n".join(declarations)).replace("__GROUP_SELECT__", "\n".join(selection))


def source_closure(project: Path, main: Path) -> list[Path]:
    """Copy only reachable non-embedded source imports beside the selected consumer."""
    manifest = json.loads((ROOT / "website/src/data/library-sources.json").read_text())
    embedded = {row["module"] for row in manifest["modules"] if row["embedded"]}
    imports = lambda source: re.findall(r"^\s*import\s+([A-Za-z0-9_.]+)", source, re.MULTILINE)
    pending = imports(main.read_text())
    found = set()
    result = []
    while pending:
        name = pending.pop()
        if name in found or name in embedded:
            continue
        found.add(name)
        original = ROOT / "compiler/prelude" / (name.replace(".", "/") + ".cinch")
        text = original.read_text()
        path = project / f"{name}.cinch"
        path.write_text(text)
        result.append(path)
        pending.extend(imports(text))
    return result


def main() -> None:
    """Build a selected suite once and preserve complete source/ownership evidence."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--suite", choices=["codec", "url"], required=True)
    parser.add_argument("--frontend", choices=["rust", "native"], required=True)
    parser.add_argument("--cpu", type=int, default=2)
    parser.add_argument("--build-root", type=Path, default=BUILD_ROOT)
    parser.add_argument("--artifacts", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    os.sched_setaffinity(0, {args.cpu})
    artifacts = args.artifacts.resolve()
    project = artifacts / "project"
    project.mkdir(parents=True, exist_ok=True)
    source = project / "Main.cinch"
    source.write_text(consumer(args.suite))
    inputs = source_closure(project, source) + [source]
    executable = artifacts / "consumer"
    compiler = args.build_root / ("prim-wide-frozen/cinch" if args.frontend == "rust" else "stdlib-qualified-tail-stage3-frozen/cinch")
    if args.frontend == "rust":
        command = [str(compiler), "build", str(source), "-o", str(executable), "--opt-level", "2", "--rc-debug"]
    else:
        bundle = artifacts / "bundle.cinch"
        bundle.write_text("\n\n".join(path.read_text() for path in sorted(inputs)))
        wrapper = args.build_root / "stdlib-qualified-tail-stage3-frozen/frontend.py"
        command = ["python3", str(wrapper), "--compile-only", "--interfaces", str(wrapper.with_name("base-interfaces.json")), "--rc-debug", str(bundle), str(artifacts / "consumer.ll"), str(executable), str(artifacts / "link-report.txt")]
    started = time.monotonic()
    built = subprocess.run(command, cwd=project, capture_output=True, text=True)
    (artifacts / "compile.stdout").write_text(built.stdout)
    (artifacts / "compile.stderr").write_text(built.stderr)
    if built.returncode != 0:
        diagnostic = artifacts / "consumer.ll.diagnostics"
        detail = diagnostic.read_text() if diagnostic.exists() else built.stdout
        raise RuntimeError(f"{args.frontend} compile failed:\n{detail[-8000:]}\n{built.stderr[-8000:]}")
    elapsed = time.monotonic() - started
    runs = []
    groups = re.findall(r'else if name == "([a-z-]+)"', source.read_text())
    for debug, group in [(False, "all")] + [(True, group) for group in groups]:
        environment = {**os.environ, "CINCH_RC_DEBUG": "1" if debug else "0", "CINCH_LEAK_CHECK": "1", "CINCH_HEAP_LIMIT": "256M" if debug else "32M", "CINCH_STACK_SIZE": "8M"}
        started = time.monotonic()
        result = subprocess.run([str(executable), group], capture_output=True, text=True, env=environment)
        if result.returncode != 0 or "leaked=0" not in result.stderr:
            raise RuntimeError(f"debug={debug}, exit={result.returncode}: {result.stdout}\n{result.stderr}")
        runs.append({"group": group, "debug_poisoning": debug, "heap_limit": environment["CINCH_HEAP_LIMIT"], "stack_limit": "8M", "elapsed_seconds": time.monotonic() - started, "stdout": result.stdout, "stderr": result.stderr})
    report = {"suite": args.suite, "frontend": args.frontend, "compiler_sha256": digest(compiler), "compiler_command": command, "compile_seconds": elapsed, "cpu_affinity": [args.cpu], "sources": {path.name: digest(path) for path in inputs}, "fixture_sha256": digest(ROOT / "compiler/tests/fixtures/url-vectors.json"), "executable_sha256": digest(executable), "runs": runs, "all_passed": True}
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(report, indent=2) + "\n")
    print(f"{args.frontend} {args.suite} URL suite passed both memory modes", flush=True)


if __name__ == "__main__":
    main()
