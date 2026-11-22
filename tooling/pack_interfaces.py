#!/usr/bin/env python3
"""Publish a typed, memory-mappable Cinch interface package."""
import argparse
from functools import cached_property
import json
import os
from pathlib import Path
import struct
import tempfile

STRUCTURED_ABI_MAGIC = b"CINCHPK4"
# Value tags in a structured contract. Integers take two little-endian
# words after the tag. Text and object keys use common string-table IDs.
NULL, FALSE, TRUE, NAT, TEXT, ARRAY, OBJECT = range(7)
NEGATIVE = 8
BODY_ROOT = 9
FLAT_HEADER_SIZE = 88
MISSING = 0xFFFFFFFF

LOCAL, NUMBER, BOOL, PRIMITIVE, CALL, LET, IF = range(7)
BODY_FIELDS = {
    "local": {"node", "slot"},
    "number": {"node", "sort", "decimal"},
    "bool": {"node", "value"},
    "primitive": {"node", "operation", "arguments"},
    "call": {"node", "target", "arguments"},
    "let": {"node", "value", "body"},
    "if": {"node", "condition", "yes", "no"},
}


def canonical(value):
    return json.dumps(value, ensure_ascii=False, sort_keys=True, separators=(",", ":")).encode()


def words(*values):
    return struct.pack("<" + "I" * len(values), *values)


class Arena:
    def __init__(self):
        self.data = bytearray()
        self.index = {}

    def add(self, value):
        if isinstance(value, str):
            value = value.encode()
        row = self.index.get(value)
        if row is None:
            row = (len(self.data), len(value))
            self.data.extend(value)
            self.index[value] = row
        return row


class StringArena(Arena):
    def __init__(self):
        super().__init__()
        self.ids = {}

    def id(self, value):
        self.add(value)
        return self.ids.setdefault(value, len(self.ids))

    def encoded(self):
        table = bytearray(words(len(self.ids)))
        for value in self.ids:
            table.extend(words(*self.add(value)))
        table.extend(self.data)
        return table


def body_graphs(value):
    if isinstance(value, dict):
        for key, child in value.items():
            if key == "total_body" and child is not None:
                if child["version"] != 1:
                    raise ValueError("unsupported total-body graph")
                yield child["definitions"]
            else:
                yield from body_graphs(child)
    elif isinstance(value, list):
        for child in value:
            yield from body_graphs(child)


