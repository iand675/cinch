# Safety, no-faults, and formal verification

Cinch is designed for **bulletproof programs**: if a program type-checks, unexpected runtime faults should be statically impossible or surfaced as explicit typed failures, while keeping the trusted computing base (TCB) small and auditable.

User-facing documentation lives in [`reference/22-safety-assertions-and-llvm.md`](../reference/22-safety-assertions-and-llvm.md) and [`specifications/attribute-annotations-specification.md`](../specifications/attribute-annotations-specification.md).

---

## 1. The no-faults model

Cinch distinguishes three outcomes:

- **Fault (eliminated):** an undeclared abnormal abort (division by zero, out-of-bounds index, underflow, unhandled panic). Source-level opt-in faulting (`#[may-fault]`) is deliberately excluded.
- **Typed failure (allowed):** an explicit outcome in the type (`Option`, `Result`, resource budget boundaries) that callers must handle.
- **Compiler-bug trap:** an internal runtime invariant assertion (`exit 71`, `compiler bug: ...`) unreachable by any well-typed program if the compiler and runtime are correct.

### Turning partial operations total
Partial operations are replaced (not merely supplemented) by total forms in priority order:
1. **Type-level evidence (zero runtime cost):** `NonZero` divisors, `Fin n` or refinement-bounded indices (`i < bytes-len b`), and `Nat` subtraction requiring `using (y <= x)` on `Sub_Nat`.
2. **Typed failure (`Option`/`Result`):** parsing, checked conversions, checked subtraction (`checked-sub`), and fallible lookups.
3. **Explicitly named total arithmetic:** `saturating-sub`, wrapping/saturating machine arithmetic families.

### Resource exhaustion (`exit 73`)
Memory, stack, and OS thread limits are finite. Rather than crashing with an undefined segfault:
- **Implicit top-level `Exhausted` outcome (implemented):** heap cap (`CINCH_HEAP_LIMIT`, default 8 GiB), stack overflow (detected via a 64 KiB guard band + alternate signal stack + `"probe-stack"="inline-asm"`), and thread exhaustion all exit cleanly with code **73** and a single stderr line (`cinch: exhausted: heap|stack|threads`).
- **Proven bounds (`jpl` tier):** programs whose reachable definitions satisfy `#[no-alloc]` / `#[fip]` / `#[alloc "bounded(n)"]` and `#[recursion "bounded(n)"]` can reserve all resources at startup so exhaustion cannot occur after initialization.
- **Typed budget boundaries (planned):** `with (heap = budget ...) in expr :: Result Exhausted a` will catch budget exhaustion inside a program and deterministically release RC allocations on unwind.
- **Foreign code:** external libraries remain a declared trust boundary listed in `assumptions.declared` and can be sandboxed via Wasm components or forbidden by profile.

---

## 2. Compile-time safety assertions (JPL Power of 10)

Cinch maps NASA/JPL's "Power of 10" rules onto inherent language guarantees and compiler-verified attributes (`#[profile "jpl"]`):

| JPL rule | Cinch enforcement |
| --- | --- |
| 1. Simple control flow; bounded recursion | No `goto`; `#[recursion "none"\|"tail"\|"bounded(n)"]`; guarded `corec` |
| 2. Fixed upper bounds on loops | `#[recursion "bounded(n)"]` via structural/measure proofs; bounded `Nat`/`Fin` counters |
| 3. No dynamic memory after init | `#[no-alloc]`, `#[alloc "once"]`, planned `#[fip]` and `#[alloc "bounded(n)"]` |
| 4. Small, simple functions | `#[max-complexity n]` (cyclomatic), `#[max-lines n]` |
| 5. Runtime assertions | `#[assert-min n]` counting explicit runtime checks |
| 6. Restrict data scope | Lexical scoping, `%1` linearity, region escape checking, no mutable globals |
| 7. Check return values & params | Must-use `Result` checking, `#[checked-params]`, refinement boundaries |
| 8. Restrict preprocessor | Hygienic, budgeted macros; `#[no-macros]` |
| 9. Restrict pointers | Offset-checked, deallocator-paired foreign pointers; `#[no-raw-ptr]` |
| 10. Zero warnings | `#[deny-warnings]` / `warn # error` |

Verified assertions are **contract facts**: they are recorded in `.cinchif`, hashed into `iface_hash`, reported in `build/<Entry>.manifest.json`, and translated to LLVM attributes where expressible.

---

## 3. Verified primitives and minimal trusted base

Machine-integer laws and custom representation proofs ultimately bottom out in bit-level facts about shifts, masks, and overflow checks. Cinch's long-term goal is a **smaller trusted base than Lean** for machine arithmetic:

1. **Bit-vector models:** pure bit-vector specifications for every machine scalar (`I8`..`I128`, `U8`..`U128`) and `prim` operation.
2. **Kernel-checked certificates:** external SAT/SMT solvers produce checkable certificates (e.g., LRAT) verified by kernel evaluation—avoiding compiled-code axioms that pull the entire compiler into the proof TCB (an explicit compiled-check tier may be offered only when recorded in the manifest).
3. **`prim` body validation:** formalize the LLVM IR subset used by `prim` bodies (Alive2-style semantics) and ISA models for `asm` bodies (Sail ARMv8-A / RISC-V models) as Cinch definitions so `prim` bodies are proven equivalent to their models rather than assumed.
4. **Measurable shrink of trust:** progress is measured directly by shrinking `assumptions.declared` in compiled manifests toward empty.

---

## 4. Formal verification roadmap

Cinch aims to support ordinary programming, refined contracts, dependent programming, and general theorem proving in one unified logical foundation:

```text
ordinary code + contracts + proofs + macros
  -> elaboration -> typed goals & Core terms
  -> untrusted search (local inference / tactics / SAT / SMT) -> proof terms & certificates
  -> small trusted proof kernel -> verified artifact -> checked erasure -> LLVM
```

### Target milestones
1. **Unified retained proof kernel:** stable-identity core terms, independent proof-artifact replay, and reflected verification of `DiffCert` and `LinCert` certificates inside the kernel.
2. **Richer automation:** integer Presburger arithmetic (`omega`), rewriting (`simp`), congruence closure with E-matching (`grind`), ring/AC normalization, and verified LRAT/SMT certificate checkers cached in the content-addressed store.
3. **Interactive proof tooling:** goal-aware `stage` tactics, typed holes in LSP/editor sessions, and proof-oriented diagnostics.
4. **Stateful & effectful verification:** compositional framing and logical snapshots over `Mut` regions, plus specification preservation across handlers and hot upgrades.
5. **Independent assurance:** complete transitive trust accounting in build manifests distinguishing kernel-checked proofs, compiled checks, sample-checked laws, and declared foreign/axiom assumptions.
