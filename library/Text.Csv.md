# Text.Csv

CSV parsing/rendering with explicit field, record, and malformed-input handling.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--source">Ship source</span> <code>import Text.Csv</code></div><div class="doc-mod-bar-links"><a href="/library/source/Text.Csv/">View source</a><a href="/library/search/?q=%2BText.Csv">Search module</a><a href="/reference-sources/Text.Csv.cinch" download="Text.Csv.cinch">Download .cinch</a><a href="/library/00-index/">All modules</a></div></div>

**Dependencies:** [Data.Bool](Data.Bool.md), [Data.Char](Data.Char.md), [Data.List](Data.List.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Text.Lex](Text.Lex.md), [Text.Parse](Text.Parse.md), [Text.Show](Text.Show.md).

<details class="doc-closure"><summary>Ship-source closure (4 files)</summary>

Copy these modules beside your entry file before importing `Text.Csv`: [Text.Csv](Text.Csv.md), [Text.Lex](Text.Lex.md), [Text.Parse](Text.Parse.md), [Text.Scan](Text.Scan.md).

```sh
cp seed/prelude/Text/Csv.cinch ./Text.Csv.cinch
cp seed/prelude/Text/Lex.cinch ./Text.Lex.cinch
cp seed/prelude/Text/Parse.cinch ./Text.Parse.cinch
cp seed/prelude/Text/Scan.cinch ./Text.Scan.cinch
```

</details>

<details class="doc-notes"><summary>Module notes</summary>

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

<details class="doc-synopsis" open><summary>Synopsis (8 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-CsvError">CsvError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">CsvError</strong></code></li><li><a class="syn-name" href="#v-csv-error-message">csv-error-message</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">csv-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Csv/#t-CsvError" title="Text.Csv.CsvError">CsvError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-csv-error-span">csv-error-span</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">csv-error-span</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Csv/#t-CsvError" title="Text.Csv.CsvError">CsvError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Lex/#t-Span" title="Text.Lex.Span">Span</a></code></li><li><a class="syn-name" href="#v-csv-error-to-parse-error">csv-error-to-parse-error</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">csv-error-to-parse-error</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Csv/#t-CsvError" title="Text.Csv.CsvError">CsvError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Parse/#t-ParseError" title="Text.Parse.ParseError">ParseError</a></code></li><li><a class="syn-name" href="#v-csv-parse">csv-parse</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">csv-parse</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Text.Csv/#t-CsvError" title="Text.Csv.CsvError">CsvError</a> (<a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a>))</code></li><li><a class="syn-name" href="#v-csv-parse-with-headers">csv-parse-with-headers</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">csv-parse-with-headers</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Text.Csv/#t-CsvError" title="Text.Csv.CsvError">CsvError</a> (<a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a>, <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a>, <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a>)))</code></li><li><a class="syn-name" href="#v-csv-print">csv-print</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">csv-print</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-render-csv-error">render-csv-error</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">render-csv-error</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Csv/#t-CsvError" title="Text.Csv.CsvError">CsvError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-CsvError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">CsvError</strong></code><span class="doc-sig-links"><a href="#t-CsvError" title="Permalink">#</a><a href="/library/source/Text.Csv/#L102" title="View source at line 102">Source</a></span></div>
<div class="doc-body">

The error vocabulary: one named variant per refusal, each carrying
the `Span` it stands at, plus the bridge into `Text.Parse`'s
vocabulary so `render-bundle` renders it (the `Text.Json` shape).

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-CsvUnterminatedQuote"><code><strong class="tok-constructor">CsvUnterminatedQuote</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Lex/#t-Span" title="Text.Lex.Span">Span</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Csv/#t-CsvError" title="Text.Csv.CsvError">CsvError</a></code></li><li id="k-CsvStrayQuote"><code><strong class="tok-constructor">CsvStrayQuote</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Lex/#t-Span" title="Text.Lex.Span">Span</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Csv/#t-CsvError" title="Text.Csv.CsvError">CsvError</a></code></li><li id="k-CsvTextAfterQuote"><code><strong class="tok-constructor">CsvTextAfterQuote</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Lex/#t-Span" title="Text.Lex.Span">Span</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Csv/#t-CsvError" title="Text.Csv.CsvError">CsvError</a></code></li><li id="k-CsvBareCarriageReturn"><code><strong class="tok-constructor">CsvBareCarriageReturn</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Lex/#t-Span" title="Text.Lex.Span">Span</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Csv/#t-CsvError" title="Text.Csv.CsvError">CsvError</a></code></li><li id="k-CsvRowArity"><code><strong class="tok-constructor">CsvRowArity</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Lex/#t-Span" title="Text.Lex.Span">Span</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Csv/#t-CsvError" title="Text.Csv.CsvError">CsvError</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-csv-error-message">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">csv-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Csv/#t-CsvError" title="Text.Csv.CsvError">CsvError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-csv-error-message" title="Permalink">#</a><a href="/library/source/Text.Csv/#L120" title="View source at line 120">Source</a></span></div>
<div class="doc-body">

The diagnostic line for a variant: what was refused, and (for the
arity case) both counts, since the row alone does not say which
side moved.

</div>
</article>

<article class="doc-entry" id="v-csv-error-span">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">csv-error-span</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Csv/#t-CsvError" title="Text.Csv.CsvError">CsvError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Lex/#t-Span" title="Text.Lex.Span">Span</a></code><span class="doc-sig-links"><a href="#v-csv-error-span" title="Permalink">#</a><a href="/library/source/Text.Csv/#L109" title="View source at line 109">Source</a></span></div>
</article>

<article class="doc-entry" id="v-csv-error-to-parse-error">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">csv-error-to-parse-error</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Csv/#t-CsvError" title="Text.Csv.CsvError">CsvError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Parse/#t-ParseError" title="Text.Parse.ParseError">ParseError</a></code><span class="doc-sig-links"><a href="#v-csv-error-to-parse-error" title="Permalink">#</a><a href="/library/source/Text.Csv/#L134" title="View source at line 134">Source</a></span></div>
<div class="doc-body">

Every variant is a fancy error at its own span (nothing is an
`EUnexpected`: the scanner's refusals are statements about the
dialect, not a missing token).

</div>
</article>

<article class="doc-entry" id="v-csv-parse">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">csv-parse</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Text.Csv/#t-CsvError" title="Text.Csv.CsvError">CsvError</a> (<a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a>))</code><span class="doc-sig-links"><a href="#v-csv-parse" title="Permalink">#</a><a href="/library/source/Text.Csv/#L217" title="View source at line 217">Source</a></span></div>
</article>