class BodyNodes:
    """Four little-endian words per node, with postorder child indices."""

    def __init__(self, strings, definitions):
        self.strings = strings
        self.definitions = definitions
        self.nodes = bytearray()
        self.operands = bytearray()
        self.targets = []

    def append_operands(self, entries):
        first = len(self.operands) // 4
        self.operands.extend(words(*entries))
        return first, len(entries)

    def append_node(self, tag, first, second=0, third=0):
        index = len(self.nodes) // 16
        self.nodes.extend(words(tag, first, second, third))
        return index

    def expression(self, expression, targets):
        kind = expression.get("node")
        if expression.keys() != BODY_FIELDS.get(kind):
            raise ValueError(f"unsupported total-body fields for {kind!r}")
        if kind == "local":
            return self.append_node(LOCAL, expression["slot"])
        if kind == "number":
            return self.append_node(NUMBER, self.strings.id(expression["sort"]),
                                    self.strings.id(expression["decimal"]))
        if kind == "bool":
            if not isinstance(expression["value"], bool):
                raise ValueError("total-body bool requires a boolean value")
            return self.append_node(BOOL, int(expression["value"]))
        if kind in ("primitive", "call"):
            operands = [self.expression(arg, targets) for arg in expression["arguments"]]
            start, count = self.append_operands(operands)
            if kind == "call":
                target = self.definitions[expression["target"]]
                targets.add(target)
                return self.append_node(CALL, target, start, count)
            return self.append_node(PRIMITIVE, self.strings.id(expression["operation"]), start, count)
        if kind == "let":
            value = self.expression(expression["value"], targets)
            body = self.expression(expression["body"], targets)
            return self.append_node(LET, value, body)
        if kind == "if":
            condition = self.expression(expression["condition"], targets)
            yes = self.expression(expression["yes"], targets)
            no = self.expression(expression["no"], targets)
            return self.append_node(IF, condition, yes, no)
        raise AssertionError(f"missing packed encoder for {kind!r}")

    def definition(self, definition, edges):
        start, count = self.append_operands([self.strings.id(sort) for sort in definition["parameters"]])
        targets = set()
        root = self.expression(definition["body"], targets)
        first_target = len(edges) // 4
        ordered = sorted(targets)
        self.targets.append(ordered)
        edges.extend(words(*ordered))
        return words(self.strings.id(definition["identity"]), self.strings.id(definition["result"]),
                     start, count, root, first_target, len(targets), 0)

    def reference(self, root, graph):
        if graph.keys() != {"version", "definitions"} or graph["version"] != 1:
            raise ValueError(f"unsupported total-body graph for {root!r}")
        if root not in self.definitions:
            raise ValueError(f"missing total-body root {root!r}")
        rows = graph["definitions"]
        indices = [self.definitions[row["identity"]] for row in rows]
        if any(a >= b for a, b in zip(indices, indices[1:])):
            raise ValueError(f"unordered or duplicate total-body definitions for {root}")
        selected = set(indices)
        root_id = self.definitions[root]
        pending = [root_id]
        seen = set()
        while pending:
            node = pending.pop()
            if node in seen:
                continue
            if node not in selected:
                raise ValueError(f"missing total-body dependency for {root}")
            seen.add(node)
            pending.extend(self.targets[node])
        if seen != selected:
            raise ValueError(f"unrelated total-body definition for {root}")
        return words(BODY_ROOT, root_id)


def encode_value(out, value, strings, bodies=None):
    if value is None:
        out.extend(words(NULL))
    elif isinstance(value, bool):
        out.extend(words(TRUE if value else FALSE))
    elif isinstance(value, int):
        magnitude = abs(value)
        if magnitude >= 1 << 64:
            raise ValueError("integer exceeds the interface wire range")
        out.extend(words(NEGATIVE if value < 0 else NAT, magnitude & MISSING, magnitude >> 32))
    elif isinstance(value, str):
        out.extend(words(TEXT, strings.id(value)))
    elif isinstance(value, list):
        out.extend(words(ARRAY, len(value)))
        for item in value:
            encode_value(out, item, strings, bodies)
    elif isinstance(value, dict):
        out.extend(words(OBJECT, len(value)))
        for key, child in sorted(value.items()):
            out.extend(words(strings.id(key)))
            if bodies is not None and key == "total_body" and child is not None:
                root = value.get("symbol", value.get("implementation", value.get("identity")))
                out.extend(bodies.reference(root, child))
            else:
                encode_value(out, child, strings, bodies)
    else:
        raise ValueError(f"unsupported structured contract value {type(value).__name__}")


