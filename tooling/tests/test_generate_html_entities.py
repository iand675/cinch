"""Reproducible packed HTML table generation and complete UTF-8 replacement storage."""
from pathlib import Path
import json
import subprocess
import sys
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "tooling"))
import generate_html_entities as generator


class HtmlGenerationTests(unittest.TestCase):
    """Check packed data and the source preservation boundary independently of the library."""

    def test_all_names_and_values_fit_without_loss(self) -> None:
        """Every ASCII name and one/two-scalar UTF-8 replacement round trips exactly."""
        entities = json.loads(generator.DATA.read_text())["entities"]
        self.assertEqual(len(entities), 2231)
        self.assertEqual(sum(not key.endswith(";") for key in entities), 106)
        self.assertEqual(len(generator.records()), 2231)
        for name, value in entities.items():
            words = generator.packed_name(name)
            raw = b"".join(word.to_bytes(8, "little") for word in words[:4])
            self.assertEqual(raw[:words[4]], name.encode("ascii"))
            self.assertEqual(raw[words[4]:], bytes(32 - words[4]))
            packed = generator.replacement_word(value)
            self.assertEqual((packed & ((1 << 56) - 1)).to_bytes(7, "little")[:packed >> 56], value.encode())

    def test_boundary_names_and_expanding_references_are_preserved(self) -> None:
        """The longest name and both six-byte expanding replacements remain complete."""
        self.assertEqual(generator.packed_name("CounterClockwiseContourIntegral;")[-1], 32)
        self.assertEqual(generator.replacement_word("≪⃒") >> 56, 6)
        self.assertEqual(generator.replacement_word("≫⃒") >> 56, 6)
        with self.assertRaises(ValueError):
            generator.packed_name("A" * 33)
        with self.assertRaises(ValueError):
            generator.packed_name("é;")

    def test_suffix_check_preserves_handwritten_source_and_never_rewrites(self) -> None:
        """Only generated declarations change; stale checking leaves source untouched."""
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "Text.Html.cinch"
            prefix = "-- | Handwritten public HTML boundary.\nmodule Text.Html where\n  -- | Public identity.\n  identity :: U64 -> U64\n  identity value = value\n"
            path.write_text(prefix)
            command = [sys.executable, str(ROOT / "tooling/generate_html_entities.py"), "--module", str(path)]
            subprocess.run(command, check=True)
            first = path.read_text()
            self.assertTrue(first.startswith(prefix))
            self.assertEqual(first.count(generator.MARKER), 1)
            subprocess.run(command + ["--check"], check=True)
            subprocess.run(command, check=True)
            self.assertEqual(first, path.read_text())
            stale = first.replace("entity-bucket-a", "stale-entity-bucket-a", 1)
            path.write_text(stale)
            result = subprocess.run(command + ["--check"], capture_output=True, text=True)
            self.assertEqual(result.returncode, 1)
            self.assertIn("stale", result.stderr)
            self.assertEqual(stale, path.read_text())
            subprocess.run(command, check=True)
            self.assertEqual(first, path.read_text())


if __name__ == "__main__":
    unittest.main()
