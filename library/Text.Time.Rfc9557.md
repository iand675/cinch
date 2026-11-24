# Text.Time.Rfc9557

RFC 9557 date-time text with bracketed time zone annotations for zoned values.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Text.Time.Rfc9557</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Text.Time.Rfc9557/source/">View source</a><a href="/packages/search/?q=%2BText.Time.Rfc9557">Search module</a><a href="/reference-sources/Text.Time.Rfc9557.cinch" download="Text.Time.Rfc9557.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Algebra.Arithmetic](Algebra.Arithmetic.md), [Data.Bytes](Data.Bytes.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Data.Time.Date](Data.Time.Date.md), [Data.Time.DateTime](Data.Time.DateTime.md), [Data.Time.Offset](Data.Time.Offset.md), [Data.Time.TimeOfDay](Data.Time.TimeOfDay.md), [Data.Time.Timestamp](Data.Time.Timestamp.md), [Data.Time.Zone](Data.Time.Zone.md), [Data.Time.Zoned](Data.Time.Zoned.md), [Relation.Eq](Relation.Eq.md), [Relation.Ord](Relation.Ord.md), [Text.Time.Iso](Text.Time.Iso.md), [Text.Time.Scan](Text.Time.Scan.md).

<details class="doc-notes"><summary>Module notes</summary>

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

