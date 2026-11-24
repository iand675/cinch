# Data.Time.Weekday

The seven days of the week with ordering and offset arithmetic.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--source">Ship source</span> <code>import Data.Time.Weekday</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Data.Time.Weekday/source/">View source</a><a href="/packages/search/?q=%2BData.Time.Weekday">Search module</a><a href="/reference-sources/Data.Time.Weekday.cinch" download="Data.Time.Weekday.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Algebra.Arithmetic](Algebra.Arithmetic.md), [Data.Machine](Data.Machine.md), [Data.Option](Data.Option.md), [Data.Ordering](Data.Ordering.md), [Relation.Eq](Relation.Eq.md), [Relation.Ord](Relation.Ord.md).

<details class="doc-closure"><summary>Ship-source closure (1 file)</summary>

Copy these modules beside your entry file before importing `Data.Time.Weekday`: [Data.Time.Weekday](Data.Time.Weekday.md).

```sh
cp seed/prelude/Data/Time/Weekday.cinch ./Data.Time.Weekday.cinch
```

</details>

<details class="doc-notes"><summary>Module notes</summary>

Data.Time.Weekday: the seven days of the week.

A `Weekday` is one of seven constructors, `Monday` through `Sunday`.
It is civil vocabulary only: it names a position in the seven-day
cycle and knows nothing about dates (Data.Time.Date's `weekday` is
the road from a date to its `Weekday`).

Two numberings are in common use and both are provided, each in both
directions:
  - ISO 8601, Monday = 1 .. Sunday = 7 (`to-monday-one`,
    `from-monday-one`);
  - the C `tm_wday` convention, Sunday = 0 .. Saturday = 6
    (`to-sunday-zero`, `from-sunday-zero`).
`to-monday-zero` (Monday = 0 .. Sunday = 6) is the offset the cycle
arithmetic itself uses and is exported for the same reason.

Design law shape. Every `to-` conversion returns its range in its
type, and every `from-` conversion over a refined argument is total
and inverts it: `from-monday-one (to-monday-one d)` is `d`, and the
same for Sunday zero. The `-checked` readers take a plain `I64` and
answer `None` exactly outside the numbering's range. `next` and
`previous` are inverse rotations of period seven; `add-days n d` is
`next` applied n times for n >= 0 and `previous` applied -n times
otherwise, for EVERY `I64` (the count is reduced modulo seven before
anything is added, so nothing can overflow). `days-until a b` is the
forward distance from `a` to `b` in `0..=6`:
`add-days (days-until a b) a` is `b`, and `days-until a a` is 0.
`days-since a b` is `days-until b a`. The order (`cmp`, `Eq`, `Ord`)
is the ISO order, Monday first and Sunday last.

NAMED REFUSALS:
  - weekday names, abbreviations and locale-dependent "first day of
    the week": no text lives here (Text.Time.* owns rendering), and
    the order is fixed at ISO's Monday-first.
  - refined results for `days-until`/`days-since`: machine integer
    arithmetic is opaque to the checker's difference logic, so the
    `0..=6` bound is stated in prose and tested, not carried in the
    type (a clamp would be the only way to state it, and it would be
    dead code).
  - a total machine division: `Data.Machine` has only
    `checked-rem-i64`, so the private `rem7` matches its `None` arm
    although the divisor is the literal 7 (never zero, never -1).

</details>

