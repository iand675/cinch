"""Build and structurally qualify a matched actual-entropy benchmark without timing claims."""

from pathlib import Path
import hashlib
import json
import os
import re
import resource
import shutil
import signal
import subprocess
import time


def digest(path):
    """Return the exact byte identity of one input or completed artifact."""
    return hashlib.sha256(path.read_bytes()).hexdigest()


def check_observation(mode, size, iterations, fields):
    """Check structure and deterministic controls without predicting real entropy."""
    assert len(fields) == 4 and all(field.isdecimal() for field in fields), fields
    elapsed, valid, checksum, retained = map(int, fields)
    assert valid == iterations and 0 <= checksum < 2**64
    if iterations == 0:
        assert checksum == 14695981039346656037 and retained == 0
    elif mode in ["random", "allocation"]:
        assert size <= retained <= size + 765
        if size == 0:
            assert retained == 0
    elif mode in ["v4", "v7"]:
        assert 144 <= retained <= 717
    else:
        assert retained == 0
    if mode in ["counter", "allocation"] or (mode == "random" and size == 0):
        expected = 14695981039346656037
        last = 0
        for index in range(iterations):
            if mode == "counter":
                observation = index
            else:
                observation = 0 if size == 0 else size + (index & 255)
                last = observation
            expected = ((expected ^ observation) * 1099511628211) & (2**64 - 1)
        assert checksum == expected and retained == last, (mode, size, iterations, fields, expected, last)
    return {"instrument_span_ns": elapsed, "valid": valid, "checksum": checksum,
            "retained": retained, "timing_eligible": False}


