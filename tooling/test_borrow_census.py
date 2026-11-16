import unittest
import json
from pathlib import Path
import subprocess
import sys
import tempfile
from borrow_census import census, refusal_report


class BorrowCensusTests(unittest.TestCase):
    def test_cli_breaks_down_owned_use_witnesses_and_accepts_legacy_rows(self):
        with tempfile.TemporaryDirectory() as temp:
            core = Path(temp) / "input.core"
            report = Path(temp) / "report.log"
            core.write_text("unit Main.read :: -\n  @params x :: A @obj, y :: A @obj, z :: A @obj\n  body (unit)\nend\n")
            rows = [dict(unit="Main.read", param=1, reason="owned-use", use="return"),
                    dict(unit="Main.read", param=2, reason="owned-use", use="capture"),
                    dict(unit="Main.read", param=3, reason="lent")]
            for detailed in [True, False]:
                actual = rows if detailed else [{k: v for k, v in row.items() if k != "use"} for row in rows]
                report.write_text('\n'.join('cinch-borrow: ' + json.dumps(row)
                                           for row in [dict(event="begin"), *actual, dict(event="end")]))
                result = subprocess.run([sys.executable, str(Path(__file__).with_name("borrow_census.py")),
                                         str(core), "--report", str(report), "--json"],
                                        check=True, text=True, capture_output=True)
                counted = json.loads(result.stdout)
                self.assertEqual(counted["reasons"], {"owned-use": 2, "lent": 1})
                self.assertEqual(counted["owned_uses"], {"return": 1, "capture": 1} if detailed else {})

    def test_modes_are_distinct_from_classes_and_primitive_conventions(self):
        result = census('''unit Main.read :: A -> A -> A
  @params x :: A @obj, y :: A @obj
  @borrowed(2)
  @borrow-modes(1)
  body (dup y (drop y (local x)))
end
''')
        self.assertEqual(result['params'], 2)
        self.assertEqual(result['counted'], 2)
        self.assertEqual(result['borrowed'], 1)
        self.assertEqual(result['borrowed_units'], ['Main.read'])
        self.assertEqual(result['sites'], 2)
        self.assertEqual(result['on_params'], 2)

    def test_tuple_commas_do_not_create_extra_formals(self):
        result = census("unit 'Main.weird''name' :: -\n  @params x :: (Int, Int) @obj, y :: Int @obj\nend\n")
        self.assertEqual(result['params'], 2)
        self.assertEqual(result['counted'], 2)

    def test_wrapping_marks_and_quoted_names_do_not_hide_sites(self):
        result = census('''unit Main.read :: A -> A
  @params x :: A @obj, 'odd-name' :: A @obj
  body (drop @class(obj)
    x
    (dup 'odd-name' @class(obj)
      (string "(drop fake)" 11)))
end
''')
        self.assertEqual(result['params'], 2)
        self.assertEqual(result['sites'], 2)
        self.assertEqual(result['on_params'], 2)

    def test_types_and_annotations_are_not_executable_ownership_sites(self):
        result = census('''unit Main.read :: forall drop. drop Int -> Int
  @params x :: (drop Int) @obj
  @prim("(dup fake)")
  body (int 0)
end
''')
        self.assertEqual(result['sites'], 0)

    def test_reports_do_not_add_two_inference_passes_together(self):
        text = '\n'.join([
            'unrelated diagnostic',
            'cinch-borrow: {"event":"begin"}',
            'cinch-borrow: {"unit":"Main.read","param":1,"reason":"lent"}',
            'cinch-borrow: {"event":"end"}',
            'cinch-borrow: {"event":"begin"}',
            'cinch-borrow: {"unit":"Main.read","param":1,"reason":"owned-use"}',
            'cinch-borrow: {"event":"end"}',
        ])
        rows = refusal_report(text)
        self.assertEqual(len(rows), 1)
        self.assertEqual(rows[0]['reason'], 'lent')

    def test_incomplete_report_is_not_a_measurement(self):
        with self.assertRaises(ValueError):
            refusal_report('cinch-borrow: {"event":"begin"}')

    def test_a_dependency_pass_does_not_hide_the_requested_program(self):
        text = '\n'.join([
            'cinch-borrow: {"event":"begin"}',
            'cinch-borrow: {"unit":"Dependency.read","param":1,"reason":"lent"}',
            'cinch-borrow: {"event":"end"}',
            'cinch-borrow: {"event":"begin"}',
            'cinch-borrow: {"unit":"Main.read","param":1,"reason":"owned-use"}',
            'cinch-borrow: {"event":"end"}',
        ])
        self.assertEqual(refusal_report(text, {'Main.read'})[0]['reason'], 'owned-use')
        with self.assertRaises(ValueError):
            refusal_report(text, {'Missing.read'})


if __name__ == '__main__':
    unittest.main()
