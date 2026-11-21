#!/usr/bin/env python3
"""Publish a typed, memory-mappable Cinch interface package."""
import argparse
import json
import os
from pathlib import Path
import struct
import tempfile

MAGIC = b"CINCHPK2"
# Value tags in a structured contract. Integers take two little-endian
# words after the tag. Text and object keys use common string-table IDs.
NULL, FALSE, TRUE, NAT, TEXT, ARRAY, OBJECT, DEFINITIONS, NEGATIVE = range(9)
HEADER_SIZE = 80
MISSING = 0xFFFFFFFF


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


def encode_value(out, value, strings, body_index, *, definitions=False, graph=False):
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
                encode_value(out, item, strings, body_index)
    elif isinstance(value, dict):
        out.extend(words(OBJECT, len(value)))
        for key, child in sorted(value.items()):
            out.extend(words(strings.id(key)))
            # Only the schema-10 body envelope owns a definition table.
            body_table = (graph and key == "definitions" and value.keys() == {"version", "definitions"}
                          and value["version"] == 1 and isinstance(child, list)
                          and all(isinstance(row, dict) and row.get("identity") in body_index for row in child))
            encode_value(out, child, strings, body_index, definitions=body_table, graph=key == "total_body")
    else:
        raise ValueError(f"unsupported structured contract value {type(value).__name__}")


def pack(rows, objects=None):
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
    for name in order:
        row = definitions[name]
        targets = sorted({body_index[target] for target in calls(row["body"])})
        if row.keys() != {"identity", "parameters", "result", "body"}:
            raise ValueError(f"unsupported body fields for {name}")
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
        encode_value(encoded, documents[name], strings, body_index)
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

    payloads = (strings.encoded(), modules, exports, bodies, edges, recipes.data, contracts.data, abi)
    offset = HEADER_SIZE
    sections = bytearray()
    for payload in payloads:
        sections.extend(words(offset, len(payload)))
        offset += len(payload)
    return b"".join((MAGIC, words(offset, 2), sections, *payloads))


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
