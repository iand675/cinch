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

## The attribute backlog, measured (2026-11-13)

The inventory below was taken against the seed's own IR for the whole
compiler (`cinch emit-ir` over the bundled 45,534 line
`compiler.cinch`, 5,897,007 lines of IR, 7,346 `define`s) and against
the self-hosted emitter's output (`build/bootstrap/stage3/compiler.ll`,
923,333 lines, 7,155 `define`s). Two numbers frame everything else:

| Fact | seed IR | stage3 IR |
| --- | --- | --- |
| `define`s | 7,346 | 7,155 |
| attribute groups | 7 (7 distinct) | 0 |
| `define`s carrying a group | 7,175 | 0 |
| `define`s with a `ptr` parameter | 163 | 202 |
| `noalias` | 0 | 0 |
| `mustprogress` | 0 | 0 |
| `!llvm.loop` | 0 | 0 |
| `llvm.assume` | 0 | 0 |
| `freeze` | 27 ops | 0 |
| `noreturn` | 1 | 1 |

The self-hosted emitter (`bootstrap/Bootstrap.RuntimeIr.cinch`) emits no
attribute group at all, so every claim below is a seed-side claim. The
fixed point makes the two agree in behaviour, not in this surface: the
self-hosted chain has to grow the same classification before any of it
reaches a shipped binary. That gap is the single largest item on this
list and it is a port, not a proof.

### What each pointer position actually is

Every internal `ptr` parameter in both modules is one of three shapes,
and none of them is a `Ref` or a `Mut`:

| Shape | Count (seed) | What it points at |
| --- | --- | --- |
| `ptr %self, i64 %argc, ptr %argv` | 161 | an apply stub: a closure payload and a word array |
| `i64 %f, i64 %argc, ptr %argv` | 1 | `__apply`, the indirect entry |
| `i32 %argc, ptr %argv` | 1 | the C `main` entry |

A `Ref`/`Mut` parameter does not cross as a pointer. `repr_of` maps
`Ref r a` to the representation of `a` and `Mut r a` to `Obj`, so a
borrow of a `Nat` is a word and a borrow of a boxed value is the block
word. That is why `noalias` has no position to attach to.

### Landed

**`norecurse` is now a proved fact.** It was claimed on 7,175 of 7,346
units, which is a claim about almost the whole program, and it was
false: `UnitClass::recursive` was initialized `false` and never
assigned, so every unit asserted that it could not call itself back.
Measured against the seed IR: 1,292 units call themselves directly and
1,142 mutual-recursion pairs exist, and every one of them claimed the
attribute. LLVM may use `norecurse` to delete a recursive call's
frame bookkeeping and to justify inlining a recursive unit, so a false
claim is a miscompile waiting for an input.

The proof is `recursive_units`, a reachability walk over the direct
call edges plus the indirect edge. A unit is recursive when it reaches
itself, or when it reaches any unit that makes an indirect application,
because `<unit>.app` applies a closure and an over-application pushes
the excess words through `__apply` at the result: a unit that reaches
an indirect call may re-enter itself with nothing in the edge set
saying so. Measured: 3,257 units keep the claim, 4,089 lose it. The
attribute is a "cannot happen anywhere" property, which is why one
cycle (or one invisible edge) below a unit is enough to falsify it.

**`noreturn` is now claimed from the lowered control flow.** The seed
already declared `cinch_panic`, `exit`, and `__cinch_blackhole`
`noreturn`, and the fault audit already put `unreachable` after every
fault call, so the vocabulary was present but no generated unit ever
claimed it. `body_never_returns` walks the emitted blocks from `entry`
and asks whether any reachable block ends in `ret`; `classify_units`
folds a `noreturn` callee into its callers, so a unit that returns only
after a call that never returns is `noreturn` too.

The attribute is a statement about the lowered control flow, never
about the effect row. A `{Diverge}` row is a permission: a definition
may declare it and still return. What licenses the attribute is that
the emitted body has no returning path at all, which is the same thing
LLVM reads it as. `noreturn` and `willreturn` are mutually exclusive by
construction, and a `debug_assert` refuses the pair.

Two existing claims were wrong for the same reason and are corrected:
`willreturn` and `speculatable` were seeded from `clean && !frees`,
which never asked whether the body returns. A body with no returning
path claimed both. They now require the local walk to find a live
`ret`.

### Refused, and why

- **`noalias` on `Ref`/`Mut` parameters**: there is no such parameter.
  A borrow crosses as its referent's representation (see the table
  above), so the attribute would attach to a word. Even if a borrow
  did cross as a pointer, the language rule does not license the
  general claim: `Mut r a` is exclusive and unique, so a `Mut` against
  any other borrow of the same region is disjoint, but `Ref r a` and
  another `Ref r a` of the same region legitimately alias for reading
  and LLVM's `noalias` says the pointer does not alias *any* memory
  accessed through another pointer, which a shared borrow cannot
  assert. Only the `Mut` against `Mut` and `Mut` against `Ref` pairs
  are disjoint, and there is no position to put them on.
- **`nonnull` on internal parameters**: every internal pointer is a
  closure payload or a word array, and both may legitimately be the
  null address for a zero-argument call (`argv` is `alloca i64, i64 1`
  and the zero-arity path passes a zero-length array). The one
  position with a non-null contract is the foreign boundary, which
  already claims `nonnull` there through `param_attrs` and restates it
  as an `llvm.assume` operand bundle, and it deliberately does not
  claim it at an export entry, where the pointer is untrusted.
- **`mustprogress`**: zero loops in the seed IR carry `!llvm.loop` and
  only 26 units have a back edge at all, of which none claims
  `willreturn`. `mustprogress` says a loop either exits or executes a
  side effect, which is a claim about termination the language does not
  make: `{Diverge}` is admitted by declaration, and a `spin` loop is
  exactly a loop that neither exits nor has a side effect. The
  `#[recursion("bounded")]` rows that could prove progress are used by
  zero definitions in the compiler's own sources, so there is nothing
  to attach the attribute to and no pass that would consume it.
- **`freeze` at the boundary**: already implemented and tested
  (`untrusted_boundary_reads_freeze`), 27 `freeze` ops in the seed IR. The remaining untrusted source is a `RawPtr` load,
  which freezes, and `Bytes` data, which freezes. Nothing is missing.
- **`llvm.assume` operand bundles**: already emitted at the checked
  boundary (`assume_ptr`, `assume_layout`). They appear zero times in
  this program because the compiler's own FFI surface crosses only
  `I64` words (see `Prim/Sys/Darwin.cinch`), which have no pointer
  contract to restate.
