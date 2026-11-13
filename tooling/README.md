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
- `tree-sitter-cinch-core/`: the grammar for the Core text format
  (`design/core-output.md` section 10), its corpus, and
  `samples/forms.core`, a hand written whole file that
  `check-real-files-parse.sh` parses. Same `test/corpus` symlink and the
  same rule that generated parser sources are not committed.
- `editors/vim`, `editors/emacs`, `editors/sublime`, `editors/zed`,
  `editors/vscode`: the integrations for the surface syntax and for
  Cinch Core (`*.core`, see "Core files" below).
- `lsp-driver/`: a two-file cargo binary that runs the compiler's
  language server over stdio (see below).
- `check-grammar-drift.sh`: the drift check (regenerates the parser and
  runs the corpus assertions). `--core` does the same for the Core
  grammar.
- `check-real-files-parse.sh`: parses every real `.cinch` file under
  `compiler/prelude/` and `examples/source/` and, for Core, every
  `*.core` under `compiler/tests/core/` and
  `tree-sitter-cinch-core/samples/`, requiring zero ERROR nodes (the
  whole-file property net). `--surface` and `--core` restrict it to one
  grammar.
- `run-bootstrap-ladder.sh`: assembles a project out of the toolkit sources,
  every `bootstrap/*.cinch` module and (optionally) one rung program, and
  either type-checks the ladder's modules or runs one rung through the real
  CLI. The compiler does not embed the P30/P31 toolkit, so this assembly is
  how a rung runs outside the test harness.
  With `--all-programs` it asks the design document's question of every
  rung: is the artifact runnable? It builds each `bootstrap/programs/Run.*.cinch`
  and reports its exit code, failing only when a program did not build.
- `shard_ir.py`: the deterministic IR splitter for the stage link
  (below), which also runs as a `CINCH_CLANG` wrapper so the pinned seed's
  own link of Stage1 gets the same treatment.

## Bootstrap stage link

Each stage is linked by clang from one `.ll` module. Above one job,
`tooling/bootstrap.py` splits that module first, compiles every shard in
parallel and links the objects in shard order:

```sh
CINCH_HEAP_LIMIT=2G python3 tooling/bootstrap.py build --stage 3 -j 8
```

`-j`, or `CINCH_BOOTSTRAP_JOBS`, is one job budget for the whole stage link;
the default is half the machine's cores. `-j 1` keeps the original single
translation unit and records no shard objects. The split changes the
binaries (cross-shard calls cannot inline), so the job count is part of the
stage's recorded inputs: changing it rebuilds the stage rather than
reusing a cached compiler.

`tooling/shard_ir.py` groups functions by the module prefix in their symbol
(`Bootstrap.Infer.foo` joins `Bootstrap.Infer`; the generated `.app`,
`.clo`, `.blk`, `.dec<N>` and `#tail` suffixes are stripped first, so a
unit and its companions shard together), balances the groups into K shards
by IR line count (sorted names, greedy into the least loaded shard, ties to
the lower index; a group larger than its fair share is cut at function
boundaries), and makes every cross-shard reference resolvable: a function
defined elsewhere gets a `declare` with the same signature and attribute
group, an externally linked global gets an `external` declaration, and an
`internal` definition is copied into each shard that uses it, because local
symbols do not resolve across objects. Module-level content (globals,
aliases, intrinsics, and the numbered metadata that `!range` attachments
need) stays with shard 0 or is copied where an attachment needs it.

The split is a pure function of the IR text and K. Shard contents, object
files and the link order therefore do not depend on scheduling, and Stage2
and Stage3 still link to byte-identical compilers. Stage1's link happens
inside the pinned Rust seed, which honours only `CINCH_CLANG`; bootstrap
points that at `shard_ir.py`, which shards one large `.ll` link and execs
`CINCH_CLANG_REAL` (or `clang` on `PATH`) for everything else.

## Profile-guided stage compilers

A stage compiler is the chain's own hot loop: Stage1 compiles Stage2 and
Stage2 compiles Stage3, so a faster compiler shortens the chain and every
check the chain runs. Each stage is linked with a profile by default;
`--no-pgo`, or `CINCH_BOOTSTRAP_PGO=0`, restores the plain `-O2` link:

