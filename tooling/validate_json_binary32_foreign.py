#!/usr/bin/env python3
"""Compile the exact production binary32 representation primitives for foreign targets."""
from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import resource
import subprocess

ROOT = Path(__file__).resolve().parents[1]


def digest(path: Path) -> str:
    """Record an exact production, compiler or object identity."""
    return hashlib.sha256(path.read_bytes()).hexdigest()


def execute(command: list[str], directory: Path) -> str:
    """Compile with the established host stack and preserve every diagnostic."""
    def host_stack() -> None:
        """Set the compiler host stack without changing consumer bounds."""
        _, hard = resource.getrlimit(resource.RLIMIT_STACK)
        resource.setrlimit(resource.RLIMIT_STACK, (64 * 1024 * 1024, hard))
    result = subprocess.run(command, cwd=directory, text=True, capture_output=True,
                            timeout=60, preexec_fn=host_stack)
    if result.returncode:
        raise RuntimeError(f"command failed: {command}\n{result.stdout}\n{result.stderr}")
    return result.stdout + result.stderr


def main() -> None:
    """Extract both actual bitcasts and compile reachable calls without claiming foreign execution."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--compiler", type=Path, required=True)
    parser.add_argument("--artifacts", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--cpu", type=int, default=2)
    args = parser.parse_args()
    os.sched_setaffinity(0, {args.cpu})
    compiler = args.compiler.resolve()
    directory = args.artifacts.resolve()
    directory.mkdir(parents=True, exist_ok=True)
    production = ROOT / "compiler/prelude/Text/Json/Binary32.cinch"
    source = production.read_text()
    primitives = []
    for name in ["binary32-bits", "binary32-value"]:
        begin = source.index(f"  prim {name} ::")
        end = source.index("\n  -- |", begin)
        primitives.append("  -- | Actual production representation primitive.\n" + source[begin:end])
    consumer = "-- | Reach both production binary32 bitcasts on foreign targets.\nmodule Main where\n  import Data.Machine\n" + "\n".join(primitives) + '''
  -- | Keep runtime bit representations reachable without a foreign execution claim.
  main :: Int
  main = let owner = bytes-new 4 in let bits = wrapping-i64-to-u32 (bytes-data owner) in if eq-u32 (binary32-bits (binary32-value bits)) bits then 0 else 1
'''
    entry = directory / "Main.cinch"
    entry.write_text(consumer)
    (directory / "validator.py").write_bytes(Path(__file__).read_bytes())
    report = {"production_source_sha256": digest(production), "compiler_sha256": digest(compiler),
              "runner_sha256": digest(Path(__file__)), "consumer_sha256": digest(entry),
              "cpu_affinity": [args.cpu], "compiler_host_stack_kib": 65536,
              "foreign_execution_claimed": False, "targets": []}
    for triple, name in [("wasm32-unknown-unknown", "wasm"), ("aarch64-unknown-linux-gnu", "arm")]:
        ir = directory / f"{name}.ll"
        output = directory / f"{name}.o"
        diagnostics = execute([str(compiler), "emit-ir", str(entry), "--target", triple, "-o", str(ir)], directory)
        llvm = ir.read_text()
        if "bitcast i32" not in llvm or "bitcast float" not in llvm:
            raise RuntimeError("the actual production representation bitcasts disappeared from the foreign input")
        diagnostics += execute([os.environ.get("CINCH_CLANG", "clang"), "-target", triple, "-O2", "-c", "-x", "ir", str(ir), "-o", str(output)], directory)
        (directory / f"{name}.log").write_text(diagnostics)
        report["targets"].append({"triple": triple, "llvm_sha256": digest(ir),
                                  "object_sha256": digest(output), "object_bytes": output.stat().st_size})
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(report, indent=2) + "\n")


if __name__ == "__main__":
    main()
