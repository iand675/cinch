"""Reproducible private Base64 kernel generation and offline consumer projection."""
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "tooling"))
import generate_base64_kernels as generator
import validate_base64_encoding as consumer


class Base64GenerationTests(unittest.TestCase):
    """Keep generated suffix replacement safe and test consumers independent of a reference SDK."""

    def test_generation_preserves_prefix_is_idempotent_and_check_does_not_write(self) -> None:
        """Only the marked private suffix may change, including on stale-check failure."""
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "Encoding.cinch"
            prefix = "-- | Handwritten public boundary.\nmodule Encoding where\n  -- | Published size.\n  size :: Nat\n  size = 64\n"
            path.write_text(prefix)
            command = [sys.executable, str(ROOT / "tooling/generate_base64_kernels.py"), "--module", str(path)]
            subprocess.run(command, check=True)
            expected = path.read_text()
            self.assertTrue(expected.startswith(prefix))
            self.assertEqual(expected.count(generator.MARKER), 1)
            subprocess.run(command, check=True)
            self.assertEqual(path.read_text(), expected)
            subprocess.run(command + ["--check"], check=True)
            stale = expected.replace("%a = shufflevector", "%stale = shufflevector", 1)
            path.write_text(stale)
            result = subprocess.run(command + ["--check"], capture_output=True, text=True)
            self.assertEqual(result.returncode, 1)
            self.assertIn("stale", result.stderr)
            self.assertEqual(path.read_text(), stale)

    def test_missing_or_duplicate_generation_markers_refuse_ambiguous_replacement(self) -> None:
        """An unmarked existing kernel or duplicated suffix must never be silently appended."""
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "Encoding.cinch"
            command = [sys.executable, str(ROOT / "tooling/generate_base64_kernels.py"), "--module", str(path)]
            for text in ["module Encoding where\n  prim encode-values :: U64 -> U64\n", "module Encoding where\n" + generator.MARKER * 2]:
                path.write_text(text)
                result = subprocess.run(command, capture_output=True, text=True)
                self.assertNotEqual(result.returncode, 0)
                self.assertEqual(path.read_text(), text)

    def test_offline_projection_covers_every_fixed_and_custom_configuration(self) -> None:
        """Consumers project stable checked fixtures without invoking a compiler or reference SDK."""
        for name in consumer.GROUPS:
            first = consumer.source(name)
            self.assertEqual(first, consumer.source(name))
            self.assertIn("module Main where", first)
            self.assertIn("main :: Unit ->{IO r} Int", first)
            self.assertIn("configuration :: Nat -> Base64Encoding", first)
        with self.assertRaises(ValueError):
            consumer.source("unknown")


if __name__ == "__main__":
    unittest.main()
