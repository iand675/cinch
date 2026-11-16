"""Exact constant derivation and reproducible private SHA-256 source generation."""
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "tooling"))
import generate_sha256_kernels as generator


class Sha256GenerationTests(unittest.TestCase):
    """Check the numeric recipe and preservation boundary independently of compilation tests."""

    def test_prime_roots_match_the_published_constants(self) -> None:
        """Fractional prime-root bits must reproduce both standard initial states exactly."""
        rounds, initial256, initial224 = generator.constants()
        self.assertEqual(rounds[:4], [0x428A2F98, 0x71374491, 0xB5C0FBCF, 0xE9B5DBA5])
        self.assertEqual(rounds[-4:], [0x90BEFFFA, 0xA4506CEB, 0xBEF9A3F7, 0xC67178F2])
        self.assertEqual(initial256, [0x6A09E667, 0xBB67AE85, 0x3C6EF372, 0xA54FF53A, 0x510E527F, 0x9B05688C, 0x1F83D9AB, 0x5BE0CD19])
        self.assertEqual(initial224, [0xC1059ED8, 0x367CD507, 0x3070DD17, 0xF70E5939, 0xFFC00B31, 0x68581511, 0x64F98FA7, 0xBEFA4FA4])

    def test_integer_cube_roots_preserve_rounding_at_boundaries(self) -> None:
        """Integer derivation must never round a nearby cube to the wrong side."""
        for root in [1, 2, 3, 311, 1 << 32, (1 << 64) - 1]:
            self.assertEqual(generator.cube_root(root ** 3), root)
            self.assertEqual(generator.cube_root(root ** 3 - 1), root - 1)
            if root > 1:
                self.assertEqual(generator.cube_root(root ** 3 + 1), root)

    def test_generation_preserves_public_source_and_check_never_rewrites(self) -> None:
        """Only the generated suffix may change; stale checking must leave the file intact."""
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "Text.Sha256.cinch"
            prefix = "-- | Handwritten checked boundary.\nmodule Text.Sha256 where\n  -- | Public algorithm width.\n  size :: Nat\n  size = 32\n"
            path.write_text(prefix)
            command = [sys.executable, str(ROOT / "tooling/generate_sha256_kernels.py"), "--module", str(path)]
            subprocess.run(command, check=True)
            generated = path.read_text()
            self.assertTrue(generated.startswith(prefix))
            self.assertEqual(generated.count(generator.MARKER), 1)
            subprocess.run(command + ["--check"], check=True)
            subprocess.run(command, check=True)
            self.assertEqual(path.read_text(), generated)
            stale = generated.replace("sha256-feed-pair", "stale-feed-pair", 1)
            path.write_text(stale)
            checked = subprocess.run(command + ["--check"], capture_output=True, text=True)
            self.assertEqual(checked.returncode, 1)
            self.assertIn("stale", checked.stderr)
            self.assertEqual(path.read_text(), stale)
            subprocess.run(command, check=True)
            self.assertEqual(path.read_text(), generated)


if __name__ == "__main__":
    unittest.main()
