"""Build the pinned library dependency bundle, never compiler implementations."""
from pathlib import Path
import json
import os
import re
import shutil

from bootstrap_interfaces import content_hash

ENTRY = b"module CinchBaseObject where\n  export marker\n  marker :: Unit -> Unit\n  marker unit = unit\n"


def canonical(value):
    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=False).encode()


def defining_symbol(encoded):
    if not re.fullmatch(r"i\.(?:[0-9]+_)+", encoded):
        return None
    try:
        codes = [int(word) for word in encoded[2:-1].split("_")]
        if any(code > 0x10ffff or 0xd800 <= code <= 0xdfff or code < 32 for code in codes):
            return None
        symbol = "".join(chr(code) for code in codes)
        return symbol if "i." + "".join(str(ord(char)) + "_" for char in symbol) == encoded else None
    except (ValueError, OverflowError):
        return None


def requested_roots(diagnostics):
    """Extract only actual retained-Core runtime representation demands."""
    units, dictionaries, methods = set(), set(), set()
    prefix = r"^bootstrap\.runtime-ir:[0-9]+:[0-9]+: "
    missing = re.compile(prefix + r"checked (external|factory) variant is absent from the runtime object: (i\.[0-9_]+) \[([ROT]*)\]$")
    method = re.compile(prefix + r"method representation has no planned declaring-class slot: (i\.[0-9_]+):([0-9]+) \[([ROT]*)\]$")
    for line in diagnostics.splitlines():
        match = missing.fullmatch(line)
        if match:
            variety, encoded, classes = match.groups()
            symbol = defining_symbol(encoded)
            if symbol:
                (units if variety == "external" else dictionaries).add((symbol, classes))
        match = method.fullmatch(line)
        if match:
            encoded, slot, own = match.groups()
            symbol = defining_symbol(encoded)
            if symbol:
                methods.add((symbol, int(slot), own))
    return {"schema": 1,
            "units": [{"symbol": symbol, "classes": classes} for symbol, classes in sorted(units)],
            "dictionaries": [{"symbol": symbol, "classes": classes} for symbol, classes in sorted(dictionaries)],
            "methods": [{"class": symbol, "slot": slot, "own": own} for symbol, slot, own in sorted(methods)]}


def source_snapshot(driver, lock):
    snapshot = driver.BUILD / "library-sources" / lock["source_sha256"]
    if not snapshot.exists():
        archive = driver.native_source_archive(lock["revision"])
        temporary = snapshot.with_name(snapshot.name + ".preparing")
        if temporary.exists():
            shutil.rmtree(temporary)
        temporary.mkdir(parents=True)
        driver.extract_sources(archive, temporary)
        if driver.source_hash(driver.seed_sources(temporary)) != lock["source_sha256"]:
            raise driver.BootstrapError("library producer archive differs from its pinned source identity")
        temporary.rename(snapshot)
    if driver.source_hash(driver.seed_sources(snapshot)) != lock["source_sha256"]:
        raise driver.BootstrapError("the pinned library producer sources changed")
    return snapshot


def admitted_interfaces(driver, metadata, package, directory):
    """Admit only this pin's base package and the generated trivial root."""
    allowed = {row["name"] for row in package["modules"]} | {"Prelude.Generated"}
    if any(name == "Bootstrap" or name.startswith("Bootstrap.") for name in allowed):
        raise driver.BootstrapError("base package contains compiler implementation modules")
    closure = allowed | {"CinchBaseObject"}
    rows, names = [], set()
    for row in metadata.get("interfaces", []):
        name = row["module"]
        doc = row["interface"]
        if name not in closure or name in names or doc.get("module") != name:
            raise driver.BootstrapError("library object contains an unexpected or duplicate interface: " + name)
        names.add(name)
        if any(dep["module"] not in closure for dep in doc.get("imports", [])):
            raise driver.BootstrapError("library object depends on compiler implementation interfaces")
        path = directory / "library" / "build" / (name + ".cinchif")
        if not path.is_file() or driver.read_json(path) != doc:
            raise driver.BootstrapError("library object differs from its actual checked interface file: " + name)
        if name in allowed:
            rows.append([name, row["iface_hash"], str(path.resolve())])
    if not rows:
        raise driver.BootstrapError("library object has no checked base contracts")
    for unit in metadata.get("units", []):
        symbol = unit["symbol"]
        generated = (symbol.startswith(("<builtin>.", "cinch.prim.", "cinch.runtime."))
                     or re.fullmatch(r"Tuple\.[0-9]+(?:@[ROT]+)?", symbol) is not None)
        if not generated and not any(symbol.startswith(name + ".") for name in closure):
            raise driver.BootstrapError("library object contains a compiler or unknown implementation unit: " + symbol)
    return {"schema": 1, "interfaces": sorted(rows)}


def record_source_digests(driver, metadata, package, snapshot):
    """Bind the provider's checked homes to the exact pinned source bytes."""
    prelude = (snapshot / "compiler/prelude").resolve()
    paths = {row["name"]: row["path"] for row in package["modules"]}
    digests = {}
    for row in metadata["interfaces"]:
        name = row["module"]
        if name not in paths:
            continue
        path = (prelude / paths[name]).resolve()
        if not path.is_relative_to(prelude):
            raise driver.BootstrapError("library module source leaves the pinned package: " + name)
        source = path.read_bytes()
        if row["interface"].get("source_hash") != content_hash(source):
            raise driver.BootstrapError("library source disagrees with the emitted interface: " + name)
        digests[name] = driver.digest(source)
    metadata["source_sha256"] = digests


