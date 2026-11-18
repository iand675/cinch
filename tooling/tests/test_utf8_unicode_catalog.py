"""Check reproducible Unicode name generation and independent UTF-8 corpus boundaries."""
from __future__ import annotations
import json
from pathlib import Path
import sys
import unittest

ROOT=Path(__file__).resolve().parents[2]
sys.path.insert(0,str(ROOT/'tooling'))
from generate_unicode_names import generated
from render_utf8_consumer import rendered
from render_utf8_text_consumer import rendered as rendered_text


class Utf8UnicodeCatalogTests(unittest.TestCase):
    """Keep every full-domain chunk and malformed replacement policy observable."""

    def test_generated_catalog_is_fresh(self):
        """Catalog names, aliases and shallow literal selectors reproduce the maintained source."""
        self.assertEqual(generated(),(ROOT/'compiler/prelude/Data/Unicode/Names.cinch').read_text())

    def test_complete_scalar_and_pair_chunks(self):
        """Independent checks cover every code point and exactly all two-byte sequences."""
        data=json.loads((ROOT/'compiler/tests/fixtures/utf8-vectors.json').read_text())
        self.assertEqual(data['sdk_version'],'go1.27.4')
        for field,count,width in [('scalar_chunks',544,2048),('pair_chunks',256,256)]:
            rows=data[field];self.assertEqual(len(rows),count)
            for index,row in enumerate(rows):
                self.assertEqual((row['start'],row['end']),(index*width,(index+1)*width))
                self.assertTrue(0<=row['hash']<1<<64)

    def test_malformed_and_alignment_records(self):
        """Truncated, overlong, surrogate, supplementary and arbitrary replacement bytes are retained."""
        rows=json.loads((ROOT/'compiler/tests/fixtures/utf8-vectors.json').read_text())['rows']
        inputs={bytes.fromhex(row['input']) for row in rows}
        self.assertEqual(len(inputs),12773)
        for value in [b'',b'\x80',b'\xc0\x80\x80',b'\xed\xa0\x80',b'\xf4\x90\x80\x80',b'\xf0\x9f',b'\xf0\x9f\x98\x80']:
            self.assertIn(value,inputs)
        replacements={bytes.fromhex(row['replacement']) for row in rows}
        self.assertIn(b'\xff\0\x80',replacements)
        self.assertIn(b'',replacements)
        self.assertTrue(any(row['repair']!=row['each'] for row in rows))
        for row in rows:
            for key in ['input','replacement','repair','each']:bytes.fromhex(row[key])

    def test_checked_in_text_consumer_is_reproducible(self):
        """SDK-free CI executes the exact shallow consumer generated from independent checked-in rows."""
        data=json.loads((ROOT/'compiler/tests/fixtures/utf8-text-vectors.json').read_text())
        self.assertEqual(data['sdk_version'],'go1.27.4')
        self.assertEqual((len(data['rows']),len(data['long'])),(752,52))
        self.assertEqual(rendered_text(),(ROOT/'compiler/tests/fixtures/utf8-text-consumer.cinch').read_text())

    def test_rendered_pages_are_bounded(self):
        """Runtime corpus construction avoids a giant refined numeric-list AST."""
        source=rendered();self.assertNotIn('__ROWS__',source);self.assertNotIn('__EXPECTED__',source)
        self.assertNotIn('extractelement',source)
        for page in source.split(' index = case index of')[1:]:
            block=page.split('    other -> 0',1)[0]
            self.assertLessEqual(block.count(' -> '),64)


if __name__=='__main__':unittest.main()