```sh
CINCH_HEAP_LIMIT=2G python3 tooling/bootstrap.py build --stage 3            # --pgo is the default
CINCH_HEAP_LIMIT=2G python3 tooling/bootstrap.py build --stage 3 --no-pgo
```

The pipeline is the one measured in `design/compilation-speed.md` section 8:
link the emitted IR with `-fprofile-generate`, run the resulting compiler
over a training workload, merge the counters with `llvm-profdata merge`
(`xcrun -f llvm-profdata` on macOS, `llvm-profdata` elsewhere), and relink
with `-fprofile-use`. The installed `cinch` is always the profile-use link.
The instrumented binary and its counters live in the stage's `pgo/` work
directory and are removed before the stage is recorded, so nothing
instrumented becomes an artifact.

Where the flags attach follows where the link happens:

- Stage1 is linked inside the pinned Rust seed, which builds its own clang
  command line. `shard_ir.py` adds flags the seed cannot be told about
  (`CINCH_CLANG_APPEND`) and records the seed's own link arguments
  (`CINCH_CLANG_RECORD`); the profile-use relink replays exactly those
  arguments, so only the profile flag and the output path differ from the
  command the seed ran.
- Stages 2 and 3 are linked by `bootstrap.py`, so the flags go straight to
  `clang`, or to every shard above one job.

The training workload is `Text.Lex`, `Text.Scan`, `Text.Parse`, `Text.Json`
and `Text.Builder` plus a `Main` that imports all five so the checker walks
the whole toolkit (about 4,000 lines): the same workload section 8 measured,
and a check is small enough to retrain on every source revision. Stage1's
manifest for that check is built from the interfaces the seed emitted; later
stages use their own `base-interfaces.json`.

A profile is keyed, and rechecked on every use, by what it depends on: the
compiler source digest, the workload digest, the `llvm-profdata` version, and
the digest of the IR it was trained on. It lives at
`build/bootstrap/pgo/<source sha256>-<family>.profdata` with a `.json` record
beside it. The family is `seed` for Stage1's module, which the Rust seed
emits, and `cinch` for Stage2 and Stage3, whose IR the fixed point keeps byte
identical, so Stage3 reuses what Stage2's instrumented build trained. A
profile that no longer matches those digests is retrained rather than reused,
the cache keeps one live profile per family, and the profile's digest is part
of the stage's recorded inputs, so a stage built against one profile is
rebuilt if the profile changes. `status` prints the profile each stage was
linked with.

Profile data moves basic-block weights and inline costs and nothing else, so
the emitted IR, every `.cinchif` and every build manifest are unchanged and
the Stage2/Stage3 fixed point still holds on all of them. The executables do
change, because inlining decisions differ; that is why both stages must link
the same IR with the same profile, which they do.

One property of profile-use compiles matters for that last point: clang's
output for a module depends on the path it was handed. The same bytes
compiled from two directories gave objects that differ by 38% (a profile's
records are matched per module, so a module compiled from a different path
matches differently). Two consequences are wired in. Every stage links its
module from one content-addressed path, `build/bootstrap/shards/<ir
digest>/`, so Stage2 and Stage3 compile identical input at identical paths
and a rebuilt stage reproduces its binary; and `shard_ir.py` writes a
wrapper's shards to a directory named after the IR rather than a temporary
one, so the instrumented link and the profile-use relink of Stage1 compile
the same paths. A profile trained under a different path scheme is refused
by the record's schema version rather than reused.

Measured on this machine at `-j 9`, the two chains' Stage3 compilers against
each other on the toolkit check, five interleaved runs each: 22.53 s median
wall and 20.07 s CPU without a profile, 20.62 s and 18.06 s with one (8.5%
wall, 10.0% CPU; the machine was fully loaded throughout, and an earlier,
quieter measurement of the same pair gave 15.47 s against 13.96 s). The
profile is 5.5 MB for the Cinch family and 9.9 MB for the seed family. The
cost is one instrumented link plus one training run per source revision,
about 26 s and 20 s at this shard count. `__text` is unchanged within 2%
(12.08 MB to 11.91 MB on the Stage2 module); the executable file grows from
15.98 MB to 20.57 MB because cold call sites stay out of line and our
mangled names are long, so the symbol table in `__LINKEDIT` is 33k entries
larger.
\\\\\\\        to: qrutmztx 0052933a "Add a build profile harness for the ladder" (rebased revision)
- `profile_build.py`: measures where the ladder spends its time and writes
  `bootstrap/speed-baseline.json` (see below).

