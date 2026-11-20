"""Keep SDK-free PEM corpus/kernel snapshots exact and independent writer failure prefixes observable."""
from __future__ import annotations
import json
from pathlib import Path
import sys
import unittest

ROOT=Path(__file__).resolve().parents[2]
sys.path.insert(0,str(ROOT/'tooling'))
from render_pem_consumer import rendered
from render_pem_kernels import rendered as kernels


class PemFixtureTests(unittest.TestCase):
    """Guard complete checked-in coverage and exact maintained-kernel extraction without invoking an SDK."""

    def test_independent_fixture_shape_and_binary_metadata(self):
        """Preserve every framing/output observation and unrepresentable-as-String metadata byte."""
        data=json.loads((ROOT/'seed/tests/fixtures/pem-vectors.json').read_text())
        self.assertEqual(data['sdk_version'],'go1.27.4')
        self.assertEqual((len(data['encodes']),len(data['decodes']),len(data['writers'])),(473,1716,51))
        self.assertTrue(any(bytes.fromhex(row['block']['label'])==b'\xff\0' for row in data['encodes']))
        self.assertTrue(any(len(row['block']['headers'])==1000 for row in data['encodes']))
        self.assertTrue(any(not row['valid'] for row in data['encodes']))
        self.assertTrue(any(not row['found'] and row['input']==row['rest'] for row in data['decodes']))

    def test_checked_in_consumer_is_fresh(self):
        """CI consumes exact generated observations with strict row counts and shallow readonly offsets."""
        source=rendered()
        self.assertEqual(source,(ROOT/'seed/tests/fixtures/pem-consumer.cinch').read_text())
        self.assertNotIn('__ENCODE',source);self.assertNotIn('__DECODE',source);self.assertNotIn('__WORDS__',source)
        self.assertIn('else False\n\n  -- | Select one exact independent',source)

    def test_actual_kernel_snapshots_are_fresh(self):
        """Host mutations and foreign objects compile the maintained primitive bodies, not test substitutes."""
        for foreign,name in [(False,'pem-kernels.cinch'),(True,'pem-kernels-foreign.cinch')]:
            self.assertEqual(kernels(foreign),(ROOT/'seed/tests/fixtures'/name).read_text())
        self.assertEqual(kernels(True).count('  prim '),11)
        self.assertEqual(kernels(True).count('    unsafe llvm {'),7)

    def test_actual_writer_budget_prefixes(self):
        """Every independently observed stream failure retains exactly its accepted byte prefix."""
        data=json.loads((ROOT/'seed/tests/fixtures/pem-vectors.json').read_text())
        expected=b'-----BEGIN TEST-----\nYWJjZGVm\n-----END TEST-----\n'
        self.assertEqual([row['budget'] for row in data['writers']],list(range(len(expected)+2)))
        for row in data['writers']:
            self.assertEqual(bytes.fromhex(row['output']),expected[:row['budget']])
            self.assertEqual(row['failed'],row['budget']<len(expected))


if __name__=='__main__':unittest.main()
