#!/usr/bin/env python3
"""Exercise packed interfaces through the native and Rust compiler CLIs."""
import argparse
import json
import os
from pathlib import Path
import re
import statistics
import struct
import subprocess
import tempfile
import time

from pack_interfaces import pack, publish, read_abi, read_manifest

HELLO = """module Main where
  import Data.Path (path-from-string)
  import Effect.Fs (fs-write)
  main :: Unit ->{IO r} Int
  main unit = with (io = newIO ()) in
    case fs-write (path-from-string "/dev/stdout") (string-to-bytes "Hello, world!\\n") of
      Ok _ -> 0
      Err error -> 1
"""


def invoke(command, cwd, *, status=0, leak=False):
    environment = os.environ.copy()
    environment["CINCH_LEAK_CHECK"] = str(int(leak))
    environment["CINCH_ALLOC_STATS"] = str(int(leak))
    result = subprocess.run(command, cwd=cwd, env=environment, text=True, capture_output=True, timeout=180)
    if result.returncode != status:
        raise RuntimeError(f"{command!r} exited {result.returncode}\n{result.stdout}{result.stderr}")
    if leak:
        if "cinch-leak: live=0" not in result.stderr:
            raise RuntimeError(f"missing clean Cinch leak line\n{result.stderr}")
    return result


def words(data, offset, count):
    return struct.unpack_from(f"<{count}I", data, offset)


def native(stage, work, repeats):
    manifest = stage / "base-interfaces.json"
    rows = read_manifest(manifest)
    metadata_path, metadata = read_abi(stage / "objects.json")
    contents = pack(rows, metadata)
    package = work / "base.cpk"
    publish(package, contents)
    hello = work / "Hello.cinch"
    hello.write_text(HELLO)
    output = work / "hello"
    diagnostics = work / "hello.diagnostics"

    def command(source, interfaces):
        return [stage / "cinch", "--check-types", "--interfaces", interfaces, source, output]

    cases = {"json": command(hello, manifest), "mapped": command(hello, package)}
    allocations = {}
    for name, args in cases.items():
        result = invoke(args, work, leak=True)
        counter = re.search(r"^cinch-alloc: (.+)$", result.stderr, re.M)
        if counter is None:
            raise RuntimeError("missing native allocation counters")
        allocations[name] = counter[1]
        assert diagnostics.read_bytes() == b""

    bad = work / "Bad.cinch"
    bad.write_text(HELLO.replace('"/dev/stdout"', "123"))
    refusals = []
    for interfaces in (manifest, package):
        invoke(command(bad, interfaces), work, status=4, leak=True)
        refusals.append(diagnostics.read_bytes())
    assert refusals[0] and refusals[0] == refusals[1]

    modules, _ = words(contents, 24, 2)
    documents, _ = words(contents, 64, 2)
    root, _ = words(contents, modules + 24, 2)
    corruptions = (("table", 24, len(contents)), ("string-table", 80, len(contents)),
                   ("row", modules + 32, len(rows)), ("tag", documents + root, 255))
    for name, offset, value in corruptions:
        altered = bytearray(contents)
        struct.pack_into("<I", altered, offset, value)
        path = work / f"bad-{name}.cpk"
        publish(path, altered)
        invoke(command(hello, path), work, status=7, leak=True)
        assert "bootstrap.interfaces:" in diagnostics.read_text()

    mismatched = work / "bad-identity.cpk"
    name, _, raw = rows[0]
    publish(mismatched, pack([(name, "not-the-identity", raw), *rows[1:]]))
    invoke(command(hello, mismatched), work, status=7, leak=True)
    assert "interface contract identity mismatch" in diagnostics.read_text()

    ir = work / "hello.ll"
    provider = stage / "objects.json"
    for interfaces in (manifest, package):
        invoke([stage / "cinch", "--emit-ir-only", "--interfaces", interfaces, "--objects",
                provider, hello, ir], work, leak=True)
        if interfaces == manifest:
            legacy_ir = ir.read_bytes()
        else:
            assert ir.read_bytes() == legacy_ir

    times = {name: [] for name in cases}
    order = list(cases)
    for i in range(repeats):
        for name in order[i % 2:] + order[:i % 2]:
            start = time.perf_counter_ns()
            invoke(cases[name], work)
            times[name].append((time.perf_counter_ns() - start) / 1e6)
    return {"interfaces_and_abi_bytes": sum(len(raw) for _, _, raw in rows) + metadata_path.stat().st_size,
            "package_bytes": len(contents), "samples_ms": times,
            "median_ms": {name: statistics.median(values) for name, values in times.items()},
            "allocations": allocations}


def rust(seed, work):
    dependency = work / "dependency"
    project = work / "project"
    dependency.mkdir()
    project.mkdir()
    (dependency / "Tiny.cinch").write_text("module Tiny where\n  export answer\n  answer :: Int\n  answer = 42\n")
    (dependency / ".cinchpkg").write_text(json.dumps(
        {"schema": 2, "name": "test/tiny", "version": "0.1.0",
         "modules": [{"name": "Tiny", "path": "Tiny.cinch"}], "order": ["Tiny"]}))
    (project / ".cinchpkg").write_text(json.dumps(
        {"schema": 2, "name": "test/client", "version": "0.1.0",
         "modules": [{"name": "Main", "path": "Main.cinch"}], "order": ["Main"],
         "dependencies": [{"name": "test/tiny", "source": "path", "path": "../dependency"}]}))
    client = project / "Main.cinch"
    client.write_text("module Main where\n  import Tiny (answer)\n  main :: Int\n  main = answer\n")
    invoke([seed, "pkg", "install"], project)
    installed = project / "vendor/test/tiny/0.1.0"
    artifact = installed / "package.o"
    execution = json.loads((installed / "build/package.cinchif").read_bytes())
    metadata = execution["interfaces"]
    rows = [(row["module"], row["iface_hash"], json.dumps(row["interface"]).encode()) for row in metadata]
    command = [seed, "check", client, "--package-object", artifact]
    invoke(command, project)
    package = installed / "build/package.cpk"
    publish(package, pack(rows, execution))
    invoke(command, project)
    name, _, raw = rows[0]
    publish(package, pack([(name, "not-the-identity", raw), *rows[1:]]))
    result = invoke(command, project, status=1)
    assert "package interface identity mismatch" in result.stderr


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--native", type=Path, help="Stage3 directory containing cinch and its base provider")
    parser.add_argument("--seed", type=Path, help="current Rust compiler executable")
    parser.add_argument("--repeats", type=int, default=7)
    args = parser.parse_args()
    if not args.native and not args.seed:
        parser.error("provide --native, --seed, or both")
    if args.repeats < 1:
        parser.error("provide at least one timing repeat")
    with tempfile.TemporaryDirectory(prefix="cinch-package-") as scratch:
        work = Path(scratch)
        if args.native:
            print(json.dumps(native(args.native.resolve(), work, args.repeats), indent=2), flush=True)
        if args.seed:
            rust(args.seed.resolve(), work)
            print("Rust package-object compiler check passed", flush=True)


if __name__ == "__main__":
    main()
