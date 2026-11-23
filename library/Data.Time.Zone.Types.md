# Data.Time.Zone.Types

Shared vocabulary for time zones: transitions, ambiguity and disambiguation policies.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Data.Time.Zone.Types`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Time.Zone.Types.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Machine](Data.Machine.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.Time.DateTime](Data.Time.DateTime.md), [Data.Time.Offset](Data.Time.Offset.md), [Data.Time.Timestamp](Data.Time.Timestamp.md), [Relation.Eq](Relation.Eq.md).

Non-embedded source closure: [Data.Time.Date](Data.Time.Date.md), [Data.Time.DateTime](Data.Time.DateTime.md), [Data.Time.Duration](Data.Time.Duration.md), [Data.Time.Offset](Data.Time.Offset.md), [Data.Time.TimeOfDay](Data.Time.TimeOfDay.md), [Data.Time.Timestamp](Data.Time.Timestamp.md), [Data.Time.Weekday](Data.Time.Weekday.md), [Data.Time.Zone.Types](Data.Time.Zone.Types.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Data.Time.Zone.Types: the vocabulary the zone modules share
(design/datetime.md, layer L2, lane C).

`Data.Time.Zone` imports `Data.Time.Zone.Posix` (a zone's tail is a
POSIX rule), and both need `OffsetInfo` and `Ambiguous`, so the
shared types live here and `Data.Time.Zone` re-exports this whole
module. Import `Data.Time.Zone` unless you are writing a zone
module.

The two mappings a time zone is (T4):
  - instant to offset: total, an `OffsetInfo` (the offset, whether
    it is daylight saving time, and the abbreviation such as "EST").
  - civil reading to offset: `Ambiguous`. A reading is
    `Unambiguous o` (it occurs once, at offset `o`), a
    `Gap before after` (it never occurs: the clocks jumped from
    offset `before` to `after` over it), or a `Fold before after`
    (it occurs twice: first at offset `before`, then at `after`).

Design law shape. Ambiguity is a value; a `Disambiguation` is an
eliminator for it, never applied silently. With `at o x` the
instant whose civil reading at offset `o` is `x`
(`offset-datetime-to-timestamp`), `ambiguous-resolve s x a` is, as
Temporal and Jiff define the four strategies:

  a               Compatible     Earlier        Later          Reject
  Unambiguous o   at o x         at o x         at o x         at o x
  Gap b a         at b x         at a x         at b x         Err
  Fold b a        at b x         at b x         at a x         Err

In a gap the reading does not exist, so it is read with the offset
that was in force on one side of the jump: reading it with the
offset BEFORE the gap gives the later instant (the reading shifted
forward by the gap's length, shown with the offset after it:
02:30 in New York's spring gap becomes 03:30 -04:00), and reading it
with the offset AFTER the gap gives the earlier instant (01:30
-05:00). In a fold the first occurrence has the offset before the
transition. "Compatible" (RFC 5545's rule) is the later instant in
a gap and the earlier one in a fold.
`ambiguous-from-lookups` derives the `Ambiguous` of a reading from
a zone's two instant-side lookups and is the one implementation
every kind of zone uses, so a gap or fold is exactly the set of
readings that zero or two instants map back to.

NAMED REFUSALS:
  - a default strategy: no function here picks one silently.
  - an instant outside the timestamp range: `AmbiguityOutOfRange`
    (only within 26 hours of the ends of the civil range).
  - sub-second offsets and leap seconds: as `Data.Time.Offset`.

</details>

# Types and constructors

<a id="type-416d626967756974794572726f72"></a>

## `AmbiguityError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">AmbiguityError</span></code></pre>

