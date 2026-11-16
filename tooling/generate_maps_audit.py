#!/usr/bin/env python3
"""Reproduce the static maps operation inventory without invoking a compiler."""
from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path
import re

from audit_stdlib_surface import declarations


ROOT = Path(__file__).resolve().parents[1]
BASIS = "bfb4cb57113016a6cb054fd5c460b6cea6fcf60d"
OUTPUT = ROOT / "bootstrap/stdlib-maps-audit.json"
HOMES = (
    "Data.Map", "Data.HashMap", "Data.Mutable.HashTable",
    "Data.Mutable.BTree", "Data.Mutable.IndexMap",
)
SUPPORT = ("Data.Producer", "Data.Hash", "Relation.Eq", "Relation.Ord", "Data.Compare", "Data.Machine")
TESTS = (
    "compiler/tests/lib_map_set_ops.rs", "compiler/tests/lib_hashmap_real.rs",
    "compiler/tests/lib_hash_keys.rs", "compiler/tests/lib_mutable.rs",
    "compiler/tests/fixtures/mutable_containers.cinch",
)


def digest(path: Path) -> str:
    """Identify an exact source input by its SHA-256 digest."""
    return hashlib.sha256(path.read_bytes()).hexdigest()


def module_inventory(module: str) -> dict:
    """Record actual exports and their declaration lines, not inferred names."""
    path = Path("compiler/prelude") / (module.replace(".", "/") + ".cinch")
    lines = (ROOT / path).read_text().splitlines()
    exports = [name.strip() for line in lines if line.startswith("  export ")
               for name in line.removeprefix("  export ").split(",")]
    if module == "Data.Machine":
        exports = [name for name in exports if name in {
            "feq-f32", "feq-f64", "flt-f32", "flt-f64", "fgt-f32", "fgt-f64",
            "is-na-nf32", "is-na-nf64", "total-order-f32", "total-order-f64",
        }]
    signatures = {}
    for index, line in enumerate(lines, 1):
        match = re.search(r"\b([a-z][a-z0-9-]*) :: (.*)", line)
        if match and match[1] in exports:
            signatures[match[1]] = {"line": index, "type": match[2]}
    return {"module": module, "path": str(path), "sha256": digest(ROOT / path),
            "exports": exports, "value_signatures": signatures}


def operations() -> list[dict]:
    """Keep each of the ten reference records open with concrete capability gaps."""
    rows = [
        ("All", "An invocation enumerates current key/value pairs; yield False stops calls immediately.",
         ["map-to-list/map-fold", "hash-map-to-list/hash-map-fold", "hash-table-to-list/hash-table-for-each", "btree-to-list/btree-for-each", "index-map-to-list/index-map-fold"],
         "Lists are eager; pure folds do not stop their storage walk; effectful for-each returns Unit. No pair producer or effectful stop protocol. Mutable for-each snapshots differ from live deletion semantics.",
         ["stop after zero/one/middle/all entries", "repeat same iterator after source mutation", "delete unvisited entry", "insert during callback", "retained key/value owners"]),
        ("Clone", "A shallow independent map preserving nil; keys and values are assigned, not recursively cloned.",
         ["persistent Map/HashMap may safely share immutable structure", "mutable to-list plus fresh from-list is a building block"],
         "No named clone. Returning a mutable handle aliases its cells, so cannot implement clone. Rebuilding changes layout/order/capacity and requires key capabilities; non-reflexive entries must remain present. Nil has no base-map representation.",
         ["source/clone insert-delete isolation", "shared mutable payload remains shared", "drop source before clone", "nil versus allocated empty", "NaN-key entry multiplicity"]),
        ("Collect", "Consume a pair sequence into a fresh allocated map; the last equal-key occurrence wins.",
         ["mutable hash-table-from-list/btree-from-list/index-map-from-list are last-wins", "persistent insert in a left fold is last-wins", "Data.Producer is a pure pull-state building block"],
         "No sequence consumer. map-from-pairs and hash-map-from-pairs are first-wins and are not direct equivalents. Existing Producer steps cannot perform arbitrary Mut/IO effects.",
         ["empty input yields allocated empty", "duplicate pairs keep last value", "managed values survive input release", "sequence callback timing"]),
        ("Copy", "Overwrite/add every source binding in an existing destination; source values win equal keys.",
         ["map-union source destination returns the persistent result", "hash-map-union source destination returns the persistent result", "mutable source snapshot plus destination insert"],
         "No mutable bulk-copy helper or cross-home adapter. Persistent return-value adaptation must be explicit. Snapshot self-copy safety must be specified, not assumed for non-reflexive keys. Nil destination cannot accept a nonempty source.",
         ["disjoint/overlap/empty", "same mutable source and destination", "cross-home/capability agreement", "managed key/value replacement", "nil destination with empty/nonempty source"]),
        ("DeleteFunc", "Call a key/value predicate and delete the current key when it returns True.",
         ["map-filter with negated pure predicate returns a new map", "hash-map-filter with negated pure predicate returns a new map", "mutable snapshot plus key deletion is a restricted building block"],
         "No mutable delete-where helper. Pure filter cannot run mutation callbacks. Snapshot predicates may observe entries already removed by earlier callbacks, unlike live iteration. Deleting a NaN key by lookup does not remove it under IEEE equality.",
         ["none/all/subset", "predicate call count on empty", "delete current/next/other key", "predicate reinsertion or clear", "NaN predicate True still fails key deletion"]),
        ("Equal", "Compare sizes, then corresponding key lookups and value equality; nil and empty compare equal.",
         ["all homes expose size, lookup and entry traversal"],
         "No map equality helper/instance. Pointer identity cannot shortcut equality: NaN keys/values can make a map unequal to itself. Equality across homes needs matching key equivalence and hash/order policies.",
         ["empty/nil combinations", "same size missing key", "value mismatch", "hash collisions", "NaN key and value self-comparison", "positive/negative zero"]),
        ("EqualFunc", "As Equal, with a supplied value predicate that may compare different value types; key equality is unchanged.",
         ["all homes expose size, lookup and entry traversal", "explicit feq-f32/feq-f64 can compare floating values"],
         "No heterogeneously typed value-equality helper or effectful predicate variant. The supplied predicate does not replace key comparison. Strict existing folds cannot avoid all later storage reads, although predicates can be skipped after mismatch.",
         ["different value types", "no predicate call after decisive mismatch", "value NaN policy", "missing NaN key prevents value callback", "cross-home key policy mismatch"]),
        ("Insert", "Consume pairs into an existing destination; later equal-key occurrences overwrite earlier values.",
         ["all homes expose a single-binding insert", "persistent left fold over Producer can return an updated map"],
         "No public sequence/bulk-insert into an existing mutable map. A constructor creates a different map. Need explicit producer effects, ownership and self-derived sequence mutation policy.",
         ["last duplicate wins", "empty sequence", "overwrite releases old value", "sequence reads/mutates destination", "early producer failure policy"]),
        ("Keys", "Repeated invocations enumerate current keys, stop on yield False, and do not promise order.",
         ["map-keys", "hash-map-keys", "hash-table-keys", "btree-keys", "index-map-keys"],
         "All existing key views are eager lists. No map-specific lazy/reusable producer. Mutable capture-at-creation versus capture-at-invocation and live deletion behavior remain unresolved.",
         ["empty", "stop without reading later keys", "reuse after mutation", "order-insensitive membership including NaN multiplicity"]),
        ("Values", "Repeated invocations enumerate current values, preserve duplicates, and stop on yield False.",
         ["map-values", "hash-map-values", "hash-table-values", "btree-values", "index-map-values"],
         "All existing value views are eager lists. No early-stop mutable sequence. Do not deduplicate values or pair independently obtained key/value views from an unstable-order map.",
         ["duplicate values", "owner retained after source clear", "stop timing", "reuse after mutation", "NaN values and signed zeros"]),
    ]
    return [{"name": name, "status": "partial-static-audit", "reference_behavior": behavior,
             "existing_building_blocks": blocks, "remaining": missing,
             "required_future_fixtures": fixtures}
            for name, behavior, blocks, missing, fixtures in rows]


