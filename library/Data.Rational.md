# Data.Rational

Rational numbers with explicit denominator/error policy and arithmetic operations.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Data.Rational`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Rational.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Hash](Data.Hash.md), [Data.Int](Data.Int.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Ordering](Data.Ordering.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Relation.Eq](Relation.Eq.md), [Relation.Ord](Relation.Ord.md), [Text.Show](Text.Show.md).

Non-embedded source closure: [Data.Rational](Data.Rational.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Data.Rational: the exact rational number over the bignum `Int`.

A `Rational` is an integer pair `n/d` kept NORMALISED: the
denominator is strictly positive and the two are coprime, so equal
mathematical values have exactly one representative and `Eq` is
structural. This is Ruby's `Rational` (design/stdlib-parity-ruby.md,
P38 PORT, "Rational ... as concrete types with explicit operations,
the P2 discipline: types and operations, no numeric tower"):
`Rational(1,2) == Rational(2,4)`, the sign rides on the numerator
(`Rational(-1,2).denominator` is `2`), and arithmetic is exact
because it is bignum, never a machine float.

Design law shape. The operations are named functions over one
concrete type (no `Fractional` class, no numeric tower): `add`,
`sub` and `mul` are total and exact, `from-int` embeds the integers,
and the order is cross multiplication (`rational-cmp r s` has the
sign of `n r * d s` against `n s * d r`, both denominators positive).
The `Eq` and `Hash` instances are consistent with reduction by
construction: the only route to a value normalises it, so `r == s`
exactly when their unique representatives agree, and equal values
hash alike.

The type is ABSTRACT. The raw constructor stays off the export list
(the surface rule D143 and the constructor privacy of D308), so a
`Rational` violating the invariant is unrepresentable: `rational`
is the only general constructor, `from-int` cannot fail, and
`numerator`/`denominator` are the only observers of the pair.

Named refusals (each a named value, never a silent fault; Ruby's
`ZeroDivisionError` in the two places valuation reaches it):
  a zero denominator cannot be constructed: `rational n 0` is
    `Err ZeroDenominator`;
  a zero divisor cannot be inverted: `div x y` with `y` equal to
    `from-int 0` is `Err DivideByZero`. `div` is the only partial
    operation; `add`/`sub`/`mul` are total.

`to-f64` boundary (documented, not silent): a rational whose
numerator and denominator each fit `I64` divides their two nearest
`F64` values, so the result is within two units in the last place of
the exact quotient, and exact when both magnitudes are at most 2^53.
Outside `I64` each is assembled from 32-bit limbs (`int-to-f64`), so
a huge one is a rounded float or an infinity rather than a fault.

Module scheme (design/base-library.md): `Data.*` is concrete types,
and the type's home carries its `Eq`/`Ord`/`Hash` instance family.

</details>

# Types and constructors

<a id="type-526174696f6e616c"></a>

## `Rational`

```cinch
data Rational
```

The rational number: `Rat n d` is `n/d`, `d` strictly positive and
`gcd(|n|, d)` equal to 1. The constructor is module-private: only
`normalise` (reached through `rational`, `from-int` and the
operations) builds one, so every value the surface hands out
satisfies the invariant.

[Source](/reference-sources/Data.Rational.cinch) · line 70

<a id="type-526174696f6e616c4572726f72"></a>

## `RationalError`

```cinch
data RationalError
```

Why a rational is not a value or is not invertible: both refusals
reach a zero denominator, and both are reported rather than
faulted.

[Source](/reference-sources/Data.Rational.cinch) · line 76

```cinch
ZeroDenominator :: RationalError
```

```cinch
DivideByZero :: RationalError
```

# Values

<a id="value-616464"></a>

## `add`

```cinch
add :: (Rational -> (Rational -> Rational))
```

Exact arithmetic: one cross-product over the bignum, then one
reduction. The denominators are positive products, and `normalise`
resolves the sign the addition itself cannot introduce.

[Source](/reference-sources/Data.Rational.cinch) · line 141

<a id="value-64656e6f6d696e61746f72"></a>

## `denominator`

```cinch
denominator :: (Rational -> Int)
```

[Source](/reference-sources/Data.Rational.cinch) · line 129

<a id="value-646976"></a>

## `div`

```cinch
div :: (Rational -> (Rational -> Result RationalError Rational))
```

Division, the one partial operation: a zero divisor (a zero
numerator on the right) is the named refusal, otherwise the cross
product `n1*d2 / d1*n2` normalises.

[Source](/reference-sources/Data.Rational.cinch) · line 160

<a id="value-66726f6d2d696e74"></a>

## `from-int`

```cinch
from-int :: (Int -> Rational)
```

The integer embedding `x/1`, already normalised (1 is coprime to
every integer).

[Source](/reference-sources/Data.Rational.cinch) · line 135

<a id="value-6d756c"></a>

## `mul`

```cinch
mul :: (Rational -> (Rational -> Rational))
```

[Source](/reference-sources/Data.Rational.cinch) · line 153

<a id="value-6e756d657261746f72"></a>

## `numerator`

```cinch
numerator :: (Rational -> Int)
```

[Source](/reference-sources/Data.Rational.cinch) · line 125

<a id="value-726174696f6e616c"></a>

## `rational`

```cinch
rational :: (Int -> (Int -> Result RationalError Rational))
```

The smart constructor and the only general route to a `Rational`:
a zero denominator is the named refusal, everything else
normalises.

[Source](/reference-sources/Data.Rational.cinch) · line 120

<a id="value-726174696f6e616c2d636d70"></a>

## `rational-cmp`

```cinch
rational-cmp :: (Rational -> (Rational -> Ordering))
```

Order by cross multiplication: both denominators are positive, so
`n1*d2` against `n2*d1` has the sign of `x - y`. `Relation.Ord`
carries no `cmp` method, so this named comparison is the spelling,
and the instance below derives the operators from it.

[Source](/reference-sources/Data.Rational.cinch) · line 169

<a id="value-72656e6465722d726174696f6e616c2d6572726f72"></a>

## `render-rational-error`

```cinch
render-rational-error :: (RationalError -> String)
```

[Source](/reference-sources/Data.Rational.cinch) · line 204

<a id="value-737562"></a>

## `sub`

```cinch
sub :: (Rational -> (Rational -> Rational))
```

[Source](/reference-sources/Data.Rational.cinch) · line 147

<a id="value-746f2d663634"></a>

## `to-f64`

```cinch
to-f64 :: (Rational -> F64)
```

The value as a machine float (see the boundary note in the
header).

[Source](/reference-sources/Data.Rational.cinch) · line 178

<a id="value-746f2d737472696e67"></a>

## `to-string`

```cinch
to-string :: (Rational -> String)
```

The reduced rendering `n/d`: `1/2` for `Rational(1,2)`, `-1/2` for
`Rational(-1,2)`, and `n/1` (not `n`) for a whole number, Ruby's
spelling.

[Source](/reference-sources/Data.Rational.cinch) · line 200

# Instances

These are checked exported instance entries. Law/sample-check status is separate from the existence of an instance; see [laws and optimization](../reference/12-rewrites-and-optimization.md).

| Instance | Target | Resolution |
| --- | --- | --- |
| `Data.Rational.Eq_Rational` | `Eq Rational` | Implicit |
| `Data.Rational.Hash_Rational` | `Hash Rational` | Implicit |
| `Data.Rational.Ord_Rational` | `Ord Rational` | Implicit |

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Data/Rational.cinch`. Source SHA-256: `87c8c6ce39162e1571a7f18ea2e6972fc514739a0d33788fb0ae0dfd557164ac`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
