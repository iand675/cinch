#!/usr/bin/env python3
"""The blocks-per-interface-byte law, measured against a stage compiler.

Arms, each a two-line program checked with `--check` against a manifest of
interface contracts, with `CINCH_ALLOC_STATS=1` reading the block count the
runtime prints at exit:

  empty     a manifest with no interfaces: the floor the set sits on
  control   the base interface set as the provider emits it
  pad1k     the same set with `abi.symbols` padded by 1 KiB per module
  pad2k     ... by 2 KiB
  pad4k     ... by 4 KiB

The marginal price of a JSON byte is (blocks(padN) - blocks(control)) /
(bytes(padN) - bytes(control)); three points so the linearity shows.

Padding changes an interface's bytes, so its identity changes, so every
importer's recorded `imports[].iface_hash` changes too, cascading up the
dependency graph. The arms re-settle identities in topological order (an
interface's identity is FNV-1a 64 over its canonical projection, the rule
`Bootstrap.Load.load-interface` and `interface.rs`'s `Iface::iface_hash`
apply), because `verify-set` refuses a stale dependency and an arm that
exits 7 measures nothing. `abi.symbols` is the padding site section 2.17
used: `strings-p` validates it structurally and no consumer reads its
contents.

  python3 experiments/iface-scan/measure.py --stage 3 --out /tmp/iface-scan
"""

import argparse
import json
import os
import shutil
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]

PROGRAM = "module Main where\n\n  main :: Int\n  main = 0\n"

PAD_SIZES = (1024, 2048, 4096)

FNV_BASIS = 14695981039346656037
FNV_PRIME = 1099511628211
MASK64 = (1 << 64) - 1


def canonical(doc):
    """The canonical JSON the interface writer emits and the reader hashes."""
    return json.dumps(doc, ensure_ascii=False, sort_keys=True,
                      separators=(",", ":"), allow_nan=False)


def fnv1a64(data):
    result = FNV_BASIS
    for byte in data:
        result = ((result ^ byte) * FNV_PRIME) & MASK64
    return str(result)


def projection(doc):
    """The contract a consumer must agree with: the top-level `compiler` and
    `source_hash` blanked and `units` cleared (`contract-projection`)."""
    return {**doc, "compiler": "", "source_hash": "", "units": []}


def interface_hash(doc):
    return fnv1a64(canonical(projection(doc)).encode())


def pad_symbols(doc, extra):
    """Append `extra` bytes to `abi.symbols`, keeping every other field.

    `envelope-valid` requires `abi` to be an object with `convention`,
    `symbols` (a string array) and `layouts` (an object array), so the
    padding goes into the first existing symbol and a module without an
    `abi` section gets the envelope's shape.
    """
    abi = doc.get("abi")
    if not isinstance(abi, dict):
        abi = {"convention": "uniform-word, curried apply stub",
               "layouts": [], "symbols": []}
        doc["abi"] = abi
    abi.setdefault("convention", "uniform-word, curried apply stub")
    abi.setdefault("layouts", [])
    symbols = abi.get("symbols")
    if not isinstance(symbols, list):
        symbols = []
        abi["symbols"] = symbols
    if not symbols:
        symbols.append("")
    symbols[0] = symbols[0] + ("x" * extra)
    return doc


def dependency_order(docs):
    """Modules ordered so every module follows the modules it imports.

    `verify-set` walks the set and refuses a stale dependency, so identities
    settle bottom up. A cycle is reported rather than looped on.
    """
    by_name = {doc["module"]: doc for doc in docs}
    order, state = [], {}

    def visit(name, trail):
        if state.get(name) == "done":
            return
        if state.get(name) == "busy":
            raise SystemExit("interface import cycle: " + " -> ".join(trail + [name]))
        state[name] = "busy"
        for dep in by_name[name].get("imports", []):
            dep_name = dep.get("module")
            if dep_name in by_name:
                visit(dep_name, trail + [name])
        state[name] = "done"
        order.append(name)

    for name in sorted(by_name):
        visit(name, [])
    return order