Why a civil reading was not turned into an instant: `Reject` met a
gap or a fold (the two offsets are the `Ambiguous` value's), or
the instant is outside the timestamp range.

[Source](/reference-sources/Data.Time.Zone.Types.cinch) · line 111

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">RejectedGap</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Offset</span>.<span class="tok-constructor">Offset</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Offset</span>.<span class="tok-constructor">Offset</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Types</span>.<span class="tok-constructor">AmbiguityError</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">RejectedFold</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Offset</span>.<span class="tok-constructor">Offset</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Offset</span>.<span class="tok-constructor">Offset</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Types</span>.<span class="tok-constructor">AmbiguityError</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">AmbiguityOutOfRange</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Types</span>.<span class="tok-constructor">AmbiguityError</span></code></pre>

<a id="type-416d626967756f7573"></a>

## `Ambiguous`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Ambiguous</span></code></pre>

The offsets a civil reading can have (T4). `Gap before after` and
`Fold before after` name the offset in force before the
transition and the one after it.

[Source](/reference-sources/Data.Time.Zone.Types.cinch) · line 96

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Unambiguous</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Offset</span>.<span class="tok-constructor">Offset</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Types</span>.<span class="tok-constructor">Ambiguous</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Gap</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Offset</span>.<span class="tok-constructor">Offset</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Offset</span>.<span class="tok-constructor">Offset</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Types</span>.<span class="tok-constructor">Ambiguous</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Fold</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Offset</span>.<span class="tok-constructor">Offset</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Offset</span>.<span class="tok-constructor">Offset</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Types</span>.<span class="tok-constructor">Ambiguous</span>))</code></pre>

<a id="type-446973616d626967756174696f6e"></a>

## `Disambiguation`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Disambiguation</span></code></pre>

