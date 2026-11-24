# Text.Time.Duration

ISO 8601 and friendly duration text for spans and exact durations, printing and parsing.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Text.Time.Duration`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.Time.Duration.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Algebra.Arithmetic](Algebra.Arithmetic.md), [Data.Bytes](Data.Bytes.md), [Data.Machine](Data.Machine.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Data.Time.Duration](Data.Time.Duration.md), [Data.Time.Span](Data.Time.Span.md), [Data.Time.Unit](Data.Time.Unit.md), [Data.Wide](Data.Wide.md), [Relation.Eq](Relation.Eq.md), [Relation.Ord](Relation.Ord.md), [Text.Time.Scan](Text.Time.Scan.md).

Non-embedded source closure: [Data.Time.Date](Data.Time.Date.md), [Data.Time.DateTime](Data.Time.DateTime.md), [Data.Time.Duration](Data.Time.Duration.md), [Data.Time.Span](Data.Time.Span.md), [Data.Time.TimeOfDay](Data.Time.TimeOfDay.md), [Data.Time.Unit](Data.Time.Unit.md), [Data.Time.Weekday](Data.Time.Weekday.md), [Text.Time.Duration](Text.Time.Duration.md), [Text.Time.Scan](Text.Time.Scan.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Text.Time.Duration: ISO 8601 durations and Jiff's "friendly"
duration format, printing and parsing, for `Span` (Data.Time.Span,
each unit kept) and `Duration` (Data.Time.Duration, an exact count
of nanoseconds).

THE ISO GRAMMAR (ISO 8601 as TC39 Temporal reads it; strict):

    duration = [sign] "P" (date [ "T" time ] | "T" time)
    sign     = "+" | "-"                       ; of the whole duration
    date     = [n "Y"] [n "M"] [n "W"] [n "D"]  ; at least one
    time     = [n "H"] [n "M"] [n "S"]          ; at least one
    n        = 1*38DIGIT

and the LAST time component present (hours, minutes or seconds, and
only that one) may carry a fraction: ("." | ",") 1*9DIGIT. The
fraction is converted exactly into the smaller units: `PT1.5H` is 1
hour 30 minutes, `PT0.5M` is 30 seconds, `PT1.5S` is 1 second 500
milliseconds. Units appear at most once and in that order.

Strict means upper case designators only (`P`, `T`, `Y`, `M`, `W`,
`D`, `H`, `S`). Temporal and Jiff accept lower case; here that is
the `-lenient` parsers (T9: the default accepts exactly the grammar,
the lenient form has its own name), and the strict parsers refuse a
lower case designator with its own error,
`DurationTextLowercaseDesignator`.

THE FRIENDLY GRAMMAR (Jiff's `jiff::fmt::friendly`):

    friendly = [sign] units | units [ws "ago"]
    units    = clock | component *(sep component) [sep clock]
    component = n [fraction] *ws designator
    sep      = "," *ws | *ws                  ; a comma, or space, or nothing
    clock    = n ":" n ":" n [fraction]        ; hours, minutes, seconds
    fraction = ("." | ",") 1*9DIGIT
    ws       = space, tab, LF, VT, FF, CR

Components are in strictly descending unit order, each unit at most
once. Designators (lower case only):

    years         y yr yrs year years
    months        mo mos month months
    weeks         w wk wks week weeks
    days          d day days
    hours         h hr hrs hour hours
    minutes       m min mins minute minutes
    seconds       s sec secs second seconds
    milliseconds  ms msec msecs milli millis millisecond milliseconds
    microseconds  us usec usecs micro micros microsecond microseconds
                  and `s` after U+00B5 MICRO SIGN or U+03BC GREEK
                  SMALL LETTER MU (their UTF-8 bytes)
    nanoseconds   ns nsec nsecs nano nanos nanosecond nanoseconds

A fraction is allowed only on the last component and only on hours,
minutes, seconds, milliseconds (at most 6 digits) and microseconds
(at most 3 digits), so that it is always a whole number of
nanoseconds; it is converted exactly into the smaller units
(`0.0021s` is 2 milliseconds 100 microseconds). The clock form
stands in place of the hour, minute and second components and may
only follow days or larger (`1 day, 01:01:01.1`); its three numbers
are plain unit values (any number of digits, not limited to 59).
` ago` negates; a sign prefix and ` ago` together are refused.

Design law shape.
  - LOSSLESS FRIENDLY. For every span `s`, every designator style,
    every spacing, with or without the comma, and every sign style,
    `span-from-friendly-string (span-print-friendly p s)` is `Ok` of
    a span fieldwise equal to `s` (each non-zero unit is printed
    with its own designator: 2000 milliseconds is `2000ms`, never
    `2s`), provided the printer has neither a fractional unit nor
    the clock form set. Those two options fold smaller units into a
    fraction and so keep the length but not the breakdown (and a
    fraction of an hour or a minute is cut at nine digits).
  - LOSSY ISO. ISO 8601 has no designator below seconds, so
    `span-to-iso-string` folds milliseconds, microseconds and
    nanoseconds into the seconds field as a fraction: 2000
    milliseconds prints `PT2S`, which parses as 2 seconds. THE UNIT
    BREAKDOWN BELOW HOURS-MINUTES-SECONDS IS LOST; the length is
    not. Years, months, weeks, days, hours and minutes come back
    fieldwise, and for a clock span the parsed span `span-compare`s
    `EQ` with the original. One more limit: the folded seconds can
    exceed the seconds limit of a span (three sub-second units near
    their limits add up to about three times it); such a span still
    prints, and the parser refuses the text with
    `DurationTextUnitLimit`.
  - EXACT DURATIONS. For every `Duration` `d`, in both formats and
    under every printer option that does not truncate (no fixed
    precision; no fractional hour or minute), parsing the print is
    `Ok d`. A duration prints hours, minutes, seconds and (friendly)
    the non-zero sub-second units, never days: the largest duration,
    2^127 - 1 nanoseconds, prints
    `PT47261439850130342147690917H41M55.884105727S` and round trips,
    as does its negation, the smallest.
  - `span-from-string` and `duration-from-string` read either
    format, as Jiff's `FromStr` does: ISO when the first byte after
    an optional sign is `P` or `p`, friendly otherwise.
  - A parser for `Duration` or for `Span Clock` refuses every
    component of days or larger, whatever its value (`P0D`
    included), with `DurationTextCalendarUnit`: Jiff's "requires
    that the duration contain a time component and no components of
    days or greater".
  - Parsers are state machines over the input's bytes (the `st-*`
    and `fr-*` functions, one per state, named below); no `Char`, no
    list, and no slice is built. Printers run their writer twice,
    once to measure and once to fill a `Bytes` of exactly that
    length (a write outside a buffer is skipped by Text.Time.Scan,
    so the measuring run writes nothing).
  - Negative values print a leading `-` in ISO. The friendly
    printer's default sign style is Jiff's `Auto`: the suffix ` ago`
    when units are spaced, a `-` prefix with `SpacingNone` or the
    clock form.

No operation faults: every parser answers
`Result DurationTextError`, each constructor carrying the byte
position of the refusal, and every printer is total.

NAMED REFUSALS:
  - lower case ISO designators in the default parsers (above).
  - white space before or after either format, and anything else
    outside the grammar: `DurationTextTrailingInput` or the error of
    the state that met it.
  - a sign inside an ISO duration (`P-1D`): one sign, before `P`.
  - an ISO fraction on years, months, weeks or days, or on a
    component that is not the last (`PT1.5H30M`).
  - a friendly fraction finer than a nanosecond (seven or more
    digits on milliseconds, four or more on microseconds): Jiff
    truncates it; here it is `DurationTextTooManyFractionDigits`.
  - numbers of more than 38 digits (`DurationTextNumberTooLarge`):
    38 digits always fit an `I128`, which is what lets the largest
    `Duration` (26 digits of hours) be read back.
  - the friendly `ago` after a comma, and a comma before the clock
    form's ` ago`; white space is what precedes `ago`.
  - upper case or abbreviated-with-period friendly designators
    (`1 Hour`, `1 hr.`), and Jiff's `HumanTime` designator style and
    padding option: not offered.
  - white space between the sign and the first number.
  - a `+` for a positive zero under `SignForce`: zero prints
    unsigned.
  - rounding in the printer: a fixed precision truncates (rounding
    can carry into a larger unit; round the span first).
  - `Text.Builder` forms: as in Text.Time.Iso, the builder is not in
    the base package.
  - a wide argument late in a long parameter list: a state function
    of eleven parameters whose eighth was the `I128` number just
    read (seven word arguments, then the wide one, so it is passed
    on the stack at an odd word) compiled and then crashed at run
    time (a segmentation fault in the `tailcc` body). The three
    designator and clock states therefore take the number as their
    second or third parameter, where it travels in registers.
  - a positioned backstop: the per-unit limits are checked as each
    component is read (the table is Data.Time.Span's header; that
    module does not export it, so it is repeated in `limit-of`), and
    `span-from-units` is called once at the end; its `Err`, which
    the earlier checks make unreachable, would read as
    `DurationTextUnitLimit 0`.

</details>

# Types and constructors

<a id="type-4475726174696f6e546578744572726f72"></a>

## `DurationTextError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">DurationTextError</span></code></pre>

