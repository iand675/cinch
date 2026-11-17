#!/usr/bin/env python3
"""Validate the isolated archive audit without invoking a compiler or reference SDK."""
from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path
import re


ROOT = Path(__file__).resolve().parent


def digest(path: Path) -> str:
    """Identify exact source inputs without inferring their execution or correctness."""
    return hashlib.sha256(path.read_bytes()).hexdigest()


def reference_declarations(sdk: Path) -> list[str]:
    """Read cumulative installed compatibility records without running the SDK."""
    found: set[str] = set()
    for path in sorted((sdk / "api").glob("go1*.txt")):
        for line in path.read_text().splitlines():
            match = re.match(r"pkg archive/tar(?: \([^)]+\))?, (.*?)(?: #\d+)?$", line)
            if match:
                found.add(match[1])
    return sorted(found)


def declarations(path: Path) -> int:
    """Require canonical comments on each datatype, constructor, function and inline kernel."""
    count = 0
    lines = path.read_text().splitlines()
    pattern = re.compile(r"^\s+(?:data \S+|prim \S+ ::|[\w-]+ ::)")
    for index, line in enumerate(lines):
        if not pattern.match(line):
            continue
        previous = index - 1
        comments = []
        while previous >= 0 and lines[previous].lstrip().startswith("--"):
            comments.append(lines[previous].lstrip())
            previous -= 1
        if not any(comment.startswith("-- |") for comment in comments):
            raise AssertionError(f"{path}:{index + 1}: declaration lacks canonical documentation")
        count += 1
    assert not re.search(r"\b(?:golang|go|ruby)\b", path.read_text(), re.I), path
    return count


def main() -> None:
    """Check exact mapping coverage, documentation and source-only receipt integrity."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--sdk", type=Path, default=Path("/tmp/cinch-go-current/go"))
    parser.add_argument("--output", type=Path)
    args = parser.parse_args()
    mapping = json.loads((ROOT / "operation-mapping.json").read_text())
    rows = mapping["rows"]
    expected = reference_declarations(args.sdk)
    assert len(expected) == len(rows) == mapping["record_count"] == 85
    assert [row["reference_declaration"] for row in rows] == expected
    assert all(row["candidate_mapping"] and row["contract"] for row in rows)
    assert all(row["status"] == "designed-not-checked" for row in rows)
    sources = sorted((ROOT / "sources").rglob("*.cinch"))
    source_report = {str(path.relative_to(ROOT)): {"sha256": digest(path), "documented_declarations": declarations(path)} for path in sources}
    for path, expected_hash in mapping["reference_source_hashes"].items():
        assert digest(Path(path)) == expected_hash, path
    report = {
        "status": "static-structure-check-only",
        "operation_records": len(rows),
        "sources": source_report,
        "reference_generator_sha256": digest(ROOT / "reference/main.go"),
        "compiler_invoked": False,
        "sdk_invoked": False,
        "runtime_executed": False,
        "qualification": "Source syntax, kinds, refinements, runtime behavior, portable objects and performance remain unqualified.",
    }
    encoded = json.dumps(report, indent=2) + "\n"
    if args.output:
        args.output.write_text(encoded)
    else:
        print(encoded, end="")


if __name__ == "__main__":
    main()
