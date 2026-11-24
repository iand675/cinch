# Data.Time.Zone.Types

Shared vocabulary for time zones: transitions, ambiguity and disambiguation policies.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Data.Time.Zone.Types</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Data.Time.Zone.Types/source/">View source</a><a href="/packages/search/?q=%2BData.Time.Zone.Types">Search module</a><a href="/reference-sources/Data.Time.Zone.Types.cinch" download="Data.Time.Zone.Types.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Machine](Data.Machine.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.Time.DateTime](Data.Time.DateTime.md), [Data.Time.Offset](Data.Time.Offset.md), [Data.Time.Timestamp](Data.Time.Timestamp.md), [Relation.Eq](Relation.Eq.md).

<details class="doc-notes"><summary>Module notes</summary>

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

<details class="doc-synopsis"><summary>Synopsis (19 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-AmbiguityError">AmbiguityError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">AmbiguityError</strong></code></li><li><a class="syn-name" href="#t-Ambiguous">Ambiguous</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Ambiguous</strong></code></li><li><a class="syn-name" href="#t-Disambiguation">Disambiguation</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Disambiguation</strong></code></li><li><a class="syn-name" href="#t-OffsetInfo">OffsetInfo</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">OffsetInfo</strong></code></li><li><a class="syn-name" href="#v-ambiguous-compatible">ambiguous-compatible</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ambiguous-compatible</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-Ambiguous" title="Data.Time.Zone.Types.Ambiguous">Ambiguous</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-AmbiguityError" title="Data.Time.Zone.Types.AmbiguityError">AmbiguityError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a></code></li><li><a class="syn-name" href="#v-ambiguous-earlier">ambiguous-earlier</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ambiguous-earlier</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-Ambiguous" title="Data.Time.Zone.Types.Ambiguous">Ambiguous</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-AmbiguityError" title="Data.Time.Zone.Types.AmbiguityError">AmbiguityError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a></code></li><li><a class="syn-name" href="#v-ambiguous-from-lookups">ambiguous-from-lookups</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ambiguous-from-lookups</strong> :: (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a>) <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-Ambiguous" title="Data.Time.Zone.Types.Ambiguous">Ambiguous</a></code></li><li><a class="syn-name" href="#v-ambiguous-is-ambiguous">ambiguous-is-ambiguous</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ambiguous-is-ambiguous</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-Ambiguous" title="Data.Time.Zone.Types.Ambiguous">Ambiguous</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-ambiguous-is-fold">ambiguous-is-fold</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ambiguous-is-fold</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-Ambiguous" title="Data.Time.Zone.Types.Ambiguous">Ambiguous</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-ambiguous-is-gap">ambiguous-is-gap</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ambiguous-is-gap</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-Ambiguous" title="Data.Time.Zone.Types.Ambiguous">Ambiguous</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-ambiguous-later">ambiguous-later</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ambiguous-later</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-Ambiguous" title="Data.Time.Zone.Types.Ambiguous">Ambiguous</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-AmbiguityError" title="Data.Time.Zone.Types.AmbiguityError">AmbiguityError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a></code></li><li><a class="syn-name" href="#v-ambiguous-offset">ambiguous-offset</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ambiguous-offset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-Disambiguation" title="Data.Time.Zone.Types.Disambiguation">Disambiguation</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-Ambiguous" title="Data.Time.Zone.Types.Ambiguous">Ambiguous</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a></code></li><li><a class="syn-name" href="#v-ambiguous-reject">ambiguous-reject</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ambiguous-reject</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-Ambiguous" title="Data.Time.Zone.Types.Ambiguous">Ambiguous</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-AmbiguityError" title="Data.Time.Zone.Types.AmbiguityError">AmbiguityError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a></code></li><li><a class="syn-name" href="#v-ambiguous-resolve">ambiguous-resolve</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ambiguous-resolve</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-Disambiguation" title="Data.Time.Zone.Types.Disambiguation">Disambiguation</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-Ambiguous" title="Data.Time.Zone.Types.Ambiguous">Ambiguous</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-AmbiguityError" title="Data.Time.Zone.Types.AmbiguityError">AmbiguityError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a></code></li><li><a class="syn-name" href="#v-ambiguous-unambiguous">ambiguous-unambiguous</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ambiguous-unambiguous</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-Ambiguous" title="Data.Time.Zone.Types.Ambiguous">Ambiguous</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a></code></li><li><a class="syn-name" href="#v-offset-info">offset-info</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">offset-info</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-OffsetInfo" title="Data.Time.Zone.Types.OffsetInfo">OffsetInfo</a></code></li><li><a class="syn-name" href="#v-offset-info-abbreviation">offset-info-abbreviation</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">offset-info-abbreviation</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-OffsetInfo" title="Data.Time.Zone.Types.OffsetInfo">OffsetInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-offset-info-is-dst">offset-info-is-dst</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">offset-info-is-dst</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-OffsetInfo" title="Data.Time.Zone.Types.OffsetInfo">OffsetInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-offset-info-offset">offset-info-offset</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">offset-info-offset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-OffsetInfo" title="Data.Time.Zone.Types.OffsetInfo">OffsetInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a></code></li></ul>
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
