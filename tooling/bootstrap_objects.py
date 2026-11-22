"""Build the pinned library dependency bundle, never compiler implementations."""
from pathlib import Path
import fcntl
import heapq
import json
import os
import re
import shutil
import subprocess


ENTRY = b"module CinchBaseObject where\n  export marker\n  marker :: Unit -> Unit\n  marker unit = unit\n"
ARTIFACT_ENTRY = (b"-- | Root a complete explicit base artifact without introducing an implementation override.\n"
                  b"module CinchBaseObject where\n  export marker\n"
                  b"  -- | Retain one closed executable artifact root.\n  marker :: Unit -> Unit\n  marker unit = unit\n")


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

    The grammar is compiler/Bootstrap.Nominal.cinch's: `$` plus two lowercase
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
        snapshot.parent.mkdir(parents=True, exist_ok=True)
        with snapshot.with_suffix(".lock").open("a") as guard:
            fcntl.flock(guard, fcntl.LOCK_EX)
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


def ordered_base_manifest(manifest, metadata):
    """Present the base in the same dependency order its native loader uses."""
    rows = manifest["interfaces"]
    positions = {row[0]: pos for pos, row in enumerate(rows)}
    if len(positions) != len(rows):
        return manifest
    counts = {}
    users = {name: [] for name in positions}
    for row in metadata.get("interfaces", []):
        name = row["module"]
        if name not in positions:
            continue
        imports = row["interface"].get("imports", [])
        counts[name] = len(imports)
        for dep in imports:
            home = dep["module"]
            if home not in users:
                return manifest
            users[home].append(name)
    if counts.keys() != positions.keys():
        return manifest
    ready = [positions[name] for name, count in counts.items() if not count]
    heapq.heapify(ready)
    ordered = []
    while ready:
        row = rows[heapq.heappop(ready)]
        ordered.append(row)
        for name in users[row[0]]:
            counts[name] -= 1
            if not counts[name]:
                heapq.heappush(ready, positions[name])
    if len(ordered) != len(rows):
        return manifest
    return {**manifest, "interfaces": ordered}


def cached(driver, directory, inputs):
    record = driver.read_json(directory / "provider.json")
    required = {"cinch", "library.o", "library.ll", "library.abi.json", "interfaces.json", "libcinchruntime.a"}
    artifacts = record.get("artifacts", {})
    return (record.get("status") == "built" and record.get("inputs") == inputs
            and os.access(directory / "cinch", os.X_OK)
            and required <= artifacts.keys()
            and all((directory / name).is_file() and driver.file_hash(directory / name) == expected
                    for name, expected in artifacts.items()))


def published_package(directory, metadata, artifacts, interfaces):
    from pack_interfaces import packed_manifest

    if "interface_package_sha256" not in metadata and "library.cpk" not in artifacts:
        return None
    digest = artifacts.get("library.cpk")
    if digest is None or metadata.get("interface_package_sha256") != digest:
        raise ValueError("packed interface package differs from its object ABI metadata")
    path = directory / "library.cpk"
    actual = packed_manifest(path.read_bytes())
    expected = [(name, identity) for name, identity, _ in interfaces["interfaces"]]
    if sorted(actual) != sorted(expected):
        raise ValueError("packed interface package does not name the checked base contracts")
    return path


