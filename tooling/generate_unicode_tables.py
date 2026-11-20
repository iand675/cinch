#!/usr/bin/env python3
"""Generate allocation-free Unicode 17 classification, ranges and simple-case tables.

Ordinary bounded machine-scalar case pages lower to private read-only lookup data
at optimization level two. They preserve simple semantics at level zero without
large vector spills, raw String access or giant constructor-list evaluation.
Source facts, original licensing and hashes live in unicode17-tables.json.
"""
from __future__ import annotations
import argparse
from array import array
import json
from prepare_unicode_tables_candidate import local_words
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
DATA = ROOT / "tooling/data/unicode17-tables.json"
PAGE = 64
MAX_CODE = 0x10FFFF
CASE_BIAS = 0x110000


def suffix(index: int) -> str:
    """Give arbitrary page indices stable alphabetic identifier suffixes."""
    result = ""
    index += 1
    while index:
        index, digit = divmod(index - 1, 26)
        result = chr(97 + digit) + result
    return result


def table_points(table: dict):
    """Yield exactly the strided members, including nonscalar Unicode code points."""
    for low, high, stride in table["r16"] + table["r32"]:
        yield from range(low, high + 1, stride)


def model(data: dict) -> dict:
    """Build a compact combined class database and preserve every original range observation."""
    tables, catalogs = data["tables"], data["catalogs"]
    categories = sorted(name for name in catalogs["Categories"] if len(name) == 2 and name != "LC")
    assert len(categories) == 30
    scripts = sorted(catalogs["Scripts"])
    properties = sorted(catalogs["Properties"])
    folds = [(kind, name) for kind in ["FoldCategory", "FoldScript"] for name in sorted(catalogs[kind])]
    assert len(scripts) <= 255 and len(properties) + len(folds) <= 51
    default = categories.index("Cn") | 255 << 5
    classes = array("Q", [default]) * (MAX_CODE + 1)
    assigned = bytearray(MAX_CODE + 1)
    for index, name in enumerate(categories):
        for cp in table_points(tables[catalogs["Categories"][name]]):
            assert not assigned[cp], (name, cp)
            assigned[cp] = 1
            classes[cp] = (classes[cp] & ~31) | index
    assert all(assigned), "general categories must partition the complete code-point domain"
    for index, name in enumerate(scripts):
        for cp in table_points(tables[catalogs["Scripts"][name]]):
            classes[cp] = (classes[cp] & ~(255 << 5)) | index << 5
    for index, name in enumerate(properties):
        for cp in table_points(tables[catalogs["Properties"][name]]):
            classes[cp] |= 1 << (13 + index)
    for index, (kind, name) in enumerate(folds):
        for cp in table_points(tables[catalogs[kind][name]]):
            classes[cp] |= 1 << (13 + len(properties) + index)
    assert max(classes) < 1 << 64
    metadata, metadata_ids = [default], {default: 0}
    directory, runs, pages = [], [], {}
    for start in range(0, MAX_CODE + 1, 256):
        page, previous = [], None
        for low in range(256):
            value = classes[start + low]
            if value not in metadata_ids:
                metadata_ids[value] = len(metadata)
                metadata.append(value)
            index = metadata_ids[value]
            if index != previous:
                page.append((low, index))
                previous = index
        encoded = tuple(((page[i + 1][0] - 1) if i + 1 < len(page) else 255) | index << 8 for i, (low, index) in enumerate(page))
        if encoded not in pages:
            pages[encoded] = (len(runs), len(encoded))
            runs.extend(encoded)
        offset, count = pages[encoded]
        assert count <= 256 and offset < 1 << 23
        directory.append(offset << 9 | count)
    assert len(metadata) <= 65536 and max(runs) < 1 << 24
    names = sorted(tables)
    range_words, info, descriptors = [], [], []
    for name in names:
        table = tables[name]
        offset, n16, n32, latin = len(range_words), len(table["r16"]), len(table["r32"]), table["latin_offset"]
        assert offset < 1 << 32 and n16 < 1024 and n32 < 1024 and latin < 4096
        info.append(offset | n16 << 32 | n32 << 42 | latin << 52)
        for low, high, stride in table["r16"] + table["r32"]:
            assert max(low, high, stride) < 1 << 21
            range_words.append(low | high << 21 | stride << 42)
        descriptor = None
        for key, target in catalogs["Categories"].items():
            if target == name:
                mask = sum(1 << index for index, primary in enumerate(categories) if primary == key or len(key) == 1 and primary.startswith(key) or key == "LC" and primary in ["Ll", "Lt", "Lu"])
                descriptor = mask
                break
        if descriptor is None:
            for index, key in enumerate(scripts):
                if catalogs["Scripts"][key] == name:
                    descriptor = 1 << 60 | index
                    break
        if descriptor is None:
            for index, key in enumerate(properties):
                if catalogs["Properties"][key] == name:
                    descriptor = 2 << 60 | (13 + index)
                    break
        if descriptor is None:
            for index, (kind, key) in enumerate(folds):
                if catalogs[kind][key] == name:
                    descriptor = 2 << 60 | (13 + len(properties) + index)
                    break
        assert descriptor is not None, name
        descriptors.append(descriptor)
    case_bounds, case_deltas = [], []
    for low, high, upper, lower, title in data["case_ranges"]:
        assert all(0 <= delta + CASE_BIAS < 1 << 22 for delta in [upper, lower, title])
        case_bounds.append(low | high << 21 | (title + CASE_BIAS) << 42)
        case_deltas.append((upper + CASE_BIAS) | (lower + CASE_BIAS) << 22)
    orbit = [first | last << 21 for first, last in data["fold_orbit"]]
    locale_bounds, locale_deltas = [], []
    for low, high, upper, lower, title in data["special_cases"]["Turkish"]:
        locale_bounds.append(low | high << 21 | (title + CASE_BIAS) << 42)
        locale_deltas.append((upper + CASE_BIAS) | (lower + CASE_BIAS) << 22)
    category_masks = {name: sum(1 << index for index, primary in enumerate(categories) if primary == name or len(name) == 1 and primary.startswith(name) or name == "LC" and primary in ["Ll", "Lt", "Lu"]) for name in catalogs["Categories"]}
    return {"categories": categories, "scripts": scripts, "properties": properties, "folds": folds, "table_names": names, "table_ids": {name: index for index, name in enumerate(names)}, "category_masks": category_masks, "counts": {"code_points": len(classes), "distinct_metadata": len(metadata), "unique_pages": len(pages), "page_directory": len(directory), "class_runs": len(runs), "range_records": len(range_words), "tables": len(names), "case_ranges": len(case_bounds), "fold_orbit": len(orbit)}, "words": {"class-directory": (32, directory), "class-runs": (32, runs), "class-metadata": (64, metadata), "table-info": (64, info), "table-descriptor": (64, descriptors), "range-records": (64, range_words), "case-bounds": (64, case_bounds), "case-deltas": (64, case_deltas), "fold-orbit": (64, orbit), "ascii-fold": (32, data["ascii_fold"]), "turkic-bounds": (64, locale_bounds), "turkic-deltas": (64, locale_deltas)}}


