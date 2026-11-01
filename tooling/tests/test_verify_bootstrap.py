from pathlib import Path
import sys
import tempfile
import unittest

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import verify_bootstrap as verify


class FrozenVerification(unittest.TestCase):
    def fixture(self, root):
        for part in verify.PARTS:
            (root / part).mkdir(parents=True)
        (root / "compiler/src").mkdir()
        (root / "compiler/src/main.rs").write_text("frozen source")

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

    def test_existing_cache_directory_cannot_be_reused_as_fresh_evidence(self):
        with tempfile.TemporaryDirectory() as temp:
            evidence = Path(temp) / "evidence"
            (evidence / "cargo").mkdir(parents=True)
            with self.assertRaises(ValueError):
                verify.verify(Path(temp) / "unused-source", evidence)
