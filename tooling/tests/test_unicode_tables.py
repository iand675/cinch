"""Versioned Unicode source provenance and bounded immutable table representation."""
from pathlib import Path
import json
import re
import sys
import unittest

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / 'tooling'))
import import_unicode_tables as importer
import generate_unicode_tables as generator


class UnicodeTableTests(unittest.TestCase):
    """Protect complete imported facts, original metadata and the no-vector-spill format."""

    @classmethod
    def setUpClass(cls) -> None:
        """Build the compact interpretation once from the pinned checked-in facts."""
        cls.source = json.loads((ROOT / 'tooling/data/unicode17-tables.json').read_text())
        cls.database = generator.model(cls.source)

    def test_source_snapshot_has_full_versioned_catalogs_and_license(self) -> None:
        """The source edition and original redistribution notice remain explicit and complete."""
        self.assertEqual(self.source['unicode_version'], '17.0.0')
        self.assertEqual(self.source['sdk_version'], 'go1.27.4')
        self.assertEqual(len(self.source['tables']), 259)
        self.assertEqual({key: len(value) for key, value in self.source['catalogs'].items()}, {'Categories': 38, 'Scripts': 174, 'Properties': 39, 'FoldCategory': 6, 'FoldScript': 3})
        self.assertEqual(len(self.source['category_aliases']), 42)
        license_text = (ROOT / 'tooling/data/unicode17-BSD-LICENSE.txt').read_text()
        self.assertIn('Copyright 2009 The Go Authors.', license_text)
        self.assertIn('Redistributions of source code must retain', license_text)
        self.assertTrue(all(len(row['sha256']) == 64 for row in self.source['sources'].values()))

    def test_literal_integer_parser_rejects_executable_expressions(self) -> None:
        """Importing factual deltas cannot evaluate arbitrary code or silently accept a new expression grammar."""
        for text, expected in [('0x130 - 0x69', 199), ('-232', -232), ('UpperLower', 1114112), ('+12', 12)]:
            self.assertEqual(importer.integer(text), expected)
        for text in ['True', '2 * 3', '1 / 2', 'f()', '__import__("os")', '[1][0]']:
            with self.assertRaises(ValueError):
                importer.integer(text)

    def test_original_range_metadata_is_preserved(self) -> None:
        """Indexed public observations preserve all source bounds, strides and exact Latin-1 metadata."""
        words = self.database['words']['range-records'][1]
        info = self.database['words']['table-info'][1]
        for index, name in enumerate(self.database['table_names']):
            table = self.source['tables'][name]
            value = info[index]
            start = value & 0xFFFFFFFF
            self.assertEqual((value >> 32) & 1023, len(table['r16']))
            self.assertEqual((value >> 42) & 1023, len(table['r32']))
            self.assertEqual(value >> 52, table['latin_offset'])
            for offset, expected in enumerate(table['r16'] + table['r32']):
                word = words[start + offset]
                self.assertEqual([word & 2097151, (word >> 21) & 2097151, word >> 42], expected)

    def test_compact_pages_cover_the_complete_codepoint_domain(self) -> None:
        """Every code-point high byte has complete ordered runs with bounded valid metadata identities."""
        words = self.database['words']
        directory, runs, metadata = words['class-directory'][1], words['class-runs'][1], words['class-metadata'][1]
        self.assertEqual(len(directory), 0x110000 // 256)
        for info in directory:
            start, count = info >> 9, info & 511
            self.assertGreater(count, 0)
            self.assertLessEqual(count, 256)
            page = runs[start:start + count]
            ends = [word & 255 for word in page]
            self.assertEqual(ends[-1], 255)
            self.assertEqual(ends, sorted(set(ends)))
            self.assertTrue(all((word >> 8) < len(metadata) for word in page))
        self.assertLessEqual(len(metadata), 65536)
        self.assertTrue(all((word & 31) < 30 for word in metadata))

    def test_case_words_roundtrip_all_signed_deltas_and_fold_edges(self) -> None:
        """Sentinels and negative deltas survive compact packing without a signed-width ambiguity."""
        words = self.database['words']
        for bounds, deltas, expected in zip(words['case-bounds'][1], words['case-deltas'][1], self.source['case_ranges']):
            actual = [bounds & 2097151, (bounds >> 21) & 2097151, (deltas & 4194303) - 1114112, ((deltas >> 22) & 4194303) - 1114112, (bounds >> 42) - 1114112]
            self.assertEqual(actual, expected)
        for packed, expected in zip(words['fold-orbit'][1], self.source['fold_orbit']):
            self.assertEqual([packed & 2097151, packed >> 21], expected)

    def test_generated_sources_are_fresh_and_every_case_page_is_shallow(self) -> None:
        """Generation cannot reintroduce giant literal constructors or dynamic constant-vector stack copies."""
        source = generator.generated(self.database)
        self.assertEqual(source, (ROOT / 'compiler/prelude/Data/Unicode/Tables.cinch').read_text())
        self.assertNotIn('extractelement', source)
        self.assertIsNone(re.search(r'^\s*data\s', source, re.MULTILINE))
        for text in [source]:
            for page in text.split('index = case index of')[1:]:
                body = page.split('\n  -- |', 1)[0]
                numeric_cases = [line for line in body.splitlines() if line.strip().split(' ', 1)[0].isdigit()]
                self.assertLessEqual(len(numeric_cases), 64)


if __name__ == '__main__':
    unittest.main()
