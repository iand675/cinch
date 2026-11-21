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
import shlex
import shutil
import subprocess
import sys
import tarfile
import time

# Profile-guided optimization: the stage link's flags, workload and cache.
# The module is local, so importing it costs one read of a file that needs
# nothing beyond the standard library.
import pgo

ROOT = Path(__file__).resolve().parents[1]
# CINCH_BOOTSTRAP_BUILD lets a second chain (another pin or source set) build
# beside an existing one without overwriting its stages.
BUILD = Path(os.environ["CINCH_BOOTSTRAP_BUILD"]).resolve() if os.environ.get("CINCH_BOOTSTRAP_BUILD") else ROOT / "build" / "bootstrap"
LIBRARY_ARTIFACT = None
TOOLKIT = ("Lex", "Scan", "Parse", "Json", "Builder")
# Prelude source modules the compiler itself compiles, keyed by the name the
# bundle gives them. `Data.Array` is not in the embedded base package, and
# `parMap` lowers through its two conversions (`Bootstrap.Infer`'s
# `par-map-array-call`), so a `parMap` anywhere in the compiler's own source
# resolves only when the module is in the same source set. One entry, because
# nothing else the compiler reaches for is outside the base package.
COMPILER_PRELUDE = ("Data.Array", "Data.Bytes.Word", "Text.Fnv")
PROTOCOL = 4
HOST = {"system": platform.system(), "machine": platform.machine()}
FROZEN_SOURCES = None
RUNTIME_ABIS = (5, 6, 7, 8, 9, 10)
# Parallel clang jobs for the stage link. 1 keeps the single-module
# translation unit; above 1 the module is sharded (see tooling/shard_ir.py).
JOBS = 1
# How many shards the stage module is cut into, and how. By default the count
# is JOBS and the partition is the balanced one. A development build
# (`--dev`) separates the two: many shards placed by a hash of their module's
# name, so an edit changes the text of the shards it touches and no others,
# and tooling/shard_ir.py's object cache (CINCH_SHARD_CACHE) recompiles only
# those. Both are part of a stage's cache identity, because the linked
# binary depends on where the module was cut.
SHARDS = 1
STABLE_SHARDS = False
DEV_SHARDS = 48
# Profile-guided optimization of the stage links (see tooling/pgo.py). The
# shipped default is on. `main` resolves --pgo/--no-pgo or
# CINCH_BOOTSTRAP_PGO into PGO, so a caller that never resolved it builds the
# plain -O2 link the tests assert.
PGO_DEFAULT = True
PGO = False
# Whether Stage 1, which the seed links, gets a profile too. Off unless asked
# for: a profile costs a second compile of the seed's module, which is six
# times the size of a stage's own, and what it buys is a faster build of
# Stage 2 alone. That trade was made when Stage 1's frontend ran for nine
# minutes. At a minute and a half the second link costs four minutes to save
# about ten seconds. Stages 2 and 3 are unaffected: Stage 1 emits the same
# module with or without a profile.
PGO_SEED = False


class BootstrapError(RuntimeError):
    pass


