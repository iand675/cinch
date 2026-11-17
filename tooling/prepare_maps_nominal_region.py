#!/usr/bin/env python3
"""Prepare declaration-only Region proposals and inspect preserved interfaces without invoking a compiler."""
from __future__ import annotations

import difflib
import hashlib
import json
from pathlib import Path

from generate_maps_cases import float_source

ROOT = Path(__file__).resolve().parents[1]
OUTPUT = ROOT / "build/maps-nominal-region-static"


def digest(path: Path) -> str:
    """Identify the exact original, proposal, or historical evidence bytes."""
    return hashlib.sha256(path.read_bytes()).hexdigest()


def annotate(path: Path, names: tuple[str, ...], destination: Path) -> dict:
    """Copy one source while changing only the named nominal declaration binders."""
    original = path.read_text()
    proposed = original
    for name in names:
        old = f"  data {name} r "
        new = f"  data {name} (r :: Region) "
        assert proposed.count(old) == 1, (path, name)
        proposed = proposed.replace(old, new)
    # Inverting only these exact declaration substitutions must recover every original byte.
    restored = proposed
    for name in names:
        restored = restored.replace(f"  data {name} (r :: Region) ", f"  data {name} r ")
    assert restored == original
    destination.parent.mkdir(parents=True, exist_ok=True)
    destination.write_text(proposed)
    return {"original": str(path.relative_to(ROOT)), "original_sha256": digest(path),
            "proposal": str(destination.relative_to(ROOT)), "proposal_sha256": digest(destination),
            "declarations": list(names), "expected_parameter_role": "Region",
            "patch": "".join(difflib.unified_diff(original.splitlines(keepends=True), proposed.splitlines(keepends=True),
                                                   fromfile=str(path.relative_to(ROOT)), tofile=str(destination.relative_to(ROOT)), n=0))}


def header(path: Path) -> dict:
    """Record actual nominal headers while leaving all preserved interface files unchanged."""
    data = json.loads(path.read_text())
    return {"path": str(path.relative_to(ROOT)), "sha256": digest(path), "source_hash": data["source_hash"],
            "headers": [{key: row[key] for key in ("identity", "name", "parameters", "params")}
                        for row in data["exports"]["types"]]}


def main() -> None:
    """Prepare isolated source proposals, historical header evidence, and the unchanged IEEE64 consumer."""
    OUTPUT.mkdir(parents=True, exist_ok=True)
    specifications = [
        (ROOT / "build/maps-entry-candidate/sources/Data.Mutable.Map.cinch", ("MapEntry", "MutableMap"), "candidate/Data.Mutable.Map.cinch"),
        (ROOT / "compiler/prelude/Data/Mutable/HashTable.cinch", ("HashTable",), "common/Data.Mutable.HashTable.cinch"),
        (ROOT / "compiler/prelude/Data/Mutable/Var.cinch", ("Var",), "common/Data.Mutable.Var.cinch"),
        (ROOT / "compiler/prelude/Data/Mutable/Map.cinch", ("MutableMap",), "baseline/Data.Mutable.Map.cinch"),
        (ROOT / "compiler/prelude/Data/Mutable/LinkedList.cinch", ("LinkedList", "ListElement"), "baseline/Data.Mutable.LinkedList.cinch"),
    ]
    proposals = [annotate(path, names, OUTPUT / target) for path, names, target in specifications]
    assert proposals[0]["original_sha256"] == "c839d9b0868aaddfc9914fe515eeb613ba23bd934cc07dc4fa58e44463c00078"
    assert proposals[3]["original_sha256"] == "0fc095f0bdcbe26f9ba3f26e5db8cb76891698784b744bb76e2e420dc14eb7c6"
    patches = {"candidate": proposals[:3], "baseline": proposals[1:]}
    patch_paths = {}
    for name, rows in patches.items():
        path = ROOT / f"tooling/map-{name}-nominal-region-proposal.patch"
        path.write_text("".join(row["patch"] for row in rows))
        patch_paths[name] = {"path": str(path.relative_to(ROOT)), "sha256": digest(path)}
    for proposal in proposals:
        del proposal["patch"]
    historical = []
    for epoch, modules in (("maps-855-entry-source", ("Map", "HashTable", "Var")),
                           ("maps-855-baseline-source", ("Map", "HashTable", "Var", "LinkedList"))):
        for module in modules:
            historical.append(header(ROOT / f"build/{epoch}/source/build/Data.Mutable.{module}.cinchif"))
    old_interface_receipt = ROOT / "bootstrap/stdlib-maps-interface-results.json"
    receipt = {"kind": "static-only-map-nominal-region-review", "compiler_invoked": False,
               "source_checks_executed": False, "runtime_executed": False, "candidate_selected": False,
               "production_sources_modified": False, "historical_interfaces_modified": False,
               "proposals": proposals, "patches": patch_paths, "actual_historical_headers": historical,
               "historical_refusals": {"path": str(old_interface_receipt.relative_to(ROOT)), "sha256": digest(old_interface_receipt)},
               "qualification": "Requires a new explicit nominal Region feature epoch; frozen855 source success cannot qualify these proposals."}
    (ROOT / "bootstrap/stdlib-maps-nominal-region-static-results.json").write_text(json.dumps(receipt, indent=2) + "\n")
    output = OUTPUT / "ieee64/Main.cinch"
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text(float_source(64))
    old_consumer = ROOT / "build/maps-region-safe/ieee/float-64/Main.cinch"
    assert output.read_bytes() == old_consumer.read_bytes(), "IEEE64 consumer changed from the qualified historical fixture"
    vectors = ROOT / "compiler/tests/fixtures/maps-vectors.json"
    rows = [row for row in json.loads(vectors.read_text())["floats"] if row["width"] == 64]
    patterns = [0x7ff8000000000001, 0x7ff0000000000001, 0xfff8000000000017, 0x8000000000000000]
    assert all(str(pattern) in output.read_text() for pattern in patterns)
    assert rows[3]["observed"] == 0 and len(rows[0]["expected"]) == 2 and len(rows[-1]["expected"]) == 4
    ieee = {"kind": "static-ieee64-fixture-preparation", "runtime_executed": False,
            "candidate_selected": False, "observations": len(rows), "vectors_sha256": digest(vectors),
            "consumer": str(output.relative_to(ROOT)), "consumer_sha256": digest(output),
            "historical_consumer": str(old_consumer.relative_to(ROOT)), "historical_consumer_sha256": digest(old_consumer),
            "byte_identical_to_historical_fixture": True, "input_bit_patterns": [f"0x{pattern:016x}" for pattern in patterns],
            "remaining_gate": "Run both unchanged baseline and unselected C839 IEEE64 consumers on frozen855/c4 after the emission/runtime hold is released."}
    (ROOT / "bootstrap/stdlib-maps-ieee64-preparation-results.json").write_text(json.dumps(ieee, indent=2) + "\n")
    print(json.dumps({"status": "static-preparation-complete", "nominal_declarations": sum(len(row[1]) for row in specifications),
                      "source_bytes": sum((OUTPUT / row[2]).stat().st_size for row in specifications),
                      "ieee64_sha256": digest(output)}))


if __name__ == "__main__":
    main()
