# Cinch

Cinch is a strict functional systems language with dependent and refinement types, row-typed effects, linear ownership, precise reference counting (no garbage collector), and native LLVM and WebAssembly compilation.

The repository contains both the **Rust bootstrap compiler** (`seed/`) and the **self-hosted Cinch compiler** (`compiler/`), together with the runtime (`seed/runtime/`), standard library (`seed/prelude/`, 331 modules), editor tooling (`tooling/`), package registry prototype (`registry/`), and documentation site (`website/`, published at [cinch.systems](https://cinch.systems)).

## Highlights

- **Dependent & refinement types:** stratified universes (`Type 0 : Type 1 : ...`, `Prop`), dependent functions and pairs (`Sigma`), indexed inductive families (`Vec`, `Fin`, `Dec`), zero-cost refinements (`NonEmpty`, `NonZero`, `InRange`) backed by table-driven implication, difference logic, and linear Farkas solvers, plus predicative Rank-N polymorphism.
- **Memory safety without a GC:** path-sensitive `%0`/`%1`/`%aff`/`%many` usage checking, region-scoped borrows (`Ref r a`, `Mut r a`), Perceus precise reference counting with borrow inference and in-place reuse, ABI 10 self-describing block headers, and stack allocation of non-escaping blocks.
- **Row-typed effects & lexical capabilities:** algebraic effect rows, one-shot `handle`/`resume`, lexically passed capabilities (`using (io :: Io r)`, `with (io = ...) in`), and selectable `do` sequencing modules.
- **Concurrency & data parallelism:** stackful green threads (`sched.rs`), linear `Task` handles (`withScope`, `spawn`, `joinAllOrCancel`), native `MVar`, TL2 `Effect.Stm`, pure parallelism, and data-parallel arrays with Typed Array IR fusion and an Apple Metal GPU backend.
- **Native & Wasm deployment:** LLVM IR codegen with selective `tailcc` guaranteed tail calls, contification, call-pattern specialization, constructed product results, C/C++ and Godot GDExtension interop (`cinch cxx`), `wasm32` / WIT targets, and compile-time safety assertions (`#[no-alloc]`, `#[profile "jpl"]`).
- **Self-hosting:** the Cinch compiler in `compiler/` (65 `Bootstrap.*.cinch` modules) compiles itself through a four-stage bootstrap ladder to a byte-for-byte Stage 2/Stage 3 fixed point on interface schema 10 and runtime ABI 10.

## Quick start

Enter the [devenv](https://devenv.sh) shell for Rust, LLVM/`clang`, Tree-sitter, and Node 24:

```sh
devenv shell
```

Build the Rust compiler and run an example:

```sh
cargo build --manifest-path seed/Cargo.toml --release --bin cinch
seed/target/release/cinch run examples/compiler/fib.cinch
```

Common workflows:

```sh
# Check, build, or inspect Core/LLVM IR
cinch check path/to/Main.cinch
cinch build path/to/Main.cinch -o main
cinch core emit path/to/Main.cinch
cinch repl
cinch lsp

# Run the Rust compiler and runtime test suites
cargo test --manifest-path seed/Cargo.toml --lib
cargo test --manifest-path seed/runtime/Cargo.toml

# Run the four-stage self-hosting bootstrap ladder
tooling/bootstrap.sh check
```

Standalone Stage 3 nightly binaries for Linux (`x86_64`, `aarch64`) and macOS (Apple Silicon) are published via GitHub prereleases and documented in [`tooling/nightly-release.md`](tooling/nightly-release.md).

## Documentation

| Guide | Contents |
| --- | --- |
| [**Documentation hub**](documentation.md) | Entry point for readers (served at `/documentation/`) |
| [**Language tutorial**](learn/00-tutorial.md) | 22-chapter hands-on tutorial with checked examples and exercises |
| [**Compiler user's guide**](reference/00-guide.md) | 29-chapter reference manual covering CLI, syntax, types, effects, FFI, layout, REPL, LSP, and packages |
| [**Support matrix & roadmap**](reference/16-support-and-roadmap.md) | Per-feature implementation status in `seed/` |
| [**Standard library reference**](library/00-index.md) | Generated API reference for all checked stdlib modules (published with Hoogle-style search at `/packages/stdlib/`) |
| [**Specifications**](specifications/) | Normative language, grammar, primitive, FFI, concurrency, Wasm, and runtime evolution specs |
| [**Design architecture**](design/README.md) | Consolidated architecture docs covering type system, reclamation, concurrency, codegen, self-hosting, and roadmap |

## Repository layout

| Path | Description |
| --- | --- |
| [`seed/`](seed/README.md) | Rust bootstrap compiler (Stage 0), CLI (`repl`, `lsp`, `doc`, `pkg`, `jit`, `cxx`), `cinchruntime` (`seed/runtime/`), and standard library sources (`seed/prelude/`) |
| [`compiler/`](compiler/README.md) | Self-hosted Cinch compiler (`Bootstrap.*.cinch`), bootstrap pins (`pins/`), conformance log (`CONFORMANCE.md`), and verification evidence (`evidence/`) |
| [`tooling/`](tooling/README.md) | Bootstrap ladder driver, deterministic IR sharder, modular build/profiling scripts, Nix cache derivations ([`tooling/nix/`](tooling/nix/README.md)), Tree-sitter grammars, and editor integrations |
| [`registry/`](registry/README.md) | Zero-dependency Node 24 + SQLite package registry prototype with public/private namespaces, DSSE statements, and SLSA verification |
| [`website/`](website/README.md) | Astro + Starlight documentation site, package catalogue, nightly downloads, and live WebAssembly Game of Life demo |
| [`examples/`](examples/catalogue.md) | Language examples, C/C++ interop ([`examples/cxx/`](examples/cxx/README.md)), Godot GDExtension ([`examples/godot/`](examples/godot/README.md)), and Metal GPU pipeline ([`examples/gpu-pipeline/`](examples/gpu-pipeline/README.md)) |
| [`benchmarks/`](benchmarks/stdlib/README.md) | Standard library and array/Futhark benchmark suites |
| [`learn/`](learn/00-tutorial.md), [`reference/`](reference/00-guide.md), [`library/`](library/00-index.md) | Published tutorial, reference manual, and generated stdlib API docs |
| [`specifications/`](specifications/), [`design/`](design/README.md) | Normative specifications and consolidated architecture docs |
