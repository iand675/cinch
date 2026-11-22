"""Explicit base artifacts preserve source, ABI and ownership admission boundaries."""
import copy
import json
from pathlib import Path
import sys
import tempfile
import unittest
from unittest.mock import patch

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import bootstrap
import bootstrap_objects
import build_base_artifact
import verify_stage3
import validate_native_string_construction
from bootstrap_interfaces import content_hash, interface_hash
from pack_interfaces import pack


class CurrentBaseArtifactContract(unittest.TestCase):
    """Exercise coherent artifacts and independently resealed incompatibilities."""

    def setUp(self):
        """Retain a small genuine interface shape with independent current-source bytes."""
        self.temporary = tempfile.TemporaryDirectory()
        self.addCleanup(self.temporary.cleanup)
        base = Path(self.temporary.name).resolve()
        self.root, self.directory = base / "checkout", base / "artifact"
        (self.root / "compiler").mkdir(parents=True)
        (self.root / "seed/prelude/Data").mkdir(parents=True)
        self.directory.mkdir()
        package = {"modules": [{"name": "Data.String", "path": "Data/String.cinch"}]}
        bootstrap.write_json(self.root / "seed/prelude/cinch-base.cinchpkg", package)
        source = b"-- | Retain the exact construction source identity.\nmodule Data.String where\n"
        (self.root / "seed/prelude/Data/String.cinch").write_bytes(source)
        bootstrap.write_json(self.root / "compiler/pins/library-provider.lock.json", {
            "schema": 1, "runtime_abi": 10, "revision": "a" * 40, "source_sha256": "b" * 64})
        (self.root / "compiler/Bootstrap.RuntimeAbi.cinch").write_bytes(bootstrap.runtime_abi_source(10))
        bootstrap.write_json(self.root / "compiler/pins/library-object-roots.json", {"schema": 1, "units": []})
        self.target = "x86_64-unknown-linux-gnu"
        self.host = {"machine": "x86_64", "system": "Linux"}
        self.documents = {}
        template = json.loads((Path(__file__).parent / "fixtures/checked-interface.json").read_text())
        for module, raw in [("Data.String", source), ("CinchBaseObject", bootstrap_objects.ARTIFACT_ENTRY)]:
            document = copy.deepcopy(template)
            document.update(module=module, runtime_abi=10, source_hash=content_hash(raw), imports=[])
            document["exports"] = {key: [] for key in document["exports"]}
            document["abi"] = {"convention": "uniform-word, curried apply stub", "symbols": [], "layouts": []}
            self.documents[module] = document
        self.metadata = {"schema": 1, "runtime_abi": 10, "target": self.target, "word_size": 64,
                         "endian": "little", "reclaim": True, "self_describing": True,
                         "interfaces": [{"module": name, "iface_hash": interface_hash(document), "interface": document}
                                        for name, document in self.documents.items()],
                         "units": [{"symbol": "Data.String.string-repeat"}, {"symbol": "Data.String.string-concat-all"}],
                         "object_content_hash": content_hash(b"object"), "ir_content_hash": content_hash(b"LLVM")}
        files = {"cinch": b"producer", "library.o": b"object", "library.ll": b"LLVM",
                 "libcinchruntime.a": b"matching runtime", "base/Data/String.cinch": source,
                 "base/cinch-base.cinchpkg": (self.root / "seed/prelude/cinch-base.cinchpkg").read_bytes(),
                 "roots.json": (self.root / "compiler/pins/library-object-roots.json").read_bytes(),
                 "library/CinchBaseObject.cinch": bootstrap_objects.ARTIFACT_ENTRY}
        for name, raw in files.items():
            path = self.directory / name
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(raw)
        (self.directory / "cinch").chmod(0o755)
        self.receipt = {"schema": 1, "kind": "current-base-artifact", "host": self.host, "target": self.target,
                        "runtime_abi": 10, "producer": {"mode": "fresh-embedded", "source_sha256": "a" * 64},
                        "runtime": {"runtime_abi": 10, "source_sha256": "b" * 64},
                        "sources": {"Data.String": {"path": "base/Data/String.cinch"}}}
        self.seal()
        self.root_patch = patch.object(bootstrap, "ROOT", self.root)
        self.root_patch.start()
        self.addCleanup(self.root_patch.stop)
        self.host_patch = patch.object(bootstrap, "HOST", self.host)
        self.host_patch.start()
        self.addCleanup(self.host_patch.stop)

    def test_named_variant_limit_keeps_its_actual_emission_phase(self):
        """A known representation refusal does not excuse another compiler phase or a different missing unit."""
        text = "bootstrap.runtime-ir:7:74: checked external variant is absent from the runtime object: i.Data$2eList$2eis$2dempty [O]\n"
        classify = validate_native_string_construction.missing_list_object_variant
        self.assertTrue(classify("list-string-O", 5, text))
        self.assertFalse(classify("list-string-O", 4, text))
        self.assertFalse(classify("concrete-string", 5, text))
        self.assertFalse(classify("list-string-O", 5, text.replace("[O]", "[R]")))

    def seal(self, *, repack=True):
        """Reseal file digests so semantic rejection cases cannot pass merely through a stale checksum."""
        if repack:
            self.package_public_contract(self.directory / "library.cpk")
        artifacts = {str(path.relative_to(self.directory)): bootstrap.file_hash(path)
                     for path in self.directory.rglob("*") if path.is_file() and path.name != "provider.json"}
        self.receipt["artifacts"] = artifacts
        self.receipt["producer"]["compiler_sha256"] = artifacts["cinch"]
        self.receipt["runtime"]["sha256"] = artifacts["libcinchruntime.a"]
        self.receipt["sources"]["Data.String"]["sha256"] = artifacts["base/Data/String.cinch"]
        bootstrap.write_json(self.directory / "provider.json", self.receipt)

    def admit(self):
        """Select this exact receipt without invoking a compiler or falling back to a pin."""
        return bootstrap_objects.artifact_provider(bootstrap, self.directory / "provider.json")

    def package_public_contract(self, path):
        name = "Data.String"
        row = next(row for row in self.metadata["interfaces"] if row["module"] == name)
        rows = [(name, row["iface_hash"], json.dumps(row["interface"]).encode())]
        data = pack(rows, self.metadata)
        path.write_bytes(data)
        return rows, bootstrap.digest(data)

    def test_current_bytes_and_checked_interfaces_are_admitted(self):
        """A matched source/object/interface/runtime family retains its explicit identity."""
        provider = self.admit()
        self.assertEqual(provider["runtime"], self.directory / "libcinchruntime.a")
        self.assertEqual(provider["identity"]["sources"], self.receipt["sources"])
        self.assertEqual(provider["configuration"]["object_sha256"], bootstrap.file_hash(provider["object"]))

    def test_producer_package_is_bound_to_checked_public_interfaces(self):
        path = self.directory / "library.cpk"
        self.assertEqual(self.admit()["interface_package"], path)
        rows, _ = self.package_public_contract(path)
        metadata = copy.deepcopy(self.metadata)
        embedded = metadata.pop("interfaces")
        metadata["interface_refs"] = [{key: value for key, value in row.items() if key != "interface"}
                                      for row in embedded]
        metadata["private_interfaces"] = [embedded[1]]
        metadata["interface_refs"][0]["iface_hash"] = "wrong"
        path.write_bytes(pack(rows, metadata))
        self.seal(repack=False)
        with self.assertRaisesRegex(bootstrap.BootstrapError, "invalid receipt or checked metadata"):
            self.admit()

    def test_changed_object_runtime_and_interface_bytes_are_rejected(self):
        """Replacing any actual payload invalidates the original receipt."""
        for name in ["library.o", "libcinchruntime.a", "library.cpk"]:
            path = self.directory / name
            original = path.read_bytes()
            with self.subTest(name=name):
                path.write_bytes(b"different payload")
                with self.assertRaisesRegex(bootstrap.BootstrapError, "digest mismatch"):
                    self.admit()
                path.write_bytes(original)

    def test_requested_representation_must_be_present_in_actual_metadata(self):
        """A coherent receipt cannot claim a demanded generic variant that its object did not emit."""
        for kind, demand, message in [
            ("units", {"symbol": "Data.List.is-empty", "classes": "O"}, "unit"),
            ("dictionaries", {"symbol": "Data.List.FromList_List", "classes": "O"}, "dictionary"),
            ("methods", {"class": "Category.Foldable.Foldable", "slot": 2, "own": "O"}, "method"),
        ]:
            with self.subTest(kind=kind):
                roots = {"schema": 1, kind: [demand]}
                bootstrap.write_json(self.root / "compiler/pins/library-object-roots.json", roots)
                bootstrap.write_json(self.directory / "roots.json", roots)
                self.seal()
                with self.assertRaisesRegex(bootstrap.BootstrapError, message + " specialization is absent"):
                    self.admit()

    def test_resealed_incompatible_abi_and_ownership_modes_are_rejected(self):
        """Hash agreement cannot waive physical or ownership ABI requirements."""
        for field, value in [("schema", True), ("runtime_abi", 9), ("target", "other"), ("word_size", 32),
                             ("endian", "big"), ("reclaim", False), ("self_describing", False)]:
            with self.subTest(field=field):
                original = self.metadata[field]
                self.metadata[field] = value
                self.seal()
                with self.assertRaisesRegex(bootstrap.BootstrapError, "incompatible object ABI"):
                    self.admit()
                self.metadata[field] = original
        self.seal()

    def test_host_runtime_identity_and_boolean_receipt_schema_are_rejected(self):
        """A selected receipt cannot reinterpret another host, runtime or format."""
        for field, value in [("host", {"machine": "other", "system": "Linux"}), ("runtime_abi", 9), ("schema", True)]:
            with self.subTest(field=field):
                original = self.receipt[field]
                self.receipt[field] = value
                self.seal()
                with self.assertRaises(bootstrap.BootstrapError):
                    self.admit()
                self.receipt[field] = original
        self.receipt["runtime"]["runtime_abi"] = 9
        self.seal()
        with self.assertRaisesRegex(bootstrap.BootstrapError, "runtime identity"):
            self.admit()

    def test_resealed_object_cannot_disagree_with_its_metadata(self):
        """A new receipt digest cannot replace the object's own content identity."""
        (self.directory / "library.o").write_bytes(b"different code")
        self.seal()
        with self.assertRaisesRegex(bootstrap.BootstrapError, "object differs"):
            self.admit()

    def test_old_source_cannot_claim_the_current_base_epoch(self):
        """Current checkout bytes are an independent source-freshness boundary."""
        (self.root / "seed/prelude/Data/String.cinch").write_bytes(b"new current implementation")
        with self.assertRaisesRegex(bootstrap.BootstrapError, "differs from current sources"):
            self.admit()

    def test_missing_body_and_forged_interface_hash_are_rejected(self):
        """Checked interface rows must identify actual retained construction units."""
        units = self.metadata["units"]
        self.metadata["units"] = []
        self.seal()
        with self.assertRaisesRegex(bootstrap.BootstrapError, "bodies are absent"):
            self.admit()
        self.metadata["units"] = units
        self.metadata["interfaces"][0]["iface_hash"] = "0"
        self.seal()
        with self.assertRaisesRegex(bootstrap.BootstrapError, "interface manifest|interface hash"):
            self.admit()

    def test_missing_and_escaping_artifact_paths_are_rejected(self):
        """Artifact selection never reads an unretained or out-of-directory replacement."""
        self.receipt["artifacts"]["../outside"] = "a" * 64
        bootstrap.write_json(self.directory / "provider.json", self.receipt)
        with self.assertRaisesRegex(bootstrap.BootstrapError, "invalid artifact path"):
            self.admit()
        self.receipt["artifacts"].pop("../outside")
        self.receipt["artifacts"].pop("library.o")
        bootstrap.write_json(self.directory / "provider.json", self.receipt)
        with self.assertRaisesRegex(bootstrap.BootstrapError, "required artifact"):
            self.admit()

    def test_explicit_selection_never_falls_back_to_a_pin(self):
        """A rejected explicit source epoch remains a rejection at the driver boundary."""
        with patch.object(bootstrap, "LIBRARY_ARTIFACT", self.directory / "missing.json"), \
                patch.object(bootstrap_objects, "provider", side_effect=AssertionError("silent fallback")):
            with self.assertRaisesRegex(bootstrap.BootstrapError, "explicit library artifact"):
                bootstrap.runtime_provider()

    def test_duplicate_fields_and_unretained_interfaces_are_rejected(self):
        """A receipt cannot hide field ambiguity or omit the actual checked interface bytes."""
        path = self.directory / "provider.json"
        original = path.read_text()
        path.write_text(original.replace('"schema": 1', '"schema": 1, "schema": 1', 1))
        with self.assertRaisesRegex(bootstrap.BootstrapError, "duplicate artifact field"):
            self.admit()
        self.receipt["artifacts"].pop("library.cpk")
        bootstrap.write_json(path, self.receipt)
        with self.assertRaisesRegex(bootstrap.BootstrapError, "required artifact bytes"):
            self.admit()

    def test_default_provider_and_stage3_selection_remain_explicit(self):
        """Default pin selection stays intact, while an explicit Stage3 cannot masquerade as the pinned epoch."""
        with patch.object(bootstrap, "LIBRARY_ARTIFACT", None), \
                patch.object(bootstrap_objects, "provider", return_value="pinned") as pinned:
            self.assertEqual(bootstrap.runtime_provider(), "pinned")
            pinned.assert_called_once()
        build = self.directory / "stage-cache"
        bootstrap.write_json(build / "stage3/stage.json", {"inputs": {"library_provider": {"kind": "current-base-artifact"}}})
        with patch.object(bootstrap, "BUILD", build), patch.object(bootstrap, "LIBRARY_ARTIFACT", None):
            with self.assertRaisesRegex(bootstrap.BootstrapError, "explicit --library-artifact"):
                verify_stage3.stage3_artifacts()

    def test_builder_uses_fresh_embedded_producer_without_source_shadow(self):
        """The artifact workflow invokes Cargo and emits one complete object from the embedded family."""
        output, cargo = self.directory / "new-artifact", self.directory / "cargo"
        commands = []

        def execute(command, **kwargs):
            """Model successful producer outputs while preserving every requested command boundary."""
            commands.append(command)
            if command[0] == "cargo":
                target = Path(command[command.index("--target-dir") + 1]) / "release"
                target.mkdir(parents=True)
                binary = target / ("cinch" if "--bin" in command else "libcinchruntime.a")
                binary.write_bytes(b"fresh producer" if "--bin" in command else b"fresh runtime")
                binary.chmod(0o755)
            else:
                self.assertEqual(command[1], "emit-object")
                self.assertEqual(Path(command[2]).read_bytes(), bootstrap_objects.ARTIFACT_ENTRY)
                self.assertFalse((Path(command[2]).parent / "Data.String.cinch").exists())
                self.assertIn("--interface-package-only", command)
                self.assertEqual(command[command.index("--interface-package-private") + 1], "CinchBaseObject")
                (output / "library.o").write_bytes(b"object")
                (output / "library.ll").write_bytes(b"LLVM")
                for name, document in self.documents.items():
                    bootstrap.write_json(output / "library/build" / (name + ".cinchif"), document)
                self.package_public_contract(output / "library.cpk")

        sources = {"compiler/src/driver.rs": b"embedded producer source", "compiler/runtime/src/lib.rs": b"runtime source"}
        with patch.object(bootstrap, "seed_sources", return_value=sources), \
                patch.object(build_base_artifact.subprocess, "run", side_effect=execute):
            receipt = build_base_artifact.build(output, cargo)
        provider = bootstrap_objects.artifact_provider(bootstrap, receipt)
        self.assertEqual(len(commands), 3)
        self.assertEqual(provider["identity"]["producer"]["mode"], "fresh-embedded")
        self.assertEqual(provider["interface_package"], output / "library.cpk")
        self.assertEqual((output / "base/Data/String.cinch").read_bytes(),
                         (self.root / "seed/prelude/Data/String.cinch").read_bytes())

    def test_stage3_verifier_accepts_the_exact_explicit_library_epoch(self):
        """Stage3 selection validates the chosen provider instead of requiring an unrelated pin identity."""
        provider = self.admit()
        build = self.directory / "verified-stage-cache"
        stage = build / "stage3"
        stage.mkdir(parents=True)
        (build / "stage2").mkdir()
        for binary in [stage / "cinch", build / "stage2/cinch"]:
            binary.write_bytes(b"same self-built compiler")
        package = stage / "base.cpk"
        package.write_bytes(provider["interface_package"].read_bytes())
        bootstrap.write_json(stage / "base-objects.json",
                             {**provider["configuration"], "metadata": str(package),
                              "metadata_sha256": bootstrap.file_hash(package)})
        files = {"Main.cinch": b"frozen compiler source"}
        inputs = {"source_sha256": bootstrap.source_hash(files), "built_by_stage": 2,
                  "protocol": bootstrap.PROTOCOL, "host": self.host, "library_provider": provider["identity"],
                  "builder_sha256": bootstrap.file_hash(build / "stage2/cinch"),
                  "interface_set_sha256": bootstrap.file_hash(stage / "base.cpk"),
                  "object_configuration_sha256": bootstrap.file_hash(stage / "base-objects.json"),
                  "runtime_archive_sha256": bootstrap.file_hash(provider["runtime"])}
        bootstrap.write_json(stage / "stage.json", {"inputs": inputs, "executable_sha256": bootstrap.file_hash(stage / "cinch")})
        with patch.object(bootstrap, "BUILD", build), \
                patch.object(bootstrap, "LIBRARY_ARTIFACT", self.directory / "provider.json"), \
                patch.object(bootstrap, "collect_compiler_sources", return_value=files), \
                patch.object(bootstrap, "cached", return_value=True):
            compiler, interfaces, objects, runtime, hashes = verify_stage3.stage3_artifacts()
        self.assertEqual(compiler, stage / "cinch")
        self.assertEqual(runtime, provider["runtime"])
        self.assertEqual(hashes[provider["object"]], provider["configuration"]["object_sha256"])


if __name__ == "__main__":
    unittest.main()
