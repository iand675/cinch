// SafeLang VS Code / Cursor extension: language registration plus LSP
// hookup.
//
// The server command: the compiler binary has no `lsp` subcommand today
// (compiler/src/main.rs: check/emit-ir/build/run). The real server is
// `safelang::lsp::LspServer::run` over stdio (Content-Length framed
// JSON-RPC), exactly the invocation exercised by compiler/tests/lsp.rs.
// The default command therefore runs this repository's driver crate
// (tooling/lsp-driver), which is that invocation verbatim. When
// `safelang lsp` lands in the binary, set `safelang.lsp.command` to it
// (or change DEFAULT_COMMAND below).

const { workspace, ExtensionContext } = require('vscode');
const { LanguageClient } = require('vscode-languageclient/node');
const path = require('path');

const DEFAULT_COMMAND = path.join(__dirname, 'bin', 'safelang-lsp');

let client;

function activate(context) {
  const config = workspace.getConfiguration('safelang');
  const command = config.get('lsp.command') || DEFAULT_COMMAND;

  client = new LanguageClient(
    'safelang',
    'SafeLang Language Server',
    { command: command },
    { documentSelector: [{ scheme: 'file', language: 'safelang' }] },
  );
  client.start();
  context.subscriptions.push({ dispose: () => client && client.stop() });
}

function deactivate() {
  return client && client.stop();
}

module.exports = { activate, deactivate };
