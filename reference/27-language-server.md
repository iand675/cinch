# Language server and editor integration

<span class="support-tag support-tag--planned">Coming soon</span> **This chapter describes the intended complete editor experience.** Cinch's Rust compiler already includes a stdio language server and several useful requests. Packaged editor setup, comprehensive project handling, and the richer assistance described below remain work in progress. The [current implementation table](#current-implementation) distinguishes working protocol handlers from the idealized workflow.

The language server connects an editor to Cinch's compiler analysis. It should check unsaved source, explain types and diagnostics, navigate between declarations, and help make checked edits across a project. Its answers must follow the same language rules as the command-line compiler, including module visibility, instances, effects, refinements, ownership, and dependent indices.

Syntax highlighting and language-server analysis are separate facilities. An editor grammar can colour a file without checking it. A running language server adds compiler-backed information; the presence of coloured tokens alone does not establish that it is connected.

# Intended setup

The intended setup is to install a matching compiler and editor integration, open the directory containing your Cinch project, and open a `.cinch` file. The editor starts one server for the project and connects its diagnostics, hover, completion, navigation, and semantic highlighting to the normal editor controls.

The server command is:

```sh
cinch lsp
```

**This command already exists.** It is a protocol process, not an interactive terminal program. Standard input and standard output carry Content-Length-framed JSON-RPC messages. The editor supplies requests; a terminal running the process directly will normally appear to wait for input. Launchers must send any human-readable logging to standard error.

Use these settings when configuring a generic LSP client:

| Setting | Value |
| --- | --- |
| Executable | `cinch`, or the absolute path to the compiler binary. |
| Arguments | `lsp` as a separate argument. |
| Transport | Standard input/output. |
| Language identifier | `cinch`. |
| Source file extension | `.cinch`. |
| Workspace root | The source project's directory, supplied as `rootUri`. |

These describe the existing launch protocol. Automatic project discovery, installation, version matching, and a polished first-run setup belong to the **Coming soon** editor experience. There is no published extension or downloadable compiler bundle implied by this chapter; the site's [downloads](/downloads/) are still labelled samples.

## VS Code and Cursor

The repository contains an extension under `tooling/editors/vscode/`. It registers `.cinch`, provides a TextMate grammar, and starts a language client for file-backed Cinch documents. It is source tooling, rather than a claim of marketplace distribution.

The existing `cinch.lsp.command` setting is a single executable path. It is **not** a shell command string split into an executable and arguments. If overriding it, point it to a small executable launcher that runs the compiler with `lsp`:

```sh
#!/bin/sh
exec /absolute/path/to/cinch/compiler/target/release/cinch lsp
```

Save the launcher at a path appropriate to your machine, make it executable, and set `cinch.lsp.command` to that launcher's absolute path. The path in the example is a placeholder. Use an actual compiler built from the checkout you are editing. Do not set the current field to the literal string `cinch lsp`.

The extension also contains a default launcher intended to prefer `cinch` on `PATH` and fall back to the standalone Rust driver. Automatic source-checkout discovery and fallback setup need validation before being treated as a supported installation path. The direct launcher above makes the selected compiler explicit.

**Coming soon:** packaged installation, separate executable/argument configuration, useful startup failures, server restart controls, and connected protocol tracing. The manifest currently declares `cinch.lsp.trace`, but the extension does not wire that setting into the client; changing it should not be described as a working tracing procedure.

## Other editors

The repository also contains Vim, Emacs, Sublime Text, and Zed syntax integrations under `tooling/editors/`. These files identify or highlight Cinch source; their presence does not imply that each editor already has a complete language-client installation.

For an editor with an LSP client, configure the executable, arguments, language identifier, and project root listed above. Prefer a direct compiler invocation while developing the integration. `tooling/lsp-driver/` is a standalone Rust executable for the same server:

```sh
cargo run --quiet --manifest-path tooling/lsp-driver/Cargo.toml --release
```

Run this from the repository root, or supply an absolute manifest path. Cargo's build messages must remain off the protocol output stream. The driver is an alternative stdio launcher, not an additional server with different language semantics.

# Project and document model

The intended server maintains a checked view of the workspace together with unsaved editor buffers. An open buffer takes precedence over the saved file for that document. Changes propagate to affected dependents; unrelated modules should not need a fresh full check after every keystroke.

