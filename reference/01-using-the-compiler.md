# Using the Rust compiler

The `compiler/` crate builds the `cinch` executable. It is the authority for what the language accepts and how it is executed natively. Every command, flag, exit status, file, and diagnostic on this page is taken from a debug build of that executable. The Cinch-written bootstrap frontend is tracked separately in [self-hosting](../design/self-hosting.md); its narrower scope does not redefine what this compiler does.

# Build and install

```sh
cargo build --manifest-path compiler/Cargo.toml
export PATH="$PWD/compiler/target/debug:$PATH"
cinch check examples/compiler/fib.cinch
```

The compiler crate declares Rust dependencies in `compiler/Cargo.toml` (`crossterm`, `ratatui`, `regex`, `serde`, `serde_json`, `sha2`, `shlex`, `tempfile`, `tree-sitter`, `tree-sitter-language`, `unicode-width`), so the first build needs those crates available. The native runtime is a separate crate at `compiler/runtime` with no dependencies: `build` and `run` build it with `cargo` on demand and link `compiler/runtime/target/debug/libcinchruntime.a`, rebuilding when any runtime source is newer than the archive. The path is resolved from the compiler source directory recorded when `cinch` was built, so keep the binary and that source tree together, and keep `cargo` on `PATH` for any native command. The download ZIPs offered elsewhere on this site are not compiler distributions.

# The usage block

`cinch --help`, `cinch` with no arguments, and any command name the driver does not recognize all print this block to stderr and exit 2. It is not a successful command.

```text
cinch: bootstrap compiler for the safe language design

usage:
  cinch check <file.cinch> [--rebuild]         typecheck only
  cinch emit-ir <file.cinch> [-o <out.ll>] [--rebuild]
  cinch emit-object <library.cinch> [-o <out.o>] [--object-roots <roots.json>] [--rebuild]
  cinch build <file.cinch> [-o <out>] [--rebuild]
  cinch run <file.cinch> [args...] [--rebuild] compile and run; exit code is the program's
  cinch lsp                        run the language server over stdio
  cinch doc extract <file.cinch> [--iface <cinchif>] [--legacy] [--out <json>]
  cinch doc render <file.cinch> [--iface <cinchif>] [--legacy] [--plain] [--out <markdown>]
  cinch doc lookup <symbol> [--dir <root>] [--legacy] [--json]
  cinch doc index <root> [--legacy] [--out <json>]
  cinch doc test <root...> [--compiler <cinch>] [--legacy] [--match <text>]
  cinch doc markdown [<file.md>] [--out <html>]  highlight cinch fences on stdin or a file
  cinch doc highlight <file.cinch> [--out <html>]
  cinch repl                       interactive session in the current project
  cinch jit <file.cinch>           JIT-compile and run (requires the jit feature)
  cinch jit --package-object <package.o> [--package-dependency <dep.o>]... --call <symbol>
                                   load an installed package object and its
                                   dependency objects, then call one zero-arity
                                   unit (requires the jit feature)
  cinch pkg add NAME@VERSION | PATH_DIR | URL --git-rev REV
                                   install a registry, path, or git package
  cinch pkg add --path DIR | --git URL --git-rev REV
                                   install a local or git source explicitly
  cinch pkg install [--locked]     install the project .cinchpkg dependencies

flags:
  -o <path>   the output path (emit-ir, emit-object and build)
  --object-roots <path>  concrete library variants demanded by a consumer
  --package-object <path>  link an installed package object in build/run
  --rebuild   recompute a package-store entry that fails verification
              instead of failing the compile with the reason
  --target <triple>  the target to build for (this compiler's own triple)
  --target-version <version>  the platform version package conditions see
  --print-link-plan  report every contribution to the final link
  --allow-raw-link NAME  consent to one package's raw link words
  --          everything after it is a program argument (run), so an
              argument that looks like a flag can still be passed

`run` forwards every word after the entry file to the program's own
command line, which the program reads with `process-args`
(Effect.Process).
```

# Commands

Every command takes one entry source file, named on the command line. Imports resolve beside the entry file first, then against the embedded base family.

