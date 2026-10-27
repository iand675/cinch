# LLVM contract attributes: adopting the rest of the vocabulary

Plan note (2026-10-26, user question: what else like `musttail` should
we adopt from LLVM). The pattern: Cinch's type/effect/ownership
analyses already prove most of what LLVM's attribute vocabulary
asserts, so adoption is translation, not new analysis. No em dashes.

## Translate from existing analyses (implementation in the P20/P18 era)

1. **Effect-derived function attributes**: an empty effect row with no
   foreign calls becomes `memory(none) speculatable`; `Io`-headed or
   mutating definitions become `memory(read)`/`memory(readwrite)`;
   P20 `#[no-alloc]` adds `nofree`; `willreturn` only where the
   totality/`bounded` machinery proves return (never guessed).
2. **Usage-derived parameter attributes**: P5 non-escape becomes
   `captures(none)`; P11 non-null default becomes `nonnull`; scalars
   get `noundef`; `repr`/layout facts give `align` (and `!align` on
   loads through borrows).
3. **`!range` metadata** from primitive widths, `Fin n`, `Bool`, and
   literal range checking, on returns and typed loads.
4. **`llvm.assume` with operand bundles** (`align`, `nonnull`,
   `dereferenceable`, `range`) from P20's verified assertions and
   `repr` facts. A satisfied safety assertion becomes an optimizer
   input; the IR carries a second machine-readable statement of the
   contract.
5. **`noreturn` on fault functions + `unreachable` after fault calls**
   (audit pass: every fault path should mark its successor dead).
6. **`byval`/`sret`** as the native spelling of D66's
   const-pointer-in/results-out aggregate ABI at FFI boundaries.

## Adopt with discipline

- **`freeze` at untrusted boundaries**: values read from `RawPtr`,
  foreign buffers, and `Bytes` data freeze before branching or
  comparison. Poison/undef cannot leak from foreign input into
  otherwise-safe code; this is a safety property.
- **`!llvm.loop`**: `mustprogress` where progress is proven
  (`#[recursion "bounded"]` or totality); unroll and vectorize hints
  only under P10 rewrite contracts (wrapping reassociates freely,
  checked does not, FP needs its own contract).
- **`immarg`** for `prim` declaration parameters that must be
  immediates (shift amounts, SIMD lane indices); **`llvm.sideeffect`**
  on `asm` fragments so DCE cannot drop them (P18).

## Not adopting

- Fast-math flags as defaults (IEEE default-rounding contract); only
  under an explicit rewrite-level relaxation.
- `returns_twice`/setjmp-family anything (JPL rule 1; no such control
  flow exists).
- Coroutine intrinsics (thunks and the cooperative task backend are
  our own state machines).
- `convergent` unless the SIMD barrier story demands it.
- CFI/sanitizer metadata (later; orthogonal to the language contract).

## Where it lands

The translations attach to the P20 implementation (assertions to
`llvm.assume`) and the P18 `prim` work (`immarg`, `sideeffect`); the
`freeze`/`noreturn`/`!range`/ABI-attribute pass is its own hardening
slice (tracked as the LLVM contract surface phase). Evidence style:
IR-observation tests (the attribute appears on the emitted unit where
the analysis proves the fact; its absence where it does not), never
wiring tests.
