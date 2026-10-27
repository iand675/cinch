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

## Composability: instances live on the predicate (user direction,
## 2026-10-27; supersedes the per-nominal transfer of D254)

The nominal reading (each `refine` declaration mints a type with its
own transferred instances) is not composable: `Sorted && NonEmpty` is
a separate type needing its own transfer, and generic code cannot
quantify over "any refinement whose predicate the method preserves."
The corrected shape:

1. **Predicate-level instances**: the transfer rule is general - for
   any `refined p`, the container's class method is available when it
   preserves `p`. Coherence is BETTER than the nominal reading: by
   rep-identity there is exactly one dictionary per (class,
   container), unique by construction.
2. **Weakening along a decidable implication lattice**: `refined p ->
   refined q` when `p` implies `q`; implication between conjunctions
   of atoms is decidable by atom-set inclusion (a syntactic check, no
   SMT), so implicit weakening along that lattice is admitted;
   semantic implication needs an explicit `Dec`-carrying cast.
   `unrefine` remains the explicit forgetful arrow to the base type
   (a different direction, never conflated).
3. **Meets and joins**: `&&`/`||` (D256) are the lattice operations;
   `refined (p && q)` is the intersection and instances follow from
   rule 1 automatically.
4. **Method outputs carry derived predicates**: `<> : refined p ->
   refined q -> refined (post p q)` with the law layer checking the
   postcondition; preservation is the closure special case `post p p
   = p`, and a method with no sound postcondition simply has no
   instance (append has no `Sorted` instance).

Nominal names (`NonEmpty`, `Sorted`) are named predicates for
readability, nothing more. The refusal line is unchanged: no
inference, no SMT; weakening only where implication is decidable;
discharge always explicit.

## Narrowings and the implication theory (user direction,
## 2026-10-27)

Atom-set inclusion is too weak for useful implication: `> 1 && < 9`
implies `> 0 && < 10` by interval containment, not inclusion. The
implication procedure becomes a small **table-driven theory**, each
row decidable, no SMT:

- comparison/measure atoms relate by interval containment
  (`lo >= lo'` and `hi <= hi'`); the same machinery covers length
  bounds, byte ranges, enum ranges;
- vocabulary rows document structural implications (`length xs > 0`
  implies `NotNull xs`; `Ascii` implies byte-range facts);
- `p && q implies p` stays syntactic;
- unknown pairs are NOT implied: the cast needs a `Dec`, or refuses.

**Weakening is implicit and free** (the same word; every `refined p`
value is a `refined q` value when `p` implies `q`). **Strengthening
(narrowing) is always checked**: a `narrow` cast carries a `Dec` of
the stronger predicate, discharged statically where foldable or at
runtime through the same evidence machinery; silent flow up the
lattice is exactly the forged-witness hole the D192/D249 rules close.

Instances and values flow down the lattice for free (D257): a value at
`>1 && <9` weakens to `>0 && <10` keeping every instance; method
results at `post p q` weaken the same way. The interval-containment
table is also the home of bounded arithmetic for `bounded(n)` depth
bounds and temporal `recur` (D113/D202) - one bounded-arithmetic
machinery, not three.

## Partial weakening and `forget` (user direction, 2026-10-27)

Dropping SOME conjuncts is semantically already weakening to an
intermediate lattice point (`A && B && C implies A && C`), free and
implicit; instance availability is monotone under it (weaker
predicates preserve more easily, so forgetting cannot lose methods).
The feature is therefore explicitness at boundaries, in two layers:

1. **Type ascription is the everyday form** (no new syntax): `let ys :
   refined (A && C) = xs` means keep A and C, and implicit weakening
   fires at the annotation.
2. **`forget` names what is given up**:
   `forget B from xs` / `forget [B, C] from xs`, typing to the
   conjunction minus the named atoms. It operates on TOP-LEVEL
   conjuncts only (the atom-set representation); structural drops
   inside disjunctions or comparisons use explicit ascription.
   Forgetting an atom that is not there is an error naming it. Zero
   runtime cost (same word; the dropped evidence is simply no longer
   required), and genuinely irreversible: restoring the atom needs a
   fresh `narrow`/`Dec`.

The keep-set form (`weaken xs to {A, C}`) duplicates ascription and is
left out unless API practice shows a need.

## Instance heads at refinements (user direction, 2026-10-27;
## revised to the constraint reading same day)

The primary form is the Haskell-style CONTEXT reading: refinement
predicates are obligations in `using` contexts, on instances and
signatures alike (`instance Semigroup (List Nat) using (Sorted ...)` -
the user's `instance (Sorted list) => Semigroup ...`). The head form
`Semigroup (refined Sorted (List Nat))` is sugar for a head plus its
predicate obligation; the two are inter-derivable and the constraint
form is more general:

- Signatures carry refinement obligations
  (`merge : ... using (Sorted xs, Sorted ys)`), the caller discharging
  them through the `Dec` evidence path with D192 staleness applying.
- The D260 `Preserves` constraint class is unnecessary: preservation
  obligations are written in the predicate language over the method's
  arguments and outputs, so the generic transfer instance is an
  ordinary constraint-parameterized instance.
- Constraint-parameterized instances resolve by supplying evidence at
  the use site (static where the D258 table decides, runtime `Dec`
  otherwise); most-specific resolution orders constraint sets by the
  same implication lattice; incomparable sets matching a common use
  are the coherence error.
- Constraints entail nothing (the standing rule): a `Sorted`
  obligation is explicit and separate.

Below is the earlier head-form framing, kept for the resolution and
coherence rules which are unchanged.

Instance declarations admit refinements in their heads, which both
covers specialized methods and makes the transfer rule a library
instance instead of compiler machinery:

- Specialized methods at refined heads (`instance Semigroup (refined
  Sorted (List Nat))` with `(<>) = merge` - the container's append
  does not preserve sortedness, so rep-identity transfer cannot
  produce it).
- The general transfer is the predicate-parametric instance
  `instance Semigroup (refined p (List a)) using (Semigroup (List a),
  Preserves (<>) p)`, where `Preserves` is a built-in constraint the
  law layer discharges (decidable where the D258 table and the
  preservation checks decide; `not checked` at the honest tier
  otherwise).
- **Resolution is most-specific along the implication lattice**: at
  `refined r`, candidates are instances whose head predicate `r`
  implies (the value weakens to the head); the strongest head wins.
  The generic `Preserves` instance sits at the lattice top and is
  overridden by specificity alone. Two instances at INCOMPARABLE
  heads matching a common refinement have no winner: a coherence
  error naming both heads and the ambiguous refinement (D119
  extended).
- Applicability follows predicate implication, never method totality
  (merge is total on unsorted lists; its instance must not apply
  there or the laws would lie).
- Spelling of `Preserves` in instance contexts (method-named vs the
  `Signature.law` identity spelling) is the implementing slice's
  call.

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
