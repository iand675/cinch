# Data.Time.Zone

Time zones: offsets for instants, resolution of civil times, and fixed or rule-based zones.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Data.Time.Zone`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Time.Zone.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Algebra.Arithmetic](Algebra.Arithmetic.md), [Data.Bytes](Data.Bytes.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Data.Time.DateTime](Data.Time.DateTime.md), [Data.Time.Offset](Data.Time.Offset.md), [Data.Time.Timestamp](Data.Time.Timestamp.md), [Data.Time.Zone.Posix](Data.Time.Zone.Posix.md), [Data.Time.Zone.Types](Data.Time.Zone.Types.md), [Relation.Eq](Relation.Eq.md), [Relation.Ord](Relation.Ord.md).

Non-embedded source closure: [Data.Time.Date](Data.Time.Date.md), [Data.Time.DateTime](Data.Time.DateTime.md), [Data.Time.Duration](Data.Time.Duration.md), [Data.Time.Offset](Data.Time.Offset.md), [Data.Time.TimeOfDay](Data.Time.TimeOfDay.md), [Data.Time.Timestamp](Data.Time.Timestamp.md), [Data.Time.Weekday](Data.Time.Weekday.md), [Data.Time.Zone](Data.Time.Zone.md), [Data.Time.Zone.Posix](Data.Time.Zone.Posix.md), [Data.Time.Zone.Types](Data.Time.Zone.Types.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Data.Time.Zone: time zones
(design/datetime.md, layer L2, lane C).

A `TimeZone` is the pair of mappings between instants and civil
readings in one place: every instant has exactly one offset
(`zone-offset-at`), and a civil reading has zero, one or two
(`zone-to-ambiguous`, T4). A zone is one of
  - a fixed offset (`zone-fixed`, `zone-utc`),
  - a table of transitions read from a TZif file
    (`Data.Time.Zone.Tzif`), with an optional POSIX rule for the
    instants after the table's last transition, or
  - a POSIX rule alone (`zone-posix`).
Getting a zone by name is an effect and lives in `Effect.Time.Zone`
(T5); everything here is pure. This module re-exports
`Data.Time.Zone.Types` (`Ambiguous`, `Disambiguation`, `OffsetInfo`
and their eliminators).

REPRESENTATION of a table zone. The transition instants are Unix
seconds in one packed `Array I64` (`Data.Array.Packed`'s
`array-from-i64s`: eight bytes an element, no per-element object),
each transition's local time type index is a byte in a packed
`Array U8`, and the local time types are a small boxed
`Array OffsetInfo` (at most 256 entries) so a lookup returns the
table's own `OffsetInfo` without building one. A lookup is a binary
search over the instants. `array-get` is the only reader the
compiler offers for an `Array` and it answers `Option`, so each
probe passes through an `Option I64`; nothing else is built.
`Data.Array` and `Data.Array.Packed` are "ship source" modules (not
in the embedded base package), so importing them would oblige every
user of the datetime family to copy them beside their entry file;
this module instead binds the four runtime entry points it needs
privately, under the names and types those modules give them.

Design law shape. For a table of transitions `t_0 < t_1 < ...`
with types `k_i` (RFC 9636): an instant before `t_0` has local time
type 0; an instant in `[t_i, t_i+1)` has type `k_i`; an instant at
or after the last transition follows the POSIX rule when there is
one, and type `k_last` otherwise. `zone-to-datetime z t` is the
civil reading of `t` at `zone-offset-at z t`. `zone-to-ambiguous`
is derived from the same instant-side lookups
(`ambiguous-from-lookups`), so: a reading is `Unambiguous o`
exactly when one instant has it as its civil reading (and that
instant's offset is `o`), a `Fold` when two do, and a `Gap` when
none does; and for every instant `t`, resolving
`zone-to-datetime z t` with any strategy but `Reject` gives an
instant with the same civil reading. `zone-next-transition` and
`zone-previous-transition` are strict on both sides.

NAMED REFUSALS:
  - a default disambiguation (T4): `zone-to-timestamp` takes the
    strategy and the four named forms carry it in their names.
  - lookup by name (T5): `Effect.Time.Zone`.
  - leap seconds: a table zone is on the Unix time scale.
  - a "transition" is whatever the table or rule lists, including
    one that changes only the abbreviation or the daylight flag;
    they are not filtered.
  - an unboxed probe: see REPRESENTATION.
  - `Eq` on `TimeZone`: two zones with the same rules under
    different names are neither clearly equal nor clearly distinct;
    `Data.Time.Zoned` compares names and says so.

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

<a id="type-54696d655a6f6e65"></a>

## `TimeZone`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">TimeZone</span></code></pre>

A zone (header). The constructors are not exported: a table zone
is only made by `zone-from-table`, which checks it.

[Source](/reference-sources/Data.Time.Zone.cinch) · line 106

<a id="type-5a6f6e655461626c654572726f72"></a>

## `ZoneTableError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">ZoneTableError</span></code></pre>

