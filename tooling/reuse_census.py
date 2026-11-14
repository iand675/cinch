#!/usr/bin/env python3
"""Census the Perceus reuse the compiler's own code collects, and misses.

usage: reuse_census.py <owned-core-dir> [--by-module] [--why] [--by-host]

`<owned-core-dir>` holds `*.owned.core` files, the `--dump-core owned` output
of `cinch emit-ir <module>.cinch --dump-core owned` for each `bootstrap`
module. `check` never reaches the owned stage, so `emit-ir` is the command;
it writes the dump before refusing for want of a `main`.

For every `(release ...)` site the report classifies the outcome the way
`compiler/src/rc.rs` decides it:

  reused        the release carries `@token(`, so `insert_reuse` found a
                compatible allocation on the arm's spine
  opportunity   a constructor allocation exists in the arm but off the
                spine, so the search never reached it. This is the only
                bucket a pass change can win.
  no allocation nothing in the arm to reuse the dying cell into
  incompatible  an allocation is on the spine but its shape does not match
                the dying block (field count, or a planned boundary layout)

`--why` breaks the opportunity bucket down by the node that stopped the
spine walk (a `case`, an `if`, a call), which is the actionable part.

`--by-host` splits the collected pairs by the construct that hosts the
reuse: `spine` for a pair on the release body's straight line, and `case`
or `if` for one inside an arm. It also splits each host's pairs into the
ones that skip a store (`@kept` is non-empty: the cell already holds those
payload words) and the ones that write the whole payload.

The spine the walk follows is `insert_reuse`'s: a `Let` value, a `Let` body,
a `Dup` body, a `Drop` body, a call's arguments in order, and a jump's
arguments in order. Anything else ends the search, because a reuse under a
branch could be skipped and the kept cell would leak.

Numbers from the 2026-11-13 run over all 44 `bootstrap/*.cinch` modules
(5,927 units) are in `design/linear-in-place.md` section 5. The 2026-11-14
run, which added the call-argument and branch-arm positions and the host
split, is in `design/compilation-speed.md` section 2.21.
"""
import argparse
import collections
import pathlib
import re
import sys

sys.setrecursionlimit(300000)


def parse(text, i):
    """Parse one s-expression at `i`; return (node, next index)."""
    while i < len(text) and text[i] in " \n\t\r":
        i += 1
    if i >= len(text):
        return None, i
    if text[i] == "(":
        i += 1
        items = []
        while True:
            while i < len(text) and text[i] in " \n\t\r":
                i += 1
            if i >= len(text):
                return items, i
            if text[i] == ")":
                return items, i + 1
            node, i = parse(text, i)
            if node is None:
                return items, i
            items.append(node)
    j = i
    while j < len(text) and text[j] not in " \n\t\r()":
        j += 1
    return text[i:j], j


def head(n):
    return n[0] if isinstance(n, list) and n and isinstance(n[0], str) else None


def releases(n, out):
    """Every `release` node in `n`, in order."""
    if isinstance(n, list):
        if head(n) == "release":
            out.append(n)
        for c in n:
            releases(c, out)
    return out


def has_ctor(n):
    """Whether `n` allocates any constructor anywhere."""
    if isinstance(n, list):
        if head(n) == "app" and len(n) >= 2 and head(n[1]) == "ctor":
            return True
        return any(has_ctor(c) for c in n)
    return False


def spine(e):
    """Walk `insert_reuse`'s spine; return ('alloc', node) or ('blocked', node).

    This mirrors `rc.rs insert_reuse` exactly, including the two positions
    that were added after the 2026-11-13 census: a call's arguments
    (`insert_reuse_in_args`, in order) and a jump's arguments. The head of
    a call is not searched, and a branch is never entered, so a `case` or
    an `if` is still where the walk stops.
    """
    h = head(e)
    if h == "let" and len(e) >= 5:
        kind, node = spine(e[3])
        if kind == "alloc":
            return kind, node
        return spine(e[4])
    if h in ("dup", "drop"):
        return spine(e[-1])
    if h == "app" and len(e) >= 2 and head(e[1]) == "ctor":
        return "alloc", e
    if h in ("app", "jump"):
        # `(app head arg...)` and `(jump id arg...)`: every argument is
        # evaluated before the transfer, so each is on the spine. The
        # call's head is not searched: it is a unit, a primitive or a
        # variable in every shape that reaches here.
        for arg in e[2:]:
            kind, node = spine(arg)
            if kind == "alloc":
                return kind, node
        return "blocked", e
    return "blocked", e


def reuses(n, branches=(), out=None):
    """Every `reuse` under `n` with the branch constructs that enclose it.

    `branches` is the chain of `case`/`if` heads on the path from the
    release body down to this node, outermost first, so the innermost
    entry is the construct that hosts the branch the reuse sits in. A
    `reuse` on the release body's straight-line spine has an empty chain.

    The walk stops at a nested `release`: that node's reuses belong to its
    own token, and `insert_reuse` cannot see past it either (it is not a
    spine position). A `reuse` found under a `case` or `if` is always in an
    arm, because `insert_reuse` stops at the branch and never enters a
    scrutinee or a condition.
    """
    if out is None:
        out = []
    if isinstance(n, list):
        h = head(n)
        if h == "reuse":
            out.append((branches, n))
        elif h == "release":
            return out
        else:
            inner = branches + (h,) if h in ("case", "if") else branches
            for c in n:
                reuses(c, inner, out)
    return out


