# Data.Time.DateTime

Civil date and wall-clock time pairs without a time zone.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--source">Ship source</span> <code>import Data.Time.DateTime</code></div><div class="doc-mod-bar-links"><a href="/library/source/Data.Time.DateTime/">View source</a><a href="/library/search/?q=%2BData.Time.DateTime">Search module</a><a href="/reference-sources/Data.Time.DateTime.cinch" download="Data.Time.DateTime.cinch">Download .cinch</a><a href="/library/00-index/">All modules</a></div></div>

**Dependencies:** [Algebra.Arithmetic](Algebra.Arithmetic.md), [Data.Machine](Data.Machine.md), [Data.Option](Data.Option.md), [Data.Ordering](Data.Ordering.md), [Data.Time.Date](Data.Time.Date.md), [Data.Time.TimeOfDay](Data.Time.TimeOfDay.md), [Data.Time.Weekday](Data.Time.Weekday.md), [Data.Wide](Data.Wide.md), [Relation.Eq](Relation.Eq.md), [Relation.Ord](Relation.Ord.md).

<details class="doc-closure"><summary>Ship-source closure (4 files)</summary>

Copy these modules beside your entry file before importing `Data.Time.DateTime`: [Data.Time.Date](Data.Time.Date.md), [Data.Time.DateTime](Data.Time.DateTime.md), [Data.Time.TimeOfDay](Data.Time.TimeOfDay.md), [Data.Time.Weekday](Data.Time.Weekday.md).

```sh
cp seed/prelude/Data/Time/Date.cinch ./Data.Time.Date.cinch
cp seed/prelude/Data/Time/DateTime.cinch ./Data.Time.DateTime.cinch
cp seed/prelude/Data/Time/TimeOfDay.cinch ./Data.Time.TimeOfDay.cinch
cp seed/prelude/Data/Time/Weekday.cinch ./Data.Time.Weekday.cinch
```

</details>

<details class="doc-notes"><summary>Module notes</summary>

Data.Time.DateTime: a civil date and wall-clock time.

A `DateTime` is a `Date` and a `TimeOfDay` together: what a calendar
and a wall clock show at once, attached to no place. It is NOT an
instant. Turning a civil reading into an instant needs an offset or
a time zone, and that conversion is the zone layer's
(Data.Time.Offset, Data.Time.Zone); the two `-utc` functions here
are the arithmetic that layer is built from, and their names say
what they assume.

Representation. A two-field `data` behind a private constructor
holding the `Date` and the `TimeOfDay` (design/datetime.md section
4: "two fields, boxed until unpacking lands"). As held today
(verified against the emitted LLVM): one heap object with two
reference fields, each pointing at the one-word object of its part,
so `date-part` and `time-part` are a field read and build nothing.
Storing the two `I64` words inline instead would save two objects
per value but needs an unchecked `Date`-from-word constructor
exported from Data.Time.Date, which would let any importer forge an
invalid date; the invariant was kept.

Design law shape. `datetime d t` followed by `date-part` and
`time-part` returns `d` and `t`. The order is lexicographic, date
then time, and agrees with the "as if UTC" line:
`cmp a b` is the sign of `diff-nanos a b` reversed in argument order
(`diff-nanos start finish` is finish minus start) and the order of
the `(seconds, nanoseconds)` pairs of `to-epoch-seconds-nanos-utc`.
`from-epoch-seconds-nanos-utc` inverts `to-epoch-seconds-nanos-utc`
on the whole range. `add-seconds-nanos s n x` is the value whose
"as if UTC" line position is `x`'s plus `s` seconds and `n`
nanoseconds, or `None` when the date leaves -9999-01-01..=9999-12-31;
it carries through midnight, month ends and year ends in either
direction. `add-days`, `add-months-constrain` and
`add-years-constrain` move the date exactly as Data.Time.Date's do
and keep the time.

No operation faults (T2): everything is total or answers `Option`.

