// Cinch VS Code / Cursor extension: language registration plus LSP
// hookup.
//
// The server command: `cinch lsp` dispatches to
// `cinch::lsp::LspServer::run` over stdio (Content-Length framed
// JSON-RPC), exactly the invocation exercised by compiler/tests/lsp.rs.
// The default command is this repository's driver crate
// (tooling/lsp-driver), the same invocation in a standalone binary;
// bin/cinch-lsp prefers `cinch lsp` when the binary is on PATH. Set
// `cinch.lsp.command` (or change DEFAULT_COMMAND below) to override.

const { workspace, ExtensionContext } = require('vscode');
const { LanguageClient } = require('vscode-languageclient/node');
const path = require('path');

const DEFAULT_COMMAND = path.join(__dirname, 'bin', 'cinch-lsp');

let client;

function activate(context) {
  const config = workspace.getConfiguration('cinch');
  const command = config.get('lsp.command') || DEFAULT_COMMAND;

  client = new LanguageClient(
    'cinch',
    'Cinch Language Server',
    { command: command },
    { documentSelector: [{ scheme: 'file', language: 'cinch' }] },
  );
  client.start();
  context.subscriptions.push({ dispose: () => client && client.stop() });
}

function deactivate() {
  return client && client.stop();
}

module.exports = { activate, deactivate };