def kept_slots(reuse):
    """The payload slots a `reuse` node skips a store for.

    A mark prints as two atoms, its name and its bracketed payload
    (`@kept` then `(0)`), so the count is the payload list beside the
    `@kept` atom. An empty mark means every word is written: the pair
    consumes the token and stores the whole payload.
    """
    for i, x in enumerate(reuse):
        if x == "@kept" and i + 1 < len(reuse) and isinstance(reuse[i + 1], list):
            return len(reuse[i + 1])
    return 0


def classify(release):
    """(outcome, blocker) for one release node."""
    if any(isinstance(x, str) and x.startswith("@token") for x in release):
        return "reused", None
    kind, node = spine(release[-1])
    if kind == "alloc":
        return "incompatible", None
    blocker = head(node) or "atom"
    if has_ctor(node):
        return "opportunity", blocker
    return "no allocation", blocker


def units_of(text):
    """Each unit's parsed body, skipping the file header and unparsable bodies."""
    start = text.find("unit ")
    if start < 0:
        return
    for chunk in re.split(r"(?m)^unit ", text[start:])[1:]:
        body = chunk.find("body ")
        if body < 0:
            continue
        try:
            tree, _ = parse(chunk[body + 5:], 0)
        except RecursionError:
            continue
        yield chunk.split("\n", 1)[0].strip(), tree


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("core_dir", type=pathlib.Path)
    ap.add_argument("--by-module", action="store_true", help="one row per core file")
    ap.add_argument("--why", action="store_true", help="break opportunities down by blocker")
    ap.add_argument("--by-host", action="store_true",
                    help="split the collected pairs by the construct that hosts the reuse")
    args = ap.parse_args()

    files = sorted(args.core_dir.glob("*.owned.core"))
    if not files:
        raise SystemExit(f"{args.core_dir}: no *.owned.core files (run `cinch emit-ir <module> --dump-core owned`)")

    total = collections.Counter()
    blockers = collections.Counter()
    # The host of a collected pair: `spine` for a reuse on the release
    # body's straight line, else the innermost `case` or `if` whose arm
    # holds it. `skipped` counts the pairs whose `@kept` mark is non-empty,
    # which is the store-skipping half of the extension; `stored` counts
    # the ones that write the whole payload.
    hosts = collections.Counter()
    skipped = collections.Counter()
    rows = []
    units = 0
    for path in files:
        counts = collections.Counter()
        for _, tree in units_of(path.read_text()):
            units += 1
            for release in releases(tree, []):
                outcome, blocker = classify(release)
                counts[outcome] += 1
                if outcome == "opportunity":
                    blockers[blocker] += 1
                if outcome == "reused":
                    for chain, node in reuses(release[-1]):
                        host = chain[-1] if chain else "spine"
                        hosts[host] += 1
                        skipped[host] += 1 if kept_slots(node) else 0
        rows.append((path.name.removesuffix(".owned.core"), counts))
        total.update(counts)

    grand = sum(total.values())
    if args.by_module:
        print(f"{'module':34} {'release':>8} {'reused':>7} {'opp':>6} {'none':>6} {'incomp':>7}")
        for name, c in rows:
            if sum(c.values()):
                print(f"{name:34} {sum(c.values()):8} {c['reused']:7} {c['opportunity']:6} "
                      f"{c['no allocation']:6} {c['incompatible']:7}")
        print("-" * 72)

    print(f"units: {units}")
    print(f"{'outcome':22} {'count':>8} {'share':>8}")
    for outcome in ("reused", "opportunity", "no allocation", "incompatible"):
        n = total[outcome]
        print(f"{outcome:22} {n:8} {100 * n / grand:7.1f}%")
    print(f"{'total':22} {grand:8}")

    if args.why:
        print()
        print("opportunities by the node that stopped the spine walk:")
        for blocker, n in blockers.most_common():
            print(f"  {blocker:20} {n:7} {100 * n / max(total['opportunity'], 1):6.1f}%")

    if args.by_host:
        pairs = sum(hosts.values())
        print()
        print("collected pairs by the construct that hosts the reuse:")
        print(f"  {'host':12} {'pairs':>7} {'share':>8} {'skip':>7} {'store':>7}")
        for host in sorted(hosts, key=lambda h: (-hosts[h], h)):
            n = hosts[host]
            print(f"  {host:12} {n:7} {100 * n / max(pairs, 1):7.1f}% "
                  f"{skipped[host]:7} {n - skipped[host]:7}")
        print(f"  {'total':12} {pairs:7} {100.0 if pairs else 0:7.1f}% "
              f"{sum(skipped.values()):7} {pairs - sum(skipped.values()):7}")


if __name__ == "__main__":
    main()
