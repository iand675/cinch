# Language server and editor integration

<span class="support-tag support-tag--planned">Coming soon</span> **The packaged editor experience is work in progress.** A stdio language server exists in the compiler and answers six request types over the project's open documents. This page documents what runs today from a real protocol session, then describes the intended experience and marks it as intended.

The server shares `compiler/src/analysis.rs` with the REPL, so its answers follow the same module visibility, instance, effect, refinement, and ownership rules as the command line.

Syntax highlighting and language-server analysis are separate facilities. An editor grammar colours a file without checking it; a running server adds compiler-backed answers. Coloured tokens alone do not show that a server is connected.

# Launch and transport

```sh
cinch lsp
```

`cinch lsp` exists and is a protocol process, not a terminal program. Standard input and standard output carry JSON-RPC 2.0 messages framed with `Content-Length`, and nothing else. A terminal that runs it directly appears to wait, because it is waiting for a frame. Standard error is free for logging.

`tooling/lsp-driver/` is a standalone Rust binary that runs the same `LspServer::run` over stdio, for editors that prefer a dedicated executable:

```sh
cargo run --quiet --manifest-path tooling/lsp-driver/Cargo.toml --release
```

Neither launcher changes the language rules; they differ only in which process hosts the server.

For a generic client:

| Setting | Value |
| --- | --- |
| Executable | `cinch`, or the absolute path to the compiler |
| Arguments | `lsp` as a separate argument |
| Transport | standard input and output |
| Language identifier | `cinch` |
| File extension | `.cinch` |
| Workspace root | the project directory, sent as `rootUri` |

# Implemented protocol methods

The dispatch in `compiler/src/lsp.rs` is the complete list. Anything absent from it gets a `-32601` error, not a partial answer.

| Method | Kind | What it does |
| --- | --- | --- |
| `initialize` | request | Sets `rootUri` and `root_path`, creates the analysis rooted there, returns the capabilities |
| `initialized` | notification | Ignored |
| `workspace/didChangeConfiguration` | notification | Ignored; no configuration schema is read |
| `textDocument/didOpen` | notification | Stores the buffer as an open document, updates, and publishes diagnostics for every open document |
| `textDocument/didChange` | notification | Takes the last entry of `contentChanges` as the whole document, updates, and publishes for every open document |
| `textDocument/didClose` | notification | Drops the buffer and publishes an empty diagnostic list for that document |
| `textDocument/hover` | request | Markdown code block with `name :: scheme`, or `null` |
| `textDocument/definition` | request | One `Location` with a one-character range on the declaration's signature line, or `null` |
| `textDocument/completion` | request | `{isIncomplete, items}` for the word at the cursor |
| `textDocument/semanticTokens/full` | request | One full-document token array, delta encoded |
| `shutdown` | request | Sets the shutdown flag and answers `null` |
| `exit` | notification | Ends the read loop |

Two of these acknowledge a parameter they do not use. `didChange` reads the final `text` and ignores any range, which matches the advertised full synchronization; a client configured for incremental ranges will lose text. `didOpen` reads `uri` and `text` and ignores `languageId` and `version`.

# A recorded protocol session

The session below ran against a project with `Main.cinch` and `Util.cinch` side by side. Requests and responses are copied from the wire, with `-->` for a message sent to the server and `<--` for a message received. The message bodies are shown without their `Content-Length` frames.

The project:

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

`cinch run Main.cinch` accepts the pair and exits 0, so the diagnostics in the session below are the server's own answers about a program the checker accepts.

## initialize

```text
--> {"jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {"processId": null, "rootUri": "file:///tmp/cinch-docs/ref-tooling/lsp-proj", "capabilities": {}}}
<-- {"id": 1, "jsonrpc": "2.0", "result": {"capabilities": {"completionProvider": {"resolveProvider": false, "triggerCharacters": ["."]}, "definitionProvider": true, "hoverProvider": true, "semanticTokensProvider": {"full": true, "legend": {"tokenModifiers": [], "tokenTypes": ["keyword", "variable", "function", "type", "class", "enumMember", "method", "property", "namespace", "number", "string", "operator"]}}, "textDocumentSync": 1}, "serverInfo": {"name": "cinch", "version": "0.1.0"}}}
```