| Command | Reads | Writes | Exit |
| --- | --- | --- | --- |
| `check` | entry source, the sources it imports, the embedded base family | `build/` artifacts beside the entry | 0 on success, 1 on a diagnostic |
| `emit-ir` | same | the same artifacts, then LLVM IR text to `-o` (or stdout) | 0, 1 |
| `emit-object` | same | the same artifacts, then a closed library object `<out>.o` with `<out>.ll` and `<out>.abi.json` companions | 0, 1 |
| `build` | same | the same artifacts, `<out>` native executable, and `<out>.ll` | 0, 1 |
| `run` | same | the same artifacts, then a temporary directory under the system temp root holding `program.ll` and `program`, removed after the run | the executed program's status |
| `repl` | the current directory as a project | no files | see [REPL](26-repl.md) |
| `lsp` | framed JSON-RPC on stdin | framed JSON-RPC on stdout | 0 on clean end of input |
| `doc extract`, `render`, `lookup`, `index`, `test`, `highlight`, `markdown` | sources, interfaces, or Markdown | JSON, Markdown, or HTML as asked | 0, 1, 2 (see [documentation](28-documentation.md)) |
| `jit` | a source file, or a package object plus dependency objects | nothing on disk | 2 in a build without the feature |
| `pkg add`, `install`, `publish`, `verify` | `.cinchpkg`, `.cinchlock`, `vendor/` | the same | see [packages](29-package-management.md) |

`build` and `emit-object` without `-o` name the output after the entry path with its extension removed: `cinch build Main.cinch` writes `Main` and `Main.ll` in the current directory. The `build/` directory always sits beside the entry source, whatever `-o` says. `emit-object` refuses an output path that collides with its companions:

```sh
cinch emit-object Lib.cinch -o Lib.ll
cinch: emit-object output must differ from its .ll and .abi.json companions
```

`emit-ir`, `build`, and `run` require an entry point. `check` and `emit-object` do not, which is why a library is checked and emitted as an object but not linked into an executable:

```sh
cinch build Lib.cinch -o prog
cinch: this program has no `main`; `emit-ir`, `build`, and `run` need `main :: Int` in the entry module
```

The top-level usage block lists `pkg add` and `pkg install` only. `cinch pkg` with no arguments prints the longer list, which also has `publish` and `verify`:

```text
cinch pkg: usage:
  cinch pkg add NAME@VERSION [--registry URL] [--project DIR] [--token TOKEN]
  cinch pkg add PATH_DIR | --path DIR [--project DIR]
  cinch pkg add URL --git-rev REV | --git URL --git-rev REV [--project DIR]
  cinch pkg install [--project DIR] [--registry URL] [--token TOKEN] [--locked]
  cinch pkg publish [--registry URL] [--project DIR] [--token TOKEN] [--visibility public|private]
  cinch pkg verify [--slsa] [--project DIR]
```

The first word of `cinch doc` with no further argument prints `cinch doc: expected extract, render, lookup, index, test, or highlight` and exits 2. `doc markdown` is accepted even though that message omits it.

# Flags

| Flag | Accepted by | Effect |
| --- | --- | --- |
| `-o <path>` | `emit-ir`, `emit-object`, `build` | Sets the output path. `check` and `run` parse it and never use it, so `cinch check Main.cinch -o ignored.txt` exits 0 and creates no file. The path is written without any check that it is not the entry source; `cinch emit-ir Main.cinch -o Main.cinch` overwrites the source with IR. |
| `--object-roots <path>` | all modes parse it, `emit-object` uses it | Names a JSON document of concrete library variants. An empty document is accepted everywhere and does nothing; a document with a row in any other mode is refused. |
| `--package-object <path>` | `build`, `run` | Links one separately produced package object. |
| `--rebuild` | `check`, `emit-ir`, `emit-object`, `build`, `run` | Forces a rewrite of this compile's package-store entry. |
| `--target <triple>` | `check`, `emit-ir`, `emit-object`, `build`, `run` | Sets the target. Only this compiler's own triple is accepted, because a cross compile needs a runtime built for the target. |
| `--target-version <version>` | the same modes | States the target platform version that a package manifest's `os_version` conditions are evaluated against. A condition with no version refuses rather than reading the host's. |
| `--print-link-plan` | the same modes | Reports every contribution to the final link with the package that asked for it, before anything is linked. |
| `--allow-raw-link <name>` | the same modes | Consents to one package's raw link words, the flag form of the root manifest's `raw_link`. |
| `--` | all | Everything after it is a program argument, taken verbatim. |

The object-roots document is JSON with `schema` set to 1 and up to three arrays, `units`, `dictionaries`, and `methods`. A unit or dictionary row has `symbol` and `classes`, and a method row has `class`, `slot`, and `own`. A class vector spells `R`, `O`, or `T` per class. The document is read by every mode, and only `emit-object` acts on a non-empty one.

```sh
echo '{}' > roots.json
cinch check Main.cinch --object-roots roots.json
cinch: object roots require schema 1 and only units/dictionaries/methods arrays
```

