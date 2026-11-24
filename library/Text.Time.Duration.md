# Text.Time.Duration

ISO 8601 and friendly duration text for spans and exact durations, printing and parsing.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--source">Ship source</span> <code>import Text.Time.Duration</code></div><div class="doc-mod-bar-links"><a href="/library/source/Text.Time.Duration/">View source</a><a href="/library/search/?q=%2BText.Time.Duration">Search module</a><a href="/reference-sources/Text.Time.Duration.cinch" download="Text.Time.Duration.cinch">Download .cinch</a><a href="/library/00-index/">All modules</a></div></div>

**Dependencies:** [Algebra.Arithmetic](Algebra.Arithmetic.md), [Data.Bytes](Data.Bytes.md), [Data.Machine](Data.Machine.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Data.Time.Duration](Data.Time.Duration.md), [Data.Time.Span](Data.Time.Span.md), [Data.Time.Unit](Data.Time.Unit.md), [Data.Wide](Data.Wide.md), [Relation.Eq](Relation.Eq.md), [Relation.Ord](Relation.Ord.md), [Text.Time.Scan](Text.Time.Scan.md).

<details class="doc-closure"><summary>Ship-source closure (9 files)</summary>

Copy these modules beside your entry file before importing `Text.Time.Duration`: [Data.Time.Date](Data.Time.Date.md), [Data.Time.DateTime](Data.Time.DateTime.md), [Data.Time.Duration](Data.Time.Duration.md), [Data.Time.Span](Data.Time.Span.md), [Data.Time.TimeOfDay](Data.Time.TimeOfDay.md), [Data.Time.Unit](Data.Time.Unit.md), [Data.Time.Weekday](Data.Time.Weekday.md), [Text.Time.Duration](Text.Time.Duration.md), [Text.Time.Scan](Text.Time.Scan.md).

```sh
cp seed/prelude/Data/Time/Date.cinch ./Data.Time.Date.cinch
cp seed/prelude/Data/Time/DateTime.cinch ./Data.Time.DateTime.cinch
cp seed/prelude/Data/Time/Duration.cinch ./Data.Time.Duration.cinch
cp seed/prelude/Data/Time/Span.cinch ./Data.Time.Span.cinch
cp seed/prelude/Data/Time/TimeOfDay.cinch ./Data.Time.TimeOfDay.cinch
cp seed/prelude/Data/Time/Unit.cinch ./Data.Time.Unit.cinch
cp seed/prelude/Data/Time/Weekday.cinch ./Data.Time.Weekday.cinch
cp seed/prelude/Text/Time/Duration.cinch ./Text.Time.Duration.cinch
cp seed/prelude/Text/Time/Scan.cinch ./Text.Time.Scan.cinch
```

</details>

<details class="doc-notes"><summary>Module notes</summary>

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

