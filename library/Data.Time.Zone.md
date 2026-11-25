# Data.Time.Zone

Time zones: offsets for instants, resolution of civil times, and fixed or rule-based zones.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Data.Time.Zone</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Data.Time.Zone/source/">View source</a><a href="/packages/search/?q=%2BData.Time.Zone">Search module</a><a href="/reference-sources/Data.Time.Zone.cinch" download="Data.Time.Zone.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Algebra.Arithmetic](Algebra.Arithmetic.md), [Data.Bytes](Data.Bytes.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Data.Time.DateTime](Data.Time.DateTime.md), [Data.Time.Offset](Data.Time.Offset.md), [Data.Time.Timestamp](Data.Time.Timestamp.md), [Data.Time.Zone.Posix](Data.Time.Zone.Posix.md), [Data.Time.Zone.Types](Data.Time.Zone.Types.md), [Relation.Eq](Relation.Eq.md), [Relation.Ord](Relation.Ord.md).

<details class="doc-notes"><summary>Module notes</summary>

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

<details class="doc-synopsis"><summary>Synopsis (43 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-AmbiguityError">AmbiguityError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">AmbiguityError</strong></code></li><li><a class="syn-name" href="#t-Ambiguous">Ambiguous</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Ambiguous</strong></code></li><li><a class="syn-name" href="#t-Disambiguation">Disambiguation</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Disambiguation</strong></code></li><li><a class="syn-name" href="#t-OffsetInfo">OffsetInfo</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">OffsetInfo</strong></code></li><li><a class="syn-name" href="#t-TimeZone">TimeZone</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">TimeZone</strong></code></li><li><a class="syn-name" href="#t-ZoneTableError">ZoneTableError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ZoneTableError</strong></code></li><li><a class="syn-name" href="#v-ambiguous-compatible">ambiguous-compatible</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ambiguous-compatible</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-Ambiguous" title="Data.Time.Zone.Types.Ambiguous">Ambiguous</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-AmbiguityError" title="Data.Time.Zone.Types.AmbiguityError">AmbiguityError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a></code></li><li><a class="syn-name" href="#v-ambiguous-earlier">ambiguous-earlier</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ambiguous-earlier</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-Ambiguous" title="Data.Time.Zone.Types.Ambiguous">Ambiguous</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-AmbiguityError" title="Data.Time.Zone.Types.AmbiguityError">AmbiguityError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a></code></li><li><a class="syn-name" href="#v-ambiguous-from-lookups">ambiguous-from-lookups</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ambiguous-from-lookups</strong> :: (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a>) <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-Ambiguous" title="Data.Time.Zone.Types.Ambiguous">Ambiguous</a></code></li><li><a class="syn-name" href="#v-ambiguous-is-ambiguous">ambiguous-is-ambiguous</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ambiguous-is-ambiguous</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-Ambiguous" title="Data.Time.Zone.Types.Ambiguous">Ambiguous</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-ambiguous-is-fold">ambiguous-is-fold</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ambiguous-is-fold</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-Ambiguous" title="Data.Time.Zone.Types.Ambiguous">Ambiguous</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-ambiguous-is-gap">ambiguous-is-gap</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ambiguous-is-gap</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-Ambiguous" title="Data.Time.Zone.Types.Ambiguous">Ambiguous</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-ambiguous-later">ambiguous-later</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ambiguous-later</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-Ambiguous" title="Data.Time.Zone.Types.Ambiguous">Ambiguous</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-AmbiguityError" title="Data.Time.Zone.Types.AmbiguityError">AmbiguityError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a></code></li><li><a class="syn-name" href="#v-ambiguous-offset">ambiguous-offset</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ambiguous-offset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-Disambiguation" title="Data.Time.Zone.Types.Disambiguation">Disambiguation</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-Ambiguous" title="Data.Time.Zone.Types.Ambiguous">Ambiguous</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a></code></li><li><a class="syn-name" href="#v-ambiguous-reject">ambiguous-reject</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ambiguous-reject</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-Ambiguous" title="Data.Time.Zone.Types.Ambiguous">Ambiguous</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-AmbiguityError" title="Data.Time.Zone.Types.AmbiguityError">AmbiguityError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a></code></li><li><a class="syn-name" href="#v-ambiguous-resolve">ambiguous-resolve</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ambiguous-resolve</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-Disambiguation" title="Data.Time.Zone.Types.Disambiguation">Disambiguation</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-Ambiguous" title="Data.Time.Zone.Types.Ambiguous">Ambiguous</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-AmbiguityError" title="Data.Time.Zone.Types.AmbiguityError">AmbiguityError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a></code></li><li><a class="syn-name" href="#v-ambiguous-unambiguous">ambiguous-unambiguous</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ambiguous-unambiguous</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-Ambiguous" title="Data.Time.Zone.Types.Ambiguous">Ambiguous</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a></code></li><li><a class="syn-name" href="#v-offset-info">offset-info</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">offset-info</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-OffsetInfo" title="Data.Time.Zone.Types.OffsetInfo">OffsetInfo</a></code></li><li><a class="syn-name" href="#v-offset-info-abbreviation">offset-info-abbreviation</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">offset-info-abbreviation</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-OffsetInfo" title="Data.Time.Zone.Types.OffsetInfo">OffsetInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-offset-info-is-dst">offset-info-is-dst</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">offset-info-is-dst</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-OffsetInfo" title="Data.Time.Zone.Types.OffsetInfo">OffsetInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-offset-info-offset">offset-info-offset</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">offset-info-offset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-OffsetInfo" title="Data.Time.Zone.Types.OffsetInfo">OffsetInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a></code></li><li><a class="syn-name" href="#v-zone-fixed">zone-fixed</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zone-fixed</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-TimeZone" title="Data.Time.Zone.TimeZone">TimeZone</a></code></li><li><a class="syn-name" href="#v-zone-fixed-named">zone-fixed-named</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zone-fixed-named</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-TimeZone" title="Data.Time.Zone.TimeZone">TimeZone</a></code></li><li><a class="syn-name" href="#v-zone-fixed-offset">zone-fixed-offset</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zone-fixed-offset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-TimeZone" title="Data.Time.Zone.TimeZone">TimeZone</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a></code></li><li><a class="syn-name" href="#v-zone-from-table">zone-from-table</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zone-from-table</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-OffsetInfo" title="Data.Time.Zone.Types.OffsetInfo">OffsetInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone.Posix/#t-PosixRule" title="Data.Time.Zone.Posix.PosixRule">PosixRule</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-ZoneTableError" title="Data.Time.Zone.ZoneTableError">ZoneTableError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-TimeZone" title="Data.Time.Zone.TimeZone">TimeZone</a></code></li><li><a class="syn-name" href="#v-zone-info-at">zone-info-at</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zone-info-at</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-TimeZone" title="Data.Time.Zone.TimeZone">TimeZone</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-OffsetInfo" title="Data.Time.Zone.Types.OffsetInfo">OffsetInfo</a></code></li><li><a class="syn-name" href="#v-zone-info-at-unix-seconds">zone-info-at-unix-seconds</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zone-info-at-unix-seconds</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-TimeZone" title="Data.Time.Zone.TimeZone">TimeZone</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-OffsetInfo" title="Data.Time.Zone.Types.OffsetInfo">OffsetInfo</a></code></li><li><a class="syn-name" href="#v-zone-name">zone-name</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zone-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-TimeZone" title="Data.Time.Zone.TimeZone">TimeZone</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-zone-next-transition">zone-next-transition</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zone-next-transition</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-TimeZone" title="Data.Time.Zone.TimeZone">TimeZone</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a></code></li><li><a class="syn-name" href="#v-zone-next-transition-unix-seconds">zone-next-transition-unix-seconds</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zone-next-transition-unix-seconds</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-TimeZone" title="Data.Time.Zone.TimeZone">TimeZone</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-zone-offset-at">zone-offset-at</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zone-offset-at</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-TimeZone" title="Data.Time.Zone.TimeZone">TimeZone</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a></code></li><li><a class="syn-name" href="#v-zone-posix">zone-posix</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zone-posix</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone.Posix/#t-PosixRule" title="Data.Time.Zone.Posix.PosixRule">PosixRule</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-TimeZone" title="Data.Time.Zone.TimeZone">TimeZone</a></code></li><li><a class="syn-name" href="#v-zone-previous-transition">zone-previous-transition</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zone-previous-transition</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-TimeZone" title="Data.Time.Zone.TimeZone">TimeZone</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a></code></li><li><a class="syn-name" href="#v-zone-previous-transition-unix-seconds">zone-previous-transition-unix-seconds</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zone-previous-transition-unix-seconds</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-TimeZone" title="Data.Time.Zone.TimeZone">TimeZone</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-zone-to-ambiguous">zone-to-ambiguous</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zone-to-ambiguous</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-TimeZone" title="Data.Time.Zone.TimeZone">TimeZone</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-Ambiguous" title="Data.Time.Zone.Types.Ambiguous">Ambiguous</a></code></li><li><a class="syn-name" href="#v-zone-to-datetime">zone-to-datetime</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zone-to-datetime</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-TimeZone" title="Data.Time.Zone.TimeZone">TimeZone</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a></code></li><li><a class="syn-name" href="#v-zone-to-timestamp">zone-to-timestamp</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zone-to-timestamp</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-Disambiguation" title="Data.Time.Zone.Types.Disambiguation">Disambiguation</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-TimeZone" title="Data.Time.Zone.TimeZone">TimeZone</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-AmbiguityError" title="Data.Time.Zone.Types.AmbiguityError">AmbiguityError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a></code></li><li><a class="syn-name" href="#v-zone-to-timestamp-compatible">zone-to-timestamp-compatible</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zone-to-timestamp-compatible</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-TimeZone" title="Data.Time.Zone.TimeZone">TimeZone</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-AmbiguityError" title="Data.Time.Zone.Types.AmbiguityError">AmbiguityError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a></code></li><li><a class="syn-name" href="#v-zone-to-timestamp-earlier">zone-to-timestamp-earlier</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zone-to-timestamp-earlier</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-TimeZone" title="Data.Time.Zone.TimeZone">TimeZone</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-AmbiguityError" title="Data.Time.Zone.Types.AmbiguityError">AmbiguityError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a></code></li><li><a class="syn-name" href="#v-zone-to-timestamp-later">zone-to-timestamp-later</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zone-to-timestamp-later</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-TimeZone" title="Data.Time.Zone.TimeZone">TimeZone</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-AmbiguityError" title="Data.Time.Zone.Types.AmbiguityError">AmbiguityError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a></code></li><li><a class="syn-name" href="#v-zone-to-timestamp-reject">zone-to-timestamp-reject</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zone-to-timestamp-reject</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-TimeZone" title="Data.Time.Zone.TimeZone">TimeZone</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-AmbiguityError" title="Data.Time.Zone.Types.AmbiguityError">AmbiguityError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a></code></li><li><a class="syn-name" href="#v-zone-transition-count">zone-transition-count</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zone-transition-count</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-TimeZone" title="Data.Time.Zone.TimeZone">TimeZone</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-zone-utc">zone-utc</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zone-utc</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-TimeZone" title="Data.Time.Zone.TimeZone">TimeZone</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-AmbiguityError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">AmbiguityError</strong></code><span class="doc-sig-links"><a href="#t-AmbiguityError" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zone.Types/source/#L111" title="View source at line 111">Source</a></span></div>
<div class="doc-body">

Why a civil reading was not turned into an instant: `Reject` met a
gap or a fold (the two offsets are the `Ambiguous` value's), or
the instant is outside the timestamp range.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-RejectedGap"><code><strong class="tok-constructor">RejectedGap</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-AmbiguityError" title="Data.Time.Zone.Types.AmbiguityError">AmbiguityError</a></code></li><li id="k-RejectedFold"><code><strong class="tok-constructor">RejectedFold</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-AmbiguityError" title="Data.Time.Zone.Types.AmbiguityError">AmbiguityError</a></code></li><li id="k-AmbiguityOutOfRange"><code><strong class="tok-constructor">AmbiguityOutOfRange</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-AmbiguityError" title="Data.Time.Zone.Types.AmbiguityError">AmbiguityError</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-Ambiguous">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Ambiguous</strong></code><span class="doc-sig-links"><a href="#t-Ambiguous" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zone.Types/source/#L96" title="View source at line 96">Source</a></span></div>
<div class="doc-body">

The offsets a civil reading can have (T4). `Gap before after` and
`Fold before after` name the offset in force before the
transition and the one after it.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-Unambiguous"><code><strong class="tok-constructor">Unambiguous</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-Ambiguous" title="Data.Time.Zone.Types.Ambiguous">Ambiguous</a></code></li><li id="k-Gap"><code><strong class="tok-constructor">Gap</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-Ambiguous" title="Data.Time.Zone.Types.Ambiguous">Ambiguous</a></code></li><li id="k-Fold"><code><strong class="tok-constructor">Fold</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-Ambiguous" title="Data.Time.Zone.Types.Ambiguous">Ambiguous</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-Disambiguation">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Disambiguation</strong></code><span class="doc-sig-links"><a href="#t-Disambiguation" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zone.Types/source/#L102" title="View source at line 102">Source</a></span></div>
<div class="doc-body">

How to choose (the header's table).

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-Compatible"><code><strong class="tok-constructor">Compatible</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-Disambiguation" title="Data.Time.Zone.Types.Disambiguation">Disambiguation</a></code></li><li id="k-Earlier"><code><strong class="tok-constructor">Earlier</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-Disambiguation" title="Data.Time.Zone.Types.Disambiguation">Disambiguation</a></code></li><li id="k-Later"><code><strong class="tok-constructor">Later</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-Disambiguation" title="Data.Time.Zone.Types.Disambiguation">Disambiguation</a></code></li><li id="k-Reject"><code><strong class="tok-constructor">Reject</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-Disambiguation" title="Data.Time.Zone.Types.Disambiguation">Disambiguation</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-OffsetInfo">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">OffsetInfo</strong></code><span class="doc-sig-links"><a href="#t-OffsetInfo" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zone.Types/source/#L75" title="View source at line 75">Source</a></span></div>
<div class="doc-body">

What a zone says about one instant: the offset in force, whether
it is daylight saving time, and the abbreviation ("EST", "+0330").
Abstract; `offset-info` builds one (any three values are valid).

</div>
</article>

<article class="doc-entry" id="t-TimeZone">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">TimeZone</strong></code><span class="doc-sig-links"><a href="#t-TimeZone" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zone/source/#L106" title="View source at line 106">Source</a></span></div>
<div class="doc-body">

A zone (header). The constructors are not exported: a table zone
is only made by `zone-from-table`, which checks it.

</div>
</article>

<article class="doc-entry" id="t-ZoneTableError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ZoneTableError</strong></code><span class="doc-sig-links"><a href="#t-ZoneTableError" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zone/source/#L117" title="View source at line 117">Source</a></span></div>
<div class="doc-body">

Why `zone-from-table` refused a table.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-ZoneNoTypes"><code><strong class="tok-constructor">ZoneNoTypes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-ZoneTableError" title="Data.Time.Zone.ZoneTableError">ZoneTableError</a></code></li><li id="k-ZoneLengthMismatch"><code><strong class="tok-constructor">ZoneLengthMismatch</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-ZoneTableError" title="Data.Time.Zone.ZoneTableError">ZoneTableError</a></code></li><li id="k-ZoneUnsortedTransitions"><code><strong class="tok-constructor">ZoneUnsortedTransitions</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-ZoneTableError" title="Data.Time.Zone.ZoneTableError">ZoneTableError</a></code></li><li id="k-ZoneTypeIndexOutOfRange"><code><strong class="tok-constructor">ZoneTypeIndexOutOfRange</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-ZoneTableError" title="Data.Time.Zone.ZoneTableError">ZoneTableError</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-ambiguous-compatible">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ambiguous-compatible</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-Ambiguous" title="Data.Time.Zone.Types.Ambiguous">Ambiguous</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-AmbiguityError" title="Data.Time.Zone.Types.AmbiguityError">AmbiguityError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a></code><span class="doc-sig-links"><a href="#v-ambiguous-compatible" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zone.Types/source/#L182" title="View source at line 182">Source</a></span></div>
<div class="doc-body">

The four strategies by name.

</div>
</article>

<article class="doc-entry" id="v-ambiguous-earlier">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ambiguous-earlier</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-Ambiguous" title="Data.Time.Zone.Types.Ambiguous">Ambiguous</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-AmbiguityError" title="Data.Time.Zone.Types.AmbiguityError">AmbiguityError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a></code><span class="doc-sig-links"><a href="#v-ambiguous-earlier" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zone.Types/source/#L185" title="View source at line 185">Source</a></span></div>
</article>

<article class="doc-entry" id="v-ambiguous-from-lookups">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ambiguous-from-lookups</strong> :: (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a>) <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-Ambiguous" title="Data.Time.Zone.Types.Ambiguous">Ambiguous</a></code><span class="doc-sig-links"><a href="#v-ambiguous-from-lookups" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zone.Types/source/#L217" title="View source at line 217">Source</a></span></div>
<div class="doc-body">

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

</div>
</article>

<article class="doc-entry" id="v-ambiguous-is-ambiguous">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ambiguous-is-ambiguous</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-Ambiguous" title="Data.Time.Zone.Types.Ambiguous">Ambiguous</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-ambiguous-is-ambiguous" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zone.Types/source/#L121" title="View source at line 121">Source</a></span></div>
<div class="doc-body">

A gap or a fold.

</div>
</article>

<article class="doc-entry" id="v-ambiguous-is-fold">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ambiguous-is-fold</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-Ambiguous" title="Data.Time.Zone.Types.Ambiguous">Ambiguous</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-ambiguous-is-fold" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zone.Types/source/#L133" title="View source at line 133">Source</a></span></div>
</article>

<article class="doc-entry" id="v-ambiguous-is-gap">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ambiguous-is-gap</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-Ambiguous" title="Data.Time.Zone.Types.Ambiguous">Ambiguous</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-ambiguous-is-gap" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zone.Types/source/#L127" title="View source at line 127">Source</a></span></div>
</article>

<article class="doc-entry" id="v-ambiguous-later">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ambiguous-later</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-Ambiguous" title="Data.Time.Zone.Types.Ambiguous">Ambiguous</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-AmbiguityError" title="Data.Time.Zone.Types.AmbiguityError">AmbiguityError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a></code><span class="doc-sig-links"><a href="#v-ambiguous-later" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zone.Types/source/#L188" title="View source at line 188">Source</a></span></div>
</article>

<article class="doc-entry" id="v-ambiguous-offset">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ambiguous-offset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-Disambiguation" title="Data.Time.Zone.Types.Disambiguation">Disambiguation</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-Ambiguous" title="Data.Time.Zone.Types.Ambiguous">Ambiguous</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a></code><span class="doc-sig-links"><a href="#v-ambiguous-offset" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zone.Types/source/#L150" title="View source at line 150">Source</a></span></div>
<div class="doc-body">

The offset the strategy reads the civil value with (the header's
table), or `None` when `Reject` meets a gap or a fold. Note that
for a gap this is the offset the reading is INTERPRETED with; the
resulting instant then displays with the other one.

</div>
</article>

<article class="doc-entry" id="v-ambiguous-reject">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ambiguous-reject</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-Ambiguous" title="Data.Time.Zone.Types.Ambiguous">Ambiguous</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-AmbiguityError" title="Data.Time.Zone.Types.AmbiguityError">AmbiguityError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a></code><span class="doc-sig-links"><a href="#v-ambiguous-reject" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zone.Types/source/#L191" title="View source at line 191">Source</a></span></div>
</article>

<article class="doc-entry" id="v-ambiguous-resolve">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ambiguous-resolve</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-Disambiguation" title="Data.Time.Zone.Types.Disambiguation">Disambiguation</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-Ambiguous" title="Data.Time.Zone.Types.Ambiguous">Ambiguous</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-AmbiguityError" title="Data.Time.Zone.Types.AmbiguityError">AmbiguityError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a></code><span class="doc-sig-links"><a href="#v-ambiguous-resolve" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zone.Types/source/#L173" title="View source at line 173">Source</a></span></div>
</article>

<article class="doc-entry" id="v-ambiguous-unambiguous">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ambiguous-unambiguous</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-Ambiguous" title="Data.Time.Zone.Types.Ambiguous">Ambiguous</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a></code><span class="doc-sig-links"><a href="#v-ambiguous-unambiguous" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zone.Types/source/#L140" title="View source at line 140">Source</a></span></div>
<div class="doc-body">

The one offset, when there is exactly one.

</div>
</article>

<article class="doc-entry" id="v-offset-info">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">offset-info</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-OffsetInfo" title="Data.Time.Zone.Types.OffsetInfo">OffsetInfo</a></code><span class="doc-sig-links"><a href="#v-offset-info" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zone.Types/source/#L78" title="View source at line 78">Source</a></span></div>
</article>

<article class="doc-entry" id="v-offset-info-abbreviation">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">offset-info-abbreviation</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-OffsetInfo" title="Data.Time.Zone.Types.OffsetInfo">OffsetInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-offset-info-abbreviation" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zone.Types/source/#L89" title="View source at line 89">Source</a></span></div>
</article>

<article class="doc-entry" id="v-offset-info-is-dst">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">offset-info-is-dst</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-OffsetInfo" title="Data.Time.Zone.Types.OffsetInfo">OffsetInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-offset-info-is-dst" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zone.Types/source/#L85" title="View source at line 85">Source</a></span></div>
</article>

<article class="doc-entry" id="v-offset-info-offset">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">offset-info-offset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-OffsetInfo" title="Data.Time.Zone.Types.OffsetInfo">OffsetInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a></code><span class="doc-sig-links"><a href="#v-offset-info-offset" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zone.Types/source/#L81" title="View source at line 81">Source</a></span></div>
</article>

<article class="doc-entry" id="v-zone-fixed">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zone-fixed</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-TimeZone" title="Data.Time.Zone.TimeZone">TimeZone</a></code><span class="doc-sig-links"><a href="#v-zone-fixed" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zone/source/#L138" title="View source at line 138">Source</a></span></div>
<div class="doc-body">

A fixed offset. Its name and abbreviation are the offset as
`+hh:mm` (`+hh:mm:ss` when the seconds are not zero); offset zero
is `zone-utc`.

</div>
</article>

<article class="doc-entry" id="v-zone-fixed-named">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zone-fixed-named</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-TimeZone" title="Data.Time.Zone.TimeZone">TimeZone</a></code><span class="doc-sig-links"><a href="#v-zone-fixed-named" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zone/source/#L147" title="View source at line 147">Source</a></span></div>
<div class="doc-body">

A fixed offset under a name of the caller's choosing (also its
abbreviation).

</div>
</article>

<article class="doc-entry" id="v-zone-fixed-offset">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zone-fixed-offset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-TimeZone" title="Data.Time.Zone.TimeZone">TimeZone</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a></code><span class="doc-sig-links"><a href="#v-zone-fixed-offset" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zone/source/#L213" title="View source at line 213">Source</a></span></div>
<div class="doc-body">

The offset of a fixed-offset zone.

</div>
</article>

<article class="doc-entry" id="v-zone-from-table">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zone-from-table</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-OffsetInfo" title="Data.Time.Zone.Types.OffsetInfo">OffsetInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone.Posix/#t-PosixRule" title="Data.Time.Zone.Posix.PosixRule">PosixRule</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-ZoneTableError" title="Data.Time.Zone.ZoneTableError">ZoneTableError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-TimeZone" title="Data.Time.Zone.TimeZone">TimeZone</a></code><span class="doc-sig-links"><a href="#v-zone-from-table" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zone/source/#L159" title="View source at line 159">Source</a></span></div>
<div class="doc-body">

A table zone (the header's law). `instants` are Unix seconds and
must be strictly ascending; `indices` gives each transition's
local time type as an index into `types`, which must not be
empty and whose first element is the type before the first
transition; `tail` is the rule after the last transition.

</div>
</article>

<article class="doc-entry" id="v-zone-info-at">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zone-info-at</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-TimeZone" title="Data.Time.Zone.TimeZone">TimeZone</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-OffsetInfo" title="Data.Time.Zone.Types.OffsetInfo">OffsetInfo</a></code><span class="doc-sig-links"><a href="#v-zone-info-at" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zone/source/#L322" title="View source at line 322">Source</a></span></div>
<div class="doc-body">

Offset, daylight flag and abbreviation in force at an instant.

</div>
</article>

<article class="doc-entry" id="v-zone-info-at-unix-seconds">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zone-info-at-unix-seconds</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-TimeZone" title="Data.Time.Zone.TimeZone">TimeZone</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-OffsetInfo" title="Data.Time.Zone.Types.OffsetInfo">OffsetInfo</a></code><span class="doc-sig-links"><a href="#v-zone-info-at-unix-seconds" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zone/source/#L268" title="View source at line 268">Source</a></span></div>
<div class="doc-body">

What the zone says at Unix second `t` (the header's law).

</div>
</article>

<article class="doc-entry" id="v-zone-name">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zone-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-TimeZone" title="Data.Time.Zone.TimeZone">TimeZone</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-zone-name" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zone/source/#L206" title="View source at line 206">Source</a></span></div>
<div class="doc-body">

The zone's name: the IANA name it was looked up under, "UTC", a
fixed offset's label, or whatever its maker called it.

</div>
</article>

<article class="doc-entry" id="v-zone-next-transition">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zone-next-transition</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-TimeZone" title="Data.Time.Zone.TimeZone">TimeZone</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a></code><span class="doc-sig-links"><a href="#v-zone-next-transition" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zone/source/#L374" title="View source at line 374">Source</a></span></div>
<div class="doc-body">

The first transition strictly after `t`; `None` when there is
none (a fixed zone, a table without a rule past its end) or it is
outside the timestamp range.

</div>
</article>

<article class="doc-entry" id="v-zone-next-transition-unix-seconds">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zone-next-transition-unix-seconds</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-TimeZone" title="Data.Time.Zone.TimeZone">TimeZone</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-zone-next-transition-unix-seconds" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zone/source/#L281" title="View source at line 281">Source</a></span></div>
<div class="doc-body">

The first transition strictly after Unix second `t`.

</div>
</article>

<article class="doc-entry" id="v-zone-offset-at">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zone-offset-at</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-TimeZone" title="Data.Time.Zone.TimeZone">TimeZone</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a></code><span class="doc-sig-links"><a href="#v-zone-offset-at" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zone/source/#L325" title="View source at line 325">Source</a></span></div>
</article>

<article class="doc-entry" id="v-zone-posix">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zone-posix</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone.Posix/#t-PosixRule" title="Data.Time.Zone.Posix.PosixRule">PosixRule</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-TimeZone" title="Data.Time.Zone.TimeZone">TimeZone</a></code><span class="doc-sig-links"><a href="#v-zone-posix" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zone/source/#L151" title="View source at line 151">Source</a></span></div>
<div class="doc-body">

A zone that follows a POSIX rule for every instant.

</div>
</article>

<article class="doc-entry" id="v-zone-previous-transition">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zone-previous-transition</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-TimeZone" title="Data.Time.Zone.TimeZone">TimeZone</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a></code><span class="doc-sig-links"><a href="#v-zone-previous-transition" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zone/source/#L383" title="View source at line 383">Source</a></span></div>
<div class="doc-body">

The last transition strictly before `t`. A transition at `t`'s
own whole second is before `t` exactly when `t` has a fractional
part. The seconds are within +-3.8e11, so the increment cannot
wrap.

</div>
</article>

<article class="doc-entry" id="v-zone-previous-transition-unix-seconds">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zone-previous-transition-unix-seconds</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-TimeZone" title="Data.Time.Zone.TimeZone">TimeZone</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-zone-previous-transition-unix-seconds" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zone/source/#L295" title="View source at line 295">Source</a></span></div>
<div class="doc-body">

The last transition strictly before Unix second `t`. Past the
table's end it is the rule's, unless the table's last transition
is later than that.

</div>
</article>

<article class="doc-entry" id="v-zone-to-ambiguous">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zone-to-ambiguous</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-TimeZone" title="Data.Time.Zone.TimeZone">TimeZone</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-Ambiguous" title="Data.Time.Zone.Types.Ambiguous">Ambiguous</a></code><span class="doc-sig-links"><a href="#v-zone-to-ambiguous" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zone/source/#L340" title="View source at line 340">Source</a></span></div>
<div class="doc-body">

The offsets a civil reading can have in the zone (T4).

</div>
</article>

<article class="doc-entry" id="v-zone-to-datetime">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zone-to-datetime</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-TimeZone" title="Data.Time.Zone.TimeZone">TimeZone</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a></code><span class="doc-sig-links"><a href="#v-zone-to-datetime" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zone/source/#L329" title="View source at line 329">Source</a></span></div>
<div class="doc-body">

The civil reading of an instant in the zone.

</div>
</article>

<article class="doc-entry" id="v-zone-to-timestamp">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zone-to-timestamp</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-Disambiguation" title="Data.Time.Zone.Types.Disambiguation">Disambiguation</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-TimeZone" title="Data.Time.Zone.TimeZone">TimeZone</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-AmbiguityError" title="Data.Time.Zone.Types.AmbiguityError">AmbiguityError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a></code><span class="doc-sig-links"><a href="#v-zone-to-timestamp" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zone/source/#L348" title="View source at line 348">Source</a></span></div>
<div class="doc-body">

The instant of a civil reading, by the given strategy
(`Data.Time.Zone.Types`' table).

</div>
</article>

<article class="doc-entry" id="v-zone-to-timestamp-compatible">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zone-to-timestamp-compatible</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-TimeZone" title="Data.Time.Zone.TimeZone">TimeZone</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-AmbiguityError" title="Data.Time.Zone.Types.AmbiguityError">AmbiguityError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a></code><span class="doc-sig-links"><a href="#v-zone-to-timestamp-compatible" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zone/source/#L352" title="View source at line 352">Source</a></span></div>
<div class="doc-body">

In a gap the later instant, in a fold the earlier one.

</div>
</article>

<article class="doc-entry" id="v-zone-to-timestamp-earlier">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zone-to-timestamp-earlier</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-TimeZone" title="Data.Time.Zone.TimeZone">TimeZone</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-AmbiguityError" title="Data.Time.Zone.Types.AmbiguityError">AmbiguityError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a></code><span class="doc-sig-links"><a href="#v-zone-to-timestamp-earlier" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zone/source/#L356" title="View source at line 356">Source</a></span></div>
<div class="doc-body">

The earlier instant in a gap and in a fold.

</div>
</article>

<article class="doc-entry" id="v-zone-to-timestamp-later">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zone-to-timestamp-later</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-TimeZone" title="Data.Time.Zone.TimeZone">TimeZone</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-AmbiguityError" title="Data.Time.Zone.Types.AmbiguityError">AmbiguityError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a></code><span class="doc-sig-links"><a href="#v-zone-to-timestamp-later" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zone/source/#L360" title="View source at line 360">Source</a></span></div>
<div class="doc-body">

The later instant in a gap and in a fold.

</div>
</article>

<article class="doc-entry" id="v-zone-to-timestamp-reject">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zone-to-timestamp-reject</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-TimeZone" title="Data.Time.Zone.TimeZone">TimeZone</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-AmbiguityError" title="Data.Time.Zone.Types.AmbiguityError">AmbiguityError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a></code><span class="doc-sig-links"><a href="#v-zone-to-timestamp-reject" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zone/source/#L364" title="View source at line 364">Source</a></span></div>
<div class="doc-body">

Refuses a gap and a fold.

</div>
</article>

<article class="doc-entry" id="v-zone-transition-count">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zone-transition-count</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-TimeZone" title="Data.Time.Zone.TimeZone">TimeZone</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-zone-transition-count" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zone/source/#L220" title="View source at line 220">Source</a></span></div>
<div class="doc-body">

How many transitions a table zone lists (0 for the other kinds).

</div>
</article>

<article class="doc-entry" id="v-zone-utc">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zone-utc</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-TimeZone" title="Data.Time.Zone.TimeZone">TimeZone</a></code><span class="doc-sig-links"><a href="#v-zone-utc" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zone/source/#L132" title="View source at line 132">Source</a></span></div>
<div class="doc-body">

UTC: offset zero, named and abbreviated "UTC".

</div>
</article>