```sh
echo '{"schema":1,"units":[]}' > roots.json
cinch check Main.cinch --object-roots roots.json        # exit 0, nothing to demand
echo '{"schema":1,"units":[{"symbol":"Lib.answer","classes":""}]}' > roots1.json
cinch check Main.cinch --object-roots roots1.json
cinch: explicit object roots require emit-object mode
```

```sh
echo '{"schema":1,"units":[{"symbol":"Lib.answer","classes":"X"}]}' > roots2.json
cinch emit-object Main.cinch -o oo.o --object-roots roots2.json
cinch: object root vectors contain only R/O/T
cinch emit-object Main.cinch -o oo.o --object-roots roots1.json
cinch: unknown object unit root Lib.answer
```

The last error names a root that the module does not export, which is how a consumer's demand for a concrete representation variant is checked.

`--package-object` resolves the object's modules from `<dir>/build/package.cinchif` when that exists and from `<out>.abi.json` otherwise, and verifies the object bytes against the ABI's `object_content_hash` before linking:

```sh
cinch build Use.cinch -o prog --package-object Lib.o
cinch build Use.cinch -o prog2 --package-object Lib-bad.o
cinch: Lib-bad.o: object content hash does not match package.abi.json
```

Only `build` and `run` treat `--package-object` as a flag. In the other modes the word falls through to the positional rule below, so `cinch check Main.cinch --package-object nope.o` exits 0 and links nothing.

An object whose ABI file is missing reports the ABI path it looked for:

```sh
cinch build Main.cinch -o prog --package-object /tmp/nope.o
cinch: /tmp/nope.abi.json: No such file or directory (os error 2)
```

Only `--package-object` and `--object-roots` are positional in that sense. A word the driver does not recognize is not a flag error: before the entry file it is taken as the entry path, and after the entry file it becomes a program argument. Both outcomes are worth knowing, because neither says "unknown flag":

```sh
cinch check -O2 Main.cinch
cinch: -O2: No such file or directory (os error 2)
```

```sh
cinch check Main.cinch -O2             # exits 0, the word is a program argument nobody reads
cinch run Main.cinch -O2               # the program receives one argument
cinch build Main.cinch -o prog --target wasm32   # exits 0, and links a native binary
```

A flag that expects a value and does not get one is a usage error, exit 2:

```sh
cinch check Main.cinch --object-roots
cinch: --object-roots needs a path
cinch emit-object Main.cinch -o
cinch: -o needs a path
```

# Exit statuses

| Code | Meaning |
| --- | --- |
| 0 | The command did what it says: the program typechecked, the IR or object was written, the executable was linked, or the program exited 0. |
| 1 | A compile diagnostic, an unreadable input, a link failure, or the program's own nonzero status under `run`. |
| 2 | A usage error: no arguments, an unknown command, `--help`, a flag missing its value, an unparsable object-roots document, or a `jit` or `pkg` refusal. |
| program status | Under `run` the compiler exits with the executed program's status, so a `main` that returns 7 produces 7. |

`run` therefore folds two meanings into one number. Run `check` first when you need to know which one you got.

# Diagnostics

Two shapes appear, and the position is the difference between them.

Lexer, layout, and parser failures name the file first, then the position:

```text
cinch: Main.cinch: 2:1: tabs are rejected in indentation
cinch: Main.cinch: 2:3: expected a declaration, found Let
cinch: Main.cinch: 3:7: unknown escape sequence
```

Checker failures and every later stage place the position first and put the file in parentheses at the end. This program is refused by the coverage check:

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

Driver failures carry no position at all:

```text
cinch: Nope.cinch: No such file or directory (os error 2)
cinch: Main.cinch: no `module` declaration
```

All of these go to stderr and exit 1.

# What one compile writes

Each invocation writes the same artifact set beside the entry source, and the set does not depend on the mode: `check`, `emit-ir`, and `build` of one program produce byte-identical `build/` trees.

| Path | Contents |
| --- | --- |
| `build/<Module>.cinchif` | One checked interface per module in the program, including every module of the embedded base family, so a one-line program writes about fifty of them. |
| `build/<Entry>.manifest.json` | The artifact manifest: `modules`, `target`, `assumptions`, `prims`, `laws`, `safety`, `temporal`, `trust`, `lowering`, `staging`, `specialization`, `tail_calls`, `literal_folds`, `data_imports`, `wasm`, `runtime_abi`, `entry`. |
| `build/<Entry>.stage.snap` | The captured compile-time staging inputs, tab separated, one row per input. |

