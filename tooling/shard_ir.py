#!/usr/bin/env python3
"""Split one LLVM IR module into deterministic, independently compiled shards.

One 218 MB translation unit is one thread of clang. The backend phases that
dominate an optimized stage build (the IR optimizer and the machine-code
passes) shard by function, so this module cuts the emitter's single module
into K shards, one `clang -O<n> -c` each, and links the objects in shard
order.

The split is a pure function of the IR text and K: functions are grouped by
the module prefix already spelled in their symbol, groups are assigned to the
least loaded shard in sorted name order, and every synthesized `declare` or
duplicated `internal` definition is emitted in sorted symbol order. Same
input, same K, same shards, byte for byte.

Shards stay linkable:

* a function defined in another shard is `declare`d here with its original
  signature and attribute group,
* an `internal` or `private` definition is copied into every shard that
  references it, because local symbols do not resolve across objects,
* an externally linked definition elsewhere is declared `external` here,
* module-level content (globals, aliases, metadata, comdats) lives in shard 0
  and is declared or copied into the shards that reference it.

This file also runs as the `CINCH_CLANG` wrapper, so a compiler that invokes
clang itself (the pinned Rust seed building Stage1) gets the same split. It
shards only a link of exactly one huge `.ll` input; anything else is exec'd to
the real clang unchanged.
"""
from __future__ import annotations

import concurrent.futures
import os
from pathlib import Path
import re
import shutil
import subprocess
import sys
import tempfile

# Definition-only words on a `define` line. A declaration has no linkage, so
# they are removed when the signature is reused for a `declare`.
STRIP_TOKENS = frozenset({
    "external", "internal", "private", "linkonce", "linkonce_odr", "weak",
    "weak_odr", "common", "appending", "available_externally", "dso_local",
    "dso_preemptable", "default", "hidden", "protected", "dllimport",
    "dllexport", "extern_weak", "unnamed_addr", "local_unnamed_addr",
})

# Same set, but on the right hand side of a global definition, where the
# linkage precedes the `global`/`constant`/`alias` kind.
GLOBAL_SKIP_TOKENS = STRIP_TOKENS | {"thread_local", "externally_initialized"}

# A quoted symbol uses `\XX` escapes; ordinary names never need unescaping
# for identity, so the raw content is the key.
SYMBOL_RE = re.compile(r'@(?:"((?:[^"\\]|\\.)*)"|([A-Za-z$._][A-Za-z0-9$._-]*))')
# A symbol reference, with string literals matched first so an `@` inside a
# constant string is not mistaken for a global.
REFERENCE_RE = re.compile(
    r'c?"(?:[^"\\]|\\.)*"|@(?:"((?:[^"\\]|\\.)*)"|([A-Za-z$._][A-Za-z0-9$._-]*))')

FALLBACK_GROUP = "@misc"


class ShardError(RuntimeError):
    pass


def _balanced(text: str) -> bool:
    depth = 0
    in_string = False
    escaped = False
    for ch in text:
        if in_string:
            if escaped:
                escaped = False
            elif ch == "\\":
                escaped = True
            elif ch == '"':
                in_string = False
            continue
        if ch == '"':
            in_string = True
        elif ch in "([{":
            depth += 1
        elif ch in ")]}":
            depth -= 1
    return depth == 0 and not in_string


def _symbol_text(match: re.Match[str]) -> str:
    return match.group(0)


def _symbol_name(match: re.Match[str]) -> str:
    return match.group(1) if match.group(1) is not None else match.group(2)


class Unit:
    """One top-level definition, with the fields the split needs."""

    __slots__ = ("kind", "symbol", "name", "text", "linkage", "body_kind")

    def __init__(self, kind: str, symbol: str, name: str, text: str,
                 linkage: str = "", body_kind: str = ""):
        self.kind = kind
        self.symbol = symbol
        self.name = name
        self.text = text
        self.linkage = linkage
        self.body_kind = body_kind

    def lines(self) -> int:
        return self.text.count("\n") or 1


class Module:
    def __init__(self) -> None:
        self.header: list[str] = []
        self.functions: list[Unit] = []
        self.globals: list[Unit] = []
        self.declares: dict[str, str] = {}
        self.metadata: list[str] = []
        self.other: list[str] = []
        self.has_metadata_attachments = False


