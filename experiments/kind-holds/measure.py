#!/usr/bin/env python3
"""Blocks and bytes for the `Kind` carrier round trip, before and after.

The question this answers: `Kind`'s `KValue` holds a `UType` as a JSON
document, so every kind observation of a type encodes the whole type tree
and decodes it again. Arms are the same two programs checked through two
stage-3 compilers, interleaved, with a fresh `CINCH_STORE` per run:

  two-line   `module Main where` / `main :: Int` / `main = 0`, which pays
             the fixed cost: 51 interface contracts loaded and elaborated,
             every one of them full of quantified types.
  toolkit    the five P30 toolkit modules (`Text.{Lex,Scan,Parse,Json,
             Builder}`) plus a `Main`, the per-line workload.

Each arm is one compiler binary; the binaries are given as
`--arm label=path`. Repeats alternate the arms so drift is shared.

  python3 experiments/kind-holds/measure.py --out /tmp/kind-holds \
      --arm before=../dasl-scc-order/build/bootstrap/stage3/cinch \
      --arm after=build/bootstrap/stage3/cinch

The runtime's `CINCH_ALLOC_STATS=1` line is the source: `blocks` and
`bytes` are cumulative over every allocation the process makes.
"""

import argparse
import json
import os
import shutil
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
TOOLKIT = ("Lex", "Scan", "Parse", "Json", "Builder")

TWO_LINE = "module Main where\n\n  main :: Int\n  main = 0\n"


def toolkit_bundle(directory: Path) -> Path:
    """The five toolkit modules as one file, with a `Main` that imports them."""
    out = directory / "toolkit.cinch"
    out.parent.mkdir(parents=True, exist_ok=True)
    parts = [f"-- source: {name}\n".encode()
             + (ROOT / "compiler" / "prelude" / "Text" / f"{name}.cinch").read_bytes()
             for name in TOOLKIT]
    imports = "".join(f"  import Text.{name} as Bundle{name}\n" for name in TOOLKIT)
    parts.append(("module Main where\n" + imports + "  main :: Int\n  main = 0\n").encode())
    out.write_bytes(b"\n\n".join(parts) + b"\n")
    return out


def allocations(log: str, kind: str):
    """The value after `kind=` on the runtime's `cinch-alloc:` line."""
    for line in log.splitlines():
        if line.startswith("cinch-alloc:"):
            for field in line[len("cinch-alloc:"):].split():
                key, _, value = field.partition("=")
                if key == kind:
                    return int(value)
    return None


def provider_manifest(compiler: Path) -> Path:
    """The interface manifest that belongs to a stage-3 build.

    `base-interfaces.json` sits beside the binary the build produced, so an
    arm's compiler is always paired with the interfaces it was built
    against; a compiler run against another tree's manifest measures a
    different program.
    """
    candidate = compiler.parent / "base-interfaces.json"
    if not candidate.is_file():
        sys.exit(f"no base-interfaces.json beside {compiler}")
    return candidate


def run(compiler: Path, manifest: Path, source: Path, store: Path, work: Path):
    """One `--check` with a fresh store and a private working directory."""
    if store.exists():
        shutil.rmtree(store)
    store.mkdir(parents=True)
    diagnostics = work / "diagnostics.txt"
    diagnostics.unlink(missing_ok=True)
    env = {k: v for k, v in os.environ.items() if not k.startswith("CINCH_")}
    env["CINCH_ALLOC_STATS"] = "1"
    env["CINCH_STORE"] = str(store)
    env["CINCH_HEAP_LIMIT"] = "4G"
    command = [str(compiler), "--check", "--interfaces", str(manifest),
               str(source), str(diagnostics)]
    completed = subprocess.run(command, cwd=work, env=env, capture_output=True, text=True)
    log = completed.stdout + completed.stderr
    return (completed.returncode, allocations(log, "blocks"),
            allocations(log, "bytes"), allocations(log, "peak"), log)


def median(values):
    ordered = sorted(values)
    if not ordered:
        return None
    middle = len(ordered) // 2
    if len(ordered) % 2:
        return ordered[middle]
    return (ordered[middle - 1] + ordered[middle]) / 2


