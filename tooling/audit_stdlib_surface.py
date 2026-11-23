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
    "bufio": ["Effect.Stream", "Effect.Buffered.Core"], "bytes": ["Data.Bytes", "Data.Bytes.Word", "Data.Bytes.Search", "Data.Bytes.Build", "Data.StringIo", "Data.Bytes.View"],
    "html": ["Text.Html"],
    "cmp": ["Data.Compare", "Relation.Ord"], "compress/flate": ["Text.Zlib"], "compress/zlib": ["Text.Zlib"],
    "container/heap": ["Data.Mutable.Heap", "Data.Mutable.HeapView"], "container/list": ["Data.List", "Data.Mutable.LinkedList"], "container/ring": ["Data.Mutable.Ring"], "context": ["Effect.Concurrent", "Effect.Clock"],
    "crypto/hmac": ["Text.Digest"], "crypto/md5": ["Text.Digest"], "crypto/rand": ["Effect.Random"],
    "crypto/sha1": ["Text.Digest"], "crypto/sha256": ["Text.Digest", "Text.Sha256", "Effect.Sha256"], "crypto/sha512": ["Text.Digest"],
    "crypto/subtle": ["Crypto.ConstantTime"],
    "encoding/ascii85": ["Text.Ascii85", "Effect.Ascii85"],
    "encoding/base32": ["Text.Base32", "Effect.Base32"], "encoding/base64": ["Text.Base64", "Text.Base64.Encoding", "Effect.Base64"],
    "encoding/binary": ["Text.Binary", "Text.Binary.Layout", "Effect.Binary"], "encoding/csv": ["Text.Csv", "Text.Csv.Record", "Effect.Csv"], "encoding/hex": ["Text.Hex", "Effect.Hex"],
    "encoding/json": ["Text.Json", "Text.Json.Value", "Text.Json.Codec", "Text.Json.Binary32", "Data.Json"], "errors": ["Data.Result"],
    "encoding/pem": ["Text.Pem", "Effect.Pem"],
    "mime": ["Text.Mime", "Effect.Mime"],
    "flag": ["Text.OptionParser"], "fmt": ["Text.Show", "Text.Scan"],
    "hash": ["Data.Hash"], "hash/adler32": ["Text.Checksum", "Text.Adler32", "Effect.Adler32"], "hash/crc32": ["Text.Checksum", "Text.Crc32", "Effect.Crc32"],
    "hash/crc64": ["Text.Checksum", "Text.Crc64", "Effect.Crc64"], "hash/fnv": ["Text.Fnv", "Text.Fnv.State", "Effect.Fnv"], "hash/maphash": ["Data.HashMap"],
    "io": ["Effect.Stream"], "io/fs": ["Effect.Fs", "Effect.Find"],
    "io/ioutil": ["Effect.Fs", "Effect.Stream"], "iter": ["Data.Producer", "Data.List"],
    "log": ["Effect.Log"], "log/slog": ["Effect.Log"], "maps": ["Data.Map", "Data.HashMap", "Data.Mutable.HashTable", "Data.Mutable.BTree", "Data.Mutable.IndexMap"],
    "math": ["Data.Machine"], "math/big": ["Data.Nat", "Data.Int", "Data.Rational", "Data.BigDecimal"],
    "math/bits": ["Algebra.Bits", "Data.Machine", "Data.Word"], "math/cmplx": ["Data.Complex"],
    "math/rand": ["Effect.Random"], "math/rand/v2": ["Effect.Random"],
    "net": ["Effect.Net", "Effect.Resolv", "Data.IpAddr"], "net/http": ["Effect.Http"],
    "net/netip": ["Data.IpAddr"], "net/url": ["Text.Uri", "Text.Url", "Text.Url.Codec", "Text.Url.Error", "Text.Url.Query", "Text.Url.Userinfo", "Text.Url.Model", "Text.Url.Parser", "Text.Url.Resolve", "Text.Url.Ip", "Text.Url.Scan"],
    "os": ["Effect.Fs", "Effect.Process", "Effect.Stream"], "os/exec": ["Effect.Process"],
    "os/signal": ["Effect.Signal"], "path": ["Data.Path", "Text.Path"], "path/filepath": ["Data.Path", "Effect.Find", "Text.FilePath"],
    "regexp": ["Text.Regex"], "regexp/syntax": ["Text.Regex"],
    "slices": ["Data.List", "Data.Array", "Data.Mutable.Vec"], "sort": ["Data.List", "Data.Sorted", "Data.Mutable.Vec", "Data.Search"],
    "strconv": ["Text.Show", "Text.Scan", "Data.Machine"], "strings": ["Data.String", "Text.Scan", "Text.Builder"],
    "sync": ["Effect.Stm", "Effect.Concurrent.MVar"], "sync/atomic": ["Effect.Stm"], "syscall": ["Prim.Sys"],
    "testing": ["Effect.Test", "Effect.Benchmark", "Effect.Coverage"],
    "text/scanner": ["Text.Lex", "Text.Scan"], "time": ["Data.Date", "Data.Time", "Effect.Clock"],
    "unicode": ["Data.Char", "Data.Unicode", "Data.Unicode.Kernel", "Data.Unicode.Tables", "Data.Unicode.Range", "Data.Unicode.Names"], "unicode/utf16": ["Text.Utf16"], "unicode/utf8": ["Data.Char", "Data.String", "Data.Bytes.Utf8", "Text.Utf8"],
}

