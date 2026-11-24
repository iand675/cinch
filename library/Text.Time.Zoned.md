# Text.Time.Zoned

Offset and zoned-value text functions.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--source">Ship source</span> <code>import Text.Time.Zoned</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Text.Time.Zoned/source/">View source</a><a href="/packages/search/?q=%2BText.Time.Zoned">Search module</a><a href="/reference-sources/Text.Time.Zoned.cinch" download="Text.Time.Zoned.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bytes](Data.Bytes.md), [Data.Machine](Data.Machine.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Data.Time.DateTime](Data.Time.DateTime.md), [Data.Time.Offset](Data.Time.Offset.md), [Data.Time.Timestamp](Data.Time.Timestamp.md), [Data.Time.Zone](Data.Time.Zone.md), [Data.Time.Zoned](Data.Time.Zoned.md), [Effect.Time.Zone](Effect.Time.Zone.md), [Relation.Eq](Relation.Eq.md), [Relation.Ord](Relation.Ord.md), [Text.Time.Iso](Text.Time.Iso.md), [Text.Time.Rfc9557](Text.Time.Rfc9557.md), [Text.Time.Scan](Text.Time.Scan.md), [Text.Time.Strtime](Text.Time.Strtime.md).

<details class="doc-closure"><summary>Ship-source closure (20 files)</summary>

Copy these modules beside your entry file before importing `Text.Time.Zoned`: [Data.Time.Date](Data.Time.Date.md), [Data.Time.DateTime](Data.Time.DateTime.md), [Data.Time.Duration](Data.Time.Duration.md), [Data.Time.Monotonic](Data.Time.Monotonic.md), [Data.Time.Offset](Data.Time.Offset.md), [Data.Time.TimeOfDay](Data.Time.TimeOfDay.md), [Data.Time.Timestamp](Data.Time.Timestamp.md), [Data.Time.Weekday](Data.Time.Weekday.md), [Data.Time.Zone](Data.Time.Zone.md), [Data.Time.Zone.Posix](Data.Time.Zone.Posix.md), [Data.Time.Zone.Types](Data.Time.Zone.Types.md), [Data.Time.Zone.Tzif](Data.Time.Zone.Tzif.md), [Data.Time.Zoned](Data.Time.Zoned.md), [Effect.Time](Effect.Time.md), [Effect.Time.Zone](Effect.Time.Zone.md), [Text.Time.Iso](Text.Time.Iso.md), [Text.Time.Rfc9557](Text.Time.Rfc9557.md), [Text.Time.Scan](Text.Time.Scan.md), [Text.Time.Strtime](Text.Time.Strtime.md), [Text.Time.Zoned](Text.Time.Zoned.md).

```sh
cp seed/prelude/Data/Time/Date.cinch ./Data.Time.Date.cinch
cp seed/prelude/Data/Time/DateTime.cinch ./Data.Time.DateTime.cinch
cp seed/prelude/Data/Time/Duration.cinch ./Data.Time.Duration.cinch
cp seed/prelude/Data/Time/Monotonic.cinch ./Data.Time.Monotonic.cinch
cp seed/prelude/Data/Time/Offset.cinch ./Data.Time.Offset.cinch
cp seed/prelude/Data/Time/TimeOfDay.cinch ./Data.Time.TimeOfDay.cinch
cp seed/prelude/Data/Time/Timestamp.cinch ./Data.Time.Timestamp.cinch
cp seed/prelude/Data/Time/Weekday.cinch ./Data.Time.Weekday.cinch
cp seed/prelude/Data/Time/Zone.cinch ./Data.Time.Zone.cinch
cp seed/prelude/Data/Time/Zone/Posix.cinch ./Data.Time.Zone.Posix.cinch
cp seed/prelude/Data/Time/Zone/Types.cinch ./Data.Time.Zone.Types.cinch
cp seed/prelude/Data/Time/Zone/Tzif.cinch ./Data.Time.Zone.Tzif.cinch
cp seed/prelude/Data/Time/Zoned.cinch ./Data.Time.Zoned.cinch
cp seed/prelude/Effect/Time.cinch ./Effect.Time.cinch
cp seed/prelude/Effect/Time/Zone.cinch ./Effect.Time.Zone.cinch
cp seed/prelude/Text/Time/Iso.cinch ./Text.Time.Iso.cinch
cp seed/prelude/Text/Time/Rfc9557.cinch ./Text.Time.Rfc9557.cinch
cp seed/prelude/Text/Time/Scan.cinch ./Text.Time.Scan.cinch
cp seed/prelude/Text/Time/Strtime.cinch ./Text.Time.Strtime.cinch
cp seed/prelude/Text/Time/Zoned.cinch ./Text.Time.Zoned.cinch
```

