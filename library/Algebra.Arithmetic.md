# Algebra.Arithmetic

Arithmetic operation signatures and their fixities, including NonZero divisors and InRange refinements.

<span class="support-tag support-tag--current">Embedded</span> Available from the compiler's embedded base family.

Import with `import Algebra.Arithmetic`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Algebra.Arithmetic.cinch) · [How library modules are loaded](00-index.md)

<details><summary>Behaviour and boundary notes from the source</summary>

Algebra.Arithmetic: the arithmetic operation classes.

The primitive types specification's arithmetic classes (section 8):
`Add`, `Sub`, `Mul` and `Neg` on the mathematical types, `Div` and
`Rem` for their division. These are operation classes, not the
named algebraic structures of the base library scheme
(design/base-library.md's Algebra.* family is Semigroup, Monoid,
Group, Semiring, Lattice); they group here as the arithmetic
operations kind. Machine scalars get NO arithmetic instances: a
class method has a single result type and cannot express which
overflow family is meant (primitive types specification section 8).
Division and remainder are total by construction: the divisor is a
`NonZero a` (design/no-faults.md). Instances live with their type.

</details>

# Types and constructors

<a id="type-496e52616e6765"></a>

## `InRange`

```cinch
type InRange (lo :: Int) (hi :: Int) (a :: Type) = refine (x :: a) with (lo <= x && x <= hi)
```

A range of a number, the result type of every operation whose
result lies in known bounds by construction (`u32-to-i64 :: U32 ->
InRange 0 4294967295 I64`, `bytes-get`'s `InRange 0 255 Nat`,
`char-code`'s `InRange 0 1114111 Nat`). The bounds are inclusive
index parameters, so the predicate is two comparisons the
difference-logic discharge reads directly: a let-bound result
carries `lo <= x && x <= hi` as a fact, an application in an
obligation carries it for its term, and where no range is demanded
the value is its number (`c - 48`, `nat-add c 1`).

[Source](/reference-sources/Algebra.Arithmetic.cinch) · line 41

<a id="type-4e6f6e5a65726f"></a>

## `NonZero`

```cinch
type NonZero (a :: Type) = refine (n :: a) with NonZero n
```

The divisor type, declared here because the `Div`/`Rem` classes
and the `Nat`/`Int` homes (checked right after this module) name
it in their signatures; its total constructors and instances live
in its public home, `Data.NonZero`. One refinement states the
predicate for every carrier it decides (`NonZero Nat`, `NonZero
Int`, a machine integer): the carrier is the container, so a
`NonZero Nat` IS its `Nat` at runtime, and a numeral decides its
predicate at compile time.

[Source](/reference-sources/Algebra.Arithmetic.cinch) · line 30

# Signatures and methods

<a id="class-416464"></a>

## `Add`

```cinch
signature Add a
```

[Source](/reference-sources/Algebra.Arithmetic.cinch) · line 44

<a id="method-4164642e2b"></a>

## `Add.+`

```cinch
+ :: forall a . using (dict :: Add a) => (a -> (a -> a))
```

[Source](/reference-sources/Algebra.Arithmetic.cinch) · line 45

<a id="class-446976"></a>

## `Div`

```cinch
signature Div a
```

[Source](/reference-sources/Algebra.Arithmetic.cinch) · line 68

<a id="method-4469762e2f"></a>

## `Div./`

```cinch
/ :: forall a . using (dict :: Div a) => (a -> (NonZero a -> a))
```

[Source](/reference-sources/Algebra.Arithmetic.cinch) · line 69

<a id="class-4d756c"></a>

## `Mul`

```cinch
signature Mul a
```

[Source](/reference-sources/Algebra.Arithmetic.cinch) · line 50

<a id="method-4d756c2e2a"></a>

## `Mul.*`

```cinch
* :: forall a . using (dict :: Mul a) => (a -> (a -> a))
```

[Source](/reference-sources/Algebra.Arithmetic.cinch) · line 51

<a id="class-4e6567"></a>

## `Neg`

```cinch
signature Neg a
```

[Source](/reference-sources/Algebra.Arithmetic.cinch) · line 53

<a id="method-4e65672e6e6567617465"></a>

## `Neg.negate`

```cinch
negate :: forall a . using (dict :: Neg a) => (a -> a)
```

[Source](/reference-sources/Algebra.Arithmetic.cinch) · line 54

<a id="class-52656d"></a>

## `Rem`

```cinch
signature Rem a
```

Modulo and division for the mathematical types (decision D91).
The divisor carries its evidence: a `NonZero a`, so no instance
can reach a zero divisor. A numeral divisor needs nothing written
(`n / 10` and `n % 10` decide `NonZero 10` at compile time); any
other divisor is a threaded `NonZero a` or the product of a
runtime decision (`decide (NonZero d)`). A total division whose
divisor may be zero is not this operation: `Data.Complex.div`'s
IEEE quotient stays a named function. Machine integers keep the
checked family only, per the primitive types specification.

[Source](/reference-sources/Algebra.Arithmetic.cinch) · line 65

<a id="method-52656d2e25"></a>

## `Rem.%`

```cinch
% :: forall a . using (dict :: Rem a) => (a -> (NonZero a -> a))
```

[Source](/reference-sources/Algebra.Arithmetic.cinch) · line 66

<a id="class-537562"></a>

## `Sub`

```cinch
signature Sub a
```

[Source](/reference-sources/Algebra.Arithmetic.cinch) · line 47

<a id="method-5375622e2d"></a>

## `Sub.-`

```cinch
- :: forall a . using (dict :: Sub a) => (a -> (a -> a))
```

[Source](/reference-sources/Algebra.Arithmetic.cinch) · line 48

# Operator fixities

```cinch
infixl 9 %
infixl 9 *
infixl 8 +
infixl 8 -
infixl 9 /
```

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Algebra/Arithmetic.cinch`. Source SHA-256: `2fac63baa07b5f42a8c907b0ba821bde5401a6e50f76f5d81f24e30d392f5ee0`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
