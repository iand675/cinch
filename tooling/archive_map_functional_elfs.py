#!/usr/bin/env python3
"""Archive only completed map functional ELFs with a durable, hash-verified recovery index."""
from __future__ import annotations

import argparse
import gzip
import hashlib
import json
import os
from pathlib import Path
import shutil

from validate_maps import ROOT, digest


def sync_directory(directory: Path) -> None:
    """Make archive/index replacement and original removal durable in their parent directory."""
    descriptor = os.open(directory, os.O_RDONLY | os.O_DIRECTORY)
    try:
        os.fsync(descriptor)
    finally:
        os.close(descriptor)


def save_index(path: Path, index: dict) -> None:
    """Persist the recovery mapping before removing any verified original ELF."""
    temporary = path.with_suffix(".json.tmp")
    with temporary.open("w") as output:
        output.write(json.dumps(index, indent=2) + "\n")
        output.flush()
        os.fsync(output.fileno())
    os.replace(temporary, path)
    sync_directory(path.parent)


def unpacked_identity(path: Path) -> tuple[str, int]:
    """Verify the complete gzip payload and exact decompressed byte count by streaming it."""
    checksum, size = hashlib.sha256(), 0
    with gzip.open(path, "rb") as source:
        while block := source.read(1024 * 1024):
            checksum.update(block)
            size += len(block)
    return checksum.hexdigest(), size


def main() -> None:
    """Select only passed functional receipts, excluding every benchmark and compiler input."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--cpu", type=int, required=True)
    parser.add_argument("--receipt", type=Path, default=ROOT / "compiler/evidence/stdlib/stdlib-maps-runtime-results.json")
    parser.add_argument("--archive", type=Path, default=ROOT / "build/maps-functional-elf-archive")
    args = parser.parse_args()
    os.sched_setaffinity(0, {args.cpu})
    receipt = json.loads(args.receipt.read_text())
    archive = args.archive.resolve()
    archive.mkdir(parents=True, exist_ok=True)
    receipt_snapshot = archive / "receipt.json"
    if not receipt_snapshot.exists():
        with receipt_snapshot.open("xb") as output:
            output.write(args.receipt.read_bytes())
            output.flush()
            os.fsync(output.fileno())
        sync_directory(archive)
    assert digest(receipt_snapshot) == digest(args.receipt)
    index_path = archive / "index.json"
    index = json.loads(index_path.read_text()) if index_path.exists() else {
        "kind": "lossless-completed-map-functional-elf-archive", "receipt_sha256": digest(args.receipt), "entries": {},
    }
    assert index["receipt_sha256"] == digest(args.receipt)
    for report_name, report_hash in receipt["reports"].items():
        report_path = ROOT / report_name
        assert digest(report_path) == report_hash
        report = json.loads(report_path.read_text())
        assert report["status"] == "passed"
        for suite in report["suites"]:
            assert suite["status"] == "passed" and len(suite["runs"]) == 2
            original = Path(suite["runs"][0]["command"][0]).resolve()
            assert original.is_relative_to(ROOT / "build/maps-region-safe")
            assert original.name == "consumer"
            original_hash = suite["executable_sha256"]
            key = str(original.relative_to(ROOT))
            name = hashlib.sha256(key.encode()).hexdigest()[:16] + "-" + original.parent.name + ".elf.gz"
            compressed = archive / name
            if key in index["entries"]:
                entry = index["entries"][key]
                assert entry["original_sha256"] == original_hash
                assert digest(compressed) == entry["archive_sha256"]
                assert unpacked_identity(compressed) == (original_hash, entry["original_bytes"])
            else:
                assert original.is_file() and digest(original) == original_hash
                original_size = original.stat().st_size
                temporary = compressed.with_suffix(".gz.tmp")
                with temporary.open("wb") as output:
                    with gzip.GzipFile(filename="", mode="wb", compresslevel=1, mtime=0, fileobj=output) as packed:
                        with original.open("rb") as source:
                            shutil.copyfileobj(source, packed, length=1024 * 1024)
                    output.flush()
                    os.fsync(output.fileno())
                os.replace(temporary, compressed)
                sync_directory(archive)
                assert unpacked_identity(compressed) == (original_hash, original_size)
                entry = {"original": key, "original_sha256": original_hash, "original_bytes": original_size,
                         "archive": str(compressed.relative_to(ROOT)), "archive_sha256": digest(compressed),
                         "archive_bytes": compressed.stat().st_size, "decompressed_sha256": original_hash,
                         "report": report_name, "report_sha256": report_hash, "original_removed": False}
                index["entries"][key] = entry
                save_index(index_path, index)
            if original.exists():
                assert digest(original) == original_hash
                original.unlink()
                sync_directory(original.parent)
            entry["original_removed"] = True
            save_index(index_path, index)
            print(f"Archived {suite['name']}: {entry['original_bytes']} -> {entry['archive_bytes']} bytes", flush=True)
    assert len(index["entries"]) == 12


if __name__ == "__main__":
    main()
