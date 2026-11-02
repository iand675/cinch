import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest

HERE = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(HERE))
from engine import Engine
from editor_generate import generate, generate_expression, generate_project


class TreeSitterIntegration(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.scratch = tempfile.TemporaryDirectory(prefix='cinch-syntax-editor-')
        cls.env = {**os.environ, 'XDG_CACHE_HOME': str(Path(cls.scratch.name) / 'cache')}
        cls.grammars = {}
        for path in sorted((HERE / 'syntax').glob('*.syntax.json')):
            generated = generate(path)
            result = subprocess.run(['tree-sitter', 'generate'], cwd=generated, text=True, capture_output=True)
            if result.returncode: raise AssertionError(result.stderr)
            cls.grammars[str(path.resolve())] = generated
        cls.expression = generate_expression()
        result = subprocess.run(['tree-sitter', 'generate'], cwd=cls.expression, text=True, capture_output=True)
        if result.returncode: raise AssertionError(result.stderr)
        cls.project = generate_project(sorted((HERE / 'examples').glob('*.cinch')))
        result = subprocess.run(['tree-sitter', 'generate'], cwd=cls.project, text=True, capture_output=True)
        if result.returncode: raise AssertionError(result.stderr)

    @classmethod
    def tearDownClass(cls): cls.scratch.cleanup()

    def parse(self, directory, text, quiet=True):
        path = Path(self.scratch.name) / 'probe.txt'
        path.write_text(text)
        command = ['tree-sitter', 'parse'] + (['-q'] if quiet else []) + [str(path)]
        return subprocess.run(command, cwd=directory, env=self.env, text=True, capture_output=True)

    def test_all_source_islands_parse_with_their_library_grammars(self):
        count = 0
        for path in sorted((HERE / 'examples').glob('*.cinch')):
            source = path.read_text()
            expansion = Engine(source, path).expand()
            for trace in expansion.trace:
                if trace['rule'] != 'root': continue
                count += 1
                directory = self.grammars[trace['library']]
                text = source[trace['start'] - 1:trace['end']]
                with self.subTest(example=path.stem, library=directory.name):
                    result = self.parse(directory, text)
                    self.assertEqual(result.returncode, 0, result.stdout + result.stderr + '\n' + text)
        self.assertGreaterEqual(count, 15)

    def test_outer_cinch_grammar_accepts_all_original_sources(self):
        for path in sorted((HERE / 'examples').glob('*.cinch')):
            with self.subTest(example=path.stem):
                result = self.parse(self.project, path.read_text())
                self.assertEqual(result.returncode, 0, result.stdout + result.stderr)

    def test_outer_reader_keeps_raw_text_out_of_host_lexical_modes(self):
        source = '''module Main where
  import syntax "../syntax/html.syntax.json" as html
  view = html!{<p>Don't treat this unmatched " as a Cinch string {f '}'}</p>}
'''
        result = self.parse(self.project, source)
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)

    def test_host_boundary_scanner_handles_cinch_lexical_modes(self):
        path = self.grammars[str((HERE / 'syntax/html.syntax.json').resolve())]
        samples = [
            '{<p>{let s = "}" in s}</p>}',
            "{<p>{let c = '}' in f c 'name}</p>}",
            '{<p>{f {- { nested {- } -} -} x}</p>}',
            '{<p>{f -- } is a comment\n x}</p>}',
            "{<p>{f '\\u{7d}'}</p>}",
        ]
        for sample in samples:
            with self.subTest(sample=sample):
                result = self.parse(path, sample)
                self.assertEqual(result.returncode, 0, result.stdout + result.stderr)

    def test_embedded_cinch_uses_the_actual_host_grammar(self):
        for expression in ['let name = "🦊" in f name', '\\() -> 42', 'person <$> people', 'if ok then x else y']:
            with self.subTest(expression=expression):
                result = self.parse(self.expression, expression)
                self.assertEqual(result.returncode, 0, result.stdout + result.stderr)

    def test_unfinished_html_retains_a_structural_tree(self):
        path = self.grammars[str((HERE / 'syntax/html.syntax.json').resolve())]
        result = self.parse(path, '{<section><h1>Hello</h1><Button onClick={', quiet=False)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn('capture_tag', result.stdout)

    def test_highlight_queries_are_executable(self):
        path = self.grammars[str((HERE / 'syntax/html.syntax.json').resolve())]
        sample = Path(self.scratch.name) / 'highlight.txt'
        sample.write_text('{<p class="x">Hello {person}</p>}')
        result = subprocess.run(['tree-sitter', 'query', str(path / 'queries/highlights.scm'), str(sample)], cwd=path, env=self.env, text=True, capture_output=True)
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertIn('tag', result.stdout)

    def test_full_document_highlighting_composes_both_injection_levels(self):
        config = Path(self.scratch.name) / 'config.json'
        config.write_text(json.dumps({'parser-directories': [str(HERE / 'editor')],
            'theme': {'tag': '#ff5555', 'keyword': '#5555ff', 'string': '#55ff55',
                      'variable': '#ffffff', 'function': '#ffff55', 'operator': '#55ffff'}}))
        source = HERE / 'examples/Html.cinch'
        result = subprocess.run(['tree-sitter', 'highlight', '--html', '--css-classes',
            '--scope', 'source.cinch_project', '--config-path', str(config), str(source)],
            env=self.env, text=True, capture_output=True)
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertIn("class='tag'", result.stdout)
        self.assertIn("class='function'", result.stdout)
        self.assertIn("class='keyword'", result.stdout)


if __name__ == '__main__': unittest.main()
