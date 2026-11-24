# Text.Time.Strtime

POSIX strftime/strptime-style formatting and parsing over broken-down time fields.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Text.Time.Strtime`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.Time.Strtime.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Algebra.Arithmetic](Algebra.Arithmetic.md), [Data.Bytes](Data.Bytes.md), [Data.Machine](Data.Machine.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Data.Time.Date](Data.Time.Date.md), [Data.Time.DateTime](Data.Time.DateTime.md), [Data.Time.TimeOfDay](Data.Time.TimeOfDay.md), [Data.Time.Timestamp](Data.Time.Timestamp.md), [Data.Time.Weekday](Data.Time.Weekday.md), [Relation.Eq](Relation.Eq.md), [Relation.Ord](Relation.Ord.md), [Text.Time.Scan](Text.Time.Scan.md).

Non-embedded source closure: [Data.Time.Date](Data.Time.Date.md), [Data.Time.DateTime](Data.Time.DateTime.md), [Data.Time.Duration](Data.Time.Duration.md), [Data.Time.TimeOfDay](Data.Time.TimeOfDay.md), [Data.Time.Timestamp](Data.Time.Timestamp.md), [Data.Time.Weekday](Data.Time.Weekday.md), [Text.Time.Scan](Text.Time.Scan.md), [Text.Time.Strtime](Text.Time.Strtime.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Text.Time.Strtime: POSIX `strftime`/`strptime` as Jiff supports them,
over a `BrokenDown` value.

A `BrokenDown` is a bag of optional fields (year, month, day, hour,
twelve-hour clock hour, minute, second, subsecond nanoseconds,
weekday, day of year, UTC offset in seconds, meridiem, ISO week year
and week, the two POSIX week numbers, Unix seconds, a zone
abbreviation and an IANA zone name). Parsing fills the fields a
format names and nothing else; the conversions out
(`broken-down-to-date` and friends) decide whether the fields make a
value and refuse by name when one is missing or two disagree.
Formatting reads a field, or derives it from the others when it is
absent (the weekday from the date, the twelve-hour hour from the
hour, the Unix second from the civil fields and the offset).

Conversions (the same set for printing and parsing unless noted):

  %Y year, sign and at least 4 digits   %C century   %y year in century
  %m month 01..12      %d day 01..31     %e day, space padded
  %j day of year 001..366
  %H hour 00..23       %I hour 01..12    %M minute    %S second 00..59
  %f fraction of a second: `%f` the shortest exact digits (at least
     one), `%3f` `%6f` `%9f` (any 1..9) exactly that many, truncating;
     `%.f` a point and the shortest digits, or nothing when zero;
     `%.3f` a point and exactly that many digits
  %p AM/PM   %P am/pm
  %A %a weekday name, full and abbreviated   %B %b %h month name
  %u weekday 1..7 (Monday 1)   %w weekday 0..6 (Sunday 0)
  %G ISO week year   %g its last two digits   %V ISO week 01..53
  %U week of the year, Sunday first, 00..53   %W the same, Monday first
  %z offset `+HHMM`   %:z `+HH:MM`   %::z `+HH:MM:SS` (the first two
     add the seconds only when the offset has them)
  %Z zone abbreviation (PRINT ONLY)   %Q IANA zone name, or the
     offset as `%:z` when there is no name
  %s Unix seconds
  %D `%m/%d/%y`   %F `%Y-%m-%d`   %T `%H:%M:%S`   %R `%H:%M`
  %n newline   %t tab   %% a percent sign

Between `%` and the conversion: one optional flag, an optional width
(at most 255), then colons (before `z`) or a point and an optional
precision (before `f`).
  `-` no padding     `_` pad with spaces    `0` pad with zeros
  `^` upper case     `#` swap case: names and `%P` become upper case,
                     `%p` and `%Z` become lower case (as glibc and Jiff)
