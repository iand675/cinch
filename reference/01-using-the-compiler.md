# Using the compiler

The `cinch` executable is a single driver covering typechecking, native and WebAssembly code generation, Core IR inspection, C++ interop, documentation extraction and search, an interactive REPL, a language server, and package management.

# Installation

Standalone nightly Stage 3 binaries for Linux (x86-64, ARM64) and Apple Silicon macOS are published on the [downloads page](/downloads/).

To build the Rust compiler from source:

```sh
cargo build --manifest-path seed/Cargo.toml
export PATH="$PWD/seed/target/debug:$PATH"
cinch check examples/compiler/fib.cinch
```

Building requires Rust, `clang` 19 or newer on `PATH` (or pointed to by `CINCH_CLANG`), and `tree-sitter` to generate the Cinch parser during `cargo build`. Native `build` and `run` commands compile the zero-dependency runtime crate (`seed/runtime`) on demand via `cargo` and link `libcinchruntime.a`, rebuilding automatically whenever a runtime source changes.

# Commands

Every compilation command takes one entry `.cinch` file. Imports resolve against sibling files beside the entry first, then vendored packages, then the embedded standard library.

| Command | Purpose | Primary outputs |
| --- | --- | --- |
| `cinch check <file.cinch>` | Parse, typecheck, and verify all obligations without emitting code. | `build/` interfaces & manifest |
| `cinch emit-ir <file.cinch> [-o out.ll]` | Emit LLVM IR text (defaults to stdout). Requires `main`. | LLVM IR + `build/` artifacts |
| `cinch emit-object <lib.cinch> [-o out.o]` | Emit a closed library object file (must not declare `main`). | `<out>.o`, `<out>.ll`, `<out>.abi.json` |
| `cinch build <file.cinch> [-o out]` | Compile and link an executable, Wasm module, or `--shared` library. | `<out>`, `<out>.ll`, `build/` artifacts |
| `cinch run <file.cinch> [args...]` | Compile to a temporary directory, execute, and return the program's exit code. | `build/` artifacts (temp binary removed) |
| `cinch core emit\|verify\|diff ...` | Dump, verify, or structurally diff Core IR artifacts. | `build/<entry>.core` (or `-o`) |
| `cinch cxx [build] <bindings.json> --out-dir <dir>` | Generate C++17 bindings or build native C++ exports / GDExtensions. | Generated C++/Cinch files and binaries |
| `cinch doc <subcommand> ...` | Extract `.cinchdoc`, render Markdown/HTML, Hoogle search, lookup, index, or run doctests. | See [Chapter 28](28-documentation.md) |
| `cinch repl` | Start an interactive session in the current project. | See [Chapter 26](26-repl.md) |
| `cinch lsp` | Run the Language Server Protocol daemon over stdio. | See [Chapter 27](27-language-server.md) |
| `cinch pkg add\|install\|info\|publish\|verify` | Manage project dependencies, lockfiles, and registry releases. | See [Chapter 29](29-package-management.md) |
| `cinch jit ...` | Execute via LLVM ORC LLJIT or a staged native frontend (requires `--features jit`). | None on disk |

Running `cinch` with no arguments, `--help`, or an unrecognized subcommand prints the full usage summary to stderr and exits `2`.

Without `-o`, `build` and `emit-object` derive the output filename from the entry path without its extension (`cinch build Main.cinch` writes `Main` and `Main.ll`). The `build/` directory is always placed beside the entry source regardless of `-o`.

`emit-ir`, `build` (unless `--shared` is passed), and `run` require an entry point `main`. `check` and `emit-object` accept library modules without `main`, and `emit-object` refuses modules that define one.

# Flags

## Output, linking, and optimization