Every refusal, each with the byte position it was detected at.

[Source](/reference-sources/Text.Time.Duration.cinch) · line 202

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">DurationTextEmpty</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">DurationTextError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">DurationTextMissingP</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">DurationTextError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">DurationTextLowercaseDesignator</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">DurationTextError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">DurationTextMissingComponent</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">DurationTextError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">DurationTextExpectedNumber</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">DurationTextError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">DurationTextBadDesignator</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">DurationTextError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">DurationTextUnitsOutOfOrder</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">DurationTextError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">DurationTextRepeatedUnit</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">DurationTextError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">DurationTextFractionNotLast</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">DurationTextError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">DurationTextFractionOnUnit</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">DurationTextError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">DurationTextTooManyFractionDigits</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">DurationTextError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">DurationTextEmptyFraction</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">DurationTextError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">DurationTextNumberTooLarge</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">DurationTextError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">DurationTextUnitLimit</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">DurationTextError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">DurationTextTimeWithoutComponent</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">DurationTextError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">DurationTextSignTwice</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">DurationTextError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">DurationTextTrailingInput</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">DurationTextError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">DurationTextCalendarUnit</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">DurationTextError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">DurationTextExpectedColon</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">DurationTextError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">DurationTextOutOfRange</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">DurationTextError</span>)</code></pre>

