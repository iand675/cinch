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

## The brainstorm menu (resolved 2026-10-26, user approved)

Pulled in: `Algebra.Group` and `Algebra.Semiring` (the Sum/Product
newtypes keep the numeric monoids out of any Num tower); `Data.Set`
plus `Data.Hash`/`Data.HashMap` (Sym interning makes symbol hashing
trivial); law declarations wired into the rewrite `justify` machinery
(riding the P18 law/assumption identity tier: a class `law` is citable
evidence, and instances are checked against their laws); `Text.Show`
and `Text.Codec` (aligned with the A6 CodecGenerator).

Queued: `Data.Enum` and range syntax `[1..10]` ride the next surface
slice (parser work, not bolted onto P19 mid-flight). `Data.NonEmpty`
is already building with the Traversable family.

Held until a consumer appears: `Category.Profunctor`/`Category.Category`
/`Kleisli`, `Effect.State`/`Effect.Reader` as newtypes.

Deliberately out (unchanged): the monad-transformer/mtl zoo (row
effects with handlers (P6) do that job natively), IO-as-a-monad
(`Io r` is capability-typed and lexical per D21), a numeric tower
beyond Semiring (the typed families with explicit conversions are the
spec's deliberate choice), lenses (later).
