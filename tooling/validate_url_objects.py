#!/usr/bin/env python3
"""Compile actual URL SIMD/SWAR and exact-load kernels to foreign objects without linking or execution."""
from __future__ import annotations
import argparse
import json
import os
from pathlib import Path
import subprocess
from validate_url import ROOT, BUILD_ROOT, digest


def source() -> str:
    """Extract the maintained primitive declarations and retain every kernel in one reachable entry."""
    result = "-- | Actual URL foreign-object kernels.\nmodule Main where\n  import Data.Bytes\n  import Data.Machine\n"
    for module, names in [("Codec", ["load-eight", "load-one", "classify-one", "classify-eight", "decode-flags-eight", "hex-pair"]), ("Scan", ["mask-eight", "population"])]:
        text = (ROOT / f"compiler/prelude/Text/Url/{module}.cinch").read_text()
        for name in names:
            start = text.index(f"  prim {name} ::")
            end = text.find("\n  -- |", start)
            result += "\n  -- | Actual retained URL primitive.\n" + text[start:end if end != -1 else None]
    return result + "\n  -- | Retain backing storage throughout one exact private load.\n  keep :: forall a. Bytes -> a -> a\n  keep bytes value = case bytes-len bytes of _ -> value\n  -- | Retain every actual primitive without foreign execution.\n  main :: Int\n  main = let bytes = bytes-new 8 in let address = bytes-data bytes in let word = keep bytes (load-eight address) in let byte = keep bytes (load-one address) in let single = classify-one byte byte in let encoded = classify-eight word byte in let decoded = decode-flags-eight word byte in let mask = mask-eight word single decoded in let count = population mask in let pair = hex-pair byte single in let folded = bit-xor-u64 count (u16-to-u64 pair) in if eq-u64 folded encoded then 0 else 1\n"


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
    report = {"compiler_sha256": digest(compiler), "kernel_source_sha256": digest(main), "cpu": args.cpu, "results": rows, "all_passed": True, "big_endian_gate": "Provided in lib_url Rust target; standalone CLI rejects the unsupported powerpc64 profile, so no fallback target is substituted."}
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(report, indent=2) + "\n")
    print("Actual URL kernels compile for WebAssembly and AArch64", flush=True)


if __name__ == "__main__":
    main()
