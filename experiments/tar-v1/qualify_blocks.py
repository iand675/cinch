#!/usr/bin/env python3
"""Run one bounded fixed-block runtime unit and archive only verified closed functional outputs."""
from __future__ import annotations
import argparse
from decimal import Decimal
import gzip
import hashlib
import json
import os
from pathlib import Path
import shutil
import signal
import subprocess
import tarfile
import time
from qualify_source import COMPILER_SHA, stack_limit

HERE = Path(__file__).resolve().parent
LANE = HERE.parents[1]
RUNTIME_SHA = "bdf524d891ad27d2f36676ef3133b22f10b196d40ea97c08cc88787b07cc1cf3"


def digest(path: Path) -> str:
    """Identify actual selected code and closed outputs before retaining or archiving them."""
    return hashlib.sha256(path.read_bytes()).hexdigest()


def directory_size(path: Path) -> int:
    """Count every new output in the selected unit's directory."""
    return sum(p.stat().st_size for p in path.rglob("*") if p.is_file())


def sample_free(report: dict) -> int:
    """Retain the minimum observed free-space sample without claiming continuous global telemetry."""
    free = shutil.disk_usage(HERE).free
    report["minimum_sampled_free_bytes"] = min(report.get("minimum_sampled_free_bytes", free), free)
    return free


def fsync_directory(path: Path) -> None:
    """Make directory updates durable after archive/index creation and original unlink."""
    fd = os.open(path, os.O_DIRECTORY)
    try:
        os.fsync(fd)
    finally:
        os.close(fd)


def atomic_json(path: Path, value: dict) -> None:
    """Flush an index before replacing its name and synchronizing the containing directory."""
    temporary = path.with_suffix(path.suffix + ".new")
    with temporary.open("w") as output:
        output.write(json.dumps(value, indent=2) + "\n")
        output.flush()
        os.fsync(output.fileno())
    os.replace(temporary, path)
    fsync_directory(path.parent)


def archive_closed(directory: Path, report: dict) -> dict:
    """Verify decoded bytes and exact metadata, then index durably before unlinking only ELF/LLVM outputs."""
    home = LANE / "build/tar-closed-functional-archives"
    home.mkdir(exist_ok=True)
    archive = home / (directory.name + ".tar.gz")
    assert not archive.exists()
    paths = [directory / "consumer", directory / "consumer.ll"]
    entries = []
    for path in paths:
        st = path.stat()
        key = "executable_sha256" if path.name == "consumer" else "llvm_sha256"
        assert digest(path) == report[key]
        entries.append({"path": str(path), "member": path.name, "sha256": digest(path), "size": st.st_size,
                        "mode": st.st_mode & 0o7777, "uid": st.st_uid, "gid": st.st_gid, "mtime_ns": st.st_mtime_ns})
    assert shutil.disk_usage(home).free >= 260 * 1024**2
    with archive.open("xb") as raw:
        with gzip.GzipFile(fileobj=raw, mode="wb", compresslevel=6, mtime=0) as compressed:
            with tarfile.open(fileobj=compressed, mode="w|", format=tarfile.PAX_FORMAT) as tar:
                for path, entry in zip(paths, entries):
                    info = tar.gettarinfo(str(path), arcname=path.name)
                    info.pax_headers["mtime"] = str(Decimal(entry["mtime_ns"]) / Decimal(10**9))
                    with path.open("rb") as source:
                        tar.addfile(info, source)
                    assert raw.tell() <= 10 * 1024**2, "Archive scratch allowance exceeded"
        raw.flush()
        os.fsync(raw.fileno())
    assert archive.stat().st_size <= 10 * 1024**2
    sample_free(report)
    with tarfile.open(archive, "r:gz") as tar:
        members = tar.getmembers()
        assert len(members) == len(entries)
        for member, expected in zip(members, entries):
            assert member.name == expected["member"] and member.size == expected["size"]
            assert (member.mode, member.uid, member.gid) == (expected["mode"], expected["uid"], expected["gid"])
            assert Decimal(member.pax_headers["mtime"]) * Decimal(10**9) == expected["mtime_ns"]
            source = tar.extractfile(member)
            assert source is not None
            hasher = hashlib.sha256()
            for chunk in iter(lambda: source.read(65536), b""):
                hasher.update(chunk)
            assert hasher.hexdigest() == expected["sha256"]
    fsync_directory(home)
    item = {"archive": str(archive), "archive_sha256": digest(archive), "compressed_size": archive.stat().st_size,
            "completed_report": str(directory / "completed-report.json"),
            "completed_report_sha256": digest(directory / "completed-report.json"), "originals": entries}
    index_path = home / "index.json"
    index = json.loads(index_path.read_text()) if index_path.exists() else {"schema": 1, "archives": []}
    index["archives"].append(item)
    atomic_json(index_path, index)
    for path in paths:
        path.unlink()
    fsync_directory(directory)
    return item


