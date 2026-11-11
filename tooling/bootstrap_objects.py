"""Build the pinned library dependency bundle, never compiler implementations."""
from pathlib import Path
import json
import os
import re
import shutil


ENTRY = b"module CinchBaseObject where\n  export marker\n  marker :: Unit -> Unit\n  marker unit = unit\n"


def canonical(value):
    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=False).encode()


PLAIN_IDENTITY_CHARS = frozenset(
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789_")
HEX_PAIR = re.compile(r"[0-9a-f]{2}")


def identity_encoding(symbol):
    """The one canonical spelling of a defining identity."""
    out = []
    for character in symbol:
        if character in PLAIN_IDENTITY_CHARS:
            out.append(character)
        else:
            out.extend("$%02x" % byte for byte in character.encode("utf-8"))
    return "".join(out)


def defining_symbol(encoded):
    """The source identity an `i.` linkage spelling names, or `None`.

    The grammar is bootstrap/Bootstrap.Nominal.cinch's: `$` plus two lowercase
    hex digits is one byte of a UTF-8 scalar, a plain character is itself, and
    a spelling is canonical only when re-encoding the decoded identity
    reproduces it exactly.
    """
    if not re.fullmatch(r"i\.[A-Za-z0-9_$]+", encoded):
        return None
    body = encoded[2:]
    raw = bytearray()
    index = 0
    while index < len(body):
        character = body[index]
        if character == "$":
            pair = body[index + 1:index + 3]
            if not HEX_PAIR.fullmatch(pair):
                return None
            raw.append(int(pair, 16))
            index += 3
        else:
            raw.append(ord(character))
            index += 1
    try:
        symbol = raw.decode("utf-8")
    except UnicodeDecodeError:
        return None
    return symbol if identity_encoding(symbol) == body else None


def requested_roots(diagnostics):
    """Extract only actual retained-Core runtime representation demands."""
    units, dictionaries, methods = set(), set(), set()
    prefix = r"^bootstrap\.runtime-ir:[0-9]+:[0-9]+: "
    missing = re.compile(prefix + r"checked (external|factory) variant is absent from the runtime object: (i\.[A-Za-z0-9_$]+) \[([ROT]*)\]$")
    method = re.compile(prefix + r"method representation has no planned declaring-class slot: (i\.[A-Za-z0-9_$]+):([0-9]+) \[([ROT]*)\]$")
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
    lock = driver.library_provider_lock()
    roots = (driver.ROOT / "bootstrap/library-object-roots.json").read_bytes()
    inputs = {"protocol": 2, "host": driver.HOST, "producer": lock,
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
