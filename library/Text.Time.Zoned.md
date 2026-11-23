# Text.Time.Zoned

Offset and zoned-value text functions.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Text.Time.Zoned`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.Time.Zoned.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bytes](Data.Bytes.md), [Data.Machine](Data.Machine.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Data.Time.DateTime](Data.Time.DateTime.md), [Data.Time.Offset](Data.Time.Offset.md), [Data.Time.Timestamp](Data.Time.Timestamp.md), [Data.Time.Zone](Data.Time.Zone.md), [Data.Time.Zoned](Data.Time.Zoned.md), [Effect.Time.Zone](Effect.Time.Zone.md), [Relation.Eq](Relation.Eq.md), [Relation.Ord](Relation.Ord.md), [Text.Time.Iso](Text.Time.Iso.md), [Text.Time.Rfc9557](Text.Time.Rfc9557.md), [Text.Time.Scan](Text.Time.Scan.md), [Text.Time.Strtime](Text.Time.Strtime.md).

Non-embedded source closure: [Data.Time.Date](Data.Time.Date.md), [Data.Time.DateTime](Data.Time.DateTime.md), [Data.Time.Duration](Data.Time.Duration.md), [Data.Time.Monotonic](Data.Time.Monotonic.md), [Data.Time.Offset](Data.Time.Offset.md), [Data.Time.TimeOfDay](Data.Time.TimeOfDay.md), [Data.Time.Timestamp](Data.Time.Timestamp.md), [Data.Time.Weekday](Data.Time.Weekday.md), [Data.Time.Zone](Data.Time.Zone.md), [Data.Time.Zone.Posix](Data.Time.Zone.Posix.md), [Data.Time.Zone.Types](Data.Time.Zone.Types.md), [Data.Time.Zone.Tzif](Data.Time.Zone.Tzif.md), [Data.Time.Zoned](Data.Time.Zoned.md), [Effect.Time](Effect.Time.md), [Effect.Time.Zone](Effect.Time.Zone.md), [Text.Time.Iso](Text.Time.Iso.md), [Text.Time.Rfc9557](Text.Time.Rfc9557.md), [Text.Time.Scan](Text.Time.Scan.md), [Text.Time.Strtime](Text.Time.Strtime.md), [Text.Time.Zoned](Text.Time.Zoned.md).

<details><summary>Behaviour and boundary notes from the source</summary>

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

# Types and constructors

<a id="type-5a6f6e6564546578744572726f72"></a>

## `ZonedTextError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">ZonedTextError</span></code></pre>

Which stage refused.

[Source](/reference-sources/Text.Time.Zoned.cinch) · line 101

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ZonedTextRfc9557</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc9557</span>.<span class="tok-constructor">Rfc9557Error</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">ZonedTextError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ZonedTextStrtime</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">StrtimeError</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">ZonedTextError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ZonedTextLookup</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">TzLookupError</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">ZonedTextError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ZonedTextMissingZone</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">ZonedTextError</span></code></pre>

# Values

<a id="value-62726f6b656e2d646f776e2d66726f6d2d74696d657374616d702d6174"></a>

## `broken-down-from-timestamp-at`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">broken-down-from-timestamp-at</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Offset</span>.<span class="tok-constructor">Offset</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">BrokenDown</span>))</code></pre>

The civil reading at `o`, that offset and the Unix second.

[Source](/reference-sources/Text.Time.Zoned.cinch) · line 161

<a id="value-62726f6b656e2d646f776e2d66726f6d2d7a6f6e6564"></a>

## `broken-down-from-zoned`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">broken-down-from-zoned</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">BrokenDown</span>)</code></pre>

Everything a `Zoned` knows (header).

[Source](/reference-sources/Text.Time.Zoned.cinch) · line 167

<a id="value-62726f6b656e2d646f776e2d746f2d7a6f6e6564"></a>

## `broken-down-to-zoned`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">broken-down-to-zoned</span> :: ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">TimeZone</span>) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">BrokenDown</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">ZonedTextError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span>)))</code></pre>

With the defaults: in `Some zone`; at the offset for `None`;
`ZonedTextMissingZone` when there is neither.

[Source](/reference-sources/Text.Time.Zoned.cinch) · line 224

<a id="value-62726f6b656e2d646f776e2d746f2d7a6f6e65642d61742d6f6666736574"></a>

## `broken-down-to-zoned-at-offset`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">broken-down-to-zoned-at-offset</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">BrokenDown</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">ZonedTextError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span>))</code></pre>

The instant the fields name at their own offset, in the
fixed-offset zone of that offset. `StrtimeMissingOffset` when there
is none.

[Source](/reference-sources/Text.Time.Zoned.cinch) · line 214

<a id="value-62726f6b656e2d646f776e2d746f2d7a6f6e65642d696e"></a>

## `broken-down-to-zoned-in`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">broken-down-to-zoned-in</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc9557</span>.<span class="tok-constructor">OffsetConflict</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Types</span>.<span class="tok-constructor">Disambiguation</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">TimeZone</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">BrokenDown</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">ZonedTextError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span>)))))</code></pre>

The fields read in `z` (header).

[Source](/reference-sources/Text.Time.Zoned.cinch) · line 190

<a id="value-74696d657374616d702d616e642d6f66667365742d66726f6d2d737472696e67"></a>

