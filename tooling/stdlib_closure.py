"""Resolve transitive non-base standard library module dependencies from seed/prelude."""
from __future__ import annotations

import json
import re
from pathlib import Path

IMPORT_RE = re.compile(r"^\s*import\s+([A-Z][A-Za-z0-9_]*(?:\.[A-Z][A-Za-z0-9_]*)*)\b")


def base_package_modules(root: Path) -> set[str]:
    """Return module names supplied by the embedded cinch-base package."""
    manifest = json.loads((root / "seed" / "prelude" / "cinch-base.cinchpkg").read_text())
    return {row["name"] for row in manifest.get("modules", [])} | {"Prelude.Generated"}


def imported_modules(source: bytes) -> list[str]:
    """Extract imported module names from a Cinch source file."""
    found: list[str] = []
    for raw in source.decode("utf-8", errors="ignore").splitlines():
        stripped = raw.lstrip()
        if stripped.startswith("--"):
            continue
        match = IMPORT_RE.match(raw)
        if match:
            found.append(match.group(1))
    return found


def resolve_stdlib_closure(root: Path, files: dict[str, bytes]) -> dict[str, bytes]:
    """Pull every transitive non-base seed/prelude module imported by `files`."""
    base = base_package_modules(root)
    prelude = root / "seed" / "prelude"
    result = dict(files)
    queue = list(result.values())
    seen = {name[:-6] for name in result if name.endswith(".cinch")}
    while queue:
        src = queue.pop(0)
        for mod in imported_modules(src):
            if mod in base or mod in seen:
                continue
            candidate = prelude / f"{mod.replace('.', '/')}.cinch"
            if candidate.is_file():
                seen.add(mod)
                data = candidate.read_bytes()
                result[f"{mod}.cinch"] = data
                queue.append(data)
    return result
