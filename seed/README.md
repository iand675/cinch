# Cinch bootstrap compiler (`seed/`)

`seed/` is the Rust implementation of the Cinch compiler. It serves two roles:

1. **Stage 0 bootstrap seed** for the self-hosted Cinch compiler in [`compiler/`](../compiler/README.md).
2. **Host runtime, standard library, and interactive CLI**, owning `cinchruntime` (`seed/runtime/`), the standard library sources (`seed/prelude/`, 331 modules including the 70-module embedded `cinch-base` family), and the `cinch` CLI (`check`, `build`, `run`, `core`, `repl`, `lsp`, `doc`, `pkg`, `jit`, `cxx`).

Both `seed/` and `compiler/` speak **interface schema 10** and **runtime ABI 10**.

## Toolchain

The [devenv](https://devenv.sh) shell supplies Rust, LLVM, `clang`, Tree-sitter, and Node 24. Cargo builds the Tree-sitter parser automatically; outside the shell, ensure `tree-sitter` and Node are on `PATH` (or set `TREE_SITTER`). Set `CINCH_CLANG` to override the `clang` binary used for compilation and linking.

```sh
devenv shell        # rustc, cargo, clang, llvm, tree-sitter, node
cinch-test          # cargo test
```

## CLI overview

### Compilation and inspection

```sh
cinch check examples/compiler/fib.cinch
cinch run examples/compiler/fib.cinch [args...]
cinch emit-ir examples/compiler/fib.cinch -o fib.ll
cinch build examples/compiler/fib.cinch -o fib
cinch build Library.cinch --shared -o libLibrary.so
cinch core emit|verify|diff examples/compiler/fib.cinch
cinch repl
cinch lsp
```

- `check`, `build`, and `emit-ir` emit compiled interfaces (`build/<module>.cinchif`) and a packed interface closure (`build/<entry>.cpk`).
- `--target <triple>` supports five targets: `aarch64-apple-darwin`, `x86_64-apple-darwin`, `x86_64-unknown-linux-gnu`, `aarch64-unknown-linux-gnu`, and `wasm32-unknown-unknown`. `check` and `emit-ir` work across all five directly; cross-target `build` requires `--runtime-archive <path>`.
- `--shared` builds a self-contained native shared library with load-time runtime initialization (atomic output replacement); `--native-object` links additional native objects or archives.
- `--array-report` / `--array-report-json` inspect Typed Array IR fusion and GPU scheduling decisions; `--array-target=metal-required` enforces Metal GPU eligibility at compile time.

### Packages

```sh
cinch pkg add alice/library@0.1.0 --registry http://127.0.0.1:8787
cinch pkg add ./libs/bytes --project .
cinch pkg add --git https://example.org/bytes --git-rev v1.2.0 --project .
cinch pkg install --project . [--locked]
cinch pkg info acme/watch [--target TRIPLE] [--target-version VERSION]
cinch pkg verify [--slsa] --project .
cinch pkg publish --registry http://127.0.0.1:8787 --project . --token "$CINCH_REGISTRY_TOKEN"
```

`cinch pkg` resolves registry, `path`, and `git` dependencies into one closure, vendors sources under `vendor/`, builds `package.o`, `package.abi.json`, and `build/package.cinchif`, and pins SHA-256 package identities in `.cinchlock`. Git checkouts cache under `$CINCH_STORE/git-packages/`. See [`reference/29-package-management.md`](../reference/29-package-management.md).

### Documentation

```sh
cinch doc extract seed/prelude/Data/Option.cinch --legacy -o /tmp/option.cinchdoc
cinch doc render seed/prelude/Data/Option.cinch --legacy
cinch doc html seed/prelude --out-dir build/docs
cinch doc test seed/prelude
cinch doc markdown README.md
```

### JIT

Build with `--features jit` (requires LLVM ORC C headers/libraries; select `llvm-config` via `CINCH_LLVM_CONFIG`):

```sh
cargo build --manifest-path seed/Cargo.toml --features jit --bin cinch
cinch jit examples/compiler/fib.cinch
cinch jit --package-object vendor/acme/top/0.1.0/package.o \
          --package-dependency vendor/acme/base/0.1.0/package.o \
          --call Top.answer
```

On macOS and Linux, `cinch jit --native-stage` drives a staged self-hosted compiler against a cached base package (adding `--serve` keeps it resident across saves):

```sh
cinch jit --native-stage build/bootstrap/stage3 Main.cinch -- arg1
cinch jit --native-stage build/bootstrap/stage3 --serve Main.cinch -- arg1
```

### C++ and Godot interop

```sh
cinch cxx bindings.json --out-dir DIR
cinch cxx build bindings.json --out-dir DIR [--godot-cpp DIR]
```

Generates direct no-unwind native symbol imports, C++17 C-ABI shims with linear destructor checking, Cinch-implemented C++ classes, and Godot 4 GDExtensions. See [`examples/cxx/README.md`](../examples/cxx/README.md), [`examples/cxx/exports/README.md`](../examples/cxx/exports/README.md), and [`examples/godot/README.md`](../examples/godot/README.md).

### Library objects

Two object-linkage paths avoid recompiling dependencies:

- **Installed package objects (`--package-object`):** links `package.o` produced by `cinch pkg`, verifying `build/package.cpk` / `.cinchif` and emitting external declarations for imported units.
- **Standalone library objects (`cinch emit-object`):** emits a closed object (`library.o`), LLVM IR (`library.ll`), physical ABI manifest (`library.abi.json`), and packed interfaces (`.cpk`) with concrete specialization roots from `--object-roots roots.json`. See [`design/compilation-and-interfaces.md`](../design/compilation-and-interfaces.md).

## Compilation pipeline

```text
source -> lexer -> layout -> parser (+ fixity pass) -> checker/elaborator
       -> Core IR -> specialization & simplifier -> Typed Array IR -> wide-edge & RC passes
       -> stack/escape analysis -> LLVM IR text -> clang -> native executable / .so / .o / wasm
```

Emitting LLVM IR text and invoking `clang` keeps LLVM out of the default Rust dependency graph while enabling full LLVM optimization (`--opt-level 0..3`) and cross-target IR emission.

## Implemented features

- **Syntax & modules:** indentation layout, multi-file `import`/`export` with re-exports, `.cinchif`/`.cpk` interfaces, fixity resolution, `data`, `record`, `signature`, `instance` / `implicit instance`, `upgrade`, `stage`, `rewrite`, `prim`, `foreign import`/`export ccall`, and `letrec`.
- **Types & dependent core:** stratified universes (`Type 0 : ...`, `Prop`), bidirectional checking with HM local generalization, Rank-N polymorphism, dependent functions and `Sigma` pairs, indexed inductive families with SCC positivity and index-aware coverage, `spec`/`proof` totality and erasure, `refl`/`congr`/`decide`, and refinement types with difference-logic and Farkas linear solvers.
- **Primitives & arrays:** arbitrary-precision `Int`/`Nat` (tagged immediates + bignums), machine scalars (`I8`..`I128`, `U8`..`U128`, `F32`, `F64`, `Char`, `Usize`, `Isize`), `String`, `Bytes`, interned `Sym`, `Array a`, `Tensor a`, `Columnar row`, shape-indexed views (`View1`/`View2`/`View3`), and Metal GPU execution.
- **Effects, laziness & concurrency:** row-typed effects, one-shot `handle`/`resume`, `do` sequencing modules, lexical capabilities (`using (io :: Io r)`), affine `Thunk`, region memo bindings (`~ x = e`), guarded `codata`/`corec`/`Later`, green-thread fibers (`sched.rs`), linear `Task` handles, native `MVar`, and TL2 `Effect.Stm`.
- **Ownership & layout:** `%0`/`%1`/`%aff`/`%many` multiplicities, structural `Copy`/`Share`, `Ref r a`/`Mut r a` borrows, Perceus RC with reuse and frame allocation, ABI 10 self-describing blocks, `#[repr(...)]`, `#[unpack]`, `#[byval]`/`#[sret]`, and safety assertions (`#[no-alloc]`, `#[profile "jpl"]`).

### Known deliberate boundaries

- Matching constructors directly on rigid family indices is rejected (dispatch via `decide` or computed indices).
- `~` is reserved for memo bindings (`~ x = e`), not irrefutable argument patterns.
- Polymorphic or `using`-constrained helpers must be top-level declarations (no local `forall`/`using` signatures).
- Functions list required class constraints explicitly (no superclass entailment bundles).
- Cross-function specialization of concrete effect handler vtables is not yet implemented.

## Repository layout

```text
seed/
  Cargo.toml
  prelude/
    Prelude.cinch             default prelude facade and re-exports
    cinch-base.cinchpkg       embedded base package manifest (70 modules)
    Data/Core.cinch           core inductive types re-exported by home modules
    ...                       331 stdlib modules across Algebra..Text
  runtime/                    cinchruntime staticlib + cdylib (small-object heap, RC, fibers, STM, MVar, Metal)
  src/
    lexer.rs, layout.rs       lexical analysis and indentation layout
    parser.rs, ast.rs         surface grammar, fixity pass, AST
    check.rs, check_terms.rs  bidirectional checker, elaboration, pattern matching
    check_decls.rs, check_dictionary.rs  declarations, repr layouts, dictionary resolution
    core.rs, coretext.rs      Core IR and textual Core printer/verifier
    specialize.rs, simplify.rs  representation specialization, contification, inlining
    array_ir.rs, arrays_codegen.rs  Typed Array IR fusion and CPU/Metal lowering
    wide.rs, rc.rs, stack.rs  wide boxing edges, Perceus RC, frame allocation
    codegen.rs, cabi.rs       LLVM IR emission and C ABI lowering
    interface.rs, store.rs, incremental.rs, query.rs  schema 10 interfaces and caching
    driver.rs, main.rs        pipeline driver and CLI dispatch
    repl.rs, repl_tui.rs      interactive REPL (line loop and TUI)
    lsp.rs, analysis.rs       stdio LSP server and shared analysis layer
    pkg.rs, cinchdoc.rs       package manager and documentation generator
    jit.rs, native_jit.rs     optional LLVM ORC JIT and native-stage runner
    cxx/                      C++ binding generator and Godot GDExtension builder
  tests/                      integration, runtime, FFI, LSP, REPL, and bootstrap suites
```

## Status and tracking

- **Feature support matrix & roadmap:** [`reference/16-support-and-roadmap.md`](../reference/16-support-and-roadmap.md)
- **Architecture & open decisions:** [`design/README.md`](../design/README.md)
- **Standard library architecture & Go parity:** [`design/stdlib-architecture.md`](../design/stdlib-architecture.md)
- **Self-hosting progress:** [`compiler/README.md`](../compiler/README.md) and [`compiler/CHECKPOINT.md`](../compiler/CHECKPOINT.md)
