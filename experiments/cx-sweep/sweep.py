#!/usr/bin/env python3
"""The complexity sweep: one exponent per scaling axis of the self-hosted compiler.

Each axis is a family of generated programs parameterised by a size. The driver
runs the stage compiler over every point, records the runtime's own allocation
counters (`CINCH_ALLOC_STATS=1`), the emitted IR line count and the wall clock,
and fits `blocks(n) - blocks(n_min) = a * n^k`.

Blocks are the primary signal because they are load independent: this machine
runs several lanes at once, so a wall-clock slope taken across a sweep is a
slope through someone else's build. Wall clock is recorded and reported as a
secondary column with that caveat.

  python3 experiments/cx-sweep/sweep.py                 # every axis, stage 3
  python3 experiments/cx-sweep/sweep.py --axes definitions,patterns
  python3 experiments/cx-sweep/sweep.py --sizes 25 50 100 200 --out /tmp/cx

The output is `cx-sweep.json` (every point and every fit) plus a printed table.
`--check-only` uses the seed's `check` mode instead of the stage compiler's
`--emit-ir`, which is useful for validating a generator on a tree with no build.
"""

import argparse
import json
import os
import shutil
import statistics
import subprocess
import sys
import time
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
sys.path.insert(0, str(HERE))
sys.path.insert(0, str(ROOT / "tooling"))

import fit  # noqa: E402
import generators as gen  # noqa: E402


# --------------------------------------------------------------- measurements


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


def run(command, cwd, timeout):
    """One compiler invocation: exit code, blocks, bytes, wall seconds, log."""
    env = dict(os.environ)
    env["CINCH_ALLOC_STATS"] = "1"
    env.pop("CINCH_ALLOC_KINDS", None)
    started = time.perf_counter()
    try:
        completed = subprocess.run(command, cwd=cwd, env=env, capture_output=True,
                                   text=True, timeout=timeout)
        elapsed = time.perf_counter() - started
        log = completed.stdout + completed.stderr
        return {"exit": completed.returncode, "wall_s": elapsed, "log": log}
    except subprocess.TimeoutExpired as expired:
        elapsed = time.perf_counter() - started
        text = (expired.stdout or "") + (expired.stderr or "")
        if isinstance(text, bytes):
            text = text.decode("utf-8", "replace")
        return {"exit": "timeout", "wall_s": elapsed, "log": text}


def stage_cli(compiler):
    """Which command line this binary speaks.

    The Rust seed takes `cinch check FILE` / `cinch emit-ir FILE -o OUT`. The
    self-hosted compiler takes `--check --interfaces MANIFEST SOURCE DIAG` and
    `--emit-ir --interfaces MANIFEST --objects CONFIG SOURCE IR`, because it has
    no embedded prelude and resolves the base family through its own build's
    manifest. `base-interfaces.json` beside the binary is the tell.
    """
    return "bootstrap" if (compiler.parent / "base-interfaces.json").is_file() else "seed"


def commands(compiler, cli, source, work):
    """The check and emit-ir argv for one point, in this binary's dialect."""
    if cli == "seed":
        return {
            "check": [str(compiler), "check", str(source)],
            "emit-ir": [str(compiler), "emit-ir", str(source), "-o", str(work / "out.ll")],
        }
    manifest = compiler.parent / "base-interfaces.json"
    objects = compiler.parent / "objects.json"
    return {
        "check": [str(compiler), "--check", "--interfaces", str(manifest),
                  str(source), str(work / "diagnostics.txt")],
        "emit-ir": [str(compiler), "--emit-ir", "--interfaces", str(manifest),
                    "--objects", str(objects), str(source), str(work / "out.ll")],
    }


