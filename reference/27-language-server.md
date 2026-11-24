# Language server and editor integration

<span class="support-tag support-tag--partial">Partial</span> **The compiler includes a working stdio Language Server Protocol (LSP) server (`cinch lsp`) covering diagnostics, hover, go-to-definition, completion, and semantic tokens across open documents.** Packaged marketplace extensions, disk discovery, and refactoring actions are still in progress.

The server runs on the same analysis engine as `cinch repl`, so hover types, diagnostics, visibility, effects, refinements, and ownership errors match `cinch check`.

Syntax highlighting (editor grammars or tree-sitter) and language-server analysis are independent: coloured tokens alone do not mean `cinch lsp` is running.

# Launching the server

```sh
cinch lsp
```

`cinch lsp` speaks JSON-RPC 2.0 framed with `Content-Length` headers over stdin/stdout (stderr is reserved for logs). Running it directly in a terminal blocks waiting for a framed message.

A standalone wrapper binary is also available at `tooling/lsp-driver/` for clients that prefer a dedicated executable:

```sh
cargo run --quiet --manifest-path tooling/lsp-driver/Cargo.toml --release
```

Generic LSP client settings:

| Setting | Value |
| --- | --- |
| Command | `cinch` (or absolute path to the compiler binary) |
| Arguments | `["lsp"]` |
| Transport | stdio |
| Language ID | `cinch` |
| File extension | `.cinch` |
| Workspace root | Project root directory (`rootUri`) |

# Supported protocol methods

Any request not listed below returns JSON-RPC error `-32601` (`method not found`).

| Method | Kind | Behaviour |
| --- | --- | --- |
| `initialize` | request | Records `rootUri`, initializes project analysis, and returns server capabilities |
| `initialized` | notification | No-op |
| `workspace/didChangeConfiguration` | notification | No-op (no configuration schema is read) |
| `textDocument/didOpen` | notification | Registers the buffer and publishes diagnostics for all open documents |
| `textDocument/didChange` | notification | Replaces the buffer with the last entry of `contentChanges` (full sync) and republishes diagnostics for all open documents |
| `textDocument/didClose` | notification | Drops the buffer from analysis and clears its diagnostics |
| `textDocument/hover` | request | Returns a Markdown `name :: scheme` code block, or `null` |
| `textDocument/definition` | request | Returns a `Location` pointing to the first character of the declaration's signature line, or `null` |
| `textDocument/completion` | request | Returns `{isIncomplete: false, items}` filtered by the identifier prefix at the cursor |
| `textDocument/semanticTokens/full` | request | Returns the delta-encoded full-document semantic token array |
| `shutdown` | request | Marks shutdown and returns `null` |
| `exit` | notification | Exits the message loop |

Because the server advertises full synchronization (`textDocumentSync: 1`), `didChange` uses only the final entry's `text` and ignores incremental ranges; clients configured to send range edits will corrupt server buffers. `didOpen` ignores `languageId` and `version`.

# Protocol walkthrough

Consider a two-file project with `Util.cinch` and `Main.cinch`:

<!-- cinch:project group=27-language-server-session file=Util.cinch -->
```cinch
module Util where

double :: Int -> Int
double n = n + n

quad :: Int -> Int
quad n = double (double n)
```

<!-- cinch:project group=27-language-server-session file=Main.cinch exit=84 -->
```cinch
module Main where

import Util

main :: Int
main = quad 21
```

`cinch run Main.cinch` compiles both modules and exits **84**. The wire transcript below shows JSON-RPC payloads (without `Content-Length` headers), with `-->` for client-to-server messages and `<--` for server responses.

## `initialize`

```text
--> {"jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {"processId": null, "rootUri": "file:///path/to/project", "capabilities": {}}}
<-- {"id": 1, "jsonrpc": "2.0", "result": {"capabilities": {"completionProvider": {"resolveProvider": false, "triggerCharacters": ["."]}, "definitionProvider": true, "hoverProvider": true, "semanticTokensProvider": {"full": true, "legend": {"tokenModifiers": [], "tokenTypes": ["keyword", "variable", "function", "type", "class", "enumMember", "method", "property", "namespace", "number", "string", "operator", "comment"]}}, "textDocumentSync": 1}, "serverInfo": {"name": "cinch", "version": "0.1.0"}}}
```

## `textDocument/didOpen`