| Flag | Modes | Effect |
| --- | --- | --- |
| `-o <path>` | `emit-ir`, `emit-object`, `build` | Output path. `emit-object` refuses paths that collide with its `.ll` or `.abi.json` companions. |
| `--opt-level <0\|1\|2\|3>` | `build`, `run`, `emit-object` | Optimization level passed to `clang` as `-O<n>`. Defaults to `0` for `build`/`run` (fast compile) and `2` for `emit-object`. Reference-counting helpers are private per module and inline at `-O2`. |
| `--shared` | `build` | Emit a self-contained native shared library (no `main` required; atomic output replacement). |
| `--native-object <path>` | `build`, `run` | Link an additional native object or static archive (repeatable). |
| `--package-object <path>` | `build`, `run` | Link a separately compiled Cinch package object after verifying its SHA-256 against its `.abi.json` manifest. |
| `--object-roots <path>` | `emit-object` | JSON file (`schema: 1`, `units`/`dictionaries`/`methods` arrays with `R`/`O`/`T` class vectors) specifying concrete library variants to retain. |
| `--print-link-plan` | `check`, `emit-ir`, `emit-object`, `build`, `run` | Print every native link contribution and its requesting package before linking. |
| `--allow-raw-link <name>` | same | Consent to one package's raw linker flags (CLI form of `raw_link` in `.cinchpkg`). |
| `--rebuild` | same | Recompute the package-store entry instead of reusing an existing one. |
| `--` | `run` | Stop parsing compiler flags; forward all subsequent words verbatim to the program (`process-args`). |

## Targets and cross-compilation

| Flag | Modes | Effect |
| --- | --- | --- |
| `--target <triple>` | `check`, `emit-ir`, `emit-object`, `build`, `run` | Select one of five supported triples: `aarch64-apple-darwin`, `x86_64-apple-darwin`, `x86_64-unknown-linux-gnu`, `aarch64-unknown-linux-gnu`, or `wasm32-unknown-unknown`. |
| `--runtime-archive <path>` | `build`, `run` | Path to a prebuilt `libcinchruntime.a` for a foreign native `--target`. Verified for archive validity and target architecture before linking. |
| `--target-version <ver>` | all compile modes | Platform version evaluated by package manifest `os_version` conditions. |

## Core simplifier and lowering controls

These flags control optimization and memory-lowering passes during `emit-ir`, `emit-object`, `build`, and `run`:

| Flag | Effect |
| --- | --- |
| `--dump-core <stage>` | Write Core IR beside other build artifacts (`checked`, `owned`, or `lowered`). |
| `--max-simplifier-iterations <n>` | Cap Core simplifier passes after specialization (default `4`; `0` disables simplification). |
| `--inline-model <benefit\|threshold>` | Choose Flambda 2 benefit-vs-size inlining (default) or GHC weighted-threshold inlining. |
| `--no-inline` | Disable the simplifier's inliner (`#[inline(never)]` always blocks inlining). |
| `--no-direct-dict-calls` | Keep static dictionary projections as indirect `__apply` calls. |
| `--no-saturate-partial` | Keep single-use partial applications as closures instead of saturated calls. |
| `--no-contify` | Keep tail-only local continuations as closures rather than join points. |
| `--no-specialise-arguments` | Disable call-site argument specialization for known closures and function records. |
| `--no-stack-alloc` | Place all allocations on the heap instead of stack-allocating non-escaping blocks. |
| `--stack-frame-limit <bytes>` | Maximum frame bytes reserved per unit for stack allocation (default `16384`). |
| `--escape-report` | Print the per-class escape census after stack allocation (same as `CINCH_ESCAPE_REPORT=1`). |
| `--no-reclaim` | Omit reference-counting operations (leak-everything baseline; same as `CINCH_RECLAIM=0`). |
| `--emit-interface-optimisations` | Include unfoldings in the `.cpk` package (default for `emit-object`). |
| `--interface-optimisations <pkg>` | Inline unfoldings from a separately built package at call sites. |

Unrecognized words are **not** rejected as unknown flags: before the entry file they are treated as the entry path, and after the entry file they are forwarded as program arguments. A recognized flag missing its value exits `2`.

# Core output

Core is the typed, annotated intermediate representation between checking and LLVM emission. Dumping Core lets you inspect specialization, ownership insertion, and target lowering without reading LLVM IR or compiler source.

```sh
cinch core emit Main.cinch                                  # build/Main.core (lowered)
cinch core emit Main.cinch --stage=owned -o Main.owned.core
cinch core verify build/Main.core
cinch core diff build/Main.core build/Main.owned.core
```

## Pipeline stages

| Stage | Contents | Target-dependent? |
| --- | --- | --- |
| `checked` (`solved`, `elaborated`) | Elaborated program with resolved dictionaries and evidence; no RC ops or target facts. | No (byte-identical across `--target`) |
| `owned` | Reference-count pass (`dup`, `drop`, `release`, binder classes) applied to `checked`. | No |
| `lowered` | Post-specialization and reference counting — the exact input to LLVM codegen, with target header and selected `prim` bodies. | Yes |

