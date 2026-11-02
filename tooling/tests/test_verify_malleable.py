from pathlib import Path
import sys
import tempfile
import unittest

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import verify_bootstrap
import verify_malleable


class CompleteExperimentVerification(unittest.TestCase):
    def test_frozen_experiment_inputs_detect_later_changes(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp) / "repo"
            for part in verify_bootstrap.PARTS:
                (root / part).mkdir(parents=True)
            original = root / "experiments/malleable-syntax/poc.py"
            original.parent.mkdir(parents=True)
            original.write_text("checked experiment")
            snapshot, manifest = verify_bootstrap.capture(root, Path(temp) / "evidence")
            self.assertIn("experiments/malleable-syntax/poc.py", manifest["files"])
            (snapshot / "experiments/malleable-syntax/poc.py").write_text("changed experiment")
            with self.assertRaises(ValueError):
                verify_bootstrap.audit(snapshot, manifest)

    def test_generated_experiment_files_do_not_change_input_identity(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            for part in verify_bootstrap.PARTS:
                (root / part).mkdir()
            before = verify_bootstrap.inputs(root)
            for name in ("examples/.generated/Main.cinch", "editor/tree-sitter-cinch-project/src/parser.c"):
                path = root / "experiments/malleable-syntax" / name
                path.parent.mkdir(parents=True, exist_ok=True)
                path.write_text("derived")
            self.assertEqual(verify_bootstrap.inputs(root), before)

    def test_syntax_gate_uses_the_preceding_frozen_compiler_build(self):
        with tempfile.TemporaryDirectory() as temp:
            evidence = Path(temp)
            gates = verify_bootstrap.gates(evidence / "source", evidence)
            names = [name for name, cwd, command in gates]
            self.assertLess(names.index("compiler-bin-build"), names.index("malleable-syntax"))
            command = next(command for name, cwd, command in gates if name == "malleable-syntax")
            self.assertEqual(command[command.index("--compiler") + 1], str(evidence / "cargo/compiler/debug/cinch"))
            self.assertEqual(command[command.index("--source") + 1], str(evidence / "source"))

    def test_empty_skipped_and_failed_runs_cannot_report_complete_success(self):
        for text, code in [("Ran 0 tests in 0s\nOK\n", 0),
                           ("Ran 39 tests in 1s\nOK (skipped=1)\n", 0),
                           ("Ran 39 tests in 1s\nFAILED\n", 1)]:
            self.assertEqual(verify_malleable.summary(text, code)["status"], "failed")
        result = verify_malleable.summary("Ran 39 tests in 1s\nOK\n", 0)
        self.assertEqual((result["status"], result["tests"], result["skipped"]), ("passed", 39, 0))


if __name__ == "__main__":
    unittest.main()
