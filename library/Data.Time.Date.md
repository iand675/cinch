# Data.Time.Date

Civil calendar dates with validated construction, weekday and ordinal queries, and day arithmetic.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Data.Time.Date</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Data.Time.Date/source/">View source</a><a href="/packages/search/?q=%2BData.Time.Date">Search module</a><a href="/reference-sources/Data.Time.Date.cinch" download="Data.Time.Date.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Algebra.Arithmetic](Algebra.Arithmetic.md), [Data.Machine](Data.Machine.md), [Data.Option](Data.Option.md), [Data.Ordering](Data.Ordering.md), [Data.Time.Weekday](Data.Time.Weekday.md), [Relation.Eq](Relation.Eq.md), [Relation.Ord](Relation.Ord.md).

<details class="doc-notes"><summary>Module notes</summary>

Data.Time.Date: a civil calendar date.

A `Date` is a day in the proleptic Gregorian calendar, attached to no
place and no instant: a year in -9999..=9999 (astronomical numbering,
so year 0 exists and is a leap year), a month in 1..=12 and a day in
1..=that month's length in that year. There are no leap seconds and
no reform cutover (design/datetime.md T3).

Representation. One `I64` word behind a private constructor:

    word = year * 512 + month * 32 + day

The year is NOT biased: for a negative year the word is negative,
and because `month * 32 + day` is in 33..=415 (below 512) the year is
exactly the arithmetic shift `word >> 9` (which floors) and the low
nine bits are `month * 32 + day` in two's complement for either sign.
The encoding is order preserving: the year is the most significant
part, then the month, then the day, and each lower part is smaller
than the weight of the part above it, so comparing two dates is
comparing two words (`cmp`, `Eq`, `Ord` are one machine compare and
never run the calendar). Field access is a shift and a mask, so a
printer needs no division by 146097.

How the value is held (verified against the emitted LLVM, see NAMED
REFUSALS): the declaration carries `#[repr(transparent)]`, which the
compiler admits on a one-field `data` and records in the interface as
size 8, align 8, but the code generator still holds a `Date` as a
reference to a one-field heap object. The word is unboxed inside that
object; the `Date` itself is not yet a bare scalar.

The constructor is not exported, so every `Date` came through `date`,
`date-constrain`, `date-of`, `from-epoch-days` or arithmetic on a
valid date, and an invalid date (a 30 February, a month 13, a year
10000) has no representation.