Because specialization runs before reference counting in a real build, `owned` is not a literal prefix of `lowered`; it shows the standalone RC pass on `checked`. Passing `--dump-core=<stage>` to `check`, `emit-ir`, `build`, or `run` writes `build/<entry>.core` for `lowered` and `build/<entry>.<stage>.core` for earlier stages (recording the file and its SHA-256 in `build/<entry>.manifest.json`). `check` stops after checking, so any `--dump-core` on `check` writes `checked`.

## Format, verification, and diffing

A Core file has a deterministic header (`core 1`, `module` lines, optional `target`, `source sha256:...`, and external `global` declarations) followed by canonically sorted units whose binders are alpha-numbered (`t0`, `t1`, ...) so diffs are stable across runs:

```text
core 1
module Main
target aarch64-apple-darwin opt-level 0
source sha256:3196f4bd983c5d0d1821cbb8ba28c1df4a9fe4fc64c81aa0cc08e0277590c964
global Data.Core.Nil

unit Main.main :: Int
  @stage(lowered)
  body (let t0 @class(owned)
    (ctor Data.Core.Nil @classes(T) 0 0)
    (drop t0 @class(owned) (int 0)))
end
```

- **`cinch core verify <file.core>`** re-parses and checks grammar, unit uniqueness, `global`/`local` scoping, dictionary slot tables, ownership ops, and `@span` annotations. Exits `0` on valid Core, `1` with `unit:path: message` on error.
- **`cinch core diff <a.core> <b.core>`** compares two dumps unit by unit in canonical order, ignoring header target lines, and prints the first structural path where they diverge (exit `0` if equal, `1` if different).

# Cross compilation

`check`, `emit-ir`, and `emit-object` support all five triples on any host. The compiler derives OS, architecture, C library, word size, and byte order solely from `--target`, emits the matching LLVM `target triple` and `target datalayout`, and selects matching `when` branches for `prim` bodies, imports, and declarations:

```sh
cinch emit-ir Main.cinch --target aarch64-unknown-linux-gnu | grep -E 'target (triple|datalayout)'
```

- **WebAssembly (`wasm32-unknown-unknown`)**: `build` builds the freestanding `seed/runtime` archive on demand and links with `clang -c` plus `wasm-ld` (set `CINCH_WASM_LD` if not beside `clang` or on `PATH`). `run` executes the resulting `.wasm` module under Node and returns its exit code.
- **Foreign native targets**: `build` requires `--runtime-archive <path>/libcinchruntime.a` compiled for that target (`cargo build --manifest-path seed/runtime/Cargo.toml --target <triple>`) and a cross-capable `clang` via `CINCH_CLANG`. `run` refuses foreign native binaries because the host cannot execute them.

# Entry points and exit statuses

A pure executable defines `main :: Int`. An effectful executable defines `main :: Unit ->{effects} Int` and introduces capabilities such as `newIO ()`. The return value becomes the process exit code; returning an integer does not print anything (use `Effect.Io` for output).

<!-- cinch:run exit=0 -->
```cinch
module Main where
  fib :: Int -> Int
  fib n = if n == 0 then 0 else if n == 1 then 1 else fib (n - 1) + fib (n - 2)

  main :: Int
  main = if fib 10 == 55 then 0 else 1
```

Command-line arguments passed after `<file.cinch>` in `cinch run` are read via `process-args` (`Effect.Process`). Use `--` to pass arguments that look like compiler flags:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  import Data.List
  import Data.Nat
  import Effect.Process

  count :: List String -> Nat
  count xs = case xs of
    Nil -> 0
    Cons x rest -> 1 + count rest

  main :: Unit ->{IO r} Int
  main u = with (io = newIO ()) in case process-args () of
    Err e -> 99
    Ok words -> nat-to-int (count words)
