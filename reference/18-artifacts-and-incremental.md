# Artifacts, interfaces, stores, and incremental compilation

Every compilation writes three artifacts into `build/` beside the entry file and updates a content-addressed entry in a per-user package store. Understanding these artifacts lets you answer two operational questions directly: which inputs determined a build output, and why a module was rechecked or rebuilt.

The package store is strictly advisory on the CLI compile path: it verifies and writes entries on every compile, while batch CLI invocations always check from source. Cache corruption therefore costs at most a rewrite and never alters program semantics.

# Build outputs

`cinch check`, `emit-ir`, `emit-object`, `build`, and `run` all populate `build/` beside the entry source file (regardless of `-o`). For an entry module `Main.cinch`:

```text
build/
  Main.cpk               packed binary-indexed interface package for the module closure
  Main.manifest.json     physical build, target, trust, and provenance record
  Main.stage.snap        captured compile-time staging inputs (one record per line)
```

For `check`, `emit-ir`, `build`, and `run` targeting the same profile, these outputs are deterministic and rewritten on every invocation. The only file read back from a prior invocation is `<Entry>.stage.snap`, and only under `CINCH_STAGE_MODE=replay`.

| Artifact | Contents | Consumers |
| --- | --- | --- |
| `<Entry>.cpk` | Packed `.cinchif` interface contracts for every module in the checked closure (user modules, vendored packages, and imported `cinch/stdlib` modules), plus optional unfoldings when `--emit-interface-optimisations` or `emit-object` is active | Downstream packages, `cinch doc`, JIT, object admission |
| `<Entry>.manifest.json` | Target profile, module contract hashes, assumptions, safety assertions, specialization report, staging log, and checked-vs-trusted lowering boundary | Build audits, tooling, native/JIT admission |
| `<Entry>.stage.snap` | Recorded compile-time I/O captured inside `snapshot` blocks | Replay builds (`CINCH_STAGE_MODE=replay`) |

# Module interfaces (`.cinchif`) and `.cpk` packages

Each checked module has a canonical interface contract (`.cinchif`, schema version `10`, runtime ABI `10`). On disk, the compiler packs the entire checked closure into a single binary-indexed `<Entry>.cpk` archive (or `build/package.cpk` for installed packages and `interfaces.cpk` in the package store), avoiding per-module file sprawl while allowing consumers to borrow individual contracts and optional unfoldings without re-parsing JSON. Standalone `.cinchif` JSON documents use the same schema and are accepted by tools such as `cinch doc --iface`.

Logically, one module's `.cinchif` contract has the following canonical JSON structure:

```json
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
  "runtime_abi": 10,
  "schema": 10,
  "source_hash": "8502167534619123955",
  "units": [],
  "upgrades": []
}
```

| Field | Meaning |
| --- | --- |
| `schema` | Interface schema version (`10`). |
| `runtime_abi` | Runtime ABI contract version (`10`). |
| `language` | Surface language version (`draft-0.2+amendments`). |
| `compiler` | Compiler package version (`0.1.0`). |
| `module` | Dotted module identity. |
| `source_hash` | 64-bit FNV-1a digest of the module's source text, formatted as a decimal string. |
| `params` | Module parameters (for example `["S :: Api.Stepper"]`). |
| `imports` | One entry per imported module recording the `iface_hash` it was checked against. |
| `exports.values` | Exported value schemes, emitted symbol names, and `total_body` graphs where applicable. |
| `exports.types` | Exported type constructors, tags, and resolved schemes. |
| `exports.signatures` | Type classes, methods, visibility, and associated laws. |
| `exports.instances` | Dictionary instances (`class`, `target`, `implicit`, method identities). |
| `abi` | Calling convention, explicit layout entries, and exported symbol mappings. |
| `assertions` | Verified safety assertions per definition (hashed into `iface_hash`). |
| `assumptions` | Primitives, foreign symbols, and unverified declared assumptions the module depends on. |
| `total_body` | Closed, sorted first-order body graph for supported monomorphic pure definitions, or `null`. |

## Contract hashing and interface chain verification