## Build profile

`design/compilation-speed.md` sections 11 and 12 say what to measure. This
script produces one JSON object so later runs diff against the first.

```sh
tooling/profile_build.py all                              # chain, check, targets
tooling/profile_build.py chain --stage 3                  # stages 0..3 from clean
tooling/profile_build.py check examples/source/01-...cinch --repeat 7
tooling/profile_build.py targets bootstrap_check lib_json --jobs 4
```

- `chain` rebuilds `build/bootstrap` from nothing, one `bootstrap.py build
  --stage N` invocation per stage. Per stage it records `wall_s`, `seed_s`
  (the `Finished in` cargo line(s) that stage's invocation wrote),
  `frontend_s` (the residual: the builder's own run), `clang_s` (measured by
  a `CINCH_CLANG` wrapper, so it is the clang the ladder actually ran), and
  `clang_trace` from re-linking that stage's IR with `-ftime-trace`
  (`ExecuteCompiler`, `Frontend`, `Optimizer`, `CodeGenPasses`, and the five
  largest `Total *` entries). Stage 1's IR is re-emitted with the seed because
  the seed deletes its sidecar.
- `check` times `cinch check` and `cinch emit-ir` on a copy of one source in
  `build/bootstrap-profile/check` (median and p95 over `--repeat` runs) and then runs
  `sample <pid> 10` on the slower of the two, reporting the top 15 symbols by
  self and by inclusive samples.
- `targets` runs `run_stage_targets.py` at stage 1 and stage 3 and records
  each target's exit code, passed and failed test counts, and wall time (log
  birth to close, so contention between parallel targets is included).
- `sweep` times `emit-ir` and the sharded `clang -O2` backend at each shard
  count over the four workload shapes of `design/compilation-speed.md`
  section 12: `many-small` (40 unconnected modules), `wide-graph` (a root
  over 40 leaves over 4 shared bases), `deep-chain` (40 modules each
  importing the last) and `huge-module` (one module of 1,200 definitions).
  The shard count is a stage input, so both halves are recorded per
  (workload, K): `frontend_s` is the `emit-ir` run and `backend_s` is
  `tooling/shard_ir.py`'s split compile of that same module, so the K>1
  numbers are one program split differently, not a different program.

```sh
tooling/profile_build.py sweep                       # K = 1, 2, 4, 8
tooling/profile_build.py sweep --jobs 1 3 9 --workloads huge-module
```

Reading the sample profile: `parse_sample` reports self and inclusive
sample counts, and it resolves the frames `sample` leaves as `???` by
looking the printed load address up in `nm -n` of the sampled binary. A
frame the caller of a hot leaf does not appear under means the frame was
dropped, not that it was free: the group markers `sample` draws in the
gutter (`+`, `!`, `:`, `|`) are one per two columns of indent and are part
of the depth, and a parser that matches only one marker silently flattens
the graph.

Only the standard library, `clang` and `sample` are needed. Everything the
run generates lands in `build/bootstrap-profile` or `build/bootstrap`, both ignored by
git, so measuring never edits a source file. The one exception is the
baseline itself, `bootstrap/speed-baseline.json`.

## Drift check (CI)

```sh
tooling/check-grammar-drift.sh            # surface grammar
tooling/check-grammar-drift.sh --core     # Core grammar
```

Each mode regenerates that grammar's parser from its `grammar.js` and
runs `tree-sitter test` against its `corpus/`; any divergence from the
asserted trees exits nonzero. Requirements: the `tree-sitter` CLI (0.26 tested;
the script falls back to `npx --yes tree-sitter-cli`) and a C compiler.
There is no CI config in this repository today; the script is what CI
would call. The generated parser (`src/parser.c`,
`src/grammar.json`, `src/node-types.json`) is deliberately not
committed: the drift check always regenerates from `grammar.js`, so the
assertions cannot pass against a stale parser.

