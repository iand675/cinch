#!/usr/bin/env python3
"""Snapshot versioned Unicode ranges and simple-case data with reproducible provenance.

Only generated factual tables and locale mappings are imported. Runtime algorithms
are implemented independently. The original BSD notice is preserved beside the
snapshot; source paths, URLs and exact hashes make the transformation reviewable.
"""
from __future__ import annotations

import argparse
import ast
import hashlib
import json
from pathlib import Path
import re

ROOT = Path(__file__).resolve().parents[1]
NUMBER = r"(?:0[xX][0-9a-fA-F]+|[0-9]+)"


def digest(path: Path) -> str:
    """Identify the exact source or imported data bytes."""
    return hashlib.sha256(path.read_bytes()).hexdigest()


def block(source: str, pattern: str) -> str:
    """Read a balanced literal block after an exact declaration prefix."""
    match = re.search(pattern, source, re.MULTILINE)
    if match is None:
        raise ValueError(f"missing table block: {pattern}")
    start = source.index("{", match.start())
    depth = 1
    at = start + 1
    while at < len(source) and depth:
        depth += (source[at] == "{") - (source[at] == "}")
        at += 1
    if depth:
        raise ValueError(f"unbalanced table block: {pattern}")
    return source[start + 1:at - 1]


def integer(expression: str) -> int:
    """Evaluate only integer literals, the alternating-case sentinel and signed sums."""
    tree = ast.parse(expression.strip(), mode="eval")

    def visit(node: ast.AST) -> int:
        """Reject executable syntax while accepting the table's small integer grammar."""
        if isinstance(node, ast.Constant) and type(node.value) is int:
            return node.value
        if isinstance(node, ast.Name) and node.id == "UpperLower":
            return 0x110000
        if isinstance(node, ast.UnaryOp) and isinstance(node.op, (ast.USub, ast.UAdd)):
            value = visit(node.operand)
            return -value if isinstance(node.op, ast.USub) else value
        if isinstance(node, ast.BinOp) and isinstance(node.op, (ast.Add, ast.Sub)):
            left, right = visit(node.left), visit(node.right)
            return left + right if isinstance(node.op, ast.Add) else left - right
        raise ValueError(f"unsupported table integer: {expression}")

    return visit(tree.body)


def ranges(body: str, width: int) -> list[list[int]]:
    """Parse one independently ordered 16-bit or 32-bit strided range slice."""
    match = re.search(rf"R{width}:\s*\[\]Range{width}\s*\{{", body)
    if match is None:
        return []
    content = block(body[match.start():], rf"R{width}:\s*\[\]Range{width}\s*\{{")
    rows = [[integer(value) for value in values] for values in re.findall(rf"\{{\s*({NUMBER})\s*,\s*({NUMBER})\s*,\s*({NUMBER})\s*\}}", content)]
    previous = -1
    for low, high, stride in rows:
        if not (previous < low <= high <= (1 << width) - 1 and stride > 0):
            raise ValueError(f"invalid or unordered R{width} entry: {(low, high, stride)}")
        previous = high
    return rows


def mapping(source: str, name: str, strings: bool = False) -> dict[str, str]:
    """Parse canonical public name mappings without evaluating source expressions."""
    body = block(source, rf"^var {name} = map\[string\][^\n]+\{{")
    values = re.findall(r'^\s*"([^"\\]+)":\s*("[^"\\]+"|[A-Za-z_][A-Za-z_0-9]*),', body, re.MULTILINE)
    if not values:
        raise ValueError(f"empty name map: {name}")
    return {key: json.loads(value) if strings else value for key, value in values}


def case_ranges(body: str) -> list[list[int]]:
    """Parse source bounds and three exact signed simple-case deltas."""
    rows = []
    for low, high, deltas in re.findall(rf"(?:CaseRange)?\{{\s*({NUMBER}),\s*({NUMBER}),\s*d\{{([^{{}}]+)\}}\s*\}}", body):
        values = [integer(value) for value in deltas.split(",")]
        if len(values) != 3:
            raise ValueError("simple case entry must have three deltas")
        rows.append([integer(low), integer(high), *values])
    if not rows:
        raise ValueError("empty simple case table")
    previous = -1
    for low, high, upper, lower, title in rows:
        if not (previous < low <= high <= 0x10FFFF):
            raise ValueError(f"invalid simple case bounds: {(low, high)}")
        if 0x110000 in (upper, lower, title) and (upper, lower, title) != (0x110000,) * 3:
            raise ValueError("partial alternating-case sentinel")
        previous = high
    return rows


