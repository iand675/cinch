# Ownership, usage and regions design

Design note for the Cinch toolchain, roadmap slice P5 in
`compiler/CONFORMANCE.md`. Companion to the core language specification
(sections 3, 7) and the laziness and reclamation specification, which state
obligations; this note fixes the checking discipline.

Status: design accepted for implementation.

# 1 What must be enforced

The core specification requires: no use-after-free, double free, invalid
mutable aliasing or data races in safe code; usage tracking over erased,
affine, linear and unrestricted multiplicities; consuming transfer, shared
and exclusive borrows, and deterministic destruction. The surface marks
multiplicities on arrows: `%0`, `%1`, `%aff`, `%many`.

# 2 Multiplicities

A type or binder carries a usage class:

| Class | Meaning | Runtime consequence |
| --- | --- | --- |
| `%0` erased | never used at runtime | no representation; erased before codegen |
| `%1` linear | used exactly once on every path, including error and cancellation paths | ownership transfer; no copy, no drop-without-use |
| `%aff` affine | used at most once | may be dropped (destructor runs), never copied |
| `%many` unrestricted | used any number of times | freely duplicated; requires a `Share` basis for duplicating values |

Checking model: usage environments map binders to multiplicities; function
types carry the multiplicity of their parameter (`a %1 -> b`). Composition
follows the standard algebra (semiring-like: parallel composition adds,
sequential composition multiplies, with `0` annihilating). The checker
computes a usage vector per definition and compares it against the declared
multiplicities; the default for unannotated parameters is `%aff` for owned
resources and `%many` for `Share` values (decision **open**: the draft says
"the proposed foundation tracks erased, affine, linear, and unrestricted
usage where appropriate", so the default is a policy choice).

Linear obligations must hold on error and cancellation paths: this is a
path-sensitive check over the control-flow the pattern-match compiler
already builds. Dropping a linear value is an error; dropping an affine one
runs its destructor.

# 3 Sharing

`Share` is the class of values that may be duplicated at will: immutable,
indirect, no linear parts (machine scalars, immutable ADTs of `Share`
fields, `Ref r a` for shared borrows). It backs `%many` on owned values and
the memo cells of the laziness specification (a cached value must be
`Share`). Core section 7 leaves immutable sharing unsettled; the bootstrap
choice is: `Share` is a compiler-known class with structural instances, not
user-extensible, and copying is explicit at the surface when a value leaves
a borrow (the `copyOut` style of the core examples). **Open**: user-extensible
`Share` instances and their coherence with the usage algebra.

# 4 Regions and borrows

`Ref r a` (shared) and `Mut r a` (exclusive) are parameterized by region
names, as in the core notation table. Rules:

1. A region name is bound by a scope form (`withScope`, a `where`/`let`
   block, or a function parameter in generic code).
2. A borrow never outlives its region; the checker rejects escape in return
   types, stored fields of escaping structures, and captured environments of
   escaping closures (the memo/thunk machinery already tracks `Eval r` labels
   the same way; a borrow carries its region in the type, which is what makes
   the escape check local).
3. `Mut r a` is unique: no other borrow of the same region may exist while
   it is live. The bootstrap enforces uniqueness lexically (at most one live
   `Mut r` per region in scope, and no `Ref r` while it is live), which is
   conservative but sound. **Open**: two-phase borrows and reborrowing.
4. Destructors: dropping a value with owned resources runs its destructors in
   declaration order; a drop of an affine value in a branch is allowed, a
   drop of a linear value is an error unless it is an explicit consumption.

# 5 Thunks and memo cells

The laziness specification's rules fit on top: `Thunk {e} a` at `%1` or
`%aff` (force consumes), `%many` duplicable thunks recompute, memo cells
require `Share` cached values, and `Lazy {e} r a` cannot outlive `r`. The
`%1` discipline for thunks is exactly item 2's linear check applied to the
forcing paths.

# 6 Implementation mapping

- `types.rs`: multiplicities as a type-level annotation on `Ty::Fun`
  (parameter multiplicity), a `Usage` lattice, and `Ty::Ref`/`Ty::Mut` with
  region parameters (regions already exist as `Ty::Lazy`'s region field and
  effect label arguments, so the machinery generalizes).
- `check_terms.rs`: usage environments threaded through the pattern-matrix
  compiler (each branch contributes a usage vector; merges take the lattice
  join for affine/unrestricted and require exact counts for linear).
- `check.rs`: escape checking at generalization and at `Scheme` construction
  (no region of an inner scope may appear in an exported scheme).
- Erasure of `%0` binders runs in the same pass the dependent core defines
  (P7); until then `%0` is rejected at the declaration site.
- Runtime: no new runtime support is needed; ownership is a static property.
  Deterministic destruction is what the laziness spec already provides for
  thunks and cells.

# 7 Library signatures: the linear-base comparison

At the user's direction (2026-10-29) the library's signatures were
reviewed against `linear-base-0.8.1` for places where a multiplicity
should carry the discipline. What the language has: `%0`, `%1`, `%aff`
and `%many` on arrows, checked path-sensitively (the cases in
`tests/ownership.rs`). Three homes use them in signatures today:
`Effect.Pty` (`Pty %1 ->` on all seven operations, each returning the
terminal except `pty-close`), `Effect.Clock` (the continuation at `%1`
in `worker-body`, `run-timeout`, `timeout`) and `Text.Lex` (`%0` for the
tokens a rule cannot reach, `%aff`/`%many`/`%1` elsewhere).

What maps, and where it is applied:

- A handle threaded by hand. `Effect.Pty` is the worked case
  (D377/D378): taking the terminal at `%1` and handing it back on every
  path makes reading or closing one twice, closing on only one branch,
  and using it after a close into COMPILE errors rather than runtime
  faults, with the negative programs pinned in `tests/lib_pty.rs`. The
  module header states what the discipline does and does not buy:
  misuse becomes unwritable; a leak is still possible, because a `%1`
  parameter cannot force a path to consume it.
- A capability passed to a callback exactly once (`Effect.Clock`'s
  continuations).

What does not map, with the reason:

- `Ur a`: linear-base's unrestricted wrapper needs a data type whose
  field is used `%many`, which the linear discipline rejects at its own
  declaration, so there is no `Ur Pty`.
- Class-quantified `Consumable`/`Dupable`/`Movable`: a class method
  cannot let its caller pick a multiplicity here, and there are no
  multiplicity-polymorphic binders (`forall (p :: Multiplicity)`) and no
  inline `%1 ->` lambdas, so a linear continuation has to be a named
  function (the limit `Effect/Pty.cinch`'s header records).
- The polarized/mutable containers (`Data.Array.Polarized`,
  `Data.Vector.Mutable.Linear`, `Data.Unrestricted.Linear`) are library
  surface rather than language capability: they are writable with the
  multiplicities above wherever the representation is a value Cinch can
  express, and nothing in the language blocks them.

Still available, not yet applied: `Effect.Process` exposes a `Child`
value that a caller holds across `child-close`, which is exactly the
`Effect.Pty` shape and would catch a double close the same way; the
`Pty` precedent (a handle returned on every non-consuming path, plus
negative tests) is the mechanism, not a new one. `Effect.Fs` keeps its
directory handle as a raw `I64` inside its own open/close pairs and does
not hand it to callers, so there is nothing to annotate until that
surface is made a value.

# 8 Open decisions

- Default multiplicity for unannotated parameters (`%aff` vs `%many`).
- User-extensible `Share` instances and copy semantics at the surface.
- Two-phase borrows, reborrowing, and borrow lifetimes shorter than a region.
- Interaction of usage with effect handlers (resumption multiplicity) and
  with hot replacement (resources crossing an upgrade boundary).