def main() -> None:
    """Select the explicit granted tuple and run normal/poison ownership checks without elapsed claims."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--consumer", type=Path, required=True)
    parser.add_argument("--artifacts", type=Path, required=True)
    parser.add_argument("--opt-level", choices=("0", "2"), required=True)
    args = parser.parse_args()
    compiler = Path("/workspace/dasl-stdlib-io/build/stdlib-entropy-docs-producer-frozen/cinch")
    runtime = Path("/workspace/dasl-stdlib-io/build/stdlib-entropy-runtime-frozen/libcinchruntime.a")
    clang = Path("/workspace/.tools/llvm/usr/lib/llvm-20/bin/clang")
    assert digest(compiler) == COMPILER_SHA and digest(runtime) == RUNTIME_SHA
    assert shutil.disk_usage(HERE).free >= 350 * 1024**2
    os.sched_setaffinity(0, {2})
    directory = args.artifacts.resolve()
    assert not directory.exists(), "Every failed project remains plain; choose a fresh epoch"
    directory.mkdir(parents=True)
    modules = {"Data.Tar": HERE / "sources/Data/Tar.cinch", "Text.Tar.Block": HERE / "sources/Text/Tar/Block.cinch",
               "Data.Bytes.Word": Path("/workspace/dasl-stdlib-io/compiler/prelude/Data/Bytes/Word.cinch")}
    for name, path in modules.items():
        (directory / (name + ".cinch")).write_bytes(path.read_bytes())
    (directory / "Main.cinch").write_bytes(args.consumer.read_bytes())
    report = {"status": "started", "compiler_sha256": digest(compiler), "runtime_sha256": digest(runtime),
              "runner_sha256": digest(Path(__file__)),
              "clang_sha256": digest(clang), "consumer_sha256": digest(args.consumer), "opt_level": args.opt_level,
              "sources": {name: digest(path) for name, path in modules.items()},
              "embedded_core_authority": COMPILER_SHA, "free_before": shutil.disk_usage(HERE).free,
              "scope": "fixed-block/numeric component only; no full header/stream/archive parity", "runs": []}
    sample_free(report)
    env = {**os.environ, "LD_LIBRARY_PATH": "/workspace/.tools/llvm/usr/lib/x86_64-linux-gnu" + (":" + os.environ["LD_LIBRARY_PATH"] if os.environ.get("LD_LIBRARY_PATH") else ""),
           "CINCH_CLANG": str(clang), "RUST_MIN_STACK": "67108864"}
    command = [str(compiler), "build", str(directory / "Main.cinch"), "--opt-level", args.opt_level,
               "--runtime-archive", str(runtime), "-o", str(directory / "consumer")]
    try:
        started = time.monotonic()
        with (directory / "build.log").open("w") as log:
            child = subprocess.Popen(command, cwd=directory, env=env, stdout=log, stderr=log,
                                     preexec_fn=stack_limit, start_new_session=True)
            try:
                while child.poll() is None:
                    assert directory_size(directory) <= 29 * 1024**2, "Runtime unit output allowance reached"
                    assert sample_free(report) >= 250 * 1024**2, "Runtime unit reserve reached"
                    assert time.monotonic() - started < 60, "Runtime unit compiler deadline reached"
                    time.sleep(.1)
            finally:
                if child.poll() is None:
                    os.killpg(child.pid, signal.SIGTERM)
                    child.wait()
        report["build"] = {"command": command, "exit": child.returncode, "seconds": time.monotonic() - started}
        assert child.returncode == 0
        report["executable_sha256"] = digest(directory / "consumer")
        report["llvm_sha256"] = digest(directory / "consumer.ll")
        for poison in (False, True):
            assert sample_free(report) >= 250 * 1024**2
            profile = {**env, "CINCH_HEAP_LIMIT": "256M" if poison else "32M", "CINCH_STACK_SIZE": "8M",
                       "CINCH_RC_DEBUG": "1" if poison else "0", "CINCH_LEAK_CHECK": "1", "CINCH_ALLOC_STATS": "1"}
            run = subprocess.run([str(directory / "consumer")], cwd=directory, env=profile,
                                 capture_output=True, text=True, timeout=15)
            item = {"poison": poison, "heap": profile["CINCH_HEAP_LIMIT"], "stack": "8M", "exit": run.returncode,
                    "stdout": run.stdout, "stderr": run.stderr}
            report["runs"].append(item)
            sample_free(report)
            (directory / ("poison.log" if poison else "normal.log")).write_text(run.stdout + run.stderr)
            assert run.returncode == 0 and run.stdout == "", item
            assert "cinch-leak: live=0 immortal=0 leaked=0" in run.stderr, item
        report["output_bytes"] = directory_size(directory)
        assert report["output_bytes"] <= 30 * 1024**2
        report["status"] = "normal-and-poison-component-passed"
        atomic_json(directory / "completed-report.json", report)
        report["archive"] = archive_closed(directory, report)
        sample_free(report)
    except BaseException as error:
        report["status"] = "failed-preserved-plain"
        report["failure"] = repr(error)
        raise
    finally:
        report["free_after"] = shutil.disk_usage(HERE).free
        atomic_json(directory / "report.json", report)
    print(json.dumps({key: report.get(key) for key in ("status", "output_bytes", "free_after")}))


if __name__ == "__main__":
    main()
