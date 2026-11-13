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
  cinch core emit <file.cinch> [--stage=<name>] [--target <triple>] [-o <out.core>]
                                   write the Core dump codegen consumes
  cinch core verify <file.core>    parse and re-check a Core dump
  cinch core diff <a.core> <b.core>
                                   report the first structural divergence
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
  --opt-level <0|1|2|3>  clang's optimization level for build, run and
              emit-object (default 0 for build and run, 2 for emit-object)
  --dump-core <stage>  also write build/<entry>.core at that pipeline point:
              checked (solved and elaborated are aliases), owned, or lowered
              (the default stage, and the exact code generation input); a
              non-default stage writes build/<entry>.<stage>.core, and
              `check` dumps the point it reaches, marked in @stage
  --rebuild   recompute a package-store entry that fails verification
              instead of failing the compile with the reason
  --target <triple>  the target to compile for: aarch64-apple-darwin,
              x86_64-apple-darwin, x86_64-unknown-linux-gnu, or
              aarch64-unknown-linux-gnu (check and emit-ir work for all of
              them here; build needs the target's runtime archive)
  --runtime-archive <path>  the cinchruntime archive built for a foreign
              --target, which a cross build links
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
| `build` | same | the same artifacts, `<out>` executable for the target, and `<out>.ll` | 0, 1 |
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
| `--opt-level <0\|1\|2\|3>` | `build`, `run`, `emit-object` | The optimization level clang assembles the emitted IR at, passed as `-O<n>`. `build` and `run` default to 0, which compiles fastest; `emit-object` defaults to 2, because a library object is assembled once and linked into every consumer. Reference counting is emitted as calls to small helpers private to each module, which an optimizing level inlines. Any other value, or the flag in another mode, exits 2. |
| `--rebuild` | `check`, `emit-ir`, `emit-object`, `build`, `run` | Forces a rewrite of this compile's package-store entry. |
| `--target <triple>` | `check`, `emit-ir`, `emit-object`, `build`, `run` | Sets the target. Five triples are accepted: `aarch64-apple-darwin`, `x86_64-apple-darwin`, `x86_64-unknown-linux-gnu`, `aarch64-unknown-linux-gnu`, and `wasm32-unknown-unknown`. Any of them checks and emits IR on any host, and the emitted module states that target's `target triple` and `target datalayout`, because layout and `when arch` body selection follow the target. The profile's operating system, architecture, C library, word size, and byte order come from the triple, never from the host, and the profile is named after the triple so the package store key and the artifact manifest identify the same target on every host. Any other triple is refused with the list of the five. A cross `build` needs `--runtime-archive`, except for `wasm32-unknown-unknown`, whose freestanding runtime archive this tree builds on demand (`cargo build --target wasm32-unknown-unknown` in `compiler/runtime` produces the same file). `run` executes on the host: a native target runs its executable directly, and `wasm32-unknown-unknown` runs its module under Node, so a program's exit code means the same thing under both. Every other foreign target refuses `run`, because the host cannot start its binary. |
| `--runtime-archive <path>` | `build`, `run` | The `libcinchruntime.a` built for a foreign `--target`, which the cross build links. The archive is read before the link: a path that does not exist, a file that is not a static archive, and an archive whose objects belong to another architecture are each refused by name (a Wasm object is read as `wasm32`, so a host archive offered for the wasm32 target is refused before the link rather than inside it). Without it a cross `build` fails naming the flag and the triple, except for `wasm32-unknown-unknown`, whose archive the build produces itself. A link for a target other than the host also passes `-target` to the assembler and linker, so the cross toolchain has to be one that produces that target's objects (`CINCH_CLANG` names it). |
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
cinch build Main.cinch -o prog --target wasm32   # exits 1, refused by name
cinch: unknown target `wasm32`: cross compilation supports exactly these triples: aarch64-apple-darwin, x86_64-apple-darwin, x86_64-unknown-linux-gnu, aarch64-unknown-linux-gnu, wasm32-unknown-unknown
```

`--target` is a flag, so it is consumed wherever it appears, and `wasm32` on its own is not one of the five triples: the target needs its operating system and C library components named, and `wasm32-unknown-unknown` is the freestanding one.

A `build` for `wasm32-unknown-unknown` produces a module rather than a host executable, and `run` executes it under Node:

```sh
cinch build --target wasm32-unknown-unknown Main.cinch -o prog.wasm
cinch run --target wasm32-unknown-unknown Main.cinch    # exits with the program's `main` result
```

Two tools beyond a native toolchain are needed, and each refusal names what is missing: clang 20 or newer (the emitted IR carries `range(...)` attributes older LLVM rejects as `expected type`), and `wasm-ld` (found beside clang, then on `PATH`, or named by `CINCH_WASM_LD`).

A flag that expects a value and does not get one is a usage error, exit 2:

```sh
cinch check Main.cinch --object-roots
cinch: --object-roots needs a path
cinch emit-object Main.cinch -o
cinch: -o needs a path
```

# Core output

Core is the annotated text between checking and code generation: the program code generation consumes, printed so a wrong answer, a seed/self-host divergence, or a performance change can be read out of an artifact instead of out of the compiler (`design/core-output.md`).

```sh
cinch core emit Main.cinch                     # build/Main.core, the lowered stage
cinch core emit Main.cinch --stage=owned -o Main.owned.core
cinch core verify build/Main.core
cinch core diff build/Main.core build/Main.owned.core
```

`core emit` runs the compile pipeline through the dump and stops there: no IR and no binary. Its `--target` and `--target-version` mean what they mean for `emit-ir`.

## Stages

This compiler fuses elaboration with dictionary and evidence solving in one checking pass, so `elaborated` and `solved` are accepted as aliases of `checked`; there is no program between them to print.

| Stage | The program it prints |
| --- | --- |
| `checked` | what a check returns: instances resolved by the same pass, no reference counts, no target facts |
| `owned` | the reference-count pass applied to the checked program: `dup`, `drop`, `release`, the classes each binder carries |
| `lowered` | after specialization and reference counting: exactly what the LLVM emitter reads, with the target line and the selected `prim` bodies |
| `solved`, `elaborated` | aliases of `checked` |

`checked` and `owned` are target independent and print no target line, so the same source and the same modules give byte-identical text for every `--target`; that equality is the check that no target fact leaked into the frontend. Only `lowered` prints `target <triple> opt-level <n>` and may differ between targets.

The pipeline runs specialization before reference counting, so `owned` is not a prefix of `lowered`: it is the count pass over the checked program. The dumps of a build are nonetheless consistent, because `lowered` is the exact code generation input and every other stage says which stage it is in `@stage`.

## `--dump-core`

`check`, `emit-ir`, `build` and `run` accept `--dump-core=<stage>` and write the dump beside the other artifacts, without changing what the compile otherwise does:

```sh
cinch build Main.cinch --dump-core=owned
ls build/Main.owned.core
```

The last stage keeps the plain name (`build/<entry>.core`); an earlier stage appends its name (`build/<entry>.checked.core`, `build/<entry>.owned.core`). Each dump is listed in `build/<entry>.manifest.json` under `core`, with its stage, target and the SHA-256 of the text. The stage is the point the pipeline actually reached: `check` never runs the reference-count pass or specialization, so `--dump-core=owned` and `--dump-core=lowered` on `check` write `build/<entry>.checked.core`, and every unit in it says `@stage(checked)`. A dump never claims a stage the mode did not reach.

The dump covers the entry file's modules. A unit it references and does not define (a prelude unit, a dictionary built elsewhere) is declared once in the header as `global <name>`, so the file still resolves every reference on its own.

## What the file looks like

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

The header is fixed and ordered: `core 1`, one `module` line per dumped module, the `target` line only at `lowered`, `source sha256:<64 hex>` over the checked sources of those modules, then any `global` declarations. Units print sorted by canonical name; annotations print in one documented order; ownership classes, instantiation vectors and alternative weights print as `@` marks on the node they belong to; a node whose children fit in 100 columns prints on one line, and everything else indents two spaces per level.

Binders print as names, not as ids. A parameter keeps the name the artifact records when that name means something (`f0` for a constructor's field, `p0` for a primitive's); every other binder prints `t<n>`, numbered from zero in first-appearance order within the unit, so the text is dense, stable across runs, and never uses the compiler's `LocalId`. Two binders of one unit never share a name, so a source binder shadowed in an inner scope prints as two distinct `t<n>`, and a closure literal's formals take the enclosing unit's next numbers because that is where its binder list sits. The checker records `v<n>` placeholders for an ordinary definition's parameters, and a placeholder is not a name, so it takes the same numbering; this is why `@params` prints `t0`, not `v4343`.

## Reading it back

`core verify` parses the text and re-checks it without re-elaborating source:

* the header and the grammar, including annotation names and their order;
* unit names unique, and each unit's module declared;
* every `(global X)` names a unit of the file or one of its `global` lines;
* every `(local X)` is in scope, where a `let` binds its name only in its body, a `lam` in its body, and a `case` alternative's pattern in that alternative;
* every `(dictproj d slot Class)` against the slot table of `d` when `d` is a `(instdict ...)` of a unit the file prints, and its class against the class that unit implements;
* `dup`, `drop`, `release`, `refset`, `boxinit`, `boxclear`, `lazy-init` name in-scope locals;
* every `@span`, when present, as `file:line:col`.

A clean file prints one summary line and exits 0; errors print as `unit:path: message` (for example `Main.map:body:alt 2: slot 3 is not in Main.OrdNat's static slot table`) and exit 1.

## Comparing two dumps

`core diff A B` walks both files unit by unit in canonical order and prints the first divergence as a structural path, not a line number:

```sh
cinch core diff build/Main.checked.core build/Main.owned.core
Main.main:@stage: left (checked) right (owned)
cinch core diff a.core b.core
Main.map:body:alt 2:field 1: left (int 1) right (int 2)
```

A unit present on one side prints as `Main.helper: only in the right file`. Identical files print nothing and exit 0; differing files exit 1. The header is not compared, because two stages of one build legitimately differ in their target line.

## What this compiler's stages do not print

The format is a checked artifact, not a debug print, so a fact the stage does not carry is omitted or printed as `-` rather than guessed:

* A unit's declared type comes from the checked interface; a unit the interface does not export (a hoisted lambda, the `main` adapter, an instance method, a foreign wrapper) prints `-` in the type slot. A parameter's type prints where the scheme decomposes into the unit's runtime arrows exactly, and `-` for every parameter where it does not.
* Local binder types are gone before any stage this compiler prints, so a `let` prints its name and class, with no type slot.
* `@span` is omitted, because this compiler's Core records no per-unit source span; `core verify` checks a span when a hand-written or later file carries one.
* `@borrowed(...)` prints from the borrow inference the program records (`Program::lent`), which this compiler keeps for primitive units only; primitive ABI parameters are scalars, which are never counted, so a source program does not currently exhibit the annotation.
* Join points print when the Core has them; this compiler's code generation builds blocks itself and its Core carries no join node, so no `join`/`jump` appears.
* A `vector` index prints as the resolved type (`Vec 3 Int`); the design's `@compile-time` and `@runtime(n)` annotations need a recorded index decision this compiler does not keep.

# Cross compilation

`--target` selects one of five triples. `check` and `emit-ir` work on any host for all five: the target profile comes from the triple, the emitted module carries that target's `target triple` and `target datalayout`, and a `prim` body whose `when` names the target's architecture is the one selected. `wasm32-unknown-unknown` is the fifth, and the one `build` links without a prebuilt archive. A program that imports `Prim.Sys.Linux` therefore emits the aarch64 syscall bodies under one target and the x86_64 bodies under the other:

```sh
cinch emit-ir Main.cinch --target aarch64-unknown-linux-gnu | grep -E 'target (triple|datalayout)'
# target triple = "aarch64-unknown-linux-gnu"
# target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i8:8:32-i16:16:32-i64:64-i128:128-n32:64-S128"
cinch emit-ir Main.cinch --target x86_64-unknown-linux-gnu | grep 'mov rax, 257'
#   %r = call i64 asm sideeffect "mov rax, 257\0A mov rdi, $1\0A ...
```

`build` for a foreign target links two things the host build does not have. The first is the compiler runtime archive for the target, named by `--runtime-archive`; the host's `libcinchruntime.a` is the natural mistake and is refused by architecture before the link starts. The second is a linker that produces the target's objects: the link passes `-target <triple>` to `clang`, and `CINCH_CLANG` names a compiler that has a cross linker for that target.

```sh
cinch build Main.cinch -o prog --target aarch64-unknown-linux-gnu
cinch: target `aarch64-unknown-linux-gnu`: a cross build links the compiler runtime built for that target, and none was named; pass --runtime-archive <path>/libcinchruntime.a built for `aarch64-unknown-linux-gnu` (cargo build --manifest-path compiler/runtime/Cargo.toml --target aarch64-unknown-linux-gnu produces one)

cinch build Main.cinch -o prog --target aarch64-unknown-linux-gnu \
    --runtime-archive ../runtime/target/aarch64-unknown-linux-gnu/debug/libcinchruntime.a
# exited 0 when the cross linker is present, and names it when it is not
```

`run` builds the program and executes it here, so it accepts only the host's own target and says so:

```sh
cinch run Main.cinch --target x86_64-unknown-linux-gnu
cinch: target `x86_64-unknown-linux-gnu`: `run` executes the program on this host, whose target is `aarch64-apple-darwin`, and a binary built for `x86_64-unknown-linux-gnu` does not run here; use `cinch build --target x86_64-unknown-linux-gnu --runtime-archive <path>` and copy the artifact to the target
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

There is no optimization-level flag, no `--watch`, and no foreign-object flag on `build` or `run`; `build` and `run` take dependency objects only through `--package-object`. `-o` takes one path. `jit` is not part of the default build: this build prints `cinch: this build was compiled without the jit feature` and exits 2, and `cargo build --features jit` is what enables it.

<span class="support-tag support-tag--partial">Partial</span> **cross compilation.** `--target` selects one of five triples, and `check`, `emit-ir`, and `emit-object` work for all of them on any host. `build` needs two things this repository does not ship prebuilt: a `libcinchruntime.a` for the target, which `--runtime-archive` names, and a cross linker, which `CINCH_CLANG` has to point at. `wasm32-unknown-unknown` is the exception on both counts: the freestanding runtime archive is built from this tree's own runtime crate, and the link uses `clang -c` plus `wasm-ld` rather than clang's driver. `run` executes a native target's executable and a `wasm32-unknown-unknown` module under Node; every other foreign target refuses `run` because the host cannot start its binary. There is no bundle or distribution workflow on top of the produced artifact yet.

<span class="support-tag support-tag--planned">Coming soon</span> **portable distribution and target workflows.** Source builds and the native commands above are usable today. Target-profile APIs, component adapters, and runtime update mechanisms do not yet form a CLI for distributing cross-target bundles.

Implementation: `compiler/src/main.rs` (argument parsing and the usage block), `compiler/src/driver.rs` (modes, artifacts, targets, linking), `compiler/src/codegen.rs` (the module header and its datalayout), `compiler/src/store.rs` (the package store), `compiler/src/incremental.rs` (the session API). Executable suites: `argv.rs`, `modules.rs`, `targets.rs`, `package_store.rs`, `incremental.rs`, `staging.rs`.
