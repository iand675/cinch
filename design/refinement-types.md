# Refinement types: sub-kinds are refinements, not wrappers

Design note (2026-10-26, user direction revising D150). The rule: a
type that differs from its container only by a TRUTH about its
inhabitants is a refinement, not a wrapper. `Sum`, `Product`, `Dual`,
`Endo`, `First`, `Last` are newtypes (distinct identities are the
point: two monoids over `Nat` need distinct types). `NonEmpty`,
`Sorted`, `NonZero`, `Ascii` are refinements (same data, constrained
inhabitants); wrapping those pays wrap/unwrap and writes delegation
code to recover the type that was always the same. No em dashes.

## Shape

- A refinement is a decidable predicate over the container's values:
  `type NonEmpty a = refine (xs : List a) with NotNull xs`. The
  runtime representation IS the container: the witness is `Prop`-
  erased (the D31 erasure discipline), so zero cost with no coercion
  or delegation machinery.
- Discharge is explicit and lives in the `Dec`/`decide` world (P7):
  `from-list : List a -> Option (NonEmpty a)` is the total API, and
  direct construction takes a `Dec (NotNull xs)` argument that erases.
  No SMT and no refinement inference: predicates inhabit the decidable
  logic the language already has. (LiquidHaskell-style inference is
  undecidable and is deliberately out.)
- Instances are the container's own methods, valid exactly when the
  methods PRESERVE the refinement. `Semigroup` transfers to `NonEmpty`
  (append of non-empties is non-empty); `Monoid` cannot exist (its
  identity violates the predicate). The preservation condition replaces
  D150's per-class delegation choice, and the P18 law-axiom layer
  checks it per instance.
- Projection is free (same representation): the container is the
  value; `toList : NonEmpty a -> List a` is the identity function at
  runtime.
- The invariant is no longer API surface: no constructor hiding or
  smart-constructor discipline is needed, because constructing a
  refined value requires discharging the predicate. Boundary
  validation (the JPL rule 7 story) becomes a type-level event.

## Interaction with what exists

- `Vec n a` and `Fin n` are the dependent end of the same idea
  (indices instead of predicates); refinements do not replace them.
- P20's `#[checked-params]` and runtime assertions become the
  discharge points at boundaries where the predicate is checked
  rather than proven.
- Newtypes remain for semantic wrappers; `repr(transparent)` remains
  the representation story for them. D150's delegation mechanism is
  kept ONLY for true newtypes where a distinct type must borrow
  container instances (rare; `Coercible`-style), and the NonEmpty
  replacement is a refinement, not a delegating wrapper.

## Open questions for the implementing slice

- The refinement surface spelling (`refine (xs : T) with p` vs
  annotation-style `T where p` vs a `Refine` former).
- Which predicate forms are admissible beyond `Dec` propositions
  (equality on indices? bounded numeric comparisons?).
- Whether refinement types participate in unification as their base
  type (subtyping/implicit coercion) or require explicit projection;
  conservative first answer: explicit, no silent subtyping.