## Core grammar

`tree-sitter-cinch-core/` parses the Core text format of
`design/core-output.md` sections 4 and 10: the fixed header (`core 1`,
`module`, `target`, `source`), units with their annotations and parameter
lists, types, and one named node per expression form in section 10's
table. It has no external scanner. The format is line oriented and every
expression is a parenthesised form, so a child list is just the children
between a form's parentheses.

Node types: `source_file`, `header`, `module_line`, `target_line`,
`source_line`, `target_triple`, `unit`, `unit_name`, `name`,
`annotation`, `annotation_name`, `annotation_args`, `ownership_counts`,
`params`, `param`, `class`, `span`, `type`, `rigid_var`, `meta_var`,
`refinement_type`, `row_type`, `comment`, and the 32 expression nodes
`int_lit`, `nat_lit`, `bytes_lit`, `string_lit`, `sym_lit`, `bool_lit`,
`unit_lit`, `local_ref`, `global_ref`, `ctor_ref`, `prim_ref`,
`foreign_ref`, `app`, `lam`, `dict_proj`, `proj`, `inst_dict`, `let`,
`if`, `case`, `alt`, `pat`, `dup`, `drop`, `release`, `box_init`,
`box_get`, `box_clear`, `ref_set`, `quote`, `splice`, `syntax_lit`,
`demand`. The grammar name is `cinch_core`, the scope
`source.cinch-core`, the file type `core`.

The corpus has 50 cases across `corpus/`: one per expression form, the
four header lines, units with and without annotations and params, the
annotation forms of section 4, the type shapes (`forall`, application,
row, refinement, rigid and metavariables), literals with escapes, every
pattern shape including the empty case, `release` with empty moved and
rest lists, a one-line node and the same node wrapped, and comments.
`tooling/check-grammar-drift.sh --core` regenerates the parser and runs
them. `tooling/check-real-files-parse.sh --core` parses the printer's
golden `compiler/tests/core/*.core` files when that directory exists and
`samples/forms.core` always, requiring zero `ERROR` and `MISSING` nodes.
The golden files are another slice's deliverable; until they land the
script reports that it skipped them.

Decisions the design note leaves open or states only in prose:

- Comments. Section 4's sample carries a `-- ...` trailing comment and
  the surface language spells line comments the same way, so Core
  accepts `--` to end of line as an `extra` named `comment`. The design
  note does not define a comment syntax, so this is the one lexical
  addition the grammar makes.
- `end` is an anonymous token, like `unit`, `body`, `@`, `forall`,
  `refine`, `with`, `moved`, `rest`, `from`, `true`/`false`, the five
  class words standing alone, and every form keyword. None of them is a
  node: the rule that names the keyword already requires the right one,
  and a missing `end` or `body` still fails the parse as a `MISSING`
  token. Editors match them as string literals in queries.
- Annotations attach as siblings of the unit's `type`, in source order,
  with no field recording which line they came from. Section 4 allows
  each annotation on the unit line or on an indented `@` line, and both
  are the same whitespace-separated position here. `@span` yields a
  structured `span`, `@params` a `params` list, `@class` and `@<class>`
  (with optional `(n,m)` counts) a `class`. Every other annotation is an
  `annotation_name` plus `annotation_args`, the arguments captured
  verbatim between balanced parentheses, optionally followed by
  `from <name>`. The note fixes those names and their print order but
  not each argument's internal grammar, so no structure is invented
  below the name. Annotation order and uniqueness are the verifier's
  invariants, not the grammar's.
- Layout. Section 4 fixes two spaces per depth and folds a node onto one
  line when its children fit in 100 columns. A scannerless grammar
  cannot check columns, so whitespace is trivia and a wrapped node and
  its one-line form parse to the same tree; `corpus/layout.txt` asserts
  exactly that. What the grammar does check is that `body` is followed
  by a complete expression and that `end` closes the unit.
