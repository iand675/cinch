# Data.Time.Monotonic

Monotonic clock readings for measuring elapsed time; never related to the civil calendar.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--source">Ship source</span> <code>import Data.Time.Monotonic</code></div><div class="doc-mod-bar-links"><a href="/library/source/Data.Time.Monotonic/">View source</a><a href="/library/search/?q=%2BData.Time.Monotonic">Search module</a><a href="/reference-sources/Data.Time.Monotonic.cinch" download="Data.Time.Monotonic.cinch">Download .cinch</a><a href="/library/00-index/">All modules</a></div></div>

**Dependencies:** [Data.Machine](Data.Machine.md), [Data.Option](Data.Option.md), [Data.Ordering](Data.Ordering.md), [Data.Time.Duration](Data.Time.Duration.md), [Data.Wide](Data.Wide.md), [Relation.Eq](Relation.Eq.md), [Relation.Ord](Relation.Ord.md).

<details class="doc-closure"><summary>Ship-source closure (2 files)</summary>

Copy these modules beside your entry file before importing `Data.Time.Monotonic`: [Data.Time.Duration](Data.Time.Duration.md), [Data.Time.Monotonic](Data.Time.Monotonic.md).

```sh
cp seed/prelude/Data/Time/Duration.cinch ./Data.Time.Duration.cinch
cp seed/prelude/Data/Time/Monotonic.cinch ./Data.Time.Monotonic.cinch
```

</details>

<details class="doc-notes"><summary>Module notes</summary>

Data.Time.Monotonic: a reading of the monotonic clock
(design/datetime.md, layer L0 "machine time", lane A).

A `Monotonic` answers "how long did that take?" and nothing else. It
is a count of nanoseconds since an origin the runtime picks at
start-up, so a single reading means nothing and only the difference
of two readings from the same process does. There is deliberately no
way to turn one into a `Timestamp` or a date: "when did that
happen?" is `Data.Time.Timestamp`'s question.

REPRESENTATION. One `I64` (what `cinch_clock_monotonic_ns` returns,
unconverted) behind the private constructor `MkMonotonic`. As for
`Duration`, the scalar is unboxed and the single-constructor `data`
wrapper is one 8-byte heap object per value (`cinch_alloc_ctor(8,
0)`); `#[repr(transparent)] record` does not remove the box in
native code and would make the type constructible by importers.

Design law shape. `monotonic-elapsed-between a b` is `b - a`, signed
and exact (the difference of two `I64`s always fits a `Duration`),
so it is antisymmetric (`elapsed-between b a` is its negation) and
zero on equal readings. `monotonic-checked-add m d` is the reading
`d` later, so `elapsed-between m (add m d)` is `d` whenever the sum
exists, and `checked-sub m d` is `checked-add m (neg d)`.
`monotonic-cmp` is the integer order, and it is the order of the
instants when both readings come from one process. The checks are
seed/tests/lib_time_a.rs.

NAMED REFUSALS (boundaries this module deliberately does not
cross):
  - conversion to or from wall-clock time: no function relates a
    `Monotonic` to a `Timestamp`.
  - readings from different processes or boots: the origin is
    per-process; comparing across processes is meaningless and the
    type cannot detect it.
  - a friend-only constructor: Cinch has no friend modules, so the
    raw constructor `monotonic-from-raw-nanos` is exported for
    `Effect.Time`. Only a clock reader should call it (tests use it
    to build fixed readings).
  - sums outside `I64` nanoseconds (about 292 years from the
    origin): `monotonic-checked-add` answers `None`, the
    `saturating-` twin clamps.
  - `Int`/`Nat` (bignums): neither appears.