## `timestamp-and-offset-from-string`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">timestamp-and-offset-from-string</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Iso</span>.<span class="tok-constructor">IsoError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span>, <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Offset</span>.<span class="tok-constructor">Offset</span>)))</code></pre>

An RFC 3339 timestamp as the instant and the offset it was written
at (strict: Text.Time.Iso's grammar).

[Source](/reference-sources/Text.Time.Zoned.cinch) · line 140

<a id="value-74696d657374616d702d616e642d6f66667365742d66726f6d2d737472696e672d6c656e69656e74"></a>

## `timestamp-and-offset-from-string-lenient`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">timestamp-and-offset-from-string-lenient</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Iso</span>.<span class="tok-constructor">IsoError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span>, <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Offset</span>.<span class="tok-constructor">Offset</span>)))</code></pre>

[Source](/reference-sources/Text.Time.Zoned.cinch)

<a id="value-74696d657374616d702d7374726674696d652d6174"></a>

## `timestamp-strftime-at`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">timestamp-strftime-at</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Offset</span>.<span class="tok-constructor">Offset</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">StrtimeError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))))</code></pre>

The instant read at `o`: `%z` prints the offset; `%Z` and `%Q` have
no zone to name (`%Q` prints the offset).

[Source](/reference-sources/Text.Time.Zoned.cinch) · line 182

<a id="value-74696d657374616d702d746f2d737472696e672d6174"></a>

## `timestamp-to-string-at`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">timestamp-to-string-at</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Offset</span>.<span class="tok-constructor">Offset</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

The civil reading of the instant at `o`, then the numeric offset
(`+00:00` for UTC, never `Z`), with the shortest exact fraction.
Total (header).

[Source](/reference-sources/Text.Time.Zoned.cinch) · line 120

<a id="value-74696d657374616d702d746f2d737472696e672d61742d707265636973696f6e"></a>

## `timestamp-to-string-at-precision`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">timestamp-to-string-at-precision</span> :: ((<span class="tok-variable">digits</span> :: (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">0</span> <span class="tok-number">9</span> <span class="tok-constructor">I64</span>)) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Offset</span>.<span class="tok-constructor">Offset</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)))</code></pre>

Exactly `digits` fraction digits, truncating.

[Source](/reference-sources/Text.Time.Zoned.cinch) · line 126

<a id="value-7a6f6e65642d66726f6d2d737472696e67"></a>

## `zoned-from-string`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zoned-from-string</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">TzDb</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">ZonedTextError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span>)))</code></pre>

Parse (strict), look the zone up, resolve with `OffsetReject` and
the compatible strategy.

[Source](/reference-sources/Text.Time.Zoned.cinch) · line 244

<a id="value-7a6f6e65642d66726f6d2d737472696e672d6c656e69656e74"></a>

## `zoned-from-string-lenient`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zoned-from-string-lenient</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">TzDb</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">ZonedTextError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span>)))</code></pre>

The same with the lenient grammar (a date with no time is the
first instant of that day in the zone).

[Source](/reference-sources/Text.Time.Zoned.cinch) · line 249

<a id="value-7a6f6e65642d66726f6d2d737472696e672d77697468"></a>

## `zoned-from-string-with`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zoned-from-string-with</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc9557</span>.<span class="tok-constructor">OffsetConflict</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Types</span>.<span class="tok-constructor">Disambiguation</span> -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">TzDb</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">ZonedTextError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span>)))))</code></pre>

Strict, with the conflict policy and the strategy chosen.

[Source](/reference-sources/Text.Time.Zoned.cinch) · line 255

<a id="value-7a6f6e65642d7374726674696d65"></a>

## `zoned-strftime`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zoned-strftime</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">StrtimeError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)))</code></pre>

[Source](/reference-sources/Text.Time.Zoned.cinch)

<a id="value-7a6f6e65642d7374726674696d652d666f726d6174"></a>

## `zoned-strftime-format`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zoned-strftime-format</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">Format</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Strtime</span>.<span class="tok-constructor">StrtimeError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)))</code></pre>

With a format compiled once (`strtime-format-compile`).

[Source](/reference-sources/Text.Time.Zoned.cinch) · line 177

<a id="value-7a6f6e65642d7374727074696d65"></a>

## `zoned-strptime`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zoned-strptime</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">TzDb</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">ZonedTextError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span>))))</code></pre>

`strptime`, then the zone: the `%Q` name looked up in `db` when
one was read, otherwise the `%z` offset as a fixed zone, otherwise
`ZonedTextMissingZone`. (`%Q` also reads a numeric offset, which
arrives here as the offset field.)

[Source](/reference-sources/Text.Time.Zoned.cinch) · line 271

<a id="value-7a6f6e65642d746578742d6c6f6f6b7570"></a>

## `zoned-text-lookup`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zoned-text-lookup</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">TzDb</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc9557</span>.<span class="tok-constructor">ZonedText</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">TzLookupError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">TimeZone</span>)))</code></pre>

The zone a text's annotation names: a database lookup for a name,
`zone-fixed` (no lookup) for an offset.

[Source](/reference-sources/Text.Time.Zoned.cinch) · line 237

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Text/Time/Zoned.cinch`. Source SHA-256: `f4942fd2b7cc3f238527aed3bce3d0a8df17178dbe40fba507c21da52ab15e5b`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