- Names. A bare canonical name is dot separated segments starting with a
  letter, `_` or `$`, with dashes only between word characters. That is
  what keeps `a->b` three tokens instead of the name `a-`, and
  `forall a b.` two names plus the binder terminator. Anything outside
  that spelling is quoted (`'weird name'`), as section 4 says, and the
  quoted form is accepted verbatim.
- Refinement predicates (`refine (x :: T) with p`) are parsed greedily
  as a term over names, numerals, operators and nested parentheses, up
  to the next delimiter the enclosing type needs. A predicate that ends
  in an application therefore needs parentheses around the application,
  the same convention the printed format already uses for types.
- A row applied to a result (`{IO r, Diverge} a`) is type application:
  `row_type` is only the braced row, and the application supplies the
  result. That removes the ambiguity between `{} a` and `{}` followed by
  an atom.
- The parenthesised pattern shape `(<Name> <pat>*)` covers both a
  variable pattern `(x)` and a nullary constructor pattern `(Nil)`;
  nothing in the text distinguishes them, and the verifier knows the
  constructor set.
- `core <digits>` and `opt-level <digits>` accept more than the single
  digit of section 10, so a later format version still parses instead of
  failing at the header. Version specific meaning stays the reader's
  job.

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

- `extension.toml`: the grammar entries (`[grammars.cinch]` and
  `[grammars.cinch-core]`) and the language server entry
  (`[language_servers.cinch-lsp]`, languages `Cinch` and `Cinch Core`,
  LSP `languageId`s `cinch` and `cinch-core`).
- `languages/cinch/config.toml`: file suffix `cinch`, `-- ` line
  comments, nested `{- ... -}` block comments, brackets that do not
  fire inside strings or raw text, `'`, `!` and `?` as word characters
  (they continue an identifier, so `empty?` and `print!` are one word
  to motions and completion), and `.` as a completion query character.
- `languages/cinch-core/config.toml`: file suffix `core`, `-- ` line
  comments, no block comments, `(`/`)` and `{`/`}` brackets, and `$`,
  `.` and `-` as word characters (they are canonical-name characters).
- `languages/cinch/{highlights,brackets,indents,outline,textobjects,overrides}.scm`
  and `languages/cinch-core/{highlights,brackets,indents,outline}.scm`:
  tree-sitter queries over each grammar's own node names.
- `src/lib.rs`: the only procedural part. It resolves the server through
  the worktree's login-shell environment (`cinch-lsp`, then `cinch lsp`)
  and reports a message naming the build command when neither exists.
- `sync-grammar.sh`: Zed does not run `tree-sitter generate`. It clones
  a git repository, checks out one revision, and compiles `src/parser.c`
  (plus `src/scanner.c`) with wasi-sdk clang. The generated parsers are
  not committed, so this script builds one small local repository per
  grammar from its `grammar.js`, generates the parser there, commits it,
  and pins that revision in `extension.toml` (`grammar-repo` for
  `cinch`, `grammar-repo-core` for `cinch-core`). All four values are
  machine local: rerun it after any grammar change, then rebuild the
  extension in Zed. It checks every grammar source before generating
  anything, so with `tooling/tree-sitter-cinch-core/` absent it exits 1
  naming that directory and leaves `extension.toml` unchanged.

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

## Core files

`.core` files are Cinch Core, the textual Core format of
`design/core-output.md`: the text format is section 4, the normative
grammar is section 10, and the editor support implemented here is section
8. Every integration uses:

- extension `.core`, language id `cinch-core`, display name "Cinch Core"
- line comments `--`, no block comments
- the first line `core 1` as the file signature, so a file without the
  extension is still recognised by content
- two-space indentation, one s-expression node per line

`comment` is a named node in the Core grammar, unlike the surface grammar
where comments are scanner trivia, so the Core highlight queries colour
comments directly and `cinch-core` needs no `semantic_tokens` setting in
Zed.

The highlighter categories are the format's own (section 8.3): unit names,
dictionary bindings (`$d.*`, `$static`), ownership classes (`owned`,
`borrowed`, `lent`, `uncounted`, `immortal`), annotation names (`@span`,
`@class`, `@captures`, `@specialised`, `@rewrite`, `@prim`, `@resolved`,
`@tailrec`, `@join`, `@stage`), numerals, string and byte literals, the
header (`core`, `module`, `target`, `source`, `opt-level`), and `end`.

