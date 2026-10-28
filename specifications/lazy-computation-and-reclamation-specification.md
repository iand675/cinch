# Lazy computation and reclamation specification

Source: local draft in this repository (added after the documentation export)  
Added: 2026-10-22

---

[Documentation overview](../overview.md)

Draft 0.1. The language is strict call-by-value, and stays so: there is no implicit general laziness. This chapter adds an explicit laziness layer that does not require a garbage collector. Every deferred value states, in its type, when its captured environment dies. MUST denotes a requirement of this draft. Proposed choices are identified explicitly; unresolved items are listed in Open decisions.

# 1 The problem laziness actually poses

Deferring a computation is cheap: a closure already does it. The hard part is reclamation. A deferred value keeps its captured environment alive until the value is either forced or destroyed, and with sharing it may be kept alive across scopes that the language cannot predict. A garbage collector answers "when is the environment dead?" at runtime. This design answers it statically instead: every lazy value carries a reclamation policy among three deterministic options, and no fourth is needed.

| Mechanism | Sharing | Death time of captures | Needs GC |
| --- | --- | --- | --- |
| Affine thunk `Thunk {e} a` | None: force consumes | At force, or at destruction | No |
| Region memo cell `Lazy {e} r a` | Yes, inside region `r` | Region teardown | No |
| Duplicable thunk `%many` | Copies recompute | Each copy's own consumption | No |

Two capabilities must not be conflated: deferring work (cheap) and sharing one evaluation among many uses (needs a cell with a lifetime). The design makes the second explicit and scope-bound instead of universal.

# 2 What stays unchanged

- `Later a` and `delay (\() -> expression)` in the core specification remain the guarded observation mechanism for productivity and coinduction. `Later` is a logical modality, may be erased, and carries no reclamation obligations. It is not an unrestricted thunk and MUST NOT be given one.
- `delay` and `force` remain ordinary library names, not control syntax, as [Surface syntax and grammar](surface-syntax-and-grammar.md) states. The forms in this chapter are prelude types and operations plus the `~` binding form in `let` and `where` blocks (section 5).
- Evaluation of executable code stays strict and left-to-right. A function's strictness never changes because a thunk exists somewhere in its type.

# 3 Affine thunks

```haskell
-- The effect row of the body is part of the thunk type.
Thunk :: {e} -> Type -> Type

delay  :: forall a e. (() ->{|e} a) -> Thunk {e} a
force  :: forall a e. Thunk {e} a ->{|e} a
destroy :: forall a e. Thunk {e} a ->{DestructorEffects e} ()
```

- `delay f` allocates a closure capturing `f`'s environment under the ordinary closure rules: owned captures transfer, borrowed captures keep their lifetimes ("a callback cannot outlive a borrowed environment" applies verbatim).
- `force` **consumes** the thunk. A thunk is used at multiplicity `%1` or `%aff` by default (the multiplicities of [Surface syntax and grammar](surface-syntax-and-grammar.md) apply). With `%1`, every path, including error and cancellation paths, must force or destroy the thunk exactly once; with `%aff`, at most once. Destroying an unforced thunk runs the destructors of its captures, deterministically and in reverse capture order.
- Because forcing consumes, there is no memoization cell and no sharing question: the body runs at most once. This is laziness without a GC and without a cache.
- Effects are honest. `force` has exactly the effect row of the body. `Thunk {e} a` cannot hide `IO`, failure or suspension inside a pure type, matching "an explicit pure signature cannot hide them". Divergence appears in `e` as `Diverge` whenever the body's termination is not established; forcing a thunk is never a way to run a diverging computation while claiming totality.
- A thunk is a value in `Type`. Forcing is computation, so erased evidence may live inside a thunk, but the erasure rules of the core specification apply to the body unchanged: a failed check inside a thunk produces no evidence, and erasure must not remove a force whose effects are observable.

This level covers the common need: "compute this later, at most once, and clean up deterministically", for example validating an expensive argument only on the path that uses it, or building an error payload only when the error happens.

# 4 Duplicable thunks

A thunk used at multiplicity `%many` is duplicable and unshared: every duplicate re-runs the body. Copies are ordinary owned values; each dies with its owner. This gives call-by-later semantics for total pure code where recomputation is acceptable, without a cell and without a collector.

Sharing one evaluation among many uses is a different feature and is section 5's. The compiler MUST NOT silently turn a `%many` thunk into a shared memo: that would introduce a hidden lifetime.

