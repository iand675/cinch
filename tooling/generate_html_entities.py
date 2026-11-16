#!/usr/bin/env python3
"""Generate a packed, allocation-free HTML named-reference lookup.

The checked-in input is the HTML5 table shipped by Python 3.12.14. Its provenance
and license live beside the data. The generated lookup checks all packed name
bytes after routing by a deterministic hash, so hash collisions cannot admit an
unknown reference. Only the generated suffix of Text.Html is replaced.
"""
from __future__ import annotations

import argparse
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
DATA = ROOT / "tooling/data/html-entities.json"
MARKER = "  -- | Generated packed HTML named-reference lookup; regenerate with tooling/generate_html_entities.py.\n"
MASK64 = (1 << 64) - 1
MULTIPLIERS = (0x9E3779B185EBCA87, 0xC2B2AE3D27D4EB4F, 0x165667B19E3779F9, 0x85EBCA77C2B2AE63)


def packed_name(name: str) -> tuple[int, int, int, int, int]:
    """Pack at most 32 ASCII name bytes in four little-endian words and a length."""
    data = name.encode("ascii")
    if not 1 <= len(data) <= 32:
        raise ValueError(f"unsupported entity name length: {name}")
    padded = data.ljust(32, b"\0")
    words = tuple(int.from_bytes(padded[i:i + 8], "little") for i in range(0, 32, 8))
    return (*words, len(data))


def name_hash(words: tuple[int, int, int, int, int]) -> int:
    """Mix packed words with defined unsigned wrapping; full equality remains authoritative."""
    value = words[4]
    for word, multiplier in zip(words[:4], MULTIPLIERS):
        value ^= (word * multiplier) & MASK64
    return value


def replacement_word(value: str) -> int:
    """Store up to six UTF-8 replacement bytes with their count in the top byte."""
    data = value.encode("utf-8")
    if not 1 <= len(data) <= 6:
        raise ValueError(f"unsupported entity value length: {value!r}")
    return int.from_bytes(data, "little") | len(data) << 56


def records() -> list[tuple[int, tuple[int, int, int, int, int], int]]:
    """Load fixed independent data and verify each generated name has an unambiguous route."""
    entities = json.loads(DATA.read_text())["entities"]
    result = sorted((name_hash(packed_name(name)), packed_name(name), replacement_word(value))
                    for name, value in entities.items())
    if len({record[0] for record in result}) != len(result):
        raise ValueError("entity hash collision; select another deterministic multiplier set")
    return result


def suffix(index: int) -> str:
    """Give private buckets readable identifier suffixes without numeric word segments."""
    return chr(97 + index) if index < 26 else "a" + chr(97 + index - 26)


def tree(items: list[tuple[int, tuple[int, int, int, int, int], int]], indent: int) -> list[str]:
    """Render a balanced raw-word search whose leaves validate every name byte."""
    prefix = " " * indent
    if len(items) == 1:
        hashed, words, value = items[0]
        tests = [f"eq-u64 h {hashed}"] + [f"eq-u64 {field} {word}" for field, word in zip("abcdn", words)]
        return [prefix + "if " + " && ".join(tests) + f" then {value} else 0"]
    middle = len(items) // 2
    return ([prefix + f"if lt-u64 h {items[middle][0]} then"] + tree(items[:middle], indent + 2)
            + [prefix + "else"] + tree(items[middle:], indent + 2))


def generated() -> str:
    """Create the complete private lookup using ordinary machine-word function boundaries."""
    items = records()
    groups = [items[i:i + 64] for i in range(0, len(items), 64)]
    lines = [MARKER.rstrip(), "  -- Input data and licensing: tooling/data/html-entities.json and html-entities-PYTHON-LICENSE.txt.",
             "  entity-lookup :: U64 -> U64 -> U64 -> U64 -> U64 -> U64",
             "  entity-lookup a b c d n = entity-route (entity-name-hash a b c d n) a b c d n", "",
             "  -- | Route a complete packed name through a bounded balanced machine-word search.",
             "  entity-route :: U64 -> U64 -> U64 -> U64 -> U64 -> U64 -> U64",
             "  entity-route h a b c d n ="]

    def route(start: int, end: int, indent: int) -> list[str]:
        """Render branch routing without allocating an index or constructing a table."""
        prefix = " " * indent
        if end - start == 1:
            return [prefix + f"entity-bucket-{suffix(start)} h a b c d n"]
        middle = (start + end) // 2
        return ([prefix + f"if lt-u64 h {groups[middle][0][0]} then"] + route(start, middle, indent + 2)
                + [prefix + "else"] + route(middle, end, indent + 2))

    lines.extend(route(0, len(groups), 4))
    for index, group in enumerate(groups):
        name = f"entity-bucket-{suffix(index)}"
        lines += ["", f"  -- | Match one bounded group of {len(group)} complete packed HTML names.",
                  f"  {name} :: U64 -> U64 -> U64 -> U64 -> U64 -> U64 -> U64",
                  f"  {name} h a b c d n ="]
        lines.extend(tree(group, 4))
    return "\n".join(lines) + "\n"


def main() -> None:
    """Replace only generated declarations, or fail without writing when the suffix is stale."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--module", type=Path, default=ROOT / "compiler/prelude/Text/Html.cinch")
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    existing = args.module.read_text()
    if existing.count(MARKER) > 1:
        parser.error("duplicate generated entity lookup markers")
    expected = existing.split(MARKER, 1)[0].rstrip() + "\n\n" + generated()
    if args.check:
        if existing != expected:
            parser.exit(1, f"Generated HTML entity lookup is stale: {args.module}\n")
    elif existing != expected:
        args.module.write_text(expected)


if __name__ == "__main__":
    main()
