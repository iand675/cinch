# Effect.Time

Typed readers for the system and monotonic clocks.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Effect.Time</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Effect.Time/source/">View source</a><a href="/packages/search/?q=%2BEffect.Time">Search module</a><a href="/reference-sources/Effect.Time.cinch" download="Effect.Time.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Time.Duration](Data.Time.Duration.md), [Data.Time.Monotonic](Data.Time.Monotonic.md), [Data.Time.Timestamp](Data.Time.Timestamp.md), [Effect.Clock](Effect.Clock.md), [Relation.Eq](Relation.Eq.md), [Relation.Ord](Relation.Ord.md).

<details class="doc-notes"><summary>Module notes</summary>

Effect.Time: the typed clock readers (design/datetime.md, layer L5,
lane A).

`Effect.Clock` reads the runtime clocks as bare `Nat` nanosecond
counts and measures in unsigned milliseconds. This module is the
same two clocks with the machine-time types: the wall clock reads a
`Timestamp` ("when did that happen?"), the monotonic clock reads a
`Monotonic` ("how long did that take?"), and everything between is
a signed, nanosecond `Duration`. Every reading requires the `Io r`
capability and performs `IO r`, exactly as `Effect.Clock`'s do, so
nothing ambient exists and no reading can enter a build product
(D45; a `stage` body cannot call a foreign reader).

Design law shape. Two `monotonic-now` readings in one process never
go backwards, so `elapsed-since m` is never negative for an `m` that
`monotonic-now` returned. `now` is the wall clock, which can step
(NTP, the operator), so it orders nothing and is never used to
measure here. A `Deadline` is a monotonic reading in the future:
`deadline-remaining` is the signed time to it (negative once it has
passed), `deadline-expired` is `remaining <= 0`, and neither is
affected by a wall-clock step. `sleep-for d` suspends for at least
`d` (rounded UP to a whole millisecond, the unit of the runtime's
`ioSleep`), and for zero milliseconds when `d` is zero or negative;
it is always a suspension point, so it always observes cancellation.
`from-clock-duration (to-clock-duration d)` is `d` rounded up to a
millisecond for a non-negative `d` within `I64` milliseconds. The
checks are seed/tests/lib_time_a.rs.