</details>

<details class="doc-notes"><summary>Module notes</summary>

Text.Time.Zoned: the `Offset`- and `Zoned`-typed text functions
(design/datetime.md, layer L4; T5, T9, T10).

Text.Time.Iso and Text.Time.Strtime sit below the zone layer and take
an offset as plain `I64` seconds; Text.Time.Rfc9557 is pure and takes
an already looked-up `TimeZone`. This module is where those meet the
zone layer:
  - RFC 3339 for a `Timestamp` at an `Offset`
    (`timestamp-to-string-at`, `timestamp-and-offset-from-string`);
  - `strftime` for a `Zoned` and for a `Timestamp` at an `Offset`
    (`zoned-strftime`, `timestamp-strftime-at`), through
    `broken-down-from-zoned`, which fills the offset, the zone
    abbreviation and the zone name so `%z`, `%Z` and `%Q` work;
  - `strptime` results to a `Zoned` (`broken-down-to-zoned-in`,
    `broken-down-to-zoned-at-offset`, `broken-down-to-zoned`);
  - the one effectful step, looking a zone up by the name a text
    carries (`zoned-text-lookup`), and the parse, look up and resolve
    conveniences built on it (`zoned-from-string`, `zoned-strptime`).

Design law shape.
  - `timestamp-to-string-at o t` is TOTAL where Text.Time.Iso's
    `timestamp-to-string-with-offset` answers `Option`: that `None` is
    for an `I64` outside +-93599, and an `Offset` is inside it by
    construction; the civil reading is `offset-timestamp-to-datetime`,
    which is total (T3). It writes the numeric offset, never `Z`, and
    `timestamp-and-offset-from-string` reads back the same instant
    and offset.
  - `timestamp-and-offset-from-string` reads `Z` and `-00:00` as
    `offset-utc` (the spelling is Text.Time.Iso's
    `timestamp-from-string-with-offset` to ask for).
  - `broken-down-from-zoned x` has the civil fields of
    `zoned-datetime x`, the offset in force, the Unix second, the
    abbreviation in force and, when the zone's name is a zone name of
    RFC 9557's grammar (every IANA name; not `zone-fixed`'s `+05:30`
    label), that name; so `%Q` prints the name, or the offset for a
    fixed-offset zone.
  - `broken-down-to-zoned-in conflict strategy zone b` reads the
    fields in `zone` by Text.Time.Rfc9557's resolution rules: a Unix
    second (`%s`) is the instant; otherwise the civil reading, with
    the offset (`%z`) checked against the zone by the
    `OffsetConflict`, or with no offset resolved by the
    `Disambiguation`. `zone` is the zone the caller looked up for
    `broken-down-zone-name b` (or chose).
  - `broken-down-to-zoned-at-offset b` needs no zone: the instant is
    the civil reading at the offset and the zone is `zone-fixed` of
    it.
  - `broken-down-to-zoned zone b` is the two with the defaults
    (`OffsetReject`, `Compatible`): in `Some zone`; at the offset for
    `None`; and `ZonedTextMissingZone` when there is neither a zone
    nor an offset (a civil reading names no instant).
  - `zoned-from-string db s` is `parse-zoned-text`, a `tzdb-lookup` of
    the annotation's name (none for an offset annotation), and
    `zoned-text-resolve-default`. `zoned-strptime db format s` is
    `strptime`, a lookup of the `%Q` name when one was read, and
    `broken-down-to-zoned`.
  - No operation faults; `ZonedTextError` says which stage refused.

NAMED REFUSALS:
  - a zone abbreviation as input (`%Z`, `EST`): Text.Time.Strtime
    refuses to parse one, and so does this module.
  - a default zone for a text that names none: `zoned-strptime`
    answers `ZonedTextMissingZone`; a caller with a zone in hand uses
    `broken-down-to-zoned-in`.
  - whether the zone handed to the pure functions is the one the text
    names is not checked (as Text.Time.Rfc9557).
  - the unreachable `None` of `broken-down-from-timestamp-at-offset`
    (an offset outside +-93599) reads as the civil fields alone.

