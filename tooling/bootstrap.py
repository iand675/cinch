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
PROTOCOL = 4
HOST = {"system": platform.system(), "machine": platform.machine()}
FROZEN_SOURCES = None
RUNTIME_ABIS = (5, 6, 7)


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


NATIVE_BUILD_SCRIPT = "compiler/build.rs"
NATIVE_GRAMMAR = "tooling/tree-sitter-cinch"
NATIVE_GRAMMAR_INPUTS = ("grammar.js", "tree-sitter.json", "src/scanner.c")


def seed_sources(directory: Path) -> dict[str, bytes]:
    files = {}
    for part in ("compiler/src", "compiler/prelude", "compiler/runtime/src"):
        for path in (directory / part).rglob("*"):
            if path.is_file():
                files[str(path.relative_to(directory))] = path.read_bytes()
    for part in ("compiler/Cargo.toml", "compiler/Cargo.lock", "compiler/.cargo/config.toml",
                 "compiler/runtime/Cargo.toml", "compiler/runtime/Cargo.lock"):
        files[part] = (directory / part).read_bytes()
    if (directory / NATIVE_BUILD_SCRIPT).is_file():
        files[NATIVE_BUILD_SCRIPT] = (directory / NATIVE_BUILD_SCRIPT).read_bytes()
        for part in NATIVE_GRAMMAR_INPUTS:
            path = f"{NATIVE_GRAMMAR}/{part}"
            files[path] = (directory / path).read_bytes()
    return files


def native_source_archive(revision: str) -> bytes:
    paths = ["compiler"]
    build_script = subprocess.run(["git", "ls-tree", "--name-only", revision, "--", NATIVE_BUILD_SCRIPT],
                                  cwd=ROOT, check=True, stdout=subprocess.PIPE).stdout
    if build_script:
        paths.extend(f"{NATIVE_GRAMMAR}/{part}" for part in NATIVE_GRAMMAR_INPUTS)
    return subprocess.run(["git", "archive", revision, *paths], cwd=ROOT, check=True,
                          stdout=subprocess.PIPE).stdout


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
        return (("compiler.ll", "link-report.txt", "base-interfaces.json", "objects.json")
                if inputs.get("frontend_mode") == "typed-ir-objects" else
                ("compiler.ll", "link-report.txt", "base-interfaces.json"))
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


def library_provider_lock() -> dict:
    lock = read_json(ROOT / "bootstrap/library-provider.lock.json")
    if not isinstance(lock, dict):
        raise BootstrapError("unsupported or missing library producer lock")
    revision, source = lock.get("revision"), lock.get("source_sha256")
    if (type(lock.get("schema")) is not int or lock["schema"] != 1
            or type(lock.get("runtime_abi")) is not int or lock["runtime_abi"] not in RUNTIME_ABIS
            or not isinstance(revision, str) or not re.fullmatch(r"[0-9a-f]{40}", revision)
            or not isinstance(source, str) or not re.fullmatch(r"[0-9a-f]{64}", source)):
        raise BootstrapError("unsupported or missing library producer lock")
    source_abi = ROOT / "bootstrap/Bootstrap.RuntimeAbi.cinch"
    if not source_abi.is_file() or source_abi.read_bytes() != runtime_abi_source(lock["runtime_abi"]):
        raise BootstrapError("library producer runtime ABI differs from the compiler's target ABI")
    return lock


def build_seed() -> Path:
    lock, patch = seed_inputs()
    inputs = {"protocol": PROTOCOL, "host": HOST, "seed": lock}
    directory = BUILD / "stage0"
    snapshot_id = digest(json.dumps(lock, sort_keys=True).encode())
    snapshot = BUILD / "seed-sources" / snapshot_id
    if not snapshot.exists():
        archive = native_source_archive(lock["revision"])
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


