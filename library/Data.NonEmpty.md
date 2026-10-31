# Data.NonEmpty

Lists refined by nonemptiness. The total constructor returns Option; transferred operations must preserve the predicate.

<span class="support-tag support-tag--current">Embedded</span> Available from the compiler's embedded base family.

Import with `import Data.NonEmpty`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.NonEmpty.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Algebra.Semigroup](Algebra.Semigroup.md), [Category.Applicative](Category.Applicative.md), [Category.Foldable](Category.Foldable.md), [Category.Traversable](Category.Traversable.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Data.NonEmpty: the non-empty list as a refinement (refinement types
design; decision D151: `NonEmpty` is a decidable predicate over
`List`, not a wrapper).

`type NonEmpty a = refine (xs :: List a) with NotNull xs`: `NonEmpty`
IS `List a` at runtime (the witness is `Prop`-erased, decision D31),
so `unrefine` projects at zero cost and no delegation code exists.
The transparent constructor `NonEmpty xs w` takes the list and its
witness: a `Dec (NotNull xs)` discharge that erases, or the `Yes`
evidence of a `decide`.

The total constructor `non-empty` discharges at runtime through the
`Dec`/`decide` world and returns `Option` (the total API of the
design note). Forging is impossible: a witness is checked evidence,
and evidence about mutable state dies with the write (the D192
invalidation rule).

Instances are the container's own methods (the transfer rule),
checked sample-wise by the P18 law layer exactly when they preserve
the predicate: `Semigroup` transfers (append of non-empties is
non-empty), `Monoid` cannot (its identity is empty; both asserted in
compiler/tests/refinements.rs). The traversals preserve
non-emptiness too, though through the applicative they report `not
checked` at samples (the honest law boundary).

Status: this module is the P28 refinement of the bespoke `NonEmpty`
data type it replaces (decision D151).

</details>

# Types and constructors

<a id="type-4e6f6e456d707479"></a>

## `NonEmpty`

```cinch
type NonEmpty (a :: Type) = refine (xs :: List a) with NotNull xs
```

[Source](/reference-sources/Data.NonEmpty.cinch) · line 36

# Values

<a id="value-6e6f6e2d656d707479"></a>

## `non-empty`

```cinch
non-empty :: forall a . (List a -> Option (NonEmpty a))
```

The total constructor (refinement types design: the total API):
the runtime decision lives in the `Dec`/`decide` world and its
witness erases into the transparent constructor.

[Source](/reference-sources/Data.NonEmpty.cinch) · line 41

# Instances

These are checked exported instance entries. Law/sample-check status is separate from the existence of an instance; see [laws and optimization](../reference/12-rewrites-and-optimization.md).

| Instance | Target | Resolution |
| --- | --- | --- |
| `Data.NonEmpty.Foldable_NonEmpty` | `Foldable NonEmpty` | Implicit |
| `Data.NonEmpty.Semigroup_NonEmpty` | `Semigroup (NonEmpty x)` | Implicit |
| `Data.NonEmpty.Traversable_NonEmpty` | `Traversable NonEmpty` | Implicit |

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Data/NonEmpty.cinch`. Source SHA-256: `62b6ff5b85deab84882b0b901a807f1531d7d9db7cd252e2ff5c31e9ecc0d900`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
