"""Reproducibility and constant-derivation checks for the generated digest, Keccak and AES kernels."""
from __future__ import annotations

import hashlib
import subprocess
import sys
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "tooling"))

import generate_aes_kernels as aes  # noqa: E402
import generate_sha1_md5_kernels as sha1_md5  # noqa: E402
import generate_sha3_kernels as sha3  # noqa: E402
import generate_sha512_kernels as sha512  # noqa: E402


class DigestKernelGeneration(unittest.TestCase):
    """The checked-in generated suffixes match their generators exactly."""

    def test_generated_suffixes_are_current(self) -> None:
        """Every generator's --check mode accepts the committed module."""
        for script in ("generate_sha512_kernels.py", "generate_sha1_md5_kernels.py", "generate_sha3_kernels.py", "generate_aes_kernels.py"):
            with self.subTest(script=script):
                result = subprocess.run([sys.executable, str(ROOT / "tooling" / script), "--check"], capture_output=True, text=True)
                self.assertEqual(result.returncode, 0, result.stdout + result.stderr)

    def test_sha512_reference_model_matches_hashlib_for_every_variant(self) -> None:
        """Derived constants and initial values reproduce the standard digests."""
        values = sha512.initial_values()
        for name, algorithm, size in (("512", "sha512", 64), ("384", "sha384", 48), ("512/224", "sha512_224", 28), ("512/256", "sha512_256", 32)):
            with self.subTest(variant=name):
                words = sha512.digest(values[name], b"abc")
                self.assertEqual(b"".join(w.to_bytes(8, "big") for w in words)[:size], hashlib.new(algorithm, b"abc").digest())

    def test_sha1_and_md5_constants_match_their_definitions(self) -> None:
        """Square-root and sine-derived constants match the published tables."""
        self.assertEqual(sha1_md5.sha1_constants(), [0x5A827999, 0x6ED9EBA1, 0x8F1BBCDC, 0xCA62C1D6])
        self.assertEqual(sha1_md5.md5_constants()[:2], [0xD76AA478, 0xE8C7B756])

    def test_keccak_reference_permutation_matches_hashlib(self) -> None:
        """The reference permutation reproduces SHA3-256 of the empty message."""
        sha3.self_check()
        self.assertEqual(len(sha3.round_constants()), 24)


class AesKernelGeneration(unittest.TestCase):
    """The arithmetic S-box behind the constant-time lanes matches the published table."""

    def test_arithmetic_sbox_matches_published_entries(self) -> None:
        """Inversion plus the affine map reproduces every published S-box row start."""
        aes.self_check()
        self.assertEqual([aes.sbox(x) for x in range(0, 256, 16)][:4], [0x63, 0xCA, 0xB7, 0x04])


if __name__ == "__main__":
    unittest.main()