```

| Exit code | Meaning |
| --- | --- |
| `0` | Success (or program exited `0` under `run`). |
| `1` | Compile/link error, unreadable input, or program exited `1` under `run`. |
| `2` | CLI usage error (`--help`, unknown command, missing flag value, invalid JSON input, or `jit` unavailable). |
| *N* | Under `run`, the executed program's exit status. Run `check` separately to distinguish compile errors from exit code `1`. |

# Diagnostics

Lexer, layout, and parser errors name the file first (`cinch: Main.cinch: 2:1: ...`). Checker and backend errors put `line:col` first with the file at the end:

<!-- cinch:reject contains="non-exhaustive patterns: constructor None is not covered (in Main.cinch)" -->
```cinch
module Main where
  data Opt where
    None :: Opt
    Some :: Int -> Opt

  pick :: Opt -> Int
  pick o = case o of
    Some n -> n

  main :: Int
  main = pick None
```

```text
cinch: 7:12: non-exhaustive patterns: constructor None is not covered (in Main.cinch)
```

See [Chapter 21](21-editors-and-diagnostics.md) for a phase-by-phase guide to diagnostics.

# Build artifacts and the package store

Every compile writes `build/` beside the entry file:

| Path | Contents |
| --- | --- |
| `build/<entry>.cpk` (or `build/<Module>.cinchif`) | Checked interface closure for the program and base library. |
| `build/<entry>.manifest.json` | Build manifest: modules, target/host profiles, assumptions, laws, safety, staging, specialization, and ABI metadata. |
| `build/<entry>.stage.snap` | Captured compile-time staging inputs (replayed when `CINCH_STAGE_MODE=replay`). |

Each compile also populates a per-user package store (`CINCH_STORE`, defaulting to `~/Library/Caches/cinch/store` on macOS or `$XDG_CACHE_HOME/cinch/store` / `~/.cache/cinch/store` on Linux). Writes stage into a temporary directory and swap atomically. Corrupt or incomplete entries are silently rebuilt on the next compile; `--rebuild` forces a rewrite.

Batch CLI invocations currently recheck from source on each run; there is no `--watch` daemon. In-process incremental rechecking is used by `cinch repl` and `cinch lsp` (see [Chapter 18](18-artifacts-and-incremental.md)).

# Environment variables

| Variable | Scope | Effect |
| --- | --- | --- |
| `CINCH_CLANG` | `build`, `run`, `emit-object` | Override the `clang` binary (must be Clang 19+). |
| `CINCH_WASM_LD` | `wasm32` `build`, `run` | Override the `wasm-ld` linker path. |
| `CINCH_STORE` | All compile modes | Override the package-store cache directory. |
| `CINCH_STAGE_MODE` | All compile modes | Staging input mode: `refresh` (default), `replay`, or `live`. |
| `CINCH_RECLAIM` | Codegen | Set to `0` to omit reference-counting ops (same as `--no-reclaim`). |
| `CINCH_NO_CPR` | Codegen | Set to non-`0` to disable constructed-product-result register returns. |
| `CINCH_ESCAPE_REPORT` | Codegen | Set to `1` to print the stack-allocation escape census. |
| `CINCH_SCHEMA_TOOL` | Staging | Database schema introspection tool invoked by `Stage.withDatabase`. |
| `CINCH_REGISTRY`, `CINCH_REGISTRY_TOKEN` | `cinch pkg` | Default registry URL and auth token. |
| `CINCH_LLVM_CONFIG` | Building with `--features jit` | Path to `llvm-config` for ORC LLJIT headers and libraries. |

Runtime tuning variables (`CINCH_HEAP_LIMIT`, `CINCH_STACK_SIZE`, `CINCH_ALLOC_STATS`, `CINCH_LEAK_CHECK`, `CINCH_RC_DEBUG`) are read by compiled executables and forwarded unchanged by `cinch run`; see [Chapter 25](25-allocators.md#environment-variables).

# Current boundaries

- **JIT**: Not compiled into the default binary; build with `cargo build --manifest-path seed/Cargo.toml --features jit`.
- **Cross-target bundles**: <span class="support-tag support-tag--partial">Partial</span> `check`, `emit-ir`, and `emit-object` work for all five triples on any host; native cross `build` needs a target `--runtime-archive` and cross `CINCH_CLANG`, while `wasm32-unknown-unknown` builds its own runtime archive and runs under Node.
- **Watch mode and bundle distribution**: <span class="support-tag support-tag--planned">Coming soon</span> No `--watch` CLI or distributable cross-target bundle packaging yet.