def digest(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def file_hash(path: Path) -> str:
    return pgo.file_digest(path)


def same_file_bytes(left: Path, right: Path) -> bool:
    with left.open("rb") as first, right.open("rb") as second:
        while True:
            chunk = first.read(1024 * 1024)
            if chunk != second.read(1024 * 1024):
                return False
            if not chunk:
                return True


def source_hash(files: dict[str, bytes]) -> str:
    h = hashlib.sha256()
    for name, data in sorted(files.items()):
        h.update(name.encode() + b"\0" + str(len(data)).encode() + b"\0" + data)
    return h.hexdigest()


NATIVE_BUILD_SCRIPT = "compiler/build.rs"
NATIVE_GRAMMAR = "tooling/tree-sitter-cinch"
# The layout the pin's source hash is recorded in. A pinned archive keeps this
# name; the working tree calls the same sources `seed/`.
SEED_TREE = "compiler"
SEED_WORKING_TREE = "seed"
NATIVE_GRAMMAR_INPUTS = ("grammar.js", "tree-sitter.json", "src/scanner.c")
NATIVE_GRAMMAR_BINDINGS = ("Cargo.toml", "bindings/rust/build.rs", "bindings/rust/lib.rs")


def seed_source_root(directory: Path) -> Path:
    """The seed source tree inside `directory`, whatever this tree names it."""
    working = directory / SEED_WORKING_TREE
    return working if working.is_dir() else directory / SEED_TREE


def native_grammar_inputs(repository: Path, revision: str) -> list[str]:
    manifest = f"{NATIVE_GRAMMAR}/Cargo.toml"
    has_binding = subprocess.check_output(["git", "ls-tree", "--name-only", revision, "--", manifest],
                                          cwd=repository)
    parts = NATIVE_GRAMMAR_INPUTS + (NATIVE_GRAMMAR_BINDINGS if has_binding else ())
    return [f"{NATIVE_GRAMMAR}/{part}" for part in parts]


def seed_sources(directory: Path) -> dict[str, bytes]:
    root = seed_source_root(directory)
    files = {}
    for part in ("src", "prelude", "runtime/src"):
        for path in (root / part).rglob("*"):
            if path.is_file():
                files[f"{SEED_TREE}/{part}/{path.relative_to(root / part).as_posix()}"] = path.read_bytes()
    for part in ("Cargo.toml", "Cargo.lock", ".cargo/config.toml",
                 "runtime/Cargo.toml", "runtime/Cargo.lock"):
        files[f"{SEED_TREE}/{part}"] = (root / part).read_bytes()
    if (root / "build.rs").is_file():
        files[NATIVE_BUILD_SCRIPT] = (root / "build.rs").read_bytes()
        grammar = directory / NATIVE_GRAMMAR
        has_binding = (grammar / "Cargo.toml").is_file()
        for part in NATIVE_GRAMMAR_INPUTS + (NATIVE_GRAMMAR_BINDINGS if has_binding else ()):
            path = f"{NATIVE_GRAMMAR}/{part}"
            files[path] = (directory / path).read_bytes()
    return files


def git_root() -> Path:
    """The checkout that holds the git repository.

    A jj workspace (``.jj/repo`` is a file naming the main checkout) has no
    ``.git`` of its own, so git commands run in the main checkout instead.
    """
    marker = ROOT / ".jj" / "repo"
    # AGENTS.md: never grep for agent files. This reads the workspace pointer.
    if marker.is_file():
        target = Path(marker.read_text().strip())
        if not target.is_absolute():
            target = ROOT / ".jj" / target
        return target.resolve().parents[1]
    return ROOT


def native_source_archive(revision: str) -> bytes:
    # A sandboxed build (Nix) has no git history. CINCH_SOURCE_ARCHIVES names
    # a directory of `<revision>.tar` files to read instead; the callers
    # check the extracted tree against the pinned source hash either way, so
    # an archive from elsewhere cannot stand in for the pinned one.
    supplied = os.environ.get("CINCH_SOURCE_ARCHIVES")
    if supplied:
        archive = Path(supplied) / f"{revision}.tar"
        if not archive.is_file():
            raise BootstrapError(f"CINCH_SOURCE_ARCHIVES has no archive for the pinned revision: {archive}")
        return archive.read_bytes()
    where = git_root()
    tree = SEED_TREE
    if subprocess.run(["git", "ls-tree", "--name-only", revision, "--", SEED_WORKING_TREE],
                      cwd=where, check=True, stdout=subprocess.PIPE).stdout.strip():
        tree = SEED_WORKING_TREE
    paths = [tree]
    build_script = subprocess.run(["git", "ls-tree", "--name-only", revision, "--", f"{tree}/build.rs"],
                                  cwd=where, check=True, stdout=subprocess.PIPE).stdout
    if build_script:
        paths.extend(native_grammar_inputs(where, revision))
    return subprocess.run(["git", "archive", revision, *paths], cwd=where, check=True,
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
    # stage builds and any test subprocesses serial here as well, unless the
    # caller owns the machine and says otherwise: CINCH_CARGO_JOBS widens the
    # Cargo worker pool (dependency crates build side by side; the outputs
    # are the same, since the codegen-unit count below is still pinned).
    env["CARGO_BUILD_JOBS"] = os.environ.get("CINCH_CARGO_JOBS") or "1"
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


def run(command: list[str | Path], cwd: Path, log: Path, *, append: bool = False,
        env_extra: dict[str, str] | None = None) -> int:
    log.parent.mkdir(parents=True, exist_ok=True)
    process_env = environment()
    if env_extra:
        process_env.update(env_extra)
    with log.open("ab" if append else "wb") as stream:
        return subprocess.run([str(x) for x in command], cwd=cwd, env=process_env,
                              stdout=stream, stderr=subprocess.STDOUT).returncode


def default_jobs() -> int:
    # The stage compilers themselves are not parallel and a shard compile
    # saturates a core, so a stage's own parallelism is bounded by the build
    # slots (tooling/build_slots.py): several lanes may build at once, and
    # each one uses a share of the cores rather than all of them.
    import build_slots

    cores = os.cpu_count() or 2
    return max(1, min(cores // 2, max(1, cores // build_slots.slot_count())))


def resolve_jobs(flag: int | None) -> int:
    value = flag if flag is not None else os.environ.get("CINCH_BOOTSTRAP_JOBS")
    if value is None or value == "":
        return default_jobs()
    try:
        jobs = int(value)
    except (TypeError, ValueError):
        raise BootstrapError("--jobs (or CINCH_BOOTSTRAP_JOBS) must be a positive integer")
    if not 1 <= jobs <= 256:
        raise BootstrapError("--jobs (or CINCH_BOOTSTRAP_JOBS) must be between 1 and 256")
    return jobs


def resolve_pgo(flag: bool | None) -> bool:
    if flag is not None:
        return flag
    value = (os.environ.get("CINCH_BOOTSTRAP_PGO") or "").strip().lower()
    if not value:
        return PGO_DEFAULT
    if value in ("1", "yes", "on", "true"):
        return True
    if value in ("0", "no", "off", "false"):
        return False
    raise BootstrapError("CINCH_BOOTSTRAP_PGO must be 0 or 1")


def pgo_dir() -> Path:
    """The profile cache: one profile per compiler source digest and IR family."""
    return BUILD / "pgo"


def seed_flags() -> list[str]:
    """Extra flags for the seed's own build (`CINCH_SEED_FLAGS`).

    Part of the stage-1 cache key: a flag that changes the produced compiler
    must invalidate it.
    """
    return os.environ.get("CINCH_SEED_FLAGS", "").split()


def pgo_workload() -> tuple[Path, str]:
    """The training bundle and its digest, written once per workload text."""
    data = pgo.workload_source(ROOT)
    workload = digest(data)
    path = pgo_dir() / "workload" / f"toolkit-{workload}.cinch"
    if not path.is_file():
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_bytes(data)
    return path, workload


def pgo_profile(family: str, source: str, ir: Path | None = None) -> dict | None:
    """The cached profile for these sources and IR, or None.

    The record carries the source digest, the workload digest, the
    llvm-profdata version and the digest of the IR it was trained on, so a
    profile that outlived a code generation change is rebuilt rather than
    silently applied.
    """
    _, workload = pgo_workload()
    _, version = pgo.profdata_tool()
    profile = pgo.load(pgo_dir(), source, family, workload, version)
    if profile is not None and ir is not None and profile.get("ir_sha256") != file_hash(ir):
        return None
    return profile


def pgo_inputs(profile: dict) -> dict:
    """The stage input record for a compiler linked with this profile."""
    return {"profdata_sha256": profile["profdata_sha256"],
            "workload_sha256": profile["workload_sha256"]}


def pgo_train(family: str, source: str, ir: Path, trainer: Path, directory: Path,
              log: Path, instrumented_seconds: float | None, interfaces: dict) -> dict:
    """Train one profile with the instrumented compiler and install it in the cache.

    The trainer is the instrumented link of the same IR, so its counters
    describe exactly the functions the profile-use link will optimize. The
    work directory is training state and does not survive.
    """
    workload, workload_sha = pgo_workload()
    _, version = pgo.profdata_tool()
    work = directory / "pgo"
    if work.exists():
        shutil.rmtree(work)
    work.mkdir(parents=True)
    manifest = work / "interfaces.json"
    write_json(manifest, interfaces)
    profdata, seconds = pgo.train(trainer, workload, manifest, work, environment(), log)
    profile = pgo.store(pgo_dir(), source, family, workload_sha, file_hash(ir), profdata,
                        trainer=f"{directory.name}: instrumented link",
                        tool_version=version, instrumented_seconds=instrumented_seconds,
                        train_seconds=seconds)
    shutil.rmtree(work)
    print(f"{directory.name}: trained the {family} profile for {source[:12]} "
          f"({profile['profdata_bytes']} bytes) in {seconds:.1f} s", flush=True)
    return profile


def pgo_base_interfaces(directory: Path, defined: set[str]) -> dict:
    """A check manifest from the interfaces a seed build already emitted.

    Stage1 has no library manifest of its own, and building one would need
    the library producer. The seed emits every interface it checked, so the
    training workload can be checked against exactly those. Modules the
    workload or the compiler defines are left out: the workload supplies the
    toolkit bodies, and a compiler module's interface is an implementation
    detail whose dependencies the manifest would then have to carry too.
    """
    from bootstrap_interfaces import interface_hash, read_contract
    rows = []
    for path in sorted((directory / "sources" / "build").glob("*.cinchif")):
        doc = read_contract(path)
        if doc["module"] in defined:
            continue
        rows.append([doc["module"], interface_hash(doc), str(path.resolve())])
    if not rows:
        raise BootstrapError("no base interfaces to train the profile against")
    return {"schema": 1, "interfaces": rows}


def pgo_relink(exe: Path, ir: Path, record: Path, source: str,
               directory: Path, log: Path, interfaces: dict) -> dict:
    """Rebuild the seed's link with the profile and install it as the stage compiler."""
    if not ir.is_file():
        raise BootstrapError("the seed did not retain the link input the PGO relink needs")
    profile = pgo_profile("seed", source, ir)
    if profile is None:
        profile = pgo_train("seed", source, ir, exe, directory, log, None, interfaces)
    print(f"{directory.name}: relinking with profile {profile['profdata_sha256'][:12]}", flush=True)
    argv, cwd = pgo.recorded_link(record)
    wrapper = (ROOT / "tooling" / "shard_ir.py").resolve()
    env = stage_link_environment(shlex.join(pgo.use_flags(profile, pgo_dir())))
    code = run([wrapper, *argv, "-o", exe], cwd, log, append=True, env_extra=env)
    if code:
        raise BootstrapError(f"the profile-use relink of {directory.name} failed (exit {code}): {log}")
    # A link that produces a non-runnable compiler must not be installed.
    startup = run([exe], directory, log, append=True)
    if startup != 1:
        raise BootstrapError(f"the profile-use compiler of {directory.name} does not start "
                             f"(invoked without arguments it returned {startup}, not 1)")
    return profile


def stage_link_environment(append: str | None = None,
                           record: Path | None = None) -> dict[str, str]:
    """Point the seed's own clang call at the sharding wrapper.

    The pinned seed and the separate host link both honour `CINCH_CLANG`.
    The wrapper shards one huge `.ll` link and execs the real
    clang for everything else. JOBS == 1 and no appended flags leaves the
    seed untouched, because then the wrapper has nothing to do.
    """
    if SHARDS <= 1 and append is None:
        return {}
    wrapper = (ROOT / "tooling" / "shard_ir.py").resolve()
    real = os.environ.get("CINCH_CLANG") or shutil.which("clang") or "clang"
    if Path(real).resolve() == wrapper:
        real = shutil.which("clang") or "clang"
    env = {"CINCH_CLANG": str(wrapper), "CINCH_CLANG_REAL": str(real),
           "CINCH_BOOTSTRAP_JOBS": str(JOBS), "CINCH_BOOTSTRAP_SHARDS": str(SHARDS),
           "CINCH_SHARD_PARTITION": "stable" if STABLE_SHARDS else "balanced"}
    if append is not None:
        env["CINCH_CLANG_APPEND"] = append
    if record is not None:
        env["CINCH_CLANG_RECORD"] = str(record)
    return env


def shard_identity() -> dict:
    # Absent for the balanced partition, so existing receipts stay valid.
    return {"shard_partition": "stable"} if STABLE_SHARDS else {}


def required_artifacts(inputs: dict) -> tuple[str, ...]:
    if inputs.get("built_by_stage", 0) > 0:
        outputs = ("compiler.ll", "link-report.txt", "base-interfaces.json")
        if inputs.get("frontend_mode") == "typed-ir-objects":
            outputs += ("objects.json",)
        if "interface_package_sha256" in inputs:
            outputs += ("base.cpk",)
        return outputs
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
                                    for p in sorted(directory.glob("compiler.shard*.o"))})
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
    lock = read_json(ROOT / "compiler" / "pins" / "stage0.lock.json")
    if lock.get("schema") != 1:
        raise BootstrapError("unsupported or missing compiler/pins/stage0.lock.json")
    patch = (ROOT / lock["patch"]).read_bytes()
    if digest(patch) != lock["patch_sha256"]:
        raise BootstrapError("the stage-0 patch differs from its lock; update the seed intentionally")
    return lock, patch


def library_provider_lock() -> dict:
    lock = read_json(ROOT / "compiler/pins/library-provider.lock.json")
    if not isinstance(lock, dict):
        raise BootstrapError("unsupported or missing library producer lock")
    revision, source = lock.get("revision"), lock.get("source_sha256")
    if (type(lock.get("schema")) is not int or lock["schema"] != 1
            or type(lock.get("runtime_abi")) is not int or lock["runtime_abi"] not in RUNTIME_ABIS
            or not isinstance(revision, str) or not re.fullmatch(r"[0-9a-f]{40}", revision)
            or not isinstance(source, str) or not re.fullmatch(r"[0-9a-f]{64}", source)):
        raise BootstrapError("unsupported or missing library producer lock")
    source_abi = ROOT / "compiler/Bootstrap.RuntimeAbi.cinch"
    if not source_abi.is_file() or source_abi.read_bytes() != runtime_abi_source(lock["runtime_abi"]):
        raise BootstrapError("library producer runtime ABI differs from the compiler's target ABI")
    return lock


def prebuilt_compiler(revision: str) -> Path | None:
    """A pinned Rust compiler somebody else already built, or None.

    CINCH_PREBUILT_COMPILERS names a directory of `<revision>/cinch`
    executables. It exists for builds where the cargo step is its own cached
    unit (the Nix derivations in tooling/nix): the release build of a pinned
    revision is a function of that revision and the toolchain, so it should
    not be repeated because a file in tooling/ changed. Whoever sets the
    variable vouches for the binary; the source snapshot is still checked
    against the pin, and the default (unset) builds from it as before.
    """
    supplied = os.environ.get("CINCH_PREBUILT_COMPILERS")
    if not supplied:
        return None
    product = Path(supplied) / revision / "cinch"
    if not (product.is_file() and os.access(product, os.X_OK)):
        raise BootstrapError(f"CINCH_PREBUILT_COMPILERS has no executable for the pinned revision: {product}")
    return product


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
    product = prebuilt_compiler(lock["revision"])
    if product is not None:
        log.write_text(f"prebuilt pinned compiler: {product}\n")
        code = 0
    else:
        code = run(["cargo", "build", "--locked", "--release", "--bin", "cinch",
                    "--target-dir", target], seed_source_root(snapshot), log)
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
    runtime = seed_source_root(snapshot) / "runtime"
    archive = runtime / "target/release/libcinchruntime.a"
    log = BUILD / "stage0/runtime-build.log"
    code = run(["cargo", "build", "--locked", "--release", "--manifest-path",
                runtime / "Cargo.toml", "--target-dir", runtime / "target"],
               seed_source_root(snapshot), log)
    if code or not archive.is_file():
        raise BootstrapError(f"the optimized seed runtime did not build: {log}")
    # The pinned Native linker prefers a fresh release archive. Never let a
    # failed/stale optimized build quietly select its debug fallback.
    sources = [runtime / "Cargo.toml", *[p for p in (runtime / "src").rglob("*") if p.is_file()]]
    if any(p.stat().st_mtime_ns > archive.stat().st_mtime_ns for p in sources):
        raise BootstrapError("the optimized seed runtime is older than its source")
    return archive


# The parts of a build tree that depend only on the pins (the seed, the
# library producer and their sources), never on the compiler sources. Their
# receipts hold hashes, not paths, so a copy still verifies. One path does
# survive a copy: the seed's driver reads its runtime from the directory it
# was compiled in, so the tree a copy came from (or, for a Nix build, the
# seed's own store path) has to stay where it was. They are the
# expensive, rarely-changing half of a cold chain (two release cargo builds),
# so they are what a binary cache keeps across commits. Stage directories are
# deliberately absent: a stage-2/3 receipt covers `objects.json`, which names
# the library object by absolute path.
PIN_KEYED = ("stage0", "seed-sources", "library-sources", "library-provider")
# Cargo's own target directories are rebuilt inputs, not artifacts. The seed
# runtime's target stays: `build_seed_runtime` and the seed's own driver read
# the archive in place, and it is a few megabytes.
PIN_SCRATCH = ("cargo", "library-cargo", "library-runtime")


def build_pins() -> None:
    """Build everything the pins determine, and nothing the sources do."""
    build_seed()
    build_seed_runtime()
    runtime_provider()
    print("bootstrap: pin-keyed artifacts built", flush=True)


def hydrate(source: Path) -> None:
    """Copy pin-keyed artifacts from a prebuilt tree into this build tree.

    The copy is advisory: every consumer re-verifies what it finds against
    the pins (`cached`, `source_hash`, the provider receipt), so a tree
    built for other pins is rebuilt rather than trusted. Existing entries
    are kept, which makes the command safe to repeat.
    """
    if not source.is_dir():
        raise BootstrapError(f"no prebuilt bootstrap tree at {source}")
    copied = []
    for name in PIN_KEYED:
        origin, target = source / name, BUILD / name
        if not origin.is_dir():
            continue
        children = [origin] if name == "stage0" else sorted(p for p in origin.iterdir() if p.is_dir())
        for child in children:
            destination = target if name == "stage0" else target / child.name
            if destination.exists():
                continue
            destination.parent.mkdir(parents=True, exist_ok=True)
            temporary = destination.with_name(destination.name + ".hydrating")
            shutil.rmtree(temporary, ignore_errors=True)
            # Preserve modification times: the seed runtime check compares
            # the archive's against its sources'.
            shutil.copytree(origin if name == "stage0" else child, temporary, symlinks=True)
            for path in [temporary, *temporary.rglob("*")]:
                if not path.is_symlink():
                    path.chmod(path.stat().st_mode | 0o200 | (0o100 if path.is_dir() else 0))
            temporary.rename(destination)
            copied.append(str(destination.relative_to(BUILD)))
    print("bootstrap: hydrated " + (", ".join(copied) if copied else "nothing (already present)"), flush=True)


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


def host_source() -> bytes:
    # The staged CLI defaults `--target` to the machine it runs on, and a
    # compiled Cinch program cannot ask Rust for its own triple the way the
    # seed does (`host_target_components`). So the build host is rendered into
    # a module before the sources are hashed, exactly as the runtime ABI is.
    # The rendering is a pure function of the host, with no timestamp and a
    # fixed field order, so one machine renders identical bytes every run and
    # a hash change can only mean a genuinely different host.
    system, machine = HOST["system"], HOST["machine"]
    os = {"Darwin": "darwin", "Linux": "linux", "Windows": "windows",
          "FreeBSD": "freebsd"}.get(system)
    if os is None:
        raise BootstrapError(f"unsupported build host for target profiles: {system!r}")
    # Kept in the triple spelling (`aarch64`, `x86_64`) so `--target` compares
    # against a real triple.
    arch = {"arm64": "aarch64", "aarch64": "aarch64",
            "x86_64": "x86_64", "AMD64": "x86_64", "x86": "x86_64"}.get(machine)
    if arch is None:
        raise BootstrapError(f"unsupported build host architecture: {machine!r}")
    libc = {"darwin": "libSystem", "linux": "glibc", "windows": "msvcrt"}.get(os, "none")
    vendor = {"darwin": "apple", "windows": "pc"}.get(os, "unknown")
    family = {"darwin": "darwin", "linux": "linux-gnu", "windows": "windows-msvc"}.get(os, os)
    return ("-- The machine this compiler was built on. The staged builder renders\n"
            "-- this module from the build host before hashing the complete sources,\n"
            "-- because `--target` defaults to the host and a compiled program cannot\n"
            "-- ask Rust for its own triple. Nothing derives the target from it.\n"
            "module Bootstrap.Host where\n"
            "  export host-triple, host-os, host-arch, host-libc\n"
            "  host-triple :: String\n"
            f"  host-triple = \"{arch}-{vendor}-{family}\"\n"
            "  host-os :: String\n"
            f"  host-os = \"{os}\"\n"
            "  host-arch :: String\n"
            f"  host-arch = \"{arch}\"\n"
            "  host-libc :: String\n"
            f"  host-libc = \"{libc}\"\n").encode()


def collect_compiler_sources() -> dict[str, bytes]:
    # The seed links Stage1. The compiler inside Stage1 targets the pinned
    # library and runtime used to build and run Stage2 and later.
    lock = library_provider_lock()
    # Reproduction clients retained next to compiler sources are not compiler
    # modules and must not become aliases or duplicate the pipeline's Main.
    files = {p.name: p.read_bytes() for p in sorted((ROOT / "compiler").glob("Bootstrap.*.cinch"))}
    files["Bootstrap.RuntimeAbi.cinch"] = runtime_abi_source(lock["runtime_abi"])
    files["Bootstrap.Host.cinch"] = host_source()
    for module in TOOLKIT:
        files[f"Text.{module}.cinch"] = (ROOT / "seed" / "prelude" / "Text" / f"{module}.cinch").read_bytes()
    for module in COMPILER_PRELUDE:
        files[f"{module}.cinch"] = (ROOT / "seed" / "prelude" / f"{module.replace('.', '/')}.cinch").read_bytes()
    files["Main.cinch"] = (ROOT / "compiler" / "programs" / "Run.Pipeline.cinch").read_bytes()
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
    for name in ("cinch", "cinch.instrumented", "compiler.ll", "compiler.ll.diagnostics",
                 "link-report.txt", "requested-roots.json", "seed-link.json"):
        (directory / name).unlink(missing_ok=True)
    for shard in directory.glob("compiler.shard*"):
        shard.unlink()
    for interface in directory.rglob("*.cinchif"):
        interface.unlink()
    # Training state never outlives the build that produced it.
    shutil.rmtree(directory / "pgo", ignore_errors=True)


def build_stage(stage: int) -> Path:
    if stage == 0:
        return build_seed()
    return build_next_stage(stage, build_stage(stage - 1))


def compile_seed_stage(builder: Path, sources: Path, compiler: Path, runtime: Path, log: Path,
                       flags: list[str], link_env: dict[str, str]) -> int:
    if flags:
        # Custom seed options can select their own target, objects and linker.
        return run([builder, "build", sources / "Main.cinch", "-o", compiler,
                    "--opt-level", "2", *flags], sources, log, env_extra=link_env)

    # Let the pinned frontend exit before clang optimizes the module. The
    # seed otherwise holds the checked compiler and its IR throughout linking.
    ir = compiler.with_suffix(".ll")
    code = run([builder, "emit-ir", sources / "Main.cinch", "-o", ir], sources, log)
    if code:
        return code
    manifest = read_json(sources / "build" / "Main.manifest.json")
    plan = manifest.get("link", {})
    words = plan.get("words") if isinstance(plan, dict) else None
    if (not isinstance(words, list) or not all(isinstance(word, str) for word in words)
            or plan.get("native_objects") != [] or plan.get("shared") is not False):
        with log.open("a") as stream:
            stream.write("the seed emitted an unsupported compiler link plan\n")
        return 1
    clang = link_env.get("CINCH_CLANG") or os.environ.get("CINCH_CLANG") or "clang"
    command = [clang, ir, "-O2", runtime, "-o", compiler, *words]
    if HOST["system"] == "Linux":
        command.extend(["-ldl", "-lpthread", "-lm"])
    # Use the pinned driver's lock while clang reads its runtime archive.
    with (runtime.parent.parent / ".cinch-runtime.lock").open("a") as lock:
        fcntl.flock(lock, fcntl.LOCK_SH)
        return run(command, sources, log, append=True, env_extra=link_env)


def build_next_stage(stage: int, builder: Path) -> Path:
    files = compiler_sources()
    if stage >= 2:
        source = BUILD / "stage1" / "compiler.cinch"
        if source.read_bytes() != bundle(files):
            raise BootstrapError("compiler sources changed while preparing the stage chain")
        return build_self(builder, files, BUILD / f"stage{stage}", stage - 1, source)
    runtime = build_seed_runtime()
    inputs = {"protocol": PROTOCOL, "host": HOST, "source_sha256": source_hash(files),
              "builder_sha256": file_hash(builder), "built_by_stage": stage - 1,
              "shard_count": SHARDS, **shard_identity(), "seed_flags": seed_flags(),
              "seed_runtime_profile": "release", "seed_runtime_sha256": file_hash(runtime)}
    directory = BUILD / f"stage{stage}"
    compiler = directory / "cinch"
    # The seed writes this sidecar for clang. The fixed point compares the
    # later stages' compiler.ll, so a successful Stage1 can discard it.
    seed_ir = compiler.with_suffix(".ll")
    guided = PGO and PGO_SEED
    profile = pgo_profile("seed", inputs["source_sha256"]) if guided else None
    if profile is not None:
        inputs["pgo"] = pgo_inputs(profile)
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
    # Stage1 does the largest compile of the chain (it builds Stage2), so the
    # seed links it optimized. Above one job the clang call goes
    # through tooling/shard_ir.py, which shards its single `.ll` module. With
    # PGO the seed links an instrumented module, the resulting compiler trains
    # the profile, and the recorded seed command is replayed with it.
    record = directory / "seed-link.json"
    append = None
    if guided:
        record.unlink(missing_ok=True)
        append = shlex.join(pgo.generate_flags(directory / "pgo" / "raw"))
    code = compile_seed_stage(builder, sources, compiler, runtime, log, inputs["seed_flags"],
                              stage_link_environment(append, record if guided else None))
    if code == 0 and guided:
        profile = pgo_relink(compiler, seed_ir, record, inputs["source_sha256"],
                             directory, log,
                             pgo_base_interfaces(directory,
                                                 {name[:-6] for name in files
                                                  if name.endswith(".cinch")}
                                                 | pgo.workload_modules()))
        inputs["pgo"] = pgo_inputs(profile)
    record_build(directory, inputs, code, log)
    seed_ir.unlink(missing_ok=True)
    return compiler


def link_directory(ir: Path, fingerprint: str) -> Path:
    """The one path every stage hands clang for this IR module.

    A profile-use compile of identical bytes from two different paths
    produces different objects: a profile's records are matched per module,
    and the module's identity is the path clang was given. Stage2 and Stage3
    link byte-identical IR, so they must also compile it from one path, and a
    rebuilt stage must reuse the path its profile was trained under. The
    directory is keyed by the IR digest, which keeps the choice content
    addressed; the older directories in it can never be used again.
    """
    root = BUILD / "shards"
    root.mkdir(parents=True, exist_ok=True)
    directory = root / fingerprint[:16]
    shared = directory / ir.name
    if shared.is_file() and file_hash(shared) == fingerprint:
        return shared
    for stale in root.iterdir():
        if stale != directory:
            shutil.rmtree(stale, ignore_errors=True)
    directory.mkdir(parents=True, exist_ok=True)
    shutil.copyfile(ir, shared)
    return shared


# The bootstrap lock covers the shared IR directory throughout a stage chain.
# Record only the bytes split by this process. Each stage still compiles its
# own emitted IR into native objects and links its own executable.
_link_shards: dict[tuple[Path, str, int, bool], dict[Path, str]] = {}


def prepare_link_ir(ir: Path, jobs: int) -> tuple[Path, list[Path]]:
    from shard_ir import write_shards

    fingerprint = file_hash(ir)
    shared = link_directory(ir, fingerprint)
    if jobs == 1:
        return shared, [shared]
    key = (shared, fingerprint, jobs, STABLE_SHARDS)
    previous = _link_shards.get(key)
    if previous and all(path.is_file() and file_hash(path) == expected
                        for path, expected in previous.items()):
        return shared, list(previous)
    shards = write_shards(shared, shared.parent, jobs, stable=STABLE_SHARDS)
    _link_shards.clear()
    _link_shards[key] = {path: file_hash(path) for path in shards}
    return shared, shards


def build_self(builder: Path, files: dict[str, bytes], directory: Path,
               built_by_stage: int, source: Path | None = None,
               pgo_enabled: bool | None = None, opt_level: int = 2,
               publish_interfaces: bool = True) -> Path:
    from pack_interfaces import pack, publish

    provider = runtime_provider()
    base = provider["interfaces"]
    base_bytes = json.dumps(base, sort_keys=True, separators=(",", ":")).encode()
    try:
        packed = pack([(name, identity, Path(path).read_bytes())
                       for name, identity, path in base["interfaces"]], flat_bodies=True)
    except (KeyError, TypeError, ValueError) as error:
        raise BootstrapError(f"cannot pack the base interfaces: {error}") from error
    configuration = json.dumps(provider["configuration"], sort_keys=True, separators=(",", ":")).encode()
    runtime = provider["runtime"]
    sources = source_hash(files)
    inputs = {"protocol": PROTOCOL, "host": HOST, "source_sha256": sources,
              "builder_sha256": file_hash(builder), "built_by_stage": built_by_stage,
              "shard_count": SHARDS, **shard_identity(),
              "interface_set_sha256": digest(base_bytes), "frontend_mode": "typed-ir-objects",
              "interface_package_sha256": digest(packed),
              "interface_output": "checked-schema10" if publish_interfaces else "none",
              "library_provider": provider["identity"], "object_configuration_sha256": digest(configuration),
              "runtime_archive_sha256": file_hash(runtime)}
    # Normal stages keep the existing -O2 cache identity. A one-shot
    # behavioral probe records its different clang setting explicitly.
    if opt_level != 2:
        inputs["clang_opt_level"] = opt_level
    guided = PGO if pgo_enabled is None else pgo_enabled
    cached_profile = pgo_profile("cinch", sources) if guided else None
    if cached_profile is not None:
        inputs["pgo"] = pgo_inputs(cached_profile)
    if cached(directory, inputs):
        print(f"{directory.name}: verified cached compiler", flush=True)
        return directory / "cinch"
    prepare_outputs(directory)
    manifest = directory / "base-interfaces.json"
    manifest.write_bytes(base_bytes)
    package = directory / "base.cpk"
    publish(package, packed)
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
    mode = "--emit-ir-with-interfaces" if publish_interfaces else "--emit-ir-only"
    code = run([builder, mode, "--interfaces", package, "--objects", objects, source, ir], directory, log)
    if code:
        from bootstrap_objects import requested_roots
        diagnostics = directory / "compiler.ll.diagnostics"
        if diagnostics.is_file():
            write_json(directory / "requested-roots.json", requested_roots(diagnostics.read_text(errors="replace")))
    profile = pgo_profile("cinch", sources, ir) if guided and code == 0 else None
    if profile is not None:
        print(f"{directory.name}: linking with profile {profile['profdata_sha256'][:12]}", flush=True)
    if code == 0:
        # Optimized: the stage compilers are the bootstrap's hot loop, and
        # identical IR under identical flags still yields identical binaries.
        # Above one job the module is cut into deterministic shards, each
        # compiled in parallel, and the objects are linked in shard order.
        # Shard contents and link order depend only on the IR text and JOBS,
        # never on scheduling. The input path is the stage's shared one
        # because a profile-use compile depends on it: identical IR compiled
        # from another path would not be the same binary.
        #
        # No -flto here, deliberately. Measured 2026-11-11 on the stage-3
        # module with this profile: -flto=thin cut the executable from 19.7 MB
        # to 13.3 MB (the export table shrinks, __text 14.4 MB to 11.5 MB) and
        # ran the 40K-line self-check within 2% of this link's CPU, at 26% to
        # 33% more link time; -flto and a sharded thin link cost 4% to 9% of
        # check CPU. LTO has nothing to inline outside the module: the library
        # object and the runtime archive are native Mach-O, not bitcode, so
        # 25,390 of the module's 203,698 call sites stay cross-object calls.
        # design/compilation-speed.md section 8 has the tables.
        from shard_ir import compile_shards, object_cache, object_paths
        clang = os.environ.get("CINCH_CLANG", "clang")
        link_ir, shards = prepare_link_ir(ir, SHARDS)
        shard_objects = object_paths(directory, SHARDS) if SHARDS > 1 else None

        def link(output: Path, flags: tuple[str, ...], sharded: bool = True) -> int:
            """Compile the emitted IR and link it into output.

            The instrumented training link is never sharded: a profile
            counter block per translation unit makes a sharded
            -fprofile-generate link crash on its first run (exit -10) while
            the same link as one unit runs the workload, so the one link
            whose speed does not matter is built the way that works.
            """
            if sharded and SHARDS > 1:
                failure, shard_log = compile_shards(shards, shard_objects, clang, opt_level, JOBS,
                                                    directory, environment(), flags,
                                                    object_cache(clang, flags))
                with log.open("ab") as stream:
                    stream.write(shard_log.encode())
                if failure:
                    return failure
                command = [clang, *shard_objects, provider["object"], runtime, "-o", output, *flags]
            else:
                command = [clang, f"-O{opt_level}", "-Wno-override-module", *flags, link_ir,
                           provider["object"], runtime, "-o", output]
            if HOST["system"] == "Linux":
                command.extend(["-ldl", "-lpthread", "-lm"])
            return run(command, directory, log, append=True)

        if guided and profile is None:
            # One instrumented link per source revision trains the profile:
            # the counters describe exactly the functions the profile-use
            # link optimizes, and the instrumented binary is training state.
            instrumented = directory / "cinch.instrumented"
            started = time.monotonic()
            code = link(instrumented, tuple(pgo.generate_flags(directory / "pgo" / "raw")), sharded=False)
            seconds = time.monotonic() - started
            if code == 0:
                profile = pgo_train("cinch", sources, ir, instrumented, directory, log,
                                    seconds, base)
            instrumented.unlink(missing_ok=True)
        if code == 0:
            code = link(exe, tuple(pgo.use_flags(profile, pgo_dir())) if profile else ())
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
    if profile is not None:
        inputs["pgo"] = pgo_inputs(profile)
    return record_build(directory, inputs, code, log)


def runtime_provider() -> dict:
    """Select the pinned base by default, or fail closed on an explicitly supplied artifact receipt."""
    from bootstrap_objects import artifact_provider, provider
    return (artifact_provider(sys.modules[__name__], LIBRARY_ARTIFACT) if LIBRARY_ARTIFACT is not None
            else provider(sys.modules[__name__]))


def runtime_archive() -> Path:
    lock, _ = seed_inputs()
    snapshot = BUILD / "seed-sources" / digest(json.dumps(lock, sort_keys=True).encode())
    if source_hash(seed_sources(snapshot)) != lock["source_sha256"]:
        raise BootstrapError("the pinned runtime source snapshot changed before linking")
    for profile in ("release", "debug"):
        archive = seed_source_root(snapshot) / "runtime/target" / profile / "libcinchruntime.a"
        if archive.is_file():
            return archive
    raise BootstrapError("the verified stage-1 build has no matching pinned runtime archive")


def smoke(stage: int, compiler: Path) -> None:
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
    compiler = build_seed()
    smoke(0, compiler)
    for stage in (1, 2, 3):
        compiler = build_next_stage(stage, compiler)
        smoke(stage, compiler)
    second, third = BUILD / "stage2", BUILD / "stage3"
    if file_hash(second / "cinch") != file_hash(third / "cinch"):
        raise BootstrapError("stage-2 and stage-3 compiler executables differ")
    if not same_file_bytes(second / "compiler.ll", third / "compiler.ll"):
        raise BootstrapError("stage-2 and stage-3 LLVM IR differ")
    interfaces = {str(p.relative_to(second)): file_hash(p) for p in compiler_interface_paths(second)}
    rebuilt = {str(p.relative_to(third)): file_hash(p) for p in compiler_interface_paths(third)}
    require_compiler_interfaces(second, compiler_sources())
    if not interfaces or interfaces != rebuilt:
        raise BootstrapError("canonical stage-2/stage-3 interfaces are missing or differ")
    # The consumer validates the third stage's contracts before using them.
    check_interface_consumer(third, compiler_sources())
    check_changed_source()
    print("bootstrap: stages 2 and 3 reached a fixed point", flush=True)


def compiler_interface_paths(directory: Path):
    """The emitter's canonical per-module interfaces.

    A checked build also leaves the admitted closure in `build/`: the
    library's interfaces plus a second copy of every source module. Those
    are not canonical and would read as duplicates of the emitted set.
    """
    for path in sorted(directory.rglob("*.cinchif")):
        if "build" in path.relative_to(directory).parts:
            continue
        yield path


def require_compiler_interfaces(directory: Path, files: dict[str, bytes]) -> None:
    from bootstrap_interfaces import interface_hash, read_contract, validate_contract, verify_dependencies

    expected = {name[:-6] for name in files if name.endswith(".cinch")}
    documents = {}
    for path in compiler_interface_paths(directory):
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
    for path in compiler_interface_paths(directory):
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
    code = run([directory / "cinch", "--check-types", "--interfaces", manifest, source, output],
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
    # This executable runs once on a two-line refusal. It still compiles the
    # complete changed source with Stage2 and links its actual emitted IR.
    compiler = build_self(BUILD / "stage2" / "cinch", files, directory, 2, pgo_enabled=False,
                          opt_level=0, publish_interfaces=False)
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
    parser.add_argument("command", choices=("build", "smoke", "check", "status", "pins", "hydrate"))
    parser.add_argument("--from", dest="prebuilt", type=Path,
                        help="hydrate: a prebuilt bootstrap tree (for example a Nix output) to copy pin-keyed artifacts from")
    parser.add_argument("--stage", type=int, choices=range(4), default=1)
    parser.add_argument("-j", "--jobs", type=int, default=None,
                        help="parallel clang shard jobs (default: half the cores; "
                             "CINCH_BOOTSTRAP_JOBS; 1 keeps the single module)")
    parser.add_argument("--dev", action="store_true",
                        help="development build: no PGO, a stable partition into "
                             f"{DEV_SHARDS} shards, and shard objects reused by content from "
                             "CINCH_SHARD_CACHE (default: <build>/shard-cache). Not for an audited chain")
    parser.add_argument("--shards", type=int, default=None,
                        help="shard count when it should differ from --jobs (CINCH_BOOTSTRAP_SHARDS)")
    parser.add_argument("--pgo", dest="pgo", action="store_true", default=None,
                        help="link the stage compilers with a source-keyed profile "
                             f"(default: {'on' if PGO_DEFAULT else 'off'}; CINCH_BOOTSTRAP_PGO)")
    parser.add_argument("--pgo-seed", action="store_true",
                        help="also link Stage 1 with a profile (CINCH_BOOTSTRAP_PGO_SEED=1); "
                             "it costs a second compile of the seed's module")
    parser.add_argument("--no-pgo", dest="pgo", action="store_false",
                        help="plain -O2 stage links, as before profile-guided optimization")
    parser.add_argument("--library-artifact", type=Path,
                        help="verified current-source base receipt for stages 2/3; leaves the seed and library pins unchanged")
    args = parser.parse_args()
    try:
        global JOBS, PGO, PGO_SEED, LIBRARY_ARTIFACT, SHARDS, STABLE_SHARDS
        LIBRARY_ARTIFACT = args.library_artifact.resolve() if args.library_artifact is not None else None
        JOBS = resolve_jobs(args.jobs)
        PGO = resolve_pgo(args.pgo)
        PGO_SEED = args.pgo_seed or (os.environ.get("CINCH_BOOTSTRAP_PGO_SEED") or "").strip().lower() in ("1", "yes", "on", "true")
        requested = args.shards if args.shards is not None else os.environ.get("CINCH_BOOTSTRAP_SHARDS")
        SHARDS = int(requested) if requested else JOBS
        STABLE_SHARDS = os.environ.get("CINCH_SHARD_PARTITION") == "stable"
        if args.dev:
            if args.pgo:
                raise BootstrapError("--dev builds without a profile; drop --pgo")
            PGO, STABLE_SHARDS = False, True
            SHARDS = int(requested) if requested else DEV_SHARDS
            os.environ.setdefault("CINCH_SHARD_CACHE", str(BUILD / "shard-cache"))
        if not 1 <= SHARDS <= 4096 or (STABLE_SHARDS and SHARDS < 2):
            raise BootstrapError("--shards must be between 1 and 4096, and at least 2 for a stable partition")
        environment()
        BUILD.mkdir(parents=True, exist_ok=True)
        # Serialize mutations of the stage cache; ordinary cargo test runs
        # use independent snapshots and do not hold this lock.
        if args.command != "status":
            import build_slots

            # Bound the number of heavy builds across every lane before this
            # tree's own stage lock, so a lane waiting for a slot never holds
            # a lock another process needs.
            build_slots.hold(owner=f"{args.command} stage{args.stage} {BUILD}")
            stage_lock = (BUILD / ".lock").open("a")
            try:
                fcntl.flock(stage_lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
            except OSError:
                raise BootstrapError(
                    "a test suite holds this tree's stage lock "
                    f"({BUILD / '.lock'}); wait for tooling/run_stage_targets.py to finish, "
                    "or build with CINCH_BOOTSTRAP_BUILD set to another directory")
            # The pin-keyed commands read no compiler sources, and a build
            # that selects only their inputs (tooling/nix) has none.
            if args.command not in ("pins", "hydrate"):
                global FROZEN_SOURCES
                FROZEN_SOURCES = collect_compiler_sources()
        if args.command == "build":
            build_stage(args.stage)
        elif args.command == "pins":
            build_pins()
        elif args.command == "hydrate":
            if args.prebuilt is None:
                raise BootstrapError("hydrate needs --from DIR")
            hydrate(args.prebuilt.resolve())
        elif args.command == "smoke":
            smoke(args.stage, build_stage(args.stage))
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
                elif status == "built" and stage >= 2 and LIBRARY_ARTIFACT is not None and inputs.get("library_provider") != runtime_provider()["identity"]:
                    status = "different explicit library artifact"
                elif status == "built" and stage >= 2 and LIBRARY_ARTIFACT is None and (
                        inputs.get("library_provider", {}).get("producer") != read_json(ROOT / "compiler/pins/library-provider.lock.json")
                        or inputs.get("library_provider", {}).get("roots_sha256") != file_hash(ROOT / "compiler/pins/library-object-roots.json")):
                    status = "stale library inputs"
                elif status == "built" and inputs.get("host") != HOST:
                    status = "different build host"
                profile = inputs.get("pgo") if isinstance(inputs.get("pgo"), dict) else None
                print(f"stage{stage}: {status}"
                      + (f" (profile {profile['profdata_sha256'][:12]})" if profile else ""))
            print(f"pgo: {'on' if PGO else 'off'} for this invocation; cache {pgo_dir()}")
        return 0
    except (BootstrapError, pgo.ProfileError, OSError, subprocess.CalledProcessError) as error:
        print(f"bootstrap: {error}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