`textDocumentSync: 1` is full synchronization. `resolveProvider: false` means the server does not hold back completion details for a second request. There is no `renameProvider`, no `documentFormattingProvider`, and no `signatureHelpProvider`, so a client should not offer those actions for a Cinch buffer.

## didOpen

The client opened `Util.cinch` first, then `Main.cinch`. Each open published diagnostics for every open document, which is why the second open produced two notifications.

```text
--> {"jsonrpc": "2.0", "method": "textDocument/didOpen", "params": {"textDocument": {"uri": "file:///tmp/cinch-docs/ref-tooling/lsp-proj/Util.cinch", "languageId": "cinch", "version": 1, "text": "module Util where\n\ndouble :: Int -> Int\ndouble n = n + n\n\nquad :: Int -> Int\nquad n = double (double n)\n"}}}
<-- {"jsonrpc": "2.0", "method": "textDocument/publishDiagnostics", "params": {"diagnostics": [], "uri": "file:///tmp/cinch-docs/ref-tooling/lsp-proj/Util.cinch"}}
--> {"jsonrpc": "2.0", "method": "textDocument/didOpen", "params": {"textDocument": {"uri": "file:///tmp/cinch-docs/ref-tooling/lsp-proj/Main.cinch", "languageId": "cinch", "version": 1, "text": "module Main where\n\nimport Util\n\nmain :: Int\nmain = quad 21\n"}}}
<-- {"jsonrpc": "2.0", "method": "textDocument/publishDiagnostics", "params": {"diagnostics": [], "uri": "file:///tmp/cinch-docs/ref-tooling/lsp-proj/Main.cinch"}}
<-- {"jsonrpc": "2.0", "method": "textDocument/publishDiagnostics", "params": {"diagnostics": [], "uri": "file:///tmp/cinch-docs/ref-tooling/lsp-proj/Util.cinch"}}
```

The order matters in practice. The server analyses open documents only: it does not read sibling files from disk. Opening `Main.cinch` alone reports `unknown name quad (in Main.cinch)`, because `Util` is not in the analysis yet. Open the imported files, which an editor does as you browse the project, before trusting a cross-module diagnostic.

## hover

Position `{line: 5, character: 7}` is the `q` of `quad` in `main = quad 21`, using zero-based lines and characters.

```text
--> {"jsonrpc": "2.0", "id": 2, "method": "textDocument/hover", "params": {"textDocument": {"uri": "file:///tmp/cinch-docs/ref-tooling/lsp-proj/Main.cinch"}, "position": {"line": 5, "character": 7}}}
<-- {"id": 2, "jsonrpc": "2.0", "result": {"contents": {"kind": "markdown", "value": "```cinch\nquad :: (Int -> Int)\n```"}}}
```

The scheme comes from `Util`'s checked interface, which renders arrows in parentheses. `null` is the answer for a position that is not a recorded declaration or reference, including a literal:

```text
--> {"jsonrpc": "2.0", "id": 6, "method": "textDocument/hover", "params": {"textDocument": {"uri": "file:///tmp/cinch-docs/ref-tooling/lsp-proj/Main.cinch"}, "position": {"line": 5, "character": 12}}}
<-- {"id": 6, "jsonrpc": "2.0", "result": null}
```

Character 12 is the `2` of `21`.

## completion

```text
--> {"jsonrpc": "2.0", "id": 3, "method": "textDocument/completion", "params": {"textDocument": {"uri": "file:///tmp/cinch-docs/ref-tooling/lsp-proj/Main.cinch"}, "position": {"line": 5, "character": 10}}}
<-- {"id": 3, "jsonrpc": "2.0", "result": {"isIncomplete": false, "items": [{"detail": "(Int -> Int)", "kind": 3, "label": "quad"}]}}
```

The server takes the identifier ending at the cursor, `qua`, and filters session, imported, and prelude names by that prefix. With an empty prefix it returns the whole visible set, which is large: the first request in this session, at character 7 before any name was typed, returned a list beginning `%`, `%`, `&`, `&&`, `*`, `*`, `+`, so a client should do its own filtering as the user types.

`kind: 3` is a function, which is why `detail` carries the scheme. The mapping from declaration kind to LSP completion kind is in `completion_json`: value, law, instance, and effect operation `3`, method `2`, type and effect `25`, constructor `4`, field `5`, class `7`.

## definition