def snapshot(sdk: Path) -> dict:
    """Import complete named tables, case ranges, fold cycles and exact source metadata."""
    if (sdk / "VERSION").read_text().splitlines()[0] != "go1.27.4":
        raise ValueError("explicit SDK version review required; expected go1.27.4")
    tables_path = sdk / "src/unicode/tables.go"
    locale_path = sdk / "src/unicode/casetables.go"
    source = tables_path.read_text()
    version = re.search(r'^const Version = "([^"]+)"', source, re.MULTILINE).group(1)
    if version != "17.0.0":
        raise ValueError(f"explicit Unicode version review required: {version}")
    tables = {}
    for match in re.finditer(r"^var ([A-Za-z_][A-Za-z_0-9]*) = &RangeTable\{", source, re.MULTILINE):
        name = match.group(1)
        body = block(source[match.start():], r"^var [A-Za-z_][A-Za-z_0-9]* = &RangeTable\{")
        r16, r32 = ranges(body, 16), ranges(body, 32)
        latin = re.search(r"LatinOffset:\s*([0-9]+)", body)
        offset = int(latin.group(1)) if latin else 0
        if offset > len(r16) or any(high > 255 for low, high, stride in r16[:offset]):
            raise ValueError(f"inconsistent Latin-1 offset: {name}")
        tables[name] = {"r16": r16, "r32": r32, "latin_offset": offset}
    aliases = dict(re.findall(r"^\s*([A-Za-z][A-Za-z_0-9]*)\s*=\s*([A-Za-z_][A-Za-z_0-9]*)\s*(?://[^\n]*)?$", source, re.MULTILINE))

    def target(name: str) -> str:
        """Resolve a public alias to its factual range payload, rejecting alias loops."""
        seen = set()
        while name not in tables:
            if name in seen or name not in aliases:
                raise ValueError(f"unresolved range alias: {name}")
            seen.add(name)
            name = aliases[name]
        return name

    catalogs = {name: {key: target(value) for key, value in mapping(source, name).items()} for name in ["Categories", "Scripts", "Properties", "FoldCategory", "FoldScript"]}
    exported = {name: target(name) for name in aliases if name[0].isupper() and (name in tables or aliases[name] in tables)}
    folds = block(source, r"^var caseOrbit = \[\]foldPair\{")
    orbit = [[integer(a), integer(b)] for a, b in re.findall(rf"\{{\s*({NUMBER}),\s*({NUMBER})\s*\}}", folds)]
    ascii_body = block(source, r"^var asciiFold = \[MaxASCII \+ 1\]uint16\{")
    ascii_fold = [integer(value) for value in re.findall(rf"^\s*({NUMBER}),", ascii_body, re.MULTILINE)]
    if len(ascii_fold) != 128 or len({a for a, b in orbit}) != len(orbit):
        raise ValueError("incomplete or ambiguous fold-cycle data")
    locales = case_ranges(block(locale_path.read_text(), r"^var _TurkishCase = SpecialCase\{"))
    inputs = [tables_path, locale_path, sdk / "LICENSE", sdk / "VERSION"]
    return {"schema": 1, "unicode_version": version, "sdk_version": (sdk / "VERSION").read_text().splitlines()[0], "sources": {str(path.relative_to(sdk)): {"sha256": digest(path), "url": "https://github.com/golang/go/blob/go1.27.4/" + str(path.relative_to(sdk))} for path in inputs}, "license": "unicode17-BSD-LICENSE.txt", "tables": tables, "catalogs": catalogs, "category_aliases": mapping(source, "CategoryAliases", True), "exported_aliases": exported, "case_ranges": case_ranges(block(source, r"^var _CaseRanges = \[\]CaseRange\{")), "ascii_fold": ascii_fold, "fold_orbit": orbit, "special_cases": {"Turkish": locales, "Azeri": locales}}


def main() -> None:
    """Write a deterministic source snapshot and retain its original redistribution notice."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--sdk", type=Path, default=Path("/tmp/cinch-go-current/go"))
    parser.add_argument("--output", type=Path, default=ROOT / "tooling/data/unicode17-tables.json")
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    encoded = json.dumps(snapshot(args.sdk), sort_keys=True, indent=2) + "\n"
    license_path = args.output.with_name("unicode17-BSD-LICENSE.txt")
    license_text = (args.sdk / "LICENSE").read_text()
    if args.check:
        if not args.output.exists() or args.output.read_text() != encoded or not license_path.exists() or license_path.read_text() != license_text:
            raise SystemExit("stale Unicode source snapshot or license")
    else:
        args.output.parent.mkdir(parents=True, exist_ok=True)
        args.output.write_text(encoded)
        license_path.write_text(license_text)


if __name__ == "__main__":
    main()
