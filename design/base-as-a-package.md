# A minimal embedded core, packages for the rest, and a runtime that owns its syscalls

Status: proposal for review, 2026-11-09. Direction settled with the user in session: keep a small embedded set that exports the absolute basic primitives, move everything built on top of it into registry packages fetched at install time, give the prelude the Haskell treatment (a default that a package configuration can replace), and teach the manifest to contribute link fields, linker options, and pkgconfig. The runtime gains a syscall layer this project owns rather than borrowing libc's C ABI.

Mid-clause in the user's note: "tbc i don't want embedded to be ..." followed by the explicit clarification that the minimal primitive layer still ships embedded. So the embedded tier shrinks; it does not disappear. The exact boundary of that minimal set is the one open item this note proposes an answer to.

# 1 The embedded core is `cinch/prim`

The only source that ships inside the compiler is one package, `cinch/prim`, holding the primops that make the predeclared primitive types usable. Nothing else is embedded.

The rule that decides membership, and it is a rule rather than a list: **if the checker or codegen must know its representation, its declaration lives in `cinch/prim`.** That covers:

- the preregistered types: `Nat`, `Int`, `Bool`, `Unit`, `Bytes`, `Char`, `Sym`, the thirteen machine scalars (`I8`..`U64`, `F32`, `F64`, `Usize`, `Isize`), and the runtime carriers the checker types by fiat (`Io r`, `Scope`, `Task`, `Array`, `Tensor`, `Columnar`, `Ref`, `Mut`, `Code`, `Syntax`, and the builtin index families `Vec`, `Fin`, `Dec`). These stay in the checker because size, alignment, representation, and codegen are properties no source declaration can carry: `opaque type` says nothing about width, and the layout pass needs a width before any module is read. So the criterion is sharper than "the checker mentions it": a type is preregistered exactly when the compiler must know how it is built. `cinch/prim` still owns the operations over it, and a test keeps the two tables in step, so `Int is a builtin primitive type and cannot be redefined` stays true while the operations are ordinary source;
- the structural formers, which have no declaring module to live in: the arrow, and tuples of every arity (`Former::Tuple(n)` in `compiler/src/types.rs`). Tuples are the reason a purely source-declared world does not work, and they carry a gap the base plan has to close: with no home module there is nowhere to put a tuple's instances, which `design/base-library.md` already flags by naming `Data.Tuple` as the honest home;
- the primops: the `cinch.prim.*` and `cinch.runtime.*` bindings that make those types usable, which is what `Data/Nat.cinch`, `Data/Int.cinch`, `Data/Bool.cinch`, `Data/Bytes.cinch`, `Data/Char.cinch`, `Data/Machine.cinch`, and `Data/Sym.cinch` contain today, minus everything operator-shaped;
- the effect and capability vocabulary the checker enforces: the label names and the `Unit ->{IO r}` entry shape.

What `cinch/prim` does not contain is any instance. No `Add Int`, no `Eq Nat`, no `Show`, no `Semigroup`. So a prim-only build has types and machine operations but no operator to apply them: `1 + 2` fails with `no instance for Add Int`, and the fix is `import` from base rather than a special case in the checker. That failure is the boundary test: if a name is reachable with only `cinch/prim` in scope and it would need an instance to be useful, it belongs in base.

Module naming follows from that split, and it collides with the current layout, so it needs deciding before the cut lands:

| Today | After |
| --- | --- |
| `Data.Nat`, `Data.Int`, ... carry the primops for a type the checker predeclares, and also carry that type's instances | `Prim.Nat`, `Prim.Int`, ... in `cinch/prim` carry the primops only; `Data.Nat`, `Data.Int`, ... in `cinch/base` re-export them and declare the instances, which is the "a type's INSTANCE family stays in its public home" rule in `design/base-library.md` |

`cinch/prim` gets no implicit module scope either. Its modules are imported like any other, so `import Prim.Machine` is how a program reaches `u8-to-i32`, and a package with `prelude: none` and no imports has nothing but the predeclared type names. What stays privileged is narrower than a module: a fixed table of *type* names is universe scope, because the layout pass needs their width before any module is read, and the checker already enforces the reservation (`data Int where` is `Int is a builtin primitive type and cannot be redefined`). That is the whole privilege: one table of types, no modules, no values.

The installed-value table should shrink to match. Today the checker also installs values such as `ioWrite`, `ioRead`, `ioSleep`, and `pollCancel`, and a module may quietly define its own `ioWrite` (verified). Once `cinch/prim` exists those wrappers are ordinary declarations in a module that a prelude imports, so they obey the same import and shadowing rules as anything else, and what remains installed is primops and the handful of names codegen itself emits.

Nothing else stays embedded, including every module that reaches these primitives and builds on them: `Data.List`, `Data.Map`, `Data.HashMap`, `Data.Set`, `Data.NonEmpty`, `Data.Rational`, `Data.BigDecimal`, `Category.*`, `Algebra.*`, `Relation.*`, `Text.*`, `Effect.Fs`, `Effect.Net`, `Effect.Http`, and the facade.