def measure_point(compiler, cli, mode, axis, size, text, work, repeats, timeout):
    """Run one point `repeats` times and keep the median block count."""
    if text is not None:
        (work / "Main.cinch").write_text(text)
    source = work / "Main.cinch"
    argv = commands(compiler, cli, source, work)["emit-ir" if mode == "emit-ir" else "check"]
    runs = []
    for _ in range(repeats):
        result = run(argv, work, timeout)
        runs.append({"exit": result["exit"], "wall_s": result["wall_s"],
                     "blocks": allocations(result["log"], "blocks"),
                     "bytes": allocations(result["log"], "bytes"),
                     "peak": allocations(result["log"], "peak")})
        if result["exit"] != 0:
            (work / f"{axis}-{size}.log").write_text(result["log"])
            break
    ir = work / "out.ll"
    ir_lines = sum(1 for _ in ir.open()) if mode == "emit-ir" and ir.is_file() else None
    successful = [r for r in runs if r["exit"] == 0]
    return {
        "axis": axis, "size": size, "mode": mode,
        "exit": runs[-1]["exit"],
        "blocks": statistics.median([r["blocks"] for r in successful])
                  if successful and all(r["blocks"] is not None for r in successful) else None,
        "bytes": statistics.median([r["bytes"] for r in successful])
                 if successful and all(r["bytes"] is not None for r in successful) else None,
        "wall_s": statistics.median([r["wall_s"] for r in successful]) if successful else None,
        "ir_lines": ir_lines,
        "runs": len(successful),
    }


# ------------------------------------------------------------- the control arm


def fnv1a64(data):
    basis, prime, mask = 14695981039346656037, 1099511628211, (1 << 64) - 1
    result = basis
    for byte in data:
        result ^= byte
        result = (result * prime) & mask
    return str(result)


def canonical(doc):
    return json.dumps(doc, ensure_ascii=False, sort_keys=True,
                      separators=(",", ":"), allow_nan=False)


def interface_hash(doc):
    projection = {**doc, "compiler": "", "source_hash": "", "units": []}
    return fnv1a64(canonical(projection).encode())


def pad_symbols(doc, extra):
    abi = doc.get("abi")
    if not isinstance(abi, dict):
        abi = {"convention": "uniform-word, curried apply stub", "layouts": [], "symbols": []}
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


def build_control_arm(directory, manifest, pad):
    """Write the provider's contracts with `pad` extra bytes in one symbol.

    This is `experiments/iface-scan/measure.py`'s construction, reused rather
    than reinvented: every interface is rewritten canonically, its identity
    recomputed after padding, and every importer's recorded dependency identity
    settled bottom up, so the set stays a valid one at every pad size.
    """
    directory.mkdir(parents=True, exist_ok=True)
    rows = []
    for row in manifest["interfaces"]:
        doc = json.loads(Path(row[2]).read_text())
        if pad:
            doc = pad_symbols(doc, pad)
        rows.append([row[0], doc])
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
    ordered = [[row[0], by_name[row[0]][0], by_name[row[0]][2]]
               for row in manifest["interfaces"]]
    out = directory / "interfaces.json"
    out.write_text(json.dumps({"schema": manifest.get("schema", 1), "interfaces": ordered},
                              sort_keys=True, separators=(",", ":")))
    return out, total_bytes


CONTROL_PROGRAM = "module Main where\n\n  main :: Int\n  main = 0\n"


