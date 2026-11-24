# Text.Time.Rfc9557

RFC 9557 date-time text with bracketed time zone annotations for zoned values.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Text.Time.Rfc9557`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.Time.Rfc9557.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Algebra.Arithmetic](Algebra.Arithmetic.md), [Data.Bytes](Data.Bytes.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Data.Time.Date](Data.Time.Date.md), [Data.Time.DateTime](Data.Time.DateTime.md), [Data.Time.Offset](Data.Time.Offset.md), [Data.Time.TimeOfDay](Data.Time.TimeOfDay.md), [Data.Time.Timestamp](Data.Time.Timestamp.md), [Data.Time.Zone](Data.Time.Zone.md), [Data.Time.Zoned](Data.Time.Zoned.md), [Relation.Eq](Relation.Eq.md), [Relation.Ord](Relation.Ord.md), [Text.Time.Iso](Text.Time.Iso.md), [Text.Time.Scan](Text.Time.Scan.md).

Non-embedded source closure: [Data.Time.Date](Data.Time.Date.md), [Data.Time.DateTime](Data.Time.DateTime.md), [Data.Time.Duration](Data.Time.Duration.md), [Data.Time.Offset](Data.Time.Offset.md), [Data.Time.TimeOfDay](Data.Time.TimeOfDay.md), [Data.Time.Timestamp](Data.Time.Timestamp.md), [Data.Time.Weekday](Data.Time.Weekday.md), [Data.Time.Zone](Data.Time.Zone.md), [Data.Time.Zone.Posix](Data.Time.Zone.Posix.md), [Data.Time.Zone.Types](Data.Time.Zone.Types.md), [Data.Time.Zoned](Data.Time.Zoned.md), [Text.Time.Iso](Text.Time.Iso.md), [Text.Time.Rfc9557](Text.Time.Rfc9557.md), [Text.Time.Scan](Text.Time.Scan.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Text.Time.Rfc9557: RFC 9557 ("IXDTF") text for `Zoned`: an RFC 3339
date-time followed by a bracketed time zone annotation, as Temporal's
ZonedDateTime and Jiff's Zoned write it
(design/datetime.md, layer L4; T4, T5, T9, T10).

    2026-11-12T15:36:00-05:00[America/New_York]

Grammar (strict; nothing before or after it):

    zoned       = date "T" time [offset] 1*annotation
    offset      = "Z" | ("+" | "-") hour [":" minute [":" second]]
    annotation  = "[" ["!"] (zone | key "=" value) "]"
    zone        = zone-name | ("+" | "-") hour ":" minute
    zone-name   = part *("/" part)      ; at most 255 bytes in all
    part        = (ALPHA | "." | "_") *(ALPHA | DIGIT | "." | "_" | "-" | "+")
                                        ; and not "." or ".."
    key         = (a-z | "_") *(a-z | DIGIT | "-" | "_")
    value       = 1*alphanum *("-" 1*alphanum)

`date` and `time` are Text.Time.Iso's (the same machines, continued
from their cursor). The offset is this module's own machine because
RFC 9557 strings in the wild carry the hour-only form
(`2024-09-10T23:37:20-04[America/New_York]`, which Temporal and Jiff
both read) and Text.Time.Iso refuses it for a plain RFC 3339
timestamp. Exactly one annotation is a time zone and it comes first;
`key=value` annotations may follow it.

The lenient parser (`-lenient`) accepts in addition: everything
Text.Time.Iso's lenient date-time accepts (a `t` or one space as the
separator, the basic date, second 60 read as 59), a lower case `z`,
the basic offsets `+HHMM` and `+HHMMSS`, an offset with seconds in the
zone annotation (`[+05:30:15]`, which is what the printer writes for
a zone it can only name by a sub-minute offset), and a date with no
time (`2026-11-12[America/New_York]`), which resolves to the first
instant of that day in the zone.

Parsing is two pure steps so that zone lookup stays an effect (T5):
  1. `parse-zoned-text :: String -> Result Rfc9557Error ZonedText`
     reads the text and nothing else. A `ZonedText` holds the civil
     date and time, the offset as written, the zone annotation (a
     name or an offset) and the other annotations.
  2. `zoned-text-resolve conflict strategy zone text` turns it into a
     `Zoned`, given the zone the caller looked up for
     `zoned-text-zone-name` (an offset annotation needs no lookup:
     `zoned-text-fixed-zone`). The lookup-and-resolve convenience
     under the `Io r` capability is `Text.Time.Zoned.zoned-from-string`.
     `zoned-resolve-datetime` applies the same rules to a civil
     reading and an offset that did not come from this parser.

Resolution is Temporal's InterpretISODateTimeOffset:
  - no offset written: the civil reading is resolved in the zone with
    the given `Disambiguation`.
  - `Z` (or `-00:00`, RFC 3339's "offset unknown", which RFC 9557 and
    Jiff read as `Z`): the civil reading is UTC, so the instant is
    exact; it is then shown in the zone. No conflict is possible.
  - a numeric offset, by the `OffsetConflict`:
      OffsetReject  the offset must be one the zone has for that civil
                    reading (in a fold either one, and it selects the
                    occurrence; in a gap none is); otherwise
                    `Rfc9557OffsetMismatch`.
      OffsetUse     the instant is the civil reading less the offset,
                    then shown in the zone (whose offset then may
                    differ from the one written).
      OffsetPrefer  as OffsetReject when the offset is valid,
                    otherwise the offset is dropped and the civil
                    reading is resolved with the `Disambiguation`.
      OffsetIgnore  the offset is dropped; the civil reading is
                    resolved with the `Disambiguation`.
    An offset written in whole minutes also matches a zone offset
    that rounds (half away from zero) to it, Temporal's rule for the
    sub-minute local mean times of old data (`+00:20` matches
    Amsterdam's +00:19:32); the instant then uses the zone's offset.
  - a date with no time (lenient only): the first instant of that
    civil day (`zoned-start-of-day`'s law: midnight, or the instant
    the clocks jumped when midnight was skipped).

Design law shape.
  - Round trip: for every `Zoned` `x` whose zone name is a
    `zone-name` of the grammar (every IANA name), or whose zone is a
    fixed offset in whole minutes, parsing `zoned-to-string x` and
    resolving with `OffsetReject` and ANY strategy in a zone of the
    same rules gives the same instant: the printed offset is the one
    in force, so it selects the occurrence in a fold.
  - `zoned-to-string` writes the civil reading with the shortest
    exact fraction, the offset in force as `+HH:MM` (`+HH:MM:SS` only
    when it has seconds; never `Z`), and the zone in brackets: the
    zone's name when it is a `zone-name` of the grammar (`UTC`, an
    IANA name), and otherwise the offset in force (`[+05:30]` for
    `zone-fixed`). `zoned-to-string-precision k` writes exactly `k`
    fraction digits, truncating.
  - `zoned-to-rfc3339` is the same text without the annotation: the
    instant as an RFC 3339 timestamp at the offset in force.
  - The parser is a state machine over the input's bytes (`st-*`),
    never going back; it allocates the zone name, the annotations
    (none in the common case) and the result. The printer computes
    the exact length, allocates one buffer and fills it.
  - No operation faults; every refusal is an `Rfc9557Error`
    constructor, with the byte position where one exists.

NAMED REFUSALS:
  - a date-time with no zone annotation (`2024-07-13T19:09:59Z`):
    `Rfc9557MissingZoneAnnotation`. It is a `Timestamp` (or a civil
    `DateTime`), not a `Zoned`; Text.Time.Iso reads it.
  - a date with no time in strict mode: `Rfc9557MissingTime`.
  - a sub-minute offset in the zone annotation (`[+05:30:15]`): the
    RFC's `time-numoffset` has hours and minutes only, so strict mode
    answers `Rfc9557SubMinuteZoneOffset`. The printer still writes the
    seconds for a zone it must name by such an offset rather than
    round it (an exact text over a valid one); the lenient parser
    reads it back.
  - a zone that has neither a grammatical name nor a fixed offset (a
    POSIX rule, a hand-built table named `my zone`): the printer
    writes the offset in force in the brackets, as Jiff does. The
    text then keeps the instant and loses the zone's rules.
  - calendars: `[u-ca=iso8601]` and `[u-ca=gregory]` are accepted and
    ignored; any other calendar is `Rfc9557UnsupportedCalendar` (only
    the proleptic Gregorian calendar exists here).
  - unknown `key=value` annotations are kept in the `ZonedText` and
    otherwise ignored, unless flagged critical (`[!u-foo=bar]`):
    `Rfc9557CriticalUnknownAnnotation`.
  - the critical flag on the zone annotation (`[!Europe/Paris]`) is
    accepted and reported (`zoned-text-zone-is-critical`) and does not
    change resolution: the `OffsetConflict` decides, as in Temporal.
    A caller that wants the RFC's reading passes `OffsetReject` (the
    default) for a critical zone.
  - whether the zone handed to `zoned-text-resolve` is the one the
    annotation names is not checked: the caller did the lookup, and
    may have mapped an alias.
  - fractional offsets, offsets beyond +-25:59:59, leap seconds: as
    Text.Time.Iso.
  - zone lookup: `Effect.Time.Zone` (T5); `Text.Time.Zoned` joins the
    two.

</details>

# Types and constructors

<a id="type-4f6666736574436f6e666c696374"></a>

## `OffsetConflict`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">OffsetConflict</span></code></pre>

What to do when a numeric offset and the zone disagree (the
header's table; Temporal's `offset` option).

[Source](/reference-sources/Text.Time.Rfc9557.cinch) · line 257

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">OffsetReject</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc9557</span>.<span class="tok-constructor">OffsetConflict</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">OffsetUse</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc9557</span>.<span class="tok-constructor">OffsetConflict</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">OffsetPrefer</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc9557</span>.<span class="tok-constructor">OffsetConflict</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">OffsetIgnore</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc9557</span>.<span class="tok-constructor">OffsetConflict</span></code></pre>

<a id="type-52666339353537416e6e6f746174696f6e"></a>

## `Rfc9557Annotation`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Rfc9557Annotation</span></code></pre>

A `key=value` annotation: key, value and the critical flag.
Abstract.

[Source](/reference-sources/Text.Time.Rfc9557.cinch) · line 278

<a id="type-526663393535374572726f72"></a>

## `Rfc9557Error`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Rfc9557Error</span></code></pre>

One constructor per refusal. A position is a byte index into the
input.

[Source](/reference-sources/Text.Time.Rfc9557.cinch) · line 181

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Rfc9557DateTime</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Iso</span>.<span class="tok-constructor">IsoError</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc9557</span>.<span class="tok-constructor">Rfc9557Error</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Rfc9557MissingTime</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc9557</span>.<span class="tok-constructor">Rfc9557Error</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Rfc9557MissingZoneAnnotation</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc9557</span>.<span class="tok-constructor">Rfc9557Error</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Rfc9557TrailingInput</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc9557</span>.<span class="tok-constructor">Rfc9557Error</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Rfc9557UnclosedAnnotation</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc9557</span>.<span class="tok-constructor">Rfc9557Error</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Rfc9557EmptyAnnotation</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc9557</span>.<span class="tok-constructor">Rfc9557Error</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Rfc9557InvalidAnnotation</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc9557</span>.<span class="tok-constructor">Rfc9557Error</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Rfc9557InvalidZoneName</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc9557</span>.<span class="tok-constructor">Rfc9557Error</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Rfc9557InvalidZoneOffset</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc9557</span>.<span class="tok-constructor">Rfc9557Error</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Rfc9557SubMinuteZoneOffset</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc9557</span>.<span class="tok-constructor">Rfc9557Error</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Rfc9557ZoneOffsetOutOfRange</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc9557</span>.<span class="tok-constructor">Rfc9557Error</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Rfc9557DuplicateZoneAnnotation</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc9557</span>.<span class="tok-constructor">Rfc9557Error</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Rfc9557ZoneAnnotationNotFirst</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc9557</span>.<span class="tok-constructor">Rfc9557Error</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Rfc9557CriticalUnknownAnnotation</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc9557</span>.<span class="tok-constructor">Rfc9557Error</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Rfc9557UnsupportedCalendar</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc9557</span>.<span class="tok-constructor">Rfc9557Error</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Rfc9557OffsetMismatch</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Offset</span>.<span class="tok-constructor">Offset</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Types</span>.<span class="tok-constructor">Ambiguous</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc9557</span>.<span class="tok-constructor">Rfc9557Error</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Rfc9557Ambiguous</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Types</span>.<span class="tok-constructor">AmbiguityError</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc9557</span>.<span class="tok-constructor">Rfc9557Error</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Rfc9557OutOfRange</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc9557</span>.<span class="tok-constructor">Rfc9557Error</span></code></pre>

<a id="type-5a6f6e65416e6e6f746174696f6e"></a>

## `ZoneAnnotation`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">ZoneAnnotation</span></code></pre>

The time zone annotation: a name to look up, or a fixed offset.

[Source](/reference-sources/Text.Time.Rfc9557.cinch) · line 272

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ZoneAnnotationName</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc9557</span>.<span class="tok-constructor">ZoneAnnotation</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ZoneAnnotationOffset</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Offset</span>.<span class="tok-constructor">Offset</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc9557</span>.<span class="tok-constructor">ZoneAnnotation</span>)</code></pre>

<a id="type-5a6f6e656454657874"></a>

## `ZonedText`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">ZonedText</span></code></pre>

What the text says, before any zone is consulted: the date, the
time (absent for a lenient date-only text), the offset as written,
the zone annotation and its critical flag, and the `key=value`
annotations in the order written. Abstract: only the parsers make
one.

[Source](/reference-sources/Text.Time.Rfc9557.cinch) · line 298

<a id="type-5a6f6e6564546578744f6666736574"></a>

## `ZonedTextOffset`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">ZonedTextOffset</span></code></pre>

The offset as written between the time and the annotation.

[Source](/reference-sources/Text.Time.Rfc9557.cinch) · line 264

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ZonedTextNoOffset</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc9557</span>.<span class="tok-constructor">ZonedTextOffset</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ZonedTextZulu</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc9557</span>.<span class="tok-constructor">ZonedTextOffset</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ZonedTextNumeric</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Offset</span>.<span class="tok-constructor">Offset</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc9557</span>.<span class="tok-constructor">ZonedTextOffset</span>)</code></pre>

# Values

<a id="value-70617273652d7a6f6e65642d74657874"></a>

## `parse-zoned-text`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">parse-zoned-text</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc9557</span>.<span class="tok-constructor">Rfc9557Error</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc9557</span>.<span class="tok-constructor">ZonedText</span>))</code></pre>

Parsing. States, in order:
  (Iso date) -> st-after-date -> (Iso time) -> st-offset
  -> [st-offset-hour -> [st-offset-minute -> [st-offset-second]]]
  -> st-annotations -> (st-zone | st-key)* -> done.

[Source](/reference-sources/Text.Time.Rfc9557.cinch) · line 378

<a id="value-70617273652d7a6f6e65642d746578742d6c656e69656e74"></a>

## `parse-zoned-text-lenient`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">parse-zoned-text-lenient</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc9557</span>.<span class="tok-constructor">Rfc9557Error</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc9557</span>.<span class="tok-constructor">ZonedText</span>))</code></pre>

The lenient forms of the header, including a date with no time.

[Source](/reference-sources/Text.Time.Rfc9557.cinch) · line 382

<a id="value-726663393535372d616e6e6f746174696f6e2d69732d637269746963616c"></a>

## `rfc9557-annotation-is-critical`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rfc9557-annotation-is-critical</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc9557</span>.<span class="tok-constructor">Rfc9557Annotation</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

[Source](/reference-sources/Text.Time.Rfc9557.cinch)

<a id="value-726663393535372d616e6e6f746174696f6e2d6b6579"></a>

## `rfc9557-annotation-key`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rfc9557-annotation-key</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc9557</span>.<span class="tok-constructor">Rfc9557Annotation</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

[Source](/reference-sources/Text.Time.Rfc9557.cinch)

<a id="value-726663393535372d616e6e6f746174696f6e2d76616c7565"></a>

## `rfc9557-annotation-value`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rfc9557-annotation-value</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc9557</span>.<span class="tok-constructor">Rfc9557Annotation</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

[Source](/reference-sources/Text.Time.Rfc9557.cinch)

<a id="value-726663393535372d6572726f722d706f736974696f6e"></a>

## `rfc9557-error-position`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rfc9557-error-position</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc9557</span>.<span class="tok-constructor">Rfc9557Error</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">I64</span>))</code></pre>

The byte position of a parse refusal (for `Rfc9557DateTime`, the
inner error's); `None` for the three resolution refusals.

[Source](/reference-sources/Text.Time.Rfc9557.cinch) · line 234

<a id="value-726663393535372d76616c69642d7a6f6e652d6e616d65"></a>

## `rfc9557-valid-zone-name`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rfc9557-valid-zone-name</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Whether a string is a `zone-name` of the grammar (what the printer
writes in the brackets as a name). Every name `tzdb-valid-name`
accepts whose parts start with a letter, `.` or `_` is one.

[Source](/reference-sources/Text.Time.Rfc9557.cinch) · line 621

<a id="value-726663393535372d7a6f6e65642d6279746573"></a>

## `rfc9557-zoned-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rfc9557-zoned-bytes</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