<details class="doc-synopsis"><summary>Synopsis (35 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-DurationTextError">DurationTextError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">DurationTextError</strong></code></li><li><a class="syn-name" href="#t-FriendlyDesignator">FriendlyDesignator</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">FriendlyDesignator</strong></code></li><li><a class="syn-name" href="#t-FriendlyFractional">FriendlyFractional</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">FriendlyFractional</strong></code></li><li><a class="syn-name" href="#t-FriendlyPrinter">FriendlyPrinter</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">FriendlyPrinter</strong></code></li><li><a class="syn-name" href="#t-FriendlySign">FriendlySign</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">FriendlySign</strong></code></li><li><a class="syn-name" href="#t-FriendlySpacing">FriendlySpacing</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">FriendlySpacing</strong></code></li><li><a class="syn-name" href="#v-duration-from-friendly-string">duration-from-friendly-string</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">duration-from-friendly-string</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-DurationTextError" title="Text.Time.Duration.DurationTextError">DurationTextError</a> <a class="doc-type-link tok-constructor" href="/library/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a></code></li><li><a class="syn-name" href="#v-duration-from-iso-string">duration-from-iso-string</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">duration-from-iso-string</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-DurationTextError" title="Text.Time.Duration.DurationTextError">DurationTextError</a> <a class="doc-type-link tok-constructor" href="/library/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a></code></li><li><a class="syn-name" href="#v-duration-from-iso-string-lenient">duration-from-iso-string-lenient</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">duration-from-iso-string-lenient</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-DurationTextError" title="Text.Time.Duration.DurationTextError">DurationTextError</a> <a class="doc-type-link tok-constructor" href="/library/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a></code></li><li><a class="syn-name" href="#v-duration-from-string">duration-from-string</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">duration-from-string</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-DurationTextError" title="Text.Time.Duration.DurationTextError">DurationTextError</a> <a class="doc-type-link tok-constructor" href="/library/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a></code></li><li><a class="syn-name" href="#v-duration-print-friendly">duration-print-friendly</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">duration-print-friendly</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-FriendlyPrinter" title="Text.Time.Duration.FriendlyPrinter">FriendlyPrinter</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-duration-text-error-position">duration-text-error-position</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">duration-text-error-position</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-DurationTextError" title="Text.Time.Duration.DurationTextError">DurationTextError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-duration-to-friendly-string">duration-to-friendly-string</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">duration-to-friendly-string</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-duration-to-iso-string">duration-to-iso-string</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">duration-to-iso-string</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-friendly-printer-default">friendly-printer-default</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">friendly-printer-default</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-FriendlyPrinter" title="Text.Time.Duration.FriendlyPrinter">FriendlyPrinter</a></code></li><li><a class="syn-name" href="#v-friendly-with-clock-form">friendly-with-clock-form</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">friendly-with-clock-form</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-FriendlyPrinter" title="Text.Time.Duration.FriendlyPrinter">FriendlyPrinter</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-FriendlyPrinter" title="Text.Time.Duration.FriendlyPrinter">FriendlyPrinter</a></code></li><li><a class="syn-name" href="#v-friendly-with-comma">friendly-with-comma</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">friendly-with-comma</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-FriendlyPrinter" title="Text.Time.Duration.FriendlyPrinter">FriendlyPrinter</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-FriendlyPrinter" title="Text.Time.Duration.FriendlyPrinter">FriendlyPrinter</a></code></li><li><a class="syn-name" href="#v-friendly-with-designator">friendly-with-designator</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">friendly-with-designator</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-FriendlyDesignator" title="Text.Time.Duration.FriendlyDesignator">FriendlyDesignator</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-FriendlyPrinter" title="Text.Time.Duration.FriendlyPrinter">FriendlyPrinter</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-FriendlyPrinter" title="Text.Time.Duration.FriendlyPrinter">FriendlyPrinter</a></code></li><li><a class="syn-name" href="#v-friendly-with-fractional">friendly-with-fractional</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">friendly-with-fractional</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-FriendlyFractional" title="Text.Time.Duration.FriendlyFractional">FriendlyFractional</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-FriendlyPrinter" title="Text.Time.Duration.FriendlyPrinter">FriendlyPrinter</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-FriendlyPrinter" title="Text.Time.Duration.FriendlyPrinter">FriendlyPrinter</a></code></li><li><a class="syn-name" href="#v-friendly-with-precision">friendly-with-precision</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">friendly-with-precision</strong> :: (<span class="tok-variable">digits</span> :: <a class="doc-type-link tok-constructor" href="/library/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 9 <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-FriendlyPrinter" title="Text.Time.Duration.FriendlyPrinter">FriendlyPrinter</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-FriendlyPrinter" title="Text.Time.Duration.FriendlyPrinter">FriendlyPrinter</a></code></li><li><a class="syn-name" href="#v-friendly-with-shortest-precision">friendly-with-shortest-precision</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">friendly-with-shortest-precision</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-FriendlyPrinter" title="Text.Time.Duration.FriendlyPrinter">FriendlyPrinter</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-FriendlyPrinter" title="Text.Time.Duration.FriendlyPrinter">FriendlyPrinter</a></code></li><li><a class="syn-name" href="#v-friendly-with-sign">friendly-with-sign</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">friendly-with-sign</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-FriendlySign" title="Text.Time.Duration.FriendlySign">FriendlySign</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-FriendlyPrinter" title="Text.Time.Duration.FriendlyPrinter">FriendlyPrinter</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-FriendlyPrinter" title="Text.Time.Duration.FriendlyPrinter">FriendlyPrinter</a></code></li><li><a class="syn-name" href="#v-friendly-with-spacing">friendly-with-spacing</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">friendly-with-spacing</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-FriendlySpacing" title="Text.Time.Duration.FriendlySpacing">FriendlySpacing</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-FriendlyPrinter" title="Text.Time.Duration.FriendlyPrinter">FriendlyPrinter</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-FriendlyPrinter" title="Text.Time.Duration.FriendlyPrinter">FriendlyPrinter</a></code></li><li><a class="syn-name" href="#v-friendly-with-unicode-micro">friendly-with-unicode-micro</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">friendly-with-unicode-micro</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-FriendlyPrinter" title="Text.Time.Duration.FriendlyPrinter">FriendlyPrinter</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-FriendlyPrinter" title="Text.Time.Duration.FriendlyPrinter">FriendlyPrinter</a></code></li><li><a class="syn-name" href="#v-friendly-with-zero-unit">friendly-with-zero-unit</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">friendly-with-zero-unit</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Time.Unit/#t-TimeUnit" title="Data.Time.Unit.TimeUnit">TimeUnit</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-FriendlyPrinter" title="Text.Time.Duration.FriendlyPrinter">FriendlyPrinter</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-FriendlyPrinter" title="Text.Time.Duration.FriendlyPrinter">FriendlyPrinter</a></code></li><li><a class="syn-name" href="#v-span-clock-from-friendly-string">span-clock-from-friendly-string</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">span-clock-from-friendly-string</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-DurationTextError" title="Text.Time.Duration.DurationTextError">DurationTextError</a> (<a class="doc-type-link tok-constructor" href="/library/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-constructor">Clock</span>)</code></li><li><a class="syn-name" href="#v-span-clock-from-iso-string">span-clock-from-iso-string</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">span-clock-from-iso-string</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-DurationTextError" title="Text.Time.Duration.DurationTextError">DurationTextError</a> (<a class="doc-type-link tok-constructor" href="/library/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-constructor">Clock</span>)</code></li><li><a class="syn-name" href="#v-span-clock-from-string">span-clock-from-string</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">span-clock-from-string</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-DurationTextError" title="Text.Time.Duration.DurationTextError">DurationTextError</a> (<a class="doc-type-link tok-constructor" href="/library/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-constructor">Clock</span>)</code></li><li><a class="syn-name" href="#v-span-from-friendly-string">span-from-friendly-string</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">span-from-friendly-string</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-DurationTextError" title="Text.Time.Duration.DurationTextError">DurationTextError</a> (<a class="doc-type-link tok-constructor" href="/library/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-constructor">Calendar</span>)</code></li><li><a class="syn-name" href="#v-span-from-iso-string">span-from-iso-string</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">span-from-iso-string</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-DurationTextError" title="Text.Time.Duration.DurationTextError">DurationTextError</a> (<a class="doc-type-link tok-constructor" href="/library/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-constructor">Calendar</span>)</code></li><li><a class="syn-name" href="#v-span-from-iso-string-lenient">span-from-iso-string-lenient</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">span-from-iso-string-lenient</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-DurationTextError" title="Text.Time.Duration.DurationTextError">DurationTextError</a> (<a class="doc-type-link tok-constructor" href="/library/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-constructor">Calendar</span>)</code></li><li><a class="syn-name" href="#v-span-from-string">span-from-string</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">span-from-string</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-DurationTextError" title="Text.Time.Duration.DurationTextError">DurationTextError</a> (<a class="doc-type-link tok-constructor" href="/library/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-constructor">Calendar</span>)</code></li><li><a class="syn-name" href="#v-span-print-friendly">span-print-friendly</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">span-print-friendly</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">c</span> :: <a class="doc-type-link tok-constructor" href="/library/Data.Time.Span/#t-SpanClass" title="Data.Time.Span.SpanClass">SpanClass</a>). <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-FriendlyPrinter" title="Text.Time.Duration.FriendlyPrinter">FriendlyPrinter</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-variable">c</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-span-to-friendly-string">span-to-friendly-string</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">span-to-friendly-string</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">c</span> :: <a class="doc-type-link tok-constructor" href="/library/Data.Time.Span/#t-SpanClass" title="Data.Time.Span.SpanClass">SpanClass</a>). <a class="doc-type-link tok-constructor" href="/library/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-variable">c</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-span-to-iso-string">span-to-iso-string</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">span-to-iso-string</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">c</span> :: <a class="doc-type-link tok-constructor" href="/library/Data.Time.Span/#t-SpanClass" title="Data.Time.Span.SpanClass">SpanClass</a>). <a class="doc-type-link tok-constructor" href="/library/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-variable">c</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-DurationTextError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">DurationTextError</strong></code><span class="doc-sig-links"><a href="#t-DurationTextError" title="Permalink">#</a><a href="/library/source/Text.Time.Duration/#L202" title="View source at line 202">Source</a></span></div>
<div class="doc-body">

Every refusal, each with the byte position it was detected at.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-DurationTextEmpty"><code><strong class="tok-constructor">DurationTextEmpty</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-DurationTextError" title="Text.Time.Duration.DurationTextError">DurationTextError</a></code></li><li id="k-DurationTextMissingP"><code><strong class="tok-constructor">DurationTextMissingP</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-DurationTextError" title="Text.Time.Duration.DurationTextError">DurationTextError</a></code></li><li id="k-DurationTextLowercaseDesignator"><code><strong class="tok-constructor">DurationTextLowercaseDesignator</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-DurationTextError" title="Text.Time.Duration.DurationTextError">DurationTextError</a></code></li><li id="k-DurationTextMissingComponent"><code><strong class="tok-constructor">DurationTextMissingComponent</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-DurationTextError" title="Text.Time.Duration.DurationTextError">DurationTextError</a></code></li><li id="k-DurationTextExpectedNumber"><code><strong class="tok-constructor">DurationTextExpectedNumber</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-DurationTextError" title="Text.Time.Duration.DurationTextError">DurationTextError</a></code></li><li id="k-DurationTextBadDesignator"><code><strong class="tok-constructor">DurationTextBadDesignator</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-DurationTextError" title="Text.Time.Duration.DurationTextError">DurationTextError</a></code></li><li id="k-DurationTextUnitsOutOfOrder"><code><strong class="tok-constructor">DurationTextUnitsOutOfOrder</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-DurationTextError" title="Text.Time.Duration.DurationTextError">DurationTextError</a></code></li><li id="k-DurationTextRepeatedUnit"><code><strong class="tok-constructor">DurationTextRepeatedUnit</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-DurationTextError" title="Text.Time.Duration.DurationTextError">DurationTextError</a></code></li><li id="k-DurationTextFractionNotLast"><code><strong class="tok-constructor">DurationTextFractionNotLast</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-DurationTextError" title="Text.Time.Duration.DurationTextError">DurationTextError</a></code></li><li id="k-DurationTextFractionOnUnit"><code><strong class="tok-constructor">DurationTextFractionOnUnit</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-DurationTextError" title="Text.Time.Duration.DurationTextError">DurationTextError</a></code></li><li id="k-DurationTextTooManyFractionDigits"><code><strong class="tok-constructor">DurationTextTooManyFractionDigits</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-DurationTextError" title="Text.Time.Duration.DurationTextError">DurationTextError</a></code></li><li id="k-DurationTextEmptyFraction"><code><strong class="tok-constructor">DurationTextEmptyFraction</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-DurationTextError" title="Text.Time.Duration.DurationTextError">DurationTextError</a></code></li><li id="k-DurationTextNumberTooLarge"><code><strong class="tok-constructor">DurationTextNumberTooLarge</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-DurationTextError" title="Text.Time.Duration.DurationTextError">DurationTextError</a></code></li><li id="k-DurationTextUnitLimit"><code><strong class="tok-constructor">DurationTextUnitLimit</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-DurationTextError" title="Text.Time.Duration.DurationTextError">DurationTextError</a></code></li><li id="k-DurationTextTimeWithoutComponent"><code><strong class="tok-constructor">DurationTextTimeWithoutComponent</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-DurationTextError" title="Text.Time.Duration.DurationTextError">DurationTextError</a></code></li><li id="k-DurationTextSignTwice"><code><strong class="tok-constructor">DurationTextSignTwice</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-DurationTextError" title="Text.Time.Duration.DurationTextError">DurationTextError</a></code></li><li id="k-DurationTextTrailingInput"><code><strong class="tok-constructor">DurationTextTrailingInput</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-DurationTextError" title="Text.Time.Duration.DurationTextError">DurationTextError</a></code></li><li id="k-DurationTextCalendarUnit"><code><strong class="tok-constructor">DurationTextCalendarUnit</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-DurationTextError" title="Text.Time.Duration.DurationTextError">DurationTextError</a></code></li><li id="k-DurationTextExpectedColon"><code><strong class="tok-constructor">DurationTextExpectedColon</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-DurationTextError" title="Text.Time.Duration.DurationTextError">DurationTextError</a></code></li><li id="k-DurationTextOutOfRange"><code><strong class="tok-constructor">DurationTextOutOfRange</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-DurationTextError" title="Text.Time.Duration.DurationTextError">DurationTextError</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-FriendlyDesignator">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">FriendlyDesignator</strong></code><span class="doc-sig-links"><a href="#t-FriendlyDesignator" title="Permalink">#</a><a href="/library/source/Text.Time.Duration/#L1246" title="View source at line 1246">Source</a></span></div>
<div class="doc-body">

How units are named. Compact: `y mo w d h m s ms us ns`. Short:
`yr mo wk day hr min sec msec usec nsec`, with `s` added for any
value but 1. Verbose: `year month week day hour minute second
millisecond microsecond nanosecond`, likewise.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-DesignatorCompact"><code><strong class="tok-constructor">DesignatorCompact</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-FriendlyDesignator" title="Text.Time.Duration.FriendlyDesignator">FriendlyDesignator</a></code></li><li id="k-DesignatorShort"><code><strong class="tok-constructor">DesignatorShort</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-FriendlyDesignator" title="Text.Time.Duration.FriendlyDesignator">FriendlyDesignator</a></code></li><li id="k-DesignatorVerbose"><code><strong class="tok-constructor">DesignatorVerbose</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-FriendlyDesignator" title="Text.Time.Duration.FriendlyDesignator">FriendlyDesignator</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-FriendlyFractional">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">FriendlyFractional</strong></code><span class="doc-sig-links"><a href="#t-FriendlyFractional" title="Permalink">#</a><a href="/library/source/Text.Time.Duration/#L1269" title="View source at line 1269">Source</a></span></div>
<div class="doc-body">

The unit, if any, that the smaller units fold into as a fraction.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-FractionalNone"><code><strong class="tok-constructor">FractionalNone</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-FriendlyFractional" title="Text.Time.Duration.FriendlyFractional">FriendlyFractional</a></code></li><li id="k-FractionalHour"><code><strong class="tok-constructor">FractionalHour</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-FriendlyFractional" title="Text.Time.Duration.FriendlyFractional">FriendlyFractional</a></code></li><li id="k-FractionalMinute"><code><strong class="tok-constructor">FractionalMinute</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-FriendlyFractional" title="Text.Time.Duration.FriendlyFractional">FriendlyFractional</a></code></li><li id="k-FractionalSecond"><code><strong class="tok-constructor">FractionalSecond</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-FriendlyFractional" title="Text.Time.Duration.FriendlyFractional">FriendlyFractional</a></code></li><li id="k-FractionalMillisecond"><code><strong class="tok-constructor">FractionalMillisecond</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-FriendlyFractional" title="Text.Time.Duration.FriendlyFractional">FriendlyFractional</a></code></li><li id="k-FractionalMicrosecond"><code><strong class="tok-constructor">FractionalMicrosecond</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-FriendlyFractional" title="Text.Time.Duration.FriendlyFractional">FriendlyFractional</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-FriendlyPrinter">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">FriendlyPrinter</strong></code><span class="doc-sig-links"><a href="#t-FriendlyPrinter" title="Permalink">#</a><a href="/library/source/Text.Time.Duration/#L1282" title="View source at line 1282">Source</a></span></div>
<div class="doc-body">

Designator style (0 compact, 1 short, 2 verbose); spacing (0, 1,
2); comma; sign style (0 auto, 1 prefix, 2 force, 3 suffix);
fractional unit's rank (-1 for none, else 4..=8); precision (-1 for
the shortest exact, else 0..=9); clock form; zero unit's rank;
Unicode micro sign.

