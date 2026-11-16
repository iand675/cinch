#!/usr/bin/env python3
"""Verify the exact-source binary32 checkpoint without blending historical JSON epochs."""
from __future__ import annotations

import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def digest(path: Path) -> str:
    """Identify exact selected source, reference and observed receipt bytes."""
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main() -> None:
    """Require both exact compiler/runtime epochs, ownership controls and canonical documentation."""
    receipts = ["rust-v4", "native-raw-v1", "control-rust-v1", "control-native-raw-v1", "affected-rust-v1"]
    coverage = set()
    matrix = []
    for name in receipts:
        path = ROOT / f"bootstrap/stdlib-json-binary32-{name}-results.json"
        receipt = json.loads(path.read_text())
        native = receipt["frontend"] == "native-raw"
        assert receipt["opt_level"] == 2 and receipt["compiler_host_stack_kib"] == 65536
        assert receipt["consumer_stack_limit"] == "8M"
        assert receipt["compiler_sha256"] == ("2f07707e0022b55e92bd8c2412962d06d2e4455022261c3e36dfa27e57fb231f" if native else "cad70e533fcab8305fe6e79ae95f135f61252338c6a5fd5e5358e8f27563e3f4")
        assert receipt["observed_runtime_archive_sha256"] == ("80bd5b546742f8374221ac445ec20b7cfd0df0958d5c39313cde077204d6e141" if native else "c4b1097b4b8e26541400228bdbbdcba1376101014823aeb2f5af619fbede57ac")
        if native:
            assert receipt["interfaces_sha256"] == "1d22f22c4e4e0831e1176b70b4dd77cfa13c5d6db83869a596f64b34cd252256"
            assert receipt["objects_manifest_sha256"] == "22ab850df180636adac27eafb7a789dfd89009000c572b194c05d0a192bc5624"
            assert receipt["provider_object_sha256"] == "46a7e57f467dd36135238903910feee82998526cbb3edaa94580a2a40ba47e64"
            assert "-O2" in receipt["raw_link_command"]
        for module, identity in receipt["sources"].items():
            source = ROOT / "compiler/prelude" / (module.replace(".", "/") + ".cinch")
            assert digest(source) == identity, (path, module)
        for suite in receipt["suites"]:
            fixture = ROOT / f"compiler/tests/fixtures/json-codec-{suite['suite']}.cinch"
            assert digest(fixture) == suite["consumer_source_sha256"]
            reference_name = suite["suite"]
            reference = ROOT / (f"compiler/tests/fixtures/json-codec-{reference_name}-vectors.json" if reference_name in {"float32", "float"} else "compiler/tests/fixtures/json-codec-vectors.json")
            assert receipt["reference_fixtures"][reference_name] == digest(reference)
            assert len(suite["runs"]) == 2
            assert sorted(run["poisoned_free_validation"] for run in suite["runs"]) == [False, True]
            for run in suite["runs"]:
                assert run["stdout"] == "0\n" and run["leaked"] == 0 and run["stats"]["live"] == 0
                assert run["heap_limit"] == ("256M" if run["poisoned_free_validation"] else "32M")
            coverage.add((receipt["frontend"], suite["suite"]))
            matrix.append({"suite": suite["suite"], "frontend": receipt["frontend"], "opt_level": 2,
                           "receipt": str(path.relative_to(ROOT)), "receipt_sha256": digest(path),
                           "compiler_sha256": receipt["compiler_sha256"],
                           "runtime_archive_sha256": receipt["observed_runtime_archive_sha256"],
                           "consumer_sha256": suite["consumer_source_sha256"], "llvm_sha256": suite["llvm_sha256"],
                           "normal_peak_bytes": suite["runs"][0]["stats"]["peak"],
                           "poison_peak_bytes": suite["runs"][1]["stats"]["peak"]})
    for frontend in ["rust", "native-raw"]:
        for suite in ["float32", "float32-control"]:
            assert (frontend, suite) in coverage
    for suite in ["float", "number"]:
        assert ("rust", suite) in coverage
    docs = {}
    for module in ["Binary32", "Value", "Codec"]:
        path = ROOT / f"build/json-binary32-docs/{module}.json"
        parsed = json.loads(path.read_text())
        assert parsed["sourceSha256"] == digest(ROOT / f"compiler/prelude/Text/Json/{module}.cinch")
        assert not parsed["diagnostics"] and not parsed["orphans"]
        declarations = list(parsed["declarations"])
        count = 0
        while declarations:
            declaration = declarations.pop()
            declarations.extend(declaration.get("children", []))
            count += 1
            doc = declaration.get("doc")
            assert doc and not doc["legacy"], declaration["id"]
            assert not any(word in doc["markdown"].lower() for word in ["golang", "go's", "go "]), declaration["id"]
        docs[module] = {"canonical_declarations": count, "diagnostics": 0, "orphans": 0,
                        "extract_sha256": digest(path)}
    foreign_path = ROOT / "bootstrap/stdlib-json-binary32-foreign-results.json"
    foreign = json.loads(foreign_path.read_text())
    assert foreign["production_source_sha256"] == digest(ROOT / "compiler/prelude/Text/Json/Binary32.cinch")
    assert len(foreign["targets"]) == 2 and not foreign["foreign_execution_claimed"]
    assert {target["triple"] for target in foreign["targets"]} == {"wasm32-unknown-unknown", "aarch64-unknown-linux-gnu"}
    reference_path = ROOT / "compiler/tests/fixtures/json-codec-float32-vectors.json"
    reference = json.loads(reference_path.read_text())
    observations = {key: len(reference[key]) for key in ["float32_encoding", "float32_conversion", "typed"]}
    assert observations == {"float32_encoding": 425, "float32_conversion": 85, "typed": 90}
    report = {
        "status": "Exact binary32 increment; full 92-record JSON package remains partial",
        "sources": {module: digest(ROOT / f"compiler/prelude/Text/Json/{module}.cinch") for module in docs},
        "documentation": docs, "independent_observations_per_frontend": observations,
        "ownership_control_groups_per_frontend": 5, "retained_pipeline_iterations_per_control_run": 512,
        "same_selected_source_memory_processes": len(matrix) * 2, "matrices": matrix,
        "reference_fixture_sha256": digest(reference_path),
        "reference_generator_sha256": digest(ROOT / "tooling/go_json_codec_vectors.go"),
        "foreign_receipt": str(foreign_path.relative_to(ROOT)), "foreign_receipt_sha256": digest(foreign_path),
        "rounding_review": "Compare exact normalized decimal coefficients against adjacent binary32 rational midpoints. Bound parsed exponent by input length plus 1000. Preserve all significant input digits and signed zero; refuse finite-range overflow through checked/typed APIs.",
        "shortest_review": "Construct exact coefficient from significand times powers of 2 or 5. At each precision from 1 to 9, test the nearest ties-even coefficient then its neighbor; asymmetric power-of-two intervals require that second candidate. Choose the first exact parseback and apply JSON notation thresholds.",
        "historical_failures": "Rust v1/v2 stopped at new case layouts and v3 at inferred conditional refinement before selected v4 passed. Their durable logs/artifacts are separate epochs; no failure is relabeled as a successful observation.",
        "performance_status": "No JSON speedup claim; paired public pipeline benchmarks remain open.",
        "parent_owned_checks_pending": ["Expanded lib_json_codec/lib_json_value Cargo targets", "Big-endian actual-bitcast foreign gate", "Integration/provider/full-suite gates"],
        "remaining_scope": ["Raw Number string casts and numeric outcomes", "Dynamic JsonAny and legacy bridges", "Containing-record error metadata and independent capabilities", "Bounded incremental Effect.Json reader/writer", "Full 92-record review and paired public pipeline benchmarks"],
    }
    output = ROOT / "bootstrap/stdlib-json-binary32-evidence.json"
    output.write_text(json.dumps(report, indent=2) + "\n")
    print(f"binary32 checkpoint: {sum(row['canonical_declarations'] for row in docs.values())} canonical declarations, {len(matrix)*2} selected-source memory processes")


if __name__ == "__main__":
    main()
