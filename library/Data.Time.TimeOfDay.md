# Data.Time.TimeOfDay

Civil wall-clock times of day with nanosecond precision.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Data.Time.TimeOfDay</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Data.Time.TimeOfDay/source/">View source</a><a href="/packages/search/?q=%2BData.Time.TimeOfDay">Search module</a><a href="/reference-sources/Data.Time.TimeOfDay.cinch" download="Data.Time.TimeOfDay.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Algebra.Arithmetic](Algebra.Arithmetic.md), [Data.Machine](Data.Machine.md), [Data.Option](Data.Option.md), [Data.Ordering](Data.Ordering.md), [Relation.Eq](Relation.Eq.md), [Relation.Ord](Relation.Ord.md).

<details class="doc-notes"><summary>Module notes</summary>

Data.Time.TimeOfDay: a civil wall-clock time.

A `TimeOfDay` is what a wall clock shows, attached to no date and no
place: an hour in 0..=23, a minute and a second in 0..=59 and a
subsecond in 0..=999999999 nanoseconds. There is no leap second
(second 60 has no representation) and no 24:00.

Representation. One `I64` behind a private constructor: the
nanoseconds since midnight, in 0..=86399999999999. The order of
times is the order of that count, so `cmp`, `Eq` and `Ord` are one
machine compare, and each field is a division of the count by a
constant.

How the value is held (verified against the emitted LLVM): the
declaration carries `#[repr(transparent)]`, which the compiler
admits and records as size 8, align 8, but the code generator holds
a `TimeOfDay` as a reference to a one-field heap object with the
`I64` unboxed inside it. See Data.Time.Date's NAMED REFUSALS for the
evidence and for why this is a `data` and not a `record`.

