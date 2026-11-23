# Text.Time.Rfc2822

RFC 2822/5322 internet-message date-time printing and parsing.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Text.Time.Rfc2822`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.Time.Rfc2822.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Algebra.Arithmetic](Algebra.Arithmetic.md), [Data.Bytes](Data.Bytes.md), [Data.Machine](Data.Machine.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Data.Time.Date](Data.Time.Date.md), [Data.Time.DateTime](Data.Time.DateTime.md), [Data.Time.Offset](Data.Time.Offset.md), [Data.Time.TimeOfDay](Data.Time.TimeOfDay.md), [Data.Time.Timestamp](Data.Time.Timestamp.md), [Data.Time.Weekday](Data.Time.Weekday.md), [Data.Time.Zone](Data.Time.Zone.md), [Data.Time.Zoned](Data.Time.Zoned.md), [Relation.Eq](Relation.Eq.md), [Relation.Ord](Relation.Ord.md), [Text.Time.Scan](Text.Time.Scan.md).

Non-embedded source closure: [Data.Time.Date](Data.Time.Date.md), [Data.Time.DateTime](Data.Time.DateTime.md), [Data.Time.Duration](Data.Time.Duration.md), [Data.Time.Offset](Data.Time.Offset.md), [Data.Time.TimeOfDay](Data.Time.TimeOfDay.md), [Data.Time.Timestamp](Data.Time.Timestamp.md), [Data.Time.Weekday](Data.Time.Weekday.md), [Data.Time.Zone](Data.Time.Zone.md), [Data.Time.Zone.Posix](Data.Time.Zone.Posix.md), [Data.Time.Zone.Types](Data.Time.Zone.Types.md), [Data.Time.Zoned](Data.Time.Zoned.md), [Text.Time.Rfc2822](Text.Time.Rfc2822.md), [Text.Time.Scan](Text.Time.Scan.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Text.Time.Rfc2822: the date-time of internet mail and HTTP-era
headers (RFC 2822 section 3.3, unchanged in RFC 5322), printing and
parsing (design/datetime.md, layer L4; T9, T10).

    Sat, 13 Jul 2024 15:09:59 -0400

Strict grammar (RFC 5322 section 3.3 without the obsolete forms and
without comments; `ws` is one or more spaces or tabs, names match in
any letter case):

    date-time = [ws] [day-name [ws] "," [ws]] day ws month ws year ws
                hour ":" minute [":" second] ws zone [ws]
    day-name  = "Mon" | "Tue" | "Wed" | "Thu" | "Fri" | "Sat" | "Sun"
    day       = 1*2DIGIT
    month     = "Jan" | ... | "Dec"
    year      = 4DIGIT
    zone      = ("+" | "-") 4DIGIT          ; hours and minutes

The lenient parsers (`-lenient`) accept the RFC's obsolete syntax as
well: comments in parentheses (nested, with `\` quoting) and line
breaks wherever white space may appear, and around the colons of the
time; no white space before the zone; two-digit years (00..=49 are
2000..=2049, 50..=99 are 1950..=1999) and three-digit years (1900 is
added); the zone names `UT` and `GMT` (+0000) and the North American
`EST` -0500, `EDT` -0400, `CST` -0600, `CDT` -0500, `MST` -0700,
`MDT` -0600, `PST` -0800, `PDT` -0700; and second 60, read as second
59 (Text.Time.Iso's reading of a leap second).

Design law shape.
  - A parse answers the instant and the offset as written, as
    `(Timestamp, Offset)`; `zoned-from-rfc2822` is the same in the
    fixed-offset zone of that offset. The format names no time zone,
    only an offset, so no `TzDb` is involved.
  - A day name, when present, must be the date's weekday
    (`Rfc2822WeekdayMismatch`).
  - `-0000` (the RFC's "the local offset is not known") reads as
    offset zero, like `+0000`.
  - The printer writes the day name, a two-digit day, the month
    name, a four-digit year, `HH:MM:SS` and `+HHMM`: always 31 bytes.
    The subsecond is dropped (the format has no fraction), so
    printing then parsing gives the instant truncated to its second
    and the same offset.
  - The parser is a state machine over the input's bytes (`st-*`),
    never going back; the printer fills one 31-byte buffer.
  - No operation faults; every refusal is an `Rfc2822Error`
    constructor, with the byte position where one exists.

NAMED REFUSALS:
  - printing a year before 0000 or (never, with T3's range) after
    9999: `Rfc2822YearNotPrintable`. The format's year has no sign.
  - printing at an offset with seconds: `Rfc2822OffsetNotPrintable`.
    The format's zone has hours and minutes only; Jiff rounds the
    offset here, which makes the text name a different instant.
  - the obsolete forms in strict mode, each by name:
    `Rfc2822ObsoleteYear`, `Rfc2822ObsoleteZone`,
    `Rfc2822CommentNotAllowed`.
  - zone names outside the list above, including the military
    single letters (which RFC 2822 itself says were defined wrongly
    and carry no information): `Rfc2822UnknownZone`.
  - a year of five or more digits, or of one: `Rfc2822InvalidYear`.
  - an offset beyond +-25:59 or with minutes above 59:
    `Rfc2822OffsetOutOfRange`.
  - the rest of a mail header (the field name, unfolding of a header
    line before the date begins): the caller's.

</details>

# Types and constructors

<a id="type-526663323832324572726f72"></a>

## `Rfc2822Error`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Rfc2822Error</span></code></pre>

One constructor per refusal. A position is a byte index into the
input.

[Source](/reference-sources/Text.Time.Rfc2822.cinch) · line 105

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Rfc2822UnexpectedEnd</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc2822</span>.<span class="tok-constructor">Rfc2822Error</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Rfc2822ExpectedDigit</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc2822</span>.<span class="tok-constructor">Rfc2822Error</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Rfc2822ExpectedSpace</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc2822</span>.<span class="tok-constructor">Rfc2822Error</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Rfc2822ExpectedComma</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc2822</span>.<span class="tok-constructor">Rfc2822Error</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Rfc2822ExpectedColon</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc2822</span>.<span class="tok-constructor">Rfc2822Error</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Rfc2822ExpectedZone</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc2822</span>.<span class="tok-constructor">Rfc2822Error</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Rfc2822InvalidDayName</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc2822</span>.<span class="tok-constructor">Rfc2822Error</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Rfc2822InvalidMonth</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc2822</span>.<span class="tok-constructor">Rfc2822Error</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Rfc2822InvalidYear</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc2822</span>.<span class="tok-constructor">Rfc2822Error</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Rfc2822ObsoleteYear</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc2822</span>.<span class="tok-constructor">Rfc2822Error</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Rfc2822ObsoleteZone</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc2822</span>.<span class="tok-constructor">Rfc2822Error</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Rfc2822UnknownZone</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc2822</span>.<span class="tok-constructor">Rfc2822Error</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Rfc2822CommentNotAllowed</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc2822</span>.<span class="tok-constructor">Rfc2822Error</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Rfc2822UnclosedComment</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc2822</span>.<span class="tok-constructor">Rfc2822Error</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Rfc2822DayOutOfRange</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc2822</span>.<span class="tok-constructor">Rfc2822Error</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Rfc2822HourOutOfRange</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc2822</span>.<span class="tok-constructor">Rfc2822Error</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Rfc2822MinuteOutOfRange</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc2822</span>.<span class="tok-constructor">Rfc2822Error</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Rfc2822SecondOutOfRange</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc2822</span>.<span class="tok-constructor">Rfc2822Error</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Rfc2822LeapSecond</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc2822</span>.<span class="tok-constructor">Rfc2822Error</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Rfc2822OffsetOutOfRange</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc2822</span>.<span class="tok-constructor">Rfc2822Error</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Rfc2822WeekdayMismatch</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc2822</span>.<span class="tok-constructor">Rfc2822Error</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Rfc2822TrailingInput</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc2822</span>.<span class="tok-constructor">Rfc2822Error</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Rfc2822TimestampOutOfRange</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc2822</span>.<span class="tok-constructor">Rfc2822Error</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Rfc2822YearNotPrintable</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc2822</span>.<span class="tok-constructor">Rfc2822Error</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Rfc2822OffsetNotPrintable</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc2822</span>.<span class="tok-constructor">Rfc2822Error</span></code></pre>

# Values

<a id="value-726663323832322d6572726f722d706f736974696f6e"></a>

## `rfc2822-error-position`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rfc2822-error-position</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc2822</span>.<span class="tok-constructor">Rfc2822Error</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">I64</span>))</code></pre>

The byte position of a parse refusal; `None` for the range and the
two printing refusals.

[Source](/reference-sources/Text.Time.Rfc2822.cinch) · line 158

<a id="value-74696d657374616d702d66726f6d2d72666332383232"></a>

## `timestamp-from-rfc2822`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">timestamp-from-rfc2822</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc2822</span>.<span class="tok-constructor">Rfc2822Error</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span>, <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Offset</span>.<span class="tok-constructor">Offset</span>)))</code></pre>

Parsing. States, in order:
  st-start -> [st-comma] -> st-day -> st-month -> st-year
  -> st-hour -> st-minute -> [st-second] -> st-zone
  -> (st-zone-digits | st-zone-name) -> st-end.
`lenient` is carried through; `skip` is the white space (and, in
lenient mode, comment and line break) reader between tokens.

[Source](/reference-sources/Text.Time.Rfc2822.cinch) · line 239

<a id="value-74696d657374616d702d66726f6d2d726663323832322d6c656e69656e74"></a>

## `timestamp-from-rfc2822-lenient`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">timestamp-from-rfc2822-lenient</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc2822</span>.<span class="tok-constructor">Rfc2822Error</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span>, <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Offset</span>.<span class="tok-constructor">Offset</span>)))</code></pre>

Also the obsolete forms (header).

[Source](/reference-sources/Text.Time.Rfc2822.cinch) · line 243

<a id="value-74696d657374616d702d746f2d72666332383232"></a>

## `timestamp-to-rfc2822`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">timestamp-to-rfc2822</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Offset</span>.<span class="tok-constructor">Offset</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc2822</span>.<span class="tok-constructor">Rfc2822Error</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)))</code></pre>

The civil reading of the instant at `o` (header). The subsecond is
dropped.

[Source](/reference-sources/Text.Time.Rfc2822.cinch) · line 192

<a id="value-7a6f6e65642d66726f6d2d72666332383232"></a>

## `zoned-from-rfc2822`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zoned-from-rfc2822</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc2822</span>.<span class="tok-constructor">Rfc2822Error</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span>))</code></pre>

The instant in the fixed-offset zone of the offset written.

[Source](/reference-sources/Text.Time.Rfc2822.cinch) · line 247

<a id="value-7a6f6e65642d66726f6d2d726663323832322d6c656e69656e74"></a>

## `zoned-from-rfc2822-lenient`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zoned-from-rfc2822-lenient</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc2822</span>.<span class="tok-constructor">Rfc2822Error</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span>))</code></pre>

[Source](/reference-sources/Text.Time.Rfc2822.cinch)

<a id="value-7a6f6e65642d746f2d72666332383232"></a>

## `zoned-to-rfc2822`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zoned-to-rfc2822</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc2822</span>.<span class="tok-constructor">Rfc2822Error</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

At the zone's offset in force. The zone's name is not written (the
format has no place for one).

[Source](/reference-sources/Text.Time.Rfc2822.cinch) · line 197

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Text/Time/Rfc2822.cinch`. Source SHA-256: `d264b18fb08807d3449b90d1f0025894e8b1a22b3e67dcf3b06665ce767576bc`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
