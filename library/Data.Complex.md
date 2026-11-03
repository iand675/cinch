# Data.Complex

Complex values and numeric operations using the module's current floating representation.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Data.Complex`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Complex.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Algebra.Arithmetic](Algebra.Arithmetic.md), [Data.Machine](Data.Machine.md), [Data.String](Data.String.md), [Relation.Eq](Relation.Eq.md).

Non-embedded source closure: [Data.Complex](Data.Complex.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Data.Complex: complex numbers over the binary64 scalar.

Ruby's `Complex` (design/stdlib-parity-ruby.md, the P38 numeric family:
PORT as a concrete type with explicit operations, the P2 discipline of
types and operations rather than a numeric tower). The carrier is fixed
to `F64` (Data.Machine's binary64 scalar): both parts are machine floats,
so the operations below are the field operations named explicitly rather
than a promotion system. `complex` is the smart constructor (the
representation stays private, like `Data.String`'s `StrBuf`), and
`real-part`/`imag-part` are the observations that cross back to `F64`.

Design law shape (design/base-library.md: `Data.*` is concrete types, and
the type's home carries its instance family): the defining identities the
tests assert are
  the imaginary unit:   mul (complex 0 1) (complex 0 1) == complex -1 0
  the sample product:   mul (complex 1 2) (complex 3 4) == complex -5 10
  distributivity:       mul x (add y z) == add (mul x y) (mul x z)
  conjugation:          conjugate (conjugate z) == z
  the field inverse:    div (mul x y) y == x within the tolerance, where
                        y is not zero (division by zero is IEEE, not a
                        fault: the parts go through `div-f64`)
and the instance family (`Eq`, `Add`, `Sub`, `Mul`, `Neg`) is the
class surface over those named functions. The arithmetic-operation
classes DO admit this carrier: the class method is over `Complex`, not
over `F64`, so `Add Complex` adds both parts with `add-f64` (the refusal
of arithmetic instances on the machine scalars themselves, primitive
types specification section 8, is not touched).

Named refusals:
  * No `Complex` over `Int` (nor any carrier but `F64`): the type is
    fixed here. Ruby's `Complex` promotes across `Numeric`; this slice
    deliberately does not, and mixed real/complex arithmetic is refused
    too (`from-real`, `real-part` and `imag-part` are the crossings).
  * No ordering: comparison is absent on purpose. The parts are `F64`,
    which has no `Ord` instance (primitive types specification section
    8), and there is no total order on the complex field compatible with
    its arithmetic. The bit-pattern order `total-order-f64` orders NaN
    and signed zeros, not values, so it is not an ordering either.
  * `Eq` is structural on the parts through `feq-f64`, so a value with a
    NaN part is unequal to itself (IEEE equality, matching Ruby's
    componentwise `==`).
  * No `Div` instance: `Algebra.Arithmetic`'s `(/)` divides by a
    `NonZero` divisor (division is total by construction), and the
    predicate `NonZero` decides numbers, not `Complex`. Complex
    division is total in its own way (IEEE parts, never a fault), so
    it stays the named function `div`.
  * No `Semiring`/`Group` instances: this module provides exactly the
    arithmetic-operation classes (`Algebra.Arithmetic`) and leaves the
    algebraic-structure classes to a later curation (a class constraint
    entails nothing, so nothing is lost).
  * `to-string` renders each part through the P31 round-trip decimal
    (Data.Machine's `f64-to-decimal`: the shortest round-trip spelling)
    with a trailing `e0` elided, so whole and simple fractional values
    read as plain decimals. A part whose decimal needs a nonzero
    exponent keeps its scientific spelling; no fixed-notation expansion
    is attempted in this slice.
  * `arg`/`atan-approx` are a named APPROXIMATION. The binary64 scalar
    carries no arctangent primitive (Data.Machine has no `atan`), so
    `atan-approx` reduces the argument four times through
    `t/(1+sqrt(1+t^2))` (each reduction halves the angle) and sums the
    first eight odd terms of the arctangent series. The absolute error
    is below 1e-12 over the whole real line: the reduced truncation (the
    reduced argument is at most 2^-4) is below 2^-60 and the four
    doublings scale it by 2^4, with the `pi/2` constant adding binary64
    rounding noise. `arg` is the four-quadrant `atan2` built on it.

Instance homes. Instances live with their type (design/base-library.md);
the class names come through the implicit prelude facade, the same shape
Data.Int's instances use. Names are dash style (D106); operators are
class methods with the base fixity table (D2).

</details>

# Types and constructors

<a id="type-436f6d706c6578"></a>

## `Complex`

```cinch
data Complex
```

The pair of binary64 parts. The constructor is module-private (the
observations below are the only way out); `complex` is the surface.

[Source](/reference-sources/Data.Complex.cinch) · line 83

# Values

<a id="value-616273"></a>

## `abs`

```cinch
abs :: (Complex -> F64)
```

The magnitude |a + bi| = sqrt(a^2 + b^2). Naive formula (no scaling):
a part large enough to overflow the squares yields an infinite
magnitude, which is the honest IEEE result for this spelling.

[Source](/reference-sources/Data.Complex.cinch) · line 148

<a id="value-616464"></a>

## `add`

```cinch
add :: (Complex -> (Complex -> Complex))
```

(a + bi) + (c + di): componentwise (the field's addition).

[Source](/reference-sources/Data.Complex.cinch) · line 103

<a id="value-617267"></a>

## `arg`

```cinch
arg :: (Complex -> F64)
```

The four-quadrant argument: the angle of `a + bi` in (-pi, pi], zero
for the origin. The axis cases are exact; the other quadrants defer to
`atan-approx` (the approximation's bound is the error bound here).

[Source](/reference-sources/Data.Complex.cinch) · line 228

<a id="value-6174616e2d617070726f78"></a>

## `atan-approx`

```cinch
atan-approx :: (F64 -> F64)
```

The named arctangent approximation (the header's refusal entry):
`atan-approx x` is the principal value in (-pi/2, pi/2).

[Source](/reference-sources/Data.Complex.cinch) · line 160

<a id="value-636f6d706c6578"></a>

## `complex`

```cinch
complex :: (F64 -> (F64 -> Complex))
```

The smart constructor (Ruby's `Complex(a, b)`).

[Source](/reference-sources/Data.Complex.cinch) · line 87

<a id="value-636f6e6a7567617465"></a>

## `conjugate`

```cinch
conjugate :: (Complex -> Complex)
```

The conjugate: the reflection across the real axis. An involution
(`conjugate (conjugate z) == z`).

[Source](/reference-sources/Data.Complex.cinch) · line 142

<a id="value-646976"></a>

## `div`

```cinch
div :: (Complex -> (Complex -> Complex))
```

(a + bi)/(c + di) = ((ac + bd) + (bc - ad)i) / (c^2 + d^2). A zero
divisor yields IEEE NaN parts through `div-f64`, never a fault.

[Source](/reference-sources/Data.Complex.cinch) · line 125

<a id="value-6571"></a>

## `eq`

```cinch
eq :: (Complex -> (Complex -> Bool))
```

Structural equality: IEEE on each part.

[Source](/reference-sources/Data.Complex.cinch) · line 155

<a id="value-66726f6d2d7265616c"></a>

## `from-real`

```cinch
from-real :: (F64 -> Complex)
```

A complex number with zero imaginary part.

[Source](/reference-sources/Data.Complex.cinch) · line 99

<a id="value-696d61672d70617274"></a>

## `imag-part`

```cinch
imag-part :: (Complex -> F64)
```

[Source](/reference-sources/Data.Complex.cinch) · line 94

<a id="value-6d756c"></a>

## `mul`

```cinch
mul :: (Complex -> (Complex -> Complex))
```

(a + bi)(c + di) = (ac - bd) + (ad + bc)i (the imaginary unit squares
to -1, which is the `mul (complex 0 1) (complex 0 1) == complex -1 0`
identity above).

[Source](/reference-sources/Data.Complex.cinch) · line 115

<a id="value-6e6567"></a>

## `neg`

```cinch
neg :: (Complex -> Complex)
```

The additive inverse, componentwise.

[Source](/reference-sources/Data.Complex.cinch) · line 137

<a id="value-7265616c2d70617274"></a>

## `real-part`

```cinch
real-part :: (Complex -> F64)
```

[Source](/reference-sources/Data.Complex.cinch) · line 90

<a id="value-737562"></a>

## `sub`

```cinch
sub :: (Complex -> (Complex -> Complex))
```

(a + bi) - (c + di): componentwise.

[Source](/reference-sources/Data.Complex.cinch) · line 108

<a id="value-746f2d737472696e67"></a>

## `to-string`

```cinch
to-string :: (Complex -> String)
```

The `a+bi` form: a negative imaginary part renders with `-` and its
magnitude (so `complex 1 -1` is `1-1i`), a non-negative one with `+`.

[Source](/reference-sources/Data.Complex.cinch) · line 257

# Instances

These are checked exported instance entries. Law/sample-check status is separate from the existence of an instance; see [laws and optimization](../reference/12-rewrites-and-optimization.md).

| Instance | Target | Resolution |
| --- | --- | --- |
| `Data.Complex.Add_Complex` | `Add Complex` | Implicit |
| `Data.Complex.Eq_Complex` | `Eq Complex` | Implicit |
| `Data.Complex.Mul_Complex` | `Mul Complex` | Implicit |
| `Data.Complex.Neg_Complex` | `Neg Complex` | Implicit |
| `Data.Complex.Sub_Complex` | `Sub Complex` | Implicit |

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Data/Complex.cinch`. Source SHA-256: `16b1436d6025030f3ab3dbfd6955fb926883ead5dc83af6a373ac42919918a51`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
