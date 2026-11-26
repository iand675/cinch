# Cinch build, verification, and editor tooling (`tooling/`)

`tooling/` houses the bootstrap orchestration scripts, profiling/benchmark drivers, Nix binary-cache derivations, Tree-sitter grammars for Cinch and Cinch Core, the standalone LSP launcher, and editor integrations.

## Directory overview

| Path | Purpose |
| --- | --- |
| `bootstrap.py`, `bootstrap.sh` | Four-stage self-hosting ladder driver (`build`, `smoke`, `check`, `status`, `hydrate`) |
| `shard_ir.py` | Deterministic LLVM IR splitter for parallel Clang compilation and linking |
| `module_build.py` | Fast incremental, per-module parallel builder for `compiler/*.cinch` |
| `verify_bootstrap.py` | Frozen-snapshot end-to-end verification across compiler, runtime, tooling, and Stage 3 |
| `run-bootstrap-ladder.sh` | Type-checks `compiler/*.cinch` modules or runs individual `compiler/programs/Run.*.cinch` rungs |
| `run_stage_targets.py` | Routes portable Rust integration suites through a staged Cinch compiler |
| `check-compiler-layout.py` | Guards `compiler/` top-level layout, evidence categories, and stale path references |
| `profile_build.py`, `bench_compiler.py` | Stage build profiler and compiler latency regression harness |
| `bench_stdlib.py`, `bench_stdlib_compare.py` | Standard library benchmark driver and paired interleaved comparison tool |
| `audit_stdlib_surface.py` | Audits `design/stdlib-surface.json` against a reference Go SDK |
| `publish_nightly.py`, `nightly-release.md` | Stage 3 nightly binary verification and GitHub prerelease publisher |
| `nix/` | Nix derivations (`cinch-seed`, `cinch-provider`, `cinch-pins`, `cinch-ladder`); see [`nix/README.md`](nix/README.md) |
| `tree-sitter-cinch/` | Tree-sitter grammar and external scanner (`src/scanner.c`) for `.cinch` surface syntax |
| `tree-sitter-cinch-core/` | Tree-sitter grammar for textual `.core` files (`design/core-output.md` section 10) |
| `editors/` | Integrations for Vim/Neovim, Emacs, Sublime Text, VS Code/Cursor, and Zed |
| `lsp-driver/` | Standalone Cargo binary launching `cinch::lsp::LspServer::run` over stdio |
| `check-grammar-drift.sh`, `check-real-files-parse.sh` | Tree-sitter regeneration, corpus assertions, and whole-tree parse checks |
| `tests/` | Python `unittest` suite for bootstrap, sharding, verification, and tooling scripts |

---

## Bootstrap builds, sharding, and PGO

### Parallel sharded linking (`shard_ir.py`)
Above `-j 1`, `bootstrap.py` splits each stage's monolithic `.ll` file into `K` balanced shards grouped by module prefix (keeping `.app`/`.clo`/`.blk`/`.dec<N>`/`#tail` companions together), compiles shards in parallel, and links them in deterministic shard order:

```sh
CINCH_HEAP_LIMIT=2G python3 tooling/bootstrap.py build --stage 3 -j 8
```

