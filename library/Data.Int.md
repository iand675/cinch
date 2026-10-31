# Data.Int

Mathematical signed integers, arithmetic, division/remainder, formatting support, and explicit conversions.

<span class="support-tag support-tag--current">Embedded</span> Available from the compiler's embedded base family.

Import with `import Data.Int`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Int.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Algebra.Arithmetic](Algebra.Arithmetic.md), [Algebra.Group](Algebra.Group.md), [Algebra.Semiring](Algebra.Semiring.md), [Data.Core](Data.Core.md), [Data.Hash](Data.Hash.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Data.Int: the mathematical integers.

`Int` is a core runtime type (primitive types specification section
1): a boxed bignum, distinct from every machine scalar. Division
and remainder are total by construction: the divisor is a `NonZero
Int` (see Data.Nat).

Module scheme (design/base-library.md): the type's home carries its
instance family. `Int` gets no `Semigroup` instance (two monoids
live on it, chosen by `Sum`/`Product` in Algebra.Monoid); its
`Semiring` names both operations, and its `Group` is the additive
group of that semiring (`invert` is `int-neg`, the only total
inversion on `Int`).

</details>

# Types and constructors

This module exports no type declarations.

# Values

<a id="value-636865636b65642d696e742d746f2d693634"></a>

## `checked-int-to-i64`

```cinch
checked-int-to-i64 :: (Int -> Option I64)
```

[Source](/reference-sources/Data.Int.cinch) · line 38

<a id="value-696e742d616464"></a>

## `int-add`

```cinch
int-add :: (Int -> (Int -> Int))
```

[Source](/reference-sources/Data.Int.cinch) · line 24

<a id="value-696e742d636d70"></a>

## `int-cmp`

```cinch
int-cmp :: (Int -> (Int -> I64))
```

[Source](/reference-sources/Data.Int.cinch) · line 30

<a id="value-696e742d646976"></a>

## `int-div`

```cinch
int-div :: (Int -> (NonZero Int -> Int))
```

[Source](/reference-sources/Data.Int.cinch) · line 27

<a id="value-696e742d66726f6d2d693634"></a>

## `int-from-i64`

```cinch
int-from-i64 :: ((a0 :: I64) -> refined (r == a0) Int)
```

The `I64` conversions keep the value; an `Int` past the `I64`
range is no `I64`, so `int-to-i64` states the range it needs (a
guard or a conversion result supplies it) and `checked-int-to-i64`
answers `None` outside it.

[Source](/reference-sources/Data.Int.cinch) · line 35

<a id="value-696e742d6d6f642d6e6174"></a>

## `int-mod-nat`

```cinch
int-mod-nat :: (Int -> (NonZero Nat -> Nat))
```

The floored residue of `x` modulo `d` as a `Nat`, `0 <= r < d`
for every `x` (a negative `x` counts up from `d`): an encoding into
`d` values that needs no range of `x`.

[Source](/reference-sources/Data.Int.cinch) · line 45

<a id="value-696e742d6d756c"></a>

## `int-mul`

```cinch
int-mul :: (Int -> (Int -> Int))
```

[Source](/reference-sources/Data.Int.cinch) · line 26

<a id="value-696e742d6e6567"></a>

## `int-neg`

```cinch
int-neg :: (Int -> Int)
```

[Source](/reference-sources/Data.Int.cinch) · line 29

<a id="value-696e742d72656d"></a>

## `int-rem`

```cinch
int-rem :: (Int -> (NonZero Int -> Int))
```

[Source](/reference-sources/Data.Int.cinch) · line 28

<a id="value-696e742d737562"></a>

## `int-sub`

```cinch
int-sub :: (Int -> (Int -> Int))
```

[Source](/reference-sources/Data.Int.cinch) · line 25

<a id="value-696e742d746f2d693634"></a>

## `int-to-i64`

```cinch
int-to-i64 :: using (-9223372036854775808 <= arg0, arg0 <= 9223372036854775807) => ((arg0 :: Int) -> refined (r == arg0) I64)
```

[Source](/reference-sources/Data.Int.cinch) · line 36

# Instances

These are checked exported instance entries. Law/sample-check status is separate from the existence of an instance; see [laws and optimization](../reference/12-rewrites-and-optimization.md).

| Instance | Target | Resolution |
| --- | --- | --- |
| `Data.Int.Add_Int` | `Add Int` | Implicit |
| `Data.Int.Div_Int` | `Div Int` | Implicit |
| `Data.Int.Enum_Int` | `Enum Int` | Implicit |
| `Data.Int.Eq_Int` | `Eq Int` | Implicit |
| `Data.Int.Group_Int` | `Group Int` | Implicit |
| `Data.Int.Hash_Int` | `Hash Int` | Implicit |
| `Data.Int.Mul_Int` | `Mul Int` | Implicit |
| `Data.Int.Neg_Int` | `Neg Int` | Implicit |
| `Data.Int.Ord_Int` | `Ord Int` | Implicit |
| `Data.Int.Rem_Int` | `Rem Int` | Implicit |
| `Data.Int.Semiring_Int` | `Semiring Int` | Implicit |
| `Data.Int.Sub_Int` | `Sub Int` | Implicit |

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Data/Int.cinch`. Source SHA-256: `3298641d85d00bf4173a13060bbdfd1e8e63b8c3ff79b96e635525485bfd45ea`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