On closing a file, the intended project model resumes its saved source if that file still belongs to the project. Renames, deletions, new modules, and external changes should update the dependency graph and clear stale diagnostics. Broken source should leave enough information for useful completion and navigation while the user repairs it.

**Coming soon: complete workspace lifecycle.** The current protocol handles document open, full-text change, and close. It does not provide the whole discovery, disk watching, closed-file restoration, versioned asynchronous analysis, or multi-root workspace experience described here. Open the correct project root, and use `cinch check` on saved source to distinguish integration gaps from compiler errors.

The completed implementation should associate answers with the document revision that produced them. A slow check of an older buffer must not overwrite newer diagnostics or supply refactoring edits at stale positions. Cancellation should discard obsolete editor work without changing checked source semantics.

# Diagnostics

The editor should underline the relevant source range and show the compiler's explanation, including the originating contract when available. Errors and warnings keep their distinct severity. When a problem is fixed, the previous diagnostic must be cleared rather than accumulating in the Problems view.

Useful diagnostics include syntax/layout errors, unknown names, module/import failures, mismatched types, unresolved instances, effect/capability errors, ownership escapes, and unsatisfied refinement or dependent-proof obligations. An editor can help locate these conditions; it cannot make an invalid program acceptable by hiding a message.

The current server publishes source diagnostics after opening or changing a document, and clears that document's diagnostics on close. Richer related locations, stable diagnostic codes, explanations for generated expansions, and quick fixes are part of the intended completion of the experience. Do not assume every CLI diagnostic already has a dedicated editor action.

For a proof error, inspect the required predicate and the facts available at that point. For an ownership error, inspect the value's uses, captures, and lifetime. The [diagnostics guide](21-editors-and-diagnostics.md) maps common symptoms to the relevant language rules.

# Hover and signature help

Hover should answer what the selected name means in this scope: its type or kind, relevant constraints, and useful declaration documentation. Effects and dependent indices belong in the displayed signature; omitting them can hide the actual calling contract.

The existing server supplies Markdown hover information through the analysis layer. The idealized interface extends that foundation with linked documentation, clear origins for imported names, and explanations of relevant evidence where available.

**Coming soon: signature help.** While applying a function, the editor should display its signature and identify the active argument, including explicit dictionary or capability requirements. It must respect Cinch's curried applications and operators. The current server does not advertise a signature-help handler.

# Completion

Completion should offer names that are meaningful at the cursor, including local bindings, visible declarations, imported exports, prelude names, and relevant qualified members. Entries should show enough type information to distinguish similar names and should preserve dash-style identifiers.

Basic analysis-backed name completion is implemented. The current server advertises `.` as a completion trigger and returns completion entries; it does not advertise a completion-resolve request.

**Coming soon:** richer context filtering, documentation on selection, and explicit import edits for names outside the current import scope. A completion that adds an import must present or apply a real source edit, honour export visibility, and handle conflicting names. Merely suggesting a name does not satisfy its instance, effect, or proof requirements.

# Navigation and refactoring

Go to Definition should take a use to the declaration it resolves to, including across source modules. It should use declaration identity, so two equally spelled names in different scopes do not become interchangeable. Definition lookup across files is already implemented.

The following complete the intended navigation and refactoring surface:

| Feature | Intended behaviour | Status |
| --- | --- | --- |
| Find References | Find uses of the selected declaration across the checked project, respecting shadowing and import aliases. | **Coming soon** |
| Rename | Preview consistent edits to a declaration and its uses; check name collisions and import/export consequences. | **Coming soon** |
| Document/workspace symbols | Navigate declarations by name and kind without searching raw source text. | **Coming soon** |
| Code actions | Offer justified source edits for supported diagnostics, with explicit imports and reviewable changes. | **Coming soon** |
| Formatting | Preserve parsing, layout, comments, and attribute attachment while applying consistent source formatting. | **Coming soon** |

These are target capabilities, not existing protocol promises. Their detailed UI and configuration are not yet finalized. Refactoring must preserve meaning; text replacement is insufficient for a rename, and a formatter must not move a `#[...]` attribute onto a different declaration or branch. A suggested fix must not silently weaken a refinement, discard an effect, or insert an unsafe escape to remove a diagnostic.

# Semantic highlighting