How to choose (the header's table).

[Source](/reference-sources/Data.Time.Zone.Types.cinch) · line 102

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Compatible</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Types</span>.<span class="tok-constructor">Disambiguation</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Earlier</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Types</span>.<span class="tok-constructor">Disambiguation</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Later</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Types</span>.<span class="tok-constructor">Disambiguation</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Reject</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Types</span>.<span class="tok-constructor">Disambiguation</span></code></pre>

<a id="type-4f6666736574496e666f"></a>

## `OffsetInfo`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">OffsetInfo</span></code></pre>

What a zone says about one instant: the offset in force, whether
it is daylight saving time, and the abbreviation ("EST", "+0330").
Abstract; `offset-info` builds one (any three values are valid).

[Source](/reference-sources/Data.Time.Zone.Types.cinch) · line 75

# Values

<a id="value-616d626967756f75732d636f6d70617469626c65"></a>

## `ambiguous-compatible`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ambiguous-compatible</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Types</span>.<span class="tok-constructor">Ambiguous</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Types</span>.<span class="tok-constructor">AmbiguityError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span>)))</code></pre>

The four strategies by name.

[Source](/reference-sources/Data.Time.Zone.Types.cinch) · line 182

<a id="value-616d626967756f75732d6561726c696572"></a>

## `ambiguous-earlier`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ambiguous-earlier</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Types</span>.<span class="tok-constructor">Ambiguous</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Types</span>.<span class="tok-constructor">AmbiguityError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span>)))</code></pre>

[Source](/reference-sources/Data.Time.Zone.Types.cinch)

<a id="value-616d626967756f75732d66726f6d2d6c6f6f6b757073"></a>

## `ambiguous-from-lookups`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ambiguous-from-lookups</span> :: ((<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Offset</span>.<span class="tok-constructor">Offset</span>) -&gt; ((<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">I64</span>)) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Types</span>.<span class="tok-constructor">Ambiguous</span>)))</code></pre>

`ambiguous-from-lookups offset-at next x`: for a zone given by
`offset-at` (the offset in force at a Unix second) and `next` (the
first transition instant strictly after a Unix second, if any).

With `l` the reading's position on the civil line in seconds, a
period of constant offset `o` running over the instants `[s, e)`
contains the reading exactly when `s <= l - o < e`. Offsets are
within 93599 seconds of UTC, so the walk starts at the period
holding the instant `l - 93599`, which the reading cannot precede,
and steps forward while the reading is past the period's end. At
the first period it is not past: if it is at or after the
period's start it occurs there (and occurs again, a fold, when it
is also at or after the start of the next period read with that
period's offset); if it is before the start it fell between two
periods, a gap. A transition that does not change the offset can
produce neither. Each step moves to a strictly later transition
and stops at the latest once the period's end passes `l + 93599`,
so the walk ends. All sums are of Unix seconds within +-3.8e11
and offsets, so none wraps.

[Source](/reference-sources/Data.Time.Zone.Types.cinch) · line 217

<a id="value-616d626967756f75732d69732d616d626967756f7573"></a>

## `ambiguous-is-ambiguous`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ambiguous-is-ambiguous</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Types</span>.<span class="tok-constructor">Ambiguous</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

A gap or a fold.

[Source](/reference-sources/Data.Time.Zone.Types.cinch) · line 121

<a id="value-616d626967756f75732d69732d666f6c64"></a>

## `ambiguous-is-fold`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ambiguous-is-fold</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Types</span>.<span class="tok-constructor">Ambiguous</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

[Source](/reference-sources/Data.Time.Zone.Types.cinch)

<a id="value-616d626967756f75732d69732d676170"></a>

## `ambiguous-is-gap`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ambiguous-is-gap</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Types</span>.<span class="tok-constructor">Ambiguous</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

[Source](/reference-sources/Data.Time.Zone.Types.cinch)

<a id="value-616d626967756f75732d6c61746572"></a>

## `ambiguous-later`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ambiguous-later</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Types</span>.<span class="tok-constructor">Ambiguous</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Types</span>.<span class="tok-constructor">AmbiguityError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span>)))</code></pre>

[Source](/reference-sources/Data.Time.Zone.Types.cinch)

<a id="value-616d626967756f75732d6f6666736574"></a>

## `ambiguous-offset`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ambiguous-offset</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Types</span>.<span class="tok-constructor">Disambiguation</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Types</span>.<span class="tok-constructor">Ambiguous</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Offset</span>.<span class="tok-constructor">Offset</span>)))</code></pre>

The offset the strategy reads the civil value with (the header's
table), or `None` when `Reject` meets a gap or a fold. Note that
for a gap this is the offset the reading is INTERPRETED with; the
resulting instant then displays with the other one.

[Source](/reference-sources/Data.Time.Zone.Types.cinch) · line 150

<a id="value-616d626967756f75732d72656a656374"></a>

## `ambiguous-reject`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ambiguous-reject</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Types</span>.<span class="tok-constructor">Ambiguous</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Types</span>.<span class="tok-constructor">AmbiguityError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span>)))</code></pre>

[Source](/reference-sources/Data.Time.Zone.Types.cinch)

<a id="value-616d626967756f75732d7265736f6c7665"></a>

## `ambiguous-resolve`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ambiguous-resolve</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Types</span>.<span class="tok-constructor">Disambiguation</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Types</span>.<span class="tok-constructor">Ambiguous</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Types</span>.<span class="tok-constructor">AmbiguityError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span>))))</code></pre>

[Source](/reference-sources/Data.Time.Zone.Types.cinch)

<a id="value-616d626967756f75732d756e616d626967756f7573"></a>

## `ambiguous-unambiguous`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ambiguous-unambiguous</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Types</span>.<span class="tok-constructor">Ambiguous</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Offset</span>.<span class="tok-constructor">Offset</span>))</code></pre>

The one offset, when there is exactly one.

[Source](/reference-sources/Data.Time.Zone.Types.cinch) · line 140

<a id="value-6f66667365742d696e666f"></a>

## `offset-info`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">offset-info</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Offset</span>.<span class="tok-constructor">Offset</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Types</span>.<span class="tok-constructor">OffsetInfo</span>)))</code></pre>

[Source](/reference-sources/Data.Time.Zone.Types.cinch)

<a id="value-6f66667365742d696e666f2d616262726576696174696f6e"></a>

## `offset-info-abbreviation`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">offset-info-abbreviation</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Types</span>.<span class="tok-constructor">OffsetInfo</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

[Source](/reference-sources/Data.Time.Zone.Types.cinch)

<a id="value-6f66667365742d696e666f2d69732d647374"></a>

## `offset-info-is-dst`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">offset-info-is-dst</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Types</span>.<span class="tok-constructor">OffsetInfo</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

[Source](/reference-sources/Data.Time.Zone.Types.cinch)

<a id="value-6f66667365742d696e666f2d6f6666736574"></a>

## `offset-info-offset`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">offset-info-offset</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Types</span>.<span class="tok-constructor">OffsetInfo</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Offset</span>.<span class="tok-constructor">Offset</span>)</code></pre>

[Source](/reference-sources/Data.Time.Zone.Types.cinch)

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Data/Time/Zone/Types.cinch`. Source SHA-256: `9ae9aa1e61e7319241e258b3bea890069e31bc1c35c689e1f361f2db050b1ebe`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
