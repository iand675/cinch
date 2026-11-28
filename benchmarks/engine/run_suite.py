#!/usr/bin/env python3
"""Reproducible benchmark & hardware-counter harness for Cinch Engine Waves 1-7.

Builds and executes `benchmarks/engine/microbench.cinch` (verifying `allocs == 0`,
`atomic_rc == 0`, and optional `CINCH_RC_DEBUG=1 CINCH_LEAK_CHECK=1`), optionally
captures Linux `perf stat` hardware counters (`--perf`), optionally drives external
HTTP/1.1 (`wrk`/`wrk2`), HTTP/2 (`h2load`), and KV (`redis-benchmark`) load
generators (`--external`), and writes a structured JSON report (`--out`).
"""

from __future__ import annotations

import argparse
import datetime as dt
import json
import os
import platform
import re
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path
from typing import Any

HERE = Path(__file__).resolve().parent
REPO_ROOT = HERE.parents[1]
DEFAULT_MICROBENCH = HERE / "microbench.cinch"

PERF_EVENTS = [
    "cycles",
    "instructions",
    "L1-dcache-load-misses",
    "LLC-load-misses",
    "branch-misses",
    "cs",
]


def cmd_out(cmd: list[str], cwd: Path | None = None, env: dict[str, str] | None = None) -> subprocess.CompletedProcess[str]:
    return subprocess.run(cmd, cwd=cwd, env=env, capture_output=True, text=True)


def resolve_compiler(explicit: str | None) -> Path:
    if explicit:
        p = Path(explicit)
        if p.is_file():
            return p.resolve()
        found = shutil.which(explicit)
        if found:
            return Path(found).resolve()
        raise FileNotFoundError(f"specified compiler not found: {explicit}")

    candidates = [
        REPO_ROOT / "seed" / "target" / "release" / "cinch",
        REPO_ROOT / "build" / "bootstrap" / "stage3" / "cinch",
        REPO_ROOT / "seed" / "target" / "debug" / "cinch",
    ]
    for c in candidates:
        if c.is_file():
            return c.resolve()
    on_path = shutil.which("cinch")
    if on_path:
        return Path(on_path).resolve()
    raise FileNotFoundError(
        "could not locate Cinch compiler; tried seed/target/release/cinch, "
        "build/bootstrap/stage3/cinch, seed/target/debug/cinch, and PATH"
    )


def collect_host_info(compiler: Path, opt_level: int) -> dict[str, Any]:
    sys_name = platform.system()
    cpu = platform.processor() or platform.machine()
    ram_gb: Any = "unknown"
    os_desc = f"{sys_name} {platform.release()} ({platform.machine()})"

    if sys_name == "Darwin":
        r = cmd_out(["sysctl", "-n", "machdep.cpu.brand_string"])
        if r.returncode == 0 and r.stdout.strip():
            cpu = r.stdout.strip()
        r_mem = cmd_out(["sysctl", "-n", "hw.memsize"])
        if r_mem.returncode == 0 and r_mem.stdout.strip().isdigit():
            ram_gb = round(int(r_mem.stdout.strip()) / (1024**3))
        r_ver = cmd_out(["sw_vers", "-productVersion"])
        if r_ver.returncode == 0 and r_ver.stdout.strip():
            os_desc = f"macOS {r_ver.stdout.strip()} ({platform.machine()})"
    elif sys_name == "Linux":
        cpuinfo = Path("/proc/cpuinfo")
        if cpuinfo.is_file():
            for line in cpuinfo.read_text(encoding="utf-8", errors="replace").splitlines():
                if line.lower().startswith("model name") and ":" in line:
                    cpu = line.split(":", 1)[1].strip()
                    break
        meminfo = Path("/proc/meminfo")
        if meminfo.is_file():
            for line in meminfo.read_text(encoding="utf-8", errors="replace").splitlines():
                if line.startswith("MemTotal:"):
                    parts = line.split()
                    if len(parts) >= 2 and parts[1].isdigit():
                        ram_gb = round(int(parts[1]) / (1024**2))
                    break

    rev = "working-copy"
    jj_res = cmd_out(["jj", "log", "-r", "@", "--no-graph", "-T", 'change_id ++ " " ++ commit_id'], cwd=REPO_ROOT)
    if jj_res.returncode == 0 and jj_res.stdout.strip():
        rev = jj_res.stdout.strip()
    else:
        git_res = cmd_out(["git", "rev-parse", "--short", "HEAD"], cwd=REPO_ROOT)
        if git_res.returncode == 0 and git_res.stdout.strip():
            rev = git_res.stdout.strip()

    return {
        "timestamp_utc": dt.datetime.now(dt.timezone.utc).isoformat(),
        "os": os_desc,
        "arch": platform.machine(),
        "cpu": cpu,
        "cpu_cores": os.cpu_count() or 1,
        "ram_gb": ram_gb,
        "compiler": str(compiler),
        "opt_level": opt_level,
        "revision": rev,
    }


