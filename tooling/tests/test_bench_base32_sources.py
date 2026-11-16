"""Read-only source-root and override freezing for isolated Base32 measurements."""
import json
from pathlib import Path
import sys
import tempfile
import unittest
from unittest import mock

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "tooling"))
from bench_base32_encoding import frozen_closure


class Base32SourceFreezingTests(unittest.TestCase):
    """Keep compilation bytes independent of later source edits and avoid checkout writes."""

    def test_selected_root_overrides_and_cycles_are_read_once(self) -> None:
        """Resolve overrides against the selected root while freezing each module's original bytes."""
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory) / "prelude"
            root.mkdir()
            (root / "cinch-base.cinchpkg").write_text(json.dumps({"modules": [{"name": "Data.Bytes"}]}))
            original = b"module A where\n  import B\n  import Data.Bytes\n"
            (root / "A.cinch").write_bytes(original)
            (root / "B.cinch").write_text("module B where\n")
            override = Path(directory) / "B.cinch"
            override.write_text("module B where\n  import A\n")
            reads = {}
            actual_read = Path.read_bytes

            def record(path: Path) -> bytes:
                """Observe source reads without changing their values or selected paths."""
                reads[path] = reads.get(path, 0) + 1
                return actual_read(path)

            with mock.patch.object(Path, "read_bytes", record):
                found = frozen_closure("module Main where\n  import A\n", root, {"B": override})
            self.assertEqual(set(found), {"A", "B"})
            self.assertEqual(found["B"][0], override.resolve())
            self.assertEqual(reads, {root / "A.cinch": 1, override: 1})
            (root / "A.cinch").write_text("module A where\n")
            self.assertEqual(found["A"][1], original)
            self.assertEqual((root / "B.cinch").read_text(), "module B where\n")

    def test_unused_overrides_are_rejected(self) -> None:
        """A requested baseline must be reachable instead of being silently ignored."""
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / "cinch-base.cinchpkg").write_text('{"modules": []}')
            with self.assertRaisesRegex(ValueError, "Unused"):
                frozen_closure("module Main where\n", root, {"Unused": root / "missing.cinch"})


if __name__ == "__main__":
    unittest.main()