def main():
    parser = argparse.ArgumentParser(description=__doc__,
                                     formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--arm", action="append", required=True,
                        metavar="LABEL=PATH", help="a stage-3 compiler; repeatable")
    parser.add_argument("--out", default="/tmp/kind-holds")
    parser.add_argument("--repeats", type=int, default=3,
                        help="interleaved repeats per arm and workload")
    parser.add_argument("--workload", action="append",
                        choices=["two-line", "toolkit"], default=None)
    parser.add_argument("--label", default=None)
    args = parser.parse_args()

    arms = []
    for spec in args.arm:
        label, _, path = spec.partition("=")
        compiler = Path(path).resolve()
        if not compiler.is_file():
            sys.exit(f"no compiler at {compiler}")
        arms.append((label, compiler, provider_manifest(compiler)))
    workloads = args.workload or ["two-line", "toolkit"]

    out = Path(args.out)
    if out.exists():
        shutil.rmtree(out)
    out.mkdir(parents=True)
    work = out / "work"
    work.mkdir()
    sources = {"two-line": work / "Main.cinch"}
    sources["two-line"].write_text(TWO_LINE)
    if "toolkit" in workloads:
        sources["toolkit"] = toolkit_bundle(work)

    results = {}
    for repeat in range(args.repeats):
        order = list(arms)
        if repeat % 2:
            order.reverse()
        for label, compiler, manifest in order:
            for workload in workloads:
                key = f"{label}/{workload}"
                store = out / "stores" / f"{key.replace('/', '-')}-{repeat}"
                code, blocks, allocated, peak, log = run(
                    compiler, manifest, sources[workload], store, work)
                row = results.setdefault(key, {"arm": label, "workload": workload,
                                               "compiler": str(compiler),
                                               "manifest": str(manifest), "runs": []})
                row["runs"].append({"exit": code, "blocks": blocks,
                                    "bytes": allocated, "peak": peak})
                if code != 0:
                    (out / f"{key.replace('/', '-')}.log").write_text(log)
                    row["log"] = str(out / f"{key.replace('/', '-')}.log")

    for row in results.values():
        row["exit"] = row["runs"][-1]["exit"]
        row["blocks"] = median([r["blocks"] for r in row["runs"] if r["blocks"] is not None])
        row["bytes"] = median([r["bytes"] for r in row["runs"] if r["bytes"] is not None])
        row["peak"] = median([r["peak"] for r in row["runs"] if r["peak"] is not None])
        row["stable"] = len({r["blocks"] for r in row["runs"]}) == 1

    deltas = []
    labels = [label for label, _, _ in arms]
    for workload in workloads:
        first = results.get(f"{labels[0]}/{workload}")
        for label in labels[1:]:
            later = results.get(f"{label}/{workload}")
            if not first or not later or first["blocks"] is None or later["blocks"] is None:
                continue
            deltas.append({
                "workload": workload,
                "from": labels[0], "to": label,
                "blocks_before": first["blocks"], "blocks_after": later["blocks"],
                "blocks_delta": later["blocks"] - first["blocks"],
                "blocks_percent": 100.0 * (later["blocks"] - first["blocks"]) / first["blocks"],
                "bytes_before": first["bytes"], "bytes_after": later["bytes"],
                "bytes_delta": later["bytes"] - first["bytes"],
                "bytes_percent": 100.0 * (later["bytes"] - first["bytes"]) / first["bytes"],
                "peak_before": first["peak"], "peak_after": later["peak"],
            })

    document = {"label": args.label, "repeats": args.repeats,
                "arms": results, "deltas": deltas}
    (out / "measure.json").write_text(json.dumps(document, indent=1, sort_keys=True))

    for row in sorted(results.values(), key=lambda r: (r["workload"], r["arm"])):
        print(f"{row['workload']:9s} {row['arm']:8s} exit={row['exit']} "
              f"blocks={row['blocks']} bytes={row['bytes']} peak={row['peak']} "
              f"stable={row['stable']}")
    for delta in deltas:
        print(f"{delta['workload']:9s} {delta['from']} -> {delta['to']}: "
              f"blocks {delta['blocks_delta']:+d} ({delta['blocks_percent']:+.2f}%), "
              f"bytes {delta['bytes_delta']:+d} ({delta['bytes_percent']:+.2f}%)")


if __name__ == "__main__":
    main()
