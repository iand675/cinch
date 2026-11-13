#!/usr/bin/env python3
"""The two other workloads the interface-scan change is measured on.

  toolkit     the PGO training bundle: `Text.{Lex,Scan,Parse,Json,Builder}`
              plus an entry that imports each, checked with `--check`
  interfaces  the same bundle compiled in `--emit-interfaces` mode, which is
              the writer path as well as the reader

Both report the runtime's own block count (`CINCH_ALLOC_STATS=1`), so they are
comparable to `measure.py`'s arms and to the figures in
`design/compilation-speed.md`.

  python3 experiments/iface-scan/workloads.py --stage 3 --out /tmp/iface-workloads
"""

import argparse
import json
import os
import shutil
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "tooling"))

import pgo  # noqa: E402


def allocations(log, kind):
    for line in log.splitlines():
        if line.startswith("cinch-alloc:"):
            for field in line[len("cinch-alloc:"):].split():
                key, _, value = field.partition("=")
                if key == kind:
                    return int(value)
    return None


def run(compiler, args, cwd):
    env = dict(os.environ)
    env["CINCH_ALLOC_STATS"] = "1"
    env.pop("CINCH_ALLOC_KINDS", None)
    completed = subprocess.run([str(compiler), *args], cwd=cwd, env=env,
                               capture_output=True, text=True)
    log = completed.stdout + completed.stderr
    return completed.returncode, allocations(log, "blocks"), allocations(log, "bytes"), log


def median(values):
    ordered = sorted(v for v in values if v is not None)
    if not ordered:
        return None
    middle = len(ordered) // 2
    return ordered[middle] if len(ordered) % 2 else (ordered[middle - 1] + ordered[middle]) / 2


def main():
    parser = argparse.ArgumentParser(description=__doc__,
                                     formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--stage", type=int, default=3)
    parser.add_argument("--out", default="/tmp/iface-workloads")
    parser.add_argument("--repeats", type=int, default=3)
    parser.add_argument("--manifest", default=None)
    parser.add_argument("--label", default=None)
    args = parser.parse_args()

    compiler = ROOT / "build" / "bootstrap" / f"stage{args.stage}" / "cinch"
    if not compiler.is_file():
        sys.exit(f"no stage{args.stage} compiler at {compiler}")
    manifest = Path(args.manifest or (ROOT / "build" / "bootstrap" / f"stage{args.stage}"
                                      / "base-interfaces.json"))
    if not manifest.is_file():
        sys.exit(f"no interface manifest at {manifest}")

    out = Path(args.out)
    if out.exists():
        shutil.rmtree(out)
    work = out / "work"
    work.mkdir(parents=True)
    bundle = work / "Main.cinch"
    bundle.write_bytes(pgo.workload_source(ROOT))
    lines = bundle.read_text().count("\n")
    print(f"toolkit bundle: {lines} lines")

    modes = {"toolkit": "--check", "interfaces": "--emit-interfaces"}
    results = {}
    for repeat in range(args.repeats):
        order = list(modes)
        if repeat % 2:
            order.reverse()
        for name in order:
            mode = modes[name]
            target = work / f"{name}.out"
            target.unlink(missing_ok=True)
            code, blocks, allocated, log = run(
                compiler, [mode, "--interfaces", str(manifest), str(bundle), str(target)], work)
            row = results.setdefault(name, {"mode": mode, "runs": []})
            row["runs"].append({"exit": code, "blocks": blocks, "bytes": allocated})
            if code != 0:
                (out / f"{name}.log").write_text(log)
                row["log"] = str(out / f"{name}.log")

    for row in results.values():
        row["exit"] = row["runs"][-1]["exit"]
        row["blocks"] = median([r["blocks"] for r in row["runs"]])
        row["allocated_bytes"] = median([r["bytes"] for r in row["runs"]])

    document = {"label": args.label, "stage": args.stage, "compiler": str(compiler),
                "manifest": str(manifest), "bundle_lines": lines,
                "repeats": args.repeats, "workloads": results}
    (out / "workloads.json").write_text(json.dumps(document, indent=1, sort_keys=True))
    for name, row in results.items():
        print(f"  {name:12s} exit={row['exit']} blocks={row['blocks']} bytes={row['allocated_bytes']}")
    print(f"wrote {out / 'workloads.json'}")


if __name__ == "__main__":
    main()
