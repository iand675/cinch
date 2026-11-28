#!/usr/bin/env python3
"""Compare a Cinch Engine benchmark report against `baselines.json`.

Computes percentage throughput deltas for every Wave 1 microbenchmark and exits
non-zero (exit code 1) if:
  1. ANY zero-allocation invariant regresses (`allocs > 0` or `atomic_rc > 0`), or
  2. ANY benchmark's throughput (`ops_per_sec` / `gb_per_sec`) regresses by more
     than the configured threshold (per-metric `max_regression_pct` or `--threshold`).
"""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path
from typing import Any

HERE = Path(__file__).resolve().parent
DEFAULT_BASELINE = HERE / "baselines.json"


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def compare_reports(
    current: dict[str, Any],
    baseline: dict[str, Any],
    default_threshold_pct: float,
) -> dict[str, Any]:
    base_metrics: dict[str, dict[str, Any]] = baseline.get("wave1_microbenchmarks", {})
    cur_list: list[dict[str, Any]] = current.get("microbenchmarks", {}).get("metrics", [])
    cur_by_name = {m["metric"]: m for m in cur_list}

    comparisons: list[dict[str, Any]] = []
    invariant_failures: list[dict[str, Any]] = []
    throughput_regressions: list[dict[str, Any]] = []
    missing_metrics: list[str] = []

    all_names = list(base_metrics.keys()) + [n for n in cur_by_name if n not in base_metrics]
    for name in all_names:
        cur = cur_by_name.get(name)
        base = base_metrics.get(name)
        if cur is None:
            missing_metrics.append(name)
            comparisons.append({"metric": name, "status": "MISSING"})
            continue

        allocs = int(cur.get("allocs", 0))
        atomic_rc = int(cur.get("atomic_rc", 0))
        inv_ok = (allocs == 0) and (atomic_rc == 0)
        if not inv_ok:
            invariant_failures.append(
                {
                    "metric": name,
                    "allocs": allocs,
                    "atomic_rc": atomic_rc,
                    "reason": "zero-allocation / zero-atomic-RC invariant violated",
                }
            )

        is_byte_metric = "gb_per_sec" in cur or (base is not None and "gb_per_sec" in base)
        unit = "GB/s" if is_byte_metric else "ops/s"
        cur_val = float(cur["gb_per_sec"] if is_byte_metric else cur["ops_per_sec"])

        if base is None:
            comparisons.append(
                {
                    "metric": name,
                    "unit": unit,
                    "current": cur_val,
                    "baseline": None,
                    "delta_pct": None,
                    "allocs": allocs,
                    "atomic_rc": atomic_rc,
                    "status": "NEW" if inv_ok else "FAIL_INVARIANT",
                }
            )
            continue

        base_val = float(base["gb_per_sec"] if is_byte_metric else base["ops_per_sec"])
        threshold = float(base.get("max_regression_pct", default_threshold_pct))
        delta_pct = ((cur_val - base_val) / base_val * 100.0) if base_val > 0 else 0.0
        regressed = delta_pct < -threshold

        if regressed:
            throughput_regressions.append(
                {
                    "metric": name,
                    "unit": unit,
                    "baseline": base_val,
                    "current": cur_val,
                    "delta_pct": round(delta_pct, 2),
                    "threshold_pct": threshold,
                }
            )

        status = "PASS"
        if not inv_ok:
            status = "FAIL_INVARIANT"
        elif regressed:
            status = "FAIL_REGRESSION"

        comparisons.append(
            {
                "metric": name,
                "unit": unit,
                "baseline": base_val,
                "current": cur_val,
                "delta_pct": round(delta_pct, 2),
                "threshold_pct": threshold,
                "allocs": allocs,
                "atomic_rc": atomic_rc,
                "status": status,
            }
        )

    passed = (
        len(invariant_failures) == 0
        and len(throughput_regressions) == 0
        and len(missing_metrics) == 0
    )
    return {
        "passed": passed,
        "threshold_pct": default_threshold_pct,
        "comparisons": comparisons,
        "invariant_failures": invariant_failures,
        "throughput_regressions": throughput_regressions,
        "missing_metrics": missing_metrics,
    }


