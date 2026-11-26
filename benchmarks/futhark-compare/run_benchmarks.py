#!/usr/bin/env python3
"""
Independent head-to-head benchmark harness comparing Cinch (Multicore CPU & Apple Metal GPU)
against Futhark 0.25.37 (c, multicore, opencl) on identical inputs, matched numerical
contracts, and pinned toolchains.

Outputs:
  - benchmarks/futhark-compare/results.json
  - benchmarks/futhark-compare/RESULTS.md
"""

import json
import math
import os
import platform
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
BENCH_DIR = Path(__file__).resolve().parent
BUILD_DIR = BENCH_DIR / "build"
WORKER = BUILD_DIR / "bench_worker"
CINCH_BIN = ROOT / "seed" / "target" / "release" / "cinch"

BACKENDS_ALL = [
    "futhark-c", "cinch-seq",
    "futhark-mc", "cinch-cpu", "cinch-ispc",
    "futhark-ocl", "cinch-opencl", "cinch-metal",
]
BACKENDS_FLOAT_EXACT = [
    "futhark-c", "cinch-seq",
    "futhark-mc", "cinch-cpu", "cinch-cpu-exact", "cinch-ispc",
    "futhark-ocl", "cinch-opencl", "cinch-opencl-exact", "cinch-metal", "cinch-metal-exact",
]

CASES = [
    # (workload, size, label, backends, check_kind)
    ("dot_f32", "100000", "Dot F32 (100k)", BACKENDS_FLOAT_EXACT, "float"),
    ("dot_f32", "1000000", "Dot F32 (1M)", BACKENDS_FLOAT_EXACT, "float"),
    ("dot_f32", "10000000", "Dot F32 (10M)", BACKENDS_FLOAT_EXACT, "float"),
    ("scan_i32", "100000", "Inclusive Scan I32 (100k)", BACKENDS_ALL, "hash"),
    ("scan_i32", "1000000", "Inclusive Scan I32 (1M)", BACKENDS_ALL, "hash"),
    ("scan_i32", "5000000", "Inclusive Scan I32 (5M)", BACKENDS_ALL, "hash"),
    ("filter_i32", "100000", "Stable Compaction I32 (100k)", BACKENDS_ALL, "hash"),
    ("filter_i32", "1000000", "Stable Compaction I32 (1M)", BACKENDS_ALL, "hash"),
    ("filter_i32", "5000000", "Stable Compaction I32 (5M)", BACKENDS_ALL, "hash"),
    ("hist_uniform", "262144", "Histogram U8 Uniform (256k)", BACKENDS_ALL, "hash"),
    ("hist_uniform", "4194304", "Histogram U8 Uniform (4M)", BACKENDS_ALL, "hash"),
    ("hist_uniform", "16777216", "Histogram U8 Uniform (16M)", BACKENDS_ALL, "hash"),
    ("hist_skewed", "262144", "Histogram U8 80%-Hot-Bin (256k)", BACKENDS_ALL, "hash"),
    ("hist_skewed", "4194304", "Histogram U8 80%-Hot-Bin (4M)", BACKENDS_ALL, "hash"),
    ("hist_skewed", "16777216", "Histogram U8 80%-Hot-Bin (16M)", BACKENDS_ALL, "hash"),
    ("stencil_f32", "512", "2D 3x3 Stencil F32 3-step (512x512)", BACKENDS_ALL, "float"),
    ("stencil_f32", "1024", "2D 3x3 Stencil F32 3-step (1024x1024)", BACKENDS_ALL, "float"),
    ("stencil_f32", "2048", "2D 3x3 Stencil F32 3-step (2048x2048)", BACKENDS_ALL, "float"),
    ("stencil_hist_many", "1024x64x64", "Target Pipeline: Many Small Frames (1024x64x64, 3 steps)", BACKENDS_ALL, "hash"),
    ("stencil_hist_few", "2x2048x2048", "Target Pipeline: Few Large Frames (2x2048x2048, 3 steps)", BACKENDS_ALL, "hash"),
    ("spmv_csr", "4096", "CSR SpMV F32 w/ 10% Empty Rows (4096 rows, ~65k nnz)", BACKENDS_FLOAT_EXACT, "float"),
    ("spmv_csr", "32768", "CSR SpMV F32 w/ 10% Empty Rows (32768 rows, ~524k nnz)", BACKENDS_FLOAT_EXACT, "float"),
    ("spmv_csr", "131072", "CSR SpMV F32 w/ 10% Empty Rows (131072 rows, ~2.1M nnz)", BACKENDS_FLOAT_EXACT, "float"),
]


def cmd_out(args, cwd=None):
    p = subprocess.run(args, cwd=cwd, capture_output=True, text=True)
    return p.returncode, p.stdout.strip(), p.stderr.strip()


