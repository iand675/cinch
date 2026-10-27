# SafeLang editor tooling

Tree-sitter grammar and editor integrations for the SafeLang surface
syntax. The normative authorities are
`specifications/surface-syntax-and-grammar.md` (draft 0.2 EBNF),
`specifications/surface-grammar-local-amendments.md` (sections 1-13),
and the decision log in `compiler/CONFORMANCE.md` (D102-D129, D143-D155,
D198-D205, D221-D227 where the specs understate).

## Layout

- `tree-sitter-safelang/`: the grammar (`grammar.js`) and the external
  scanner (`src/scanner.c`), a port of `compiler/src/lexer.rs` and
  `compiler/src/layout.rs`.
- `tree-sitter-safelang/corpus/`: parse-tree assertions for the tricky
  cases (the minus table, dash names, tick disambiguation, operator
  runs like `<$>`/`>>=`/`<>`, attributes, prim/contract forms, layout).
  `test/corpus` is a symlink so `tree-sitter test` finds them; the
  corpus lives in `corpus/` as named in the deliverables.
- `editors/vim`, `editors/emacs`, `editors/sublime`, `editors/zed`,
  `editors/vscode`: the integrations.
- `lsp-driver/`: a two-file cargo binary that runs the compiler's
  language server over stdio (see below).
- `check-grammar-drift.sh`: the drift check.

## Drift check (CI)

```sh
tooling/check-grammar-drift.sh
```

It regenerates the parser from `grammar.js` and runs
`tree-sitter test` against `corpus/`; any divergence from the asserted
trees exits nonzero. Requirements: the `tree-sitter` CLI (0.26 tested;
the script falls back to `npx --yes tree-sitter-cli`) and a C compiler.
There is no CI config in this repository today; the script is what CI
would call. The generated parser (`src/parser.c`,
`src/grammar.json`, `src/node-types.json`) is deliberately not
committed: the drift check always regenerates from `grammar.js`, so the
assertions cannot pass against a stale parser.

## Language server wiring

The compiler binary has no `lsp` subcommand today
(`compiler/src/main.rs` offers `check`/`emit-ir`/`build`/`run`). The
server is `safelang::lsp::LspServer::run` over stdio (Content-Length
framed JSON-RPC), exactly the invocation exercised by
`compiler/tests/lsp.rs`. `lsp-driver/` is that invocation verbatim:

```sh
cargo run --manifest-path tooling/lsp-driver/Cargo.toml --release
```

The VS Code/Cursor extension's launcher (`editors/vscode/bin/safelang-lsp`)
builds and execs the driver, and prefers `safelang lsp` automatically
once that subcommand exists; the `safelang.lsp.command` setting
overrides the command either way.

## Scanner and grammar decisions (unnumbered)

- All terminals come from `src/scanner.c`, a straight port of
  `compiler/src/lexer.rs` plus the `layout.rs` frame machine. Regex
  tokens cannot express the context-sensitive rules: dash glue vs the
  minus whitespace contract (D102/D128), negative literal signs
  (D129), the tick Char-vs-Symbol rule (D123), `$` continuing but never
  starting operator runs, comment priority over symbol runs, and
  layout.
- Layout: `where`/`of`/`do`/`let`/`letrec` open implicit blocks;
  equal indentation separates items, lower closes them; `in` closes the
  nearest implicit `let` block without crossing an explicit delimiter;
  the `in` right after a `with (...)` clause belongs to the clause
  (the documented paren ambiguity in amendments section 7 behaves the
  way `layout.rs` does: the paren group reads as a supply clause and the
  parser then rejects it with the supply-binding diagnostic shape);
  explicit `(`/`[`/`{` shield layout.
- Infix chains use the documented prelude/base-library fixity table as
  precedence layers (levels 9 through 1 plus `$`), including `<>`,
  `<*>`, `<$>`, `<|>`, `>>=`. Equal-precedence runs with clashing
  associativity (level 6 `&` vs `<>`, level 3 `<|>` vs the comparisons,
  level 1 `>>=` vs `||`) may only appear parenthesized in valid
  programs, so the split between their sub-levels is observable only on
  programs the compiler's fixity pass rejects. Operators outside the
  table parse in one flat layer directly above `$`.
- `unsafe` (D221-D227) and `macro name attributes (...)` are decided
  with a one-token lookahead at the scanner level (the language keeps
  `unsafe` usable as an identifier).
- `prim` body fragments, macro token trees, and attribute arguments are
  captured verbatim (balanced, string-aware), as the lexer does.
- The scanner mirrors `layout.rs`'s "must indent farther" rejection by
  refusing to open the block, so the parse reports the divergence.
- Tree-sitter cannot express two of the lexer's hard errors: a tab in
  indentation is treated as whitespace (the compiler rejects it), and
  a `#[` whose name does not immediately follow on the same line is
  accepted (the compiler rejects it). Both are noted here rather than
  faked.
- Sym literal names follow identifier spelling (apostrophes included),
  so the tick rule "tick, one character, tick is a Char and wins" is
  what disambiguates `'a'` from the symbol `'a'`.

## Known gaps (honest)

- The corpus was last fully green (88/88 asserting trees generated, 87
  passing) during development; a later scanner change around the
  layout lookahead needs one more `tooling/check-grammar-drift.sh` run
  before this is called green. The grammar itself parsed all 59 real
  `.lang` files in the repository (the whole prelude plus every
  example) with zero ERROR nodes at that point.
- `npx tree-sitter-cli` is the documented fallback when the CLI is not
  on PATH; it was not exercised here (the nix-profile `tree-sitter`
  0.26.11 is what ran).