def build_arm(directory, manifest, pad):
    """Write one arm's contract files and manifest; returns (manifest, bytes).

    Every interface is written canonically, its identity recomputed after
    the padding, and every importer's recorded dependency identity updated
    to the settled value, in dependency order.
    """
    directory.mkdir(parents=True, exist_ok=True)
    rows = []
    for row in manifest["interfaces"]:
        module, _digest, path = row[0], row[1], row[2]
        doc = json.loads(Path(path).read_text())
        if pad:
            doc = pad_symbols(doc, pad)
        rows.append([module, doc])
    by_name = {row[0]: row for row in rows}
    total_bytes = 0
    for name in dependency_order([row[1] for row in rows]):
        doc = by_name[name][1]
        if pad:
            for dep in doc.get("imports", []):
                settled = by_name.get(dep.get("module"))
                if settled is not None:
                    dep["iface_hash"] = interface_hash(settled[1])
        text = canonical(doc)
        total_bytes += len(text.encode())
        target = directory / f"{name}.cinchif"
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_text(text)
        by_name[name][0] = interface_hash(doc)
        by_name[name].append(str(target.resolve()))
    out_rows = [[name, by_name[name][0], by_name[name][2]] for name in by_name]
    # The manifest's order is the provider's; `verify-set` orders the set
    # itself, so this only has to name every module once.
    ordered = [[row[0], row[1], row[2]] for row in
               ([m, by_name[m][0], by_name[m][2]] for m in
                [r[0] for r in manifest["interfaces"]])]
    out = directory / "interfaces.json"
    out.write_text(json.dumps({"schema": manifest.get("schema", 1), "interfaces": ordered},
                              sort_keys=True, separators=(",", ":")))
    return out, total_bytes


def empty_manifest(directory):
    directory.mkdir(parents=True, exist_ok=True)
    out = directory / "interfaces.json"
    out.write_text(json.dumps({"schema": 1, "interfaces": []}, separators=(",", ":")))
    return out


def allocations(log, kind):
    """The value after `kind=` on the runtime's `cinch-alloc:` stats line.

    The line is space separated, one `key=value` per field:
    `cinch-alloc: blocks=19663 bytes=455018 peak=17748 header-blocks=14949
    header-bytes=415464 immortal=0 freed=14949 live=0`.
    """
    for line in log.splitlines():
        if line.startswith("cinch-alloc:"):
            for field in line[len("cinch-alloc:"):].split():
                key, _, value = field.partition("=")
                if key == kind:
                    return int(value)
    return None


def run_check(compiler, manifest, work, source):
    diagnostics = work / "diagnostics.txt"
    diagnostics.unlink(missing_ok=True)
    env = dict(os.environ)
    env["CINCH_ALLOC_STATS"] = "1"
    env.pop("CINCH_ALLOC_KINDS", None)
    command = [str(compiler), "--check", "--interfaces", str(manifest),
               str(source), str(diagnostics)]
    completed = subprocess.run(command, cwd=work, env=env, capture_output=True, text=True)
    log = completed.stdout + completed.stderr
    return completed.returncode, allocations(log, "blocks"), allocations(log, "bytes"), log


def median(values):
    ordered = sorted(values)
    if not ordered:
        return None
    middle = len(ordered) // 2
    if len(ordered) % 2:
        return ordered[middle]
    return (ordered[middle - 1] + ordered[middle]) / 2


def find_provider_manifest():
    root = ROOT / "build" / "bootstrap" / "library-provider"
    candidates = sorted(root.glob("*/interfaces.json"), key=lambda p: p.stat().st_mtime)
    if not candidates:
        sys.exit(f"no provider interfaces.json under {root}")
    return candidates[-1]


