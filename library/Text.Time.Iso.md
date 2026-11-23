# Text.Time.Iso

ISO 8601 extended format and RFC 3339 printing and parsing for dates, times, date-times and timestamps.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Text.Time.Iso`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.Time.Iso.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Algebra.Arithmetic](Algebra.Arithmetic.md), [Data.Bytes](Data.Bytes.md), [Data.Machine](Data.Machine.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Data.Time.Date](Data.Time.Date.md), [Data.Time.DateTime](Data.Time.DateTime.md), [Data.Time.TimeOfDay](Data.Time.TimeOfDay.md), [Data.Time.Timestamp](Data.Time.Timestamp.md), [Relation.Eq](Relation.Eq.md), [Relation.Ord](Relation.Ord.md), [Text.Time.Scan](Text.Time.Scan.md).

Non-embedded source closure: [Data.Time.Date](Data.Time.Date.md), [Data.Time.DateTime](Data.Time.DateTime.md), [Data.Time.Duration](Data.Time.Duration.md), [Data.Time.TimeOfDay](Data.Time.TimeOfDay.md), [Data.Time.Timestamp](Data.Time.Timestamp.md), [Data.Time.Weekday](Data.Time.Weekday.md), [Text.Time.Iso](Text.Time.Iso.md), [Text.Time.Scan](Text.Time.Scan.md).

<details><summary>Behaviour and boundary notes from the source</summary>

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

# Types and constructors

<a id="type-49736f4572726f72"></a>

## `IsoError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">IsoError</span></code></pre>

Every refusal, each with the byte position it was detected at.

[Source](/reference-sources/Text.Time.Iso.cinch) · line 124

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">IsoUnexpectedEnd</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Iso</span>.<span class="tok-constructor">IsoError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">IsoExpectedDigit</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Iso</span>.<span class="tok-constructor">IsoError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">IsoExpectedDash</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Iso</span>.<span class="tok-constructor">IsoError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">IsoExpectedColon</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Iso</span>.<span class="tok-constructor">IsoError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">IsoExpectedTimeSeparator</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Iso</span>.<span class="tok-constructor">IsoError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">IsoMissingTime</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Iso</span>.<span class="tok-constructor">IsoError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">IsoUnexpectedTime</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Iso</span>.<span class="tok-constructor">IsoError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">IsoTrailingInput</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Iso</span>.<span class="tok-constructor">IsoError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">IsoNegativeZeroYear</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Iso</span>.<span class="tok-constructor">IsoError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">IsoYearOutOfRange</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Iso</span>.<span class="tok-constructor">IsoError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">IsoMonthOutOfRange</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Iso</span>.<span class="tok-constructor">IsoError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">IsoDayOutOfRange</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Iso</span>.<span class="tok-constructor">IsoError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">IsoHourOutOfRange</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Iso</span>.<span class="tok-constructor">IsoError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">IsoMinuteOutOfRange</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Iso</span>.<span class="tok-constructor">IsoError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">IsoSecondOutOfRange</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Iso</span>.<span class="tok-constructor">IsoError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">IsoLeapSecond</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Iso</span>.<span class="tok-constructor">IsoError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">IsoEmptyFraction</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Iso</span>.<span class="tok-constructor">IsoError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">IsoFractionTooLong</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Iso</span>.<span class="tok-constructor">IsoError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">IsoMissingOffset</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Iso</span>.<span class="tok-constructor">IsoError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">IsoExpectedOffset</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Iso</span>.<span class="tok-constructor">IsoError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">IsoOffsetOutOfRange</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Iso</span>.<span class="tok-constructor">IsoError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">IsoTimestampOutOfRange</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Iso</span>.<span class="tok-constructor">IsoError</span>)</code></pre>

<a id="type-49736f4d6f6465"></a>

## `IsoMode`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">IsoMode</span></code></pre>

[Source](/reference-sources/Text.Time.Iso.cinch)

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">IsoStrict</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Iso</span>.<span class="tok-constructor">IsoMode</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">IsoLenient</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Iso</span>.<span class="tok-constructor">IsoMode</span></code></pre>

<a id="type-49736f4f6666736574"></a>

## `IsoOffset`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">IsoOffset</span></code></pre>

