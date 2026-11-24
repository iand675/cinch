# Text.Time.Iso

ISO 8601 extended format and RFC 3339 printing and parsing for dates, times, date-times and timestamps.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Text.Time.Iso</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Text.Time.Iso/source/">View source</a><a href="/packages/search/?q=%2BText.Time.Iso">Search module</a><a href="/reference-sources/Text.Time.Iso.cinch" download="Text.Time.Iso.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Algebra.Arithmetic](Algebra.Arithmetic.md), [Data.Bytes](Data.Bytes.md), [Data.Machine](Data.Machine.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Data.Time.Date](Data.Time.Date.md), [Data.Time.DateTime](Data.Time.DateTime.md), [Data.Time.TimeOfDay](Data.Time.TimeOfDay.md), [Data.Time.Timestamp](Data.Time.Timestamp.md), [Relation.Eq](Relation.Eq.md), [Relation.Ord](Relation.Ord.md), [Text.Time.Scan](Text.Time.Scan.md).

<details class="doc-notes"><summary>Module notes</summary>

Text.Time.Iso: ISO 8601 extended format and RFC 3339, printing and
parsing, for `Date`, `TimeOfDay`, `DateTime` and `Timestamp`.

Grammars (strict; each parser accepts exactly its grammar and nothing
before or after it, design/datetime.md T9):

    date       = year "-" month "-" day
    year       = 4DIGIT | ("+" | "-") 6DIGIT      ; "-000000" refused
    time       = hour ":" minute [":" second [("." | ",") 1*9DIGIT]]
    date-time  = date "T" time
    offset     = "Z" | ("+" | "-") hour ":" minute [":" second]
    timestamp  = date-time offset

The four-digit year is 0000..=9999; the signed six-digit form is the
RFC 9557 / Temporal expanded year and is accepted for every year in
-9999..=9999 (so `+002024-01-01` reads as 2024-01-01). The printer
writes four digits for years 0..=9999 and the signed six-digit form
otherwise (`-000044-03-15`).

The lenient parsers (`-lenient` in the name, `IsoLenient` at the
cursor level) accept, in addition: the basic date `YYYYMMDD`; a
lower case `t` or one space between date and time; a lower case `z`;
and second 60, read as second 59 with the fraction kept (Temporal's
reading of a leap second, since Unix time has none).