def render_words(name: str, width: int, words: list[int]) -> str:
    """Render shallow ordinary scalar pages and a balanced machine-only page router."""
    if not words:
        raise ValueError(f"empty generated word table: {name}")
    prefix = "unicode-data-" + name
    pages = [words[start:start + PAGE] for start in range(0, len(words), PAGE)]
    out = [f"  -- | Read one immutable {name} entry; an out-of-range index returns zero.", f"  {prefix} :: U32 -> U{width}", f"  {prefix} index = if lt-u32 index {len(words)} then", "    let page = wrapping-shr-u32 index 6 in", f"    {prefix}-route page index", "    else 0", "", f"  -- | Route a bounded {name} index without constructing a runtime table.", f"  {prefix}-route :: U32 -> U32 -> U{width}", f"  {prefix}-route page index ="]

    def route(start: int, end: int, indent: int) -> None:
        """Select an ordinary constant page with logarithmic bounded routing depth."""
        pad = " " * indent
        if end - start == 1:
            out.extend([pad + f"let local = wrapping-sub-u32 index {start * PAGE} in", pad + f"{prefix}-page-{suffix(start)} local"])
        else:
            middle = (start + end) // 2
            out.append(pad + f"if lt-u32 page {middle} then")
            route(start, middle, indent + 2)
            out.append(pad + "else")
            route(middle, end, indent + 2)

    route(0, len(pages), 4)
    for index, page in enumerate(pages):
        out.extend(["", f"  -- | Immutable {name} page {index}; scalar cases permit constant-data lowering without vector spills.", f"  {prefix}-page-{suffix(index)} :: U32 -> U{width}", f"  {prefix}-page-{suffix(index)} index = case index of"])
        out.extend(f"    {offset} -> {value}" for offset, value in enumerate(page))
        out.append("    other -> 0")
    return "\n".join(out) + "\n"


