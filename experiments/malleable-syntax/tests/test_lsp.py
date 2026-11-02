from pathlib import Path
import sys
import unittest

HERE = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(HERE))
from editor_lsp import Bridge
from engine import utf16_position


class NativeLspTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.bridge = Bridge()
        cls.bridge.handle({'jsonrpc': '2.0', 'id': 1, 'method': 'initialize', 'params': {}})
        cls.uri = (HERE / 'examples/LspProbe.cinch').as_uri()
        cls.source = '''module Main where
  import Ui
  import syntax "../syntax/html.syntax.json" as html
  greeting :: String
  greeting = "hello"
  person :: String -> Html
  person name = html!{<p>🦊 {greeting}{name}</p>}
  main :: Int
  main = 0
'''
        cls.notifications = cls.open(cls.source)

    @classmethod
    def tearDownClass(cls): cls.bridge.close()

    @classmethod
    def open(cls, source):
        return cls.bridge.handle({'jsonrpc': '2.0', 'method': 'textDocument/didOpen', 'params': {'textDocument': {
            'uri': cls.uri, 'languageId': 'cinch', 'version': 1, 'text': source}}})

    def request(self, method, source, needle):
        line, char = utf16_position(source, source.index(needle))
        return self.bridge.handle({'jsonrpc': '2.0', 'id': 9, 'method': method, 'params': {
            'textDocument': {'uri': self.uri}, 'position': {'line': line, 'character': char}}})[-1]['result']

    def test_01_clean_diagnostics(self):
        diagnostics = [m['params']['diagnostics'] for m in self.notifications if m.get('method') == 'textDocument/publishDiagnostics']
        self.assertTrue(diagnostics)
        self.assertFalse([d for ds in diagnostics for d in ds if d.get('severity') == 1], diagnostics)

    def test_02_hover_inside_embedded_code_after_unicode(self):
        hover = self.request('textDocument/hover', self.source, 'greeting}{name}')
        self.assertIsNotNone(hover)
        self.assertIn('String', str(hover))

    def test_03_definition_returns_original_source(self):
        definition = self.request('textDocument/definition', self.source, 'greeting}{name}')
        self.assertIsNotNone(definition)
        self.assertEqual(definition['uri'], self.uri)
        self.assertEqual(definition['range']['start']['line'], 3)

    def test_04_semantic_tokens_are_original_utf16_positions(self):
        result = self.bridge.handle({'jsonrpc': '2.0', 'id': 10, 'method': 'textDocument/semanticTokens/full', 'params': {
            'textDocument': {'uri': self.uri}}})[-1]['result']['data']
        line, char, positions = 0, 0, []
        for i in range(0, len(result), 5):
            dl, dc, size, kind, mods = result[i:i+5]
            line += dl; char = char + dc if dl == 0 else dc
            positions.append((line, char, size))
        name_pos = utf16_position(self.source, self.source.index('greeting}{name}'))
        self.assertIn((*name_pos, 8), positions)

    def test_05_type_errors_map_to_the_host_hole(self):
        broken = self.source.replace('{name}', '{name + 1}')
        notifications = self.open(broken)
        errors = [d for m in notifications if m.get('method') == 'textDocument/publishDiagnostics' and m['params']['uri'] == self.uri for d in m['params']['diagnostics'] if d.get('severity') == 1]
        self.assertTrue(errors, notifications)
        self.assertEqual(errors[0]['range']['start']['line'], 6)
        self.open(self.source)

    def test_06_unfinished_markup_reports_and_recovers(self):
        notifications = self.open(self.source.replace('</p>}', ''))
        errors = [d for m in notifications if m.get('method') == 'textDocument/publishDiagnostics' and m['params']['uri'] == self.uri for d in m['params']['diagnostics']]
        self.assertTrue(any(d.get('source') == 'cinch-syntax' for d in errors), notifications)
        self.open(self.source)
        self.assertIsNotNone(self.request('textDocument/hover', self.source, 'greeting}{name}'))


if __name__ == '__main__': unittest.main()
