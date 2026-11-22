#!/usr/bin/env python3
"""Exercise packed interfaces through the native and Rust compiler CLIs."""
import argparse
from copy import deepcopy
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

from pack_interfaces import BODY_ROOT, PackedInterfaces, pack, publish
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


def unpack_contract(prefix, module):
    package = prefix.with_name(prefix.name + ".cpk")
    [document] = [doc for name, _, doc in PackedInterfaces(package.read_bytes()).documents if name == module]
    return document


def native_interface_only_publication(stage, work, base):
    source = work / "InterfaceOnly.cinch"
    target = work / "interface-only"
    source.write_text("""module InterfaceOnly where
  export answer
  answer :: Int
  answer = 42
""")
    command = [stage / "cinch", "--emit-interfaces", "--interfaces", base, source, target]
    invoke(command, work, leak=True)
    document = unpack_contract(target, "InterfaceOnly")
    [value] = document["exports"]["values"]
    assert value["name"] == "answer"
    assert value["total_body"]["definitions"][0]["body"]["decimal"] == "42"
    assert not target.exists()
    output = target.with_suffix(".cpk")
    source.write_text("module InterfaceOnly where\n  answer :: Int\n  answer = False\n")
    invoke(command, work, status=4, leak=True)
    assert output.read_bytes() == b""


def body_definitions(document, identity):
    # The packed reader can share one definition among several root closures.
    return {id(row): row for value in document["exports"]["values"]
            for row in value["total_body"]["definitions"] if row["identity"] == identity}.values()