Design law shape. `time-of-day h m s ns` followed by the four
accessors returns `h`, `m`, `s`, `ns`; `to-nanos-of-day` and
`from-nanos-of-day` invert on 0..=86399999999999, and
`from-nanos-of-day` is `None` outside it. The subsecond splits as
`subsec-nanosecond t == millisecond t * 1000000 + microsecond t *
1000 + nanosecond t` (Jiff's three three-digit components).
`add-seconds-nanos-wrapping s n t` answers `(carry, t')` with
`carry * 86400000000000 + to-nanos-of-day t'` equal to
`to-nanos-of-day t + s * 1000000000 + n` as an exact integer, for
EVERY pair of `I64`s; `add-nanos-wrapping n` is that with `s = 0`.
`diff-nanos start finish` is finish minus start, in
-86399999999999..=86399999999999, and `cmp` is its sign.

No operation faults (T2): the constructors are total over refined
fields or answer `Option`, and the arithmetic is total by its named
`wrapping` convention (it wraps around midnight and reports how many
midnights it crossed; it never discards them silently).

NAMED REFUSALS:
  - an unboxed `TimeOfDay`: as for `Date`, `#[repr(transparent)]` is
    accepted and recorded but the value is a one-field heap object.
  - refined results by proof: machine integer arithmetic is opaque to
    the checker, so each accessor establishes its declared range with
    a clamp that never changes a value.
  - a total machine division: `Data.Machine` has only
    `checked-div-i64`/`checked-rem-i64`; the private `quot`/`rem`
    take a divisor at `InRange 2 9223372036854775807 I64` (which
    excludes both `None` cases) and still must match the `None` arm.
  - leap seconds, 24:00:00, and rounding (Data.Time.Arithmetic owns
    `round`); text (Text.Time.Iso).
  - a `Duration`- or `Span`-typed surface: this module is plain `I64`
    so machine time and civil time stay independent layers; the
    typed wrappers are Data.Time.Arithmetic's.

</details>

<details class="doc-synopsis"><summary>Synopsis (22 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-TimeOfDay">TimeOfDay</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">TimeOfDay</strong></code></li><li><a class="syn-name" href="#v-add-nanos-wrapping">add-nanos-wrapping</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">add-nanos-wrapping</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a>)</code></li><li><a class="syn-name" href="#v-add-seconds-nanos-wrapping">add-seconds-nanos-wrapping</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">add-seconds-nanos-wrapping</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a>)</code></li><li><a class="syn-name" href="#v-cmp">cmp</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cmp</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Ordering/#t-Ordering" title="Data.Ordering.Ordering">Ordering</a></code></li><li><a class="syn-name" href="#v-diff-nanos">diff-nanos</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">diff-nanos</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-from-nanos-of-day">from-nanos-of-day</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">from-nanos-of-day</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a></code></li><li><a class="syn-name" href="#v-hms">hms</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">hms</strong> :: (<span class="tok-variable">h</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 23 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">m</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 59 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">s</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 59 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a></code></li><li><a class="syn-name" href="#v-hour">hour</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">hour</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 23 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-max-time">max-time</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">max-time</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a></code></li><li><a class="syn-name" href="#v-microsecond">microsecond</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">microsecond</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 999 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-midnight">midnight</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">midnight</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a></code></li><li><a class="syn-name" href="#v-millisecond">millisecond</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">millisecond</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 999 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-min-time">min-time</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">min-time</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a></code></li><li><a class="syn-name" href="#v-minute">minute</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">minute</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 59 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-nanos-per-day">nanos-per-day</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">nanos-per-day</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-nanosecond">nanosecond</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">nanosecond</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 999 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-noon">noon</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">noon</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a></code></li><li><a class="syn-name" href="#v-second">second</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">second</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 59 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-subsec-nanosecond">subsec-nanosecond</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">subsec-nanosecond</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 999999999 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-time-of-day">time-of-day</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">time-of-day</strong> :: (<span class="tok-variable">h</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 23 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">m</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 59 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">s</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 59 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">ns</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 999999999 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a></code></li><li><a class="syn-name" href="#v-time-of-day-checked">time-of-day-checked</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">time-of-day-checked</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a></code></li><li><a class="syn-name" href="#v-to-nanos-of-day">to-nanos-of-day</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">to-nanos-of-day</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-TimeOfDay">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">TimeOfDay</strong></code><span class="doc-sig-links"><a href="#t-TimeOfDay" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.TimeOfDay/source/#L74" title="View source at line 74">Source</a></span></div>
<div class="doc-body">

Nanoseconds since midnight (header). Not exported.

<details class="doc-instances"><summary>Instances (2)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Time.TimeOfDay/#instances">Data.Time.TimeOfDay</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Time.TimeOfDay/#instances">Data.Time.TimeOfDay</a></span></li></ul></details>
</div>
</article>

# Values

<article class="doc-entry" id="v-add-nanos-wrapping">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">add-nanos-wrapping</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a>)</code><span class="doc-sig-links"><a href="#v-add-nanos-wrapping" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.TimeOfDay/source/#L216" title="View source at line 216">Source</a></span></div>
<div class="doc-body">

Adds `n` nanoseconds (signed, any `I64`), wrapping around midnight:
the signed whole-day carry and the wrapped time.
`add-nanos-wrapping 1 max-time` is `(1, midnight)` and
`add-nanos-wrapping (-1) midnight` is `(-1, max-time)`.

</div>
</article>

<article class="doc-entry" id="v-add-seconds-nanos-wrapping">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">add-seconds-nanos-wrapping</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a>)</code><span class="doc-sig-links"><a href="#v-add-seconds-nanos-wrapping" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.TimeOfDay/source/#L196" title="View source at line 196">Source</a></span></div>
<div class="doc-body">

Adds `secs` seconds and `nanos` nanoseconds (each signed, each any
`I64`), wrapping around midnight. The result is the signed number
of whole days carried (negative when the sum went back past
midnight) and the wrapped time, so that exactly
  carry * 86400000000000 + result
    == start + secs * 1000000000 + nanos.

Overflow argument (a division argument, no wide intermediate): the
seconds are split by truncated division into whole days `qs`
(|qs| <= 2^63 / 86400 < 1.07e14) and a remainder that is moved
into 0..=86399; the nanoseconds likewise into whole days `qn`
(|qn| <= 2^63 / 86400e9 < 106752) and a remainder in
0..=86399999999999. The three non-negative parts (the start, the
second remainder times 1e9, the nanosecond remainder) are each
below one day, so their sum is below three days (2.6e14) and is
reduced by at most two subtractions. The carry is `qs + qn` plus
at most 2 minus at most 2, far inside `I64`.

</div>
</article>

<article class="doc-entry" id="v-cmp">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cmp</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Ordering/#t-Ordering" title="Data.Ordering.Ordering">Ordering</a></code><span class="doc-sig-links"><a href="#v-cmp" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.TimeOfDay/source/#L229" title="View source at line 229">Source</a></span></div>
<div class="doc-body">

Order: a compare of the nanosecond counts.

</div>
</article>

<article class="doc-entry" id="v-diff-nanos">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">diff-nanos</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-diff-nanos" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.TimeOfDay/source/#L222" title="View source at line 222">Source</a></span></div>
<div class="doc-body">

`diff-nanos start finish`: finish minus start in nanoseconds,
negative when `finish` is the earlier time. Both counts are in
0..=86399999999999, so the difference cannot wrap.

</div>
</article>

<article class="doc-entry" id="v-from-nanos-of-day">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">from-nanos-of-day</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a></code><span class="doc-sig-links"><a href="#v-from-nanos-of-day" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.TimeOfDay/source/#L104" title="View source at line 104">Source</a></span></div>
<div class="doc-body">

From nanoseconds since midnight: `None` outside 0..=86399999999999.

</div>
</article>

<article class="doc-entry" id="v-hms">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">hms</strong> :: (<span class="tok-variable">h</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 23 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">m</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 59 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">s</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 59 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a></code><span class="doc-sig-links"><a href="#v-hms" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.TimeOfDay/source/#L90" title="View source at line 90">Source</a></span></div>
<div class="doc-body">

Hour, minute, second with a zero subsecond.

</div>
</article>

<article class="doc-entry" id="v-hour">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">hour</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 23 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-hour" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.TimeOfDay/source/#L136" title="View source at line 136">Source</a></span></div>
<div class="doc-body">

Fields. Each is a division of the count; the clamp states the
range in the type and never fires (see NAMED REFUSALS).

</div>
</article>

<article class="doc-entry" id="v-max-time">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">max-time</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a></code><span class="doc-sig-links"><a href="#v-max-time" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.TimeOfDay/source/#L124" title="View source at line 124">Source</a></span></div>
<div class="doc-body">

23:59:59.999999999.

</div>
</article>

<article class="doc-entry" id="v-microsecond">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">microsecond</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 999 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-microsecond" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.TimeOfDay/source/#L165" title="View source at line 165">Source</a></span></div>
</article>

<article class="doc-entry" id="v-midnight">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">midnight</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a></code><span class="doc-sig-links"><a href="#v-midnight" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.TimeOfDay/source/#L114" title="View source at line 114">Source</a></span></div>
<div class="doc-body">

00:00:00 (also the least time) and 12:00:00.

</div>
</article>

<article class="doc-entry" id="v-millisecond">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">millisecond</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 999 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-millisecond" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.TimeOfDay/source/#L160" title="View source at line 160">Source</a></span></div>
<div class="doc-body">

The fractional second's three three-digit components (Jiff's
`millisecond`, `microsecond`, `nanosecond`): 12:00:00.123456789 has
millisecond 123, microsecond 456, nanosecond 789.

</div>
</article>

<article class="doc-entry" id="v-min-time">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">min-time</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a></code><span class="doc-sig-links"><a href="#v-min-time" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.TimeOfDay/source/#L120" title="View source at line 120">Source</a></span></div>
</article>

<article class="doc-entry" id="v-minute">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">minute</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 59 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-minute" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.TimeOfDay/source/#L141" title="View source at line 141">Source</a></span></div>
</article>

<article class="doc-entry" id="v-nanos-per-day">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">nanos-per-day</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-nanos-per-day" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.TimeOfDay/source/#L128" title="View source at line 128">Source</a></span></div>
<div class="doc-body">

The length of a civil day in nanoseconds (no leap seconds).

</div>
</article>

<article class="doc-entry" id="v-nanosecond">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">nanosecond</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 999 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-nanosecond" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.TimeOfDay/source/#L170" title="View source at line 170">Source</a></span></div>
</article>

<article class="doc-entry" id="v-noon">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">noon</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a></code><span class="doc-sig-links"><a href="#v-noon" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.TimeOfDay/source/#L117" title="View source at line 117">Source</a></span></div>
</article>

<article class="doc-entry" id="v-second">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">second</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 59 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-second" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.TimeOfDay/source/#L146" title="View source at line 146">Source</a></span></div>
</article>

<article class="doc-entry" id="v-subsec-nanosecond">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">subsec-nanosecond</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 999999999 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-subsec-nanosecond" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.TimeOfDay/source/#L152" title="View source at line 152">Source</a></span></div>
<div class="doc-body">

The whole fractional second, in nanoseconds.

</div>
</article>

<article class="doc-entry" id="v-time-of-day">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">time-of-day</strong> :: (<span class="tok-variable">h</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 23 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">m</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 59 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">s</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 59 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">ns</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 999999999 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a></code><span class="doc-sig-links"><a href="#v-time-of-day" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.TimeOfDay/source/#L86" title="View source at line 86">Source</a></span></div>
<div class="doc-body">

Total: every range is in an argument type, so
`time-of-day 13 45 30 0` checks with nothing else written and
`time-of-day 24 0 0 0` is refused at compile time. The count is at
most 23 * 3600e9 + 59 * 60e9 + 59 * 1e9 + 999999999 =
86399999999999, so nothing wraps.

</div>
</article>

<article class="doc-entry" id="v-time-of-day-checked">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">time-of-day-checked</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a></code><span class="doc-sig-links"><a href="#v-time-of-day-checked" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.TimeOfDay/source/#L95" title="View source at line 95">Source</a></span></div>
<div class="doc-body">

The same over plain `I64` fields: `None` when any field is outside
its range.

</div>
</article>

<article class="doc-entry" id="v-to-nanos-of-day">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">to-nanos-of-day</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-to-nanos-of-day" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.TimeOfDay/source/#L109" title="View source at line 109">Source</a></span></div>
<div class="doc-body">

Nanoseconds since midnight, in 0..=86399999999999.

</div>
</article>

# Instances

Exported checked instances declared in this module. For law checking, see [rewrites and optimization](../reference/12-rewrites-and-optimization.md).

<ul class="doc-module-instances">
<li id="i-Eq_TimeOfDay"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Time.TimeOfDay.Eq_TimeOfDay</code></span></li>
<li id="i-Ord_TimeOfDay"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Time.TimeOfDay.Ord_TimeOfDay</code></span></li>
</ul>