# 5 Region memo cells: `Lazy {e} r a` and `~` bindings

For call-by-need with sharing, evaluation state must live somewhere with a reclamation time. It lives in a region, like the `Ref r a` and `Mut r a` of the core specification.

```haskell
Lazy :: {e} -> Region -> Type -> Type

-- `~` bindings work in `let` blocks and `where` blocks alike:
--   f x = y * 2
--     where ~y = expensive x
demand :: forall r a e. Lazy {e} r a ->{Eval r | e} Ref r a
```

- The memo binding form is `~ pattern = term` inside a `let` block or a `where` block. The bare `~` is reserved punctuation, like `$`, `=` and `|`, so it is not an operator; symbol operators such as `~>` and `<~>` remain available. In Haskell `~p` marks an irrefutable pattern; here `~` marks the memo binding itself, and the shared intuition is the same: the binding is not evaluated eagerly. All `~` bindings of one block form one memo group: one fresh region `r`, one effect label `Eval r`, mutually visible cells. `let ~x = e in body` is the single-binding case of the same form. A bare use of a `~`-bound name elaborates to `demand`, so `f x = y * 2 where ~y = expensive x` demands `y` at that use; holding a cell undemanded means storing the `Lazy {e} r a` value explicitly.
- A memo cell's type records the effect row `e` of its body. `demand` has the row `Eval r | e`: the first demand runs `e`, later demands run nothing, and a demand in effect-polymorphic code simply carries `e` along. The row is an over-approximation on every demand; refinement may prove later demands pure.
- A `~` group in a `where` block is scoped to one evaluation of the equation. The cells are allocated at call entry, are visible in the equation body and in the block's other bindings, and the whole group is reclaimed at return, in reverse allocation order. Two calls of the same function own disjoint groups.
- Strict `where` bindings evaluate in declaration order at call entry and may see earlier strict bindings and every `~` binding of the block. A `~` body may see all bindings of the block. If a strict binding demands the group and a `~` body refers back to that strict binding, the composition carries the group's `Eval r` into the cell's row and is rejected exactly like a direct cycle (section 6).
- The first `demand x` evaluates `e`; later demands observe the same value. `x` MAY be used at multiplicity `%many`.
- The cell's captures die at **region teardown**: for a `where` block group, at return from the call; for a `let` block group, at the end of the enclosing block. Teardown reclaims the cells and everything only they own, in reverse allocation order, running destructors then. Regions can reclaim cycles, so mutually referential cells are reclamation-safe.
- `Lazy {e} r a` does not outlive `r`, exactly like `Ref r a`. It may be stored in data structures that are themselves parameterized by `r`; it may not escape them. A value needed beyond its region must be forced and moved out, or rebuilt as an owned thunk.
- Because a cached value is observed many times, the cached type `a` MUST be duplicable (an immutable `Share` value). Linear or affine resources use section 3's affine thunks instead: lazily opening a file is a `%aff` thunk, not a memo cell. `demand` returns a borrow `Ref r a` so the cached value is never silently duplicated; the caller copies explicitly when it needs an owned value.
- Re-entrancy: a cell has states `Unevaluated`, `Evaluating`, `Evaluated`. Demanding a cell that is `Evaluating` is a black hole. Section 6 rules black holes out statically through the `Eval r` effect and defines the runtime fault for the paths that cross a trust boundary.
- Concurrency: cells are single-threaded by construction and belong to the scope that allocated them. Moving a thunk or a demand across task boundaries follows the ownership rules of [Concurrency specification](concurrency-specification.md): an affine thunk moves by ownership transfer, a `Lazy r a` does not cross its region's thread. A thread-shared memo needs an explicit synchronized structure, not this mechanism.

# 6 Black-hole freedom and the demand effect

Demanding a cell while it is evaluating is a black hole. Two rules rule it out: a static discipline that makes it impossible in well-typed code, and a defined runtime fault at the boundaries where the discipline can be bypassed.

**Demand is an effect.** Every memo region `r` has a fresh effect label `Eval r`, and `demand` performs it (section 5). Creating a thunk performs nothing; `force` performs its thunk's row. Effects are honest, so the evaluation row of a cell body records every demand that body can perform, directly or through higher-order code, exactly as `IO`, `Cancel` and `Diverge` propagate through rows today.

