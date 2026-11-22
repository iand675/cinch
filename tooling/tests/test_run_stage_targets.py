from pathlib import Path
import json
import subprocess
import sys
import tempfile
import unittest
from unittest.mock import patch

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import check_compare
import run_stage_targets as runner
from verify_stage3 import target_artifacts_match


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


class StageTargetInputs(unittest.TestCase):
    def test_reused_target_checks_package_bytes_and_absolute_paths(self):
        with tempfile.TemporaryDirectory() as temp:
            output = Path(temp)
            paths = [output / name for name in ("objects.json", "library.o", "library.cpk")]
            for path in paths:
                path.parent.mkdir(parents=True, exist_ok=True)
                path.write_text(path.name)
            record = {"directory": str(output.resolve()),
                      "sha256": {str(path.relative_to(output)): runner.bootstrap.file_hash(path) for path in paths}}
            self.assertTrue(target_artifacts_match(output, record))
            self.assertFalse(target_artifacts_match(output, dict(record, directory="moved")))
            paths[-1].write_text("a different producer interface")
            self.assertFalse(target_artifacts_match(output, record))
            paths[-1].unlink()
            self.assertFalse(target_artifacts_match(output, record))

    def test_later_stages_use_the_recorded_inputs_even_when_the_lane_pin_changes(self):
        with tempfile.TemporaryDirectory() as temp, patch.object(runner.bootstrap, 'runtime_provider') as prepare:
            directory = Path(temp)
            compiler, manifest, objects, library, metadata, runtime = (
                directory / name for name in ('cinch', 'base.cpk', 'base-objects.json', 'library.o',
                                               'library.abi.json', 'libcinchruntime.a'))
            for path in (compiler, manifest, library, metadata, runtime):
                path.write_bytes(path.name.encode())
            digest = runner.bootstrap.file_hash
            objects.write_text(json.dumps(dict(object=str(library), metadata=str(metadata),
                                               object_sha256=digest(library), metadata_sha256=digest(metadata))))
            inputs = dict(interface_set_sha256=digest(manifest), object_configuration_sha256=digest(objects),
                          runtime_archive_sha256=digest(runtime))
            (directory / 'stage.json').write_text(json.dumps(dict(status='built', inputs=inputs,
                                                                  executable_sha256=digest(compiler))))
            self.assertEqual(runner.selected_stage_inputs(3, directory, compiler),
                             (manifest.read_bytes(), objects.read_bytes(), library, runtime))
            library.write_bytes(b'a different generation')
            with self.assertRaisesRegex(runner.bootstrap.BootstrapError, 'differs from its receipt'):
                runner.selected_stage_inputs(3, directory, compiler)
            prepare.assert_not_called()


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
