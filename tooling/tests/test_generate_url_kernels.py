"""Byte-policy completeness and reproducible URL classifier generation."""
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "tooling"))
import generate_url_kernels as generator


class UrlKernelGenerationTests(unittest.TestCase):
    """Check all policy domains and preserve the handwritten source boundary."""

    def test_contexts_preserve_only_their_own_ascii_alphabets(self) -> None:
        """Contexts differ at structural delimiters while every high byte requires escaping."""
        for mode in range(1, 10):
            self.assertTrue(all(generator.allowed(byte, mode) for byte in b"AZaz09-_.~"))
            self.assertFalse(any(generator.allowed(byte, mode) for byte in range(128, 256)))
            self.assertFalse(generator.allowed(0, mode))
        self.assertTrue(generator.allowed(ord('/'), 1))
        self.assertFalse(generator.allowed(ord('/'), 2))
        self.assertFalse(generator.allowed(ord('+'), 6))
        self.assertTrue(generator.allowed(ord('+'), 2))
        self.assertFalse(generator.allowed(ord('?'), 1))
        self.assertTrue(generator.allowed(ord('?'), 7))
        self.assertFalse(generator.allowed(ord('\''), 7))
        self.assertTrue(generator.allowed(ord('\''), 9))
        self.assertFalse(generator.allowed(ord('%'), 1))
        self.assertTrue(generator.allowed(ord('%'), 8))

    def test_all_percent_spellings_are_exact_and_uppercase(self) -> None:
        """The complete 768-byte table contains one exact uppercase triplet per byte."""
        source = generator.generated()
        table = source.split('percent-triplets unit = string-to-bytes "', 1)[1].split('"', 1)[0]
        self.assertEqual(len(table), 768)
        for byte in range(256):
            self.assertEqual(table[byte * 3:byte * 3 + 3], f"%{byte:02X}")

    def test_suffix_generation_is_idempotent_and_check_never_rewrites(self) -> None:
        """Only generated declarations may change, and stale detection preserves the file."""
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "Text.Url.Codec.cinch"
            prefix = "-- | Checked URL boundary.\nmodule Text.Url.Codec where\n  -- | Handwritten identity.\n  value :: U64 -> U64\n  value x = x\n"
            path.write_text(prefix)
            command = [sys.executable, str(ROOT / "tooling/generate_url_kernels.py"), "--module", str(path)]
            subprocess.run(command, check=True)
            first = path.read_text()
            self.assertTrue(first.startswith(prefix))
            subprocess.run(command + ["--check"], check=True)
            subprocess.run(command, check=True)
            self.assertEqual(first, path.read_text())
            stale = first.replace("classify-eight", "stale-classify-eight", 1)
            path.write_text(stale)
            result = subprocess.run(command + ["--check"], capture_output=True, text=True)
            self.assertEqual(result.returncode, 1)
            self.assertIn("stale", result.stderr)
            self.assertEqual(stale, path.read_text())


if __name__ == "__main__":
    unittest.main()
