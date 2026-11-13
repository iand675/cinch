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

  Where the compiler itself names a library type or class (the type of a list literal, the step form the `vecStreamStep` builtin produces, the class a collection literal resolves through), it names it at its identity, in one table: `compiler/src/known.rs`. Every constant there is `Owner.Name`, and a test parses the embedded sources and fails by name when a declaration it points at has moved or been renamed, so a stale entry is reported in that table rather than as a type mismatch in an unrelated program. A bare spelling such as `"List"` is surface syntax only. Types installed by the checker under the owner `<builtin>` still have no home and keep their bare identities; they are the remaining set the cut has to give homes: `Nat`, `Int`, `Bool`, `Unit`, `Bytes`, `Sym`, the machine scalars and `Char`, the `C*` foreign scalar spellings, `Array`, `Tensor`, `Columnar`, `Io`, `Scope`, `Task`, `Code`, `Syntax`, `SyntaxBinding`, `TypeSyntax`, `PackedCode`, `Declarations`, `Name`, `Secret`, `IoCap`, `FileReadCap`, `FileWriteCap`, `ProcCap`, `SchemaSnapshot`, `Database`, `SchemaRead`, the indexed families `Vec`, `Fin`, `Dec`, and the type formers `Thunk`, `Lazy`, `Later`, `Ref`, `Mut` and the boundary pointer roles. Classes have the same split: `Copy`, `Share` and `Io` are `<builtin>` keys, everything else is `Owner.Name`.
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

