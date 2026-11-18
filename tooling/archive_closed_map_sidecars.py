#!/usr/bin/env python3
"""Preserve closed map LLVM/cache bytes in verified archives before removing redundant local files."""
from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import tarfile

from archive_map_functional_elfs import ROOT, digest, save_index, sync_directory


def main() -> None:
    """Archive only the twelve already-completed functional predecessors, excluding every live input."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--cpu", type=int, required=True)
    args = parser.parse_args()
    os.sched_setaffinity(0, {args.cpu})
    receipt = ROOT / "build/maps-functional-elf-archive/receipt.json"
    records = json.loads(receipt.read_text())
    directory = ROOT / "build/maps-functional-sidecar-archive"
    directory.mkdir(parents=True, exist_ok=True)
    index_path = directory / "index.json"
    assert not index_path.exists(), "retain the immutable recovery mapping from any earlier run"
    index = {"kind": "completed-map-functional-sidecars", "receipt_sha256": digest(receipt), "entries": []}
    for report_name, report_sha in records["reports"].items():
        report_path = ROOT / report_name
        assert digest(report_path) == report_sha
        report = json.loads(report_path.read_text())
        assert report["status"] == "passed"
        for suite in report["suites"]:
            assert suite["status"] == "passed" and len(suite["runs"]) == 2
            executable = Path(suite["runs"][0]["command"][0]).resolve()
            assert executable.is_relative_to(ROOT / "build/maps-region-safe") and executable.name == "consumer"
            llvm = executable.with_suffix(".ll")
            assert digest(llvm) == suite["llvm_sha256"]
            files = [llvm] + sorted(path for path in (executable.parent / "build").rglob("*") if path.is_file())
            members = {str(path.relative_to(ROOT)): {"sha256": digest(path), "bytes": path.stat().st_size} for path in files}
            name = hashlib.sha256(str(executable).encode()).hexdigest()[:16] + "-" + suite["name"] + ".tar.gz"
            archive = directory / name
            temporary = archive.with_suffix(".gz.tmp")
            assert not archive.exists() and not temporary.exists()
            with temporary.open("wb") as output:
                with tarfile.open(fileobj=output, mode="w:gz", compresslevel=1) as packed:
                    for path in files:
                        packed.add(path, arcname=str(path.relative_to(ROOT)), recursive=False)
                output.flush()
                os.fsync(output.fileno())
            with tarfile.open(temporary, "r:gz") as packed:
                actual = packed.getmembers()
                assert len(actual) == len(members)
                for member in actual:
                    expected = members[member.name]
                    stream = packed.extractfile(member)
                    checksum = hashlib.sha256()
                    size = 0
                    while data := stream.read(1024 * 1024):
                        checksum.update(data)
                        size += len(data)
                    assert (checksum.hexdigest(), size) == (expected["sha256"], expected["bytes"])
            os.replace(temporary, archive)
            sync_directory(archive.parent)
            entry = {"suite": suite["name"], "report": report_name, "report_sha256": report_sha,
                     "archive": str(archive.relative_to(ROOT)), "archive_sha256": digest(archive),
                     "archive_bytes": archive.stat().st_size, "members": members, "originals_removed": False}
            index["entries"].append(entry)
            save_index(index_path, index)
            for path in files:
                assert digest(path) == members[str(path.relative_to(ROOT))]["sha256"]
                path.unlink()
            for parent in {path.parent for path in files}:
                sync_directory(parent)
            entry["originals_removed"] = True
            save_index(index_path, index)
            recovered = sum(row["bytes"] for row in members.values()) - entry["archive_bytes"]
            print(f"{suite['name']}: {len(members)} exact sidecars archived; recovered {recovered} bytes", flush=True)
    assert len(index["entries"]) == 12


if __name__ == "__main__":
    main()