`source_hash` and `iface_hash` are 64-bit FNV-1a digests rendered as decimal strings (used for change detection and consistency, not cryptographic authentication).

- `source_hash` covers the full source text.
- `iface_hash` covers only the exported contract projection (schemes, layouts, instances, verified assertions, and assumptions). Editing the body of a private helper changes `source_hash` while leaving `iface_hash` unchanged, so downstream dependents do not go stale.
- `total_body` is emitted only for supported monomorphic pure signatures; consumers validate the defining root, carriers, scoping, and acyclicity before trusting it.
- The `imports` array forms a hash-linked dependency chain. Before accepting a loaded artifact set, chain verification recomputes each imported module's `iface_hash` and rejects any mismatch or missing dependency so stale interfaces cannot silently mix.

# The build manifest

`build/<Entry>.manifest.json` records the physical and provenance state of the whole build: target profile, data imports, staging metadata, safety assertions, specialization results, trust level, and the checked-versus-trusted lowering boundary.

```json
{
  "entry": "Main",
  "runtime_abi": 10,
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
  "trust": {
    "level": 2,
    "name": "full-recheck",
    "rechecked": ["<builtin>.newIO", "<builtin>.toNat"],
    "trusted": ["type preservation by construction", "effect-order preservation by construction"]
  },
  "lowering": {
    "checked": [
      "core structure and scoping",
      "usage of %1/%aff parameters",
      "memo and demand sharing",
      "decision-point weights",
      "contract identity"
    ],
    "trusted": ["LLVM code generation", "the runtime and its instruction model"]
  },
  "specialization": {
    "ran": true,
    "fuel": 8,
    "measure": 8,
    "discarded": [],
    "fused": [],
    "representation_unrecorded": 0,
    "representation_versions": ["<builtin>.vecStreamStep@R", "Algebra.Semigroup.combine-pairs@O"]
  },
  "wasm": {
    "artifact": "native",
    "canonical_abi": "none",
    "adapters": [],
    "required": [],
    "lowerings": ["tasks:os-thread"]
  },
  "modules": [{"iface_hash": "13309812562950837481", "module": "Algebra.Arithmetic"}]
}
```

In `specialization.representation_versions`, each specialized unit carries its representation class vector after `@`: `R` (raw word), `O` (object pointer), or `T` (tagged/boxed value).

# The stage snapshot

`build/<Entry>.stage.snap` records compile-time external reads captured inside `snapshot` blocks (see [Staging and macros](14-staging-and-macros.md)). Each line is a six-column tab-separated record:

```text
<kind>	<snapshot-name>	<input-name>	<content-hash>	<recorded|live>	<escaped-content>
```

For example, reading `input.txt` containing `42\n` inside `snapshot "inputs"` records:

```text
file	inputs	input.txt	3157674352912849835	recorded	42\n
```

An empty file indicates that the build captured no external staging inputs. Select the staging mode via `CINCH_STAGE_MODE`:

| Mode | Behavior |
| --- | --- |
| `refresh` (default) | Executes compile-time reads and writes their captured values to `<Entry>.stage.snap`. |
| `replay` | Reads captured values from `<Entry>.stage.snap` without touching the external file or process; fails if a requested input is missing from the snapshot. |
| `live` | Performs live reads without replaying from disk. |

# The package store

The package store is a per-user, content-keyed cache of compiled interface artifacts for the toolchain's standard-library core (`cinch-base`). Deleting it is always safe.

| Platform / environment | Store root |
| --- | --- |
| `CINCH_STORE` set | `$CINCH_STORE` |
| macOS | `$HOME/Library/Caches/cinch/store` |
| Linux / Unix with `XDG_CACHE_HOME` | `$XDG_CACHE_HOME/cinch/store` |
| Other | `$HOME/.cache/cinch/store` |

Each entry lives at `<root>/<package>/<version>/<key>/`:

```text
<root>/cinch-base/0.1.0/<key>/
  interfaces.cpk     checked contract package for the closure
  store.json         entry manifest (schema 3)
```

The store layout also reserves per-module `<Module>.o` (`object`) and `<Module>.ir` (`ir`) slots; the compile path currently writes `interface` (`interfaces.cpk`) entries only.