def control_points(compiler, cli, sizes, work, repeats, timeout):
    """The interface-size control: blocks against one document's byte length.

    A staged compiler can only be measured this way, since the seed has no
    manifest to pad. The program is the smallest possible one, so every block
    above the empty-manifest floor belongs to reading the padded set.
    """
    if cli != "bootstrap":
        return None, "control needs the staged compiler's --interfaces mode"
    manifest_path = compiler.parent / "base-interfaces.json"
    manifest = json.loads(manifest_path.read_text())
    (work / "Main.cinch").write_text(CONTROL_PROGRAM)

    def measure_arm(pad):
        """One arm: the provider's set with `pad` bytes added to one symbol."""
        arm = work / f"pad{pad}"
        if arm.exists():
            shutil.rmtree(arm)
        arm.mkdir(parents=True)
        (arm / "Main.cinch").write_text(CONTROL_PROGRAM)
        path, total = build_control_arm(arm / "interfaces", manifest, pad)
        argv = [str(compiler), "--check", "--interfaces", str(path),
                str(arm / "Main.cinch"), str(arm / "diagnostics.txt")]
        results = []
        for _ in range(repeats):
            results.append(run(argv, arm, timeout))
            if results[-1]["exit"] != 0:
                (arm / "run.log").write_text(results[-1]["log"])
                break
        ok = [r for r in results if r["exit"] == 0]
        return {
            "axis": gen.CONTROL, "size": pad, "mode": "check",
            "exit": results[-1]["exit"],
            "bytes": statistics.median([allocations(r["log"], "bytes") for r in ok])
                     if ok else None,
            "blocks": statistics.median([allocations(r["log"], "blocks") for r in ok])
                      if ok else None,
            "wall_s": statistics.median([r["wall_s"] for r in ok]) if ok else None,
            "ir_lines": None,
            "document_bytes": total,
            "runs": len(ok),
        }

    # The baseline is the UNPADDED provider set, which is what the published
    # law is measured against: the deltas are then the cost of the padding
    # alone, at 6.0 blocks per added byte.
    floor = measure_arm(0)
    points = [measure_arm(size) for size in sizes]
    return {"floor_blocks": floor["blocks"], "floor_bytes": floor["bytes"],
            "floor_document_bytes": floor["document_bytes"],
            "points": points}, None


# ------------------------------------------------------------------ the report


def axis_row(axis, points):
    """Fit one axis and assemble the numbers the table prints."""
    usable = [p for p in points if p["blocks"] is not None and p["exit"] == 0]
    if len(usable) < 2:
        return {"axis": axis, "description": gen.description(axis),
                "error": "fewer than two usable points", "points": points}
    sizes = [p["size"] for p in usable]
    blocks = [p["blocks"] for p in usable]
    result = fit.fit_axis(sizes, blocks,
                          [p["bytes"] for p in usable],
                          [p["ir_lines"] for p in usable],
                          [p["wall_s"] for p in usable])
    result["axis"] = axis
    result["description"] = gen.description(axis)
    result["points"] = points
    result["excluded"] = [p["size"] for p in points if p not in usable]
    return result


def num(value, spec="{:.3f}"):
    """Format a number, or a dash when the fit could not produce one.

    A point that failed, or a fit with fewer than two positive deltas, leaves
    a None where the table expects a float; printing it as a dash keeps the
    column alignment and keeps the failure visible.
    """
    return spec.format(value) if value is not None else "-"


def verdict(exponent):
    if exponent is None:
        return "no fit"
    if exponent < 1.15:
        return "linear"
    if exponent < 1.5:
        return "look"
    return "SUPERLINEAR"


def format_table(rows):
    """The printed table: one line per axis, then every axis's own points.

    `k` is the floor-fitted exponent, the one to read: it fits
    `floor + a*n^k` with the floor free, so a large fixed cost does not
    masquerade as a small exponent. `delta` subtracts the smallest point
    instead and is reported beside it because the two disagreeing is itself
    the sign of a floor-dominated axis.
    """
    lines = []
    header = (f"{'axis':14s} {'k':>6s} {'r2':>6s} {'floor':>11s} {'delta':>6s} "
              f"{'bytes':>6s} {'ir':>6s} {'local':>16s}  verdict")
    lines.append(header)
    lines.append("-" * len(header))
    for row in rows:
        if "error" in row:
            lines.append(f"{row['axis']:14s} {'--':>6s} {'--':>6s} {'--':>11s} {'--':>6s} "
                         f"{'--':>6s} {'--':>6s} {'--':>16s}  {row['error']}")
            continue
        local = ",".join(f"{x:.2f}" if x is not None else "-" for x in row["local"])
        exponent = row["floor_exponent"]
        floor = row.get("floor_value")
        floor_text = f"{floor / 1e6:.1f}M" if floor and floor > 1e6 else num(floor, "{:.0f}")
        lines.append(
            f"{row['axis']:14s} {num(exponent, '{:6.2f}'):>6s} "
            f"{num(row['floor_r2'], '{:6.3f}'):>6s} {floor_text:>11s} "
            f"{num(row['delta_exponent'], '{:6.2f}'):>6s} "
            f"{num(row['bytes_exponent'], '{:6.2f}'):>6s} "
            f"{num(row['ir_exponent'], '{:6.2f}'):>6s} {local:>16s}  "
            f"{verdict(exponent)}")
    lines.append("")
    lines.append("per-axis points (n, blocks, delta, bytes, ir_lines, wall_s):")
    for row in rows:
        if "error" in row:
            continue
        lines.append(f"  {row['axis']}: {row['description']}")
        for point, delta in zip(row["points"], row["delta_blocks"]):
            if point["blocks"] is None:
                lines.append(f"    n={point['size']:>6} FAILED exit={point['exit']}")
                continue
            lines.append(f"    n={point['size']:>6} blocks={point['blocks']:>12} "
                         f"delta={delta:>12} bytes={point['bytes']} "
                         f"ir={point['ir_lines']} wall={num(point['wall_s'], '{:.2f}')}s")
    return "\n".join(lines)