`<Entry>` is the entry file's stem, so `main.cinch` writes `build/main.manifest.json`.

Nothing in that set is read back to skip work. The next invocation re-parses, re-checks, and rewrites every file in it, with one exception: `build/<Entry>.stage.snap` is read at the start of the next compile and supplies the previous stage inputs, which is what `CINCH_STAGE_MODE=replay` replays. Two consecutive `check` runs of an unchanged program move the modification time of `Main.cinchif`, `Data.Core.cinchif`, and `Main.stage.snap` every time.

# The package store

Every compile also files the base family's interfaces in a per-user store, outside the project. The root is `CINCH_STORE` when that is set. Otherwise it is `$HOME/Library/Caches/cinch/store` on macOS, `$XDG_CACHE_HOME/cinch/store` when `XDG_CACHE_HOME` is set on other platforms, and `$HOME/.cache/cinch/store` as the remaining fallback.

```text
<root>/cinch-base/0.1.0/<key>/Data.Core.cinchif      the interface
<root>/cinch-base/0.1.0/<key>/<Module>.o             an object, once a producer exists
<root>/cinch-base/0.1.0/<key>/<Module>.ir            a module IR form, once a producer exists
<root>/cinch-base/0.1.0/<key>/store.json             the entry manifest
```

`<key>` is the decimal package key over the base manifest and sources plus the profile and the adapter set, so a change to any embedded source moves it. `store.json` records `key`, `package`, `version`, `compiler` (`0.1.0` here), `language` (`draft-0.2+amendments`), `iface_schema`, `runtime_abi`, `profile` (`native64`), `store_schema`, `kinds`, and the per-module `modules` rows with their `iface_hash` and `kinds`. Today's compile path writes interfaces only, so `kinds` is `["interface"]` and `bodies` is empty for every module.

A write stages the whole set in a dotted `.staged` directory inside the package's store directory (`.<key>.<pid>.<n>.staged`) and swaps it in with one rename, so a reader sees the previous set, the new set, or nothing. An ordinary compile never fails over the entry: it keeps an entry that is exactly this compile's set and rewrites anything else (a truncated artifact, an aborted write, a differing identity or kind set).

```sh
mkdir -p /tmp/store-demo
CINCH_STORE=/tmp/store-demo cinch check Main.cinch     # writes the entry, exit 0
CINCH_STORE=/tmp/store-demo cinch check Main.cinch     # entry intact, left alone
CINCH_STORE=/tmp/store-demo cinch check Main.cinch --rebuild   # entry rewritten
```

`--rebuild` is the rewrite switch, not an optimizer flag. Corrupting an interface inside an existing entry and compiling again repairs it and still exits 0:

```sh
printf garbage > /tmp/store-demo/cinch-base/0.1.0/*/Data.Core.cinchif
CINCH_STORE=/tmp/store-demo cinch check Main.cinch
```

The compile path calls `Store::load` for one decision: whether the entry is exactly this compile's artifact set, in which case the entry is left alone, or anything else, in which case it is rewritten. A load failure is swallowed there rather than reported, because a corrupt cache must not fail a build. The loader's named errors (missing, unreadable, truncated, or mismatched artifact, mismatched key) reach callers that are about to rely on the artifacts; no CLI command consumes the store that way, and `compiler/tests/package_store.rs` exercises the loader directly. Nothing in the store shortens a compile yet: a warm compile cannot skip the base family, because the checker needs its checked state, not the rendered contracts an interface carries.

# Incremental behaviour

Each `cinch` invocation starts from sources and recompiles the program it is given. There is no watch mode, no daemon, and no persistent compiler process between shell invocations, so changing one module rechecks the program rather than that module.

The declaration-level machinery exists in the library: `compiler/src/incremental.rs` holds an item graph with dirty closures, a recompilation session that reports `rechecked`, `retyped`, and `kept` declarations, and a declaration-keyed object cache. `compiler/tests/incremental.rs` drives it. The command line does not. The editor front ends reach the same analysis layer in process: [REPL](26-repl.md) and the [language server](27-language-server.md) keep their state for the life of the session, and the [artifacts page](18-artifacts-and-incremental.md) covers identities, invalidation, and the design goals.

# Environment variables