**The rule.** For a `~`-binding memo region `r`, the effect row of every cell's evaluation MUST NOT contain `Eval r`. Demanding a cell of a different region is fine. Storing a thunk that demands `r` without forcing it is fine: the thunk's `Eval r` sits inside `Thunk {Eval r | e} a` and runs only when someone forces it, which is later than the cell's evaluation.

**Why it is sufficient.** A black hole requires a demand of a cell of `r` during the evaluation of a cell of `r`. By effect soundness that demand's `Eval r` appears in the evaluation row of the cell body, and the rule forbids exactly that. The argument needs only that effects are honest, which the language already requires. It needs no termination, sized-type or productivity analysis.

**Discharge and scoping.** The memo construct binds `r` like a region binder and discharges `Eval r` from the residual row at scope end; it is, in effect, a built-in handler of its own label. This is sound because nothing that can perform `Eval r` escapes the scope: `Lazy {e} r a`, `Ref r a` and `Thunk {Eval r | e} a` all mention `r` in their types and cannot outlive it (section 5's escape rule). The effects of evaluating cells themselves are not discharged: `IO`, failure and `Diverge` run at first demand and propagate to the enclosing row like any other effect.

**Recursion and re-entry.** A function whose body uses `~` bindings in its `where` block has a public effect row that does not mention its group's `Eval r`, so a cell body may call its own function recursively: the recursive call allocates a fresh group and reaches the caller's cells only by handle. Handing a `Lazy {e} r a` handle to such a call, or capturing one in a closure it runs, makes the demand of `r` visible in the row again and is rejected. The label is per binding site, so this covers re-entry across activations of the same function, not just a single cell instance.

**Why guarding with `delay` alone is not sufficient.** Syntactic guardedness does not help: `let ~x = force (delay (\() -> demand x))` has a cycle through a `delay`, and still re-enters `x` on the first demand. What defuses a cycle is deferring the demand past the cell's evaluation, and the effect row is what detects whether that actually happened.

**What the rule rejects.** It is conservative in one direction: demanding a *different* cell of the same region during evaluation is a dependency, not a cycle, and is rejected too. The program-level fix is structure, not annotation: put acyclic dependencies in separate `~` groups, since each group gets its own region name and its own `Eval` label, or name the memo region explicitly and split it. Per-cell labels would be finer and are not proposed in this draft.

**Runtime fault as defense in depth.** The cell states of section 5 remain, and a demand that finds `Evaluating` fails with a defined `BlackHole` fault, in the same policy class as stack exhaustion and allocation failure (core specification, section 6). Well-typed code cannot reach it; code that crosses a declared trusted assumption, a foreign boundary, or a runtime bug can. A deployment profile claiming the fault cannot occur MUST discharge the effect discipline and record its trust boundaries in the assumption manifest.

**Relation to divergence.** A black hole is morally an infinite regress. In code whose row already contains `Diverge`, a cycle is one more way to diverge; the `BlackHole` fault remains its observable behavior. `Diverge` does not grant `Eval r`, so the rule still applies.

# 7 Data structures and unboxed values

- Fields are strict by default. A field MAY be `Lazy {e} r a`, and then the structure's type mentions `r` and the body row `e`; implicit lazy fields with hidden regions are rejected.
- Primitive unboxed values (see [Primitive types specification](primitive-types-specification.md)) are never lazy: deferring an `I64` computation is a thunk around the computation, not a lazy `I64`.
- Streams and other codata keep using `Later` for guarded productivity. An infinite stream is not a memo of its elements; unbounded memoization is exactly what a collector would be needed for and is not provided here.

# 8 Erasure, upgrades and trust

- Forcing a pure total thunk is an ordinary computation step; no runtime evidence survives erasure unless the core rules retain it.
- An unforced thunk pins the code version that will run when forced, exactly like a registered foreign callback. [Runtime evolution specification](runtime-evolution-specification.md) obligations apply: an upgrade package must drain, migrate or retain thunks, and old code stays loaded while any thunk from it can still run. A memo cell that is already `Evaluated` pins nothing further.
- The reclamation claims of this chapter are about deterministic deallocation, not about absence of leaks in the large: a region that is never exited keeps its cells alive, and a `%1` obligation violated at runtime is a checked usage error, not a safety hole.

# 9 Required rejection cases

- An implicit lazy argument or lazy field whose region is not in its type.
- A `Thunk {e} a` whose declared effect row omits an effect of its body, or a diverging body typed as total.
- Forcing a `%1` thunk twice, or destroying it after forcing; using an affine thunk after force; a `%1` obligation skipped on an error or cancellation path.
- A `Lazy {e} r a` used after `r` ends, or returned from the scope that defines `r`.
- Demanding a `Lazy {e} r a` whose cached type `a` is linear or affine.
- A `~` cell body (in `let` or `where`) whose evaluation row contains `Eval r` for its own group: a demand cycle, whether direct, through forcing a thunk, or through a strict `where` sibling that demands the group (section 6).
- Escaping a memo group: a `Lazy {e} r a`, `Ref r a` or `Thunk {Eval r | e} a` in an equation's result, a returned closure, or any other value that outlives the group's scope.
- A thread-shared memo cell built from `Lazy {e} r a` across task boundaries.
- An erased proof whose availability depends on whether a thunk was forced.

# 10 Open decisions

- The exact `Share` class for duplicable values, and whether it is primitive or derived from the usage algebra. Core section 7 leaves immutable sharing unsettled; this chapter needs it.
- Demand labels finer than one per `~` group (per cell, or per dependency edge) so that acyclic demand inside one group is accepted. The group rule of section 6 is conservative in exactly this one direction; finer labels should wait until the group rule has been used in practice.
- Whether `~` should also mark irrefutable patterns in function arguments (Haskell's `~p`), with the fields of such a pattern elaborating to thunks. This draft reserves `~` for the memo binding form only.
- Whether `demand` should have an owned variant for `Share` types (`forceShared : Lazy {e} r a ->{|e} a` when `a : Share`), trading a borrow for convenience.
- Destructor order at region teardown beyond "reverse allocation order" when cells borrow from each other.
- Interaction with `Later`: whether `Later` observations compile to erased thunks, sized structures, or something else remains the guarded-recursion decision of core section 9.
- Cost model: whether the compiler may allocate a `~` cell on the stack when `r` does not escape, or elide the cell when demand count is statically one. These are optimizations and MUST NOT change observable reclamation.

# Implementation status

The bootstrap compiler in `compiler/` implements this chapter's runtime shape: `Thunk` as a closure over `Unit` (`delay`, `force`), `~ name = term` memo bindings with one memo group per block, demand-on-use elaboration, and the `Eval r` black-hole rule of section 6 enforced statically through row lacks constraints (cell bodies may not perform their group's label). Not yet implemented there: the multiplicity discipline of section 3, region escape checking, `Ref r a` (the bootstrap `demand` returns the value), and the `Share` class, so a `Lazy {e} r a` value can outlive its region in the bootstrap, which is the same lifetime-checking gap the rest of the bootstrap has. The `BlackHole` runtime fault exists as defense in depth.

# Amendments

Dated notes recording landed decisions (`compiler/CONFORMANCE.md`,
D-numbers) that amend or interpret this chapter's normative text.
2026-10-28, P15 conformance sweep.

- **`Later` is a usage-multiplicity rule (D36), 2026-10-28.** This
  amends section 2: `Later a` is a distinct type former that never
  unifies with `Thunk`; a `Later` binder lives at `%1`/`%aff` only
  (default `%aff`) and `force` consumes it; `%many` is rejected as
  unrestricted-thunk semantics. Kinding: `Later` takes exactly one
  type argument and no effect row.
- **Usage defaults and sharing (D15-D19), 2026-10-28.** This amends
  sections 3-5: `Thunk`-typed binders default `%aff` while unannotated
  binders default `%many` (D15); memo bare-use requires the cached type
  to be `Copy`, discharged through `copyOut` (D16/D19), and `demand`
  returns `Ref r a` so the cached value is never silently duplicated;
  lambda/thunk bodies count uses as unbounded and memo bodies at most
  once (D18). The bootstrap's multiplicity/region gaps named in this
  chapter's implementation status are closed for the `Copy`/`Ref`
  story by these rules (the runtime shape was already present).
- **Black-hole runtime and memo groups (D131-D137), 2026-10-28.** The
  runtime `BlackHole` fault (exit 70) backs the `Eval r` lacks rule of
  section 6 as defense in depth (`runtime/src/abi.rs::cinch_blackhole`);
  `letrec` fixpoint boxes use the same fault for an early read of an
  unevaluated knot, and the compile-time evaluator refuses fixpoint
  boxes with a named error (D136).
- **Cinch naming (D278-D281), 2026-10-28.** The language is Cinch:
  sources `.cinch`, interfaces `.cinchif`, runtime externs `cinch_*`;
  artifact names in this chapter read with those names.

[Documentation overview](../overview.md)
