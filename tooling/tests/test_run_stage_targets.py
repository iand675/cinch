from pathlib import Path
import subprocess
import sys
import tempfile
import unittest
from unittest.mock import patch

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import check_compare
import run_stage_targets as runner


class StageTargetCounts(unittest.TestCase):
    def run_log(self, text, code):
        def run(*args, **kwargs):
            kwargs["stdout"].write(text.encode())
            return subprocess.CompletedProcess(args[0], code)

        with tempfile.TemporaryDirectory() as temp, patch.object(runner.subprocess, "run", run):
            return runner.run_in_slot("fixture", {}, Path(temp), Path(temp) / "target")

    def test_child_output_cannot_hide_a_successful_test(self):
        row = self.run_log("test first ... child output\nok\n"
                           "test second ... ok\n"
                           "test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out;\n", 0)
        self.assertEqual(row, dict(target="fixture", exit=0, passed=2, failed=[], failed_count=0))

    def test_child_output_cannot_hide_a_failed_test(self):
        row = self.run_log("test failed_case ... child output\nFAILED\n"
                           "test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out;\n", 101)
        self.assertEqual(row, dict(target="fixture", exit=101, passed=0, failed=[], failed_count=1))

    def test_aborted_runs_keep_the_exit_status_and_partial_counts(self):
        row = self.run_log("test first ... ok\nprocess aborted\n", 101)
        self.assertEqual(row, dict(target="fixture", exit=101, passed=1, failed=[], failed_count=0))

    def test_existing_failure_names_are_preserved(self):
        row = self.run_log("test failed_case ... FAILED\n"
                           "test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out;\n", 101)
        self.assertEqual(row["failed"], ["failed_case"])
        self.assertEqual(row["failed_count"], 1)


class CorpusEmitMode(unittest.TestCase):
    def test_fresh_runs_probe_the_selected_compiler(self):
        for status, expected in ((1, "--emit-ir"), (2, "--emit-ir-only")):
            compiler = Path("/selected/cinch")
            with self.subTest(status=status), patch.object(
                check_compare.subprocess, "run", return_value=subprocess.CompletedProcess([], status)
            ) as probe:
                self.assertEqual(check_compare.corpus_emit_mode(compiler, []), expected)
                self.assertEqual(probe.call_args.args[0][:2], [compiler, "--emit-ir-only"])

    def test_resume_reuses_the_recorded_command(self):
        rows = [{"frontend": "rust"}, {"frontend": "native", "command": ["cinch", "--emit-ir"]}]
        with patch.object(check_compare.subprocess, "run") as probe:
            self.assertEqual(check_compare.corpus_emit_mode(Path("cinch"), rows), "--emit-ir")
            probe.assert_not_called()

    def test_unexpected_status_fails_the_run(self):
        with patch.object(check_compare.subprocess, "run", return_value=subprocess.CompletedProcess([], 4)), self.assertRaises(RuntimeError):
            check_compare.corpus_emit_mode(Path("cinch"), [])


if __name__ == "__main__":
    unittest.main()
