# Data.Time.TimeOfDay

Civil wall-clock times of day with nanosecond precision.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Data.Time.TimeOfDay`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Time.TimeOfDay.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Algebra.Arithmetic](Algebra.Arithmetic.md), [Data.Machine](Data.Machine.md), [Data.Option](Data.Option.md), [Data.Ordering](Data.Ordering.md), [Relation.Eq](Relation.Eq.md), [Relation.Ord](Relation.Ord.md).

Non-embedded source closure: [Data.Time.TimeOfDay](Data.Time.TimeOfDay.md).

<details><summary>Behaviour and boundary notes from the source</summary>

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

# Types and constructors

<a id="type-54696d654f66446179"></a>

## `TimeOfDay`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">TimeOfDay</span></code></pre>

Nanoseconds since midnight (header). Not exported.

[Source](/reference-sources/Data.Time.TimeOfDay.cinch) · line 74

# Values

<a id="value-6164642d6e616e6f732d7772617070696e67"></a>

## `add-nanos-wrapping`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">add-nanos-wrapping</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">TimeOfDay</span>.<span class="tok-constructor">TimeOfDay</span> -&gt; (<span class="tok-constructor">I64</span>, <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">TimeOfDay</span>.<span class="tok-constructor">TimeOfDay</span>)))</code></pre>

Adds `n` nanoseconds (signed, any `I64`), wrapping around midnight:
the signed whole-day carry and the wrapped time.
`add-nanos-wrapping 1 max-time` is `(1, midnight)` and
`add-nanos-wrapping (-1) midnight` is `(-1, max-time)`.

[Source](/reference-sources/Data.Time.TimeOfDay.cinch) · line 216

<a id="value-6164642d7365636f6e64732d6e616e6f732d7772617070696e67"></a>

## `add-seconds-nanos-wrapping`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">add-seconds-nanos-wrapping</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">TimeOfDay</span>.<span class="tok-constructor">TimeOfDay</span> -&gt; (<span class="tok-constructor">I64</span>, <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">TimeOfDay</span>.<span class="tok-constructor">TimeOfDay</span>))))</code></pre>

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

[Source](/reference-sources/Data.Time.TimeOfDay.cinch) · line 196

<a id="value-636d70"></a>

## `cmp`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cmp</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">TimeOfDay</span>.<span class="tok-constructor">TimeOfDay</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">TimeOfDay</span>.<span class="tok-constructor">TimeOfDay</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Ordering</span>.<span class="tok-constructor">Ordering</span>))</code></pre>

Order: a compare of the nanosecond counts.

[Source](/reference-sources/Data.Time.TimeOfDay.cinch) · line 229

<a id="value-646966662d6e616e6f73"></a>

## `diff-nanos`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">diff-nanos</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">TimeOfDay</span>.<span class="tok-constructor">TimeOfDay</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">TimeOfDay</span>.<span class="tok-constructor">TimeOfDay</span> -&gt; <span class="tok-constructor">I64</span>))</code></pre>

`diff-nanos start finish`: finish minus start in nanoseconds,
negative when `finish` is the earlier time. Both counts are in
0..=86399999999999, so the difference cannot wrap.

[Source](/reference-sources/Data.Time.TimeOfDay.cinch) · line 222

<a id="value-66726f6d2d6e616e6f732d6f662d646179"></a>

## `from-nanos-of-day`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">from-nanos-of-day</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">TimeOfDay</span>.<span class="tok-constructor">TimeOfDay</span>))</code></pre>

From nanoseconds since midnight: `None` outside 0..=86399999999999.

[Source](/reference-sources/Data.Time.TimeOfDay.cinch) · line 104

<a id="value-686d73"></a>

## `hms`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hms</span> :: ((<span class="tok-variable">h</span> :: (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">0</span> <span class="tok-number">23</span> <span class="tok-constructor">I64</span>)) -&gt; ((<span class="tok-variable">m</span> :: (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">0</span> <span class="tok-number">59</span> <span class="tok-constructor">I64</span>)) -&gt; ((<span class="tok-variable">s</span> :: (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">0</span> <span class="tok-number">59</span> <span class="tok-constructor">I64</span>)) -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">TimeOfDay</span>.<span class="tok-constructor">TimeOfDay</span>)))</code></pre>

Hour, minute, second with a zero subsecond.

[Source](/reference-sources/Data.Time.TimeOfDay.cinch) · line 90

<a id="value-686f7572"></a>

## `hour`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hour</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">TimeOfDay</span>.<span class="tok-constructor">TimeOfDay</span> -&gt; (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">0</span> <span class="tok-number">23</span> <span class="tok-constructor">I64</span>))</code></pre>

Fields. Each is a division of the count; the clamp states the
range in the type and never fires (see NAMED REFUSALS).

[Source](/reference-sources/Data.Time.TimeOfDay.cinch) · line 136

<a id="value-6d61782d74696d65"></a>

## `max-time`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">max-time</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">TimeOfDay</span>.<span class="tok-constructor">TimeOfDay</span></code></pre>

23:59:59.999999999.

[Source](/reference-sources/Data.Time.TimeOfDay.cinch) · line 124

<a id="value-6d6963726f7365636f6e64"></a>

## `microsecond`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">microsecond</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">TimeOfDay</span>.<span class="tok-constructor">TimeOfDay</span> -&gt; (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">0</span> <span class="tok-number">999</span> <span class="tok-constructor">I64</span>))</code></pre>

[Source](/reference-sources/Data.Time.TimeOfDay.cinch)

<a id="value-6d69646e69676874"></a>

## `midnight`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">midnight</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">TimeOfDay</span>.<span class="tok-constructor">TimeOfDay</span></code></pre>

00:00:00 (also the least time) and 12:00:00.

[Source](/reference-sources/Data.Time.TimeOfDay.cinch) · line 114

<a id="value-6d696c6c697365636f6e64"></a>

## `millisecond`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">millisecond</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">TimeOfDay</span>.<span class="tok-constructor">TimeOfDay</span> -&gt; (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">0</span> <span class="tok-number">999</span> <span class="tok-constructor">I64</span>))</code></pre>

The fractional second's three three-digit components (Jiff's
`millisecond`, `microsecond`, `nanosecond`): 12:00:00.123456789 has
millisecond 123, microsecond 456, nanosecond 789.

[Source](/reference-sources/Data.Time.TimeOfDay.cinch) · line 160

<a id="value-6d696e2d74696d65"></a>

## `min-time`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">min-time</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">TimeOfDay</span>.<span class="tok-constructor">TimeOfDay</span></code></pre>

[Source](/reference-sources/Data.Time.TimeOfDay.cinch)

<a id="value-6d696e757465"></a>

## `minute`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">minute</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">TimeOfDay</span>.<span class="tok-constructor">TimeOfDay</span> -&gt; (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">0</span> <span class="tok-number">59</span> <span class="tok-constructor">I64</span>))</code></pre>

[Source](/reference-sources/Data.Time.TimeOfDay.cinch)

<a id="value-6e616e6f732d7065722d646179"></a>

## `nanos-per-day`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">nanos-per-day</span> :: <span class="tok-constructor">I64</span></code></pre>

The length of a civil day in nanoseconds (no leap seconds).

[Source](/reference-sources/Data.Time.TimeOfDay.cinch) · line 128

<a id="value-6e616e6f7365636f6e64"></a>

## `nanosecond`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">nanosecond</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">TimeOfDay</span>.<span class="tok-constructor">TimeOfDay</span> -&gt; (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">0</span> <span class="tok-number">999</span> <span class="tok-constructor">I64</span>))</code></pre>

[Source](/reference-sources/Data.Time.TimeOfDay.cinch)

<a id="value-6e6f6f6e"></a>

## `noon`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">noon</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">TimeOfDay</span>.<span class="tok-constructor">TimeOfDay</span></code></pre>

[Source](/reference-sources/Data.Time.TimeOfDay.cinch)

<a id="value-7365636f6e64"></a>

## `second`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">second</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">TimeOfDay</span>.<span class="tok-constructor">TimeOfDay</span> -&gt; (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">0</span> <span class="tok-number">59</span> <span class="tok-constructor">I64</span>))</code></pre>

[Source](/reference-sources/Data.Time.TimeOfDay.cinch)

<a id="value-7375627365632d6e616e6f7365636f6e64"></a>

## `subsec-nanosecond`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">subsec-nanosecond</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">TimeOfDay</span>.<span class="tok-constructor">TimeOfDay</span> -&gt; (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">0</span> <span class="tok-number">999999999</span> <span class="tok-constructor">I64</span>))</code></pre>

The whole fractional second, in nanoseconds.

[Source](/reference-sources/Data.Time.TimeOfDay.cinch) · line 152

<a id="value-74696d652d6f662d646179"></a>

## `time-of-day`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">time-of-day</span> :: ((<span class="tok-variable">h</span> :: (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">0</span> <span class="tok-number">23</span> <span class="tok-constructor">I64</span>)) -&gt; ((<span class="tok-variable">m</span> :: (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">0</span> <span class="tok-number">59</span> <span class="tok-constructor">I64</span>)) -&gt; ((<span class="tok-variable">s</span> :: (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">0</span> <span class="tok-number">59</span> <span class="tok-constructor">I64</span>)) -&gt; ((<span class="tok-variable">ns</span> :: (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">0</span> <span class="tok-number">999999999</span> <span class="tok-constructor">I64</span>)) -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">TimeOfDay</span>.<span class="tok-constructor">TimeOfDay</span>))))</code></pre>

Total: every range is in an argument type, so
`time-of-day 13 45 30 0` checks with nothing else written and
`time-of-day 24 0 0 0` is refused at compile time. The count is at
most 23 * 3600e9 + 59 * 60e9 + 59 * 1e9 + 999999999 =
86399999999999, so nothing wraps.

[Source](/reference-sources/Data.Time.TimeOfDay.cinch) · line 86

<a id="value-74696d652d6f662d6461792d636865636b6564"></a>

## `time-of-day-checked`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">time-of-day-checked</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">TimeOfDay</span>.<span class="tok-constructor">TimeOfDay</span>)))))</code></pre>

The same over plain `I64` fields: `None` when any field is outside
its range.

[Source](/reference-sources/Data.Time.TimeOfDay.cinch) · line 95

<a id="value-746f2d6e616e6f732d6f662d646179"></a>

## `to-nanos-of-day`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">to-nanos-of-day</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">TimeOfDay</span>.<span class="tok-constructor">TimeOfDay</span> -&gt; <span class="tok-constructor">I64</span>)</code></pre>

Nanoseconds since midnight, in 0..=86399999999999.

[Source](/reference-sources/Data.Time.TimeOfDay.cinch) · line 109

# Instances

These are checked exported instance entries. Law/sample-check status is separate from the existence of an instance; see [laws and optimization](../reference/12-rewrites-and-optimization.md).

| Instance | Target | Resolution |
| --- | --- | --- |
| `Data.Time.TimeOfDay.Eq_TimeOfDay` | `Relation.Eq.Eq Data.Time.TimeOfDay.TimeOfDay` | Implicit |
| `Data.Time.TimeOfDay.Ord_TimeOfDay` | `Relation.Ord.Ord Data.Time.TimeOfDay.TimeOfDay` | Implicit |

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Data/Time/TimeOfDay.cinch`. Source SHA-256: `c5a8af68643957b15c1eabba4f52a562b75b064188e6e588624d7cc415090296`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
