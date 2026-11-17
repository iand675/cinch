"""Qualify one unchanged public fixture against an explicitly identified runtime pair."""

from pathlib import Path
import hashlib
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
    """Return the SHA256 of an exact input or completed output file."""
    return hashlib.sha256(path.read_bytes()).hexdigest()


def size_of(directory):
    """Include compiler project artifacts and temporary files in the output cap."""
    return sum(path.stat().st_size for path in directory.rglob("*") if path.is_file())


def main():
    """Emit one fixed object and link/run it against both fresh archives."""
    base = Path(__file__).resolve().parent
    pair = base.parent
    manifest_path = base / "source-manifest.json"
    manifest = json.loads(manifest_path.read_text())
    name = sys.argv[1]
    assert name in manifest["fixtures"]
    project = base / name
    for filename, expected in manifest["fixtures"][name]["files"].items():
        assert digest(project / filename) == expected, filename
    compiler = Path(manifest["compiler_path"])
    assert digest(compiler) == manifest["compiler_sha256"]
    assert shutil.disk_usage("/workspace").free >= 350 * 1024**2, "350 MiB start guard"
    output = project / "run-1"
    output.mkdir()
    (output / "scratch").mkdir()
    clang = Path("/workspace/.tools/llvm/usr/lib/llvm-20/bin/clang")
    environment = os.environ.copy()
    environment.update(
        LD_LIBRARY_PATH="/workspace/.tools/llvm/usr/lib/x86_64-linux-gnu",
        TMPDIR=str(output / "scratch"), CINCH_CLANG=str(clang),
        RUST_MIN_STACK="67108864", CINCH_RECLAIM="1",
    )
    for variable in ["CINCH_ALLOC_STATS", "CINCH_LEAK_CHECK", "CINCH_RC_DEBUG"]:
        environment.pop(variable, None)
    receipt = {
        "fixture": name, "scope": "actual public Cinch fixture; fixed object, fresh archive pair; no timing",
        "source_manifest_sha256": digest(manifest_path),
        "runner_sha256": digest(Path(__file__).resolve()),
        "compiler_path": str(compiler), "compiler_sha256": digest(compiler),
        "clang_path": str(clang), "clang_sha256": digest(clang),
        "start_free_bytes": shutil.disk_usage("/workspace").free,
        "environment": {key: environment[key] for key in ["LD_LIBRARY_PATH", "TMPDIR", "CINCH_CLANG", "RUST_MIN_STACK", "CINCH_RECLAIM"]},
        "commands": [], "variants": {},
    }

    def run(label, arguments, extra=None):
        """Run a single CPU-4 process group, preserving diagnostics on any failure."""
        command = ["taskset", "-c", "4", *map(str, arguments)]
        child_env = environment.copy()
        child_env.update(extra or {})
        start = time.monotonic()
        peak = 0
        minimum = shutil.disk_usage("/workspace").free
        violation = None

        def limits():
            """Bound individual files as well as the measured aggregate growth."""
            resource.setrlimit(resource.RLIMIT_FSIZE, (30 * 1024**2, 30 * 1024**2))

        with (output / (label + ".stdout")).open("wb") as stdout, (output / (label + ".stderr")).open("wb") as stderr:
            child = subprocess.Popen(command, cwd=project, env=child_env, stdout=stdout,
                                     stderr=stderr, preexec_fn=limits, start_new_session=True)
            while child.poll() is None:
                size = size_of(project)
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
        final_size = size_of(project)
        if final_size > 30 * 1024**2:
            violation = violation or {"output_bytes": final_size, "reason": "final output limit"}
        record = {
            "command": command, "extra_environment": extra or {},
            "exit_code": child.returncode, "elapsed_seconds": time.monotonic() - start,
            "peak_observed_project_bytes": max(peak, final_size),
            "min_observed_free_bytes": minimum, "guard_violation": violation,
        }
        receipt["commands"].append(record)
        (output / "receipt.json").write_text(json.dumps(receipt, indent=2) + "\n")
        assert child.returncode == 0 and violation is None, record
        return (output / (label + ".stdout")).read_text(), (output / (label + ".stderr")).read_text()

    run("emit-ir", [compiler, "emit-ir", project / "Main.cinch", "--rebuild", "-o", output / "consumer.ll"])
    run("compile-object", [clang, "-O2", "-c", "-x", "ir", output / "consumer.ll", "-o", output / "consumer.o"])
    receipt.update(ir_sha256=digest(output / "consumer.ll"), common_object_sha256=digest(output / "consumer.o"))
    for variant in ["baseline", "candidate"]:
        archive = pair / "archives-v1" / variant / "libcinchruntime.a"
        archive_identity = json.loads((archive.parent / "receipt.json").read_text())
        assert digest(archive) == archive_identity["archive_sha256"]
        executable = output / ("consumer-" + variant)
        run("link-" + variant, [clang, output / "consumer.o", archive, "-ldl", "-lpthread", "-lm", "-o", executable])
        modes = []
        for mode in ["normal", "poison"]:
            extra = {"CINCH_ALLOC_STATS": "1", "CINCH_LEAK_CHECK": "1",
                     "CINCH_HEAP_LIMIT": "256M" if mode == "poison" else "32M",
                     "CINCH_STACK_SIZE": "8M", "CINCH_RC_DEBUG": "1" if mode == "poison" else "0"}
            stdout, stderr = run(variant + "-" + mode, [executable], extra)
            assert stdout == "", stdout
            leaks = [line for line in stderr.splitlines() if line.startswith("cinch-leak:")]
            assert leaks and all(re.search(r"leaked=0(?:\s|$)", line) for line in leaks), stderr
            modes.append({"mode": mode, "zero_leak": True, "leak_lines": leaks})
        receipt["variants"][variant] = {
            "archive_sha256": digest(archive), "executable_sha256": digest(executable), "runs": modes,
        }
    receipt.update(status="PASS", output_project_bytes=size_of(project),
                   end_free_bytes=shutil.disk_usage("/workspace").free)
    (output / "receipt.json").write_text(json.dumps(receipt, indent=2) + "\n")
    print(json.dumps(receipt, indent=2), flush=True)


if __name__ == "__main__":
    main()
