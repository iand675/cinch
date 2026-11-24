#!/usr/bin/env python3
"""Execute independent multipart fixtures through explicitly selected public CLIs.

Preparation and execution require no SDK. Actual emitted vector bodies and
independent scalar C are linked as real objects. Frozen/current compiler and
runtime identities, exact sources, failures and separate memory profiles survive
incremental execution rather than being silently replaced.
"""
from __future__ import annotations
import argparse
import gzip
import hashlib
import json
import os
from pathlib import Path
import re
import resource
import shutil
import time
from bench_stdlib import source_closure
from check_compare import corpus_emit_mode, digest, initialize_runtime, list_of, run
from check_quoted_printable import byte_literal, literal

#: Repository root for immutable source and fixture inputs.
ROOT = Path(__file__).resolve().parents[1]
#: Exact generic callback identities used by both independent trace models.
ERRORS = {"": 0, "header failure": 1, "body failure": 2, "first error": 3, "last error": 4, "footer failure": 5}


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


def fingerprint(data: bytes, value: int = 14695981039346656037) -> int:
    """Hash exact bytes with a wrapping independent scalar algorithm."""
    for byte in data:
        value = ((value ^ byte) * 1099511628211) & ((1 << 64) - 1)
    return value


def trace_fingerprint(events: list[dict]) -> int:
    """Serialize all offers, accepted counts and generic error identities."""
    value = 14695981039346656037
    for event in events:
        offer = bytes.fromhex(event["offer"])
        data = len(offer).to_bytes(8, "little") + offer
        data += event["count"].to_bytes(8, "little") + ERRORS[event["error"]].to_bytes(8, "little")
        value = fingerprint(data, value)
    return value


def writer_expression(row: dict) -> str:
    """Retain every public operation, output byte and callback status."""
    size = max(len(row["limits"] or []), max((int(key) + 1 for key in row["failures"]), default=0))
    script = []
    for at in range(size):
        limit = row["limits"][at] if row["limits"] is not None and at < len(row["limits"]) else -1
        script.append(f"(({limit}), {ERRORS[row['failures'].get(str(at), '')]})")
    actions = list_of([f"({literal(event['operation'])}, {event.get('count', 0)}, {literal(event['error'])})" for event in row["actions"]])
    return f"writer-is {list_of(script)} {actions} {len(row['trace'])} {trace_fingerprint(row['trace'])} {fingerprint(bytes.fromhex(row['output']))} {len(bytes.fromhex(row['output']))} {byte_literal(row['boundary'])} {byte_literal(row['content_type'])}"


def source_fingerprint(events: list[dict]) -> int:
    """Record exact buffered read-ahead, prefix bytes and sticky source statuses."""
    value = 14695981039346656037
    for event in events:
        output = bytes.fromhex(event["output"])
        code = 1 if event["error"] == "EOF" else 2 if event["error"] else 0
        payload = event["request"].to_bytes(8, "little") + len(output).to_bytes(8, "little")
        value = fingerprint(payload + output + code.to_bytes(8, "little"), value)
    return value


def header_expression(rows: list[dict]) -> str:
    """Preserve sorted original keys and every ordered raw value."""
    return list_of([f"({byte_literal(row['key'])}, {list_of([byte_literal(value) for value in row['values']])})" for row in rows])


def expected_parts(row: dict) -> str:
    """Preserve every independently observed part, read prefix and simultaneous status."""
    parts = []
    for part in row["parts"]:
        assert "".join(read["output"] for read in part["reads"]) == part["body"]
        reads = list_of([f"(ExpectedRead {read['request']} {byte_literal(read['output'])} {literal(read['error'])})" for read in part["reads"]])
        parts.append(f"(ExpectedPart {header_expression(part['headers'])} {byte_literal(part['name'])} {byte_literal(part['filename'])} {reads} {literal(part['close_error'])})")
    return list_of(parts)


def reader_expression(row: dict) -> str:
    """Check each public part/read and its complete actual source callback trace."""
    terminal = 2 if row["terminal"] == "failure" else 3 if row["terminal"] == "late-eof" else 1
    return f"reader-is {byte_literal(row['input'])} {byte_literal(row['boundary'])} {row['raw']} {row['chunk']} {terminal} {expected_parts(row)} {literal(row['error'])} {source_fingerprint(row['source_trace'])} {len(row['source_trace'])}"