A use inside `Util.cinch`, at the first `double` of `quad n = double (double n)`:

```text
--> {"jsonrpc": "2.0", "id": 4, "method": "textDocument/definition", "params": {"textDocument": {"uri": "file:///tmp/cinch-docs/ref-tooling/lsp-proj/Util.cinch"}, "position": {"line": 6, "character": 9}}}
<-- {"id": 4, "jsonrpc": "2.0", "result": {"range": {"end": {"character": 1, "line": 2}, "start": {"character": 0, "line": 2}}, "uri": "file:///tmp/cinch-docs/ref-tooling/lsp-proj/Util.cinch"}}
```

Line 2 is `double :: Int -> Int`, the signature line, and the range covers its first character. The answer points at the signature, not at the equation.

A use in another module resolves across files:

```text
--> {"jsonrpc": "2.0", "id": 5, "method": "textDocument/definition", "params": {"textDocument": {"uri": "file:///tmp/cinch-docs/ref-tooling/lsp-proj/Main.cinch"}, "position": {"line": 5, "character": 7}}}
<-- {"id": 5, "jsonrpc": "2.0", "result": {"range": {"end": {"character": 1, "line": 5}, "start": {"character": 0, "line": 5}}, "uri": "file:///tmp/cinch-docs/ref-tooling/lsp-proj/Util.cinch"}}
```

Its `uri` is built from the client's own `rootUri` plus the workspace-relative key, so the server never invents an absolute path.

## diagnostics

The client replaced the document with one whose argument type is wrong:

```text
--> {"jsonrpc": "2.0", "method": "textDocument/didChange", "params": {"textDocument": {"uri": "file:///tmp/cinch-docs/ref-tooling/lsp-proj/Main.cinch", "version": 2}, "contentChanges": [{"text": "module Main where\n\nimport Util\n\nmain :: Int\nmain = quad \"x\"\n"}]}}
<-- {"jsonrpc": "2.0", "method": "textDocument/publishDiagnostics", "params": {"diagnostics": [{"message": "type mismatch: expected String, got Int (in Main.cinch)", "range": {"end": {"character": 13, "line": 5}, "start": {"character": 12, "line": 5}}, "severity": 1, "source": "cinch"}], "uri": "file:///tmp/cinch-docs/ref-tooling/lsp-proj/Main.cinch"}}
<-- {"jsonrpc": "2.0", "method": "textDocument/publishDiagnostics", "params": {"diagnostics": [], "uri": "file:///tmp/cinch-docs/ref-tooling/lsp-proj/Util.cinch"}}
```

The publish covers every open document, so `Util.cinch` is republished even though nothing in it changed.

Four properties of these diagnostics follow from the shape:

- `severity` is `1` for an error, `2` for a warning, `3` for information.
- `source` is always `cinch`.
- The range is one character wide, at the diagnostic's position. A client that underlines the range shows a single-character mark rather than the offending expression.
- The message keeps the command-line suffix `(in Main.cinch)`, so the file is named twice: once by `uri`, once in the text.

## close

```text
--> {"jsonrpc": "2.0", "method": "textDocument/didClose", "params": {"textDocument": {"uri": "file:///tmp/cinch-docs/ref-tooling/lsp-proj/Main.cinch"}}}
<-- {"jsonrpc": "2.0", "method": "textDocument/publishDiagnostics", "params": {"diagnostics": [], "uri": "file:///tmp/cinch-docs/ref-tooling/lsp-proj/Main.cinch"}}
```

Closing clears the document's diagnostics and drops its buffer. The server does not fall back to the file's saved contents, so close a buffer only when you are done with it.

## An unimplemented method

```text
--> {"jsonrpc": "2.0", "id": 7, "method": "textDocument/rename", "params": {"textDocument": {"uri": "file:///tmp/cinch-docs/ref-tooling/lsp-proj/Main.cinch"}, "position": {"line": 5, "character": 7}, "newName": "twice"}}
<-- {"error": {"code": -32601, "message": "method not found: textDocument/rename"}, "id": 7, "jsonrpc": "2.0"}
```

## shutdown and exit

```text
--> {"jsonrpc": "2.0", "id": 8, "method": "shutdown", "params": null}
<-- {"id": 8, "jsonrpc": "2.0", "result": null}
--> {"jsonrpc": "2.0", "method": "exit", "params": null}
```