def build_seed_runtime() -> Path:
    """Link Stage1 with an optimized runtime from the exact seed archive."""
    lock, _ = seed_inputs()
    snapshot = BUILD / "seed-sources" / digest(json.dumps(lock, sort_keys=True).encode())
    if source_hash(seed_sources(snapshot)) != lock["source_sha256"]:
        raise BootstrapError("the seed runtime source differs from the pinned seed")
    runtime = snapshot / "compiler/runtime"
    archive = runtime / "target/release/libcinchruntime.a"
    log = BUILD / "stage0/runtime-build.log"
    code = run(["cargo", "build", "--locked", "--release", "--manifest-path",
                runtime / "Cargo.toml", "--target-dir", runtime / "target"],
               snapshot / "compiler", log)
    if code or not archive.is_file():
        raise BootstrapError(f"the optimized seed runtime did not build: {log}")
    # The pinned Native linker prefers a fresh release archive. Never let a
    # failed/stale optimized build quietly select its debug fallback.
    sources = [runtime / "Cargo.toml", *[p for p in (runtime / "src").rglob("*") if p.is_file()]]
    if any(p.stat().st_mtime_ns > archive.stat().st_mtime_ns for p in sources):
        raise BootstrapError("the optimized seed runtime is older than its source")
    return archive


def runtime_abi_source(abi: int) -> bytes:
    # This is build configuration, not permission to translate artifacts.
    # Interfaces and the linked archive must come from the same producer.
    if type(abi) is not int or abi not in RUNTIME_ABIS:
        raise BootstrapError(f"unsupported compiler runtime ABI: {abi!r}")
    return ("-- One exact ABI per compiler build. The pinned stage builder renders this\n"
            "-- module from the target library lock before hashing the complete sources.\n"
            "module Bootstrap.RuntimeAbi where\n"
            "  export runtime-abi, runtime-abi-int\n"
            "  runtime-abi :: Nat\n"
            f"  runtime-abi = {abi}\n"
            "  runtime-abi-int :: Int\n"
            f"  runtime-abi-int = {abi}\n").encode()


def collect_compiler_sources() -> dict[str, bytes]:
    # The seed links Stage1. The compiler inside Stage1 targets the pinned
    # library and runtime used to build and run Stage2 and later.
    lock = library_provider_lock()
    files = {p.name: p.read_bytes() for p in sorted((ROOT / "bootstrap").glob("*.cinch"))}
    files["Bootstrap.RuntimeAbi.cinch"] = runtime_abi_source(lock["runtime_abi"])
    for module in TOOLKIT:
        files[f"Text.{module}.cinch"] = (ROOT / "compiler" / "prelude" / "Text" / f"{module}.cinch").read_bytes()
    files["Main.cinch"] = (ROOT / "bootstrap" / "programs" / "Run.Pipeline.cinch").read_bytes()
    # Every bundled compiler module participates in the source compilation.
    # Make the seed-built stage check that same complete closure, including
    # modules that the CLI does not call, and publish their real interfaces.
    # Qualified aliases keep their source exports out of the CLI's scope.
    header = b"module Main where\n"
    if files["Main.cinch"].count(header) != 1:
        raise BootstrapError("the compiler entry must have one Main module header")
    modules = sorted(name[:-6] for name in files if name.endswith(".cinch") and name != "Main.cinch")
    imports = "".join(f"  import {name} as CompilerSource{i}\n"
                      for i, name in enumerate(modules)).encode()
    files["Main.cinch"] = files["Main.cinch"].replace(header, header + imports, 1)
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
    for name in ("cinch", "compiler.ll", "compiler.ll.diagnostics", "link-report.txt", "requested-roots.json"):
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
    runtime = build_seed_runtime()
    inputs = {"protocol": PROTOCOL, "host": HOST, "source_sha256": source_hash(files),
              "builder_sha256": file_hash(builder), "built_by_stage": stage - 1,
              "seed_runtime_profile": "release", "seed_runtime_sha256": file_hash(runtime)}
    directory = BUILD / f"stage{stage}"
    compiler = directory / "cinch"
    # The seed writes this sidecar for clang. The fixed point compares the
    # later stages' compiler.ll, so a successful Stage1 can discard it.
    seed_ir = compiler.with_suffix(".ll")
    if cached(directory, inputs):
        seed_ir.unlink(missing_ok=True)
        print(f"stage{stage}: verified cached compiler", flush=True)
        return compiler
    prepare_outputs(directory)
    log = directory / "build.log"
    sources = directory / "sources"
    if sources.exists():
        shutil.rmtree(sources)
    sources.mkdir()
    for name, data in files.items():
        (sources / name).write_bytes(data)
    (directory / "compiler.cinch").write_bytes(bundle(files))
    code = run([builder, "build", sources / "Main.cinch", "-o", compiler], sources, log)
    record_build(directory, inputs, code, log)
    seed_ir.unlink(missing_ok=True)
    return compiler