NAMED REFUSALS (boundaries this module deliberately does not
cross):
  - compile-time readings: as `Effect.Clock` (D45).
  - wall times before the epoch or past `I64` nanoseconds (the year
    2262): the runtime clamps `cinch_clock_wall_ns` to
    `0..=i64::MAX`, so `now` reads within 1970..=2262.
  - sub-millisecond sleeps: `ioSleep` takes milliseconds, so
    `sleep-for` rounds up; a sleep longer than `I64` milliseconds
    (292 million years) is clamped to that.
  - a direct `I64` clock word: `Effect.Clock` exports its readers as
    `Nat` only (its `I64` foreign imports are private and the module
    is not edited by this work, T1), so each reading converts
    `I64 -> Nat -> I64`. The `Nat` is transient and never stored;
    both conversions are the named ones (`nat-from-i64` there,
    `checked-nat-to-i64` here). A reading above `I64` cannot occur
    (the runtime's clamp) and would saturate.
  - killing a timed-out computation, and work rows wider than
    `{IO r, Cancel r}`: `timeout-after` is `Effect.Clock.timeout`
    with the deadline given as a `Duration`, and inherits both
    boundaries.
  - the system time zone and zoned "now": `Effect.Time.Zone`.

Naming. The readers keep the short names the specification gives
them (`now`, `monotonic-now`, `sleep-for`); the deadline operations
are prefixed `deadline-`. `Effect.Clock` is imported qualified
because its millisecond type is also called `Duration` (and it
exports its own `duration-from-millis`); a program that imports
`Effect.Clock` and `Data.Time.Duration` together must select or
alias one of them, and one that imports this module needs neither
for sleeping, deadlines or timeouts.

Module scheme (design/base-library.md): `Effect.*` is the effect
system's vocabulary.

</details>

<details class="doc-synopsis" open><summary>Synopsis (15 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-Deadline">Deadline</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Deadline</strong></code></li><li><a class="syn-name" href="#t-Timeout">Timeout</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Timeout</strong> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-deadline-after">deadline-after</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">deadline-after</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Time/#t-Deadline" title="Effect.Time.Deadline">Deadline</a></code></li><li><a class="syn-name" href="#v-deadline-at">deadline-at</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">deadline-at</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Monotonic/#t-Monotonic" title="Data.Time.Monotonic.Monotonic">Monotonic</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Time/#t-Deadline" title="Effect.Time.Deadline">Deadline</a></code></li><li><a class="syn-name" href="#v-deadline-expired">deadline-expired</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">deadline-expired</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Time/#t-Deadline" title="Effect.Time.Deadline">Deadline</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-deadline-instant">deadline-instant</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">deadline-instant</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Time/#t-Deadline" title="Effect.Time.Deadline">Deadline</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Monotonic/#t-Monotonic" title="Data.Time.Monotonic.Monotonic">Monotonic</a></code></li><li><a class="syn-name" href="#v-deadline-remaining">deadline-remaining</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">deadline-remaining</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Time/#t-Deadline" title="Effect.Time.Deadline">Deadline</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a></code></li><li><a class="syn-name" href="#v-elapsed-since">elapsed-since</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elapsed-since</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Monotonic/#t-Monotonic" title="Data.Time.Monotonic.Monotonic">Monotonic</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a></code></li><li><a class="syn-name" href="#v-from-clock-duration">from-clock-duration</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">from-clock-duration</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Clock/#t-Duration" title="Effect.Clock.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a></code></li><li><a class="syn-name" href="#v-monotonic-now">monotonic-now</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">monotonic-now</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Monotonic/#t-Monotonic" title="Data.Time.Monotonic.Monotonic">Monotonic</a></code></li><li><a class="syn-name" href="#v-now">now</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">now</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a></code></li><li><a class="syn-name" href="#v-sleep-for">sleep-for</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sleep-for</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/13-concurrency-and-runtime/" title="Cancel">Cancel</a> r, <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code></li><li><a class="syn-name" href="#v-sleep-until">sleep-until</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sleep-until</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Time/#t-Deadline" title="Effect.Time.Deadline">Deadline</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/13-concurrency-and-runtime/" title="Cancel">Cancel</a> r, <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code></li><li><a class="syn-name" href="#v-timeout-after">timeout-after</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">timeout-after</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/13-concurrency-and-runtime/" title="Cancel">Cancel</a> r, <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <span class="tok-variable">a</span>) <span class="tok-keyword">%1</span> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/13-concurrency-and-runtime/" title="Async">Async</a> r, <a class="doc-type-link tok-constructor" href="/reference/13-concurrency-and-runtime/" title="Cancel">Cancel</a> r, <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Diverge">Diverge</a>, <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Clock/#t-Timeout" title="Effect.Clock.Timeout">Timeout</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-to-clock-duration">to-clock-duration</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">to-clock-duration</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Clock/#t-Duration" title="Effect.Clock.Duration">Duration</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-Deadline">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Deadline</strong></code><span class="doc-sig-links"><a href="#t-Deadline" title="Permalink">#</a><a href="/packages/stdlib/Effect.Time/source/#L149" title="View source at line 149">Source</a></span></div>
<div class="doc-body">

A point on the monotonic clock to finish by: the reading's `I64`
nanosecond word, held directly (one object, not a box around a
`Monotonic` box). The constructor is private; `deadline-at` and
`deadline-instant` are its public spellings.

</div>
</article>

<article class="doc-entry" id="t-Timeout">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Timeout</strong> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#t-Timeout" title="Permalink">#</a><a href="/packages/stdlib/Effect.Clock/source/#L88" title="View source at line 88">Source</a></span></div>
<div class="doc-body">

The outcome of a computation raced against its deadline: it
finished (`Finished x`), or the deadline fired first and cancelled
it (`TimedOut`).

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-TimedOut"><code><strong class="tok-constructor">TimedOut</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Clock/#t-Timeout" title="Effect.Clock.Timeout">Timeout</a> <span class="tok-variable">a</span></code></li><li id="k-Finished"><code><strong class="tok-constructor">Finished</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Clock/#t-Timeout" title="Effect.Clock.Timeout">Timeout</a> <span class="tok-variable">a</span></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-deadline-after">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">deadline-after</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Time/#t-Deadline" title="Effect.Time.Deadline">Deadline</a></code><span class="doc-sig-links"><a href="#v-deadline-after" title="Permalink">#</a><a href="/packages/stdlib/Effect.Time/source/#L164" title="View source at line 164">Source</a></span></div>
<div class="doc-body">

The deadline `d` from now. Saturating: a `d` past the clock's
`I64` range gives the farthest representable deadline (292 years
from the origin), and a negative `d` one that has already passed.

</div>
</article>

<article class="doc-entry" id="v-deadline-at">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">deadline-at</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Monotonic/#t-Monotonic" title="Data.Time.Monotonic.Monotonic">Monotonic</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Time/#t-Deadline" title="Effect.Time.Deadline">Deadline</a></code><span class="doc-sig-links"><a href="#v-deadline-at" title="Permalink">#</a><a href="/packages/stdlib/Effect.Time/source/#L153" title="View source at line 153">Source</a></span></div>
<div class="doc-body">

The deadline at a known monotonic reading.

</div>
</article>

<article class="doc-entry" id="v-deadline-expired">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">deadline-expired</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Time/#t-Deadline" title="Effect.Time.Deadline">Deadline</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-deadline-expired" title="Permalink">#</a><a href="/packages/stdlib/Effect.Time/source/#L173" title="View source at line 173">Source</a></span></div>
<div class="doc-body">

Whether the deadline has been reached, read now.

</div>
</article>

<article class="doc-entry" id="v-deadline-instant">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">deadline-instant</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Time/#t-Deadline" title="Effect.Time.Deadline">Deadline</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Monotonic/#t-Monotonic" title="Data.Time.Monotonic.Monotonic">Monotonic</a></code><span class="doc-sig-links"><a href="#v-deadline-instant" title="Permalink">#</a><a href="/packages/stdlib/Effect.Time/source/#L157" title="View source at line 157">Source</a></span></div>
<div class="doc-body">

The monotonic reading the deadline falls on.

</div>
</article>

<article class="doc-entry" id="v-deadline-remaining">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">deadline-remaining</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Time/#t-Deadline" title="Effect.Time.Deadline">Deadline</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a></code><span class="doc-sig-links"><a href="#v-deadline-remaining" title="Permalink">#</a><a href="/packages/stdlib/Effect.Time/source/#L169" title="View source at line 169">Source</a></span></div>
<div class="doc-body">

The time left, read now: positive before the deadline, zero on it,
negative after it (`sleep-for` treats that as zero).

</div>
</article>

<article class="doc-entry" id="v-elapsed-since">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elapsed-since</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Monotonic/#t-Monotonic" title="Data.Time.Monotonic.Monotonic">Monotonic</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a></code><span class="doc-sig-links"><a href="#v-elapsed-since" title="Permalink">#</a><a href="/packages/stdlib/Effect.Time/source/#L107" title="View source at line 107">Source</a></span></div>
<div class="doc-body">

The time since the reading `start`, read now.

</div>
</article>

<article class="doc-entry" id="v-from-clock-duration">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">from-clock-duration</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Clock/#t-Duration" title="Effect.Clock.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a></code><span class="doc-sig-links"><a href="#v-from-clock-duration" title="Permalink">#</a><a href="/packages/stdlib/Effect.Time/source/#L132" title="View source at line 132">Source</a></span></div>
<div class="doc-body">

Exact for every count within `I64` milliseconds; saturating at
that bound for a larger `Nat`.

</div>
</article>

<article class="doc-entry" id="v-monotonic-now">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">monotonic-now</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Monotonic/#t-Monotonic" title="Data.Time.Monotonic.Monotonic">Monotonic</a></code><span class="doc-sig-links"><a href="#v-monotonic-now" title="Permalink">#</a><a href="/packages/stdlib/Effect.Time/source/#L103" title="View source at line 103">Source</a></span></div>
<div class="doc-body">

The monotonic clock: non-decreasing across calls in one process.

</div>
</article>

<article class="doc-entry" id="v-now">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">now</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a></code><span class="doc-sig-links"><a href="#v-now" title="Permalink">#</a><a href="/packages/stdlib/Effect.Time/source/#L99" title="View source at line 99">Source</a></span></div>
<div class="doc-body">

The wall clock: the current instant. It can step backwards or
forwards when the system clock is set; measure with
`monotonic-now`.

</div>
</article>

<article class="doc-entry" id="v-sleep-for">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sleep-for</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/13-concurrency-and-runtime/" title="Cancel">Cancel</a> r, <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code><span class="doc-sig-links"><a href="#v-sleep-for" title="Permalink">#</a><a href="/packages/stdlib/Effect.Time/source/#L138" title="View source at line 138">Source</a></span></div>
<div class="doc-body">

Suspend for at least `d`: rounded up to a whole millisecond, and
zero milliseconds for a zero or negative `d` (never an error and
never a negative sleep). Always a suspension point.

</div>
</article>

<article class="doc-entry" id="v-sleep-until">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sleep-until</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Time/#t-Deadline" title="Effect.Time.Deadline">Deadline</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/13-concurrency-and-runtime/" title="Cancel">Cancel</a> r, <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code><span class="doc-sig-links"><a href="#v-sleep-until" title="Permalink">#</a><a href="/packages/stdlib/Effect.Time/source/#L178" title="View source at line 178">Source</a></span></div>
<div class="doc-body">

Suspend until the deadline (not at all, beyond the suspension
point itself, when it has passed).

</div>
</article>

<article class="doc-entry" id="v-timeout-after">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">timeout-after</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/13-concurrency-and-runtime/" title="Cancel">Cancel</a> r, <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <span class="tok-variable">a</span>) <span class="tok-keyword">%1</span> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/13-concurrency-and-runtime/" title="Async">Async</a> r, <a class="doc-type-link tok-constructor" href="/reference/13-concurrency-and-runtime/" title="Cancel">Cancel</a> r, <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Diverge">Diverge</a>, <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Clock/#t-Timeout" title="Effect.Clock.Timeout">Timeout</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-timeout-after" title="Permalink">#</a><a href="/packages/stdlib/Effect.Time/source/#L189" title="View source at line 189">Source</a></span></div>
<div class="doc-body">

`Effect.Clock.timeout` with the limit as a `Duration` (rounded up
to a millisecond; zero for a negative one). The result type is
`Effect.Clock`'s `Timeout` (`TimedOut`, `Finished`), re-exported
from this module so a caller needs no `Effect.Clock` import.

</div>
</article>

<article class="doc-entry" id="v-to-clock-duration">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">to-clock-duration</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Clock/#t-Duration" title="Effect.Clock.Duration">Duration</a></code><span class="doc-sig-links"><a href="#v-to-clock-duration" title="Permalink">#</a><a href="/packages/stdlib/Effect.Time/source/#L127" title="View source at line 127">Source</a></span></div>
<div class="doc-body">

The millisecond `Effect.Clock.Duration` not shorter than `d`
(rounded up to a millisecond, so a deadline is never early);
saturating: zero for a negative `d`, `I64` milliseconds at most.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Effect/Time.cinch`](/packages/stdlib/Effect.Time/source/). Source SHA-256: `55fb18e3413088cd5db3e41302ae0984f1f0039c0e386ae8ca72e4e4fe7be4e4`.