def cached(driver, directory, inputs):
    record = driver.read_json(directory / "provider.json")
    required = {"cinch", "library.o", "library.ll", "library.abi.json", "interfaces.json", "libcinchruntime.a"}
    artifacts = record.get("artifacts", {})
    return (record.get("status") == "built" and record.get("inputs") == inputs
            and os.access(directory / "cinch", os.X_OK)
            and required <= artifacts.keys()
            and all((directory / name).is_file() and driver.file_hash(directory / name) == expected
                    for name, expected in artifacts.items()))


def provider(driver):
    lock = driver.read_json(driver.ROOT / "bootstrap/library-provider.lock.json")
    if (type(lock.get("schema")) is not int or lock.get("schema") != 1 or type(lock.get("runtime_abi")) is not int
            or lock.get("runtime_abi") not in (5, 6)
            or not isinstance(lock.get("revision"), str)
            or not isinstance(lock.get("source_sha256"), str)
            or not re.fullmatch(r"[0-9a-f]{40}", lock["revision"])
            or not re.fullmatch(r"[0-9a-f]{64}", lock["source_sha256"])):
        raise driver.BootstrapError("unsupported or missing library producer lock")
    seed, _ = driver.seed_inputs()
    if lock["runtime_abi"] != seed["runtime_abi"]:
        raise driver.BootstrapError("library producer and pinned stage runtime ABIs differ; select matching inputs intentionally")
    roots = (driver.ROOT / "bootstrap/library-object-roots.json").read_bytes()
    inputs = {"protocol": 3, "host": driver.HOST, "producer": lock,
              "roots_sha256": driver.digest(roots), "entry_sha256": driver.digest(ENTRY)}
    snapshot = source_snapshot(driver, lock)
    directory = driver.BUILD / "library-provider" / driver.digest(canonical(inputs))
    if not cached(driver, directory, inputs):
        directory.mkdir(parents=True, exist_ok=True)
        log = directory / "build.log"
        toolchain = driver.digest(canonical({"producer": lock, "host": driver.HOST}))
        target = driver.BUILD / "library-cargo" / toolchain
        code = driver.run(["cargo", "build", "--locked", "--release", "--bin", "cinch", "--target-dir", target],
                          snapshot / "compiler", log)
        if code:
            raise driver.BootstrapError(f"library producer did not build (exit {code}): {log}")
        shutil.copy2(target / "release/cinch", directory / "cinch")
        runtime_target = driver.BUILD / "library-runtime" / toolchain
        code = driver.run(["cargo", "build", "--locked", "--release", "--manifest-path",
                           snapshot / "compiler/runtime/Cargo.toml", "--target-dir", runtime_target],
                          snapshot / "compiler", log, append=True)
        if code:
            raise driver.BootstrapError(f"library runtime did not build (exit {code}): {log}")
        shutil.copy2(runtime_target / "release/libcinchruntime.a", directory / "libcinchruntime.a")
        library = directory / "library"
        library.mkdir(exist_ok=True)
        source = library / "CinchBaseObject.cinch"
        source.write_bytes(ENTRY)
        requested = directory / "roots.json"
        requested.write_bytes(roots)
        code = driver.run([directory / "cinch", "emit-object", source, "--object-roots", requested,
                           "-o", directory / "library.o"], library, log, append=True)
        if code:
            raise driver.BootstrapError(f"library object did not emit (exit {code}): {log}")
        metadata = driver.read_json(directory / "library.abi.json")
        if (metadata.get("schema") != 1 or metadata.get("runtime_abi") != lock["runtime_abi"]
                or metadata.get("word_size") != 64 or metadata.get("endian") != "little"
                or metadata.get("reclaim") is not True or metadata.get("escaped_descriptors") is not False):
            raise driver.BootstrapError("library object has an incompatible runtime/descriptor ABI")
        package = driver.read_json(snapshot / "compiler/prelude/cinch-base.cinchpkg")
        interfaces = admitted_interfaces(driver, metadata, package, directory)
        record_source_digests(driver, metadata, package, snapshot)
        driver.write_json(directory / "library.abi.json", metadata)
        driver.write_json(directory / "interfaces.json", interfaces)
        artifacts = {name: driver.file_hash(directory / name) for name in
                     ("cinch", "library.o", "library.ll", "library.abi.json", "interfaces.json", "libcinchruntime.a")}
        artifacts.update({str(p.relative_to(directory)): driver.file_hash(p)
                          for p in library.rglob("*.cinchif")})
        driver.write_json(directory / "provider.json", {"status": "built", "inputs": inputs, "artifacts": artifacts})
    metadata = driver.read_json(directory / "library.abi.json")
    configuration = {"schema": 1, "target": metadata["target"],
                     "object": str((directory / "library.o").resolve()),
                     "metadata": str((directory / "library.abi.json").resolve()),
                     "object_sha256": driver.file_hash(directory / "library.o"),
                     "metadata_sha256": driver.file_hash(directory / "library.abi.json")}
    identity = {**inputs, "artifacts": driver.read_json(directory / "provider.json")["artifacts"]}
    return {"runtime": directory / "libcinchruntime.a", "object": directory / "library.o",
            "interfaces": driver.read_json(directory / "interfaces.json"),
            "configuration": configuration, "identity": identity}
