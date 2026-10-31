# Data.Nat

Mathematical nonnegative integers, exact arithmetic, checked preconditions, and admitted machine conversions.

<span class="support-tag support-tag--current">Embedded</span> Available from the compiler's embedded base family.

Import with `import Data.Nat`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Nat.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Algebra.Arithmetic](Algebra.Arithmetic.md), [Algebra.Semiring](Algebra.Semiring.md), [Data.Core](Data.Core.md), [Data.Hash](Data.Hash.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Data.Nat: the mathematical natural numbers.

`Nat` is a core runtime type (primitive types specification section
1): a boxed bignum, distinct from every machine scalar. The runtime
operations are representation-level and declared through `foreign
import`. Division and remainder are total by construction: the
divisor is a `NonZero Nat` (a numeral divisor decides at compile
time, any other comes from `decide (NonZero d)` or is threaded), and
`checked-div`/`checked-rem` answer `None` where a zero divisor is a
meaningful input. Subtraction is total the same way
(design/no-faults.md section 7): `a - b` and `nat-sub a b` require
evidence that `b <= a` (numerals decide it at compile time; a branch
condition such as `if b <= a then` supplies it), `saturating-sub`
is the truncating form (`max(a - b, 0)`, "monus") under its own
name, and `checked-sub` answers `None` when `b > a`.

Module scheme (design/base-library.md): the type's home carries its
instance family. `Nat` gets no `Semigroup` instance: two monoids
live on it and the newtypes `Sum`/`Product` choose (Algebra.Monoid).
`Semiring` names BOTH operations, so the carrier itself can be a
semiring: this is the `Sum`/`Product` story extended (no Num tower;
machine scalars keep their explicit overflow families and get no
`Semiring` instance).

</details>

# Types and constructors

This module exports no type declarations.

# Values

<a id="value-636865636b65642d646976"></a>

## `checked-div`

```cinch
checked-div :: (Nat -> (Nat -> Option Nat))
```

Division where a zero divisor is an ordinary input the caller
handles: `None` exactly when `d` is zero. The decision is the
divisor's evidence, so the division inside is the total one.

[Source](/reference-sources/Data.Nat.cinch) · line 63

<a id="value-636865636b65642d6e61742d66726f6d2d693634"></a>

## `checked-nat-from-i64`

```cinch
checked-nat-from-i64 :: (I64 -> Option Nat)
```

The `I64` conversions where an out-of-range value is an ordinary
input: `None` exactly then.

[Source](/reference-sources/Data.Nat.cinch) · line 86

<a id="value-636865636b65642d6e61742d746f2d693634"></a>

## `checked-nat-to-i64`

```cinch
checked-nat-to-i64 :: (Nat -> Option I64)
```

[Source](/reference-sources/Data.Nat.cinch) · line 89

<a id="value-636865636b65642d72656d"></a>

## `checked-rem`

```cinch
checked-rem :: (Nat -> (Nat -> Option Nat))
```

[Source](/reference-sources/Data.Nat.cinch) · line 68

<a id="value-636865636b65642d737562"></a>

## `checked-sub`

```cinch
checked-sub :: (Nat -> (Nat -> Option Nat))
```

Subtraction where `b > a` is an ordinary input the caller handles:
`None` exactly then. The test is the subtraction's evidence.

[Source](/reference-sources/Data.Nat.cinch) · line 81

<a id="value-6e61742d616464"></a>

## `nat-add`

```cinch
nat-add :: (Nat -> (Nat -> Nat))
```

The mathematical `Nat` and `Int` (primitive types specification,
section 1): boxed bignums, distinct from every machine scalar.

[Source](/reference-sources/Data.Nat.cinch) · line 37

<a id="value-6e61742d636d70"></a>

## `nat-cmp`

```cinch
nat-cmp :: (Nat -> (Nat -> I64))
```

[Source](/reference-sources/Data.Nat.cinch) · line 45

<a id="value-6e61742d646976"></a>

## `nat-div`

```cinch
nat-div :: (Nat -> (NonZero Nat -> Nat))
```

[Source](/reference-sources/Data.Nat.cinch) · line 41

<a id="value-6e61742d66726f6d2d693634"></a>

## `nat-from-i64`

```cinch
nat-from-i64 :: using (0 <= arg0) => ((arg0 :: I64) -> refined (n == arg0) Nat)
```

The `I64` conversions keep the value (`n == x`), and each states
the range it needs: a negative `I64` is no `Nat`, and a `Nat` past
`2^63 - 1` is no `I64`. A conversion result (`u32-to-i64`), a mask
(`mask-i64 x 255`) or a guard supplies the evidence; where the
value may be out of range, `checked-nat-from-i64`/
`checked-nat-to-i64` answer `None`.

[Source](/reference-sources/Data.Nat.cinch) · line 52

<a id="value-6e61742d66726f6d2d696e74"></a>

## `nat-from-int`

```cinch
nat-from-int :: using (0 <= arg0) => ((arg0 :: Int) -> refined (n == arg0) Nat)
```

[Source](/reference-sources/Data.Nat.cinch) · line 58

<a id="value-6e61742d6d756c"></a>

## `nat-mul`

```cinch
nat-mul :: (Nat -> (Nat -> Nat))
```

[Source](/reference-sources/Data.Nat.cinch) · line 40

<a id="value-6e61742d72656d"></a>

## `nat-rem`

```cinch
nat-rem :: (Nat -> ((a1 :: NonZero Nat) -> refined (r < a1) Nat))
```

The remainder is below the divisor (`r < d`, a fact about each
call's result: `let r = nat-rem v 256` knows `r < 256`).

[Source](/reference-sources/Data.Nat.cinch) · line 44

<a id="value-6e61742d737562"></a>

## `nat-sub`

```cinch
nat-sub :: using (arg1 <= arg0) => ((arg0 :: Nat) -> ((arg1 :: Nat) -> Nat))
```

The exact difference: the caller proves it is a `Nat`.

[Source](/reference-sources/Data.Nat.cinch) · line 39

<a id="value-6e61742d746f2d693634"></a>

## `nat-to-i64`

```cinch
nat-to-i64 :: using (arg0 <= 9223372036854775807) => ((arg0 :: Nat) -> refined (x == arg0) I64)
```

[Source](/reference-sources/Data.Nat.cinch) · line 53

<a id="value-6e61742d746f2d696e74"></a>

## `nat-to-int`

```cinch
nat-to-int :: ((a0 :: Nat) -> refined (x == a0) Int)
```

The `Int` conversions: every `Nat` is an `Int`, and a non-negative
`Int` is a `Nat`, both keeping the value (no `I64` in between, so a
value of any size converts).

[Source](/reference-sources/Data.Nat.cinch) · line 57

<a id="value-73617475726174696e672d737562"></a>

## `saturating-sub`

```cinch
saturating-sub :: (Nat -> (Nat -> Nat))
```

Truncated subtraction ("monus", Rust's `saturating_sub`): `a - b`
when `b <= a`, else `0`. Use it where clamping at zero is what the
code means, not to silence the obligation of `-`.

[Source](/reference-sources/Data.Nat.cinch) · line 76

# Instances

These are checked exported instance entries. Law/sample-check status is separate from the existence of an instance; see [laws and optimization](../reference/12-rewrites-and-optimization.md).

| Instance | Target | Resolution |
| --- | --- | --- |
| `Data.Nat.Add_Nat` | `Add Nat` | Implicit |
| `Data.Nat.Div_Nat` | `Div Nat` | Implicit |
| `Data.Nat.Enum_Nat` | `Enum Nat` | Implicit |
| `Data.Nat.Eq_Nat` | `Eq Nat` | Implicit |
| `Data.Nat.Hash_Nat` | `Hash Nat` | Implicit |
| `Data.Nat.Mul_Nat` | `Mul Nat` | Implicit |
| `Data.Nat.Ord_Nat` | `Ord Nat` | Implicit |
| `Data.Nat.Rem_Nat` | `Rem Nat` | Implicit |
| `Data.Nat.Semiring_Nat` | `Semiring Nat` | Implicit |
| `Data.Nat.Sub_Nat` | `Sub Nat` | Implicit |

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Data/Nat.cinch`. Source SHA-256: `6cd6f13c374b0fdaa230e5e5e0266d5cd75a88e8f60e8e2f81d5a355a6a15d5b`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