<article class="doc-entry" id="v-csv-parse-with-headers">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">csv-parse-with-headers</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Text.Csv/#t-CsvError" title="Text.Csv.CsvError">CsvError</a> (<a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a>, <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a>, <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a>)))</code><span class="doc-sig-links"><a href="#v-csv-parse-with-headers" title="Permalink">#</a><a href="/library/source/Text.Csv/#L225" title="View source at line 225">Source</a></span></div>
<div class="doc-body">

The header is the first row; every later row is keyed by it as
(name, field) pairs in header order. A document with no rows has no
header and no keyed rows.

</div>
</article>

<article class="doc-entry" id="v-csv-print">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">csv-print</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-csv-print" title="Permalink">#</a><a href="/library/source/Text.Csv/#L366" title="View source at line 366">Source</a></span></div>
<div class="doc-body">

The document's pieces are collected and flattened once: nesting
`string-concat` would re-copy the rest of the document at every row
and field.

</div>
</article>

<article class="doc-entry" id="v-render-csv-error">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">render-csv-error</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Csv/#t-CsvError" title="Text.Csv.CsvError">CsvError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-render-csv-error" title="Permalink">#</a><a href="/library/source/Text.Csv/#L140" title="View source at line 140">Source</a></span></div>
<div class="doc-body">

The megaparsec-style rendering: the error against the source it
came from (`Text.Parse.render-bundle`), the shape a grammar failure
renders in.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Text/Csv.cinch`](/library/source/Text.Csv/). Source SHA-256: `d21273e9e6f5a4ff4f641e9766e2567e232cd80b36eeaec4626a50914c2f4455`.
