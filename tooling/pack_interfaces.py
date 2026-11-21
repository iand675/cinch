#!/usr/bin/env python3
"""Publish a typed, memory-mappable Cinch interface package."""
import argparse
import json
import os
from pathlib import Path
import struct
import tempfile

MAGIC = b"CINCHPK2"
FLAT_MAGIC = b"CINCHPK3"
# Value tags in a structured contract. Integers take two little-endian
# words after the tag. Text and object keys use common string-table IDs.
NULL, FALSE, TRUE, NAT, TEXT, ARRAY, OBJECT, DEFINITIONS, NEGATIVE = range(9)
BODY_ROOT = 9
HEADER_SIZE = 80
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


def calls(expression):
    if isinstance(expression, dict):
        if expression.get("node") == "call":
            yield expression["target"]
        for child in expression.values():
            yield from calls(child)
    elif isinstance(expression, list):
        for child in expression:
            yield from calls(child)


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


def encode_value(out, value, strings, body_index, *, definitions=False, graph=False, flat=None):
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
        if definitions:
            # Consecutive IDs share one span. Preserve the producer's exact
            # order and membership, including any duplicate a verifier must refuse.
            spans = []
            for row in value:
                index = body_index[row["identity"]]
                if spans and index == sum(spans[-1]):
                    first, count = spans[-1]
                    spans[-1] = (first, count + 1)
                else:
                    spans.append((index, 1))
            out.extend(words(DEFINITIONS, len(spans)))
            for first, count in spans:
                out.extend(words(first, count))
        else:
            out.extend(words(ARRAY, len(value)))
            for item in value:
                encode_value(out, item, strings, body_index, flat=flat)
    elif isinstance(value, dict):
        out.extend(words(OBJECT, len(value)))
        for key, child in sorted(value.items()):
            out.extend(words(strings.id(key)))
            if flat is not None and key == "total_body" and child is not None:
                root = value.get("symbol", value.get("implementation", value.get("identity")))
                out.extend(flat.reference(root, child))
                continue
            # Only the schema-10 body envelope owns a definition table.
            body_table = (graph and key == "definitions" and value.keys() == {"version", "definitions"}
                          and value["version"] == 1 and isinstance(child, list)
                          and all(isinstance(row, dict) and row.get("identity") in body_index for row in child))
            encode_value(out, child, strings, body_index, definitions=body_table, graph=key == "total_body", flat=flat)
    else:
        raise ValueError(f"unsupported structured contract value {type(value).__name__}")


def pack(rows, objects=None, *, flat_bodies=False):
    strings, recipes, contracts = StringArena(), Arena(), Arena()
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
    nodes = BodyNodes(strings, body_index) if flat_bodies else None
    for name in order:
        row = definitions[name]
        if row.keys() != {"identity", "parameters", "result", "body"}:
            raise ValueError(f"unsupported body fields for {name}")
        if nodes is not None:
            bodies.extend(nodes.definition(row, edges))
            continue
        targets = sorted({body_index[target] for target in calls(row["body"])})
        encoded = bytearray()
        encode_value(encoded, row, strings, {})
        bodies.extend(words(*strings.add(name), *strings.add(row["result"]),
                            *recipes.add(bytes(encoded)), len(edges) // 4, len(targets)))
        for target in targets:
            edges.extend(words(target))

    sorted_rows = sorted(rows, key=lambda row: row[0])
    positions = {name: i for i, (name, _, _) in enumerate(sorted_rows)}
    input_to_sorted = [positions[name] for name, _, _ in rows]
    for i, (name, digest, _) in enumerate(sorted_rows):
        values = sorted(documents[name]["exports"]["values"], key=lambda row: row["name"])
        if any(a["name"] == b["name"] for a, b in zip(values, values[1:])):
            raise ValueError(f"duplicate value export in {name}")
        encoded = bytearray()
        encode_value(encoded, documents[name], strings, body_index, flat=nodes)
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
        abi = canonical(metadata)

    prefix = (strings.encoded(), modules, exports, bodies, edges)
    suffix = (contracts.data, abi)
    if nodes is not None:
        magic, version, offset = FLAT_MAGIC, 3, FLAT_HEADER_SIZE
        payloads = prefix + (nodes.nodes, nodes.operands) + suffix
    else:
        magic, version, offset = MAGIC, 2, HEADER_SIZE
        payloads = prefix + (recipes.data,) + suffix
    sections = bytearray()
    for payload in payloads:
        sections.extend(words(offset, len(payload)))
        offset += len(payload)
    return b"".join((magic, words(offset, version), sections, *payloads))


def read_manifest(path):
    manifest = json.loads(path.read_bytes())
    if manifest["schema"] != 1:
        raise ValueError("unsupported interface manifest")
    return [(name, digest, (path.parent / filename).read_bytes()) for name, digest, filename in manifest["interfaces"]]


def read_abi(config_path):
    config = json.loads(config_path.read_bytes())
    path = config_path.parent / config["metadata"]
    return path, json.loads(path.read_bytes())


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
