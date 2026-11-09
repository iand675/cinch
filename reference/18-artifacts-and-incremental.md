# Artifacts, interfaces, stores, and incremental compilation

Every compile writes three kinds of artifact beside your sources and one entry in a per-user cache. Knowing what each one is lets you answer the two questions that come up when a build misbehaves: which inputs decided this output, and why was it rebuilt.

“Rebuilt” is the important word. The store is a cache with a write path and a strictly checked read path, and today only the write path is wired into the CLI, so a cache problem costs time and never changes what a program means.

# What one compile writes

`cinch check`, `build`, `run`, `emit-ir`, and `emit-object` all write into `build/` beside the entry file. For a project consisting of one line, `Main.cinch` containing `module Main where` and `main :: Int` returning `0`:

```text
build/
  Algebra.Arithmetic.cinchif     one interface per module in the closure
  …
  Data.Core.cinchif
  …
  Main.cinchif
  Main.manifest.json             the build manifest for the entry
  Main.stage.snap                captured compile-time inputs, one line each
  …                              55 further .cinchif files, one per base module
```

The `build/` directory holds one interface for every module the compile reached, including the whole embedded base library. A one-line program produces 56 interfaces, because the closure is what was checked, not what was written.

Every one of those files is rewritten on every invocation, and the bytes are identical across `check`, `emit-ir`, `build`, and `run` for the same target. Exactly one artifact is ever read back from a previous invocation: `<Entry>.stage.snap`, and only to supply recorded compile-time inputs to a replay build. Nothing in `build/` is used to skip typechecking.

| Artifact | Contents | Consumer |
| --- | --- | --- |
| `<Module>.cinchif` | The checked contract of one module | The next compile, `doc`, object admission |
| `<Entry>.manifest.json` | The physical and provenance record of the whole build | Tools, audits, the JIT and native object admission |
| `<Entry>.stage.snap` | The captured bytes of compile-time inputs | A `CINCH_STAGE_MODE=replay` build |

# The interface file

A `.cinchif` is canonical JSON. This is the complete file for a module whose only declaration is `main :: Int` returning `0`:

```text
{
  "abi": {
    "convention": "uniform-word, curried apply stub",
    "layouts": [],
    "symbols": ["main=Main.main"]
  },
  "assertions": [],
  "assumptions": {"declared": [], "foreign": [], "primitives": []},
  "compiler": "0.1.0",
  "contracts": [],
  "exports": {
    "fixities": [],
    "foreign": [],
    "instances": [],
    "signatures": [],
    "types": [],
    "values": [
      {
        "arity": 0,
        "name": "main",
        "resolved": {"measures": [], "nominals": [], "scheme": "Int"},
        "scheme": "Int",
        "symbol": "Main.main",
        "total_body": {
          "definitions": [
            {
              "body": {"decimal": "0", "node": "number", "sort": "Int"},
              "identity": "Main.main",
              "parameters": [],
              "result": "Int"
            }
          ],
          "version": 1
        }
      }
    ]
  },
  "imports": [{"iface_hash": "12777687666220289504", "module": "Prelude"}],
  "language": "draft-0.2+amendments",
  "laws": [],
  "module": "Main",
  "must_use": [],
  "params": [],
  "runtime_abi": 6,
  "schema": 10,
  "source_hash": "8502167534619123955",
  "units": [],
  "upgrades": []
}
```

Field by field:

| Field | Meaning |
| --- | --- |
| `schema` | Interface schema version, `10` in this compiler |
| `runtime_abi` | Runtime ABI the interface was produced against, `6` |
| `language` | Surface language version, `draft-0.2+amendments` |
| `compiler` | `CARGO_PKG_VERSION` of the producing compiler |
| `module` | The module's identity, dotted |
| `source_hash` | FNV-1a 64 of the module's source text, as a decimal string |
| `params` | Module parameters, for example `["S :: Api.Stepper"]` |
| `imports` | One row per module this one was checked against, with that module's `iface_hash` |
| `exports.values` | Each value's `scheme`, its emitted `symbol`, and its `total_body` if it has one |
| `exports.types` | Each type's constructors, tags, and resolved schemes |
| `exports.signatures` | Each class's methods, with `exports` visibility and any `laws` |
| `exports.instances` | Each dictionary: `class`, `target`, `implicit`, method identities |
| `abi` | Calling convention, layout rows, and symbol names |
| `assumptions` | Primitives, foreign symbols, and declared assumptions the module relies on |
| `total_body` | A closed, sorted first-order body graph, or `null` |

