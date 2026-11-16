#!/usr/bin/env python3
"""Census which ownership sites sit on function parameters.

Borrowing is the difference between a parameter the callee may keep and one
it only reads. The seed decides that whole-program (`borrow_modes` in
`compiler/src/`); the self-hosted emitter decides per unit and this census
measures what it actually emits, so the size of the gap is a number rather
than an impression.

Produce the input with the seed compiler, from the repository root:

    cinch core emit build/bootstrap/stage1/compiler.cinch --stage=owned \\
        -o /tmp/compiler.owned.core

`build/bootstrap/stage1/compiler.cinch` is the bundle of every bootstrap
module, so one dump covers the whole compiler. A single module cannot be
dumped on its own: the seed refuses with `module Text.Json is imported, but
bootstrap/Text.Json.cinch does not exist`, because the prelude modules are
not beside the entry file.

    python3 tooling/borrow_census.py /tmp/compiler.owned.core

Measured 2026-11-16 on `main` = 63663f0f, no-PGO stage 3:

    units 6652, parameter declarations 16964 (owned 16394, borrowed 0)
    ownership sites 237825, on parameters 111273 (46.8%)

A site is `(dup x`, `(drop x` or `(release x` whose operand is exactly one of
the enclosing unit's `@params` names. `@borrowed` is the marker a borrowed
parameter would carry; a count of zero means the mechanism is absent, not
merely underused, and that is the finding this census exists to keep visible.
"""

import argparse
import collections
import re


def units(text):
    return re.split(r"(?m)^unit ", text)[1:]


def census(text):
    total = on_params = 0
    params = owned = borrowed = 0
    by_module = collections.Counter()
    borrowed_units = []
    for unit in units(text):
        name = unit.split(" ::")[0].strip()
        declared = set()
        header = re.search(r"@params (.*)", unit)
        if header:
            for part in header.group(1).split(","):
                entry = part.strip()
                parameter = entry.split(" ")[0]
                if not parameter:
                    continue
                declared.add(parameter)
                params += 1
                if "@owned" in entry:
                    owned += 1
                if "@borrowed" in entry:
                    borrowed += 1
                    borrowed_units.append(name)
        sites = re.findall(r"\((?:dup|drop|release)\s+([A-Za-z0-9_]+)", unit)
        # `dup`, `drop` and `release` all name their operand first, so the
        # operand is what the site is about; a site on a parameter is one a
        # borrow-mode pass could have deleted.
        hit = [site for site in sites if site in declared]
        total += len(sites)
        on_params += len(hit)
        by_module[".".join(name.split(".")[:2])] += len(hit)
    return {
        "units": len(units(text)),
        "params": params,
        "owned": owned,
        "borrowed": borrowed,
        "borrowed_units": borrowed_units,
        "sites": total,
        "on_params": on_params,
        "by_module": by_module,
    }


def main():
    parser = argparse.ArgumentParser(description=__doc__,
                                     formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("core", help="a *.owned.core dump")
    parser.add_argument("--by-module", action="store_true", help="per-module breakdown")
    parser.add_argument("--top", type=int, default=10, help="modules to show")
    args = parser.parse_args()

    result = census(open(args.core).read())
    share = 100 * result["on_params"] / result["sites"] if result["sites"] else 0.0
    print(f"units {result['units']}, parameter declarations {result['params']} "
          f"(owned {result['owned']}, borrowed {result['borrowed']})")
    print(f"ownership sites {result['sites']}, on parameters {result['on_params']} "
          f"({share:.1f}%)")
    if result["borrowed"] == 0:
        print("no parameter is marked @borrowed: the borrow mechanism is absent "
              "from this emitter, so every parameter site is a candidate rather "
              "than a rejected one")
    if args.by_module:
        print()
        for module, count in result["by_module"].most_common(args.top):
            print(f"  {module:28} {count}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
