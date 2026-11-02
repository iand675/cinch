#!/usr/bin/env python3
"""Run library syntax experiments through the real Cinch compiler."""
from pathlib import Path
import argparse
import json
import os
import re
import subprocess
import sys

from engine import Engine, SyntaxFailure, utf16_position

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]


def compiler():
    return os.environ.get("CINCH_EXPERIMENT_COMPILER", str(ROOT / "compiler/target/debug/cinch"))


def library_project(engine):
    """Expand only explicitly imported source libraries, in dependency order."""
    index = {p.stem: p for p in (HERE / "libraries").glob("*.cinch")}
    active, done, ordered = [], set(), []
    def visit(name):
        if name in done or name not in index: return
        if name in active: raise ValueError("library import cycle: " + " -> ".join(active + [name]))
        active.append(name)
        path = index[name]
        library = Engine(path.read_text(), path)
        for dependency in library.runtime_imports: visit(dependency)
        ordered.append((path, library.expand()))
        active.pop(); done.add(name)
    for dependency in engine.runtime_imports: visit(dependency)
    return ordered


def prepare(path):
    path = Path(path).resolve()
    engine = Engine(path.read_text(), path)
    expansion = engine.expand()
    directory = HERE / ".generated" / path.stem
    directory.mkdir(parents=True, exist_ok=True)
    origins = {}
    # A stale source in this private build directory must not introduce an
    # ambient module or a competing declaration into the next project.
    for stale in directory.glob("*.cinch"): stale.unlink()
    for library, generated_library in library_project(engine):
        target = directory / library.name
        target.write_text(generated_library.fragment.text)
        target.with_suffix(".origins.json").write_text(json.dumps(generated_library.manifest(), indent=2) + "\n")
        origins[target] = (library, generated_library)
    output = directory / "Main.cinch"
    output.write_text(expansion.fragment.text)
    output.with_suffix(".origins.json").write_text(json.dumps(expansion.manifest(), indent=2) + "\n")
    origins[output] = (path, expansion)
    return output, origins


def drive(mode, path):
    generated, origins = prepare(path)
    result = subprocess.run([compiler(), mode, str(generated)], text=True, capture_output=True)
    diagnostic = result.stderr
    for generated_path, (original_path, expansion) in origins.items():
        diagnostic = map_diagnostic(diagnostic, generated_path, original_path, expansion)
    return result.returncode, result.stdout, diagnostic


def map_diagnostic(diagnostic, generated, path, expansion):
    # CLI positions are scalar columns, unlike the LSP's UTF-16 columns.
    def original_position(line, col):
        line, col = int(line) - 1, int(col) - 1
        lines = expansion.fragment.text.splitlines(keepends=True)
        offset = sum(len(x) for x in lines[:line]) + col
        if not expansion.fragment.origins: return (line + 1, col + 1)
        origin = expansion.fragment.origins[min(offset, len(expansion.fragment.origins) - 1)]
        before = expansion.source[:origin]
        return before.count(chr(10)) + 1, len(before.rsplit(chr(10), 1)[-1]) + 1
    pattern = re.compile(re.escape(str(generated)) + r": (\d+):(\d+):")
    def original(match):
        line, col = original_position(match.group(1), match.group(2))
        return f"{Path(path).resolve()}: {line}:{col}:"
    if f"(in {generated})" in diagnostic:
        def checked(match):
            line, col = original_position(match.group(1), match.group(2))
            return f"cinch: {line}:{col}:"
        diagnostic = re.sub(r"^cinch: (\d+):(\d+):", checked, diagnostic)
        diagnostic = diagnostic.replace(f"(in {generated})", f"(in {Path(path).resolve()})")
    diagnostic = pattern.sub(original, diagnostic)
    return diagnostic


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("command", choices=["expand", "check", "run", "explain", "lsp"])
    parser.add_argument("source", nargs="?")
    args = parser.parse_args()
    if args.command == "lsp":
        from editor_lsp import serve
        serve(); return 0
    if args.source is None: parser.error("source is required")
    try:
        if args.command in ("expand", "explain"):
            expansion = Engine(Path(args.source).read_text(), args.source).expand()
            if args.command == "expand": print(expansion.fragment.text, end="")
            else: print(json.dumps({"imports": expansion.imports, "trace": expansion.trace}, indent=2))
            return 0
        code, out, error = drive(args.command, args.source)
        print(out, end=""); print(error, end="", file=sys.stderr)
        return code
    except SyntaxFailure as error:
        text = Path(args.source).read_text()
        line, col = utf16_position(text, error.position)
        print(f"{args.source}:{line + 1}:{col + 1}: {error.message}", file=sys.stderr)
        return 1


if __name__ == "__main__": sys.exit(main())