</details>

<details class="doc-synopsis" open><summary>Synopsis (18 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-ZonedTextError">ZonedTextError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ZonedTextError</strong></code></li><li><a class="syn-name" href="#v-broken-down-from-timestamp-at">broken-down-from-timestamp-at</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">broken-down-from-timestamp-at</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Strtime/#t-BrokenDown" title="Text.Time.Strtime.BrokenDown">BrokenDown</a></code></li><li><a class="syn-name" href="#v-broken-down-from-zoned">broken-down-from-zoned</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">broken-down-from-zoned</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Strtime/#t-BrokenDown" title="Text.Time.Strtime.BrokenDown">BrokenDown</a></code></li><li><a class="syn-name" href="#v-broken-down-to-zoned">broken-down-to-zoned</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">broken-down-to-zoned</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-TimeZone" title="Data.Time.Zone.TimeZone">TimeZone</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Strtime/#t-BrokenDown" title="Text.Time.Strtime.BrokenDown">BrokenDown</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Zoned/#t-ZonedTextError" title="Text.Time.Zoned.ZonedTextError">ZonedTextError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a></code></li><li><a class="syn-name" href="#v-broken-down-to-zoned-at-offset">broken-down-to-zoned-at-offset</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">broken-down-to-zoned-at-offset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Strtime/#t-BrokenDown" title="Text.Time.Strtime.BrokenDown">BrokenDown</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Zoned/#t-ZonedTextError" title="Text.Time.Zoned.ZonedTextError">ZonedTextError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a></code></li><li><a class="syn-name" href="#v-broken-down-to-zoned-in">broken-down-to-zoned-in</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">broken-down-to-zoned-in</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-OffsetConflict" title="Text.Time.Rfc9557.OffsetConflict">OffsetConflict</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-Disambiguation" title="Data.Time.Zone.Types.Disambiguation">Disambiguation</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-TimeZone" title="Data.Time.Zone.TimeZone">TimeZone</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Strtime/#t-BrokenDown" title="Text.Time.Strtime.BrokenDown">BrokenDown</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Zoned/#t-ZonedTextError" title="Text.Time.Zoned.ZonedTextError">ZonedTextError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a></code></li><li><a class="syn-name" href="#v-timestamp-and-offset-from-string">timestamp-and-offset-from-string</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">timestamp-and-offset-from-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Iso/#t-IsoError" title="Text.Time.Iso.IsoError">IsoError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a>)</code></li><li><a class="syn-name" href="#v-timestamp-and-offset-from-string-lenient">timestamp-and-offset-from-string-lenient</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">timestamp-and-offset-from-string-lenient</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Iso/#t-IsoError" title="Text.Time.Iso.IsoError">IsoError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a>)</code></li><li><a class="syn-name" href="#v-timestamp-strftime-at">timestamp-strftime-at</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">timestamp-strftime-at</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Strtime/#t-StrtimeError" title="Text.Time.Strtime.StrtimeError">StrtimeError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-timestamp-to-string-at">timestamp-to-string-at</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">timestamp-to-string-at</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-timestamp-to-string-at-precision">timestamp-to-string-at-precision</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">timestamp-to-string-at-precision</strong> :: (<span class="tok-variable">digits</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 9 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-zoned-from-string">zoned-from-string</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zoned-from-string</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Time.Zone/#t-TzDb" title="Effect.Time.Zone.TzDb">TzDb</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Zoned/#t-ZonedTextError" title="Text.Time.Zoned.ZonedTextError">ZonedTextError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a></code></li><li><a class="syn-name" href="#v-zoned-from-string-lenient">zoned-from-string-lenient</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zoned-from-string-lenient</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Time.Zone/#t-TzDb" title="Effect.Time.Zone.TzDb">TzDb</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Zoned/#t-ZonedTextError" title="Text.Time.Zoned.ZonedTextError">ZonedTextError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a></code></li><li><a class="syn-name" href="#v-zoned-from-string-with">zoned-from-string-with</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zoned-from-string-with</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-OffsetConflict" title="Text.Time.Rfc9557.OffsetConflict">OffsetConflict</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-Disambiguation" title="Data.Time.Zone.Types.Disambiguation">Disambiguation</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Time.Zone/#t-TzDb" title="Effect.Time.Zone.TzDb">TzDb</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Zoned/#t-ZonedTextError" title="Text.Time.Zoned.ZonedTextError">ZonedTextError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a></code></li><li><a class="syn-name" href="#v-zoned-strftime">zoned-strftime</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zoned-strftime</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Strtime/#t-StrtimeError" title="Text.Time.Strtime.StrtimeError">StrtimeError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-zoned-strftime-format">zoned-strftime-format</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zoned-strftime-format</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Strtime/#t-Format" title="Text.Time.Strtime.Format">Format</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Strtime/#t-StrtimeError" title="Text.Time.Strtime.StrtimeError">StrtimeError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-zoned-strptime">zoned-strptime</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zoned-strptime</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Time.Zone/#t-TzDb" title="Effect.Time.Zone.TzDb">TzDb</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Zoned/#t-ZonedTextError" title="Text.Time.Zoned.ZonedTextError">ZonedTextError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a></code></li><li><a class="syn-name" href="#v-zoned-text-lookup">zoned-text-lookup</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zoned-text-lookup</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Time.Zone/#t-TzDb" title="Effect.Time.Zone.TzDb">TzDb</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-ZonedText" title="Text.Time.Rfc9557.ZonedText">ZonedText</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Time.Zone/#t-TzLookupError" title="Effect.Time.Zone.TzLookupError">TzLookupError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-TimeZone" title="Data.Time.Zone.TimeZone">TimeZone</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-ZonedTextError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ZonedTextError</strong></code><span class="doc-sig-links"><a href="#t-ZonedTextError" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Zoned/source/#L101" title="View source at line 101">Source</a></span></div>
<div class="doc-body">

Which stage refused.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-ZonedTextRfc9557"><code><strong class="tok-constructor">ZonedTextRfc9557</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-Rfc9557Error" title="Text.Time.Rfc9557.Rfc9557Error">Rfc9557Error</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Zoned/#t-ZonedTextError" title="Text.Time.Zoned.ZonedTextError">ZonedTextError</a></code></li><li id="k-ZonedTextStrtime"><code><strong class="tok-constructor">ZonedTextStrtime</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Strtime/#t-StrtimeError" title="Text.Time.Strtime.StrtimeError">StrtimeError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Zoned/#t-ZonedTextError" title="Text.Time.Zoned.ZonedTextError">ZonedTextError</a></code></li><li id="k-ZonedTextLookup"><code><strong class="tok-constructor">ZonedTextLookup</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Time.Zone/#t-TzLookupError" title="Effect.Time.Zone.TzLookupError">TzLookupError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Zoned/#t-ZonedTextError" title="Text.Time.Zoned.ZonedTextError">ZonedTextError</a></code></li><li id="k-ZonedTextMissingZone"><code><strong class="tok-constructor">ZonedTextMissingZone</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Zoned/#t-ZonedTextError" title="Text.Time.Zoned.ZonedTextError">ZonedTextError</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-broken-down-from-timestamp-at">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">broken-down-from-timestamp-at</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Strtime/#t-BrokenDown" title="Text.Time.Strtime.BrokenDown">BrokenDown</a></code><span class="doc-sig-links"><a href="#v-broken-down-from-timestamp-at" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Zoned/source/#L161" title="View source at line 161">Source</a></span></div>
<div class="doc-body">

The civil reading at `o`, that offset and the Unix second.

</div>
</article>

<article class="doc-entry" id="v-broken-down-from-zoned">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">broken-down-from-zoned</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Strtime/#t-BrokenDown" title="Text.Time.Strtime.BrokenDown">BrokenDown</a></code><span class="doc-sig-links"><a href="#v-broken-down-from-zoned" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Zoned/source/#L167" title="View source at line 167">Source</a></span></div>
<div class="doc-body">

Everything a `Zoned` knows (header).

</div>
</article>

<article class="doc-entry" id="v-broken-down-to-zoned">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">broken-down-to-zoned</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-TimeZone" title="Data.Time.Zone.TimeZone">TimeZone</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Strtime/#t-BrokenDown" title="Text.Time.Strtime.BrokenDown">BrokenDown</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Zoned/#t-ZonedTextError" title="Text.Time.Zoned.ZonedTextError">ZonedTextError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a></code><span class="doc-sig-links"><a href="#v-broken-down-to-zoned" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Zoned/source/#L224" title="View source at line 224">Source</a></span></div>
<div class="doc-body">

With the defaults: in `Some zone`; at the offset for `None`;
`ZonedTextMissingZone` when there is neither.

</div>
</article>

<article class="doc-entry" id="v-broken-down-to-zoned-at-offset">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">broken-down-to-zoned-at-offset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Strtime/#t-BrokenDown" title="Text.Time.Strtime.BrokenDown">BrokenDown</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Zoned/#t-ZonedTextError" title="Text.Time.Zoned.ZonedTextError">ZonedTextError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a></code><span class="doc-sig-links"><a href="#v-broken-down-to-zoned-at-offset" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Zoned/source/#L214" title="View source at line 214">Source</a></span></div>
<div class="doc-body">

The instant the fields name at their own offset, in the
fixed-offset zone of that offset. `StrtimeMissingOffset` when there
is none.

</div>
</article>

<article class="doc-entry" id="v-broken-down-to-zoned-in">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">broken-down-to-zoned-in</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-OffsetConflict" title="Text.Time.Rfc9557.OffsetConflict">OffsetConflict</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-Disambiguation" title="Data.Time.Zone.Types.Disambiguation">Disambiguation</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-TimeZone" title="Data.Time.Zone.TimeZone">TimeZone</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Strtime/#t-BrokenDown" title="Text.Time.Strtime.BrokenDown">BrokenDown</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Zoned/#t-ZonedTextError" title="Text.Time.Zoned.ZonedTextError">ZonedTextError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a></code><span class="doc-sig-links"><a href="#v-broken-down-to-zoned-in" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Zoned/source/#L190" title="View source at line 190">Source</a></span></div>
<div class="doc-body">

The fields read in `z` (header).

</div>
</article>

<article class="doc-entry" id="v-timestamp-and-offset-from-string">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">timestamp-and-offset-from-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Iso/#t-IsoError" title="Text.Time.Iso.IsoError">IsoError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a>)</code><span class="doc-sig-links"><a href="#v-timestamp-and-offset-from-string" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Zoned/source/#L140" title="View source at line 140">Source</a></span></div>
<div class="doc-body">

