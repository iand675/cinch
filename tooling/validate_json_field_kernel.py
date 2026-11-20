#!/usr/bin/env python3
"""Compile actual load-only JSON field/key classifiers into little- and big-endian foreign objects."""
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
    """Identify exact source and foreign object bytes for the compilation receipt."""
    return hashlib.sha256(path.read_bytes()).hexdigest()


def run(command: list[str], directory: Path) -> str:
    """Preserve compiler diagnostics without requiring any foreign runtime execution."""
    def host_stack() -> None:
        """Apply the established compiler-host main-stack limit only to this child."""
        _, hard = resource.getrlimit(resource.RLIMIT_STACK)
        resource.setrlimit(resource.RLIMIT_STACK, (64 * 1024 * 1024, hard))
    result = subprocess.run(command, cwd=directory, text=True, capture_output=True, timeout=60, preexec_fn=host_stack)
    if result.returncode:
        raise RuntimeError(f"command failed: {command}\n{result.stdout}\n{result.stderr}")
    return result.stdout + result.stderr


def main() -> None:
    """Extract complete production primitives and compile reachable calls without claiming foreign execution."""
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
    production = ROOT / "seed/prelude/Text/Json/Codec.cinch"
    source = production.read_text()
    start = source.index("  prim folded-ascii16 ::")
    end = source.index("\n  -- |", start)
    primitive = source[start:end]
    for name in ["key-ascii16", "key-ascii-byte"]:
        begin = source.index(f"  prim {name} ::")
        finish = source.index("\n  -- |", begin)
        primitive += "\n  -- | Actual complete-interval ASCII key classifier.\n" + source[begin:finish]
    consumer = "-- | Compile complete-interval JSON name loads for foreign targets.\nmodule Main where\n  import Data.Machine\n  -- | Exact production classifier; both callers retain complete sixteen-byte owners.\n" + primitive + '''
  -- | Keep a byte owner live through its strict primitive argument.
  keep :: Bytes -> U8 -> U8
  keep bytes result = case bytes-len bytes of _ -> result
  -- | Retain both policy branches without claiming execution on a foreign target.
  main :: Int
  main = let bytes = bytes-new 16 in let a = keep bytes (folded-ascii16 (bytes-data bytes) (bytes-data bytes) 0) in let b = keep bytes (folded-ascii16 (bytes-data bytes) (bytes-data bytes) 1) in let ascii = keep bytes (key-ascii16 (bytes-data bytes)) in let fringe = keep bytes (key-ascii-byte (bytes-data bytes)) in if eq-u8 a 2 && eq-u8 b 2 && eq-u8 ascii 1 && eq-u8 fringe 1 then 0 else 1
'''
    entry = directory / "Main.cinch"
    entry.write_text(consumer)
    (directory / "validator.py").write_bytes(Path(__file__).read_bytes())
    report = {"production_source_sha256": digest(production), "primitive_sha256": hashlib.sha256(primitive.encode()).hexdigest(), "consumer_sha256": digest(entry), "compiler_sha256": digest(compiler), "runner_sha256": digest(Path(__file__)), "cpu_affinity": [args.cpu], "compiler_host_stack_kib": 65536, "foreign_execution_claimed": False, "targets": []}
    for triple, name in [("wasm32-unknown-unknown", "wasm"), ("aarch64-unknown-linux-gnu", "arm")]:
        ir = directory / f"{name}.ll"
        output = directory / f"{name}.o"
        diagnostics = run([str(compiler), "emit-ir", str(entry), "--target", triple, "-o", str(ir)], directory)
        llvm = ir.read_text()
        if "load <16 x i8>" not in llvm:
            raise RuntimeError("the actual complete-interval vector loads disappeared from the foreign input")
        diagnostics += run([os.environ.get("CINCH_CLANG", "clang"), "-target", triple, "-O2", "-c", "-x", "ir", str(ir), "-o", str(output)], directory)
        (directory / f"{name}.log").write_text(diagnostics)
        report["targets"].append({"triple": triple, "llvm_sha256": digest(ir), "object_sha256": digest(output), "object_bytes": output.stat().st_size})
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(report, indent=2) + "\n")


if __name__ == "__main__":
    main()
