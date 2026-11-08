import json
import os
from pathlib import Path
import sys
import tempfile
import unittest
from unittest.mock import patch

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import bootstrap
import bootstrap_objects


class LibraryProviderContract(unittest.TestCase):
    def test_demands_are_actual_concrete_vectors_and_defining_identities(self):
        encode = lambda symbol: "i." + "".join(str(ord(char)) + "_" for char in symbol)
        function, factory, signature = map(encode, ["Data.List.reverse", "Data.List.FromList_List", "Library.Π"])
        lines = [f"bootstrap.runtime-ir:1:2: checked external variant is absent from the runtime object: {function} [T]",
                 f"bootstrap.runtime-ir:1:3: checked external variant is absent from the runtime object: {function} [R]",
                 f"bootstrap.runtime-ir:1:4: checked factory variant is absent from the runtime object: {factory} [O]",
                 f"bootstrap.runtime-ir:2:1: method representation has no planned declaring-class slot: {signature}:3 [RT]",
                 f"bootstrap.check:1:2: checked external variant is absent from the runtime object: {function} [T]",
                 f"bootstrap.runtime-ir:1:2: checked external variant is absent from the runtime object: {function} [P]",
                 "bootstrap.runtime-ir:1:2: checked external variant is absent from the runtime object: i.55296_ [R]"]
        roots = bootstrap_objects.requested_roots("\n".join(lines + lines))
        self.assertEqual(roots, {"schema": 1,
                                "units": [{"symbol": "Data.List.reverse", "classes": "R"}, {"symbol": "Data.List.reverse", "classes": "T"}],
                                "dictionaries": [{"symbol": "Data.List.FromList_List", "classes": "O"}],
                                "methods": [{"class": "Library.Π", "slot": 3, "own": "RT"}]})

    def fixture(self, directory):
        root = directory / "checkout"
        (root / "bootstrap").mkdir(parents=True)
        lock = {"schema": 1, "runtime_abi": 5, "revision": "a" * 40, "source_sha256": "b" * 64}
        bootstrap.write_json(root / "bootstrap/library-provider.lock.json", lock)
        bootstrap.write_json(root / "bootstrap/library-object-roots.json", {"schema": 1, "units": []})
        snapshot = directory / "pinned-source"
        (snapshot / "compiler/prelude").mkdir(parents=True)
        bootstrap.write_json(snapshot / "compiler/prelude/cinch-base.cinchpkg", {"modules": [{"name": "Data.Core"}]})
        return root, snapshot

    def test_only_generated_library_root_reaches_native_and_corrupt_inputs_rebuild(self):
        with tempfile.TemporaryDirectory() as temp:
            directory = Path(temp)
            root, snapshot = self.fixture(directory)
            calls = []

            def run(args, cwd, log, **kwargs):
                calls.append(args)
                log.write_text("observed producer command")
                if str(args[0]) == "cargo":
                    target = Path(args[args.index("--target-dir") + 1])
                    (target / "release").mkdir(parents=True, exist_ok=True)
                    if "--bin" in args:
                        binary = target / "release/cinch"
                        binary.write_bytes(b"pinned producer binary")
                        binary.chmod(0o755)
                    else:
                        (target / "release/libcinchruntime.a").write_bytes(b"matching runtime")
                else:
                    self.assertEqual(args[1], "emit-object")
                    self.assertEqual(Path(args[2]).read_bytes(), bootstrap_objects.ENTRY)
                    self.assertNotIn(b"Bootstrap.", Path(args[2]).read_bytes())
                    output = Path(args[-1])
                    output.write_bytes(b"native library code")
                    output.with_suffix(".ll").write_bytes(b"native library LLVM")
                    rows = []
                    for name in ["Data.Core", "CinchBaseObject"]:
                        doc = {"module": name, "imports": []}
                        interface = Path(args[2]).parent / "build" / (name + ".cinchif")
                        bootstrap.write_json(interface, doc)
                        rows.append({"module": name, "iface_hash": name + "-hash", "interface": doc})
                    metadata = {"schema": 1, "runtime_abi": 5, "word_size": 64, "endian": "little",
                                "reclaim": True, "escaped_descriptors": False, "target": "test-host",
                                "interfaces": rows, "units": [{"symbol": "CinchBaseObject.marker"}]}
                    bootstrap.write_json(output.with_suffix(".abi.json"), metadata)
                return 0

            with patch.object(bootstrap, "ROOT", root), patch.object(bootstrap, "BUILD", directory / "build"), \
                    patch.object(bootstrap, "seed_inputs", return_value=({"runtime_abi": 5}, b"")), \
                    patch.object(bootstrap_objects, "source_snapshot", return_value=snapshot), \
                    patch.object(bootstrap, "run", side_effect=run):
                provider = bootstrap_objects.provider(bootstrap)
                self.assertEqual(len(calls), 3)
                self.assertEqual([row[0] for row in provider["interfaces"]["interfaces"]], ["Data.Core"])
                self.assertEqual(provider["configuration"]["object_sha256"], bootstrap.file_hash(provider["object"]))
                bootstrap_objects.provider(bootstrap)
                self.assertEqual(len(calls), 3)
                contract = Path(provider["interfaces"]["interfaces"][0][2])
                contract.write_bytes(b"corrupted checked interface")
                bootstrap_objects.provider(bootstrap)
                self.assertEqual(len(calls), 6, "checked dependency bytes cannot be waived by a cache hit")
                provider["object"].write_bytes(b"corrupted library code")
                bootstrap_objects.provider(bootstrap)
                self.assertEqual(len(calls), 9)

    def test_abi_mismatch_is_rejected_before_a_native_command(self):
        with tempfile.TemporaryDirectory() as temp:
            directory = Path(temp)
            root, _ = self.fixture(directory)
            with patch.object(bootstrap, "ROOT", root), \
                    patch.object(bootstrap, "seed_inputs", return_value=({"runtime_abi": 6}, b"")), \
                    patch.object(bootstrap, "run") as run:
                with self.assertRaisesRegex(bootstrap.BootstrapError, "ABIs differ"):
                    bootstrap_objects.provider(bootstrap)
                run.assert_not_called()

    def test_compiler_contract_or_unit_cannot_hide_in_the_library_closure(self):
        with tempfile.TemporaryDirectory() as temp:
            directory = Path(temp)
            package = {"modules": [{"name": "Data.Core"}]}
            for metadata in [
                {"interfaces": [{"module": "Bootstrap.Infer", "interface": {"module": "Bootstrap.Infer"}}]},
                {"interfaces": [], "units": [{"symbol": "Bootstrap.Infer.infer"}]},
            ]:
                if not metadata["interfaces"]:
                    doc = {"module": "Data.Core", "imports": []}
                    bootstrap.write_json(directory / "library/build/Data.Core.cinchif", doc)
                    metadata["interfaces"] = [{"module": "Data.Core", "iface_hash": "hash", "interface": doc}]
                with self.assertRaises(bootstrap.BootstrapError):
                    bootstrap_objects.admitted_interfaces(bootstrap, metadata, package, directory)
            with self.assertRaisesRegex(bootstrap.BootstrapError, "compiler implementation"):
                bootstrap_objects.admitted_interfaces(bootstrap, {}, {"modules": [{"name": "Bootstrap.Check"}]}, directory)

    def test_actual_generated_primitive_and_tuple_units_are_library_dependencies(self):
        with tempfile.TemporaryDirectory() as temp:
            directory = Path(temp)
            doc = {"module": "Data.Core", "imports": []}
            bootstrap.write_json(directory / "library/build/Data.Core.cinchif", doc)
            metadata = {"interfaces": [{"module": "Data.Core", "iface_hash": "hash", "interface": doc}],
                        "units": [{"symbol": name} for name in ["<builtin>.Fin", "Tuple.2@OT", "cinch.prim.eqChar", "cinch.runtime.bytesGet"]]}
            rows = bootstrap_objects.admitted_interfaces(bootstrap, metadata, {"modules": [{"name": "Data.Core"}]}, directory)
            self.assertEqual(len(rows["interfaces"]), 1)
            metadata["units"].append({"symbol": "Tuple.Compiler.infer"})
            with self.assertRaises(bootstrap.BootstrapError):
                bootstrap_objects.admitted_interfaces(bootstrap, metadata, {"modules": [{"name": "Data.Core"}]}, directory)

    def test_source_pin_is_checked_even_for_an_existing_snapshot(self):
        with tempfile.TemporaryDirectory() as temp:
            directory = Path(temp)
            snapshot = directory / "library-sources" / ("a" * 64)
            snapshot.mkdir(parents=True)
            with patch.object(bootstrap, "BUILD", directory), \
                    patch.object(bootstrap, "seed_sources", return_value={"compiler/src/lib.rs": b"changed"}):
                with self.assertRaisesRegex(bootstrap.BootstrapError, "sources changed"):
                    bootstrap_objects.source_snapshot(bootstrap, {"source_sha256": "a" * 64})


if __name__ == "__main__":
    unittest.main()