An RFC 3339 timestamp as the instant and the offset it was written
at (strict: Text.Time.Iso's grammar).

</div>
</article>

<article class="doc-entry" id="v-timestamp-and-offset-from-string-lenient">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">timestamp-and-offset-from-string-lenient</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Iso/#t-IsoError" title="Text.Time.Iso.IsoError">IsoError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a>)</code><span class="doc-sig-links"><a href="#v-timestamp-and-offset-from-string-lenient" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Zoned/source/#L143" title="View source at line 143">Source</a></span></div>
</article>

<article class="doc-entry" id="v-timestamp-strftime-at">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">timestamp-strftime-at</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Strtime/#t-StrtimeError" title="Text.Time.Strtime.StrtimeError">StrtimeError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-timestamp-strftime-at" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Zoned/source/#L182" title="View source at line 182">Source</a></span></div>
<div class="doc-body">

The instant read at `o`: `%z` prints the offset; `%Z` and `%Q` have
no zone to name (`%Q` prints the offset).

</div>
</article>

<article class="doc-entry" id="v-timestamp-to-string-at">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">timestamp-to-string-at</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-timestamp-to-string-at" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Zoned/source/#L120" title="View source at line 120">Source</a></span></div>
<div class="doc-body">

The civil reading of the instant at `o`, then the numeric offset
(`+00:00` for UTC, never `Z`), with the shortest exact fraction.
Total (header).

