#!/usr/bin/env python3
"""Build explicit compiler stages without letting a later stage call the seed."""
from __future__ import annotations

import argparse
import fcntl
import hashlib
import io
import json
import os
import platform
from pathlib import Path, PurePosixPath
import re
import shutil
import subprocess
import sys
import tarfile

ROOT = Path(__file__).resolve().parents[1]
BUILD = ROOT / "build" / "bootstrap"
TOOLKIT = ("Lex", "Scan", "Parse", "Json", "Builder")
PROTOCOL = 3
HOST = {"system": platform.system(), "machine": platform.machine()}
FROZEN_SOURCES = None


class BootstrapError(RuntimeError):
    pass


def digest(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def file_hash(path: Path) -> str:
    return digest(path.read_bytes())


def source_hash(files: dict[str, bytes]) -> str:
    h = hashlib.sha256()
    for name, data in sorted(files.items()):
        h.update(name.encode() + b"\0" + str(len(data)).encode() + b"\0" + data)
    return h.hexdigest()


def seed_sources(directory: Path) -> dict[str, bytes]:
    files = {}
    for part in ("compiler/src", "compiler/prelude", "compiler/runtime/src"):
        for path in (directory / part).rglob("*"):
            if path.is_file():
                files[str(path.relative_to(directory))] = path.read_bytes()
    for part in ("compiler/Cargo.toml", "compiler/Cargo.lock", "compiler/.cargo/config.toml",
                 "compiler/runtime/Cargo.toml", "compiler/runtime/Cargo.lock"):
        files[part] = (directory / part).read_bytes()
    return files


def extract_sources(archive: bytes, directory: Path) -> None:
    # Python 3.9 is the macOS system Python. Validate tar entries before
    # extraction instead of depending on 3.12's extraction filters.
    with tarfile.open(fileobj=io.BytesIO(archive)) as tar:
        members = tar.getmembers()
        for member in members:
            path = PurePosixPath(member.name)
            if path.is_absolute() or ".." in path.parts or not (member.isfile() or member.isdir()):
                raise BootstrapError(f"unsafe seed archive entry: {member.name}")
        tar.extractall(directory, members=members)


def environment() -> dict[str, str]:
    env = os.environ.copy()
    # Root-launched tools do not read compiler/.cargo/config.toml. Keep
    # stage builds and any test subprocesses serial here as well.
    env["CARGO_BUILD_JOBS"] = "1"
    env["RUST_TEST_THREADS"] = "1"
    for profile in ("DEV", "TEST", "RELEASE"):
        env[f"CARGO_PROFILE_{profile}_CODEGEN_UNITS"] = "1"
    value = env.get("CINCH_HEAP_LIMIT", "2G")
    match = re.fullmatch(r"([0-9]+)([KkMmGg]?)", value)
    if not match:
        raise BootstrapError("CINCH_HEAP_LIMIT must be a positive size at most 2G")
    amount = int(match[1]) * {"": 1, "K": 1 << 10, "M": 1 << 20, "G": 1 << 30}[match[2].upper()]
    if not 0 < amount <= 2 << 30:
        raise BootstrapError("CINCH_HEAP_LIMIT must be positive and at most 2G")
    env["CINCH_HEAP_LIMIT"] = value
    return env


def read_json(path: Path) -> dict:
    try:
        return json.loads(path.read_text())
    except (FileNotFoundError, json.JSONDecodeError):
        return {}


def write_json(path: Path, value: dict) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    temp = path.with_suffix(".tmp")
    temp.write_text(json.dumps(value, indent=2, sort_keys=True) + "\n")
    temp.replace(path)


def run(command: list[str | Path], cwd: Path, log: Path, *, append: bool = False) -> int:
    log.parent.mkdir(parents=True, exist_ok=True)
    with log.open("ab" if append else "wb") as stream:
        return subprocess.run([str(x) for x in command], cwd=cwd, env=environment(),
                              stdout=stream, stderr=subprocess.STDOUT).returncode


def required_artifacts(inputs: dict) -> tuple[str, ...]:
    if inputs.get("built_by_stage", 0) > 0:
        return ("compiler.ll", "link-report.txt", "base-interfaces.json")
    if "source_sha256" in inputs:
        return ("compiler.cinch", "sources/build/Main.manifest.json")
    return ()


def cached(directory: Path, inputs: dict) -> bool:
    record = read_json(directory / "stage.json")
    exe = directory / "cinch"
    return (record.get("status") == "built" and record.get("inputs") == inputs
            and exe.is_file() and os.access(exe, os.X_OK)
            and record.get("executable_sha256") == file_hash(exe)
            and all(name in record.get("artifacts", {}) for name in required_artifacts(inputs))
            and all((directory / name).is_file() and file_hash(directory / name) == expected
                    for name, expected in record.get("artifacts", {}).items()))


def record_build(directory: Path, inputs: dict, code: int, log: Path) -> Path:
    exe = directory / "cinch"
    required = required_artifacts(inputs)
    good = (code == 0 and exe.is_file() and os.access(exe, os.X_OK)
            and all((directory / name).is_file() for name in required))
    if good and inputs.get("built_by_stage", 0) > 0:
        # A compiler invoked without arguments returns its
        # bad-arguments code (1). A successful clang link alone is not
        # evidence that the produced compiler entry point runs.
        good = (directory / "link-report.txt").read_text() == "clang-exit: 0\nartifact-exit: 1\n"
    record = {"status": "built" if good else "failed", "inputs": inputs,
              "exit_code": code, "log": str(log)}
    if good:
        record["executable_sha256"] = file_hash(exe)
        record["artifacts"] = {name: file_hash(directory / name) for name in required}
        record["artifacts"].update({str(p.relative_to(directory)): file_hash(p)
                                    for p in directory.rglob("*.cinchif")})
    write_json(directory / "stage.json", record)
    if not good:
        diagnostics = directory / "compiler.ll.diagnostics"
        has_diagnostics = diagnostics.is_file() and bool(diagnostics.read_text(errors="replace").strip())
        detail = diagnostics if has_diagnostics else log
        lines = detail.read_text(errors="replace").splitlines()
        preview = "\n".join(lines[:6] if has_diagnostics else lines[-6:])
        raise BootstrapError(f"{directory.name} did not build (exit {code}).\n{preview}\nEvidence: {detail}")
    print(f"{directory.name}: built {exe}", flush=True)
    return exe


def seed_inputs() -> tuple[dict, bytes]:
    lock = read_json(ROOT / "bootstrap" / "stage0.lock.json")
    if lock.get("schema") != 1:
        raise BootstrapError("unsupported or missing bootstrap/stage0.lock.json")
    patch = (ROOT / lock["patch"]).read_bytes()
    if digest(patch) != lock["patch_sha256"]:
        raise BootstrapError("the stage-0 patch differs from its lock; update the seed intentionally")
    return lock, patch


def build_seed() -> Path:
    lock, patch = seed_inputs()
    inputs = {"protocol": PROTOCOL, "host": HOST, "seed": lock}
    directory = BUILD / "stage0"
    snapshot_id = digest(json.dumps(lock, sort_keys=True).encode())
    snapshot = BUILD / "seed-sources" / snapshot_id
    if not snapshot.exists():
        archive = subprocess.run(["git", "archive", lock["revision"], "compiler"], cwd=ROOT,
                                 check=True, stdout=subprocess.PIPE).stdout
        temporary = snapshot.with_name(snapshot.name + ".preparing")
        if temporary.exists():
            shutil.rmtree(temporary)
        temporary.mkdir(parents=True)
        extract_sources(archive, temporary)
        patched = subprocess.run(["patch", "--batch", "-p1"], input=patch,
                                 cwd=temporary, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
        if patched.returncode:
            raise BootstrapError("could not apply the pinned seed patch: " + patched.stdout.decode(errors="replace"))
        temporary.rename(snapshot)
    if source_hash(seed_sources(snapshot)) != lock["source_sha256"]:
        raise BootstrapError("the seed source snapshot differs from the pinned source hash")
    if cached(directory, inputs):
        print("stage0: verified cached seed", flush=True)
        return directory / "cinch"
    directory.mkdir(parents=True, exist_ok=True)
    target = BUILD / "cargo" / snapshot_id
    log = directory / "build.log"
    code = run(["cargo", "build", "--locked", "--release", "--bin", "cinch",
                "--target-dir", target], snapshot / "compiler", log)
    product = target / "release" / "cinch"
    if code == 0 and product.is_file():
        shutil.copy2(product, directory / "cinch")
    else:
        (directory / "cinch").unlink(missing_ok=True)
    return record_build(directory, inputs, code, log)


def collect_compiler_sources() -> dict[str, bytes]:
    files = {p.name: p.read_bytes() for p in sorted((ROOT / "bootstrap").glob("*.cinch"))}
    for module in TOOLKIT:
        files[f"Text.{module}.cinch"] = (ROOT / "compiler" / "prelude" / "Text" / f"{module}.cinch").read_bytes()
    files["Main.cinch"] = (ROOT / "bootstrap" / "programs" / "Run.Pipeline.cinch").read_bytes()
    return files


def compiler_sources() -> dict[str, bytes]:
    # One command uses one source snapshot even if the developer edits
    # the checkout while the stages are being compiled.
    return collect_compiler_sources() if FROZEN_SOURCES is None else FROZEN_SOURCES


def bundle(files: dict[str, bytes]) -> bytes:
    return b"\n\n".join(b"-- source: " + name.encode() + b"\n" + data
                           for name, data in sorted(files.items())) + b"\n"


def prepare_outputs(directory: Path) -> None:
    directory.mkdir(parents=True, exist_ok=True)
    for name in ("cinch", "compiler.ll", "compiler.ll.diagnostics", "link-report.txt"):
        (directory / name).unlink(missing_ok=True)
    for interface in directory.rglob("*.cinchif"):
        interface.unlink()


def build_stage(stage: int) -> Path:
    if stage == 0:
        return build_seed()
    builder = build_stage(stage - 1)
    files = compiler_sources()
    if stage >= 2:
        source = BUILD / "stage1" / "compiler.cinch"
        if source.read_bytes() != bundle(files):
            raise BootstrapError("compiler sources changed while preparing the stage chain")
        return build_self(builder, files, BUILD / f"stage{stage}", stage - 1, source)
    inputs = {"protocol": PROTOCOL, "host": HOST, "source_sha256": source_hash(files),
              "builder_sha256": file_hash(builder), "built_by_stage": stage - 1}
    directory = BUILD / f"stage{stage}"
    if cached(directory, inputs):
        print(f"stage{stage}: verified cached compiler", flush=True)
        return directory / "cinch"
    prepare_outputs(directory)
    log = directory / "build.log"
    sources = directory / "sources"
    if sources.exists():
        shutil.rmtree(sources)
    sources.mkdir()
    for name, data in files.items():
        (sources / name).write_bytes(data)
    (directory / "compiler.cinch").write_bytes(bundle(files))
    code = run([builder, "build", sources / "Main.cinch", "-o", directory / "cinch"], sources, log)
    return record_build(directory, inputs, code, log)


def build_self(builder: Path, files: dict[str, bytes], directory: Path,
               built_by_stage: int, source: Path | None = None) -> Path:
    base = base_interface_manifest()
    base_bytes = json.dumps(base, sort_keys=True, separators=(",", ":")).encode()
    runtime = runtime_archive()
    inputs = {"protocol": PROTOCOL, "host": HOST, "source_sha256": source_hash(files),
              "builder_sha256": file_hash(builder), "built_by_stage": built_by_stage,
              "interface_set_sha256": digest(base_bytes), "frontend_mode": "typed-ir",
              "runtime_archive_sha256": file_hash(runtime)}
    if cached(directory, inputs):
        print(f"{directory.name}: verified cached compiler", flush=True)
        return directory / "cinch"
    prepare_outputs(directory)
    manifest = directory / "base-interfaces.json"
    manifest.write_bytes(base_bytes)
    if source is None:
        source = directory / "compiler.cinch"
        source.write_bytes(bundle(files))
    log = directory / "build.log"
    # Only the preceding Cinch compiler reads the source. Its retained-Core
    # backend emits ABI5 LLVM; the platform linker consumes that IR and the
    # exact runtime built from the pinned seed. No Native frontend fallback.
    ir, exe = directory / "compiler.ll", directory / "cinch"
    code = run([builder, "--emit-ir", "--interfaces", manifest, source, ir], directory, log)
    if code == 0:
        command = [os.environ.get("CINCH_CLANG", "clang"), ir, runtime, "-o", exe]
        if HOST["system"] == "Linux":
            command.extend(["-ldl", "-lpthread", "-lm"])
        code = run(command, directory, log, append=True)
        report = f"clang-exit: {code}\n"
        if code == 0:
            if exe.is_file() and os.access(exe, os.X_OK):
                startup = run([exe], directory, log, append=True)
                report += f"artifact-exit: {startup}\n"
                code = 0 if startup == 1 else startup or 1
            else:
                with log.open("ab") as stream:
                    stream.write(b"linker succeeded without a runnable compiler\n")
                code = 1
        (directory / "link-report.txt").write_text(report)
    return record_build(directory, inputs, code, log)


def runtime_archive() -> Path:
    lock, _ = seed_inputs()
    snapshot = BUILD / "seed-sources" / digest(json.dumps(lock, sort_keys=True).encode())
    if source_hash(seed_sources(snapshot)) != lock["source_sha256"]:
        raise BootstrapError("the pinned runtime source snapshot changed before linking")
    for profile in ("release", "debug"):
        archive = snapshot / "compiler/runtime/target" / profile / "libcinchruntime.a"
        if archive.is_file():
            return archive
    raise BootstrapError("the verified stage-1 build has no matching pinned runtime archive")


def base_interface_manifest() -> dict:
    stage1 = BUILD / "stage1"
    record = read_json(stage1 / "stage.json")
    if not cached(stage1, record.get("inputs", {})):
        raise BootstrapError("base contracts require a verified stage-1 build")
    lock, _ = seed_inputs()
    snapshot_id = digest(json.dumps(lock, sort_keys=True).encode())
    package = read_json(BUILD / "seed-sources" / snapshot_id / "compiler/prelude/cinch-base.cinchpkg")
    allowed = {m["name"] for m in package["modules"]} | {"Prelude.Generated"}
    produced = read_json(stage1 / "sources/build/Main.manifest.json")
    rows = []
    for entry in produced["modules"]:
        name = entry["module"]
        if name not in allowed:
            continue
        path = stage1 / "sources/build" / f"{name}.cinchif"
        doc = read_json(path)
        if any(dep["module"] not in allowed for dep in doc["imports"]):
            raise BootstrapError(f"base contract {name} depends on compiler sources")
        rows.append([name, entry["iface_hash"], str(path.resolve())])
    if not rows:
        raise BootstrapError("the stage-1 build produced no checked base contracts")
    return {"schema": 1, "interfaces": sorted(rows)}


def smoke(stage: int) -> None:
    compiler = build_stage(stage)
    directory = BUILD / "smoke" / f"stage{stage}"
    directory.mkdir(parents=True, exist_ok=True)
    cases = [("fib", (ROOT / "examples" / "compiler" / "fib.cinch").read_text(), 0),
             ("precedence", "module Main where\n  main :: Int\n  main = 2 + 3 * 4\n", 14),
             ("refusal", "module Main where\n  main :: Int\n  main = unknown-name\n", None)]
    for name, source_text, expected in cases:
        case = directory / name
        case.mkdir(exist_ok=True)
        source = case / "Main.cinch"
        source.write_text(source_text)
        exe = case / "program"
        for output in (exe, case / "program.ll", case / "program.ll.diagnostics", case / "report.txt"):
            output.unlink(missing_ok=True)
        if stage == 0:
            command = [compiler, "build", source, "-o", exe]
        else:
            command = [compiler, source, case / "program.ll", exe, case / "report.txt"]
        code = run(command, case, case / "compile.log")
        if expected is None:
            diagnostics = case / ("compile.log" if stage == 0 else "program.ll.diagnostics")
            expected_code = 1 if stage == 0 else 4
            if (code != expected_code or exe.exists() or not diagnostics.is_file()
                    or "unknown-name" not in diagnostics.read_text(errors="replace")):
                raise BootstrapError(f"stage{stage} did not give a named refusal for the invalid smoke program")
        else:
            if code or not exe.is_file():
                raise BootstrapError(f"stage{stage} failed the {name} smoke compile: {case / 'compile.log'}")
            answer = run([exe], case, case / "run.log")
            if answer != expected:
                raise BootstrapError(f"stage{stage} {name}: expected {expected}, got {answer}")
        print(f"stage{stage}: {name} smoke passed", flush=True)


def check_fixed_point() -> None:
    for stage in (0, 1, 2, 3):
        smoke(stage)
    second, third = BUILD / "stage2", BUILD / "stage3"
    if (second / "compiler.ll").read_bytes() != (third / "compiler.ll").read_bytes():
        raise BootstrapError("stage-2 and stage-3 LLVM IR differ")
    interfaces = {str(p.relative_to(second)): file_hash(p) for p in second.rglob("*.cinchif")}
    rebuilt = {str(p.relative_to(third)): file_hash(p) for p in third.rglob("*.cinchif")}
    if not interfaces or interfaces != rebuilt:
        raise BootstrapError("canonical stage-2/stage-3 interfaces are missing or differ")
    check_changed_source()
    print("bootstrap: stages 2 and 3 reached a fixed point", flush=True)


def check_changed_source() -> None:
    files = dict(compiler_sources())
    marker = b'  rung = "bootstrap.check"'
    changed_marker = b'  rung = "bootstrap.check.changed-source"'
    name = "Bootstrap.Check.cinch"
    if files[name].count(marker) != 1:
        raise BootstrapError("update the changed-source probe for Bootstrap.Check's diagnostic label")
    files[name] = files[name].replace(marker, changed_marker)
    directory = BUILD / "changed-source" / "stage2"
    compiler = build_self(BUILD / "stage1" / "cinch", files, directory, 1)
    case = directory / "probe"
    case.mkdir(exist_ok=True)
    evidence = case / "checked.json"
    evidence.unlink(missing_ok=True)
    source = case / "Main.cinch"
    source.write_text("module Main where\n  main :: Int\n  main = True\n")
    exe, ir, report = case / "program", case / "program.ll", case / "report.txt"
    diagnostics = case / "program.ll.diagnostics"
    for output in (exe, ir, report, diagnostics):
        output.unlink(missing_ok=True)
    code = run([compiler, source, ir, exe, report], case, case / "compile.log")
    if (code != 4 or exe.exists() or not diagnostics.is_file()
            or "bootstrap.check.changed-source" not in diagnostics.read_text(errors="replace")
            or "type mismatch" not in diagnostics.read_text(errors="replace")):
        raise BootstrapError("the compiler built from changed sources did not show the changed diagnostic behavior")
    write_json(evidence, {"source_sha256": source_hash(files),
                          "compiler_sha256": file_hash(compiler),
                          "diagnostics_sha256": file_hash(diagnostics)})
    print("bootstrap: changed compiler source rebuilt and changed executable behavior", flush=True)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("command", choices=("build", "smoke", "check", "status"))
    parser.add_argument("--stage", type=int, choices=range(4), default=1)
    args = parser.parse_args()
    try:
        environment()
        BUILD.mkdir(parents=True, exist_ok=True)
        # Serialize mutations of the stage cache; ordinary cargo test runs
        # use independent snapshots and do not hold this lock.
        if args.command != "status":
            stage_lock = (BUILD / ".lock").open("a")
            fcntl.flock(stage_lock, fcntl.LOCK_EX)
            global FROZEN_SOURCES
            FROZEN_SOURCES = collect_compiler_sources()
        if args.command == "build":
            build_stage(args.stage)
        elif args.command == "smoke":
            smoke(args.stage)
        elif args.command == "check":
            check_fixed_point()
        else:
            for stage in range(4):
                directory = BUILD / f"stage{stage}"
                record = read_json(directory / "stage.json")
                status = record.get("status", "not built")
                inputs = record.get("inputs", {})
                if status == "built" and not cached(directory, inputs):
                    status = "invalid artifacts"
                elif status == "built" and stage and inputs.get("source_sha256") != source_hash(compiler_sources()):
                    status = "stale sources"
                elif status == "built" and not stage and inputs.get("seed") != seed_inputs()[0]:
                    status = "stale seed"
                elif status == "built" and inputs.get("host") != HOST:
                    status = "different build host"
                print(f"stage{stage}: {status}")
        return 0
    except (BootstrapError, OSError, subprocess.CalledProcessError) as error:
        print(f"bootstrap: {error}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
