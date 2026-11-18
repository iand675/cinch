"""Execute one explicitly authorized existing-binary measurement window."""

from pathlib import Path
import argparse
import hashlib
import importlib.util
import json
import os
import resource
import shutil
import signal
import subprocess
import sys
import time


def digest(path):
    """Hash a plain input without modifying its bytes, mode or directory."""
    value = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            value.update(chunk)
    return value.hexdigest()


def build_schedule(conditions, warmups, samples):
    """Rotate condition starts and all six variant permutations in complete triples."""
    orders = [
        ["baseline", "candidate", "sdk"], ["candidate", "sdk", "baseline"],
        ["sdk", "baseline", "candidate"], ["sdk", "candidate", "baseline"],
        ["candidate", "baseline", "sdk"], ["baseline", "sdk", "candidate"],
    ]
    schedule = []
    for round_index in range(warmups + samples):
        phase = "warmup" if round_index < warmups else "timed"
        sample_index = round_index if phase == "warmup" else round_index - warmups
        offset = round_index % len(conditions)
        selected = conditions[offset:] + conditions[:offset]
        for position, condition in enumerate(selected):
            order = orders[(round_index * len(conditions) + position) % len(orders)]
            triple = f"{phase}-{sample_index}-{condition['mode']}-{condition['size']}"
            for variant_position, variant in enumerate(order):
                schedule.append({"phase": phase, "sample_index": sample_index,
                                 "round_index": round_index, "triple_id": triple,
                                 "mode": condition["mode"], "size": condition["size"],
                                 "variant": variant, "variant_order": order,
                                 "variant_position": variant_position})
    return schedule


def telemetry():
    """Capture actual affinity, cgroup quota/throttling and load at a boundary."""
    result = {"utc": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()),
              "monotonic_ns": time.monotonic_ns(), "affinity": sorted(os.sched_getaffinity(0)),
              "loadavg": Path("/proc/loadavg").read_text().strip(),
              "free_bytes": shutil.disk_usage("/workspace").free}
    for name in ["cpu.stat", "cpu.max", "cpuset.cpus.effective"]:
        path = Path("/sys/fs/cgroup") / name
        result[name] = path.read_text().strip() if path.is_file() else None
    result["cpu_stat_values"] = dict(line.split() for line in (result["cpu.stat"] or "").splitlines())
    runnable = []
    membership = Path("/sys/fs/cgroup/cgroup.procs")
    for pid in membership.read_text().split() if membership.is_file() else []:
        try:
            stat = (Path("/proc") / pid / "stat").read_text()
            name = stat[stat.index("(") + 1:stat.rindex(")")]
            fields = stat[stat.rindex(")") + 2:].split()
            if fields[0] in ["R", "D"]:
                runnable.append({"pid": int(pid), "ppid": int(fields[1]),
                                 "state": fields[0], "processor": int(fields[36]), "comm": name})
        except (FileNotFoundError, ProcessLookupError):
            continue
    result["runnable_cgroup_processes"] = runnable
    return result


def verify_inputs(manifest):
    """Verify every sealed executable, source, object and runtime provenance input."""
    results = {}
    for label, identity in manifest["inputs"].items():
        path = Path(identity["path"])
        assert path.is_file() and not path.is_symlink(), label
        value = digest(path)
        assert value == identity["sha256"], (label, value, identity["sha256"])
        results[label] = value
    return results


