# Text.Builder

An efficient text-construction vocabulary for assembling many fragments before rendering.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Text.Builder`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.Builder.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bytes](Data.Bytes.md), [Data.Nat](Data.Nat.md), [Data.String](Data.String.md), [Text.Show](Text.Show.md).

Non-embedded source closure: [Text.Builder](Text.Builder.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Text.Builder: an append-cheap text and byte builder (the codegen rung's
accumulator).

The ladder's IR rung renders LLVM IR as TEXT with `string-concat`, which
copies its LEFT operand, so emitting a module piece by piece is quadratic in
the number of pieces: survivable for a toy function, hostile for the whole
prelude's 55 modules, and the same shape would bite the manifest writer and
every diagnostic renderer. This module is the efficient accumulator the text
homes did not have.

The representation is a SPINE of pending pieces (the `Bootstrap.Ast`
`*-spine` convention, D387's answer to a data type that cannot nest a `List`
of itself): each append allocates one cell and RETAINS its argument, so an
append of a string or a byte slice copies NOTHING and runs in constant time,
independent of the piece's length. The spine holds the pieces most-recent
first; the flatten lays them out in APPEND order by filling the output
backward from its end. `builder-run`/`builder-bytes` measure the whole spine
once (`builder-size`), allocate ONE output buffer, and copy each piece
exactly once: O(total bytes) time and one allocation, never quadratic.

`builder-size` is EXACT (the sum of each piece's byte length, computed
without flattening) and O(number of pieces). `builder-chunks` counts the
pending pieces, one per append (an append of a line is still one piece), so
a caller can assert that appends do not re-copy.

`builder-add-nat`/`builder-add-int` render through `Text.Show`'s
`nat-to-string`/`int-to-string` and append the resulting `String` as one
piece. `builder-add-line` appends the text and a single LF as one piece, so
the line semantics are the builder's, not a caller's.

`builder-run` decodes the flattened UTF-8. A builder built only through the
string, line, nat and int surface is always valid UTF-8, so the decode never
fails there; a raw byte chunk that is not valid UTF-8 (only
`builder-add-bytes` can insert one) still flattens exactly through
`builder-bytes`, and `builder-run` then yields the empty string. That is the
defined total fallback, the same shape `Text.Show`'s `wrap-bytes` uses.

Module scheme (design/base-library.md): `Text.*` is conversion to and from
String/Bytes, so the builder lives here.

</details>

# Types and constructors

<a id="type-4275696c646572"></a>

## `Builder`

```cinch
data Builder
```

A pending piece, each a retained argument with no copy made at append
time. `BEnd` is the spine's end marker; `BLine` carries the text before
its newline; `BBytes` carries a raw byte slice (which need not be UTF-8).

[Source](/reference-sources/Text.Builder.cinch) · line 53

# Values

<a id="value-6275696c6465722d6164642d6279746573"></a>

## `builder-add-bytes`

```cinch
builder-add-bytes :: (Bytes -> (Builder -> Builder))
```

Append a raw byte slice as one piece: O(1), no copy of the slice.

[Source](/reference-sources/Text.Builder.cinch) · line 78

<a id="value-6275696c6465722d6164642d696e74"></a>

## `builder-add-int`

```cinch
builder-add-int :: (Int -> (Builder -> Builder))
```

Append a decimal `Int` (rendered by `Text.Show`).

[Source](/reference-sources/Text.Builder.cinch) · line 90

<a id="value-6275696c6465722d6164642d6c696e65"></a>

## `builder-add-line`

```cinch
builder-add-line :: (String -> (Builder -> Builder))
```

Append the text and a single LF as one piece.

[Source](/reference-sources/Text.Builder.cinch) · line 82

<a id="value-6275696c6465722d6164642d6e6174"></a>

## `builder-add-nat`

```cinch
builder-add-nat :: (Nat -> (Builder -> Builder))
```

Append a decimal `Nat` (rendered by `Text.Show`).

[Source](/reference-sources/Text.Builder.cinch) · line 86

<a id="value-6275696c6465722d6164642d737472696e67"></a>

## `builder-add-string`

```cinch
builder-add-string :: (String -> (Builder -> Builder))
```

Append a string as one piece: O(1), no copy of the string's bytes.

[Source](/reference-sources/Text.Builder.cinch) · line 74

<a id="value-6275696c6465722d6279746573"></a>

## `builder-bytes`

```cinch
builder-bytes :: (Builder -> Bytes)
```

The flattened bytes: one allocation, one copy pass.

[Source](/reference-sources/Text.Builder.cinch) · line 139

<a id="value-6275696c6465722d6368756e6b73"></a>

## `builder-chunks`

```cinch
builder-chunks :: (Builder -> Nat)
```

How many pieces are pending: one per append, so a test can assert that
appends do not re-copy. O(number of pieces).

[Source](/reference-sources/Text.Builder.cinch) · line 95

<a id="value-6275696c6465722d69732d656d707479"></a>

## `builder-is-empty`

```cinch
builder-is-empty :: (Builder -> Bool)
```

Whether a builder has no pending pieces. Structural: a builder is empty
exactly when it was never appended to, so an appended empty string counts
as a piece.

[Source](/reference-sources/Text.Builder.cinch) · line 66

<a id="value-6275696c6465722d6e6577"></a>

## `builder-new`

```cinch
builder-new :: Builder
```

The empty builder (no pending pieces).

[Source](/reference-sources/Text.Builder.cinch) · line 60

<a id="value-6275696c6465722d72756e"></a>

## `builder-run`

```cinch
builder-run :: (Builder -> String)
```

The flattened string: flatten once, then decode the UTF-8. A builder
built through the string, line, nat and int surface is always valid UTF-8;
a raw byte chunk that is not yields the empty string (see the header).

[Source](/reference-sources/Text.Builder.cinch) · line 145

<a id="value-6275696c6465722d73697a65"></a>

## `builder-size`

```cinch
builder-size :: (Builder -> Nat)
```

The exact total byte length of the flattened result, without flattening
(a line piece counts its LF). O(number of pieces).

[Source](/reference-sources/Text.Builder.cinch) · line 104

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Text/Builder.cinch`. Source SHA-256: `212242bd8c516223446f1023b982c053f80dfed0e591c7af4f6e0a3d027a05ab`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