`source_hash` and `iface_hash` are 64-bit FNV-1a digests rendered as decimal strings, not cryptographic digests. `iface_hash` covers the contract projection, so an edit to a private helper body changes `source_hash` and leaves `iface_hash` alone. `total_body` is what makes a body usable by a consumer: it is emitted only for a supported monomorphic pure signature, and a consumer validates the defining root, the carriers, the scope, and the acyclicity before trusting it.

The `imports` rows form a chain. `verify_chain` re-derives each imported interface's hash from the artifact set and refuses the build when a row does not match, so a stale interface cannot be mistaken for the checked one.

# The build manifest

`<Entry>.manifest.json` is the physical record of the build: modules and their interface hashes, target features, assumptions, specialization results, staging inputs, data imports, trust level, and the checked-versus-trusted lowering split.

```text
{
  "entry": "Main",
  "runtime_abi": 1,
  "target": {
    "profile": "native64",
    "endian": "little",
    "word_size": 64,
    "vector_shape": "fixed(8)",
    "features": ["u32.wrappingAdd"],
    "admission": "the deployment contract must satisfy the recorded feature assumptions",
    "conditions": []
  },
  "data_imports": {
    "bindings": [
      {"converter": "txt", "file": "note.txt", "hash": "11831194018420276491", "module": "Main", "name": "note"}
    ]
  },
  "staging": {"compiler": "0.1.0", "generated": {}, "inputs": [], "mode": "refresh"},
  "trust": {"level": 2, "name": "full-recheck", "rechecked": ["<builtin>.newIO", "<builtin>.toNat", "…"], "trusted": ["type preservation by construction", "effect-order preservation by construction"]},
  "lowering": {
    "checked": ["core structure and scoping", "usage of %1/%aff parameters", "memo and demand sharing", "decision-point weights", "contract identity"],
    "trusted": ["LLVM code generation", "the runtime and its instruction model"]
  },
  "specialization": {"ran": true, "fuel": 8, "measure": 8, "discarded": [], "fused": [], "representation_unrecorded": 0, "representation_versions": ["<builtin>.vecStreamStep@R", "Algebra.Semigroup.combine-pairs@O", "…"]},
  "wasm": {"artifact": "native", "canonical_abi": "none", "adapters": [], "required": [], "lowerings": ["tasks:os-thread"]},
  "modules": [{"iface_hash": "13309812562950837481", "module": "Algebra.Arithmetic"}]
}
```

`runtime_abi` is `1` here while the interface carries `6`. They are different counters: the manifest's is the manifest schema, the interface's is the runtime contract. `trust.rechecked` lists the units this build actually rechecked, and `specialization.representation_versions` lists every specialized variant, with the class vector appended after `@` (`@R` raw, `@O` object, `@T` tagged object).

# The stage snapshot

`<Entry>.stage.snap` has one tab-separated line per captured compile-time input, in six fields: kind, snapshot name, input name, content hash, whether the value is live or recorded, and the captured content with backslash escapes.

```text
file	inputs	input.txt	3157674352912849835	recorded	42\n
```

That line is a program that called `snapshot "inputs"` around a `readFile` of `input.txt` containing `42` and a newline. The snapshot name is the label the generator passed to `snapshot`; the input name is the file or subprocess. An empty file means the build captured nothing.

The three stage modes are selected with `CINCH_STAGE_MODE`: `refresh` (the default) executes the reads and records them, `replay` uses the recorded values and refuses a missing input, and `live` performs current reads. `replay` after a `refresh` build still produces the same value when the input file has been deleted, which is the point of the file.

# The package store

The store is a per-user cache of compiled artifacts, keyed by content. It is advisory: deleting it costs time and nothing else.

| Platform | Default root |
| --- | --- |
| Any, when `CINCH_STORE` is set | `$CINCH_STORE` |
| macOS | `$HOME/Library/Caches/cinch/store` |
| Elsewhere with `XDG_CACHE_HOME` | `$XDG_CACHE_HOME/cinch/store` |
| Elsewhere | `$HOME/.cache/cinch/store` |

An entry is `<root>/<package>/<version>/<key>/`, holding one artifact per module plus the entry manifest:

```text
$HOME/Library/Caches/cinch/store/cinch-base/0.1.0/13085239440471859661/
  Algebra.Arithmetic.cinchif
  Algebra.Bits.cinchif
  …
  store.json
```

`store.json` records what the entry is:

```json
{
  "compiler": "0.1.0",
  "iface_schema": 10,
  "key": "13085239440471859661",
  "kinds": ["interface"],
  "language": "draft-0.2+amendments",
  "modules": [
    {"bodies": [], "iface_hash": "5562156715741670451", "kinds": ["interface"], "module": "Data.Core"}
  ],
  "package": "cinch-base",
  "profile": "native64",
  "runtime_abi": 6,
  "store_schema": 2,
  "version": "0.1.0"
}
```