def native_indexed_body_nodes(stage, work, package, rows):
    source = work / "Probe.cinch"
    source.write_text("""module PackedProbe where
  export branch, wrapper
  branch :: Nat -> Nat
  branch n = let small = n < 128 in if small then 1 else 4
  wrapper :: Nat -> Nat
  wrapper n = branch n
module Main where
  main :: Int
  main = 0
""")
    ir = work / "probe.ll"
    invoke([stage / "cinch", "--emit-ir-with-interfaces", "--interfaces", package,
            "--objects", stage / "base-objects.json", source, ir], work, leak=True)
    document = unpack_contract(ir, "PackedProbe")
    document["compiler"] = "producer metadata with a quote \" and a newline \n"
    document["source_hash"] = "producer metadata with a backslash \\"
    document["digest_probe"] = {
        "compiler": "".join(map(chr, range(32))) + '\\"π𐍈',
        "source_hash": "nested fields are part of the contract",
        "units": [False, None, True, -(2**64 - 1), 2**64 - 1, 0, 9, 10, 99, 100, -9, -10, -99, -100, [], {}],
    }
    for definition in body_definitions(document, "PackedProbe.branch"):
        # Keep the producer's meaning while exercising a literal Bool node
        # that optimizations can otherwise fold away.
        definition["body"] = {
            "node": "if", "condition": {"node": "bool", "value": True},
            "yes": definition["body"], "no": {"node": "number", "sort": "Nat", "decimal": "1"},
        }
    # Two export rows can point to the same checked graph. Each has its
    # own canonical digest position even when its closure is shared.
    branch = next(row for row in document["exports"]["values"] if row["name"] == "branch")
    document["exports"]["values"].append({**deepcopy(branch), "name": "branch-alias"})
    document["abi"]["symbols"].append("branch-alias=PackedProbe.branch")
    digest = interface_hash(document)
    data = pack([*rows, ("PackedProbe", digest, canonical(document).encode())])
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

    # Both imported declarations return Nat. The client can prove its
    # refinement only by evaluating the admitted body and following its call.
    changed = deepcopy(document)
    for definition in body_definitions(changed, "PackedProbe.branch"):
        definition["body"]["yes"]["body"]["no"]["decimal"] = "9"
    changed_digest = interface_hash(changed)
    publish(package, pack([*rows, ("PackedProbe", changed_digest, canonical(changed).encode())]))
    invoke(command, work, status=4, leak=True)
    assert "undischarged refinement obligation" in output.with_suffix(".diagnostics").read_text()

    document["digest_probe"]["source_hash"] = "changed nested field"
    publish(package, pack([*rows, ("PackedProbe", digest, canonical(document).encode())]))
    invoke(command, work, status=7, leak=True)
    assert "interface contract identity mismatch" in output.with_suffix(".diagnostics").read_text()

    let_node = next(offset for offset in range(nodes, nodes + size, 16) if words(data, offset, 1)[0] == 5)
    corrupt = bytearray(data)
    struct.pack_into("<I", corrupt, let_node + 8, words(data, let_node + 4, 1)[0])
    publish(package, corrupt)
    invoke(command, work, status=7, leak=True)
    assert "packed body child is not in postorder" in output.with_suffix(".diagnostics").read_text()

    string_table, _ = words(data, 16, 2)
    (string_count,) = words(data, string_table, 1)
    string_data = string_table + 4 + 8 * string_count

    def string_at(index):
        offset, length = words(data, string_table + 4 + 8 * index, 2)
        return data[string_data + offset:string_data + offset + length].decode()

    bodies, body_size = words(data, 40, 2)
    edges, _ = words(data, 48, 2)
    operands, _ = words(data, 64, 2)
    wrapper, row = next((index, offset) for index, offset in
                        enumerate(range(bodies, bodies + body_size, 32))
                        if string_at(words(data, offset, 1)[0]) == "PackedProbe.wrapper")
    root, first_edge, edge_count = words(data, row + 16, 3)
    call = nodes + 16 * root
    kind, callee, first_arg, argc = words(data, call, 4)
    assert kind == 4 and argc == edge_count == 1 and callee != wrapper
    arg = nodes + 16 * words(data, operands + 4 * first_arg, 1)[0]
    assert words(data, arg, 1)[0] == 0
    edge = edges + 4 * first_edge

    # Point the wrapper's contract at the callee's otherwise valid graph.
    # The body table still validates, but the export no longer owns its root.
    documents, length = words(data, 72, 2)
    key = next(index for index in range(string_count) if string_at(index) == "total_body")
    reference = struct.pack("<III", key, BODY_ROOT, wrapper)
    at = data.find(reference, documents, documents + length)
    assert at >= documents and data.find(reference, at + 1, documents + length) == -1
    corrupt = bytearray(data)
    struct.pack_into("<I", corrupt, at + 8, callee)
    publish(package, corrupt)
    invoke(command, work, status=7, leak=True)
    assert "malformed interface contract envelope" in output.with_suffix(".diagnostics").read_text()

    # At this point no document selects the wrapper's root. Its nodes
    # must still be validated, even though no JSON projection needs them.
    struct.pack_into("<I", corrupt, call, 0xffffffff)
    publish(package, corrupt)
    invoke(command, work, status=7, leak=True)
    assert "invalid packed body node" in output.with_suffix(".diagnostics").read_text()

    for updates, refusal in (
        ([(arg + 4, 0xffffffff)], "packed body local slot outside its scope"),
        ([(call + 12, 0)], "packed body argument arity mismatch"),
        ([(edge, wrapper)], "packed body dependencies disagree with its expression"),
        ([(call + 4, wrapper), (edge, wrapper)], "cyclic packed total-body graph"),
    ):
        corrupt = bytearray(data)
        for offset, value in updates:
            struct.pack_into("<I", corrupt, offset, value)
        publish(package, corrupt)
        invoke(command, work, status=7, leak=True)
        assert refusal in output.with_suffix(".diagnostics").read_text()