`store.json` records the entry identity and per-module contract hashes:

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
  "runtime_abi": 10,
  "store_schema": 3,
  "version": "0.1.0"
}
```

## Key derivation

The store key is the 64-bit FNV-1a digest (formatted as a decimal string) over a canonical tab-separated, newline-terminated record containing, in order:

```text
package       package name from the manifest
version       package version
manifest      FNV-1a hash of the manifest bytes
iface_schema  interface schema version (10)
runtime_abi   runtime ABI version (10)
language      surface language version
compiler      compiler package version
profile       rendered target profile fields
adapters      rendered WIT adapter set
module        one row per module in check order: name, path, facade/module kind
source        one row per module: path, FNV-1a hash of its source text
```

Any change to a module's source, the target profile, the WIT adapter set, or the compiler/schema version numbers produces a new key. Note one current limitation during compiler development: changing compiler internals without bumping the compiler version or schema constants leaves the key unchanged, so two development builds can legitimately produce different artifacts under the same key.

## Write policy, atomic replacement, and `--rebuild`

The store has two deliberately asymmetric paths:

- **Write path (used by CLI compiles):** Never fails a build over cache state. Without `--rebuild`, the driver loads any existing entry for the key and compares it against the artifacts just produced. If the entry is missing, truncated, corrupt, or disagrees with the current compile, the driver stages a fresh set in `.{key}.{pid}.{seq}.staged`, removes the old directory, and swaps the staged directory in via an atomic rename. Concurrent readers therefore observe the old entry, the new entry, or nothing—never a partial mix.

| Write outcome | Condition |
| --- | --- |
| `Wrote` | No entry existed for the key. |
| `Verified` | An intact entry existed and matched the current compile byte-for-byte. |
| `Repaired` | An entry existed but was corrupt, mismatched, or overridden by `--rebuild`, and was atomically replaced. |
| `Skipped` | The compilation did not cover the standard-library core package. |

- **Load path (strict verification):** Validates the entry manifest, schema/ABI/compiler identity, every artifact's readability and content hash, and the interface import chain, returning a typed error (`Missing`, `Corrupt`, `KeyMismatch`, `IdentityMismatch`, `ArtifactUnreadable`, `HashMismatch`, `ChainBroken`, `Package`, `Io`) on any discrepancy.

Two operational notes follow from this split:

- **`--rebuild` forces an unconditional rewrite.** Accepted by `check`, `build`, `run`, `emit-ir`, and `emit-object`, it skips the equality comparison and atomically overwrites the store entry from the current compile.
- **No CLI command skips typechecking via the store yet.** Checking installs the base library's elaborated state (schemes, instances, fixities) and codegen requires function bodies, whereas cached `.cpk` entries carry contract projections rather than full elaborated bodies. Store entries currently serve as verified build records rather than a warm-compile bypass.

# Library objects and `--object-roots`

`cinch emit-object` compiles a library module (one with no `main` entry point) into a relocatable native object, its LLVM IR text, and a physical ABI manifest:

```text
module Library where
  export choose

  choose :: forall a. (a -> Bool) -> a -> a -> a
  choose p x y = if p x then x else y