`exit` produces no response; the process leaves its read loop.

# Positions and ranges

Two conventions differ from what a first client expects.

| Direction | Convention |
| --- | --- |
| Request positions | Zero-based `line` and `character`, as LSP specifies |
| Recorded sites | The analysis stores one-based line and column, so the server adds one on the way out and subtracts one on the way in |
| Matching | A request must name the exact recorded start column, not any column inside the name |
| Answers | A `null` for any position that is not exactly a recorded declaration or reference site |

Because matching is exact, a hover request one character into a name returns `null`. Ask at the first character of the identifier.

Unicode: the `initialize` result carries no `positionEncoding`, so nothing is negotiated, and the checker's columns count Unicode scalar values (`Lexer::advance` increments the column once per `char`). A client that measures columns in UTF-16 code units, which is the LSP default, will disagree on any line containing a character outside the Basic Multilingual Plane. There are no Unicode tests for the protocol yet, so keep editor documents to the Basic Multilingual Plane until that is closed.

# Semantic tokens

The server advertises one full-document request with a fixed legend:

```text
keyword, variable, function, type, class, enumMember,
method, property, namespace, number, string, operator
```

For `Main.cinch` with both files open:

```text
--> {"jsonrpc": "2.0", "id": 2, "method": "textDocument/semanticTokens/full", "params": {"textDocument": {"uri": "file:///tmp/cinch-docs/ref-tooling/lsp-proj/Main.cinch"}}}
<-- {"id": 2, "jsonrpc": "2.0", "result": {"data": [0, 0, 6, 0, 0, 0, 7, 4, 3, 0, 0, 5, 5, 0, 0, 2, 0, 6, 0, 0, 0, 7, 4, 3, 0, 2, 0, 4, 2, 0, 0, 8, 3, 3, 0, 1, 0, 4, 2, 0, 0, 7, 4, 2, 0, 0, 5, 2, 9, 0]}}
```

Each group of five numbers is `deltaLine, deltaStart, length, tokenType, tokenModifiers`. Decoded:

| Line | Columns | Text | Token type |
| --- | --- | --- | --- |
| 0 | 0 to 6 | `module` | keyword |
| 0 | 7 to 11 | `Main` | type |
| 0 | 12 to 17 | `where` | keyword |
| 2 | 0 to 6 | `import` | keyword |
| 2 | 7 to 11 | `Util` | type |
| 4 | 0 to 4 | `main` | function |
| 4 | 8 to 11 | `Int` | type |
| 5 | 0 to 4 | `main` | function |
| 5 | 7 to 11 | `quad` | function |
| 5 | 12 to 14 | `21` | number |

`deltaStart` is relative to the previous token's start only when `deltaLine` is zero, and absolute otherwise. Every response is a full document; the server does not implement the delta request, so a client must not advertise or request `semanticTokens/full/delta`.

A reference is classified `function` only when its declaration is in the analysis. With `Util.cinch` closed, the same request classifies `quad` as `variable`, because the reference is unresolved. Grammar highlighting is unaffected either way, and neither classification claims that the buffer checks.

# What the server does not do

These are current boundaries, not configuration problems. Each one is visible from the capability set the server returns.

| Missing | Consequence |
| --- | --- |
| Disk discovery and file watching | A module that is not open is invisible, so imports it defines report as unknown names |
| Closed-buffer restoration | Closing a file removes its definitions from the analysis instead of reverting to the saved text |
| Incremental text changes | Only the final `text` of a change is used |
| Diagnostic codes and related locations | A diagnostic carries a message, a one-character range, a severity, and the source `cinch` |
| Quick fixes, code actions, rename, references, formatting, signature help | The method is absent and answers `-32601` |
| Cancellation and revision tracking | A slow update is not cancelled; answers carry no version |
| Multi-root workspaces | One `rootUri` per process |

For a program that does not check, `cinch check` on the saved source is the reliable authority while these gaps remain.

# The intended experience

Everything in this section is a design target. Nothing in it has a handler today.