Opening `Util.cinch` and then `Main.cinch` publishes diagnostics for every open buffer on each update:

```text
--> {"jsonrpc": "2.0", "method": "textDocument/didOpen", "params": {"textDocument": {"uri": "file:///path/to/project/Util.cinch", "languageId": "cinch", "version": 1, "text": "module Util where\n\ndouble :: Int -> Int\ndouble n = n + n\n\nquad :: Int -> Int\nquad n = double (double n)\n"}}}
<-- {"jsonrpc": "2.0", "method": "textDocument/publishDiagnostics", "params": {"diagnostics": [], "uri": "file:///path/to/project/Util.cinch"}}
--> {"jsonrpc": "2.0", "method": "textDocument/didOpen", "params": {"textDocument": {"uri": "file:///path/to/project/Main.cinch", "languageId": "cinch", "version": 1, "text": "module Main where\n\nimport Util\n\nmain :: Int\nmain = quad 21\n"}}}
<-- {"jsonrpc": "2.0", "method": "textDocument/publishDiagnostics", "params": {"diagnostics": [], "uri": "file:///path/to/project/Main.cinch"}}
<-- {"jsonrpc": "2.0", "method": "textDocument/publishDiagnostics", "params": {"diagnostics": [], "uri": "file:///path/to/project/Util.cinch"}}
```

**Open-buffers-only rule:** the server does not currently read unopened sibling files from disk. Opening `Main.cinch` without `Util.cinch` open produces `unknown name quad (in Main.cinch)` until `Util.cinch` is opened.

## `textDocument/hover`

Querying `{line: 5, character: 7}` (the `q` of `quad` in `main = quad 21`):

```text
--> {"jsonrpc": "2.0", "id": 2, "method": "textDocument/hover", "params": {"textDocument": {"uri": "file:///path/to/project/Main.cinch"}, "position": {"line": 5, "character": 7}}}
<-- {"id": 2, "jsonrpc": "2.0", "result": {"contents": {"kind": "markdown", "value": "```cinch\nquad :: (Int -> Int)\n```"}}}
```

Positions that are not a recorded declaration or reference site (such as literals) return `null`:

```text
--> {"jsonrpc": "2.0", "id": 6, "method": "textDocument/hover", "params": {"textDocument": {"uri": "file:///path/to/project/Main.cinch"}, "position": {"line": 5, "character": 12}}}
<-- {"id": 6, "jsonrpc": "2.0", "result": null}
```

## `textDocument/completion`

```text
--> {"jsonrpc": "2.0", "id": 3, "method": "textDocument/completion", "params": {"textDocument": {"uri": "file:///path/to/project/Main.cinch"}, "position": {"line": 5, "character": 10}}}
<-- {"id": 3, "jsonrpc": "2.0", "result": {"isIncomplete": false, "items": [{"detail": "(Int -> Int)", "kind": 3, "label": "quad"}]}}
```

Completion extracts the identifier prefix ending at the cursor (`qua`) and filters visible session, imported, and prelude symbols. An empty prefix returns the entire visible symbol set, so clients should apply local fuzzy filtering as the user types.

LSP `CompletionItemKind` mapping: value/law/instance/effect-operation = `3` (Function), method = `2` (Method), constructor = `4` (Constructor), field = `5` (Field), class = `7` (Class), type/effect = `25` (TypeParameter).

## `textDocument/definition`

Definitions resolve within a file and across open modules, pointing to the first character of the declaration's signature line:

```text
--> {"jsonrpc": "2.0", "id": 4, "method": "textDocument/definition", "params": {"textDocument": {"uri": "file:///path/to/project/Util.cinch"}, "position": {"line": 6, "character": 9}}}
<-- {"id": 4, "jsonrpc": "2.0", "result": {"range": {"end": {"character": 1, "line": 2}, "start": {"character": 0, "line": 2}}, "uri": "file:///path/to/project/Util.cinch"}}

--> {"jsonrpc": "2.0", "id": 5, "method": "textDocument/definition", "params": {"textDocument": {"uri": "file:///path/to/project/Main.cinch"}, "position": {"line": 5, "character": 7}}}
<-- {"id": 5, "jsonrpc": "2.0", "result": {"range": {"end": {"character": 1, "line": 5}, "start": {"character": 0, "line": 5}}, "uri": "file:///path/to/project/Util.cinch"}}
```