def build_microbench(compiler: Path, source: Path, out_bin: Path, opt_level: int) -> dict[str, Any]:
    cmd = [str(compiler), "build", "--opt-level", str(opt_level), "-o", str(out_bin), str(source)]
    proc = cmd_out(cmd, cwd=REPO_ROOT)
    return {
        "command": cmd,
        "returncode": proc.returncode,
        "stdout": proc.stdout.strip(),
        "stderr": proc.stderr.strip(),
        "passed": proc.returncode == 0 and out_bin.is_file(),
    }


def parse_microbench_output(stdout: str) -> list[dict[str, Any]]:
    rows: list[dict[str, Any]] = []
    for raw in stdout.splitlines():
        line = raw.strip()
        if not line.startswith("{"):
            continue
        item = json.loads(line)
        ops = int(item.get("ops", 0))
        elapsed_ns = int(item.get("elapsed_ns", 0))
        denom = elapsed_ns if elapsed_ns > 0 else 1
        item["ns_per_op"] = round(elapsed_ns / ops, 3) if ops > 0 else 0.0
        item["ops_per_sec"] = round((ops * 1_000_000_000) / denom, 2)
        if "bytes" in item:
            total_bytes = int(item["bytes"])
            item["gb_per_sec"] = round(total_bytes / denom, 3)
        rows.append(item)
    return rows


def run_microbench(bin_path: Path, leak_check: bool) -> dict[str, Any]:
    env = dict(os.environ)
    if leak_check:
        env["CINCH_RC_DEBUG"] = "1"
        env["CINCH_LEAK_CHECK"] = "1"
    proc = cmd_out([str(bin_path)], env=env)
    rows = parse_microbench_output(proc.stdout)

    leak_clean = True
    if leak_check:
        stderr_l = proc.stderr.lower()
        leak_clean = ("leaked=0" in stderr_l) or (
            "leak" not in stderr_l and proc.returncode == 0
        )

    zero_alloc_violations = [
        r["metric"]
        for r in rows
        if int(r.get("allocs", 1)) != 0 or int(r.get("atomic_rc", 1)) != 0
    ]

    passed = (
        proc.returncode == 0
        and len(rows) > 0
        and len(zero_alloc_violations) == 0
        and leak_clean
    )
    return {
        "passed": passed,
        "returncode": proc.returncode,
        "leak_check": leak_check,
        "leak_clean": leak_clean,
        "zero_alloc_violations": zero_alloc_violations,
        "metrics": rows,
        "stderr": proc.stderr.strip(),
    }


def run_perf_stat(bin_path: Path, requested: bool) -> dict[str, Any]:
    if not requested:
        return {"status": "skipped", "reason": "not requested (pass --perf on Linux)"}
    if platform.system() != "Linux":
        return {"status": "skipped", "reason": f"perf stat requires Linux (host is {platform.system()})"}
    perf_bin = shutil.which("perf")
    if not perf_bin:
        return {"status": "skipped", "reason": "tool not installed: perf"}

    events_arg = ",".join(PERF_EVENTS)
    cmd = [perf_bin, "stat", "-x,", "-e", events_arg, "--", str(bin_path)]
    proc = cmd_out(cmd)
    if proc.returncode != 0:
        return {
            "status": "skipped",
            "reason": f"perf stat failed (returncode={proc.returncode}); check perf_event_paranoid",
            "stderr": proc.stderr.strip(),
        }

    counters: dict[str, Any] = {}
    for line in proc.stderr.splitlines():
        parts = [p.strip() for p in line.split(",")]
        if len(parts) < 3:
            continue
        val_raw, _unit, event_name = parts[0], parts[1], parts[2]
        if not event_name:
            continue
        if val_raw in ("<not supported>", "<not counted>", ""):
            counters[event_name] = val_raw or "unavailable"
        else:
            try:
                counters[event_name] = int(val_raw) if val_raw.isdigit() else float(val_raw)
            except ValueError:
                counters[event_name] = val_raw

    ipc = None
    if isinstance(counters.get("cycles"), (int, float)) and isinstance(counters.get("instructions"), (int, float)):
        if counters["cycles"] > 0:
            ipc = round(counters["instructions"] / counters["cycles"], 3)

    return {
        "status": "ok",
        "events": PERF_EVENTS,
        "counters": counters,
        "ipc": ipc,
    }


def _parse_wrk_rps(stdout: str) -> float | None:
    m = re.search(r"Requests/sec:\s*([0-9.]+)", stdout)
    return float(m.group(1)) if m else None


def _parse_h2load_rps(stdout: str) -> float | None:
    m = re.search(r"finished in [^,]+,\s*([0-9.]+)\s*req/s", stdout)
    return float(m.group(1)) if m else None