| Variable | Read by | Effect |
| --- | --- | --- |
| `CINCH_CLANG` | the linker step of `build` and `run` | The clang executable to invoke instead of `clang` on `PATH`. |
| `CINCH_STORE` | every compile | The package-store root, replacing the per-user default. |
| `CINCH_STAGE_MODE` | every compile | `refresh` (default), `replay`, or `live`. Anything else falls back to `refresh`. It decides how compile-time staging inputs are observed, so a program with no staged inputs compiles identically under all three. |
| `CINCH_RECLAIM` | every compile | Reclamation is inserted unless this is `0`. `CINCH_RECLAIM=0 cinch emit-ir Main.cinch -o norecl.ll` emits 148438 lines of IR where the default emits 333682 for a program whose body is `main = 0`, and the `cinch_drop_mt`, `cinch_dup_mt`, `cinch_free`, and `cinch_reclaim_enable` calls are absent. |
| `CINCH_SCHEMA_TOOL` | every compile | The introspection tool that `Stage.withDatabase` runs. The driver reads it and hands it to the staging checker; `compiler/tests/staging.rs::schema_build_identity_records_the_tool_not_the_credential` drives it through the driver and asserts the credential never reaches the manifest. |

Clang is only needed to link. `check` and `emit-ir` succeed with a broken clang configured; `build` and `run` fail:

```sh
CINCH_CLANG=/usr/bin/false cinch emit-ir Main.cinch -o m.ll   # exit 0
CINCH_CLANG=/usr/bin/false cinch build Main.cinch -o prog
cinch: /usr/bin/false failed to link prog.ll
CINCH_CLANG=/nonexistent/clang cinch build Main.cinch -o prog
cinch: failed to run /nonexistent/clang: No such file or directory (os error 2)
```

The first message carries clang's own stderr after the source path when clang printed any. `build` writes `<out>.ll` first, so a link failure leaves the IR behind for inspection.

The produced program reads its own variables at run time: `CINCH_HEAP_LIMIT`, `CINCH_STACK_SIZE`, `CINCH_ALLOC_STATS`, `CINCH_LEAK_CHECK`, and `CINCH_RC_DEBUG` are documented with their output formats in [concurrency and runtime](13-concurrency-and-runtime.md), and they pass through `cinch run` unchanged:

```sh
CINCH_ALLOC_STATS=1 cinch run Main.cinch
cinch-alloc: blocks=6 bytes=282 peak=218 header-blocks=0 header-bytes=0 immortal=0 freed=0 live=0
CINCH_LEAK_CHECK=1 cinch run Main.cinch
cinch-leak: live=0 immortal=0 leaked=0
```

`pkg` reads `CINCH_REGISTRY` and `CINCH_REGISTRY_TOKEN` in place of its flags, and a publish with neither is refused: `cinch pkg: publish needs --token or CINCH_REGISTRY_TOKEN`. The `jit` feature reads `CINCH_LLVM_CONFIG` when it is built.

# Entry points

A pure executable declares `main :: Int`. An effectful executable declares `main :: Unit ->{effects} Int` and constructs its capabilities, typically with `newIO ()`. The generated entry glue calls it and converts the result to the process exit status. Returning an integer does not print it; use `Effect.Io` for output.

<!-- cinch:run exit=0 -->
```cinch
module Main where
  fib :: Int -> Int
  fib n = if n == 0 then 0 else if n == 1 then 1 else fib (n - 1) + fib (n - 2)

  main :: Int
  main = if fib 10 == 55 then 0 else 1
```

An effectful entry point receives its command line from `process-args`, which reports the words after the program name. The words come from `cinch run` verbatim, spaces included, and `--` protects a word that looks like a compiler flag:

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

Running that program with three words exits 3, with none exits 0, and with `-- --rebuild -x` exits 2, while a bare `--rebuild` is consumed by the compiler and the program sees nothing.

# Not available

There is no `--target` flag, no optimization-level flag, no `--watch`, and no foreign-object flag on `build` or `run`; `build` and `run` take dependency objects only through `--package-object`. `-o` takes one path. `jit` is not part of the default build: this build prints `cinch: this build was compiled without the jit feature` and exits 2, and `cargo build --features jit` is what enables it.

<span class="support-tag support-tag--planned">Coming soon</span> **portable distribution and target workflows.** Source builds and the native commands above are usable today. Target-profile APIs, component adapters, and runtime update mechanisms do not yet form a CLI for distributing cross-target bundles.

Implementation: `compiler/src/main.rs` (argument parsing and the usage block), `compiler/src/driver.rs` (modes, artifacts, linking), `compiler/src/store.rs` (the package store), `compiler/src/incremental.rs` (the session API). Executable suites: `argv.rs`, `modules.rs`, `package_store.rs`, `incremental.rs`, `staging.rs`.
