"""Run target-count structural checks using existing binaries and no build tools."""

from pathlib import Path
import hashlib
import importlib.util
import json
import os
import re
import resource
import shutil
import signal
import subprocess
import sys
import time


def digest(path):
    """Verify an exact existing executable without changing its bytes or mode."""
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    """Apply the explicit 300/250 MiB existing-binary exception to 65 cases."""
    sys.dont_write_bytecode = True
    bench = Path(__file__).resolve().parent
    assert shutil.disk_usage("/workspace").free >= 300 * 1024**2, "300 MiB existing-binary start guard"
    out = bench / "target-count-existing-v3"
    out.mkdir()
    identity = json.loads((bench / "combined-preflight-identity.json").read_text())
    protocol = json.loads((bench / "protocol-v2.json").read_text())
    binaries = {"baseline": bench / "preflight-v2/benchmark-baseline",
                "candidate": bench / "preflight-v2/benchmark-candidate",
                "sdk": bench / "sdk-preflight-v2/reference"}
    expected = {variant: identity["Cinch_variants"][variant]["executable_sha256"]
                for variant in ["baseline", "candidate"]}
    expected["sdk"] = identity["SDK_executable_sha256"]
    before = {variant: digest(path) for variant, path in binaries.items()}
    assert before == expected, (before, expected)
    spec = importlib.util.spec_from_file_location("entropy_observer", bench / "preflight.py")
    observer = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(observer)
    receipt = {
        "scope": "existing-binary-only target4096 structural preflight; no timing eligibility",
        "resource_exception": {"start_bytes": 300 * 1024**2, "reserve_bytes": 250 * 1024**2,
                               "report_cap_bytes": 5 * 1024**2,
                               "normal_new_build_start_bytes_unchanged": 350 * 1024**2},
        "runner_sha256": digest(Path(__file__)), "executable_sha256_before": before,
        "source_identity_sha256": digest(bench / "combined-preflight-identity.json"),
        "start_free_bytes": shutil.disk_usage("/workspace").free,
        "commands": [], "observations": [], "eligible_timing_samples": 0,
        "no_compiler_SDK_build_or_dependency_cache_writes": True,
    }

    def report_size():
        """Account for every report file produced by this exception unit."""
        return sum(path.stat().st_size for path in out.rglob("*") if path.is_file())

    def run_case(variant, executable, mode, size, poisoned):
        """Execute one pre-existing program under CPU, report and reserve guards."""
        env = os.environ.copy()
        env.update(GOMAXPROCS="1", PYTHONDONTWRITEBYTECODE="1")
        for name in ["CINCH_LEAK_CHECK", "CINCH_RC_DEBUG", "CINCH_ALLOC_STATS"]:
            env.pop(name, None)
        if variant != "sdk":
            env.update(CINCH_LEAK_CHECK="1", CINCH_RC_DEBUG="1" if poisoned else "0",
                       CINCH_ALLOC_STATS="1", CINCH_HEAP_LIMIT="256M" if poisoned else "32M",
                       CINCH_STACK_SIZE="8M")
        label = f"{variant}-{mode}-{size}-{'poison' if poisoned else 'normal'}"
        command = ["taskset", "-c", "4", str(executable), mode, str(size), "4096"]
        start = time.monotonic()
        peak = 0
        minimum = shutil.disk_usage("/workspace").free
        violation = None

        def limits():
            """Limit individual report files alongside aggregate output checks."""
            resource.setrlimit(resource.RLIMIT_FSIZE, (5 * 1024**2, 5 * 1024**2))

        with (out / (label + ".stdout")).open("wb") as stdout, (out / (label + ".stderr")).open("wb") as stderr:
            child = subprocess.Popen(command, env=env, stdout=stdout, stderr=stderr,
                                     preexec_fn=limits, start_new_session=True)
            while child.poll() is None:
                size_now = report_size()
                free = shutil.disk_usage("/workspace").free
                peak = max(peak, size_now)
                minimum = min(minimum, free)
                if size_now > 5 * 1024**2 or free < 250 * 1024**2 or time.monotonic() - start > 60:
                    violation = {"report_bytes": size_now, "free_bytes": free,
                                 "elapsed_seconds": time.monotonic() - start}
                    os.killpg(child.pid, signal.SIGTERM)
                    break
                time.sleep(0.02)
            try:
                child.wait(timeout=5)
            except subprocess.TimeoutExpired:
                os.killpg(child.pid, signal.SIGKILL)
                child.wait()
        final_size = report_size()
        if final_size > 5 * 1024**2:
            violation = violation or {"report_bytes": final_size, "reason": "final report cap"}
        record = {"command": command, "environment": {key: env.get(key) for key in
                  ["GOMAXPROCS", "CINCH_LEAK_CHECK", "CINCH_RC_DEBUG", "CINCH_ALLOC_STATS",
                   "CINCH_HEAP_LIMIT", "CINCH_STACK_SIZE", "PYTHONDONTWRITEBYTECODE"]},
                  "exit_code": child.returncode, "elapsed_seconds": time.monotonic() - start,
                  "guard_violation": violation, "peak_observed_report_bytes": max(peak, final_size),
                  "min_free_bytes": minimum}
        receipt["commands"].append(record)
        (out / "receipt.json").write_text(json.dumps(receipt, indent=2) + "\n")
        assert child.returncode == 0 and violation is None, record
        observed = observer.check_observation(mode, size, 4096, (out / (label + ".stdout")).read_text().split())
        stderr = (out / (label + ".stderr")).read_text()
        leaks = [line for line in stderr.splitlines() if line.startswith("cinch-leak:")]
        if variant != "sdk":
            assert leaks and all(re.search(r"leaked=0(?:\s|$)", line) for line in leaks), stderr
        else:
            assert stderr == "", stderr
        observed.update(variant=variant, mode=mode, size=size, iterations=4096,
                        poisoned=poisoned, leak_lines=leaks)
        receipt["observations"].append(observed)

    try:
        for variant, executable in binaries.items():
            for condition in protocol["conditions"]:
                for poisoned in ([False] if variant == "sdk" else [False, True]):
                    run_case(variant, executable, condition["mode"], condition["size"], poisoned)
            print(variant, "target4096 structural PASS", flush=True)
        receipt.update(status="PASS", structural_cases=len(receipt["observations"]))
    finally:
        after = {variant: digest(path) for variant, path in binaries.items()}
        receipt.update(executable_sha256_after=after, executables_unchanged=after == before,
                       end_free_bytes=shutil.disk_usage("/workspace").free,
                       output_bytes=report_size())
        (out / "receipt.json").write_text(json.dumps(receipt, indent=2) + "\n")
        assert after == before
    assert receipt["structural_cases"] == 65
    print("target4096 PASS", receipt["structural_cases"], "cases; eligible timing samples=0; bytes", receipt["output_bytes"], flush=True)


if __name__ == "__main__":
    main()