| Editor | Files | Check |
| --- | --- | --- |
| vim | `editors/vim/ftdetect/cinch-core.vim`, `editors/vim/syntax/cinch-core.vim`, `editors/vim/queries/cinch-core/highlights.scm` (the nvim-treesitter path) | `editors/vim/test/run.sh` |
| emacs | `editors/emacs/cinch-core-mode.el` | `editors/emacs/test/run.sh` |
| sublime | `editors/sublime/CinchCore.sublime-syntax`, `editors/sublime/Comments.tmPreferences` | `python3 editors/sublime/test/check.py` |
| vscode | `editors/vscode/package.json`, `editors/vscode/language-configuration-core.json`, `editors/vscode/syntaxes/cinch-core.tmLanguage.json`, `editors/vscode/extension.js` | `node editors/vscode/test/check.mjs` |
| zed | `editors/zed/languages/cinch-core/{config.toml,highlights.scm,outline.scm,brackets.scm,indents.scm}`, `editors/zed/extension.toml`, `editors/zed/sync-grammar.sh` | `python3.11 editors/zed/test/check.py` |

Every `editors/<editor>/test/fixture.core` is the same representative
file: a header, a unit with `params` and a nested `case`, a unit with
`@params`, a static dictionary unit, every annotation, and every literal
form.

What each check proves, and what it cannot:

- vim: `editors/vim/test/run.sh` runs `vim -Nu NONE -es -S test/check.vim`
  and asserts the filetype (by extension and by the `core 1` signature)
  and the syntax group at a known token for every category, 33 checks
  (`:filetype detect` only re-runs the `filetypedetect` group, so the
  signature check fires the detection autocmds with `doautocmd BufRead`).
  It takes the binary as its argument, so `run.sh /usr/bin/vim` and
  `run.sh $(command -v nvim)` both pass.
- emacs: `editors/emacs/test/run.sh` runs `emacs -Q --batch` with
  `test/check.el` and asserts the `auto-mode-alist` entry, `prog-mode`
  derivation, non-empty `font-lock-keywords`, the `imenu` unit index, `--`
  as a comment, and the face at a known token per category, 24 checks.
  Emacs is not installed on this machine; the check ran through
  `nix shell nixpkgs#emacs-nox --command editors/emacs/test/run.sh <emacs>`.
- sublime: Sublime Text cannot be driven here, so `check.py` validates the
  definition as data: the YAML parses, every `include` resolves, every
  section 8.3 category has a scope, unit names carry
  `entity.name.function` (the Goto Symbol list, which is this editor's
  imenu), and all 33 form names appear. Rendering is unverified. A
  `.sublime-syntax` file has no line-comment key, so the `--` marker is
  declared the TextMate way in `Comments.tmPreferences` (`TM_COMMENT_START`
  scoped to `source.cinch-core`), which is what Toggle Comment reads; the
  check parses that plist and asserts both values.
- vscode: `check.mjs` validates every JSON file, that both languages and
  both grammars are contributed with distinct configurations, that every
  `include` resolves and every regex compiles, that the section 8.3 scopes
  are present, and that `extension.js` passes `node --check` and selects
  both language ids. The extension is not activated in a running VS Code.
- zed: `check.py` parses both TOML files, requires every language
  referenced by `extension.toml` to exist on disk, requires both grammar
  entries, and validates every query file (balanced, and every node name
  and anonymous token present in the grammar contract below).

The query files are also compiled against the real parser. After
`editors/zed/sync-grammar.sh` has built `grammar-repo-core`:

```sh
cd tooling/editors/zed/grammar-repo-core
for q in ../languages/cinch-core/{highlights,outline,brackets,indents}.scm \
         ../../vim/queries/cinch-core/highlights.scm; do
  tree-sitter query "$q" ../../../tree-sitter-cinch-core/samples/forms.core
done
```