- **Identity comes from the declaring module.** `List` is the type declared by whichever module declares it. If `PreludeA` and `PreludeB` both re-export `Data.List`'s `List`, they share one type and signatures interoperate with no annotation. If each declares its own `List`, those are two types with one name, and mixing them is the ordinary "expected X, got Y" mismatch that already names both. This is the rule everywhere, embedded or not: every `data`, `record`, `codata`, `refine`, `opaque type` and `signature` is identified by `Owner.Name` and by nothing else, so a declaration's identity never depends on what else is declared. A project that ships its own source for a base module's name keeps the same identities by construction. Diagnostics print the bare name while it is unambiguous in the message and the identity when it is not.
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
    "rpaths": ["@loader_path/../lib"],
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
      "link": { "frameworks": ["Security"] }
    },
    {
      "when": { "os": ["darwin"], "os_version": ">=15.0" },
      "link": { "libs": ["dl"] }
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

**The in-source conditions carry the same vocabulary.** The requirement, stated after the syscall layer moved into Cinch: implementation choice belongs in the module that implements it, so a source file must be able to ask the same questions a manifest can. A `when` condition is a conjunction of `Available <capability>`, `arch <name>`, `os <name>`, `libc <name>`, `endian <little|big>`, `word-size <bits>` and `target <triple>`, each also negatable, evaluated against the compile target's `TargetProfile` and never against the host. It sits on a `prim` body, on an import (`import X when <clause>`) and on a declaration (`when <atom> <declaration>`), which is the compile-time macro the syscall and package layers actually need: `Prim.Sys` becomes a facade whose import of the Darwin or the Linux backend is conditional, and a name can carry two implementations under disjoint conditions.

Manifest conditions stay, because they also carry link words and module exposure, which are not the source file's business. The two vocabularies are deliberately parallel, so a condition reads the same where it is written and where it is resolved.

**What landed.** `compiler/prelude/Prim/Sys.cinch` is the facade: it imports `Prim.Sys.Darwin` `when os darwin` and `Prim.Sys.Linux` `when os linux`, and re-exports whichever holds, so `Effect.Fs` imports `Prim.Sys` and nothing platform specific. The two backends still declare their own `SysError`, and that duplication does not reach a caller: only one backend is re-exported per target, so the facade's `SysError` is one type on every target. A shared `Prim.Sys.Error` module would be the tidier home for it once a third backend exists, because then two backends must agree on one nominal type rather than two structurally identical ones.

A target with neither backend (`wasm32`, or a Windows profile the negation tests build) gets `Prim.Sys.Portable`, which is the same surface with every entry point answering `Err SysIoError`. That module exists because the prelude family is checked whole for every target: without a floor, `Effect.Fs` would fail to resolve its platform names on any profile that has no backend, which is a prelude-family question rather than a facade question. The portable floor issues no call and links nothing, so it costs a target nothing and states the absence honestly instead of hiding it.

Where a condition is evaluated: the module scanner reads import conditions before parsing, because the file set and the check order are what the graph decides; the parser reads the same clause into the import's syntax; and the checker takes the target's view of every module (dropping false imports, false declarations and the re-exports that named them) before scope, exports, interfaces or code generation read anything. The two readings share one lowering, so they cannot disagree.

**One compiler binary, every supported target.** The cross-compilation requirement (stated the same day): a target must not need its own compiler build. `--target <triple>` builds the `TargetProfile` from the triple rather than from the host, `check` and `emit-ir` work for any supported target, and the emitted module names its own triple and data layout so the assembler cannot fall back to the host's. `build` needs two things the compiler cannot invent: a runtime archive built for the target, supplied by flag, and a linker that produces that target's objects. Both are named in the failure rather than assumed, and `run` stays host only, because running a foreign binary is not a compiler question.

**Host and target are different questions.** A build has two profiles, not one: the host, where anything that runs during the build executes (staged units, macros, `eval`), and the target, which the artifact is emitted for. Module availability is therefore a per-profile fact, and both resolutions happen in one build. The manifest's `select` and the source's `when` conditions are evaluated against whichever profile the module is being resolved for, so a package can ship a Darwin-only module and a Linux CI machine can still cross-build for Darwin, with the module reachable from the target's code and unreachable from the host's.

The rule that follows: a module reached from emitted code must be available on the target, and a module reached from compile-time code must be available on the host. When one is available for the target and not for the host, the diagnostic names both contexts rather than saying "not available", because the fix differs: move the code out of the compile-time path, or widen the package's conditions.

**What landed.** `core::BuildProfiles` holds the two profiles and nothing derives one from the other: the host is `TargetProfile::native()`, and the target is that profile unless `--target` names a triple. `core::condition_context(build, context)` is the single place that decides a position's profile, and `core::BuildContext` is the two answers it takes: emitted code and its modules resolve against the target, anything that runs during the build resolves against the host. A `prim` body reads the target in every case, including when a staged definition reaches the `prim`, because the body becomes part of the artifact.

Availability is computed once per build rather than per import: `driver::load_project_for` resolves the project's manifests for the target (the module set the compilation uses) and once more for the host, and `driver::ModuleAvailability` answers "is this module available for the host", "... for the target", which package declares it, and which `select` condition would select it. A module no manifest declares (a file beside the entry, a prelude family member) is available for both profiles, because no condition governs it. Two consequences follow from how the resolver already worked and are kept. First, the directory index resolves an import to a file whatever the manifests say, so the manifest's resolution is enforced at import time: a module a manifest declares but the target's resolution leaves out is refused or recorded, never loaded because its file happens to sit under the project. Second, the artifact and its interfaces are the target's, so the interface records the target's selection only.

The compile-time closure is defined mechanically: a module is compile-time code when it declares a `stage` or `macro` definition (a `when` wrapping the declaration is read against the target, because that is the program the build compiles), when it writes `import stage M`, or when one of those modules imports it transitively. Anything else is emitted code. A module in that closure that the host's resolution lacks is refused, and so is an import the target cannot resolve whose importer is in the closure.

**The refusals.** A host-only module reached from compile-time code is refused with the module, the package and its condition, both triples with their operating systems, the fact that compile-time code runs on the host, the fact that the build compiles the target's module set, and the two fixes (move the code out of the build-time path, or widen the conditions so the module resolves for both). The mirror case (a module the target resolves and the host does not) uses the same shape with the requirement stated the other way: compile-time code runs on the host, so a module it reaches must be host-available. Ordinary code that reaches a host-only module is refused as unavailable for the target, naming the condition that would select it, because the target is the only profile an artifact has. A module neither profile resolves is refused at load time with both profiles named, since no fix in the source makes it available.

**The interface cache needs no change, and the rule is why.** `store::package_key_for` folds `profile_identity(target)` into the package key, and the compile path passes the target profile, so an interface compiled for a target is keyed by the profile that selected its modules. The host resolution produces no interface, no object and no cache entry: it answers availability questions and nothing else, so there is no host-keyed entry to collide with a target-keyed one. A later slice that caches host-resolved artifacts (an executor's results, for instance) adds them under the host profile's identity, the same field, and a rebuild under a different host then cannot reuse the wrong module set.

**What is not honest yet.** The compiler cannot ask the host for its platform version, so a manifest condition on `os_version` is evaluated for the host with the target's stated version when the target stated one, and both the manifest's `profiles.host.os_version_from` and this paragraph say so; that is a substitution, not a host fact. A host resolution that fails outright (a manifest whose `order` names a module only some targets resolve) is recorded as `profiles.host.resolution: unresolved` with the reason, and the build then reports host availability as `undecided` rather than inventing either answer. Nothing yet checks that a staged definition's *body* only names host-safe operations beyond the existing interpreter refusals; this slice decides availability, not what compile-time code may do.

**Evaluation has a host and a target too, so it needs an executor.** Macros and staged units are evaluated by an interpreter inside the compiler process, and that interpreter cannot run target code: today a `foreign import` in compile-time code is refused with "a foreign import cannot run at compile time (it needs target code and a linked foreign library)". The refusal is correct and it is also the gap: a macro that wants a syscall, a foreign call, or the target's own arithmetic is exactly the macro that has to be evaluated where the target exists.

So evaluation gets an executor, named per build:

| Executor | Where it runs | What it can evaluate |
| --- | --- | --- |
| `local` | in the compiler process | pure macros only, which is the current interpreter and its refusals |
| `remote` | a process that runs on the target, reached by command or endpoint | anything a target binary can run, including foreign imports and `prim` bodies |

The rules that make this sound rather than convenient:

- **The executor states its triple and the build checks it.** A mismatch is a hard error naming both, not a warning. The executor can lie, so this is a trust boundary like `raw` link words: the build records who ran what under which triple rather than pretending to prove it, and the artifact manifest carries that record.
- **A remote result is cached by content, not by convenience**: the unit's hash, the argument hash, the target triple and the executor's identity all key the cache entry, so a cached result is reused only for the same target and the same executor.
- **Pure macros stay local.** Sending everything to a remote executor would make every build depend on a network hop and a service; only the evaluations the interpreter cannot do cross the boundary.
- **Evaluation results are facts about the target**, so they belong to the target profile's part of the interface cache, exactly like target-selected modules.

This is a stated goal, not scheduled work. Nothing in the current compiler depends on it: the interpreter keeps refusing target code, and the refusal stays honest until an executor exists. The host and target split comes first for the mechanical reason that an executor cannot be asked for "the target" until a build names both profiles, and a refusal cannot say which context it came from.

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
- **Portability is mostly a per-target build question, with one narrow source annotation.** A project that imports a platform module simply does not build for the other targets, and `cinch pkg verify --target TRIPLE` reports the import that has no resolution. The manifest's `select` needs no source syntax, and a module that is one platform's implementation never carries a conditional. The one conditional a source file does carry is a `prim` body's `when` clause (section 5), which gates an inline instruction sequence on the compile target. It is deliberately smaller than a preprocessor: a clause is one atom, repeated clauses are a conjunction, and no declaration or macro is conditionally compiled.
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
- **Cross-compilation (planned).** A lock would hold one resolution per target, and `cinch pkg install --target` would add a target's record beside the others. Today a lock holds the host's single record, because `--target` refuses any triple but the host's: cross compilation needs a runtime built for the target first, and pretending otherwise would record a resolution nothing can build.
- **A missing target is a refusal, not a default.** If no `select` entry applies and the package has no unconditional modules that suffice, resolution fails with the package name, the target triple, and the condition keys that were checked. A package that silently builds wrong on an untested platform is worse than one that refuses.
- **Verification.** `cinch pkg verify --target TRIPLE` re-derives the resolution from the manifest and the recorded `pkgconfig` versions and fails when the lock and the manifest disagree, which is the same shape as the existing identity and object-hash checks.

`frameworks` and `soname` are refused on a target whose linker does not take them, because silently dropping a linker field links something other than what the package asked for. The remaining link rules stand as stated above: a missing library reports the failing package plus clang's own message, and only installed packages may contribute flags.
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

Landed in this revision, with tests:

- `runtime/src/sys/mod.rs` defines the interface in words and bytes: `write(fd, buf)`, `read(fd, buf)`, `close(fd)`, `open(path, flags)`, `open_read(path)`, `exit(code)`, `getpid()`, `kill(pid, sig)`, `pause()`, `atexit(f)`, `mmap_anon(size)`, `munmap(addr, size)`, `mprotect(addr, size, prot)`, `getpagesize()`, `ioctl(fd, request, arg)`, `sigaction`, `sigaltstack`, `this_thread_stack()`, the thread-key calls, the pty calls, and `getenv`. Every fallible call answers `(result, errno)`: `result` is the call's own value or `-1`, and `errno` is the platform's positive number or `0`. Names are NUL-terminated by one helper, which refuses an embedded NUL or an over-long name with `EINVAL`.
- `sys/linux.rs` issues the `syscall` instruction with numbers pinned per architecture (x86_64 and aarch64 tables), converting the kernel's `-errno` band to `(-1, errno)`. `pause` is `rt_sigsuspend` over an all-zero mask; `open` is `openat(AT_FDCWD, ...)`; `_exit` is `exit_group`.
- `sys/darwin.rs` declares the `libSystem` entries and reads errno through `__error()`, which is why Darwin has no numbers table. One real bug surfaced here: Apple arm64 passes variadic arguments on the stack, so `open` and `ioctl` must be declared variadic and a fixed-argument declaration silently loses the window-size argument.
- A test walks the runtime sources and fails when `unsafe extern "C"` names a foreign function outside `sys/`, with a second test proving the scanner sees a declaration, so the audit cannot pass blind.
- Errno stays ours: `platform.rs` keeps its map from platform number to the named `NetError`/`FsError` values, so the boundary contract survives unchanged.
- **The layer is Cinch, not Rust.** The user's constraint, restated and then verified: the syscall surface has to be bootstrapped like everything else, so the entry points are Cinch `prim` declarations and `foreign import ccall` declarations, and the Rust runtime keeps only what Cinch cannot express yet (allocation and reference-count headers, the task scheduler's threads, the fault path, the C-ABI entry). Two facts are measured on this machine: a Cinch module calling `open`, `read`, `write` and `close` through libSystem reads a file with no Rust wrapper in the path, so the Darwin catalogue needs no assembly at all; and a `prim` body with `unsafe llvm` carries inline assembly, so the Linux catalogue is prim bodies issuing `syscall`, one body per architecture behind `when arch`. The one primitive the seam needs from Rust is `bytes-data` (the payload address of a `Bytes`), which is also where the ownership rule bites: the unit must not release the buffer whose address it hands out.
- The layer's shape, settled with the user: **each supported OS gets its own comprehensive syscall module tree**, `sys/linux/` and `sys/darwin/`, split by area (`fs`, `proc`, `net`, `time`, `mem`, `thread`, `term`, plus the constants each OS needs). Every entry is documented with its number (Linux) or its libSystem symbol (Darwin), and the coverage is the primitive groups of `design/official-packages.md` section 3 rather than only what today's wrappers happen to call. `sys/mod.rs` stays a thin façade over them, and an entry with no cross-OS meaning (inotify, kqueue, epoll, `uname` fields) lives only in its own OS's module, because a package reaches per-OS capability through the manifest's `select` on `os`, not through a lowest-common-denominator API.
- The direction the user set for the Linux stubs themselves: they should be Cinch `prim` bodies carrying inline-asm IR (one body per architecture behind `when`), not Rust `asm!`, so the syscall is written in the language. The mechanism is verified on the current compiler: a `prim` with an `unsafe llvm { ... }` body whose IR is `call i64 asm sideeffect "mov $0, $1", "=r,r"(i64 %0)` checks, emits, links and runs. What the switchover still needs is a Linux host to execute the stubs, because this machine can only prove they compile and that the mechanism runs. One detail decides the shape: a prim body's `when` clause is a conjunction of atoms over the compile target profile (`Available <capability>`, `arch`, `os`, `libc`, `endian`, `word-size`, `target`, each also usable negated), so one prim holds both architectures' bodies, and a module can hold two OS implementations when their shapes agree. `Prim.Sys.Linux` is therefore one module with an `aarch64` body, an `x86_64` body and a portable baseline per call, and the numbers stay next to the call they belong to. The manifest's `select` on `arch` (section 4.1) is still the tool for what source cannot express: whole modules, link words, and platform versions.
- What is still libc-direct on purpose: the Linux pthread family, `sigaction`/`sigaltstack` (their struct ABI is the C library's), `getpagesize`, the pty helpers, `getenv`, and `atexit`, each in a marked section of `sys/linux.rs`; all of Darwin by design; and `std` for allocation, threads, mutexes, subprocess spawning and the filesystem wrappers not moved yet.
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

## 6.1 What the split costs, measured

The claim this note makes is that a program should pay for what it imports rather than for the whole prelude. That claim is now measured, and the measurement says the split as specified here does not deliver it. What it does deliver is smaller than the note implies, and the actual lever is a different one.

The instrument: the self-hosted stage 3 compiler (`build/bootstrap/stage3/cinch`), `--check --interfaces <manifest>`, with `CINCH_ALLOC_STATS=1` so the allocator prints `blocks=` for the whole process. The program is two lines, `module Main where` plus `main :: Int` / `main = 0`. The interface manifest is `build/bootstrap/stage3/base-interfaces.json`, filtered to a chosen set, so the only variable is which interfaces the loader reads. Every row below is one process; the block count is the whole compile, not a phase.

| Interface set | modules | exported entries | interface bytes | blocks | allocated bytes |
| --- | --- | --- | --- | --- | --- |
| none | 0 | 0 | 0 | 19.7 thousand | 455,343 |
| minimal floor (types, laws, arithmetic, `Int`, `Nat`) | 13 | 172 | 134,797 | 3,970,094 | 89,961,429 |
| this note's cut (the seven primitive homes, section 1) | 33 | 2,193 | 984,188 | 19,757,837 | 448,691,125 |
| today: the facade's implicit closure | 46 | 3,432 | 1,523,011 | 30,204,339 | 688,313,130 |

What follows, and the first is the important one.

**The cost is interface loading, not compilation.** A program that names nothing the interfaces declare still pays the full 30.2 M blocks; the same program against an empty manifest pays about 19.7 thousand. The two-line program contributes about 8,000 blocks of its own work. So 30.2 M blocks of 30.2 M are spent reading, parsing, validating and scoping 1.5 MB of interface text that this program cannot use. Packaging the prelude does not touch that number. Only loading less does.

**The cost is linear in interface bytes, at about 19 blocks per byte.** Reading the four rows as marginal steps, each additional byte of interface costs 18.6 blocks going from the minimal floor to this note's cut, 19.4 blocks going from that cut to the full facade closure, and 18.8 blocks for the 416,890 bytes the `Data.Machine` re-export contributes on its own. The ratio is flat across a twelvefold range of sizes, so there is no superlinear term to exploit and no constant factor hiding a cheaper path: the only way to spend fewer blocks is to hand the loader fewer bytes. The absolute ratio is higher for the smallest set (29.5 blocks per byte at the minimal floor), because a fixed cost per module and per interface is amortised over fewer bytes.

**This note's cut pays 35 percent, at the price of the largest rename in the plan.** The seven primitive homes plus their closure are 33 modules, 2,193 entries and 984 KB, for 19.8 M blocks against today's 30.2 M. That is a real 35 percent, and it is bought with the `Prim.*` rename that section 1 calls the largest single edit in the plan. It also does not deliver "pay for what you import": a program that names nothing still pays 2,193 entries, because the cut includes `Data.Machine`, which is one of the seven homes.

**One module is 38 percent of the bill.** `Data.Machine` is 581,881 bytes and 1,682 exported entries, against `Data.List` at 53,380 bytes and 63 entries. It is 38 percent of the facade closure's bytes and 49 percent of its entries, and the facade re-exports it directly (`export module Data.Machine`), so every program with the default prelude gets all 820 machine scalar and float declarations whether or not it does arithmetic. Measured: dropping just that one re-export from the facade takes the two-line program from 30.2 M blocks to 22.4 M, a 26.0 percent cut from one line of the facade.

**What a closure-selected set would actually pay.** If the loader read the closure of what a program imports rather than the whole facade, the two-line program would fall to the floor and a program importing `Data.List` would pay its own 18-module closure instead of all 46:

| Program | today (facade closure) | closure of its imports | change |
| --- | --- | --- | --- |
| two lines, `main = 0` | 30.2 M blocks | 19.7 thousand blocks | -99.9% |
| imports `Data.List`, checks green | 30.29 M blocks | 4.43 M blocks | -85% |
| the toolkit, 5 sources, 165 KB | 239.5 M blocks | not available, see below | - |

That is the win the note is reaching for, and it is an order of magnitude larger than what packaging delivers. It is also not a packaging change. It is a loader change: resolve a program's imports first, then load the interfaces that resolution reaches.

**The toolkit row is the honest limit, and it is why the closure is not simply "the imports".** The toolkit bundle does not check against the closure of its explicit imports: it fails with `unknown name (name is not in scope: eq-i64) in Text.Lex:652`. `Text.Lex` imports `Data.Bool`, `Data.Bytes`, `Data.Char`, `Data.List`, `Data.Nat`, `Data.Option`, `Data.String` and `Text.Show`, and none of them declares `eq-i64`; it reaches that name through the injected facade, which re-exports `Data.Machine`. So the facade is load-bearing for the toolkit's own sources in a way their import lists do not record, and closure selection would have to be closure over imports plus the injected prelude, which for the default prelude is the facade and its 46 modules again.

**And on the toolkit the interface load is not the cost anyway.** The same 46 interfaces cost 30.2 M blocks under a two-line program and 239.5 M under the toolkit bundle. So 209.3 M blocks, 87.4 percent, is compiling 165 KB of toolkit source, and the interface load is 12.6 percent. Cutting interface bytes by 85 percent would save at most 12.6 percent of a toolkit check, and the toolkit is the case the split is supposed to leave alone.

A second differential run says the same thing from the other direction. Eight programs checked against the full 64-module set and against the closure of their explicit imports agreed on six and disagreed on two: `main = int-add 1 2` and `main = nat-to-int 3` check against the full set and fail against an empty closure with `unknown name int-add`. Both name prelude values while importing nothing, which is what the injected prelude is for. So the closure has to be over explicit imports *plus the injected prelude*, and under the default prelude that is the facade and its 46 modules again. Closure selection therefore only pays for a program that opts into a narrower prelude, and shrinking the default prelude is a curation decision with a source-compatibility cost that section 3's negative-space rule permits but does not take.

**So: the restructure does not pay, and the measurement is the deliverable.** A package split changes where the prelude's sources live and how they are installed. It does not change how many interface bytes a compile reads, which is where the cost is. Landing it would add a package dependency and a store lookup to every build to buy 35 percent, and 26 of those points are available from one line of facade curation without a package at all. The lever worth pulling is lazy interface loading driven by name resolution, and the second lever is `Data.Machine`'s interface: 820 of its 1,682 entries are foreign declarations averaging 332 bytes each, and the facade re-exports the module unconditionally, so every program with the default prelude pays for all 820 machine scalar and float declarations whether it does arithmetic or not.

The invariants held across the measurement: stage 2 and stage 3 produce byte-identical `compiler.ll`, byte-identical `cinch` binaries and byte-identical `.cinchif` files (49 of 49), so the content-addressed identity of an interface set is unchanged by any of this. `runtime_abi` is untouched at 9. The three smokes pass, `CINCH_LEAK_CHECK=1` prints `live=0 immortal=0 leaked=0`, and `cargo test --lib` passes 275 tests with 0 failures. No lock was bumped and no lock needs bumping, because nothing here changes the compiler's source set, the library provider or the runtime ABI.

# 7 Open items

Landed in this revision, with tests: target identity (`TargetProfile` carries `os`, `arch`, `libc`, `os_version` and a real triple; `--target` and `--target-version`), manifest schema 2 (`select` conditions, `exposed`, `prelude`, and the `link` tiers with the raw consent gate), package resolution wired into loading with ownership and cross-package exposure, per-package prelude injection recorded as a dependency, name deduplication by declaring origin, the link plan with `--print-link-plan`, pkg-config resolved at install into `.cinchlock`, and `cinch pkg info` for per-target availability.

Still open:

- **The `Prim.*` module rename and the base package split, now measured and deprioritised.** Section 6.1 measures the split's actual payoff: the cut pays 35 percent of a compile's blocks and delivers none of the "pay for what you import" property, because the cost is interface bytes read rather than sources compiled. The rename remains the largest single edit in the plan and it is still not in this revision. What replaced it as the thing worth doing is lazy interface loading: the loader should read the closure of a module's resolved imports and injected prelude rather than the whole facade, which measured -86 percent for a `Data.List` program and -37 percent for the toolkit. That is a loader change, not a packaging one, and it is not in this revision either.
- **`Data.Machine`'s interface is the single largest line item in every compile.** 581,881 bytes and 1,682 entries, of which 820 are foreign declarations averaging 332 bytes each. It is 38 percent of the facade closure's bytes and 49 percent of its entries, and the facade re-exports it unconditionally, so every program with the default prelude loads all 820 of those foreign declarations. Dropping that one re-export measured a 26 percent cut. Whether to curate it out of the facade, or to make the foreign table load lazily, is a curation decision this note does not take.
- **Offline bootstrap.** Registry delivery plus an optional vendored snapshot, as in section 2, or strict network-on-first-use.
- **Prelude diagnostics.** The mechanism is settled and shipped (section 3): injected import, identity by declaring module, re-exports tracked, one name when two paths reach one declaration. What remains is message quality for an ambiguous instance: today it names the class and the type, and a prelude collision should also name both preludes and both declaring modules.
- **Link policy.** Whether `libs` and `pkgconfig` are open to any namespace or gated to vetted ones, and whether `cinch pkg verify` refuses a lock whose resolved link record is missing. The semantic, raw, and local tiers are settled in section 4.3; what remains is who may publish each tier.

# 8 Retiring the per-op primitive implementations

The direction the user set: the primitive operations should be written in Cinch with LLVM IR bodies, not implemented per op in Rust or special-cased per op in the compiler. This section records what stands in the way and the order that keeps the tree green.

What exists today, measured:

| Surface | Count | Where |
| --- | --- | --- |
| Compiler primitive operations | 132 variants, 113 name arms | `PrimOp` in `compiler/src/core.rs`, lowered in `compiler/src/codegen.rs` |
| Runtime exports | 187 `#[no_mangle]` functions | `compiler/runtime/src/*.rs`, mostly `abi.rs` |
| Runtime symbols the prelude declares | 87 | `cinch.runtime.*` in `compiler/prelude/*/*.cinch` |
| Machine scalar and float symbols the prelude declares | 830 names in 117 families, most of them ten widths each | `cinch.prim.*` |

The mechanism works: a `prim` with an `unsafe llvm { ... }` body can carry inline assembly or any IR fragment, and containment records the trust as an assumption. Verified on the current compiler with a body whose IR is `call i64 asm sideeffect "mov $0, $1", "=r,r"(i64 %0)`: it checked, emitted, linked and ran. Linux syscalls want exactly that shape, with one body per architecture guarded by `when`, because the register names differ between `x86_64` and `aarch64`.

Three constraints decide the order:

- **The prim surface and the internal word representation are different widths.** A `prim :: Bool -> Bool -> Bool` body sees `i8` arguments while the compiler's own lowering works on `i64` words, and a body that returns `i64` is refused with "does not return the declared result shape (i8)". So a converted op is a real function at the canonical width with marshalling at its call sites, not a drop-in for the word-level lowering. That is fine for inlining but it means each conversion has to state which representation it is written against.
- **The checker constructs some prims itself.** Negating a boolean term builds a `Prim(NotBool)` node; the totality checker folds boolean and comparison prims into predicates; the staging interpreter evaluates them at compile time. Retiring a variant therefore needs either an internal `Core::Prim` node that the surface no longer names, or a rewrite of those three passes to treat every prim body generically.
- **`cinch.runtime.*` is the other half.** Bignum `Nat`/`Int`, bytes, arrays, tasks, I/O and the fault path are Rust functions that the prelude declares. Moving them into Cinch prims means writing limb loops, allocation calls and scheduling in Cinch, on top of prims that reach the allocator.

Order that keeps each step verifiable:

The 830 names are 117 families repeated across widths (`bitAnd`, `checkedAdd`, `leadingZeros`, `checkedI8To`, and so on), so the migration is generated rather than hand written: one IR body template per family, specialised per width, then the family's `PrimOp` arm deleted. That is also why step 1 is worth doing first: it converts the largest surface with the least checker risk.

1. **Surface-only machine ops.** The float operations and the machine conversions that no checker pass constructs are the easy majority: each becomes a `prim` with an `llvm` body over the canonical width, and its `PrimOp` variant and codegen arm are deleted. Oracle: `compiler/tests/primitives.rs`, the machine conversion suites, and an IR assertion per family.
2. **Ops the checker folds.** The concrete shape for this step, read off the three call sites: `elab_cond` detects the prelude's `not` by name and currently lowers it to an internal `Prim(NotBool)` node; `check_total` folds boolean and comparison prims into predicates; and the staging interpreter evaluates them as values. Converting the prelude's `not` into a prim with an `llvm` body while the checker keeps building `Prim(NotBool)` would leave two representations of one operation. The conversion therefore has to choose per site: the condition elaborator should emit a call to the prelude's unit (which it can resolve, because it already recognized that definition), the fold tables should key on the resolved unit rather than on a variant, and the interpreter should evaluate any prim body it meets rather than a known list.
   Oracle: the `safety` and `total` suites, which already assert folded proofs.
3. **Runtime-carried ops.** Bytes, arrays and bignum move after a Cinch-level allocator prim exists; that prim is the runtime's own allocator behind one symbol, which is the smallest possible Rust surface and the one place the no-faults rules must stay.
4. **OS wrappers.** `cinch_fs_*`, `cinch_net_*`, `cinch_proc_*`, `cinch_pty_*`, and `cinch_signal_*` become Cinch modules over the syscall prims this revision added, which is also when `sys/linux.rs` loses its Rust `asm!` and the runtime keeps only `sys` and the allocator side.

Each step bumps the runtime ABI and invalidates the store, which is expected: the primitive identity is part of the artifact, so a converted op is a different artifact by construction.

# 9 Aspects this design has not yet faced

Collected while reviewing the proposal against the tree. Each entry says why it matters and what it would take; none of them is answered above.

**Target identity (fixed).** `TargetProfile` now carries `os`, `arch`, `libc`, and `os_version` beside a real triple (`aarch64-apple-darwin`), the profile identity in the store key includes them, and `--target`/`--target-version` set them. `--target` accepts only the host triple and refuses another with the reason, because a cross compile needs a runtime built for the target. `os_version` comes from the flag and never from the host, so a condition on it refuses rather than making the lock machine-specific.

**Preludes need only dependency tracking, and the tracking costs two things.** The user's rule: compile with the prelude injected as an implicit module import, then let module dependencies and re-export origins answer whether `PreludeA` and `PreludeB` share one `List` and one instance set. That removes the whole question of a global prelude, and it leaves two pieces of work. First, the injected module must be in the module's recorded dependency set and in the check order, or fixity and eager-reference collection see a module that names things it never imported. Second, diagnostics have to resolve through re-exports: today an `ambiguous instance` names the class and the type, and a prelude collision needs the declaring module and the prelude path for each side, otherwise the reader cannot tell "one instance re-exported twice" from "two instances with the same shape".

**Raw link flags are trust, so structure them for audit and reproducibility, not for sandboxing.** The user's objection is correct: a project that links a native library runs that library's code before `main` and inside its address space, so no arrangement of flags makes linking safe, and pretending otherwise would only cripple the FFI. The reasons to structure the field anyway are different ones. Reproducibility: a structured `rpaths`/`libdirs`/`soname` prevents the everyday failure where a lock records `-L/opt/homebrew/opt/openssl@3/lib` and no other machine can build. Audit: a declared `libs: ["ssl"]` can be reviewed across a closure, while an opaque argument list cannot, and a leaf package's arguments reach the final link of consumers that never import it, so the blast radius of an unreviewable field is the whole build. Blast radius again: `-Wl,--script`, `-nostdlib`, `-Wl,-e`, `-Xlinker`, and response files change what the produced binary *is*, which a package that merely wants OpenSSL never needs. What does deserve an explicit statement is the semantic consequence: a linked native library can run code at load time, so a program's effect row and its no-faults claim cover the checked fragment only, and that should be said in the package's documentation and in the language's own claims rather than implied away.

**`pkg-config` is neither hermetic nor portable (handled, with a caveat).** Its output carries absolute paths (`-L/opt/homebrew/opt/openssl@3/lib`), `.pc` files can inject arbitrary flags, macOS ships no `pkg-config` at all, and cross-compilation needs `PKG_CONFIG_SYSROOT_DIR` and `--define-prefix`. Recording the resolved words in the lock hides the problem instead of solving it: the lock then contains machine-specific paths. The durable record is a soname plus a version range to check at runtime, with library paths required to be relative or `@loader_path`-based unless the package came from a local path dependency.

**Source packages versus binary packages.** If a registry ships `package.o`, that object is bound to one triple, one runtime ABI, one layout table, and the exact base build it was compiled against, so a base upgrade invalidates every binary in the store and every dependent's lock. Decide which the registry publishes: source only (rebuild at install, slow but portable), or binary with automatic rebuild on mismatch (fast but the mismatch is a compile in disguise). The lock should record which it got, and the provenance claim should say "built from these sources by this compiler" rather than only naming the builder.

**Capability vocabulary for the new surface.** `Io r` is one blanket authority. Filesystem, network, clock, entropy, and watch are different authorities, and the whole design of this language is that a signature tells you what a function may do. Six new packages behind one `Io r` label would quietly undo that: a package that only reads files would look identical to one that opens sockets. The narrow labels probably want to be `Fs r`, `Net r`, `Clock r`, `Entropy r`, `Watch r`, with a rule that a package's public API names the narrowest one it needs.

**Foreign effects must be discharged at the package boundary.** A public signature carrying `Foreign Lib` or `Diverge` pushes that requirement onto every consumer. The base family already shows the discipline (wrappers turn platform words into named values in `Result`), and the package rules need to say it: the boundary module owns the foreign effect, the exported API does not leak it.

**Stage safety.** Compile-time staging means a package can run during compilation. A networking or filesystem package imported by a `stage` function performs I/O at build time, which the staging design confines to `StageFile`, `StageSecret`, `StageProcess`, and `StageEmit`. Packages need a declared stage class, and `stage` code that reaches a runtime-only effect should be refused rather than silently doing the work in the compiler process.

**Caches keyed on the resolved module set.** `package_key` folds in the profile identity, which contains the triple, plus the sources, so a changing `select` result changes the key. The incremental session is the weaker spot: its dirty sets are keyed by declaration hashes, and nothing today says a cross-target session must not reuse units whose module list differs. Either the session key grows the target, or the session is per-target by construction.

**Namespace reservation and the review surface `select` creates.** Nothing stops a registry publisher from taking `Unix.Stat` before the official package does, and nothing reserves `Net.*`, `Crypto.*`, `Sql.*`, or `Term.*` for first-party releases. Separately, `select` is a place to hide code: a manifest with three platform branches means a reviewer who installs on Linux never compiles the Darwin branch. The registry should review every declared branch, `cinch pkg info` should print all of them, and reserved roots should be enforced by the compiler, not only by policy.

**The testing and cross-build cost is the real one.** A package that declares three targets promises three checked builds, and checking them requires cross-compiling `cinchruntime` itself: Rust targets, a sysroot, and a libc for musl. Decide whether the toolchain supplies those or the user's environment must, and make `cinch pkg verify --matrix` the gate that a release has to pass. This is also where the syscall layer pays or does not: one `sys` module per target is one cross-build configuration per target.

**Documentation follows the conditions.** The support matrix needs a per-target column, generated library pages need `available_for` on every platform module, and the package pages need to show `select` in human form. Otherwise the only place a platform constraint is visible is a compile error.
