import copy
import json
from pathlib import Path
import sys
import tempfile
import unittest
from unittest.mock import patch

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import bootstrap_interfaces as contracts
import bootstrap


class CompilerInterfaceGate(unittest.TestCase):
    def setUp(self):
        # Produced by Stage2 from a module with no imported contracts.
        self.doc = json.loads((Path(__file__).parent / "fixtures/checked-interface.json").read_text())

    def test_actual_writer_contract_and_closed_chain_pass(self):
        contracts.validate_contract(self.doc, 6)
        child = {**self.doc, "module": "Client", "imports": [
            {"module": "Main", "iface_hash": contracts.interface_hash(self.doc)}]}
        contracts.verify_dependencies({"Main": self.doc, "Client": child}, {})

    def test_placeholder_and_malformed_contracts_are_rejected(self):
        mutations = {
            "placeholder": lambda d: d.clear(),
            "abi": lambda d: d.update(runtime_abi=5),
            "previous-schema": lambda d: d.update(schema=9),
            "schema-bool": lambda d: d.update(schema=True),
            "source": lambda d: d.update(source_hash="not-a-content-hash"),
            "missing-scheme": lambda d: d["exports"]["values"][0].pop("scheme"),
            "empty-scheme": lambda d: d["exports"]["values"][0].update(scheme=""),
            "unowned-symbol": lambda d: d["exports"]["values"][0].update(symbol="main"),
            "missing-resolved-boundary": lambda d: d["exports"]["values"][0].pop("resolved"),
            "missing-body-boundary": lambda d: d["exports"]["values"][0].pop("total_body"),
            "arity-bool": lambda d: d["exports"]["values"][0].update(arity=True),
            "duplicate-value": lambda d: d["exports"]["values"].append(d["exports"]["values"][0]),
            "calling-convention": lambda d: d["abi"].update(convention="invented"),
            "private-fixity": lambda d: d["exports"]["fixities"].append({"op": "hidden", "prec": 4, "assoc": "left"}),
        }
        for name, mutate in mutations.items():
            with self.subTest(name=name):
                doc = copy.deepcopy(self.doc)
                mutate(doc)
                with self.assertRaises((KeyError, TypeError, ValueError)):
                    contracts.validate_contract(doc, 6)

    def test_instance_metadata_uses_the_writer_encoding(self):
        declaration = {"resolved": {"scheme": "Nat", "nominals": [], "measures": []}}
        method = {"name": "operation", "slot": 0, "scheme": "Nat",
                  "checked": {"schema": 1, "head": declaration,
                              "implementation": {**declaration, "identity": "Main.method"}}}
        instance = {"id": "Main.Instance", "class": "Main.Class", "target": "Nat",
                    "resolved": None, "implicit": True, "ctx": [], "methods": ["Main.method"],
                    "method_contracts": [method]}
        self.doc["exports"]["instances"] = [instance]
        contracts.validate_contract(self.doc, 6)

        for name, mutate in (
            ("non-object-method", lambda i: i["method_contracts"].append(None)),
            ("display-only", lambda i: i["method_contracts"][0].pop("checked")),
            ("null-checked", lambda i: i["method_contracts"][0].update(checked=None)),
            ("null-body", lambda i: i["method_contracts"][0]["checked"]["implementation"].update(body=None)),
            ("empty-bodies", lambda i: i.update(method_bodies=[])),
        ):
            with self.subTest(name=name):
                doc = copy.deepcopy(self.doc)
                mutate(doc["exports"]["instances"][0])
                with self.assertRaises(ValueError):
                    contracts.validate_contract(doc, 6)

    def test_dependency_changes_cannot_be_hidden_by_stable_output(self):
        child = {**self.doc, "module": "Client", "imports": [
            {"module": "Main", "iface_hash": contracts.interface_hash(self.doc)}]}
        changed = copy.deepcopy(self.doc)
        changed["exports"]["values"][0]["scheme"] = "Bool"
        for documents in ({"Client": child}, {"Client": child, "Main": changed}):
            with self.assertRaisesRegex(ValueError, "missing|stale"):
                contracts.verify_dependencies(documents, {})
        child["imports"].append(child["imports"][0])
        with self.assertRaisesRegex(ValueError, "duplicate"):
            contracts.verify_dependencies({"Main": self.doc, "Client": child}, {})

    def test_noncanonical_and_duplicate_key_documents_are_rejected(self):
        with tempfile.TemporaryDirectory() as temp:
            path = Path(temp) / "Main.cinchif"
            path.write_text(contracts.canonical(self.doc))
            self.assertEqual(contracts.read_contract(path), self.doc)
            for text in (json.dumps(self.doc, indent=2), '{"module":"Main","module":"Other"}'):
                path.write_text(text)
                with self.assertRaises(ValueError):
                    contracts.read_contract(path)

    def test_final_consumer_uses_self_built_compiler_and_emitted_contracts(self):
        for runtime_abi in (5, 6):
            with self.subTest(runtime_abi=runtime_abi), patch.object(bootstrap, "seed_inputs", return_value=({"runtime_abi": runtime_abi}, b"")):
                self.assert_final_consumer(runtime_abi)

    def assert_final_consumer(self, runtime_abi):
        doc = {**self.doc, "runtime_abi": runtime_abi}
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            (root / "cinch").write_bytes(b"self-built compiler")
            (root / "Main.cinchif").write_text(contracts.canonical(doc))
            dependency = {**doc, "module": "Bootstrap.Check"}
            path = root / "Bootstrap.Check.cinchif"
            path.write_text(contracts.canonical(dependency))
            bootstrap.write_json(root / "base-interfaces.json", {"schema": 1, "interfaces": []})
            files = {"Main.cinch": b"module Main where\n  import Bootstrap.Check\n", "Bootstrap.Check.cinch": b"checked source"}

            def run(command, cwd, log):
                self.assertEqual(command[0], root / "cinch")
                self.assertEqual(command[1:3], ["--check", "--interfaces"])
                manifest = bootstrap.read_json(command[3])
                self.assertEqual(manifest["interfaces"], [["Bootstrap.Check", contracts.interface_hash(dependency), str(path.resolve())]])
                self.assertEqual(Path(command[4]).read_bytes(), files["Main.cinch"])
                (cwd / "entry.diagnostics").write_text("")
                return 0

            with patch.object(bootstrap, "run", side_effect=run):
                bootstrap.check_interface_consumer(root, files)
            self.assertEqual(bootstrap.read_json(root / "interface-consumer/checked.json")["status"], "passed")
            for diagnostic, code in ((None, 0), ("missing defining contract", 0), ("", 4)):
                def refuse(command, cwd, log):
                    if diagnostic is not None:
                        (cwd / "entry.diagnostics").write_text(diagnostic)
                    return code
                with self.subTest(diagnostic=diagnostic, code=code), patch.object(bootstrap, "run", side_effect=refuse):
                    with self.assertRaisesRegex(bootstrap.BootstrapError, "cannot consume its actual compiler interfaces"):
                        bootstrap.check_interface_consumer(root, files)


if __name__ == "__main__":
    unittest.main()