A width pads numbers to that many digits (the sign, if any, comes
before the digits and is not counted) and pads text on the left with
spaces. Flags and widths are read but have no effect when parsing,
except that `_` (and `%e`) skips leading spaces.

Parsing rules (Jiff's): a numeric conversion reads one to its maximum
number of digits (`%d` reads `7` or `07`); `%Y` and `%G` take an
optional sign and up to four digits; names match in any letter case
and `%a`/`%A` (and `%b`/`%B`) each accept either the abbreviated or
the full name; white space in the format (and `%n`, `%t`) matches any
run of white space in the input, including none; every other format
byte must match exactly; the whole input must be consumed. `%y`
without `%C` reads 69..99 as 1969..1999 and 00..68 as 2000..2068.
A second of 60 is refused (no value represents it).

Design law shape.
  - A format is compiled once, left to right, by a state machine over
    its bytes (`fmt-literal`, `fmt-flag`, `fmt-width`, `fmt-modifier`,
    `fmt-colons`, `fmt-precision`, `fmt-conversion`) into a `Format`:
    a spine of literal bytes and conversions. `strtime-format-compile`
    exposes it so a format is parsed once and applied many times;
    `strftime`/`strptime` compile and apply. Every format refusal
    (unknown conversion, a format ending inside a conversion, a
    modifier on a conversion that has none) is found at compile time,
    before any value or input is looked at.
  - `strftime` allocates one buffer whose size bounds the output
    (every conversion writes at most 32 bytes plus its width plus the
    zone strings), fills it left to right, and takes the written
    prefix. No list of pieces and no `string-concat`.
  - `strptime` reads the input's bytes by index, one state per
    conversion kind (`p-*`), never going back and allocating only the
    updated `BrokenDown`.
  - Round trip: for a format that names enough fields,
    `strptime f (strftime f b)` has the same named fields as `b`.
  - `broken-down-to-date` builds the date from year, month and day;
    or year and day of year; or ISO week year, ISO week and weekday;
    or year, a POSIX week number and weekday; and then every other
    date field that is present must agree with it (a parsed weekday
    that contradicts the date is `StrtimeWeekdayMismatch`).

NAMED REFUSALS:
  - locales: month, weekday and meridiem names are English only, and
    `%c`, `%x`, `%X`, `%r`, `%E*`, `%O*` are unknown conversions.
    Locale-aware formatting is out of scope (design/datetime.md 10.6).
  - parsing `%Z`: a zone abbreviation is ambiguous (`CST` is three
    zones), so `strptime` refuses a format with `%Z`
    (`StrtimeZoneAbbreviationNotParsed`). Printing one is fine.
  - more than one flag (`%-^a`): the second flag is an unknown
    conversion, as in Jiff.
  - leap seconds: `%S` refuses 60.
  - `Offset`, `TimeZone` and `Zoned`: offsets here are plain `I64`
    seconds east of UTC within +-93599; the zone layer wraps them.
    `%Q` only carries the name; resolving it needs a `TzDb`.
  - `Text.Builder`: not imported (it is outside the base package);
    the printer writes one preallocated `Bytes` instead.
  - a no-copy view of the format or the input: see Text.Time.Scan.

</details>

# Types and constructors

<a id="type-42726f6b656e446f776e"></a>

## `BrokenDown`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">BrokenDown</span></code></pre>

[Source](/reference-sources/Text.Time.Strtime.cinch)

<a id="type-466f726d6174"></a>

## `Format`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Format</span></code></pre>

[Source](/reference-sources/Text.Time.Strtime.cinch)

<a id="type-4d6572696469656d"></a>

## `Meridiem`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Meridiem</span></code></pre>

[Source](/reference-sources/Text.Time.Strtime.cinch)

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Am</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">Meridiem</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Pm</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">Meridiem</span></code></pre>

<a id="type-53747274696d654572726f72"></a>

## `StrtimeError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">StrtimeError</span></code></pre>

One constructor per refusal. A position is a byte index: into the
FORMAT for the first four, into the INPUT for the parse refusals.
A conversion is named by its letter's byte (`Y` is 89).

[Source](/reference-sources/Text.Time.Strtime.cinch) · line 161

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">StrtimeUnknownConversion</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">StrtimeError</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">StrtimeDanglingPercent</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">StrtimeError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">StrtimeWidthTooLarge</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">StrtimeError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">StrtimeModifierNotAllowed</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">StrtimeError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">StrtimeMissingField</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">StrtimeError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">StrtimeUnexpectedEnd</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">StrtimeError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">StrtimeExpectedDigit</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">StrtimeError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">StrtimeExpectedName</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">StrtimeError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">StrtimeExpectedOffset</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">StrtimeError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">StrtimeLiteralMismatch</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">StrtimeError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">StrtimeFieldOutOfRange</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">StrtimeError</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">StrtimeTrailingInput</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">StrtimeError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">StrtimeZoneAbbreviationNotParsed</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">StrtimeError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">StrtimeMissingYear</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">StrtimeError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">StrtimeMissingMonth</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">StrtimeError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">StrtimeMissingDay</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">StrtimeError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">StrtimeMissingHour</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">StrtimeError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">StrtimeMissingMeridiem</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">StrtimeError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">StrtimeMissingOffset</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">StrtimeError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">StrtimeInvalidDate</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">StrtimeError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">StrtimeDateMismatch</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">StrtimeError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">StrtimeWeekdayMismatch</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">StrtimeError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">StrtimeDayOfYearMismatch</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">StrtimeError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">StrtimeIsoWeekMismatch</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">StrtimeError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">StrtimeWeekNumberMismatch</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">StrtimeError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">StrtimeHourMismatch</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">StrtimeError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">StrtimeTimestampOutOfRange</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">StrtimeError</span></code></pre>

# Values

<a id="value-62726f6b656e2d646f776e2d646179"></a>

## `broken-down-day`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">broken-down-day</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">BrokenDown</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">I64</span>))</code></pre>

[Source](/reference-sources/Text.Time.Strtime.cinch)

<a id="value-62726f6b656e2d646f776e2d6461792d6f662d79656172"></a>

## `broken-down-day-of-year`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">broken-down-day-of-year</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">BrokenDown</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">I64</span>))</code></pre>

[Source](/reference-sources/Text.Time.Strtime.cinch)

<a id="value-62726f6b656e2d646f776e2d656d707479"></a>

## `broken-down-empty`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">broken-down-empty</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">BrokenDown</span></code></pre>

[Source](/reference-sources/Text.Time.Strtime.cinch)

<a id="value-62726f6b656e2d646f776e2d66726f6d2d64617465"></a>

## `broken-down-from-date`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">broken-down-from-date</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Date</span>.<span class="tok-constructor">Date</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">BrokenDown</span>)</code></pre>

Conversions in. Only the defining fields are set; the printer
derives the rest (weekday, day of year, week numbers).

[Source](/reference-sources/Text.Time.Strtime.cinch) · line 481

<a id="value-62726f6b656e2d646f776e2d66726f6d2d6461746574696d65"></a>

## `broken-down-from-datetime`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">broken-down-from-datetime</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">BrokenDown</span>)</code></pre>

[Source](/reference-sources/Text.Time.Strtime.cinch)

<a id="value-62726f6b656e2d646f776e2d66726f6d2d74696d65"></a>

## `broken-down-from-time`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">broken-down-from-time</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">TimeOfDay</span>.<span class="tok-constructor">TimeOfDay</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">BrokenDown</span>)</code></pre>

[Source](/reference-sources/Text.Time.Strtime.cinch)

<a id="value-62726f6b656e2d646f776e2d66726f6d2d74696d657374616d702d61742d6f6666736574"></a>

## `broken-down-from-timestamp-at-offset`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">broken-down-from-timestamp-at-offset</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">BrokenDown</span>)))</code></pre>

The civil reading at `off` seconds east of UTC, that offset and the
Unix second. `None` for an offset outside +-93599 (every timestamp
has a civil reading at every offset inside it, T3). The seconds are
within +-3.8e11, so the sum does not wrap.

[Source](/reference-sources/Text.Time.Strtime.cinch) · line 500

<a id="value-62726f6b656e2d646f776e2d66726f6d2d74696d657374616d702d757463"></a>

## `broken-down-from-timestamp-utc`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">broken-down-from-timestamp-utc</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">BrokenDown</span>)</code></pre>

The UTC reading, offset 0 and the Unix second.

[Source](/reference-sources/Text.Time.Strtime.cinch) · line 491

<a id="value-62726f6b656e2d646f776e2d686f7572"></a>

## `broken-down-hour`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">broken-down-hour</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">BrokenDown</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">I64</span>))</code></pre>

[Source](/reference-sources/Text.Time.Strtime.cinch)

<a id="value-62726f6b656e2d646f776e2d686f75723132"></a>

## `broken-down-hour12`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">broken-down-hour12</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">BrokenDown</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">I64</span>))</code></pre>

[Source](/reference-sources/Text.Time.Strtime.cinch)

<a id="value-62726f6b656e2d646f776e2d69736f2d7765656b"></a>

## `broken-down-iso-week`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">broken-down-iso-week</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">BrokenDown</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">I64</span>))</code></pre>

[Source](/reference-sources/Text.Time.Strtime.cinch)

<a id="value-62726f6b656e2d646f776e2d69736f2d7765656b2d79656172"></a>

## `broken-down-iso-week-year`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">broken-down-iso-week-year</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">BrokenDown</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">I64</span>))</code></pre>

[Source](/reference-sources/Text.Time.Strtime.cinch)

<a id="value-62726f6b656e2d646f776e2d6d6572696469656d"></a>

## `broken-down-meridiem`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">broken-down-meridiem</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">BrokenDown</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">Meridiem</span>))</code></pre>

[Source](/reference-sources/Text.Time.Strtime.cinch)

<a id="value-62726f6b656e2d646f776e2d6d696e757465"></a>

## `broken-down-minute`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">broken-down-minute</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">BrokenDown</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">I64</span>))</code></pre>

[Source](/reference-sources/Text.Time.Strtime.cinch)

<a id="value-62726f6b656e2d646f776e2d6d6f6e7468"></a>

## `broken-down-month`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">broken-down-month</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">BrokenDown</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">I64</span>))</code></pre>

[Source](/reference-sources/Text.Time.Strtime.cinch)

<a id="value-62726f6b656e2d646f776e2d6f6666736574"></a>

## `broken-down-offset`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">broken-down-offset</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">BrokenDown</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">I64</span>))</code></pre>

Seconds east of UTC.

[Source](/reference-sources/Text.Time.Strtime.cinch) · line 380

<a id="value-62726f6b656e2d646f776e2d7365636f6e64"></a>

## `broken-down-second`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">broken-down-second</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">BrokenDown</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">I64</span>))</code></pre>

[Source](/reference-sources/Text.Time.Strtime.cinch)

<a id="value-62726f6b656e2d646f776e2d7375627365632d6e616e6f7365636f6e64"></a>

## `broken-down-subsec-nanosecond`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">broken-down-subsec-nanosecond</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">BrokenDown</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">I64</span>))</code></pre>

[Source](/reference-sources/Text.Time.Strtime.cinch)

<a id="value-62726f6b656e2d646f776e2d746f2d64617465"></a>

## `broken-down-to-date`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">broken-down-to-date</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">BrokenDown</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">StrtimeError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Date</span>.<span class="tok-constructor">Date</span>))</code></pre>

The date the fields name (see the header for the four ways), after
which every other date field that is present must agree with it.

[Source](/reference-sources/Text.Time.Strtime.cinch) · line 521

<a id="value-62726f6b656e2d646f776e2d746f2d6461746574696d65"></a>

## `broken-down-to-datetime`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">broken-down-to-datetime</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">BrokenDown</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">StrtimeError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span>))</code></pre>

A date is required; with no time field at all the time is midnight.

[Source](/reference-sources/Text.Time.Strtime.cinch) · line 644

<a id="value-62726f6b656e2d646f776e2d746f2d74696d65"></a>

## `broken-down-to-time`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">broken-down-to-time</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">BrokenDown</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">StrtimeError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">TimeOfDay</span>.<span class="tok-constructor">TimeOfDay</span>))</code></pre>

The hour comes from the hour field, or from the twelve-hour hour
and the meridiem; when several are present they must agree. A
missing minute, second or subsecond is zero.

[Source](/reference-sources/Text.Time.Strtime.cinch) · line 608

<a id="value-62726f6b656e2d646f776e2d746f2d74696d657374616d70"></a>

## `broken-down-to-timestamp`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">broken-down-to-timestamp</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">BrokenDown</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">StrtimeError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span>))</code></pre>

The instant: the Unix second when it is set (`%s`; the subsecond
field is added), otherwise the civil date-time read at the offset,
which is then required (`StrtimeMissingOffset`: a civil reading is
not an instant). The civil seconds are within +-3.8e11 and the
offset within +-93599, so the subtraction does not wrap.

[Source](/reference-sources/Text.Time.Strtime.cinch) · line 659

<a id="value-62726f6b656e2d646f776e2d756e69782d7365636f6e6473"></a>

## `broken-down-unix-seconds`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">broken-down-unix-seconds</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">BrokenDown</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">I64</span>))</code></pre>

[Source](/reference-sources/Text.Time.Strtime.cinch)

<a id="value-62726f6b656e2d646f776e2d7765656b2d6d6f6e646179"></a>

## `broken-down-week-monday`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">broken-down-week-monday</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">BrokenDown</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">I64</span>))</code></pre>

`%W`: the week of the year with Monday first, 0..=53.

[Source](/reference-sources/Text.Time.Strtime.cinch) · line 399

<a id="value-62726f6b656e2d646f776e2d7765656b2d73756e646179"></a>

## `broken-down-week-sunday`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">broken-down-week-sunday</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">BrokenDown</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">I64</span>))</code></pre>

`%U`: the week of the year with Sunday first, 0..=53.

[Source](/reference-sources/Text.Time.Strtime.cinch) · line 395

<a id="value-62726f6b656e2d646f776e2d7765656b646179"></a>

## `broken-down-weekday`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">broken-down-weekday</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">BrokenDown</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Weekday</span>.<span class="tok-constructor">Weekday</span>))</code></pre>

[Source](/reference-sources/Text.Time.Strtime.cinch)

<a id="value-62726f6b656e2d646f776e2d776974682d646179"></a>

## `broken-down-with-day`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">broken-down-with-day</span> :: ((<span class="tok-variable">d</span> :: (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">1</span> <span class="tok-number">31</span> <span class="tok-constructor">I64</span>)) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">BrokenDown</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">BrokenDown</span>))</code></pre>

[Source](/reference-sources/Text.Time.Strtime.cinch)

<a id="value-62726f6b656e2d646f776e2d776974682d6461792d6f662d79656172"></a>

## `broken-down-with-day-of-year`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">broken-down-with-day-of-year</span> :: ((<span class="tok-variable">n</span> :: (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">1</span> <span class="tok-number">366</span> <span class="tok-constructor">I64</span>)) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">BrokenDown</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">BrokenDown</span>))</code></pre>

[Source](/reference-sources/Text.Time.Strtime.cinch)

<a id="value-62726f6b656e2d646f776e2d776974682d686f7572"></a>

## `broken-down-with-hour`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">broken-down-with-hour</span> :: ((<span class="tok-variable">h</span> :: (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">0</span> <span class="tok-number">23</span> <span class="tok-constructor">I64</span>)) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">BrokenDown</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">BrokenDown</span>))</code></pre>

[Source](/reference-sources/Text.Time.Strtime.cinch)

<a id="value-62726f6b656e2d646f776e2d776974682d686f75723132"></a>

## `broken-down-with-hour12`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">broken-down-with-hour12</span> :: ((<span class="tok-variable">h</span> :: (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">1</span> <span class="tok-number">12</span> <span class="tok-constructor">I64</span>)) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">BrokenDown</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">BrokenDown</span>))</code></pre>

[Source](/reference-sources/Text.Time.Strtime.cinch)

<a id="value-62726f6b656e2d646f776e2d776974682d69736f2d7765656b"></a>

## `broken-down-with-iso-week`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">broken-down-with-iso-week</span> :: ((<span class="tok-variable">w</span> :: (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">1</span> <span class="tok-number">53</span> <span class="tok-constructor">I64</span>)) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">BrokenDown</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">BrokenDown</span>))</code></pre>

[Source](/reference-sources/Text.Time.Strtime.cinch)

<a id="value-62726f6b656e2d646f776e2d776974682d69736f2d7765656b2d79656172"></a>

## `broken-down-with-iso-week-year`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">broken-down-with-iso-week-year</span> :: ((<span class="tok-variable">y</span> :: (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">-9999</span> <span class="tok-number">9999</span> <span class="tok-constructor">I64</span>)) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">BrokenDown</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">BrokenDown</span>))</code></pre>

[Source](/reference-sources/Text.Time.Strtime.cinch)

<a id="value-62726f6b656e2d646f776e2d776974682d6d6572696469656d"></a>

## `broken-down-with-meridiem`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">broken-down-with-meridiem</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">Meridiem</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">BrokenDown</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">BrokenDown</span>))</code></pre>

[Source](/reference-sources/Text.Time.Strtime.cinch)

<a id="value-62726f6b656e2d646f776e2d776974682d6d696e757465"></a>

## `broken-down-with-minute`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">broken-down-with-minute</span> :: ((<span class="tok-variable">m</span> :: (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">0</span> <span class="tok-number">59</span> <span class="tok-constructor">I64</span>)) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">BrokenDown</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">BrokenDown</span>))</code></pre>

[Source](/reference-sources/Text.Time.Strtime.cinch)

<a id="value-62726f6b656e2d646f776e2d776974682d6d6f6e7468"></a>

## `broken-down-with-month`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">broken-down-with-month</span> :: ((<span class="tok-variable">m</span> :: (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">1</span> <span class="tok-number">12</span> <span class="tok-constructor">I64</span>)) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">BrokenDown</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">BrokenDown</span>))</code></pre>

[Source](/reference-sources/Text.Time.Strtime.cinch)

<a id="value-62726f6b656e2d646f776e2d776974682d6f6666736574"></a>

## `broken-down-with-offset`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">broken-down-with-offset</span> :: ((<span class="tok-variable">off</span> :: (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">-93599</span> <span class="tok-number">93599</span> <span class="tok-constructor">I64</span>)) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">BrokenDown</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">BrokenDown</span>))</code></pre>

[Source](/reference-sources/Text.Time.Strtime.cinch)

<a id="value-62726f6b656e2d646f776e2d776974682d7365636f6e64"></a>

## `broken-down-with-second`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">broken-down-with-second</span> :: ((<span class="tok-variable">s</span> :: (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">0</span> <span class="tok-number">59</span> <span class="tok-constructor">I64</span>)) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">BrokenDown</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">BrokenDown</span>))</code></pre>

[Source](/reference-sources/Text.Time.Strtime.cinch)

<a id="value-62726f6b656e2d646f776e2d776974682d7375627365632d6e616e6f7365636f6e64"></a>

## `broken-down-with-subsec-nanosecond`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">broken-down-with-subsec-nanosecond</span> :: ((<span class="tok-variable">ns</span> :: (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">0</span> <span class="tok-number">999999999</span> <span class="tok-constructor">I64</span>)) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">BrokenDown</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">BrokenDown</span>))</code></pre>

[Source](/reference-sources/Text.Time.Strtime.cinch)

<a id="value-62726f6b656e2d646f776e2d776974682d756e69782d7365636f6e6473"></a>

## `broken-down-with-unix-seconds`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">broken-down-with-unix-seconds</span> :: ((<span class="tok-variable">s</span> :: (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">-377705023201</span> <span class="tok-number">253402207200</span> <span class="tok-constructor">I64</span>)) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">BrokenDown</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">BrokenDown</span>))</code></pre>

The `Timestamp` range in whole seconds.

[Source](/reference-sources/Text.Time.Strtime.cinch) · line 467

<a id="value-62726f6b656e2d646f776e2d776974682d7765656b2d6d6f6e646179"></a>

## `broken-down-with-week-monday`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">broken-down-with-week-monday</span> :: ((<span class="tok-variable">w</span> :: (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">0</span> <span class="tok-number">53</span> <span class="tok-constructor">I64</span>)) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">BrokenDown</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">BrokenDown</span>))</code></pre>

[Source](/reference-sources/Text.Time.Strtime.cinch)

<a id="value-62726f6b656e2d646f776e2d776974682d7765656b2d73756e646179"></a>

## `broken-down-with-week-sunday`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">broken-down-with-week-sunday</span> :: ((<span class="tok-variable">w</span> :: (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">0</span> <span class="tok-number">53</span> <span class="tok-constructor">I64</span>)) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">BrokenDown</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">BrokenDown</span>))</code></pre>

[Source](/reference-sources/Text.Time.Strtime.cinch)

<a id="value-62726f6b656e2d646f776e2d776974682d7765656b646179"></a>

## `broken-down-with-weekday`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">broken-down-with-weekday</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Weekday</span>.<span class="tok-constructor">Weekday</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">BrokenDown</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">BrokenDown</span>))</code></pre>

[Source](/reference-sources/Text.Time.Strtime.cinch)

<a id="value-62726f6b656e2d646f776e2d776974682d79656172"></a>

## `broken-down-with-year`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">broken-down-with-year</span> :: ((<span class="tok-variable">y</span> :: (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">-9999</span> <span class="tok-number">9999</span> <span class="tok-constructor">I64</span>)) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">BrokenDown</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">BrokenDown</span>))</code></pre>

Setters: the range is the argument's type, so a literal checks at
compile time and no setter can store a value outside its field.

[Source](/reference-sources/Text.Time.Strtime.cinch) · line 416

<a id="value-62726f6b656e2d646f776e2d776974682d7a6f6e652d616262726576696174696f6e"></a>

## `broken-down-with-zone-abbreviation`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">broken-down-with-zone-abbreviation</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">BrokenDown</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">BrokenDown</span>))</code></pre>

[Source](/reference-sources/Text.Time.Strtime.cinch)

<a id="value-62726f6b656e2d646f776e2d776974682d7a6f6e652d6e616d65"></a>

## `broken-down-with-zone-name`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">broken-down-with-zone-name</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">BrokenDown</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">BrokenDown</span>))</code></pre>

[Source](/reference-sources/Text.Time.Strtime.cinch)

<a id="value-62726f6b656e2d646f776e2d79656172"></a>

## `broken-down-year`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">broken-down-year</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">BrokenDown</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">I64</span>))</code></pre>

Getters: `None` for a field that is not set. The hour, twelve-hour
hour and meridiem are read as stored (not derived from each other).

[Source](/reference-sources/Text.Time.Strtime.cinch) · line 349

<a id="value-62726f6b656e2d646f776e2d7a6f6e652d616262726576696174696f6e"></a>

## `broken-down-zone-abbreviation`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">broken-down-zone-abbreviation</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">BrokenDown</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

[Source](/reference-sources/Text.Time.Strtime.cinch)

<a id="value-62726f6b656e2d646f776e2d7a6f6e652d6e616d65"></a>

## `broken-down-zone-name`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">broken-down-zone-name</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">BrokenDown</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

[Source](/reference-sources/Text.Time.Strtime.cinch)

<a id="value-646174652d7374726674696d65"></a>

## `date-strftime`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">date-strftime</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Date</span>.<span class="tok-constructor">Date</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">StrtimeError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)))</code></pre>

Convenience.

[Source](/reference-sources/Text.Time.Strtime.cinch) · line 1442

<a id="value-646174652d7374727074696d65"></a>

## `date-strptime`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">date-strptime</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">StrtimeError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Date</span>.<span class="tok-constructor">Date</span>)))</code></pre>

[Source](/reference-sources/Text.Time.Strtime.cinch)

<a id="value-6461746574696d652d7374726674696d65"></a>

## `datetime-strftime`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">datetime-strftime</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">StrtimeError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)))</code></pre>

[Source](/reference-sources/Text.Time.Strtime.cinch)

<a id="value-6461746574696d652d7374727074696d65"></a>

## `datetime-strptime`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">datetime-strptime</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">StrtimeError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span>)))</code></pre>

[Source](/reference-sources/Text.Time.Strtime.cinch)

<a id="value-7374726674696d65"></a>

## `strftime`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">strftime</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">BrokenDown</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">StrtimeError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)))</code></pre>

Formatting.

[Source](/reference-sources/Text.Time.Strtime.cinch) · line 820

<a id="value-7374726674696d652d666f726d6174"></a>

## `strftime-format`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">strftime-format</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">Format</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">BrokenDown</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">StrtimeError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)))</code></pre>

[Source](/reference-sources/Text.Time.Strtime.cinch)

<a id="value-7374727074696d65"></a>

## `strptime`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">strptime</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">StrtimeError</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">BrokenDown</span>)))</code></pre>

Parsing. `parse-items` is the driver state; each `p-*` state reads
one conversion at `i`, stores its field and returns to the driver
with the rest of the format.

[Source](/reference-sources/Text.Time.Strtime.cinch) · line 1211

<a id="value-7374727074696d652d666f726d6174"></a>

## `strptime-format`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">strptime-format</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">Format</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">StrtimeError</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">BrokenDown</span>)))</code></pre>

[Source](/reference-sources/Text.Time.Strtime.cinch)

<a id="value-73747274696d652d666f726d61742d636f6d70696c65"></a>

## `strtime-format-compile`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">strtime-format-compile</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">StrtimeError</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">Format</span>))</code></pre>

Compiles a format once; every format refusal is reported here.

[Source](/reference-sources/Text.Time.Strtime.cinch) · line 695

<a id="value-74696d652d7374726674696d65"></a>

## `time-strftime`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">time-strftime</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">TimeOfDay</span>.<span class="tok-constructor">TimeOfDay</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">StrtimeError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)))</code></pre>

[Source](/reference-sources/Text.Time.Strtime.cinch)

<a id="value-74696d652d7374727074696d65"></a>

## `time-strptime`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">time-strptime</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">StrtimeError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">TimeOfDay</span>.<span class="tok-constructor">TimeOfDay</span>)))</code></pre>

[Source](/reference-sources/Text.Time.Strtime.cinch)

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Text/Time/Strtime.cinch`. Source SHA-256: `f38085e1b9841abb2a8cba49b53f7b1cacb1d544ce5d34ab1c06bfdfda12b20f`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
