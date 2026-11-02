from pathlib import Path
import sys
import unittest

HERE = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(HERE))
from poc import drive
from engine import Engine


class NativeProofsOfConcept(unittest.TestCase):
    def test_positive_examples_execute_as_native_cinch(self):
        for path in sorted((HERE / 'examples').glob('*.cinch')):
            with self.subTest(example=path.stem):
                code, output, errors = drive('run', path)
                self.assertEqual(code, 0, errors)

    def test_negative_examples_are_rejected_by_cinch(self):
        reasons = {
            'Html': 'integer literal cannot have type',
            'Once': 'multiplicity %1',
            'Query': 'unknown record field salary',
            'Tasks': 'must be joined exactly once',
            'Proof': 'obligation disproved',
            'Format': 'predicate',
            'Protocol': 'type mismatch',
            'Differentiate': 'type mismatch',
            'Reactive': 'type mismatch',
            'Audio': 'index mismatch',
            'Circuit': 'index mismatch',
            'ChoreographyState': 'type mismatch',
            'ChoreographyBranch': 'type mismatch',
            'ChoreographyWire': 'type mismatch',
            'ChoreographyReplay': 'multiplicity %aff',
            'ChoreographyForge': 'unknown constructor Checkout.MkCheckoutBuyerP1',
            'TensorADShape': 'index mismatch',
            'TensorPrivate': 'not exported',
        }
        self.assertEqual(set(reasons), {p.stem for p in (HERE / 'examples/negative').glob('*.cinch')},
                         'Every rejection fixture must be checked for its intended reason')
        for name, reason in reasons.items():
            path = HERE / 'examples/negative' / (name + '.cinch')
            with self.subTest(example=name):
                if name == 'ChoreographyForge':
                    library = HERE / 'libraries/Checkout.cinch'
                    # This refusal must concern a real private constructor,
                    # rather than a misspelled constructor that never existed.
                    self.assertIn('MkCheckoutBuyerP1 ::', Engine(library.read_text(), library).expand().fragment.text)
                code, output, errors = drive('check', path)
                self.assertNotEqual(code, 0, 'The checker accepted the invalid program')
                self.assertIn(reason, errors)
                self.assertIn(str(path), errors)
                self.assertNotIn('.generated', errors)


if __name__ == '__main__': unittest.main()
