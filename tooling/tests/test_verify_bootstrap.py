from pathlib import Path
import json
import sys
import tempfile
import unittest
from unittest.mock import patch

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import verify_bootstrap as verify
import bootstrap_checklist as checklist
import verify_stage3


class FrozenVerification(unittest.TestCase):
    def fixture(self, root):
        for part in verify.PARTS:
            (root / part).mkdir(parents=True)
        (root / "compiler/src").mkdir()
        (root / "compiler/src/main.rs").write_text("frozen source")

    def test_stage3_counts_reviewed_rust_assertions_only_on_the_exact_test_source(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            manifest = root / "base-interfaces.json"
            manifest.write_text("checked interfaces")
            source = root / "mixed.rs"
            source.write_text("reviewed mixed test source")
            log = root / "mixed.log"
            log.write_text("test portable_case ... ok\ntest native_case ... ok\n"
                           "test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out;\n")
            receipt = root / "invocations/mixed/case.json"
            receipt.parent.mkdir(parents=True)
            receipt.write_text(json.dumps({"test": "portable_case", "invocation": {
                "compiler": str(root / "frontend.py"), "interfaces": str(manifest)}}))
            row = dict(name="mixed", status="passed", source=str(source), exit_code=0,
                       log=str(log), log_sha256=checklist.digest(log),
                       invocation_receipts={receipt.name: checklist.digest(receipt)})
            interfaces_hash = checklist.digest(manifest)
            review = {"mixed": {"source_sha256": checklist.digest(source),
                                "cases": {"native_case": "direct Rust API assertion"}}}
            self.assertEqual(checklist.published_stage3_status(row, root, interfaces_hash)[0], "recheck")
            self.assertEqual(checklist.published_stage3_status(row, root, interfaces_hash, review)[0], "verified")
            source.write_text("changed test source")
            self.assertEqual(checklist.published_stage3_status(row, root, interfaces_hash, review)[0], "recheck")

    def test_copied_inputs_do_not_follow_live_file_links(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp) / "repo"
            self.fixture(root)
            (root / "compiler/src/alias.rs").symlink_to("main.rs")
            snapshot, manifest = verify.capture(root, Path(temp) / "evidence")
            (root / "compiler/src/main.rs").write_text("new development")
            self.assertEqual((snapshot / "compiler/src/alias.rs").read_text(), "frozen source")
            self.assertFalse((snapshot / "compiler/src/alias.rs").is_symlink())
            verify.audit(snapshot, manifest)

    def test_build_caches_and_brand_assets_are_not_source_inputs(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp) / "repo"
            self.fixture(root)
            for name in ("compiler/target", "compiler/runtime/target", "design/brand", "website"):
                (root / name).mkdir(parents=True)
                (root / name / "private-or-generated").write_text("excluded")
            snapshot, manifest = verify.capture(root, Path(temp) / "evidence")
            self.assertEqual(set(manifest["files"]), {"compiler/src/main.rs"})
            self.assertFalse((snapshot / "compiler/target").exists())
            self.assertFalse((snapshot / "compiler/runtime/target").exists())

    def test_internal_directory_links_are_copied_as_frozen_directories(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp) / "repo"
            self.fixture(root)
            (root / "tooling/corpus").mkdir()
            (root / "tooling/corpus/input.txt").write_text("original corpus")
            (root / "tooling/alias").symlink_to("corpus", target_is_directory=True)
            snapshot, manifest = verify.capture(root, Path(temp) / "evidence")
            (root / "tooling/corpus/input.txt").write_text("changed corpus")
            self.assertFalse((snapshot / "tooling/alias").is_symlink())
            self.assertEqual((snapshot / "tooling/alias/input.txt").read_text(), "original corpus")
            verify.audit(snapshot, manifest)

    def test_directory_link_cycles_are_rejected(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp) / "repo"
            self.fixture(root)
            (root / "tooling/loop").symlink_to(".", target_is_directory=True)
            with self.assertRaisesRegex(ValueError, "cycle"):
                verify.capture(root, Path(temp) / "evidence")

    def test_directory_links_cannot_capture_outside_the_source_tree(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp) / "repo"
            self.fixture(root)
            outside = Path(temp) / "outside"
            outside.mkdir()
            (root / "tooling/outside").symlink_to(outside, target_is_directory=True)
            with self.assertRaisesRegex(ValueError, "leaves the source tree"):
                verify.capture(root, Path(temp) / "evidence")

    def test_audit_rejects_mutation_and_new_source_files(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp) / "repo"
            self.fixture(root)
            snapshot, manifest = verify.capture(root, Path(temp) / "evidence")
            extra = snapshot / "bootstrap/New.cinch"
            extra.write_text("unexpected source")
            with self.assertRaises(ValueError):
                verify.audit(snapshot, manifest)
            extra.unlink()
            (snapshot / "compiler/src/main.rs").write_text("changed")
            with self.assertRaises(ValueError):
                verify.audit(snapshot, manifest)

    def test_compiler_and_runtime_use_distinct_private_cargo_targets(self):
        with tempfile.TemporaryDirectory() as temp:
            evidence = Path(temp)
            plan = verify.gates(evidence / "source", evidence)
            self.assertEqual(plan[0][0], "source-paths")
            compiler, runtime = [row for row in plan if row[0] in ("compiler", "runtime")]
            for name, cwd, command in (compiler, runtime):
                target = Path(command[command.index("--target-dir") + 1])
                self.assertEqual(target, evidence / "cargo" / name)
                self.assertNotEqual(target, cwd / "target")
                self.assertIn("--include-ignored", command)
            self.assertNotEqual(compiler[2], runtime[2])
            self_hosting = [row for row in plan if row[0] == "self-hosting"]
            self.assertEqual(len(self_hosting), 1)
            self.assertEqual(self_hosting[0][1], evidence / "source")
            self.assertEqual(self_hosting[0][2][-2:], ["tooling/bootstrap.py", "check"])
            self.assertEqual(plan[-2], self_hosting[0])
            self.assertEqual(plan[-1][0], "stage3-frontend")
            self.assertEqual(plan[-1][2][-2:], ["--cargo-target", str(evidence / "cargo/compiler")])

    def test_stage3_gate_requires_a_selected_invocation_in_every_portable_target(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            (root / "bootstrap").mkdir()
            (root / "compiler/tests").mkdir(parents=True)
            scope = dict(portable_targets=["portable"], interface_api_targets=["api"],
                         original_targets=["portable"], new_targets_beyond_original=["api"], current_total_targets=2)
            (root / "bootstrap/current-frontend-acceptance-scope.json").write_text(json.dumps(scope))
            compiler, manifest, objects, runtime = (root / name for name in ("cinch", "interfaces", "objects", "runtime"))
            for path in (compiler, manifest, objects, runtime):
                path.write_text(path.name)
            hashes = {path: checklist.digest(path) for path in (compiler, manifest, objects, runtime)}
            called = []
            record_api = False

            def cargo(command, cwd, env, stdout, stderr):
                target = command[command.index("--test") + 1]
                called.append(target)
                self.assertIn("--include-ignored", command)
                self.assertEqual(env["CINCH_TEST_API_EMIT_INTERFACES"], "1" if target == "api" else "0")
                self.assertEqual(env["CINCH_TEST_INTERFACES"], str(manifest))
                wrapper = Path(env["CINCH_TEST_COMPILER"])
                self.assertIn(f"compiler={str(compiler)!r}", wrapper.read_text().splitlines())
                if target != "api" or record_api:
                    receipts = Path(env["CINCH_TEST_INVOCATIONS"])
                    receipts.mkdir(parents=True)
                    (receipts / "case.json").write_text(json.dumps({
                        "test": "actual_case", "invocation": {"compiler": str(wrapper), "interfaces": str(manifest)}}))
                stdout.write(b"test actual_case ... ok\ntest result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out;\n")
                return type("Result", (), {"returncode": 0})()

            with patch.object(verify.bootstrap, "ROOT", root), \
                    patch.object(verify_stage3, "stage3_artifacts", return_value=(compiler, manifest, objects, runtime, hashes)), \
                    patch.object(verify_stage3.subprocess, "run", side_effect=cargo):
                self.assertEqual(verify_stage3.verify(root / "missing", root / "cargo"), 1)
                record_api = True
                self.assertEqual(verify_stage3.verify(root / "complete", root / "cargo"), 0)
            self.assertEqual(called, ["portable", "api", "portable", "api"])

    def test_stage3_preflights_the_source_of_reviewed_rust_assertions(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            (root / "bootstrap").mkdir()
            (root / "compiler/tests").mkdir(parents=True)
            source = root / "compiler/tests/portable.rs"
            source.write_text("changed direct assertion")
            scope = dict(portable_targets=["portable"], interface_api_targets=[],
                         original_targets=["portable"], new_targets_beyond_original=[],
                         current_total_targets=1, native_api_assertions={
                             "portable": {"source_sha256": "0" * 64, "cases": {"direct": "Rust-only assertion"}}
                         })
            (root / "bootstrap/current-frontend-acceptance-scope.json").write_text(json.dumps(scope))
            with patch.object(verify.bootstrap, "ROOT", root), \
                    patch.object(verify_stage3, "stage3_artifacts") as artifacts:
                with self.assertRaisesRegex(ValueError, "source review for portable"):
                    verify_stage3.verify(root / "evidence", root / "cargo")
                artifacts.assert_not_called()

    def test_stage3_adapter_links_the_library_and_runtime_from_its_paths(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            compiler, manifest, objects, runtime, library, source, ir, exe, report = [
                root / name for name in ("cinch", "interfaces.json", "objects.json", "runtime.a", "library.o",
                                         "Main.cinch", "Main.ll", "program", "link-report")]
            objects.write_text(json.dumps({"object": str(library)}))
            args = ["--compile-only", "--interfaces", *(str(path) for path in (manifest, source, ir, exe, report))]
            with patch.object(verify_stage3.subprocess, "call", return_value=0) as run, \
                    patch.dict(verify_stage3.os.environ, {"CINCH_TEST_API_EMIT_INTERFACES": "0", "CINCH_CLANG": "clang"}):
                self.assertEqual(verify_stage3.compile_client(str(compiler), str(objects), str(runtime), args), 0)
            frontend, link = (call.args[0] for call in run.call_args_list)
            self.assertEqual(frontend, [str(compiler), "--emit-ir", "--interfaces", manifest,
                                        "--objects", str(objects), source, ir])
            self.assertEqual(link[:6], ["clang", ir, str(library), str(runtime), "-o", exe])
            self.assertEqual(report.read_text(), "clang-exit: 0\n")

    def test_stage3_adapter_publishes_the_emitted_manifest_under_its_entry(self):
        for link_code in (0, 2):
            with self.subTest(link_code=link_code), tempfile.TemporaryDirectory() as temp:
                root = Path(temp)
                work = root / ".cinch-self-tests" / "1"
                work.mkdir(parents=True)
                compiler, interfaces, objects, runtime = [
                    root / name for name in ("cinch", "interfaces.json", "objects.json", "runtime.a")
                ]
                interfaces.write_text(json.dumps({"interfaces": []}))
                objects.write_text(json.dumps({"object": str(root / "library.o")}))
                source, ir, exe, report = [work / name for name in ("source.cinch", "output.ll", "program", "link-report")]
                manifest = b'{"entry":"Entry.Project","runtime_abi":7}\n'
                (work / "output.ll.manifest.json").write_bytes(manifest)
                args = ["--compile-only", "--interfaces", *(str(path) for path in (interfaces, source, ir, exe, report))]
                with patch.object(verify_stage3.subprocess, "call", side_effect=[0, link_code]), \
                        patch.dict(verify_stage3.os.environ, {"CINCH_TEST_API_EMIT_INTERFACES": "1"}):
                    self.assertEqual(verify_stage3.compile_client(str(compiler), str(objects), str(runtime), args), link_code)
                published = root / "build" / "Entry.Project.manifest.json"
                if link_code:
                    self.assertFalse(published.exists())
                else:
                    self.assertEqual(published.read_bytes(), manifest)
                self.assertFalse((root / "build" / "source.manifest.json").exists())

    def test_existing_cache_directory_cannot_be_reused_as_fresh_evidence(self):
        with tempfile.TemporaryDirectory() as temp:
            evidence = Path(temp) / "evidence"
            (evidence / "cargo").mkdir(parents=True)
            with self.assertRaises(ValueError):
                verify.verify(Path(temp) / "unused-source", evidence)