</div>
</article>

<article class="doc-entry" id="v-timestamp-to-string-at-precision">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">timestamp-to-string-at-precision</strong> :: (<span class="tok-variable">digits</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 9 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-timestamp-to-string-at-precision" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Zoned/source/#L126" title="View source at line 126">Source</a></span></div>
<div class="doc-body">

Exactly `digits` fraction digits, truncating.

</div>
</article>

<article class="doc-entry" id="v-zoned-from-string">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zoned-from-string</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Time.Zone/#t-TzDb" title="Effect.Time.Zone.TzDb">TzDb</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Zoned/#t-ZonedTextError" title="Text.Time.Zoned.ZonedTextError">ZonedTextError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a></code><span class="doc-sig-links"><a href="#v-zoned-from-string" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Zoned/source/#L244" title="View source at line 244">Source</a></span></div>
<div class="doc-body">

Parse (strict), look the zone up, resolve with `OffsetReject` and
the compatible strategy.

</div>
</article>

<article class="doc-entry" id="v-zoned-from-string-lenient">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zoned-from-string-lenient</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Time.Zone/#t-TzDb" title="Effect.Time.Zone.TzDb">TzDb</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Zoned/#t-ZonedTextError" title="Text.Time.Zoned.ZonedTextError">ZonedTextError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a></code><span class="doc-sig-links"><a href="#v-zoned-from-string-lenient" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Zoned/source/#L249" title="View source at line 249">Source</a></span></div>
<div class="doc-body">