def collect_env():
    _, chip, _ = cmd_out(["sysctl", "-n", "machdep.cpu.brand_string"])
    _, ncpu, _ = cmd_out(["sysctl", "-n", "hw.ncpu"])
    _, mem_b, _ = cmd_out(["sysctl", "-n", "hw.memsize"])
    _, macos_v, _ = cmd_out(["sw_vers", "-productVersion"])
    _, clang_v, _ = cmd_out(["clang", "--version"])
    _, fut_v, _ = cmd_out(["nix", "shell", "nixpkgs#futhark", "--command", "futhark", "--version"])
    _, jj_rev, _ = cmd_out(["jj", "log", "-r", "@", "--no-graph", "-T", "change_id ++ \" \" ++ commit_id"], cwd=ROOT)
    return {
        "chip": chip,
        "cpu_cores": int(ncpu) if ncpu.isdigit() else ncpu,
        "ram_gb": round(int(mem_b) / (1024**3)) if mem_b.isdigit() else mem_b,
        "os": f"macOS {macos_v} ({platform.machine()})",
        "clang": clang_v.splitlines()[0] if clang_v else "unknown",
        "futhark": " / ".join(fut_v.splitlines()[:2]) if fut_v else "unknown",
        "cinch_rev": jj_rev or "working-copy",
    }


def capture_cinch_array_report():
    """Compile benchmarks/futhark-compare/workloads.cinch with Cinch --array-report."""
    if not CINCH_BIN.exists():
        return "(release cinch binary not built)"
    with tempfile.TemporaryDirectory(prefix="cinch_bench_report_") as td:
        tdir = Path(td)
        for rel in ["Data/Array.cinch", "Data/Array/Packed.cinch", "Data/Tensor.cinch",
                    "Data/Array/Compute.cinch", "Data/Device.cinch"]:
            src = ROOT / "seed" / "prelude" / rel
            dst = tdir / rel.replace("/", ".")
            shutil.copy2(src, dst)
        shutil.copy2(BENCH_DIR / "workloads.cinch", tdir / "Workloads.cinch")
        main_src = """module Main where
  import Data.Array
  import Data.Array.Packed
  import Data.Array.Compute
  import Data.Machine
  import Workloads

  main :: Int
  main =
    let xs = array-from-f32s [i64-to-f32 1, i64-to-f32 2, i64-to-f32 3]
        ys = array-from-f32s [i64-to-f32 4, i64-to-f32 5, i64-to-f32 6]
        d = dot-f32-composed xs ys FloatExactTree
        vals = array-from-f32s [i64-to-f32 2, i64-to-f32 3]
        cols = array-from-i32s [wrapping-i64-to-i32 0, wrapping-i64-to-i32 1]
        rptr = array-from-u32s [wrapping-i64-to-u32 0, wrapping-i64-to-u32 2, wrapping-i64-to-u32 2]
        yv = spmv-csr-f32 vals cols rptr xs FloatExactTree
    in if saturating-f32-to-i64 d == 32 && array-length yv == 2 then 0 else 1
"""
        (tdir / "Main.cinch").write_text(main_src)
        rep_path = tdir / "report.txt"
        ir_path = tdir / "out.ll"
        rc, out, err = cmd_out([
            str(CINCH_BIN), "emit-ir", str(tdir / "Main.cinch"),
            "-o", str(ir_path), "--array-report", "--array-report-out", str(rep_path),
        ])
        if rc != 0:
            return f"(cinch emit-ir failed rc={rc}: {err})"
        return rep_path.read_text().strip() if rep_path.exists() else (err or out)


def run_one(backend, workload, size):
    p = subprocess.run(
        [str(WORKER), backend, workload, size, str(BUILD_DIR)],
        capture_output=True, text=True, timeout=180
    )
    if p.returncode != 0:
        return {
            "backend": backend, "workload": workload, "size": size,
            "error": (p.stderr.strip() or f"exit {p.returncode}"),
        }
    line = p.stdout.strip().splitlines()[-1]
    return json.loads(line)


def verify_case(rows, check_kind):
    valid = [r for r in rows if "error" not in r]
    if not valid:
        return "ERROR (no valid runs)"
    ref = valid[0]
    if check_kind == "hash":
        for r in valid[1:]:
            if r["result_hash"] != ref["result_hash"] or r["result_len"] != ref["result_len"]:
                return f"MISMATCH ({r['backend']} hash={r['result_hash']} vs {ref['backend']} hash={ref['result_hash']})"
        return f"BIT-EXACT ({ref['result_hash'][:12]}, len={ref['result_len']})"
    else:
        max_rel = 0.0
        exact_pair = None
        cpu_ex = next((r for r in valid if r["backend"] == "cinch-cpu-exact"), None)
        gpu_ex = next((r for r in valid if r["backend"] == "cinch-metal-exact"), None)
        ocl_ex = next((r for r in valid if r["backend"] == "cinch-opencl-exact"), None)
        if cpu_ex and gpu_ex:
            exact_pair = (
                cpu_ex["result_val"] == gpu_ex["result_val"]
                and (ocl_ex is None or ocl_ex["result_val"] == cpu_ex["result_val"])
            )
        for r in valid[1:]:
            diff = abs(r["result_val"] - ref["result_val"])
            denom = max(1.0, abs(ref["result_val"]))
            rel = diff / denom
            if rel > max_rel:
                max_rel = rel
        status = f"MATCH (val={ref['result_val']:.4f}, max_rel_err={max_rel:.2e}"
        if exact_pair is not None:
            status += f", FloatExactTree CPU==OpenCL==Metal: {'BIT-EXACT' if exact_pair else 'DIFF'}"
        status += ")"
        return status