def main():
    """Use one frozen source/object and two sealed archives for all preflight runs."""
    bench = Path(__file__).resolve().parent
    out = bench / "preflight-v2"
    project = out / "project"
    assert json.loads((out / "source-check-receipt.json").read_text())["exit_code"] == 0
    assert shutil.disk_usage("/workspace").free >= 350 * 1024**2, "350 MiB start guard"
    manifest = json.loads((out / "source-manifest.json").read_text())
    for name, expected in manifest["source_files"].items():
        assert digest(project / name) == expected, name
    protocol = json.loads((bench / "protocol-v2.json").read_text())
    compiler = Path("/workspace/dasl-stdlib-io/build/stdlib-projector-release-frozen/cinch")
    assert digest(compiler) == protocol["compiler_sha256"]
    clang = Path("/workspace/.tools/llvm/usr/lib/llvm-20/bin/clang")
    env = os.environ.copy()
    env.update(LD_LIBRARY_PATH="/workspace/.tools/llvm/usr/lib/x86_64-linux-gnu",
               CINCH_CLANG=str(clang), TMPDIR=str(out / "scratch"), RUST_MIN_STACK="67108864")
    for variable in ["CINCH_RC_DEBUG", "CINCH_LEAK_CHECK", "CINCH_ALLOC_STATS"]:
        env.pop(variable, None)
    receipt = {"scope": "structural preflight only; no performance observations eligible",
               "compiler_sha256": digest(compiler), "clang_sha256": digest(clang),
               "runner_sha256": digest(Path(__file__)), "protocol_sha256": digest(bench / "protocol-v2.json"),
               "start_free_bytes": shutil.disk_usage("/workspace").free,
               "environment": {key: env[key] for key in ["LD_LIBRARY_PATH", "CINCH_CLANG", "TMPDIR", "RUST_MIN_STACK"]},
               "commands": [], "observations": [], "variants": {}}

    def size_of():
        """Include all project outputs, interfaces and compiler/linker scratch."""
        return sum(path.stat().st_size for path in out.rglob("*") if path.is_file())

    def run(label, arguments, extra=None):
        """Run one CPU-4 process group with reserve, aggregate and wall guards."""
        command = ["taskset", "-c", "4", *map(str, arguments)]
        child_env = env.copy()
        child_env.update(extra or {})
        start = time.monotonic()
        peak = 0
        minimum = shutil.disk_usage("/workspace").free
        violation = None

        def limits():
            """Bound each produced file in addition to aggregate monitoring."""
            resource.setrlimit(resource.RLIMIT_FSIZE, (30 * 1024**2, 30 * 1024**2))

        with (out / (label + ".stdout")).open("wb") as stdout, (out / (label + ".stderr")).open("wb") as stderr:
            child = subprocess.Popen(command, cwd=project, env=child_env, stdout=stdout,
                                     stderr=stderr, preexec_fn=limits, start_new_session=True)
            while child.poll() is None:
                size = size_of()
                free = shutil.disk_usage("/workspace").free
                peak = max(peak, size)
                minimum = min(minimum, free)
                if size > 30 * 1024**2 or free < 250 * 1024**2 or time.monotonic() - start > 60:
                    violation = {"output_bytes": size, "free_bytes": free,
                                 "elapsed_seconds": time.monotonic() - start}
                    os.killpg(child.pid, signal.SIGTERM)
                    break
                time.sleep(0.02)
            try:
                child.wait(timeout=5)
            except subprocess.TimeoutExpired:
                os.killpg(child.pid, signal.SIGKILL)
                child.wait()
        final_size = size_of()
        if final_size > 30 * 1024**2:
            violation = violation or {"output_bytes": final_size, "reason": "final output cap"}
        record = {"command": command, "extra_environment": extra or {}, "exit_code": child.returncode,
                  "elapsed_seconds": time.monotonic() - start, "guard_violation": violation,
                  "peak_observed_output_bytes": max(peak, final_size), "min_free_bytes": minimum}
        receipt["commands"].append(record)
        (out / "receipt.json").write_text(json.dumps(receipt, indent=2) + "\n")
        assert child.returncode == 0 and violation is None, record
        return (out / (label + ".stdout")).read_text(), (out / (label + ".stderr")).read_text()

    run("emit-ir", [compiler, "emit-ir", project / "Main.cinch", "--rebuild", "-o", out / "benchmark.ll"])
    run("compile-object", [clang, "-O2", "-c", "-x", "ir", out / "benchmark.ll", "-o", out / "benchmark.o"])
    run("assembly", [clang, "-O2", "-S", "-x", "ir", out / "benchmark.ll", "-o", out / "benchmark.s"])
    receipt.update(llvm_sha256=digest(out / "benchmark.ll"), common_object_sha256=digest(out / "benchmark.o"),
                   assembly_sha256=digest(out / "benchmark.s"))
    for variant in ["baseline", "candidate"]:
        archive = bench.parent / "staticlib-pair-v1/archives-v1" / variant / "libcinchruntime.a"
        assert digest(archive) == protocol["archive_pair"][variant]
        executable = out / ("benchmark-" + variant)
        run("link-" + variant, [clang, out / "benchmark.o", archive, "-ldl", "-lpthread", "-lm", "-o", executable])
        receipt["variants"][variant] = {"archive_sha256": digest(archive), "executable_sha256": digest(executable)}
        for condition in protocol["conditions"]:
            mode, size = condition["mode"], condition["size"]
            for count in protocol["preflight_iterations"]:
                for poisoned in [False, True]:
                    label = f"{variant}-{mode}-{size}-{count}-{'poison' if poisoned else 'normal'}"
                    extra = {"CINCH_ALLOC_STATS": "1", "CINCH_LEAK_CHECK": "1",
                             "CINCH_RC_DEBUG": "1" if poisoned else "0",
                             "CINCH_HEAP_LIMIT": "256M" if poisoned else "32M", "CINCH_STACK_SIZE": "8M"}
                    stdout, stderr = run(label, [executable, mode, size, count], extra)
                    observed = check_observation(mode, size, count, stdout.split())
                    leaks = [line for line in stderr.splitlines() if line.startswith("cinch-leak:")]
                    assert leaks and all(re.search(r"leaked=0(?:\s|$)", line) for line in leaks), stderr
                    observed.update(variant=variant, mode=mode, size=size, iterations=count,
                                    poisoned=poisoned, leak_lines=leaks)
                    receipt["observations"].append(observed)
        print(variant, "104 structural/leak cases PASS", flush=True)
    receipt.update(status="PASS", eligible_timing_samples=0, output_bytes=size_of(),
                   end_free_bytes=shutil.disk_usage("/workspace").free)
    (out / "receipt.json").write_text(json.dumps(receipt, indent=2) + "\n")
    print("preflight PASS", len(receipt["observations"]), "zero-leak cases", receipt["output_bytes"], "output bytes", flush=True)


if __name__ == "__main__":
    main()
