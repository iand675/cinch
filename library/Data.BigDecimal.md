# Data.BigDecimal

Decimal arithmetic with explicit precision/scale and rounding/error behaviour in the exported API.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Data.BigDecimal`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.BigDecimal.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Char](Data.Char.md), [Data.Int](Data.Int.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Ordering](Data.Ordering.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Relation.Eq](Relation.Eq.md), [Relation.Ord](Relation.Ord.md), [Text.Show](Text.Show.md).

Non-embedded source closure: [Data.BigDecimal](Data.BigDecimal.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Data.BigDecimal: exact decimal arithmetic over the bignum `Int`.

A `BigDecimal` is an unscaled `Int` times ten to an `Int` exponent:
`BigDecimal u e` stands for `u * 10^e`. Every value this family
hands out is in CANONICAL FORM: `unscaled` is zero with exponent
zero, or it is not divisible by ten. The smart constructor
`big-decimal` is the only way in (the representation constructor is
not exported, so an unnormalised value cannot exist outside this
module), which makes the canonical form an invariant of the type and
not a convention its users must remember.

Design law shape. Canonical form makes field equality value
equality, and the laws below are what that buys:
  - `0.1` and `0.10` are the same value, because both normalise to
    `BigDecimal 1 (-1)`: `from-string`/`big-decimal` normalise what
    they read, so a decimal has exactly one spelling.
  - round trip: `from-string (to-string x)` has the same fields as
    `x`, and `to-string` never emits exponent notation.
  - arithmetic: `add`/`sub`/`mul` are EXACT (the result is the
    arithmetic on the represented values, with no rounding and no
    precision guess), and `div`/`round` are the only rounding
    operations, each naming its fractional-digit count and its
    rounding mode as arguments.
  - comparison: `cmp` is the order that `sub` induces, `cmp x y` is
    the sign of `sub x y`; `Eq`/`Ord` here are the class spellings
    of that same relation.

NAMED REFUSALS (boundaries this family deliberately does not cross):
  - No `NaN` and no `Infinity`. Ruby's `BigDecimal` carries both
    because its precision is ambient state (`BigDecimal.limit`) and
    a quotient can be asked for without one. Here every rounding
    operation takes its precision, so a quotient always exists at
    the caller's precision when the divisor is not zero; `div` by
    zero is `None` (division is total by construction, and a zero
    divisor has no quotient to name). An unrepresentable remainder is
    rounded to the requested precision, never widened to a float.
  - No implicit coercion to `F64` inside arithmetic. `to-f64` is the
    one conversion out and it renders the exact decimal and lets the
    runtime's correctly-rounded decimal reader parse it, so no
    arithmetic path can acquire a binary rounding error; there is no
    `BigDecimal`/`F64` mixed operation at all.
  - No precision or rounding mode taken from anywhere but the
    arguments: `div` and `round` name both (Ruby's `/` picks up
    `BigDecimal.limit`, its rounding picks up `BigDecimal.mode`),
    and no operand's precision is inferred from the other's.
  - No exponent notation in `to-string` (the plain decimal is the
    only spelling emitted; `to-f64` renders that same text) and no
    exponent parsing in `from-string`: the accepted literal is
    `[+-]? ( digits ( "." digits )? | "." digits )`. Anything else,
    trailing junk included, is `MalformedLiteral`, never a silent
    zero and never a truncated parse.
  - No rounding above the units place: `round` takes a `Nat`
    fractional-digit count, so Ruby's negative precision (round to
    tens and above) is outside the surface.

Home (design/base-library.md): `Data.*` holds concrete types, and
this is the exact decimal. `Rounding` and `BigDecimalError` are this
family's own vocabulary (a rounding policy and a literal error are
not a mathematical kind the scheme names), so they live here with
the type; the `Eq`/`Ord` instances live with their type, as the
scheme requires. Ported item: Ruby's `BigDecimal`
(design/stdlib-parity-ruby.md, "Numeric: `BigDecimal`
(arbitrary-precision decimal; money and measurement)") under the P2
discipline (types and operations, no numeric tower: D125 lists
numeric towers beyond `Semiring` as deliberately out).

</details>

# Types and constructors

<a id="type-426967446563696d616c"></a>

## `BigDecimal`

```cinch
data BigDecimal
```

The representation: unscaled times ten to the exponent. Not
exported (the canonical-form invariant above is the reason; D143
is the surface rule that makes the omission bind).

[Source](/reference-sources/Data.BigDecimal.cinch) · line 90

```cinch
BigDecimal :: (Int -> (Int -> BigDecimal))
```

The representation: unscaled times ten to the exponent. Not
exported (the canonical-form invariant above is the reason; D143
is the surface rule that makes the omission bind).

<a id="type-426967446563696d616c4572726f72"></a>

## `BigDecimalError`

```cinch
data BigDecimalError
```

The literal reader's only error. It carries the offending text so
the caller's diagnostic can name it; there is no representation
for "unparsed" that a value could be confused with.

[Source](/reference-sources/Data.BigDecimal.cinch) · line 109

```cinch
MalformedLiteral :: (String -> BigDecimalError)
```

<a id="type-526f756e64696e67"></a>

## `Rounding`

```cinch
data Rounding
```

The rounding modes (Ruby's `ROUND_*` constants, spelled as
constructors per the house naming): away from zero, toward zero,
tie away from zero, tie toward zero, tie to even, toward positive
infinity, toward negative infinity.

[Source](/reference-sources/Data.BigDecimal.cinch) · line 97

```cinch
RoundUp :: Rounding
```

```cinch
RoundDown :: Rounding
```

```cinch
RoundHalfUp :: Rounding
```

```cinch
RoundHalfDown :: Rounding
```

```cinch
RoundHalfEven :: Rounding
```

```cinch
RoundCeiling :: Rounding
```

```cinch
RoundFloor :: Rounding
```

# Values

<a id="value-616273"></a>

## `abs`

```cinch
abs :: (BigDecimal -> BigDecimal)
```

[Source](/reference-sources/Data.BigDecimal.cinch) · line 183

<a id="value-616464"></a>

## `add`

```cinch
add :: (BigDecimal -> (BigDecimal -> BigDecimal))
```

Exact addition: both operands are read at the smaller exponent
(the scaling is exact, a power of ten times a bignum), so every
digit of either operand survives.

[Source](/reference-sources/Data.BigDecimal.cinch) · line 158

<a id="value-6269672d646563696d616c"></a>

## `big-decimal`

```cinch
big-decimal :: (Int -> (Int -> BigDecimal))
```

Construction and observation.

The normalising smart constructor: the trailing zeros of the
unscaled part move into the exponent, so `big-decimal 10 (-2)`,
`big-decimal 1 (-1)` and `from-string "0.10"` are one value. Zero
is `BigDecimal 0 0` whatever exponent it arrived with.

[Source](/reference-sources/Data.BigDecimal.cinch) · line 125

<a id="value-636d70"></a>

## `cmp`

```cinch
cmp :: (BigDecimal -> (BigDecimal -> Ordering))
```

Comparison (the order `sub` induces).

[Source](/reference-sources/Data.BigDecimal.cinch) · line 191

<a id="value-646976"></a>

## `div`

```cinch
div :: (Nat -> (Rounding -> (BigDecimal -> (BigDecimal -> Option BigDecimal))))
```

Division and rounding (the rounding operations).

`p` fractional digits, rounded by `mode`, then normalised.

The precision is the fractional-digit count of the quotient
BEFORE normalisation, so `div 10 RoundHalfUp one three` is
`Some 0.3333333333` and `div 2 RoundHalfUp one eight` is `Some
0.13`. The magnitude is rounded (the mode names below read as
their names say from there) and the sign is re-applied, so
`RoundUp` is away from zero, `RoundCeiling` is toward positive
infinity, and `RoundHalfUp` breaks a tie away from zero. A zero
divisor is `None`: there is no result to return and no infinity
to spell.

[Source](/reference-sources/Data.BigDecimal.cinch) · line 269

<a id="value-6578706f6e656e74"></a>

## `exponent`

```cinch
exponent :: (BigDecimal -> Int)
```

[Source](/reference-sources/Data.BigDecimal.cinch) · line 138

<a id="value-66726f6d2d696e74"></a>

## `from-int`

```cinch
from-int :: (Int -> BigDecimal)
```

[Source](/reference-sources/Data.BigDecimal.cinch) · line 131

<a id="value-66726f6d2d737472696e67"></a>

## `from-string`

```cinch
from-string :: (String -> Result BigDecimalError BigDecimal)
```

The literal reader: `[+-]? ( digits ( "." digits )? | "." digits )`.
Every other shape (empty, a lone sign, a lone point, a trailing
point, trailing junk, an exponent part) is `MalformedLiteral`
carrying the text.

[Source](/reference-sources/Data.BigDecimal.cinch) · line 400

<a id="value-6d756c"></a>

## `mul`

```cinch
mul :: (BigDecimal -> (BigDecimal -> BigDecimal))
```

Exact multiplication: the unscaled parts multiply and the
exponents add, which is the representation's own product rule.

[Source](/reference-sources/Data.BigDecimal.cinch) · line 174

<a id="value-6e6567"></a>

## `neg`

```cinch
neg :: (BigDecimal -> BigDecimal)
```

[Source](/reference-sources/Data.BigDecimal.cinch) · line 179

<a id="value-72656e6465722d6269672d646563696d616c2d6572726f72"></a>

## `render-big-decimal-error`

```cinch
render-big-decimal-error :: (BigDecimalError -> String)
```

[Source](/reference-sources/Data.BigDecimal.cinch) · line 112

<a id="value-726f756e64"></a>

## `round`

```cinch
round :: (Nat -> (Rounding -> (BigDecimal -> BigDecimal)))
```

Rounding a value to `p` fractional digits: the division by one at
the same precision, written without a divisor test. With `k`
fractional digits to spare the value is exact; otherwise the
divisor is `10^j` and the quotient is `j` divisions by the numeral
ten (truncation toward zero composes), so no runtime zero test
stands between `round` and its result.

[Source](/reference-sources/Data.BigDecimal.cinch) · line 283

<a id="value-737562"></a>

## `sub`

```cinch
sub :: (BigDecimal -> (BigDecimal -> BigDecimal))
```

Subtraction is addition of the negation: the alignment above is
the only place exponents are reconciled, and a negated value is
still canonical, so the exactness argument is the same one.

[Source](/reference-sources/Data.BigDecimal.cinch) · line 169

<a id="value-746f2d663634"></a>

## `to-f64`

```cinch
to-f64 :: (BigDecimal -> F64)
```

The one conversion to a binary float: the exact decimal text,
read by the runtime's correctly-rounded decimal parser. Nothing
else in this module sees an `F64`.

[Source](/reference-sources/Data.BigDecimal.cinch) · line 393

<a id="value-746f2d737472696e67"></a>

## `to-string`

```cinch
to-string :: (BigDecimal -> String)
```

Text.

The plain decimal: no exponent notation, sign first, a leading
`0.` when the fraction is wider than the digit run.

[Source](/reference-sources/Data.BigDecimal.cinch) · line 367

<a id="value-756e7363616c6564"></a>

## `unscaled`

```cinch
unscaled :: (BigDecimal -> Int)
```

[Source](/reference-sources/Data.BigDecimal.cinch) · line 134

# Instances

These are checked exported instance entries. Law/sample-check status is separate from the existence of an instance; see [laws and optimization](../reference/12-rewrites-and-optimization.md).

| Instance | Target | Resolution |
| --- | --- | --- |
| `Data.BigDecimal.Eq_BigDecimal` | `Eq BigDecimal` | Implicit |
| `Data.BigDecimal.Ord_BigDecimal` | `Ord BigDecimal` | Implicit |

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Data/BigDecimal.cinch`. Source SHA-256: `0c7f5ae4c2f2c74b137f0f968f43abd45da709a984d7f635f028315dfa17023c`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
