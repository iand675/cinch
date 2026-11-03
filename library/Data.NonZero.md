# Data.NonZero

Total nonzero-number constructors. The NonZero type is also available through the arithmetic vocabulary for safe divisors.

<span class="support-tag support-tag--current">Embedded</span> Available from the compiler's embedded base family.

Import with `import Data.NonZero`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.NonZero.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Algebra.Monoid](Algebra.Monoid.md), [Algebra.Semigroup](Algebra.Semigroup.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Data.NonZero: the non-zero number as a refinement (refinement types
design; decision D151).

`type NonZero a = refine (n :: a) with NonZero n` (declared in
Algebra.Arithmetic, whose `Div`/`Rem` classes name it, and in the
implicit surface from there; this is its public home): a `NonZero
Nat` IS its `Nat` at runtime, a `NonZero Int` its `Int` (the witness
is `Prop`-erased, decision D31). The carrier is any type the
predicate `NonZero` decides (`Nat`, `Int`, machine integers); any
other carrier is refused naming the predicate.

A numeral is a `NonZero` with nothing written: `10` at `NonZero Nat`
decides its predicate at compile time (and `0` is a compile error).
The total constructors `non-zero`/`non-zero-nat` discharge at
runtime through the `Dec`/`decide` world and return `Option`; the
transparent constructor `NonZero n w` takes the number and its
witness.

The instances below are the multiplicative ones on `NonZero Int`,
written at the container's types (the transfer rule): the product of
non-zeros is non-zero (preserved) and 1 is non-zero, so unlike
`Monoid (NonEmpty a)` (whose identity is empty) `Monoid (NonZero
Int)` exists. The unit laws state over `Semigroup.<>`, which the
Monoid instance does not carry, so they report `not checked`
honestly (the P18 law boundary); asserted in
compiler/tests/refinements.rs.

</details>

# Types and constructors

This module exports no type declarations.

# Values

<a id="value-6e6f6e2d7a65726f"></a>

## `non-zero`

```cinch
non-zero :: (Int -> Option (NonZero Int))
```

The total constructors (refinement types design: the total API).

[Source](/reference-sources/Data.NonZero.cinch) · line 36

<a id="value-6e6f6e2d7a65726f2d6e6174"></a>

## `non-zero-nat`

```cinch
non-zero-nat :: (Nat -> Option (NonZero Nat))
```

[Source](/reference-sources/Data.NonZero.cinch) · line 41

# Instances

These are checked exported instance entries. Law/sample-check status is separate from the existence of an instance; see [laws and optimization](../reference/12-rewrites-and-optimization.md).

| Instance | Target | Resolution |
| --- | --- | --- |
| `Data.NonZero.Monoid_NonZero` | `Monoid (NonZero Int)` | Implicit |
| `Data.NonZero.Semigroup_NonZero` | `Semigroup (NonZero Int)` | Implicit |

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Data/NonZero.cinch`. Source SHA-256: `2eba752e6ccb3d4bb38eb4049a959af604b8d4c03015ccb0f5f15aa9910c4bde`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