```

```sh
cinch emit-object Library.cinch
```

This writes `Library.o`, `Library.ll`, `Library.abi.json`, and the standard `build/` directory. `Library.abi.json` records `schema`, `runtime_abi`, `target`, `word_size`, `endian`, `reclaim`, `escaped_descriptors`, `layout_entries`, `layout_words`, `types`, `constructors`, `units`, `dictionaries`, `method_families`, `interfaces`, `modules`, `ir_content_hash`, and `object_content_hash`.

Code generation emits only reachable monomorphic and specialized units. Because `choose` above is polymorphic and has no callers inside `Library.cinch`, `Library.o` contains no `Library.choose` symbol unless a consumer declares which concrete specializations it needs via a roots JSON file (`schema: 1`):

```json
{"schema": 1, "units": [{"symbol": "Library.choose", "classes": "T"}]}
```

```sh
cinch emit-object Library.cinch -o Library.o --object-roots roots.json
```

With that root, the emitted object defines `Library.choose@T` (parameters `["O", "T", "T"]`, result `T`, and closure accessor `Library.choose@T.object-closure`). Class letters in representation vectors are `R` (raw word), `O` (object pointer), and `T` (tagged/boxed value).

A roots document may contain three arrays:

- `units` — `{"symbol": "<Module.name>", "classes": "<R/O/T vector>"}`
- `dictionaries` — `{"symbol": "<Module.Dict>", "classes": "<R/O/T vector>"}`
- `methods` — `{"class": "<Module.Class>", "slot": <n>, "own": "<R/O/T vector>"}`

Malformed roots files and invalid mode combinations are rejected explicitly:

| Condition | Diagnostic | Exit |
| --- | --- | --- |
| Non-empty roots file passed outside `emit-object` (`check`, `emit-ir`, `build`, `run`) | `cinch: explicit object roots require emit-object mode` | 1 |
| Unknown symbol | `cinch: unknown object unit root Library.nope` | 1 |
| Invalid class letter | `cinch: object root vectors contain only R/O/T` | 2 |
| Wrong schema or unexpected keys | `cinch: object roots require schema 1 and only units/dictionaries/methods arrays` | 2 |

# Package objects (`--package-object`)

An installed package pairs its compiled native object (`package.o`) with a package interface archive (`build/package.cpk`, falling back to `package.abi.json`) carrying its module list, execution kind, and `object_content_hash`.

`cinch build` and `cinch run` accept precompiled package objects via `--package-object <path>`. Before linking, the driver verifies that the object file's bytes hash to the recorded `object_content_hash` and excludes the package's modules from the project's emitted unit set. A tampered or mismatched object is rejected before the linker runs:

```text
cinch: vendor/acme/greet/0.1.0/package.o: object content hash does not match package.abi.json
```

<span class="support-tag support-tag--partial">Partial</span> **Linking prelude-using projects against `--package-object`.** Admission, hash verification, and module exclusion are active, but linking a project that uses the Prelude against an external package object currently leaves base-library symbols (such as `Data.String.string-concat` and `<builtin>.newIO`) unresolved at link time. Until base-library package linking lands, compile vendored package sources together with the project via `cinch build` or `cinch run`.

# Incremental compilation

The compiler includes a three-layer incremental engine used in-process by `cinch repl` and `cinch lsp`:

1. **Item dependency graph:** Separates *surface edges* (signatures, layouts, instances, contract facts) from *body edges*. Editing a function body rechecks only that declaration; editing a signature or contract fact invalidates the transitive closure of dependents.
2. **Persistent session:** Tracks declaration-level dirty sets across edits and reports `rechecked`, `retyped`, and `kept` modules along with per-rebuild timings.
3. **Declaration-keyed object cache:** Caches and reassembles compiled units keyed by declaration content hashes.

Two boundaries apply today:

- **Whole-program LLVM emission:** Code generation still emits a single LLVM module per program before extracting units for the object cache; per-unit LLVM emission is planned.
- **No CLI watch daemon:** Batch commands (`cinch check`, `cinch build`, `cinch run`) always run once and exit. In-process incremental rechecking runs inside `cinch lsp` and `cinch repl`.

# Diagnosing a stale build

1. **Re-run `cinch check`.** Because the package store cannot fail a CLI compile, any failure from `cinch check` is a real source, import, or interface problem.
2. **Inspect interface-chain errors.** Chain verification reports `M was checked against X at HASH, but the loaded interface is HASH` on a contract mismatch, and `M imports X, but no interface for it is in the artifact set` on a missing dependency.
3. **Check `source_hash` vs. source text.** A mismatch indicates stale generated artifacts from an older source tree.
4. **Remove `build/` and recompile.** Everything under `build/` is regenerated from source on the next invocation (except `.stage.snap` when replaying staged builds).
5. **Pass `--rebuild` or clear the store entry.** Safe at any time; `--rebuild` overwrites the store entry atomically from the current compile.
6. **Isolate stores across checkouts with `CINCH_STORE`.** Because stored keys and metadata include source paths, separate checkouts should use distinct `CINCH_STORE` roots when debugging cache behavior.
