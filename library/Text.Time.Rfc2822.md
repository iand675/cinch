# Text.Time.Rfc2822

RFC 2822/5322 internet-message date-time printing and parsing.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--source">Ship source</span> <code>import Text.Time.Rfc2822</code></div><div class="doc-mod-bar-links"><a href="/library/source/Text.Time.Rfc2822/">View source</a><a href="/library/search/?q=%2BText.Time.Rfc2822">Search module</a><a href="/reference-sources/Text.Time.Rfc2822.cinch" download="Text.Time.Rfc2822.cinch">Download .cinch</a><a href="/library/00-index/">All modules</a></div></div>

**Dependencies:** [Algebra.Arithmetic](Algebra.Arithmetic.md), [Data.Bytes](Data.Bytes.md), [Data.Machine](Data.Machine.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Data.Time.Date](Data.Time.Date.md), [Data.Time.DateTime](Data.Time.DateTime.md), [Data.Time.Offset](Data.Time.Offset.md), [Data.Time.TimeOfDay](Data.Time.TimeOfDay.md), [Data.Time.Timestamp](Data.Time.Timestamp.md), [Data.Time.Weekday](Data.Time.Weekday.md), [Data.Time.Zone](Data.Time.Zone.md), [Data.Time.Zoned](Data.Time.Zoned.md), [Relation.Eq](Relation.Eq.md), [Relation.Ord](Relation.Ord.md), [Text.Time.Scan](Text.Time.Scan.md).

<details class="doc-closure"><summary>Ship-source closure (13 files)</summary>

Copy these modules beside your entry file before importing `Text.Time.Rfc2822`: [Data.Time.Date](Data.Time.Date.md), [Data.Time.DateTime](Data.Time.DateTime.md), [Data.Time.Duration](Data.Time.Duration.md), [Data.Time.Offset](Data.Time.Offset.md), [Data.Time.TimeOfDay](Data.Time.TimeOfDay.md), [Data.Time.Timestamp](Data.Time.Timestamp.md), [Data.Time.Weekday](Data.Time.Weekday.md), [Data.Time.Zone](Data.Time.Zone.md), [Data.Time.Zone.Posix](Data.Time.Zone.Posix.md), [Data.Time.Zone.Types](Data.Time.Zone.Types.md), [Data.Time.Zoned](Data.Time.Zoned.md), [Text.Time.Rfc2822](Text.Time.Rfc2822.md), [Text.Time.Scan](Text.Time.Scan.md).

```sh
cp seed/prelude/Data/Time/Date.cinch ./Data.Time.Date.cinch
cp seed/prelude/Data/Time/DateTime.cinch ./Data.Time.DateTime.cinch
cp seed/prelude/Data/Time/Duration.cinch ./Data.Time.Duration.cinch
cp seed/prelude/Data/Time/Offset.cinch ./Data.Time.Offset.cinch
cp seed/prelude/Data/Time/TimeOfDay.cinch ./Data.Time.TimeOfDay.cinch
cp seed/prelude/Data/Time/Timestamp.cinch ./Data.Time.Timestamp.cinch
cp seed/prelude/Data/Time/Weekday.cinch ./Data.Time.Weekday.cinch
cp seed/prelude/Data/Time/Zone.cinch ./Data.Time.Zone.cinch
cp seed/prelude/Data/Time/Zone/Posix.cinch ./Data.Time.Zone.Posix.cinch
cp seed/prelude/Data/Time/Zone/Types.cinch ./Data.Time.Zone.Types.cinch
cp seed/prelude/Data/Time/Zoned.cinch ./Data.Time.Zoned.cinch
cp seed/prelude/Text/Time/Rfc2822.cinch ./Text.Time.Rfc2822.cinch
cp seed/prelude/Text/Time/Scan.cinch ./Text.Time.Scan.cinch
```

</details>

<details class="doc-notes"><summary>Module notes</summary>

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