<a id="type-467269656e646c7944657369676e61746f72"></a>

## `FriendlyDesignator`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">FriendlyDesignator</span></code></pre>

How units are named. Compact: `y mo w d h m s ms us ns`. Short:
`yr mo wk day hr min sec msec usec nsec`, with `s` added for any
value but 1. Verbose: `year month week day hour minute second
millisecond microsecond nanosecond`, likewise.

[Source](/reference-sources/Text.Time.Duration.cinch) · line 1246

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">DesignatorCompact</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">FriendlyDesignator</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">DesignatorShort</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">FriendlyDesignator</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">DesignatorVerbose</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">FriendlyDesignator</span></code></pre>

<a id="type-467269656e646c794672616374696f6e616c"></a>

## `FriendlyFractional`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">FriendlyFractional</span></code></pre>

The unit, if any, that the smaller units fold into as a fraction.

[Source](/reference-sources/Text.Time.Duration.cinch) · line 1269

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">FractionalNone</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">FriendlyFractional</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">FractionalHour</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">FriendlyFractional</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">FractionalMinute</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">FriendlyFractional</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">FractionalSecond</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">FriendlyFractional</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">FractionalMillisecond</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">FriendlyFractional</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">FractionalMicrosecond</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">FriendlyFractional</span></code></pre>

<a id="type-467269656e646c795072696e746572"></a>

## `FriendlyPrinter`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">FriendlyPrinter</span></code></pre>

Designator style (0 compact, 1 short, 2 verbose); spacing (0, 1,
2); comma; sign style (0 auto, 1 prefix, 2 force, 3 suffix);
fractional unit's rank (-1 for none, else 4..=8); precision (-1 for
the shortest exact, else 0..=9); clock form; zero unit's rank;
Unicode micro sign.

[Source](/reference-sources/Text.Time.Duration.cinch) · line 1282

<a id="type-467269656e646c795369676e"></a>

## `FriendlySign`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">FriendlySign</span></code></pre>