The same with the lenient grammar (a date with no time is the
first instant of that day in the zone).

</div>
</article>

<article class="doc-entry" id="v-zoned-from-string-with">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zoned-from-string-with</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-OffsetConflict" title="Text.Time.Rfc9557.OffsetConflict">OffsetConflict</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-Disambiguation" title="Data.Time.Zone.Types.Disambiguation">Disambiguation</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Time.Zone/#t-TzDb" title="Effect.Time.Zone.TzDb">TzDb</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Zoned/#t-ZonedTextError" title="Text.Time.Zoned.ZonedTextError">ZonedTextError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a></code><span class="doc-sig-links"><a href="#v-zoned-from-string-with" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Zoned/source/#L255" title="View source at line 255">Source</a></span></div>
<div class="doc-body">

Strict, with the conflict policy and the strategy chosen.

</div>
</article>

<article class="doc-entry" id="v-zoned-strftime">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zoned-strftime</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Strtime/#t-StrtimeError" title="Text.Time.Strtime.StrtimeError">StrtimeError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-zoned-strftime" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Zoned/source/#L173" title="View source at line 173">Source</a></span></div>
</article>

<article class="doc-entry" id="v-zoned-strftime-format">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zoned-strftime-format</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Strtime/#t-Format" title="Text.Time.Strtime.Format">Format</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Strtime/#t-StrtimeError" title="Text.Time.Strtime.StrtimeError">StrtimeError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-zoned-strftime-format" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Zoned/source/#L177" title="View source at line 177">Source</a></span></div>
<div class="doc-body">

With a format compiled once (`strtime-format-compile`).

</div>
</article>

<article class="doc-entry" id="v-zoned-strptime">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zoned-strptime</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Time.Zone/#t-TzDb" title="Effect.Time.Zone.TzDb">TzDb</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Zoned/#t-ZonedTextError" title="Text.Time.Zoned.ZonedTextError">ZonedTextError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a></code><span class="doc-sig-links"><a href="#v-zoned-strptime" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Zoned/source/#L271" title="View source at line 271">Source</a></span></div>
<div class="doc-body">

`strptime`, then the zone: the `%Q` name looked up in `db` when
one was read, otherwise the `%z` offset as a fixed zone, otherwise
`ZonedTextMissingZone`. (`%Q` also reads a numeric offset, which
arrives here as the offset field.)

</div>
</article>

<article class="doc-entry" id="v-zoned-text-lookup">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zoned-text-lookup</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Time.Zone/#t-TzDb" title="Effect.Time.Zone.TzDb">TzDb</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc9557/#t-ZonedText" title="Text.Time.Rfc9557.ZonedText">ZonedText</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Time.Zone/#t-TzLookupError" title="Effect.Time.Zone.TzLookupError">TzLookupError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-TimeZone" title="Data.Time.Zone.TimeZone">TimeZone</a></code><span class="doc-sig-links"><a href="#v-zoned-text-lookup" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Zoned/source/#L237" title="View source at line 237">Source</a></span></div>
<div class="doc-body">

The zone a text's annotation names: a database lookup for a name,
`zone-fixed` (no lookup) for an offset.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Text/Time/Zoned.cinch`](/packages/stdlib/Text.Time.Zoned/source/). Source SHA-256: `f4942fd2b7cc3f238527aed3bce3d0a8df17178dbe40fba507c21da52ab15e5b`.
