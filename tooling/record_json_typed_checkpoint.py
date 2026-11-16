#!/usr/bin/env python3
"""Verify the exact-source typed JSON checkpoint matrix and summarize its explicit remaining scope."""
from __future__ import annotations

import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def digest(path: Path) -> str:
    """Identify source and receipts without relying on historical filenames."""
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main() -> None:
    """Reject stale source, consumer, documentation or memory observations before recording a checkpoint."""
    source = ROOT / "compiler/prelude/Text/Json/Codec.cinch"
    source_hash = digest(source)
    summaries = []
    for family in ["typed", "float", "field", "codec-control"]:
        for frontend in ["rust", "native"]:
            path = ROOT / f"bootstrap/stdlib-json-{family}-{frontend}-validation-results.json"
            receipt = json.loads(path.read_text())
            assert receipt["sources"]["Text.Json.Codec"] == source_hash, path
            for name, recorded in receipt["sources"].items():
                assert digest(ROOT / ("compiler/prelude/" + name.replace(".", "/") + ".cinch")) == recorded, name
            assert receipt["compiler_host_stack_kib"] == 65536
            for suite in receipt["suites"]:
                fixture = ROOT / f"compiler/tests/fixtures/json-codec-{suite['suite']}.cinch"
                assert digest(fixture) == suite["consumer_source_sha256"], fixture
                assert len(suite["runs"]) == 2
                for run in suite["runs"]:
                    assert run["stdout"] == "0\n" and run["leaked"] == 0 and run["stats"]["live"] == 0
                    assert run["heap_limit"] == ("256M" if run["poisoned_free_validation"] else "32M")
                summaries.append({"family":family,"frontend":frontend,"receipt":str(path.relative_to(ROOT)),"receipt_sha256":digest(path),"compiler_sha256":receipt.get("native_compiler_sha256",receipt["compiler_sha256"]),"runtime_archive_sha256":receipt["observed_runtime_archive_sha256"],"opt_level":receipt["opt_level"],"consumer_sha256":suite["consumer_source_sha256"],"llvm_sha256":suite["llvm_sha256"],"normal_peak_bytes":suite["runs"][0]["stats"]["peak"],"poison_peak_bytes":suite["runs"][1]["stats"]["peak"]})
    extra_path = ROOT / "bootstrap/stdlib-json-codec-control-rust-o0-validation-results.json"
    extra = json.loads(extra_path.read_text())
    assert extra["sources"]["Text.Json.Codec"] == source_hash and extra["opt_level"] == 0
    assert extra["suites"][0]["consumer_source_sha256"] == digest(ROOT / "compiler/tests/fixtures/json-codec-codec-control.cinch")
    for run in extra["suites"][0]["runs"]:
        assert run["stdout"] == "0\n" and run["leaked"] == 0 and run["stats"]["live"] == 0
        assert run["heap_limit"] == ("256M" if run["poisoned_free_validation"] else "32M")
    summaries.append({"family":"codec-control-extra-o0","frontend":"rust","receipt":str(extra_path.relative_to(ROOT)),"receipt_sha256":digest(extra_path),"compiler_sha256":extra["compiler_sha256"],"runtime_archive_sha256":extra["observed_runtime_archive_sha256"],"opt_level":0,"consumer_sha256":extra["suites"][0]["consumer_source_sha256"],"llvm_sha256":extra["suites"][0]["llvm_sha256"],"normal_peak_bytes":extra["suites"][0]["runs"][0]["stats"]["peak"],"poison_peak_bytes":extra["suites"][0]["runs"][1]["stats"]["peak"]})
    documentation = json.loads((ROOT / "build/json-codec-docs.json").read_text())
    assert documentation["sourceSha256"] == source_hash
    assert not documentation["diagnostics"] and not documentation["orphans"]
    declarations = list(documentation["declarations"])
    count = 0
    while declarations:
        declaration = declarations.pop()
        declarations.extend(declaration.get("children", []))
        doc = declaration.get("doc")
        assert doc and not doc["legacy"], declaration["id"]
        assert not any(word in doc["markdown"].lower() for word in ["golang", "go's", "go "]), declaration["id"]
        count += 1
    foreign = ROOT / "bootstrap/stdlib-json-field-kernel-validation.json"
    kernel = json.loads(foreign.read_text())
    assert kernel["production_source_sha256"] == source_hash and not kernel["foreign_execution_claimed"]
    assert len(kernel["targets"]) == 2
    report = {
        "status":"partial typed-codec checkpoint; full JSON package remains open",
        "source_sha256":source_hash,"core_dependency_sha256":digest(ROOT / "compiler/prelude/Text/Json/Value.cinch"),
        "canonical_declarations":count,"documentation_diagnostics":0,"documentation_orphans":0,
        "independent_observations_per_frontend":{"typed_updates":56,"float64":163,"field_matching":104},
        "additional_controls":14,"same_source_memory_runs":len(summaries)*2,"matrices":summaries,
        "reference_generator_sha256":digest(ROOT / "tooling/go_json_codec_vectors.go"),
        "reference_fixtures":{name:digest(ROOT / f"compiler/tests/fixtures/json-codec-{name}vectors.json") for name in ["","float-","field-"]},
        "foreign_kernel_receipt_sha256":digest(foreign),
        "parent_owned_checks_pending":["Cargo lib_json_codec target (default Rust O0)","Full integration and provider gates"],
        "performance_status":"No JSON speedup claim. Public-pipeline comparisons and classifier lowering review remain a separate checkpoint.",
        "remaining_scope":["Unsigned/narrow integers and binary32 descriptors","Map/key descriptors and binary byte values","Dynamic value codecs and explicit legacy codec adapters","General numeric string-cast outcomes including hexadecimal floats and saturated errors","Full error field/record metadata and reviewed language adaptations","Incremental effectful Decoder/Encoder and mixed Token/More/Decode semantics","Full 92-record capability review and public pipeline benchmarks"]
    }
    (ROOT / "bootstrap/stdlib-json-typed-evidence.json").write_text(json.dumps(report,indent=2)+"\n")
    print(f"typed checkpoint: {count} canonical declarations, {len(summaries)*2} same-source memory runs")


if __name__ == "__main__":
    main()
