# Data.Date

Civil date values, validation, parsing/formatting, and calendar calculations.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--source">Ship source</span> <code>import Data.Date</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Data.Date/source/">View source</a><a href="/packages/search/?q=%2BData.Date">Search module</a><a href="/reference-sources/Data.Date.cinch" download="Data.Date.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bytes](Data.Bytes.md), [Data.Char](Data.Char.md), [Data.Int](Data.Int.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Ordering](Data.Ordering.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Relation.Eq](Relation.Eq.md), [Relation.Ord](Relation.Ord.md), [Text.Show](Text.Show.md).

<details class="doc-closure"><summary>Ship-source closure (1 file)</summary>

Copy these modules beside your entry file before importing `Data.Date`: [Data.Date](Data.Date.md).

```sh
cp seed/prelude/Data/Date.cinch ./Data.Date.cinch
```

</details>

<details class="doc-notes"><summary>Module notes</summary>

Data.Date: the proleptic Gregorian calendar date.

A `Date` is a calendar date in the proleptic Gregorian calendar: a
year (`Int`, so the years before 1 are representable and the year 0
is ISO 8601's astronomical year 0), a month (1..=12) and a day of
month (1..=that month's length that year). The representation
constructor is not exported (D143's surface rule is what makes the
omission bind): `date` is the only way in, and it refuses a month
outside 1..=12 and a day outside the month's own length, so an
invalid date cannot exist. Arithmetic preserves the invariant by
construction: days move on the epoch-day line
(`to-epoch-days`/`from-epoch-days`), never field by field.

Design law shape. A year is a leap year exactly when it is divisible
by 4 and not by 100 unless also by 400 (`is-leap-year`), and that one
predicate fixes every month length (`days-in-month`). On top of it:
the epoch-day conversions invert on the whole line, so
`to-epoch-days` is a bijection with `from-epoch-days` and 1970-01-01
is 0 by construction; `add-days` and `sub-days` undo each other and
`diff-days` measures their difference (`diff-days d (add-days n d)`
is n, and adding m then n days is adding m + n); `cmp` is a total
order whose sign is `diff-days`'s; `day-of-year` is 1 on January 1
and the year's length on December 31; `day-of-week` is
`to-epoch-days` read modulo 7 with 1970-01-01 (a Thursday) at 4,
which is Ruby's `wday` convention (0 is Sunday); and ISO 8601
`to-string`/`from-string` round trip. The checks are
seed/tests/lib_civil_time.rs.

NAMED REFUSALS (boundaries this module deliberately does not
cross):
  - timezones and UTC offsets: a `Date` is a calendar value, not an
    instant, and carries no zone; no offset and no tz database exist
    in this slice (Data.Time's conversions are UTC-explicit, and its
    wall-nanosecond line is the only instant surface here).
  - locale-aware formatting, and month or weekday names: the
    bootstrap carries no locale or name data, so the ISO 8601
    `YYYY-MM-DD` rendering is the only one.
  - the Julian calendar and the Gregorian reform: the proleptic
    Gregorian calendar is extended before 1582 by definition, and no
    reform cutover is modelled (a date before it is read and written
    proleptically).
  - a day count wider than `I64`: `add-days`/`sub-days` take `Nat`
    counts and convert through `nat-to-int`, the machine-width step
    every `Nat`-to-`Int` conversion in the prelude takes (a count
    above `I64`'s range faults in the runtime conversion).

Home (design/base-library.md): `Data.*` holds the concrete types, so
the `Eq`/`Ord` instances live with `Date` here. Ported item: Ruby's
`Date` (design/stdlib-parity-ruby.md, P38 "Time: civil
`Date`/`Time`/`DateTime` (calendars, formatting, parsing) atop the
P34 `Clock` capability").

</details>

<details class="doc-synopsis" open><summary>Synopsis (18 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-Date">Date</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Date</strong></code></li><li><a class="syn-name" href="#t-DateParseError">DateParseError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">DateParseError</strong></code></li><li><a class="syn-name" href="#v-add-days">add-days</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">add-days</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Date/#t-Date" title="Data.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Date/#t-Date" title="Data.Date.Date">Date</a></code></li><li><a class="syn-name" href="#v-cmp">cmp</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cmp</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Date/#t-Date" title="Data.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Date/#t-Date" title="Data.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Ordering/#t-Ordering" title="Data.Ordering.Ordering">Ordering</a></code></li><li><a class="syn-name" href="#v-date">date</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">date</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Date/#t-Date" title="Data.Date.Date">Date</a></code></li><li><a class="syn-name" href="#v-day">day</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">day</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Date/#t-Date" title="Data.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-day-of-week">day-of-week</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">day-of-week</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Date/#t-Date" title="Data.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-day-of-year">day-of-year</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">day-of-year</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Date/#t-Date" title="Data.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-days-in-month">days-in-month</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">days-in-month</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-diff-days">diff-days</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">diff-days</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Date/#t-Date" title="Data.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Date/#t-Date" title="Data.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a></code></li><li><a class="syn-name" href="#v-from-epoch-days">from-epoch-days</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">from-epoch-days</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Date/#t-Date" title="Data.Date.Date">Date</a></code></li><li><a class="syn-name" href="#v-from-string">from-string</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">from-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Date/#t-DateParseError" title="Data.Date.DateParseError">DateParseError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Date/#t-Date" title="Data.Date.Date">Date</a></code></li><li><a class="syn-name" href="#v-is-leap-year">is-leap-year</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">is-leap-year</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-month">month</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">month</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Date/#t-Date" title="Data.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-sub-days">sub-days</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sub-days</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Date/#t-Date" title="Data.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Date/#t-Date" title="Data.Date.Date">Date</a></code></li><li><a class="syn-name" href="#v-to-epoch-days">to-epoch-days</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">to-epoch-days</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Date/#t-Date" title="Data.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a></code></li><li><a class="syn-name" href="#v-to-string">to-string</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">to-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Date/#t-Date" title="Data.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-year">year</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">year</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Date/#t-Date" title="Data.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-Date">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Date</strong></code><span class="doc-sig-links"><a href="#t-Date" title="Permalink">#</a><a href="/packages/stdlib/Data.Date/source/#L78" title="View source at line 78">Source</a></span></div>
<div class="doc-body">

The calendar date: year, month, day of month. Not exported (the
validity invariant above; `date` is the only constructor).

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-Date"><code><strong class="tok-constructor">Date</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Date/#t-Date" title="Data.Date.Date">Date</a></code></li></ul></div>
<details class="doc-instances"><summary>Instances (4)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Date/#t-Date" title="Data.Date.Date">Date</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Date/#instances">Data.Date</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Date/#t-Date" title="Data.Date.Date">Date</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Date/#instances">Data.Date</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Time.Date/#instances">Data.Time.Date</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Time.Date/#instances">Data.Time.Date</a></span></li></ul></details>
</div>
</article>

<article class="doc-entry" id="t-DateParseError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">DateParseError</strong></code><span class="doc-sig-links"><a href="#t-DateParseError" title="Permalink">#</a><a href="/packages/stdlib/Data.Date/source/#L84" title="View source at line 84">Source</a></span></div>
<div class="doc-body">

The literal reader's errors, one per refusal: the text is not a
date at all (`[-]dddd-MM-DD`), or the fields name no date. A parse
never yields a partially trusted value, so there is no third case.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-NotADate"><code><strong class="tok-constructor">NotADate</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Date/#t-DateParseError" title="Data.Date.DateParseError">DateParseError</a></code></li><li id="k-InvalidDate"><code><strong class="tok-constructor">InvalidDate</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Date/#t-DateParseError" title="Data.Date.DateParseError">DateParseError</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-add-days">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">add-days</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Date/#t-Date" title="Data.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Date/#t-Date" title="Data.Date.Date">Date</a></code><span class="doc-sig-links"><a href="#v-add-days" title="Permalink">#</a><a href="/packages/stdlib/Data.Date/source/#L184" title="View source at line 184">Source</a></span></div>
<div class="doc-body">

`n` days later (crossing month and year boundaries by construction).

</div>
</article>

<article class="doc-entry" id="v-cmp">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cmp</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Date/#t-Date" title="Data.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Date/#t-Date" title="Data.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Ordering/#t-Ordering" title="Data.Ordering.Ordering">Ordering</a></code><span class="doc-sig-links"><a href="#v-cmp" title="Permalink">#</a><a href="/packages/stdlib/Data.Date/source/#L197" title="View source at line 197">Source</a></span></div>
</article>

<article class="doc-entry" id="v-date">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">date</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Date/#t-Date" title="Data.Date.Date">Date</a></code><span class="doc-sig-links"><a href="#v-date" title="Permalink">#</a><a href="/packages/stdlib/Data.Date/source/#L95" title="View source at line 95">Source</a></span></div>
<div class="doc-body">

The only constructor: month 1..=12, day 1..=that month's length.
`None` is the refusal for every other field combination, so an
out-of-range `Date` has no representation.

</div>
</article>

<article class="doc-entry" id="v-day">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">day</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Date/#t-Date" title="Data.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-day" title="Permalink">#</a><a href="/packages/stdlib/Data.Date/source/#L111" title="View source at line 111">Source</a></span></div>
</article>

<article class="doc-entry" id="v-day-of-week">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">day-of-week</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Date/#t-Date" title="Data.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-day-of-week" title="Permalink">#</a><a href="/packages/stdlib/Data.Date/source/#L211" title="View source at line 211">Source</a></span></div>
<div class="doc-body">

The weekday, 0 for Sunday through 6 for Saturday (Ruby's `wday`).
1970-01-01 (day 0) is a Thursday, hence the +4; the modulo is
floored so the pre-epoch days land in range too.

</div>
</article>

<article class="doc-entry" id="v-day-of-year">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">day-of-year</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Date/#t-Date" title="Data.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-day-of-year" title="Permalink">#</a><a href="/packages/stdlib/Data.Date/source/#L203" title="View source at line 203">Source</a></span></div>
<div class="doc-body">

1 on January 1, the year's length on December 31.

</div>
</article>

<article class="doc-entry" id="v-days-in-month">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">days-in-month</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-days-in-month" title="Permalink">#</a><a href="/packages/stdlib/Data.Date/source/#L127" title="View source at line 127">Source</a></span></div>
<div class="doc-body">

The month's length in `y`: 31, 30, 29 or 28. A month outside
1..=12 is 0 (out of the calendar; `date` refuses it before this
is consulted with one).

</div>
</article>

<article class="doc-entry" id="v-diff-days">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">diff-days</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Date/#t-Date" title="Data.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Date/#t-Date" title="Data.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a></code><span class="doc-sig-links"><a href="#v-diff-days" title="Permalink">#</a><a href="/packages/stdlib/Data.Date/source/#L194" title="View source at line 194">Source</a></span></div>
<div class="doc-body">

The days elapsed from the first argument to the second: `diff-days
start finish`, negative when `finish` precedes `start`. The exact
inverse of `add-days`/`sub-days` in both directions.

</div>
</article>

<article class="doc-entry" id="v-from-epoch-days">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">from-epoch-days</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Date/#t-Date" title="Data.Date.Date">Date</a></code><span class="doc-sig-links"><a href="#v-from-epoch-days" title="Permalink">#</a><a href="/packages/stdlib/Data.Date/source/#L169" title="View source at line 169">Source</a></span></div>
<div class="doc-body">

The inverse: every `Int` is a day, so the result is total. The
fields come out of the decomposition already in range (the
construction is the proof), so the private constructor is used
directly rather than routing through `date`'s `Option`.

</div>
</article>

<article class="doc-entry" id="v-from-string">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">from-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Date/#t-DateParseError" title="Data.Date.DateParseError">DateParseError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Date/#t-Date" title="Data.Date.Date">Date</a></code><span class="doc-sig-links"><a href="#v-from-string" title="Permalink">#</a><a href="/packages/stdlib/Data.Date/source/#L241" title="View source at line 241">Source</a></span></div>
<div class="doc-body">

Strict ISO 8601: an optional `-`, at least four year digits, and
two-digit month and day separated by `-`, with nothing before or
after. `NotADate` is the shape refusal (`2024-2-9`, `junk`, a
trailing time part); `InvalidDate` is the fields refusal
(`2023-02-29`, month 13, day 0).

</div>
</article>

<article class="doc-entry" id="v-is-leap-year">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">is-leap-year</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-is-leap-year" title="Permalink">#</a><a href="/packages/stdlib/Data.Date/source/#L118" title="View source at line 118">Source</a></span></div>
<div class="doc-body">

The Gregorian leap rule, total over `Int` (the proleptic calendar
defines it for every year, including year 0 and the negative
ones). Ruby's `Date.gregorian_leap?` is the same predicate.

</div>
</article>

<article class="doc-entry" id="v-month">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">month</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Date/#t-Date" title="Data.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-month" title="Permalink">#</a><a href="/packages/stdlib/Data.Date/source/#L107" title="View source at line 107">Source</a></span></div>
</article>

<article class="doc-entry" id="v-sub-days">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sub-days</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Date/#t-Date" title="Data.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Date/#t-Date" title="Data.Date.Date">Date</a></code><span class="doc-sig-links"><a href="#v-sub-days" title="Permalink">#</a><a href="/packages/stdlib/Data.Date/source/#L188" title="View source at line 188">Source</a></span></div>
<div class="doc-body">

`n` days earlier.

</div>
</article>

<article class="doc-entry" id="v-to-epoch-days">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">to-epoch-days</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Date/#t-Date" title="Data.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a></code><span class="doc-sig-links"><a href="#v-to-epoch-days" title="Permalink">#</a><a href="/packages/stdlib/Data.Date/source/#L153" title="View source at line 153">Source</a></span></div>
<div class="doc-body">

The day number of `d` counted from 1970-01-01 (negative before it).
This is the shifted-era decomposition (days-from-civil), which is
exactly the calendar's month term plus the year term; the era and
year-of-era steps use FLOORED division so the decomposition stays
at a non-negative remainder for the negative years too.

</div>
</article>

<article class="doc-entry" id="v-to-string">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">to-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Date/#t-Date" title="Data.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-to-string" title="Permalink">#</a><a href="/packages/stdlib/Data.Date/source/#L227" title="View source at line 227">Source</a></span></div>
</article>

<article class="doc-entry" id="v-year">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">year</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Date/#t-Date" title="Data.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a></code><span class="doc-sig-links"><a href="#v-year" title="Permalink">#</a><a href="/packages/stdlib/Data.Date/source/#L103" title="View source at line 103">Source</a></span></div>
</article>

# Instances

Exported checked instances declared in this module. For law checking, see [rewrites and optimization](../reference/12-rewrites-and-optimization.md).

<ul class="doc-module-instances">
<li id="i-Eq_Date"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Date/#t-Date" title="Data.Date.Date">Date</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Date.Eq_Date</code></span><div class="doc-child-prose">Instances (the class spellings of cmp; instances live with their type per design/base-library.md).</div></li>
<li id="i-Ord_Date"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Date/#t-Date" title="Data.Date.Date">Date</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Date.Ord_Date</code></span></li>
</ul>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Data/Date.cinch`](/packages/stdlib/Data.Date/source/). Source SHA-256: `5ab1002e8509257ef7fdfaf392112954771bc4163276747a213e8dd5aadf661e`.