How a negative value is marked. `SignAuto` (the default, Jiff's):
the suffix ` ago` when units are spaced, the prefix `-` with
`SpacingNone` or the clock form. `SignPrefix`: always `-`.
`SignForce`: `-`, and `+` on a positive value (zero is unsigned).
`SignSuffix`: always ` ago`.

[Source](/reference-sources/Text.Time.Duration.cinch) · line 1262

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SignAuto</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">FriendlySign</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SignPrefix</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">FriendlySign</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SignForce</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">FriendlySign</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SignSuffix</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">FriendlySign</span></code></pre>

<a id="type-467269656e646c7953706163696e67"></a>

## `FriendlySpacing`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">FriendlySpacing</span></code></pre>

`1y2mo`, `1y 2mo` (the default), `1 y 2 mo`.

[Source](/reference-sources/Text.Time.Duration.cinch) · line 1252

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SpacingNone</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">FriendlySpacing</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SpacingBetweenUnits</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">FriendlySpacing</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SpacingBetweenUnitsAndDesignators</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">FriendlySpacing</span></code></pre>

# Values

<a id="value-6475726174696f6e2d66726f6d2d667269656e646c792d737472696e67"></a>

## `duration-from-friendly-string`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">duration-from-friendly-string</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">DurationTextError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">Duration</span>))</code></pre>

A friendly duration of hours and smaller as an exact duration.

[Source](/reference-sources/Text.Time.Duration.cinch) · line 1217

<a id="value-6475726174696f6e2d66726f6d2d69736f2d737472696e67"></a>

## `duration-from-iso-string`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">duration-from-iso-string</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">DurationTextError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">Duration</span>))</code></pre>

An ISO 8601 duration of hours, minutes and seconds as an exact
duration. A component of days or larger is refused with
`DurationTextCalendarUnit`; a total beyond the range with
`DurationTextOutOfRange`.

[Source](/reference-sources/Text.Time.Duration.cinch) · line 818

<a id="value-6475726174696f6e2d66726f6d2d69736f2d737472696e672d6c656e69656e74"></a>

## `duration-from-iso-string-lenient`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">duration-from-iso-string-lenient</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">DurationTextError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">Duration</span>))</code></pre>

[Source](/reference-sources/Text.Time.Duration.cinch)

<a id="value-6475726174696f6e2d66726f6d2d737472696e67"></a>

## `duration-from-string`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">duration-from-string</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">DurationTextError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">Duration</span>))</code></pre>

[Source](/reference-sources/Text.Time.Duration.cinch)

<a id="value-6475726174696f6e2d7072696e742d667269656e646c79"></a>

## `duration-print-friendly`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">duration-print-friendly</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">FriendlyPrinter</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">Duration</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

A duration balanced into hours, minutes, seconds and the non-zero
sub-second units (`2h 30m`, `1s 500ms`), under the given options.

[Source](/reference-sources/Text.Time.Duration.cinch) · line 1555

<a id="value-6475726174696f6e2d746578742d6572726f722d706f736974696f6e"></a>

## `duration-text-error-position`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">duration-text-error-position</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">DurationTextError</span> -&gt; <span class="tok-constructor">I64</span>)</code></pre>

[Source](/reference-sources/Text.Time.Duration.cinch)

<a id="value-6475726174696f6e2d746f2d667269656e646c792d737472696e67"></a>

## `duration-to-friendly-string`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">duration-to-friendly-string</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">Duration</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

[Source](/reference-sources/Text.Time.Duration.cinch)

<a id="value-6475726174696f6e2d746f2d69736f2d737472696e67"></a>

## `duration-to-iso-string`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">duration-to-iso-string</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">Duration</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Hours, minutes and fractional seconds only (`PT5H30M`,
`PT0.000000001S`, `-PT1.5S`); never days. Exact.

[Source](/reference-sources/Text.Time.Duration.cinch) · line 518

<a id="value-667269656e646c792d7072696e7465722d64656661756c74"></a>

## `friendly-printer-default`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">friendly-printer-default</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">FriendlyPrinter</span></code></pre>

Compact designators, a space between units, no comma, `SignAuto`,
no fractional unit, the shortest exact precision, no clock form,
zero printed in seconds, ASCII `us`.

[Source](/reference-sources/Text.Time.Duration.cinch) · line 1288

<a id="value-667269656e646c792d776974682d636c6f636b2d666f726d"></a>

