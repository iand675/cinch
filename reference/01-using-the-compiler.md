# Using the Rust compiler

This guide documents the Rust implementation in `compiler/`. It is the authority for current acceptance and native execution. The separate Cinch bootstrap implementation has a narrower, explicitly tracked frontend; its status does not redefine which language features the Rust compiler supports.

# Installation from source

Build the repository's `compiler/Cargo.toml` with Rust and Cargo:

```sh
cargo build --manifest-path compiler/Cargo.toml
export PATH="$PWD/compiler/target/debug:$PATH"
cinch check examples/compiler/fib.cinch
```

The crate has no third-party Rust dependencies. Native code generation requires a compatible clang on `PATH`, or an executable selected with `CINCH_CLANG`. The driver builds and links the Rust runtime static library from `compiler/runtime/`. Keep that source tree available when running a development build of the compiler. The current website's sample download ZIPs are not compiler distributions.

The checked-in `devenv.nix` still contains older `safelang` script and environment names. The Cargo command above and `CINCH_CLANG` match the current executable. Do not rely on the old shell aliases as the current command specification.

# Commands

| Command | Result |
| --- | --- |
| `cinch check file.cinch` | Load imports, parse, check and emit checked metadata; do not link a native executable. |
| `cinch emit-ir file.cinch -o file.ll` | Check and emit LLVM IR text. |
| `cinch build file.cinch -o program [--package-object package.o]` | Check, emit IR, invoke clang, link the native runtime, and optionally link installed package objects. Project IR excludes the object's modules and declares their symbols. |
| `cinch run file.cinch [args...]` | Compile and execute; return the program's exit status. |
| `cinch repl` | Start the interactive [REPL](26-repl.md) in the current project. |
| `cinch jit file.cinch` | JIT-compile and run through embedded LLVM ORC when the compiler is built with the `jit` feature. |
| `cinch jit --package-object package.o [--package-dependency dep.o]... --call Library.value` | Load installed package objects and their separately produced dependency objects through embedded LLVM ORC, then print one zero-arity unit's value. |
| `cinch pkg add NAME@VERSION` | Install one registry release, its semver dependency closure, and its object/interfaces into the project; update `.cinchlock`. |
| `cinch pkg add PATH_DIR` or `--path DIR` | Install the package at that directory, its path dependencies relative to it, and any registry or git dependencies it declares. |
| `cinch pkg add URL --git-rev REV` or `--git URL --git-rev REV` | Clone a local path, `file://`, or https repository at a revision and install the package at its root. |
| `cinch pkg install` | Install the project `.cinchpkg` dependency declarations, including path and git sources, without publishing. |
| `cinch pkg publish` | Publish the project `.cinchpkg` and module files to an authorized registry namespace. |
| `cinch pkg verify [--slsa]` | Recheck lock entries against installed sources, object hashes, and interfaces; `--slsa` also checks the recorded L1 provenance. |
| `cinch lsp` | Run the [language server](27-language-server.md) over standard input/output. |

`cinch pkg add` uses `--registry URL` (or `CINCH_REGISTRY`), `--project DIR`, and `--token` (or `CINCH_REGISTRY_TOKEN`) for private releases. It materializes sources under `vendor/`, runs the package check and object emission, resolves registry dependencies with exact or semver ranges, and resolves installed modules through the vendor tree. `--path DIR` and `--git URL` select a local source and are mutually exclusive; `--git-rev REV` is required for a git source and can accompany a positional URL. A positional argument that names an existing directory is a path source. Path and git packages are identified by a content root over their manifest and source bytes, never by directory or revision, and the lock records the source kind and the authored path, URL, and revision. `cinch pkg install` installs the project manifest's declared dependencies; `--locked` makes any lock change an error, which reports a dirty path dependency. `publish` uses the same authentication; `verify` is local. See [packages, manifests, and the registry](29-package-management.md).

`cinch jit` is not part of the default build. Build it with `cargo build --features jit`; the build needs LLVM with the C ORC headers and library, and `CINCH_LLVM_CONFIG` can select a versioned `llvm-config`. The command emits the checked program's LLVM IR, loads the runtime shared library, installs the layout table, and calls the generated `main` through LLJIT. A build without the feature reports that the command is unavailable. A native `cinch build`/`cinch run` remains the default path.

`cinch build --package-object package.o` emits project modules only, declares the object's modules, and links the object. `cinch jit --package-object package.o --package-dependency dep.o --call Top.answer` loads installed package objects and their dependency objects. It checks every package execution `.cinchif` (`build/package.cinchif`, or legacy `package.abi.json`) against this host and the object bytes, verifies installed `.cinchif` files, adds dependency objects first with `LLVMOrcLLJITAddObjectFile`, registers the root object's layout prefix and reclamation mode, and prints one zero-arity unit. Package objects define only their own modules and generated primitive helpers. JIT and native `build` both consume the same object format; JIT loads dependency objects first, while `build` passes them to the linker and emits only project modules.

`-o` takes one path and is meaningful for `emit-ir` and `build`. For reproducible scripts, pass it explicitly. `--rebuild` recomputes a package-store entry that fails verification instead of stopping on the verification error. It is not an optimizer flag or a command to disable all incremental work.

For `run`, `--` ends compiler argument interpretation. Use `cinch run Main.cinch -- --rebuild` to pass the literal argument `--rebuild` to the program. Program arguments are available through `process-args` in `Effect.Process`; they exclude the executable name. Do not assume shell tokenization is performed again inside the program.

There is currently no `--target`, `-O2`, or `--watch`, and `build` and `run` accept no dependency-object flag or arbitrary foreign-object option. For the current package format, local store, and registry prototype, see [packages, manifests, and the registry](29-package-management.md). Unsupported command names print usage and exit 2. Omitting `-o`'s path is also a usage error. `--help` is not a distinct successful command; invoking an unknown command takes the usage path.

# Entry points and outcomes

A pure executable can use `main :: Int`. An effectful executable uses `main :: Unit ->{effects} Int`, with the admitted effects in its row. The generated entry glue invokes it and converts the result to a process exit status. Returning an integer does not print it.

Compiler errors exit 1 and print a diagnostic to stderr. A `run` invocation returns the executed program's status, so an exit 1 can also be an application result. Run `check` separately when distinguishing these cases. Runtime exhaustion is exit 73; other defensive boundary failures have their own diagnostics, covered in [runtime operation](13-concurrency-and-runtime.md).

# Files produced

The project build directory contains `.cinchif` interfaces and an entry manifest such as `Main.manifest.json`. Metadata includes source identities, dependency contracts, assumptions, and selected lowering facts. `emit-ir` writes textual LLVM IR; `build` writes a native executable. Cache contents are verified rather than trusted merely because a path exists. See [artifacts and incremental work](18-artifacts-and-incremental.md).

<span class="support-tag support-tag--planned">Coming soon</span> **portable distribution and target workflows.** Source builds and the current native commands are usable today. Target-profile APIs, component adapters, and runtime update mechanisms do not yet constitute a complete CLI for distributing cross-target bundles.

Implementation: `compiler/src/main.rs`, `compiler/src/driver.rs`; executable tests: `argv.rs`, `modules.rs`, `package_store.rs` in `compiler/tests/`.