def _first_word(text: str) -> str:
    words = text.strip().split(None, 1)
    return words[0] if words else ""


def _global_fields(text: str) -> tuple[str, str]:
    """The linkage and the `global`/`constant`/`alias` kind of a definition."""
    _, _, rhs = text.partition(" = ")
    words = rhs.strip().split()
    linkage = ""
    index = 0
    while index < len(words) and words[index] in GLOBAL_SKIP_TOKENS:
        if words[index] in ("internal", "private", "linkonce", "linkonce_odr",
                            "weak", "weak_odr", "common", "appending",
                            "available_externally", "external"):
            linkage = words[index]
        index += 1
    kind = words[index] if index < len(words) else ""
    return linkage, kind


def parse_module(text: str) -> Module:
    """Split IR text into top-level units without interpreting their bodies."""
    module = Module()
    lines = text.splitlines(keepends=True)
    index = 0
    while index < len(lines):
        line = lines[index]
        if not line.strip():
            index += 1
            continue
        if line.startswith("define"):
            start = index
            index += 1
            while index < len(lines) and not lines[index].startswith("}"):
                index += 1
            if index >= len(lines):
                raise ShardError(f"function at line {start + 1} has no closing brace")
            index += 1
            block = "".join(lines[start:index])
            match = SYMBOL_RE.search(lines[start])
            if match is None:
                raise ShardError(f"function at line {start + 1} has no symbol")
            linkage = ""
            for word in lines[start].split()[1:]:
                if word not in STRIP_TOKENS:
                    break
                if word in ("internal", "private", "linkonce", "linkonce_odr",
                            "weak", "weak_odr", "available_externally"):
                    linkage = word
            module.functions.append(Unit("function", _symbol_text(match),
                                         _symbol_name(match), block, linkage))
            continue
        if line.startswith("declare"):
            block = line
            index += 1
            while not _balanced(block):
                if index >= len(lines):
                    raise ShardError("unterminated declare")
                block += lines[index]
                index += 1
            match = SYMBOL_RE.search(block)
            if match is None:
                raise ShardError("declare without a symbol")
            module.declares[_symbol_name(match)] = block
            continue
        if line.startswith("@"):
            block = line
            index += 1
            while not _balanced(block):
                if index >= len(lines):
                    raise ShardError("unterminated global")
                block += lines[index]
                index += 1
            match = SYMBOL_RE.match(block)
            if match is None:
                raise ShardError(f"global without a symbol: {block[:80]!r}")
            linkage, kind = _global_fields(block)
            module.globals.append(Unit("global", _symbol_text(match),
                                       _symbol_name(match), block, linkage, kind))
            continue
        if line.startswith("attributes"):
            module.header.append(line)
            index += 1
            continue
        if line.startswith("!"):
            block = line
            index += 1
            while not _balanced(block):
                if index >= len(lines):
                    raise ShardError("unterminated metadata")
                block += lines[index]
                index += 1
            module.metadata.append(block)
            continue
        word = _first_word(line)
        if (line.startswith(";") or word in ("target", "source_filename")
                or line.startswith("module asm")):
            module.header.append(line)
        elif (line.startswith("$") or word in ("uselistorder", "uselistorder_bb")
              or line.startswith("module ")):
            module.other.append(line)
        else:
            raise ShardError(f"unrecognised top-level line {index + 1}: {line[:80]!r}")
        index += 1
    # Metadata attachments inside a body are numeric and cross-reference other
    # metadata definitions. The emitter does not produce them today; when a
    # later mode does, every shard must carry the whole metadata block.
    for unit in module.functions:
        if re.search(r",\s*![A-Za-z0-9_.]+\s+!\d", unit.text):
            module.has_metadata_attachments = True
            break
    return module


def references(text: str) -> set[str]:
    """Every symbol name referenced in text, string literals excluded.

    Keys are names without `@` or quotes, so a bare and a quoted spelling of
    the same symbol compare equal.
    """
    found: set[str] = set()
    for match in REFERENCE_RE.finditer(text):
        if match.group(0).startswith(('"', 'c"')):
            continue
        found.add(_symbol_name(match))
    return found


GENERATED_SUFFIX = re.compile(
    r"(?:\.clo#h|\.blk#h|\.dec[0-9]+|\.app|\.clo|\.blk|\.mk|\.exit|#tail|#h)$")


