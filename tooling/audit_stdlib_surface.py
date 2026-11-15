#!/usr/bin/env python3
"""Inventory a pinned reference SDK without mistaking module names for coverage.

Public packages are discovered from the actual SDK. Export declarations come
from its cumulative compatibility files, deduplicated across target platforms.
Existing review/evidence fields survive regeneration. Unmapped or unaudited
packages stay explicitly open; this tool never promotes a package to complete.
"""
from __future__ import annotations

import argparse
import json
import os
from pathlib import Path
import re
import subprocess

ROOT = Path(__file__).resolve().parents[1]
DEFAULT = ROOT / "design/stdlib-surface.json"

HOMES = {
    "bufio": ["Effect.Stream"], "bytes": ["Data.Bytes", "Data.Bytes.Word", "Data.StringIo"],
    "cmp": ["Relation.Ord"], "compress/flate": ["Text.Zlib"], "compress/zlib": ["Text.Zlib"],
    "container/heap": ["Data.Mutable.Heap"], "container/list": ["Data.List"], "container/ring": ["Data.Mutable.Deque"], "context": ["Effect.Concurrent", "Effect.Clock"],
    "crypto/hmac": ["Text.Digest"], "crypto/md5": ["Text.Digest"], "crypto/rand": ["Effect.Random"],
    "crypto/sha1": ["Text.Digest"], "crypto/sha256": ["Text.Digest"], "crypto/sha512": ["Text.Digest"],
    "encoding/base32": ["Text.Base32"], "encoding/base64": ["Text.Base64"],
    "encoding/binary": ["Text.Binary"], "encoding/csv": ["Text.Csv"], "encoding/hex": ["Text.Hex"],
    "encoding/json": ["Text.Json", "Data.Json"], "errors": ["Data.Result"],
    "flag": ["Text.OptionParser"], "fmt": ["Text.Show", "Text.Scan"],
    "hash": ["Data.Hash"], "hash/adler32": ["Text.Checksum"], "hash/crc32": ["Text.Checksum"],
    "hash/crc64": ["Text.Checksum"], "hash/fnv": ["Text.Fnv"], "hash/maphash": ["Data.HashMap"],
    "io": ["Effect.Stream"], "io/fs": ["Effect.Fs", "Effect.Find"],
    "io/ioutil": ["Effect.Fs", "Effect.Stream"], "iter": ["Data.Producer", "Data.List"],
    "log": ["Effect.Log"], "log/slog": ["Effect.Log"], "maps": ["Data.Map", "Data.HashMap", "Data.Mutable.HashTable", "Data.Mutable.BTree", "Data.Mutable.IndexMap"],
    "math": ["Data.Machine"], "math/big": ["Data.Nat", "Data.Int", "Data.Rational", "Data.BigDecimal"],
    "math/bits": ["Algebra.Bits", "Data.Machine"], "math/cmplx": ["Data.Complex"],
    "math/rand": ["Effect.Random"], "math/rand/v2": ["Effect.Random"],
    "net": ["Effect.Net", "Effect.Resolv", "Data.IpAddr"], "net/http": ["Effect.Http"],
    "net/netip": ["Data.IpAddr"], "net/url": ["Text.Uri"],
    "os": ["Effect.Fs", "Effect.Process", "Effect.Stream"], "os/exec": ["Effect.Process"],
    "os/signal": ["Effect.Signal"], "path": ["Data.Path"], "path/filepath": ["Data.Path", "Effect.Find"],
    "regexp": ["Text.Regex"], "regexp/syntax": ["Text.Regex"],
    "slices": ["Data.List", "Data.Array", "Data.Mutable.Vec"], "sort": ["Data.List", "Data.Sorted", "Data.Mutable.Vec"],
    "strconv": ["Text.Show", "Text.Scan", "Data.Machine"], "strings": ["Data.String", "Text.Scan", "Text.Builder"],
    "sync": ["Effect.Stm"], "sync/atomic": ["Effect.Stm"], "syscall": ["Prim.Sys"],
    "testing": ["Effect.Test", "Effect.Benchmark", "Effect.Coverage"],
    "text/scanner": ["Text.Lex", "Text.Scan"], "time": ["Data.Date", "Data.Time", "Effect.Clock"],
    "unicode": ["Data.Char"], "unicode/utf8": ["Data.Char", "Data.String"],
}