def main():
    """Run only after a parent-issued lease, retaining every raw observation."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--lease-id", required=True)
    parser.add_argument("--plan-sha256", required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    sys.dont_write_bytecode = True
    bench = Path(__file__).resolve().parent
    plan_path = bench / "execution-plan-v2.json"
    assert digest(plan_path) == args.plan_sha256
    plan = json.loads(plan_path.read_text())
    assert digest(Path(__file__)) == plan["launcher_sha256"]
    manifest_path = bench / "timing-inputs-v2.json"
    assert digest(manifest_path) == plan["input_manifest_sha256"]
    manifest = json.loads(manifest_path.read_text())
    assert shutil.disk_usage("/workspace").free >= plan["start_guard_bytes"]
    assert args.output.resolve().parent == bench.resolve(), "output must be a new local benchmark epoch"
    args.output.mkdir()
    os.sched_setaffinity(0, {0})
    started = time.monotonic()
    sample_deadline = started + plan["sample_phase_deadline_seconds"]
    out = args.output
    summary = {"lease_id": args.lease_id, "plan_sha256": args.plan_sha256,
               "launcher_sha256": plan["launcher_sha256"], "status": "started",
               "observations": 0, "warmup_observations": 0, "timed_observations": 0,
               "controls_omitted": [], "regressions_filtered": False}
    before = verify_inputs(manifest)
    (out / "inputs-before.json").write_text(json.dumps(before, indent=2) + "\n")
    spec = importlib.util.spec_from_file_location("entropy_observation", bench / "preflight.py")
    observer = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(observer)
    schedule = plan["schedule"]
    (out / "planned-order.json").write_text(json.dumps(schedule, indent=2) + "\n")
    pre = telemetry()
    (out / "telemetry-before.json").write_text(json.dumps(pre, indent=2) + "\n")
    assert pre["affinity"] == [0] and pre["cpu.stat"] is not None and pre["cpu.max"] is not None
    raw_path = out / "observations.jsonl"
    active = None
    error = None

    def report_size():
        """Measure only new report artifacts; no executable copies are produced."""
        return sum(path.stat().st_size for path in out.rglob("*") if path.is_file())

    def file_limits():
        """Bound file output in addition to the aggregate report budget."""
        resource.setrlimit(resource.RLIMIT_FSIZE, (plan["report_cap_bytes"], plan["report_cap_bytes"]))

    try:
        with raw_path.open("a", buffering=1) as raw:
            for entry in schedule:
                assert time.monotonic() < sample_deadline, "sample phase deadline reached"
                assert shutil.disk_usage("/workspace").free >= plan["reserve_bytes"], "disk reserve guard"
                assert report_size() < plan["report_cap_bytes"], "report cap"
                environment = os.environ.copy()
                environment.update(plan["runtime_environment"])
                for name in plan["unset_environment"]:
                    environment.pop(name, None)
                executable = manifest["executables"][entry["variant"]]
                command = [executable, entry["mode"], str(entry["size"]), str(plan["iterations"])]
                record = {**entry, "command": command,
                          "launch_offset_seconds": time.monotonic() - started}
                active = subprocess.Popen(command, env=environment, stdout=subprocess.PIPE,
                                          stderr=subprocess.PIPE, preexec_fn=file_limits,
                                          start_new_session=True)
                timed_out = False
                try:
                    stdout, stderr = active.communicate(timeout=min(plan["child_timeout_seconds"],
                                                                  max(0.001, sample_deadline - time.monotonic())))
                except subprocess.TimeoutExpired:
                    timed_out = True
                    os.killpg(active.pid, signal.SIGTERM)
                    try:
                        stdout, stderr = active.communicate(timeout=0.2)
                    except subprocess.TimeoutExpired:
                        os.killpg(active.pid, signal.SIGKILL)
                        stdout, stderr = active.communicate()
                record.update(exit_code=active.returncode, stdout=stdout.decode(errors="replace"),
                              stderr=stderr.decode(errors="replace"), timed_out=timed_out)
                active = None
                passed = record["exit_code"] == 0 and not timed_out and record["stderr"] == ""
                if passed:
                    try:
                        observation = observer.check_observation(entry["mode"], entry["size"],
                                                                 plan["iterations"], record["stdout"].split())
                        record.update(span_ns=observation["instrument_span_ns"], valid=observation["valid"],
                                      checksum_decimal=str(observation["checksum"]),
                                      retained_decimal=str(observation["retained"]))
                    except Exception as exception:
                        passed = False
                        record["structural_error"] = repr(exception)
                record["structural_pass"] = passed
                raw.write(json.dumps(record, separators=(",", ":")) + "\n")
                summary["observations"] += 1
                summary[entry["phase"] + "_observations"] += 1
                assert passed, "existing benchmark process failed; raw output retained"
        summary["status"] = "complete"
    except BaseException as exception:
        error = repr(exception)
        summary.update(status="incomplete", error=error)
    finally:
        if active is not None and active.poll() is None:
            os.killpg(active.pid, signal.SIGKILL)
            active.wait()
        post = telemetry()
        (out / "telemetry-after.json").write_text(json.dumps(post, indent=2) + "\n")
        try:
            after = verify_inputs(manifest)
            summary["input_hashes_unchanged"] = after == before
            (out / "inputs-after.json").write_text(json.dumps(after, indent=2) + "\n")
        except Exception as exception:
            summary.update(input_hashes_unchanged=False, input_verification_error=repr(exception))
        summary.update(wall_seconds=time.monotonic() - started,
                       report_bytes=report_size(), hard_wall_limit_seconds=plan["wall_limit_seconds"],
                       throttled_period_delta=int(post["cpu_stat_values"].get("nr_throttled", 0)) - int(pre["cpu_stat_values"].get("nr_throttled", 0)),
                       throttled_usec_delta=int(post["cpu_stat_values"].get("throttled_usec", 0)) - int(pre["cpu_stat_values"].get("throttled_usec", 0)))
        summary["complete_requested_counts"] = (summary["warmup_observations"] == plan["expected_warmup_observations"]
                                                   and summary["timed_observations"] == plan["expected_timed_observations"])
        summary["within_report_cap"] = summary["report_bytes"] < plan["report_cap_bytes"]
        summary["within_wall_limit"] = summary["wall_seconds"] <= plan["wall_limit_seconds"]
        summary["all_raw_observations_retained"] = True
        (out / "summary.json").write_text(json.dumps(summary, indent=2) + "\n")
    print(json.dumps(summary, indent=2), flush=True)
    assert error is None and summary["complete_requested_counts"] and summary["input_hashes_unchanged"]
    assert summary["within_report_cap"] and summary["within_wall_limit"]


if __name__ == "__main__":
    main()