<details class="doc-synopsis" open><summary>Synopsis (17 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-Weekday">Weekday</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Weekday</strong></code></li><li><a class="syn-name" href="#v-add-days">add-days</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">add-days</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Weekday/#t-Weekday" title="Data.Time.Weekday.Weekday">Weekday</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Weekday/#t-Weekday" title="Data.Time.Weekday.Weekday">Weekday</a></code></li><li><a class="syn-name" href="#v-cmp">cmp</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cmp</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Weekday/#t-Weekday" title="Data.Time.Weekday.Weekday">Weekday</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Weekday/#t-Weekday" title="Data.Time.Weekday.Weekday">Weekday</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Ordering/#t-Ordering" title="Data.Ordering.Ordering">Ordering</a></code></li><li><a class="syn-name" href="#v-days-since">days-since</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">days-since</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Weekday/#t-Weekday" title="Data.Time.Weekday.Weekday">Weekday</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Weekday/#t-Weekday" title="Data.Time.Weekday.Weekday">Weekday</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-days-until">days-until</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">days-until</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Weekday/#t-Weekday" title="Data.Time.Weekday.Weekday">Weekday</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Weekday/#t-Weekday" title="Data.Time.Weekday.Weekday">Weekday</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-from-monday-one">from-monday-one</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">from-monday-one</strong> :: (<span class="tok-variable">n</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 1 7 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Weekday/#t-Weekday" title="Data.Time.Weekday.Weekday">Weekday</a></code></li><li><a class="syn-name" href="#v-from-monday-one-checked">from-monday-one-checked</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">from-monday-one-checked</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Weekday/#t-Weekday" title="Data.Time.Weekday.Weekday">Weekday</a></code></li><li><a class="syn-name" href="#v-from-monday-zero">from-monday-zero</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">from-monday-zero</strong> :: (<span class="tok-variable">n</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 6 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Weekday/#t-Weekday" title="Data.Time.Weekday.Weekday">Weekday</a></code></li><li><a class="syn-name" href="#v-from-monday-zero-checked">from-monday-zero-checked</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">from-monday-zero-checked</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Weekday/#t-Weekday" title="Data.Time.Weekday.Weekday">Weekday</a></code></li><li><a class="syn-name" href="#v-from-sunday-zero">from-sunday-zero</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">from-sunday-zero</strong> :: (<span class="tok-variable">n</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 6 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Weekday/#t-Weekday" title="Data.Time.Weekday.Weekday">Weekday</a></code></li><li><a class="syn-name" href="#v-from-sunday-zero-checked">from-sunday-zero-checked</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">from-sunday-zero-checked</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Weekday/#t-Weekday" title="Data.Time.Weekday.Weekday">Weekday</a></code></li><li><a class="syn-name" href="#v-is-weekend">is-weekend</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">is-weekend</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Weekday/#t-Weekday" title="Data.Time.Weekday.Weekday">Weekday</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-next">next</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">next</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Weekday/#t-Weekday" title="Data.Time.Weekday.Weekday">Weekday</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Weekday/#t-Weekday" title="Data.Time.Weekday.Weekday">Weekday</a></code></li><li><a class="syn-name" href="#v-previous">previous</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">previous</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Weekday/#t-Weekday" title="Data.Time.Weekday.Weekday">Weekday</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Weekday/#t-Weekday" title="Data.Time.Weekday.Weekday">Weekday</a></code></li><li><a class="syn-name" href="#v-to-monday-one">to-monday-one</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">to-monday-one</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Weekday/#t-Weekday" title="Data.Time.Weekday.Weekday">Weekday</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 1 7 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-to-monday-zero">to-monday-zero</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">to-monday-zero</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Weekday/#t-Weekday" title="Data.Time.Weekday.Weekday">Weekday</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 6 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-to-sunday-zero">to-sunday-zero</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">to-sunday-zero</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Weekday/#t-Weekday" title="Data.Time.Weekday.Weekday">Weekday</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 6 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-Weekday">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Weekday</strong></code><span class="doc-sig-links"><a href="#t-Weekday" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Weekday/source/#L60" title="View source at line 60">Source</a></span></div>
<div class="doc-body">
<div class="doc-children"><h4>Constructors</h4><ul><li id="k-Monday"><code><strong class="tok-constructor">Monday</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Weekday/#t-Weekday" title="Data.Time.Weekday.Weekday">Weekday</a></code></li><li id="k-Tuesday"><code><strong class="tok-constructor">Tuesday</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Weekday/#t-Weekday" title="Data.Time.Weekday.Weekday">Weekday</a></code></li><li id="k-Wednesday"><code><strong class="tok-constructor">Wednesday</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Weekday/#t-Weekday" title="Data.Time.Weekday.Weekday">Weekday</a></code></li><li id="k-Thursday"><code><strong class="tok-constructor">Thursday</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Weekday/#t-Weekday" title="Data.Time.Weekday.Weekday">Weekday</a></code></li><li id="k-Friday"><code><strong class="tok-constructor">Friday</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Weekday/#t-Weekday" title="Data.Time.Weekday.Weekday">Weekday</a></code></li><li id="k-Saturday"><code><strong class="tok-constructor">Saturday</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Weekday/#t-Weekday" title="Data.Time.Weekday.Weekday">Weekday</a></code></li><li id="k-Sunday"><code><strong class="tok-constructor">Sunday</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Weekday/#t-Weekday" title="Data.Time.Weekday.Weekday">Weekday</a></code></li></ul></div>
<details class="doc-instances"><summary>Instances (2)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Weekday/#t-Weekday" title="Data.Time.Weekday.Weekday">Weekday</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Time.Weekday/#instances">Data.Time.Weekday</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Weekday/#t-Weekday" title="Data.Time.Weekday.Weekday">Weekday</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Time.Weekday/#instances">Data.Time.Weekday</a></span></li></ul></details>
</div>
</article>

# Values

<article class="doc-entry" id="v-add-days">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">add-days</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Weekday/#t-Weekday" title="Data.Time.Weekday.Weekday">Weekday</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Weekday/#t-Weekday" title="Data.Time.Weekday.Weekday">Weekday</a></code><span class="doc-sig-links"><a href="#v-add-days" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Weekday/source/#L180" title="View source at line 180">Source</a></span></div>
<div class="doc-body">

`n` days after `d` (before it for a negative `n`), for every
`I64`. Overflow argument: `n` is first reduced to its truncated
remainder modulo 7, in -6..=6; adding the weekday's offset (0..=6)
and 7 gives 1..=19, so the wrapping additions never wrap.

</div>
</article>

<article class="doc-entry" id="v-cmp">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cmp</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Weekday/#t-Weekday" title="Data.Time.Weekday.Weekday">Weekday</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Weekday/#t-Weekday" title="Data.Time.Weekday.Weekday">Weekday</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Ordering/#t-Ordering" title="Data.Ordering.Ordering">Ordering</a></code><span class="doc-sig-links"><a href="#v-cmp" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Weekday/source/#L205" title="View source at line 205">Source</a></span></div>
<div class="doc-body">

The ISO order: Monday first, Sunday last.

</div>
</article>

<article class="doc-entry" id="v-days-since">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">days-since</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Weekday/#t-Weekday" title="Data.Time.Weekday.Weekday">Weekday</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Weekday/#t-Weekday" title="Data.Time.Weekday.Weekday">Weekday</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-days-since" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Weekday/source/#L195" title="View source at line 195">Source</a></span></div>
<div class="doc-body">

The backward distance from `a` to `b`, in 0..=6: the number of
times `previous` must be applied to `a` to reach `b`.

</div>
</article>

<article class="doc-entry" id="v-days-until">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">days-until</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Weekday/#t-Weekday" title="Data.Time.Weekday.Weekday">Weekday</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Weekday/#t-Weekday" title="Data.Time.Weekday.Weekday">Weekday</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-days-until" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Weekday/source/#L188" title="View source at line 188">Source</a></span></div>
<div class="doc-body">

The forward distance from `a` to `b`, in 0..=6: the number of
times `next` must be applied to `a` to reach `b`. Both offsets
are in 0..=6, so the difference is in -6..=6 and nothing wraps.

</div>
</article>

<article class="doc-entry" id="v-from-monday-one">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">from-monday-one</strong> :: (<span class="tok-variable">n</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 1 7 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Weekday/#t-Weekday" title="Data.Time.Weekday.Weekday">Weekday</a></code><span class="doc-sig-links"><a href="#v-from-monday-one" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Weekday/source/#L108" title="View source at line 108">Source</a></span></div>
<div class="doc-body">

Total over the refined argument: the last arm is the range's last
value (7), not an impossible case.

</div>
</article>

<article class="doc-entry" id="v-from-monday-one-checked">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">from-monday-one-checked</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Weekday/#t-Weekday" title="Data.Time.Weekday.Weekday">Weekday</a></code><span class="doc-sig-links"><a href="#v-from-monday-one-checked" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Weekday/source/#L140" title="View source at line 140">Source</a></span></div>
<div class="doc-body">

The readers for an unrefined `I64`: `None` exactly outside the
numbering's range.

</div>
</article>

<article class="doc-entry" id="v-from-monday-zero">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">from-monday-zero</strong> :: (<span class="tok-variable">n</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 6 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Weekday/#t-Weekday" title="Data.Time.Weekday.Weekday">Weekday</a></code><span class="doc-sig-links"><a href="#v-from-monday-zero" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Weekday/source/#L118" title="View source at line 118">Source</a></span></div>
</article>

<article class="doc-entry" id="v-from-monday-zero-checked">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">from-monday-zero-checked</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Weekday/#t-Weekday" title="Data.Time.Weekday.Weekday">Weekday</a></code><span class="doc-sig-links"><a href="#v-from-monday-zero-checked" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Weekday/source/#L144" title="View source at line 144">Source</a></span></div>
</article>

<article class="doc-entry" id="v-from-sunday-zero">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">from-sunday-zero</strong> :: (<span class="tok-variable">n</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 6 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Weekday/#t-Weekday" title="Data.Time.Weekday.Weekday">Weekday</a></code><span class="doc-sig-links"><a href="#v-from-sunday-zero" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Weekday/source/#L128" title="View source at line 128">Source</a></span></div>
</article>

<article class="doc-entry" id="v-from-sunday-zero-checked">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">from-sunday-zero-checked</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Weekday/#t-Weekday" title="Data.Time.Weekday.Weekday">Weekday</a></code><span class="doc-sig-links"><a href="#v-from-sunday-zero-checked" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Weekday/source/#L148" title="View source at line 148">Source</a></span></div>
</article>

<article class="doc-entry" id="v-is-weekend">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">is-weekend</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Weekday/#t-Weekday" title="Data.Time.Weekday.Weekday">Weekday</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-is-weekend" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Weekday/source/#L198" title="View source at line 198">Source</a></span></div>
</article>

<article class="doc-entry" id="v-next">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">next</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Weekday/#t-Weekday" title="Data.Time.Weekday.Weekday">Weekday</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Weekday/#t-Weekday" title="Data.Time.Weekday.Weekday">Weekday</a></code><span class="doc-sig-links"><a href="#v-next" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Weekday/source/#L156" title="View source at line 156">Source</a></span></div>
<div class="doc-body">

The cycle.

</div>
</article>

<article class="doc-entry" id="v-previous">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">previous</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Weekday/#t-Weekday" title="Data.Time.Weekday.Weekday">Weekday</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Weekday/#t-Weekday" title="Data.Time.Weekday.Weekday">Weekday</a></code><span class="doc-sig-links"><a href="#v-previous" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Weekday/source/#L166" title="View source at line 166">Source</a></span></div>
</article>

<article class="doc-entry" id="v-to-monday-one">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">to-monday-one</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Weekday/#t-Weekday" title="Data.Time.Weekday.Weekday">Weekday</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 1 7 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-to-monday-one" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Weekday/source/#L74" title="View source at line 74">Source</a></span></div>
<div class="doc-body">

ISO 8601: Monday = 1 .. Sunday = 7.

</div>
</article>

<article class="doc-entry" id="v-to-monday-zero">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">to-monday-zero</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Weekday/#t-Weekday" title="Data.Time.Weekday.Weekday">Weekday</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 6 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-to-monday-zero" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Weekday/source/#L85" title="View source at line 85">Source</a></span></div>
<div class="doc-body">

Monday = 0 .. Sunday = 6: the days elapsed since Monday.

</div>
</article>

<article class="doc-entry" id="v-to-sunday-zero">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">to-sunday-zero</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Weekday/#t-Weekday" title="Data.Time.Weekday.Weekday">Weekday</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 6 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-to-sunday-zero" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Weekday/source/#L96" title="View source at line 96">Source</a></span></div>
<div class="doc-body">

C's `tm_wday`: Sunday = 0 .. Saturday = 6.

</div>
</article>

# Instances

Exported checked instances declared in this module. For law checking, see [rewrites and optimization](../reference/12-rewrites-and-optimization.md).

<ul class="doc-module-instances">
<li id="i-Eq_Weekday"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Weekday/#t-Weekday" title="Data.Time.Weekday.Weekday">Weekday</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Time.Weekday.Eq_Weekday</code></span></li>
<li id="i-Ord_Weekday"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Weekday/#t-Weekday" title="Data.Time.Weekday.Weekday">Weekday</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Time.Weekday.Ord_Weekday</code></span></li>
</ul>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Data/Time/Weekday.cinch`](/packages/stdlib/Data.Time.Weekday/source/). Source SHA-256: `5f0d65473156ec4f948989a0a1bf99b155c644495f8d29da28f5b31e0e6ddaf8`.
