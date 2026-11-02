import json
from pathlib import Path
import sys
import tempfile
import unittest

HERE = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(HERE))
from engine import Engine, Grammar, SyntaxFailure, utf16_offset, utf16_position


class SyntaxEngineTests(unittest.TestCase):
    def expand(self, body, name='html'):
        source = f'import syntax "../syntax/{name}.syntax.json" as {name}\n{body}'
        return Engine(source, HERE / 'examples/Test.cinch').expand()

    def test_html_is_a_library_not_a_builtin(self):
        result = self.expand('html!{<div title={"<not a tag>"}>hello {name}</div>}')
        self.assertIn('Ui.element "div"', result.fragment.text)
        self.assertIn('Ui.child (name)', result.fragment.text)
        self.assertIn('Ui.attribute "title" ("<not a tag>")', result.fragment.text)
        self.assertTrue(any(t['rule'] == 'element' for t in result.trace))

    def test_second_library_and_fresh_name_avoidance(self):
        result = self.expand('__syntax_1_once = 9\nonce!{ {f x} as y in {y + y} }', 'once')
        self.assertIn('let __syntax_2_once = (f x)', result.fragment.text)
        self.assertEqual(result.fragment.text.count('f x'), 1)

    def test_tag_mismatch_and_unfinished_code(self):
        for body in ['html!{<div></span>}', 'html!{<div>{hello', 'html!{<div>']:
            with self.subTest(body=body), self.assertRaises(SyntaxFailure): self.expand(body)

    def test_nested_comments_strings_characters_and_tick_names(self):
        result = self.expand("html!{<p>{let x = \"}\" in {- { {- } -} -} f 'name '}' x}</p>}")
        self.assertIn("f 'name '}' x", result.fragment.text)

    def test_nested_extension_inside_host_expression(self):
        result = self.expand('html!{<div>{if ok then html!{<b>yes</b>} else html!{<i>no</i>}}</div>}')
        self.assertNotIn('html!{', result.fragment.text)
        self.assertIn('if ok then (Ui.fragment', result.fragment.text)

    def test_strings_and_comments_do_not_activate_syntax(self):
        body = '"html!{<bad>}" -- html!{broken\n{- html!{broken -}\n'
        self.assertIn(body, self.expand(body).fragment.text)

    def test_source_map_round_trip_and_unicode(self):
        result = self.expand('html!{<p>🦊 {person.name}</p>}')
        source_offset = result.source.index('person')
        original = utf16_position(result.source, source_offset)
        generated = result.generated_position(*original)
        self.assertIsNotNone(generated)
        self.assertEqual(result.original_position(*generated), original)
        self.assertEqual(utf16_offset('🦊abc', 0, 2), 1)

    def test_no_ambient_grammar(self):
        text = 'html!{<p>hello</p>}'
        self.assertEqual(Engine(text, HERE / 'examples/Test.cinch').expand().fragment.text, text)

    def test_duplicate_imports_refused(self):
        source = 'import syntax "../syntax/html.syntax.json" as h\n' * 2
        with self.assertRaisesRegex(SyntaxFailure, 'duplicate'): Engine(source, HERE / 'examples/T.cinch')

    def test_macro_policy_cannot_be_laundered(self):
        with self.assertRaisesRegex(SyntaxFailure, 'no-macros'):
            self.expand('#[no-macros]\nanswer = html!{<p>x</p>}')

    def test_left_recursion_is_bounded(self):
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory)
            (path / 'bad.json').write_text(json.dumps({'version': 1, 'entry': 'root', 'rules': {'root': {'pattern': {'ref': 'root'}}}}))
            with self.assertRaisesRegex(SyntaxFailure, 'left recursion'):
                Engine('import syntax "bad.json" as bad\nbad!{}', path / 'T.cinch').expand()

    def test_import_in_multiline_string_is_inert(self):
        source = '\"start\nimport syntax "missing.json" as h\nend\"'
        self.assertEqual(Engine(source, HERE / 'examples/T.cinch').grammars, {})

    def test_import_in_raw_markup_is_inert(self):
        result = self.expand('html!{<p>\nimport syntax "missing.json" as evil\n</p>}')
        self.assertEqual(len(result.imports), 1)
        self.assertIn('missing.json', result.fragment.text)

    def test_module_scope_is_never_silently_widened(self):
        source = 'module A where\n  import syntax "../syntax/html.syntax.json" as html\n  x = html!{<p>a</p>}\nmodule B where\n  y = 0\n'
        with self.assertRaisesRegex(ValueError, 'one module'):
            Engine(source, HERE / 'examples/T.cinch')

    def test_html_fragments_and_self_closing_components(self):
        result = self.expand('html!{<><input disabled/><Button onClick={\\() -> 7}/></>}')
        self.assertIn('Ui.attribute "disabled" True', result.fragment.text)
        self.assertIn('Ui.component-Button', result.fragment.text)

    def test_input_budget_is_checked_before_parsing(self):
        with self.assertRaisesRegex(SyntaxFailure, 'input budget'):
            Engine('x' * 1_000_001, HERE / 'examples/T.cinch')


if __name__ == '__main__': unittest.main()
