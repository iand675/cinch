# Text.Scan

Scanning primitives over input text and the shared position/error vocabulary.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Text.Scan`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.Scan.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Char](Data.Char.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Text.Show](Text.Show.md).

Non-embedded source closure: [Text.Scan](Text.Scan.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Text.Scan: the literal-string scanning and formatting utilities
(the Ruby String surface, redesigned where our principles say so).

SCANNING (the subject-last argument order throughout: the needle,
separator or pattern names what happens, the subject is last):
  `string-index-of` / `string-contains` / `string-starts-with` /
  `string-ends-with` over scalar indices; `string-char-index-of`
  for a single scalar; `string-split-on` (an empty separator splits
  into single scalars, Ruby's `split("")`); `string-join`;
  `string-lines` (splits at LF; a single trailing empty line is
  dropped) and its inverse `string-unlines` (each line followed by
  LF); `string-words` (ASCII whitespace runs) and its inverse
  `string-unwords` (single-space join); `string-take` /
  `string-drop` / `string-split-at` cutting at a scalar count, and
  `string-take-while` / `string-drop-while` cutting at a scalar
  predicate. Prefix and suffix tests ARE `string-starts-with` /
  `string-ends-with` (no separate `is-prefix-of` spelling).

FORMATTING: `format` is the template interpolation (`{}` consumes
  the next argument in order, `{{` and `}}` are literal braces;
  this replaces sprintf-style `%` formatting on purpose: the
  arguments' types do the rendering through `Text.Show`).
  `string-pad-left` / `string-pad-right` / `string-center` pad to
  a scalar width with spaces; `string-strip` / `string-lstrip` /
  `string-rstrip` / `string-chomp` / `string-chop` trim; the case
  family maps ASCII letters only. The regex-driven scanning family
  (scan, split, replace) rides the engine in `Text.Regex`.

UNICODE SCOPE, named honestly at the UTF-8 layers' boundary:
every operation here is scalar-value aware (indices are scalar
indices, lengths are scalar counts, slices cut at scalar
boundaries). What is NOT here: canonical normalization
(NFC/NFD/NFKC/NFKD) and grapheme clusters (the UTF-8 machinery
carries no Unicode character database), Unicode or locale-aware
case mapping (`string-upcase-ascii` and friends map A-Z/a-z only
and leave every other scalar untouched), and non-ASCII whitespace
classes. `string-normalize-newlines` is the one normalization
provided: CRLF and lone CR line endings fold to LF.

</details>

# Types and constructors

<a id="type-466f726d61744572726f72"></a>

## `FormatError`

```cinch
data FormatError
```

`format`: template interpolation over `{}` holes (the sprintf
replacement; the arguments render themselves at their own
types).

[Source](/reference-sources/Text.Scan.cinch) · line 401

```cinch
MissingFormatArgument :: (Nat -> FormatError)
```

```cinch
UnusedFormatArgument :: (Nat -> FormatError)
```

```cinch
UnmatchedFormatBrace :: (Nat -> FormatError)
```

# Values

<a id="value-666f726d6174"></a>

## `format`

```cinch
format :: (String -> (List String -> Result FormatError String))
```

[Source](/reference-sources/Text.Scan.cinch) · line 412

<a id="value-72656e6465722d666f726d61742d6572726f72"></a>

## `render-format-error`

```cinch
render-format-error :: (FormatError -> String)
```

[Source](/reference-sources/Text.Scan.cinch) · line 406

<a id="value-737472696e672d6361706974616c697a652d6173636969"></a>

## `string-capitalize-ascii`

```cinch
string-capitalize-ascii :: (String -> String)
```

[Source](/reference-sources/Text.Scan.cinch) · line 336

<a id="value-737472696e672d63656e746572"></a>

## `string-center`

```cinch
string-center :: (Nat -> (String -> String))
```

Center in `w` scalars; the extra space of an odd pad goes right.

[Source](/reference-sources/Text.Scan.cinch) · line 362

<a id="value-737472696e672d636861722d696e6465782d6f66"></a>

## `string-char-index-of`

```cinch
string-char-index-of :: (Char -> (String -> Option Nat))
```

The scalar index of the first occurrence of a single scalar (the
one-scalar special case of `string-index-of`; `None` when the
scalar is absent, so it never coincides with the empty-needle
rule above).

[Source](/reference-sources/Text.Scan.cinch) · line 154

<a id="value-737472696e672d63686f6d70"></a>

## `string-chomp`

```cinch
string-chomp :: (String -> String)
```

Drop one trailing line ending: LF, CRLF or CR.

[Source](/reference-sources/Text.Scan.cinch) · line 309

<a id="value-737472696e672d63686f70"></a>

## `string-chop`

```cinch
string-chop :: (String -> String)
```

Drop the last scalar.

[Source](/reference-sources/Text.Scan.cinch) · line 323

<a id="value-737472696e672d636f6e7461696e73"></a>

## `string-contains`

```cinch
string-contains :: (String -> (String -> Bool))
```

[Source](/reference-sources/Text.Scan.cinch) · line 145

<a id="value-737472696e672d646f776e636173652d6173636969"></a>

## `string-downcase-ascii`

```cinch
string-downcase-ascii :: (String -> String)
```

[Source](/reference-sources/Text.Scan.cinch) · line 333

<a id="value-737472696e672d64726f70"></a>

## `string-drop`

```cinch
string-drop :: (Nat -> (String -> String))
```

[Source](/reference-sources/Text.Scan.cinch) · line 268

<a id="value-737472696e672d64726f702d7768696c65"></a>

## `string-drop-while`

```cinch
string-drop-while :: ((Char -> Bool) -> (String -> String))
```

[Source](/reference-sources/Text.Scan.cinch) · line 277

<a id="value-737472696e672d656e64732d77697468"></a>

## `string-ends-with`

```cinch
string-ends-with :: (String -> (String -> Bool))
```

[Source](/reference-sources/Text.Scan.cinch) · line 165

<a id="value-737472696e672d696e6465782d6f66"></a>

## `string-index-of`

```cinch
string-index-of :: (String -> (String -> Option Nat))
```

Scanning (subject last).

The scalar index where `needle` first occurs in `s` (an empty
needle occurs at 0).

[Source](/reference-sources/Text.Scan.cinch) · line 135

<a id="value-737472696e672d6a6f696e"></a>

## `string-join`

```cinch
string-join :: (String -> (List String -> String))
```

Join with a separator between every pair.

[Source](/reference-sources/Text.Scan.cinch) · line 204

<a id="value-737472696e672d6c696e6573"></a>

## `string-lines`

```cinch
string-lines :: (String -> List String)
```

The lines of a string: the segments between LF scalars, with one
trailing empty segment dropped when the string ends in LF (an
empty string has no lines).

[Source](/reference-sources/Text.Scan.cinch) · line 217

<a id="value-737472696e672d6c7374726970"></a>

## `string-lstrip`

```cinch
string-lstrip :: (String -> String)
```

[Source](/reference-sources/Text.Scan.cinch) · line 297

<a id="value-737472696e672d6e6f726d616c697a652d6e65776c696e6573"></a>

## `string-normalize-newlines`

```cinch
string-normalize-newlines :: (String -> String)
```

Line-ending normalization (the one normalization the UTF-8
layers permit: CRLF and lone CR fold to LF).

[Source](/reference-sources/Text.Scan.cinch) · line 378

<a id="value-737472696e672d7061642d6c656674"></a>

## `string-pad-left`

```cinch
string-pad-left :: (Nat -> (String -> String))
```

Padding (scalar widths, spaces).

[Source](/reference-sources/Text.Scan.cinch) · line 351

<a id="value-737472696e672d7061642d7269676874"></a>

## `string-pad-right`

```cinch
string-pad-right :: (Nat -> (String -> String))
```

[Source](/reference-sources/Text.Scan.cinch) · line 356

<a id="value-737472696e672d727374726970"></a>

## `string-rstrip`

```cinch
string-rstrip :: (String -> String)
```

[Source](/reference-sources/Text.Scan.cinch) · line 300

<a id="value-737472696e672d73706c69742d6174"></a>

## `string-split-at`

```cinch
string-split-at :: (Nat -> (String -> (String, String)))
```

Splitting at a scalar count, over scalar boundaries (a count past
the end puts everything in the first part; 0 puts everything in
the second).

[Source](/reference-sources/Text.Scan.cinch) · line 262

<a id="value-737472696e672d73706c69742d6f6e"></a>

## `string-split-on`

```cinch
string-split-on :: (String -> (String -> List String))
```

Split at every occurrence of the separator (the separators are
removed). Trailing empty pieces are dropped (Ruby's default) and
an empty separator splits into single-scalar strings (Ruby's
`split("")`).

[Source](/reference-sources/Text.Scan.cinch) · line 172

<a id="value-737472696e672d7374617274732d77697468"></a>

## `string-starts-with`

```cinch
string-starts-with :: (String -> (String -> Bool))
```

[Source](/reference-sources/Text.Scan.cinch) · line 162

<a id="value-737472696e672d7374726970"></a>

## `string-strip`

```cinch
string-strip :: (String -> String)
```

Trimming and case (ASCII).

[Source](/reference-sources/Text.Scan.cinch) · line 294

<a id="value-737472696e672d74616b65"></a>

## `string-take`

```cinch
string-take :: (Nat -> (String -> String))
```

[Source](/reference-sources/Text.Scan.cinch) · line 265

<a id="value-737472696e672d74616b652d7768696c65"></a>

## `string-take-while`

```cinch
string-take-while :: ((Char -> Bool) -> (String -> String))
```

The longest prefix (suffix) whose scalars all satisfy `p`; unlike
`string-take`/`string-drop` the cut is predicate-driven, so the
caller supplies the scalar test.

[Source](/reference-sources/Text.Scan.cinch) · line 274

<a id="value-737472696e672d756e6c696e6573"></a>

## `string-unlines`

```cinch
string-unlines :: (List String -> String)
```

The inverse of `string-lines`: every line followed by LF, so
`string-unlines (string-lines s)` is `s` with exactly one trailing
LF (and the empty list is the empty string, matching
`string-lines ""`). It cannot be a bare join: a join would leave
the last line unterminated.

[Source](/reference-sources/Text.Scan.cinch) · line 246

<a id="value-737472696e672d756e776f726473"></a>

## `string-unwords`

```cinch
string-unwords :: (List String -> String)
```

The inverse of `string-words`: a single space between elements
(the empty list is the empty string, matching `string-words ""`).

[Source](/reference-sources/Text.Scan.cinch) · line 256

<a id="value-737472696e672d7570636173652d6173636969"></a>

## `string-upcase-ascii`

```cinch
string-upcase-ascii :: (String -> String)
```

ASCII case mapping only (the honest boundary: no Unicode case
data exists in the UTF-8 layers).

[Source](/reference-sources/Text.Scan.cinch) · line 330

<a id="value-737472696e672d776f726473"></a>

## `string-words`

```cinch
string-words :: (String -> List String)
```

The whitespace-separated words (ASCII whitespace runs delimit).

[Source](/reference-sources/Text.Scan.cinch) · line 229

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Text/Scan.cinch`. Source SHA-256: `bc12b7a6cb5fe9a9b448047f9aeeb0fc3eac1e29927e2270e7be760014a93208`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
