#!/usr/bin/env python3
"""Durably archive one completed bounded map runtime unit without changing its recoverable bytes."""
from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import tarfile

from archive_map_functional_elfs import ROOT, digest, save_index, sync_directory


def main() -> None:
    """Verify complete normal/poison verdicts and every archive member before unlinking closed outputs."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--report", type=Path, required=True)
    parser.add_argument("--cpu", type=int, required=True)
    args = parser.parse_args()
    os.sched_setaffinity(0, {args.cpu})
    report = json.loads(args.report.read_text())
    assert report["status"] == "passed" and len(report["suites"]) == 1
    suite = report["suites"][0]
    assert suite["status"] == "passed" and len(suite["runs"]) == 2
    for run in suite["runs"]:
        assert run["exit"] == 0 and "cinch-leak: live=0 immortal=0 leaked=0" in run["stderr"]
    executable = Path(suite["runs"][0]["command"][0]).resolve()
    assert executable.is_relative_to(ROOT / "build") and executable.parent.parent.name.startswith("maps-855-")
    assert executable.name == "consumer" and digest(executable) == suite["executable_sha256"]
    llvm = executable.with_suffix(".ll")
    assert digest(llvm) == suite["llvm_sha256"]
    files = [executable, llvm] + sorted(path for path in (executable.parent / "build").rglob("*") if path.is_file())
    members = {str(path.relative_to(ROOT)): {"sha256": digest(path), "bytes": path.stat().st_size} for path in files}
    directory = ROOT / "build/maps-855-closed-unit-archive"
    directory.mkdir(parents=True, exist_ok=True)
    key = hashlib.sha256(str(executable).encode()).hexdigest()[:16] + "-" + suite["name"]
    archive, snapshot = directory / (key + ".tar.gz"), directory / (key + ".result.json")
    assert not archive.exists() and not snapshot.exists(), "retain earlier closed units without overwriting"
    with snapshot.open("xb") as output:
        output.write(args.report.read_bytes())
        output.flush()
        os.fsync(output.fileno())
    temporary = archive.with_suffix(".gz.tmp")
    with temporary.open("wb") as output:
        with tarfile.open(fileobj=output, mode="w:gz", compresslevel=1) as packed:
            for path in files:
                packed.add(path, arcname=str(path.relative_to(ROOT)), recursive=False)
        output.flush()
        os.fsync(output.fileno())
    with tarfile.open(temporary, "r:gz") as packed:
        assert len(packed.getmembers()) == len(members)
        for member in packed.getmembers():
            expected = members[member.name]
            checksum, size = hashlib.sha256(), 0
            stream = packed.extractfile(member)
            while block := stream.read(1024 * 1024):
                checksum.update(block)
                size += len(block)
            assert (checksum.hexdigest(), size) == (expected["sha256"], expected["bytes"])
    os.replace(temporary, archive)
    sync_directory(directory)
    index_path = directory / "index.json"
    index = json.loads(index_path.read_text()) if index_path.exists() else {"kind": "closed-map-runtime-units", "entries": []}
    entry = {"report": str(args.report.resolve()), "result_snapshot": str(snapshot.relative_to(ROOT)),
             "result_snapshot_sha256": digest(snapshot), "archive": str(archive.relative_to(ROOT)),
             "archive_sha256": digest(archive), "archive_bytes": archive.stat().st_size,
             "members": members, "originals_removed": False}
    index["entries"].append(entry)
    save_index(index_path, index)
    report["closed_archive"] = {"index": str(index_path.relative_to(ROOT)), **entry}
    save_index(args.report.resolve(), report)
    for path in files:
        assert digest(path) == members[str(path.relative_to(ROOT))]["sha256"]
        path.unlink()
    for parent in {path.parent for path in files}:
        sync_directory(parent)
    entry["originals_removed"] = True
    save_index(index_path, index)
    report["closed_archive"]["originals_removed"] = True
    save_index(args.report.resolve(), report)
    print(f"Archived {suite['name']}: recovered {sum(row['bytes'] for row in members.values()) - entry['archive_bytes']} bytes", flush=True)


if __name__ == "__main__":
    main()