Semantic tokens distinguish names using compiler analysis rather than spelling alone. The current server advertises a full-document token response with these token types:

```text
keyword, variable, function, type, class, enumMember,
method, property, namespace, number, string, operator
```

The token legend determines how the editor maps these categories to its theme. Grammar highlighting remains useful before analysis succeeds. Semantic highlighting does not execute the program or constitute proof that every part of a broken buffer has checked.

Full responses encode token positions relative to preceding tokens within that response. That encoding is different from a semantic-token delta-update endpoint; the current server advertises full responses only.

# Protocol contract for client authors

The existing server accepts standard Content-Length-framed messages over stdio. Initialize it with the project root, observe its advertised capabilities, then send document lifecycle notifications and supported requests. Finish with `shutdown`, followed by `exit`.

| Area | Current protocol surface |
| --- | --- |
| Initialization | `initialize`, `initialized`. |
| Document synchronization | `textDocument/didOpen`, `didChange`, `didClose`; full-text synchronization (`textDocumentSync: 1`). |
| Diagnostics | Server notifications through `textDocument/publishDiagnostics`. |
| Queries | `textDocument/hover`, `definition`, `completion`. |
| Semantic tokens | `textDocument/semanticTokens/full`. |
| Termination | `shutdown`, `exit`. |

For each `didChange`, send the complete new document as the text change, rather than a range edit. The current implementation takes the final change's `text` as the whole document. Advertising full synchronization is not support for applying incremental range patches, even though the analysis underneath can reuse unchanged work.

`workspace/didChangeConfiguration` is accepted without implementing a configuration schema. Rename, references, formatting, code actions, signature help, and semantic-token delta requests are not current handlers. Clients must not infer them from the idealized feature descriptions in this chapter.

**Coming soon: protocol hardening.** Complete client compatibility includes robust URI/path handling, negotiated position encodings and Unicode tests, revision tracking, cancellation, and lifecycle recovery. The current handler set is a useful foundation, rather than a blanket claim of conformance to every optional LSP capability.

# Troubleshooting

| Symptom | Check |
| --- | --- |
| Highlighting works, but no type information appears. | Confirm the language client started, the file is recognized as `cinch`, and the selected executable launches the server. |
| The server waits in a terminal. | Connect an LSP client; the process expects framed protocol messages. |
| The client cannot find an executable. | Use an absolute compiler or launcher path; check the environment inherited by the editor. |
| A command override fails despite working in a shell. | The current VS Code setting takes one executable path, not `cinch lsp` as a shell string. |
| Protocol parsing fails on startup. | Remove shell banners or other text from standard output. |
| Edits produce nonsensical parse errors. | Confirm the client honours full-text synchronization. |
| Imports resolve differently from the CLI. | Check the workspace root, module names, source availability, compiler revision, and unsaved buffers. |
| A menu action reports an unsupported request. | Compare the request with the running server's capabilities; several intended features are coming soon. |
| A diagnostic persists after an external file change. | Reopen/restart analysis and check saved source with `cinch check`; complete file watching is coming soon. |

A useful bug report includes the compiler revision, editor/client, root selection, minimal source files, whether buffers were saved, the request or action, and its response/diagnostic. Reproduce the saved program with `cinch check` where possible. Protocol traces may contain full source text; include only what is needed to reproduce the issue.

# Current implementation

| Surface | Present boundary |
| --- | --- |
| `cinch lsp` | Implemented stdio server in the Rust executable. |
| Shared analysis | Implemented diagnostics, hover, definition, completion, semantic tokens, and incremental analysis reports. |
| Editor integrations | Repository syntax integrations and a VS Code/Cursor language-client scaffold; packaged setup is **Coming soon**. |
| Complete workspace experience | **Coming soon**, including robust disk/buffer lifecycle and stale-result handling. |
| Rich assistance and refactoring | **Coming soon**, as identified in the feature sections above. |

Implementation: `compiler/src/lsp.rs`, `compiler/src/analysis.rs`, `compiler/src/main.rs`, `tooling/lsp-driver/`, and `tooling/editors/`; behaviour suite: `compiler/tests/lsp.rs`. See the [REPL guide](26-repl.md) for interactive programming through the same analysis layer, and [the support matrix](16-support-and-roadmap.md) for the wider compiler boundary.
