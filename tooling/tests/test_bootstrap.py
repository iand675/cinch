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
import bootstrap_objects


class BootstrapContract(unittest.TestCase):
    def test_seed_runtime_failure_cannot_fall_back_to_a_debug_archive(self):
        with tempfile.TemporaryDirectory() as temp:
            build = Path(temp)
            files = {"compiler/runtime/src/lib.rs": b"pinned runtime"}
            lock = {"revision": "seed", "source_sha256": bootstrap.source_hash(files)}
            archive = build / "seed-sources" / bootstrap.digest(json.dumps(lock, sort_keys=True).encode())
            runtime = archive / "compiler/runtime"
            (runtime / "src").mkdir(parents=True)
            (runtime / "src/lib.rs").write_bytes(files["compiler/runtime/src/lib.rs"])
            (runtime / "Cargo.toml").write_text("pinned package")
            debug = runtime / "target/debug/libcinchruntime.a"
            debug.parent.mkdir(parents=True)
            debug.write_bytes(b"working debug runtime")
            with patch.object(bootstrap, "BUILD", build), \
                    patch.object(bootstrap, "seed_inputs", return_value=(lock, b"")), \
                    patch.object(bootstrap, "seed_sources", return_value=files), \
                    patch.object(bootstrap, "run", return_value=1):
                with self.assertRaisesRegex(bootstrap.BootstrapError, "optimized seed runtime did not build"):
                    bootstrap.build_seed_runtime()
            self.assertEqual(debug.read_bytes(), b"working debug runtime")

    def test_stage1_cache_is_invalidated_when_its_seed_runtime_changes(self):
        with tempfile.TemporaryDirectory() as temp:
            build = Path(temp)
            stage = build / "stage1"
            (stage / "sources/build").mkdir(parents=True)
            files = {"Main.cinch": b"frozen compiler source"}
            builder = build / "seed"
            builder.write_bytes(b"same seed compiler")
            runtime = build / "runtime.a"
            runtime.write_bytes(b"optimized runtime A")
            inputs = {"protocol": bootstrap.PROTOCOL, "host": bootstrap.HOST,
                      "source_sha256": bootstrap.source_hash(files),
                      "builder_sha256": bootstrap.file_hash(builder), "built_by_stage": 0,
                      "shard_count": bootstrap.JOBS,
                      "seed_runtime_profile": "release", "seed_runtime_sha256": bootstrap.file_hash(runtime)}
            (stage / "compiler.cinch").write_bytes(bootstrap.bundle(files))
            (stage / "sources/build/Main.manifest.json").write_text("{}")
            compiler = stage / "cinch"
            compiler.write_bytes(b"previous compiler")
            compiler.chmod(0o755)
            seed_ir = compiler.with_suffix(".ll")
            seed_ir.write_bytes(b"old linker input")
            log = stage / "build.log"
            log.write_text("")
            bootstrap.record_build(stage, inputs, 0, log)

            def fail_seed_build(command, cwd, log, **kwargs):
                seed_ir.write_bytes(b"failed linker input")
                return 9

            with patch.object(bootstrap, "BUILD", build), \
                    patch.object(bootstrap, "build_seed", return_value=builder), \
                    patch.object(bootstrap, "compiler_sources", return_value=files), \
                    patch.object(bootstrap, "build_seed_runtime", return_value=runtime), \
                    patch.object(bootstrap, "run", side_effect=fail_seed_build) as invoke:
                self.assertEqual(bootstrap.build_stage(1), compiler)
                invoke.assert_not_called()
                self.assertFalse(seed_ir.exists())
                runtime.write_bytes(b"optimized runtime B")
                with self.assertRaises(bootstrap.BootstrapError):
                    bootstrap.build_stage(1)
                self.assertEqual(invoke.call_count, 1)
                self.assertFalse(compiler.exists(), "a failed runtime-triggered rebuild left a stale compiler")
                self.assertEqual(seed_ir.read_bytes(), b"failed linker input")

    def test_stage1_releases_seed_ir_after_a_verified_link(self):
        with tempfile.TemporaryDirectory() as temp:
            build = Path(temp)
            builder, runtime, files = self.stage_fixture(build)
            compiler = build / "stage1/cinch"
            seed_ir = compiler.with_suffix(".ll")

            def seed_build(command, cwd, log, **kwargs):
                seed_ir.write_bytes(b"linker input")
                compiler.write_bytes(b"linked compiler")
                compiler.chmod(0o755)
                (cwd / "build").mkdir()
                (cwd / "build/Main.manifest.json").write_text("{}")
                log.write_text("")
                return 0

            with patch.object(bootstrap, "BUILD", build), \
                    patch.object(bootstrap, "build_seed", return_value=builder), \
                    patch.object(bootstrap, "compiler_sources", return_value=files), \
                    patch.object(bootstrap, "build_seed_runtime", return_value=runtime), \
                    patch.object(bootstrap, "run", side_effect=seed_build):
                self.assertEqual(bootstrap.build_stage(1), compiler)
            self.assertFalse(seed_ir.exists())
            self.assertTrue(bootstrap.cached(compiler.parent, bootstrap.read_json(compiler.parent / "stage.json")["inputs"]))

    def test_seed_stage_imports_every_bundled_compiler_module(self):
        files = bootstrap.collect_compiler_sources()
        entry = files["Main.cinch"].decode()
        modules = sorted(name[:-6] for name in files if name.endswith(".cinch") and name != "Main.cinch")
        for index, module in enumerate(modules):
            self.assertEqual(entry.count(f"  import {module} as CompilerSource{index}\n"), 1)
        original = (bootstrap.ROOT / "bootstrap/programs/Run.Pipeline.cinch").read_text()
        without_imports = "".join(line for line in entry.splitlines(keepends=True)
                                  if not line.startswith("  import ") or " as CompilerSource" not in line)
        self.assertEqual(without_imports, original)

    def test_the_compiler_source_set_carries_the_parmap_prelude_module(self):
        # `parMap` lowers through `Data.Array`'s `array-from-list` and
        # `array-to-list` (`Bootstrap.Infer.par-map-array-call`), and that
        # module is not in the embedded base package, so a `parMap` anywhere
        # in the compiler resolves only when the module is in the same source
        # set. The bundle has to carry the module's real bytes, because the
        # fixed point publishes its interface and the seed checks the closure.
        files = bootstrap.collect_compiler_sources()
        self.assertIn("Data.Array", bootstrap.COMPILER_PRELUDE)
        source = bootstrap.ROOT / "compiler/prelude/Data/Array.cinch"
        self.assertEqual(files["Data.Array.cinch"], source.read_bytes())
        self.assertEqual(files["Data.Array.cinch"].count(b"module Data.Array where"), 1)
        # It is imported like every other bundled module, under a qualified
        # alias, so its exports stay out of the entry's own scope.
        entry = files["Main.cinch"].decode()
        modules = sorted(name[:-6] for name in files if name.endswith(".cinch") and name != "Main.cinch")
        self.assertIn("Data.Array", modules)
        self.assertEqual(entry.count(f"  import Data.Array as CompilerSource{modules.index('Data.Array')}\n"), 1)
        # The base package must not also provide it: a module that is both
        # embedded and a source would be a duplicate definition, and the
        # whole point of the entry is that the base package lacks it.
        package = json.loads((bootstrap.ROOT / "compiler/prelude/cinch-base.cinchpkg").read_text())
        self.assertNotIn("Data.Array", [module["name"] for module in package["modules"]])

    def test_fixed_point_requires_matching_executables(self):
        with tempfile.TemporaryDirectory() as temp:
            build = Path(temp)
            for stage in ("stage2", "stage3"):
                directory = build / stage
                directory.mkdir()
                (directory / "cinch").write_bytes(stage.encode())
                (directory / "compiler.ll").write_bytes(b"identical IR")
                (directory / "Main.cinchif").write_bytes(b"identical interface")
            with patch.object(bootstrap, "BUILD", build), patch.object(bootstrap, "smoke"), \
                    patch.object(bootstrap, "compiler_sources", return_value={}), \
                    patch.object(bootstrap, "require_compiler_interfaces"), \
                    patch.object(bootstrap, "check_interface_consumer") as consumer, \
                    patch.object(bootstrap, "check_changed_source") as changed:
                with self.assertRaisesRegex(bootstrap.BootstrapError, "compiler executables differ"):
                    bootstrap.check_fixed_point()
                consumer.assert_not_called()
                changed.assert_not_called()
                (build / "stage3/cinch").write_bytes(b"stage2")
                bootstrap.check_fixed_point()
                consumer.assert_called_once()
                changed.assert_called_once()

    def test_base_interfaces_cannot_stand_in_for_compiler_fixed_point_contracts(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            files = {"Main.cinch": b"source", "Bootstrap.Check.cinch": b"source"}
            bootstrap.write_json(root / "Data.Core.cinchif", {"module": "Data.Core"})
            with self.assertRaisesRegex(bootstrap.BootstrapError, "missing actual compiler interfaces"):
                bootstrap.require_compiler_interfaces(root, files)
            for name in ["Main", "Bootstrap.Check"]:
                bootstrap.write_json(root / (name + ".cinchif"), {"module": name})
            with self.assertRaisesRegex(bootstrap.BootstrapError, "invalid actual compiler interface"):
                bootstrap.require_compiler_interfaces(root, files)
            template = bootstrap.read_json(Path(__file__).parent / "fixtures/checked-interface.json")
            runtime_abi = bootstrap.library_provider_lock()["runtime_abi"]
            for name in ["Main", "Bootstrap.Check"]:
                doc = {**template, "module": name, "runtime_abi": runtime_abi}
                (root / (name + ".cinchif")).write_text(json.dumps(doc, ensure_ascii=False, sort_keys=True, separators=(",", ":")))
            bootstrap.require_compiler_interfaces(root, files)
            wrong_abi = 5 if runtime_abi == 6 else 6
            (root / "Main.cinchif").write_text(json.dumps({**template, "module": "Main", "runtime_abi": wrong_abi}, ensure_ascii=False, sort_keys=True, separators=(",", ":")))
            with self.assertRaisesRegex(bootstrap.BootstrapError, "wrong runtime ABI"):
                bootstrap.require_compiler_interfaces(root, files)
            (root / "Main.cinchif").write_text(json.dumps({**template, "module": "Main", "runtime_abi": runtime_abi}, ensure_ascii=False, sort_keys=True, separators=(",", ":")))
            bootstrap.write_json(root / "extra.cinchif", {"module": "Bootstrap.Check"})
            with self.assertRaisesRegex(bootstrap.BootstrapError, "duplicate canonical compiler interface"):
                bootstrap.require_compiler_interfaces(root, files)

    def test_base_manifest_excludes_compiler_sources_and_verifies_artifacts(self):
        with tempfile.TemporaryDirectory() as temp:
            directory = Path(temp)
            artifacts = directory / "library/build"
            base = {"module": "Data.Core", "imports": []}
            row = {"module": "Data.Core", "iface_hash": "checked-base-hash", "interface": base}
            metadata = {"interfaces": [row]}
            package = {"modules": [{"name": "Data.Core"}]}
            base_file = artifacts / "Data.Core.cinchif"
            bootstrap.write_json(base_file, base)
            bootstrap.write_json(artifacts / "Bootstrap.Check.cinchif", {"module": "Bootstrap.Check"})
            manifest = bootstrap_objects.admitted_interfaces(bootstrap, metadata, package, directory)
            self.assertEqual(manifest, {"schema": 1, "interfaces": [
                ["Data.Core", "checked-base-hash", str(base_file.resolve())]]})
            changed = {**base, "imports": [{"module": "Bootstrap.Check"}]}
            bootstrap.write_json(base_file, changed)
            with self.assertRaisesRegex(bootstrap.BootstrapError, "differs from its actual checked interface"):
                bootstrap_objects.admitted_interfaces(bootstrap, metadata, package, directory)
            row["interface"] = changed
            with self.assertRaisesRegex(bootstrap.BootstrapError, "depends on compiler implementation"):
                bootstrap_objects.admitted_interfaces(bootstrap, metadata, package, directory)

    def test_runtime_abi_preparation_is_exact_and_part_of_source_identity(self):
        live = (bootstrap.ROOT / "bootstrap/Bootstrap.RuntimeAbi.cinch").read_bytes()
        self.assertEqual(live, bootstrap.runtime_abi_source(7))
        pinned = bootstrap.collect_compiler_sources()
        lock = bootstrap.library_provider_lock()
        self.assertEqual(pinned["Bootstrap.RuntimeAbi.cinch"], bootstrap.runtime_abi_source(lock["runtime_abi"]))
        prepared = {}
        for runtime_abi in (5, 6, 7):
            with self.subTest(runtime_abi=runtime_abi), patch.object(bootstrap, "library_provider_lock", return_value={**lock, "runtime_abi": runtime_abi}):
                prepared[runtime_abi] = bootstrap.collect_compiler_sources()
            self.assertEqual(prepared[runtime_abi]["Bootstrap.RuntimeAbi.cinch"], bootstrap.runtime_abi_source(runtime_abi))
            self.assertIn(f"runtime-abi = {runtime_abi}".encode(), bootstrap.bundle(prepared[runtime_abi]))
            self.assertIn(f"runtime-abi-int = {runtime_abi}".encode(), prepared[runtime_abi]["Bootstrap.RuntimeAbi.cinch"])
        self.assertEqual(len({bootstrap.source_hash(files) for files in prepared.values()}), 3)
        for invalid in (None, True, "5", 0, 4, 8):
            with self.assertRaises(bootstrap.BootstrapError):
                bootstrap.runtime_abi_source(invalid)
        with patch.object(bootstrap, "library_provider_lock", return_value={"runtime_abi": 8}):
            with self.assertRaises(bootstrap.BootstrapError):
                bootstrap.collect_compiler_sources()

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

    def provider_fixture(self, runtime):
        library = runtime.parent / "library.o"
        metadata = runtime.parent / "library.abi.json"
        if not library.exists():
            library.write_bytes(b"verified library-only object")
            metadata.write_bytes(b"verified library-only metadata")
        return {"runtime": runtime, "object": library,
                "interfaces": {"schema": 1, "interfaces": []},
                "configuration": {"schema": 1, "target": "test-host", "object": str(library),
                                  "metadata": str(metadata), "object_sha256": bootstrap.file_hash(library),
                                  "metadata_sha256": bootstrap.file_hash(metadata)},
                "identity": {"producer_sha256": "verified library producer", "runtime_abi": 5}}

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
                    self.assertEqual(args[1:3], ["--emit-ir-with-interfaces", "--interfaces"])
                    self.assertEqual(len(args), 8)
                    self.assertEqual(args[4:6], ["--objects", directory / "objects.json"])
                    Path(args[-1]).write_text("typed IR")
                    return 0
                if str(args[0]) == "selected-clang":
                    self.assertTrue(append)
                    self.assertEqual(args[1:3], ["-O2", "-Wno-override-module"])
                    # Every stage compiles its module from one content
                    # addressed path, never from the stage directory.
                    self.assertEqual(args[3].name, "compiler.ll")
                    self.assertIn("shards", args[3].parts)
                    self.assertEqual(args[3].read_text(), "typed IR")
                    self.assertEqual(args[4:6], [runtime.parent / "library.o", runtime])
                    exe = directory / "cinch"
                    exe.write_bytes(b"linked compiler")
                    exe.chmod(0o755)
                    return 0
                self.assertEqual(args, [directory / "cinch"])
                self.assertTrue(append)
                return 1

            with patch.object(bootstrap, "runtime_provider", side_effect=lambda: self.provider_fixture(runtime)), \
                    patch.object(bootstrap, "BUILD", root), \
                    patch.dict(os.environ, {"CINCH_CLANG": "selected-clang"}), \
                    patch.object(bootstrap, "run", side_effect=command):
                bootstrap.build_self(builder, files, directory, 1)
                self.assertEqual(len(calls), 3)
                self.assertEqual((directory / "link-report.txt").read_text(), "clang-exit: 0\nartifact-exit: 1\n")
                inputs = bootstrap.read_json(directory / "stage.json")["inputs"]
                self.assertEqual(inputs["frontend_mode"], "typed-ir-objects")
                self.assertEqual(inputs["interface_output"], "checked-schema10")
                self.assertEqual(inputs["runtime_archive_sha256"], bootstrap.file_hash(runtime))
                bootstrap.build_self(builder, files, directory, 1)
                self.assertEqual(len(calls), 3, "identical frozen inputs should reuse the verified compiler")
                runtime.write_bytes(b"different runtime object")
                bootstrap.build_self(builder, files, directory, 1)
                self.assertEqual(len(calls), 6, "a changed runtime object must invalidate the cached compiler")
                (root / "library.o").write_bytes(b"different native library object")
                bootstrap.build_self(builder, files, directory, 1)
                self.assertEqual(len(calls), 9, "changed dependency code must invalidate the cached compiler")
                (directory / "objects.json").write_text("corrupted stage configuration")
                bootstrap.build_self(builder, files, directory, 1)
                self.assertEqual(len(calls), 12, "a changed admission configuration must invalidate the cache")

    def test_typed_refusal_never_calls_a_linker_or_falls_back_to_native(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            builder, runtime, files = self.stage_fixture(root)
            directory = root / "stage2"

            def refused(args, cwd, log, **kwargs):
                self.assertEqual(args[0], builder)
                self.assertEqual(args[1], "--emit-ir-with-interfaces")
                log.write_text("")
                (directory / "compiler.ll.diagnostics").write_text("bootstrap.runtime-ir:1:1: unavailable layout")
                return 5

            with patch.object(bootstrap, "runtime_provider", side_effect=lambda: self.provider_fixture(runtime)), \
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

            with patch.object(bootstrap, "runtime_provider", side_effect=lambda: self.provider_fixture(runtime)), \
                    patch.object(bootstrap, "BUILD", root), \
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

            with patch.object(bootstrap, "runtime_provider", side_effect=lambda: self.provider_fixture(runtime)), \
                    patch.object(bootstrap, "BUILD", root), \
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
                    patch.object(bootstrap, "build_self", return_value=compiler) as build:
                with self.assertRaisesRegex(bootstrap.BootstrapError, "changed diagnostic behavior"):
                    bootstrap.check_changed_source()
                self.assertEqual(build.call_args.args[0], directory / "stage2" / "cinch")
                self.assertEqual(build.call_args.args[3], 2)
            self.assertFalse((product / "probe" / "checked.json").exists())


if __name__ == "__main__":
    unittest.main()