Naming: every export is prefixed `monotonic-` (see
`Data.Time.Duration`'s header).

</details>

<details class="doc-synopsis" open><summary>Synopsis (9 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-Monotonic">Monotonic</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Monotonic</strong></code></li><li><a class="syn-name" href="#v-monotonic-checked-add">monotonic-checked-add</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">monotonic-checked-add</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Time.Monotonic/#t-Monotonic" title="Data.Time.Monotonic.Monotonic">Monotonic</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.Time.Monotonic/#t-Monotonic" title="Data.Time.Monotonic.Monotonic">Monotonic</a></code></li><li><a class="syn-name" href="#v-monotonic-checked-sub">monotonic-checked-sub</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">monotonic-checked-sub</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Time.Monotonic/#t-Monotonic" title="Data.Time.Monotonic.Monotonic">Monotonic</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.Time.Monotonic/#t-Monotonic" title="Data.Time.Monotonic.Monotonic">Monotonic</a></code></li><li><a class="syn-name" href="#v-monotonic-cmp">monotonic-cmp</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">monotonic-cmp</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Time.Monotonic/#t-Monotonic" title="Data.Time.Monotonic.Monotonic">Monotonic</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Time.Monotonic/#t-Monotonic" title="Data.Time.Monotonic.Monotonic">Monotonic</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Ordering/#t-Ordering" title="Data.Ordering.Ordering">Ordering</a></code></li><li><a class="syn-name" href="#v-monotonic-elapsed-between">monotonic-elapsed-between</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">monotonic-elapsed-between</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Time.Monotonic/#t-Monotonic" title="Data.Time.Monotonic.Monotonic">Monotonic</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Time.Monotonic/#t-Monotonic" title="Data.Time.Monotonic.Monotonic">Monotonic</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a></code></li><li><a class="syn-name" href="#v-monotonic-from-raw-nanos">monotonic-from-raw-nanos</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">monotonic-from-raw-nanos</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Time.Monotonic/#t-Monotonic" title="Data.Time.Monotonic.Monotonic">Monotonic</a></code></li><li><a class="syn-name" href="#v-monotonic-saturating-add">monotonic-saturating-add</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">monotonic-saturating-add</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Time.Monotonic/#t-Monotonic" title="Data.Time.Monotonic.Monotonic">Monotonic</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Time.Monotonic/#t-Monotonic" title="Data.Time.Monotonic.Monotonic">Monotonic</a></code></li><li><a class="syn-name" href="#v-monotonic-saturating-sub">monotonic-saturating-sub</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">monotonic-saturating-sub</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Time.Monotonic/#t-Monotonic" title="Data.Time.Monotonic.Monotonic">Monotonic</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Time.Monotonic/#t-Monotonic" title="Data.Time.Monotonic.Monotonic">Monotonic</a></code></li><li><a class="syn-name" href="#v-monotonic-to-raw-nanos">monotonic-to-raw-nanos</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">monotonic-to-raw-nanos</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Time.Monotonic/#t-Monotonic" title="Data.Time.Monotonic.Monotonic">Monotonic</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-Monotonic">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Monotonic</strong></code><span class="doc-sig-links"><a href="#t-Monotonic" title="Permalink">#</a><a href="/library/source/Data.Time.Monotonic/#L66" title="View source at line 66">Source</a></span></div>
<div class="doc-body">

Nanoseconds since the runtime's origin. Not exported (named
apart from the type so that `export Monotonic` does not export it).

<details class="doc-instances"><summary>Instances (2)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/library/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/library/Data.Time.Monotonic/#t-Monotonic" title="Data.Time.Monotonic.Monotonic">Monotonic</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Time.Monotonic/#instances">Data.Time.Monotonic</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <a class="doc-type-link tok-constructor" href="/library/Data.Time.Monotonic/#t-Monotonic" title="Data.Time.Monotonic.Monotonic">Monotonic</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Time.Monotonic/#instances">Data.Time.Monotonic</a></span></li></ul></details>
</div>
</article>

# Values

<article class="doc-entry" id="v-monotonic-checked-add">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">monotonic-checked-add</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Time.Monotonic/#t-Monotonic" title="Data.Time.Monotonic.Monotonic">Monotonic</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.Time.Monotonic/#t-Monotonic" title="Data.Time.Monotonic.Monotonic">Monotonic</a></code><span class="doc-sig-links"><a href="#v-monotonic-checked-add" title="Permalink">#</a><a href="/library/source/Data.Time.Monotonic/#L99" title="View source at line 99">Source</a></span></div>
<div class="doc-body">

The reading `d` after `m` (before it for a negative `d`); `None`
when that leaves `I64` nanoseconds.

</div>
</article>

<article class="doc-entry" id="v-monotonic-checked-sub">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">monotonic-checked-sub</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Time.Monotonic/#t-Monotonic" title="Data.Time.Monotonic.Monotonic">Monotonic</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.Time.Monotonic/#t-Monotonic" title="Data.Time.Monotonic.Monotonic">Monotonic</a></code><span class="doc-sig-links"><a href="#v-monotonic-checked-sub" title="Permalink">#</a><a href="/library/source/Data.Time.Monotonic/#L107" title="View source at line 107">Source</a></span></div>
<div class="doc-body">

The reading `d` before `m`. Negation is total on durations.

</div>
</article>

<article class="doc-entry" id="v-monotonic-cmp">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">monotonic-cmp</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Time.Monotonic/#t-Monotonic" title="Data.Time.Monotonic.Monotonic">Monotonic</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Time.Monotonic/#t-Monotonic" title="Data.Time.Monotonic.Monotonic">Monotonic</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Ordering/#t-Ordering" title="Data.Ordering.Ordering">Ordering</a></code><span class="doc-sig-links"><a href="#v-monotonic-cmp" title="Permalink">#</a><a href="/library/source/Data.Time.Monotonic/#L118" title="View source at line 118">Source</a></span></div>
</article>

<article class="doc-entry" id="v-monotonic-elapsed-between">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">monotonic-elapsed-between</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Time.Monotonic/#t-Monotonic" title="Data.Time.Monotonic.Monotonic">Monotonic</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Time.Monotonic/#t-Monotonic" title="Data.Time.Monotonic.Monotonic">Monotonic</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a></code><span class="doc-sig-links"><a href="#v-monotonic-elapsed-between" title="Permalink">#</a><a href="/library/source/Data.Time.Monotonic/#L89" title="View source at line 89">Source</a></span></div>
<div class="doc-body">

The time from `earlier` to `later`: `later - earlier`, negative
when the arguments are the other way round. Total: the difference
of two `I64`s has magnitude below 2^64, so the wrapping `I128`
subtraction is exact and the result is a duration.

</div>
</article>

<article class="doc-entry" id="v-monotonic-from-raw-nanos">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">monotonic-from-raw-nanos</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Time.Monotonic/#t-Monotonic" title="Data.Time.Monotonic.Monotonic">Monotonic</a></code><span class="doc-sig-links"><a href="#v-monotonic-from-raw-nanos" title="Permalink">#</a><a href="/library/source/Data.Time.Monotonic/#L77" title="View source at line 77">Source</a></span></div>
<div class="doc-body">

RAW: wrap a monotonic clock word. For clock readers
(`Effect.Time.monotonic-now`) and tests only; a number that did
not come from this process's monotonic clock makes a reading that
compares meaninglessly with real ones.

</div>
</article>

<article class="doc-entry" id="v-monotonic-saturating-add">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">monotonic-saturating-add</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Time.Monotonic/#t-Monotonic" title="Data.Time.Monotonic.Monotonic">Monotonic</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Time.Monotonic/#t-Monotonic" title="Data.Time.Monotonic.Monotonic">Monotonic</a></code><span class="doc-sig-links"><a href="#v-monotonic-saturating-add" title="Permalink">#</a><a href="/library/source/Data.Time.Monotonic/#L111" title="View source at line 111">Source</a></span></div>
<div class="doc-body">

The same, clamped to the `I64` bounds.

</div>
</article>

<article class="doc-entry" id="v-monotonic-saturating-sub">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">monotonic-saturating-sub</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Time.Monotonic/#t-Monotonic" title="Data.Time.Monotonic.Monotonic">Monotonic</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Time.Monotonic/#t-Monotonic" title="Data.Time.Monotonic.Monotonic">Monotonic</a></code><span class="doc-sig-links"><a href="#v-monotonic-saturating-sub" title="Permalink">#</a><a href="/library/source/Data.Time.Monotonic/#L115" title="View source at line 115">Source</a></span></div>
</article>

<article class="doc-entry" id="v-monotonic-to-raw-nanos">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">monotonic-to-raw-nanos</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Time.Monotonic/#t-Monotonic" title="Data.Time.Monotonic.Monotonic">Monotonic</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-monotonic-to-raw-nanos" title="Permalink">#</a><a href="/library/source/Data.Time.Monotonic/#L82" title="View source at line 82">Source</a></span></div>
<div class="doc-body">

RAW: the clock word back (for logging and tracing; it is not a
time of day and not comparable across processes).

</div>
</article>

# Instances

Exported checked instances declared in this module. For law checking, see [rewrites and optimization](../reference/12-rewrites-and-optimization.md).

<ul class="doc-module-instances">
<li id="i-Eq_Monotonic"><code><a class="doc-type-link tok-constructor" href="/library/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/library/Data.Time.Monotonic/#t-Monotonic" title="Data.Time.Monotonic.Monotonic">Monotonic</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Time.Monotonic.Eq_Monotonic</code></span></li>
<li id="i-Ord_Monotonic"><code><a class="doc-type-link tok-constructor" href="/library/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <a class="doc-type-link tok-constructor" href="/library/Data.Time.Monotonic/#t-Monotonic" title="Data.Time.Monotonic.Monotonic">Monotonic</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Time.Monotonic.Ord_Monotonic</code></span></li>
</ul>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Data/Time/Monotonic.cinch`](/library/source/Data.Time.Monotonic/). Source SHA-256: `35ad9089126831a35cefed441b34eec60c6b807e9770bd8185dc3eec72c63ca6`.
