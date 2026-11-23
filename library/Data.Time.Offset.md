# Data.Time.Offset

Fixed offsets from UTC with validated ranges.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Data.Time.Offset`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Time.Offset.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Machine](Data.Machine.md), [Data.Option](Data.Option.md), [Data.Ordering](Data.Ordering.md), [Data.Time.DateTime](Data.Time.DateTime.md), [Data.Time.Timestamp](Data.Time.Timestamp.md), [Relation.Eq](Relation.Eq.md), [Relation.Ord](Relation.Ord.md).

Non-embedded source closure: [Data.Time.Date](Data.Time.Date.md), [Data.Time.DateTime](Data.Time.DateTime.md), [Data.Time.Duration](Data.Time.Duration.md), [Data.Time.Offset](Data.Time.Offset.md), [Data.Time.TimeOfDay](Data.Time.TimeOfDay.md), [Data.Time.Timestamp](Data.Time.Timestamp.md), [Data.Time.Weekday](Data.Time.Weekday.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Data.Time.Offset: a fixed distance from UTC
(design/datetime.md, layer L2 "zone", lane C).

An `Offset` is a whole number of seconds east of UTC: the amount
added to an instant's UTC reading to get the civil reading in some
place at that instant. It is the smallest piece of the zone layer:
a `TimeZone` (`Data.Time.Zone`) is a rule that picks an `Offset`
for each instant, and the two conversions here (instant to civil
reading at an offset, and back) are the arithmetic every zone
conversion is built from.

REPRESENTATION. One `I64` (seconds east) behind the private
constructor `MkOffset`, always within -93599..=93599 (25:59:59,
T3, Jiff's bound). Boxed today like every one-field `data` (see
`Data.Time.Duration`'s header).

Design law shape. `offset-from-seconds (offset-to-seconds o)` is
`Some o`; the order and equality are those of the seconds.
`offset-negate` is an involution (the range is symmetric).
`offset-hours`, `offset-minutes` and `offset-seconds` are the
magnitude's components and `offset-is-negative` its sign, so
`|seconds| = hours * 3600 + minutes * 60 + seconds`.
`offset-timestamp-to-datetime o t` is the civil reading of `t` at
`o`: the "as if UTC" civil line position `unix seconds + o`. It is
total because the timestamp range was chosen (T3) so that every
instant has a civil reading in years -9999..=9999 at every offset.
`offset-datetime-to-timestamp o x` is its inverse: `Some t`
exactly when the instant `x - o` is inside the timestamp range, and
then `offset-timestamp-to-datetime o t` is `x`.

NAMED REFUSALS:
  - offsets beyond 25:59:59 either way: `offset-from-seconds`
    answers `None`; the refined constructors do not admit them.
  - sub-second offsets (some historical local mean times carry
    fractions in the source data; TZif stores whole seconds).
  - a faulting or silently clamping conversion: the unreachable
    `None` in `offset-timestamp-to-datetime` (the civil module
    cannot know the timestamp bound) reads as the nearest end of the
    civil range, `min-datetime` or `max-datetime`; no input reaches
    it (see the function).
  - text (`+05:30`): `Text.Time.Iso` owns it.
  - a refined `InRange` result type on the component accessors is
    kept; the values are produced by a clamp that never fires
    (machine arithmetic is opaque to the refinement checker).

</details>

# Types and constructors

<a id="type-4f6666736574"></a>

## `Offset`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Offset</span></code></pre>

Seconds east of UTC, within -93599..=93599. Not exported.

[Source](/reference-sources/Data.Time.Offset.cinch) · line 67

# Values

<a id="value-6d61782d6f6666736574"></a>

## `max-offset`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">max-offset</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Offset</span>.<span class="tok-constructor">Offset</span></code></pre>

[Source](/reference-sources/Data.Time.Offset.cinch)

<a id="value-6d696e2d6f6666736574"></a>

## `min-offset`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">min-offset</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Offset</span>.<span class="tok-constructor">Offset</span></code></pre>

-25:59:59 and +25:59:59.

[Source](/reference-sources/Data.Time.Offset.cinch) · line 79

<a id="value-6f66667365742d636d70"></a>

## `offset-cmp`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">offset-cmp</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Offset</span>.<span class="tok-constructor">Offset</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Offset</span>.<span class="tok-constructor">Offset</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Ordering</span>.<span class="tok-constructor">Ordering</span>))</code></pre>

Order: the seconds' order (further east is greater).

[Source](/reference-sources/Data.Time.Offset.cinch) · line 189

<a id="value-6f66667365742d6461746574696d652d746f2d74696d657374616d70"></a>

## `offset-datetime-to-timestamp`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">offset-datetime-to-timestamp</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Offset</span>.<span class="tok-constructor">Offset</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span>)))</code></pre>

The instant whose civil reading at `o` is `x`: the civil line
position less the offset. `None` when that instant is outside the
timestamp range (only within 26 hours of the ends of the civil
range). The civil seconds are within +-3.8e11, so the subtraction
cannot wrap.

[Source](/reference-sources/Data.Time.Offset.cinch) · line 180

<a id="value-6f66667365742d66726f6d2d686d73"></a>

## `offset-from-hms`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">offset-from-hms</span> :: (<span class="tok-constructor">Bool</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Offset</span>.<span class="tok-constructor">Offset</span>)))))</code></pre>

Sign and magnitude components: `negative` then hours, minutes and
seconds. `None` unless hours is 0..=25 and minutes and seconds are
0..=59. The sum is at most 25 * 3600 + 59 * 60 + 59 = 93599.

[Source](/reference-sources/Data.Time.Offset.cinch) · line 104

<a id="value-6f66667365742d66726f6d2d7365636f6e6473"></a>

## `offset-from-seconds`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">offset-from-seconds</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Offset</span>.<span class="tok-constructor">Offset</span>))</code></pre>

`None` outside -93599..=93599.

[Source](/reference-sources/Data.Time.Offset.cinch) · line 86

<a id="value-6f66667365742d686f757273"></a>

## `offset-hours`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">offset-hours</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Offset</span>.<span class="tok-constructor">Offset</span> -&gt; (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">0</span> <span class="tok-number">25</span> <span class="tok-constructor">I64</span>))</code></pre>

The magnitude's whole hours, minutes of the hour and seconds of
the minute. Each clamp below never fires (the magnitude is at most
93599); it exists because machine arithmetic is opaque to the
refinement checker.

[Source](/reference-sources/Data.Time.Offset.cinch) · line 141

<a id="value-6f66667365742d69732d6e65676174697665"></a>

## `offset-is-negative`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">offset-is-negative</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Offset</span>.<span class="tok-constructor">Offset</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

West of UTC.

[Source](/reference-sources/Data.Time.Offset.cinch) · line 125

<a id="value-6f66667365742d69732d757463"></a>

## `offset-is-utc`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">offset-is-utc</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Offset</span>.<span class="tok-constructor">Offset</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

[Source](/reference-sources/Data.Time.Offset.cinch)

<a id="value-6f66667365742d6d696e75746573"></a>

## `offset-minutes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">offset-minutes</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Offset</span>.<span class="tok-constructor">Offset</span> -&gt; (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">0</span> <span class="tok-number">59</span> <span class="tok-constructor">I64</span>))</code></pre>

