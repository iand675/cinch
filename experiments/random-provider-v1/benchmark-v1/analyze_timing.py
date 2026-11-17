"""Analyze every retained condition from the sealed full entropy measurement window."""

from pathlib import Path
import collections
import hashlib
import json
import statistics


def quantile(values, fraction):
    """Interpolate one descriptive quantile without dropping any observation."""
    ordered = sorted(values)
    position = (len(ordered) - 1) * fraction
    low = int(position)
    high = min(low + 1, len(ordered) - 1)
    return ordered[low] + (ordered[high] - ordered[low]) * (position - low)


def describe(values):
    """Keep all values and report median, full range and dispersion."""
    return {"count": len(values), "median": statistics.median(values),
            "mean": statistics.mean(values), "minimum": min(values), "maximum": max(values),
            "p10": quantile(values, 0.1), "p25": quantile(values, 0.25),
            "p75": quantile(values, 0.75), "p90": quantile(values, 0.9),
            "sample_standard_deviation": statistics.stdev(values), "all_values": values}


def main():
    """Validate full counts/order and compare all thirteen conditions honestly."""
    bench = Path(__file__).resolve().parent
    out = bench / "timing-full-v1"
    raw_path = out / "observations.jsonl"
    summary = json.loads((out / "summary.json").read_text())
    plan = json.loads((bench / "execution-plan-v2.json").read_text())
    rows = [json.loads(line) for line in raw_path.read_text().splitlines()]
    assert len(rows) == 1365 and summary["mechanical_qualification_pass"]
    for row, planned in zip(rows, plan["schedule"]):
        assert all(row[key] == value for key, value in planned.items()), (row, planned)
    assert summary["input_hashes_unchanged"] and not summary["throttling_observed"]
    assert all(row["structural_pass"] and row["exit_code"] == 0 and row["stderr"] == "" for row in rows)
    assert sum(row["phase"] == "warmup" for row in rows) == 195
    timed = [row for row in rows if row["phase"] == "timed"]
    assert len(timed) == 1170
    groups = collections.defaultdict(list)
    triples = collections.defaultdict(dict)
    for row in timed:
        assert row["valid"] == plan["iterations"]
        groups[(row["mode"], row["size"], row["variant"])].append(row)
        assert row["variant"] not in triples[row["triple_id"]]
        triples[row["triple_id"]][row["variant"]] = row
    assert len(triples) == 390 and all(set(triple) == {"baseline", "candidate", "sdk"} for triple in triples.values())
    results = {
        "scope": "all thirteen conditions from one complete balanced window; no filtering or normalization",
        "raw_sha256": hashlib.sha256(raw_path.read_bytes()).hexdigest(),
        "plan_sha256": hashlib.sha256((bench / "execution-plan-v2.json").read_bytes()).hexdigest(),
        "observations": 1365, "warmups_retained": 195, "timed_observations": 1170,
        "samples_per_condition_variant": 30, "iterations_per_observation": plan["iterations"],
        "input_hashes_unchanged": True, "added_throttled_periods": 0, "added_throttled_usec": 0,
        "wall_seconds": summary["wall_seconds"], "conditions": [],
        "limits": [
            "One controlled host window is evidence for this compiler/source/runtime pair, not cross-platform parity.",
            "The SDK uses an observed secure kernel vDSO path; the candidate uses a raw syscall for nonempty entropy draws.",
            "The zero-byte Cinch provider retains device-open failure semantics absent from the SDK empty read.",
            "Allocation controls expose distinct RC/GC ownership and release costs; they are not used to normalize measurements.",
            "All raw warmups, timed samples, variable checksums, controls, regressions and full ranges are retained.",
            "Earlier D91/SDK timings used their own identities and are not relabeled as this matched pair.",
        ],
    }
    table = ["| Condition | Baseline ns/op | Candidate ns/op | SDK ns/op | Candidate / baseline |",
             "| --- | ---: | ---: | ---: | ---: |"]
    for condition in plan["conditions"]:
        mode, size = condition["mode"], condition["size"]
        record = {"mode": mode, "size": size, "variants": {}}
        for variant in ["baseline", "candidate", "sdk"]:
            selected = groups[(mode, size, variant)]
            assert len(selected) == 30
            assert collections.Counter(row["variant_position"] for row in selected) == {0: 10, 1: 10, 2: 10}
            orders = collections.Counter(tuple(row["variant_order"]) for row in selected)
            assert len(orders) == 6 and set(orders.values()) == {5}
            values = [row["span_ns"] / plan["iterations"] for row in selected]
            record["variants"][variant] = describe(values)
        selected_triples = [triple for triple in triples.values()
                            if triple["baseline"]["mode"] == mode and triple["baseline"]["size"] == size]
        assert len(selected_triples) == 30
        paired = [triple["candidate"]["span_ns"] / triple["baseline"]["span_ns"] for triple in selected_triples]
        paired_sdk = [triple["candidate"]["span_ns"] / triple["sdk"]["span_ns"] for triple in selected_triples]
        record["paired_candidate_baseline_ratios"] = describe(paired)
        record["paired_candidate_SDK_ratios"] = describe(paired_sdk)
        baseline = record["variants"]["baseline"]["median"]
        candidate = record["variants"]["candidate"]["median"]
        sdk = record["variants"]["sdk"]["median"]
        record.update(candidate_baseline_median_ratio=candidate / baseline,
                      baseline_candidate_median_speedup=baseline / candidate,
                      candidate_SDK_median_ratio=candidate / sdk,
                      median_reduction_fraction=1 - candidate / baseline)
        results["conditions"].append(record)
        label = f"{mode} {size}" if mode in ["random", "allocation"] else mode
        table.append(f"| {label} | {baseline:.2f} | {candidate:.2f} | {sdk:.2f} | {candidate / baseline:.4f} |")
    (out / "analysis.json").write_text(json.dumps(results, indent=2) + "\n")
    (out / "all-conditions-table.md").write_text("\n".join(table) + "\n")
    print("\n".join(table), flush=True)


if __name__ == "__main__":
    main()
