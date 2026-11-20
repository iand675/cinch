#!/usr/bin/env python3
"""Compile and execute independent slash-path fixtures through selected frontends.

The fixture SDK is required only for regeneration. Normal consumers use fixed
JSON bytes, exact public operations and separately bounded poisoned-free runs.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[1]

HELPERS = (ROOT / "seed/tests/fixtures/path-consumer-helpers.cinchpart").read_text()


def literal(value: str) -> str:
    """Render exact Unicode and control scalars with the language's supported escapes."""
    escaped = []
    for char in value:
        if char == '"':
            escaped.append('\\"')
        elif char == "\\":
            escaped.append("\\\\")
        elif char == "\n":
            escaped.append("\\n")
        elif char == "\r":
            escaped.append("\\r")
        elif char == "\t":
            escaped.append("\\t")
        elif ord(char) < 32 or ord(char) == 127:
            escaped.append(f"\\u{{{ord(char):x}}}")
        else:
            escaped.append(char)
    return '"' + "".join(escaped) + '"'


def flag(value: bool) -> str:
    """Render the explicit Boolean carrier."""
    return "True" if value else "False"


def checks(group: str, rows: list[dict]) -> list[str]:
    """Turn independent observations into public-API runtime assertions."""
    if group == "lexical":
        return ["lexical-case " + " ".join([literal(row[key]) for key in ["input", "clean", "base", "dir", "ext"]] + [flag(row["absolute"])] + [literal(row[key]) for key in ["split_dir", "split_base"]]) for row in rows]
    if group == "joins":
        return ["join-case [" + ",".join(map(literal, row["elements"])) + "] " + literal(row["output"]) for row in rows]
    return ["glob-case " + " ".join([literal(row["pattern"]), literal(row["name"]), flag(row["matched"]), flag(row["error"])]) for row in rows]


def consumer(expressions: list[str]) -> str:
    """Keep individual source batches bounded while retaining one actual executable."""
    source = HELPERS
    names = []
    for index in range(0, len(expressions), 192):
        name = f"checks{index // 192}"
        names.append(name + " ()")
        source += f"\n  -- | Observe independent fixture batch {index // 192}.\n  {name} :: Unit -> Bool\n  {name} unit = all [\n" + ",\n".join(f"    ({value})" for value in expressions[index:index + 192]) + "\n    ]\n"
    source += "\n  -- | Return failure if any complete fixture observation differs.\n  main :: Int\n  main = if " + (" && ".join(names) or "True") + " then 0 else 1\n"
    return source


def run(command: list[str], **kwargs) -> subprocess.CompletedProcess:
    """Preserve compiler diagnostics and stop dependent operations on failure."""
    result = subprocess.run(command, capture_output=True, text=True, **kwargs)
    if result.returncode:
        raise RuntimeError(f"command failed: {command}\n{result.stdout}\n{result.stderr}")
    return result


def main() -> None:
    """Compile source-distributed modules and validate each fixture group in both heap modes."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--compiler", type=Path, required=True)
    parser.add_argument("--native-interfaces", type=Path)
    parser.add_argument("--artifacts", type=Path, required=True)
    parser.add_argument("--groups", nargs="+", choices=["lexical", "joins", "globs", "owners"], default=["lexical", "joins", "globs"])
    parser.add_argument("--module-source", type=Path, default=ROOT / "seed/prelude/Text/Path.cinch")
    parser.add_argument("--first", type=int, default=0)
    parser.add_argument("--limit", type=int)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    args.artifacts.mkdir(parents=True, exist_ok=True)
    fixture = json.loads((ROOT / "seed/tests/fixtures/path-vectors.json").read_text())
    report = {"frontend": str(args.compiler.resolve()), "frontend_sha256": hashlib.sha256(args.compiler.read_bytes()).hexdigest(), "native_interfaces": str(args.native_interfaces) if args.native_interfaces else None, "source_sha256": hashlib.sha256(args.module_source.read_bytes()).hexdigest(), "results": []}
    actual_compiler = args.compiler.parent / "cinch" if args.native_interfaces else args.compiler
    report["compiler_sha256"] = hashlib.sha256(actual_compiler.read_bytes()).hexdigest()
    for group in args.groups:
        work = args.artifacts / group
        work.mkdir(parents=True, exist_ok=True)
        for name, path in [("Text.Path", args.module_source), ("Data.Bytes.Word", ROOT / "seed/prelude/Data/Bytes/Word.cinch")]:
            shutil.copyfile(path, work / f"{name}.cinch")
        rows = [None] if group == "owners" else fixture[group][args.first:]
        if args.limit is not None:
            rows = rows[:args.limit]
        source = consumer(["owner-case ()"]) if group == "owners" else consumer(checks(group, rows))
        if group == "owners":
            source = source.replace("  -- | Return failure", (ROOT / "seed/tests/fixtures/path-owner-consumer.cinchpart").read_text() + "\n  -- | Return failure")
        (work / "Main.cinch").write_text(source)
        exe = (work / "consumer").resolve()
        if args.native_interfaces:
            bundle = (work / "Bundle.cinch").resolve()
            bundle.write_text((ROOT / "seed/prelude/Data/Bytes/Word.cinch").read_text() + "\n" + args.module_source.read_text() + "\n" + source)
            run([sys.executable, str(args.compiler.resolve()), "--rc-debug", "--compile-only", "--interfaces", str(args.native_interfaces.resolve()), str(bundle), str(exe.with_suffix('.ll')), str(exe), str(exe.with_suffix('.link.txt'))])
        else:
            run([str(args.compiler.resolve()), "build", str((work / "Main.cinch").resolve()), "-o", str(exe), "--opt-level", "2", "--rc-debug"])
        observed = []
        for debug in [False, True]:
            output = run([str(exe)], env={**os.environ, "CINCH_RC_DEBUG": "1" if debug else "0", "CINCH_LEAK_CHECK": "1", "CINCH_HEAP_LIMIT": "256M" if debug else "32M", "CINCH_STACK_SIZE": "8M"})
            assert "leaked=0" in output.stderr, output.stderr
            observed.append({"debug": debug, "zero_leaks": True})
        report["results"].append({"group": group, "cases": len(rows), "runs": observed})
        print(f"{group}: {len(rows)} cases, both heap modes passed", flush=True)
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(report, indent=2) + "\n")


if __name__ == "__main__":
    main()