A parsed offset: seconds east of UTC within +-93599, and its
spelling. `Z` and `-00:00` both carry 0 seconds.

[Source](/reference-sources/Text.Time.Iso.cinch) · line 204

<a id="type-49736f4f6666736574466f726d"></a>

## `IsoOffsetForm`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">IsoOffsetForm</span></code></pre>

How an offset was spelled.

[Source](/reference-sources/Text.Time.Iso.cinch) · line 197

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">IsoWrittenZ</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Iso</span>.<span class="tok-constructor">IsoOffsetForm</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">IsoWrittenNumeric</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Iso</span>.<span class="tok-constructor">IsoOffsetForm</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">IsoWrittenNegativeZero</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Iso</span>.<span class="tok-constructor">IsoOffsetForm</span></code></pre>

# Values

<a id="value-646174652d66726f6d2d737472696e67"></a>

## `date-from-string`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">date-from-string</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Iso</span>.<span class="tok-constructor">IsoError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Date</span>.<span class="tok-constructor">Date</span>))</code></pre>

Whole-string parsers.

[Source](/reference-sources/Text.Time.Iso.cinch) · line 219

<a id="value-646174652d66726f6d2d737472696e672d6c656e69656e74"></a>

## `date-from-string-lenient`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">date-from-string-lenient</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Iso</span>.<span class="tok-constructor">IsoError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Date</span>.<span class="tok-constructor">Date</span>))</code></pre>

Also accepts the basic format `YYYYMMDD`.

[Source](/reference-sources/Text.Time.Iso.cinch) · line 225

<a id="value-646174652d746f2d737472696e67"></a>

## `date-to-string`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">date-to-string</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Date</span>.<span class="tok-constructor">Date</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Whole-value printers.

[Source](/reference-sources/Text.Time.Iso.cinch) · line 616

<a id="value-6461746574696d652d66726f6d2d737472696e67"></a>

## `datetime-from-string`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">datetime-from-string</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Iso</span>.<span class="tok-constructor">IsoError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span>))</code></pre>

[Source](/reference-sources/Text.Time.Iso.cinch)

<a id="value-6461746574696d652d66726f6d2d737472696e672d6c656e69656e74"></a>

## `datetime-from-string-lenient`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">datetime-from-string-lenient</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Iso</span>.<span class="tok-constructor">IsoError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span>))</code></pre>

Also accepts `t` or one space as the separator, the basic date and
second 60.

[Source](/reference-sources/Text.Time.Iso.cinch) · line 248

<a id="value-6461746574696d652d746f2d737472696e67"></a>

## `datetime-to-string`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">datetime-to-string</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

[Source](/reference-sources/Text.Time.Iso.cinch)

<a id="value-6461746574696d652d746f2d737472696e672d707265636973696f6e"></a>

## `datetime-to-string-precision`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">datetime-to-string-precision</span> :: ((<span class="tok-variable">digits</span> :: (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">0</span> <span class="tok-number">9</span> <span class="tok-constructor">I64</span>)) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

[Source](/reference-sources/Text.Time.Iso.cinch)

<a id="value-69736f2d646174652d6279746573"></a>

## `iso-date-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">iso-date-bytes</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Date</span>.<span class="tok-constructor">Date</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Piece forms: each answers one exactly-sized fresh buffer, which is
what `Text.Builder.builder-add-bytes` appends without copying (see
NAMED REFUSALS for why the builder itself is not imported here).

[Source](/reference-sources/Text.Time.Iso.cinch) · line 702

<a id="value-69736f2d646174652d6c656e677468"></a>

## `iso-date-length`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">iso-date-length</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Date</span>.<span class="tok-constructor">Date</span> -&gt; <span class="tok-constructor">I64</span>)</code></pre>

10 for years 0..=9999 (`YYYY-MM-DD`), 13 otherwise (`+YYYYYY-MM-DD`).

[Source](/reference-sources/Text.Time.Iso.cinch) · line 561

<a id="value-69736f2d6461746574696d652d6279746573"></a>

## `iso-datetime-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">iso-datetime-bytes</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

[Source](/reference-sources/Text.Time.Iso.cinch)

<a id="value-69736f2d6572726f722d706f736974696f6e"></a>