EVIDENCE = {
    "crypto/subtle": (["lib_constant_time"], ["constant-time-equality", "constant-time-copy", "constant-time-xor"]),
    "container/list": (["lib_linked_list"], ["append-drain", "prepend-drain", "move-front", "remove-reuse", "self-copy"]),
    "encoding/json": (["lib_json_value"], []),
    "bytes": (["lib_byte_search", "lib_byte_build"], []),
    "container/heap": (["lib_heap_view"], ["owned-build-drain", "view-build-drain", "owned-root-repair", "view-root-repair"]),
    "cmp": (["lib_compare"], ["integer", "float", "choice", "string"]),
    "unicode": (["lib_unicode_kernel", "lib_unicode_catalog"], []),
    "encoding/base64": (["lib_base64", "lib_base64_encoding"], ["encode-bytes", "decode-bytes", "custom-encode", "custom-decode", "legacy-encode", "legacy-decode", "stream-encode64", "stream-decode64"]),
    "html": (["lib_html"], ["escape-bytes", "unescape-bytes", "escape-text", "unescape-text", "escape-length"]),
    "path": (["lib_path"], ["clean-component", "clean-segments", "base", "ext", "compiled-match", "match", "join"]),
    "crypto/sha256": (["lib_sha256"], ["sha256", "sha224", "sha256-state", "sha224-state", "sha256-roundtrip", "sha224-roundtrip", "sha256-stream", "sha224-stream", "sha256-stream4096", "sha224-stream4096", "sha256-runtime"]),
    "math/bits": (["lib_word_ops::word_bit_operations_match_reference_widths_and_rotations", "lib_word_ops::word_wide_arithmetic_matches_reference_carry_borrow_and_products", "lib_word_ops::word_division_matches_reference_and_names_invalid_inputs"], ["word-add64", "word-mul64", "word-div64", "word-bits64", "word-load"]),
    "unicode/utf16": (["lib_utf16::utf16_matches_scalar_and_surrogate_reference_vectors", "lib_utf16::utf16_byte_codecs_match_text_and_reject_odd_lengths"], ["utf16le-encode", "utf16le-decode", "utf16be-encode", "utf16be-decode"]),
    "encoding/ascii85": (["lib_ascii85", "lib_ascii85_buffers"], ["ascii85-encode", "ascii85-decode", "ascii85-stream-encode", "ascii85-stream-decode", "ascii85-bytes-encode", "ascii85-into-encode", "ascii85-into-decode", "ascii85-prefix-decode", "ascii85-controls-decode", "ascii85-controls-prefix-decode", "ascii85-offset-decode"]),
    "encoding/base32": (["lib_go_stdlib::base32_matches_go_rfc4648_both_alphabets_and_raw_forms", "lib_go_stdlib::base32_scalar_tails_follow_long_simd_prefixes", "lib_base32_encoding"], ["base32-encode", "base32-decode", "encode-bytes", "decode-bytes", "custom-encode", "custom-decode", "stream-encode64", "stream-decode64"]),
    "encoding/csv": (["lib_csv_record"], ["read-plain", "read-quoted", "write-plain", "write-quoted", "stream-read", "stream-write"]),
    "encoding/binary": (["lib_go_stdlib::fixed_width_binary_matches_go_and_validates_writes", "lib_go_stdlib::varints_match_go_including_signed_extrema_and_malformed_input"], ["binary", "varint", "encode", "roundtrip"]),
    "encoding/hex": (["lib_hex", "lib_hex_stream"], ["hex-encode", "hex-decode", "hex-append-encode", "hex-append-decode", "hex-into-encode", "hex-into-decode", "hex-dump", "hex-stream-encode", "hex-stream-decode", "hex-stream-dump", "hex-bytes-encode", "hex-prefix-decode"]),
    "hash/adler32": (["lib_go_stdlib::checksums_match_go_and_update_over_every_split", "lib_adler32"], ["adler32", "adler32-state", "adler32-roundtrip", "adler32-stream", "adler32-stream4096"]),
    "hash/crc32": (["lib_go_stdlib::checksums_match_go_and_update_over_every_split", "lib_crc32::prepared_crc32_checkpoint_and_stream"], ["crc32", "crc32c"]),
    "hash/crc64": (["lib_go_stdlib::checksums_match_go_and_update_over_every_split", "lib_crc64"], ["crc64", "crc64-table", "crc64-state", "crc64-roundtrip", "crc64-stream", "crc64-stream4096", "crc64-table-build"]),
    "hash/fnv": (["lib_go_stdlib::fnv_matches_go_and_updates_over_every_split", "lib_fnv128::fnv128_matches_reference_vectors_and_every_chunk_split", "lib_fnv128::fnv128_updates_preserve_arbitrary_high_bits_and_exact_tails", "lib_fnv_state"], ["fnv32a", "fnv64a", "fnv128", "fnv128a"] + [f"fnv-{kind}{width}" for kind in ("state", "roundtrip", "stream") for width in ("32", "32a", "64", "64a", "128", "128a")]),
    "net/url": (["lib_url"], ["query-escape", "query-unescape", "path-escape", "path-unescape", "parse-render", "query-values", "resolve", "join"]),
    "io": (["lib_stream"], ["stream-copy-buffer", "stream-copy-discard", "stream-read-all"]),
    "unicode/utf8": (["lib_word_utf8", "lib_utf8"], []),
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
    print(f"Recorded {len(entries)} packages: {sum(p['status']=='complete' for p in entries)} complete, {sum(p['status']=='partial' for p in entries)} partial, {sum(p['status']=='unreviewed' for p in entries)} unreviewed")


if __name__ == "__main__":
    main()
