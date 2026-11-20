#!/usr/bin/env python3
"""Verify the exact-source JSON numeric/binary matrix without combining older Codec epochs."""
from __future__ import annotations

import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def digest(path: Path) -> str:
    """Identify exact current source, reference and receipt bytes."""
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main() -> None:
    """Reject stale sources, consumers and memory results before recording the reviewed increment."""
    source = ROOT / "compiler/prelude/Text/Json/Codec.cinch"
    source_hash = digest(source)
    receipts = ["numeric-binary-final-rust", "numeric-binary-final-rust-tail", "numeric-binary-final-native", "numeric-binary-final-native-tail", "binary-rust", "binary-control-rust", "binary-native", "numeric-binary-final-control-rust-o2", "numeric-binary-final-control-rust-o0", "numeric-binary-final-control-native"]
    matrix = []
    coverage = {}
    for name in receipts:
        path = ROOT / f"bootstrap/evidence/stdlib/stdlib-json-{name}-validation-results.json"
        item = json.loads(path.read_text())
        assert item["sources"]["Text.Json.Codec"] == source_hash, path
        for module, identity in item["sources"].items():
            assert digest(ROOT / ("compiler/prelude/" + module.replace(".", "/") + ".cinch")) == identity, module
        assert item["compiler_host_stack_kib"] == 65536
        expected_runtime = "c4b1097b4b8e26541400228bdbbdcba1376101014823aeb2f5af619fbede57ac" if item["frontend"] == "rust" else "12945b719660f1fe3a9b450c7da03161da0b93498d7a504bcb2b9e069de5bf90"
        assert item["observed_runtime_archive_sha256"] == expected_runtime
        expected_compiler = "cad70e533fcab8305fe6e79ae95f135f61252338c6a5fd5e5358e8f27563e3f4" if item["frontend"] == "rust" else "9e2a64e0e519fc908f17cd7329b72be90f1161513084bfe2e5e3ebbd4a846f90"
        assert item.get("native_compiler_sha256",item["compiler_sha256"]) == expected_compiler
        for suite in item["suites"]:
            consumer = ROOT / f"compiler/tests/fixtures/json-codec-{suite['suite']}.cinch"
            assert digest(consumer) == suite["consumer_source_sha256"], consumer
            assert len(suite["runs"]) == 2
            for result in suite["runs"]:
                assert result["stdout"] == "0\n" and result["stats"]["live"] == 0 and result["leaked"] == 0
                assert result["heap_limit"] == ("256M" if result["poisoned_free_validation"] else "32M")
            key = (suite["suite"], item["frontend"], item["opt_level"])
            coverage[key] = coverage.get(key, 0) + 1
            matrix.append({"suite":suite["suite"],"frontend":item["frontend"],"opt_level":item["opt_level"],"receipt":str(path.relative_to(ROOT)),"receipt_sha256":digest(path),"compiler_sha256":item.get("native_compiler_sha256",item["compiler_sha256"]),"runtime_archive_sha256":item["observed_runtime_archive_sha256"],"consumer_sha256":suite["consumer_source_sha256"],"llvm_sha256":suite["llvm_sha256"],"normal_peak_bytes":suite["runs"][0]["stats"]["peak"],"poison_peak_bytes":suite["runs"][1]["stats"]["peak"]})
    for suite in ["integer","typed","float","field","binary","binary-control","codec-control"]:
        assert coverage[(suite,"rust",2)] == coverage[(suite,"native",0)] == 1, suite
    assert coverage[("codec-control","rust",0)] == 1
    docs = json.loads((ROOT / "build/json-codec-numeric-binary-docs.json").read_text())
    assert docs["sourceSha256"] == source_hash and not docs["diagnostics"] and not docs["orphans"]
    declarations = list(docs["declarations"])
    count = 0
    while declarations:
        item = declarations.pop(); declarations.extend(item.get("children",[])); count += 1
        doc = item.get("doc")
        assert doc and not doc["legacy"], item["id"]
        assert not any(word in doc["markdown"].lower() for word in ["golang","go's","go "]), item["id"]
    words_path = ROOT / "bootstrap/evidence/stdlib/stdlib-json-word-kernel-validation.json"
    words = json.loads(words_path.read_text())
    assert words["production_source_sha256"] == source_hash and len(words["runs"]) == 4
    assert words["cases_per_run"] == 2048 and not words["foreign_execution_claimed"]
    for observation in words["runs"]:
        assert "live=0" in observation["observation"] and "leaked=0" in observation["observation"]
        assert observation["heap_limit"] == ("256M" if observation["poisoned"] else "32M")
    primitives = []
    production_text = source.read_text()
    for name in ["decimal-eight","binary-linebreak-eight"]:
        start = production_text.index(f"  prim {name} ::")
        end = production_text.index("\n  -- |",start)
        primitives.append("  -- | Exact load-free production SWAR primitive.\n" + production_text[start:end])
    word_consumer = "-- | Independent runtime and foreign JSON word-kernel validation.\nmodule Main where\n  import Data.Machine\n  import Data.Bytes.Word\n" + "\n".join(primitives) + (ROOT / "compiler/tests/fixtures/json-codec-word-kernel-body.cinch.inc").read_text()
    assert hashlib.sha256(word_consumer.encode()).hexdigest() == words["consumer_sha256"]
    report = {
        "status":"partial numeric/binary checkpoint; full JSON remains open",
        "source_sha256":source_hash,"core_dependency_sha256":digest(ROOT / "compiler/prelude/Text/Json/Value.cinch"),
        "canonical_declarations":count,"documentation_diagnostics":0,"documentation_orphans":0,
        "independent_observations_per_frontend":{"typed":56,"float":163,"field":104,"integer":390,"binary":86},
        "additional_control_groups":{"codec":14,"binary":10},"full_codec_same_source_memory_runs":len(matrix)*2,
        "isolated_word_kernel_memory_runs":4,"word_kernel_lane_cases_per_run":2048,"word_kernel_receipt_sha256":digest(words_path),
        "matrices":matrix,"reference_generator_sha256":digest(ROOT / "tooling/go_json_codec_vectors.go"),
        "reference_fixtures":{name:digest(ROOT / f"compiler/tests/fixtures/json-codec-{name}vectors.json") for name in ["","float-","field-","integer-","binary-"]},
        "binary_error_adaptation":"Return an explicit fresh prior value overlaid with completed binary prefix only when the reservation fits its owned logical buffer. Preserve original aliases. Never infer hidden spare capacity or mutate prior backing. No-progress failures retain the supplied application value.",
        "binary_linebreak_policy":"The loose-text flag permits CR/LF. Nonzero unused Base64 bits remain accepted independently. All padding/alphabet requirements remain enforced.",
        "parent_owned_checks_pending":["Expanded Cargo lib_json_codec target","Final integration/provider gates"],
        "performance_status":"Public pipeline comparisons remain open; no JSON speedup claimed.",
        "remaining_scope":["Binary32 exact rounding/formatting","Map/key and dynamic value descriptors","Raw numeric string-cast outcomes","Complete containing-record error metadata and precise language adaptations","Real incremental Effect.Json Decoder/Encoder","Full 92-record review and public pipeline benchmarks"]
    }
    (ROOT / "bootstrap/evidence/stdlib/stdlib-json-numeric-binary-evidence.json").write_text(json.dumps(report,indent=2)+"\n")
    print(f"numeric/binary checkpoint: {count} canonical declarations, {len(matrix)*2} full-source plus 4 isolated memory runs")


if __name__ == "__main__":
    main()