Two consequences worth naming:

- The embedded set must be closed under what the checker's installed names mention, in two parts. An installed type name has no home module, so nothing outside the checker may claim it; an installed value has a home, and that home must be a module in `cinch/prim`'s manifest. Two assertions, and together they are the guard against the cut drifting.
- `library/` generation splits: `cinch/prim` pages keep coming from `include_str!`; pages for base and every other package come from resolved package interfaces.

# 2 The base as registry packages

Delivery, per the user's decision: fetched from the registry, pinned in `.cinchpkg` and `.cinchlock`, so the base a project was verified against is the base it builds against and two projects on one machine can differ.

Proposed package split, which keeps each install small and lets a project take only what it uses:

| Package | Contents | Notes |
| --- | --- | --- |
| `cinch/base` | the public homes of the primitive types (`Data.Nat`, `Data.Int`, `Data.Bool`, `Data.Bytes`, `Data.Char`, `Data.Machine`, `Data.Sym`), their instances, and the conversions built on them | sits directly on `cinch/prim`, and is what a project almost always needs |
| `cinch/prelude` | the facade a project imports by default, plus the curated re-exports | small, depends on the four below |
| `cinch/data` | `Data.List`, `Map`, `HashMap`, `Set`, `NonEmpty`, `Sorted`, `Array`, `Matrix`, `Tensor`, `Columnar`, `Rational`, `BigDecimal`, `Date`, `Time` | no effects |
| `cinch/category` | `Category.*`, `Algebra.*`, `Relation.*`, `Function.Combinators` | the dictionary vocabulary; nearly every other package depends on it |
| `cinch/text` | `Text.*`, `Data.Json`, `Data.Path`, `Data.Uri`, `Data.IpAddr` | no effects, no OS |
| `cinch/effect` | `Effect.Sequencing`, `Io`, `Concurrent`, `Clock`, `Random`, `Log`, `Trace`, `Otlp`, `Fs`, `FileUtils`, `Find`, `Process`, `Pty`, `Signal`, `Resolv`, `Net`, `Http` | the OS-facing half, and the package that grows with part 4 |
| `cinch/test` | `Effect.Test`, `Effect.Benchmark`, `Effect.Coverage` | optional |

The dependency direction is acyclic by construction: `text` and `data` depend on `category` and the embedded core; `effect` adds the OS; `prelude` sits on top.

Operational gap this creates, stated rather than hidden: with registry delivery, a fresh checkout with no network and an empty store cannot compile a program that uses `Data.List`. Three ways out, and the choice belongs to the user: (a) accept it and require `cinch pkg install` before the first build; (b) ship a compressed snapshot of the five base packages inside the toolchain and use the registry only for upgrades; (c) let `cinch build` fetch a missing base package automatically when the manifest pins it, failing loudly in offline mode. The proposal prefers (c) plus (b), because it keeps the toolchain usable in an air-gapped container while never making the vendored copy a hidden second source of truth.

# 3 The prelude, the Haskell way

A prelude needs no new semantics. It is an ordinary module, injected as an implicit `import` at the top of every module in the package that names it. Everything else follows from machinery that already exists:

- **Identity comes from the declaring module.** `List` is the type declared by whichever module declares it. If `PreludeA` and `PreludeB` both re-export `Data.List`'s `List`, they share one type and signatures interoperate with no annotation. If each declares its own `List`, those are two types with one name, and mixing them is the ordinary "expected X, got Y" mismatch that already names both.
- **Instances follow the same rule, and the selection rule already matches.** `implicit` instances are the only candidates for automatic resolution (`check_dictionary.rs` skips every instance without the flag unless it is the module's own), and a non-implicit instance is reachable by name through a module-argument import (`named_instance_arg` in `check_decls.rs`, documented as explicit dictionary selection). So overlapping explicit instances cannot make anything ambiguous: nothing selects them silently. What is missing is provenance on the candidate list, and it is missing one level up: names, instance names included, are deduplicated by name text and module rather than by declaring origin. `import A` beside `import B` where B re-exports A fails with ``the name `x` is imported from both A and B``, verified for a value and for an instance name alike. Deduplicating by origin fixes the diamond, makes two preludes that re-export one `Data.List` coherent, and reduces instance ambiguity to its real case: two different origins, both implicit, one head. Under a parameterized module the origin has to be the origin plus its instantiation, because each instantiation publishes its own copy of the module's instances.
- **Re-export chains are already tracked**, because `export module` and export lists record origins for the interface. So "do these two preludes share `List` and its instances" is a query over module dependencies and export origins, not a special prelude check. `cinch pkg verify` can print it for any closure.
- **The injection is a dependency.** The implicit import belongs in the module's dependency set, in the interface, and in the check order, because the family's `order` list is load-bearing for fixity, eager references, and the check/interface split. A prelude that is an ordinary module slots into that list like any other.
- **`.cinchpkg` gains one field**, `prelude`: a module name, or `none`. The root project's setting applies to its own modules; a dependency's setting applies to that dependency's modules. Nothing global is assumed, and nothing about the root's choice reaches a dependency. `none` leaves the predeclared type names and nothing else, so a module with no imports can write `main :: Int` and cannot call `int-add`.
- **Explicit imports win over the injected one**, matching the existing rule that explicit source wins over the embedded layer. A module-level redeclaration of a prelude module's name is an error, not a replacement.

What this removes that exists today: the accidental behaviour where a sibling file named `Effect.Net.cinch` silently replaces the whole embedded module. Verified on the current compiler while writing this note:

```
$ cat Effect.Net.cinch
module Effect.Net where
  shadowed :: Int
  shadowed = 99
$ cinch run ./Main.cinch        # main imports Effect.Net
99
$ cinch check ./M2.cinch        # uses a name the embedded module provided
cinch: 3:15: unknown name net-error-code (in ./M2.cinch)
```

With `Effect.Net` in a package, that file becomes a duplicate module declaration: an error naming both packages, not a silent replacement. The shadowing rule survives only for the embedded primitive layer, where it is name-level and additive, and even there it should be an error for a whole-module redeclaration.

# 4 Native code: link fields, linker options, pkgconfig

Because a package is source plus a manifest, the manifest needs to describe what the final link requires. Proposed field shape, recorded per package:

```json
{
  "name": "acme/tls",
  "link": {
    "libs": ["ssl", "crypto"],
    "flags": ["-Wl,-rpath,@loader_path/../lib"],
    "frameworks": ["Security"],
    "pkgconfig": [ { "name": "openssl", "min": "3.0" } ]
  }
}
```

Flags are never unconditional in practice: a package that binds a system library needs different words per operating system, per architecture, and per version of the platform it was built against. So the manifest carries conditions, and one condition form governs both modules and link words.

## 4.1 Conditions: platform, architecture, versions

A `select` entry holds a `when` condition plus the modules and link words it adds when the condition holds:

```json
{
  "schema": 2,
  "name": "acme/tls",
  "modules": [ { "name": "Tls", "path": "Tls.cinch" } ],
  "select": [
    {
      "when": { "os": ["darwin"], "arch": ["arm64", "x86_64"] },
      "modules": [ { "name": "Tls.SecureTransport", "path": "darwin/SecureTransport.cinch" } ],
      "link": { "frameworks": ["Security"], "flags": ["-mmacosx-version-min=14.0"] }
    },
    {
      "when": { "os": ["darwin"], "os_version": ">=15.0" },
      "link": { "flags": ["-Wl,-dead_strip_dylibs"] }
    },
    {
      "when": { "os": ["linux"], "libc": "glibc" },
      "modules": [ { "name": "Tls.Openssl", "path": "linux/openssl.cinch" } ],
      "link": { "pkgconfig": [ { "name": "openssl", "min": "3.0" } ] }
    }
  ]
}
```

The vocabulary, evaluated against the compile target rather than the build host:

| Key | Values | Meaning |
| --- | --- | --- |
| `os` | `darwin`, `linux`, `windows`, and any future target | The target operating system, from the triple |
| `arch` | `arm64`, `x86_64`, and any future architecture | The target architecture, from the triple |
| `os_version` | a comparison such as `>=14.0`, `=13.4` | The target platform version: the Darwin product version, or the kernel and libc version on Linux |
| `libc` | `glibc`, `musl`, `libSystem` | Which C library the target links, which decides whether a system library is present at all |
| `compiler` | a comparison such as `>=0.4` | The Cinch compiler version the entry needs, for packages that rely on a language feature |
| `runtime_abi` | an exact version | The runtime ABI, which a prebuilt object must match; today the compiler already refuses a mismatch in `build/package.cinchif` |
| `target` | an exact triple such as `aarch64-apple-darwin` | The escape hatch for a case the other keys cannot express |

Semantics, kept small on purpose: inside one object, keys are ANDed; a value that is a list is ORed; a missing key matches anything. An entry with no `when` applies always. Two entries may both apply, and their modules and link words are unioned, so a package can state a base set plus a version override. An entry that names a module twice with different paths is a resolution error.

## 4.2 Platform modules are an opt-in surface

`Unix.Watch` needs `kevent` on Darwin and `inotify_init1` on Linux, and both are ordinary Cinch modules in one package, so conditional `modules` lets a package ship both and compile neither one it cannot use. That is the alternative to `#ifdef`-shaped source and to one package per platform, and the chosen list is ordered by the package's own `order` field, so the total check order stays data.

`select` then does two different jobs, and both matter to a consumer:

- **Implementation choice.** `Tls.SecureTransport` on Darwin and `Tls.Openssl` on Linux are the same job done twice. A project imports `Tls` and never names either.
- **Extra surface.** The kqueue implementation of a watcher has knobs that inotify does not: filters, per-event flags, a batch read. Exposing `Unix.Watch.Kevent` on Darwin lets a project spend portability on capability, deliberately and visibly.

So a package exports the union of its unconditional modules and the modules its conditions select, and the resolver records which target made which module available:

```json
{
  "select": [
    { "when": { "os": ["darwin"] },
      "modules": [
        { "name": "Unix.Watch.Kqueue", "path": "darwin/kqueue.cinch" },
        { "name": "Unix.Watch.Kevent", "path": "darwin/kevent.cinch", "exposed": true }
      ] },
    { "when": { "os": ["linux"] },
      "modules": [
        { "name": "Unix.Watch.Inotify", "path": "linux/inotify.cinch" },
        { "name": "Unix.Watch.Inotify.Watch", "path": "linux/watch.cinch", "exposed": true }
      ] }
  ]
}
```

The rules that make the opt-in honest:

- **`exposed` separates implementation from surface.** A module without it is compiled as part of the package and is not importable from another package, which is what an implementation module should be. A module with `exposed: true` is importable by consumers, so the platform module that a project opts into is also the one the compiler will let it name. A manifest that predates this field means every module it lists is exposed, which keeps the existing `.cinchpkg` files valid.
- **A module outside the target's resolution is not importable, and the refusal says why.** `import Unix.Watch.Kevent` for a Linux target fails with the module name, the target triple, the owning package, and the condition that would make it available, rather than "no such module". That diagnostic is the whole ergonomics of the feature: the cost of the lock-in is stated at the moment a project takes it.
- **Portability is a per-target build question, not a source annotation.** A project that imports a platform module simply does not build for the other targets, and `cinch pkg verify --target TRIPLE` reports the import that has no resolution. Nothing needs a new syntax and no source file needs a conditional.
- **The portable module never re-exports the platform names.** `Unix.Watch` carries the portable subset; `Unix.Watch.Kevent` carries the rest. Re-exporting a Darwin-only name from a portable module would move the lock-in into the portable API, which is the mistake this split exists to avoid.
- **Availability is visible before a build.** The interface records `available_for: [<triple>...]` per module, `cinch pkg info` prints it, and the generated library reference labels a platform module with its targets. A reader sees the constraint without compiling anything.
- **Overlap is still an error.** Two entries that select the same module name under overlapping conditions with different paths remain a resolution error, whether the module is internal or exposed.

## 4.3 Semantic fields first, and one escape hatch with a consent gate

The compiler should understand what a package needs, not just pass words through: which libraries a closure requires, which of them resolve, whether a produced binary names a foreign library, and whether a package is asking for something a semantic field can already express. That understanding is what lets `cinch pkg verify`, `cinch pkg info`, and the docs answer questions instead of printing a command line.

So the manifest has three tiers, and only one of them is a hole:

| Tier | Shape | Understood | In the lock | Portable |
| --- | --- | --- | --- | --- |
| Semantic | `libs`, `frameworks`, `rpaths`, `libdirs`, `soname`, `whole_archive`, `pkgconfig` | yes: validated, ordered, and reported per package | resolved words | yes, when `rpaths` are relative or `@loader_path` |
| Raw | `raw: { "ldflags": [...], "cflags": [...] }` | no: recorded verbatim with a hash, flagged in `verify` | yes, verbatim | only within one machine |
| Local | project config, never package metadata | no | no, and the build records that it was used | no, by definition |

Rules for the raw tier, which exists for the case a semantic field cannot express:

- **It is refused unless the consumer consents.** A closure that contains raw link words fails the build with the package, the words, and the two ways forward: add `"raw_link": ["acme/vendorlib"]` to the root `.cinchpkg`, or pass `--allow-raw-link acme/vendorlib`. Consent is per package and per root, not global, so a transitive dependency cannot smuggle one in.
- **It is visible everywhere it matters.** `cinch pkg info` prints the words, `cinch pkg verify` reports `native.raw` for that package, `cinch build --print-link-plan` shows every contribution tagged with its origin, and the artifact manifest records that raw words were part of the link.
- **It never changes the hardening defaults.** Raw words are appended after the compiler's own, and the ones that redefine the product (`-Wl,--script`, `-nostdlib`, `-Wl,-e`, `-Xlinker`, response files) are refused even under consent, because the package that needs them needs a different mechanism rather than a bigger hammer.
- **Local overrides are separate and self-incriminating.** A machine-specific tweak (`-L/opt/local/lib`) belongs in project configuration, is applied last, is excluded from the lock, and makes the build record `local_link_used: true`, so a reproducible-build check can refuse the artifact instead of silently accepting a binary nobody else can produce.

The point of the split is not to make linking safe, which it cannot be, but to keep the analysed, reproducible, portable path as the default and confine the unknowable path to something a consumer has to say yes to, and something a verification tool can name.

## 4.4 Resolution is recorded, then the build is frozen against it

- **Resolution time.** The resolver evaluates `select` against the requested target at `cinch pkg add` / `install` time, runs `pkg-config --modversion/--libs/--cflags` for each surviving `pkgconfig` entry, and writes the effective result into `.cinchlock`: target triple, selected modules, resolved versions, ordered linker words, and include paths. A build never consults the ambient system again, so a build in CI and a build on a laptop agree or refuse instead of silently differing.
- **Cross-compilation.** A lock holds one resolution per target. `cinch pkg install --target aarch64-apple-darwin` adds that target's record beside the others, and building for a target without a record is an error naming the target.
- **A missing target is a refusal, not a default.** If no `select` entry applies and the package has no unconditional modules that suffice, resolution fails with the package name, the target triple, and the condition keys that were checked. A package that silently builds wrong on an untested platform is worse than one that refuses.
- **Verification.** `cinch pkg verify --target TRIPLE` re-derives the resolution from the manifest and the recorded `pkgconfig` versions and fails when the lock and the manifest disagree, which is the same shape as the existing identity and object-hash checks.

The remaining link rules stand as stated above: a missing library reports the failing package plus clang's own message, and only installed packages may contribute flags.
- **Provenance.** A package that declares `link` or `pkgconfig` is marked in its interface as requiring native code. The registry can restrict which namespaces may publish such a package, and `cinch pkg verify` can refuse an install whose lock lacks the resolved record.
- **Diagnostics.** A missing library is reported as the failing package, the requested entry, and clang's own message, which is the shape the compiler already uses for link failures.
- **Where the flags may come from.** Only installed packages, never the project's own manifest or the command line at first, so a build's link line is a function of the lock.

This is the mechanism that lets `Net.Tls`, `Sql.Sqlite`, and anything else binding a real library exist without the runtime having to reimplement it. The runtime's own syscall layer (part 5) is what keeps the *first-party* stack free of that dependency.

# 5 A runtime that owns its syscalls

Today `cinchruntime` has zero dependencies and declares libc symbols directly (`kill`, `pause`, `_exit`, and the `cinch_fs_*` and `cinch_net_*` wrappers around `open`/`read`/`socket`/...). Clang's default link line supplies libc plus, on Linux, `-ldl -lpthread -lm`. The ABI we promise is therefore partly somebody else's, and errno handling is a C convention we inherited.

Target shape, following Go's split:

| Target | How we reach the kernel | Why |
| --- | --- | --- |
| `linux/x86_64`, `linux/aarch64` | `syscall` instruction, numbers pinned per architecture in our own table | Stable ABI; this is what lets a static binary drop glibc |
| `darwin/arm64`, `darwin/x86_64` | direct `libSystem` symbols declared in our own module | Darwin syscall numbers are unsupported ABI and move between releases; Go also goes through libSystem on darwin |
| `windows/x86_64`, `windows/aarch64` | `kernel32.dll` and `ntdll.dll` entry points declared by us, resolved at load | The NT syscall numbers change between Windows builds, so a raw `syscall` instruction is not a portable interface; Go links the same DLL entry points |
| `wasm32/wasi` | WASI preview 2 imports | The component target has no kernel, only the adapter's imports |
| FreeBSD, NetBSD, OpenBSD | `kqueue` plus per-BSD constants, one module each | The interfaces are POSIX-shaped but the numbers and struct layouts are per system |

- `runtime/src/sys/mod.rs` defines the interface in words and bytes: `fn openat(dir: i64, path: &[u8], flags: i64, mode: i64) -> (i64, i64)`, returning the result and the platform error number as data. No `std::io`, no `io::Error`, no libc types in the signature.
- `sys/linux.rs` and `sys/darwin.rs` are the only files allowed to name a kernel entry point. A test greps the runtime sources and fails the build when `unsafe extern "C"` appears outside `sys/`.
- Errno stays ours: `platform.rs` keeps its map from platform number to the named `NetError`/`FsError` values, so the boundary contract survives unchanged.
- The first milestone keeps Rust's std for allocation, threads, and mutexes, because std links libc anyway. "No libc at all" is a later rung (`#![no_std]`, our allocator, our panic, our `_start`), and the honest goal now is: our own bindings, our own numbers, no wrapper crates, one auditable place for the OS.

What it unlocks, in order: `-nostdlib` static Linux binaries with our own `_start`; `poll`/`epoll`/`kqueue` as first-class capabilities instead of threads parked in blocking reads; and a single audit surface for the no-faults rules, where every kernel call is in `sys/` and every failure becomes a named value.

## 5.1 The seams that actually differ, and nothing else

The word "syscall layer" hides the fact that the differences between these systems are not one interface with different numbers. There are five seams, and every OS package crosses at most these:

| Seam | Linux | Darwin | Windows |
| --- | --- | --- | --- |
| Paths | bytes, `/` | bytes, `/` (HFS normalisation quirks) | UTF-16, `\`, drive letters, `\\?\` long paths, reserved names (`CON`, `NUL`, trailing dot and space) |
| Handles | fd, `i32`, close on exec flag | fd, `i32` | `HANDLE`, not interchangeable with a file descriptor, closed by `CloseHandle` |
| Readiness | `epoll` | `kqueue` | IOCP, or `WaitForMultipleObjects` for small sets; completion based, not readiness based |
| Process control | `fork`/`execve`, signals, `waitpid` | `posix_spawn`, signals, `waitpid` | `CreateProcessW`, job objects, console control events; no signals, no `fork`, no `waitpid` |
| Stack trap | `sigaltstack` plus a `SIGSEGV` handler (`compiler/runtime/src/stack.rs` already does this with per-OS constants) | the same with Darwin constants | `VirtualAlloc` guard pages plus a vectored exception handler; `stack.rs`'s approach does not port |

`runtime/src/sys/` therefore exposes five traits or modules, not one, and each target implements the five. The stack trap is the one already written per OS today, and it is the clearest evidence that the port is real work rather than a constant table: `stack.rs` uses `mmap`, `sigaltstack`, `SIGSEGV`, and `SIGBUS` numbers that differ between Darwin and Linux, and Windows has none of those calls.

## 5.2 What Windows specifically changes

- **No stable syscall numbers.** `NtCreateFile` and friends exist, but their numbers are chosen per Windows build, so calling them directly is a compatibility bet. The runtime declares `kernel32`/`ntdll` exports and resolves them at load, the way the Go runtime does, and the "we own our bindings" property survives: the declarations and the struct layouts are ours, only the entry point is dynamic.
- **Effect vocabulary has to admit absence.** `Effect.Signal` has no Windows equivalent beyond console control events, `Effect.Pty` needs `ConPTY`, and `Cancel` is a job object rather than a signal. A package must be able to say "this operation exists on these targets" without lying on the others, which is the `select` and `available_for` machinery in section 4.2.
- **Case-insensitive, reserved, and separators.** `Data.Path` cannot treat `.` and `..` as the only special components, and the checker's module file discovery (`dir_index`) assumes case-sensitive exact names today. Duplicate module names that differ only by case are legal on Linux and impossible on Windows, so the resolution rule needs a per-target stance.
- **Exit statuses differ, and the docs depend on it.** POSIX truncates to the low eight bits, which the guide documents with `300` becoming `44`. `ExitProcess` takes a full 32-bit code and a shell reports it differently, so the documented rule has to be per-target, and the example suite needs a target-conditioned expectation rather than one global claim.
- **Binary stdout.** `ioWrite` must write bytes with no newline translation and no console encoding, or every documented `stdout=` assertion becomes platform-dependent. That means binary mode on the handle, explicitly, in the runtime rather than in each program.
- **No `rpath`.** A DLL is found next to the executable, in `System32`, or through `AddDllDirectory`. The manifest's `rpaths` field has no Windows meaning, so the semantic tier needs an equivalent (`dll_dirs`) rather than a silently ignored field.

## 5.3 The manifest fields each platform needs

The semantic tier of section 4.3 is not portable either, so the fields should say what they mean per system:

| Concept | Darwin | Linux | Windows |
| --- | --- | --- | --- |
| Link a library | `libs` (`-l`), `frameworks` (`-framework`) | `libs`, plus `soname` for the produced object | `import_libs` (`.lib`), `system_libs` (`ws2_32`, `advapi32`, `bcrypt`, `ntdll`) |
| Find it at run time | `rpaths` (`@loader_path`, `@rpath`) | `rpaths` (`$ORIGIN`) | `dll_dirs` (applied with `AddDllDirectory`), or copy beside the executable |
| Read it at build time | `pkgconfig` | `pkgconfig` | `pkgconfig` when present, otherwise explicit paths; vcpkg and CMake config files are the common case and are not `pkg-config` |
| Pick the toolchain | one | `glibc` or `musl` | `msvc` or `gnu` (mingw), which decides `.lib` versus `.a` and the C runtime |

So the condition vocabulary of section 4.1 gains `toolchain` (`msvc`, `gnu`, `glibc`, `musl`), keeps `libc` for the Linux split, and `arch` grows `arm64` for Windows. A package that binds a native library on all three systems writes three `select` entries, and the resolver produces three lock records.

## 5.4 What the compiler itself has to grow

The checker and codegen are platform independent, so the work sits in four places: the driver's link step (it shells `clang`, which works as `clang-cl`, plus `cargo` for the runtime staticlib), the runtime's five seams, `cinchdoc.rs`'s one `cfg(unix)` branch, and the test suite, where every exact-stdout and exit-status expectation becomes target-conditioned. Until those exist, the honest status is what the support matrix should say: native is macOS and Linux, Windows and WASI are planned, and a package that declares them is declaring its intent rather than a passing build.

# 6 The packages the sys layer makes possible

Names are free where the base uses another root: the base owns `Data`, `Text`, `Effect`, `Algebra`, `Category`, `Function`, `Relation`, and the facade, so new roots are `Unix`, `Net`, `Crypto`, `Sql`, `Term`, `Build`, `Test`.

| Package | What it is | Needs | Pure Cinch |
| --- | --- | --- | --- |
| `Unix.Stat`, `Unix.Perm`, `Unix.User` | `stat`/`lstat`, mode bits as refinements, uid/gid to names | `statat`, `getuid`, passwd lookup | yes |
| `Unix.Watch` | directory events: inotify on Linux, kqueue on Darwin | `inotify_init1`/`add_watch`, `kevent` | yes |
| `Unix.Mmap` | mapped regions with the language's region and ownership story | `mmap`/`munmap`/`madvise` | yes |
| `Unix.Limit`, `Unix.Sys` | `rlimit`, `uname`, `sysconf`, `umask` | the same calls | yes |
| `Unix.Lock`, `Unix.Atom` | `flock`/`fcntl` locks, atomic replace with directory `fsync` | `fcntl`, `fsync`, `renameat` | yes |
| `Net.Tcp`, `Net.Udp`, `Net.Unix` | the server half of sockets, which `Effect.Net` does not have | `socket`, `bind`, `listen`, `accept4`, `setsockopt`, `poll` | yes |
| `Net.Event` | readiness loop over `epoll`/`kqueue` | the event calls plus a wake pipe | yes |
| `Net.Tls` | TLS 1.3 client and server | sockets, `getrandom`, `Crypto.*`, `link` for a platform library if we do not implement it | no |
| `Net.Http.Server`, `Net.Http2`, `Net.WebSocket` | the server side of what `Effect.Http` speaks as a client | `Net.Tcp`, `Net.Event`, `Net.Tls` | yes above TLS |
| `Crypto.Hash`, `Crypto.Mac`, `Crypto.Aead`, `Crypto.Sign`, `Crypto.Kdf` | SHA-2/SHA-3/BLAKE3, HMAC, ChaCha20-Poly1305/AES-GCM, Ed25519, HKDF/Argon2id | `getrandom` | mixed: hot loops in the runtime for constant time, policy in Cinch |
| `Sql.Sqlite` | a file-backed relational store | `mmap`, `fcntl`, `fsync` | yes, if we write the btree |
| `Sql.Postgres` | wire protocol client | `Net.Tcp`, `Net.Tls`, `Data.Bytes` | yes |
| `Text.Gzip`, `Text.Zstd`, `Text.Tar`, `Text.Zip`, `Text.Cbor`, `Text.MsgPack` | formats beside the existing `Text.Zlib`, `Text.Json`, `Text.Yaml`, `Text.Toml` | none | yes |
| `Term.Tui` | alternate screen, key decoding, cursor addressing, which the REPL implements in Rust today | `termios`, `ioctl(TIOCGWINSZ)`, `read` | yes, and it lets the REPL frontend become Cinch |
| `Build.Manifest`, `Build.Lock` | readers and writers for `.cinchpkg` and `.cinchlock` | none | yes |
| `Test.Property` | generators, shrinking, deterministic seeds | none | yes |
| `Time.Zone` | TZif parsing and local time | file reads | yes |

Ordering by what unblocks the most: `sys` socket and event calls, then `Net.Tcp`/`Net.Event`, then `Crypto.*`, then `Net.Tls`, then the format packages (they need nothing and can land in parallel), then `Sql.*`, then `Term.Tui`.

# 7 Open items

- **The `Prim.*` module rename.** The cut is settled (section 1), but moving the primitive types out of `Data.*` changes module names that the guide, the tutorial, the library reference, and every example mention. The migration is mechanical and the docs harness verifies it, but it is the largest single edit in the plan.
- **Offline bootstrap.** Registry delivery plus an optional vendored snapshot, as in section 2, or strict network-on-first-use.
- **Prelude diagnostics.** The mechanism is settled (section 3): injected import, identity by declaring module, re-exports tracked. What remains is message quality: a mismatch or an ambiguous instance should resolve through the re-export chain and name the two preludes and the two declaring modules.
- **Link policy.** Whether `libs` and `pkgconfig` are open to any namespace or gated to vetted ones, and whether `cinch pkg verify` refuses a lock whose resolved link record is missing. The semantic, raw, and local tiers are settled in section 4.3; what remains is who may publish each tier.

# 8 Aspects this design has not yet faced

Collected while reviewing the proposal against the tree. Each entry says why it matters and what it would take; none of them is answered above.

**Target identity does not exist yet.** `TargetProfile::native()` builds its triple as `format!("{}-unknown-unknown", ARCH)`, with no vendor, no operating system, and no version (`compiler/src/core.rs`), and there is no CLI flag that changes it (documented behaviour today: a `--target` word after the entry file is passed to the program as an argument). `select` conditions on `os`, `arch`, and `os_version` therefore cannot be evaluated at all until a real triple exists. Worse, `os_version` must come from an explicit target parameter and never from the host, or the same commit resolves to different lock entries on two laptops with different macOS versions, which is exactly the non-reproducibility the lock is supposed to remove.

**Preludes need only dependency tracking, and the tracking costs two things.** The user's rule: compile with the prelude injected as an implicit module import, then let module dependencies and re-export origins answer whether `PreludeA` and `PreludeB` share one `List` and one instance set. That removes the whole question of a global prelude, and it leaves two pieces of work. First, the injected module must be in the module's recorded dependency set and in the check order, or fixity and eager-reference collection see a module that names things it never imported. Second, diagnostics have to resolve through re-exports: today an `ambiguous instance` names the class and the type, and a prelude collision needs the declaring module and the prelude path for each side, otherwise the reader cannot tell "one instance re-exported twice" from "two instances with the same shape".

**Raw link flags are trust, so structure them for audit and reproducibility, not for sandboxing.** The user's objection is correct: a project that links a native library runs that library's code before `main` and inside its address space, so no arrangement of flags makes linking safe, and pretending otherwise would only cripple the FFI. The reasons to structure the field anyway are different ones. Reproducibility: a structured `rpaths`/`libdirs`/`soname` prevents the everyday failure where a lock records `-L/opt/homebrew/opt/openssl@3/lib` and no other machine can build. Audit: a declared `libs: ["ssl"]` can be reviewed across a closure, while an opaque argument list cannot, and a leaf package's arguments reach the final link of consumers that never import it, so the blast radius of an unreviewable field is the whole build. Blast radius again: `-Wl,--script`, `-nostdlib`, `-Wl,-e`, `-Xlinker`, and response files change what the produced binary *is*, which a package that merely wants OpenSSL never needs. What does deserve an explicit statement is the semantic consequence: a linked native library can run code at load time, so a program's effect row and its no-faults claim cover the checked fragment only, and that should be said in the package's documentation and in the language's own claims rather than implied away.

**`pkg-config` is neither hermetic nor portable.** Its output carries absolute paths (`-L/opt/homebrew/opt/openssl@3/lib`), `.pc` files can inject arbitrary flags, macOS ships no `pkg-config` at all, and cross-compilation needs `PKG_CONFIG_SYSROOT_DIR` and `--define-prefix`. Recording the resolved words in the lock hides the problem instead of solving it: the lock then contains machine-specific paths. The durable record is a soname plus a version range to check at runtime, with library paths required to be relative or `@loader_path`-based unless the package came from a local path dependency.

**Source packages versus binary packages.** If a registry ships `package.o`, that object is bound to one triple, one runtime ABI, one layout table, and the exact base build it was compiled against, so a base upgrade invalidates every binary in the store and every dependent's lock. Decide which the registry publishes: source only (rebuild at install, slow but portable), or binary with automatic rebuild on mismatch (fast but the mismatch is a compile in disguise). The lock should record which it got, and the provenance claim should say "built from these sources by this compiler" rather than only naming the builder.

**Capability vocabulary for the new surface.** `Io r` is one blanket authority. Filesystem, network, clock, entropy, and watch are different authorities, and the whole design of this language is that a signature tells you what a function may do. Six new packages behind one `Io r` label would quietly undo that: a package that only reads files would look identical to one that opens sockets. The narrow labels probably want to be `Fs r`, `Net r`, `Clock r`, `Entropy r`, `Watch r`, with a rule that a package's public API names the narrowest one it needs.

**Foreign effects must be discharged at the package boundary.** A public signature carrying `Foreign Lib` or `Diverge` pushes that requirement onto every consumer. The base family already shows the discipline (wrappers turn platform words into named values in `Result`), and the package rules need to say it: the boundary module owns the foreign effect, the exported API does not leak it.

**Stage safety.** Compile-time staging means a package can run during compilation. A networking or filesystem package imported by a `stage` function performs I/O at build time, which the staging design confines to `StageFile`, `StageSecret`, `StageProcess`, and `StageEmit`. Packages need a declared stage class, and `stage` code that reaches a runtime-only effect should be refused rather than silently doing the work in the compiler process.

**Caches keyed on the resolved module set.** `package_key` folds in the profile identity, which contains the triple, plus the sources, so a changing `select` result changes the key. The incremental session is the weaker spot: its dirty sets are keyed by declaration hashes, and nothing today says a cross-target session must not reuse units whose module list differs. Either the session key grows the target, or the session is per-target by construction.

**Namespace reservation and the review surface `select` creates.** Nothing stops a registry publisher from taking `Unix.Stat` before the official package does, and nothing reserves `Net.*`, `Crypto.*`, `Sql.*`, or `Term.*` for first-party releases. Separately, `select` is a place to hide code: a manifest with three platform branches means a reviewer who installs on Linux never compiles the Darwin branch. The registry should review every declared branch, `cinch pkg info` should print all of them, and reserved roots should be enforced by the compiler, not only by policy.

**The testing and cross-build cost is the real one.** A package that declares three targets promises three checked builds, and checking them requires cross-compiling `cinchruntime` itself: Rust targets, a sysroot, and a libc for musl. Decide whether the toolchain supplies those or the user's environment must, and make `cinch pkg verify --matrix` the gate that a release has to pass. This is also where the syscall layer pays or does not: one `sys` module per target is one cross-build configuration per target.

**Documentation follows the conditions.** The support matrix needs a per-target column, generated library pages need `available_for` on every platform module, and the package pages need to show `select` in human form. Otherwise the only place a platform constraint is visible is a compile error.
