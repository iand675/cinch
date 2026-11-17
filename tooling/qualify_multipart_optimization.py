#!/usr/bin/env python3
"""Relink already qualified multipart IR at an explicit optimization level.

No compiler, SDK or fixture is substituted. Every IR, foreign object, provider
and host archive is checked against its original successful emission report.
Both runtime profiles remain separate, and completed ELF files are losslessly
archived before the next consumer replaces the bounded scratch executable.
"""
from __future__ import annotations
import argparse
import gzip
import hashlib
import json
import os
from pathlib import Path
import resource
import shutil
from check_compare import digest, run


def original_ir(path: Path, expected: str, scratch: Path) -> tuple[Path, bool]:
    """Recover exact checked bytes through the explicit durable archive index."""
    if path.is_file():
        if digest(path) != expected:
            raise RuntimeError("changed original checked IR")
        return path, False
    manifest = json.loads((path.parent.parent / "completed-client-archives.json").read_text())
    row = next(entry for entry in manifest["files"] if entry["original"] == str(path))
    archive = Path(row["archive"])
    if row["original_sha256"] != expected or digest(archive) != row["archive_sha256"]:
        raise RuntimeError("changed archived checked IR")
    with gzip.open(archive, "rb") as source, scratch.open("wb") as output:
        shutil.copyfileobj(source, output, 1024 * 1024)
    if digest(scratch) != expected or scratch.stat().st_size != row["original_bytes"]:
        raise RuntimeError("checked IR archive roundtrip mismatch")
    return scratch, True


def archive_executable(path: Path, target: Path) -> dict:
    """Verify a complete ELF roundtrip without unlinking its only indexed original."""
    expected, size, mode = digest(path), path.stat().st_size, path.stat().st_mode & 0o777
    with path.open("rb") as source, target.open("wb") as raw:
        with gzip.GzipFile(filename="", fileobj=raw, mode="wb", compresslevel=1, mtime=0) as output:
            shutil.copyfileobj(source, output, 1024 * 1024)
        raw.flush(); os.fsync(raw.fileno())
    decoded, decoded_size = hashlib.sha256(), 0
    with gzip.open(target, "rb") as source:
        for chunk in iter(lambda: source.read(1024 * 1024), b""):
            decoded.update(chunk); decoded_size += len(chunk)
    if decoded.hexdigest() != expected or decoded_size != size:
        raise RuntimeError("ELF archive roundtrip mismatch")
    return {"original_sha256": expected, "original_bytes": size, "original_mode": mode,
            "archive": str(target), "archive_sha256": digest(target), "archive_bytes": target.stat().st_size,
            "verified_decoded_sha256": decoded.hexdigest()}


def durable_report(path: Path, report: dict) -> None:
    """Commit the complete recovery report and directory before any ELF unlink."""
    pending = Path(str(path) + ".new")
    with pending.open("w") as output:
        output.write(json.dumps(report, indent=2) + "\n"); output.flush(); os.fsync(output.fileno())
    pending.replace(path)
    directory = os.open(path.parent, os.O_RDONLY | os.O_DIRECTORY)
    try:
        os.fsync(directory)
    finally:
        os.close(directory)


