#!/usr/bin/env python3
"""Exercise packed interfaces through the native and Rust compiler CLIs."""
import argparse
from hashlib import sha256
import json
import os
from pathlib import Path
import re
import statistics
import struct
import subprocess
import sys
import tempfile
import time

from pack_interfaces import BODY_ROOT, pack, publish, read_abi, read_manifest
from bootstrap_interfaces import canonical, interface_hash

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


def native_indexed_body_nodes(stage, work, manifest, rows):
    source = work / "Probe.cinch"
    source.write_text("""module PackedProbe where
  import Algebra.Arithmetic (InRange)
  export branch, wrapper
  branch :: Nat -> InRange 1 4 Nat
  branch n = let small = n < 128 in if small then 1 else 4
  wrapper :: Nat -> InRange 1 4 Nat
  wrapper n = branch n
module Main where
  main :: Int
  main = 0
""")
    ir = work / "probe.ll"
    invoke([stage / "cinch", "--emit-ir-with-interfaces", "--interfaces", manifest,
            "--objects", stage / "objects.json", source, ir], work, leak=True)
    document = json.loads((work / "probe.ll.PackedProbe.cinchif").read_bytes())
    for export in document["exports"]["values"]:
        for definition in export["total_body"]["definitions"]:
            if definition["identity"] == "PackedProbe.branch":
                # Keep the producer's meaning while exercising a literal Bool
                # node that its optimizations can otherwise fold away.
                definition["body"] = {
                    "node": "if", "condition": {"node": "bool", "value": True},
                    "yes": definition["body"], "no": {"node": "number", "sort": "Nat", "decimal": "1"},
                }
    digest = interface_hash(document)
    data = pack([*rows, ("PackedProbe", digest, canonical(document).encode())], flat_bodies=True)
    nodes, size = words(data, 56, 2)
    tags = {words(data, offset, 1)[0] for offset in range(nodes, nodes + size, 16)}
    assert tags == set(range(7))
    package = work / "all-body-nodes.cpk"
    publish(package, data)
    source.write_text("""module Main where
  import Algebra.Arithmetic (InRange)
  import PackedProbe (wrapper)
  checked :: Nat -> InRange 1 4 Nat
  checked n = wrapper n
  main :: Int
  main = 0
""")
    output = work / "packed-check"
    command = [stage / "cinch", "--check-types", "--interfaces", package, source, output]
    invoke(command, work, leak=True)
    assert not output.with_suffix(".diagnostics").read_bytes()

    let_node = next(offset for offset in range(nodes, nodes + size, 16) if words(data, offset, 1)[0] == 5)
    corrupt = bytearray(data)
    struct.pack_into("<I", corrupt, let_node + 8, words(data, let_node + 4, 1)[0])
    publish(package, corrupt)
    invoke(command, work, status=7, leak=True)
    assert "packed body child is not in postorder" in output.with_suffix(".diagnostics").read_text()


def verify_native_object_references(stage, work, package, source, ir, expected):
    data = package.read_bytes()
    start, count = words(data, 80, 2)
    metadata = json.loads(data[start:start + count])
    assert metadata["interface_refs"]
    original = json.loads((stage / "objects.json").read_bytes())
    metadata_path = work / "compact-abi.json"
    configuration = work / "compact-objects.json"
    command = [stage / "cinch", "--emit-ir-only", "--interfaces", package, "--objects",
               configuration, source, ir]

    def publish_config():
        contents = canonical(metadata).encode()
        publish(metadata_path, contents)
        publish(configuration, canonical({**original, "metadata": str(metadata_path),
                                          "metadata_sha256": sha256(contents).hexdigest()}).encode())

    publish_config()
    invoke(command, work, leak=True)
    assert ir.read_bytes() == expected
    public = next(row for row in metadata["interface_refs"]
                  if row["module"] not in {private["module"] for private in metadata.get("private_interfaces", [])})
    public["iface_hash"] = "wrong-interface-identity"
    publish_config()
    invoke(command, work, status=7, leak=True)
    assert "runtime object checked interface identity mismatch" in ir.with_suffix(".ll.diagnostics").read_text()


