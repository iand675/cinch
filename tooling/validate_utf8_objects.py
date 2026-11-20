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
    text = (ROOT / "seed/prelude/Data/Bytes/Utf8.cinch").read_text()
    for name in ['load-one', 'load-four', 'load-eight', 'ascii-eight', 'ascii-sixteen', 'decode-word', 'encode-word', 'scan-lt', 'scan-ge', 'scan-gt', 'scan-eq', 'scan-add', 'scan-sub', 'scan-address', 'packed-width', 'packed-valid', 'scan-simple', 'scan-ready', 'scan-four', 'scan-from-packed', 'scan-width', 'scan-valid']:
        start = text.index(f"  prim {name} ::")
        end = text.find("\n  -- |", start)
        result += "\n  -- | Actual retained UTF-8 primitive.\n" + text[start:end if end != -1 else None]
    return result + '\n  -- | Retain source storage throughout exact private reads.\n  keep :: forall a. Bytes -> a -> a\n  keep bytes value = case bytes-len bytes of _ -> value\n  -- | Give each computed boolean a distinct observable result bit.\n  flag :: Bool -> U64 -> U64\n  flag value mask = if value then mask else 0\n  -- | Reach actual read-only kernels without algebraically contradictory exit conditions.\n  main :: Int\n  main = let bytes = bytes-new 16 in let base = bytes-data bytes in\n    let one = keep bytes (load-one base) in let four = keep bytes (load-four base) in\n    let eight = keep bytes (load-eight base) in let address = scan-address base eight in\n    let packed = decode-word four one in let encoded = encode-word (wrapping-u32-to-i32 packed) in\n    let simple = scan-simple one in let checked = scan-four four in let converted = scan-from-packed packed in\n    let a = scan-width checked in let b = packed-width packed in let next = scan-add a b in let rest = scan-sub encoded next in\n    let x = bit-or-u64 (flag (ascii-eight eight) 1) (flag (keep bytes (ascii-sixteen base)) 2) in\n    let y = bit-or-u64 (flag (scan-lt next rest) 4) (flag (scan-ge rest eight) 8) in\n    let z = bit-or-u64 (flag (scan-gt encoded next) 16) (flag (scan-eq next rest) 32) in\n    let p = bit-or-u64 (flag (packed-valid packed) 64) (flag (scan-valid converted) 128) in\n    let q = bit-or-u64 (flag (scan-ready simple) 256) (flag (ne-i64 address base) 512) in\n    let flags = bit-or-u64 (bit-or-u64 x y) (bit-or-u64 (bit-or-u64 z p) q) in\n    let score = scan-add rest flags in if eq-u64 score eight then 0 else 1\n'


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