def group_of(name: str) -> str:
    """The module prefix a symbol already spells.

    `Bootstrap.Infer.foo` is in `Bootstrap.Infer`, `Data.String.map` in
    `Data.String`. The backend's generated suffixes (`.app`, `.clo`, `.mk`,
    `.dec<N>`, `#tail`) name a unit's companions, so they are stripped first:
    a unit and its apply stub shard together. A name with no remaining dot is
    in the stable fallback group. The result never depends on the interface
    set.
    """
    name = GENERATED_SUFFIX.sub("", name)
    if "." in name:
        return name.rsplit(".", 1)[0]
    return FALLBACK_GROUP


def declaration_for(define_text: str) -> str:
    """A `declare` with the same signature and attribute group as a `define`."""
    # The signature is the first line; the body follows it. The line ends
    # with the opening brace of the body.
    body = define_text.split("\n", 1)[0].rstrip()
    if body.endswith("{"):
        body = body[:-1].rstrip()
    match = SYMBOL_RE.search(body)
    if match is None:
        raise ShardError("define without a symbol")
    head = body[:match.start()].split()
    if not head or head[0] != "define":
        raise ShardError(f"not a define line: {define_text[:60]!r}")
    words = head[1:]
    while words and words[0] in STRIP_TOKENS:
        words.pop(0)
    rest = body[match.end():]
    if not rest.startswith("("):
        raise ShardError(f"define has no parameter list: {define_text[:60]!r}")
    depth = 0
    end = None
    for offset, ch in enumerate(rest):
        if ch == "(":
            depth += 1
        elif ch == ")":
            depth -= 1
            if depth == 0:
                end = offset
                break
    if end is None:
        raise ShardError(f"define has an unbalanced parameter list: {define_text[:60]!r}")
    params = rest[1:end]
    attrs = rest[end + 1:].strip()
    line = f"declare {' '.join(words)} {match.group(0)}({params})"
    if attrs:
        line += f" {attrs}"
    return line + "\n"


def _first_type(text: str) -> str:
    """The leading type of a global's right hand side, up to its initializer."""
    depth = 0
    for offset, ch in enumerate(text):
        if ch in "([{":
            depth += 1
        elif ch in ")]}":
            depth -= 1
        elif depth == 0 and (ch.isspace() or ch == ","):
            return text[:offset]
    return text


def external_declaration(unit: Unit) -> str:
    """A declaration of a globally linked definition that lives elsewhere."""
    _, _, rhs = unit.text.partition(" = ")
    rhs = rhs.strip()
    words = rhs.split()
    index = 0
    while index < len(words) and words[index] in GLOBAL_SKIP_TOKENS:
        index += 1
    kind = words[index] if index < len(words) else "global"
    if kind in ("alias", "ifunc"):
        kind = "global"
    rest = rhs
    for _ in range(index + 1):
        rest = rest.lstrip()
        _, _, rest = rest.partition(" ")
    value_type = _first_type(rest.strip())
    if not value_type:
        raise ShardError(f"global has no type: {unit.text[:60]!r}")
    return f"{unit.symbol} = external {kind} {value_type}\n"


