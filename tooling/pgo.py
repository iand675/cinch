#!/usr/bin/env python3
"""Profile-guided optimization for the stage compilers.

Every stage is the same program, but not the same IR: Stage1's module is
emitted by the pinned Rust seed, and Stage2's and Stage3's by a Cinch
compiler, with the fixed point keeping Stage2's and Stage3's IR identical.
LLVM matches a profile to a function by name and control flow, so the cache
holds one profile per IR family: `seed` for Stage1, `cinch` for Stage2 and
Stage3. Stage3 therefore reuses what Stage2's instrumented build trained.

Invariants:

  - a cached profile is used only for the compiler sources, the workload and
    the llvm-profdata build that produced it; the record holds all three and
    every use rechecks them, and it also holds the digest of the IR it was
    trained on so a profile cannot outlive a code generation change;
  - the instrumented link is training state: it never becomes a stage's
    `cinch`, never enters `stage.json`, and its work directory is removed;
  - profile data moves basic-block weights and inline costs only, so the
    emitted IR and every `.cinchif` stay byte identical, and two stages that
    link the same IR with the same profile stay byte identical too.
"""
from __future__ import annotations

import hashlib
import json
import os
import platform
import shutil
import subprocess
import time
from pathlib import Path

# 2: the stage link compiles its module from one content-addressed path,
# because a profile's records are matched per module and the module's
# identity is the path clang was given. A profile trained under the older
# per stage paths no longer describes what the link compiles.
SCHEMA = 2
# The toolkit the compiler imports and the base package does not carry. A
# check of it is the workload the measured 10% came from, at about 4,000
# lines, and it is the same text for every stage.
TOOLKIT = ("Lex", "Scan", "Parse", "Json", "Builder")
# IR level instrumentation: counters live in the module, so a `.ll` input is
# instrumented without a frontend.
GENERATE = "-fprofile-generate"
# A function whose control flow moved, or that the run never reached, is
# skipped instead of failing the link. The cache digests, not the warnings,
# are what stop a stale profile from being used.
USE_FLAGS = ("-Wno-profile-instr-out-of-date", "-Wno-profile-instr-unprofiled")

_TOOL: tuple[str, str] | None = None


class ProfileError(RuntimeError):
    pass