def main() -> None:
    """Consume only named successful epochs with bounded scratch storage and exact objects."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--report", type=Path, nargs="+", required=True)
    parser.add_argument("--work", type=Path, required=True)
    parser.add_argument("--opt", type=int, choices=(0, 2), required=True)
    parser.add_argument("--only", nargs="+", help="Explicit frontend:case selectors")
    parser.add_argument("--cpu", type=int, required=True)
    parser.add_argument("--start-free-mib", type=int, default=0)
    parser.add_argument("--pause-free-mib", type=int, default=0)
    parser.add_argument("--unarchived-cap-mib", type=int, default=0)
    args = parser.parse_args()
    os.sched_setaffinity(0, {args.cpu})
    resource.setrlimit(resource.RLIMIT_STACK, (67108864, resource.getrlimit(resource.RLIMIT_STACK)[1]))
    work = args.work.resolve(); work.mkdir(parents=True, exist_ok=True)
    report_path = work / "report.json"
    result = json.loads(report_path.read_text()) if report_path.exists() else {"scope": __doc__, "opt": args.opt, "runs": []}
    if result["opt"] != args.opt:
        raise RuntimeError("changed optimization epoch")
    policy = {"start_free_mib": args.start_free_mib, "pause_free_mib": args.pause_free_mib,
              "unarchived_cap_mib": args.unarchived_cap_mib}
    if result.get("resource_policy", policy) != policy:
        raise RuntimeError("changed explicit resource policy")
    result["resource_policy"] = policy
    result["compiler_host_stack_bytes"] = 67108864
    result["cpu_affinity"] = [args.cpu]

    def guard(phase: str, minimum: int) -> None:
        """Keep the coordinated reserve and plain scratch bound at every process boundary."""
        free = shutil.disk_usage(work).free
        plain = sum(path.stat().st_size for path in work.rglob("*") if path.is_file() and not path.name.endswith(".gz"))
        result.setdefault("resource_boundaries", []).append({"phase": phase, "free_bytes": free, "minimum_bytes": minimum, "plain_bytes": plain})
        durable_report(report_path, result)
        if free < minimum or (args.unarchived_cap_mib and plain > args.unarchived_cap_mib * 1024 * 1024):
            raise RuntimeError("resource pause before " + phase + "; no further process started")

    guard("start", args.start_free_mib * 1024 * 1024)
    clang = os.environ.get("CINCH_CLANG", "clang")
    selected: set[str] = set()
    for source_report in args.report:
        source_report = source_report.resolve()
        original = json.loads(source_report.read_text()); folder = source_report.parent
        for original_row in original["runs"]:
            label, case = original_row["frontend"], original_row["case"]
            selector = label + ":" + case
            if args.only and selector not in args.only:
                continue
            selected.add(selector)
            completed = next((row for row in result["runs"] if row["selector"] == selector and row["source_report"] == str(source_report)), None)
            if completed is not None:
                if len(completed["profiles"]) != 2 or "executable" not in completed or "failure" in completed:
                    raise RuntimeError("previous incomplete optimization run remains preserved; use a fresh work epoch")
                if digest(source_report) != completed["source_report_sha256"] or digest(Path(completed["executable"]["archive"])) != completed["executable"]["archive_sha256"]:
                    raise RuntimeError("changed completed optimization evidence")
                continue
            if len(original_row["profiles"]) != 2 or any("leaked=0" not in p["stderr"] for p in original_row["profiles"]):
                raise RuntimeError("cannot relink an incomplete original consumer")
            identity = original["frontends"][label]; runtime = Path(identity["runtime"])
            if digest(Path(identity["compiler"])) != identity["compiler_sha256"]:
                raise RuntimeError("changed exact source compiler")
            if digest(runtime) != identity["runtime_sha256"]:
                raise RuntimeError("changed exact host runtime")
            for name, expected in original["sources"].items():
                if digest(folder / (name + ".cinch")) != expected:
                    raise RuntimeError("changed source in the qualified original closure")
            if digest(folder / (case + ".source")) != original_row["source_sha256"]:
                raise RuntimeError("changed original case source")
            for filename, field in (("fixture.o", "fixture_object_sha256"), ("kernels.o", "kernel_object_sha256")):
                if digest(folder / filename) != original[field]:
                    raise RuntimeError("changed actual foreign object")
            guard("restore-" + selector, args.pause_free_mib * 1024 * 1024)
            ir, temporary = original_ir(folder / f"{label}-{case}.ll", original_row["ir_sha256"], work / "checked-input.ll")
            executable = work / "consumer"
            command = [clang, "-O" + str(args.opt), "--target=" + original_row["target"], str(ir), str(folder / "fixture.o"), str(folder / "kernels.o")]
            if label == "native":
                snapshot = Path(identity["compiler"]).parent
                for filename in ("objects.json", "base-interfaces.json"):
                    if digest(snapshot / filename) != identity[filename]:
                        raise RuntimeError("changed admitted Native manifest")
                provider = Path(json.loads((snapshot / "objects.json").read_text())["object"])
                if digest(provider) != identity["provider_sha256"]:
                    raise RuntimeError("changed admitted Native provider")
                command.append(str(provider))
            command += [str(runtime), "-ldl", "-lpthread", "-lm", "-o", str(executable)]
            row = {"selector": selector, "source_report": str(source_report), "source_report_sha256": digest(source_report),
                   "checked_ir_sha256": original_row["ir_sha256"], "source_sha256": original_row["source_sha256"],
                   "frontend_identity": identity, "command": command, "profiles": [], "count": original_row.get("count"),
                   "source_closure_sha256": original["sources"], "oracle_sha256": original["vectors_sha256"],
                   "qualification_kind": "Exact accepted LLVM relink; original compiler emission is preserved, not repeated."}
            result["runs"].append(row)
            try:
                guard("link-" + selector, args.pause_free_mib * 1024 * 1024)
                linked = run(command, work, 180)
                row["link_stderr"] = linked.stderr
                for poison, heap in (("0", "32M"), ("1", "256M")):
                    guard("profile-" + selector + "-" + poison, args.pause_free_mib * 1024 * 1024)
                    env = {**os.environ, "CINCH_RC_DEBUG": poison, "CINCH_LEAK_CHECK": "1", "CINCH_HEAP_LIMIT": heap, "CINCH_STACK_SIZE": "8M"}
                    observed = run([str(executable)], work, 60, env)
                    if "leaked=0" not in observed.stderr:
                        raise RuntimeError("missing zero-leak observation")
                    row["profiles"].append({"poison": poison, "heap": heap, "stack": "8M", "stdout": observed.stdout, "stderr": observed.stderr})
                # Distinct qualified source epochs can contain the same case name.
                epoch = digest(source_report)[:16]
                guard("archive-" + selector, args.pause_free_mib * 1024 * 1024)
                row["executable"] = archive_executable(executable, work / f"{label}-{case}-{epoch}-O{args.opt}.gz")
                durable_report(report_path, result)
                executable.unlink()
                if temporary:
                    ir.unlink()
                directory = os.open(work, os.O_RDONLY | os.O_DIRECTORY)
                try:
                    os.fsync(directory)
                finally:
                    os.close(directory)
                print(selector, "O" + str(args.opt), flush=True)
            except Exception as error:
                row["failure"] = str(error); durable_report(report_path, result); raise
    if args.only and set(args.only) != selected:
        raise RuntimeError("unknown qualified selector(s): " + ", ".join(sorted(set(args.only) - selected)))


if __name__ == "__main__":
    main()