Returned URIs are constructed relative to the client's `rootUri`.

## `textDocument/publishDiagnostics`

Sending a type error via `didChange` republishes diagnostics for every open document:

```text
--> {"jsonrpc": "2.0", "method": "textDocument/didChange", "params": {"textDocument": {"uri": "file:///path/to/project/Main.cinch", "version": 2}, "contentChanges": [{"text": "module Main where\n\nimport Util\n\nmain :: Int\nmain = quad \"x\"\n"}]}}
<-- {"jsonrpc": "2.0", "method": "textDocument/publishDiagnostics", "params": {"diagnostics": [{"message": "type mismatch: expected String, got Int (in Main.cinch)", "range": {"end": {"character": 13, "line": 5}, "start": {"character": 12, "line": 5}}, "severity": 1, "source": "cinch"}], "uri": "file:///path/to/project/Main.cinch"}}
<-- {"jsonrpc": "2.0", "method": "textDocument/publishDiagnostics", "params": {"diagnostics": [], "uri": "file:///path/to/project/Util.cinch"}}
```

Diagnostic fields:

- `severity`: `1` (error), `2` (warning), or `3` (information).
- `source`: always `"cinch"`.
- `range`: currently a single-character span at the diagnostic's start position.
- `message`: preserves the compiler's `(in File.cinch)` suffix.

## `didClose`, unsupported methods, and shutdown

```text
--> {"jsonrpc": "2.0", "method": "textDocument/didClose", "params": {"textDocument": {"uri": "file:///path/to/project/Main.cinch"}}}
<-- {"jsonrpc": "2.0", "method": "textDocument/publishDiagnostics", "params": {"diagnostics": [], "uri": "file:///path/to/project/Main.cinch"}}

--> {"jsonrpc": "2.0", "id": 7, "method": "textDocument/rename", "params": {"textDocument": {"uri": "file:///path/to/project/Main.cinch"}, "position": {"line": 5, "character": 7}, "newName": "twice"}}
<-- {"error": {"code": -32601, "message": "method not found: textDocument/rename"}, "id": 7, "jsonrpc": "2.0"}

--> {"jsonrpc": "2.0", "id": 8, "method": "shutdown", "params": null}
<-- {"id": 8, "jsonrpc": "2.0", "result": null}
--> {"jsonrpc": "2.0", "method": "exit", "params": null}
```

Closing a document drops it from the analysis rather than falling back to the saved file on disk.

# Positions and Unicode encoding

| Convention | Current behaviour |
| --- | --- |
| Indexing | LSP requests use 0-based `line` and `character`; internally the checker stores 1-based positions and converts at the protocol boundary. |
| Exact site matching | `hover` and `definition` require the exact **start column** of the identifier; querying mid-identifier returns `null`. |
| Unicode columns | `initialize` does not negotiate `positionEncoding`. The lexer counts Unicode scalar values (`char`), whereas default LSP clients count UTF-16 code units. Positions on lines containing non-BMP characters (e.g. emoji) will diverge until encoding negotiation is added. |

# Semantic tokens

The server implements `textDocument/semanticTokens/full` (no delta requests) with this 13-entry legend:

| Index | Token type |
| --- | --- |
| 0 | `keyword` |
| 1 | `variable` |
| 2 | `function` |
| 3 | `type` |
| 4 | `class` |
| 5 | `enumMember` |
| 6 | `method` |
| 7 | `property` |
| 8 | `namespace` |
| 9 | `number` |
| 10 | `string` |
| 11 | `operator` |
| 12 | `comment` |

For `Main.cinch` with both files open:

```text
--> {"jsonrpc": "2.0", "id": 2, "method": "textDocument/semanticTokens/full", "params": {"textDocument": {"uri": "file:///path/to/project/Main.cinch"}}}
<-- {"id": 2, "jsonrpc": "2.0", "result": {"data": [0, 0, 6, 0, 0, 0, 7, 4, 3, 0, 0, 5, 5, 0, 0, 2, 0, 6, 0, 0, 0, 7, 4, 3, 0, 2, 0, 4, 2, 0, 0, 8, 3, 3, 0, 1, 0, 4, 2, 0, 0, 7, 4, 2, 0, 0, 5, 2, 9, 0]}}
```

Each 5-integer tuple is `(deltaLine, deltaStart, length, tokenType, tokenModifiers)` where `deltaStart` is relative to the previous token on the same line and absolute on a new line:

| Line | Columns | Text | Token type |
| --- | --- | --- | --- |
| 0 | 0..6 | `module` | `keyword` |
| 0 | 7..11 | `Main` | `type` |
| 0 | 12..17 | `where` | `keyword` |
| 2 | 0..6 | `import` | `keyword` |
| 2 | 7..11 | `Util` | `type` |
| 4 | 0..4 | `main` | `function` |
| 4 | 8..11 | `Int` | `type` |
| 5 | 0..4 | `main` | `function` |
| 5 | 7..11 | `quad` | `function` |
| 5 | 12..14 | `21` | `number` |

Two nuances worth knowing:

- **Comments:** comments are lexical trivia omitted from the syntax tree, so the server emits explicit `comment` semantic tokens (one per line of a block comment). Editors that disable semantic tokens by default (such as Zed) must enable them for Cinch to highlight comments from the server.
- **Unresolved references:** a reference is tagged `function` only when its declaration is present in open buffers; if `Util.cinch` is closed, `quad` falls back to `variable`.

# Current limitations

| Limitation | Consequence |
| --- | --- |
| No disk discovery or file watching | Unopened sibling modules are invisible; external disk edits require reopening the buffer |
| No closed-buffer fallback | Closing a file removes its definitions from open importers |
| Full text sync only | Incremental range edits are not supported |
| Single-character ranges | Diagnostics and definitions span 1 character; no diagnostic codes or related locations |
| Unimplemented requests | `rename`, `references`, `formatting`, `codeAction`, and `signatureHelp` return `-32601` |
| No cancellation or version tagging | In-flight updates cannot be cancelled and published diagnostics carry no document version |
| Single workspace root | One `rootUri` per server process |

When diagnosing build issues across unopened files, run `cinch check` on disk.

# Planned capabilities

<span class="support-tag support-tag--planned">Coming soon</span> Planned editor-integration work includes disk-backed workspace discovery and file watching, version-tagged diagnostics with stable codes and multi-character spans, context-aware completion with auto-import edits, go-to-references, semantic rename, signature help, formatting, and packaged editor extensions.

# Editor integrations in the repository

The repository ships unpublished editor grammars and a VS Code client scaffold:

| Editor | Path | Included |
| --- | --- | --- |
| VS Code / Cursor | `tooling/editors/vscode/` | `cinch` language ID, TextMate grammar (`source.cinch`), LSP client |
| Vim | `tooling/editors/vim/` | `ftdetect` for `.cinch` and syntax highlighting |
| Emacs | `tooling/editors/emacs/cinch-mode.el` | `cinch-mode` font-lock rules |
| Sublime Text | `tooling/editors/sublime/Cinch.sublime-syntax` | Syntax definition |
| Zed | `tooling/editors/zed/` | Extension manifest over `tooling/tree-sitter-cinch` |

In the VS Code extension, `cinch.lsp.command` expects a **single executable path** (not a command with arguments) and defaults to `bin/cinch-lsp`, which runs `cinch lsp` if `cinch` is on `PATH` or falls back to `tooling/lsp-driver`. To point at a specific build, wrap it in a small executable script:

```sh
#!/bin/sh
exec /path/to/cinch lsp
```

Note that `cinch.lsp.trace` is declared in `package.json` but currently unread by `extension.js`.

# Troubleshooting

| Symptom | Cause and fix |
| --- | --- |
| Highlighting works, no diagnostics or hover | Verify the LSP client started `cinch lsp`, the buffer language ID is `cinch`, and the executable path is valid. |
| Running `cinch lsp` in a shell hangs | Expected—it waits for `Content-Length`-framed JSON-RPC input on stdin. |
| Startup protocol parse failure | Something wrote non-LSP output to stdout; redirect logs to stderr. |
| Garbled parse errors after editing | The client sent incremental range edits instead of full document text. |
| Imported symbol reported as unknown | Open the defining `.cinch` file in the editor so it enters the server's active buffer set. |
| Hover returns nothing on a valid symbol | Place the cursor on the first character of the identifier. |
| Diagnostics persist after external git/disk edits | Reopen the buffer; file watching is not yet implemented. |

See [diagnostics and error messages](21-editors-and-diagnostics.md) for interpreting compiler diagnostics and [the REPL](26-repl.md) for interactive exploration over the same analysis layer.