All five compile with no invalid node type, field or predicate, and match on
`samples/forms.core` (316 captures for each highlights query, 10 for the
outline, 148 for the brackets, 51 for the indents; the sample parses with
zero ERROR nodes). Two findings from that run are already applied: the
bracket pairs are one top-level pattern each, because a single grouped
pattern asks for a `(...)` pair and a `{...}` pair as consecutive siblings of
one parent and matches nothing, and `(unit "end" @outdent) @indent` is
confirmed to give one indent and one outdent per unit. Zed's own query
compiler is the remaining check on install.

`editors/zed/test/check.py` needs `tomllib`, so Python 3.11 or newer. The
macOS system `python3` is 3.9, so on this machine it runs as

```sh
nix shell nixpkgs#python311 --command python3 tooling/editors/zed/test/check.py
```

The Core grammar contract these queries are written against, as recorded by
the grammar slice:

- `source_file` holds `header`, `module_line` (`name`), `target_line`
  (`target_triple`) and `source_line` as siblings, then the `unit` nodes.
- `unit` holds `unit_name` (`name`), `type`, `annotation` nodes, the
  anonymous `body` token, one expression, and the anonymous `end` token.
- `annotation` holds `annotation_name` (`name`), or `annotation_args`,
  `ownership_counts`, `span`, or `class`, depending on the annotation.
- `params` holds `param` nodes of `name`, `type`, `class`; `class` is a
  leaf and the five class words are anonymous tokens.
- `type` holds `name`, `rigid_var`, `meta_var`, `refinement_type` or
  `row_type`.
- `local_ref`, `global_ref`, `ctor_ref` and `prim_ref` each wrap a `name`;
  `foreign_ref` is a leaf.
- Every form in section 8.2 is a node, in snake_case (`int_lit`,
  `nat_lit`, `bytes_lit`, `string_lit`, `sym_lit`, `bool_lit`, `unit_lit`,
  `local_ref`, `global_ref`, `ctor_ref`, `prim_ref`, `foreign_ref`, `app`,
  `lam`, `dict_proj`, `proj`, `inst_dict`, `let`, `if`, `case`, `alt`,
  `pat`, `dup`, `drop`, `release`, `box_init`, `box_get`, `box_clear`,
  `ref_set`, `quote`, `splice`, `syntax_lit`, `demand`).
- `comment` is a named node; `--` is not a token of its own.
- Anonymous tokens: `(`, `)`, `{`, `}`, `,`, `::`, `->`, `:`, `@`, `_`,
  `.`, `-`, the form words, `unit`, `body`, `end`, `forall`, `refine`,
  `with`, `moved`, `rest`, `from`, `true`, `false`, and the class words.
  The grammar has no `=` and no `|` token.

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

- The Core grammar's corpus is green (50 cases,
  `tooling/check-grammar-drift.sh --core`), and
  `tooling/check-real-files-parse.sh --core` parses
  `tree-sitter-cinch-core/samples/forms.core` with zero ERROR or MISSING
  nodes. That same check reports that it skipped
  `compiler/tests/core/` because the printer slice has not created it
  yet; when the golden files land they are parsed with no change to the
  script.
- The corpus is green end to end: `tooling/check-grammar-drift.sh`
  regenerates the parser and passes all 88 assertions in seconds. The
  one corpus case that ever failed without a scanner fault ("minus
  table: a line comment splits x--3") had lost its expected tree and was
  restored from the asserted behavior (`y--3` lexes as identifier `y`
  plus line comment `--3`).
- `tooling/check-real-files-parse.sh --surface` is not green today: it
  parses 113 files under `compiler/prelude/` and `examples/source/` and
  26 of them report ERROR nodes. Those modules use forms the grammar does not have
  yet, refinement types first among them (`type Ascii = refine (s ::
  String) with Ascii s` in `compiler/prelude/Data/Ascii.cinch`). The
  drift check stays green, so this is missing grammar coverage rather
  than a scanner fault, and it is what an editor shows as invalid
  syntax in those files.
- `npx tree-sitter-cli` is the documented fallback when the CLI is not
  on PATH; it was not exercised here (the nix-profile `tree-sitter`
  0.26.11 is what ran).
