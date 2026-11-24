# Cinch bootstrap compiler (`seed/`)

`seed/` is the Rust implementation of the Cinch compiler. It serves two roles in this repository:

1. **Stage 0 bootstrap seed** for the self-hosted Cinch compiler in [`compiler/`](../compiler/README.md).
2. **Host runtime and embedded base provider**, owning the `cinchruntime` static/shared library (`seed/runtime/`), the embedded `prelude/` source family (`cinch-base`, 55 modules), and the development CLI (`cinch`).

The compiler targets the surface syntax in [`specifications/surface-syntax-and-grammar.md`](../specifications/surface-syntax-and-grammar.md) and the static/dynamic semantics in [`specifications/core-language-specification.md`](../specifications/core-language-specification.md). Both `seed/` and `compiler/` currently speak **interface schema 10** and **runtime ABI 10**.

## Toolchain

The [devenv](https://devenv.sh) shell supplies Rust, LLVM, `clang`, Tree-sitter, and Node. Cargo builds the Tree-sitter parser automatically; outside the shell, ensure `tree-sitter` and Node are on `PATH` (or set `TREE_SITTER`). Set `CINCH_CLANG` to override the `clang` binary used for compilation and linking.

```sh
devenv shell        # rustc, cargo, clang, llvm, tree-sitter
cinch-test          # cargo test
```

### Compilation

```sh
cinch check examples/compiler/fib.cinch
cinch run examples/compiler/fib.cinch [args...]
cinch emit-ir examples/compiler/fib.cinch -o fib.ll
cinch build examples/compiler/fib.cinch -o fib
cinch build Library.cinch --shared -o libLibrary.so
```

- `check`, `build`, and `emit-ir` write compiled interfaces (`build/<module>.cinchif`) and a packed interface closure (`build/<entry>.cpk`).
- `--target <triple>` supports five triples: `aarch64-apple-darwin`, `x86_64-apple-darwin`, `x86_64-unknown-linux-gnu`, `aarch64-unknown-linux-gnu`, and `wasm32-unknown-unknown`. `check` and `emit-ir` work for all five directly; cross-target `build` requires `--runtime-archive <path>`.
- `--shared` emits a self-contained native shared library with load-time runtime initialization (replacing the output atomically on link success); `--native-object` links additional native objects or archives.
- `cinch core emit|verify|diff` inspects, verifies, and diffs the serialized Core IR consumed by code generation.
- `cinch repl` starts the interactive terminal/line REPL; `cinch lsp` runs the stdio Language Server Protocol server.

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

`cinch pkg add` and `install` resolve registry, `path`, and `git` dependencies into a single closure (using fixed-point range intersection for registry releases), vendor sources under `vendor/`, compile `package.o`, `package.abi.json`, and `build/package.cinchif`, and pin content-addressed SHA-256 package identities in `.cinchlock`. Git checkouts are cached under `$CINCH_STORE/git-packages/`. `verify --slsa` checks local integrity and unsigned SLSA Build Level 1 provenance. See [`reference/29-package-management.md`](../reference/29-package-management.md).

### Documentation

```sh
cinch doc extract seed/prelude/Data/Option.cinch --legacy -o /tmp/option.cinchdoc
cinch doc render seed/prelude/Data/Option.cinch --legacy
cinch doc html seed/prelude --out-dir build/docs
cinch doc test seed/prelude
cinch doc markdown README.md
```

`cinch doc` extracts canonical `.cinchdoc` JSON, renders Markdown and HTML, indexes and searches symbols, executes doc-test blocks, and applies Tree-sitter syntax highlighting (`tooling/tree-sitter-cinch`) to Cinch code fences.

### JIT

Build with the optional `jit` Cargo feature (requires LLVM ORC C headers/libraries; select `llvm-config` with `CINCH_LLVM_CONFIG`):

```sh
cargo build --features jit --bin cinch
cinch jit examples/compiler/fib.cinch
cinch jit --package-object vendor/acme/top/0.1.0/package.o \
          --package-dependency vendor/acme/base/0.1.0/package.o \
          --call Top.answer
```

`cinch jit` compiles to LLVM IR and runs `main` (or a zero-arity `--call` symbol from prebuilt package objects) in-process through LLVM's LLJIT API against the shared `cinchruntime` library.

On macOS and Linux, `cinch jit --native-stage` drives a staged self-hosted compiler with a cached prebuilt base package (optionally staying resident with `--serve`, waiting for one empty line per rebuild on stdin and reporting `cinch jit: status N` on stderr):

```sh
cinch jit --native-stage build/bootstrap/stage3 Main.cinch -- arg1
cinch jit --native-stage build/bootstrap/stage3 --serve Main.cinch -- arg1
```

### C++ interop

```sh
cinch cxx bindings.json --out-dir DIR
cinch cxx build bindings.json --out-dir DIR [--godot-cpp DIR]
```

`cinch cxx` generates direct no-unwind native symbol imports as well as C++17 C-ABI shims for owned opaque classes (with linear destructor checking), borrowed methods, and scalar functions. `cinch cxx build` compiles C exports, Cinch-implemented C++ classes (fields, constructors/destructors, virtual overrides), and optional Godot GDExtensions. See [`examples/cxx/README.md`](../examples/cxx/README.md), [`examples/cxx/exports/README.md`](../examples/cxx/exports/README.md), and [`examples/godot/README.md`](../examples/godot/README.md).

### Library objects

Two separate object-linkage mechanisms exist:

- **Installed package objects (`--package-object`):** `cinch build`, `cinch run`, and `cinch jit` accept `--package-object <path>` to link an installed package object produced by `cinch pkg`. The compiler validates the object's execution interface (`build/package.cpk` / `.cinchif`), emits external declarations instead of re-emitting the package's modules, and links the object directly. Without `--package-object`, `cinch build` and `cinch run` compile vendored source files into the project directly.
- **Standalone library objects (`emit-object`):** `cinch emit-object Library.cinch -o library.o [--object-roots roots.json]` emits a closed library object (`library.o`), its LLVM text (`library.ll`), physical ABI manifest (`library.abi.json`), and packed interfaces (`.cpk`) with concrete generic/dictionary roots requested by a consumer. The self-hosted stage compiler consumes these bundles via `--interfaces MANIFEST --objects CONFIG`. See [`design/runtime-object-linkage.md`](../design/runtime-object-linkage.md).

## Compilation pipeline

```text
source -> lexer -> layout -> parser (+ fixity pass) -> checker/elaborator
       -> Core IR -> RC / ownership verification -> simplifier & specialization
       -> stack/escape analysis -> LLVM IR text -> clang -> native executable / .so / .o
```

Emitting LLVM IR text and invoking `clang` keeps LLVM out of the default Rust dependency graph while still enabling full LLVM optimization (`--opt-level 0..3`) and cross-target IR emission. The Core IR and runtime closure/dictionary ABI are documented in `seed/src/core.rs`.

## Implemented language and compiler features

- **Syntax & modules:** indentation layout (`where`, `of`, `let`, `in`, explicit braces, nested `{- -}` comments), multi-file `import`/`export` with selective lists and re-exports, `.cinchif` and `.cpk` compiled interfaces, fixity resolution, `data`, `record`, `signature`, `instance` / `implicit instance`, `upgrade`, `stage`, `rewrite`, `prim`, `foreign import`/`export ccall`, and `letrec` recursive binding groups.
- **Types & dependent core:** stratified universes (`Type 0 : Type 1 : ...`, `Prop`), bidirectional type checking with Hindley-Milner local let-generalization, rank-N polymorphism, dependent functions and pairs (`Sigma`), indexed inductive families (`Vec`, `Fin`, `Dec`, user families) with positivity and index-aware coverage, `spec`/`proof` totality and erasure, `refl`/`congr`, `decide`, and refinement types.
- **Primitive types & arrays:** arbitrary-precision mathematical `Int` and `Nat` (tagged immediates + bignums), explicit machine scalars (`I8`..`I64`, `U8`..`U64`, `F32`, `F64`, `Char`, `Usize`, `Isize`), immutable `String` and mutable `Bytes`, interned `Sym`, and native `Array a`, `Tensor a`, and `Columnar row` (`Data.Array`, `Data.Array.Packed`, `Data.Tensor`, `Data.Columnar`).
- **Effects, capabilities & laziness:** row-typed effects with handlers (`handle ... with`, one-shot `resume`), `do` notation, lexically scoped capabilities (`using (io :: Io r)`, `with (io = ...) in`, `newIO`), affine thunks (`delay`/`force`), memo bindings (`~ x = e`) with `Eval r` black-hole checking, and guarded `codata`/`corec`/`Later`.
- **Ownership, reclamation & layout:** `%0`/`%1`/`%aff`/`%many` multiplicities, structural `Copy`, `Ref r a`/`Mut r a` region borrows, automatic RC insertion with reuse and frame/stack allocation, `#[repr(C|packed|align|transparent|unboxed|uN)]`, `#[unpack]`/`#[nounpack]`, `#[byval]`/`#[sret]`, and safety assertions (`#[no-alloc]`, `#[alloc("once")]`, `jpl` profile).
- **Concurrency, FFI, staging & Wasm:** structured `spawn` with linear task handles, `joinAllOrCancel`, cooperative cancellation, typed C/C++ FFI, compile-time `stage`/`generate do` with quotations `[||]` and splices `$()`, hygienic macros, `rewrite ... justify`, `wasm32` target profiles, and WIT component adapters.

## Known deliberate boundaries

- **Rigid-index constructor matches:** matching a constructor directly on a family applied to rigid indices is rejected; dispatch through `decide` or computed indices instead.
- **No irrefutable `~p` argument patterns:** `~` is reserved for memo bindings (`~ x = e`), which bind plain variables.
- **No local `forall`/`using` signatures:** polymorphic or constrained helpers must be top-level declarations.
- **No superclass entailment bundles:** functions list each required class constraint explicitly (`using (add :: Add a, mul :: Mul a) => ...`).
- **Handler-vtable specialization:** cross-function specialization of concrete effect handler vtables is not yet implemented.

## Repository layout

```text
seed/
  Cargo.toml
  prelude/
    Prelude.cinch             default prelude facade and re-exports
    cinch-base.cinchpkg       base package manifest (55 modules, check order)
    Data/Core.cinch           core inductive types re-exported by home modules
  runtime/                    cinchruntime staticlib + cdylib (heap, RC, bignums, IO, tasks)
  src/
    lexer.rs, layout.rs       lexical analysis and indentation layout
    parser.rs, ast.rs         surface grammar, fixity pass, AST
    check.rs                  type inference, unification, elaboration
    check_terms.rs            term elaboration, letrec, pattern-match compilation
    check_decls.rs            declarations, repr layouts, program assembly
    known.rs, spelling.rs     known nominal identities and diagnostic formatting
    core.rs, rc.rs            Core IR, RC insertion, ownership verification
    specialize.rs, codegen.rs representation specialization, LLVM IR emission
    interface.rs, store.rs    schema 10 interfaces (.cinchif/.cpk) and content store
    driver.rs, main.rs        compilation pipeline and CLI dispatch
    repl.rs, repl_tui.rs      interactive REPL (line loop and TUI)
    lsp.rs, analysis.rs       stdio LSP server and shared analysis layer
    pkg.rs, cinchdoc.rs       package manager and documentation tooling
    jit.rs, native_jit.rs     optional LLVM ORC JIT and native-stage runner
    cxx/                      C++ binding generator and native export builder
  tests/                      integration, runtime, FFI, LSP, REPL, and bootstrap suites
```

## Status and roadmap

For the full feature support matrix and current implementation roadmap, see [`reference/16-support-and-roadmap.md`](../reference/16-support-and-roadmap.md). Standard library coverage against Go is tracked in [`design/stdlib-parity-go.md`](../design/stdlib-parity-go.md), and self-hosting progress is tracked in [`compiler/README.md`](../compiler/README.md) and [`compiler/CHECKPOINT.md`](../compiler/CHECKPOINT.md).
