#!/usr/bin/env python3
"""Guard the compiler directory's layout.

The compiler's own modules are the `Bootstrap.*.cinch` files in `compiler/`.
Everything that is record rather than source lives under a directory there:
historical results under `evidence/`, working notes under `notes/`,
development patches under `patches/`, and the inputs a build reads by path
under `pins/`. The Rust seed that bootstraps the ladder is a separate tree,
`seed/`.

This check fails when a file lands back at the compiler's top level, when a
result sits in the wrong evidence category, or when a tracked text file still
names a compiler file at the path it used to occupy.
"""
from __future__ import annotations

import os
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
COMPILER = ROOT / "compiler"

TOP_DIRS = {"evidence", "notes", "patches", "pins", "programs", "proposals",
            "repros", "seed", "status-ui"}
TOP_FILES = {"README.md", "CHECKPOINT.md", "CONFORMANCE.md", "progress.json"}
CATEGORIES = {"stdlib", "current", "native", "runtime", "other"}
CATEGORY_PREFIX = (("stdlib-", "stdlib"), ("current-", "current"),
                   ("native-", "native"), ("runtime-", "runtime"))

SKIP_DIRS = {".git", ".jj", "node_modules", "target", "build", "dist",
             ".astro", ".devenv", ".direnv", "result"}
LIVE_JSON = {"compiler/progress.json"}
LIVE_JSON_DIRS = ("compiler/pins/", "design/", "tooling/data/")
TEXT_SUFFIXES = {".md", ".py", ".sh", ".nix", ".yml", ".yaml", ".toml",
                 ".html", ".css", ".js", ".mjs", ".cinch", ".rs"}
HISTORICAL = ("compiler/evidence/", "seed/evidence/", "benchmarks/", "experiments/")
REFERENCE = re.compile(r"bootstrap/([A-Za-z0-9_./-]+\.(?:json|patch|md))")
# `bootstrap/` was the compiler tree's old name. Any live reference to it that
# is not the build directory means a file still points at the old layout.
STALE_TREE = re.compile(r"(?<![\w./-])((?:\.{1,2}/)*)bootstrap/([A-Za-z0-9_./-]*)")
SELF = "tooling/check-compiler-layout.py"


def category_of(name: str) -> str:
    for prefix, category in CATEGORY_PREFIX:
        if name.startswith(prefix):
            return category
    return "other"


def organized_names() -> dict[str, list[str]]:
    """Every file in an organized directory, keyed by basename."""
    found: dict[str, list[str]] = {}
    for directory in ("evidence", "notes", "patches", "pins"):
        for path in (COMPILER / directory).rglob("*"):
            if path.is_file():
                found.setdefault(path.name, []).append(path.relative_to(ROOT).as_posix())
    return found


def check_layout(errors: list[str]) -> None:
    for entry in sorted(COMPILER.iterdir()):
        name = entry.name
        if entry.is_dir():
            if name not in TOP_DIRS:
                errors.append(f"compiler/{name}: unexpected top-level directory")
            continue
        if name.endswith(".cinch") or name in TOP_FILES:
            continue
        errors.append(f"compiler/{name}: file belongs under evidence/, notes/, patches/ or pins/")

    evidence = COMPILER / "evidence"
    for entry in sorted(evidence.iterdir()):
        if entry.is_file():
            errors.append(f"compiler/evidence/{entry.name}: file belongs in a category directory")
            continue
        name = entry.name
        if name in CATEGORIES:
            for path in sorted(entry.iterdir()):
                if not path.is_file():
                    errors.append(f"{path.relative_to(ROOT)}: unexpected directory inside a category")
                    continue
                expected = category_of(path.name)
                if name != expected:
                    errors.append(f"{path.relative_to(ROOT)}: belongs in evidence/{expected}/")
        elif not name.endswith("-evidence"):
            errors.append(f"compiler/evidence/{name}: category or topic-evidence directory expected")


def is_historical(rel: str) -> bool:
    return rel.startswith(HISTORICAL)


def live_text(rel: str) -> bool:
    suffix = os.path.splitext(rel)[1]
    if suffix == ".json":
        return rel in LIVE_JSON or rel.startswith(LIVE_JSON_DIRS)
    return suffix in TEXT_SUFFIXES


def check_references(errors: list[str]) -> None:
    organized = organized_names()
    for base, directories, files in os.walk(ROOT):
        directories[:] = [d for d in directories
                          if d not in SKIP_DIRS and not d.startswith("result")]
        for name in files:
            path = Path(base) / name
            rel = path.relative_to(ROOT).as_posix()
            if is_historical(rel) or not live_text(rel):
                continue
            try:
                text = path.read_text(encoding="utf-8", errors="ignore")
            except OSError:
                continue
            for line_number, line in enumerate(text.splitlines(), 1):
                if rel != SELF:
                    for stale in STALE_TREE.finditer(line):
                        token_start = stale.start()
                        while token_start and line[token_start - 1] not in " \t`'\"()[]<>,;":
                            token_start -= 1
                        if line[token_start:stale.start()].startswith("/"):
                            continue
                        errors.append(
                            f"{rel}:{line_number}: {stale.group(0)} names the compiler tree's old directory")
                for match in REFERENCE.finditer(line):
                    token_start = match.start()
                    while token_start and line[token_start - 1] not in " \t`'\"()[]<>,;":
                        token_start -= 1
                    if line[token_start:match.start()].startswith("/"):
                        # An absolute path names some other checkout, not this one.
                        continue
                    target = match.group(0)
                    if (ROOT / target).exists():
                        continue
                    basename = os.path.basename(target)
                    if basename in organized:
                        errors.append(
                            f"{rel}:{line_number}: {target} moved to {organized[basename][0]}")


def main() -> int:
    errors: list[str] = []
    check_layout(errors)
    check_references(errors)
    for error in errors:
        print(error, file=sys.stderr)
    print(f"compiler layout: {len(errors)} problem(s)")
    return 1 if errors else 0


if __name__ == "__main__":
    raise SystemExit(main())