def generated(database: dict, inline_class_ops: bool = True) -> str:
    """Emit the complete numeric-only table module, preserving String privacy and import acyclicity."""
    counts = database["counts"]
    out = ["-- | Unicode 17.0.0 immutable classification and simple-case source data.", "-- Generated by tooling/generate_unicode_tables.py. Provenance and original", "-- licensing: tooling/data/unicode17-tables.json and unicode17-BSD-LICENSE.txt.", "-- Ordinary bounded scalar pages lower to private read-only tables; no backing", "-- pointer, mutable buffer or runtime full-table copy enters the public API.", "module Data.Unicode.Tables where", "  import Data.Machine", ""]
    for start in range(0, len(database["words"]), 4):
        names = list(database["words"])[start:start + 4]
        out.append("  export " + ", ".join("unicode-data-" + name for name in names))
    out.append("  export unicode-data-class, unicode-data-case-count, unicode-data-fold-count, unicode-data-table-count")
    out.extend(["", "  -- | Number of complete preserved simple-case range records.", "  unicode-data-case-count :: U32", f"  unicode-data-case-count = {counts['case_ranges']}", "", "  -- | Number of exceptional simple-fold cycle edges.", "  unicode-data-fold-count :: U32", f"  unicode-data-fold-count = {counts['fold_orbit']}", "", "  -- | Number of distinct named immutable range tables.", "  unicode-data-table-count :: U32", f"  unicode-data-table-count = {counts['tables']}", "", "  -- | Find the first complete class run covering a low-byte code-point position.", "  class-search :: U32 -> U32 -> U32 -> U32", "  class-search low high byte = if lt-u32 low high then", "    let middle = wrapping-shr-u32 (wrapping-add-u32 low high) 1 in", "    let word = unicode-data-class-runs middle in", "    let end = bit-and-u32 word 255 in", "    if le-u32 byte end then class-search low middle byte", "    else class-search (wrapping-add-u32 middle 1) high byte", "    else low", "", "  -- | Read one complete code-point class word without allocation; values beyond U+10FFFF return zero.", "  unicode-data-class :: U32 -> U64", "  unicode-data-class cp = if le-u32 cp 1114111 then", "    let page = wrapping-shr-u32 cp 8 in let low = bit-and-u32 cp 255 in", "    let info = unicode-data-class-directory page in", "    let start = wrapping-shr-u32 info 9 in let count = bit-and-u32 info 511 in", "    let index = class-search start (wrapping-add-u32 start count) low in", "    let run = unicode-data-class-runs index in let metadata = wrapping-shr-u32 run 8 in", "    unicode-data-class-metadata metadata", "    else 0", ""])
    for name, (width, words) in database["words"].items():
        out.append(render_words(name, width, words))
    source = "\n".join(out)
    return local_words(source, class_only=True) if inline_class_ops else source


def main() -> None:
    """Generate maintained tables and a compact interpretation manifest from the checked snapshot."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--data", type=Path, default=DATA)
    parser.add_argument("--module", type=Path, default=ROOT / "seed/prelude/Data/Unicode/Tables.cinch")
    parser.add_argument("--manifest", type=Path, default=ROOT / "tooling/data/unicode17-layout.json")
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    database = model(json.loads(args.data.read_text()))
    text = generated(database)
    manifest = json.dumps({key: value for key, value in database.items() if key != "words"}, sort_keys=True, indent=2) + "\n"
    if args.check:
        if not args.module.exists() or args.module.read_text() != text or not args.manifest.exists() or args.manifest.read_text() != manifest:
            raise SystemExit("stale generated Unicode tables or layout manifest")
    else:
        args.module.parent.mkdir(parents=True, exist_ok=True)
        args.module.write_text(text)
        args.manifest.parent.mkdir(parents=True, exist_ok=True)
        args.manifest.write_text(manifest)
    print(json.dumps(database["counts"], sort_keys=True), flush=True)


if __name__ == "__main__":
    main()
