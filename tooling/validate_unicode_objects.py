#!/usr/bin/env python3
"""Compile actual Unicode table SIMD/SWAR and exact-load kernels to foreign objects without linking or execution."""
from __future__ import annotations
import argparse
import json
import os
from pathlib import Path
import subprocess
from validate_url import ROOT, BUILD_ROOT, digest


def source(path: Path) -> str:
    """Extract the maintained primitive declarations and retain every kernel in one reachable entry."""
    result = "-- | Actual Unicode table foreign-object kernels.\nmodule Main where\n  import Data.Bytes\n  import Data.Machine\n"
    text = path.read_text()
    for name in ['table-add','table-sub','table-and','table-shr','table-lt','table-le']:
        start = text.index(f"  prim {name} ::")
        end = text.find("\n  -- |", start)
        result += "\n  -- | Actual retained UTF-8 primitive.\n" + text[start:end if end != -1 else None]
    return result + '\n  -- | Read exactly one caller-proved source word for an unknown live kernel input.\n  prim load-probe :: I64 -> U32 where\n    unsafe llvm {\n      %p = inttoptr i64 %0 to ptr\n      %word = load i32, ptr %p, align 1\n      ret i32 %word\n    }\n  -- | Keep the source owner live across its exact private input load.\n  keep :: forall a. Bytes -> a -> a\n  keep bytes value = case bytes-len bytes of _ -> value\n  -- | Make every predicate result independently observable.\n  flag :: Bool -> U32 -> U32\n  flag value mask = if value then mask else 0\n  -- | Retain the actual scalar kernels on an unknown input, without foreign execution.\n  main :: Int\n  main = let bytes = bytes-new 4 in let base = bytes-data bytes in let value = keep bytes (load-probe base) in\n    let a = table-add value 7 in let b = table-sub a 19 in let c = table-shr b value in\n    let d = table-and c value in let less = table-lt d a in let equal = table-le a b in\n    let x = flag less 1 in let y = flag equal 2 in let bits = table-add x y in\n    let score = table-add d bits in if eq-u32 score value then 0 else 1\n'


def main() -> None:
    """Use the frozen Rust emitter for supported foreign profiles and preserve exact object identities."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--source", type=Path, default=ROOT / "compiler/prelude/Data/Unicode/Tables.cinch")
    parser.add_argument("--cpu", type=int, default=2)
    parser.add_argument("--artifacts", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    os.sched_setaffinity(0, {args.cpu})
    artifacts = args.artifacts.resolve()
    artifacts.mkdir(parents=True, exist_ok=True)
    main = artifacts / "Main.cinch"
    main.write_text(source(args.source))
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
    report = {"compiler_sha256": digest(compiler), "kernel_source_sha256": digest(main), "cpu": args.cpu, "results": rows, "all_passed": True, "big_endian_gate": "Provided in lib_unicode_kernel Rust target; standalone CLI rejects the unsupported powerpc64 profile, so no fallback target is substituted."}
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(report, indent=2) + "\n")
    print("Actual Unicode table kernels compile for WebAssembly and AArch64", flush=True)


if __name__ == "__main__":
    main()
