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

    def stage_fixture(self, directory):
        builder = directory / "selected-cinch"
        builder.write_bytes(b"Cinch frontend identity")
        runtime = directory / "libcinchruntime.a"
        runtime.write_bytes(b"pinned runtime identity")
        return builder, runtime, {"Main.cinch": b"source supplied only to Cinch"}

    def test_typed_self_build_uses_only_cinch_then_links_and_checks_the_compiler_entry(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            builder, runtime, files = self.stage_fixture(root)
            directory = root / "stage2"
            calls = []

            def command(args, cwd, log, *, append=False):
                calls.append([str(arg) for arg in args])
                log.write_text("observed selected command")
                if args[0] == builder:
                    self.assertEqual(args[1:3], ["--emit-ir", "--interfaces"])
                    self.assertEqual(len(args), 6)
                    Path(args[-1]).write_text("typed IR")
                    return 0
                if str(args[0]) == "selected-clang":
                    self.assertTrue(append)
                    self.assertEqual(args[1:3], [directory / "compiler.ll", runtime])
                    exe = directory / "cinch"
                    exe.write_bytes(b"linked compiler")
                    exe.chmod(0o755)
                    return 0
                self.assertEqual(args, [directory / "cinch"])
                self.assertTrue(append)
                return 1

            with patch.object(bootstrap, "runtime_archive", return_value=runtime), \
                    patch.object(bootstrap, "base_interface_manifest", return_value={"schema": 1, "interfaces": []}), \
                    patch.dict(os.environ, {"CINCH_CLANG": "selected-clang"}), \
                    patch.object(bootstrap, "run", side_effect=command):
                bootstrap.build_self(builder, files, directory, 1)
                self.assertEqual(len(calls), 3)
                self.assertEqual((directory / "link-report.txt").read_text(), "clang-exit: 0\nartifact-exit: 1\n")
                inputs = bootstrap.read_json(directory / "stage.json")["inputs"]
                self.assertEqual(inputs["frontend_mode"], "typed-ir")
                self.assertEqual(inputs["runtime_archive_sha256"], bootstrap.file_hash(runtime))
                bootstrap.build_self(builder, files, directory, 1)
                self.assertEqual(len(calls), 3, "identical frozen inputs should reuse the verified compiler")
                runtime.write_bytes(b"different runtime object")
                bootstrap.build_self(builder, files, directory, 1)
                self.assertEqual(len(calls), 6, "a changed runtime object must invalidate the cached compiler")

    def test_typed_refusal_never_calls_a_linker_or_falls_back_to_native(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            builder, runtime, files = self.stage_fixture(root)
            directory = root / "stage2"

            def refused(args, cwd, log, **kwargs):
                self.assertEqual(args[0], builder)
                self.assertEqual(args[1], "--emit-ir")
                log.write_text("")
                (directory / "compiler.ll.diagnostics").write_text("bootstrap.runtime-ir:1:1: unavailable layout")
                return 5

            with patch.object(bootstrap, "runtime_archive", return_value=runtime), \
                    patch.object(bootstrap, "base_interface_manifest", return_value={"schema": 1, "interfaces": []}), \
                    patch.object(bootstrap, "run", side_effect=refused) as run:
                with self.assertRaisesRegex(bootstrap.BootstrapError, "unavailable layout"):
                    bootstrap.build_self(builder, files, directory, 1)
                self.assertEqual(run.call_count, 1)
                self.assertFalse((directory / "cinch").exists())
                self.assertFalse((directory / "link-report.txt").exists())

    def test_link_failure_is_visible_even_when_frontend_diagnostics_are_empty(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            builder, runtime, files = self.stage_fixture(root)
            directory = root / "stage2"

            def command(args, cwd, log, *, append=False):
                if args[0] == builder:
                    (directory / "compiler.ll").write_text("typed IR")
                    (directory / "compiler.ll.diagnostics").write_text("")
                    log.write_text("")
                    return 0
                log.write_text("linker rejected the runtime archive")
                return 19

            with patch.object(bootstrap, "runtime_archive", return_value=runtime), \
                    patch.object(bootstrap, "base_interface_manifest", return_value={"schema": 1, "interfaces": []}), \
                    patch.object(bootstrap, "run", side_effect=command) as run:
                with self.assertRaisesRegex(bootstrap.BootstrapError, "linker rejected"):
                    bootstrap.build_self(builder, files, directory, 1)
                self.assertEqual(run.call_count, 2)
                self.assertEqual((directory / "link-report.txt").read_text(), "clang-exit: 19\n")

    def test_linker_success_without_a_runnable_compiler_is_a_failed_stage(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            builder, runtime, files = self.stage_fixture(root)
            directory = root / "stage2"

            def command(args, cwd, log, **kwargs):
                log.write_text("")
                if args[0] == builder:
                    (directory / "compiler.ll").write_text("typed IR")
                return 0

            with patch.object(bootstrap, "runtime_archive", return_value=runtime), \
                    patch.object(bootstrap, "base_interface_manifest", return_value={"schema": 1, "interfaces": []}), \
                    patch.object(bootstrap, "run", side_effect=command) as run:
                with self.assertRaisesRegex(bootstrap.BootstrapError, "without a runnable compiler"):
                    bootstrap.build_self(builder, files, directory, 1)
                self.assertEqual(run.call_count, 2)

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
