# Data.Char

Unicode scalar values and checked character/code conversions, with range information for safe use.

<span class="support-tag support-tag--current">Embedded</span> Available from the compiler's embedded base family.

Import with `import Data.Char`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Char.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Algebra.Arithmetic](Algebra.Arithmetic.md), [Data.Hash](Data.Hash.md), [Data.Option](Data.Option.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Data.Char: Unicode scalar values.

`Char` is a core runtime type (primitive types specification
section 1): one scalar value. `u32-to-char` rejects values outside
the scalar range (surrogates and above 0x10FFFF) as `None`.

Module scheme (design/base-library.md): the type's home carries its
instance family (the parked-instance deviation of D159 is over).

</details>

# Types and constructors

This module exports no type declarations.

# Values

<a id="value-636861722d636f6465"></a>

## `char-code`

```cinch
char-code :: (Char -> InRange 0 1114111 Nat)
```

The scalar value as a `Nat`: the conversion the text and data
homes all spelled out by hand before this was its home. A scalar
value is at most 0x10FFFF, and the result says so.

[Source](/reference-sources/Data.Char.cinch) · line 37

<a id="value-636861722d66726f6d2d636f6465"></a>

## `char-from-code`

```cinch
char-from-code :: (Nat -> Option Char)
```

The inverse, partial exactly as `u32-to-char` is: a value outside
the scalar range (a surrogate, or above 0x10FFFF) is `None`. The
range test comes first, so a value past `U32` is `None` rather than
a wrapped scalar.

[Source](/reference-sources/Data.Char.cinch) · line 44

<a id="value-636861722d746f2d753332"></a>

## `char-to-u32`

```cinch
char-to-u32 :: ((a0 :: Char) -> refined (((0 <= r && r <= 1114111) && r == a0)) U32)
```

Primitive types specification, section 6: Char values are Unicode
scalar values, in code point order. `u32-to-char` rejects surrogates
and values above 0x10FFFF.

[Source](/reference-sources/Data.Char.cinch) · line 25

<a id="value-64696769742d76616c7565"></a>

## `digit-value`

```cinch
digit-value :: (Char -> Option Nat)
```

`Some 0..9` for the ASCII digits, `None` otherwise (the caller
has already decided what a non-digit means).

[Source](/reference-sources/Data.Char.cinch) · line 62

<a id="value-65712d63686172"></a>

## `eq-char`

```cinch
eq-char :: (Char -> (Char -> Bool))
```

[Source](/reference-sources/Data.Char.cinch) · line 27

<a id="value-67652d63686172"></a>

## `ge-char`

```cinch
ge-char :: (Char -> (Char -> Bool))
```

[Source](/reference-sources/Data.Char.cinch) · line 32

<a id="value-67742d63686172"></a>

## `gt-char`

```cinch
gt-char :: (Char -> (Char -> Bool))
```

[Source](/reference-sources/Data.Char.cinch) · line 31

<a id="value-6865782d64696769742d76616c7565"></a>

## `hex-digit-value`

```cinch
hex-digit-value :: (Char -> Option Nat)
```

`Some 0..15` for `0`-`9`, `a`-`f` and `A`-`F`; `None` otherwise.

[Source](/reference-sources/Data.Char.cinch) · line 68

<a id="value-69732d61736369692d616c706861"></a>

## `is-ascii-alpha`

```cinch
is-ascii-alpha :: (Char -> Bool)
```

[Source](/reference-sources/Data.Char.cinch) · line 102

<a id="value-69732d61736369692d616c7068612d6e756d65726963"></a>

## `is-ascii-alpha-numeric`

```cinch
is-ascii-alpha-numeric :: (Char -> Bool)
```

[Source](/reference-sources/Data.Char.cinch) · line 105

<a id="value-69732d61736369692d6469676974"></a>

## `is-ascii-digit`

```cinch
is-ascii-digit :: (Char -> Bool)
```

The ASCII digit and hex-digit test/value pairs. Every text and
data home that parses numeric text had written its own (under
`is-digit-char`, `digit-of`, `is-hex`, `hex-of`, ...), and this is
their one home. The names say ASCII because that is the whole
class they accept: there is no Unicode digit or decimal-digit
database here, and none is claimed.

[Source](/reference-sources/Data.Char.cinch) · line 55

<a id="value-69732d61736369692d6865782d6469676974"></a>

## `is-ascii-hex-digit`

```cinch
is-ascii-hex-digit :: (Char -> Bool)
```

[Source](/reference-sources/Data.Char.cinch) · line 76

<a id="value-69732d61736369692d6c6f776572"></a>

## `is-ascii-lower`

```cinch
is-ascii-lower :: (Char -> Bool)
```

[Source](/reference-sources/Data.Char.cinch) · line 97

<a id="value-69732d61736369692d7370616365"></a>

## `is-ascii-space`

```cinch
is-ascii-space :: (Char -> Bool)
```

ASCII whitespace: space, tab, LF, VT, FF, CR (the ASCII subset of
base's `isSpace`; the other three controls are the whitespace
classes that exist outside ASCII).

[Source](/reference-sources/Data.Char.cinch) · line 111

<a id="value-69732d61736369692d7570706572"></a>

## `is-ascii-upper`

```cinch
is-ascii-upper :: (Char -> Bool)
```

The ASCII character classes and the ASCII case fold, the whole
families every text home had spelled out by hand before this was
their home (`is-alpha-char`/`is-upper-char` in Text.Lex,
`is-ws-char` in Text.Scan, `ascii-up`/`ascii-down` there and in
Effect.Http). The names say ASCII because that is the whole class
they accept: there is no Unicode letter, whitespace, case or
digit database in the UTF-8 layers, and none is claimed (the
Unicode scope statement in Text.Scan's header is the house
position). `is-ascii-digit` above follows the same rule.

A-Z is `65..90`, `a`-`z` is `97..122` in code point order.

[Source](/reference-sources/Data.Char.cinch) · line 92

<a id="value-6c652d63686172"></a>

## `le-char`

```cinch
le-char :: (Char -> (Char -> Bool))
```

[Source](/reference-sources/Data.Char.cinch) · line 30

<a id="value-6c742d63686172"></a>

## `lt-char`

```cinch
lt-char :: (Char -> (Char -> Bool))
```

[Source](/reference-sources/Data.Char.cinch) · line 29

<a id="value-6e652d63686172"></a>

## `ne-char`

```cinch
ne-char :: (Char -> (Char -> Bool))
```

[Source](/reference-sources/Data.Char.cinch) · line 28

<a id="value-746f2d6c6f7765722d6173636969"></a>

## `to-lower-ascii`

```cinch
to-lower-ascii :: (Char -> Char)
```

[Source](/reference-sources/Data.Char.cinch) · line 128

<a id="value-746f2d75707065722d6173636969"></a>

## `to-upper-ascii`

```cinch
to-upper-ascii :: (Char -> Char)
```

The ASCII case fold. A scalar outside the affected range is
returned unchanged, which is the documented behaviour for every
non-ASCII scalar and every non-letter. `char-from-code` cannot
fail on a code an upper/lower test just admitted, so the `None`
arm returns the input unchanged (never a magic placeholder).

[Source](/reference-sources/Data.Char.cinch) · line 121

<a id="value-7533322d746f2d63686172"></a>

## `u32-to-char`

```cinch
u32-to-char :: (U32 -> Option Char)
```

[Source](/reference-sources/Data.Char.cinch) · line 26

# Instances

These are checked exported instance entries. Law/sample-check status is separate from the existence of an instance; see [laws and optimization](../reference/12-rewrites-and-optimization.md).

| Instance | Target | Resolution |
| --- | --- | --- |
| `Data.Char.Eq_Char` | `Eq Char` | Implicit |
| `Data.Char.Hash_Char` | `Hash Char` | Implicit |
| `Data.Char.Ord_Char` | `Ord Char` | Implicit |

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Data/Char.cinch`. Source SHA-256: `217d21faf8b93a9606f779e720aca8c012fdea1ecbc745a95298e96896d783fb`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