def pack(rows, objects=None):
    strings, contracts = StringArena(), Arena()
    modules, exports, bodies, edges = (bytearray() for _ in range(4))
    definitions = {}
    documents = {}
    for name, _, raw in rows:
        doc = json.loads(raw)
        if doc["schema"] != 10 or doc["module"] != name or name in documents:
            raise ValueError(f"wrong schema or inconsistent module {name}")
        documents[name] = doc
        for graph in body_graphs(doc["exports"]):
            for body in graph:
                identity = body["identity"]
                if definitions.setdefault(identity, body) != body:
                    raise ValueError(f"conflicting bodies for {identity}")

    order = sorted(definitions)
    body_index = {name: i for i, name in enumerate(order)}
    nodes = BodyNodes(strings, body_index)
    for name in order:
        row = definitions[name]
        if row.keys() != {"identity", "parameters", "result", "body"}:
            raise ValueError(f"unsupported body fields for {name}")
        bodies.extend(nodes.definition(row, edges))

    sorted_rows = sorted(rows, key=lambda row: row[0])
    positions = {name: i for i, (name, _, _) in enumerate(sorted_rows)}
    input_to_sorted = [positions[name] for name, _, _ in rows]
    for i, (name, digest, _) in enumerate(sorted_rows):
        values = sorted(documents[name]["exports"]["values"], key=lambda row: row["name"])
        if any(a["name"] == b["name"] for a, b in zip(values, values[1:])):
            raise ValueError(f"duplicate value export in {name}")
        encoded = bytearray()
        encode_value(encoded, documents[name], strings, nodes)
        span = contracts.add(bytes(encoded))
        modules.extend(words(*strings.add(name), *strings.add(digest),
                             len(exports) // 32, len(values), *span, input_to_sorted[i]))
        for row in values:
            root = body_index[row["symbol"]] if row["total_body"] is not None else MISSING
            exports.extend(words(*strings.add(row["name"]), *strings.add(row["symbol"]),
                                 *strings.add(row["scheme"]), row["arity"], root))

    abi = b""
    if objects is not None:
        metadata = dict(objects)
        if "interfaces" in metadata:
            embedded = metadata.pop("interfaces")
            manifest = {name: digest for name, digest, _ in rows}
            private = []
            for item in embedded:
                name = item["module"]
                if name not in manifest:
                    private.append(item)
                elif (manifest[name], documents[name]) != (item["iface_hash"], item["interface"]):
                    raise ValueError(f"object and interface disagree for {name}")
            metadata["interface_refs"] = [{key: value for key, value in row.items() if key != "interface"}
                                          for row in embedded]
            if private:
                metadata["private_interfaces"] = private
        # ABI-only IDs follow all the contract IDs. An interface-only reader
        # can leave that string table suffix untouched.
        abi = bytearray(words(len(strings.ids)))
        encode_value(abi, metadata, strings)

    payloads = (strings.encoded(), modules, exports, bodies, edges, nodes.nodes,
                nodes.operands, contracts.data, abi)
    offset = FLAT_HEADER_SIZE
    sections = bytearray()
    for payload in payloads:
        sections.extend(words(offset, len(payload)))
        offset += len(payload)
    return b"".join((STRUCTURED_ABI_MAGIC, words(offset, 4), sections, *payloads))


def read_manifest(path):
    manifest = json.loads(path.read_bytes())
    if manifest["schema"] != 1:
        raise ValueError("unsupported interface manifest")
    return [(name, digest, (path.parent / filename).read_bytes()) for name, digest, filename in manifest["interfaces"]]


def packed_sections(data):
    if len(data) < FLAT_HEADER_SIZE or data[:8] != STRUCTURED_ABI_MAGIC:
        raise ValueError("invalid packed interface header")
    if struct.unpack_from("<II", data, 8) != (len(data), 4):
        raise ValueError("invalid packed interface length")
    sections = []
    end = FLAT_HEADER_SIZE
    for at in range(16, FLAT_HEADER_SIZE, 8):
        start, length = struct.unpack_from("<II", data, at)
        if start != end or start + length > len(data):
            raise ValueError("invalid packed interface directory")
        end = start + length
        sections.append(memoryview(data)[start:end])
    if end != len(data):
        raise ValueError("invalid packed interface directory")
    return sections


def packed_manifest(data):
    """Read the public module identities from a CINCHPK4 section directory."""
    strings, modules = packed_sections(data)[:2]
    if len(strings) < 4 or len(modules) % 36:
        raise ValueError("invalid packed module index")
    text_at = 4 + 8 * struct.unpack_from("<I", strings)[0]
    if text_at > len(strings):
        raise ValueError("invalid packed string table")

    def read_text(start, length):
        first = text_at + start
        if first + length > len(strings):
            raise ValueError("invalid packed module string span")
        return bytes(strings[first:first + length]).decode("utf-8")

    count = len(modules) // 36
    rows = []
    seen = set()
    for at in range(0, len(modules), 36):
        (rank,) = struct.unpack_from("<I", modules, at + 32)
        if rank >= count or rank in seen:
            raise ValueError("invalid packed module rank")
        seen.add(rank)
        name_at, name_len, hash_at, hash_len = struct.unpack_from("<IIII", modules, rank * 36)
        rows.append((read_text(name_at, name_len), read_text(hash_at, hash_len)))
    return rows


class PackedInterfaces:
    """Project checked CINCHPK4 records for tooling that inspects contracts."""

    def __init__(self, data):
        self.data = data
        (strings, self.modules, _, self.bodies, self.edges, self.nodes,
         self.operands, self.contracts, self.abi) = packed_sections(data)
        if len(strings) < 4 or 0 < len(self.abi) < 4:
            raise ValueError("invalid packed string table or ABI")
        count, = struct.unpack_from("<I", strings)
        first = 4 + 8 * count
        if first > len(strings):
            raise ValueError("invalid packed string table")
        self.strings = [
            bytes(self.span(strings, first + start, length)).decode("utf-8")
            for start, length in struct.iter_unpack("<II", strings[4:first])
        ]

    @staticmethod
    def span(data, start, size):
        if start + size > len(data):
            raise ValueError("invalid packed value span")
        return data[start:start + size]

    @staticmethod
    def words(data, start, count):
        return tuple(word for (word,) in struct.iter_unpack("<I", PackedInterfaces.span(data, start * 4, count * 4)))

    @cached_property
    def definitions(self):
        rows = list(struct.iter_unpack("<8I", self.bodies))
        names = [self.strings[row[0]] for row in rows]
        expressions = []
        for tag, a, b, c in struct.iter_unpack("<4I", self.nodes):
            if tag == LOCAL:
                node = dict(node="local", slot=a)
            elif tag == NUMBER:
                node = dict(node="number", sort=self.strings[a], decimal=self.strings[b])
            elif tag == BOOL and a <= 1:
                node = dict(node="bool", value=bool(a))
            elif tag in (PRIMITIVE, CALL):
                args = [expressions[i] for i in self.words(self.operands, b, c)]
                node = (dict(node="primitive", operation=self.strings[a], arguments=args) if tag == PRIMITIVE
                        else dict(node="call", target=names[a], arguments=args))
            elif tag == LET:
                node = dict(node="let", value=expressions[a], body=expressions[b])
            elif tag == IF:
                node = dict(node="if", condition=expressions[a], yes=expressions[b], no=expressions[c])
            else:
                raise ValueError("invalid packed body node")
            expressions.append(node)
        return [
            (dict(identity=self.strings[name], parameters=[self.strings[i] for i in self.words(self.operands, start, count)],
                  result=self.strings[sort], body=expressions[root]),
             self.words(self.edges, first, size))
            for name, sort, start, count, root, first, size, _ in rows
        ]

    def body(self, root):
        pending = [root]
        members = set()
        while pending:
            index = pending.pop()
            if index not in members:
                members.add(index)
                pending.extend(self.definitions[index][1])
        return dict(version=1, definitions=[self.definitions[i][0] for i in sorted(members)])

    def value(self, data, *, bodies, string_count=None):
        def read(depth):
            nonlocal at
            if depth > 256:
                raise ValueError("packed value nesting limit exceeded")
            tag = word()
            if tag <= TRUE:
                return (None, False, True)[tag]
            if tag in (NAT, NEGATIVE):
                magnitude = word() | (word() << 32)
                return magnitude if tag == NAT else -magnitude
            if tag == TEXT:
                return string()
            if tag == BODY_ROOT and bodies:
                return self.body(word())
            if tag in (ARRAY, OBJECT):
                count = word()
                if count * (8 if tag == OBJECT else 4) > len(data) - at:
                    raise ValueError("invalid packed collection length")
                if tag == ARRAY:
                    return [read(depth + 1) for _ in range(count)]
                result = {}
                for _ in range(count):
                    name = string()
                    if name in result:
                        raise ValueError("duplicate packed object key")
                    result[name] = read(depth + 1)
                return result
            raise ValueError("invalid packed value tag")

        def word():
            nonlocal at
            value, = struct.unpack_from("<I", data, at)
            at += 4
            return value

        def string():
            index = word()
            if index >= string_count:
                raise ValueError("invalid packed text index")
            return self.strings[index]

        at = 0
        if string_count is None:
            string_count = len(self.strings)
        try:
            result = read(0)
        except (IndexError, struct.error) as error:
            raise ValueError("truncated or invalid packed value") from error
        if at != len(data):
            raise ValueError("trailing packed value bytes")
        return result

    @cached_property
    def documents(self):
        manifest = packed_manifest(self.data)
        rows = list(struct.iter_unpack("<9I", self.modules))
        string_count = struct.unpack_from("<I", self.abi)[0] if self.abi else len(self.strings)
        if string_count > len(self.strings):
            raise ValueError("invalid packed contract string boundary")
        documents = []
        for (name, identity), row in zip(manifest, rows):
            start, size = rows[row[8]][6:8]
            document = self.value(self.span(self.contracts, start, size), bodies=True, string_count=string_count)
            documents.append((name, identity, document))
        return documents

    def metadata(self):
        if not self.abi:
            raise ValueError("packed interface package has no object ABI")
        return self.value(self.abi[4:], bodies=False)

    def full_metadata(self):
        metadata = self.metadata()
        documents = {(name, identity): doc for name, identity, doc in self.documents}
        for row in metadata.pop("private_interfaces", []):
            key = row["module"], row["iface_hash"]
            if key in documents:
                raise ValueError("duplicate packed object interface")
            documents[key] = row["interface"]
        metadata["interfaces"] = [
            {**row, "interface": documents[row["module"], row["iface_hash"]]}
            for row in metadata.pop("interface_refs")
        ]
        return metadata


def read_abi(config_path):
    config = json.loads(config_path.read_bytes())
    path = config_path.parent / config["metadata"]
    data = path.read_bytes()
    if data.startswith(STRUCTURED_ABI_MAGIC):
        return path, PackedInterfaces(data).metadata()
    return path, json.loads(data)


def publish(path, data):
    # Readers may still map a previous version. Publish a new inode.
    with tempfile.NamedTemporaryFile(dir=path.parent, delete=False) as output:
        temporary = Path(output.name)
        try:
            output.write(data)
            output.close()
            os.replace(temporary, path)
        finally:
            temporary.unlink(missing_ok=True)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("inputs", type=Path, help="native interface manifest or Rust package execution metadata")
    parser.add_argument("output", type=Path, help="the .cpk file to publish")
    parser.add_argument("--objects", type=Path, help="native objects.json for the same interface set")
    parser.add_argument("--execution", action="store_true", help="read the Rust package execution interface")
    args = parser.parse_args()
    if args.execution:
        if args.objects:
            parser.error("--objects applies to the native manifest input")
        execution = json.loads(args.inputs.read_bytes())
        if execution.get("schema") != 1 or execution.get("kind") != "package-execution":
            parser.error("input is not a Rust package execution interface")
        rows = [(item["module"], item["iface_hash"], canonical(item["interface"]))
                for item in execution["interfaces"]]
        metadata = execution
    else:
        rows = read_manifest(args.inputs)
        metadata = read_abi(args.objects)[1] if args.objects else None
    publish(args.output, pack(rows, metadata))


if __name__ == "__main__":
    main()