def report(sdk: Path) -> dict:
    """Combine exact reference declarations, source identities and review findings."""
    reference = declarations(sdk)["maps"]
    rows = operations()
    assert len(reference) == len(rows) == 10
    for row in rows:
        row["reference_declaration"] = next(d for d in reference if d.startswith("func " + row["name"] + "["))
    sources = ("VERSION", "src/maps/maps.go", "src/maps/iter.go", "src/iter/iter.go", "doc/go_spec.html", "src/internal/runtime/maps/map.go")
    return {
        "format": 1, "kind": "static-operation-audit", "basis": BASIS,
        "reference_package": "maps", "reference_sdk": (sdk / "VERSION").read_text().splitlines()[0],
        "reference_source_hashes": {name: digest(sdk / name) for name in sources},
        "reference_declarations": reference, "reference_count": len(reference),
        "package_status": "partial", "implementation_changes": False,
        "compiler_or_runtime_gates_run": False,
        "modules": [module_inventory(name) for name in HOMES],
        "supporting_modules": [module_inventory(name) for name in SUPPORT],
        "existing_test_source_hashes": {name: digest(ROOT / name) for name in TESTS},
        "operations": rows,
        "blocking_decisions": [
            "Live mutable iteration versus explicitly separate snapshot iteration, including unvisited deletions and early termination.",
            "Effectful pair-source and predicate contracts; the existing Producer step and folds are pure.",
            "Stable captured key equality/hash/order across operations; no stored policy authority exists in current handles.",
            "Explicit IEEE non-reflexive floating keys versus reflexive key wrappers; no default F32/F64 Eq, Ord or Hash instances.",
            "Nil represented only by an explicit Option adapter when observable; allocated empty remains the default map value.",
        ],
        "notes": "See stdlib-maps-audit.md. Existing tests were inspected, not rerun. No record is promoted to complete by this report.",
    }


def main() -> None:
    """Write or verify the lane-owned audit; leave the shared inventory unchanged."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--sdk", type=Path, required=True)
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    rendered = json.dumps(report(args.sdk), indent=2) + "\n"
    if args.check:
        assert OUTPUT.read_text() == rendered, "maps audit differs from its source inputs"
        print("Static maps audit matches 10 declarations and all recorded source hashes")
    else:
        OUTPUT.write_text(rendered)
        print(OUTPUT)


if __name__ == "__main__":
    main()