NAMED REFUSALS:
  - an inline two-word value: see Representation above.
  - instants, offsets and zones: a `DateTime` has no position on the
    timeline; "epoch seconds as if UTC" is a civil-line coordinate
    for the zone layer, not a `Timestamp`.
  - a `Duration`- or `Span`-typed surface, `until`/`since` in
    calendar units, and rounding: Data.Time.Arithmetic owns those;
    this module is plain `I64` (and one `I128` for the full-range
    nanosecond difference, which does not fit `I64`: the range spans
    about 6.3e20 nanoseconds).
  - text (Text.Time.Iso).
  - a total machine division: `Data.Machine` has only
    `checked-div-i64`/`checked-rem-i64`; the private `quot`/`rem`
    take a divisor at `InRange 2 9223372036854775807 I64` (which
    excludes both `None` cases) and still must match the `None` arm.

</details>

<details class="doc-synopsis"><summary>Synopsis (31 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-DateTime">DateTime</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">DateTime</strong></code></li><li><a class="syn-name" href="#v-add-days">add-days</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">add-days</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a></code></li><li><a class="syn-name" href="#v-add-months-constrain">add-months-constrain</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">add-months-constrain</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a></code></li><li><a class="syn-name" href="#v-add-seconds-nanos">add-seconds-nanos</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">add-seconds-nanos</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a></code></li><li><a class="syn-name" href="#v-add-years-constrain">add-years-constrain</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">add-years-constrain</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a></code></li><li><a class="syn-name" href="#v-cmp">cmp</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cmp</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Ordering/#t-Ordering" title="Data.Ordering.Ordering">Ordering</a></code></li><li><a class="syn-name" href="#v-date-part">date-part</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">date-part</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a></code></li><li><a class="syn-name" href="#v-datetime">datetime</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">datetime</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a></code></li><li><a class="syn-name" href="#v-day">day</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">day</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 1 31 <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-day-of-year">day-of-year</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">day-of-year</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 1 366 <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-diff-nanos">diff-nanos</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">diff-nanos</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Wide/" title="I128">I128</a></code></li><li><a class="syn-name" href="#v-end-of-day">end-of-day</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">end-of-day</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a></code></li><li><a class="syn-name" href="#v-from-date">from-date</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">from-date</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a></code></li><li><a class="syn-name" href="#v-from-epoch-seconds-nanos-utc">from-epoch-seconds-nanos-utc</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">from-epoch-seconds-nanos-utc</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a></code></li><li><a class="syn-name" href="#v-hour">hour</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">hour</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 23 <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-max-datetime">max-datetime</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">max-datetime</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a></code></li><li><a class="syn-name" href="#v-microsecond">microsecond</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">microsecond</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 999 <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-millisecond">millisecond</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">millisecond</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 999 <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-min-datetime">min-datetime</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">min-datetime</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a></code></li><li><a class="syn-name" href="#v-minute">minute</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">minute</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 59 <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-month">month</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">month</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 1 12 <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-nanosecond">nanosecond</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">nanosecond</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 999 <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-second">second</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">second</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 59 <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-start-of-day">start-of-day</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">start-of-day</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a></code></li><li><a class="syn-name" href="#v-subsec-nanosecond">subsec-nanosecond</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">subsec-nanosecond</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 999999999 <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-time-part">time-part</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">time-part</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a></code></li><li><a class="syn-name" href="#v-to-epoch-seconds-nanos-utc">to-epoch-seconds-nanos-utc</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">to-epoch-seconds-nanos-utc</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a>, <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a>)</code></li><li><a class="syn-name" href="#v-weekday">weekday</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">weekday</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Time.Weekday/#t-Weekday" title="Data.Time.Weekday.Weekday">Weekday</a></code></li><li><a class="syn-name" href="#v-with-date">with-date</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">with-date</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a></code></li><li><a class="syn-name" href="#v-with-time">with-time</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">with-time</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a></code></li><li><a class="syn-name" href="#v-year">year</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">year</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> - 9999 9999 <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-DateTime">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">DateTime</strong></code><span class="doc-sig-links"><a href="#t-DateTime" title="Permalink">#</a><a href="/library/source/Data.Time.DateTime/#L81" title="View source at line 81">Source</a></span></div>
<div class="doc-body">

The date and the time (header). Not exported.

<details class="doc-instances"><summary>Instances (2)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/library/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/library/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Time.DateTime/#instances">Data.Time.DateTime</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <a class="doc-type-link tok-constructor" href="/library/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Time.DateTime/#instances">Data.Time.DateTime</a></span></li></ul></details>
</div>
</article>

# Values

<article class="doc-entry" id="v-add-days">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">add-days</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a></code><span class="doc-sig-links"><a href="#v-add-days" title="Permalink">#</a><a href="/library/source/Data.Time.DateTime/#L187" title="View source at line 187">Source</a></span></div>
<div class="doc-body">

`n` days later (earlier when negative), same time.

</div>
</article>

<article class="doc-entry" id="v-add-months-constrain">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">add-months-constrain</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a></code><span class="doc-sig-links"><a href="#v-add-months-constrain" title="Permalink">#</a><a href="/library/source/Data.Time.DateTime/#L194" title="View source at line 194">Source</a></span></div>
<div class="doc-body">

`n` months later, the day clamped to the target month's length
(Temporal's "constrain"), same time.

</div>
</article>

<article class="doc-entry" id="v-add-seconds-nanos">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">add-seconds-nanos</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a></code><span class="doc-sig-links"><a href="#v-add-seconds-nanos" title="Permalink">#</a><a href="/library/source/Data.Time.DateTime/#L180" title="View source at line 180">Source</a></span></div>
<div class="doc-body">

Adds `secs` seconds and `nanos` nanoseconds, each signed and each
any `I64`, carrying into the date. `None` when the date leaves the
range. The time wraps with `T.add-seconds-nanos-wrapping` (total
for every pair of `I64`s; its overflow argument is there), and the
whole-day carry it reports (within +-1.07e14) moves the date
through `D.add-days`, whose sum is checked.

</div>
</article>

<article class="doc-entry" id="v-add-years-constrain">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">add-years-constrain</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a></code><span class="doc-sig-links"><a href="#v-add-years-constrain" title="Permalink">#</a><a href="/library/source/Data.Time.DateTime/#L201" title="View source at line 201">Source</a></span></div>
<div class="doc-body">

`n` years later, 29 February clamped to 28 February where the
target year is not a leap year, same time.

</div>
</article>

<article class="doc-entry" id="v-cmp">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cmp</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Ordering/#t-Ordering" title="Data.Ordering.Ordering">Ordering</a></code><span class="doc-sig-links"><a href="#v-cmp" title="Permalink">#</a><a href="/library/source/Data.Time.DateTime/#L258" title="View source at line 258">Source</a></span></div>
<div class="doc-body">

Order: lexicographic, date then time.

</div>
</article>

<article class="doc-entry" id="v-date-part">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">date-part</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a></code><span class="doc-sig-links"><a href="#v-date-part" title="Permalink">#</a><a href="/library/source/Data.Time.DateTime/#L96" title="View source at line 96">Source</a></span></div>
</article>

<article class="doc-entry" id="v-datetime">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">datetime</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a></code><span class="doc-sig-links"><a href="#v-datetime" title="Permalink">#</a><a href="/library/source/Data.Time.DateTime/#L89" title="View source at line 89">Source</a></span></div>
<div class="doc-body">

Construction and parts. Any `Date` and any `TimeOfDay` make a
`DateTime`, so construction is total.

</div>
</article>

<article class="doc-entry" id="v-day">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">day</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 1 31 <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-day" title="Permalink">#</a><a href="/library/source/Data.Time.DateTime/#L121" title="View source at line 121">Source</a></span></div>
</article>

<article class="doc-entry" id="v-day-of-year">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">day-of-year</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 1 366 <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-day-of-year" title="Permalink">#</a><a href="/library/source/Data.Time.DateTime/#L127" title="View source at line 127">Source</a></span></div>
</article>

<article class="doc-entry" id="v-diff-nanos">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">diff-nanos</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Wide/" title="I128">I128</a></code><span class="doc-sig-links"><a href="#v-diff-nanos" title="Permalink">#</a><a href="/library/source/Data.Time.DateTime/#L212" title="View source at line 212">Source</a></span></div>
<div class="doc-body">

`diff-nanos start finish`: finish minus start in nanoseconds on
the civil line (every day is 86400 seconds), negative when
`finish` is earlier. Total: the day difference is within
+-7304483 and the time difference within +-86399999999999, so the
result is within +-6.4e20, which needs `I128` (it exceeds `I64`'s
9.2e18) and is nowhere near wrapping it.

</div>
</article>

<article class="doc-entry" id="v-end-of-day">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">end-of-day</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a></code><span class="doc-sig-links"><a href="#v-end-of-day" title="Permalink">#</a><a href="/library/source/Data.Time.DateTime/#L161" title="View source at line 161">Source</a></span></div>
<div class="doc-body">

The same date at 23:59:59.999999999, the last representable time
of the civil day.

</div>
</article>

<article class="doc-entry" id="v-from-date">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">from-date</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a></code><span class="doc-sig-links"><a href="#v-from-date" title="Permalink">#</a><a href="/library/source/Data.Time.DateTime/#L93" title="View source at line 93">Source</a></span></div>
<div class="doc-body">

The date at 00:00:00.

</div>
</article>

<article class="doc-entry" id="v-from-epoch-seconds-nanos-utc">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">from-epoch-seconds-nanos-utc</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a></code><span class="doc-sig-links"><a href="#v-from-epoch-seconds-nanos-utc" title="Permalink">#</a><a href="/library/source/Data.Time.DateTime/#L240" title="View source at line 240">Source</a></span></div>
<div class="doc-body">

The inverse. `None` when `nanos` is outside 0..=999999999 (the
pair is not normalised) or the date leaves the range. The seconds
are any `I64`: they are split by truncated division into days and
a remainder that is moved into 0..=86399, so nothing wraps, and
the day count (within +-1.07e14) is refused by `D.from-epoch-days`
when out of range.

</div>
</article>

<article class="doc-entry" id="v-hour">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">hour</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 23 <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-hour" title="Permalink">#</a><a href="/library/source/Data.Time.DateTime/#L130" title="View source at line 130">Source</a></span></div>
</article>

<article class="doc-entry" id="v-max-datetime">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">max-datetime</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a></code><span class="doc-sig-links"><a href="#v-max-datetime" title="Permalink">#</a><a href="/library/source/Data.Time.DateTime/#L108" title="View source at line 108">Source</a></span></div>
</article>

<article class="doc-entry" id="v-microsecond">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">microsecond</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 999 <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-microsecond" title="Permalink">#</a><a href="/library/source/Data.Time.DateTime/#L145" title="View source at line 145">Source</a></span></div>
</article>

<article class="doc-entry" id="v-millisecond">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">millisecond</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 999 <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-millisecond" title="Permalink">#</a><a href="/library/source/Data.Time.DateTime/#L142" title="View source at line 142">Source</a></span></div>
</article>

<article class="doc-entry" id="v-min-datetime">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">min-datetime</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a></code><span class="doc-sig-links"><a href="#v-min-datetime" title="Permalink">#</a><a href="/library/source/Data.Time.DateTime/#L105" title="View source at line 105">Source</a></span></div>
<div class="doc-body">

-9999-01-01T00:00:00 and 9999-12-31T23:59:59.999999999.

</div>
</article>

<article class="doc-entry" id="v-minute">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">minute</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 59 <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-minute" title="Permalink">#</a><a href="/library/source/Data.Time.DateTime/#L133" title="View source at line 133">Source</a></span></div>
</article>

<article class="doc-entry" id="v-month">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">month</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 1 12 <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-month" title="Permalink">#</a><a href="/library/source/Data.Time.DateTime/#L118" title="View source at line 118">Source</a></span></div>
</article>

<article class="doc-entry" id="v-nanosecond">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">nanosecond</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 999 <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-nanosecond" title="Permalink">#</a><a href="/library/source/Data.Time.DateTime/#L148" title="View source at line 148">Source</a></span></div>
</article>

<article class="doc-entry" id="v-second">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">second</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 59 <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-second" title="Permalink">#</a><a href="/library/source/Data.Time.DateTime/#L136" title="View source at line 136">Source</a></span></div>
</article>

<article class="doc-entry" id="v-start-of-day">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">start-of-day</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a></code><span class="doc-sig-links"><a href="#v-start-of-day" title="Permalink">#</a><a href="/library/source/Data.Time.DateTime/#L156" title="View source at line 156">Source</a></span></div>
<div class="doc-body">

The same date at 00:00:00.

</div>
</article>

<article class="doc-entry" id="v-subsec-nanosecond">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">subsec-nanosecond</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 999999999 <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-subsec-nanosecond" title="Permalink">#</a><a href="/library/source/Data.Time.DateTime/#L139" title="View source at line 139">Source</a></span></div>
</article>

<article class="doc-entry" id="v-time-part">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">time-part</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a></code><span class="doc-sig-links"><a href="#v-time-part" title="Permalink">#</a><a href="/library/source/Data.Time.DateTime/#L100" title="View source at line 100">Source</a></span></div>
</article>

<article class="doc-entry" id="v-to-epoch-seconds-nanos-utc">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">to-epoch-seconds-nanos-utc</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a>, <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a>)</code><span class="doc-sig-links"><a href="#v-to-epoch-seconds-nanos-utc" title="Permalink">#</a><a href="/library/source/Data.Time.DateTime/#L228" title="View source at line 228">Source</a></span></div>
<div class="doc-body">

Whole seconds since 1970-01-01T00:00:00 reading the civil value as
if it were UTC, floored, and the subsecond nanoseconds in
0..=999999999. One nanosecond before the epoch is `(-1, 999999999)`.
The seconds are `epoch-day * 86400 + second-of-day`, within
+-3.8e11, so nothing wraps; the floor is automatic because the
time of day is never negative.

</div>
</article>

<article class="doc-entry" id="v-weekday">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">weekday</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Time.Weekday/#t-Weekday" title="Data.Time.Weekday.Weekday">Weekday</a></code><span class="doc-sig-links"><a href="#v-weekday" title="Permalink">#</a><a href="/library/source/Data.Time.DateTime/#L124" title="View source at line 124">Source</a></span></div>
</article>

<article class="doc-entry" id="v-with-date">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">with-date</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a></code><span class="doc-sig-links"><a href="#v-with-date" title="Permalink">#</a><a href="/library/source/Data.Time.DateTime/#L164" title="View source at line 164">Source</a></span></div>
</article>

<article class="doc-entry" id="v-with-time">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">with-time</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a></code><span class="doc-sig-links"><a href="#v-with-time" title="Permalink">#</a><a href="/library/source/Data.Time.DateTime/#L167" title="View source at line 167">Source</a></span></div>
</article>

<article class="doc-entry" id="v-year">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">year</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> - 9999 9999 <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-year" title="Permalink">#</a><a href="/library/source/Data.Time.DateTime/#L115" title="View source at line 115">Source</a></span></div>
<div class="doc-body">

Fields, forwarded to the parts with their refined ranges.

</div>
</article>

# Instances

Exported checked instances declared in this module. For law checking, see [rewrites and optimization](../reference/12-rewrites-and-optimization.md).

<ul class="doc-module-instances">
<li id="i-Eq_DateTime"><code><a class="doc-type-link tok-constructor" href="/library/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/library/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Time.DateTime.Eq_DateTime</code></span></li>
<li id="i-Ord_DateTime"><code><a class="doc-type-link tok-constructor" href="/library/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <a class="doc-type-link tok-constructor" href="/library/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Time.DateTime.Ord_DateTime</code></span></li>
</ul>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Data/Time/DateTime.cinch`](/library/source/Data.Time.DateTime/). Source SHA-256: `777d7c3bf5a8ecc3ed4f5c357eadd07019d85f5a77898a1c163e1ca33c9619ce`.