## `iso-error-position`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">iso-error-position</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Iso</span>.<span class="tok-constructor">IsoError</span> -&gt; <span class="tok-constructor">I64</span>)</code></pre>

[Source](/reference-sources/Text.Time.Iso.cinch)

<a id="value-69736f2d6578706563742d656e64"></a>

## `iso-expect-end`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">iso-expect-end</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). (<span class="tok-constructor">Bytes</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Iso</span>.<span class="tok-constructor">IsoError</span> (<span class="tok-variable">a</span>, <span class="tok-constructor">I64</span>)) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Iso</span>.<span class="tok-constructor">IsoError</span> <span class="tok-variable">a</span>)))</code></pre>

The end of the grammar must be the end of the input.

[Source](/reference-sources/Text.Time.Iso.cinch) · line 279

<a id="value-69736f2d6672616374696f6e2d646967697473"></a>

## `iso-fraction-digits`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">iso-fraction-digits</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">TimeOfDay</span>.<span class="tok-constructor">TimeOfDay</span> -&gt; (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">0</span> <span class="tok-number">9</span> <span class="tok-constructor">I64</span>))</code></pre>

The number of fraction digits the shortest exact rendering needs:
0 when the subsecond is zero, otherwise 9 less the trailing zeros.

[Source](/reference-sources/Text.Time.Iso.cinch) · line 576

<a id="value-69736f2d6f66667365742d6279746573"></a>

## `iso-offset-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">iso-offset-bytes</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

The numeric offset; the empty buffer for one outside +-93599.

[Source](/reference-sources/Text.Time.Iso.cinch) · line 712

<a id="value-69736f2d6f66667365742d666f726d"></a>

## `iso-offset-form`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">iso-offset-form</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Iso</span>.<span class="tok-constructor">IsoOffset</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Iso</span>.<span class="tok-constructor">IsoOffsetForm</span>)</code></pre>

[Source](/reference-sources/Text.Time.Iso.cinch)

<a id="value-69736f2d6f66667365742d696e2d72616e6765"></a>

## `iso-offset-in-range`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">iso-offset-in-range</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

[Source](/reference-sources/Text.Time.Iso.cinch)

<a id="value-69736f2d6f66667365742d6c656e677468"></a>

## `iso-offset-length`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">iso-offset-length</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">I64</span>)</code></pre>

6 (`+HH:MM`), or 9 (`+HH:MM:SS`) when the offset has seconds.

[Source](/reference-sources/Text.Time.Iso.cinch) · line 597

<a id="value-69736f2d6f66667365742d7365636f6e6473"></a>

## `iso-offset-seconds`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">iso-offset-seconds</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Iso</span>.<span class="tok-constructor">IsoOffset</span> -&gt; <span class="tok-constructor">I64</span>)</code></pre>

[Source](/reference-sources/Text.Time.Iso.cinch)

<a id="value-69736f2d70617273652d646174652d6174"></a>

## `iso-parse-date-at`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">iso-parse-date-at</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Iso</span>.<span class="tok-constructor">IsoMode</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Iso</span>.<span class="tok-constructor">IsoError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Date</span>.<span class="tok-constructor">Date</span>, <span class="tok-constructor">I64</span>)))))</code></pre>

Parses a date at byte `i`; answers the date and the position after
it. Does not look at what follows.

[Source](/reference-sources/Text.Time.Iso.cinch) · line 305

<a id="value-69736f2d70617273652d6461746574696d652d6174"></a>

## `iso-parse-datetime-at`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">iso-parse-datetime-at</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Iso</span>.<span class="tok-constructor">IsoMode</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Iso</span>.<span class="tok-constructor">IsoError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span>, <span class="tok-constructor">I64</span>)))))</code></pre>

The date-time machine: date, st-time-sep, time.

[Source](/reference-sources/Text.Time.Iso.cinch) · line 441

<a id="value-69736f2d70617273652d6f66667365742d6174"></a>

## `iso-parse-offset-at`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">iso-parse-offset-at</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Iso</span>.<span class="tok-constructor">IsoMode</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Iso</span>.<span class="tok-constructor">IsoError</span> (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Iso</span>.<span class="tok-constructor">IsoOffset</span>, <span class="tok-constructor">I64</span>)))))</code></pre>