def run_external_benchmarks(args: argparse.Namespace) -> dict[str, Any]:
    if not args.external:
        return {"status": "skipped", "reason": "not requested (pass --external)", "tools": {}}

    results: dict[str, Any] = {}

    # 1. wrk (HTTP/1.1 throughput)
    wrk = shutil.which("wrk")
    if not wrk:
        results["wrk"] = {"status": "skipped", "reason": "tool not installed"}
    elif not args.http1_url:
        results["wrk"] = {"status": "skipped", "reason": "no --http1-url target specified"}
    else:
        cmd = [wrk, f"-t{args.threads}", f"-c{args.connections}", f"-d{args.duration}s", "--latency", args.http1_url]
        proc = cmd_out(cmd)
        results["wrk"] = {
            "status": "ok" if proc.returncode == 0 else "error",
            "command": cmd,
            "returncode": proc.returncode,
            "requests_per_sec": _parse_wrk_rps(proc.stdout),
            "stdout": proc.stdout.strip(),
            "stderr": proc.stderr.strip(),
        }

    # 2. wrk2 (coordinated-omission-free HTTP/1.1 tail latency)
    wrk2 = shutil.which("wrk2")
    if not wrk2:
        results["wrk2"] = {"status": "skipped", "reason": "tool not installed"}
    elif not args.http1_url:
        results["wrk2"] = {"status": "skipped", "reason": "no --http1-url target specified"}
    else:
        cmd = [
            wrk2,
            f"-t{args.threads}",
            f"-c{args.connections}",
            f"-d{args.duration}s",
            f"-R{args.wrk2_rate}",
            "--latency",
            args.http1_url,
        ]
        proc = cmd_out(cmd)
        results["wrk2"] = {
            "status": "ok" if proc.returncode == 0 else "error",
            "command": cmd,
            "returncode": proc.returncode,
            "requests_per_sec": _parse_wrk_rps(proc.stdout),
            "stdout": proc.stdout.strip(),
            "stderr": proc.stderr.strip(),
        }

    # 3. h2load (HTTP/2 & HTTP/3 multiplexed throughput)
    h2load = shutil.which("h2load")
    if not h2load:
        results["h2load"] = {"status": "skipped", "reason": "tool not installed"}
    elif not args.http2_url:
        results["h2load"] = {"status": "skipped", "reason": "no --http2-url target specified"}
    else:
        cmd = [
            h2load,
            f"-t{args.threads}",
            f"-c{args.connections}",
            "-m64",
            f"-D{args.duration}",
            args.http2_url,
        ]
        proc = cmd_out(cmd)
        results["h2load"] = {
            "status": "ok" if proc.returncode == 0 else "error",
            "command": cmd,
            "returncode": proc.returncode,
            "requests_per_sec": _parse_h2load_rps(proc.stdout),
            "stdout": proc.stdout.strip(),
            "stderr": proc.stderr.strip(),
        }

    # 4. redis-benchmark (RESP3 / share-nothing KV)
    redis_bench = shutil.which("redis-benchmark")
    if not redis_bench:
        results["redis-benchmark"] = {"status": "skipped", "reason": "tool not installed"}
    elif not args.kv_host:
        results["redis-benchmark"] = {"status": "skipped", "reason": "no --kv-host target specified"}
    else:
        cmd = [
            redis_bench,
            "-h",
            args.kv_host,
            "-p",
            str(args.kv_port),
            "-c",
            str(args.connections),
            "-n",
            "200000",
            "-P",
            "16",
            "-t",
            "get,set",
            "--csv",
        ]
        proc = cmd_out(cmd)
        results["redis-benchmark"] = {
            "status": "ok" if proc.returncode == 0 else "error",
            "command": cmd,
            "returncode": proc.returncode,
            "stdout": proc.stdout.strip(),
            "stderr": proc.stderr.strip(),
        }

    return {"status": "completed", "tools": results}


