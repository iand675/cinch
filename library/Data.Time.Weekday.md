# Data.Time.Weekday

The seven days of the week with ordering and offset arithmetic.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Data.Time.Weekday`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Time.Weekday.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Algebra.Arithmetic](Algebra.Arithmetic.md), [Data.Machine](Data.Machine.md), [Data.Option](Data.Option.md), [Data.Ordering](Data.Ordering.md), [Relation.Eq](Relation.Eq.md), [Relation.Ord](Relation.Ord.md).

Non-embedded source closure: [Data.Time.Weekday](Data.Time.Weekday.md).

<details><summary>Behaviour and boundary notes from the source</summary>

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

# Types and constructors

<a id="type-5765656b646179"></a>

## `Weekday`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Weekday</span></code></pre>

[Source](/reference-sources/Data.Time.Weekday.cinch)

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Monday</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Weekday</span>.<span class="tok-constructor">Weekday</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Tuesday</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Weekday</span>.<span class="tok-constructor">Weekday</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Wednesday</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Weekday</span>.<span class="tok-constructor">Weekday</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Thursday</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Weekday</span>.<span class="tok-constructor">Weekday</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Friday</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Weekday</span>.<span class="tok-constructor">Weekday</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Saturday</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Weekday</span>.<span class="tok-constructor">Weekday</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Sunday</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Weekday</span>.<span class="tok-constructor">Weekday</span></code></pre>

# Values

<a id="value-6164642d64617973"></a>

## `add-days`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">add-days</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Weekday</span>.<span class="tok-constructor">Weekday</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Weekday</span>.<span class="tok-constructor">Weekday</span>))</code></pre>

`n` days after `d` (before it for a negative `n`), for every
`I64`. Overflow argument: `n` is first reduced to its truncated
remainder modulo 7, in -6..=6; adding the weekday's offset (0..=6)
and 7 gives 1..=19, so the wrapping additions never wrap.

[Source](/reference-sources/Data.Time.Weekday.cinch) · line 180

<a id="value-636d70"></a>

## `cmp`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cmp</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Weekday</span>.<span class="tok-constructor">Weekday</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Weekday</span>.<span class="tok-constructor">Weekday</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Ordering</span>.<span class="tok-constructor">Ordering</span>))</code></pre>

The ISO order: Monday first, Sunday last.

[Source](/reference-sources/Data.Time.Weekday.cinch) · line 205

<a id="value-646179732d73696e6365"></a>

## `days-since`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">days-since</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Weekday</span>.<span class="tok-constructor">Weekday</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Weekday</span>.<span class="tok-constructor">Weekday</span> -&gt; <span class="tok-constructor">I64</span>))</code></pre>

The backward distance from `a` to `b`, in 0..=6: the number of
times `previous` must be applied to `a` to reach `b`.

[Source](/reference-sources/Data.Time.Weekday.cinch) · line 195

<a id="value-646179732d756e74696c"></a>

## `days-until`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">days-until</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Weekday</span>.<span class="tok-constructor">Weekday</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Weekday</span>.<span class="tok-constructor">Weekday</span> -&gt; <span class="tok-constructor">I64</span>))</code></pre>

The forward distance from `a` to `b`, in 0..=6: the number of
times `next` must be applied to `a` to reach `b`. Both offsets
are in 0..=6, so the difference is in -6..=6 and nothing wraps.

[Source](/reference-sources/Data.Time.Weekday.cinch) · line 188

<a id="value-66726f6d2d6d6f6e6461792d6f6e65"></a>

## `from-monday-one`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">from-monday-one</span> :: ((<span class="tok-variable">n</span> :: (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">1</span> <span class="tok-number">7</span> <span class="tok-constructor">I64</span>)) -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Weekday</span>.<span class="tok-constructor">Weekday</span>)</code></pre>

Total over the refined argument: the last arm is the range's last
value (7), not an impossible case.

[Source](/reference-sources/Data.Time.Weekday.cinch) · line 108

<a id="value-66726f6d2d6d6f6e6461792d6f6e652d636865636b6564"></a>

## `from-monday-one-checked`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">from-monday-one-checked</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Weekday</span>.<span class="tok-constructor">Weekday</span>))</code></pre>

The readers for an unrefined `I64`: `None` exactly outside the
numbering's range.

[Source](/reference-sources/Data.Time.Weekday.cinch) · line 140

<a id="value-66726f6d2d6d6f6e6461792d7a65726f"></a>

## `from-monday-zero`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">from-monday-zero</span> :: ((<span class="tok-variable">n</span> :: (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">0</span> <span class="tok-number">6</span> <span class="tok-constructor">I64</span>)) -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Weekday</span>.<span class="tok-constructor">Weekday</span>)</code></pre>

[Source](/reference-sources/Data.Time.Weekday.cinch)

<a id="value-66726f6d2d6d6f6e6461792d7a65726f2d636865636b6564"></a>

## `from-monday-zero-checked`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">from-monday-zero-checked</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Weekday</span>.<span class="tok-constructor">Weekday</span>))</code></pre>

[Source](/reference-sources/Data.Time.Weekday.cinch)

<a id="value-66726f6d2d73756e6461792d7a65726f"></a>

## `from-sunday-zero`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">from-sunday-zero</span> :: ((<span class="tok-variable">n</span> :: (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">0</span> <span class="tok-number">6</span> <span class="tok-constructor">I64</span>)) -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Weekday</span>.<span class="tok-constructor">Weekday</span>)</code></pre>

[Source](/reference-sources/Data.Time.Weekday.cinch)

<a id="value-66726f6d2d73756e6461792d7a65726f2d636865636b6564"></a>

## `from-sunday-zero-checked`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">from-sunday-zero-checked</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Weekday</span>.<span class="tok-constructor">Weekday</span>))</code></pre>

[Source](/reference-sources/Data.Time.Weekday.cinch)

<a id="value-69732d7765656b656e64"></a>

## `is-weekend`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">is-weekend</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Weekday</span>.<span class="tok-constructor">Weekday</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

[Source](/reference-sources/Data.Time.Weekday.cinch)

<a id="value-6e657874"></a>

## `next`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">next</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Weekday</span>.<span class="tok-constructor">Weekday</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Weekday</span>.<span class="tok-constructor">Weekday</span>)</code></pre>

The cycle.

[Source](/reference-sources/Data.Time.Weekday.cinch) · line 156

<a id="value-70726576696f7573"></a>

## `previous`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">previous</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Weekday</span>.<span class="tok-constructor">Weekday</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Weekday</span>.<span class="tok-constructor">Weekday</span>)</code></pre>

[Source](/reference-sources/Data.Time.Weekday.cinch)

<a id="value-746f2d6d6f6e6461792d6f6e65"></a>

## `to-monday-one`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">to-monday-one</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Weekday</span>.<span class="tok-constructor">Weekday</span> -&gt; (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">1</span> <span class="tok-number">7</span> <span class="tok-constructor">I64</span>))</code></pre>

ISO 8601: Monday = 1 .. Sunday = 7.

[Source](/reference-sources/Data.Time.Weekday.cinch) · line 74

<a id="value-746f2d6d6f6e6461792d7a65726f"></a>

## `to-monday-zero`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">to-monday-zero</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Weekday</span>.<span class="tok-constructor">Weekday</span> -&gt; (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">0</span> <span class="tok-number">6</span> <span class="tok-constructor">I64</span>))</code></pre>

Monday = 0 .. Sunday = 6: the days elapsed since Monday.

[Source](/reference-sources/Data.Time.Weekday.cinch) · line 85

<a id="value-746f2d73756e6461792d7a65726f"></a>

## `to-sunday-zero`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">to-sunday-zero</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Weekday</span>.<span class="tok-constructor">Weekday</span> -&gt; (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">0</span> <span class="tok-number">6</span> <span class="tok-constructor">I64</span>))</code></pre>

C's `tm_wday`: Sunday = 0 .. Saturday = 6.

[Source](/reference-sources/Data.Time.Weekday.cinch) · line 96

# Instances

These are checked exported instance entries. Law/sample-check status is separate from the existence of an instance; see [laws and optimization](../reference/12-rewrites-and-optimization.md).

| Instance | Target | Resolution |
| --- | --- | --- |
| `Data.Time.Weekday.Eq_Weekday` | `Relation.Eq.Eq Data.Time.Weekday.Weekday` | Implicit |
| `Data.Time.Weekday.Ord_Weekday` | `Relation.Ord.Ord Data.Time.Weekday.Weekday` | Implicit |

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Data/Time/Weekday.cinch`. Source SHA-256: `5f0d65473156ec4f948989a0a1bf99b155c644495f8d29da28f5b31e0e6ddaf8`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