def _assignment(module: Module, count: int) -> list[int]:
    """Greedy balance in sorted group order; ties resolve to the lower shard.

    A group larger than its fair share (`total / count`) is cut at function
    boundaries into consecutive chunks of at most that share, because one
    module bigger than a whole shard would otherwise set the wall clock for
    every K. Chunks keep the group's IR order, so the cut is deterministic.
    """
    loads = [0] * count
    # Shard 0 carries the module-level content: globals, aliases and, when a
    # body attaches metadata, the metadata it needs.
    loads[0] += sum(unit.lines() for unit in module.globals)
    loads[0] += sum(line.count("\n") or 1 for line in module.metadata)
    loads[0] += sum(line.count("\n") or 1 for line in module.other)
    total = sum(unit.lines() for unit in module.functions)
    share = max(1, -(-total // count))
    groups: dict[str, list[int]] = {}
    for index, unit in enumerate(module.functions):
        groups.setdefault(group_of(unit.name), []).append(index)
    shards = [0] * len(module.functions)

    def place(chunk: list[int]) -> None:
        target = min(range(count), key=lambda k: (loads[k], k))
        for index in chunk:
            shards[index] = target
        loads[target] += sum(module.functions[index].lines() for index in chunk)

    for group in sorted(groups):
        chunk: list[int] = []
        size = 0
        for index in groups[group]:
            lines = module.functions[index].lines()
            if chunk and size + lines > share:
                place(chunk)
                chunk, size = [], 0
            chunk.append(index)
            size += lines
            if size >= share:
                place(chunk)
                chunk, size = [], 0
        if chunk:
            place(chunk)
    return shards


def split_module(text: str, count: int) -> list[str]:
    """K shard texts. Pure function of (text, count)."""
    if count < 1:
        raise ShardError("shard count must be positive")
    if count == 1:
        return [text]
    module = parse_module(text)
    shard_of_function = _assignment(module, count)
    header = "".join(module.header)
    content = ("" if module.has_metadata_attachments else "".join(module.metadata))
    definitions: dict[str, Unit] = {unit.name: unit for unit in module.functions}
    definitions.update({unit.name: unit for unit in module.globals})
    per_shard: list[list[Unit]] = [[] for _ in range(count)]
    for unit, shard in zip(module.functions, shard_of_function):
        per_shard[shard].append(unit)
    shards: list[str] = []
    for shard in range(count):
        parts = [header]
        defined = {unit.name for unit in per_shard[shard]}
        if shard == 0:
            parts.append(content)
            parts.append("".join(unit.text for unit in module.globals))
            parts.append("".join(module.other))
            defined.update(unit.name for unit in module.globals)
        if module.has_metadata_attachments:
            # Numbered metadata is module scoped and `!range` attachments
            # need their definitions in the object that carries the body.
            parts.append("".join(module.metadata))
        parts.append("".join(unit.text for unit in per_shard[shard]))
        pending = sorted(references("".join(parts)) - defined)
        declarations: dict[str, str] = {}
        copies: dict[str, str] = {}
        seen: set[str] = set()
        while pending:
            symbol = pending.pop(0)
            if symbol in seen or symbol in defined:
                continue
            seen.add(symbol)
            unit = definitions.get(symbol)
            if unit is None:
                if symbol in module.declares:
                    declarations[symbol] = module.declares[symbol]
                elif symbol.startswith("llvm."):
                    # Intrinsics need no declaration.
                    continue
                else:
                    raise ShardError(f"reference to undefined symbol {symbol}")
                continue
            if unit.linkage in ("internal", "private"):
                copies[symbol] = unit.text
                defined.add(symbol)
                pending.extend(sorted(references(unit.text) - defined))
                pending.sort()
            elif unit.kind == "function":
                declarations[symbol] = declaration_for(unit.text)
            else:
                declarations[symbol] = external_declaration(unit)
        parts.append("".join(copies[symbol] for symbol in sorted(copies)))
        parts.append("".join(declarations[symbol] for symbol in sorted(declarations)))
        shards.append("".join(parts))
    return shards


def write_shards(ir_path: Path, directory: Path, count: int,
                 stem: str = "compiler") -> list[Path]:
    """Write `<stem>.shard<k>.ll`; K=1 returns the original IR untouched."""
    if count == 1:
        return [ir_path]
    text = ir_path.read_text()
    paths = [directory / f"{stem}.shard{k}.ll" for k in range(count)]
    for path, shard in zip(paths, split_module(text, count)):
        path.write_text(shard)
    return paths


def object_paths(directory: Path, count: int, stem: str = "compiler") -> list[Path]:
    return [directory / f"{stem}.shard{k}.o" for k in range(count)]


def run_parallel(commands: list[list[str]], jobs: int, cwd: Path,
                 env: dict[str, str] | None = None) -> list[tuple[int, str]]:
    """Run commands under one job budget, returning ordered (code, output)."""
    def one(command: list[str]) -> tuple[int, str]:
        result = subprocess.run([str(word) for word in command], cwd=cwd, env=env,
                                stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
        return result.returncode, result.stdout.decode(errors="replace")

    results: list[tuple[int, str]] = [(0, "")] * len(commands)
    with concurrent.futures.ThreadPoolExecutor(max_workers=max(1, jobs)) as pool:
        futures = {pool.submit(one, command): index
                   for index, command in enumerate(commands)}
        for future in concurrent.futures.as_completed(futures):
            results[futures[future]] = future.result()
    return results


def compile_shards(shards: list[Path], objects: list[Path], clang: str,
                   opt_level: int, jobs: int, cwd: Path,
                   env: dict[str, str] | None = None,
                   extra: tuple[str, ...] = ()) -> tuple[int, str]:
    """Compile every shard to its object; return (first failure, combined log)."""
    commands = [[clang, f"-O{opt_level}", "-Wno-override-module", *extra,
                 "-c", str(shard), "-o", str(obj)]
                for shard, obj in zip(shards, objects)]
    results = run_parallel(commands, jobs, cwd, env)
    log = "".join(f"shard{k}: exit {code}\n{output}"
                  for k, (code, output) in enumerate(results))
    failure = next((code for code, _ in results if code), 0)
    return failure, log


# ---------------------------------------------------------------- clang wrapper

# Flags that affect compilation and must be repeated for every shard. Link
# inputs and linker libraries go to the final link only.
COMPILE_FLAG_PREFIXES = ("-O", "-W", "-f", "-m", "-g", "-std=", "-D", "-I",
                         "-isystem", "-target", "-arch", "-Xclang", "-include")
LINK_VALUE_FLAGS = ("-o", "-target", "-arch", "-Xlinker", "-L", "-l", "-Wl,", "-framework")


def _clang_command(argv: list[str]) -> tuple[str | None, list[str], str | None]:
    """(real clang, positional inputs, output path) from a clang argument list."""
    real = os.environ.get("CINCH_CLANG_REAL")
    if not real:
        real = shutil.which("clang")
    inputs: list[str] = []
    output = None
    index = 0
    while index < len(argv):
        word = argv[index]
        if word == "-o":
            index += 1
            output = argv[index] if index < len(argv) else None
        elif word.startswith("-"):
            pass
        else:
            inputs.append(word)
        index += 1
    return real, inputs, output


def clang_main(argv: list[str]) -> int:
    """Shard one huge `.ll` link; otherwise hand the arguments to real clang."""
    real, inputs, output = _clang_command(argv)
    if real is None:
        print("shard_ir: clang not found (set CINCH_CLANG_REAL)", file=sys.stderr)
        return 127
    jobs = int(os.environ.get("CINCH_BOOTSTRAP_JOBS", "1"))
    threshold = int(os.environ.get("CINCH_SHARD_MIN_BYTES", str(4 << 20)))
    ir_inputs = [Path(word) for word in inputs if word.endswith(".ll")]
    compile_only = "-c" in argv or "-S" in argv or "-E" in argv
    if (compile_only or jobs <= 1 or len(ir_inputs) != 1
            or any("-target" in word for word in argv)
            or ir_inputs[0].stat().st_size < threshold
            or output is None):
        return subprocess.run([real, *argv]).returncode

    ir_path = ir_inputs[0]
    opt_level = 0
    for word in argv:
        if re.fullmatch(r"-O[0-3sz]", word):
            opt_level = 0 if word[2] in "sz" else int(word[2])
    compile_flags = tuple(word for word in argv
                          if word.startswith(COMPILE_FLAG_PREFIXES)
                          and not word.startswith(LINK_VALUE_FLAGS))
    link_flags = tuple(word for word in argv
                       if word.startswith(("-Wl,", "-L", "-l", "-framework", "-Xlinker")))
    # Positional non-IR inputs (the runtime archive, library objects) keep
    # their command-line order and go after the shard objects.
    link_inputs = [word for word in inputs if not word.endswith(".ll")]
    directory = Path(tempfile.mkdtemp(prefix="cinch-shards-"))
    try:
        shards = write_shards(ir_path, directory, jobs)
        objects = object_paths(directory, jobs)
        failure, log = compile_shards(shards, objects, real, opt_level,
                                      jobs, directory, os.environ.copy(),
                                      compile_flags)
        if failure:
            sys.stderr.write(log)
            return failure
        return subprocess.run([real, *[str(obj) for obj in objects], *link_inputs,
                               "-o", str(output), *link_flags]).returncode
    finally:
        shutil.rmtree(directory, ignore_errors=True)


if __name__ == "__main__":
    raise SystemExit(clang_main(sys.argv[1:]))
