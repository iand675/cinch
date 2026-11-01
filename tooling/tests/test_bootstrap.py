import io
import json
import os
from pathlib import Path
import sys
import tarfile
import tempfile
import unittest
from unittest.mock import patch

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import bootstrap


class BootstrapContract(unittest.TestCase):
    def test_stage_commands_keep_build_and_test_workers_serial(self):
        with patch.dict(os.environ, {"CARGO_BUILD_JOBS": "32", "RUST_TEST_THREADS": "32"}, clear=True):
            env = bootstrap.environment()
            self.assertEqual(env["CARGO_BUILD_JOBS"], "1")
            self.assertEqual(env["RUST_TEST_THREADS"], "1")
            for profile in ("DEV", "TEST", "RELEASE"):
                self.assertEqual(env[f"CARGO_PROFILE_{profile}_CODEGEN_UNITS"], "1")

    def test_source_identity_includes_names_and_bytes(self):
        first = {"B.cinch": b"b", "A.cinch": b"a"}
        self.assertEqual(bootstrap.source_hash(first), bootstrap.source_hash(dict(reversed(list(first.items())))))
        self.assertNotEqual(bootstrap.source_hash(first), bootstrap.source_hash({**first, "A.cinch": b"changed"}))
        self.assertNotEqual(bootstrap.source_hash(first), bootstrap.source_hash({"C.cinch": b"b", "A.cinch": b"a"}))
        self.assertEqual(bootstrap.bundle(first), bootstrap.bundle(dict(reversed(list(first.items())))))

    def test_heap_caps_are_preserved_and_never_disabled_or_raised(self):
        for value in ("1M", "2G", "2147483648", "0001k"):
            with patch.dict(os.environ, {"CINCH_HEAP_LIMIT": value}, clear=True):
                self.assertEqual(bootstrap.environment()["CINCH_HEAP_LIMIT"], value)
        for value in ("0", "3G", "2147483649", "bad", ""):
            with patch.dict(os.environ, {"CINCH_HEAP_LIMIT": value}, clear=True):
                with self.assertRaises(bootstrap.BootstrapError):
                    bootstrap.environment()

    def test_exit_zero_without_compiler_is_not_a_successful_stage(self):
        with tempfile.TemporaryDirectory() as temp:
            directory = Path(temp)
            log = directory / "build.log"
            log.write_text("")
            with self.assertRaises(bootstrap.BootstrapError):
                bootstrap.record_build(directory, {"built_by_stage": 1}, 0, log)
            self.assertEqual(bootstrap.read_json(directory / "stage.json")["status"], "failed")

    def test_self_build_requires_ir_and_report_as_well_as_executable(self):
        with tempfile.TemporaryDirectory() as temp:
            directory = Path(temp)
            exe = directory / "cinch"
            exe.write_bytes(b"compiler")
            exe.chmod(0o755)
            log = directory / "build.log"
            log.write_text("")
            with self.assertRaises(bootstrap.BootstrapError):
                bootstrap.record_build(directory, {"built_by_stage": 1}, 0, log)

    def test_cache_rechecks_outputs_and_inputs(self):
        with tempfile.TemporaryDirectory() as temp:
            directory = Path(temp)
            exe = directory / "cinch"
            exe.write_bytes(b"compiler")
            exe.chmod(0o755)
            (directory / "compiler.ll").write_text("IR")
            (directory / "base-interfaces.json").write_text("{}")
            (directory / "link-report.txt").write_text("clang-exit: 0\nartifact-exit: 1\n")
            log = directory / "build.log"
            log.write_text("")
            inputs = {"built_by_stage": 1, "source_sha256": "original"}
            bootstrap.record_build(directory, inputs, 0, log)
            self.assertTrue(bootstrap.cached(directory, inputs))
            self.assertFalse(bootstrap.cached(directory, {**inputs, "source_sha256": "changed"}))
            record = bootstrap.read_json(directory / "stage.json")
            del record["artifacts"]["compiler.ll"]
            bootstrap.write_json(directory / "stage.json", record)
            self.assertFalse(bootstrap.cached(directory, inputs), "a truncated manifest cannot waive required artifacts")
            bootstrap.record_build(directory, inputs, 0, log)
            (directory / "compiler.ll").write_text("corrupt")
            self.assertFalse(bootstrap.cached(directory, inputs))

    def test_seed_built_compiler_also_requires_its_frozen_source_bundle(self):
        with tempfile.TemporaryDirectory() as temp:
            directory = Path(temp)
            exe = directory / "cinch"
            exe.write_bytes(b"compiler")
            exe.chmod(0o755)
            log = directory / "build.log"
            log.write_text("")
            with self.assertRaises(bootstrap.BootstrapError):
                bootstrap.record_build(directory, {"built_by_stage": 0, "source_sha256": "compiler"}, 0, log)

    def test_base_manifest_excludes_compiler_sources_and_verifies_artifacts(self):
        with tempfile.TemporaryDirectory() as temp:
            directory = Path(temp)
            stage = directory / "stage1"
            artifacts = stage / "sources/build"
            artifacts.mkdir(parents=True)
            lock = {"revision": "fixture"}
            snapshot = bootstrap.digest(json.dumps(lock, sort_keys=True).encode())
            package = directory / "seed-sources" / snapshot / "compiler/prelude/cinch-base.cinchpkg"
            bootstrap.write_json(package, {"modules": [{"name": "Data.Core"}]})
            bootstrap.write_json(artifacts / "Data.Core.cinchif", {"imports": []})
            bootstrap.write_json(artifacts / "Bootstrap.Check.cinchif", {"imports": []})
            bootstrap.write_json(artifacts / "Main.manifest.json", {"modules": [
                {"module": "Data.Core", "iface_hash": "checked-base-hash"},
                {"module": "Bootstrap.Check", "iface_hash": "compiler-hash"},
            ]})
            (stage / "compiler.cinch").write_text("frozen compiler")
            compiler = stage / "cinch"
            compiler.write_text("compiler")
            compiler.chmod(0o755)
            log = stage / "build.log"
            log.write_text("")
            inputs = {"built_by_stage": 0, "source_sha256": "compiler"}
            bootstrap.record_build(stage, inputs, 0, log)
            with patch.object(bootstrap, "BUILD", directory), \
                    patch.object(bootstrap, "seed_inputs", return_value=(lock, b"")):
                manifest = bootstrap.base_interface_manifest()
                self.assertEqual(manifest, {"schema": 1, "interfaces": [[
                    "Data.Core", "checked-base-hash", str((artifacts / "Data.Core.cinchif").resolve())]]})
                bootstrap.write_json(artifacts / "Data.Core.cinchif", {"imports": [{"module": "Bootstrap.Check"}]})
                with self.assertRaisesRegex(bootstrap.BootstrapError, "verified stage-1"):
                    bootstrap.base_interface_manifest()
                bootstrap.record_build(stage, inputs, 0, log)
                with self.assertRaisesRegex(bootstrap.BootstrapError, "depends on compiler sources"):
                    bootstrap.base_interface_manifest()

    def test_successful_link_with_a_broken_compiler_entry_is_not_built(self):
        with tempfile.TemporaryDirectory() as temp:
            directory = Path(temp)
            exe = directory / "cinch"
            exe.write_bytes(b"compiler")
            exe.chmod(0o755)
            (directory / "compiler.ll").write_text("IR")
            (directory / "base-interfaces.json").write_text("{}")
            (directory / "link-report.txt").write_text("clang-exit: 0\nartifact-exit: 73\n")
            log = directory / "build.log"
            log.write_text("")
            with self.assertRaises(bootstrap.BootstrapError):
                bootstrap.record_build(directory, {"built_by_stage": 1}, 0, log)

    def test_seed_archive_cannot_escape_snapshot_or_add_links(self):
        for name, kind in (("../escape", tarfile.REGTYPE), ("compiler/link", tarfile.SYMTYPE)):
            archive = io.BytesIO()
            with tarfile.open(fileobj=archive, mode="w") as tar:
                item = tarfile.TarInfo(name)
                item.type = kind
                item.linkname = "/outside"
                tar.addfile(item)
            with tempfile.TemporaryDirectory() as temp:
                with self.assertRaises(bootstrap.BootstrapError):
                    bootstrap.extract_sources(archive.getvalue(), Path(temp))
                self.assertEqual(list(Path(temp).iterdir()), [])

    def test_unchanged_compiler_behavior_cannot_pass_the_changed_source_gate(self):
        with tempfile.TemporaryDirectory() as temp:
            directory = Path(temp)
            compiler = directory / "unchanged-compiler"
            compiler.write_text("#!/usr/bin/env python3\n"
                                "from pathlib import Path\nimport sys\n"
                                "Path(sys.argv[2] + '.diagnostics').write_text('bootstrap.check: type mismatch')\n"
                                "sys.exit(4)\n")
            compiler.chmod(0o755)
            product = directory / "changed-source" / "stage2"
            product.mkdir(parents=True)
            files = {"Bootstrap.Check.cinch": b'  rung = "bootstrap.check"\n'}
            with patch.object(bootstrap, "BUILD", directory), \
                    patch.object(bootstrap, "compiler_sources", return_value=files), \
                    patch.object(bootstrap, "build_self", return_value=compiler):
                with self.assertRaisesRegex(bootstrap.BootstrapError, "changed diagnostic behavior"):
                    bootstrap.check_changed_source()
            self.assertFalse((product / "probe" / "checked.json").exists())


if __name__ == "__main__":
    unittest.main()
