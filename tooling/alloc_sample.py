#!/usr/bin/env python3
"""Read a macOS `sample` call graph and report phase buckets and alloc callers."""
import re
import sys
from collections import Counter

FRAME = re.compile(r"^([ +!:|]*)(\d+) (\S+)  \(in ")
CODE = re.compile(r"(\d+)_")


def demangle(name):
    # `cinch.typed.m.77_97_105_110_.rest` carries module names as char codes.
    return CODE.sub(lambda m: chr(int(m.group(1))) if 32 <= int(m.group(1)) < 127 else "?", name)


def frames(path):
    out = []
    for line in open(path):
        m = FRAME.match(line)
        if not m:
            continue
        depth = len(m.group(1).replace(" ", ""))
        out.append((depth, int(m.group(2)), demangle(m.group(3))))
    return out


def inclusive(fs, match):
    total = 0
    for i, (depth, count, name) in enumerate(fs):
        if not match(name):
            continue
        nested = False
        for j in range(i - 1, -1, -1):
            d, _, n = fs[j]
            if d < depth:
                if match(n):
                    nested = True
                break
        if not nested:
            total += count
    return total


def callers(fs, match, limit=10):
    seen = Counter()
    for i, (depth, count, name) in enumerate(fs):
        if not match(name):
            continue
        for j in range(i - 1, -1, -1):
            d, _, n = fs[j]
            if d < depth:
                seen[n] += count
                break
    return seen.most_common(limit)


PHASES = [
    ("main", lambda n: n == "main"),
    ("read+load inputs", lambda n: ".Inputs." in n and ("read-interfaces" in n or "load-interface-references" in n)),
    ("load-interface-set", lambda n: n.endswith("load-interface-set")),
    ("parse-json", lambda n: n.endswith("parse-json")),
    ("canonical-bytes", lambda n: n.endswith("canonical-bytes") or n.endswith("emit-value") or n.endswith("emit-entries")),
    ("resolve-program", lambda n: n.endswith("resolve-program")),
    ("elaborate-program", lambda n: n.endswith("elaborate-program")),
    ("elaborate-module (infer)", lambda n: n.endswith("elaborate-module")),
    ("check-definition", lambda n: "check-definition" in n),
    ("export-interfaces", lambda n: n.endswith("export-interfaces")),
    ("emit-runtime-program", lambda n: "emit-runtime-program" in n or "emit-runtime" in n),
    ("check-modules", lambda n: n.endswith("check-modules")),
    ("set-bodies", lambda n: n.endswith("set-bodies")),
    ("index-graph-bodies", lambda n: n.endswith("index-graph-bodies")),
    ("export-entries", lambda n: n.endswith("export-entries")),
    ("read-values", lambda n: n.endswith("read-values")),
]

ALLOCS = [
    ("cinch_alloc_ctor", lambda n: n == "cinch_alloc_ctor"),
    ("cinch_alloc_shaped", lambda n: n == "cinch_alloc_shaped"),
    ("cinch_alloc_raw", lambda n: n == "cinch_alloc_raw"),
    ("cinch_alloc (any)", lambda n: n.startswith("cinch_alloc") or n.startswith("__RNv") and "allocate_block" in n),
]

if __name__ == "__main__":
    path = sys.argv[1]
    fs = frames(path)
    total = fs[0][1] if fs else 0
    print(f"samples: {total}  frames: {len(fs)}")
    print()
    print(f"{'phase':<24}{'inclusive':>10}{'percent':>9}")
    for label, match in PHASES:
        n = inclusive(fs, match)
        print(f"{label:<24}{n:>10}{100.0 * n / total:>8.1f}%")
    print()
    for label, match in ALLOCS:
        print(f"{label}: {inclusive(fs, match)}")
        for name, count in callers(fs, match):
            print(f"    {count:>6}  {name}")