Parses an offset at byte `i`. At the end of input the error is
`IsoMissingOffset`; at any other byte that cannot start an offset
it is `IsoExpectedOffset`.

[Source](/reference-sources/Text.Time.Iso.cinch) · line 472

<a id="value-69736f2d70617273652d74696d652d6174"></a>

## `iso-parse-time-at`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">iso-parse-time-at</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Iso</span>.<span class="tok-constructor">IsoMode</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Iso</span>.<span class="tok-constructor">IsoError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">TimeOfDay</span>.<span class="tok-constructor">TimeOfDay</span>, <span class="tok-constructor">I64</span>)))))</code></pre>

Parses a time at byte `i`; answers the time and the position after
it. The machine stops at the first byte that cannot continue a
time, except that a tenth fraction digit is an error rather than a
stop.

[Source](/reference-sources/Text.Time.Iso.cinch) · line 374

<a id="value-69736f2d70617273652d74696d657374616d702d6174"></a>

## `iso-parse-timestamp-at`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">iso-parse-timestamp-at</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Iso</span>.<span class="tok-constructor">IsoMode</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Iso</span>.<span class="tok-constructor">IsoError</span> ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span>, <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Iso</span>.<span class="tok-constructor">IsoOffset</span>), <span class="tok-constructor">I64</span>)))))</code></pre>

Parses a date-time and its offset at byte `i`; answers the instant,
the offset as written and the position after it (where an RFC 9557
annotation would begin).

Overflow argument: the civil seconds are within +-3.8e11 and the
offset within +-93599, so the subtraction does not wrap; the result
is then range-checked by the `Timestamp` constructor.

[Source](/reference-sources/Text.Time.Iso.cinch) · line 536

<a id="value-69736f2d74696d652d6279746573"></a>

## `iso-time-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">iso-time-bytes</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">TimeOfDay</span>.<span class="tok-constructor">TimeOfDay</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

[Source](/reference-sources/Text.Time.Iso.cinch)

<a id="value-69736f2d74696d652d6c656e677468"></a>

## `iso-time-length`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">iso-time-length</span> :: ((<span class="tok-variable">digits</span> :: (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">0</span> <span class="tok-number">9</span> <span class="tok-constructor">I64</span>)) -&gt; <span class="tok-constructor">I64</span>)</code></pre>

`HH:MM:SS` is 8 bytes; a fraction adds the point and its digits.

[Source](/reference-sources/Text.Time.Iso.cinch) · line 580

<a id="value-69736f2d77726974652d64617465"></a>

## `iso-write-date`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">iso-write-date</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Date</span>.<span class="tok-constructor">Date</span> -&gt; <span class="tok-constructor">I64</span>)))</code></pre>

[Source](/reference-sources/Text.Time.Iso.cinch)

<a id="value-69736f2d77726974652d6f6666736574"></a>

## `iso-write-offset`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">iso-write-offset</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">I64</span>)))</code></pre>

Writes a numeric offset. The caller has it within +-93599 (the
hour field keeps its low two digits otherwise). Offset 0 is
`+00:00`; `Z` is the timestamp printer's choice, not this one's.

[Source](/reference-sources/Text.Time.Iso.cinch) · line 603

<a id="value-69736f2d77726974652d74696d65"></a>

## `iso-write-time`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">iso-write-time</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; ((<span class="tok-variable">digits</span> :: (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">0</span> <span class="tok-number">9</span> <span class="tok-constructor">I64</span>)) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">TimeOfDay</span>.<span class="tok-constructor">TimeOfDay</span> -&gt; <span class="tok-constructor">I64</span>))))</code></pre>

Writes `HH:MM:SS` and, for `digits` above 0, `.` and the first
`digits` digits of the nine-digit subsecond (truncation).

[Source](/reference-sources/Text.Time.Iso.cinch) · line 585

<a id="value-74696d652d66726f6d2d737472696e67"></a>

## `time-from-string`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">time-from-string</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Iso</span>.<span class="tok-constructor">IsoError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">TimeOfDay</span>.<span class="tok-constructor">TimeOfDay</span>))</code></pre>

[Source](/reference-sources/Text.Time.Iso.cinch)

<a id="value-74696d652d66726f6d2d737472696e672d6c656e69656e74"></a>