def digest(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def file_digest(path: Path) -> str:
    return digest(path.read_bytes())


def workload_source(root: Path) -> bytes:
    """The training bundle: every toolkit module, with an entry that imports them.

    The checker follows imports, so the bundle needs a Main that names each
    module or the rest of the file is never checked. Qualified aliases keep
    the toolkit's exports out of Main's scope.
    """
    files = {f"Text.{name}.cinch": (root / "compiler" / "prelude" / "Text" / f"{name}.cinch").read_bytes()
             for name in TOOLKIT}
    imports = "".join(f"  import Text.{name} as ToolkitModule{index}\n"
                      for index, name in enumerate(TOOLKIT))
    files["Main.cinch"] = ("module Main where\n" + imports + "\n  main :: Int\n  main = 0\n").encode()
    return b"\n\n".join(b"-- source: " + name.encode() + b"\n" + data
                        for name, data in sorted(files.items())) + b"\n"


def workload_modules() -> set[str]:
    """The modules the training bundle defines, which a manifest must not shadow."""
    return {f"Text.{name}" for name in TOOLKIT} | {"Main"}


def profdata_tool() -> tuple[str, str]:
    """(llvm-profdata path, version line). The version is part of the cache key."""
    global _TOOL
    if _TOOL is not None:
        return _TOOL
    candidates: list[str] = []
    if platform.system() == "Darwin" and shutil.which("xcrun"):
        located = subprocess.run(["xcrun", "-f", "llvm-profdata"],
                                 stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
        text = located.stdout.decode(errors="replace").strip()
        if located.returncode == 0 and text:
            candidates.append(text)
    names = ("llvm-profdata", *(f"llvm-profdata-{version}" for version in range(21, 13, -1)))
    candidates.extend(found for found in (shutil.which(name) for name in names) if found)
    for candidate in candidates:
        if not os.access(candidate, os.X_OK):
            continue
        banner = subprocess.run([candidate, "--version"],
                                stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
        lines = banner.stdout.decode(errors="replace").strip().splitlines()
        if banner.returncode == 0 and lines:
            _TOOL = (candidate, lines[0])
            return _TOOL
    raise ProfileError("llvm-profdata was not found; PGO needs it (xcrun llvm-profdata on macOS)")


def profile_path(root: Path, source_sha256: str, family: str) -> Path:
    return root / f"{source_sha256}-{family}.profdata"


def load(root: Path, source_sha256: str, family: str, workload_sha256: str,
         tool_version: str) -> dict | None:
    """The profile record for exactly these inputs, or None."""
    path = profile_path(root, source_sha256, family)
    try:
        record = json.loads(path.with_suffix(".json").read_text())
    except (FileNotFoundError, json.JSONDecodeError):
        return None
    if not isinstance(record, dict):
        return None
    if (record.get("schema") != SCHEMA or record.get("source_sha256") != source_sha256
            or record.get("family") != family or record.get("workload_sha256") != workload_sha256
            or record.get("profdata_tool") != tool_version):
        return None
    if not path.is_file() or file_digest(path) != record.get("profdata_sha256"):
        return None
    return record


def train(compiler: Path, workload: Path, manifest: Path, work: Path,
          env: dict[str, str], log: Path) -> tuple[Path, float]:
    """Run the instrumented compiler on the workload and merge its counters."""
    tool, _ = profdata_tool()
    raw = work / "raw"
    if raw.exists():
        shutil.rmtree(raw)
    raw.mkdir(parents=True)
    diagnostics = work / "workload.diagnostics"
    diagnostics.unlink(missing_ok=True)
    process_env = dict(env)
    process_env["LLVM_PROFILE_FILE"] = str(raw / "%p-%m.profraw")
    started = time.monotonic()
    result = subprocess.run([str(compiler), "--check", "--interfaces", str(manifest),
                             str(workload), str(diagnostics)],
                            cwd=work, env=process_env,
                            stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
    seconds = time.monotonic() - started
    output = result.stdout.decode(errors="replace")
    with log.open("ab") as stream:
        stream.write(f"pgo: training run exit {result.returncode} in {seconds:.1f} s\n".encode())
        if output.strip():
            stream.write(output.encode() + b"\n")
    if result.returncode != 0:
        raise ProfileError(f"the instrumented compiler failed the training workload "
                           f"(exit {result.returncode}): {output.strip()[-2000:]}")
    if diagnostics.is_file() and diagnostics.read_bytes():
        raise ProfileError(f"the training workload produced diagnostics: {diagnostics}")
    raws = sorted(raw.glob("*.profraw"))
    if not raws:
        raise ProfileError("the instrumented compiler wrote no profile counters")
    merged = work / "workload.profdata"
    merge = subprocess.run([tool, "merge", "-o", str(merged), *[str(path) for path in raws]],
                           stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
    if merge.returncode != 0 or not merged.is_file():
        raise ProfileError("llvm-profdata merge failed: "
                           + merge.stdout.decode(errors="replace").strip()[-2000:])
    return merged, seconds


def store(root: Path, source_sha256: str, family: str, workload_sha256: str,
          ir_sha256: str, profdata: Path, trainer: str, tool_version: str,
          instrumented_seconds: float | None, train_seconds: float) -> dict:
    """Install a freshly merged profile under the cache root, atomically.

    `instrumented_seconds` is None when the instrumented link happened inside
    a command that was not timed on its own (the seed's build of Stage1).
    """
    defined = {"schema": SCHEMA, "source_sha256": source_sha256, "family": family,
               "workload_sha256": workload_sha256, "ir_sha256": ir_sha256,
               "trainer": trainer, "profdata_tool": tool_version,
               "instrumented_link_seconds": (None if instrumented_seconds is None
                                             else round(instrumented_seconds, 1)),
               "train_seconds": round(train_seconds, 1)}
    root.mkdir(parents=True, exist_ok=True)
    path = profile_path(root, source_sha256, family)
    temporary = path.with_name(path.name + ".preparing")
    shutil.copyfile(profdata, temporary)
    temporary.replace(path)
    # A profile is keyed by the sources it was trained from, so an older one
    # in the same family can never be used again. One live profile per family.
    for stale in root.glob(f"*-{family}.profdata*"):
        if stale.name not in (path.name, path.with_suffix(".json").name):
            stale.unlink()
    record = dict(defined, profdata_sha256=file_digest(path), profdata_bytes=path.stat().st_size)
    record_path = path.with_suffix(".json")
    staged = record_path.with_name(record_path.name + ".preparing")
    staged.write_text(json.dumps(record, indent=2, sort_keys=True) + "\n")
    staged.replace(record_path)
    return record


def generate_flags(raw: Path) -> list[str]:
    """Compiler and linker flags for the instrumented link."""
    return [f"{GENERATE}={raw}"]


def use_flags(profile: dict, root: Path) -> list[str]:
    """Compiler and linker flags for the profile-use link."""
    path = profile_path(root, profile["source_sha256"], profile["family"])
    return [*USE_FLAGS, f"-fprofile-use={path}"]


def recorded_link(path: Path) -> tuple[list[str], Path]:
    """(the seed's own link arguments without its output, the directory it ran in).

    The profile-use relink must reproduce the seed's link exactly, so the
    wrapper records the real argv and the relink swaps only the output path
    and the profile flags.
    """
    try:
        entries = [json.loads(line) for line in path.read_text().splitlines() if line.strip()]
    except FileNotFoundError:
        raise ProfileError("the seed's link command was not recorded; PGO cannot relink")
    links = [entry for entry in entries
             if isinstance(entry, dict) and isinstance(entry.get("argv"), list)
             and any(not word.startswith("-") and word.endswith(".ll") for word in entry["argv"])
             and "-o" in entry["argv"]]
    if len(links) != 1:
        raise ProfileError(f"expected one recorded stage link, found {len(links)}")
    argv = []
    skip = False
    for word in links[0]["argv"]:
        if skip:
            skip = False
        elif word == "-o":
            skip = True
        elif word.startswith("-fprofile-"):
            continue
        else:
            argv.append(word)
    return argv, Path(links[0].get("cwd") or ".")