[Source](/reference-sources/Data.Time.Offset.cinch)

<a id="value-6f66667365742d6e6567617465"></a>

## `offset-negate`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">offset-negate</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Offset</span>.<span class="tok-constructor">Offset</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Offset</span>.<span class="tok-constructor">Offset</span>)</code></pre>

The opposite offset. The range is symmetric, so this is total and
the negation cannot wrap.

[Source](/reference-sources/Data.Time.Offset.cinch) · line 121

<a id="value-6f66667365742d6f662d686f757273"></a>

## `offset-of-hours`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">offset-of-hours</span> :: ((<span class="tok-variable">h</span> :: (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">-25</span> <span class="tok-number">25</span> <span class="tok-constructor">I64</span>)) -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Offset</span>.<span class="tok-constructor">Offset</span>)</code></pre>

Whole hours. 25 * 3600 = 90000, inside the range, so the wrapping
product is exact and in range.

[Source](/reference-sources/Data.Time.Offset.cinch) · line 98

<a id="value-6f66667365742d6f662d7365636f6e6473"></a>

## `offset-of-seconds`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">offset-of-seconds</span> :: ((<span class="tok-variable">s</span> :: (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">-93599</span> <span class="tok-number">93599</span> <span class="tok-constructor">I64</span>)) -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Offset</span>.<span class="tok-constructor">Offset</span>)</code></pre>

Total over the refined range, so a literal needs no `Option`.

[Source](/reference-sources/Data.Time.Offset.cinch) · line 93

<a id="value-6f66667365742d7365636f6e6473"></a>

## `offset-seconds`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">offset-seconds</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Offset</span>.<span class="tok-constructor">Offset</span> -&gt; (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">0</span> <span class="tok-number">59</span> <span class="tok-constructor">I64</span>))</code></pre>

[Source](/reference-sources/Data.Time.Offset.cinch)

<a id="value-6f66667365742d74696d657374616d702d746f2d6461746574696d65"></a>

## `offset-timestamp-to-datetime`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">offset-timestamp-to-datetime</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Offset</span>.<span class="tok-constructor">Offset</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span>))</code></pre>

The civil reading of `t` at `o`. The Unix seconds are within
+-3.8e11 and the offset within +-93599, so the sum cannot wrap.
The timestamp range (T3) is the civil range pulled in by 93599
seconds at each end, so the sum always names a second of a year in
-9999..=9999 and the `None` arm is unreachable; if it were ever
taken (a change to either range), the answer is the nearest end of
the civil range rather than a fault.

[Source](/reference-sources/Data.Time.Offset.cinch) · line 167

<a id="value-6f66667365742d746f2d7365636f6e6473"></a>

## `offset-to-seconds`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">offset-to-seconds</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Offset</span>.<span class="tok-constructor">Offset</span> -&gt; <span class="tok-constructor">I64</span>)</code></pre>

Reading.

[Source](/reference-sources/Data.Time.Offset.cinch) · line 115

<a id="value-6f66667365742d757463"></a>

## `offset-utc`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">offset-utc</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Offset</span>.<span class="tok-constructor">Offset</span></code></pre>

UTC itself: zero seconds.

[Source](/reference-sources/Data.Time.Offset.cinch) · line 75

# Instances

These are checked exported instance entries. Law/sample-check status is separate from the existence of an instance; see [laws and optimization](../reference/12-rewrites-and-optimization.md).

| Instance | Target | Resolution |
| --- | --- | --- |
| `Data.Time.Offset.Eq_Offset` | `Relation.Eq.Eq Data.Time.Offset.Offset` | Implicit |
| `Data.Time.Offset.Ord_Offset` | `Relation.Ord.Ord Data.Time.Offset.Offset` | Implicit |

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Data/Time/Offset.cinch`. Source SHA-256: `74af282609866c44d75df7392c48eb71cafd85f5e1ee6edb2af30bb4a109cc2c`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