<details class="doc-synopsis" open><summary>Synopsis (8 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-Rfc2822Error">Rfc2822Error</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Rfc2822Error</strong></code></li><li><a class="syn-name" href="#v-rfc2822-error-position">rfc2822-error-position</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rfc2822-error-position</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Time.Rfc2822/#t-Rfc2822Error" title="Text.Time.Rfc2822.Rfc2822Error">Rfc2822Error</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-timestamp-from-rfc2822">timestamp-from-rfc2822</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">timestamp-from-rfc2822</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Rfc2822/#t-Rfc2822Error" title="Text.Time.Rfc2822.Rfc2822Error">Rfc2822Error</a> (<a class="doc-type-link tok-constructor" href="/library/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a>, <a class="doc-type-link tok-constructor" href="/library/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a>)</code></li><li><a class="syn-name" href="#v-timestamp-from-rfc2822-lenient">timestamp-from-rfc2822-lenient</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">timestamp-from-rfc2822-lenient</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Rfc2822/#t-Rfc2822Error" title="Text.Time.Rfc2822.Rfc2822Error">Rfc2822Error</a> (<a class="doc-type-link tok-constructor" href="/library/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a>, <a class="doc-type-link tok-constructor" href="/library/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a>)</code></li><li><a class="syn-name" href="#v-timestamp-to-rfc2822">timestamp-to-rfc2822</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">timestamp-to-rfc2822</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Rfc2822/#t-Rfc2822Error" title="Text.Time.Rfc2822.Rfc2822Error">Rfc2822Error</a> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-zoned-from-rfc2822">zoned-from-rfc2822</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zoned-from-rfc2822</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Rfc2822/#t-Rfc2822Error" title="Text.Time.Rfc2822.Rfc2822Error">Rfc2822Error</a> <a class="doc-type-link tok-constructor" href="/library/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a></code></li><li><a class="syn-name" href="#v-zoned-from-rfc2822-lenient">zoned-from-rfc2822-lenient</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zoned-from-rfc2822-lenient</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Rfc2822/#t-Rfc2822Error" title="Text.Time.Rfc2822.Rfc2822Error">Rfc2822Error</a> <a class="doc-type-link tok-constructor" href="/library/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a></code></li><li><a class="syn-name" href="#v-zoned-to-rfc2822">zoned-to-rfc2822</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zoned-to-rfc2822</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Rfc2822/#t-Rfc2822Error" title="Text.Time.Rfc2822.Rfc2822Error">Rfc2822Error</a> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-Rfc2822Error">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Rfc2822Error</strong></code><span class="doc-sig-links"><a href="#t-Rfc2822Error" title="Permalink">#</a><a href="/library/source/Text.Time.Rfc2822/#L105" title="View source at line 105">Source</a></span></div>
<div class="doc-body">

One constructor per refusal. A position is a byte index into the
input.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-Rfc2822UnexpectedEnd"><code><strong class="tok-constructor">Rfc2822UnexpectedEnd</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Rfc2822/#t-Rfc2822Error" title="Text.Time.Rfc2822.Rfc2822Error">Rfc2822Error</a></code></li><li id="k-Rfc2822ExpectedDigit"><code><strong class="tok-constructor">Rfc2822ExpectedDigit</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Rfc2822/#t-Rfc2822Error" title="Text.Time.Rfc2822.Rfc2822Error">Rfc2822Error</a></code></li><li id="k-Rfc2822ExpectedSpace"><code><strong class="tok-constructor">Rfc2822ExpectedSpace</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Rfc2822/#t-Rfc2822Error" title="Text.Time.Rfc2822.Rfc2822Error">Rfc2822Error</a></code></li><li id="k-Rfc2822ExpectedComma"><code><strong class="tok-constructor">Rfc2822ExpectedComma</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Rfc2822/#t-Rfc2822Error" title="Text.Time.Rfc2822.Rfc2822Error">Rfc2822Error</a></code></li><li id="k-Rfc2822ExpectedColon"><code><strong class="tok-constructor">Rfc2822ExpectedColon</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Rfc2822/#t-Rfc2822Error" title="Text.Time.Rfc2822.Rfc2822Error">Rfc2822Error</a></code></li><li id="k-Rfc2822ExpectedZone"><code><strong class="tok-constructor">Rfc2822ExpectedZone</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Rfc2822/#t-Rfc2822Error" title="Text.Time.Rfc2822.Rfc2822Error">Rfc2822Error</a></code></li><li id="k-Rfc2822InvalidDayName"><code><strong class="tok-constructor">Rfc2822InvalidDayName</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Rfc2822/#t-Rfc2822Error" title="Text.Time.Rfc2822.Rfc2822Error">Rfc2822Error</a></code></li><li id="k-Rfc2822InvalidMonth"><code><strong class="tok-constructor">Rfc2822InvalidMonth</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Rfc2822/#t-Rfc2822Error" title="Text.Time.Rfc2822.Rfc2822Error">Rfc2822Error</a></code></li><li id="k-Rfc2822InvalidYear"><code><strong class="tok-constructor">Rfc2822InvalidYear</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Rfc2822/#t-Rfc2822Error" title="Text.Time.Rfc2822.Rfc2822Error">Rfc2822Error</a></code></li><li id="k-Rfc2822ObsoleteYear"><code><strong class="tok-constructor">Rfc2822ObsoleteYear</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Rfc2822/#t-Rfc2822Error" title="Text.Time.Rfc2822.Rfc2822Error">Rfc2822Error</a></code></li><li id="k-Rfc2822ObsoleteZone"><code><strong class="tok-constructor">Rfc2822ObsoleteZone</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Rfc2822/#t-Rfc2822Error" title="Text.Time.Rfc2822.Rfc2822Error">Rfc2822Error</a></code></li><li id="k-Rfc2822UnknownZone"><code><strong class="tok-constructor">Rfc2822UnknownZone</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Rfc2822/#t-Rfc2822Error" title="Text.Time.Rfc2822.Rfc2822Error">Rfc2822Error</a></code></li><li id="k-Rfc2822CommentNotAllowed"><code><strong class="tok-constructor">Rfc2822CommentNotAllowed</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Rfc2822/#t-Rfc2822Error" title="Text.Time.Rfc2822.Rfc2822Error">Rfc2822Error</a></code></li><li id="k-Rfc2822UnclosedComment"><code><strong class="tok-constructor">Rfc2822UnclosedComment</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Rfc2822/#t-Rfc2822Error" title="Text.Time.Rfc2822.Rfc2822Error">Rfc2822Error</a></code></li><li id="k-Rfc2822DayOutOfRange"><code><strong class="tok-constructor">Rfc2822DayOutOfRange</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Rfc2822/#t-Rfc2822Error" title="Text.Time.Rfc2822.Rfc2822Error">Rfc2822Error</a></code></li><li id="k-Rfc2822HourOutOfRange"><code><strong class="tok-constructor">Rfc2822HourOutOfRange</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Rfc2822/#t-Rfc2822Error" title="Text.Time.Rfc2822.Rfc2822Error">Rfc2822Error</a></code></li><li id="k-Rfc2822MinuteOutOfRange"><code><strong class="tok-constructor">Rfc2822MinuteOutOfRange</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Rfc2822/#t-Rfc2822Error" title="Text.Time.Rfc2822.Rfc2822Error">Rfc2822Error</a></code></li><li id="k-Rfc2822SecondOutOfRange"><code><strong class="tok-constructor">Rfc2822SecondOutOfRange</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Rfc2822/#t-Rfc2822Error" title="Text.Time.Rfc2822.Rfc2822Error">Rfc2822Error</a></code></li><li id="k-Rfc2822LeapSecond"><code><strong class="tok-constructor">Rfc2822LeapSecond</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Rfc2822/#t-Rfc2822Error" title="Text.Time.Rfc2822.Rfc2822Error">Rfc2822Error</a></code></li><li id="k-Rfc2822OffsetOutOfRange"><code><strong class="tok-constructor">Rfc2822OffsetOutOfRange</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Rfc2822/#t-Rfc2822Error" title="Text.Time.Rfc2822.Rfc2822Error">Rfc2822Error</a></code></li><li id="k-Rfc2822WeekdayMismatch"><code><strong class="tok-constructor">Rfc2822WeekdayMismatch</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Rfc2822/#t-Rfc2822Error" title="Text.Time.Rfc2822.Rfc2822Error">Rfc2822Error</a></code></li><li id="k-Rfc2822TrailingInput"><code><strong class="tok-constructor">Rfc2822TrailingInput</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Rfc2822/#t-Rfc2822Error" title="Text.Time.Rfc2822.Rfc2822Error">Rfc2822Error</a></code></li><li id="k-Rfc2822TimestampOutOfRange"><code><strong class="tok-constructor">Rfc2822TimestampOutOfRange</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Time.Rfc2822/#t-Rfc2822Error" title="Text.Time.Rfc2822.Rfc2822Error">Rfc2822Error</a></code></li><li id="k-Rfc2822YearNotPrintable"><code><strong class="tok-constructor">Rfc2822YearNotPrintable</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Time.Rfc2822/#t-Rfc2822Error" title="Text.Time.Rfc2822.Rfc2822Error">Rfc2822Error</a></code></li><li id="k-Rfc2822OffsetNotPrintable"><code><strong class="tok-constructor">Rfc2822OffsetNotPrintable</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Time.Rfc2822/#t-Rfc2822Error" title="Text.Time.Rfc2822.Rfc2822Error">Rfc2822Error</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-rfc2822-error-position">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rfc2822-error-position</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Time.Rfc2822/#t-Rfc2822Error" title="Text.Time.Rfc2822.Rfc2822Error">Rfc2822Error</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-rfc2822-error-position" title="Permalink">#</a><a href="/library/source/Text.Time.Rfc2822/#L158" title="View source at line 158">Source</a></span></div>
<div class="doc-body">

The byte position of a parse refusal; `None` for the range and the
two printing refusals.

</div>
</article>

<article class="doc-entry" id="v-timestamp-from-rfc2822">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">timestamp-from-rfc2822</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Rfc2822/#t-Rfc2822Error" title="Text.Time.Rfc2822.Rfc2822Error">Rfc2822Error</a> (<a class="doc-type-link tok-constructor" href="/library/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a>, <a class="doc-type-link tok-constructor" href="/library/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a>)</code><span class="doc-sig-links"><a href="#v-timestamp-from-rfc2822" title="Permalink">#</a><a href="/library/source/Text.Time.Rfc2822/#L239" title="View source at line 239">Source</a></span></div>
<div class="doc-body">

Parsing. States, in order:
  st-start -> [st-comma] -> st-day -> st-month -> st-year
  -> st-hour -> st-minute -> [st-second] -> st-zone
  -> (st-zone-digits | st-zone-name) -> st-end.
`lenient` is carried through; `skip` is the white space (and, in
lenient mode, comment and line break) reader between tokens.

</div>
</article>

<article class="doc-entry" id="v-timestamp-from-rfc2822-lenient">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">timestamp-from-rfc2822-lenient</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Rfc2822/#t-Rfc2822Error" title="Text.Time.Rfc2822.Rfc2822Error">Rfc2822Error</a> (<a class="doc-type-link tok-constructor" href="/library/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a>, <a class="doc-type-link tok-constructor" href="/library/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a>)</code><span class="doc-sig-links"><a href="#v-timestamp-from-rfc2822-lenient" title="Permalink">#</a><a href="/library/source/Text.Time.Rfc2822/#L243" title="View source at line 243">Source</a></span></div>
<div class="doc-body">

Also the obsolete forms (header).

</div>
</article>

<article class="doc-entry" id="v-timestamp-to-rfc2822">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">timestamp-to-rfc2822</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Rfc2822/#t-Rfc2822Error" title="Text.Time.Rfc2822.Rfc2822Error">Rfc2822Error</a> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-timestamp-to-rfc2822" title="Permalink">#</a><a href="/library/source/Text.Time.Rfc2822/#L192" title="View source at line 192">Source</a></span></div>
<div class="doc-body">

The civil reading of the instant at `o` (header). The subsecond is
dropped.

</div>
</article>

<article class="doc-entry" id="v-zoned-from-rfc2822">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zoned-from-rfc2822</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Rfc2822/#t-Rfc2822Error" title="Text.Time.Rfc2822.Rfc2822Error">Rfc2822Error</a> <a class="doc-type-link tok-constructor" href="/library/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a></code><span class="doc-sig-links"><a href="#v-zoned-from-rfc2822" title="Permalink">#</a><a href="/library/source/Text.Time.Rfc2822/#L247" title="View source at line 247">Source</a></span></div>
<div class="doc-body">

The instant in the fixed-offset zone of the offset written.

</div>
</article>

<article class="doc-entry" id="v-zoned-from-rfc2822-lenient">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zoned-from-rfc2822-lenient</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Rfc2822/#t-Rfc2822Error" title="Text.Time.Rfc2822.Rfc2822Error">Rfc2822Error</a> <a class="doc-type-link tok-constructor" href="/library/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a></code><span class="doc-sig-links"><a href="#v-zoned-from-rfc2822-lenient" title="Permalink">#</a><a href="/library/source/Text.Time.Rfc2822/#L250" title="View source at line 250">Source</a></span></div>
</article>

<article class="doc-entry" id="v-zoned-to-rfc2822">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zoned-to-rfc2822</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Rfc2822/#t-Rfc2822Error" title="Text.Time.Rfc2822.Rfc2822Error">Rfc2822Error</a> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-zoned-to-rfc2822" title="Permalink">#</a><a href="/library/source/Text.Time.Rfc2822/#L197" title="View source at line 197">Source</a></span></div>
<div class="doc-body">

At the zone's offset in force. The zone's name is not written (the
format has no place for one).

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Text/Time/Rfc2822.cinch`](/library/source/Text.Time.Rfc2822/). Source SHA-256: `d264b18fb08807d3449b90d1f0025894e8b1a22b3e67dcf3b06665ce767576bc`.