def expected_read(row: dict | None) -> str:
    """Keep absent post-open observations distinct from empty successful reads."""
    if row is None:
        return "None"
    return f"(Some (ExpectedRead {row['request']} {byte_literal(row['output'])} {literal(row['error'])}))"


def expected_file(row: dict) -> str:
    """Preserve every metadata, signed seek, positioned read and close observation."""
    reads = []
    for read in row.get("reads", []):
        value = f"(ExpectedRead {read['request']} {byte_literal(read['output'])} {literal(read['error'])})"
        reads.append(f"(ExpectedPositioned ({read['offset']}) {value})" if "offset" in read else f"(ExpectedSequential {value})")
    seeks = [f"(ExpectedSeek ({seek['offset']}) ({seek['whence']}) ({seek['result']}) {literal(seek['error'])})" for seek in row.get("seeks", [])]
    return (f"(ExpectedFile {byte_literal(row['name'])} {byte_literal(row['filename'])} ({row['size']}) "
            f"{header_expression(row['headers'])} {literal(row['open_error'])} {list_of(reads)} {list_of(seeks)} "
            f"{literal(row.get('close_error', ''))} {expected_read(row.get('after_close'))} "
            f"{literal(row.get('close_again_error', ''))} {literal(row.get('edited_size_open_error', ''))} "
            f"{expected_read(row.get('edited_size_read'))})")


def form_expression(row: dict, wire: str) -> str:
    """Map the independent optional form/error outcome without hiding deferred failures."""
    fields: dict[str, list[str]] = {}
    for file in row.get("files", []):
        fields.setdefault(file["name"], []).append(expected_file(file))
    files = list_of([f"({byte_literal(name)}, {list_of(rows)})" for name, rows in fields.items()])
    values = list_of([f"({byte_literal(value['name'])}, {list_of([byte_literal(item) for item in value['values']])})" for value in row.get("values", [])])
    policy = dict(item.split("=", 1) for item in row["policy"].split(",") if item)
    return (f"form-is {byte_literal(wire)} ({row['memory']}) {'True' if row['distinct'] else 'False'} "
            f"{int(policy.get('multipartmaxparts', 1000))} {int(policy.get('multipartmaxheaders', 10000))} "
            f"{row['interrupted_close_count']} {'True' if row['form_present'] else 'False'} {literal(row['error'])} "
            f"{values} {files} {literal(row.get('remove_error', ''))} {literal(row.get('remove_again_error', ''))} "
            f"{expected_read(row.get('retained_after_remove'))} {list_of([literal(message) for message in row.get('reopened', [])])}")


def cache_expression(row: dict) -> str:
    """Retain the independent lazy-cache and stale-part observations verbatim."""
    assert row["next_error"] == ""
    return (f"cache-is {byte_literal(row['input'])} {byte_literal(row['name'])} {byte_literal(row['filename'])} "
            f"{byte_literal(row['late_name'])} {header_expression(row['headers'])} {byte_literal(row['old_body'])} "
            f"{literal(row['old_error'])} {byte_literal(row['next_body'])} {literal(row['next_body_error'])}")


