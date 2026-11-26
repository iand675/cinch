# Compilation, interfaces, and performance

This document covers Cinch's compiled module interfaces (`.cinchif` and `.cpk`), executable library objects, incremental and modular compilation, and the architectural choices that govern compilation speed.

Normative module rules live in [`specifications/modules-and-compilation-specification.md`](../specifications/modules-and-compilation-specification.md). User documentation lives in [`reference/07-modules-and-packages.md`](../reference/07-modules-and-packages.md) and [`reference/18-artifacts-and-incremental.md`](../reference/18-artifacts-and-incremental.md).

---

## 1. Compiled interfaces (`.cinchif` and `.cpk`)

Both `seed/` and `compiler/` speak **interface schema 10** and **runtime ABI 10**.

### Contract vs implementation separation
A `.cinchif` file records a module's complete checked public contract in canonical JSON:
- Module identity, imports, parameter signatures, and instantiation units (`unit_id`).
- Exported types, constructors, schemes (including Rank-N polytypes, refinements, and dependent indices), fixity tables, classes/signatures, and instances (with cross-module coherence metadata).
- Verified contracts: safety assertions (`#[no-alloc]`, JPL profile), temporal contracts, class laws, assumptions (`assumptions.declared`), foreign ABI contracts, and data-parallel kernel eligibility (`cpu_parallel`/`gpu_metal`).
- `iface_hash`: a 64-bit FNV-1a digest over the canonical contract projection with `source_hash`, `compiler`, and `units` cleared. Body-only edits that preserve the contract keep the same `iface_hash`, enabling early-cutoff incremental builds.

### Packed interface closures (`.cpk`)
Loading hundreds of individual JSON `.cinchif` files per invocation dominated cold frontend startup. A `.cpk` file packs an entire transitive dependency closure into a single memory-mapped binary archive with an indexed header table and canonical payloads, validated up front before parsing so operator fixities and dependency hashes are authenticated in one pass.

---

## 2. Executable library objects

Cinch supports two distinct object-linkage mechanisms so clients do not re-compile and re-emit the entire base library or installed dependencies on every build:

1. **Installed package objects (`--package-object`):** `cinch build`, `cinch run`, and `cinch jit` accept prebuilt `package.o` artifacts produced by `cinch pkg install`. The compiler validates the package's checked execution interface (`build/package.cpk` / `.cinchif`), emits external declarations for imported symbols, and links `package.o` directly.
2. **Standalone closed library objects (`cinch emit-object`):** emits a closed native object (`library.o`), its LLVM IR (`library.ll`), physical ABI manifest (`library.abi.json`), and packed interface closure (`.cpk`).
   - Because Cinch specializes generic functions by representation-class vector on demand, a library object cannot anticipate every concrete instantiation a future client might need.
   - `--object-roots roots.json` specifies the concrete specialization and dictionary roots to include in the object. If a downstream build requests a variant absent from the provider object, the compiler reports the missing roots in `requested-roots.json` (used by `compiler/pins/library-object-roots.json` and `compiler/pins/module-object-roots.json`).
   - Because ABI 10 uses self-describing block headers (no layout tables), library objects link with zero layout registration or cross-object numbering conflicts.

---

## 3. Incremental, parallel, and modular compilation

### Query-based incremental checking (`seed/src/query.rs`, `seed/src/incremental.rs`)
- Module compilation keys on source hashes, transitive dependency `iface_hash` values, target profile, and compiler/ABI versions in the local content-addressed store (`$CINCH_STORE`).
- Early cutoff: when a module's source changes without changing its `iface_hash`, downstream modules reuse their cached `.cinchif` and object artifacts without rechecking.
- Persistent server mode (`--serve-ir` in `compiler/`, `cinch jit --native-stage --serve` in `seed/`) keeps decoded contracts and validated object metadata resident in memory across edit-save cycles.

### Modular parallel compiler builds (`tooling/module_build.py`)
For rapid iteration on the self-hosted compiler without running the full single-module bootstrap chain, `tooling/module_build.py` compiles the `compiler/*.cinch` module graph one module at a time in parallel against its imports' interfaces, caching per-module objects by content and discovering missing specialization roots automatically.

### Deterministic IR sharding and PGO (`tooling/shard_ir.py`, `tooling/bootstrap.py`)
Compiling a monolithic multi-hundred-megabyte LLVM `.ll` file sequentially in `clang` was the primary wall-time bottleneck of the bootstrap chain:
- **Deterministic IR sharding:** above `-j 1`, `tooling/shard_ir.py` partitions functions by module prefix into `K` balanced shards (grouping `.app`/`.clo`/`.blk`/`#tail` companions together, inserting `declare`/`external` stubs and duplicating `internal` definitions as needed), compiles shards in parallel, and links objects in deterministic shard order. Because the partition is a pure function of IR text and `K`, Stage 2 and Stage 3 remain byte-identical.
- **Profile-guided optimization (PGO):** Stages 2 and 3 link with `-fprofile-generate` -> toolkit training workload -> `llvm-profdata merge` -> `-fprofile-use` by default (`--no-pgo` disables). Shards compile from content-addressed paths (`build/bootstrap/shards/<ir-digest>/`) so Clang's path-sensitive profile matching reproduces byte-identical binaries between Stage 2 and Stage 3.

---

## 4. Compilation performance architecture

Key algorithmic improvements that brought self-compilation down by more than an order of magnitude (from ~28 minutes to ~2–3 minutes per stage):

- **Undo-log unification:** replaced quadratic snapshot-copying during trial unification in `seed/` with a rollback trail.
- **Decision-tree pattern compilation:** eliminated fallthrough-row duplication across constructor branches (cutting compiler IR size by ~6x).
- **Direct AST/Type carriers & interning:** eliminated repeated JSON serialization round-trips in `compiler/` (giving `Kind` a direct `UType` field via mutual data groups), interned names (`Sym`), and indexed per-module unit/export symbol tables instead of linear scans.
- **Per-thread small-object runtime & release-profile linking:** linking the `--release` `cinchruntime` with size-classed thread-local heaps cut runtime allocation cost from ~1,800 to ~118 instructions per block and dramatically lowered peak live memory.
- **Compact per-function apply stubs:** replaced per-arity apply-stub fan-out with one stub per closure-taken unit.

### Remaining speed work
- Native multi-core parallel checking inside the self-hosted compiler (`Bootstrap.Parallel`, `Bootstrap.Incremental`).
- Eliminating remaining quadratic lookup and builder copies in `Bootstrap.Ir` and `Bootstrap.Driver`.
- Keeping `tooling/profile_build.py` and `tooling/bench_compiler.py` as the reproducible measurement harnesses across `hello`, `graph`, `symbols`, and `self` workloads.
