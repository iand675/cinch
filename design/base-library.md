# Base library: module scheme and buildout plan

Plan for the base-library buildout (2026-10-26, user direction: split
the Prelude into modules, re-export the wanted surface, flesh out the
category-theoretic classes, principled naming). No em dashes.

## The naming scheme

A module is named for the mathematical KIND of its contents:

- `Category.*`: classes over type constructors (`f :: Type -> Type` and
  friends): Functor, Applicative, Monad, Alternative, Foldable,
  Traversable, Bifunctor, and the universal visitors (Identity, Const,
  Compose).
- `Algebra.*`: value-level operations and laws on a carrier: Semigroup,
  Monoid, Group, Semiring, Lattice. The newtypes that exist only to
  carry instances (Sum, Product, Dual, First, Last, Endo) live with
  their class module.
- `Relation.*`: value-level comparison and decision: Eq, Ord,
  Decidable (the core `Dec`).
- `Data.*`: concrete types only: List, NonEmpty, Option, Result,
  Ordering, Nat, Int, Char, String, Bytes, Sym, Map, Set, Tuple.
- `Function.*`: combinators over plain functions.
- `Text.*`: conversion to and from String/Bytes (Show, Codec; the
  latter aligns with the A6 CodecGenerator vocabulary).
- `Effect.*`: the effect system's vocabulary: Effect.Sequencing (the
  D22 do providers), Effect.Io, Effect.Concurrent (D74-D81).

Ownership rules:

1. A class lives in the module named for its structure
   (`Monoid` in `Algebra.Monoid`).
2. Instances live with their type; structural instances over functions
   and tuples live with the class module; instances never live
   elsewhere (no orphans, matching the coherence discipline).
3. A class constraint does not entail other classes (the standing
   no-superbundle rule): functions state every constraint they use, so
   `Algebra.Monoid` imports `Algebra.Semigroup` explicitly.

`Prelude` becomes the re-export facade (D107's `export module X` and
`export A.b`), with an explicit curation list. The split itself is an
integration step after the P19 surface slice lands (it owns
`compiler/prelude/Prelude.lang` and is renaming functions to dash
style (D106)).

## The buildout (dispatched)

- `Algebra.Semigroup` + `Algebra.Monoid`: `Semigroup` (`<>`),
  `Monoid` (`mempty`), newtypes Sum/Dual/First/Last/Endo, instances
  for List, Option, Ordering, Nat under addition and multiplication
  (via newtypes), String, functions, and tuples.
- `Category.Functor`/`Category.Applicative`/`Category.Monad`:
  `fmap`/`<$>`, `pure`/`<*>`, `>>=`/`join`, `Identity`/`Const`/
  `Compose`, instances for List, Option, Result e, functions. The
  relationship to the `do` providers (D22) is documented, not
  changed: `do` picks providers lexically; the classes abstract over
  containers.
- `Category.Foldable`/`Category.Traversable` + `Data.NonEmpty` (the
  free semigroup): `foldr`/`foldl`/`toList`/`traverse`/`sequence`,
  instances; `traverse` into `Result e` and the effect rows is the
  showcase.

## The brainstorm menu (open to the user)

In or deferred, with the reasoning:

- `Algebra.Group` and `Algebra.Semiring`: natural next; the
  Sum/Product newtypes give the numeric monoids without a Num tower
  (the primitive-types spec deliberately avoids one).
- `Data.Set` (over Ord) and `Data.Hash` + `Data.HashMap` (needs a Hash
  class; Sym interning makes symbol hashing trivial and hashing ties
  into the interning design).
- `Text.Show` and `Text.Codec`: display and codec derivation; aligns
  with the schema vocabulary (A6).
- `Data.Enum` and range syntax `[1..10]`: useful, but range syntax is
  parser work that should ride a later surface slice.
- Class laws as declared `law` axioms wired into the rewrite `justify`
  machinery (the P18 law/assumption identity work): the differentiator
  that makes `<>` associativity citable evidence rather than a comment.
- Deliberately out: the monad-transformer/mtl zoo (row effects with
  handlers (P6) do that job natively; Reader/State/Writer appear only
  as newtypes if Traversable targets want them), IO-as-a-monad
  (`Io r` is capability-typed and lexical per D21), a numeric tower
  beyond Semiring (the typed families with explicit conversions are
  the spec's deliberate choice), lenses (later).
