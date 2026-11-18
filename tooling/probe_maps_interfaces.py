#!/usr/bin/env python3
"""Observe source-absent map contracts without restamping producer metadata."""
from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import subprocess


CLIENTS = {
    "policy-identity": "  import Data.Mutable.Map (MapKeyPolicy)\n  -- | Pass the imported opaque policy without interpreting its fields.\n  observe :: forall k. MapKeyPolicy k -> MapKeyPolicy k\n  observe value = value\n",
    "map-identity": "  import Data.Mutable.Map (MutableMap)\n  -- | Preserve an imported opaque carrier in a pure declaration.\n  observe :: forall r. MutableMap r Nat Nat -> MutableMap r Nat Nat\n  observe value = value\n",
    "map-size": "  import Data.Mutable.Map (MutableMap, mutable-map-size)\n  -- | Read a region-scoped map through its genuine imported contract.\n  observe :: forall r. MutableMap r Nat Nat ->{Mut r} Nat\n  observe map = mutable-map-size map\n",
    "map-copy": "  import Data.Mutable.Map (MutableMap, MapError, mutable-map-copy)\n  -- | Keep distinct destination/source regions in the imported bulk operation.\n  observe :: forall r s. MutableMap r Nat Nat -> MutableMap s Nat Nat ->{Mut r, Mut s} Result MapError Unit\n  observe destination source = mutable-map-copy destination source\n",
    "map-all": "  import Data.Mutable.Map (MutableMap, mutable-map-all)\n  -- | Preserve the callback's complete effect tail across imported live traversal.\n  observe :: forall r ef. MutableMap r Nat Nat -> (Nat -> Nat ->{ef} Bool) ->{Mut r | ef} Bool\n  observe map action = mutable-map-all map action\n",
}


def digest(path: Path) -> str:
    """Identify producer artifacts and consumers by their actual bytes."""
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main() -> None:
    """Probe actual admission; a completed observation run is not a package qualification."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--producer", type=Path, required=True, help="Actual Rust check/build output containing Main.manifest.json and .cinchif files")
    parser.add_argument("--producer-receipt", type=Path, required=True)
    parser.add_argument("--compiler", type=Path, action="append", required=True, help="Exact raw Native CLI; repeat for independent readers")
    parser.add_argument("--cases", nargs="+", choices=tuple(CLIENTS), default=list(CLIENTS))
    parser.add_argument("--cpu", type=int, default=4)
    parser.add_argument("--artifacts", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--require-all-admitted", action="store_true")
    args = parser.parse_args()
    os.sched_setaffinity(0, {args.cpu})
    producer, artifacts = args.producer.resolve(), args.artifacts.resolve()
    interfaces = artifacts / "interfaces"
    interfaces.mkdir(parents=True, exist_ok=True)
    rows, input_hashes = [], {}
    for row in json.loads((producer / "Main.manifest.json").read_text())["modules"]:
        name = row["module"]
        if name == "Main":
            continue
        path = producer / (name + ".cinchif")
        copied = interfaces / path.name
        copied.write_bytes(path.read_bytes())
        assert copied.read_bytes() == path.read_bytes()
        rows.append([name, row["iface_hash"], str(copied)])
        input_hashes[name] = digest(copied)
    manifest = artifacts / "interfaces.json"
    manifest.write_text(json.dumps({"schema": 1, "interfaces": rows}, separators=(",", ":")))
    report = {"kind": "source-absent-admission-observations", "producer_receipt_sha256": digest(args.producer_receipt),
              "producer_compiler_sha256": json.loads(args.producer_receipt.read_text())["compiler_sha256"],
              "producer_manifest_sha256": digest(producer / "Main.manifest.json"), "interfaces": input_hashes,
              "interface_bytes_modified": False, "library_sources_in_consumer_directory": False, "runs": []}
    args.output.parent.mkdir(parents=True, exist_ok=True)
    for number, compiler in enumerate(args.compiler):
        compiler = compiler.resolve()
        for name in args.cases:
            directory = artifacts / f"reader-{number}-{name}"
            directory.mkdir(exist_ok=True)
            source = directory / "Main.cinch"
            source.write_text("-- | Consume only genuine source-absent interface declarations.\nmodule Main where\n" + CLIENTS[name]
                              + "  -- | Keep this an interface checker fixture.\n  main :: Int\n  main = 0\n")
            output = directory / "checked"
            command = [str(compiler), "--check", "--interfaces", str(manifest), str(source), str(output)]
            result = subprocess.run(command, env={**os.environ, "CINCH_STACK_SIZE": "64M", "CINCH_LEAK_CHECK": "1", "CINCH_RC_DEBUG": "0"}, capture_output=True, text=True, timeout=90)
            diagnostics = Path(str(output) + ".diagnostics")
            row = {"case": name, "compiler_sha256": digest(compiler), "consumer_sha256": digest(source), "exit": result.returncode,
                   "admitted": result.returncode == 0, "stdout": result.stdout, "stderr": result.stderr,
                   "diagnostics": diagnostics.read_text() if diagnostics.exists() else ""}
            report["runs"].append(row)
            args.output.write_text(json.dumps(report, indent=2) + "\n")
            print(f"{number} {name}: {'admitted' if row['admitted'] else 'refused'}", flush=True)
    report["all_admitted"] = all(row["admitted"] for row in report["runs"])
    args.output.write_text(json.dumps(report, indent=2) + "\n")
    if args.require_all_admitted and not report["all_admitted"]:
        raise SystemExit(1)


if __name__ == "__main__":
    main()
