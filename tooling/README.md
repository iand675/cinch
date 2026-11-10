# Cinch editor tooling

Tree-sitter grammar and editor integrations for the Cinch surface
syntax. The normative authorities are
`specifications/surface-syntax-and-grammar.md` (draft 0.2 EBNF),
`specifications/surface-grammar-local-amendments.md` (sections 1-13),
and the decision log in `compiler/CONFORMANCE.md` (D102-D129, D143-D155,
D198-D205, D221-D227 where the specs understate).

## Layout

- `tree-sitter-cinch/`: the grammar (`grammar.js`) and the external
  scanner (`src/scanner.c`), a port of `compiler/src/lexer.rs` and
  `compiler/src/layout.rs`.
- `tree-sitter-cinch/corpus/`: parse-tree assertions for the tricky
  cases (the minus table, dash names, tick disambiguation, operator
  runs like `<$>`/`>>=`/`<>`, attributes, prim/contract forms, layout).
  `test/corpus` is a symlink so `tree-sitter test` finds them; the
  corpus lives in `corpus/` as named in the deliverables.
- `editors/vim`, `editors/emacs`, `editors/sublime`, `editors/zed`,
  `editors/vscode`: the integrations.
- `lsp-driver/`: a two-file cargo binary that runs the compiler's
  language server over stdio (see below).
- `check-grammar-drift.sh`: the drift check (regenerates the parser and
  runs the corpus assertions).
- `check-real-files-parse.sh`: parses every real `.cinch` file under
  `compiler/prelude/` and `examples/source/` and requires zero ERROR
  nodes (the whole-file property net for the scanner).
- `run-bootstrap-ladder.sh`: assembles a project out of the toolkit sources,
  every `bootstrap/*.cinch` module and (optionally) one rung program, and
  either type-checks the ladder's modules or runs one rung through the real
  CLI. The compiler does not embed the P30/P31 toolkit, so this assembly is
  how a rung runs outside the test harness.
  With `--all-programs` it asks the design document's question of every
  rung: is the artifact runnable? It builds each `bootstrap/programs/Run.*.cinch`
  and reports its exit code, failing only when a program did not build.

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

`cinch lsp` dispatches to `cinch::lsp::LspServer::run` over stdio
(Content-Length framed JSON-RPC), exactly the invocation exercised by
`compiler/tests/lsp.rs`, so editors can exec the compiler binary
directly. `lsp-driver/` is the same invocation in a standalone two-file
driver:

```sh
cargo run --manifest-path tooling/lsp-driver/Cargo.toml --release
```

The VS Code/Cursor extension's launcher (`editors/vscode/bin/cinch-lsp`)
execs `cinch lsp` when the binary is on PATH and otherwise builds and
runs the driver; the `cinch.lsp.command` setting overrides the command
either way.

## Zed extension

`editors/zed` is a dev extension for Zed 1.27 (manifest
`schema_version = 1`):

- `extension.toml`: the grammar entry (`[grammars.cinch]`) and the
  language server entry (`[language_servers.cinch-lsp]`, language
  `Cinch`, LSP `languageId` `cinch`).
- `languages/cinch/config.toml`: file suffix `cinch`, `-- ` line
  comments, nested `{- ... -}` block comments, brackets that do not
  fire inside strings or raw text, `'` as a word character (it is an
  identifier character), and `.` as a completion query character.
- `languages/cinch/{highlights,brackets,indents,outline,textobjects,overrides}.scm`:
  tree-sitter queries over the grammar's own node names.
- `src/lib.rs`: the only procedural part. It resolves the server through
  the worktree's login-shell environment (`cinch-lsp`, then `cinch lsp`)
  and reports a message naming the build command when neither exists.
- `sync-grammar.sh`: Zed does not run `tree-sitter generate`. It clones
  a git repository, checks out one revision, and compiles `src/parser.c`
  (plus `src/scanner.c`) with wasi-sdk clang. The generated parser is not
  committed, so this script builds a small local repository from
  `grammar.js`, generates the parser there, commits it, and pins that
  revision in `extension.toml`. Both values are machine local: rerun it
  after any grammar change, then rebuild the extension in Zed.

Install (once):

```sh
cargo build --release --manifest-path compiler/Cargo.toml   # the `cinch` binary
tooling/editors/zed/sync-grammar.sh
```

then in Zed, `zed: extensions` -> `Install Dev Extension` ->
`tooling/editors/zed`. Zed compiles the crate to `wasm32-wasip2`, clones
the grammar repository, and compiles the grammar with its own wasi-sdk.
`cinch` has to be reachable from the login shell Zed sees (a
`~/.local/bin` symlink or `cargo install`); otherwise set
`lsp.cinch-lsp.binary.path` and `arguments: ["lsp"]` in settings.

Comments need one more setting. The grammar keeps comments as scanner
trivia (the minus and symbol rules need to see them), so they never
become tree-sitter nodes and no highlight query can match them. The
language server reports them as `comment` semantic tokens instead, which
Zed only asks for when the language has semantic tokens enabled:

```json
{ "languages": { "Cinch": { "semantic_tokens": "combined" } } }
```

`combined` keeps the tree-sitter colours and adds the server's tokens on
top, so comments (and resolution-aware `type`/`class`/`function` tokens)
take the theme's colours.

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
- In `do` blocks the `let` statement is `let` plus a whole
  `local_block`, per the surface EBNF `do_statement`, so one `let`
  group may bind several declarations. The bootstrap compiler parses a
  single local declaration per do `let` statement; the EBNF is
  normative here, so the grammar follows it and closing the gap is the
  compiler's work.
- Sym literal names follow identifier spelling (apostrophes included),
  so the tick rule "tick, one character, tick is a Char and wins" is
  what disambiguates `'a'` from the symbol `'a'`.

## Known gaps (honest)

- The corpus is green end to end: `tooling/check-grammar-drift.sh`
  regenerates the parser and passes all 88 assertions in seconds. The
  one corpus case that ever failed without a scanner fault ("minus
  table: a line comment splits x--3") had lost its expected tree and was
  restored from the asserted behavior (`y--3` lexes as identifier `y`
  plus line comment `--3`).
- `tooling/check-real-files-parse.sh` is not green today: it parses 109
  files under `compiler/prelude/` and `examples/source/` and 25 of them
  report ERROR nodes. Those modules use forms the grammar does not have
  yet, refinement types first among them (`type Ascii = refine (s ::
  String) with Ascii s` in `compiler/prelude/Data/Ascii.cinch`). The
  drift check stays green, so this is missing grammar coverage rather
  than a scanner fault, and it is what an editor shows as invalid
  syntax in those files.
- `npx tree-sitter-cli` is the documented fallback when the CLI is not
  on PATH; it was not exercised here (the nix-profile `tree-sitter`
  0.26.11 is what ran).