def control_report(control):
    """The control's own law: blocks per byte of interface document.

    The rate is `delta blocks / delta bytes` against the unpadded provider
    set, because padding every interface by `extra` bytes adds
    `extra * interfaces` bytes to the set, not `extra`. Dividing by the pad
    instead would report a rate `interfaces` times too small.
    """
    if control is None:
        return {"error": "control arm not run"}
    floor = control["floor_blocks"]
    floor_bytes = control["floor_document_bytes"]
    law = []
    for point in control["points"]:
        if point["blocks"] is None or floor is None or floor_bytes is None:
            continue
        added = point["document_bytes"] - floor_bytes
        law.append({"pad": point["size"], "document_bytes": point["document_bytes"],
                    "blocks": point["blocks"], "delta_blocks": point["blocks"] - floor,
                    "delta_bytes": added,
                    "blocks_per_byte": (point["blocks"] - floor) / added if added else None})
    slopes = [entry["blocks_per_byte"] for entry in law if entry["blocks_per_byte"]]
    median = statistics.median(slopes) if slopes else None
    # Two published values, because the law moved: 10.0 blocks per byte is the
    # scan before the string fast path, 6.0 after it (design/compilation-speed.md
    # 2.18). A tree that predates the fix measures the first; this tree's HEAD
    # carries it and measures the second. Reproducing either is a pass, and
    # which one is reported, because a harness that reproduced neither would
    # be measuring something other than interface bytes.
    nearest = None
    if median is not None:
        nearest = min(gen.CONTROL_LAWS, key=lambda law_value: abs(median - law_value))
    return {
        "floor_blocks": floor,
        "floor_bytes": control["floor_bytes"],
        "floor_document_bytes": floor_bytes,
        "law": law,
        "median_blocks_per_byte": median,
        "expected_blocks_per_byte": nearest,
        "known_laws": list(gen.CONTROL_LAWS),
        "reproduced": nearest is not None and abs(median - nearest) <= 1.0,
        "exponent": fit.loglog([e["delta_bytes"] for e in law],
                               [e["delta_blocks"] for e in law])["exponent"],
    }


def refit(path):
    """Re-fit a recorded sweep: the raw points are the evidence, the fit is a view.

    Recompiling to change a fitting method would cost an hour and measure the
    machine again instead of the compiler, so the recorded blocks are refitted
    in place and the document is rewritten.
    """
    document = json.loads(path.read_text())
    rows = [axis_row(axis, points) for axis, points in document["raw"].items()]
    document["axes"] = rows
    path.write_text(json.dumps(document, indent=1, sort_keys=True))
    print(format_table(rows))
    control = document.get("control")
    if control and "error" not in control:
        print()
        print(f"control ({gen.CONTROL}): median "
              f"{num(control['median_blocks_per_byte'], '{:.2f}')} blocks/byte "
              f"(nearest known {num(control['expected_blocks_per_byte'], '{:.1f}')}), "
              f"reproduced={control['reproduced']}")
    print()
    print(f"refitted {path}")