`kinds` lists the artifact kinds the entry holds per module: `interface` for `.cinchif`, `object` for `.o`, `ir` for the module's IR form. The layout admits all three, and today the compile path writes interfaces only.

## How the key is computed

The key is the 64-bit FNV-1a digest of a tab-separated, newline-terminated record set, rendered as a decimal string, over these fields in this order:

```text
package       the manifest's name
version       the manifest's version
manifest      FNV-1a of the manifest bytes as read
iface_schema  the interface schema constant
runtime_abi   the runtime ABI constant
language      the language version string
compiler      CARGO_PKG_VERSION
profile       one line of target profile fields
adapters      the WIT adapter set, rendered
module        one row per module in the manifest's check order: name, path, facade-or-module
source        one row per module: path, FNV-1a of its text
```

Because the manifest bytes and every module's text are inputs, editing a base module moves the key. Because `CARGO_PKG_VERSION` and the schema constants are inputs, a compiler release that changes either moves the key for every project. A change to compiler code that leaves all of those alone does **not** move the key, which the module documentation in `store.rs` names as a known limit of this stage: two builds of the compiler can legitimately produce different artifacts under one key.

## What the write path does

One of four outcomes, none of which fails a build:

| Outcome | When |
| --- | --- |
| `Wrote` | No entry existed for the key |
| `Verified` | The entry existed and is exactly this compile's artifact set |
| `Repaired` | The entry existed but was not this compile's set, and was rewritten |
| `Skipped` | The compile's interfaces are not the embedded package's, so there is nothing to key |

`Repaired` covers a truncated artifact, an aborted write, a different kind set, and an explicit `--rebuild`. It is not an error, because nothing was consumed: a corrupt entry never fails a build, and the next compile rewrites it from the artifacts in hand. The suite pins this: `compiler/tests/package_store.rs` truncates a stored interface, reruns `cinch check`, and requires success plus byte-identical restoration.

Two consequences follow, and both are worth knowing before you go looking for a cache problem:

- `--rebuild` forces the entry to be rewritten even when it is intact and matches this compile. It is accepted by `check`, `build`, `run`, `emit-ir`, and `emit-object`.

The write path does compare against the entry. Without `--rebuild` it calls `load` and then `agrees_with_compile`, and returns `Verified` only when the stored set is exactly the one this compile just produced. A load error is not a fault on this path: `write` swallows it and rewrites, which is why a corrupt entry never fails a build and never reaches you as a diagnostic. The named hard errors belong to `load` (`Missing`, `Corrupt`, `KeyMismatch`, `IdentityMismatch`, `ArtifactUnreadable`, `HashMismatch`, `ChainBroken`, `Package`, `Io`), and no CLI path consumes stored artifacts to skip checking, so no command can surface one. `compiler/tests/package_store.rs` drives `load` directly.

A write stages the whole set in `.{key}.{pid}.{seq}.staged` inside the package directory, removes the previous entry directory, and swaps the staged one in with a single rename. A reader therefore sees the old set, the new set, or nothing, and never a mixture, which matters because two builds of the compiler can render the same sources differently under one key.

Nothing is reused yet, and `store.rs` states why: a warm compile cannot skip the base library without its bodies, because the checker installs the family's checked state (schemes, instances, fixities) and codegen needs the family's functions, while an interface carries rendered contracts rather than that state. The entry is a verified record of what this compiler produced, not yet a substitute for producing it.

# Objects and object roots

`cinch emit-object` produces a library artifact: a native object, its LLVM text, and a physical ABI manifest. Starting from this file:

```text
module Library where
  export choose

  choose :: forall a. (a -> Bool) -> a -> a -> a
  choose p x y = if p x then x else y
```

```sh
cinch emit-object Library.cinch
```

```text
Library.ll          10,288,671 bytes   the LLVM text
Library.o            2,871,480 bytes   the native object
Library.abi.json     1,903,945 bytes   the physical manifest
build/                                the interfaces, manifest, and snapshot
```

`Library.abi.json` carries `schema`, `runtime_abi`, `target`, `word_size`, `endian`, `reclaim`, `escaped_descriptors`, `layout_entries`, `layout_words`, `types`, `constructors`, `units`, `dictionaries`, `method_families`, `interfaces`, `modules`, `ir_content_hash`, and `object_content_hash`.

Only reachable units are emitted. `choose` is polymorphic and nothing in the file reaches a concrete instance of it, so without a demand the object contains no `Library.choose` unit at all. A separately compiled consumer states what it needs in a roots file:

```json
{"schema": 1, "units": [{"symbol": "Library.choose", "classes": "T"}]}
```

```sh
cinch emit-object Library.cinch -o Library.o --object-roots roots.json
```

The object now defines `Library.choose@T`, with `params` `["O", "T", "T"]`, result `T`, and closure accessor `Library.choose@T.object-closure`. Class letters are `R` (raw word), `O` (object pointer), and `T` (tagged/boxed). The three tables in the roots file are `units`, `dictionaries`, and `methods`, the last naming a class, a logical slot, and the method's own class vector.

| Malformed roots file | Diagnostic | Exit |
| --- | --- | --- |
| Unknown symbol | `cinch: unknown object unit root Library.nope` | 1 |
| Bad class letter | `cinch: object root vectors contain only R/O/T` | 2 |
| Wrong schema | `cinch: object roots require schema 1 and only units/dictionaries/methods arrays` | 2 |

`--object-roots` is accepted on the command line of any mode, but only `emit-object` uses it; a roots file passed to `check`, `build`, or `run` is parsed and then ignored.

# Package objects in a build

An installed package ships its own object and a package-level `.cinchif`. `build/package.cinchif` is the ABI manifest with three fields added, so a consumer reads one artifact instead of two:

```json
{"kind": "package-execution", "modules": ["Greet"], "object": "package.o", "object_content_hash": "15519496164294781924"}
```

`cinch build` and `cinch run` accept these objects with `--package-object`. The flag reads the object's `build/package.cinchif` (falling back to `package.abi.json`), checks that the object's bytes still hash to `object_content_hash`, and excludes the modules that object defines from the project's own emitted set.

A tampered object is refused before linking:

```text
cinch: vendor/acme/greet/0.1.0/package.o: object content hash does not match package.abi.json
```

<span class="support-tag support-tag--partial">Partial</span> **linking a project against package objects.** The admission and exclusion steps work, and the link step does not yet produce a working binary for a project that uses the prelude. Excluding the package's modules leaves only the project's own modules in the emitted set, so base library symbols such as `Data.String.string-concat` and `<builtin>.newIO` are neither emitted nor supplied by an object, and `clang` fails with undefined symbols. The same project builds and runs with `cinch run ./Main.cinch`, which compiles the vendored sources together with the prelude.

# Incremental compilation

`incremental.rs` implements three layers over the same interfaces: an item graph with surface and body edges (`ItemGraph`), a persistent session with declaration-level dirty sets (`Session`), and a declaration-hash-keyed object cache for assembly (`Session::link_units`). A body-only change rechecks the changed declaration; a signature or contract change rechecks the dependents' closure. The report states `rechecked`, `retyped`, and `kept`, plus real timings per rebuild.

Two limits are worth stating plainly. Codegen emits one LLVM module per program, so unit extraction for the object cache runs after a whole-program emission; the cache still keys, diffs, and assembles at declaration granularity and the assembly is verified against the emission. And there is no watch mode or session daemon in the CLI: `cinch build` compiles and exits, and an editor that uses the analysis API does not imply a persistent compiler process between shell invocations.

The [design note](../design/incremental-recompilation.md) records the goals and the remaining work, including per-unit emission. Latency claims in that note are targets, not measurements.

# Diagnosing a stale build

1. Re-run `cinch check`. Store entries cannot fail it, so a failure is a real source or interface problem.
2. Read the interface chain. `verify_chain` reports `M was checked against X at HASH, but the loaded interface is HASH` when two interfaces disagree about a contract, and `M imports X, but no interface for it is in the artifact set` when one is absent.
3. Compare `source_hash` in the `.cinchif` with the source you expect. A mismatch means the build directory is older than the source tree.
4. Delete `build/` and recompile when in doubt. It is regenerated in full, and nothing else reads it.
5. Delete the store entry, or pass `--rebuild`, when you suspect the cache. Both are always safe: the entry is keyed by content, compared against the current compile on every write, and never consumed in place of compiling.
6. Keep test stores separate per checkout with `CINCH_STORE`. Cached artifacts record source paths, so a store written by another checkout can name paths that this one does not have.

Implementation: `driver.rs` (`write_artifacts`), `interface.rs` (schema, hashing, `verify_chain`, `check_coherence`), `store.rs` (layout, key, write and load), `incremental.rs`, `specialize.rs` (`parse_object_roots`); suites: `incremental.rs`, `package_store.rs`, `canonical_artifacts.rs`, `interface_*`, `intrinsic_objects.rs`, `bootstrap_library_object.rs`.
