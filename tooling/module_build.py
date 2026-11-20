#!/usr/bin/env python3
"""Build the compiler one module at a time, for iteration and not for audit.

usage: module_build.py --builder CINCH --out DIR [-j N] [--opt-level N] [--check]

The audited chain (tooling/bootstrap.py) flattens every compiler module into
one program and checks it whole, so the floor for any edit is the whole
bundle. This builds the same sources as a graph instead:

  * each module is compiled alone with the builder's `--emit-object`, against
    the checked interfaces of the modules it imports and nothing else;
  * a module's compile is keyed by its own source, its roots, the builder and
    the contracts of what it imports, so a module is recompiled only when one
    of those changed, and a body edit that changes no contract recompiles one
    module;
  * modules whose imports are ready compile side by side, and a module's
    dependents start as soon as its interface exists: its machine code is a
    separate job that only the link waits for;
  * a large module's IR is cut into shards by a stable partition
    (tooling/shard_ir.py), each compiled as its own job and kept by content;
  * the entry is compiled as an ordinary client of all of them and the
    objects are linked.

The result is a development compiler. Nothing here is a claim about a fixed
point: the modules are optimised separately, and the build trusts the keys it
computes. `--check` compares what the result emits for the audited bundle with
a reference module, which is the evidence that it is the same compiler.

How a module sees its imports
-----------------------------
`--emit-object` takes one provider: an object file and its ABI manifest. It
writes the module's own object beside a layer that is the provider with that
object merged in. Chaining layers would make every module depend on every
module before it. Instead this tool keeps, for each module, the delta its
compile added to the manifest (its units, constructors, dictionaries, method
families and interface), and assembles a module's provider from the base
library plus the deltas of the modules it transitively imports. That is the
merge `Bootstrap.Pipeline.object-manifest` performs, applied in import order.

The provider's object is an empty one. A compile reads the provider object
only to check its identity against the manifest and to copy it into the
layer; the module's code never depends on its bytes. The manifest this tool
writes names the empty object's hash, so the check is a true one.

Roots
-----
A provider emits a polymorphic export only in the representations it is asked
for. When a client needs one that is absent it says so, and the request is
added to the exporting module's roots and that module is rebuilt.
`bootstrap/module-object-roots.json` holds the requests already known, so a
build from a clean tree does not rediscover them.
"""
from __future__ import annotations

import argparse
import concurrent.futures
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import sys
import time

sys.path.insert(0, str(Path(__file__).resolve().parent))
import bootstrap  # noqa: E402
from bootstrap_interfaces import interface_hash, read_contract  # noqa: E402
from bootstrap_objects import requested_roots  # noqa: E402
import shard_ir  # noqa: E402

ROOT = Path(__file__).resolve().parents[1]
ROOTS_FILE = ROOT / "bootstrap" / "module-object-roots.json"
ENTRY = "Main"
IMPORT = re.compile(rb"^\s+import\s+([A-Z][\w.]*)", re.M)
# The manifest fields a module's compile adds to, and how each merges.
MAPS = ("constructors", "dictionaries")
# A module's IR above this size is cut into shards of about this size, each
# its own clang job. clang is one thread per translation unit and the largest
# module is 19 MB of IR, which at -O1 is most of a minute on one core.
SHARD_BYTES = 2 << 20


class BuildError(RuntimeError):
    pass


def sha(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def fnv_decimal(data: bytes) -> str:
    """`Bootstrap.Interface.fnv-source-hash`: FNV-1a over the bytes, in decimal."""
    value = 0xCBF29CE484222325
    for byte in data:
        value = ((value ^ byte) * 0x100000001B3) & 0xFFFFFFFFFFFFFFFF
    return str(value)


def canonical(value) -> bytes:
    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=False).encode()


# ------------------------------------------------------------------- the graph

def module_graph(files: dict[str, bytes]) -> tuple[dict[str, bytes], dict[str, list[str]], list[str]]:
    """Sources by module name, each module's direct imports, and a fixed order.

    The order lists a module after everything it imports and depends only on
    the sources, so two builds of one tree agree on it.
    """
    sources = {name[:-6]: data for name, data in files.items() if name.endswith(".cinch")}
    imports = {name: sorted({found.decode() for found in IMPORT.findall(data)} & sources.keys() - {name})
               for name, data in sources.items()}
    order: list[str] = []
    state: dict[str, int] = {}

    def visit(name: str, path: tuple[str, ...]) -> None:
        if state.get(name) == 2:
            return
        if state.get(name) == 1:
            raise BuildError("import cycle: " + " -> ".join(path + (name,)))
        state[name] = 1
        for dependency in imports[name]:
            visit(dependency, path + (name,))
        state[name] = 2
        order.append(name)

    for name in sorted(sources):
        visit(name, ())
    return sources, imports, order