def format_markdown(env, array_report, all_results):
    lines = []
    lines.append("# Cinch vs. Futhark 0.25.37 — Independent Benchmark Comparison")
    lines.append("")
    lines.append("## Hardware & Pinned Toolchain")
    lines.append("")
    lines.append(f"- **Machine**: {env['chip']} ({env['cpu_cores']} CPU cores, {env['ram_gb']} GB unified RAM)")
    lines.append(f"- **OS**: {env['os']}")
    lines.append(f"- **Futhark**: {env['futhark']}")
    lines.append(f"- **Host C Compiler**: {env['clang']}")
    lines.append(f"- **Cinch Revision**: `{env['cinch_rev']}`")
    lines.append("- **Timing Methodology**: Warm resident time = median of 15 timed iterations after 3 warmups (with explicit GPU/context synchronization per iteration). Transfer-inclusive time = median of 7 full upload → compute → sync → download cycles. Each `(backend, workload, size)` runs in a fresh isolated OS process so `ru_maxrss` reports true unpolluted peak host RSS.")
    lines.append("")
    lines.append("## Cinch Typed Array IR Compiler Report (`workloads.cinch`)")
    lines.append("")
    lines.append("```text")
    lines.append(array_report)
    lines.append("```")
    lines.append("")
    lines.append("## Summary Tables")
    lines.append("")
    for case in all_results:
        lines.append(f"### {case['label']}")
        lines.append("")
        lines.append(f"- **Verification**: {case['verification']}")
        if case["rows"] and case["rows"][0].get("note"):
            lines.append(f"- **Dimensions**: `{case['rows'][0]['note']}`")
        lines.append("")
        lines.append("| Backend | Numerical Contract | Warm Resident (ms) | Transfer-Inclusive (ms) | Peak Host RSS (MB) | Peak Device Mem (MB) | Result |")
        lines.append("| :--- | :--- | ---: | ---: | ---: | ---: | ---: |")
        for r in case["rows"]:
            if "error" in r:
                lines.append(f"| `{r['backend']}` | — | ERR | ERR | — | — | `{r['error']}` |")
            else:
                dev_mem = f"{r['peak_dev_mb']:.2f}" if (r["backend"].startswith("cinch-metal") or r["backend"].startswith("cinch-opencl")) else "—"
                res_s = r["result_hash"][:10] if case["check_kind"] == "hash" else f"{r['result_val']:.4f}"
                lines.append(
                    f"| `{r['backend']}` | {r['contract']} | {r['warm_ms']:.3f} | {r['xfer_ms']:.3f} | {r['peak_rss_mb']:.2f} | {dev_mem} | `{res_s}` |"
                )
        lines.append("")
    return "\n".join(lines) + "\n"


def main():
    env = collect_env()
    print("Environment:", json.dumps(env, indent=2), flush=True)
    array_report = capture_cinch_array_report()
    print("\nCinch Typed Array IR Report:\n" + array_report + "\n", flush=True)

    all_results = []
    for workload, size, label, backends, check_kind in CASES:
        print(f"Running {label}...", flush=True)
        rows = []
        for b in backends:
            r = run_one(b, workload, size)
            rows.append(r)
            if "error" in r:
                print(f"  {b:18s}: ERROR {r['error']}", flush=True)
            else:
                print(
                    f"  {b:18s}: warm={r['warm_ms']:8.3f} ms  xfer={r['xfer_ms']:8.3f} ms  "
                    f"rss={r['peak_rss_mb']:6.2f} MB  dev={r['peak_dev_mb']:6.2f} MB",
                    flush=True,
                )
        ver = verify_case(rows, check_kind)
        print(f"  -> {ver}", flush=True)
        all_results.append({
            "workload": workload,
            "size": size,
            "label": label,
            "check_kind": check_kind,
            "verification": ver,
            "rows": rows,
        })

    out_json = BENCH_DIR / "results.json"
    out_md = BENCH_DIR / "RESULTS.md"
    out_json.write_text(json.dumps({"env": env, "array_report": array_report, "cases": all_results}, indent=2))
    out_md.write_text(format_markdown(env, array_report, all_results))
    print(f"\nWrote {out_json} and {out_md}", flush=True)


if __name__ == "__main__":
    main()
