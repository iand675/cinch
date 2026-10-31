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
| `cinch build file.cinch -o program` | Check, emit IR, invoke clang, and link the native runtime. |
| `cinch run file.cinch [args...]` | Compile and execute; return the program's exit status. |
| `cinch lsp` | Run the [language server](27-language-server.md) over standard input/output. |

`-o` takes one path and is meaningful for `emit-ir` and `build`. For reproducible scripts, pass it explicitly. `--rebuild` recomputes a package-store entry that fails verification instead of stopping on the verification error. It is not an optimizer flag or a command to disable all incremental work.

For `run`, `--` ends compiler argument interpretation. Use `cinch run Main.cinch -- --rebuild` to pass the literal argument `--rebuild` to the program. Program arguments are available through `process-args` in `Effect.Process`; they exclude the executable name. Do not assume shell tokenization is performed again inside the program.

There is currently no `--target`, `-O2`, `--watch`, `repl`, package-install, or arbitrary foreign-object option. Unsupported command names print usage and exit 2. Omitting `-o`'s path is also a usage error. `--help` is not a distinct successful command; invoking an unknown command takes the usage path.

# Entry points and outcomes

A pure executable can use `main :: Int`. An effectful executable uses `main :: Unit ->{effects} Int`, with the admitted effects in its row. The generated entry glue invokes it and converts the result to a process exit status. Returning an integer does not print it.

Compiler errors exit 1 and print a diagnostic to stderr. A `run` invocation returns the executed program's status, so an exit 1 can also be an application result. Run `check` separately when distinguishing these cases. Runtime exhaustion is exit 73; other defensive boundary failures have their own diagnostics, covered in [runtime operation](13-concurrency-and-runtime.md).

# Files produced

The project build directory contains `.cinchif` interfaces and an entry manifest such as `Main.manifest.json`. Metadata includes source identities, dependency contracts, assumptions, and selected lowering facts. `emit-ir` writes textual LLVM IR; `build` writes a native executable. Cache contents are verified rather than trusted merely because a path exists. See [artifacts and incremental work](18-artifacts-and-incremental.md).

<span class="support-tag support-tag--planned">Coming soon</span> **portable distribution and target workflows.** Source builds and the current native commands are usable today. Target-profile APIs, component adapters, and runtime update mechanisms do not yet constitute a complete CLI for distributing cross-target bundles.

Implementation: `compiler/src/main.rs`, `compiler/src/driver.rs`; executable tests: `argv.rs`, `modules.rs`, `package_store.rs` in `compiler/tests/`.
