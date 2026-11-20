#!/usr/bin/env python3
"""Verify the selected raw Number outcomes, owner controls and changed-wrapper regressions."""
from __future__ import annotations

import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
COMPILERS = {
    "rust": "cad70e533fcab8305fe6e79ae95f135f61252338c6a5fd5e5358e8f27563e3f4",
    "native-raw": "2f07707e0022b55e92bd8c2412962d06d2e4455022261c3e36dfa27e57fb231f",
}
RUNTIMES = {
    "rust": "c4b1097b4b8e26541400228bdbbdcba1376101014823aeb2f5af619fbede57ac",
    "native-raw": "80bd5b546742f8374221ac445ec20b7cfd0df0958d5c39313cde077204d6e141",
}


def digest(path: Path) -> str:
    """Record exact current source, consumer, reference and observed receipt identities."""
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main() -> None:
    """Require both exact seed/runtime epochs without combining published predecessor sources."""
    names = ["rust-v3", "native-raw-v1", "edge-kernel-rust-v1", "edge-native-raw-v1", "kernel-native-raw-v1", "control-rust-v1", "control-native-raw-v1", "affected-rust-v1", "affected-number-native-raw-v1", "affected-float-native-raw-v1", "affected-float32-native-raw-v1"]
    covered = set()
    matrix = []
    for name in names:
        path = ROOT / f"compiler/evidence/stdlib/stdlib-json-raw-number-{name}-results.json"
        receipt = json.loads(path.read_text())
        frontend = receipt["frontend"]
        assert receipt["compiler_sha256"] == COMPILERS[frontend]
        assert receipt["observed_runtime_archive_sha256"] == RUNTIMES[frontend]
        assert receipt["opt_level"] == 2 and receipt["compiler_host_stack_kib"] == 65536
        assert receipt["consumer_stack_limit"] == "8M"
        if frontend == "native-raw":
            assert receipt["interfaces_sha256"] == "1d22f22c4e4e0831e1176b70b4dd77cfa13c5d6db83869a596f64b34cd252256"
            assert receipt["objects_manifest_sha256"] == "22ab850df180636adac27eafb7a789dfd89009000c572b194c05d0a192bc5624"
            assert receipt["provider_object_sha256"] == "46a7e57f467dd36135238903910feee82998526cbb3edaa94580a2a40ba47e64"
            assert "-O2" in receipt["raw_link_command"]
        for module, identity in receipt["sources"].items():
            source = ROOT / "seed/prelude" / (module.replace(".", "/") + ".cinch")
            assert digest(source) == identity, (name, module)
        for suite in receipt["suites"]:
            label = suite["suite"]
            assert (frontend, label) not in covered
            covered.add((frontend, label))
            source = ROOT / f"seed/tests/fixtures/json-codec-{label}.cinch"
            assert digest(source) == suite["consumer_source_sha256"]
            reference_name = label if label in {"raw-number", "raw-number-edge", "raw-number-kernel", "float", "float32"} else ""
            reference = ROOT / (f"seed/tests/fixtures/json-codec-{reference_name}-vectors.json" if reference_name else "seed/tests/fixtures/json-codec-vectors.json")
            assert receipt["reference_fixtures"][label] == digest(reference)
            assert len(suite["runs"]) == 2
            assert sorted(run["poisoned_free_validation"] for run in suite["runs"]) == [False, True]
            for run in suite["runs"]:
                assert run["stdout"] == "0\n" and run["stats"]["live"] == run["leaked"] == 0
                assert run["heap_limit"] == ("256M" if run["poisoned_free_validation"] else "32M")
            matrix.append({"suite": label, "frontend": frontend, "opt_level": 2,
                           "receipt": str(path.relative_to(ROOT)), "receipt_sha256": digest(path),
                           "compiler_sha256": COMPILERS[frontend], "runtime_archive_sha256": RUNTIMES[frontend],
                           "consumer_sha256": suite["consumer_source_sha256"], "llvm_sha256": suite["llvm_sha256"],
                           "normal_peak_bytes": suite["runs"][0]["stats"]["peak"],
                           "poison_peak_bytes": suite["runs"][1]["stats"]["peak"]})
    for frontend in COMPILERS:
        for label in ["raw-number", "raw-number-edge", "raw-number-kernel", "raw-number-control", "number", "float", "float32"]:
            assert (frontend, label) in covered
    assert len(matrix) == 14
    documentation = {}
    for module in ["Binary32", "Value", "Codec"]:
        path = ROOT / f"build/json-raw-number-docs/{module}.json"
        parsed = json.loads(path.read_text())
        assert parsed["sourceSha256"] == digest(ROOT / f"seed/prelude/Text/Json/{module}.cinch")
        assert not parsed["diagnostics"] and not parsed["orphans"]
        declarations, count = list(parsed["declarations"]), 0
        while declarations:
            declaration = declarations.pop()
            declarations.extend(declaration.get("children", []))
            count += 1
            doc = declaration.get("doc")
            assert doc and not doc["legacy"], declaration["id"]
            assert not any(word in doc["markdown"].lower() for word in ["golang", "go's", "go "]), declaration["id"]
        documentation[module] = {"canonical_declarations": count, "extract_sha256": digest(path), "diagnostics": 0, "orphans": 0}
    assert sum(row["canonical_declarations"] for row in documentation.values()) == 652
    foreign_path = ROOT / "compiler/evidence/stdlib/stdlib-json-raw-number-foreign-results.json"
    foreign = json.loads(foreign_path.read_text())
    assert foreign["production_source_sha256"] == digest(ROOT / "seed/prelude/Text/Json/Value.cinch")
    assert foreign["compiler_sha256"] == COMPILERS["rust"]
    assert not foreign["foreign_execution_claimed"]
    assert {row["triple"] for row in foreign["targets"]} == {"wasm32-unknown-unknown", "aarch64-unknown-linux-gnu"}
    base = json.loads((ROOT / "seed/tests/fixtures/json-codec-raw-number-vectors.json").read_text())
    edge = json.loads((ROOT / "seed/tests/fixtures/json-codec-raw-number-edge-vectors.json").read_text())
    kernel = json.loads((ROOT / "seed/tests/fixtures/json-codec-raw-number-kernel-vectors.json").read_text())
    assert (len(base["raw_number"]), len(base["typed"]), len(edge["raw_number"]), len(kernel["hex_rounding"])) == (285, 13, 6, 794)
    plan = json.loads((ROOT / "compiler/evidence/stdlib/stdlib-json-capability-plan.json").read_text())
    assert plan["reference_declarations"] == len(plan["declarations"]) == 92
    report = {
        "status": "Raw numeric string casts/outcomes increment; full 92-record JSON package remains partial",
        "sources": {name: digest(ROOT / f"seed/prelude/Text/Json/{name}.cinch") for name in documentation},
        "documentation": documentation, "same_selected_source_memory_processes": len(matrix) * 2, "matrices": matrix,
        "independent_observations_per_frontend": {"raw_number": 285, "quoted_number_updates": 13, "long_significand_cancellation": 6, "production_rounding_integer_models": 794},
        "owner_api_control_groups_per_frontend": 6, "retained_midpoint_pipelines_per_owner_run": 512,
        "affected_existing_observations_per_frontend": {"checked_number": 25, "binary64": 163, "binary32": 600},
        "reference_generator_sha256": digest(ROOT / "tooling/go_json_codec_vectors.go"),
        "reference_fixtures": {label: digest(ROOT / f"seed/tests/fixtures/json-codec-{label}-vectors.json") for label in ["raw-number", "raw-number-edge", "raw-number-kernel"]},
        "portable_receipt": str(foreign_path.relative_to(ROOT)), "portable_receipt_sha256": digest(foreign_path),
        "raw_byte_adaptation": "Scalar and byte factories are explicit. Exact number/error byte views preserve original invalid UTF-8; scalar String views replace each malformed byte. Numeric errors carry function, original input and syntax/range enum, without claiming arbitrary runtime-error identity.",
        "integer_review": "Parse signed decimal through a bounded unsigned64 prefix first. Syntax returns zero; overflow returns the signed endpoint. Prefix unsigned overflow may precede a later invalid character, while signed range is checked after complete unsigned parsing.",
        "float_review": "Validate decimal separators or hexadecimal grammar completely before conversion. Retain sixteen significant hex nibbles plus sticky omitted bits, exact digit/fraction counts and exponent bounded by four times input length plus 2048. Round directly to IEEE64 normals/subnormals, preserving signed zero and overflow infinity. One-block LLVM clamps every evaluated shift and selects defined zero/64/wide outputs.",
        "nan_scope": "IEEE NaN classification is asserted; unspecified NaN payload identity is not claimed.",
        "historical_attempts": "Rust v1 stopped at an explicit nonnegative-distance obligation; v2 stopped at prohibited br/jump containment in the initial helper. Guard and branchless-select source corrections precede the selected successful v3. No checker, runtime or compiler mode was changed.",
        "parent_owned_checks_pending": ["Expanded Cargo lib_json_codec/lib_json_value targets", "Actual new primitives on PPC big-endian", "Integration/provider/full-suite gates"],
        "performance_status": "No JSON speedup claimed; paired public pipelines and lowering review remain open.",
        "full_scope_remaining": ["Dynamic JsonAny and legacy bridges", "Complete containing-record/error metadata and independent method capabilities", "Real bounded incremental Effect.Json Decoder/Encoder", "Paired public pipeline benchmarks and precise all-92-record final review"],
    }
    output = ROOT / "compiler/evidence/stdlib/stdlib-json-raw-number-evidence.json"
    output.write_text(json.dumps(report, indent=2) + "\n")
    print(f"raw Number checkpoint: 652 canonical declarations, {len(matrix)*2} selected-source memory processes")


if __name__ == "__main__":
    main()