def generated_cases(data: dict, width: int, form_width: int = 1) -> dict[str, tuple[str, int]]:
    """Keep each consumer bounded while checking complete committed families."""
    effect_prefix = (ROOT / "tooling/data/multipart-consumer.cinchpart").read_text()
    pure_prefix = (ROOT / "tooling/data/multipart-pure-consumer.cinchpart").read_text()
    families = {
        "boundaries": [f"boundary-is {byte_literal(row['input'])} {literal(row['error'])} {byte_literal(row['content_type'])}" for row in data["boundaries"]],
        "dispositions": [f"disposition-is {byte_literal(row['name'])} {byte_literal(row['filename'])} {byte_literal(row['output'])}" for row in data["dispositions"]],
        "writers": [writer_expression(row) for row in data["writers"]],
        "readers": [reader_expression(row) for row in data["readers"]],
        "forms": [form_expression(row, data["form_input"]) for row in data["forms"]],
    }
    if "input" in data["cache"]:
        families["cache"] = [cache_expression(data["cache"])]
    late_headers = [reader_expression(row) for row in data["readers"] if row["terminal"] == "late-eof"]
    if late_headers:
        families["late-headers"] = late_headers
    form_reader = next((row for row in data["readers"] if bytes.fromhex(row["boundary"]) == b"form-boundary"), None)
    if "entropy" in data:
        families["entropy"] = [f"entropy-is {byte_literal(row['input'])} {'True' if row['terminal'] == 'failure' else 'False'} {row['empty']} {byte_literal(row['boundary'])} {literal(row['error'])} {source_fingerprint(row['source_trace'])} {len(row['source_trace'])}" for row in data["entropy"]]
        families["ownership"] = ["form-value-ownership-is ()", "file-memory-ownership-is ()",
                                 f"writer-retention-is {list_of([byte_literal(value) for value in data['writer_retention']])}"]
        if form_reader is not None:
            families["ownership"].append(f"retained-reader-is {byte_literal(form_reader['input'])} {byte_literal(form_reader['boundary'])} {expected_parts(form_reader)} {literal(form_reader['error'])} {source_fingerprint(form_reader['source_trace'])} {len(form_reader['source_trace'])}")
    cases = {}
    for family, expressions in families.items():
        # Every real filesystem case closes its namespace before the next explicit observation.
        batch = form_width if family == "forms" else width
        for offset in range(0, len(expressions), batch):
            rows = expressions[offset:offset + batch]
            effect_case = family in ("writers", "readers", "forms", "cache", "entropy", "ownership", "late-headers")
            source = effect_prefix if effect_case else pure_prefix
            if family in ("forms", "entropy", "ownership"):
                source += (ROOT / "tooling/data/multipart-storage.cinchpart").read_text()
            if family in ("entropy", "ownership"):
                source += (ROOT / "tooling/data/multipart-ownership.cinchpart").read_text()
            region = "(r :: Region)" if effect_case else "r"
            for index, expression in enumerate(rows):
                name = f"fixture-{chr(97 + index // 26)}{chr(97 + index % 26)}"
                source += f"\n  -- | Check independent {family} observation {offset + index}.\n  {name} :: forall {region}. using (io :: Io r) => Unit ->{{IO r, Mut r}} Bool\n  {name} unit = {expression}\n"
            verdict = "0"
            for index in reversed(range(len(rows))):
                name = f"fixture-{chr(97 + index // 26)}{chr(97 + index % 26)}"
                verdict = f"if {name} () then {verdict} else {index + 1}"
            source += "\n  -- | Check the actual foreign SIMD/scalar object and all public observations.\n  main :: Unit ->{IO r, Mut r} Int\n  main unit = with (io = newIO ()) in if eq-i32 (raw-kernels 0) 0 then " + verdict + " else 250\n"
            cases[f"{family}-{offset:04d}"] = source, len(rows)
    # This maintained control isolates map mutation from wire parsing and storage effects.
    source = effect_prefix + (ROOT / "tooling/data/multipart-storage.cinchpart").read_text()
    source += "\n  -- | Check repeated form insertion through the full public mutation API.\n  main :: Unit ->{IO r, Mut r} Int\n  main unit = with (io = newIO ()) in if form-map-is () then 0 else 1\n"
    cases["form-map"] = source, 1
    if form_reader is not None:
        source = effect_prefix + "\n  -- | Check the independently observed multipart form wire through the ordinary reader.\n  main :: Unit ->{IO r, Mut r} Int\n  main unit = with (io = newIO ()) in if " + reader_expression(form_reader) + " then 0 else 1\n"
        cases["form-reader"] = source, 1
    return cases