The finished integration would check unsaved source, explain types and diagnostics where the cursor is, navigate between declarations, and help make checked edits across a project. Diagnostics would carry stable codes, related locations, and quick fixes that present a reviewable source edit rather than hiding a message. Completion would filter by context, show documentation on selection, and offer a real import edit for a name outside the current scope, honouring export visibility and name collisions. Rename would preview consistent edits by declaration identity. Formatting would preserve layout, comments, and attribute attachment. Answers would be associated with the document version that produced them, so a slow check of an old buffer could not overwrite newer diagnostics.

Refactoring must preserve meaning. A suggested fix must not weaken a refinement, discard an effect row, or insert an `unsafe` escape to remove a diagnostic.

# Editor configuration

The repository carries syntax files and one language-client scaffold. Nothing is published.

| Editor | Path | What it provides |
| --- | --- | --- |
| VS Code, Cursor | `tooling/editors/vscode/` | language id `cinch`, `source.cinch` grammar, `cinch.lsp.command`, and a language client |
| Vim | `tooling/editors/vim/` | `ftdetect` for `.cinch` and a syntax file |
| Emacs | `tooling/editors/emacs/cinch-mode.el` | `cinch-mode` with font-lock rules |
| Sublime Text | `tooling/editors/sublime/Cinch.sublime-syntax` | a syntax definition |
| Zed | `tooling/editors/zed/` | an extension manifest over `tooling/tree-sitter-cinch` |

For editors with a built-in LSP client, set the executable to the compiler and the arguments to `lsp`, and give the client the project directory as the root.

The VS Code extension reads one setting, `cinch.lsp.command`, and falls back to `bin/cinch-lsp`. That launcher execs `cinch lsp` when `cinch` is on `PATH` and otherwise runs the driver crate from a source checkout. The setting is a single executable path, so a value such as `cinch lsp` is a path with a space in it and will not start. Use a launcher script if you need arguments:

```sh
#!/bin/sh
exec /absolute/path/to/compiler/target/debug/cinch lsp
```

The `cinch.lsp.command` description inside `package.json` still claims the compiler has no `lsp` subcommand. It does; the description is stale. The extension also declares `cinch.lsp.trace`, and `extension.js` never reads it, so changing that setting enables no tracing.

# Troubleshooting

| Symptom | Check |
| --- | --- |
| Highlighting works and there is no analysis | The client started the server, the buffer's language is `cinch`, and the executable is the compiler or a working launcher |
| The process sits in a terminal | It is waiting for frames; connect a client |
| The client cannot find the executable | Use an absolute path and check the `PATH` the editor inherits |
| An override setting does nothing | It takes one executable path, not a shell command line |
| Startup parsing fails | Something wrote a banner to standard output; move it to standard error |
| Edits produce nonsense parse errors | The client sent range edits; send the whole document as `text` |
| An import reports as an unknown name | The defining document is not open |
| A rename or format action reports an unsupported request | The capability is absent by design today |
| A diagnostic survives an external file change | Reopen the document; there is no file watching |

A useful bug report contains the compiler revision, the client and its version, the root you selected, the minimal sources, whether buffers were saved, the exact request and response, and the result of `cinch check` on the same saved files. Protocol traces contain full document text, so remove anything sensitive before posting.

# Current implementation

| Surface | Present boundary |
| --- | --- |
| `cinch lsp` | **Available.** Stdio server in the compiler binary, also reachable through `tooling/lsp-driver/`. |
| Document synchronization | **Available for open documents.** Full text only; no disk discovery, watching, or closed-buffer restoration. |
| Diagnostics | **Available.** Published for open documents as `(range, severity, message, source)` with one-character ranges. |
| Hover | **Available.** A `name :: scheme` code block for an exact declaration or reference position. |
| Definition | **Available across open files.** Points at the signature line with a one-character range. |
| Completion | **Available.** Prefix filtered over session, imported, and prelude names; no resolve step. |
| Semantic tokens | **Available, full document only.** No delta request and no position-encoding negotiation. |
| Rename, references, formatting, code actions, signature help | **Not implemented.** The server answers `-32601`. |
| Packaged editor installation | **Coming soon.** No marketplace release and no version matching. |

Implementation: `compiler/src/lsp.rs`, `compiler/src/analysis.rs`, and `compiler/src/main.rs`; suites: `compiler/tests/lsp.rs`. The [diagnostics guide](21-editors-and-diagnostics.md) maps message text to the rule that failed, and the [REPL guide](26-repl.md) covers the other front end over the same analysis layer.