Why `zone-from-table` refused a table.

[Source](/reference-sources/Data.Time.Zone.cinch) · line 117

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ZoneNoTypes</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">ZoneTableError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ZoneLengthMismatch</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">ZoneTableError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ZoneUnsortedTransitions</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">ZoneTableError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ZoneTypeIndexOutOfRange</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">ZoneTableError</span></code></pre>

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

<a id="value-7a6f6e652d6669786564"></a>

## `zone-fixed`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zone-fixed</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Offset</span>.<span class="tok-constructor">Offset</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">TimeZone</span>)</code></pre>

A fixed offset. Its name and abbreviation are the offset as
`+hh:mm` (`+hh:mm:ss` when the seconds are not zero); offset zero
is `zone-utc`.

[Source](/reference-sources/Data.Time.Zone.cinch) · line 138

<a id="value-7a6f6e652d66697865642d6e616d6564"></a>

## `zone-fixed-named`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zone-fixed-named</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Offset</span>.<span class="tok-constructor">Offset</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">TimeZone</span>))</code></pre>

A fixed offset under a name of the caller's choosing (also its
abbreviation).

[Source](/reference-sources/Data.Time.Zone.cinch) · line 147

<a id="value-7a6f6e652d66697865642d6f6666736574"></a>

## `zone-fixed-offset`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zone-fixed-offset</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">TimeZone</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Offset</span>.<span class="tok-constructor">Offset</span>))</code></pre>

The offset of a fixed-offset zone.

[Source](/reference-sources/Data.Time.Zone.cinch) · line 213

<a id="value-7a6f6e652d66726f6d2d7461626c65"></a>

## `zone-from-table`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zone-from-table</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">I64</span>) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">I64</span>) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Types</span>.<span class="tok-constructor">OffsetInfo</span>) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Posix</span>.<span class="tok-constructor">PosixRule</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">ZoneTableError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">TimeZone</span>))))))</code></pre>

