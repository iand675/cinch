# Data.String

Immutable UTF-8 text, encoding/decoding, character access, slicing, comparison, and concatenation. Byte conversions copy to preserve immutability.

<span class="support-tag support-tag--current">Embedded</span> Available from the compiler's embedded base family.

Import with `import Data.String`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.String.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Algebra.Arithmetic](Algebra.Arithmetic.md), [Algebra.Monoid](Algebra.Monoid.md), [Algebra.Semigroup](Algebra.Semigroup.md), [Data.Bytes](Data.Bytes.md), [Data.Char](Data.Char.md), [Data.Hash](Data.Hash.md), [Data.List](Data.List.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Data.String: runtime strings, UTF-8 over Bytes.

`String` is the runtime text type (decision D88). It is distinct
from the staging interpreter's compile-time string values, which
are the same type observed at compile time (D43's deferral
resolved). A `"..."` literal in a term is a runtime `String`: the
lexer's decoded bytes in one shared read-only data-section constant
(immutability makes the sharing semantics-transparent: every
evaluation references the same object and nothing allocates).

THE IMMUTABILITY CONTRACT: a `String` never changes and no API ever
exposes its backing mutably. Conversions copy on the way in
(`string-from-bytes`) and on the way out (`string-to-bytes`);
mutation exists only on plain `Bytes`. The representation
constructor and the backing reader are module-private, so once a
buffer is wrapped no alias can mutate it, and a literal's backing
(the read-only data-section constant itself) never reaches a write:
outside this module it cannot be named, and in it every use of a
backing is a read or the source of a copy.

The functions here are the REFERENCE SEMANTICS for UTF-8. The
runtime provides SIMD-accelerated implementations of the same rules
(`cinch_utf8_*` in `compiler/runtime/src/string.rs`), tested to agree
with this code; the acceleration is a representation detail behind
the primitive operations chapter's justified bodies (D89).

Module scheme (design/base-library.md): the type's home carries its
instance family (the parked-instance deviation of D159 is over).

</details>

# Types and constructors

<a id="type-537472696e67"></a>

## `String`

```cinch
data String
```

Runtime strings: UTF-8 over Bytes.

`String` is the runtime text type (decision D88). It is distinct
from the staging interpreter's compile-time string values, which are
the same type observed at compile time (D43's deferral resolved).
A `"..."` literal in a term is a runtime `String`: the lexer's
decoded bytes in one shared read-only data-section constant (the
sharing is semantics-transparent: every evaluation references the
same object and nothing allocates).

THE IMMUTABILITY CONTRACT (the rules below implement it): a
`String` never changes and no API ever exposes its backing mutably.
Conversions copy on the way in (`string-from-bytes`) and on the way
out (`string-to-bytes`); mutation exists only on plain `Bytes`.

The functions here are the REFERENCE SEMANTICS for UTF-8. The
runtime provides SIMD-accelerated implementations of the same rules
(`cinch_utf8_*` in `compiler/runtime/src/string.rs`), tested to agree
with this code; the acceleration is a representation detail behind
the primitive operations chapter's justified bodies (D89).

The representation constructor: module-private (the immutability
contract: no API hands out a string's backing). A `StrBuf` wraps a
buffer that is either fresh (`bytes-new` filled here) or a copy,
never one a caller retains.

[Source](/reference-sources/Data.String.cinch) · line 70

# Values

<a id="value-636f6d706172652d62797465732d66726f6d"></a>

## `compare-bytes-from`

```cinch
compare-bytes-from :: (Bytes -> (Bytes -> (Nat -> I64)))
```

Byte-wise comparison; UTF-8 preserves scalar value order under
byte-wise lexicographic order, so this is also code point order.

[Source](/reference-sources/Data.String.cinch) · line 342

<a id="value-636f70792d62797465732d696e746f"></a>

## `copy-bytes-into`

```cinch
copy-bytes-into :: using ((arg2 + arg4) <= (Data.Bytes.bytes-len arg0), (arg3 + arg4) <= (Data.Bytes.bytes-len arg1)) => ((arg0 :: Bytes) -> ((arg1 :: Bytes) -> ((arg2 :: Nat) -> ((arg3 :: Nat) -> ((arg4 :: Nat) -> Unit)))))
```

Concatenation (O(n + m) copies). The range copy is the runtime's
`memmove` (`bytes-copy-range`, with its two range obligations): a
loop here boxed a `Nat` for every index step.

[Source](/reference-sources/Data.String.cinch) · line 233

<a id="value-656d7074792d737472696e67"></a>

## `empty-string`

```cinch
empty-string :: String
```

[Source](/reference-sources/Data.String.cinch) · line 166

<a id="value-7061636b"></a>

## `pack`

```cinch
pack :: (List Char -> String)
```

Haskell's `Data.String` pair: `pack` is the `List Char` view of a
string, `unpack` its inverse (new conversions: no `String`/`List
Char` pair existed before). `pack` constructs a fresh string (the
wrap/copy discipline: the buffer is fresh and never aliased).

[Source](/reference-sources/Data.String.cinch) · line 312

<a id="value-737472696e672d627974652d696e6465782d66726f6d"></a>

## `string-byte-index-from`

```cinch
string-byte-index-from :: using (arg1 <= (Data.Bytes.bytes-len arg0)) => ((arg0 :: Bytes) -> ((arg1 :: Nat) -> ((arg2 :: Nat) -> refined ((arg1 <= r && r <= (Data.Bytes.bytes-len arg0))) Nat)))
```

Byte offset of code point `n` from offset `i`; the byte length when
`n` reaches past the end. The offset never moves backwards and never
passes the end, and the result says so. A sequence whose width runs
past the end is malformed, as `utf8-at` decides; the step restates
that end test, so it answers `i` as a malformed sequence does.

[Source](/reference-sources/Data.String.cinch) · line 286

<a id="value-737472696e672d627974652d6c656e677468"></a>

## `string-byte-length`

```cinch
string-byte-length :: (String -> Nat)
```

[Source](/reference-sources/Data.String.cinch) · line 169

<a id="value-737472696e672d636861722d6174"></a>

## `string-char-at`

```cinch
string-char-at :: (String -> (Nat -> Option Char))
```

[Source](/reference-sources/Data.String.cinch) · line 193

<a id="value-737472696e672d636861722d61742d66726f6d"></a>

## `string-char-at-from`

```cinch
string-char-at-from :: (Bytes -> (Nat -> (Nat -> Option Char)))
```

The scalar value at code point index `n`, by scan (O(n)).

[Source](/reference-sources/Data.String.cinch) · line 185

<a id="value-737472696e672d636f6d70617265"></a>

## `string-compare`

```cinch
string-compare :: (String -> (String -> I64))
```

Compares the two backings in place: `string-to-bytes` would copy
both operands first, and the lexer's keyword table compares every
identifier against dozens of literals.

[Source](/reference-sources/Data.String.cinch) · line 354

<a id="value-737472696e672d636f6e636174"></a>

## `string-concat`

```cinch
string-concat :: (String -> (String -> String))
```

[Source](/reference-sources/Data.String.cinch) · line 236

<a id="value-737472696e672d636f6e6361742d616c6c"></a>

## `string-concat-all`

```cinch
string-concat-all :: (List String -> String)
```

Every string in the list, in order, in one pass: the buffer is made
once, at the end of the list, sized to the pieces' total, and each
piece is copied once on the way back. A right fold of
`string-concat` would re-copy the whole built suffix at every step,
quadratic in the result's length. The concatenation of valid UTF-8
is valid UTF-8, so the buffer wraps without a decode. This is the
`Monoid String` instance's `mconcat`.

[Source](/reference-sources/Data.String.cinch) · line 252

<a id="value-737472696e672d66726f6d2d6279746573"></a>

## `string-from-bytes`

```cinch
string-from-bytes :: (Bytes -> Option String)
```

[Source](/reference-sources/Data.String.cinch) · line 160

<a id="value-737472696e672d6c656e677468"></a>

## `string-length`

```cinch
string-length :: (String -> Nat)
```

[Source](/reference-sources/Data.String.cinch) · line 181

<a id="value-737472696e672d6c656e6774682d66726f6d"></a>

## `string-length-from`

```cinch
string-length-from :: (Bytes -> (Nat -> (Nat -> Nat)))
```

Length in Unicode scalar values (O(n) scan).

[Source](/reference-sources/Data.String.cinch) · line 173

<a id="value-737472696e672d726570656174"></a>

## `string-repeat`

```cinch
string-repeat :: (Nat -> (String -> String))
```

`s` repeated `n` times (the run-building helper the padding and
numeric-format sites spelled out as fixed byte runs): one buffer of
`n` copies, made once and filled in place.

[Source](/reference-sources/Data.String.cinch) · line 269

<a id="value-737472696e672d736c696365"></a>

## `string-slice`

```cinch
string-slice :: (String -> (Nat -> (Nat -> String)))
```

The code point range [start, end); indices past the end clamp and a
reversed range yields the empty string (the span's subtraction
takes its `start < end` evidence from the test).

[Source](/reference-sources/Data.String.cinch) · line 298

<a id="value-737472696e672d746f2d6279746573"></a>

## `string-to-bytes`

```cinch
string-to-bytes :: (String -> Bytes)
```

A copy of the backing (the conversion out copies: the caller may
mutate its copy freely and the `String` never changes).

[Source](/reference-sources/Data.String.cinch) · line 89

<a id="value-756e7061636b"></a>

## `unpack`

```cinch
unpack :: (String -> List Char)
```

[Source](/reference-sources/Data.String.cinch) · line 326

<a id="value-757466382d6174"></a>

## `utf8-at`

```cinch
utf8-at :: (Bytes -> (Nat -> Option (Char, Nat)))
```

[Source](/reference-sources/Data.String.cinch) · line 133

<a id="value-757466382d63686172"></a>

## `utf8-char`

```cinch
utf8-char :: (Nat -> (Nat -> Option (Char, Nat)))
```

[Source](/reference-sources/Data.String.cinch) · line 146

<a id="value-757466382d636f6e74"></a>

## `utf8-cont`

```cinch
utf8-cont :: using ((arg1 + arg2) <= (Data.Bytes.bytes-len arg0)) => ((arg0 :: Bytes) -> ((arg1 :: Nat) -> ((arg2 :: Nat) -> ((arg3 :: Nat) -> Option Nat))))
```

The `k` continuation bytes from `i`: the caller has checked that
they lie inside the buffer (`i + k <= bytes-len b`), so each read
is in bounds and the obligation steps with the index.

[Source](/reference-sources/Data.String.cinch) · line 114

<a id="value-757466382d656e636f64652d696e746f"></a>

## `utf8-encode-into`

```cinch
utf8-encode-into :: using ((arg1 + (Data.String.utf8-encode-width arg2)) <= (Data.Bytes.bytes-len arg0)) => ((arg0 :: Bytes) -> ((arg1 :: Nat) -> ((arg2 :: Nat) -> Nat)))
```

Encode one scalar value at `pos` (bytes are raw values; division is
the `nat-div`/`nat-rem` function family). The caller proves the
encoding fits (`pos + utf8-encode-width cp <= bytes-len b`, a
buffer sized with `utf8-encode-width`), and the branches test that
same width, so each one knows how many bytes it may write. Every
byte is taken modulo its field width, which is the same number for
every value its width admits (`cp < 128` in the first branch, `cp /
64 < 32` in the second, `cp / 4096 < 16` in the third, and `cp <
2^22` in the last, past which the unmasked lead byte did not fit a
byte at all).

[Source](/reference-sources/Data.String.cinch) · line 213

<a id="value-757466382d656e636f64652d7769647468"></a>

## `utf8-encode-width`

```cinch
utf8-encode-width :: (Nat -> InRange 1 4 Nat)
```

[Source](/reference-sources/Data.String.cinch) · line 196

<a id="value-757466382d6c6561642d7061796c6f6164"></a>

## `utf8-lead-payload`

```cinch
utf8-lead-payload :: (Nat -> (Nat -> Nat))
```

The lead byte's payload bits: `x - 192`, `x - 224`, `x - 240` for
the lead byte of a two-, three- and four-byte sequence, written as
the low bits, which is the same number for every lead byte
`utf8-width` admits and needs no `x >= 192` evidence.

[Source](/reference-sources/Data.String.cinch) · line 127

<a id="value-757466382d6d696e"></a>

## `utf8-min`

```cinch
utf8-min :: (Nat -> Nat)
```

[Source](/reference-sources/Data.String.cinch) · line 104

<a id="value-757466382d76616c69642d66726f6d"></a>

## `utf8-valid-from`

```cinch
utf8-valid-from :: (Bytes -> (Nat -> Bool))
```

[Source](/reference-sources/Data.String.cinch) · line 152

<a id="value-757466382d7769647468"></a>

## `utf8-width`

```cinch
utf8-width :: (Nat -> Nat)
```

UTF-8 decoding rules: a lead byte gives the width; continuation
bytes are 10xxxxxx; overlong encodings, surrogates, and values above
0x10FFFF are malformed.

[Source](/reference-sources/Data.String.cinch) · line 95

# Instances

These are checked exported instance entries. Law/sample-check status is separate from the existence of an instance; see [laws and optimization](../reference/12-rewrites-and-optimization.md).

| Instance | Target | Resolution |
| --- | --- | --- |
| `Data.String.Eq_String` | `Eq String` | Implicit |
| `Data.String.Hash_String` | `Hash String` | Implicit |
| `Data.String.Monoid_String` | `Monoid String` | Implicit |
| `Data.String.Ord_String` | `Ord String` | Implicit |
| `Data.String.Semigroup_String` | `Semigroup String` | Implicit |

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Data/String.cinch`. Source SHA-256: `eb33baef6f92bc39b36b9db2aac139073f131ae306237970781311ee57877ebb`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
