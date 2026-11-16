#!/usr/bin/env python3
"""Measure inferred borrowing and ownership sites in an owned Core dump.

    CINCH_BORROW_REPORT=1 compiler/target/release/cinch core emit \\
        build/bootstrap/stage1/compiler.cinch --stage=owned \\
        -o /tmp/compiler.owned.core 2>/tmp/compiler.borrow.log
    python3 tooling/borrow_census.py /tmp/compiler.owned.core \\
        --report /tmp/compiler.borrow.log --by-module

Use the whole bootstrap bundle: individual bootstrap modules lack their
prelude imports. @borrow-modes is the inference's answer; @borrowed is the
primitive calling convention, and @obj is only a representation class.
An ownership site is a dup, drop or release, counted once in the Core text.
The report contains one witness per counted parameter. Select the complete
pass with the most matching dumped formals, preferring the first on ties:
dependency builds can precede the dump, and later passes see optimized code.
Reports cover counted formals of ordinary Core functions, excluding
primitive bodies. Only units present in the dump enter the breakdown.
Counts describe this dump, not dynamic operation counts or scaling.
"""

import argparse
import collections
import json
from pathlib import Path
import re

TOKEN = re.compile(r'''"(?:\\.|[^"\\])*"|'(?:''|[^'])*'|[()]|[^()\s]+''')


def unquote(name):
    return name[1:-1].replace("''", "'") if name.startswith("'") and name.endswith("'") else name


def ownership_sites(unit):
    """Read actual node heads, ignoring strings and either mark layout."""
    body = re.search(r"(?m)^  body\s+", unit)
    if not body:
        return []
    tokens = TOKEN.findall(unit[body.end():])
    sites = []
    for i in range(len(tokens) - 1):
        if tokens[i] != "(" or tokens[i + 1] not in {"dup", "drop", "release"}:
            continue
        at = i + 2
        # Wrapped forms print `(drop @class(obj) x ...)`; inline forms
        # print `(drop x @class(obj) ...)`. Marks may have nested arguments.
        while at < len(tokens) and tokens[at].startswith("@"):
            at += 1
            if at < len(tokens) and tokens[at] == "(":
                depth = 1
                at += 1
                while at < len(tokens) and depth:
                    depth += (tokens[at] == "(") - (tokens[at] == ")")
                    at += 1
        if at >= len(tokens) or tokens[at] in {"(", ")"}:
            raise ValueError(f"{tokens[i + 1]} has no operand")
        sites.append(unquote(tokens[at]))
    return sites


def units(text):
    return re.split(r"(?m)^unit ", text)[1:]


def unit_name(unit):
    name = unit.split(" ::")[0].strip()
    return unquote(name)


def census(text):
    total = on_params = params = counted = borrowed = 0
    by_module = collections.Counter()
    borrowed_units = []
    for unit in units(text):
        name = unit_name(unit)
        declared = set()
        header = re.search(r"@params (.*)", unit)
        if header:
            for part in re.split(r",\s*(?=(?:[A-Za-z0-9_.$-]+|'(?:''|[^'])*')\s+::)", header.group(1)):
                entry = part.strip()
                if not entry:
                    continue
                declared.add(unquote(TOKEN.match(entry).group()))
                params += 1
                counted += bool(re.search(r"@(?:obj|owned)\b", entry))
        inferred = re.search(r"@borrow-modes\(([^)]*)\)", unit)
        if inferred:
            borrowed += len([p for p in inferred.group(1).split(",") if p.strip()])
            borrowed_units.append(name)
        sites = ownership_sites(unit)
        hit = sum(site in declared for site in sites)
        total += len(sites)
        on_params += hit
        by_module[".".join(name.split(".")[:2])] += hit
    return {
        "units": len(units(text)), "params": params, "counted": counted,
        "borrowed": borrowed, "borrowed_units": borrowed_units,
        "sites": total, "on_params": on_params, "by_module": by_module,
    }


def refusal_report(text, names=None):
    """Select one complete pass, without combining inference passes."""
    rows = None
    passes = []
    for line in text.splitlines():
        if not line.startswith("cinch-borrow: "):
            continue
        row = json.loads(line.removeprefix("cinch-borrow: "))
        if row.get("event") == "begin":
            rows = []
        elif row.get("event") == "end":
            if rows is not None:
                passes.append(rows)
                rows = None
        elif rows is not None:
            rows.append(row)
    if not passes:
        raise ValueError("no complete CINCH_BORROW_REPORT pass found")
    if names is None:
        return passes[0]
    matching = [[row for row in rows if row["unit"] in names] for rows in passes]
    selected = max(matching, key=lambda rows: len({(row["unit"], row["param"]) for row in rows}))
    if not selected:
        raise ValueError("no inference pass matches this Core dump")
    return selected


def main():
    parser = argparse.ArgumentParser(description=__doc__,
                                     formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("core", help="a *.owned.core dump")
    parser.add_argument("--report", help="stderr from CINCH_BORROW_REPORT=1")
    parser.add_argument("--by-module", action="store_true", help="per-module breakdown")
    parser.add_argument("--top", type=int, default=10, help="modules to show")
    parser.add_argument("--json", action="store_true", help="machine-readable census")
    args = parser.parse_args()
    core = Path(args.core).read_text()
    result = census(core)
    if args.report:
        names = {unit_name(unit) for unit in units(core)}
        rows = refusal_report(Path(args.report).read_text(), names)
        result["reasons"] = collections.Counter(row["reason"] for row in rows)
        result["owned_uses"] = collections.Counter(row["use"] for row in rows
                                                 if row["reason"] == "owned-use" and "use" in row)
    if args.json:
        print(json.dumps(result, sort_keys=True))
        return 0
    share = 100 * result["on_params"] / result["sites"] if result["sites"] else 0.0
    print(f"units {result['units']}, parameters {result['params']} "
          f"(counted {result['counted']}, inferred borrowed {result['borrowed']}, "
          f"units lending {len(result['borrowed_units'])})")
    print(f"ownership sites {result['sites']}, on parameters {result['on_params']} ({share:.1f}%)")
    if "reasons" in result:
        for reason, count in result["reasons"].most_common():
            print(f"  {reason:28} {count}")
        for use, count in result["owned_uses"].most_common():
            print(f"    owned-use/{use:18} {count}")
    if args.by_module:
        print()
        for module, count in result["by_module"].most_common(args.top):
            print(f"  {module:28} {count}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