def print_summary_table(report: dict[str, Any]) -> None:
    host = report["host"]
    print(f"Cinch Engine Wave 1 Microbenchmarks ({host['cpu']}, {host['os']}, opt=-O{host['opt_level']})")
    print("-" * 102)
    print(
        f"{'Metric':<28} {'Ops':>10} {'Elapsed (ms)':>13} {'ns/op':>9} "
        f"{'Throughput':>18} {'Allocs':>7} {'AtomRC':>7}"
    )
    print("-" * 102)
    for m in report["microbenchmarks"]["metrics"]:
        elapsed_ms = m["elapsed_ns"] / 1_000_000.0
        if "gb_per_sec" in m:
            tput = f"{m['gb_per_sec']:>7.2f} GB/s"
        else:
            tput = f"{m['ops_per_sec'] / 1_000_000.0:>7.2f} M ops/s"
        print(
            f"{m['metric']:<28} {m['ops']:>10,d} {elapsed_ms:>13.2f} {m['ns_per_op']:>9.2f} "
            f"{tput:>18} {m['allocs']:>7d} {m['atomic_rc']:>7d}"
        )
    print("-" * 102)
    mb = report["microbenchmarks"]
    print(
        f"Zero-alloc gate: {'PASS' if not mb['zero_alloc_violations'] else 'FAIL'} | "
        f"Leak check: {'PASS' if mb['leak_clean'] else 'FAIL'} (enabled={mb['leak_check']})"
    )
    perf = report.get("perf_stat", {})
    if perf.get("status") == "ok":
        print(f"Linux perf stat: IPC={perf.get('ipc')} counters={perf.get('counters')}")
    elif perf.get("reason"):
        print(f"Linux perf stat: skipped ({perf['reason']})")
    ext = report.get("external", {})
    if ext.get("status") == "completed":
        for tool, info in ext.get("tools", {}).items():
            detail = info.get("reason") or (
                f"{info.get('requests_per_sec')} req/s" if info.get("requests_per_sec") else info.get("status")
            )
            print(f"External [{tool}]: {info.get('status')} ({detail})")


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description="Run Cinch Engine Wave 1-7 benchmarks.")
    parser.add_argument("--compiler", default=None, help="Path to cinch compiler binary")
    parser.add_argument("--source", type=Path, default=DEFAULT_MICROBENCH, help="Path to microbench.cinch")
    parser.add_argument("--opt-level", type=int, default=3, choices=[0, 1, 2, 3], help="Optimization level (default: 3)")
    parser.add_argument("--out", type=Path, default=None, help="Write structured JSON report to this path")
    parser.add_argument("--no-leak-check", action="store_true", help="Disable CINCH_RC_DEBUG=1 CINCH_LEAK_CHECK=1")
    parser.add_argument("--perf", action="store_true", help="Capture Linux `perf stat` hardware counters")
    parser.add_argument("--external", action="store_true", help="Run optional external load generators (wrk, wrk2, h2load, redis-benchmark)")
    parser.add_argument("--http1-url", default=None, help="Target HTTP/1.1 URL for wrk/wrk2")
    parser.add_argument("--http2-url", default=None, help="Target HTTP/2 URL for h2load")
    parser.add_argument("--kv-host", default=None, help="Target KV host for redis-benchmark")
    parser.add_argument("--kv-port", type=int, default=6379, help="Target KV port for redis-benchmark (default: 6379)")
    parser.add_argument("--threads", type=int, default=4, help="External load-generator thread count")
    parser.add_argument("--connections", type=int, default=64, help="External load-generator connection count")
    parser.add_argument("--duration", type=int, default=10, help="External load-generator duration in seconds")
    parser.add_argument("--wrk2-rate", type=int, default=50000, help="Target RPS for wrk2")
    parser.add_argument("--json", action="store_true", help="Print JSON report to stdout instead of summary table")
    args = parser.parse_args(argv)

    try:
        compiler = resolve_compiler(args.compiler)
    except FileNotFoundError as exc:
        print(f"run_suite: {exc}", file=sys.stderr)
        return 2

    host = collect_host_info(compiler, args.opt_level)

    with tempfile.TemporaryDirectory(prefix="cinch_engine_bench_") as td:
        tdir = Path(td)
        staged_src = tdir / args.source.name
        shutil.copy2(args.source.resolve(), staged_src)
        bin_path = tdir / "microbench"
        build_res = build_microbench(compiler, staged_src, bin_path, args.opt_level)
        if not build_res["passed"]:
            report = {
                "schema_version": 1,
                "host": host,
                "build": build_res,
                "microbenchmarks": {"passed": False, "metrics": []},
            }
            if args.out:
                args.out.parent.mkdir(parents=True, exist_ok=True)
                args.out.write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
            print(f"run_suite: failed to build {args.source}:\n{build_res['stderr']}", file=sys.stderr)
            return 1

        mb_res = run_microbench(bin_path, leak_check=not args.no_leak_check)
        perf_res = run_perf_stat(bin_path, requested=args.perf)

    ext_res = run_external_benchmarks(args)

    report = {
        "schema_version": 1,
        "host": host,
        "build": {
            "passed": build_res["passed"],
            "opt_level": args.opt_level,
        },
        "microbenchmarks": mb_res,
        "perf_stat": perf_res,
        "external": ext_res,
    }

    if args.out:
        args.out.parent.mkdir(parents=True, exist_ok=True)
        args.out.write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")

    if args.json:
        print(json.dumps(report, indent=2))
    else:
        print_summary_table(report)
        if args.out:
            print(f"Report written to {args.out}")

    return 0 if mb_res["passed"] else 1


if __name__ == "__main__":
    raise SystemExit(main())