def native(stage, work, repeats):
    manifest = stage / "base-interfaces.json"
    rows = read_manifest(manifest)
    metadata_path, metadata = read_abi(stage / "objects.json")
    contents = pack(rows, metadata)
    package = work / "base.cpk"
    publish(package, contents)
    indexed = work / "base-indexed.cpk"
    invoke([sys.executable, Path(__file__).with_name("pack_interfaces.py"), manifest,
            indexed, "--objects", stage / "objects.json"], work)
    indexed_contents = indexed.read_bytes()
    assert indexed_contents.startswith(b"CINCHPK3")
    hello = work / "Hello.cinch"
    hello.write_text(HELLO)
    output = work / "hello"
    diagnostics = work / "hello.diagnostics"

    def command(source, interfaces):
        return [stage / "cinch", "--check-types", "--interfaces", interfaces, source, output]

    cases = {"json": command(hello, manifest), "mapped": command(hello, package),
             "indexed": command(hello, indexed)}
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
    for interfaces in (manifest, package, indexed):
        invoke(command(bad, interfaces), work, status=4, leak=True)
        refusals.append(diagnostics.read_bytes())
    assert refusals[0] and all(refusal == refusals[0] for refusal in refusals)

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

    bodies, _ = words(indexed_contents, 40, 2)
    _, edge_size = words(indexed_contents, 48, 2)
    nodes, node_size = words(indexed_contents, 56, 2)
    assert node_size > 0
    for name, offset, value, refusal in (
        ("node", nodes, 255, "invalid packed body node"),
        ("root", bodies + 16, node_size // 16, "packed body node outside its section"),
        ("edges", bodies + 20, edge_size // 4 + 1, "packed body span outside its section"),
    ):
        altered = bytearray(indexed_contents)
        struct.pack_into("<I", altered, offset, value)
        path = work / f"bad-indexed-{name}.cpk"
        publish(path, altered)
        invoke(command(hello, path), work, status=7, leak=True)
        assert refusal in diagnostics.read_text(), diagnostics.read_text()

    mismatched = work / "bad-identity.cpk"
    name, _, raw = rows[0]
    for flat in (False, True):
        publish(mismatched, pack([(name, "not-the-identity", raw), *rows[1:]], flat_bodies=flat))
        invoke(command(hello, mismatched), work, status=7, leak=True)
        assert "interface contract identity mismatch" in diagnostics.read_text()

    ir = work / "hello.ll"
    provider = stage / "objects.json"
    for interfaces in (manifest, package, indexed):
        invoke([stage / "cinch", "--emit-ir-only", "--interfaces", interfaces, "--objects",
                provider, hello, ir], work, leak=True)
        if interfaces == manifest:
            legacy_ir = ir.read_bytes()
        else:
            assert ir.read_bytes() == legacy_ir

    native_indexed_body_nodes(stage, work, manifest, rows)
    verify_native_object_references(stage, work, indexed, hello, ir, legacy_ir)

    times = {name: [] for name in cases}
    order = list(cases)
    for i in range(repeats):
        start_index = i % len(order)
        for name in order[start_index:] + order[:start_index]:
            start = time.perf_counter_ns()
            invoke(cases[name], work)
            times[name].append((time.perf_counter_ns() - start) / 1e6)
    return {"interfaces_and_abi_bytes": sum(len(raw) for _, _, raw in rows) + metadata_path.stat().st_size,
            "package_bytes": len(contents), "indexed_package_bytes": len(indexed_contents),
            "samples_ms": times,
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
    executable = project / "client"
    command = [seed, "build", client, "-o", executable, "--package-object", artifact]
    invoke(command, project)
    invoke([executable], project, status=42)
    package = installed / "build/package.cpk"
    name, _, raw = rows[0]
    for flat in (False, True):
        contents = pack(rows, execution, flat_bodies=flat)
        publish(package, contents)
        invoke(command, project)
        invoke([executable], project, status=42)

        publish(package, pack([(name, "not-the-identity", raw), *rows[1:]], flat_bodies=flat))
        result = invoke(command, project, status=1)
        assert "package interface identity mismatch" in result.stderr

    body_rows, body_size = words(contents, 40, 2)
    nodes, node_size = words(contents, 56, 2)
    assert body_size >= 32 and node_size >= 16, "fixture must include a checked body"
    strings, _ = words(contents, 16, 2)
    (string_count,) = words(contents, strings, 1)
    strings_start = strings + 4 + string_count * 8
    for key in range(string_count):
        at, length = words(contents, strings + 4 + key * 8, 2)
        if contents[strings_start + at:strings_start + at + length] == b"total_body":
            break
    else:
        raise AssertionError("fixture must include a total_body field")
    documents, document_size = words(contents, 72, 2)
    reference = contents.find(struct.pack("<III", key, BODY_ROOT, 0),
                              documents, documents + document_size)
    assert reference >= 0, "fixture must use an indexed body root"
    for offset, value, error in (
        (body_rows + 16, node_size // 16, "packed body node outside its section"),
        (nodes, 255, "invalid packed body node"),
        (reference + 8, body_size // 32, "packed body definition outside its section"),
    ):
        altered = bytearray(contents)
        struct.pack_into("<I", altered, offset, value)
        publish(package, altered)
        result = invoke(command, project, status=1)
        assert error in result.stderr, result.stderr

    # Installed package metadata can refer to the mapped contracts by name
    # and digest, leaving one copy in the packed artifact.
    abi_path = installed / "build/package.cinchif"
    execution["interface_refs"] = [
        {key: value for key, value in row.items() if key != "interface"}
        for row in execution.pop("interfaces")
    ]
    abi_path.write_text(json.dumps(execution))
    publish(package, contents)
    invoke(command, project)
    invoke([executable], project, status=42)
    package.unlink()
    result = invoke(command, project, status=1)
    assert "interface references require a single packed interface set" in result.stderr


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
            print("Rust package-object build, execution and refusal passed", flush=True)


if __name__ == "__main__":
    main()
