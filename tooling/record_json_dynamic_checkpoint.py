#!/usr/bin/env python3
"""Verify exact selected dynamic/legacy source, independent observations and bounded owner receipts."""
from __future__ import annotations

import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
COMPILERS = {
    "rust": "7e0d98d14cb0213d4deeb277e1ad65fce07784309716085e70e4b9db2c259102",
    "native-raw": "2f07707e0022b55e92bd8c2412962d06d2e4455022261c3e36dfa27e57fb231f",
}
RUNTIMES = {
    "rust": "c4b1097b4b8e26541400228bdbbdcba1376101014823aeb2f5af619fbede57ac",
    "native-raw": "80bd5b546742f8374221ac445ec20b7cfd0df0958d5c39313cde077204d6e141",
}


def digest(path: Path) -> str:
    """Identify exact selected source and receipt bytes independently of a mutable path name."""
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main() -> None:
    """Refuse to combine source/compiler epochs or accept a missing memory/doc/independent gate."""
    names = ["final-rust-v2", "edge-rust-v2", "native-raw-v1", "edge-native-raw-v1", "control-native-raw-v2"]
    covered, matrices = set(), []
    for name in names:
        path = ROOT / f"bootstrap/stdlib-json-dynamic-{name}-results.json"
        receipt = json.loads(path.read_text())
        frontend = receipt["frontend"]
        assert receipt["compiler_sha256"] == COMPILERS[frontend]
        assert receipt["observed_runtime_archive_sha256"] == RUNTIMES[frontend]
        assert receipt["opt_level"] == 2 and receipt["compiler_host_stack_kib"] == 65536
        assert receipt["consumer_stack_limit"] == "8M"
        for module, identity in receipt["sources"].items():
            assert digest(ROOT / "compiler/prelude" / (module.replace(".", "/") + ".cinch")) == identity, (name, module)
        if frontend == "native-raw":
            assert receipt["interfaces_sha256"] == "1d22f22c4e4e0831e1176b70b4dd77cfa13c5d6db83869a596f64b34cd252256"
            assert receipt["objects_manifest_sha256"] == "22ab850df180636adac27eafb7a789dfd89009000c572b194c05d0a192bc5624"
            assert receipt["provider_object_sha256"] == "46a7e57f467dd36135238903910feee82998526cbb3edaa94580a2a40ba47e64"
            assert "-O2" in receipt["raw_link_command"]
            if name == "control-native-raw-v2":
                assert "Data.Json" not in receipt["sources"] and "Text.Codec" not in receipt["sources"]
                embedded = receipt["embedded_legacy_dependencies"]
                for module in ["Data.Json", "Text.Codec", "Data.Machine"]:
                    assert digest(Path(embedded[module]["interface_path"])) == embedded[module]["interface_sha256"]
                for module in ["Data.Json", "Text.Codec"]:
                    assert digest(ROOT / "compiler/prelude" / (module.replace(".", "/") + ".cinch")) == embedded[module]["unchanged_source_sha256"]
        for suite in receipt["suites"]:
            label = suite["suite"]
            assert (frontend, label) not in covered
            covered.add((frontend, label))
            assert suite["consumer_source_sha256"] == digest(ROOT / f"compiler/tests/fixtures/json-codec-{label}.cinch")
            reference = ROOT / (f"compiler/tests/fixtures/json-codec-{label}-vectors.json" if label != "dynamic-control" else "compiler/tests/fixtures/json-codec-vectors.json")
            assert receipt["reference_fixtures"][label] == digest(reference)
            assert len(suite["runs"]) == 2
            assert sorted(row["poisoned_free_validation"] for row in suite["runs"]) == [False, True]
            for run in suite["runs"]:
                # These Int-entry consumers return a checked zero process status and intentionally print nothing.
                assert run["stdout"] == "" and run["stats"]["live"] == run["leaked"] == 0
                assert run["heap_limit"] == ("256M" if run["poisoned_free_validation"] else "32M")
            matrices.append({"frontend": frontend, "suite": label, "opt_level": 2,
                             "receipt": str(path.relative_to(ROOT)), "receipt_sha256": digest(path),
                             "llvm_sha256": suite["llvm_sha256"], "consumer_sha256": suite["consumer_source_sha256"],
                             "normal_peak_bytes": suite["runs"][0]["stats"]["peak"],
                             "poison_peak_bytes": suite["runs"][1]["stats"]["peak"]})
    assert covered == {(frontend, suite) for frontend in COMPILERS for suite in ["dynamic", "dynamic-edge", "dynamic-control"]}
    docs = {}
    for module in ["Binary32", "Value", "Codec", "Dynamic", "Legacy"]:
        path = ROOT / f"build/json-dynamic-docs/{module}.json"
        parsed = json.loads(path.read_text())
        assert parsed["sourceSha256"] == digest(ROOT / f"compiler/prelude/Text/Json/{module}.cinch")
        assert not parsed["diagnostics"] and not parsed["orphans"]
        pending, count = list(parsed["declarations"]), 0
        while pending:
            declaration = pending.pop()
            pending.extend(declaration.get("children", []))
            count += 1
            doc = declaration.get("doc")
            assert doc and not doc["legacy"], declaration["id"]
            assert not any(word in doc["markdown"].lower() for word in ["golang", "go's", "go "]), declaration["id"]
        docs[module] = {"canonical_declarations": count, "extract_sha256": digest(path), "diagnostics": 0, "orphans": 0}
    assert sum(row["canonical_declarations"] for row in docs.values()) == 758
    base = json.loads((ROOT / "compiler/tests/fixtures/json-codec-dynamic-vectors.json").read_text())
    edge = json.loads((ROOT / "compiler/tests/fixtures/json-codec-dynamic-edge-vectors.json").read_text())
    assert (len(base["decode"]), len(base["abort"]), len(base["application"])) == (392, 22, 16)
    assert (len(edge["decode"]), len(edge["abort"]), len(edge["application"])) == (32, 8, 0)
    plan = json.loads((ROOT / "bootstrap/stdlib-json-capability-plan.json").read_text())
    assert plan["reference_declarations"] == len(plan["declarations"]) == 92
    report = {
        "status": "Dynamic values, explicit established-codec bridges and owned semantic context increment; full92 remains partial",
        "sources": {name: digest(ROOT / f"compiler/prelude/Text/Json/{name}.cinch") for name in docs},
        "utf8_dependency": {"source_sha256": digest(ROOT / "compiler/prelude/Data/Bytes/Utf8.cinch"), "published_parent": "7e8575", "implementation_unchanged": True},
        "compiler_epochs": COMPILERS, "runtime_epochs": RUNTIMES,
        "rust_embedded_sources": {"Data.String": "e5b79f9ddb1266a75a198096921b62e4fd6db172fc9111496f37bf7fa9cd04d6", "Data.Bytes": "1e1eba93c135f07ba6eb9245a75faf3d22312110677cf1b180e795a33c88a146"},
        "documentation": docs, "matrices": matrices, "same_selected_source_memory_processes": len(matrices) * 2,
        "independent_observations_per_frontend": {"base": 430, "focused_path_offset_edges": 40},
        "control_groups_per_frontend": 10, "maximum_container_depth": 10000, "flat_array_elements": 4096, "retained_pipeline_repetitions": 512,
        "reference_generator_sha256": digest(ROOT / "tooling/go_json_dynamic_vectors.go"),
        "consumer_generator_sha256": digest(ROOT / "tooling/generate_json_dynamic_cases.py"),
        "reference_fixtures": {label: digest(ROOT / f"compiler/tests/fixtures/json-codec-{label}-vectors.json") for label in ["dynamic", "dynamic-edge"]},
        "value_review": "Private direct recursive spines; exact owned application string/name bytes; last decoded duplicate wins; normalize all names before value callbacks; explicit frame loops and one final fragment join. Present typed-null containers and opaque typed payloads stay distinct from an absent dynamic value for omission.",
        "typed_payload_review": "Explicit JsonType capabilities retain their application value/descriptor, preserve child error identity, forward options and follow the supplied application's alias contract. Opaque payloads have no fabricated decoded shape; legacy projection invokes their complete encoder once then exact-decodes.",
        "error_review": "Default numeric overflow installs floating zero and continues; first error retains source end and pointer-escaped dotted path. Disabled continuation stops with exact constructed prefix and owned semantic start offset/token/pointer/type/cause. Syntax partial mutation and other full reporting-policy behaviors remain open.",
        "legacy_review": "Explicit iterative Data.Json conversions preserve integer precision and direct floating payloads. Fraction/exponent exact numbers project to finite F64, typed null collections project to JNull, malformed strings/keys/numbers/spines refuse. Existing Text.Codec capabilities adapt explicitly with preserved mismatch/range causes and prior application values; legacy APIs are unchanged.",
        "historical_attempts": "Initial fixture layout/helper/reserved-name diagnostics and class-constraint grammar probe precede selected controls. Pre-path-fix edge found pointer escaping in default dotted paths; corrected before selected gates. Original Native ten-control redundantly shadowed unchanged embedded Data.Json/Text.Codec homes; exact object mode lost imported total-body authority and refused existing Word proof. Selected harness consumes the strict embedded class/constructor/instance contracts, with unchanged-source guards and recorded CIF identities; no Word/library/checker/runtime weakening. Original failure remains deterministically preserved under bootstrap/repros/json-dynamic-native-word-proof. Earlier source/closure receipts are not blended.",
        "portable_scope": "No new production LLVM primitive or runtime intrinsic; UTF8 and frozen binary32/numeric kernels are unchanged. Parent owns whole integration/portable target gates; no new foreign execution claimed.",
        "parent_owned_checks_pending": ["Expanded lib_json_codec and lib_json_value targets", "Module import/documentation inventory", "Integration/provider/full-suite gates"],
        "performance_status": "No speed claim; paired public pipelines/lowering remain mandatory.",
        "full_scope_remaining": ["Complete containing-record/error and independent read/write/text capabilities", "Real bounded Effect.Json Decoder/Encoder including disabled-policy syntax partial mutation", "Paired public pipeline benchmarks/lowering and precise all92 reconciliation"],
    }
    output = ROOT / "bootstrap/stdlib-json-dynamic-evidence.json"
    output.write_text(json.dumps(report, indent=2) + "\n")
    print(f"dynamic checkpoint:758 canonical declarations, {len(matrices)*2} selected-source memory processes")


if __name__ == "__main__":
    main()