def main():
    parser = argparse.ArgumentParser(description=__doc__,
                                     formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--stage", type=int, default=3)
    parser.add_argument("--compiler", default=None, help="an explicit compiler binary")
    parser.add_argument("--out", default="/tmp/cx-sweep")
    parser.add_argument("--axes", default=None,
                        help="comma separated subset of the axes (default: all)")
    parser.add_argument("--sizes", type=int, nargs="+", default=None,
                        help="override every axis's sizes (default: each axis's own)")
    parser.add_argument("--repeats", type=int, default=3)
    parser.add_argument("--timeout", type=float, default=900.0)
    parser.add_argument("--mode", choices=("emit-ir", "check"), default="emit-ir")
    parser.add_argument("--no-control", action="store_true")
    parser.add_argument("--label", default=None)
    parser.add_argument("--refit", default=None,
                        help="re-fit an existing cx-sweep.json instead of measuring; "
                             "the raw points are reused, so this is how a changed fit "
                             "is evaluated without recompiling anything")
    args = parser.parse_args()

    if args.refit:
        refit(Path(args.refit))
        return

    # Absolute: every invocation runs with a point's scratch directory as its
    # cwd, so a relative path would resolve against the wrong tree.
    compiler = (Path(args.compiler) if args.compiler else
                ROOT / "build" / "bootstrap" / f"stage{args.stage}" / "cinch").resolve()
    if not compiler.is_file():
        sys.exit(f"no compiler at {compiler}; run tooling/bootstrap.py build --stage {args.stage}")
    cli = stage_cli(compiler)

    names = [n for n in gen.AXES]
    if args.axes:
        names = [n.strip() for n in args.axes.split(",") if n.strip()]
        unknown = [n for n in names if n not in gen.AXES and n != gen.CONTROL]
        if unknown:
            sys.exit(f"unknown axes: {', '.join(unknown)}")

    out = Path(args.out)
    if out.exists():
        shutil.rmtree(out)
    out.mkdir(parents=True)

    rows, raw = [], {}
    for axis in names:
        _, _, default_sizes = gen.AXES[axis]
        sizes = args.sizes or list(default_sizes)
        work = out / axis
        work.mkdir(parents=True, exist_ok=True)
        points = []
        for size in sizes:
            point_work = work / f"n{size}"
            point_work.mkdir(parents=True, exist_ok=True)
            text = gen.generate(axis, size)
            point = measure_point(compiler, cli, args.mode, axis, size, text,
                                  point_work, args.repeats, args.timeout)
            points.append(point)
            print(f"  {axis} n={size}: exit={point['exit']} blocks={point['blocks']} "
                  f"ir={point['ir_lines']} wall={num(point['wall_s'], '{:.2f}')}s", flush=True)
        raw[axis] = points
        rows.append(axis_row(axis, points))

    control = None
    if not args.no_control:
        sizes = args.sizes or list(gen.CONTROL_SIZES)
        control_work = out / "control"
        control_work.mkdir(parents=True, exist_ok=True)
        prepared, reason = control_points(compiler, cli, sizes, control_work,
                                          args.repeats, args.timeout)
        if prepared is None:
            print(f"  control: skipped ({reason})", flush=True)
        else:
            control = control_report(prepared)

    document = {
        "label": args.label,
        "stage": args.stage,
        "compiler": str(compiler),
        "cli": cli,
        "mode": args.mode,
        "repeats": args.repeats,
        "axes": rows,
        "control": control,
        "raw": raw,
    }
    (out / "cx-sweep.json").write_text(json.dumps(document, indent=1, sort_keys=True))

    print()
    print(format_table(rows))
    if control and "error" not in control:
        print()
        print(f"control ({gen.CONTROL}): floor={control['floor_blocks']} blocks, "
              f"median {control['median_blocks_per_byte']:.2f} blocks/byte "
              f"(expected {control['expected_blocks_per_byte']}), "
              f"exponent {control['exponent']:.3f}, "
              f"reproduced={control['reproduced']}")
    print()
    print(f"wrote {out / 'cx-sweep.json'}")


if __name__ == "__main__":
    main()