<details class="doc-synopsis"><summary>Synopsis (34 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-OffsetConflict">OffsetConflict</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">OffsetConflict</strong></code></li><li><a class="syn-name" href="#t-Rfc9557Annotation">Rfc9557Annotation</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Rfc9557Annotation</strong></code></li><li><a class="syn-name" href="#t-Rfc9557Error">Rfc9557Error</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Rfc9557Error</strong></code></li><li><a class="syn-name" href="#t-ZoneAnnotation">ZoneAnnotation</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ZoneAnnotation</strong></code></li><li><a class="syn-name" href="#t-ZonedText">ZonedText</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ZonedText</strong></code></li><li><a class="syn-name" href="#t-ZonedTextOffset">ZonedTextOffset</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ZonedTextOffset</strong></code></li><li><a class="syn-name" href="#v-parse-zoned-text">parse-zoned-text</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">parse-zoned-text</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-Rfc9557Error" title="Text.Time.Rfc9557.Rfc9557Error">Rfc9557Error</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-ZonedText" title="Text.Time.Rfc9557.ZonedText">ZonedText</a></code></li><li><a class="syn-name" href="#v-parse-zoned-text-lenient">parse-zoned-text-lenient</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">parse-zoned-text-lenient</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-Rfc9557Error" title="Text.Time.Rfc9557.Rfc9557Error">Rfc9557Error</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-ZonedText" title="Text.Time.Rfc9557.ZonedText">ZonedText</a></code></li><li><a class="syn-name" href="#v-rfc9557-annotation-is-critical">rfc9557-annotation-is-critical</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rfc9557-annotation-is-critical</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-Rfc9557Annotation" title="Text.Time.Rfc9557.Rfc9557Annotation">Rfc9557Annotation</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-rfc9557-annotation-key">rfc9557-annotation-key</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rfc9557-annotation-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-Rfc9557Annotation" title="Text.Time.Rfc9557.Rfc9557Annotation">Rfc9557Annotation</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-rfc9557-annotation-value">rfc9557-annotation-value</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rfc9557-annotation-value</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-Rfc9557Annotation" title="Text.Time.Rfc9557.Rfc9557Annotation">Rfc9557Annotation</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-rfc9557-error-position">rfc9557-error-position</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rfc9557-error-position</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-Rfc9557Error" title="Text.Time.Rfc9557.Rfc9557Error">Rfc9557Error</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-rfc9557-valid-zone-name">rfc9557-valid-zone-name</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rfc9557-valid-zone-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-rfc9557-zoned-bytes">rfc9557-zoned-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rfc9557-zoned-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-zoned-resolve-datetime">zoned-resolve-datetime</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zoned-resolve-datetime</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-OffsetConflict" title="Text.Time.Rfc9557.OffsetConflict">OffsetConflict</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-Disambiguation" title="Data.Time.Zone.Types.Disambiguation">Disambiguation</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-TimeZone" title="Data.Time.Zone.TimeZone">TimeZone</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-ZonedTextOffset" title="Text.Time.Rfc9557.ZonedTextOffset">ZonedTextOffset</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-Rfc9557Error" title="Text.Time.Rfc9557.Rfc9557Error">Rfc9557Error</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a></code></li><li><a class="syn-name" href="#v-zoned-text-annotations">zoned-text-annotations</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zoned-text-annotations</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-ZonedText" title="Text.Time.Rfc9557.ZonedText">ZonedText</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-Rfc9557Annotation" title="Text.Time.Rfc9557.Rfc9557Annotation">Rfc9557Annotation</a></code></li><li><a class="syn-name" href="#v-zoned-text-date">zoned-text-date</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zoned-text-date</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-ZonedText" title="Text.Time.Rfc9557.ZonedText">ZonedText</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a></code></li><li><a class="syn-name" href="#v-zoned-text-datetime">zoned-text-datetime</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zoned-text-datetime</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-ZonedText" title="Text.Time.Rfc9557.ZonedText">ZonedText</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a></code></li><li><a class="syn-name" href="#v-zoned-text-fixed-zone">zoned-text-fixed-zone</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zoned-text-fixed-zone</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-ZonedText" title="Text.Time.Rfc9557.ZonedText">ZonedText</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-TimeZone" title="Data.Time.Zone.TimeZone">TimeZone</a></code></li><li><a class="syn-name" href="#v-zoned-text-is-date-only">zoned-text-is-date-only</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zoned-text-is-date-only</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-ZonedText" title="Text.Time.Rfc9557.ZonedText">ZonedText</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-zoned-text-is-zulu">zoned-text-is-zulu</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zoned-text-is-zulu</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-ZonedText" title="Text.Time.Rfc9557.ZonedText">ZonedText</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-zoned-text-numeric-offset">zoned-text-numeric-offset</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zoned-text-numeric-offset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-ZonedText" title="Text.Time.Rfc9557.ZonedText">ZonedText</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a></code></li><li><a class="syn-name" href="#v-zoned-text-offset">zoned-text-offset</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zoned-text-offset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-ZonedText" title="Text.Time.Rfc9557.ZonedText">ZonedText</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-ZonedTextOffset" title="Text.Time.Rfc9557.ZonedTextOffset">ZonedTextOffset</a></code></li><li><a class="syn-name" href="#v-zoned-text-resolve">zoned-text-resolve</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zoned-text-resolve</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-OffsetConflict" title="Text.Time.Rfc9557.OffsetConflict">OffsetConflict</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-Disambiguation" title="Data.Time.Zone.Types.Disambiguation">Disambiguation</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-TimeZone" title="Data.Time.Zone.TimeZone">TimeZone</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-ZonedText" title="Text.Time.Rfc9557.ZonedText">ZonedText</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-Rfc9557Error" title="Text.Time.Rfc9557.Rfc9557Error">Rfc9557Error</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a></code></li><li><a class="syn-name" href="#v-zoned-text-resolve-default">zoned-text-resolve-default</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zoned-text-resolve-default</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-TimeZone" title="Data.Time.Zone.TimeZone">TimeZone</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-ZonedText" title="Text.Time.Rfc9557.ZonedText">ZonedText</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-Rfc9557Error" title="Text.Time.Rfc9557.Rfc9557Error">Rfc9557Error</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a></code></li><li><a class="syn-name" href="#v-zoned-text-time">zoned-text-time</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zoned-text-time</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-ZonedText" title="Text.Time.Rfc9557.ZonedText">ZonedText</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a></code></li><li><a class="syn-name" href="#v-zoned-text-zone">zoned-text-zone</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zoned-text-zone</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-ZonedText" title="Text.Time.Rfc9557.ZonedText">ZonedText</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-ZoneAnnotation" title="Text.Time.Rfc9557.ZoneAnnotation">ZoneAnnotation</a></code></li><li><a class="syn-name" href="#v-zoned-text-zone-is-critical">zoned-text-zone-is-critical</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zoned-text-zone-is-critical</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-ZonedText" title="Text.Time.Rfc9557.ZonedText">ZonedText</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-zoned-text-zone-name">zoned-text-zone-name</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zoned-text-zone-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-ZonedText" title="Text.Time.Rfc9557.ZonedText">ZonedText</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-zoned-text-zone-offset">zoned-text-zone-offset</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zoned-text-zone-offset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-ZonedText" title="Text.Time.Rfc9557.ZonedText">ZonedText</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a></code></li><li><a class="syn-name" href="#v-zoned-to-rfc3339">zoned-to-rfc3339</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zoned-to-rfc3339</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-zoned-to-rfc3339-precision">zoned-to-rfc3339-precision</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zoned-to-rfc3339-precision</strong> :: (<span class="tok-variable">digits</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 9 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-zoned-to-string">zoned-to-string</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zoned-to-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-zoned-to-string-precision">zoned-to-string-precision</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zoned-to-string-precision</strong> :: (<span class="tok-variable">digits</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 9 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-OffsetConflict">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">OffsetConflict</strong></code><span class="doc-sig-links"><a href="#t-OffsetConflict" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Rfc9557/source/#L257" title="View source at line 257">Source</a></span></div>
<div class="doc-body">

What to do when a numeric offset and the zone disagree (the
header's table; Temporal's `offset` option).

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-OffsetReject"><code><strong class="tok-constructor">OffsetReject</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-OffsetConflict" title="Text.Time.Rfc9557.OffsetConflict">OffsetConflict</a></code></li><li id="k-OffsetUse"><code><strong class="tok-constructor">OffsetUse</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-OffsetConflict" title="Text.Time.Rfc9557.OffsetConflict">OffsetConflict</a></code></li><li id="k-OffsetPrefer"><code><strong class="tok-constructor">OffsetPrefer</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-OffsetConflict" title="Text.Time.Rfc9557.OffsetConflict">OffsetConflict</a></code></li><li id="k-OffsetIgnore"><code><strong class="tok-constructor">OffsetIgnore</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-OffsetConflict" title="Text.Time.Rfc9557.OffsetConflict">OffsetConflict</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-Rfc9557Annotation">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Rfc9557Annotation</strong></code><span class="doc-sig-links"><a href="#t-Rfc9557Annotation" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Rfc9557/source/#L278" title="View source at line 278">Source</a></span></div>
<div class="doc-body">

A `key=value` annotation: key, value and the critical flag.
Abstract.

</div>
</article>

<article class="doc-entry" id="t-Rfc9557Error">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Rfc9557Error</strong></code><span class="doc-sig-links"><a href="#t-Rfc9557Error" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Rfc9557/source/#L181" title="View source at line 181">Source</a></span></div>
<div class="doc-body">

One constructor per refusal. A position is a byte index into the
input.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-Rfc9557DateTime"><code><strong class="tok-constructor">Rfc9557DateTime</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Iso/#t-IsoError" title="Text.Time.Iso.IsoError">IsoError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-Rfc9557Error" title="Text.Time.Rfc9557.Rfc9557Error">Rfc9557Error</a></code></li><li id="k-Rfc9557MissingTime"><code><strong class="tok-constructor">Rfc9557MissingTime</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-Rfc9557Error" title="Text.Time.Rfc9557.Rfc9557Error">Rfc9557Error</a></code></li><li id="k-Rfc9557MissingZoneAnnotation"><code><strong class="tok-constructor">Rfc9557MissingZoneAnnotation</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-Rfc9557Error" title="Text.Time.Rfc9557.Rfc9557Error">Rfc9557Error</a></code></li><li id="k-Rfc9557TrailingInput"><code><strong class="tok-constructor">Rfc9557TrailingInput</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-Rfc9557Error" title="Text.Time.Rfc9557.Rfc9557Error">Rfc9557Error</a></code></li><li id="k-Rfc9557UnclosedAnnotation"><code><strong class="tok-constructor">Rfc9557UnclosedAnnotation</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-Rfc9557Error" title="Text.Time.Rfc9557.Rfc9557Error">Rfc9557Error</a></code></li><li id="k-Rfc9557EmptyAnnotation"><code><strong class="tok-constructor">Rfc9557EmptyAnnotation</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-Rfc9557Error" title="Text.Time.Rfc9557.Rfc9557Error">Rfc9557Error</a></code></li><li id="k-Rfc9557InvalidAnnotation"><code><strong class="tok-constructor">Rfc9557InvalidAnnotation</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-Rfc9557Error" title="Text.Time.Rfc9557.Rfc9557Error">Rfc9557Error</a></code></li><li id="k-Rfc9557InvalidZoneName"><code><strong class="tok-constructor">Rfc9557InvalidZoneName</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-Rfc9557Error" title="Text.Time.Rfc9557.Rfc9557Error">Rfc9557Error</a></code></li><li id="k-Rfc9557InvalidZoneOffset"><code><strong class="tok-constructor">Rfc9557InvalidZoneOffset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-Rfc9557Error" title="Text.Time.Rfc9557.Rfc9557Error">Rfc9557Error</a></code></li><li id="k-Rfc9557SubMinuteZoneOffset"><code><strong class="tok-constructor">Rfc9557SubMinuteZoneOffset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-Rfc9557Error" title="Text.Time.Rfc9557.Rfc9557Error">Rfc9557Error</a></code></li><li id="k-Rfc9557ZoneOffsetOutOfRange"><code><strong class="tok-constructor">Rfc9557ZoneOffsetOutOfRange</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-Rfc9557Error" title="Text.Time.Rfc9557.Rfc9557Error">Rfc9557Error</a></code></li><li id="k-Rfc9557DuplicateZoneAnnotation"><code><strong class="tok-constructor">Rfc9557DuplicateZoneAnnotation</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-Rfc9557Error" title="Text.Time.Rfc9557.Rfc9557Error">Rfc9557Error</a></code></li><li id="k-Rfc9557ZoneAnnotationNotFirst"><code><strong class="tok-constructor">Rfc9557ZoneAnnotationNotFirst</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-Rfc9557Error" title="Text.Time.Rfc9557.Rfc9557Error">Rfc9557Error</a></code></li><li id="k-Rfc9557CriticalUnknownAnnotation"><code><strong class="tok-constructor">Rfc9557CriticalUnknownAnnotation</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-Rfc9557Error" title="Text.Time.Rfc9557.Rfc9557Error">Rfc9557Error</a></code></li><li id="k-Rfc9557UnsupportedCalendar"><code><strong class="tok-constructor">Rfc9557UnsupportedCalendar</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-Rfc9557Error" title="Text.Time.Rfc9557.Rfc9557Error">Rfc9557Error</a></code></li><li id="k-Rfc9557OffsetMismatch"><code><strong class="tok-constructor">Rfc9557OffsetMismatch</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-Ambiguous" title="Data.Time.Zone.Types.Ambiguous">Ambiguous</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-Rfc9557Error" title="Text.Time.Rfc9557.Rfc9557Error">Rfc9557Error</a></code></li><li id="k-Rfc9557Ambiguous"><code><strong class="tok-constructor">Rfc9557Ambiguous</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-AmbiguityError" title="Data.Time.Zone.Types.AmbiguityError">AmbiguityError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-Rfc9557Error" title="Text.Time.Rfc9557.Rfc9557Error">Rfc9557Error</a></code></li><li id="k-Rfc9557OutOfRange"><code><strong class="tok-constructor">Rfc9557OutOfRange</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-Rfc9557Error" title="Text.Time.Rfc9557.Rfc9557Error">Rfc9557Error</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-ZoneAnnotation">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ZoneAnnotation</strong></code><span class="doc-sig-links"><a href="#t-ZoneAnnotation" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Rfc9557/source/#L272" title="View source at line 272">Source</a></span></div>
<div class="doc-body">

The time zone annotation: a name to look up, or a fixed offset.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-ZoneAnnotationName"><code><strong class="tok-constructor">ZoneAnnotationName</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-ZoneAnnotation" title="Text.Time.Rfc9557.ZoneAnnotation">ZoneAnnotation</a></code></li><li id="k-ZoneAnnotationOffset"><code><strong class="tok-constructor">ZoneAnnotationOffset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-ZoneAnnotation" title="Text.Time.Rfc9557.ZoneAnnotation">ZoneAnnotation</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-ZonedText">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ZonedText</strong></code><span class="doc-sig-links"><a href="#t-ZonedText" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Rfc9557/source/#L298" title="View source at line 298">Source</a></span></div>
<div class="doc-body">

What the text says, before any zone is consulted: the date, the
time (absent for a lenient date-only text), the offset as written,
the zone annotation and its critical flag, and the `key=value`
annotations in the order written. Abstract: only the parsers make
one.

</div>
</article>

<article class="doc-entry" id="t-ZonedTextOffset">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ZonedTextOffset</strong></code><span class="doc-sig-links"><a href="#t-ZonedTextOffset" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Rfc9557/source/#L264" title="View source at line 264">Source</a></span></div>
<div class="doc-body">

The offset as written between the time and the annotation.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-ZonedTextNoOffset"><code><strong class="tok-constructor">ZonedTextNoOffset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-ZonedTextOffset" title="Text.Time.Rfc9557.ZonedTextOffset">ZonedTextOffset</a></code></li><li id="k-ZonedTextZulu"><code><strong class="tok-constructor">ZonedTextZulu</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-ZonedTextOffset" title="Text.Time.Rfc9557.ZonedTextOffset">ZonedTextOffset</a></code></li><li id="k-ZonedTextNumeric"><code><strong class="tok-constructor">ZonedTextNumeric</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-ZonedTextOffset" title="Text.Time.Rfc9557.ZonedTextOffset">ZonedTextOffset</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-parse-zoned-text">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">parse-zoned-text</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-Rfc9557Error" title="Text.Time.Rfc9557.Rfc9557Error">Rfc9557Error</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-ZonedText" title="Text.Time.Rfc9557.ZonedText">ZonedText</a></code><span class="doc-sig-links"><a href="#v-parse-zoned-text" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Rfc9557/source/#L378" title="View source at line 378">Source</a></span></div>
<div class="doc-body">

Parsing. States, in order:
  (Iso date) -> st-after-date -> (Iso time) -> st-offset
  -> [st-offset-hour -> [st-offset-minute -> [st-offset-second]]]
  -> st-annotations -> (st-zone | st-key)* -> done.

</div>
</article>

<article class="doc-entry" id="v-parse-zoned-text-lenient">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">parse-zoned-text-lenient</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-Rfc9557Error" title="Text.Time.Rfc9557.Rfc9557Error">Rfc9557Error</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-ZonedText" title="Text.Time.Rfc9557.ZonedText">ZonedText</a></code><span class="doc-sig-links"><a href="#v-parse-zoned-text-lenient" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Rfc9557/source/#L382" title="View source at line 382">Source</a></span></div>
<div class="doc-body">

The lenient forms of the header, including a date with no time.

</div>
</article>

<article class="doc-entry" id="v-rfc9557-annotation-is-critical">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rfc9557-annotation-is-critical</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-Rfc9557Annotation" title="Text.Time.Rfc9557.Rfc9557Annotation">Rfc9557Annotation</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-rfc9557-annotation-is-critical" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Rfc9557/source/#L289" title="View source at line 289">Source</a></span></div>
</article>

<article class="doc-entry" id="v-rfc9557-annotation-key">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rfc9557-annotation-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-Rfc9557Annotation" title="Text.Time.Rfc9557.Rfc9557Annotation">Rfc9557Annotation</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-rfc9557-annotation-key" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Rfc9557/source/#L281" title="View source at line 281">Source</a></span></div>
</article>

<article class="doc-entry" id="v-rfc9557-annotation-value">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rfc9557-annotation-value</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-Rfc9557Annotation" title="Text.Time.Rfc9557.Rfc9557Annotation">Rfc9557Annotation</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-rfc9557-annotation-value" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Rfc9557/source/#L285" title="View source at line 285">Source</a></span></div>
</article>

<article class="doc-entry" id="v-rfc9557-error-position">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rfc9557-error-position</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-Rfc9557Error" title="Text.Time.Rfc9557.Rfc9557Error">Rfc9557Error</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-rfc9557-error-position" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Rfc9557/source/#L234" title="View source at line 234">Source</a></span></div>
<div class="doc-body">

The byte position of a parse refusal (for `Rfc9557DateTime`, the
inner error's); `None` for the three resolution refusals.

</div>
</article>

<article class="doc-entry" id="v-rfc9557-valid-zone-name">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rfc9557-valid-zone-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-rfc9557-valid-zone-name" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Rfc9557/source/#L621" title="View source at line 621">Source</a></span></div>
<div class="doc-body">

Whether a string is a `zone-name` of the grammar (what the printer
writes in the brackets as a name). Every name `tzdb-valid-name`
accepts whose parts start with a letter, `.` or `_` is one.

</div>
</article>

<article class="doc-entry" id="v-rfc9557-zoned-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rfc9557-zoned-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-rfc9557-zoned-bytes" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Rfc9557/source/#L786" title="View source at line 786">Source</a></span></div>
<div class="doc-body">

`zoned-to-string`'s text as one exactly-sized fresh buffer (what
`Text.Builder.builder-add-bytes` appends without copying).

</div>
</article>

<article class="doc-entry" id="v-zoned-resolve-datetime">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zoned-resolve-datetime</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-OffsetConflict" title="Text.Time.Rfc9557.OffsetConflict">OffsetConflict</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-Disambiguation" title="Data.Time.Zone.Types.Disambiguation">Disambiguation</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-TimeZone" title="Data.Time.Zone.TimeZone">TimeZone</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-ZonedTextOffset" title="Text.Time.Rfc9557.ZonedTextOffset">ZonedTextOffset</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-Rfc9557Error" title="Text.Time.Rfc9557.Rfc9557Error">Rfc9557Error</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a></code><span class="doc-sig-links"><a href="#v-zoned-resolve-datetime" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Rfc9557/source/#L687" title="View source at line 687">Source</a></span></div>
<div class="doc-body">

The same rules for a civil reading and an offset that came from
somewhere else (`strptime`'s `%z`, a database column): the header's
table, with `ZonedTextNoOffset` for a reading with no offset.

</div>
</article>

<article class="doc-entry" id="v-zoned-text-annotations">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zoned-text-annotations</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-ZonedText" title="Text.Time.Rfc9557.ZonedText">ZonedText</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-Rfc9557Annotation" title="Text.Time.Rfc9557.Rfc9557Annotation">Rfc9557Annotation</a></code><span class="doc-sig-links"><a href="#v-zoned-text-annotations" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Rfc9557/source/#L367" title="View source at line 367">Source</a></span></div>
</article>

<article class="doc-entry" id="v-zoned-text-date">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zoned-text-date</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-ZonedText" title="Text.Time.Rfc9557.ZonedText">ZonedText</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a></code><span class="doc-sig-links"><a href="#v-zoned-text-date" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Rfc9557/source/#L301" title="View source at line 301">Source</a></span></div>
</article>

<article class="doc-entry" id="v-zoned-text-datetime">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zoned-text-datetime</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-ZonedText" title="Text.Time.Rfc9557.ZonedText">ZonedText</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a></code><span class="doc-sig-links"><a href="#v-zoned-text-datetime" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Rfc9557/source/#L317" title="View source at line 317">Source</a></span></div>
<div class="doc-body">

The civil reading as written (midnight for a date-only text, which
is NOT how such a text resolves: see the header).

</div>
</article>

<article class="doc-entry" id="v-zoned-text-fixed-zone">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zoned-text-fixed-zone</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-ZonedText" title="Text.Time.Rfc9557.ZonedText">ZonedText</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-TimeZone" title="Data.Time.Zone.TimeZone">TimeZone</a></code><span class="doc-sig-links"><a href="#v-zoned-text-fixed-zone" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Rfc9557/source/#L358" title="View source at line 358">Source</a></span></div>
<div class="doc-body">

The zone itself when the annotation is an offset (`zone-fixed`; no
lookup is needed); `None` when it is a name.

</div>
</article>

<article class="doc-entry" id="v-zoned-text-is-date-only">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zoned-text-is-date-only</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-ZonedText" title="Text.Time.Rfc9557.ZonedText">ZonedText</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-zoned-text-is-date-only" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Rfc9557/source/#L310" title="View source at line 310">Source</a></span></div>
</article>

<article class="doc-entry" id="v-zoned-text-is-zulu">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zoned-text-is-zulu</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-ZonedText" title="Text.Time.Rfc9557.ZonedText">ZonedText</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-zoned-text-is-zulu" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Rfc9557/source/#L335" title="View source at line 335">Source</a></span></div>
</article>

<article class="doc-entry" id="v-zoned-text-numeric-offset">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zoned-text-numeric-offset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-ZonedText" title="Text.Time.Rfc9557.ZonedText">ZonedText</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a></code><span class="doc-sig-links"><a href="#v-zoned-text-numeric-offset" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Rfc9557/source/#L329" title="View source at line 329">Source</a></span></div>
<div class="doc-body">

The numeric offset, when one was written (`None` for `Z` and for
no offset).

</div>
</article>

<article class="doc-entry" id="v-zoned-text-offset">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zoned-text-offset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-ZonedText" title="Text.Time.Rfc9557.ZonedText">ZonedText</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-ZonedTextOffset" title="Text.Time.Rfc9557.ZonedTextOffset">ZonedTextOffset</a></code><span class="doc-sig-links"><a href="#v-zoned-text-offset" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Rfc9557/source/#L323" title="View source at line 323">Source</a></span></div>
</article>

<article class="doc-entry" id="v-zoned-text-resolve">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zoned-text-resolve</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-OffsetConflict" title="Text.Time.Rfc9557.OffsetConflict">OffsetConflict</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-Disambiguation" title="Data.Time.Zone.Types.Disambiguation">Disambiguation</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-TimeZone" title="Data.Time.Zone.TimeZone">TimeZone</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-ZonedText" title="Text.Time.Rfc9557.ZonedText">ZonedText</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-Rfc9557Error" title="Text.Time.Rfc9557.Rfc9557Error">Rfc9557Error</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a></code><span class="doc-sig-links"><a href="#v-zoned-text-resolve" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Rfc9557/source/#L678" title="View source at line 678">Source</a></span></div>
<div class="doc-body">

`zone` is the zone the caller looked up for the annotation.

</div>
</article>

<article class="doc-entry" id="v-zoned-text-resolve-default">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zoned-text-resolve-default</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-TimeZone" title="Data.Time.Zone.TimeZone">TimeZone</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-ZonedText" title="Text.Time.Rfc9557.ZonedText">ZonedText</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-Rfc9557Error" title="Text.Time.Rfc9557.Rfc9557Error">Rfc9557Error</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a></code><span class="doc-sig-links"><a href="#v-zoned-text-resolve-default" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Rfc9557/source/#L703" title="View source at line 703">Source</a></span></div>
<div class="doc-body">

`OffsetReject` and the compatible strategy: Temporal's and Jiff's
defaults.

</div>
</article>

<article class="doc-entry" id="v-zoned-text-time">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zoned-text-time</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-ZonedText" title="Text.Time.Rfc9557.ZonedText">ZonedText</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a></code><span class="doc-sig-links"><a href="#v-zoned-text-time" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Rfc9557/source/#L306" title="View source at line 306">Source</a></span></div>
<div class="doc-body">

`None` for a date-only text.

</div>
</article>

<article class="doc-entry" id="v-zoned-text-zone">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zoned-text-zone</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-ZonedText" title="Text.Time.Rfc9557.ZonedText">ZonedText</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-ZoneAnnotation" title="Text.Time.Rfc9557.ZoneAnnotation">ZoneAnnotation</a></code><span class="doc-sig-links"><a href="#v-zoned-text-zone" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Rfc9557/source/#L341" title="View source at line 341">Source</a></span></div>
</article>

<article class="doc-entry" id="v-zoned-text-zone-is-critical">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zoned-text-zone-is-critical</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-ZonedText" title="Text.Time.Rfc9557.ZonedText">ZonedText</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-zoned-text-zone-is-critical" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Rfc9557/source/#L363" title="View source at line 363">Source</a></span></div>
</article>

<article class="doc-entry" id="v-zoned-text-zone-name">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zoned-text-zone-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-ZonedText" title="Text.Time.Rfc9557.ZonedText">ZonedText</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-zoned-text-zone-name" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Rfc9557/source/#L346" title="View source at line 346">Source</a></span></div>
<div class="doc-body">

The name to look up; `None` when the annotation is an offset.

</div>
</article>

<article class="doc-entry" id="v-zoned-text-zone-offset">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zoned-text-zone-offset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-ZonedText" title="Text.Time.Rfc9557.ZonedText">ZonedText</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a></code><span class="doc-sig-links"><a href="#v-zoned-text-zone-offset" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Rfc9557/source/#L351" title="View source at line 351">Source</a></span></div>
</article>

<article class="doc-entry" id="v-zoned-to-rfc3339">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zoned-to-rfc3339</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-zoned-to-rfc3339" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Rfc9557/source/#L778" title="View source at line 778">Source</a></span></div>
<div class="doc-body">

The instant as an RFC 3339 timestamp at the zone's offset in force:
`zoned-to-string` without the annotation
(`2026-10-31T05:30:00+11:00`). The offset is numeric, never `Z`.
When the offset in force has seconds (local mean time in old data)
they are written, which RFC 3339 does not allow; rounding them
would name a different instant.

</div>
</article>

<article class="doc-entry" id="v-zoned-to-rfc3339-precision">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zoned-to-rfc3339-precision</strong> :: (<span class="tok-variable">digits</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 9 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-zoned-to-rfc3339-precision" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Rfc9557/source/#L781" title="View source at line 781">Source</a></span></div>
</article>

<article class="doc-entry" id="v-zoned-to-string">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zoned-to-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-zoned-to-string" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Rfc9557/source/#L765" title="View source at line 765">Source</a></span></div>
<div class="doc-body">

The civil reading, the offset in force and the zone annotation
(header), with the shortest exact fraction.

</div>
</article>

<article class="doc-entry" id="v-zoned-to-string-precision">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zoned-to-string-precision</strong> :: (<span class="tok-variable">digits</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 9 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-zoned-to-string-precision" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Rfc9557/source/#L769" title="View source at line 769">Source</a></span></div>
<div class="doc-body">

Exactly `digits` fraction digits, truncating.

</div>
</article>