def verify_native_object_references(stage, work, package, rows, source, ir, expected):
    data = package.read_bytes()
    start, count = words(data, 80, 2)
    metadata = PackedInterfaces(data).metadata()
    original = json.loads((stage / "base-objects.json").read_bytes())
    configuration = work / "test-objects.json"
    command = [stage / "cinch", "--emit-ir-only", "--interfaces", package, "--objects",
               configuration, source, ir]

    def use_embedded_abi(path, contents):
        publish(configuration, canonical({**original, "metadata": str(path),
                                          "metadata_sha256": sha256(contents).hexdigest()}).encode())

    private = {row["module"] for row in metadata.get("private_interfaces", [])}
    public = next(row for row in metadata["interface_refs"] if row["module"] not in private)
    public["iface_hash"] = "wrong-interface-identity"
    mismatch = work / "mismatched-abi.cpk"
    bad_abi = pack(rows, metadata)
    publish(mismatch, bad_abi)
    use_embedded_abi(mismatch, bad_abi)
    invoke(command, work, status=7, leak=True)
    assert "runtime object checked interface identity mismatch" in ir.with_suffix(".ll.diagnostics").read_text()

    broken_path = work / "bad-embedded-abi.cpk"
    use_embedded_abi(package, data)
    invoke(command, work, leak=True)
    assert ir.read_bytes() == expected

    runtime = Path(original["object"]).with_name("libcinchruntime.a")
    run_directory = work / "packed-run"
    run_directory.mkdir()
    prefix = run_directory / "hello"
    run = invoke([stage / "cinch", "--interfaces", package, "--objects", configuration,
                  "run", runtime, prefix, source], run_directory, leak=True)
    assert run.stdout == "Hello, world!\n"
    assert prefix.with_suffix(".ll").read_bytes() == expected
    assert not (run_directory / "build").exists()

    use_embedded_abi(package, b"not the package bytes")
    invoke(command, work, status=7, leak=True)
    assert "bootstrap.object: runtime object artifact digest mismatch" in ir.with_suffix(".ll.diagnostics").read_text()

    for at, value, phase, diagnostic in (
        (84, count + 1, "interfaces", "invalid interface package section directory"),
        (start, 0xffffffff, "interfaces", "invalid structured ABI string boundary"),
        (start + 4, 255, "object", "unknown structured contract tag"),
        (start + 12, 0xffffffff, "object", "field key outside package string table"),
    ):
        broken = bytearray(data)
        struct.pack_into("<I", broken, at, value)
        publish(broken_path, broken)
        use_embedded_abi(broken_path, broken)
        invoke(command, work, status=7, leak=True)
        assert diagnostic in ir.with_suffix(".ll.diagnostics").read_text()
        invoke([*command[:3], broken_path, *command[4:]], work, status=7, leak=True)
        assert f"bootstrap.{phase}: {diagnostic}" in ir.with_suffix(".ll.diagnostics").read_text()


def native_shared_body_ranges(stage, work, package, rows):
    source = work / "SharedBodies.cinch"
    names = [f"step{i:02d}" for i in range(48)]
    declarations = ["module SharedBodies where", "  export " + ", ".join(names)]
    for index, name in enumerate(names):
        # Most roots share one prefix. Every fourth step skips its immediate
        # predecessor, which also tests disjoint spans in the body index.
        previous = index - (2 if index and index % 4 == 0 else 1)
        body = "1" if not index else f"{names[previous]} n"
        declarations += [f"  {name} :: Nat -> Nat", f"  {name} n = {body}"]
    declarations += ["module Main where", "  main :: Int", "  main = 0"]
    source.write_text("\n".join(declarations) + "\n")
    ir = work / "shared.ll"
    invoke([stage / "cinch", "--emit-ir-with-interfaces", "--interfaces", package,
            "--objects", stage / "base-objects.json", source, ir], work, leak=True)
    document = unpack_contract(ir, "SharedBodies")
    graphs = [row["total_body"]["definitions"] for row in document["exports"]["values"]]
    assert sum(map(len, graphs)) > 800
    digest = interface_hash(document)
    package = work / "shared.cpk"
    output = work / "shared-check"
    main = work / "RangesMain.cinch"
    main.write_text("module Main where\n  import SharedBodies (step47)\n  main :: Int\n  main = 0\n")
    command = [stage / "cinch", "--check-types", "--interfaces", package, main, output]

    def publish_contract():
        publish(package, pack([*rows, ("SharedBodies", digest, canonical(document).encode())]))

    publish_contract()
    invoke(command, work, leak=True)
    assert not output.with_suffix(".diagnostics").read_bytes()

    # Change the numeric leaf everywhere it is shared, retaining its
    # type and edges. The original public contract digest must fail.
    for graph in graphs:
        leaf = next((row for row in graph if row["identity"] == "SharedBodies.step00"), None)
        if leaf is not None:
            leaf["body"]["decimal"] = "2"
    publish_contract()
    invoke(command, work, status=7, leak=True)
    assert "interface contract identity mismatch" in output.with_suffix(".diagnostics").read_text()