## `friendly-with-clock-form`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">friendly-with-clock-form</span> :: (<span class="tok-constructor">Bool</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">FriendlyPrinter</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">FriendlyPrinter</span>))</code></pre>

Whether hours and smaller print as `HH:MM:SS[.fraction]` (at least
two digits each; the sub-second units fold into the fraction, so
the breakdown below seconds is lost). Takes precedence over a
fractional unit. Zero prints `00:00:00`.

[Source](/reference-sources/Text.Time.Duration.cinch) · line 1349

<a id="value-667269656e646c792d776974682d636f6d6d61"></a>

## `friendly-with-comma`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">friendly-with-comma</span> :: (<span class="tok-constructor">Bool</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">FriendlyPrinter</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">FriendlyPrinter</span>))</code></pre>

Whether a comma follows each designator but the last
(`1 year, 2 months`).

[Source](/reference-sources/Text.Time.Duration.cinch) · line 1307

<a id="value-667269656e646c792d776974682d64657369676e61746f72"></a>

## `friendly-with-designator`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">friendly-with-designator</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">FriendlyDesignator</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">FriendlyPrinter</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">FriendlyPrinter</span>))</code></pre>

[Source](/reference-sources/Text.Time.Duration.cinch)

<a id="value-667269656e646c792d776974682d6672616374696f6e616c"></a>

## `friendly-with-fractional`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">friendly-with-fractional</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">FriendlyFractional</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">FriendlyPrinter</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">FriendlyPrinter</span>))</code></pre>

With a fractional unit, that unit and every smaller one print as
one number with a fraction of up to nine digits (1 hour 30 minutes
30 seconds under `FractionalMinute` is `1h 30.5m`), and a zero
value prints in that unit. Lossy for the breakdown; a fraction of
an hour or a minute is cut at nine digits.

[Source](/reference-sources/Text.Time.Duration.cinch) · line 1324

<a id="value-667269656e646c792d776974682d707265636973696f6e"></a>

## `friendly-with-precision`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">friendly-with-precision</span> :: ((<span class="tok-variable">digits</span> :: (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">0</span> <span class="tok-number">9</span> <span class="tok-constructor">I64</span>)) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">FriendlyPrinter</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">FriendlyPrinter</span>))</code></pre>

