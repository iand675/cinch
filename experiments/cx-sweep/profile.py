#!/usr/bin/env python3
"""Attribute one sweep point to functions, so an exponent can be given a site.

`sample` walks the running compiler's stack; this driver runs one generated
point, samples it, and aggregates the samples two ways:

  inclusive   a frame's sample count as `sample` prints it: the number of
              samples with that frame anywhere on the stack. This is where the
              work is spent, not necessarily where it is caused.
  subtree     samples under `--root` (default `elaborate-module`), which is the
              frontend pass every axis grows. Reading the subtree is how a
              total-cost site is told from a per-call one.

  python3 experiments/cx-sweep/profile.py --axis modules --size 64 --seconds 5

The output is a ranked list of frames with their inclusive share of the
samples taken under the root, plus the raw report path.
"""

import argparse
import os
import re
import subprocess
import sys
import time
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
sys.path.insert(0, str(HERE))

import generators as gen  # noqa: E402


def stage_argv(compiler, source, work):
    """The emit-ir command line for a staged compiler."""
    return [str(compiler), "--emit-ir",
            "--interfaces", str(compiler.parent / "base-interfaces.json"),
            "--objects", str(compiler.parent / "objects.json"),
            str(source), str(work / "out.ll")]


def frames(report):
    """Every `(depth, count, name)` line of `sample`'s call graph section.

    The report has three sections (header, `Call graph:`, and a list of every
    loaded image), and only the call graph carries counts and indentation.
    Parsing the whole file mixes the binary image list into the totals.
    """
    rows = []
    inside = False
    for line in Path(report).read_text(errors="replace").splitlines():
        if line.startswith("Call graph:"):
            inside = True
            continue
        if line.startswith("Total number in stack"):
            inside = False
        if not inside:
            continue
        match = re.match(r"^(\s*)(\d+)\s+(\S.*)$", line)
        if not match:
            continue
        rows.append((len(match.group(1)), int(match.group(2)), match.group(3)))
    return rows


def subtree(rows, root):
    """The rows under the first frame whose name contains `root`, or all rows.

    `sample` indents two columns per call level, so a child is any later row
    indented deeper than the root's own indent, up to the next row at or
    above it.
    """
    start = None
    for index, (indent, count, name) in enumerate(rows):
        # The symbol is mangled (`elaborate$2dmodule`), so the match runs on
        # the demangled spelling; matching the raw text silently finds nothing
        # and reports the whole process as the subtree.
        if root in clean(name):
            start = index
            break
    if start is None:
        return rows, None
    base = rows[start][0]
    end = len(rows)
    for index in range(start + 1, len(rows)):
        if rows[index][0] <= base:
            end = index
            break
    scoped = rows[start:end]
    if len(scoped) == 1:
        # `sample` prints a bounded stack depth, so below some level every
        # frame appears at the same indent as its parent. When that happens
        # there is no deeper indent to bound the subtree with, and the
        # remainder of the graph is the honest over-approximation: it is the
        # set of frames that could be under this root.
        scoped = rows[start:]
    return scoped, rows[start][1]


def aggregate(rows):
    """Total samples per function name, keeping the deepest-distinct spelling.

    A recursive function appears many times in one stack, once per frame, so
    the counts must not be summed across occurrences: the maximum per name is
    the number of samples that name was on the stack for, which is what
    "inclusive" means for a recursive callee.
    """
    totals = {}
    for _, count, name in rows:
        # Drop the `(in cinch) + offset [address]` tail; keep the symbol.
        symbol = re.split(r"\s+\(in ", name)[0].strip()
        totals[symbol] = max(totals.get(symbol, 0), count)
    return totals


def clean(symbol):
    """`sample`'s mangling, undone enough to read: `$2e` is `.`, `$2d` is `-`."""
    text = symbol.replace("$2e", ".").replace("$2d", "-").replace("$24", "$")
    return text


def main():
    parser = argparse.ArgumentParser(description=__doc__,
                                     formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--stage", type=int, default=3)
    parser.add_argument("--compiler", default=None)
    parser.add_argument("--axis", required=True)
    parser.add_argument("--size", type=int, required=True)
    parser.add_argument("--seconds", type=int, default=5)
    parser.add_argument("--delay", type=float, default=None,
                        help="seconds to wait before sampling (default: 40%% of --seconds). "
                             "Too short samples the interface load, too long can miss the run")
    parser.add_argument("--root", default="elaborate-module")
    parser.add_argument("--top", type=int, default=25)
    parser.add_argument("--out", default="/tmp/cx-profile")
    args = parser.parse_args()

    compiler = (Path(args.compiler) if args.compiler else
                ROOT / "build" / "bootstrap" / f"stage{args.stage}" / "cinch").resolve()
    work = Path(args.out) / f"{args.axis}-{args.size}"
    work.mkdir(parents=True, exist_ok=True)
    source = work / "Main.cinch"
    source.write_text(gen.generate(args.axis, args.size))

    environment = dict(os.environ)
    environment["CINCH_ALLOC_STATS"] = "1"
    process = subprocess.Popen(stage_argv(compiler, source, work), cwd=work,
                               env=environment, stdout=subprocess.DEVNULL,
                               stderr=subprocess.DEVNULL)
    # Give the compiler time to reach the frontend before sampling; the fixed
    # cost (reading the manifest, the base set) runs first and would dominate
    # a sample taken at process start.
    time.sleep(args.delay if args.delay is not None else min(2.0, args.seconds * 0.4))
    report = work / "sample.txt"
    subprocess.run(["sample", str(process.pid), str(args.seconds), "-mayDie",
                    "-file", str(report)], capture_output=True)
    process.wait()

    rows = frames(report)
    scoped, root_count = subtree(rows, args.root)
    totals = aggregate(scoped)
    ranked = sorted(totals.items(), key=lambda item: -item[1])
    print(f"{args.axis} n={args.size}: {root_count} samples under {args.root!r} "
          f"of {max((c for _, c, _ in rows), default=0)} total")
    for symbol, count in ranked[:args.top]:
        share = count / root_count if root_count else 0
        print(f"  {share:6.1%} {count:6d}  {clean(symbol)}")
    print(f"report: {report}")


if __name__ == "__main__":
    main()