def main():
    parser = argparse.ArgumentParser(description=__doc__,
                                     formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--stage", type=int, default=3)
    parser.add_argument("--out", default="/tmp/iface-scan")
    parser.add_argument("--repeats", type=int, default=3,
                        help="interleaved repeats per arm; the median is reported")
    parser.add_argument("--manifest", default=None,
                        help="the provider's interfaces.json (default: the built provider's)")
    parser.add_argument("--label", default=None,
                        help="a name for this measurement, recorded in measure.json")
    args = parser.parse_args()

    compiler = ROOT / "build" / "bootstrap" / f"stage{args.stage}" / "cinch"
    if not compiler.is_file():
        sys.exit(f"no stage{args.stage} compiler at {compiler}")

    manifest_path = args.manifest or find_provider_manifest()
    manifest = json.loads(Path(manifest_path).read_text())
    out = Path(args.out)
    if out.exists():
        shutil.rmtree(out)
    out.mkdir(parents=True)
    work = out / "work"
    work.mkdir()
    source = work / "Main.cinch"
    source.write_text(PROGRAM)

    arms = [("control", 0)] + [(f"pad{size // 1024}k", size) for size in PAD_SIZES]
    prepared = {}
    for name, pad in arms:
        prepared[name] = build_arm(out / name, manifest, pad)
    empty = empty_manifest(out / "empty")

    results = {}
    for repeat in range(args.repeats):
        order = ["empty"] + [name for name, _ in arms]
        if repeat % 2:
            order = list(reversed(order))
        for name in order:
            path = empty if name == "empty" else prepared[name][0]
            code, blocks, allocated, log = run_check(compiler, path, work, source)
            row = results.setdefault(name, {"arm": name, "runs": [],
                                            "bytes": 0 if name == "empty" else prepared[name][1]})
            row["runs"].append({"exit": code, "blocks": blocks, "bytes": allocated})
            if code != 0:
                (out / f"{name}.log").write_text(log)
                row["log"] = str(out / f"{name}.log")

    for row in results.values():
        row["exit"] = row["runs"][-1]["exit"]
        row["blocks"] = median([r["blocks"] for r in row["runs"] if r["blocks"] is not None])
        row["allocated_bytes"] = median([r["bytes"] for r in row["runs"] if r["bytes"] is not None])

    control = results["control"]
    law = []
    for name, _ in arms[1:]:
        arm = results[name]
        if control["blocks"] is None or arm["blocks"] is None:
            continue
        delta_blocks = arm["blocks"] - control["blocks"]
        delta_bytes = arm["bytes"] - control["bytes"]
        law.append({"arm": name, "delta_blocks": delta_blocks, "delta_bytes": delta_bytes,
                    "blocks_per_byte": delta_blocks / delta_bytes if delta_bytes else None})

    document = {
        "label": args.label,
        "stage": args.stage,
        "compiler": str(compiler),
        "manifest": str(manifest_path),
        "interfaces": len(manifest["interfaces"]),
        "repeats": args.repeats,
        "arms": results,
        "law": law,
        "empty_floor": results.get("empty", {}).get("blocks"),
    }
    (out / "measure.json").write_text(json.dumps(document, indent=1, sort_keys=True))

    print(f"stage{args.stage}: {len(manifest['interfaces'])} interfaces, "
          f"{control['bytes']} bytes")
    for name in ["empty", "control"] + [n for n, _ in arms[1:]]:
        row = results[name]
        print(f"  {name:10s} exit={row['exit']} blocks={row['blocks']} bytes={row['bytes']}")
    for entry in law:
        rate = entry["blocks_per_byte"]
        if rate is None:
            print(f"  law {entry['arm']}: no measurement")
        else:
            print(f"  law {entry['arm']:8s} +{entry['delta_bytes']} bytes "
                  f"+{entry['delta_blocks']} blocks -> {rate:.2f} blocks/byte")
    print(f"wrote {out / 'measure.json'}")


if __name__ == "__main__":
    main()
