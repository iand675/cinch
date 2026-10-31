# Text.Csv

CSV parsing/rendering with explicit field, record, and malformed-input handling.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Text.Csv`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.Csv.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bool](Data.Bool.md), [Data.Char](Data.Char.md), [Data.List](Data.List.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Text.Lex](Text.Lex.md), [Text.Parse](Text.Parse.md), [Text.Show](Text.Show.md).

Non-embedded source closure: [Text.Csv](Text.Csv.md), [Text.Lex](Text.Lex.md), [Text.Parse](Text.Parse.md), [Text.Scan](Text.Scan.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Text.Csv: RFC 4180 comma-separated text in, rows of fields out, and
back out again.

Module scheme (design/base-library.md): `Text.*` is conversion to and
from String/Bytes, so the Ruby `CSV` family (PORT in
design/stdlib-parity-ruby.md, the P37 serialization-breadth slice)
lands here as a text codec in the shape of `Text.Json`: a scanner over
the source plus a total printer, carrying the P30 toolkit's error
vocabulary (`Text.Lex`'s `Span`, `Text.Parse`'s `ParseError` and its
megaparsec-style rendering) so a CSV failure renders exactly as a
grammar failure does. A row is `List String`; a document is
`List (List String)`. The identifiers are dash style per D106.

WHY THE LEXED TOKEN STREAM IS NOT THE INPUT (reported): in CSV the
structure lives inside the tokens, not between them. A quoted field
may contain the delimiter, the row terminator and doubled quotation
marks, so `Text.Lex` would have to be taught the CSV rules before it
could tokenize one row (and its escape vocabulary is the wrong one for
a data file). The scanner here reads the source scalars directly and
keeps every rule of the dialect explicit, the same choice `Text.Json`
makes and for the same reason.

THE DIALECT (the whole of it): RFC 4180 with the row terminator
relaxed to LF or CRLF (RFC 4180 writes CRLF; the bare LF spelling is
accepted and `csv-print` writes LF, matching Ruby's default
`row_sep`). Fields are separated by one comma (44) and records by a
row terminator. A field may be quoted with quotation marks (34);
inside a quoted field a quotation mark stands for itself when
doubled, and the delimiter, CR and LF are literal scalars and are
never normalized (so `csv-parse` of `csv-print` is exact, unlike
Ruby's universal-newline translation of CRLF inside a quoted field).
Outside a quoted field only the delimiter, the terminator and the
quotation mark are special. An empty field is legal anywhere. Empty
text is a document with no rows; one terminator trailing the last row
does not open a further row, while a terminator between two rows does
(so "a\n\n" is the two rows `[a]` and `[""]`). A header is just the
first row: `csv-parse-with-headers` keys the remaining rows by it.

THE LAWS (what the tests hold this module to):
  round trip: `csv-parse (csv-print rows)` is `Ok rows` for every
    document whose rows each have at least one field;
  canonical fixed point: `csv-print` emits minimal quoting and LF
    terminators, so once printed a document is already canonical:
    `csv-print (rows-of (csv-parse (csv-print rows)))` is
    `csv-print rows` (print, parse and print again is the same text);
  total printing: every `List (List String)` prints, with no error
    case.

NAMED REFUSALS (each is a `CsvError` variant or a rule of the dialect
above, never a silent gap):
  `col_sep` (Ruby's delimiter option): the delimiter is the comma and
    is fixed; no other delimiter parses, so there is no "wrong
    delimiter" error to report;
  `converters` (Ruby's coercion of field text to Integer, Float,
    Date, ...): a field is `String` and nothing else; typing fields is
    the consumer's `Text.Codec` step (A6), not the scanner's;
  encoding handling: the input is a `String`, so it is already decoded
    scalars and there is no malformed-byte case to report. Ruby's
    transcoding has no counterpart here, a leading U+FEFF is field
    text rather than a byte-order mark (the module does not strip it),
    and a CRLF inside a quoted field stays CRLF;
  `liberal_parsing`: a quotation mark in an unquoted field
    (`CsvStrayQuote`) and text after a closing quotation mark
    (`CsvTextAfterQuote`) are refused, as Ruby's default is;
  a lone CR (`CsvBareCarriageReturn`): the terminator is LF or CRLF;
  ragged header keying (`CsvRowArity`): every row
    `csv-parse-with-headers` keys must have the header's field count
    (Ruby pads the short row with nil, which invents data);
  an unterminated quoted field (`CsvUnterminatedQuote`), reported at
    the quotation mark that opened it.

A zero-field row is outside the dialect, not a bug: RFC 4180's
`record = field *(COMMA field)` has no such production, and an empty
data row is one empty field. So the round-trip law above is stated
over rows with at least one field; `csv-print` of a zero-field row is
still total (it prints the terminator alone, which reads back as one
empty field).

</details>

# Types and constructors

<a id="type-4373764572726f72"></a>

## `CsvError`

```cinch
data CsvError
```

The error vocabulary: one named variant per refusal, each carrying
the `Span` it stands at, plus the bridge into `Text.Parse`'s
vocabulary so `render-bundle` renders it (the `Text.Json` shape).

[Source](/reference-sources/Text.Csv.cinch) · line 102

```cinch
CsvUnterminatedQuote :: (Span -> CsvError)
```

```cinch
CsvStrayQuote :: (Span -> CsvError)
```

```cinch
CsvTextAfterQuote :: (Span -> CsvError)
```

```cinch
CsvBareCarriageReturn :: (Span -> CsvError)
```

```cinch
CsvRowArity :: (Span -> (Int -> (Int -> CsvError)))
```

# Values

<a id="value-6373762d6572726f722d6d657373616765"></a>

## `csv-error-message`

```cinch
csv-error-message :: (CsvError -> String)
```

The diagnostic line for a variant: what was refused, and (for the
arity case) both counts, since the row alone does not say which
side moved.

[Source](/reference-sources/Text.Csv.cinch) · line 120

<a id="value-6373762d6572726f722d7370616e"></a>

## `csv-error-span`

```cinch
csv-error-span :: (CsvError -> Span)
```

[Source](/reference-sources/Text.Csv.cinch) · line 109

<a id="value-6373762d6572726f722d746f2d70617273652d6572726f72"></a>

## `csv-error-to-parse-error`

```cinch
csv-error-to-parse-error :: (CsvError -> ParseError)
```

Every variant is a fancy error at its own span (nothing is an
`EUnexpected`: the scanner's refusals are statements about the
dialect, not a missing token).

[Source](/reference-sources/Text.Csv.cinch) · line 134

<a id="value-6373762d7061727365"></a>

## `csv-parse`

```cinch
csv-parse :: (String -> Result CsvError (List (List String)))
```

[Source](/reference-sources/Text.Csv.cinch) · line 217

<a id="value-6373762d70617273652d776974682d68656164657273"></a>

## `csv-parse-with-headers`

```cinch
csv-parse-with-headers :: (String -> Result CsvError (List String, List (List (String, String))))
```

The header is the first row; every later row is keyed by it as
(name, field) pairs in header order. A document with no rows has no
header and no keyed rows.

[Source](/reference-sources/Text.Csv.cinch) · line 225

<a id="value-6373762d7072696e74"></a>

## `csv-print`

```cinch
csv-print :: (List (List String) -> String)
```

Printing (total): fields joined by the comma, rows by LF, and a
field quoted exactly when it must be.

The document's pieces are collected and flattened once: nesting
`string-concat` would re-copy the rest of the document at every row
and field.

[Source](/reference-sources/Text.Csv.cinch) · line 366

<a id="value-72656e6465722d6373762d6572726f72"></a>

## `render-csv-error`

```cinch
render-csv-error :: (String -> (String -> (CsvError -> String)))
```

The megaparsec-style rendering: the error against the source it
came from (`Text.Parse.render-bundle`), the shape a grammar failure
renders in.

[Source](/reference-sources/Text.Csv.cinch) · line 140

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Text/Csv.cinch`. Source SHA-256: `d21273e9e6f5a4ff4f641e9766e2567e232cd80b36eeaec4626a50914c2f4455`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