Exactly `digits` fraction digits wherever a fraction is printed
(the fractional unit, or the clock form's seconds), truncating.

[Source](/reference-sources/Text.Time.Duration.cinch) · line 1336

<a id="value-667269656e646c792d776974682d73686f72746573742d707265636973696f6e"></a>

## `friendly-with-shortest-precision`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">friendly-with-shortest-precision</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">FriendlyPrinter</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">FriendlyPrinter</span>)</code></pre>

The default: the shortest fraction that is exact in nine digits.

[Source](/reference-sources/Text.Time.Duration.cinch) · line 1341

<a id="value-667269656e646c792d776974682d7369676e"></a>

## `friendly-with-sign`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">friendly-with-sign</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">FriendlySign</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">FriendlyPrinter</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">FriendlyPrinter</span>))</code></pre>

[Source](/reference-sources/Text.Time.Duration.cinch)

<a id="value-667269656e646c792d776974682d73706163696e67"></a>

## `friendly-with-spacing`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">friendly-with-spacing</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">FriendlySpacing</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">FriendlyPrinter</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">FriendlyPrinter</span>))</code></pre>

[Source](/reference-sources/Text.Time.Duration.cinch)

<a id="value-667269656e646c792d776974682d756e69636f64652d6d6963726f"></a>

## `friendly-with-unicode-micro`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">friendly-with-unicode-micro</span> :: (<span class="tok-constructor">Bool</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">FriendlyPrinter</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">FriendlyPrinter</span>))</code></pre>

Whether the compact microsecond designator is `s` after U+00B5
MICRO SIGN (UTF-8) in place of the ASCII `us`.

[Source](/reference-sources/Text.Time.Duration.cinch) · line 1362

<a id="value-667269656e646c792d776974682d7a65726f2d756e6974"></a>

## `friendly-with-zero-unit`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">friendly-with-zero-unit</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Unit</span>.<span class="tok-constructor">TimeUnit</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">FriendlyPrinter</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">FriendlyPrinter</span>))</code></pre>

The unit a zero value prints in (`0s` by default). A `Duration`
printed with a zero unit of days or larger prints `0s`, so that it
parses back as a `Duration`.

[Source](/reference-sources/Text.Time.Duration.cinch) · line 1356

<a id="value-7370616e2d636c6f636b2d66726f6d2d667269656e646c792d737472696e67"></a>

## `span-clock-from-friendly-string`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">span-clock-from-friendly-string</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">DurationTextError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Clock</span>))</code></pre>

The same as a clock span: a component of days or larger is refused
with `DurationTextCalendarUnit`.

[Source](/reference-sources/Text.Time.Duration.cinch) · line 1213

<a id="value-7370616e2d636c6f636b2d66726f6d2d69736f2d737472696e67"></a>

## `span-clock-from-iso-string`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">span-clock-from-iso-string</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">DurationTextError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Clock</span>))</code></pre>

The same as a clock span: a component of days or larger is refused
with `DurationTextCalendarUnit`.

[Source](/reference-sources/Text.Time.Duration.cinch) · line 811

<a id="value-7370616e2d636c6f636b2d66726f6d2d737472696e67"></a>

## `span-clock-from-string`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">span-clock-from-string</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">DurationTextError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Clock</span>))</code></pre>

[Source](/reference-sources/Text.Time.Duration.cinch)

<a id="value-7370616e2d66726f6d2d667269656e646c792d737472696e67"></a>

## `span-from-friendly-string`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">span-from-friendly-string</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">DurationTextError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Calendar</span>))</code></pre>

A friendly duration as a span, each unit as written (`1.5h` is 1
hour 30 minutes; see the header's grammar).

[Source](/reference-sources/Text.Time.Duration.cinch) · line 1208

<a id="value-7370616e2d66726f6d2d69736f2d737472696e67"></a>

## `span-from-iso-string`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">span-from-iso-string</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">DurationTextError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Calendar</span>))</code></pre>

An ISO 8601 duration as a span, each unit as written (strict: the
grammar in the header, upper case designators).

[Source](/reference-sources/Text.Time.Duration.cinch) · line 801

<a id="value-7370616e2d66726f6d2d69736f2d737472696e672d6c656e69656e74"></a>

## `span-from-iso-string-lenient`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">span-from-iso-string-lenient</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">DurationTextError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Calendar</span>))</code></pre>

Also accepts lower case designators (`p1y2m3dt4h`), as Temporal and
Jiff do.

[Source](/reference-sources/Text.Time.Duration.cinch) · line 806

<a id="value-7370616e2d66726f6d2d737472696e67"></a>

## `span-from-string`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">span-from-string</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">DurationTextError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Calendar</span>))</code></pre>

Either format (strict ISO when the first byte after an optional
sign is `P` or `p`, friendly otherwise), as Jiff's `FromStr`.

[Source](/reference-sources/Text.Time.Duration.cinch) · line 1222

<a id="value-7370616e2d7072696e742d667269656e646c79"></a>

## `span-print-friendly`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">span-print-friendly</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">c</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanClass</span>) . (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">FriendlyPrinter</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-variable">c</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

The friendly form under the given options.

[Source](/reference-sources/Text.Time.Duration.cinch) · line 1541

<a id="value-7370616e2d746f2d667269656e646c792d737472696e67"></a>

## `span-to-friendly-string`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">span-to-friendly-string</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">c</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanClass</span>) . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-variable">c</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

The friendly form with the default options. LOSSLESS: every
non-zero unit with its own compact designator, a space between
units (`6y 10mo 3d 23h`, `2000ms`), `0s` for the zero span, and
` ago` after a negative one.

[Source](/reference-sources/Text.Time.Duration.cinch) · line 1550

<a id="value-7370616e2d746f2d69736f2d737472696e67"></a>

## `span-to-iso-string`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">span-to-iso-string</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">c</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanClass</span>) . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-variable">c</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

The ISO 8601 form. LOSSY for the breakdown below seconds:
milliseconds, microseconds and nanoseconds fold into fractional
seconds (2000 milliseconds prints `PT2S`); see the header. The
zero span prints `PT0S`; a negative span has a leading `-`.

[Source](/reference-sources/Text.Time.Duration.cinch) · line 511

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Text/Time/Duration.cinch`. Source SHA-256: `20fbc027b1c3295d2ad1d3ede0c48b955604990b27f2ab1d7da45334877cb7c8`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