</div>
</article>

<article class="doc-entry" id="t-FriendlySign">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">FriendlySign</strong></code><span class="doc-sig-links"><a href="#t-FriendlySign" title="Permalink">#</a><a href="/library/source/Text.Time.Duration/#L1262" title="View source at line 1262">Source</a></span></div>
<div class="doc-body">

How a negative value is marked. `SignAuto` (the default, Jiff's):
the suffix ` ago` when units are spaced, the prefix `-` with
`SpacingNone` or the clock form. `SignPrefix`: always `-`.
`SignForce`: `-`, and `+` on a positive value (zero is unsigned).
`SignSuffix`: always ` ago`.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-SignAuto"><code><strong class="tok-constructor">SignAuto</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-FriendlySign" title="Text.Time.Duration.FriendlySign">FriendlySign</a></code></li><li id="k-SignPrefix"><code><strong class="tok-constructor">SignPrefix</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-FriendlySign" title="Text.Time.Duration.FriendlySign">FriendlySign</a></code></li><li id="k-SignForce"><code><strong class="tok-constructor">SignForce</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-FriendlySign" title="Text.Time.Duration.FriendlySign">FriendlySign</a></code></li><li id="k-SignSuffix"><code><strong class="tok-constructor">SignSuffix</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-FriendlySign" title="Text.Time.Duration.FriendlySign">FriendlySign</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-FriendlySpacing">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">FriendlySpacing</strong></code><span class="doc-sig-links"><a href="#t-FriendlySpacing" title="Permalink">#</a><a href="/library/source/Text.Time.Duration/#L1252" title="View source at line 1252">Source</a></span></div>
<div class="doc-body">

`1y2mo`, `1y 2mo` (the default), `1 y 2 mo`.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-SpacingNone"><code><strong class="tok-constructor">SpacingNone</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-FriendlySpacing" title="Text.Time.Duration.FriendlySpacing">FriendlySpacing</a></code></li><li id="k-SpacingBetweenUnits"><code><strong class="tok-constructor">SpacingBetweenUnits</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-FriendlySpacing" title="Text.Time.Duration.FriendlySpacing">FriendlySpacing</a></code></li><li id="k-SpacingBetweenUnitsAndDesignators"><code><strong class="tok-constructor">SpacingBetweenUnitsAndDesignators</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-FriendlySpacing" title="Text.Time.Duration.FriendlySpacing">FriendlySpacing</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-duration-from-friendly-string">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">duration-from-friendly-string</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-DurationTextError" title="Text.Time.Duration.DurationTextError">DurationTextError</a> <a class="doc-type-link tok-constructor" href="/library/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a></code><span class="doc-sig-links"><a href="#v-duration-from-friendly-string" title="Permalink">#</a><a href="/library/source/Text.Time.Duration/#L1217" title="View source at line 1217">Source</a></span></div>
<div class="doc-body">

A friendly duration of hours and smaller as an exact duration.

</div>
</article>

<article class="doc-entry" id="v-duration-from-iso-string">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">duration-from-iso-string</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-DurationTextError" title="Text.Time.Duration.DurationTextError">DurationTextError</a> <a class="doc-type-link tok-constructor" href="/library/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a></code><span class="doc-sig-links"><a href="#v-duration-from-iso-string" title="Permalink">#</a><a href="/library/source/Text.Time.Duration/#L818" title="View source at line 818">Source</a></span></div>
<div class="doc-body">

An ISO 8601 duration of hours, minutes and seconds as an exact
duration. A component of days or larger is refused with
`DurationTextCalendarUnit`; a total beyond the range with
`DurationTextOutOfRange`.

</div>
</article>

<article class="doc-entry" id="v-duration-from-iso-string-lenient">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">duration-from-iso-string-lenient</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-DurationTextError" title="Text.Time.Duration.DurationTextError">DurationTextError</a> <a class="doc-type-link tok-constructor" href="/library/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a></code><span class="doc-sig-links"><a href="#v-duration-from-iso-string-lenient" title="Permalink">#</a><a href="/library/source/Text.Time.Duration/#L821" title="View source at line 821">Source</a></span></div>
</article>

<article class="doc-entry" id="v-duration-from-string">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">duration-from-string</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-DurationTextError" title="Text.Time.Duration.DurationTextError">DurationTextError</a> <a class="doc-type-link tok-constructor" href="/library/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a></code><span class="doc-sig-links"><a href="#v-duration-from-string" title="Permalink">#</a><a href="/library/source/Text.Time.Duration/#L1232" title="View source at line 1232">Source</a></span></div>
</article>

<article class="doc-entry" id="v-duration-print-friendly">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">duration-print-friendly</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-FriendlyPrinter" title="Text.Time.Duration.FriendlyPrinter">FriendlyPrinter</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-duration-print-friendly" title="Permalink">#</a><a href="/library/source/Text.Time.Duration/#L1555" title="View source at line 1555">Source</a></span></div>
<div class="doc-body">

A duration balanced into hours, minutes, seconds and the non-zero
sub-second units (`2h 30m`, `1s 500ms`), under the given options.

</div>
</article>

<article class="doc-entry" id="v-duration-text-error-position">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">duration-text-error-position</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-DurationTextError" title="Text.Time.Duration.DurationTextError">DurationTextError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-duration-text-error-position" title="Permalink">#</a><a href="/library/source/Text.Time.Duration/#L249" title="View source at line 249">Source</a></span></div>
</article>

<article class="doc-entry" id="v-duration-to-friendly-string">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">duration-to-friendly-string</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-duration-to-friendly-string" title="Permalink">#</a><a href="/library/source/Text.Time.Duration/#L1561" title="View source at line 1561">Source</a></span></div>
</article>

<article class="doc-entry" id="v-duration-to-iso-string">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">duration-to-iso-string</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-duration-to-iso-string" title="Permalink">#</a><a href="/library/source/Text.Time.Duration/#L518" title="View source at line 518">Source</a></span></div>
<div class="doc-body">

Hours, minutes and fractional seconds only (`PT5H30M`,
`PT0.000000001S`, `-PT1.5S`); never days. Exact.

</div>
</article>

<article class="doc-entry" id="v-friendly-printer-default">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">friendly-printer-default</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-FriendlyPrinter" title="Text.Time.Duration.FriendlyPrinter">FriendlyPrinter</a></code><span class="doc-sig-links"><a href="#v-friendly-printer-default" title="Permalink">#</a><a href="/library/source/Text.Time.Duration/#L1288" title="View source at line 1288">Source</a></span></div>
<div class="doc-body">

Compact designators, a space between units, no comma, `SignAuto`,
no fractional unit, the shortest exact precision, no clock form,
zero printed in seconds, ASCII `us`.

</div>
</article>

<article class="doc-entry" id="v-friendly-with-clock-form">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">friendly-with-clock-form</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-FriendlyPrinter" title="Text.Time.Duration.FriendlyPrinter">FriendlyPrinter</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-FriendlyPrinter" title="Text.Time.Duration.FriendlyPrinter">FriendlyPrinter</a></code><span class="doc-sig-links"><a href="#v-friendly-with-clock-form" title="Permalink">#</a><a href="/library/source/Text.Time.Duration/#L1349" title="View source at line 1349">Source</a></span></div>
<div class="doc-body">

Whether hours and smaller print as `HH:MM:SS[.fraction]` (at least
two digits each; the sub-second units fold into the fraction, so
the breakdown below seconds is lost). Takes precedence over a
fractional unit. Zero prints `00:00:00`.

</div>
</article>

<article class="doc-entry" id="v-friendly-with-comma">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">friendly-with-comma</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-FriendlyPrinter" title="Text.Time.Duration.FriendlyPrinter">FriendlyPrinter</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-FriendlyPrinter" title="Text.Time.Duration.FriendlyPrinter">FriendlyPrinter</a></code><span class="doc-sig-links"><a href="#v-friendly-with-comma" title="Permalink">#</a><a href="/library/source/Text.Time.Duration/#L1307" title="View source at line 1307">Source</a></span></div>
<div class="doc-body">

Whether a comma follows each designator but the last
(`1 year, 2 months`).

</div>
</article>

<article class="doc-entry" id="v-friendly-with-designator">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">friendly-with-designator</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-FriendlyDesignator" title="Text.Time.Duration.FriendlyDesignator">FriendlyDesignator</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-FriendlyPrinter" title="Text.Time.Duration.FriendlyPrinter">FriendlyPrinter</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-FriendlyPrinter" title="Text.Time.Duration.FriendlyPrinter">FriendlyPrinter</a></code><span class="doc-sig-links"><a href="#v-friendly-with-designator" title="Permalink">#</a><a href="/library/source/Text.Time.Duration/#L1291" title="View source at line 1291">Source</a></span></div>
</article>

<article class="doc-entry" id="v-friendly-with-fractional">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">friendly-with-fractional</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-FriendlyFractional" title="Text.Time.Duration.FriendlyFractional">FriendlyFractional</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-FriendlyPrinter" title="Text.Time.Duration.FriendlyPrinter">FriendlyPrinter</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-FriendlyPrinter" title="Text.Time.Duration.FriendlyPrinter">FriendlyPrinter</a></code><span class="doc-sig-links"><a href="#v-friendly-with-fractional" title="Permalink">#</a><a href="/library/source/Text.Time.Duration/#L1324" title="View source at line 1324">Source</a></span></div>
<div class="doc-body">

With a fractional unit, that unit and every smaller one print as
one number with a fraction of up to nine digits (1 hour 30 minutes
30 seconds under `FractionalMinute` is `1h 30.5m`), and a zero
value prints in that unit. Lossy for the breakdown; a fraction of
an hour or a minute is cut at nine digits.

</div>
</article>

<article class="doc-entry" id="v-friendly-with-precision">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">friendly-with-precision</strong> :: (<span class="tok-variable">digits</span> :: <a class="doc-type-link tok-constructor" href="/library/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 9 <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-FriendlyPrinter" title="Text.Time.Duration.FriendlyPrinter">FriendlyPrinter</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-FriendlyPrinter" title="Text.Time.Duration.FriendlyPrinter">FriendlyPrinter</a></code><span class="doc-sig-links"><a href="#v-friendly-with-precision" title="Permalink">#</a><a href="/library/source/Text.Time.Duration/#L1336" title="View source at line 1336">Source</a></span></div>
<div class="doc-body">

Exactly `digits` fraction digits wherever a fraction is printed
(the fractional unit, or the clock form's seconds), truncating.

</div>
</article>

<article class="doc-entry" id="v-friendly-with-shortest-precision">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">friendly-with-shortest-precision</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-FriendlyPrinter" title="Text.Time.Duration.FriendlyPrinter">FriendlyPrinter</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-FriendlyPrinter" title="Text.Time.Duration.FriendlyPrinter">FriendlyPrinter</a></code><span class="doc-sig-links"><a href="#v-friendly-with-shortest-precision" title="Permalink">#</a><a href="/library/source/Text.Time.Duration/#L1341" title="View source at line 1341">Source</a></span></div>
<div class="doc-body">

The default: the shortest fraction that is exact in nine digits.

</div>
</article>

<article class="doc-entry" id="v-friendly-with-sign">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">friendly-with-sign</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-FriendlySign" title="Text.Time.Duration.FriendlySign">FriendlySign</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-FriendlyPrinter" title="Text.Time.Duration.FriendlyPrinter">FriendlyPrinter</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-FriendlyPrinter" title="Text.Time.Duration.FriendlyPrinter">FriendlyPrinter</a></code><span class="doc-sig-links"><a href="#v-friendly-with-sign" title="Permalink">#</a><a href="/library/source/Text.Time.Duration/#L1311" title="View source at line 1311">Source</a></span></div>
</article>

<article class="doc-entry" id="v-friendly-with-spacing">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">friendly-with-spacing</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-FriendlySpacing" title="Text.Time.Duration.FriendlySpacing">FriendlySpacing</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-FriendlyPrinter" title="Text.Time.Duration.FriendlyPrinter">FriendlyPrinter</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-FriendlyPrinter" title="Text.Time.Duration.FriendlyPrinter">FriendlyPrinter</a></code><span class="doc-sig-links"><a href="#v-friendly-with-spacing" title="Permalink">#</a><a href="/library/source/Text.Time.Duration/#L1298" title="View source at line 1298">Source</a></span></div>
</article>

<article class="doc-entry" id="v-friendly-with-unicode-micro">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">friendly-with-unicode-micro</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-FriendlyPrinter" title="Text.Time.Duration.FriendlyPrinter">FriendlyPrinter</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-FriendlyPrinter" title="Text.Time.Duration.FriendlyPrinter">FriendlyPrinter</a></code><span class="doc-sig-links"><a href="#v-friendly-with-unicode-micro" title="Permalink">#</a><a href="/library/source/Text.Time.Duration/#L1362" title="View source at line 1362">Source</a></span></div>
<div class="doc-body">

Whether the compact microsecond designator is `s` after U+00B5
MICRO SIGN (UTF-8) in place of the ASCII `us`.

</div>
</article>

<article class="doc-entry" id="v-friendly-with-zero-unit">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">friendly-with-zero-unit</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Time.Unit/#t-TimeUnit" title="Data.Time.Unit.TimeUnit">TimeUnit</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-FriendlyPrinter" title="Text.Time.Duration.FriendlyPrinter">FriendlyPrinter</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-FriendlyPrinter" title="Text.Time.Duration.FriendlyPrinter">FriendlyPrinter</a></code><span class="doc-sig-links"><a href="#v-friendly-with-zero-unit" title="Permalink">#</a><a href="/library/source/Text.Time.Duration/#L1356" title="View source at line 1356">Source</a></span></div>
<div class="doc-body">

The unit a zero value prints in (`0s` by default). A `Duration`
printed with a zero unit of days or larger prints `0s`, so that it
parses back as a `Duration`.

</div>
</article>

<article class="doc-entry" id="v-span-clock-from-friendly-string">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">span-clock-from-friendly-string</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-DurationTextError" title="Text.Time.Duration.DurationTextError">DurationTextError</a> (<a class="doc-type-link tok-constructor" href="/library/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-constructor">Clock</span>)</code><span class="doc-sig-links"><a href="#v-span-clock-from-friendly-string" title="Permalink">#</a><a href="/library/source/Text.Time.Duration/#L1213" title="View source at line 1213">Source</a></span></div>
<div class="doc-body">

The same as a clock span: a component of days or larger is refused
with `DurationTextCalendarUnit`.

</div>
</article>

<article class="doc-entry" id="v-span-clock-from-iso-string">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">span-clock-from-iso-string</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-DurationTextError" title="Text.Time.Duration.DurationTextError">DurationTextError</a> (<a class="doc-type-link tok-constructor" href="/library/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-constructor">Clock</span>)</code><span class="doc-sig-links"><a href="#v-span-clock-from-iso-string" title="Permalink">#</a><a href="/library/source/Text.Time.Duration/#L811" title="View source at line 811">Source</a></span></div>
<div class="doc-body">

The same as a clock span: a component of days or larger is refused
with `DurationTextCalendarUnit`.

</div>
</article>

<article class="doc-entry" id="v-span-clock-from-string">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">span-clock-from-string</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-DurationTextError" title="Text.Time.Duration.DurationTextError">DurationTextError</a> (<a class="doc-type-link tok-constructor" href="/library/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-constructor">Clock</span>)</code><span class="doc-sig-links"><a href="#v-span-clock-from-string" title="Permalink">#</a><a href="/library/source/Text.Time.Duration/#L1227" title="View source at line 1227">Source</a></span></div>
</article>

<article class="doc-entry" id="v-span-from-friendly-string">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">span-from-friendly-string</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-DurationTextError" title="Text.Time.Duration.DurationTextError">DurationTextError</a> (<a class="doc-type-link tok-constructor" href="/library/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-constructor">Calendar</span>)</code><span class="doc-sig-links"><a href="#v-span-from-friendly-string" title="Permalink">#</a><a href="/library/source/Text.Time.Duration/#L1208" title="View source at line 1208">Source</a></span></div>
<div class="doc-body">

A friendly duration as a span, each unit as written (`1.5h` is 1
hour 30 minutes; see the header's grammar).

</div>
</article>

<article class="doc-entry" id="v-span-from-iso-string">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">span-from-iso-string</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-DurationTextError" title="Text.Time.Duration.DurationTextError">DurationTextError</a> (<a class="doc-type-link tok-constructor" href="/library/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-constructor">Calendar</span>)</code><span class="doc-sig-links"><a href="#v-span-from-iso-string" title="Permalink">#</a><a href="/library/source/Text.Time.Duration/#L801" title="View source at line 801">Source</a></span></div>
<div class="doc-body">

An ISO 8601 duration as a span, each unit as written (strict: the
grammar in the header, upper case designators).

</div>
</article>

<article class="doc-entry" id="v-span-from-iso-string-lenient">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">span-from-iso-string-lenient</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-DurationTextError" title="Text.Time.Duration.DurationTextError">DurationTextError</a> (<a class="doc-type-link tok-constructor" href="/library/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-constructor">Calendar</span>)</code><span class="doc-sig-links"><a href="#v-span-from-iso-string-lenient" title="Permalink">#</a><a href="/library/source/Text.Time.Duration/#L806" title="View source at line 806">Source</a></span></div>
<div class="doc-body">

Also accepts lower case designators (`p1y2m3dt4h`), as Temporal and
Jiff do.

</div>
</article>

<article class="doc-entry" id="v-span-from-string">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">span-from-string</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-DurationTextError" title="Text.Time.Duration.DurationTextError">DurationTextError</a> (<a class="doc-type-link tok-constructor" href="/library/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-constructor">Calendar</span>)</code><span class="doc-sig-links"><a href="#v-span-from-string" title="Permalink">#</a><a href="/library/source/Text.Time.Duration/#L1222" title="View source at line 1222">Source</a></span></div>
<div class="doc-body">

Either format (strict ISO when the first byte after an optional
sign is `P` or `p`, friendly otherwise), as Jiff's `FromStr`.

</div>
</article>

<article class="doc-entry" id="v-span-print-friendly">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">span-print-friendly</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">c</span> :: <a class="doc-type-link tok-constructor" href="/library/Data.Time.Span/#t-SpanClass" title="Data.Time.Span.SpanClass">SpanClass</a>). <a class="doc-type-link tok-constructor" href="/library/Text.Time.Duration/#t-FriendlyPrinter" title="Text.Time.Duration.FriendlyPrinter">FriendlyPrinter</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-variable">c</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-span-print-friendly" title="Permalink">#</a><a href="/library/source/Text.Time.Duration/#L1541" title="View source at line 1541">Source</a></span></div>
<div class="doc-body">

The friendly form under the given options.

</div>
</article>

<article class="doc-entry" id="v-span-to-friendly-string">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">span-to-friendly-string</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">c</span> :: <a class="doc-type-link tok-constructor" href="/library/Data.Time.Span/#t-SpanClass" title="Data.Time.Span.SpanClass">SpanClass</a>). <a class="doc-type-link tok-constructor" href="/library/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-variable">c</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-span-to-friendly-string" title="Permalink">#</a><a href="/library/source/Text.Time.Duration/#L1550" title="View source at line 1550">Source</a></span></div>
<div class="doc-body">

The friendly form with the default options. LOSSLESS: every
non-zero unit with its own compact designator, a space between
units (`6y 10mo 3d 23h`, `2000ms`), `0s` for the zero span, and
` ago` after a negative one.

</div>
</article>

<article class="doc-entry" id="v-span-to-iso-string">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">span-to-iso-string</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">c</span> :: <a class="doc-type-link tok-constructor" href="/library/Data.Time.Span/#t-SpanClass" title="Data.Time.Span.SpanClass">SpanClass</a>). <a class="doc-type-link tok-constructor" href="/library/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-variable">c</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-span-to-iso-string" title="Permalink">#</a><a href="/library/source/Text.Time.Duration/#L511" title="View source at line 511">Source</a></span></div>
<div class="doc-body">

The ISO 8601 form. LOSSY for the breakdown below seconds:
milliseconds, microseconds and nanoseconds fold into fractional
seconds (2000 milliseconds prints `PT2S`); see the header. The
zero span prints `PT0S`; a negative span has a leading `-`.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Text/Time/Duration.cinch`](/library/source/Text.Time.Duration/). Source SHA-256: `20fbc027b1c3295d2ad1d3ede0c48b955604990b27f2ab1d7da45334877cb7c8`.