def prepare(work: Path, vectors: Path, width: int, effect_source: Path | None = None, text_source: Path | None = None, form_width: int = 1, qp_source: Path | None = None) -> tuple[dict, dict]:
    """Freeze the exact reachable source closure in dependency order."""
    cases = generated_cases(json.loads(vectors.read_text()), width, form_width)
    per_case = {}
    for name, (source, count) in cases.items():
        overrides = {"Effect.Multipart": effect_source} if effect_source is not None and re.search(r"^\s*import Effect\.Multipart\s*$", source, re.M) else {}
        if text_source is not None:
            overrides["Text.Multipart"] = text_source
        closure = source_closure(source, overrides)
        if qp_source is not None and 'Text.QuotedPrintable' in closure:
            overrides['Text.QuotedPrintable'] = qp_source
            closure = source_closure(source, overrides)
        ordered, seen = [], set()

        def visit(module: str) -> None:
            """Order only this consumer's actual source dependency closure."""
            if module in seen:
                return
            seen.add(module)
            for dep in re.findall(r"^\s*import\s+([A-Z][\w.]*)", closure[module].read_text(), re.M):
                if dep in closure:
                    visit(dep)
            ordered.append(module)

        for module, path in closure.items():
            visit(module)
            shutil.copyfile(path, work / (module + ".cinch"))
        per_case[name] = ordered
        (work / (name + ".source")).write_text(source)
    return {name: count for name, (source, count) in cases.items()}, per_case


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


