#!/usr/bin/env python3
"""Restore exact completed functional ELF bytes from the immutable map archive mapping."""
from __future__ import annotations

import argparse
import gzip
import os
from pathlib import Path
import shutil
import json

from archive_map_functional_elfs import ROOT, digest, sync_directory, unpacked_identity


def main() -> None:
    """Verify compressed and decompressed identities before restoring one owner-executable file."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--index", type=Path, default=ROOT / "build/maps-functional-elf-archive/index.json")
    parser.add_argument("--original", required=True, help="Original path relative to this worktree, exactly as recorded by the archive")
    args = parser.parse_args()
    index = json.loads(args.index.read_text())
    entry = index["entries"][args.original]
    original = (ROOT / entry["original"]).resolve()
    assert original.is_relative_to(ROOT / "build/maps-region-safe")
    archive = ROOT / entry["archive"]
    assert digest(archive) == entry["archive_sha256"]
    assert unpacked_identity(archive) == (entry["original_sha256"], entry["original_bytes"])
    if original.exists():
        assert digest(original) == entry["original_sha256"], "refusing to replace different artifacts"
        return
    temporary = original.with_suffix(".restored.tmp")
    with gzip.open(archive, "rb") as source:
        with temporary.open("wb") as output:
            shutil.copyfileobj(source, output, length=1024 * 1024)
            output.flush()
            os.fchmod(output.fileno(), 0o700)
            os.fsync(output.fileno())
    assert temporary.stat().st_size == entry["original_bytes"]
    assert digest(temporary) == entry["original_sha256"]
    os.replace(temporary, original)
    sync_directory(original.parent)
    print(f"Restored {entry['original']}: {entry['original_sha256']}")


if __name__ == "__main__":
    main()
