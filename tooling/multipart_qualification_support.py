#!/usr/bin/env python3
"""Maintain the real multipart fixture objects and verified closed-output recovery.

These helpers serve the isolated candidate's SDK-free qualification without
importing baseline production runners or historical experiment entrypoints.
"""
from __future__ import annotations
import gzip
import hashlib
import io
import json
import os
from pathlib import Path
import re
import shutil
import tarfile
from check_compare import digest, run


#: Repository root for the maintained independent C fixture.
ROOT = Path(__file__).resolve().parents[1]


def artifact_digest(path: Path) -> str:
    """Verify exact original bytes through an explicitly indexed lossless archive."""
    if path.is_file():
        return digest(path)
    manifest = path.parent.parent / "completed-client-archives.json"
    if not manifest.is_file():
        raise FileNotFoundError(path)
    rows = json.loads(manifest.read_text())["files"]
    row = next((entry for entry in rows if Path(entry["original"]).resolve() == path.resolve()), None)
    if row is None:
        raise RuntimeError("unindexed missing artifact: " + str(path))
    archive = Path(row["archive"])
    if digest(archive) != row["archive_sha256"]:
        raise RuntimeError("changed compressed artifact: " + str(archive))
    value, size = hashlib.sha256(), 0
    with gzip.open(archive, "rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            value.update(chunk); size += len(chunk)
    if value.hexdigest() != row["original_sha256"] or size != row["original_bytes"]:
        raise RuntimeError("changed decoded artifact: " + str(archive))
    return value.hexdigest()


def archive_completed(work: Path, label: str, name: str, report: dict) -> None:
    """Archive only fully passed client ELF/LLVM after a durable verified recovery index."""
    row = next(entry for entry in report["runs"] if entry["frontend"] == label and entry["case"] == name)
    if len(row["profiles"]) != 2 or any("leaked=0" not in profile["stderr"] for profile in row["profiles"]):
        raise RuntimeError("refusing to archive incomplete client")
    manifest = work.parent / "completed-client-archives.json"
    index = json.loads(manifest.read_text()) if manifest.exists() else {"scope": "Passed functional client ELF/LLVM only; sources, benchmarks, frozen compilers, providers and runtime inputs excluded.", "files": []}
    snapshot = work / f"{label}-{name}.archive-report.json"
    if not snapshot.exists():
        with snapshot.open("w") as output:
            output.write(json.dumps(report, indent=2) + "\n"); output.flush(); os.fsync(output.fileno())
    for suffix, key in ((".ll", "ir_sha256"), ("", "executable_sha256")):
        path = work / f"{label}-{name}{suffix}"
        if any(Path(entry["original"]) == path for entry in index["files"]):
            if artifact_digest(path) != row[key]:
                raise RuntimeError("changed previously archived client")
            continue
        if not path.is_file() or path.parent != work or digest(path) != row[key]:
            raise RuntimeError("changed completed client before archive")
        size, mode = path.stat().st_size, path.stat().st_mode & 0o777
        archive = Path(str(path) + ".gz"); pending = Path(str(archive) + ".new")
        with path.open("rb") as source, pending.open("wb") as raw:
            with gzip.GzipFile(filename="", fileobj=raw, mode="wb", compresslevel=1, mtime=0) as output:
                shutil.copyfileobj(source, output, 1024 * 1024)
            raw.flush(); os.fsync(raw.fileno())
        decoded, decoded_size = hashlib.sha256(), 0
        with gzip.open(pending, "rb") as source:
            for chunk in iter(lambda: source.read(1024 * 1024), b""):
                decoded.update(chunk); decoded_size += len(chunk)
        if decoded.hexdigest() != row[key] or decoded_size != size:
            raise RuntimeError("archive roundtrip mismatch")
        pending.replace(archive)
        directory = os.open(work, os.O_RDONLY | os.O_DIRECTORY)
        try:
            os.fsync(directory)
        finally:
            os.close(directory)
        index["files"].append({"original": str(path), "original_bytes": size, "original_mode": mode, "original_sha256": row[key],
                              "archive": str(archive), "archive_bytes": archive.stat().st_size,
                              "archive_sha256": digest(archive), "verified_decoded_sha256": decoded.hexdigest(),
                              "source_report": str(snapshot), "source_report_sha256": digest(snapshot)})
        index["saved_bytes"] = sum(entry["original_bytes"] - entry["archive_bytes"] for entry in index["files"])
        pending_index = Path(str(manifest) + ".new")
        with pending_index.open("w") as output:
            output.write(json.dumps(index, indent=2) + "\n"); output.flush(); os.fsync(output.fileno())
        pending_index.replace(manifest)
        directory = os.open(work.parent, os.O_RDONLY | os.O_DIRECTORY)
        try:
            os.fsync(directory)
        finally:
            os.close(directory)
        path.unlink()
        directory = os.open(work, os.O_RDONLY | os.O_DIRECTORY)
        try:
            os.fsync(directory)
        finally:
            os.close(directory)


def build_fixture(work: Path, clang: str, compiler: Path | None = None) -> dict:
    """Compile the actual pure-Cinch Text.Multipart SIMD match kernel and deliberately scalar independent C."""
    cinch = compiler or (ROOT / "seed/target/release/cinch")
    probe = work / "_fixture_probe"
    probe.mkdir(parents=True, exist_ok=True)
    for src_path in work.glob("*.cinch"):
        shutil.copyfile(src_path, probe / src_path.name)
    mp_path = probe / "Text.Multipart.cinch"
    mp_src = mp_path.read_text()
    if "equal-sixteen-mask ::" not in mp_src:
        raise RuntimeError("missing actual selected pure-Cinch SIMD kernel")
    mp_src = re.sub(r"^(\s*export\s+)", r"\1equal-sixteen-mask, ", mp_src, count=1, flags=re.M)
    mp_path.write_text(mp_src)
    (probe / "Main.cinch").write_text(
        "-- | Root pure-Cinch Text.Multipart SIMD kernel for foreign-object oracle verification.\n"
        "module Main where\n"
        "  import Data.Bytes\n"
        "  import Data.Machine\n"
        "  import Data.Wide\n"
        "  import Text.Multipart\n"
        "  -- | Non-inlinable wrapper around Text.Multipart.equal-sixteen-mask.\n"
        "  probe-equal :: U128 -> U8 -> I64 -> U32\n"
        "  probe-equal v m k = if lt-i64 k 0 then probe-equal v m (wrapping-add-i64 k 1) else equal-sixteen-mask v m\n"
        "  -- | Keep pure-Cinch equal-sixteen-mask reachable in emitted LLVM IR.\n"
        "  main :: Int\n"
        "  main =\n"
        "    let b = bytes-new 16 in\n"
        "    int-from-i64 (u32-to-i64 (probe-equal (bytes-load-u128-at b 0) (bytes-get-u8-at b 0) 0))\n"
    )
    raw_ir = probe / "probe.ll"
    run([str(cinch), "emit-ir", str(probe / "Main.cinch"), "-o", str(raw_ir)], probe, env={**os.environ, "RUST_MIN_STACK": "67108864", "CINCH_STACK_SIZE": "64M"})
    ir = raw_ir.read_text()
    ir = re.sub(r"^define (?!internal )", "define internal ", ir, flags=re.M)
    ir = re.sub(r"^@llvm\.global_ctors = .*\n?", "", ir, flags=re.M)
    ir = re.sub(r'^(@(?:"[^"]+"|[A-Za-z0-9_.$#-]+) = )(?!private |internal |external |appending |unnamed_addr )', r"\1internal ", ir, flags=re.M)
    ir = ir.replace("define internal i32 @main(", "define internal i32 @__mp_probe_main(")
    trampoline = (
        '\ndefine i32 @multipart_equal16(i64 %0, i64 %1, i8 %2) {\n'
        '  %lo = zext i64 %0 to i128\n'
        '  %hi = zext i64 %1 to i128\n'
        '  %sh = shl i128 %hi, 64\n'
        '  %v = or i128 %lo, %sh\n'
        '  %r = call i32 @"Main.probe-equal"(i128 %v, i8 %2, i64 0)\n'
        '  ret i32 %r\n'
        '}\n'
    )
    kernel = work / "kernels.ll"
    kernel.write_text(ir + trampoline)
    fixture = work / "fixture.c"
    shutil.copyfile(ROOT / "seed/tests/fixtures/multipart-fixture.c", fixture)
    run([clang, "-O2", "-c", str(kernel), "-o", str(work / "kernels.o")], work)
    run([clang, "-O2", "-fno-vectorize", "-fno-slp-vectorize", "-c", str(fixture), "-o", str(work / "fixture.o")], work)
    return {"kernel_ir_sha256": digest(kernel), "kernel_object_sha256": digest(work / "kernels.o"),
            "fixture_source_sha256": digest(fixture), "fixture_object_sha256": digest(work / "fixture.o"),
            "scalar_oracle_vectorization": False, "kernel_cases_per_invocation": 1048576}


def archive_cache(work: Path, selector: str, proof: dict) -> None:
    """Archive closed generated caches, reusing exact verified members before durable unlink."""
    cache = work / 'build'
    files = sorted(path for path in cache.rglob('*') if path.is_file())
    if not files:
        return
    known = {}
    for index in work.glob('*.index.json'):
        data = json.loads(index.read_text())
        if 'archive' not in data or 'members' not in data:
            continue
        archive = Path(data['archive'])
        if digest(archive) != data['compressed_sha256']:
            raise RuntimeError('changed checked-cache archive')
        decoded = gzip.decompress(archive.read_bytes())
        if hashlib.sha256(decoded).hexdigest() != data['decoded_tar_sha256']:
            raise RuntimeError('changed decoded checked-cache archive')
        with tarfile.open(fileobj=io.BytesIO(decoded), mode='r') as stored:
            for row in data['members']:
                member = stored.getmember(row['relative'])
                content = stored.extractfile(member).read()
                if hashlib.sha256(content).hexdigest() != row['sha256'] or len(content) != row['bytes'] or member.mode != row['mode']:
                    raise RuntimeError('changed exact checked-cache member')
                known[(row['sha256'], row['bytes'], row['mode'])] = {'archive': str(archive), 'index': str(index), 'member': row['relative']}
    rows = [{'path': str(path), 'relative': str(path.relative_to(cache)), 'sha256': digest(path),
             'bytes': path.stat().st_size, 'mode': path.stat().st_mode & 0o777} for path in files]
    fresh = [row for row in rows if (row['sha256'], row['bytes'], row['mode']) not in known]
    label = selector.replace(':', '-')
    if fresh:
        payload = io.BytesIO()
        with tarfile.open(fileobj=payload, mode='w') as stored:
            for row in fresh:
                stored.add(row['path'], arcname=row['relative'], recursive=False)
        original = payload.getvalue()
        archive = work / (label + '-cache.tar.gz')
        with archive.open('wb') as raw:
            with gzip.GzipFile(filename='', fileobj=raw, mode='wb', compresslevel=1, mtime=0) as output:
                output.write(original)
            raw.flush(); os.fsync(raw.fileno())
        decoded = gzip.decompress(archive.read_bytes())
        if decoded != original:
            raise RuntimeError('checked-cache roundtrip differs')
        with tarfile.open(fileobj=io.BytesIO(decoded), mode='r') as stored:
            for row in fresh:
                member = stored.getmember(row['relative'])
                content = stored.extractfile(member).read()
                if hashlib.sha256(content).hexdigest() != row['sha256'] or len(content) != row['bytes'] or member.mode != row['mode']:
                    raise RuntimeError('new checked-cache member roundtrip differs')
        index = work / (label + '-cache.index.json')
        with index.open('w') as output:
            json.dump({'archive': str(archive), 'compressed_sha256': digest(archive),
                       'decoded_tar_sha256': hashlib.sha256(decoded).hexdigest(), 'decoded_tar_bytes': len(decoded), 'members': fresh}, output, indent=2)
            output.write('\n'); output.flush(); os.fsync(output.fileno())
        for row in fresh:
            known[(row['sha256'], row['bytes'], row['mode'])] = {'archive': str(archive), 'index': str(index), 'member': row['relative']}
    for row in rows:
        row['recovery'] = known[(row['sha256'], row['bytes'], row['mode'])]
    recovery = work / (label + '-cache-recovery.json')
    with recovery.open('w') as output:
        json.dump({'scope': 'Closed generated check-cache only; actual input sources remain plain.', 'proof': proof, 'files': rows}, output, indent=2)
        output.write('\n'); output.flush(); os.fsync(output.fileno())
    directory = os.open(work, os.O_RDONLY | os.O_DIRECTORY)
    try:
        os.fsync(directory)
        for path in files:
            path.unlink()
        for folder in sorted((path for path in cache.rglob('*') if path.is_dir()), key=lambda path: len(path.parts), reverse=True):
            folder.rmdir()
        cache.rmdir()
        os.fsync(directory)
    finally:
        os.close(directory)