Design law shape. One predicate, `is-leap-year` (divisible by 4 and
not by 100 unless by 400), fixes every month length
(`days-in-month`) and every year length (`days-in-year`). On top of
it:
  - `to-epoch-days` is strictly increasing in the word order and
    `from-epoch-days` inverts it on `min-epoch-day..=max-epoch-day`
    (1970-01-01 is day 0); outside that interval `from-epoch-days` is
    `None`.
  - `add-days n d` is `from-epoch-days (to-epoch-days d + n)`, so
    `diff-days d e` (finish minus start) is the `n` with
    `add-days n d == Some e`, and adding m then n days is adding
    m + n whenever every step stays in range.
  - `tomorrow`/`yesterday` are `add-days 1`/`add-days (-1)`.
  - `weekday` is the epoch day read modulo 7 with 1970-01-01 a
    Thursday, so consecutive days have consecutive weekdays.
  - `add-months-constrain` and `add-years-constrain` follow Temporal's
    "constrain" overflow: the year and month move exactly, and the day
    is clamped down to the target month's length
    (2024-05-31 + 1 month = 2024-06-30, 2024-02-29 + 1 year =
    2025-02-28). They are `None` only when the year leaves the range.
  - `day-of-year` is 1 on 1 January and `days-in-year` on 31 December.
  - `nth-weekday-of-month` and `nth-weekday` are Jiff's
    `nth_weekday_of_month` and `nth_weekday`.
  - `iso-week-year`/`iso-week` are ISO 8601 week dates (week 1 is the
    week holding the year's first Thursday).

No operation faults (T2): each is total over refined arguments
(`date-of`, `date-constrain`, `days-in-month`), total by a named
convention (`-constrain`), or answers `Option`.

NAMED REFUSALS:
  - an unboxed `Date`: `#[repr(transparent)]` is accepted and
    recorded, but the emitted code allocates a one-field object for
    the constructor and loads the word through it (`#[no-alloc]` on
    a function that builds a `Date` is refused with "the constructor
    `MkDate` is constructed here"). A `record` is held the same way
    and, unlike a `data`, lets any importer write
    `Date { word = ... }`, so a `data` with a private constructor is
    used. Nothing in the API depends on the box.
  - a total literal constructor for days 29..=31: the refinement
    vocabulary is comparisons over one value, so "day <= the length of
    this month in this year" cannot be a parameter's type. `date-of`
    is total for days 1..=28 (valid in every month); other days go
    through `date` (`Option`) or `date-constrain` (clamps). A staged
    date literal is design/datetime.md's open decision 4.
  - refined results by proof: machine integer arithmetic is opaque to
    the checker, so `year`, `month`, `day`, `day-of-year` and
    `iso-week` establish their declared ranges with a clamp. The
    clamp never changes a value (the representation invariant puts
    every field in range already); it is how the range reaches the
    type.
  - a total machine division: `Data.Machine` has only
    `checked-div-i64`/`checked-rem-i64`. The private `quot`/`rem`
    take the divisor at `InRange 2 9223372036854775807 I64`, which
    excludes both `None` cases (zero, and `min / -1`), and still must
    match the `None` arm.
  - text: no printer or parser lives here (Text.Time.Iso).
  - `Int`/`Nat`: every quantity is an `I64`.

</details>

<details class="doc-synopsis"><summary>Synopsis (36 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-Date">Date</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Date</strong></code></li><li><a class="syn-name" href="#v-add-days">add-days</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">add-days</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a></code></li><li><a class="syn-name" href="#v-add-months-constrain">add-months-constrain</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">add-months-constrain</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a></code></li><li><a class="syn-name" href="#v-add-years-constrain">add-years-constrain</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">add-years-constrain</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a></code></li><li><a class="syn-name" href="#v-cmp">cmp</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cmp</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Ordering/#t-Ordering" title="Data.Ordering.Ordering">Ordering</a></code></li><li><a class="syn-name" href="#v-date">date</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">date</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a></code></li><li><a class="syn-name" href="#v-date-constrain">date-constrain</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">date-constrain</strong> :: (<span class="tok-variable">y</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> - 9999 9999 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">m</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 1 12 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a></code></li><li><a class="syn-name" href="#v-date-of">date-of</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">date-of</strong> :: (<span class="tok-variable">y</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> - 9999 9999 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">m</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 1 12 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">d</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 1 28 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a></code></li><li><a class="syn-name" href="#v-day">day</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">day</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 1 31 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-day-of-year">day-of-year</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">day-of-year</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 1 366 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-days-in-month">days-in-month</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">days-in-month</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> (<span class="tok-variable">m</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 1 12 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 28 31 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-days-in-month-of">days-in-month-of</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">days-in-month-of</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 28 31 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-days-in-year">days-in-year</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">days-in-year</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 365 366 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-days-in-year-of">days-in-year-of</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">days-in-year-of</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 365 366 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-diff-days">diff-days</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">diff-days</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-first-of-month">first-of-month</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">first-of-month</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a></code></li><li><a class="syn-name" href="#v-first-of-year">first-of-year</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">first-of-year</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a></code></li><li><a class="syn-name" href="#v-from-epoch-days">from-epoch-days</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">from-epoch-days</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a></code></li><li><a class="syn-name" href="#v-in-leap-year">in-leap-year</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">in-leap-year</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-is-leap-year">is-leap-year</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">is-leap-year</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-iso-week">iso-week</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">iso-week</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 1 53 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-iso-week-year">iso-week-year</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">iso-week-year</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> - 9999 9999 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-last-of-month">last-of-month</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">last-of-month</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a></code></li><li><a class="syn-name" href="#v-last-of-year">last-of-year</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">last-of-year</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a></code></li><li><a class="syn-name" href="#v-max-date">max-date</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">max-date</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a></code></li><li><a class="syn-name" href="#v-max-epoch-day">max-epoch-day</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">max-epoch-day</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-min-date">min-date</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">min-date</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a></code></li><li><a class="syn-name" href="#v-min-epoch-day">min-epoch-day</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">min-epoch-day</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-month">month</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">month</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 1 12 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-nth-weekday">nth-weekday</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">nth-weekday</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Weekday/#t-Weekday" title="Data.Time.Weekday.Weekday">Weekday</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a></code></li><li><a class="syn-name" href="#v-nth-weekday-of-month">nth-weekday-of-month</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">nth-weekday-of-month</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Weekday/#t-Weekday" title="Data.Time.Weekday.Weekday">Weekday</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a></code></li><li><a class="syn-name" href="#v-to-epoch-days">to-epoch-days</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">to-epoch-days</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-tomorrow">tomorrow</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tomorrow</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a></code></li><li><a class="syn-name" href="#v-weekday">weekday</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">weekday</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Weekday/#t-Weekday" title="Data.Time.Weekday.Weekday">Weekday</a></code></li><li><a class="syn-name" href="#v-year">year</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">year</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> - 9999 9999 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-yesterday">yesterday</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">yesterday</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-Date">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Date</strong></code><span class="doc-sig-links"><a href="#t-Date" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Date/source/#L121" title="View source at line 121">Source</a></span></div>
<div class="doc-body">

The packed word (header). Not exported: the validity invariant.

<details class="doc-instances"><summary>Instances (4)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Date/#t-Date" title="Data.Date.Date">Date</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Date/#instances">Data.Date</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Date/#t-Date" title="Data.Date.Date">Date</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Date/#instances">Data.Date</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Time.Date/#instances">Data.Time.Date</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Time.Date/#instances">Data.Time.Date</a></span></li></ul></details>
</div>
</article>

# Values

<article class="doc-entry" id="v-add-days">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">add-days</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a></code><span class="doc-sig-links"><a href="#v-add-days" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Date/source/#L240" title="View source at line 240">Source</a></span></div>
<div class="doc-body">

`n` days later (earlier for a negative `n`). `None` when the result
leaves the range. The sum is a checked addition because `n` is any
`I64`; an overflowing sum is out of range a fortiori.

</div>
</article>

<article class="doc-entry" id="v-add-months-constrain">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">add-months-constrain</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a></code><span class="doc-sig-links"><a href="#v-add-months-constrain" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Date/source/#L288" title="View source at line 288">Source</a></span></div>
<div class="doc-body">

`n` months later (earlier for a negative `n`): the year and month
move exactly, and the day is clamped down to the target month's
length. `None` only when the year leaves -9999..=9999.

Overflow argument: the date's month index `year * 12 + month - 1`
is within +-120000. `n` is any `I64`, so the sum is a checked
addition, and the sum is then tested against the range's own month
indexes (-119988..=119999) before anything else is computed from
it; after the test every intermediate is below 2^18.

</div>
</article>

<article class="doc-entry" id="v-add-years-constrain">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">add-years-constrain</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a></code><span class="doc-sig-links"><a href="#v-add-years-constrain" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Date/source/#L305" title="View source at line 305">Source</a></span></div>
<div class="doc-body">

`n` years later (earlier for a negative `n`), same month; the day
is clamped down to that month's length in the target year, which
only ever turns 29 February into 28 February. `None` only when the
year leaves the range (a checked addition, because `n` is any
`I64`).

</div>
</article>

<article class="doc-entry" id="v-cmp">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cmp</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Ordering/#t-Ordering" title="Data.Ordering.Ordering">Ordering</a></code><span class="doc-sig-links"><a href="#v-cmp" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Date/source/#L416" title="View source at line 416">Source</a></span></div>
<div class="doc-body">

Order: a word compare (see the header).

</div>
</article>

<article class="doc-entry" id="v-date">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">date</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a></code><span class="doc-sig-links"><a href="#v-date" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Date/source/#L131" title="View source at line 131">Source</a></span></div>
<div class="doc-body">

The checked constructor: year, month, day. `None` for a year
outside -9999..=9999, a month outside 1..=12, or a day outside
1..=the month's length in that year.

</div>
</article>

<article class="doc-entry" id="v-date-constrain">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">date-constrain</strong> :: (<span class="tok-variable">y</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> - 9999 9999 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">m</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 1 12 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a></code><span class="doc-sig-links"><a href="#v-date-constrain" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Date/source/#L143" title="View source at line 143">Source</a></span></div>
<div class="doc-body">

Temporal's "constrain" overflow for the day: the year and month are
taken as given (their ranges are in their types, so a literal
discharges at compile time) and the day is clamped into
1..=the month's length. `date-constrain 2023 2 31` is 2023-02-28;
a day below 1 is the 1st.

</div>
</article>

<article class="doc-entry" id="v-date-of">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">date-of</strong> :: (<span class="tok-variable">y</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> - 9999 9999 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">m</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 1 12 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">d</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 1 28 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a></code><span class="doc-sig-links"><a href="#v-date-of" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Date/source/#L154" title="View source at line 154">Source</a></span></div>
<div class="doc-body">

The total constructor for the days every month has. All three
ranges are in the argument types, so `date-of 2024 2 15` checks
with nothing else written and a bad literal is refused at compile
time.

</div>
</article>

<article class="doc-entry" id="v-day">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">day</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 1 31 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-day" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Date/source/#L179" title="View source at line 179">Source</a></span></div>
</article>

<article class="doc-entry" id="v-day-of-year">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">day-of-year</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 1 366 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-day-of-year" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Date/source/#L323" title="View source at line 323">Source</a></span></div>
<div class="doc-body">

1 on 1 January, `days-in-year` on 31 December.

</div>
</article>

<article class="doc-entry" id="v-days-in-month">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">days-in-month</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> (<span class="tok-variable">m</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 1 12 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 28 31 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-days-in-month" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Date/source/#L200" title="View source at line 200">Source</a></span></div>
<div class="doc-body">

The length of month `m` in year `y`: 28, 29, 30 or 31. The month's
range is in its type, so there is no out-of-calendar case.

</div>
</article>

<article class="doc-entry" id="v-days-in-month-of">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">days-in-month-of</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 28 31 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-days-in-month-of" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Date/source/#L209" title="View source at line 209">Source</a></span></div>
</article>

<article class="doc-entry" id="v-days-in-year">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">days-in-year</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 365 366 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-days-in-year" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Date/source/#L203" title="View source at line 203">Source</a></span></div>
</article>

<article class="doc-entry" id="v-days-in-year-of">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">days-in-year-of</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 365 366 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-days-in-year-of" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Date/source/#L212" title="View source at line 212">Source</a></span></div>
</article>

<article class="doc-entry" id="v-diff-days">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">diff-days</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-diff-days" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Date/source/#L249" title="View source at line 249">Source</a></span></div>
<div class="doc-body">

`diff-days start finish`: the days from `start` to `finish`,
finish minus start, negative when `finish` is earlier. Both epoch
days are within +-4371588, so the difference is within +-7304484
and the wrapping subtraction never wraps.

</div>
</article>

<article class="doc-entry" id="v-first-of-month">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">first-of-month</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a></code><span class="doc-sig-links"><a href="#v-first-of-month" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Date/source/#L328" title="View source at line 328">Source</a></span></div>
</article>

<article class="doc-entry" id="v-first-of-year">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">first-of-year</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a></code><span class="doc-sig-links"><a href="#v-first-of-year" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Date/source/#L337" title="View source at line 337">Source</a></span></div>
</article>

<article class="doc-entry" id="v-from-epoch-days">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">from-epoch-days</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a></code><span class="doc-sig-links"><a href="#v-from-epoch-days" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Date/source/#L232" title="View source at line 232">Source</a></span></div>
<div class="doc-body">

The inverse: `None` outside `min-epoch-day..=max-epoch-day`.

</div>
</article>

<article class="doc-entry" id="v-in-leap-year">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">in-leap-year</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-in-leap-year" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Date/source/#L206" title="View source at line 206">Source</a></span></div>
</article>

<article class="doc-entry" id="v-is-leap-year">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">is-leap-year</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-is-leap-year" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Date/source/#L192" title="View source at line 192">Source</a></span></div>
<div class="doc-body">

The Gregorian leap rule, total over `I64` (the proleptic calendar
defines it for every year, including 0 and the negative ones).
Divisibility does not depend on the remainder's sign, so the
truncated remainder serves for negative years too.

</div>
</article>

<article class="doc-entry" id="v-iso-week">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">iso-week</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 1 53 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-iso-week" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Date/source/#L404" title="View source at line 404">Source</a></span></div>
<div class="doc-body">

The ISO week number, 1..=53.

</div>
</article>

<article class="doc-entry" id="v-iso-week-year">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">iso-week-year</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> - 9999 9999 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-iso-week-year" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Date/source/#L398" title="View source at line 398">Source</a></span></div>
<div class="doc-body">

The ISO week-numbering year. It differs from `year` only in the
first days of January and the last days of December. It stays in
-9999..=9999 because the range begins on a Monday (-9999-01-01
opens week 1 of -9999) and ends on a Friday (9999-12-31 is in week
52 of 9999), so neither end borrows a week from a year outside the
range; the clamp states that in the type and never fires.

</div>
</article>

<article class="doc-entry" id="v-last-of-month">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">last-of-month</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a></code><span class="doc-sig-links"><a href="#v-last-of-month" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Date/source/#L331" title="View source at line 331">Source</a></span></div>
</article>

<article class="doc-entry" id="v-last-of-year">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">last-of-year</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a></code><span class="doc-sig-links"><a href="#v-last-of-year" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Date/source/#L340" title="View source at line 340">Source</a></span></div>
</article>

<article class="doc-entry" id="v-max-date">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">max-date</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a></code><span class="doc-sig-links"><a href="#v-max-date" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Date/source/#L161" title="View source at line 161">Source</a></span></div>
</article>

<article class="doc-entry" id="v-max-epoch-day">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">max-epoch-day</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-max-epoch-day" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Date/source/#L223" title="View source at line 223">Source</a></span></div>
</article>

<article class="doc-entry" id="v-min-date">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">min-date</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a></code><span class="doc-sig-links"><a href="#v-min-date" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Date/source/#L158" title="View source at line 158">Source</a></span></div>
<div class="doc-body">

-9999-01-01 and 9999-12-31.

</div>
</article>

<article class="doc-entry" id="v-min-epoch-day">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">min-epoch-day</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-min-epoch-day" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Date/source/#L220" title="View source at line 220">Source</a></span></div>
<div class="doc-body">

to-epoch-days min-date and to-epoch-days max-date.

</div>
</article>

<article class="doc-entry" id="v-month">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">month</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 1 12 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-month" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Date/source/#L174" title="View source at line 174">Source</a></span></div>
</article>

<article class="doc-entry" id="v-nth-weekday">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">nth-weekday</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Weekday/#t-Weekday" title="Data.Time.Weekday.Weekday">Weekday</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a></code><span class="doc-sig-links"><a href="#v-nth-weekday" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Date/source/#L373" title="View source at line 373">Source</a></span></div>
<div class="doc-body">

The `n`th `wd` strictly after the date for a positive `n`, strictly
before it for a negative `n` (Jiff's `nth_weekday`): from Wednesday
2024-07-10, `nth-weekday 1 Wednesday` is 2024-07-17 and
`nth-weekday (-1) Wednesday` is 2024-07-03. `None` when `n` is 0 or
the answer leaves the range.

Overflow argument: the whole range holds 7304484 days, fewer than
1043500 weeks, so an `n` beyond +-1043500 cannot land in range and
is refused before any multiplication; within that bound
`(|n| - 1) * 7 + 7` is below 2^23.

</div>
</article>

<article class="doc-entry" id="v-nth-weekday-of-month">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">nth-weekday-of-month</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Weekday/#t-Weekday" title="Data.Time.Weekday.Weekday">Weekday</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a></code><span class="doc-sig-links"><a href="#v-nth-weekday-of-month" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Date/source/#L348" title="View source at line 348">Source</a></span></div>
<div class="doc-body">

The `n`th `wd` of the date's month (Jiff's `nth_weekday_of_month`).
`n` in 1..=5 counts from the start of the month (1 is the first
`wd`), `n` in -5..=-1 from its end (-1 is the last `wd`). `None`
when `n` is 0 or outside +-5, or when the month has no such
occurrence (a fifth Monday in a month with four).

</div>
</article>

<article class="doc-entry" id="v-to-epoch-days">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">to-epoch-days</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-to-epoch-days" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Date/source/#L228" title="View source at line 228">Source</a></span></div>
<div class="doc-body">

The day number counted from 1970-01-01, negative before it; in
`min-epoch-day..=max-epoch-day`.

</div>
</article>

<article class="doc-entry" id="v-tomorrow">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tomorrow</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a></code><span class="doc-sig-links"><a href="#v-tomorrow" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Date/source/#L254" title="View source at line 254">Source</a></span></div>
<div class="doc-body">

The next day; `None` only after 9999-12-31. Within a month this is
the word plus one (the day is the word's lowest field).

</div>
</article>

<article class="doc-entry" id="v-weekday">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">weekday</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Weekday/#t-Weekday" title="Data.Time.Weekday.Weekday">Weekday</a></code><span class="doc-sig-links"><a href="#v-weekday" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Date/source/#L319" title="View source at line 319">Source</a></span></div>
<div class="doc-body">

1970-01-01 (day 0) is a Thursday, three days after a Monday, so
the weekday is Monday moved by `epoch-day + 3` (within +-4371591;
`W.add-days` reduces modulo 7 for either sign).

</div>
</article>

<article class="doc-entry" id="v-year">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">year</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> - 9999 9999 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-year" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Date/source/#L169" title="View source at line 169">Source</a></span></div>
<div class="doc-body">

Fields. Each is a shift or a mask of the word; the clamp states
the range in the type and never fires (see NAMED REFUSALS).

</div>
</article>

<article class="doc-entry" id="v-yesterday">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">yesterday</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a></code><span class="doc-sig-links"><a href="#v-yesterday" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Date/source/#L265" title="View source at line 265">Source</a></span></div>
<div class="doc-body">

The previous day; `None` only before -9999-01-01.

</div>
</article>

# Instances

Exported checked instances declared in this module. For law checking, see [rewrites and optimization](../reference/12-rewrites-and-optimization.md).

<ul class="doc-module-instances">
<li id="i-Eq_Date"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Time.Date.Eq_Date</code></span></li>
<li id="i-Ord_Date"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Time.Date.Ord_Date</code></span></li>
</ul>
