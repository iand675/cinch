"""Build one sealed runtime snapshot with explicit resource and provenance guards."""

from pathlib import Path
import hashlib
import json
import os
import resource
import shutil
import signal
import subprocess
import sys
import time


def file_hash(path):
    """Return the exact SHA256 identity of a plain file."""
    return hashlib.sha256(path.read_bytes()).hexdigest()


def output_size(directory):
    """Measure every current output and temporary file in this build epoch."""
    return sum(path.stat().st_size for path in directory.rglob("*") if path.is_file())


def main():
    """Compile the requested source variant once, preserving all failure outputs."""
    variant = sys.argv[1]
    assert variant in ["baseline", "candidate"]
    experiment = Path(__file__).resolve().parent
    manifest_path = experiment / "source-pair-manifest.json"
    manifest = json.loads(manifest_path.read_text())
    source = experiment / manifest["source_directory"] / variant
    for relative, expected in manifest["identities"][variant]["files"].items():
        assert file_hash(source / relative) == expected, relative
    assert shutil.disk_usage("/workspace").free >= 350 * 1024**2, "350 MiB start guard"
    output = experiment / "archives-v1" / variant
    output.mkdir(parents=True)
    (output / "scratch").mkdir()
    compiler = Path("/workspace/.tools/rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/rustc")
    command = [
        "taskset", "-c", "4", str(compiler), "--edition=2021",
        "--crate-name", "cinchruntime", "--crate-type", "staticlib",
        "--target", "x86_64-unknown-linux-gnu", "-C", "opt-level=3",
        "-C", "debuginfo=0", "-C", "strip=debuginfo", "-C", "codegen-units=1",
        "-C", "lto=off", str(source / "compiler/runtime/src/lib.rs"),
        "-o", str(output / "libcinchruntime.a"),
    ]
    environment = os.environ.copy()
    environment["TMPDIR"] = str(output / "scratch")
    environment["CARGO_MANIFEST_DIR"] = str(source / "compiler/runtime")
    receipt = {
        "variant": variant, "source_commit": manifest["identities"][variant]["commit"],
        "source_manifest_sha256": file_hash(manifest_path),
        "compiler_path": str(compiler), "compiler_sha256": file_hash(compiler),
        "command": command, "cwd": str(source),
        "environment": {key: environment[key] for key in ["TMPDIR", "CARGO_MANIFEST_DIR"]},
        "start_free_bytes": shutil.disk_usage("/workspace").free,
    }
    assert receipt["compiler_sha256"] == "5073422953727c9427a76b7fce03aa9404da46315e5a1e4965389512a41e8db0"
    (output / "receipt.json").write_text(json.dumps(receipt, indent=2) + "\n")

    def limits():
        """Limit each new file in addition to the measured aggregate budget."""
        resource.setrlimit(resource.RLIMIT_FSIZE, (30 * 1024**2, 30 * 1024**2))

    start = time.monotonic()
    peak = 0
    minimum = receipt["start_free_bytes"]
    violation = None
    with (output / "compile.stdout").open("wb") as stdout, (output / "compile.stderr").open("wb") as stderr:
        child = subprocess.Popen(command, cwd=source, env=environment, stdout=stdout,
                                 stderr=stderr, preexec_fn=limits, start_new_session=True)
        while child.poll() is None:
            size = output_size(output)
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
    final_size = output_size(output)
    peak = max(peak, final_size)
    if final_size > 30 * 1024**2:
        violation = violation or {"output_bytes": final_size, "reason": "final output limit"}
    receipt.update(exit_code=child.returncode, elapsed_seconds=time.monotonic() - start,
                   peak_observed_output_bytes=peak, min_observed_free_bytes=minimum,
                   end_free_bytes=shutil.disk_usage("/workspace").free,
                   final_output_bytes=final_size, guard_violation=violation)
    archive = output / "libcinchruntime.a"
    if archive.is_file():
        receipt.update(archive_sha256=file_hash(archive), archive_bytes=archive.stat().st_size)
    (output / "receipt.json").write_text(json.dumps(receipt, indent=2) + "\n")
    print(json.dumps(receipt, indent=2), flush=True)
    if child.returncode != 0 or violation:
        raise SystemExit(1)


if __name__ == "__main__":
    main()