def build_self(builder: Path, files: dict[str, bytes], directory: Path,
               built_by_stage: int, source: Path | None = None) -> Path:
    provider = runtime_provider()
    base = provider["interfaces"]
    base_bytes = json.dumps(base, sort_keys=True, separators=(",", ":")).encode()
    configuration = json.dumps(provider["configuration"], sort_keys=True, separators=(",", ":")).encode()
    runtime = provider["runtime"]
    inputs = {"protocol": PROTOCOL, "host": HOST, "source_sha256": source_hash(files),
              "builder_sha256": file_hash(builder), "built_by_stage": built_by_stage,
              "interface_set_sha256": digest(base_bytes), "frontend_mode": "typed-ir-objects",
              "interface_output": "checked-schema10",
              "library_provider": provider["identity"], "object_configuration_sha256": digest(configuration),
              "runtime_archive_sha256": file_hash(runtime)}
    if cached(directory, inputs):
        print(f"{directory.name}: verified cached compiler", flush=True)
        return directory / "cinch"
    prepare_outputs(directory)
    manifest = directory / "base-interfaces.json"
    manifest.write_bytes(base_bytes)
    objects = directory / "objects.json"
    objects.write_bytes(configuration)
    if source is None:
        source = directory / "compiler.cinch"
        source.write_bytes(bundle(files))
    log = directory / "build.log"
    # Only the preceding Cinch compiler reads the source. Its retained-Core
    # backend emits LLVM; the platform linker consumes that IR, the exact
    # library object and its matching pinned runtime. Native builds only the
    # generated library root and its base closure, never Bootstrap sources.
    ir, exe = directory / "compiler.ll", directory / "cinch"
    code = run([builder, "--emit-ir-with-interfaces", "--interfaces", manifest, "--objects", objects, source, ir], directory, log)
    if code:
        from bootstrap_objects import requested_roots
        diagnostics = directory / "compiler.ll.diagnostics"
        if diagnostics.is_file():
            write_json(directory / "requested-roots.json", requested_roots(diagnostics.read_text(errors="replace")))
    if code == 0:
        # Optimized: the stage compilers are the bootstrap's hot loop, and
        # identical IR under identical flags still yields identical binaries.
        command = [os.environ.get("CINCH_CLANG", "clang"), "-O2", "-Wno-override-module", ir, provider["object"], runtime, "-o", exe]
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


def runtime_provider() -> dict:
    from bootstrap_objects import provider
    return provider(sys.modules[__name__])


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
    if file_hash(second / "cinch") != file_hash(third / "cinch"):
        raise BootstrapError("stage-2 and stage-3 compiler executables differ")
    if (second / "compiler.ll").read_bytes() != (third / "compiler.ll").read_bytes():
        raise BootstrapError("stage-2 and stage-3 LLVM IR differ")
    interfaces = {str(p.relative_to(second)): file_hash(p) for p in second.rglob("*.cinchif")}
    rebuilt = {str(p.relative_to(third)): file_hash(p) for p in third.rglob("*.cinchif")}
    require_compiler_interfaces(second, compiler_sources())
    require_compiler_interfaces(third, compiler_sources())
    if not interfaces or interfaces != rebuilt:
        raise BootstrapError("canonical stage-2/stage-3 interfaces are missing or differ")
    check_interface_consumer(third, compiler_sources())
    check_changed_source()
    print("bootstrap: stages 2 and 3 reached a fixed point", flush=True)


def require_compiler_interfaces(directory: Path, files: dict[str, bytes]) -> None:
    from bootstrap_interfaces import interface_hash, read_contract, validate_contract, verify_dependencies

    expected = {name[:-6] for name in files if name.endswith(".cinch")}
    documents = {}
    for path in directory.rglob("*.cinchif"):
        header = read_json(path)
        if not isinstance(header, dict) or not isinstance(header.get("module"), str):
            raise BootstrapError(f"invalid actual compiler interface header: {path}")
        name = header["module"]
        if name in expected:
            if name in documents:
                raise BootstrapError(f"duplicate canonical compiler interface for {name}: {directory}")
            documents[name] = path
    if set(documents) != expected:
        raise BootstrapError(f"{directory.name} is missing actual compiler interfaces: {', '.join(sorted(expected - set(documents)))}")
    contracts = {}
    runtime_abi = library_provider_lock()["runtime_abi"]
    for name, path in documents.items():
        try:
            doc = read_contract(path)
            validate_contract(doc, runtime_abi)
            if not any(doc["exports"][key] for key in ("values", "types", "signatures", "instances")):
                raise ValueError("compiler module has an empty checked export surface")
            contracts[name] = doc
        except (OSError, KeyError, TypeError, ValueError) as error:
            raise BootstrapError(f"invalid actual compiler interface {name}: {error}") from error
    try:
        base = {}
        for name, identity, filename in read_json(directory / "base-interfaces.json").get("interfaces", []):
            doc = read_contract(Path(filename))
            validate_contract(doc, runtime_abi)
            if doc["module"] != name or interface_hash(doc) != identity or name in base:
                raise ValueError("base interface identity disagrees with its manifest")
            base[name] = doc
        verify_dependencies(contracts, base)
    except (OSError, KeyError, TypeError, ValueError) as error:
        raise BootstrapError(f"invalid compiler interface dependency chain: {error}") from error