`-j` (or `CINCH_BOOTSTRAP_JOBS`, defaulting to half the machine's cores) is recorded as a stage input; `-j 1` compiles a single translation unit. Because the split is a pure function of IR text and `K`, Stage 2 and Stage 3 remain byte-identical. Stage 1 uses `shard_ir.py` as a `CINCH_CLANG` wrapper around the pinned Rust seed's link.

### Profile-guided stage compilers (PGO)
Stages 2 and 3 link with PGO by default (`--no-pgo` or `CINCH_BOOTSTRAP_PGO=0` disables; `--pgo-seed` opts Stage 1 in):

```sh
CINCH_HEAP_LIMIT=2G python3 tooling/bootstrap.py build --stage 3
CINCH_HEAP_LIMIT=2G python3 tooling/bootstrap.py build --stage 3 --no-pgo
```

Profiles train on the `Text.{Lex,Scan,Parse,Json,Builder}` toolkit workload, merge with `llvm-profdata`, and cache under `build/bootstrap/pgo/` keyed by compiler source, workload, `llvm-profdata`, and IR digests. Shards compile from content-addressed paths (`build/bootstrap/shards/<ir-digest>/`) so Clang's path-sensitive PGO matching preserves byte-identical Stage 2 and Stage 3 binaries.

### Fast development builds
For iterating on `compiler/*.cinch` without a full audited ladder (not a substitute for `bootstrap.py check`):

```sh
# Sharded dev stage build with content-addressed shard cache
python3 tooling/bootstrap.py build --stage 2 --dev

# Incremental per-module parallel build
python3 tooling/module_build.py --builder build/bootstrap/stage3/cinch --out build/modules -j 4
python3 tooling/run_stage_targets.py --compiler build/modules/cinch --out build/dev-evidence --allow-dirty ownership lattice
```

Missing specialization roots discovered during modular builds are recorded in `<out>/discovered-roots.json` and can be added to `compiler/pins/module-object-roots.json`.

---

## Profiling and latency checks

### Build profiler (`profile_build.py`)
Measures chain wall/frontend/Clang `-ftime-trace` times, single-file `check`/`emit-ir` latency with `sample` profiles, stage target suites, and shard scaling sweeps (`many-small`, `wide-graph`, `deep-chain`, `huge-module`):

```sh
tooling/profile_build.py all
tooling/profile_build.py chain --stage 3
tooling/profile_build.py check examples/source/01-bounded-index.cinch --repeat 7
tooling/profile_build.py sweep --jobs 1 2 4 8
```

### Compiler latency harness (`bench_compiler.py`)
Compares two compilers across `hello` (cold/warm store), `graph`, `symbols`, and `self` workloads (default 15% CPU tolerance):

```sh
python3 tooling/bench_compiler.py --compiler /path/to/stage3/cinch \
  --bundle /path/to/stage1/compiler.cinch \
  --workloads hello graph symbols self > baseline.json

python3 tooling/bench_compiler.py --compiler /path/to/candidate/stage3/cinch \
  --bundle /path/to/stage1/compiler.cinch \
  --workloads hello graph symbols self --baseline baseline.json
```

---

## Tree-sitter grammars and checks

Generated parser files (`src/parser.c`, `src/grammar.json`, `src/node-types.json`) are intentionally not committed. The drift check always regenerates from `grammar.js` before running corpus assertions:

```sh
tooling/check-grammar-drift.sh            # surface grammar (tree-sitter-cinch)
tooling/check-grammar-drift.sh --core     # Core grammar (tree-sitter-cinch-core)
tooling/check-real-files-parse.sh --core  # parse Core samples/goldens with zero ERROR nodes
tooling/check-real-files-parse.sh --surface
```

- **Surface grammar (`tree-sitter-cinch/`):** targets [`specifications/surface-syntax-and-grammar.md`](../specifications/surface-syntax-and-grammar.md) and [`specifications/surface-grammar-local-amendments.md`](../specifications/surface-grammar-local-amendments.md). `src/scanner.c` ports `seed/src/lexer.rs` and `seed/src/layout.rs` to handle context-sensitive rules (kebab-case dashes vs minus, negative literals, `'a'` vs `'sym`, `$` operator runs, layout blocks, and balanced verbatim fragments). Comments stay scanner trivia so the LSP highlights them via semantic tokens.
- **Core grammar (`tree-sitter-cinch-core/`):** scannerless grammar targeting [`design/core-output.md`](../design/core-output.md) sections 4 and 10 (`source.cinch-core`, `.core`, `core 1` file signature, `--` line comments as named `comment` nodes).

### Known surface-grammar gaps
- Tree-sitter cannot reproduce two hard lexer rejections: tabs in indentation and newlines between `#[` and an attribute name.
- `check-real-files-parse.sh --surface` still reports parse errors on prelude files using newer refinement syntax not yet added to `grammar.js`.

---

## Language server and editors

`cinch lsp` runs the stdio JSON-RPC language server directly; `lsp-driver/` exposes the same entry point as a small standalone binary (`cargo run --manifest-path tooling/lsp-driver/Cargo.toml --release`).

Every editor integration under `editors/` supports both `.cinch` (`cinch`) and `.core` (`cinch-core`):

| Editor | Location | Validation check |
| --- | --- | --- |
| Vim / Neovim | `editors/vim/` | `tooling/editors/vim/test/run.sh [vim\|nvim]` |
| Emacs | `editors/emacs/` | `tooling/editors/emacs/test/run.sh <emacs>` |
| Sublime Text | `editors/sublime/` | `python3 tooling/editors/sublime/test/check.py` |
| VS Code / Cursor | `editors/vscode/` | `node tooling/editors/vscode/test/check.mjs` |
| Zed | `editors/zed/` | `python3 tooling/editors/zed/test/check.py` (Python 3.11+) |

### Installing the Zed dev extension

```sh
cargo build --release --manifest-path seed/Cargo.toml
tooling/editors/zed/sync-grammar.sh
```

Then in Zed: `zed: extensions` -> `Install Dev Extension` -> select `tooling/editors/zed`. Enable combined semantic tokens so LSP highlights comments and resolved symbols alongside Tree-sitter:

```json
{ "languages": { "Cinch": { "semantic_tokens": "combined" } } }
```

## Tooling tests and CI

```sh
python3 tooling/check-compiler-layout.py
python3 -m unittest discover -s tooling/tests -v
```

CI workflows under `.github/workflows/` run Nix binary-cache builds (`ladder-cache.yml`), nightly Stage 3 releases (`publish-nightly.yml`), website builds (`website.yml`), and registry catalogue refreshes (`catalogue-refresh.yml`).
