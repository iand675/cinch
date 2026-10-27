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

(All three were decided by `RefinementSlice` (D248-D255): the headline
`refine (x :: T) with p` spelling; the predicate grammar is a closed
structural vocabulary plus `Dec` propositions with explicit refusals;
unification is explicit projection (`unrefine`), no silent subtyping.)

## Compound predicates (user direction, 2026-10-27; revision of D249's
conjunction refusal)

The single-predicate form is extended to conjunctions and
disjunctions, and comparison atoms admit total expressions over the
binder:

```
type SizedAscii = refine (xs :: List Char) with
  length xs > 100 && length xs <= 200 && Ascii xs
```

- The predicate grammar becomes: the closed structural vocabulary
  (`NotNull`/`NonZero`/`Ascii`/`Ordered`) plus `Dec` atoms (bounded
  numeric comparisons, index equalities, `Not`) with `&&` and `||` as
  new connectives. Conjunction and disjunction of decidable
  propositions are decidable (`Dec A` and `Dec B` decide both); what
  stays REFUSED is quantifiers (infinite domains need induction - the
  D113 discipline) and any predicate that is not total and closed over
  the binder.
- Comparison atoms admit total expressions over the binder
  (`length xs > 100`): the runtime `decide` lowering computes the
  measure (synthesized unit or the D92 evaluator) and compares, under
  the same rule as the vocabulary predicates: baked tests, never
  source a shadowing module could lie with.
- Instance transfer generalizes per-conjunct: a method must preserve
  EVERY conjunct; the preservation law identity checks each and a
  failure names the conjunct. For disjunction, the conservative
  sufficient check preserves each disjunct; the sample-wise law tier
  reports `not checked` when undecided.
- The witness stays a single erased `Dec` of the whole predicate
  (the conjunction's `Dec` is built by cases), so the representation
  story is unchanged: the refined value is the container.

- The refinement surface spelling (`refine (xs : T) with p` vs
  annotation-style `T where p` vs a `Refine` former).
- Which predicate forms are admissible beyond `Dec` propositions
  (equality on indices? bounded numeric comparisons?).
- Whether refinement types participate in unification as their base
  type (subtyping/implicit coercion) or require explicit projection;
  conservative first answer: explicit, no silent subtyping.

(All three resolved by the implementing slice (D248-D255): the
headline `refine` form; the closed vocabulary plus `Dec` propositions
with named refusals; explicit projection (`unrefine`), no silent
subtyping. The open question is the compound-predicates section
above, D256.)