A table zone (the header's law). `instants` are Unix seconds and
must be strictly ascending; `indices` gives each transition's
local time type as an index into `types`, which must not be
empty and whose first element is the type before the first
transition; `tail` is the rule after the last transition.

[Source](/reference-sources/Data.Time.Zone.cinch) · line 159

<a id="value-7a6f6e652d696e666f2d6174"></a>

## `zone-info-at`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zone-info-at</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">TimeZone</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Types</span>.<span class="tok-constructor">OffsetInfo</span>))</code></pre>

Offset, daylight flag and abbreviation in force at an instant.

[Source](/reference-sources/Data.Time.Zone.cinch) · line 322

<a id="value-7a6f6e652d696e666f2d61742d756e69782d7365636f6e6473"></a>

## `zone-info-at-unix-seconds`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zone-info-at-unix-seconds</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">TimeZone</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Types</span>.<span class="tok-constructor">OffsetInfo</span>))</code></pre>

What the zone says at Unix second `t` (the header's law).

[Source](/reference-sources/Data.Time.Zone.cinch) · line 268

<a id="value-7a6f6e652d6e616d65"></a>

## `zone-name`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zone-name</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">TimeZone</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

The zone's name: the IANA name it was looked up under, "UTC", a
fixed offset's label, or whatever its maker called it.

[Source](/reference-sources/Data.Time.Zone.cinch) · line 206

<a id="value-7a6f6e652d6e6578742d7472616e736974696f6e"></a>

## `zone-next-transition`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zone-next-transition</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">TimeZone</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span>)))</code></pre>

The first transition strictly after `t`; `None` when there is
none (a fixed zone, a table without a rule past its end) or it is
outside the timestamp range.

[Source](/reference-sources/Data.Time.Zone.cinch) · line 374

<a id="value-7a6f6e652d6e6578742d7472616e736974696f6e2d756e69782d7365636f6e6473"></a>

## `zone-next-transition-unix-seconds`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zone-next-transition-unix-seconds</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">TimeZone</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">I64</span>)))</code></pre>

The first transition strictly after Unix second `t`.

[Source](/reference-sources/Data.Time.Zone.cinch) · line 281

<a id="value-7a6f6e652d6f66667365742d6174"></a>

## `zone-offset-at`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zone-offset-at</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">TimeZone</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Offset</span>.<span class="tok-constructor">Offset</span>))</code></pre>

[Source](/reference-sources/Data.Time.Zone.cinch)

<a id="value-7a6f6e652d706f736978"></a>

## `zone-posix`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zone-posix</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Posix</span>.<span class="tok-constructor">PosixRule</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">TimeZone</span>))</code></pre>

A zone that follows a POSIX rule for every instant.

[Source](/reference-sources/Data.Time.Zone.cinch) · line 151

<a id="value-7a6f6e652d70726576696f75732d7472616e736974696f6e"></a>

## `zone-previous-transition`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zone-previous-transition</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">TimeZone</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span>)))</code></pre>

The last transition strictly before `t`. A transition at `t`'s
own whole second is before `t` exactly when `t` has a fractional
part. The seconds are within +-3.8e11, so the increment cannot
wrap.

[Source](/reference-sources/Data.Time.Zone.cinch) · line 383

<a id="value-7a6f6e652d70726576696f75732d7472616e736974696f6e2d756e69782d7365636f6e6473"></a>

## `zone-previous-transition-unix-seconds`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zone-previous-transition-unix-seconds</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">TimeZone</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">I64</span>)))</code></pre>

The last transition strictly before Unix second `t`. Past the
table's end it is the rule's, unless the table's last transition
is later than that.

[Source](/reference-sources/Data.Time.Zone.cinch) · line 295

<a id="value-7a6f6e652d746f2d616d626967756f7573"></a>

## `zone-to-ambiguous`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zone-to-ambiguous</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">TimeZone</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Types</span>.<span class="tok-constructor">Ambiguous</span>))</code></pre>

The offsets a civil reading can have in the zone (T4).

[Source](/reference-sources/Data.Time.Zone.cinch) · line 340

<a id="value-7a6f6e652d746f2d6461746574696d65"></a>

## `zone-to-datetime`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zone-to-datetime</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">TimeZone</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span>))</code></pre>

The civil reading of an instant in the zone.

[Source](/reference-sources/Data.Time.Zone.cinch) · line 329

<a id="value-7a6f6e652d746f2d74696d657374616d70"></a>

## `zone-to-timestamp`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zone-to-timestamp</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Types</span>.<span class="tok-constructor">Disambiguation</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">TimeZone</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Types</span>.<span class="tok-constructor">AmbiguityError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span>))))</code></pre>

The instant of a civil reading, by the given strategy
(`Data.Time.Zone.Types`' table).

[Source](/reference-sources/Data.Time.Zone.cinch) · line 348

<a id="value-7a6f6e652d746f2d74696d657374616d702d636f6d70617469626c65"></a>

## `zone-to-timestamp-compatible`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zone-to-timestamp-compatible</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">TimeZone</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Types</span>.<span class="tok-constructor">AmbiguityError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span>)))</code></pre>

In a gap the later instant, in a fold the earlier one.

[Source](/reference-sources/Data.Time.Zone.cinch) · line 352

<a id="value-7a6f6e652d746f2d74696d657374616d702d6561726c696572"></a>

## `zone-to-timestamp-earlier`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zone-to-timestamp-earlier</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">TimeZone</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Types</span>.<span class="tok-constructor">AmbiguityError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span>)))</code></pre>

The earlier instant in a gap and in a fold.

[Source](/reference-sources/Data.Time.Zone.cinch) · line 356

<a id="value-7a6f6e652d746f2d74696d657374616d702d6c61746572"></a>

## `zone-to-timestamp-later`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zone-to-timestamp-later</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">TimeZone</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Types</span>.<span class="tok-constructor">AmbiguityError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span>)))</code></pre>

The later instant in a gap and in a fold.

[Source](/reference-sources/Data.Time.Zone.cinch) · line 360

<a id="value-7a6f6e652d746f2d74696d657374616d702d72656a656374"></a>

## `zone-to-timestamp-reject`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zone-to-timestamp-reject</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">TimeZone</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Types</span>.<span class="tok-constructor">AmbiguityError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span>)))</code></pre>

Refuses a gap and a fold.

[Source](/reference-sources/Data.Time.Zone.cinch) · line 364

<a id="value-7a6f6e652d7472616e736974696f6e2d636f756e74"></a>

## `zone-transition-count`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zone-transition-count</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">TimeZone</span> -&gt; <span class="tok-constructor">I64</span>)</code></pre>

How many transitions a table zone lists (0 for the other kinds).

[Source](/reference-sources/Data.Time.Zone.cinch) · line 220

<a id="value-7a6f6e652d757463"></a>

## `zone-utc`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zone-utc</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">TimeZone</span></code></pre>

UTC: offset zero, named and abbreviated "UTC".

[Source](/reference-sources/Data.Time.Zone.cinch) · line 132

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Data/Time/Zone.cinch`. Source SHA-256: `b97d1abcfcca9d51d317ac70f92284ebe6b3c408d260d1364bc25d73a0548415`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