EVIDENCE = {
    "encoding/base32": (["lib_go_stdlib::base32_matches_go_rfc4648_both_alphabets_and_raw_forms", "lib_go_stdlib::base32_scalar_tails_follow_long_simd_prefixes"], ["base32-encode", "base32-decode"]),
    "encoding/binary": (["lib_go_stdlib::fixed_width_binary_matches_go_and_validates_writes", "lib_go_stdlib::varints_match_go_including_signed_extrema_and_malformed_input"], ["binary", "varint"]),
    "encoding/hex": (["lib_go_stdlib::hexadecimal_matches_go_and_names_invalid_bytes"], ["hex-encode", "hex-decode"]),
    "hash/adler32": (["lib_go_stdlib::checksums_match_go_and_update_over_every_split"], ["adler32"]),
    "hash/crc32": (["lib_go_stdlib::checksums_match_go_and_update_over_every_split"], ["crc32", "crc32c"]),
    "hash/crc64": (["lib_go_stdlib::checksums_match_go_and_update_over_every_split"], ["crc64"]),
    "hash/fnv": (["lib_go_stdlib::fnv_matches_go_and_updates_over_every_split"], ["fnv32a", "fnv64a"]),
    "io": (["lib_stream"], ["stream-copy-buffer", "stream-copy-discard", "stream-read-all"]),
    "unicode/utf8": (["lib_word_utf8"], []),
}


def command(go: Path, *args: str) -> str:
    """Query the reference toolchain without writing to a read-only home."""
    env = {**os.environ, "GOCACHE": os.environ.get("GOCACHE", "/tmp/cinch-go-surface-cache"), "GOTELEMETRY": "off"}
    return subprocess.run([str(go), *args], check=True, capture_output=True, text=True, env=env, timeout=120).stdout


def declarations(root: Path) -> dict[str, list[str]]:
    """Read cumulative public declarations, ignoring platform duplicates and issue annotations."""
    found: dict[str, set[str]] = {}
    for source in sorted((root / "api").glob("go1*.txt")):
        for line in source.read_text().splitlines():
            match = re.match(r"pkg ([^ ,]+)(?: \([^)]+\))?, (.*?)(?: #\d+)?$", line)
            if match:
                found.setdefault(match[1], set()).add(match[2])
    return {name: sorted(entries) for name, entries in found.items()}


def main() -> None:
    """Refresh inventory or print a package's exact operation-level review surface."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--go", type=Path, required=True)
    parser.add_argument("--output", type=Path, default=DEFAULT)
    parser.add_argument("--package", help="Print the full reference declarations for one package")
    parser.add_argument("--check", action="store_true", help="Verify the recorded reference package set and declaration counts")
    args = parser.parse_args()
    go = args.go.resolve()
    sdk = Path(command(go, "env", "GOROOT").strip())
    exported = declarations(sdk)
    if args.package:
        print(json.dumps({"package": args.package, "declarations": exported.get(args.package, [])}, indent=2))
        return
    packages = [name for name in command(go, "list", "std").splitlines() if not {"internal", "vendor"}.intersection(name.split("/"))]
    old = json.loads(args.output.read_text()) if args.output.exists() else {}
    previous = {p["reference_package"]: p for p in old.get("packages", [])}
    entries = []
    for name in sorted(packages):
        entry = dict(previous[name]) if name in previous else {"reference_package": name, "status": "partial" if name in HOMES else "unreviewed", "cinch_modules": HOMES.get(name, []), "remaining": []}
        if name in HOMES:
            entry["cinch_modules"] = sorted(set(entry.get("cinch_modules", []) + HOMES[name]))
            if entry["status"] == "unreviewed":
                entry["status"] = "partial"
        entry["reference_declarations"] = len(exported.get(name, []))
        if name in EVIDENCE:
            entry["tests"], entry["benchmarks"] = EVIDENCE[name]
        entries.append(entry)
    if args.check:
        assert set(previous) == set(packages), "reference package set changed"
        for entry in entries:
            assert previous[entry["reference_package"]]["reference_declarations"] == entry["reference_declarations"], entry["reference_package"]
        print(f"Inventory matches {len(entries)} public packages")
        return
    report = {"format": 1, "reference_sdk": command(go, "version").strip(),
              "scope": "All public reference packages; internal/vendor implementation packages excluded. Language/runtime-specific packages remain open until an idiomatic mapping is reviewed.",
              "completion_rule": "A package is complete only after operation-level capability mapping, implemented public declarations, runtime/error tests and relevant benchmarks have been reviewed. Module presence alone is partial coverage.",
              "packages": entries}
    args.output.write_text(json.dumps(report, indent=2) + "\n")
    print(f"Recorded {len(entries)} packages: {sum(p['status']=='partial' for p in entries)} partial, {sum(p['status']=='unreviewed' for p in entries)} unreviewed")


if __name__ == "__main__":
    main()