## `time-from-string-lenient`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">time-from-string-lenient</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Iso</span>.<span class="tok-constructor">IsoError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">TimeOfDay</span>.<span class="tok-constructor">TimeOfDay</span>))</code></pre>

Also reads second 60 as second 59.

[Source](/reference-sources/Text.Time.Iso.cinch) · line 236

<a id="value-74696d652d746f2d737472696e67"></a>

## `time-to-string`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">time-to-string</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">TimeOfDay</span>.<span class="tok-constructor">TimeOfDay</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

`HH:MM:SS` and the shortest exact fraction.

[Source](/reference-sources/Text.Time.Iso.cinch) · line 620

<a id="value-74696d652d746f2d737472696e672d707265636973696f6e"></a>

## `time-to-string-precision`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">time-to-string-precision</span> :: ((<span class="tok-variable">digits</span> :: (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">0</span> <span class="tok-number">9</span> <span class="tok-constructor">I64</span>)) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">TimeOfDay</span>.<span class="tok-constructor">TimeOfDay</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

Exactly `digits` fraction digits, truncating.

[Source](/reference-sources/Text.Time.Iso.cinch) · line 624

<a id="value-74696d657374616d702d66726f6d2d737472696e67"></a>

## `timestamp-from-string`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">timestamp-from-string</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Iso</span>.<span class="tok-constructor">IsoError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span>))</code></pre>

[Source](/reference-sources/Text.Time.Iso.cinch)

<a id="value-74696d657374616d702d66726f6d2d737472696e672d6c656e69656e74"></a>

## `timestamp-from-string-lenient`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">timestamp-from-string-lenient</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Iso</span>.<span class="tok-constructor">IsoError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span>))</code></pre>

Also accepts `z`, and everything the lenient date-time accepts.

[Source](/reference-sources/Text.Time.Iso.cinch) · line 257

<a id="value-74696d657374616d702d66726f6d2d737472696e672d776974682d6f6666736574"></a>

## `timestamp-from-string-with-offset`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">timestamp-from-string-with-offset</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Iso</span>.<span class="tok-constructor">IsoError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span>, <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Iso</span>.<span class="tok-constructor">IsoOffset</span>)))</code></pre>

The instant together with the offset as written (the zoned layer
needs both the seconds and whether it was `Z`).

[Source](/reference-sources/Text.Time.Iso.cinch) · line 262

<a id="value-74696d657374616d702d66726f6d2d737472696e672d776974682d6f66667365742d6c656e69656e74"></a>

## `timestamp-from-string-with-offset-lenient`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">timestamp-from-string-with-offset-lenient</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Iso</span>.<span class="tok-constructor">IsoError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span>, <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Iso</span>.<span class="tok-constructor">IsoOffset</span>)))</code></pre>

[Source](/reference-sources/Text.Time.Iso.cinch)

<a id="value-74696d657374616d702d746f2d737472696e67"></a>

## `timestamp-to-string`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">timestamp-to-string</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

The instant in UTC with `Z` and the shortest exact fraction.

[Source](/reference-sources/Text.Time.Iso.cinch) · line 634

<a id="value-74696d657374616d702d746f2d737472696e672d707265636973696f6e"></a>

## `timestamp-to-string-precision`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">timestamp-to-string-precision</span> :: ((<span class="tok-variable">digits</span> :: (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">0</span> <span class="tok-number">9</span> <span class="tok-constructor">I64</span>)) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

[Source](/reference-sources/Text.Time.Iso.cinch)

<a id="value-74696d657374616d702d746f2d737472696e672d776974682d6f6666736574"></a>

## `timestamp-to-string-with-offset`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">timestamp-to-string-with-offset</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)))</code></pre>

The civil reading of the instant at `off` seconds east of UTC,
followed by the numeric offset (`+00:00` for 0, never `Z`). `None`
when the offset is outside +-93599. With seconds in the offset the
output has `+HH:MM:SS`, which is not RFC 3339 (see the header).

[Source](/reference-sources/Text.Time.Iso.cinch) · line 648

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Text/Time/Iso.cinch`. Source SHA-256: `84701811cacba7dc7bbc9790b539521299d85098ecbfe2cd818fa800f761af00`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
