#!/usr/bin/env python3
"""Read a macOS `sample` call graph and attribute its samples.

usage: alloc_sample.py <sample.txt> [phase ...]

Prints the inclusive sample count under each phase frame, and, for every
sample that lands inside the runtime's allocator, the nearest caller outside
it: the Cinch function that asked for the block. Names carry module text as
`\\d+_` character codes (the code generator's spelling), decoded here.
"""
import re
import sys
from collections import Counter

FRAME = re.compile(r"^(.*?)(\d+) (\S+)  \(in ")
CODE = re.compile(r"(\d+)_")

PHASES = {
    "load inputs": ("read-interfaces", "load-interface-references", "read-files"),
    "load interface set": ("load-interface-set", "load-checked-interface-set-from-contracts",
                           "load-interface-set-from-contracts", "read-set", "read-contract-set",
                           "load-interface", "load-interface-from-contract",
                           "check-contract-envelope", "envelope-valid", "validate-interface-set"),
    "  parse-json": ("parse-json",),
    "  contract identity": ("fnv-source-hash", "fnv-step", "canonical-bytes", "emit-value"),
    "  export rows": ("loaded-exports", "export-entries", "export-rows", "loaded-identities",
                      "public-global", "name-rows-table", "value-exports"),
    "  set bodies": ("set-bodies", "index-graph-bodies", "check-total-graph"),
    "resolve": ("resolve-program", "resolve-program-with", "resolve-context", "check-source-phase",
                "interface-scopes", "interface-declarations", "materialize-externals"),
    "elaborate": ("elaborate-program", "elaborate-modules", "elaborate-module",
                  "elaborate-module-with-source-bodies", "source-total-bodies"),
    "export interfaces": ("export-interfaces", "export-runtime"),
    "runtime ir": ("emit-runtime-program", "emit-runtime-program-counting", "emit-runtime"),
}

# The allocator's own frames: a sample underneath one of these is charged to
# the nearest frame above them.
ALLOCATOR = ("allocate_block", "allocate_shaped", "allocate", "place", "place_system",
             "cinch_alloc", "cinch_alloc_ctor", "cinch_alloc_shaped", "cinch_alloc_raw",
             "cinch_free", "cinch_bytes_new", "cinch_drop", "cinch_drop_slow", "cinch_dup",
             "cinch_mark_mt", "cinch_nat_", "cinch_int_", "cinch_sym_", "cinch_text_",
             "release", "account", "alloc_block", "alloc", "dealloc", "malloc", "free",
             "memcpy", "memmove", "memset", "nanov2_", "szone_", "tiny_", "small_",
             "Counting", "cinchruntime", "cinch_bytes_", "cinch_io_", "cinch_buf_",
             "__rust", "_RNv", "abort", "posix_memalign")


def demangle(name):
    return CODE.sub(lambda m: chr(int(m.group(1))) if 32 <= int(m.group(1)) < 127 else "?", name)


def base(name):
    return demangle(name).split(".")[-1].split("@")[0]


def frames(path):
    out = []
    for line in open(path):
        m = FRAME.match(line)
        if not m:
            continue
        depth = (len(m.group(1)) - 6) // 2
        out.append((depth, int(m.group(2)), demangle(m.group(3))))
    parents = [None] * len(out)
    stack = []
    for i, (depth, _, _) in enumerate(out):
        while stack and out[stack[-1]][0] >= depth:
            stack.pop()
        parents[i] = stack[-1] if stack else None
        stack.append(i)
    return out, parents


def leaves(rows):
    return [i for i in range(len(rows)) if i + 1 == len(rows) or rows[i + 1][0] <= rows[i][0]]


def allocation_callers(rows, parents, picked, limit):
    seen = Counter()
    for i in picked:
        chain = []
        j = parents[i]
        while j is not None:
            chain.append(base(rows[j][2]))
            j = parents[j]
        # The innermost frame outside the allocator and its runtime helpers.
        for name in chain:
            if not name.startswith(ALLOCATOR) and not name.endswith("__ulock_wait"):
                seen[name] += rows[i][1]
                break
    return seen.most_common(limit)


def main():
    path = sys.argv[1]
    rows, parents = frames(path)
    picked = [i for i in leaves(rows) if rows[i][0] > 0]
    total = sum(rows[i][1] for i in picked)
    print(f"{path}: {total} samples on working threads")
    print()
    print(f"{'phase':<24}{'samples':>9}{'share':>8}")
    for label, names in PHASES.items():
        n = 0
        for i in picked:
            j = parents[i]
            while j is not None:
                if base(rows[j][2]) in names:
                    n += rows[i][1]
                    break
                j = parents[j]
        print(f"{label:<24}{n:>9}{100.0 * n / total:>7.1f}%")
    print()
    allocator = [i for i in picked
                 if base(rows[i][2]).startswith(ALLOCATOR) or (parents[i] is not None
                 and base(rows[parents[i]][2]).startswith(ALLOCATOR))]
    n = sum(rows[i][1] for i in allocator)
    print(f"samples inside the allocator or a runtime box: {n}")
    print(f"{'caller':<58}{'samples':>8}")
    for name, count in allocation_callers(rows, parents, allocator, 10):
        print(f"{name[:58]:<58}{count:>8}")


if __name__ == "__main__":
    main()