def update_baseline_from_report(baseline: dict[str, Any], current: dict[str, Any], threshold_pct: float) -> dict[str, Any]:
    updated = dict(baseline)
    updated["host"] = current.get("host", updated.get("host", {}))
    wave1: dict[str, Any] = {}
    for m in current.get("microbenchmarks", {}).get("metrics", []):
        name = m["metric"]
        prev = baseline.get("wave1_microbenchmarks", {}).get(name, {})
        entry: dict[str, Any] = {
            "ops": m["ops"],
            "elapsed_ns": m["elapsed_ns"],
            "ns_per_op": m["ns_per_op"],
            "ops_per_sec": m["ops_per_sec"],
            "allocs": m["allocs"],
            "atomic_rc": m["atomic_rc"],
            "max_regression_pct": prev.get("max_regression_pct", threshold_pct),
        }
        if "bytes" in m:
            entry["bytes"] = m["bytes"]
        if "gb_per_sec" in m:
            entry["gb_per_sec"] = m["gb_per_sec"]
        wave1[name] = entry
    updated["wave1_microbenchmarks"] = wave1
    return updated


def format_val(val: float | None, unit: str) -> str:
    if val is None:
        return "n/a"
    if unit == "GB/s":
        return f"{val:.2f} GB/s"
    return f"{val / 1_000_000.0:.2f} M/s"


def print_table(res: dict[str, Any]) -> None:
    print("Cinch Engine Baseline Comparison")
    print("-" * 100)
    print(
        f"{'Metric':<28} {'Baseline':>14} {'Current':>14} {'Delta':>10} "
        f"{'Allocs':>7} {'AtomRC':>7} {'Status':>14}"
    )
    print("-" * 100)
    for row in res["comparisons"]:
        if row["status"] == "MISSING":
            print(f"{row['metric']:<28} {'-':>14} {'MISSING':>14} {'-':>10} {'-':>7} {'-':>7} {'FAIL_MISSING':>14}")
            continue
        unit = row["unit"]
        delta_s = f"{row['delta_pct']:+.2f}%" if row["delta_pct"] is not None else "n/a"
        print(
            f"{row['metric']:<28} {format_val(row['baseline'], unit):>14} "
            f"{format_val(row['current'], unit):>14} {delta_s:>10} "
            f"{row['allocs']:>7d} {row['atomic_rc']:>7d} {row['status']:>14}"
        )
    print("-" * 100)
    print(
        f"Result: {'PASS' if res['passed'] else 'FAIL'} | "
        f"invariant_failures={len(res['invariant_failures'])} | "
        f"throughput_regressions={len(res['throughput_regressions'])} | "
        f"missing={len(res['missing_metrics'])}"
    )


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description="Compare engine microbench report against baselines.json.")
    parser.add_argument("report", type=Path, help="Path to JSON report emitted by run_suite.py")
    parser.add_argument("--baseline", type=Path, default=DEFAULT_BASELINE, help="Path to baselines.json")
    parser.add_argument("--threshold", type=float, default=20.0, help="Default max allowed regression %% (default: 20.0)")
    parser.add_argument("--update-baseline", action="store_true", help="Write measured Wave 1 numbers from report into baselines.json (only if zero-alloc passes)")
    parser.add_argument("--json", action="store_true", help="Emit JSON output")
    args = parser.parse_args(argv)

    current = load_json(args.report)
    baseline = load_json(args.baseline)

    if args.update_baseline:
        mb = current.get("microbenchmarks", {})
        if not mb.get("passed", False):
            print("compare_baseline: refusing --update-baseline because microbench report failed", file=sys.stderr)
            return 1
        updated = update_baseline_from_report(baseline, current, args.threshold)
        args.baseline.write_text(json.dumps(updated, indent=2) + "\n", encoding="utf-8")
        print(f"Updated Wave 1 baselines in {args.baseline}")
        baseline = updated

    result = compare_reports(current, baseline, args.threshold)
    if args.json:
        print(json.dumps(result, indent=2))
    else:
        print_table(result)
    return 0 if result["passed"] else 1


if __name__ == "__main__":
    raise SystemExit(main())