def artifact_provider(driver, receipt_path):
    """Admit an explicitly selected, complete base artifact without substituting the pinned provider.

    Checked interfaces must belong to this exact object and retained source
    snapshot. Character/source hashes and ABI fields are verified independently
    of the receipt's artifact digests; no source-shadow module is introduced.
    """
    from bootstrap_interfaces import content_hash, interface_hash, validate_contract, verify_dependencies

    def refuse(message):
        """Name the selected artifact as the failing input instead of rebuilding another library."""
        raise driver.BootstrapError("explicit library artifact: " + message)

    def read_document(path):
        """Accept metadata formatting while rejecting duplicate keys and non-object documents."""
        def unique(pairs):
            """Keep duplicate input fields from choosing an implicit last-value policy."""
            result = {}
            for key, value in pairs:
                if key in result:
                    raise ValueError("duplicate artifact field: " + key)
                result[key] = value
            return result
        result = json.loads(path.read_text(), object_pairs_hook=unique)
        if not isinstance(result, dict):
            raise ValueError("artifact document is not an object")
        return result

    try:
        receipt_path = Path(receipt_path).resolve()
        directory = receipt_path.parent
        receipt = read_document(receipt_path)
        if type(receipt.get("schema")) is not int or receipt["schema"] != 1 or receipt.get("kind") != "current-base-artifact":
            refuse("unsupported receipt schema or kind")
        if receipt.get("host") != driver.HOST:
            refuse("different build host")
        architectures = {"x86_64": "x86_64", "aarch64": "aarch64", "arm64": "aarch64"}
        systems = {"Linux": "unknown-linux-gnu", "Darwin": "apple-darwin"}
        if driver.HOST["machine"] not in architectures or driver.HOST["system"] not in systems:
            refuse("unsupported native artifact host")
        target = architectures[driver.HOST["machine"]] + "-" + systems[driver.HOST["system"]]
        abi = driver.library_provider_lock()["runtime_abi"]
        if receipt.get("runtime_abi") != abi or receipt.get("target") != target:
            refuse("target or runtime ABI differs from this compiler")
        artifacts = receipt.get("artifacts", {})
        required = {"cinch", "library.o", "library.ll", "library.abi.json", "interfaces.json", "libcinchruntime.a", "base/cinch-base.cinchpkg", "roots.json", "library/CinchBaseObject.cinch"}
        if not isinstance(artifacts, dict) or not required <= artifacts.keys():
            refuse("required artifact bytes are missing from the receipt")
        for name, digest in artifacts.items():
            path = (directory / name).resolve()
            if not path.is_relative_to(directory) or not path.is_file() or not re.fullmatch(r"[0-9a-f]{64}", str(digest)):
                refuse("invalid artifact path or digest: " + name)
            if driver.file_hash(path) != digest:
                refuse("artifact digest mismatch: " + name)
        if not os.access(directory / "cinch", os.X_OK):
            refuse("producer compiler is not executable")
        metadata = read_document(directory / "library.abi.json")
        if (type(metadata.get("schema")) is not int or metadata["schema"] != 1 or metadata.get("runtime_abi") != abi
                or metadata.get("target") != target or metadata.get("word_size") != 64
                or metadata.get("endian") != "little" or metadata.get("reclaim") is not True
                or metadata.get("self_describing") is not True):
            refuse("incompatible object ABI or ownership descriptors")
        if metadata.get("object_content_hash") != content_hash((directory / "library.o").read_bytes()):
            refuse("object differs from its ABI metadata")
        if metadata.get("ir_content_hash") != content_hash((directory / "library.ll").read_bytes()):
            refuse("LLVM differs from its ABI metadata")
        runtime = receipt.get("runtime", {})
        if (runtime.get("runtime_abi") != abi or runtime.get("sha256") != artifacts["libcinchruntime.a"]
                or not re.fullmatch(r"[0-9a-f]{64}", runtime.get("source_sha256", ""))):
            refuse("runtime identity differs from the object")
        producer = receipt.get("producer", {})
        if (producer.get("mode") != "fresh-embedded" or producer.get("compiler_sha256") != artifacts["cinch"]
                or not re.fullmatch(r"[0-9a-f]{64}", producer.get("source_sha256", ""))):
            refuse("producer identity is incomplete or inconsistent")
        package = read_document(directory / "base/cinch-base.cinchpkg")
        if artifacts["base/cinch-base.cinchpkg"] != driver.file_hash(driver.seed_source_root(driver.ROOT) / "prelude/cinch-base.cinchpkg"):
            refuse("base package differs from current sources")
        modules = package["modules"]
        sources = receipt.get("sources", {})
        if len({row["name"] for row in modules}) != len(modules) or set(sources) != {row["name"] for row in modules}:
            refuse("base source manifest is incomplete or duplicated")
        interfaces = admitted_interfaces(driver, metadata, package, directory)
        if read_document(directory / "interfaces.json") != interfaces:
            refuse("interface manifest does not name this object's actual interfaces")
        packed = published_package(directory, metadata, artifacts, interfaces)
        documents = {}
        for row in metadata["interfaces"]:
            document = row["interface"]
            if "library/build/" + row["module"] + ".cinchif" not in artifacts:
                refuse("checked interface bytes are not retained: " + row["module"])
            validate_contract(document, abi)
            if interface_hash(document) != row["iface_hash"]:
                refuse("checked interface hash mismatch: " + row["module"])
            documents[row["module"]] = document
        verify_dependencies(documents, {})
        if not {"Data.String.string-repeat", "Data.String.string-concat-all"} <= {row["symbol"] for row in metadata["units"]}:
            refuse("String construction bodies are absent from the object")
        for module in modules:
            name = module["name"]
            relative = "base/" + module["path"]
            if relative not in artifacts or sources[name] != {"path": relative, "sha256": artifacts[relative]}:
                refuse("unretained base source: " + name)
            if artifacts[relative] != driver.file_hash(driver.seed_source_root(driver.ROOT) / "prelude" / module["path"]):
                refuse("base module differs from current sources: " + name)
            if name not in documents or documents[name].get("source_hash") != content_hash((directory / relative).read_bytes()):
                refuse("interface source hash mismatch: " + name)
        if ((directory / "library/CinchBaseObject.cinch").read_bytes() != ARTIFACT_ENTRY
                or documents.get("CinchBaseObject", {}).get("source_hash") != content_hash(ARTIFACT_ENTRY)):
            refuse("artifact root source differs from its checked interface")
        roots_sha256 = driver.file_hash(driver.ROOT / "compiler/pins/library-object-roots.json")
        if artifacts["roots.json"] != roots_sha256:
            refuse("requested object roots differ from this compiler")
        roots = read_document(directory / "roots.json")
        units = {row["symbol"] for row in metadata["units"]}
        for root in roots.get("units", []):
            variant = root["symbol"] + ("@" + root["classes"] if root["classes"] else "")
            if variant not in units:
                refuse("requested unit specialization is absent: " + variant)
        for root in roots.get("dictionaries", []):
            variant = root["symbol"] + ("@" + root["classes"] if root["classes"] else "")
            if variant not in metadata.get("dictionaries", {}):
                refuse("requested dictionary specialization is absent: " + variant)
        methods = {(family["class"], slot["logical"], slot["own"])
                   for family in metadata.get("method_families", []) for slot in family["slots"]}
        for root in roots.get("methods", []):
            if (root["class"], root["slot"], root["own"]) not in methods:
                refuse("requested method specialization is absent: " + root["class"])
        identity = {"kind": receipt["kind"], "receipt_sha256": driver.file_hash(receipt_path),
                    "host": driver.HOST, "producer": producer, "runtime": runtime,
                    "sources": sources, "roots_sha256": roots_sha256, "artifacts": artifacts}
        configuration = {"schema": 1, "target": target, "object": str(directory / "library.o"),
                         "metadata": str(directory / "library.abi.json"),
                         "object_sha256": artifacts["library.o"], "metadata_sha256": artifacts["library.abi.json"]}
        return {"runtime": directory / "libcinchruntime.a", "object": directory / "library.o",
                "interfaces": interfaces, "configuration": configuration, "identity": identity,
                "interface_package": packed}
    except driver.BootstrapError:
        raise
    except (OSError, ValueError, TypeError, KeyError, AttributeError) as error:
        refuse("invalid receipt or checked metadata: " + str(error))


