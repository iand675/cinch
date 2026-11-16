#!/usr/bin/env python3
"""Compile actual UTF8 SIMD/SWAR and exact-load kernels to foreign objects without linking or execution."""
from __future__ import annotations
import argparse
import json
import os
from pathlib import Path
import subprocess
from validate_url import ROOT, BUILD_ROOT, digest


def source() -> str:
    """Extract the maintained primitive declarations and retain every kernel in one reachable entry."""
    result = "-- | Actual UTF8 foreign-object kernels.\nmodule Main where\n  import Data.Bytes\n  import Data.Machine\n"
    text = (ROOT / "compiler/prelude/Data/Bytes/Utf8.cinch").read_text()
    for name in ["load-one", "load-four", "load-eight", "ascii-eight", "decode-word", "encode-word"]:
        start = text.index(f"  prim {name} ::")
        end = text.find("\n  -- |", start)
        result += "\n  -- | Actual retained UTF-8 primitive.\n" + text[start:end if end != -1 else None]
    return result + "\n  -- | Retain source storage throughout exact private reads.\n  keep :: forall a. Bytes -> a -> a\n  keep bytes value = case bytes-len bytes of _ -> value\n  -- | Reach every arithmetic and load kernel without foreign execution.\n  main :: Int\n  main = let bytes = bytes-new 8 in let base = bytes-data bytes in let one = keep bytes (load-one base) in let four = keep bytes (load-four base) in let eight = keep bytes (load-eight base) in let decoded = decode-word four one in let encoded = encode-word (wrapping-u32-to-i32 decoded) in if ascii-eight eight && eq-u64 encoded 0 then 0 else 1\n"


def main() -> None:
    """Use the frozen Rust emitter for supported foreign profiles and preserve exact object identities."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--cpu", type=int, default=2)
    parser.add_argument("--artifacts", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    os.sched_setaffinity(0, {args.cpu})
    artifacts = args.artifacts.resolve()
    artifacts.mkdir(parents=True, exist_ok=True)
    main = artifacts / "Main.cinch"
    main.write_text(source())
    compiler = BUILD_ROOT / "prim-wide-frozen/cinch"
    rows = []
    for triple in ["wasm32-unknown-unknown", "aarch64-unknown-linux-gnu"]:
        ir = artifacts / f"{triple}.ll"
        obj = ir.with_suffix(".o")
        commands = [[str(compiler), "emit-ir", str(main), "--target", triple, "-o", str(ir)], [os.environ.get("CINCH_CLANG", "clang"), "-target", triple, "-O2", "-c", "-x", "ir", str(ir), "-o", str(obj)]]
        for command in commands:
            result = subprocess.run(command, capture_output=True, text=True)
            if result.returncode:
                raise RuntimeError(f"{command}: {result.stdout}\n{result.stderr}")
        rows.append({"target": triple, "commands": commands, "llvm_sha256": digest(ir), "object_sha256": digest(obj), "object_bytes": obj.stat().st_size})
    report = {"compiler_sha256": digest(compiler), "kernel_source_sha256": digest(main), "cpu": args.cpu, "results": rows, "all_passed": True, "big_endian_gate": "Provided in lib_utf8 Rust target; standalone CLI rejects the unsupported powerpc64 profile, so no fallback target is substituted."}
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(report, indent=2) + "\n")
    print("Actual UTF8 kernels compile for WebAssembly and AArch64", flush=True)


if __name__ == "__main__":
    main()