def native_packed_object_publication(stage, work, package, rows):
    source = work / "Library.cinch"
    source.write_text("""module PackedLibrary where
  export double, increment
  double :: Int -> Int
  double x = x + x
  increment :: Nat -> Nat
  increment x = helper x
  helper :: Nat -> Nat
  helper x = x + 1
""")
    ir, library = work / "library.ll", work / "library.o"
    delta, combined = work / "library-delta.cpk", work / "built-library.cpk"
    config = stage / "base-objects.json"
    command = [stage / "cinch", "--emit-object", "--interfaces", package, "--objects", config, source, ir]
    invoke([*command, "-", delta], work, leak=True)
    module = PackedInterfaces(delta.read_bytes())
    [(name, digest, document)] = module.documents
    assert name == "PackedLibrary"
    assert interface_hash(document) == digest
    assert set(module.metadata()) == {"facts", "units"}
    body = next(row["total_body"] for row in document["exports"]["values"] if row["name"] == "increment")
    definitions = {row["identity"]: row["body"] for row in body["definitions"]}
    assert definitions["PackedLibrary.increment"]["target"] == "PackedLibrary.helper"
    assert definitions["PackedLibrary.helper"]["arguments"][1]["decimal"] == "1"

    invoke([*command, library, combined], work, leak=True)
    data = combined.read_bytes()
    merged = PackedInterfaces(data)
    assert "interfaces" not in merged.metadata()
    assert [row["module"] for row in merged.metadata()["private_interfaces"]] == ["CinchBaseObject"]
    assert "CinchBaseObject" not in {name for name, _, _ in merged.documents}
    assert merged.documents[0] == (name, digest, document)
    assert {name for name, _, _ in merged.documents[1:]} == {name for name, _, _ in rows}
    assert not list(work.glob("library.*.cinchif"))
    settings = json.loads(config.read_bytes())
    runtime = Path(settings["object"]).with_name("libcinchruntime.a")
    settings.update(metadata=str(combined), metadata_sha256=sha256(data).hexdigest(),
                    object=str(library), object_sha256=sha256(library.read_bytes()).hexdigest())
    config = work / "built-library-objects.json"
    publish(config, canonical(settings).encode())
    client = work / "PackedLibraryClient.cinch"
    client.write_text("""module Main where
  import PackedLibrary (double)
  main :: Int
  main = double 21 - 42
""")
    invoke([stage / "cinch", "--interfaces", combined, "--objects", config,
            "run", runtime, work / "linked-library-client", client], work, leak=True)