def provider(driver):
    lock = driver.library_provider_lock()
    roots = (driver.ROOT / "compiler/pins/library-object-roots.json").read_bytes()
    inputs = {"protocol": 2, "host": driver.HOST, "producer": lock,
              "roots_sha256": driver.digest(roots), "entry_sha256": driver.digest(ENTRY)}
    snapshot = source_snapshot(driver, lock)
    directory = driver.BUILD / "library-provider" / driver.digest(canonical(inputs))
    directory.mkdir(parents=True, exist_ok=True)
    with (directory / ".build.lock").open("a") as guard:
        fcntl.flock(guard, fcntl.LOCK_SH)
        if not cached(driver, directory, inputs):
            fcntl.flock(guard, fcntl.LOCK_EX)
            if not cached(driver, directory, inputs):
                build_provider(driver, directory, inputs, snapshot, lock, roots)
        return provider_artifacts(driver, directory, inputs)


def build_provider(driver, directory, inputs, snapshot, lock, roots):
    log = directory / "build.log"
    toolchain = driver.digest(canonical({"producer": lock, "host": driver.HOST}))
    target = driver.BUILD / "library-cargo" / toolchain
    product = driver.prebuilt_compiler(lock["revision"])
    if product is not None:
        log.write_text(f"prebuilt pinned compiler: {product}\n")
    else:
        code = driver.run(["cargo", "build", "--locked", "--release", "--bin", "cinch", "--target-dir", target],
                          driver.seed_source_root(snapshot), log)
        if code:
            raise driver.BootstrapError(f"library producer did not build (exit {code}): {log}")
        product = target / "release/cinch"
    executable = directory / "cinch.new"
    shutil.copy2(product, executable)
    executable.replace(directory / "cinch")
    runtime_target = driver.BUILD / "library-runtime" / toolchain
    code = driver.run(["cargo", "build", "--locked", "--release", "--manifest-path",
                       driver.seed_source_root(snapshot) / "runtime/Cargo.toml", "--target-dir", runtime_target],
                      driver.seed_source_root(snapshot), log, append=True)
    if code:
        raise driver.BootstrapError(f"library runtime did not build (exit {code}): {log}")
    shutil.copy2(runtime_target / "release/libcinchruntime.a", directory / "libcinchruntime.a")
    library = directory / "library"
    library.mkdir(exist_ok=True)
    source = library / "CinchBaseObject.cinch"
    source.write_bytes(ENTRY)
    requested = directory / "roots.json"
    requested.write_bytes(roots)
    producer = directory / "cinch"
    usage = subprocess.run([producer, "--help"], env=driver.environment(), capture_output=True)
    packed_options = (["--interface-package", "--interface-package-private", "CinchBaseObject"]
                      if b"--interface-package-private" in usage.stdout + usage.stderr else [])
    code = driver.run([producer, "emit-object", source, "--object-roots", requested,
                       *packed_options, "-o", directory / "library.o"], library, log, append=True)
    if code:
        raise driver.BootstrapError(f"library object did not emit (exit {code}): {log}")
    metadata = driver.read_json(directory / "library.abi.json")
    if (metadata.get("schema") != 1 or metadata.get("runtime_abi") != lock["runtime_abi"]
            or metadata.get("word_size") != 64 or metadata.get("endian") != "little"
            or metadata.get("reclaim") is not True or metadata.get("self_describing") is not True):
        raise driver.BootstrapError("library object has an incompatible runtime/descriptor ABI")
    package = driver.read_json(snapshot / "compiler/prelude/cinch-base.cinchpkg")
    interfaces = admitted_interfaces(driver, metadata, package, directory)
    driver.write_json(directory / "interfaces.json", interfaces)
    artifacts = {name: driver.file_hash(directory / name) for name in
                 ("cinch", "library.o", "library.ll", "library.abi.json", "interfaces.json", "libcinchruntime.a")}
    if packed_options:
        artifacts["library.cpk"] = driver.file_hash(directory / "library.cpk")
        published_package(directory, metadata, artifacts, interfaces)
    artifacts.update({str(p.relative_to(directory)): driver.file_hash(p)
                      for p in library.rglob("*.cinchif")})
    driver.write_json(directory / "provider.json", {"status": "built", "inputs": inputs, "artifacts": artifacts})


def provider_artifacts(driver, directory, inputs):
    artifacts = driver.read_json(directory / "provider.json")["artifacts"]
    metadata = driver.read_json(directory / "library.abi.json")
    configuration = {"schema": 1, "target": metadata["target"],
                     "object": str((directory / "library.o").resolve()),
                     "metadata": str((directory / "library.abi.json").resolve()),
                     "object_sha256": artifacts["library.o"],
                     "metadata_sha256": artifacts["library.abi.json"]}
    identity = {**inputs, "artifacts": artifacts}
    # Leave the pinned artifact and its receipt unchanged on disk.
    manifest = driver.read_json(directory / "interfaces.json")
    try:
        package = published_package(directory, metadata, artifacts, manifest)
    except ValueError as error:
        raise driver.BootstrapError(f"pinned library interface package: {error}") from error
    return {"runtime": directory / "libcinchruntime.a", "object": directory / "library.o",
            "interfaces": ordered_base_manifest(manifest, metadata),
            "configuration": configuration, "identity": identity, "interface_package": package}
