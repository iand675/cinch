#!/usr/bin/env python3
"""Build a fresh embedded compiler and a complete, explicitly selected current-source base artifact.

This workflow does not alter the bootstrap pins. The producer consumes its own
embedded family; retained source snapshots and actual object interfaces must
agree before the receipt can be supplied to bootstrap --library-artifact.
"""
from __future__ import annotations

import argparse
from pathlib import Path
import shutil
import subprocess

import bootstrap
import bootstrap_objects
from bootstrap_interfaces import content_hash


def build(output: Path, cargo_target: Path) -> Path:
    """Compile the current embedded family and retain a self-contained ABI-matched artifact epoch."""
    output.mkdir(parents=True, exist_ok=False)
    env = bootstrap.environment()
    env["RUST_MIN_STACK"] = "67108864"
    source_files = bootstrap.seed_sources(bootstrap.ROOT)
    source_hash = bootstrap.source_hash(source_files)
    commands = []

    def execute(command: list[str], cwd: Path, log: Path) -> None:
        """Keep exact producer commands and stop immediately on the original build failure."""
        commands.append({"command": command, "cwd": str(cwd), "log": str(log)})
        with log.open("w") as stream:
            subprocess.run(command, cwd=cwd, env=env, stdout=stream, stderr=subprocess.STDOUT, check=True)

    execute(["cargo", "build", "--locked", "--release", "--bin", "cinch", "--target-dir", str(cargo_target)],
            bootstrap.seed_source_root(bootstrap.ROOT), output / "compiler-build.log")
    shutil.copy2(cargo_target / "release/cinch", output / "cinch")
    runtime_target = cargo_target / "runtime"
    execute(["cargo", "build", "--locked", "--release", "--manifest-path",
             str(bootstrap.seed_source_root(bootstrap.ROOT) / "runtime/Cargo.toml"), "--target-dir", str(runtime_target)],
            bootstrap.seed_source_root(bootstrap.ROOT), output / "runtime-build.log")
    shutil.copy2(runtime_target / "release/libcinchruntime.a", output / "libcinchruntime.a")
    package_path = bootstrap.seed_source_root(bootstrap.ROOT) / "prelude/cinch-base.cinchpkg"
    package = bootstrap.read_json(package_path)
    sources = {}
    for module in package["modules"]:
        relative = "base/" + module["path"]
        destination = output / relative
        destination.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(package_path.parent / module["path"], destination)
        sources[module["name"]] = {"path": relative, "sha256": bootstrap.file_hash(destination)}
    shutil.copy2(package_path, output / "base/cinch-base.cinchpkg")
    roots = output / "roots.json"
    shutil.copy2(bootstrap.ROOT / "compiler/pins/library-object-roots.json", roots)
    library = output / "library"
    library.mkdir()
    entry = library / "CinchBaseObject.cinch"
    entry.write_bytes(bootstrap_objects.ARTIFACT_ENTRY)
    execute([str(output / "cinch"), "emit-object", str(entry), "--object-roots", str(roots),
             "--interface-package", "--interface-package-private", "CinchBaseObject",
             "--opt-level", "2", "-o", str(output / "library.o")], library, output / "object-build.log")
    if bootstrap.source_hash(bootstrap.seed_sources(bootstrap.ROOT)) != source_hash:
        raise bootstrap.BootstrapError("compiler/base sources changed during the artifact build")
    metadata = bootstrap.read_json(output / "library.abi.json")
    interfaces = bootstrap_objects.admitted_interfaces(bootstrap, metadata, package, output)
    bootstrap.write_json(output / "interfaces.json", interfaces)
    retained = {"cinch", "libcinchruntime.a", "library.o", "library.ll", "library.abi.json", "library.cpk", "interfaces.json",
                "base/cinch-base.cinchpkg", "roots.json", "library/CinchBaseObject.cinch",
                "compiler-build.log", "runtime-build.log", "object-build.log"}
    retained.update(row["path"] for row in sources.values())
    retained.update(str(path.relative_to(output)) for path in (library / "build").glob("*.cinchif"))
    artifacts = {name: bootstrap.file_hash(output / name) for name in sorted(retained)}
    receipt = {"schema": 1, "kind": "current-base-artifact", "host": bootstrap.HOST,
               "target": metadata["target"], "runtime_abi": metadata["runtime_abi"],
               "producer": {"compiler_sha256": artifacts["cinch"], "source_sha256": source_hash,
                            "mode": "fresh-embedded", "commands": commands,
                            "builder_script_sha256": bootstrap.file_hash(Path(__file__))},
               "runtime": {"runtime_abi": metadata["runtime_abi"], "sha256": artifacts["libcinchruntime.a"],
                           "source_sha256": bootstrap.source_hash({name: value for name, value in source_files.items()
                                                                  if name.startswith("compiler/runtime/")})},
               "sources": sources, "artifacts": artifacts}
    path = output / "provider.json"
    bootstrap.write_json(path, receipt)
    bootstrap_objects.artifact_provider(bootstrap, path)
    bootstrap.write_json(output / "source-proof.json", {
        "compiler_source_sha256": source_hash, "string_sha256": sources["Data.String"]["sha256"],
        "string_source_hash": content_hash((output / sources["Data.String"]["path"]).read_bytes()),
        "provider_receipt_sha256": bootstrap.file_hash(path), "source_shadow_used": False})
    return path


def main() -> None:
    """Build only into explicitly selected lane-owned directories and print the admitted receipt path."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output", type=Path, required=True, help="new artifact directory; must not already exist")
    parser.add_argument("--cargo-target", type=Path, required=True, help="lane-owned Cargo output directory")
    args = parser.parse_args()
    print(build(args.output.resolve(), args.cargo_target.resolve()))


if __name__ == "__main__":
    main()