def native(stage, work, repeats):
    package = stage / "base.cpk"
    contents = package.read_bytes()
    packed = PackedInterfaces(contents)
    rows = [(name, identity, canonical(doc).encode()) for name, identity, doc in packed.documents]
    indexed = work / "repacked.cpk"
    indexed_contents = pack(rows, packed.full_metadata())
    publish(indexed, indexed_contents)
    for name in ("base-interfaces.json", "objects.json"):
        assert not (stage / name).exists(), f"stage wrote a redundant interface artifact: {name}"
    hello = work / "Hello.cinch"
    hello.write_text(HELLO)
    output = work / "hello"
    diagnostics = work / "hello.diagnostics"

    def command(source, interfaces):
        return [stage / "cinch", "--check-types", "--interfaces", interfaces, source, output]

    cases = {"produced": command(hello, package), "repacked": command(hello, indexed)}
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
    for interfaces in (package, indexed):
        invoke(command(bad, interfaces), work, status=4, leak=True)
        refusals.append(diagnostics.read_bytes())
    assert refusals[0] and all(refusal == refusals[0] for refusal in refusals)

    modules, _ = words(contents, 24, 2)
    documents, _ = words(contents, 72, 2)
    root, _ = words(contents, modules + 24, 2)
    corruptions = (("table", 24, len(contents)), ("string-table", 88, len(contents)),
                   ("row", modules + 32, len(rows)), ("tag", documents + root, 255))
    for name, offset, value in corruptions:
        altered = bytearray(contents)
        struct.pack_into("<I", altered, offset, value)
        path = work / f"bad-{name}.cpk"
        publish(path, altered)
        invoke(command(hello, path), work, status=7, leak=True)
        assert "bootstrap.interfaces:" in diagnostics.read_text()

    old_package = work / "old-version.cpk"
    publish(old_package, b"CINCHPK3" + contents[8:12] + struct.pack("<I", 3) + contents[16:])
    invoke(command(hello, old_package), work, status=7, leak=True)
    assert "invalid interface package header" in diagnostics.read_text()

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
    publish(mismatched, pack([(name, "not-the-identity", raw), *rows[1:]]))
    invoke(command(hello, mismatched), work, status=7, leak=True)
    assert "interface contract identity mismatch" in diagnostics.read_text()

    ir = work / "hello.ll"
    provider = stage / "base-objects.json"
    for interfaces in (package, indexed):
        invoke([stage / "cinch", "--emit-ir-only", "--interfaces", interfaces, "--objects",
                provider, hello, ir], work, leak=True)
        if interfaces == package:
            produced_ir = ir.read_bytes()
        else:
            assert ir.read_bytes() == produced_ir

    reexport = work / "Reexport.cinch"
    reexport.write_text("""module Reexport where
  import Data.String (utf8-encode-width as width)
  export width
module Main where
  main :: Int
  main = 0
""")
    output = work / "reexport.ll"
    exported = output.with_name(output.name + ".cpk")
    for interfaces in (package, indexed):
        invoke([stage / "cinch", "--emit-ir-with-interfaces", "--interfaces", interfaces,
                "--objects", provider, reexport, output], work, leak=True)
        emitted = exported.read_bytes()
        if interfaces == package:
            produced_export = emitted
            value, = unpack_contract(output, "Reexport")["exports"]["values"]
            assert value["total_body"]["definitions"]
        else:
            assert emitted == produced_export

    native_interface_only_publication(stage, work, package)
    native_indexed_body_nodes(stage, work, package, rows)
    native_shared_body_ranges(stage, work, package, rows)
    verify_native_object_references(stage, work, package, rows, hello, ir, produced_ir)
    native_packed_object_publication(stage, work, package, rows)

    assert not list(work.rglob("*.cinchif"))
    times = {name: [] for name in cases}
    order = list(cases)
    for i in range(repeats):
        start_index = i % len(order)
        for name in order[start_index:] + order[:start_index]:
            start = time.perf_counter_ns()
            invoke(cases[name], work)
            times[name].append((time.perf_counter_ns() - start) / 1e6)
    return {"package_bytes": len(contents), "repacked_bytes": len(indexed_contents),
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
    package = installed / "build/package.cpk"
    assert not list(package.parent.glob("*.cinchif"))
    contents = package.read_bytes()
    packed = PackedInterfaces(contents)
    execution = packed.metadata()
    assert execution["kind"] == "package-execution" and "interfaces" not in execution
    rows = [(name, digest, canonical(document)) for name, digest, document in packed.documents]
    executable = project / "client"
    command = [seed, "build", client, "-o", executable, "--package-object", artifact]
    invoke(command, project)
    invoke([executable], project, status=42)
    built = project / "build/Main.cpk"
    document = unpack_contract(built.with_suffix(""), "Main")
    assert document["module"] == "Main"
    assert any(value["name"] == "main" for value in document["exports"]["values"])
    assert not list(built.parent.glob("*.cinchif"))
    assert not list(built.parent.glob("*.cinchopt"))
    name, _, raw = rows[0]
    publish(package, pack([(name, "not-the-identity", raw), *rows[1:]], execution))
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
    abi, _ = words(contents, 80, 2)
    for offset, value, error in (
        (body_rows + 16, node_size // 16, "packed body node outside its section"),
        (nodes, 255, "invalid packed body node"),
        (reference + 8, body_size // 32, "packed body definition outside its section"),
        (abi, 0xffffffff, "invalid structured ABI string boundary"),
    ):
        altered = bytearray(contents)
        struct.pack_into("<I", altered, offset, value)
        publish(package, altered)
        result = invoke(command, project, status=1)
        assert error in result.stderr, result.stderr

    publish(package, contents)
    invoke(command, project)
    invoke([executable], project, status=42)
    publish(package, b"CINCHPK3" + contents[8:12] + struct.pack("<I", 3) + contents[16:])
    result = invoke(command, project, status=1)
    assert "invalid interface package header" in result.stderr
    package.unlink()
    result = invoke(command, project, status=1)
    assert "package.cpk" in result.stderr


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