def closure(imports: dict[str, list[str]], order: list[str]) -> dict[str, list[str]]:
    """Every module's transitive imports, in the fixed order."""
    position = {name: at for at, name in enumerate(order)}
    reached: dict[str, set[str]] = {}
    for name in order:
        found: set[str] = set()
        for dependency in imports[name]:
            found.add(dependency)
            found |= reached[dependency]
        reached[name] = found
    return {name: sorted(found, key=position.__getitem__) for name, found in reached.items()}


# ---------------------------------------------------------------- the manifest

class Base:
    """The pinned library a module's provider starts from."""

    def __init__(self, out: Path, clang: str):
        provider = bootstrap.runtime_provider()
        self.runtime = Path(provider["runtime"])
        self.object = Path(provider["object"])
        self.rows = {row[0]: row for row in provider["interfaces"]["interfaces"]}
        configuration = provider["configuration"]
        self.target = configuration["target"]
        metadata = Path(configuration["metadata"])
        self.manifest = json.loads(metadata.read_bytes())
        self.identity = sha(canonical([configuration["object_sha256"], configuration["metadata_sha256"],
                                       sorted(self.rows.values())]))
        # An interface the manifest embeds is, for the base set, a file the
        # interface manifest also names. Keeping the file's bytes lets a
        # provider embed them verbatim, which is what lets the compiler take
        # the interface it already loaded instead of parsing the copy.
        self.raw = {name: Path(row[2]).read_bytes() for name, row in self.rows.items()}
        self.stub = out / "provider-stub.o"
        if not self.stub.is_file():
            source = out / "provider-stub.ll"
            source.write_text(f'target triple = "{self.target}"\n')
            subprocess.run([clang, "-Wno-override-module", "-c", str(source), "-o", str(self.stub)], check=True,
                           stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
        self.stub_sha = bootstrap.file_hash(self.stub)
        self.stub_fnv = fnv_decimal(self.stub.read_bytes())


def merged_manifest(base: Base, deltas: list[dict]) -> dict:
    """The base manifest with each delta applied in order."""
    merged = dict(base.manifest)
    merged["units"] = list(base.manifest["units"])
    merged["interfaces"] = list(base.manifest["interfaces"])
    families = list(base.manifest["method_families"])
    for name in MAPS:
        merged[name] = dict(base.manifest[name])
    for delta in deltas:
        merged["units"].extend(delta["units"])
        for name in MAPS:
            merged[name].update(delta[name])
        replaced = {row["class"] for row in delta["method_families"]}
        families = delta["method_families"] + [row for row in families if row["class"] not in replaced]
        merged["interfaces"].insert(0, {"module": delta["module"], "iface_hash": delta["iface_hash"],
                                        "interface": {"$raw": delta["module"]}})
    merged["method_families"] = families
    merged["self_describing"] = True
    merged["object_content_hash"] = base.stub_fnv
    return merged


def manifest_bytes(base: Base, merged: dict, raw: dict[str, bytes]) -> bytes:
    """Serialise a manifest with every interface it can as the file's bytes."""
    parts = []
    for row in merged["interfaces"]:
        document = row["interface"]
        module = row["module"]
        if isinstance(document, dict) and set(document) == {"$raw"}:
            body = raw[document["$raw"]]
        elif module in base.raw and json.loads(base.raw[module]) == document:
            body = base.raw[module]
        else:
            body = canonical(document)
        rest = {key: value for key, value in row.items() if key != "interface"}
        parts.append(canonical(rest)[:-1] + b',"interface":' + body + b"}")
    shell = dict(merged, interfaces="$interfaces")
    return canonical(shell).replace(b'"$interfaces"', b"[" + b",".join(parts) + b"]", 1)


def delta_of(module: str, produced: dict, provider: dict, interface: Path) -> dict:
    """What one module's compile added to its provider's manifest."""
    units = produced["units"][len(provider["units"]):]
    if produced["units"][:len(provider["units"])] != provider["units"]:
        raise BuildError(f"{module}: the layer manifest does not extend its provider's units")
    delta = {"module": module, "iface_hash": interface_hash(read_contract(interface)), "units": units}
    for name in MAPS:
        delta[name] = {key: value for key, value in produced[name].items() if provider[name].get(key) != value}
    known = {canonical(row) for row in provider["method_families"]}
    delta["method_families"] = [row for row in produced["method_families"] if canonical(row) not in known]
    added = [row["module"] for row in produced["interfaces"]
             if row["module"] not in {other["module"] for other in provider["interfaces"]}]
    if added != [module]:
        raise BuildError(f"{module}: the layer adds interfaces {added}, expected only its own")
    return delta


# ------------------------------------------------------------------ one module

class Builder:
    def __init__(self, args, files: dict[str, bytes]):
        self.out = Path(args.out).resolve()
        self.out.mkdir(parents=True, exist_ok=True)
        self.compiler = Path(args.builder).resolve()
        self.clang = os.environ.get("CINCH_CLANG") or shutil.which("clang") or "clang"
        self.opt_level = args.opt_level
        self.keep = args.keep_inputs
        self.bare_modules = True
        self.jobs = args.jobs
        self.base = Base(self.out, self.clang)
        self.sources, self.imports, self.order = module_graph(files)
        self.closure = closure(self.imports, self.order)
        self.compiler_id = bootstrap.file_hash(self.compiler)
        self.roots = self.read_roots()
        self.deltas: dict[str, dict] = {}
        self.interfaces: dict[str, Path] = {}
        self.objects: dict[str, list[Path]] = {}
        self.pool = concurrent.futures.ThreadPoolExecutor(self.jobs)
        self.pending: dict[str, list[concurrent.futures.Future]] = {}
        self.cache = shard_ir.ObjectCache(self.out / "object-cache", self.clang)
        self.shards_compiled = 0
        self.shards_reused = 0
        self.compiled: list[tuple[str, float]] = []
        self.reused: list[str] = []
        self.environment = dict(bootstrap.environment(), PYTHONDONTWRITEBYTECODE="1",
                                PATH=str(Path(self.clang).resolve().parent) + os.pathsep + os.environ.get("PATH", ""))

    def read_roots(self) -> dict[str, list[dict]]:
        roots: dict[str, list[dict]] = {}
        for path in (ROOTS_FILE, self.out / "discovered-roots.json"):
            if path.is_file():
                for module, units in bootstrap.read_json(path).get("modules", {}).items():
                    for unit in units:
                        if unit not in roots.setdefault(module, []):
                            roots[module].append(unit)
        return roots

    def owner(self, symbol: str) -> str | None:
        """The module that defines a symbol: the longest module name it starts with."""
        found = [name for name in self.sources if symbol.startswith(name + ".")]
        return max(found, key=len) if found else None

    def directory(self, module: str) -> Path:
        path = self.out / "modules" / module
        path.mkdir(parents=True, exist_ok=True)
        return path

    def key(self, module: str) -> str:
        """What a module's compile is a function of.

        An import contributes its contract hash and the delta its compile
        added, not its interface file's bytes: the file also records the
        hash of the source it came from, which every edit changes and no
        dependent reads.
        """
        imported = [[name, self.deltas[name]["iface_hash"], sha(canonical(self.deltas[name]))]
                    for name in self.closure[module]]
        return sha(canonical([3, self.compiler_id, self.base.identity, self.opt_level, sha(self.sources[module]),
                              self.roots.get(module, []), imported]))

    def provider(self, module: str, directory: Path) -> tuple[dict, Path, Path]:
        deltas = [self.deltas[name] for name in self.closure[module]]
        merged = merged_manifest(self.base, deltas)
        raw = {name: self.interfaces[name].read_bytes() for name in self.closure[module]}
        metadata = directory / "provider.abi.json"
        metadata.write_bytes(manifest_bytes(self.base, merged, raw))
        # The manifest the compiler sees holds real documents again.
        for row in merged["interfaces"]:
            if isinstance(row["interface"], dict) and set(row["interface"]) == {"$raw"}:
                row["interface"] = json.loads(raw[row["interface"]["$raw"]])
        rows = dict(self.base.rows)
        for name in self.closure[module]:
            rows[name] = [name, self.deltas[name]["iface_hash"], str(self.interfaces[name])]
        manifest = directory / "interfaces.json"
        bootstrap.write_json(manifest, {"schema": 1, "interfaces": [rows[name] for name in sorted(rows)]})
        objects = directory / "objects.json"
        bootstrap.write_json(objects, {"schema": 1, "target": self.base.target, "object": str(self.base.stub),
                                       "metadata": str(metadata), "object_sha256": self.base.stub_sha,
                                       "metadata_sha256": bootstrap.file_hash(metadata)})
        return merged, manifest, objects

    def schedule_objects(self, module: str, directory: Path) -> list[Path]:
        """Queue the clang jobs for a module's IR and name the objects they write.

        The frontend's worker returns as soon as the jobs are queued, so the
        modules that import this one do not wait for its machine code.
        """
        for earlier in self.pending.pop(module, []):
            # A rebuild after a root request replaces objects still in flight.
            concurrent.futures.wait([earlier])
        ir = directory / "unit.ll"
        count = max(1, -(-ir.stat().st_size // SHARD_BYTES))
        home = directory / "objects"
        shutil.rmtree(home, ignore_errors=True)
        home.mkdir()
        if count == 1:
            shards = [ir]
        else:
            # One more than the code needs: a stable partition keeps the
            # module-level content in a shard of its own.
            # A module's workers are local to it, so they are promoted under
            # a tag of the module's own: the split would otherwise copy them
            # all into the shard that holds the public entry points.
            shards = shard_ir.write_shards(ir, home, count + 1, stem="unit", stable=True,
                                           promote="m" + sha(module.encode())[:12])
        objects = [home / f"unit.shard{k}.o" for k in range(len(shards))]
        self.pending[module] = [self.pool.submit(self.compile_object, shard, target, directory)
                                for shard, target in zip(shards, objects)]
        return objects

    def compile_object(self, shard: Path, target: Path, directory: Path) -> None:
        flags = [f"-O{self.opt_level}", "-Wno-override-module"]
        entry = self.cache.key(shard, flags)
        if entry.is_file():
            target.unlink(missing_ok=True)
            try:
                os.link(entry, target)
            except OSError:
                shutil.copyfile(entry, target)
            self.shards_reused += 1
            return
        result = subprocess.run([self.clang, *flags, "-c", str(shard), "-o", str(target)], cwd=directory,
                                stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
        if result.returncode:
            raise BuildError(f"clang refused {shard}:\n{result.stdout.decode(errors='replace')[-2000:]}")
        self.cache.store(entry, target)
        self.shards_compiled += 1

    def finish_objects(self) -> None:
        """Wait for every queued clang job; the link is the first thing that needs them."""
        for module, futures in list(self.pending.items()):
            for future in futures:
                future.result()
            (self.directory(module) / "objects.list.json").write_text(
                json.dumps([str(path) for path in self.objects[module]]))
        self.pending.clear()

    def build(self, module: str) -> tuple[str, str]:
        """Compile one module, or reuse it. Returns (status, detail)."""
        directory = self.directory(module)
        key = self.key(module)
        stamp = directory / "key"
        interface = directory / f"iface.{module}.cinchif"
        saved = directory / "delta.json"
        listed = directory / "objects.list.json"
        if stamp.is_file() and stamp.read_text() == key and interface.is_file() and saved.is_file() and listed.is_file():
            objects = [Path(path) for path in json.loads(listed.read_text())]
            if all(path.is_file() for path in objects):
                self.deltas[module] = bootstrap.read_json(saved)
                self.interfaces[module] = interface
                self.objects[module] = objects
                self.reused.append(module)
                return "reused", ""
        stamp.unlink(missing_ok=True)
        listed.unlink(missing_ok=True)
        source = directory / f"{module}.cinch"
        source.write_bytes(self.sources[module])
        provider, manifest, objects = self.provider(module, directory)
        command = [str(self.compiler), "--emit-object"]
        if self.roots.get(module):
            requested = directory / "roots.json"
            bootstrap.write_json(requested, {"schema": 1, "units": self.roots[module],
                                             "dictionaries": [], "methods": []})
            command += ["--object-roots", str(requested)]
        # `-` for the object: this build assembles the IR itself, so the
        # compiler makes neither an object nor a merged layer. A builder from
        # before that option was understood refuses the path, and is asked
        # again for the layer it knows how to make.
        command += ["--interfaces", str(manifest), "--objects", str(objects), source.name, "unit.ll"]
        tail = [["-", "layer.abi.json", "iface."], ["layer.o", "layer.abi.json", "iface."]]
        started = time.monotonic()
        diagnostics = directory / "unit.ll.diagnostics"
        for words in tail if self.bare_modules else tail[1:]:
            result = subprocess.run(command + words, cwd=directory, env=self.environment,
                                    stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
            text = diagnostics.read_text(errors="replace") if diagnostics.is_file() else ""
            if result.returncode == 0 or words[0] != "-" or "cannot read linked object" not in text:
                break
            # A builder that does not know `-` links to standard output and
            # then finds no object to read.
            self.bare_modules = False
        if result.returncode:
            return "refused", text or result.stdout.decode(errors="replace")
        self.deltas[module] = delta_of(module, bootstrap.read_json(directory / "layer.abi.json"), provider, interface)
        self.interfaces[module] = interface
        self.objects[module] = self.schedule_objects(module, directory)
        bootstrap.write_json(saved, self.deltas[module])
        # The layer and the provider copy are the provider again plus this
        # module; nothing reads them once the delta is taken.
        if not self.keep:
            for name in ("layer.o", "layer.o.provider.o", "layer.o.new.o", "-.provider.o", "-.new.o",
                         "layer.abi.json", "provider.abi.json"):
                (directory / name).unlink(missing_ok=True)
        stamp.write_text(key)
        seconds = time.monotonic() - started
        self.compiled.append((module, seconds))
        return "compiled", f"{seconds:.1f}"

    # ------------------------------------------------------------ the schedule

    def new_roots(self, text: str) -> list[tuple[str, dict]]:
        found = []
        for unit in requested_roots(text)["units"]:
            module = self.owner(unit["symbol"])
            if module is None or module == ENTRY:
                continue
            if unit not in self.roots.get(module, []):
                found.append((module, unit))
        return found

    def run(self) -> None:
        """Compile every module, in import order, as wide as the graph allows.

        A refusal that asks for a root restarts the walk: the exporting
        module's key has changed, so it and what imports it are compiled
        again and everything else is reused.
        """
        modules = [name for name in self.order if name != ENTRY]
        for attempt in range(64):
            requested = self.walk(modules)
            if not requested:
                return
            for module, unit in requested:
                self.roots.setdefault(module, []).append(unit)
            for units in self.roots.values():
                units.sort(key=lambda unit: (unit["symbol"], unit["classes"]))
            bootstrap.write_json(self.out / "discovered-roots.json", {"schema": 1, "modules": self.roots})
            print(f"roots: {len(requested)} requested of "
                  f"{', '.join(sorted({module for module, unit in requested}))}; rebuilding", flush=True)
        raise BuildError("root requests did not settle")

    def walk(self, modules: list[str]) -> list[tuple[str, dict]]:
        waiting = {name: set(self.imports[name]) - {ENTRY} for name in modules}
        dependents: dict[str, list[str]] = {name: [] for name in modules}
        for name, needs in waiting.items():
            for dependency in needs:
                dependents[dependency].append(name)
        requested: list[tuple[str, dict]] = []
        failure: list[str] = []
        pool = self.pool
        running = {pool.submit(self.build, name): name for name in modules if not waiting[name]}
        while running:
            done, _ = concurrent.futures.wait(running, return_when=concurrent.futures.FIRST_COMPLETED)
            for future in done:
                name = running.pop(future)
                status, detail = future.result()
                if status == "refused":
                    wanted = self.new_roots(detail)
                    if wanted:
                        requested.extend(item for item in wanted if item not in requested)
                    else:
                        failure.append(f"{name}:\n{detail[:3000]}")
                    continue
                if status == "compiled":
                    print(f"{name}: checked and emitted in {detail} s", flush=True)
                for dependent in dependents[name]:
                    waiting[dependent].discard(name)
                    if not waiting[dependent] and not failure:
                        running[pool.submit(self.build, dependent)] = dependent
        if failure:
            raise BuildError("\n".join(failure))
        return requested

    # --------------------------------------------------------------- the entry

    def link(self, output: Path) -> None:
        modules = [name for name in self.order if name != ENTRY]
        directory = self.directory(ENTRY)
        self.finish_objects()
        key = sha(canonical([3, self.compiler_id, self.base.identity, self.opt_level, sha(self.sources[ENTRY]),
                             [[name, [bootstrap.file_hash(path) for path in self.objects[name]],
                               sha(canonical(self.deltas[name]))] for name in modules]]))
        stamp = directory / "key"
        if stamp.is_file() and stamp.read_text() == key and output.is_file():
            print("entry: reused", flush=True)
            return
        stamp.unlink(missing_ok=True)
        self.closure[ENTRY] = modules
        (directory / "Main.cinch").write_bytes(self.sources[ENTRY])
        _, manifest, objects = self.provider(ENTRY, directory)
        started = time.monotonic()
        while True:
            result = subprocess.run([str(self.compiler), "--emit-ir-only", "--interfaces", str(manifest),
                                     "--objects", str(objects), "Main.cinch", "main.ll"], cwd=directory,
                                    env=self.environment, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
            if result.returncode == 0:
                break
            diagnostics = directory / "main.ll.diagnostics"
            text = diagnostics.read_text(errors="replace") if diagnostics.is_file() else ""
            wanted = self.new_roots(text)
            if not wanted:
                raise BuildError(f"the entry did not compile:\n{text[:3000] or result.stdout.decode(errors='replace')}")
            for module, unit in wanted:
                self.roots.setdefault(module, []).append(unit)
            bootstrap.write_json(self.out / "discovered-roots.json", {"schema": 1, "modules": self.roots})
            print(f"roots: the entry requested {len(wanted)}; rebuilding", flush=True)
            self.run()
            return self.link(output)
        command = [self.clang, f"-O{self.opt_level}", "-Wno-override-module", "main.ll",
                   *[str(path) for name in modules for path in self.objects[name]],
                   str(self.base.object), str(self.base.runtime),
                   "-o", str(output)]
        if bootstrap.HOST["system"] == "Linux":
            command += ["-ldl", "-lpthread", "-lm"]
        linked = subprocess.run(command, cwd=directory, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
        if linked.returncode:
            raise BuildError("the link failed:\n" + linked.stdout.decode(errors="replace")[-3000:])
        stamp.write_text(key)
        print(f"entry: compiled and linked in {time.monotonic() - started:.1f} s", flush=True)


def check(compiler: Path, reference: Path, base: Base, out: Path) -> None:
    """Compile the audited bundle with the result and compare the module it emits."""
    stage = reference.parent
    bundle = stage.parent / "stage1" / "compiler.cinch"
    target = out / "check.ll"
    started = time.monotonic()
    code = subprocess.run([str(compiler), "--emit-ir-only", "--interfaces", str(stage / "base-interfaces.json"),
                           "--objects", str(stage / "objects.json"), str(bundle), str(target)],
                          cwd=out, env=bootstrap.environment()).returncode
    if code or not bootstrap.same_file_bytes(target, reference):
        raise BuildError(f"the module-built compiler does not reproduce {reference} (exit {code})")
    print(f"check: reproduced {reference} byte for byte in {time.monotonic() - started:.1f} s", flush=True)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    parser.add_argument("--builder", required=True, help="the Cinch compiler that compiles the modules")
    parser.add_argument("--out", required=True, help="the build directory; it is also the cache")
    parser.add_argument("-j", "--jobs", type=int, default=os.cpu_count() or 2)
    parser.add_argument("--opt-level", type=int, default=1, choices=range(4))
    parser.add_argument("--keep-inputs", action="store_true",
                        help="leave each module's provider manifest and layer in place, to rerun its compile by hand")
    parser.add_argument("--check", type=Path, metavar="COMPILER_LL",
                        help="a stage's compiler.ll; the result must emit it again for that chain's bundle")
    args = parser.parse_args()
    started = time.monotonic()
    try:
        bootstrap.environment()
        builder = Builder(args, bootstrap.collect_compiler_sources())
        builder.run()
        output = builder.out / "cinch"
        builder.link(output)
        print(f"module build: {len(builder.compiled)} modules compiled and {len(builder.reused)} reused, "
              f"{builder.shards_compiled} objects compiled and {builder.shards_reused} reused, "
              f"{time.monotonic() - started:.1f} s: {output}", flush=True)
        if args.check:
            check(output, args.check.resolve(), builder.base, builder.out)
        return 0
    except (BuildError, bootstrap.BootstrapError, OSError, subprocess.CalledProcessError) as error:
        print(f"module build: {error}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