def check_interface_consumer(directory: Path, files: dict[str, bytes]) -> None:
    """Check the actual entry against emitted contracts, with no module bodies.

    The self-built compiler must reconstruct the imported APIs from its own
    artifacts. A structurally valid but unusable contract cannot pass merely
    because the next writer repeats the same bytes.
    """
    from bootstrap_interfaces import interface_hash, read_contract

    require_compiler_interfaces(directory, files)
    case = directory / "interface-consumer"
    case.mkdir(exist_ok=True)
    source = case / "Main.cinch"
    source.write_bytes(files["Main.cinch"])
    rows = {name: [name, identity, filename]
            for name, identity, filename in read_json(directory / "base-interfaces.json").get("interfaces", [])}
    expected = {name[:-6] for name in files if name.endswith(".cinch")}
    for path in directory.rglob("*.cinchif"):
        doc = read_contract(path)
        if doc["module"] in expected:
            rows[doc["module"]] = [doc["module"], interface_hash(doc), str(path.resolve())]
    # Main is the source client in this check, not an imported module.
    rows.pop("Main", None)
    manifest = case / "interfaces.json"
    write_json(manifest, {"schema": 1, "interfaces": [rows[name] for name in sorted(rows)]})
    output = case / "entry"
    diagnostics = case / "entry.diagnostics"
    diagnostics.unlink(missing_ok=True)
    code = run([directory / "cinch", "--check", "--interfaces", manifest, source, output],
               case, case / "check.log")
    if code != 0 or not diagnostics.is_file() or diagnostics.read_bytes():
        raise BootstrapError(f"self-built compiler cannot consume its actual compiler interfaces: {diagnostics}")
    write_json(case / "checked.json", {"status": "passed", "compiler_sha256": file_hash(directory / "cinch"),
                                      "manifest_sha256": file_hash(manifest), "source_sha256": file_hash(source)})
    print("bootstrap: self-built compiler checks its entry using emitted compiler interfaces", flush=True)


def check_changed_source() -> None:
    files = dict(compiler_sources())
    marker = b'  rung = "bootstrap.check"'
    changed_marker = b'  rung = "bootstrap.check.changed-source"'
    name = "Bootstrap.Check.cinch"
    if files[name].count(marker) != 1:
        raise BootstrapError("update the changed-source probe for Bootstrap.Check's diagnostic label")
    files[name] = files[name].replace(marker, changed_marker)
    directory = BUILD / "changed-source" / "stage2"
    compiler = build_self(BUILD / "stage2" / "cinch", files, directory, 2)
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
                elif status == "built" and inputs.get("protocol") != PROTOCOL:
                    status = "stale stage protocol"
                elif status == "built" and stage and inputs.get("source_sha256") != source_hash(compiler_sources()):
                    status = "stale sources"
                elif status == "built" and not stage and inputs.get("seed") != seed_inputs()[0]:
                    status = "stale seed"
                elif status == "built" and stage >= 2 and (
                        inputs.get("library_provider", {}).get("producer") != read_json(ROOT / "bootstrap/library-provider.lock.json")
                        or inputs.get("library_provider", {}).get("roots_sha256") != file_hash(ROOT / "bootstrap/library-object-roots.json")):
                    status = "stale library inputs"
                elif status == "built" and inputs.get("host") != HOST:
                    status = "different build host"
                print(f"stage{stage}: {status}")
        return 0
    except (BootstrapError, OSError, subprocess.CalledProcessError) as error:
        print(f"bootstrap: {error}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
