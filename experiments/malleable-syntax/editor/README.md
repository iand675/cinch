# Editor experiment

Run `python3 experiments/malleable-syntax/editor_generate.py` from the worktree
root to generate the grammar packages. The verifier generates and compiles them
and checks both parsing and complete-file highlighting with real injections.

There are three layers:

1. `cinch_project` inherits the existing Cinch grammar and adds syntax imports.
2. Each syntax library exports a generated `cinch_<library>` grammar.
3. `cinch_expr` inherits the actual Cinch grammar with an expression entry point.

An editor using Tree-sitter should register the generated grammar packages and
their `queries/highlights.scm` and `queries/injections.scm`, then use
`source.cinch_project` for these experimental sources. The generator derives
import aliases from the example project. Editor-specific installation and
automatic per-workspace registration remain part of the ongoing experiment.

The LSP command is `python3 <absolute-path-to-experiment>/poc.py lsp`. It speaks
standard Content-Length-framed JSON-RPC on stdin/stdout. Diagnostics and requests
use original source coordinates; temporary expanded files are internal to the
adapter. Use full synchronization as advertised by the server.

No global editor configuration has been modified.