`zoned-to-string`'s text as one exactly-sized fresh buffer (what
`Text.Builder.builder-add-bytes` appends without copying).

[Source](/reference-sources/Text.Time.Rfc9557.cinch) · line 786

<a id="value-7a6f6e65642d7265736f6c76652d6461746574696d65"></a>

## `zoned-resolve-datetime`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zoned-resolve-datetime</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc9557</span>.<span class="tok-constructor">OffsetConflict</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Types</span>.<span class="tok-constructor">Disambiguation</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">TimeZone</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc9557</span>.<span class="tok-constructor">ZonedTextOffset</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc9557</span>.<span class="tok-constructor">Rfc9557Error</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span>))))))</code></pre>

The same rules for a civil reading and an offset that came from
somewhere else (`strptime`'s `%z`, a database column): the header's
table, with `ZonedTextNoOffset` for a reading with no offset.

[Source](/reference-sources/Text.Time.Rfc9557.cinch) · line 687

<a id="value-7a6f6e65642d746578742d616e6e6f746174696f6e73"></a>

## `zoned-text-annotations`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zoned-text-annotations</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc9557</span>.<span class="tok-constructor">ZonedText</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc9557</span>.<span class="tok-constructor">Rfc9557Annotation</span>))</code></pre>

[Source](/reference-sources/Text.Time.Rfc9557.cinch)

<a id="value-7a6f6e65642d746578742d64617465"></a>

## `zoned-text-date`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zoned-text-date</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc9557</span>.<span class="tok-constructor">ZonedText</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Date</span>.<span class="tok-constructor">Date</span>)</code></pre>

[Source](/reference-sources/Text.Time.Rfc9557.cinch)

<a id="value-7a6f6e65642d746578742d6461746574696d65"></a>

## `zoned-text-datetime`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zoned-text-datetime</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc9557</span>.<span class="tok-constructor">ZonedText</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span>)</code></pre>

The civil reading as written (midnight for a date-only text, which
is NOT how such a text resolves: see the header).

[Source](/reference-sources/Text.Time.Rfc9557.cinch) · line 317

<a id="value-7a6f6e65642d746578742d66697865642d7a6f6e65"></a>

## `zoned-text-fixed-zone`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zoned-text-fixed-zone</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc9557</span>.<span class="tok-constructor">ZonedText</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">TimeZone</span>))</code></pre>

The zone itself when the annotation is an offset (`zone-fixed`; no
lookup is needed); `None` when it is a name.

[Source](/reference-sources/Text.Time.Rfc9557.cinch) · line 358

<a id="value-7a6f6e65642d746578742d69732d646174652d6f6e6c79"></a>

## `zoned-text-is-date-only`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zoned-text-is-date-only</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc9557</span>.<span class="tok-constructor">ZonedText</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

[Source](/reference-sources/Text.Time.Rfc9557.cinch)

<a id="value-7a6f6e65642d746578742d69732d7a756c75"></a>

## `zoned-text-is-zulu`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zoned-text-is-zulu</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc9557</span>.<span class="tok-constructor">ZonedText</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

[Source](/reference-sources/Text.Time.Rfc9557.cinch)

<a id="value-7a6f6e65642d746578742d6e756d657269632d6f6666736574"></a>

## `zoned-text-numeric-offset`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zoned-text-numeric-offset</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc9557</span>.<span class="tok-constructor">ZonedText</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Offset</span>.<span class="tok-constructor">Offset</span>))</code></pre>

The numeric offset, when one was written (`None` for `Z` and for
no offset).

[Source](/reference-sources/Text.Time.Rfc9557.cinch) · line 329

<a id="value-7a6f6e65642d746578742d6f6666736574"></a>

## `zoned-text-offset`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zoned-text-offset</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc9557</span>.<span class="tok-constructor">ZonedText</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc9557</span>.<span class="tok-constructor">ZonedTextOffset</span>)</code></pre>

[Source](/reference-sources/Text.Time.Rfc9557.cinch)

<a id="value-7a6f6e65642d746578742d7265736f6c7665"></a>

## `zoned-text-resolve`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zoned-text-resolve</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc9557</span>.<span class="tok-constructor">OffsetConflict</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Types</span>.<span class="tok-constructor">Disambiguation</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">TimeZone</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc9557</span>.<span class="tok-constructor">ZonedText</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc9557</span>.<span class="tok-constructor">Rfc9557Error</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span>)))))</code></pre>

`zone` is the zone the caller looked up for the annotation.

[Source](/reference-sources/Text.Time.Rfc9557.cinch) · line 678

<a id="value-7a6f6e65642d746578742d7265736f6c76652d64656661756c74"></a>

## `zoned-text-resolve-default`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zoned-text-resolve-default</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">TimeZone</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc9557</span>.<span class="tok-constructor">ZonedText</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc9557</span>.<span class="tok-constructor">Rfc9557Error</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span>)))</code></pre>

`OffsetReject` and the compatible strategy: Temporal's and Jiff's
defaults.

[Source](/reference-sources/Text.Time.Rfc9557.cinch) · line 703

<a id="value-7a6f6e65642d746578742d74696d65"></a>

## `zoned-text-time`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zoned-text-time</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc9557</span>.<span class="tok-constructor">ZonedText</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">TimeOfDay</span>.<span class="tok-constructor">TimeOfDay</span>))</code></pre>

`None` for a date-only text.

[Source](/reference-sources/Text.Time.Rfc9557.cinch) · line 306

<a id="value-7a6f6e65642d746578742d7a6f6e65"></a>

## `zoned-text-zone`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zoned-text-zone</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc9557</span>.<span class="tok-constructor">ZonedText</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc9557</span>.<span class="tok-constructor">ZoneAnnotation</span>)</code></pre>

[Source](/reference-sources/Text.Time.Rfc9557.cinch)

<a id="value-7a6f6e65642d746578742d7a6f6e652d69732d637269746963616c"></a>

## `zoned-text-zone-is-critical`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zoned-text-zone-is-critical</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc9557</span>.<span class="tok-constructor">ZonedText</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

[Source](/reference-sources/Text.Time.Rfc9557.cinch)

<a id="value-7a6f6e65642d746578742d7a6f6e652d6e616d65"></a>

## `zoned-text-zone-name`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zoned-text-zone-name</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc9557</span>.<span class="tok-constructor">ZonedText</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

The name to look up; `None` when the annotation is an offset.

[Source](/reference-sources/Text.Time.Rfc9557.cinch) · line 346

<a id="value-7a6f6e65642d746578742d7a6f6e652d6f6666736574"></a>

## `zoned-text-zone-offset`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zoned-text-zone-offset</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc9557</span>.<span class="tok-constructor">ZonedText</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Offset</span>.<span class="tok-constructor">Offset</span>))</code></pre>

[Source](/reference-sources/Text.Time.Rfc9557.cinch)

<a id="value-7a6f6e65642d746f2d72666333333339"></a>

## `zoned-to-rfc3339`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zoned-to-rfc3339</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

The instant as an RFC 3339 timestamp at the zone's offset in force:
`zoned-to-string` without the annotation
(`2026-10-31T05:30:00+11:00`). The offset is numeric, never `Z`.
When the offset in force has seconds (local mean time in old data)
they are written, which RFC 3339 does not allow; rounding them
would name a different instant.

[Source](/reference-sources/Text.Time.Rfc9557.cinch) · line 778

<a id="value-7a6f6e65642d746f2d726663333333392d707265636973696f6e"></a>

## `zoned-to-rfc3339-precision`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zoned-to-rfc3339-precision</span> :: ((<span class="tok-variable">digits</span> :: (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">0</span> <span class="tok-number">9</span> <span class="tok-constructor">I64</span>)) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

[Source](/reference-sources/Text.Time.Rfc9557.cinch)

<a id="value-7a6f6e65642d746f2d737472696e67"></a>

## `zoned-to-string`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zoned-to-string</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

The civil reading, the offset in force and the zone annotation
(header), with the shortest exact fraction.

[Source](/reference-sources/Text.Time.Rfc9557.cinch) · line 765

<a id="value-7a6f6e65642d746f2d737472696e672d707265636973696f6e"></a>

## `zoned-to-string-precision`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zoned-to-string-precision</span> :: ((<span class="tok-variable">digits</span> :: (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">0</span> <span class="tok-number">9</span> <span class="tok-constructor">I64</span>)) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

Exactly `digits` fraction digits, truncating.

[Source](/reference-sources/Text.Time.Rfc9557.cinch) · line 769

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Text/Time/Rfc9557.cinch`. Source SHA-256: `d7827396a4192555b96bbe750ebd54edce0041ad499d7535b231185dd0fc6bb1`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
