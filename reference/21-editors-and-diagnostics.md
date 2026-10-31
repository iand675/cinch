# Editors, analysis APIs, and diagnostics

The compiler's analysis layer is shared by the language server and REPL APIs. It retains source/definition identity, diagnostics, and incremental reports rather than implementing a second independent checker in each frontend.

# Language server

The [dedicated language server guide](27-language-server.md) describes the **Coming soon** complete editor experience, including setup, diagnostics, completion, navigation, semantic highlighting, and refactoring. It also identifies the protocol handlers that already work.

Run `cinch lsp` over stdio using Content-Length-framed JSON-RPC. Standard output carries protocol messages, so an editor launcher must not mix shell banners into that stream. The server supports initialization/shutdown, document open/change/close, hover, definition lookup, completion, full semantic tokens, and published diagnostics through its implemented request handlers.

Editor integrations are in `tooling/editors/` for VS Code/Cursor, Vim, Emacs, Sublime, and Zed. The VS Code launcher executes `cinch lsp` when available and supports an override command. `tooling/lsp-driver/` is a standalone Rust stdio launcher for the same server. The server does not promise rename, formatting, code actions, or every optional LSP request merely because it speaks LSP.

# REPL API

The [REPL guide](26-repl.md) describes the **Coming soon** terminal experience, with worked sessions, the complete command table, loading/reloading, multiline input, error recovery, and evaluation boundaries.

`cinch::repl::Repl` maintains definitions across inputs. `eval` returns output, an incomplete-input indication, or quit. Supported session commands include `:type`, `:kind`, `:browse`, `:load`, `:reload`, `:history`, `:help`, and `:quit`. Definitions, multi-line continuation, expression evaluation, and completion are tested through the Rust API.

<span class="support-tag support-tag--planned">Coming soon</span> **a CLI REPL command.** `cinch repl` is not dispatched by the current executable. The API exists and is useful to frontend authors; a terminal program integrating it is a separate missing user-facing surface. This distinction also applies to persistent/watch compilation sessions.

# Read diagnostics by phase

| Symptom | First check |
| --- | --- |
| Unexpected token or joined declarations | Indentation, explicit delimiters, comment priority, minus spacing. |
| Unknown module/name | Declared module identity, sibling source files, selected exports/imports, non-embedded dependency closure. |
| No or ambiguous instance | Every required constraint, unique implicit candidates, module policy selection. |
| Effect rows do not match | Callback rows, handler-introduced effects, capability supply and runtime identity. |
| Multiplicity/escape error | Uses per path, task/resumption captures, repeated projection places, owner lifetime. |
| Needs `Dec` evidence or a branch fact | Correct predicate, dynamic success branch, range facts, mutation invalidation. |
| Unknown type in an interface | Schema/nominal dependency closure and the compiler version that produced it. |
| Clang/link failure | Compatible clang, runtime source/library availability, external symbol/object requirements. |
| Exhausted | Heap/stack/resource diagnostic and configured runtime bounds. |

A diagnostic's span is attached to the failing source or generated expansion provenance. Prefer fixing the originating contract to weakening signatures broadly. `check` avoids native linking and is useful for isolating frontend failures. `emit-ir` helps distinguish accepted source from code-generation/link issues.

# Source of truth and bug reports

Report the Rust compiler checkout, exact command, minimal source and required sibling files, target/profile when using APIs, and the complete diagnostic. Avoid including secrets from staging input or process environments. The [support page](16-support-and-roadmap.md) identifies known current boundaries and planned features, so a named refusal can be distinguished from a regression.

Implementation: `lsp.rs`, `analysis.rs`, `repl.rs`; suites: `lsp.rs`, `repl.rs`, `incremental.rs`; parser/editor grammar checks: `tooling/check-grammar-drift.sh`, `tooling/check-real-files-parse.sh`.
