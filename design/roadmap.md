# Roadmap and open decisions

Current engineering priorities and unsettled design questions across the Cinch compiler, runtime, and ecosystem. Feature-by-feature compiler support status is tracked in [`reference/16-support-and-roadmap.md`](../reference/16-support-and-roadmap.md); self-hosting checkpoint details live in [`compiler/CHECKPOINT.md`](../compiler/CHECKPOINT.md).

---

## 1. Active engineering priorities

### Self-hosting parity (Acceptance 5)
Stages 1–3 reach a verified byte-for-byte fixed point on schema 10 / ABI 10, but full frontend parity with `seed/` across the entire test suite remains in progress:
- Close remaining self-hosted frontend gaps: foreign/intrinsic elaboration edge cases, refinement/precondition discharge in nested contexts, staging/quotations, and remaining unmodeled declarations (`upgrade`, attribute annotations, module parameters).
- Eliminate memory leaks in the Stage 3 compiler process so `tooling/run_stage_targets.py` passes with `CINCH_LEAK_CHECK=1`.
- Resolve the Darwin `tailcc` codegen bug blocking Stage 0 seed pin advancement past `cd04d7a2`.
- Complete the `Bootstrap.*` -> `Cinch.*` compiler module modernization.

### Reclamation, layouts, and runtime
- **Cycle reclamation:** implement region-scoped teardown for cyclic `letrec` knots and memo cells (`seed/runtime/CYCLE-COLLECTOR-DESIGN.md`).
- **Exact RC classes:** replace remaining conservative `Raw` slots (memo/knot values, wide joins, compile-time constants) with exact classes.
- **Allocators & budgets:** expose the user-supplied program allocator interface, scoped `Heap r` capability with arenas, typed `Exhausted` budget boundaries, and `#[fip]` / `#[alloc "bounded(n)"]` verification.
- **Layouts:** implement `#[repr(unboxed)]` value types, sub-word field packing for 32-bit/microcontroller targets, and proof-justified representation instances (`Represent t c`).
- **Cross-platform runtime verification:** run full execution tests for Linux (`x86_64` and `aarch64`) stack guard bands, `x86_64` fiber context switching, and Wasm component linking.

### Type system and verification
- **Float refinements:** implement Tier 1–2 float predicates (`NotNaN`, `Finite`, `InRange`), refined float signatures, and explicit `unsafe instance` IEEE equality.
- **Replace quiet fallbacks with proofs:** upgrade total functions that currently clamp or fall back quietly (e.g. `Data.Rational.normalise`, `Text.Builder`, `Text.Zlib`, `Text.Yaml`) to carry explicit proofs or `Option`/`Result`.
- **Kernel reflection & tactics:** move `DiffCert` and `LinCert` checking from Rust into reflected kernel proofs; build `stage`-based tactic automation (`simp`, `omega`, `grind`, LRAT certificate checking).
- **Cross-function handler specialization:** specialize concrete effect handler vtables across call boundaries.

### Concurrency and parallelism
- **Async I/O reactor:** integrate non-blocking I/O polling with the fiber scheduler so socket/file/process reads suspend fibers instead of holding OS threads; add work-stealing run queues.
- **`Parallelism` class & `parTraverse`:** resolve per-call-site class specialization for `Parallelism` methods and `Row`-kinded data parameters (or specialized `Traversable` lowering).

### Compiler performance and ecosystem
- Finish native multi-core parallel and incremental checking in the self-hosted compiler (`Bootstrap.Parallel`, `Bootstrap.Incremental`).
- Split non-core `seed/prelude/` modules into versioned official packages and add automated interface-compatibility checking to `cinch pkg`.

---

## 2. Open design decisions

| Area | Question | Reference |
| --- | --- | --- |
| **Core type system** | Definitional vs propositional proof irrelevance; record eta; full vs `Type 0` cumulativity; positivity-preserving nesting inside mutual data groups | [`type-system.md`](type-system.md) |
| **Floats** | Should lawful `NotNaN F64` (`totalOrder`) `Eq`/`Ord` instances be `implicit` or explicit-only? Final module name for unsafe IEEE instances and `Real64`/`Real32` aliases | [`type-system.md`](type-system.md) |
| **Ownership** | Default multiplicity for unannotated owned parameters (`%aff` vs `%many`); user-extensible `Share`; two-phase borrows and reborrowing | [`ownership-and-reclamation.md`](ownership-and-reclamation.md) |
| **Allocators** | Signature default/inference for `Heap r`; spelling of arena copy-out (`evacuate`) and budget boundaries; segmented vs per-boundary heap stacks | [`ownership-and-reclamation.md`](ownership-and-reclamation.md), [`safety-and-verification.md`](safety-and-verification.md) |
| **Effects** | Multi-shot resumption duplication evidence; whether effect boundaries/policies become core language features or library conventions | [`effects-and-concurrency.md`](effects-and-concurrency.md) |
| **Verification** | Tactic surface (`by <tactic>` vs `justify`); goal-context API for `stage` tactics; whether a compiled-check tier exists alongside kernel-checked certificates | [`type-system.md`](type-system.md), [`safety-and-verification.md`](safety-and-verification.md) |