def main() -> None:
    """Run one selected frontend at a time with exact source-epoch resume checks."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--work", type=Path, required=True)
    parser.add_argument("--vectors", type=Path, default=ROOT / "seed/tests/fixtures/multipart-vectors.json")
    parser.add_argument("--group-size", type=int, choices=(32, 64, 128), default=64)
    parser.add_argument("--form-group-size", type=int, choices=(1, 8, 32), default=1, help="Explicitly group independently reset real-filesystem observations")
    parser.add_argument("--prepare-only", action="store_true")
    parser.add_argument("--resume", action="store_true")
    parser.add_argument("--archive-completed", action="store_true", help="Losslessly archive fully passed functional ELF/LLVM after verified hashes and durable recovery indexing")
    parser.add_argument("--rust", type=Path)
    parser.add_argument("--rust-runtime", type=Path)
    parser.add_argument("--initialize-rust-runtime", action="store_true")
    parser.add_argument("--native", type=Path)
    parser.add_argument("--only", nargs="+")
    parser.add_argument("--cpu", type=int)
    parser.add_argument("--opt", type=int, choices=(0, 2), default=2, help="Explicit consumer optimization; use distinct work directories for each epoch")
    parser.add_argument("--effect-source", type=Path, help="Explicit immutable Effect.Multipart source candidate; its path and exact hash are recorded")
    parser.add_argument("--text-source", type=Path, help="Explicit immutable Text.Multipart source candidate; its path and exact hash are recorded")
    parser.add_argument("--qp-source", type=Path, help="Explicit reachable shared codec candidate; all source hashes remain recorded")
    parser.add_argument("--start-free-mib", type=int, default=0, help="Explicit coordinated initial disk guard; zero preserves historical invocation policy")
    parser.add_argument("--pause-free-mib", type=int, default=0, help="Explicit disk reserve checked at every seed/link/profile boundary")
    parser.add_argument("--unarchived-cap-mib", type=int, default=0, help="Bound newly generated plain outputs in this work directory")
    args = parser.parse_args()
    if args.rust and not args.rust_runtime:
        parser.error("--rust requires its exact --rust-runtime")
    if args.initialize_rust_runtime and (not args.rust or args.native or args.resume):
        parser.error("runtime initialization requires a fresh explicit Rust-only invocation")
    if not args.prepare_only and not args.rust and not args.native:
        parser.error("select at least one explicit seed/runtime pair")
    work = args.work.resolve(); work.mkdir(parents=True, exist_ok=True)
    if shutil.disk_usage(work).free < args.start_free_mib * 1024 * 1024:
        raise RuntimeError("resource pause before preparation; no compiler or runtime started")
    if not args.resume:
        shutil.copyfile(Path(__file__), work / "runner.py")
    counts, per_case_modules = prepare(work, args.vectors, args.group_size, args.effect_source, args.text_source, args.form_group_size, args.qp_source)
    if args.only and any(name not in counts for name in args.only):
        parser.error("unknown fixture group; prepared groups: " + ", ".join(counts))
    modules = sorted({module for closure in per_case_modules.values() for module in closure})
    if args.prepare_only:
        print(json.dumps(counts)); return
    if args.cpu is not None:
        os.sched_setaffinity(0, {args.cpu})
    resource.setrlimit(resource.RLIMIT_STACK, (67108864, resource.getrlimit(resource.RLIMIT_STACK)[1]))
    clang = os.environ.get("CINCH_CLANG", "clang")
    initial_plain_bytes = sum(path.stat().st_size for path in work.rglob("*") if path.is_file() and not path.name.endswith(".gz"))
    report = {"format": 1, "scope": "prepared families only; full package qualification pending",
              "initial_plain_bytes": initial_plain_bytes,
              "runner_sha256": digest(Path(__file__)), "vectors_sha256": digest(args.vectors),
              "compiler_host_stack_bytes": 67108864,
              "consumer_optimization": args.opt,
              "compiler_environment": {"RUST_MIN_STACK": "67108864", "CINCH_STACK_SIZE": "64M"},
              "source_selection": {module: {"path": str(path.resolve()), "sha256": digest(path)} for module, path in (("Effect.Multipart", args.effect_source), ("Text.Multipart", args.text_source), ("Text.QuotedPrintable", args.qp_source)) if path is not None},
              "sources": {name: digest(work / (name + ".cinch")) for name in modules},
              "consumers": {name: digest(work / (name + ".source")) for name in counts},
              "counts": counts, "case_modules": per_case_modules, "frontends": {}, "runs": [], **build_fixture(work, clang)}
    if args.resume:
        previous = json.loads((work / "report.json").read_text())
        for key in ("runner_sha256", "vectors_sha256", "consumer_optimization", "sources", "consumers", "kernel_ir_sha256", "kernel_object_sha256", "fixture_source_sha256", "fixture_object_sha256"):
            if previous[key] != report[key]:
                raise RuntimeError("changed source/object cannot resume: " + key)
        report = previous

    def guard(phase: str) -> None:
        """Record the actual reserve and plain-output growth before the next process."""
        free = shutil.disk_usage(work).free
        growth = sum(path.stat().st_size for path in work.rglob("*") if path.is_file() and not path.name.endswith(".gz")) - report.get("initial_plain_bytes", initial_plain_bytes)
        report.setdefault("resource_boundaries", []).append({"phase": phase, "free_bytes": free,
            "pause_free_mib": args.pause_free_mib, "new_unarchived_bytes": growth,
            "unarchived_cap_mib": args.unarchived_cap_mib})
        (work / "report.json").write_text(json.dumps(report, indent=2) + "\n")
        if free < args.pause_free_mib * 1024 * 1024 or (args.unarchived_cap_mib and growth > args.unarchived_cap_mib * 1024 * 1024):
            raise RuntimeError("resource pause before " + phase + "; no further process started")

    def execute(command: list, env: dict | None = None):
        """Preserve failed exact commands before returning the diagnostic boundary."""
        try:
            return run(command, work, 300, env)
        except Exception as error:
            report.setdefault("failures", []).append({"command": list(map(str, command)), "diagnostic": str(error)})
            (work / "report.json").write_text(json.dumps(report, indent=2) + "\n")
            raise

    frontends = []
    if args.rust:
        runtime = args.rust_runtime.resolve()
        if args.initialize_rust_runtime:
            runtime, report["runtime_initialization"] = initialize_runtime(args.rust.resolve(), runtime, work)
        frontends.append(("rust", args.rust.resolve(), runtime))
    if args.native:
        snap = args.native.resolve()
        frontends.append(("native", snap / "cinch", snap / "libcinchruntime.a"))
    for label, compiler, runtime in frontends:
        identity = {"compiler": str(compiler), "compiler_sha256": digest(compiler), "runtime": str(runtime),
                    "runtime_sha256": digest(runtime), "compile_rc_debug": label == "native"}
        if label == "native":
            mode = corpus_emit_mode(compiler, report["runs"])
            config = json.loads((snap / "base-objects.json").read_text()); provider = Path(config["object"])
            identity.update({name: digest(snap / name) for name in ("base-objects.json", "base.cpk")})
            identity["provider_sha256"] = digest(provider)
        if label in report["frontends"] and report["frontends"][label] != identity:
            raise RuntimeError("changed frontend cannot resume: " + label)
        report["frontends"][label] = identity
        for name in args.only or counts:
            completed = next((row for row in report["runs"] if row["frontend"] == label and row["case"] == name and len(row["profiles"]) == 2), None)
            if completed is not None:
                if artifact_digest(work / f"{label}-{name}.ll") != completed["ir_sha256"] or artifact_digest(work / f"{label}-{name}") != completed["executable_sha256"]:
                    raise RuntimeError("changed completed artifact: " + name)
                continue
            shutil.copyfile(work / (name + ".source"), work / "Main.cinch")
            (work / "bundle.source").write_text("\n".join((work / (module + ".cinch")).read_text() for module in per_case_modules[name]) + "\n" + (work / "Main.cinch").read_text())
            ir, exe = work / f"{label}-{name}.ll", work / f"{label}-{name}"
            row = next((entry for entry in report["runs"] if entry["frontend"] == label and entry["case"] == name), None)
            if row is None:
                guard("emit-" + label + "-" + name)
                command = [str(compiler), "emit-ir", str(work / "Main.cinch"), "-o", str(ir)] if label == "rust" else [str(compiler), mode, "--rc-debug", "--interfaces", str(snap / "base.cpk"), "--objects", str(snap / "base-objects.json"), str(work / "bundle.source"), str(ir)]
                started = time.monotonic()
                result = execute(command, {**os.environ, "RUST_MIN_STACK": "67108864", "CINCH_STACK_SIZE": "64M"})
                target = re.search(r'target triple = "([^"]+)"', ir.read_text())
                if target is None and label != "native":
                    raise RuntimeError("Rust emission omitted its actual target triple")
                triple = target.group(1) if target else config["target"]
                row = {"frontend": label, "case": name, "count": counts[name], "command": command,
                       "compile_seconds": time.monotonic() - started, "stdout": result.stdout, "stderr": result.stderr,
                       "source_sha256": digest(work / "Main.cinch"), "ir_sha256": digest(ir), "target": triple, "profiles": []}
                report["runs"].append(row)
            elif digest(ir) != row["ir_sha256"] or digest(work / "Main.cinch") != row["source_sha256"]:
                raise RuntimeError("changed partially completed source or checked LLVM")
            if "executable_sha256" not in row:
                link = [clang, "-O" + str(args.opt), "--target=" + row["target"], str(ir), str(work / "fixture.o"), str(work / "kernels.o")]
                if label == "native":
                    link.append(str(provider))
                guard("link-" + label + "-" + name)
                execute(link + [str(runtime), "-ldl", "-lpthread", "-lm", "-o", str(exe)])
                row["executable_sha256"] = digest(exe)
            elif digest(exe) != row["executable_sha256"]:
                raise RuntimeError("changed partially completed executable")
            for poison, heap in (("0", "32M"), ("1", "256M")):
                if any(profile["poison"] == poison for profile in row["profiles"]):
                    continue
                guard("profile-" + label + "-" + name + "-" + poison)
                env = {**os.environ, "CINCH_RC_DEBUG": poison, "CINCH_LEAK_CHECK": "1",
                       "CINCH_HEAP_LIMIT": heap, "CINCH_STACK_SIZE": "8M"}
                result = execute([str(exe)], env)
                if "leaked=0" not in result.stderr:
                    raise RuntimeError("missing zero-leak observation")
                row["profiles"].append({"poison": poison, "heap": heap, "stack": "8M", "stdout": result.stdout, "stderr": result.stderr})
            (work / "report.json").write_text(json.dumps(report, indent=2) + "\n")
            if args.archive_completed:
                archive_completed(work, label, name, report)
            print(label, name, counts[name], flush=True)
    (work / "report.json").write_text(json.dumps(report, indent=2) + "\n")


if __name__ == "__main__":
    main()