Offsets are plain `I64` seconds east of UTC within +-93599
(+-25:59:59, T3); the `Offset`-typed wrappers belong to the zone
layer. RFC 3339 has no seconds in an offset: the printer writes
`+HH:MM` and adds `:SS` only when the offset has seconds, which is
the RFC 9557 / Temporal extension and is not valid RFC 3339; the
parser accepts the same optional `:SS`. `-00:00` (RFC 3339 "offset
unknown") is accepted, reads as offset 0, and is reported as
`IsoWrittenNegativeZero`, distinct from `Z` and from `+00:00`.

Design law shape.
  - Round trip: for every value `x` of each type,
    `T-from-string (T-to-string x) == Ok x`. The time printer writes
    the shortest fraction that is exact (trailing zeros trimmed, none
    when the subsecond is zero), so nothing is lost.
  - `time-to-string-precision k` writes exactly `k` fraction digits,
    truncating (never rounding: rounding can carry into the date).
  - `timestamp-from-string` requires an offset. A date-time with no
    offset is a civil reading, not an instant, and is refused with
    its own error, `IsoMissingOffset`.
  - `timestamp-to-string-with-offset off ts` prints the civil reading
    of `ts` at `off`; parsing it back yields `ts` and `off`.
  - Parsers are state machines over the input's bytes: one function
    per state (`st-*`), each taking the cursor position, branching on
    one byte or one fixed run of digits, and never going back. No
    `Char`, no list, no slice is allocated while parsing.
  - Printers compute the exact output length, allocate one `Bytes`
    of that length and fill it (`iso-*-length` and `iso-write-*`
    write into a caller's buffer at a position; `iso-*-bytes` answer
    the exactly-sized piece).

No operation faults: parsers answer `Result IsoError`, the offset
printer answers `Option`, and the precision is a refined argument.

NAMED REFUSALS:
  - ISO 8601 forms outside the grammar above: the basic time
    (`HHMMSS`), week dates (`2024-W07-4`), ordinal dates (`2024-046`),
    reduced precision (`2024-02`, a bare hour), `24:00`, a fraction on
    the minute, offsets without a colon or without minutes. Each is a
    positioned error.
  - leap seconds: `23:59:60` is `IsoLeapSecond` in strict mode (no
    value represents it) and second 59 in lenient mode.
  - RFC 9557 annotations (`[Europe/Amsterdam]`, `[u-ca=...]`):
    Text.Time.Rfc9557, which continues from the cursor the `iso-parse-*`
    functions return.
  - `Text.Builder` forms (`iso-add-date :: Date -> Builder -> Builder`):
    `Text.Builder` is an on-disk module outside the base package, so
    importing it would make every project that uses this module ship
    it too. The `iso-*-bytes` pieces are what
    `builder-add-bytes` takes, so a builder user writes
    `builder-add-bytes (iso-date-bytes d) out` and no copy is made.
  - a no-copy view of the input: see Text.Time.Scan; one copy in, one
    copy out.
  - `Offset`, `Zoned`, `Span` and ISO 8601 durations: other modules.

</details>

<details class="doc-synopsis"><summary>Synopsis (43 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-IsoError">IsoError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">IsoError</strong></code></li><li><a class="syn-name" href="#t-IsoMode">IsoMode</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">IsoMode</strong></code></li><li><a class="syn-name" href="#t-IsoOffset">IsoOffset</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">IsoOffset</strong></code></li><li><a class="syn-name" href="#t-IsoOffsetForm">IsoOffsetForm</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">IsoOffsetForm</strong></code></li><li><a class="syn-name" href="#v-date-from-string">date-from-string</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">date-from-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Iso/#t-IsoError" title="Text.Time.Iso.IsoError">IsoError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a></code></li><li><a class="syn-name" href="#v-date-from-string-lenient">date-from-string-lenient</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">date-from-string-lenient</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Iso/#t-IsoError" title="Text.Time.Iso.IsoError">IsoError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a></code></li><li><a class="syn-name" href="#v-date-to-string">date-to-string</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">date-to-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-datetime-from-string">datetime-from-string</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">datetime-from-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Iso/#t-IsoError" title="Text.Time.Iso.IsoError">IsoError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a></code></li><li><a class="syn-name" href="#v-datetime-from-string-lenient">datetime-from-string-lenient</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">datetime-from-string-lenient</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Iso/#t-IsoError" title="Text.Time.Iso.IsoError">IsoError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a></code></li><li><a class="syn-name" href="#v-datetime-to-string">datetime-to-string</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">datetime-to-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-datetime-to-string-precision">datetime-to-string-precision</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">datetime-to-string-precision</strong> :: (<span class="tok-variable">digits</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 9 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-iso-date-bytes">iso-date-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">iso-date-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-iso-date-length">iso-date-length</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">iso-date-length</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-iso-datetime-bytes">iso-datetime-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">iso-datetime-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-iso-error-position">iso-error-position</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">iso-error-position</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Iso/#t-IsoError" title="Text.Time.Iso.IsoError">IsoError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-iso-expect-end">iso-expect-end</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">iso-expect-end</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Iso/#t-IsoError" title="Text.Time.Iso.IsoError">IsoError</a> (<span class="tok-variable">a</span>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Iso/#t-IsoError" title="Text.Time.Iso.IsoError">IsoError</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-iso-fraction-digits">iso-fraction-digits</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">iso-fraction-digits</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 9 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-iso-offset-bytes">iso-offset-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">iso-offset-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-iso-offset-form">iso-offset-form</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">iso-offset-form</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Iso/#t-IsoOffset" title="Text.Time.Iso.IsoOffset">IsoOffset</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Iso/#t-IsoOffsetForm" title="Text.Time.Iso.IsoOffsetForm">IsoOffsetForm</a></code></li><li><a class="syn-name" href="#v-iso-offset-in-range">iso-offset-in-range</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">iso-offset-in-range</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-iso-offset-length">iso-offset-length</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">iso-offset-length</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-iso-offset-seconds">iso-offset-seconds</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">iso-offset-seconds</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Iso/#t-IsoOffset" title="Text.Time.Iso.IsoOffset">IsoOffset</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-iso-parse-date-at">iso-parse-date-at</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">iso-parse-date-at</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Iso/#t-IsoMode" title="Text.Time.Iso.IsoMode">IsoMode</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Iso/#t-IsoError" title="Text.Time.Iso.IsoError">IsoError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>)</code></li><li><a class="syn-name" href="#v-iso-parse-datetime-at">iso-parse-datetime-at</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">iso-parse-datetime-at</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Iso/#t-IsoMode" title="Text.Time.Iso.IsoMode">IsoMode</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Iso/#t-IsoError" title="Text.Time.Iso.IsoError">IsoError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>)</code></li><li><a class="syn-name" href="#v-iso-parse-offset-at">iso-parse-offset-at</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">iso-parse-offset-at</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Iso/#t-IsoMode" title="Text.Time.Iso.IsoMode">IsoMode</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Iso/#t-IsoError" title="Text.Time.Iso.IsoError">IsoError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Iso/#t-IsoOffset" title="Text.Time.Iso.IsoOffset">IsoOffset</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>)</code></li><li><a class="syn-name" href="#v-iso-parse-time-at">iso-parse-time-at</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">iso-parse-time-at</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Iso/#t-IsoMode" title="Text.Time.Iso.IsoMode">IsoMode</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Iso/#t-IsoError" title="Text.Time.Iso.IsoError">IsoError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>)</code></li><li><a class="syn-name" href="#v-iso-parse-timestamp-at">iso-parse-timestamp-at</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">iso-parse-timestamp-at</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Iso/#t-IsoMode" title="Text.Time.Iso.IsoMode">IsoMode</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Iso/#t-IsoError" title="Text.Time.Iso.IsoError">IsoError</a> ((<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Iso/#t-IsoOffset" title="Text.Time.Iso.IsoOffset">IsoOffset</a>), <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>)</code></li><li><a class="syn-name" href="#v-iso-time-bytes">iso-time-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">iso-time-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-iso-time-length">iso-time-length</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">iso-time-length</strong> :: (<span class="tok-variable">digits</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 9 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-iso-write-date">iso-write-date</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">iso-write-date</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-iso-write-offset">iso-write-offset</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">iso-write-offset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-iso-write-time">iso-write-time</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">iso-write-time</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> (<span class="tok-variable">digits</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 9 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-time-from-string">time-from-string</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">time-from-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Iso/#t-IsoError" title="Text.Time.Iso.IsoError">IsoError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a></code></li><li><a class="syn-name" href="#v-time-from-string-lenient">time-from-string-lenient</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">time-from-string-lenient</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Iso/#t-IsoError" title="Text.Time.Iso.IsoError">IsoError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a></code></li><li><a class="syn-name" href="#v-time-to-string">time-to-string</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">time-to-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-time-to-string-precision">time-to-string-precision</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">time-to-string-precision</strong> :: (<span class="tok-variable">digits</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 9 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-timestamp-from-string">timestamp-from-string</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">timestamp-from-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Iso/#t-IsoError" title="Text.Time.Iso.IsoError">IsoError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a></code></li><li><a class="syn-name" href="#v-timestamp-from-string-lenient">timestamp-from-string-lenient</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">timestamp-from-string-lenient</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Iso/#t-IsoError" title="Text.Time.Iso.IsoError">IsoError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a></code></li><li><a class="syn-name" href="#v-timestamp-from-string-with-offset">timestamp-from-string-with-offset</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">timestamp-from-string-with-offset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Iso/#t-IsoError" title="Text.Time.Iso.IsoError">IsoError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Iso/#t-IsoOffset" title="Text.Time.Iso.IsoOffset">IsoOffset</a>)</code></li><li><a class="syn-name" href="#v-timestamp-from-string-with-offset-lenient">timestamp-from-string-with-offset-lenient</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">timestamp-from-string-with-offset-lenient</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Iso/#t-IsoError" title="Text.Time.Iso.IsoError">IsoError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Iso/#t-IsoOffset" title="Text.Time.Iso.IsoOffset">IsoOffset</a>)</code></li><li><a class="syn-name" href="#v-timestamp-to-string">timestamp-to-string</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">timestamp-to-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-timestamp-to-string-precision">timestamp-to-string-precision</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">timestamp-to-string-precision</strong> :: (<span class="tok-variable">digits</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 9 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-timestamp-to-string-with-offset">timestamp-to-string-with-offset</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">timestamp-to-string-with-offset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-IsoError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">IsoError</strong></code><span class="doc-sig-links"><a href="#t-IsoError" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Iso/source/#L124" title="View source at line 124">Source</a></span></div>
<div class="doc-body">

Every refusal, each with the byte position it was detected at.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-IsoUnexpectedEnd"><code><strong class="tok-constructor">IsoUnexpectedEnd</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Iso/#t-IsoError" title="Text.Time.Iso.IsoError">IsoError</a></code></li><li id="k-IsoExpectedDigit"><code><strong class="tok-constructor">IsoExpectedDigit</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Iso/#t-IsoError" title="Text.Time.Iso.IsoError">IsoError</a></code></li><li id="k-IsoExpectedDash"><code><strong class="tok-constructor">IsoExpectedDash</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Iso/#t-IsoError" title="Text.Time.Iso.IsoError">IsoError</a></code></li><li id="k-IsoExpectedColon"><code><strong class="tok-constructor">IsoExpectedColon</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Iso/#t-IsoError" title="Text.Time.Iso.IsoError">IsoError</a></code></li><li id="k-IsoExpectedTimeSeparator"><code><strong class="tok-constructor">IsoExpectedTimeSeparator</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Iso/#t-IsoError" title="Text.Time.Iso.IsoError">IsoError</a></code></li><li id="k-IsoMissingTime"><code><strong class="tok-constructor">IsoMissingTime</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Iso/#t-IsoError" title="Text.Time.Iso.IsoError">IsoError</a></code></li><li id="k-IsoUnexpectedTime"><code><strong class="tok-constructor">IsoUnexpectedTime</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Iso/#t-IsoError" title="Text.Time.Iso.IsoError">IsoError</a></code></li><li id="k-IsoTrailingInput"><code><strong class="tok-constructor">IsoTrailingInput</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Iso/#t-IsoError" title="Text.Time.Iso.IsoError">IsoError</a></code></li><li id="k-IsoNegativeZeroYear"><code><strong class="tok-constructor">IsoNegativeZeroYear</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Iso/#t-IsoError" title="Text.Time.Iso.IsoError">IsoError</a></code></li><li id="k-IsoYearOutOfRange"><code><strong class="tok-constructor">IsoYearOutOfRange</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Iso/#t-IsoError" title="Text.Time.Iso.IsoError">IsoError</a></code></li><li id="k-IsoMonthOutOfRange"><code><strong class="tok-constructor">IsoMonthOutOfRange</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Iso/#t-IsoError" title="Text.Time.Iso.IsoError">IsoError</a></code></li><li id="k-IsoDayOutOfRange"><code><strong class="tok-constructor">IsoDayOutOfRange</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Iso/#t-IsoError" title="Text.Time.Iso.IsoError">IsoError</a></code></li><li id="k-IsoHourOutOfRange"><code><strong class="tok-constructor">IsoHourOutOfRange</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Iso/#t-IsoError" title="Text.Time.Iso.IsoError">IsoError</a></code></li><li id="k-IsoMinuteOutOfRange"><code><strong class="tok-constructor">IsoMinuteOutOfRange</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Iso/#t-IsoError" title="Text.Time.Iso.IsoError">IsoError</a></code></li><li id="k-IsoSecondOutOfRange"><code><strong class="tok-constructor">IsoSecondOutOfRange</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Iso/#t-IsoError" title="Text.Time.Iso.IsoError">IsoError</a></code></li><li id="k-IsoLeapSecond"><code><strong class="tok-constructor">IsoLeapSecond</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Iso/#t-IsoError" title="Text.Time.Iso.IsoError">IsoError</a></code></li><li id="k-IsoEmptyFraction"><code><strong class="tok-constructor">IsoEmptyFraction</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Iso/#t-IsoError" title="Text.Time.Iso.IsoError">IsoError</a></code></li><li id="k-IsoFractionTooLong"><code><strong class="tok-constructor">IsoFractionTooLong</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Iso/#t-IsoError" title="Text.Time.Iso.IsoError">IsoError</a></code></li><li id="k-IsoMissingOffset"><code><strong class="tok-constructor">IsoMissingOffset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Iso/#t-IsoError" title="Text.Time.Iso.IsoError">IsoError</a></code></li><li id="k-IsoExpectedOffset"><code><strong class="tok-constructor">IsoExpectedOffset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Iso/#t-IsoError" title="Text.Time.Iso.IsoError">IsoError</a></code></li><li id="k-IsoOffsetOutOfRange"><code><strong class="tok-constructor">IsoOffsetOutOfRange</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Iso/#t-IsoError" title="Text.Time.Iso.IsoError">IsoError</a></code></li><li id="k-IsoTimestampOutOfRange"><code><strong class="tok-constructor">IsoTimestampOutOfRange</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Iso/#t-IsoError" title="Text.Time.Iso.IsoError">IsoError</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-IsoMode">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">IsoMode</strong></code><span class="doc-sig-links"><a href="#t-IsoMode" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Iso/source/#L192" title="View source at line 192">Source</a></span></div>
<div class="doc-body">
<div class="doc-children"><h4>Constructors</h4><ul><li id="k-IsoStrict"><code><strong class="tok-constructor">IsoStrict</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Iso/#t-IsoMode" title="Text.Time.Iso.IsoMode">IsoMode</a></code></li><li id="k-IsoLenient"><code><strong class="tok-constructor">IsoLenient</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Iso/#t-IsoMode" title="Text.Time.Iso.IsoMode">IsoMode</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-IsoOffset">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">IsoOffset</strong></code><span class="doc-sig-links"><a href="#t-IsoOffset" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Iso/source/#L204" title="View source at line 204">Source</a></span></div>
<div class="doc-body">

A parsed offset: seconds east of UTC within +-93599, and its
spelling. `Z` and `-00:00` both carry 0 seconds.

</div>
</article>

<article class="doc-entry" id="t-IsoOffsetForm">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">IsoOffsetForm</strong></code><span class="doc-sig-links"><a href="#t-IsoOffsetForm" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Iso/source/#L197" title="View source at line 197">Source</a></span></div>
<div class="doc-body">

How an offset was spelled.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-IsoWrittenZ"><code><strong class="tok-constructor">IsoWrittenZ</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Iso/#t-IsoOffsetForm" title="Text.Time.Iso.IsoOffsetForm">IsoOffsetForm</a></code></li><li id="k-IsoWrittenNumeric"><code><strong class="tok-constructor">IsoWrittenNumeric</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Iso/#t-IsoOffsetForm" title="Text.Time.Iso.IsoOffsetForm">IsoOffsetForm</a></code></li><li id="k-IsoWrittenNegativeZero"><code><strong class="tok-constructor">IsoWrittenNegativeZero</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Iso/#t-IsoOffsetForm" title="Text.Time.Iso.IsoOffsetForm">IsoOffsetForm</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-date-from-string">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">date-from-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Iso/#t-IsoError" title="Text.Time.Iso.IsoError">IsoError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a></code><span class="doc-sig-links"><a href="#v-date-from-string" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Iso/source/#L219" title="View source at line 219">Source</a></span></div>
<div class="doc-body">

Whole-string parsers.

</div>
</article>

<article class="doc-entry" id="v-date-from-string-lenient">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">date-from-string-lenient</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Iso/#t-IsoError" title="Text.Time.Iso.IsoError">IsoError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a></code><span class="doc-sig-links"><a href="#v-date-from-string-lenient" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Iso/source/#L225" title="View source at line 225">Source</a></span></div>
<div class="doc-body">

Also accepts the basic format `YYYYMMDD`.

</div>
</article>

<article class="doc-entry" id="v-date-to-string">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">date-to-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-date-to-string" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Iso/source/#L616" title="View source at line 616">Source</a></span></div>
<div class="doc-body">

Whole-value printers.

</div>
</article>

<article class="doc-entry" id="v-datetime-from-string">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">datetime-from-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Iso/#t-IsoError" title="Text.Time.Iso.IsoError">IsoError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a></code><span class="doc-sig-links"><a href="#v-datetime-from-string" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Iso/source/#L241" title="View source at line 241">Source</a></span></div>
</article>

<article class="doc-entry" id="v-datetime-from-string-lenient">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">datetime-from-string-lenient</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Iso/#t-IsoError" title="Text.Time.Iso.IsoError">IsoError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a></code><span class="doc-sig-links"><a href="#v-datetime-from-string-lenient" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Iso/source/#L248" title="View source at line 248">Source</a></span></div>
<div class="doc-body">

Also accepts `t` or one space as the separator, the basic date and
second 60.

</div>
</article>

<article class="doc-entry" id="v-datetime-to-string">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">datetime-to-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-datetime-to-string" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Iso/source/#L627" title="View source at line 627">Source</a></span></div>
</article>

<article class="doc-entry" id="v-datetime-to-string-precision">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">datetime-to-string-precision</strong> :: (<span class="tok-variable">digits</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 9 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-datetime-to-string-precision" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Iso/source/#L630" title="View source at line 630">Source</a></span></div>
</article>

<article class="doc-entry" id="v-iso-date-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">iso-date-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-iso-date-bytes" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Iso/source/#L702" title="View source at line 702">Source</a></span></div>
<div class="doc-body">

Piece forms: each answers one exactly-sized fresh buffer, which is
what `Text.Builder.builder-add-bytes` appends without copying (see
NAMED REFUSALS for why the builder itself is not imported here).

</div>
</article>

<article class="doc-entry" id="v-iso-date-length">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">iso-date-length</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-iso-date-length" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Iso/source/#L561" title="View source at line 561">Source</a></span></div>
<div class="doc-body">

10 for years 0..=9999 (`YYYY-MM-DD`), 13 otherwise (`+YYYYYY-MM-DD`).

</div>
</article>

<article class="doc-entry" id="v-iso-datetime-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">iso-datetime-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-iso-datetime-bytes" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Iso/source/#L708" title="View source at line 708">Source</a></span></div>
</article>

<article class="doc-entry" id="v-iso-error-position">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">iso-error-position</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Iso/#t-IsoError" title="Text.Time.Iso.IsoError">IsoError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-iso-error-position" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Iso/source/#L167" title="View source at line 167">Source</a></span></div>
</article>

<article class="doc-entry" id="v-iso-expect-end">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">iso-expect-end</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Iso/#t-IsoError" title="Text.Time.Iso.IsoError">IsoError</a> (<span class="tok-variable">a</span>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Iso/#t-IsoError" title="Text.Time.Iso.IsoError">IsoError</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-iso-expect-end" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Iso/source/#L279" title="View source at line 279">Source</a></span></div>
<div class="doc-body">

The end of the grammar must be the end of the input.

</div>
</article>

<article class="doc-entry" id="v-iso-fraction-digits">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">iso-fraction-digits</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 9 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-iso-fraction-digits" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Iso/source/#L576" title="View source at line 576">Source</a></span></div>
<div class="doc-body">

The number of fraction digits the shortest exact rendering needs:
0 when the subsecond is zero, otherwise 9 less the trailing zeros.

</div>
</article>

<article class="doc-entry" id="v-iso-offset-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">iso-offset-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-iso-offset-bytes" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Iso/source/#L712" title="View source at line 712">Source</a></span></div>
<div class="doc-body">

The numeric offset; the empty buffer for one outside +-93599.

</div>
</article>

<article class="doc-entry" id="v-iso-offset-form">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">iso-offset-form</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Iso/#t-IsoOffset" title="Text.Time.Iso.IsoOffset">IsoOffset</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Iso/#t-IsoOffsetForm" title="Text.Time.Iso.IsoOffsetForm">IsoOffsetForm</a></code><span class="doc-sig-links"><a href="#v-iso-offset-form" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Iso/source/#L211" title="View source at line 211">Source</a></span></div>
</article>

<article class="doc-entry" id="v-iso-offset-in-range">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">iso-offset-in-range</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-iso-offset-in-range" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Iso/source/#L593" title="View source at line 593">Source</a></span></div>
</article>

<article class="doc-entry" id="v-iso-offset-length">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">iso-offset-length</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-iso-offset-length" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Iso/source/#L597" title="View source at line 597">Source</a></span></div>
<div class="doc-body">

6 (`+HH:MM`), or 9 (`+HH:MM:SS`) when the offset has seconds.

</div>
</article>

<article class="doc-entry" id="v-iso-offset-seconds">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">iso-offset-seconds</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Iso/#t-IsoOffset" title="Text.Time.Iso.IsoOffset">IsoOffset</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-iso-offset-seconds" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Iso/source/#L207" title="View source at line 207">Source</a></span></div>
</article>

<article class="doc-entry" id="v-iso-parse-date-at">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">iso-parse-date-at</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Iso/#t-IsoMode" title="Text.Time.Iso.IsoMode">IsoMode</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Iso/#t-IsoError" title="Text.Time.Iso.IsoError">IsoError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>)</code><span class="doc-sig-links"><a href="#v-iso-parse-date-at" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Iso/source/#L305" title="View source at line 305">Source</a></span></div>
<div class="doc-body">

Parses a date at byte `i`; answers the date and the position after
it. Does not look at what follows.

</div>
</article>

<article class="doc-entry" id="v-iso-parse-datetime-at">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">iso-parse-datetime-at</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Iso/#t-IsoMode" title="Text.Time.Iso.IsoMode">IsoMode</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Iso/#t-IsoError" title="Text.Time.Iso.IsoError">IsoError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>)</code><span class="doc-sig-links"><a href="#v-iso-parse-datetime-at" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Iso/source/#L441" title="View source at line 441">Source</a></span></div>
<div class="doc-body">

The date-time machine: date, st-time-sep, time.

</div>
</article>

<article class="doc-entry" id="v-iso-parse-offset-at">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">iso-parse-offset-at</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Iso/#t-IsoMode" title="Text.Time.Iso.IsoMode">IsoMode</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Iso/#t-IsoError" title="Text.Time.Iso.IsoError">IsoError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Iso/#t-IsoOffset" title="Text.Time.Iso.IsoOffset">IsoOffset</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>)</code><span class="doc-sig-links"><a href="#v-iso-parse-offset-at" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Iso/source/#L472" title="View source at line 472">Source</a></span></div>
<div class="doc-body">

Parses an offset at byte `i`. At the end of input the error is
`IsoMissingOffset`; at any other byte that cannot start an offset
it is `IsoExpectedOffset`.

</div>
</article>

<article class="doc-entry" id="v-iso-parse-time-at">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">iso-parse-time-at</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Iso/#t-IsoMode" title="Text.Time.Iso.IsoMode">IsoMode</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Iso/#t-IsoError" title="Text.Time.Iso.IsoError">IsoError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>)</code><span class="doc-sig-links"><a href="#v-iso-parse-time-at" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Iso/source/#L374" title="View source at line 374">Source</a></span></div>
<div class="doc-body">

Parses a time at byte `i`; answers the time and the position after
it. The machine stops at the first byte that cannot continue a
time, except that a tenth fraction digit is an error rather than a
stop.

</div>
</article>

<article class="doc-entry" id="v-iso-parse-timestamp-at">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">iso-parse-timestamp-at</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Iso/#t-IsoMode" title="Text.Time.Iso.IsoMode">IsoMode</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Iso/#t-IsoError" title="Text.Time.Iso.IsoError">IsoError</a> ((<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Iso/#t-IsoOffset" title="Text.Time.Iso.IsoOffset">IsoOffset</a>), <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>)</code><span class="doc-sig-links"><a href="#v-iso-parse-timestamp-at" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Iso/source/#L536" title="View source at line 536">Source</a></span></div>
<div class="doc-body">

Parses a date-time and its offset at byte `i`; answers the instant,
the offset as written and the position after it (where an RFC 9557
annotation would begin).

Overflow argument: the civil seconds are within +-3.8e11 and the
offset within +-93599, so the subtraction does not wrap; the result
is then range-checked by the `Timestamp` constructor.

</div>
</article>

<article class="doc-entry" id="v-iso-time-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">iso-time-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-iso-time-bytes" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Iso/source/#L705" title="View source at line 705">Source</a></span></div>
</article>

<article class="doc-entry" id="v-iso-time-length">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">iso-time-length</strong> :: (<span class="tok-variable">digits</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 9 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-iso-time-length" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Iso/source/#L580" title="View source at line 580">Source</a></span></div>
<div class="doc-body">

`HH:MM:SS` is 8 bytes; a fraction adds the point and its digits.

</div>
</article>

<article class="doc-entry" id="v-iso-write-date">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">iso-write-date</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-iso-write-date" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Iso/source/#L566" title="View source at line 566">Source</a></span></div>
</article>

<article class="doc-entry" id="v-iso-write-offset">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">iso-write-offset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-iso-write-offset" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Iso/source/#L603" title="View source at line 603">Source</a></span></div>
<div class="doc-body">

Writes a numeric offset. The caller has it within +-93599 (the
hour field keeps its low two digits otherwise). Offset 0 is
`+00:00`; `Z` is the timestamp printer's choice, not this one's.

</div>
</article>

<article class="doc-entry" id="v-iso-write-time">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">iso-write-time</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> (<span class="tok-variable">digits</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 9 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-iso-write-time" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Iso/source/#L585" title="View source at line 585">Source</a></span></div>
<div class="doc-body">

Writes `HH:MM:SS` and, for `digits` above 0, `.` and the first
`digits` digits of the nine-digit subsecond (truncation).

</div>
</article>

<article class="doc-entry" id="v-time-from-string">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">time-from-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Iso/#t-IsoError" title="Text.Time.Iso.IsoError">IsoError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a></code><span class="doc-sig-links"><a href="#v-time-from-string" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Iso/source/#L230" title="View source at line 230">Source</a></span></div>
</article>

<article class="doc-entry" id="v-time-from-string-lenient">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">time-from-string-lenient</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Iso/#t-IsoError" title="Text.Time.Iso.IsoError">IsoError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a></code><span class="doc-sig-links"><a href="#v-time-from-string-lenient" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Iso/source/#L236" title="View source at line 236">Source</a></span></div>
<div class="doc-body">

Also reads second 60 as second 59.

</div>
</article>

<article class="doc-entry" id="v-time-to-string">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">time-to-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-time-to-string" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Iso/source/#L620" title="View source at line 620">Source</a></span></div>
<div class="doc-body">

`HH:MM:SS` and the shortest exact fraction.

</div>
</article>

<article class="doc-entry" id="v-time-to-string-precision">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">time-to-string-precision</strong> :: (<span class="tok-variable">digits</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 9 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-time-to-string-precision" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Iso/source/#L624" title="View source at line 624">Source</a></span></div>
<div class="doc-body">

Exactly `digits` fraction digits, truncating.

</div>
</article>

<article class="doc-entry" id="v-timestamp-from-string">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">timestamp-from-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Iso/#t-IsoError" title="Text.Time.Iso.IsoError">IsoError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a></code><span class="doc-sig-links"><a href="#v-timestamp-from-string" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Iso/source/#L253" title="View source at line 253">Source</a></span></div>
</article>

<article class="doc-entry" id="v-timestamp-from-string-lenient">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">timestamp-from-string-lenient</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Iso/#t-IsoError" title="Text.Time.Iso.IsoError">IsoError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a></code><span class="doc-sig-links"><a href="#v-timestamp-from-string-lenient" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Iso/source/#L257" title="View source at line 257">Source</a></span></div>
<div class="doc-body">

Also accepts `z`, and everything the lenient date-time accepts.

</div>
</article>

<article class="doc-entry" id="v-timestamp-from-string-with-offset">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">timestamp-from-string-with-offset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Iso/#t-IsoError" title="Text.Time.Iso.IsoError">IsoError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Iso/#t-IsoOffset" title="Text.Time.Iso.IsoOffset">IsoOffset</a>)</code><span class="doc-sig-links"><a href="#v-timestamp-from-string-with-offset" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Iso/source/#L262" title="View source at line 262">Source</a></span></div>
<div class="doc-body">

The instant together with the offset as written (the zoned layer
needs both the seconds and whether it was `Z`).

</div>
</article>

<article class="doc-entry" id="v-timestamp-from-string-with-offset-lenient">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">timestamp-from-string-with-offset-lenient</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Iso/#t-IsoError" title="Text.Time.Iso.IsoError">IsoError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Iso/#t-IsoOffset" title="Text.Time.Iso.IsoOffset">IsoOffset</a>)</code><span class="doc-sig-links"><a href="#v-timestamp-from-string-with-offset-lenient" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Iso/source/#L267" title="View source at line 267">Source</a></span></div>
</article>

<article class="doc-entry" id="v-timestamp-to-string">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">timestamp-to-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-timestamp-to-string" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Iso/source/#L634" title="View source at line 634">Source</a></span></div>
<div class="doc-body">

The instant in UTC with `Z` and the shortest exact fraction.

</div>
</article>

<article class="doc-entry" id="v-timestamp-to-string-precision">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">timestamp-to-string-precision</strong> :: (<span class="tok-variable">digits</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 9 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-timestamp-to-string-precision" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Iso/source/#L639" title="View source at line 639">Source</a></span></div>
</article>

<article class="doc-entry" id="v-timestamp-to-string-with-offset">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">timestamp-to-string-with-offset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-timestamp-to-string-with-offset" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Iso/source/#L648" title="View source at line 648">Source</a></span></div>
<div class="doc-body">

The civil reading of the instant at `off` seconds east of UTC,
followed by the numeric offset (`+00:00` for 0, never `Z`). `None`
when the offset is outside +-93599. With seconds in the offset the
output has `+HH:MM:SS`, which is not RFC 3339 (see the header).

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Text/Time/Iso.cinch`](/packages/stdlib/Text.Time.Iso/source/). Source SHA-256: `84701811cacba7dc7bbc9790b539521299d85098ecbfe2cd818fa800f761af00`.
