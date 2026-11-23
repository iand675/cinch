# Data.Time.Unit

Span units, rounding modes and the shared rounding kernel.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Data.Time.Unit`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Time.Unit.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Algebra.Arithmetic](Algebra.Arithmetic.md), [Data.Machine](Data.Machine.md), [Data.Option](Data.Option.md), [Data.Ordering](Data.Ordering.md), [Data.Wide](Data.Wide.md), [Relation.Eq](Relation.Eq.md), [Relation.Ord](Relation.Ord.md).

Non-embedded source closure: [Data.Time.Unit](Data.Time.Unit.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Data.Time.Unit: the units of a span, the rounding modes and the
rounding kernel.

`TimeUnit` names the ten units a `Span` keeps (design/datetime.md
L3, where it is called `Unit`; see NAMED REFUSALS), ordered from
`Nanosecond` (smallest) to `Year` (largest). `RoundMode`
is TC39 Temporal's nine rounding modes. `round-i128` is the one
rounding kernel every rounding operation in the family goes through:
an exact integer rounding of a signed `I128` quantity to a multiple
of a positive `I128` increment. No float appears anywhere.

Design law shape.
  - `unit-rank` is strictly increasing from `Nanosecond` (0) to
    `Year` (9); `unit-cmp`, `unit-larger-of` and `unit-smaller-of`
    are that order. `unit-next-larger` and `unit-next-smaller` are
    inverse steps on it and are `None` at the two ends.
  - Units split in two. `Hour` and smaller have a fixed length:
    `unit-nanos` is `Some` of it. `Day` and larger are calendar units
    (`unit-is-calendar`): their length depends on a date (and, for a
    day, on a zone), so `unit-nanos` is `None`. `nanos-per-civil-day` is
    the civil 24-hour day, a named constant for the places where
    Temporal itself treats a day as 24 hours (civil date-times, which
    have no zone).
  - `round-i128 mode inc x` is Temporal's RoundNumberToIncrement: the
    result is a multiple of `inc`, it equals `x` when `x` already is
    one, and otherwise it is one of the two multiples of `inc` that
    bracket `x`, chosen by the mode:
      Ceil       toward positive infinity
      Floor      toward negative infinity
      Expand     away from zero
      Trunc      toward zero
      HalfCeil   to the nearer; a tie toward positive infinity
      HalfFloor  to the nearer; a tie toward negative infinity
      HalfExpand to the nearer; a tie away from zero
      HalfTrunc  to the nearer; a tie toward zero
      HalfEven   to the nearer; a tie to the even multiple
    so `round-i128 mode inc (neg x)` is
    `neg (round-i128 (round-mode-negate mode) inc x)`.
  - `round-i128-as-if-positive` is Temporal's
    RoundNumberToIncrementAsIfPositive (used for instants, whose sign
    is an accident of the epoch): `Expand` behaves as `Ceil`, `Trunc`
    as `Floor`, `HalfExpand` as `HalfCeil`, `HalfTrunc` as
    `HalfFloor`, for either sign of `x`.
  - The three increment validators are Temporal's
    ValidateTemporalRoundingIncrement with the three maxima it uses
    (see each function).

NAMED REFUSALS:
  - the type name `Unit`: the compiler answers "Unit is a builtin
    primitive type and cannot be redefined", so the type is
    `TimeUnit`. The module keeps the spec's name.
  - the name `nanos-per-day-24h` (and any `-24h` spelling): a hyphen
    followed by a digit does not lex inside an identifier, so the
    24-hour day is `nanos-per-civil-day`.
  - a refined increment: a wide scalar has no solver sort, so
    "increment > 0" cannot be an `I128` parameter's type. `round-i128`
    answers `None` for an increment below 1 (and for the one input
    whose magnitude does not exist, `I128`'s minimum, and for a result
    that leaves `I128`). Callers in this family pass increments they
    validated, and say so.
  - a float kernel: not offered; rounding is exact integer work.

</details>

# Types and constructors

<a id="type-526f756e644d6f6465"></a>

## `RoundMode`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">RoundMode</span></code></pre>

Rounding modes.

[Source](/reference-sources/Data.Time.Unit.cinch) · line 187

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Ceil</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Unit</span>.<span class="tok-constructor">RoundMode</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Floor</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Unit</span>.<span class="tok-constructor">RoundMode</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Expand</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Unit</span>.<span class="tok-constructor">RoundMode</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Trunc</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Unit</span>.<span class="tok-constructor">RoundMode</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">HalfCeil</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Unit</span>.<span class="tok-constructor">RoundMode</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">HalfFloor</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Unit</span>.<span class="tok-constructor">RoundMode</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">HalfExpand</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Unit</span>.<span class="tok-constructor">RoundMode</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">HalfTrunc</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Unit</span>.<span class="tok-constructor">RoundMode</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">HalfEven</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Unit</span>.<span class="tok-constructor">RoundMode</span></code></pre>

<a id="type-54696d65556e6974"></a>

## `TimeUnit`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">TimeUnit</span></code></pre>

[Source](/reference-sources/Data.Time.Unit.cinch)

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Nanosecond</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Unit</span>.<span class="tok-constructor">TimeUnit</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Microsecond</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Unit</span>.<span class="tok-constructor">TimeUnit</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Millisecond</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Unit</span>.<span class="tok-constructor">TimeUnit</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Second</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Unit</span>.<span class="tok-constructor">TimeUnit</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Minute</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Unit</span>.<span class="tok-constructor">TimeUnit</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Hour</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Unit</span>.<span class="tok-constructor">TimeUnit</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Day</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Unit</span>.<span class="tok-constructor">TimeUnit</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Week</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Unit</span>.<span class="tok-constructor">TimeUnit</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Month</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Unit</span>.<span class="tok-constructor">TimeUnit</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Year</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Unit</span>.<span class="tok-constructor">TimeUnit</span></code></pre>

# Signatures and methods

<a id="class-4c6172676573744f7074696f6e"></a>

## `LargestOption`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">signature</span> <span class="tok-constructor">LargestOption</span> <span class="tok-variable">o</span></code></pre>

Options that also choose the largest unit of a result.

[Source](/reference-sources/Data.Time.Unit.cinch) · line 380

<a id="method-4c6172676573744f7074696f6e2e776974682d6c617267657374"></a>

## `LargestOption.with-largest`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">with-largest</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">o</span> :: <span class="tok-constructor">Type</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Unit</span>.<span class="tok-constructor">LargestOption</span> <span class="tok-variable">o</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Unit</span>.<span class="tok-constructor">TimeUnit</span> -&gt; (<span class="tok-variable">o</span> -&gt; <span class="tok-variable">o</span>))</code></pre>



<a id="class-526f756e64696e674f7074696f6e73"></a>

## `RoundingOptions`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">signature</span> <span class="tok-constructor">RoundingOptions</span> <span class="tok-variable">o</span></code></pre>

Options with a smallest unit, an increment and a mode.

[Source](/reference-sources/Data.Time.Unit.cinch) · line 374

<a id="method-526f756e64696e674f7074696f6e732e776974682d736d616c6c657374"></a>

## `RoundingOptions.with-smallest`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">with-smallest</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">o</span> :: <span class="tok-constructor">Type</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Unit</span>.<span class="tok-constructor">RoundingOptions</span> <span class="tok-variable">o</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Unit</span>.<span class="tok-constructor">TimeUnit</span> -&gt; (<span class="tok-variable">o</span> -&gt; <span class="tok-variable">o</span>))</code></pre>



<a id="method-526f756e64696e674f7074696f6e732e776974682d696e6372656d656e74"></a>

## `RoundingOptions.with-increment`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">with-increment</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">o</span> :: <span class="tok-constructor">Type</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Unit</span>.<span class="tok-constructor">RoundingOptions</span> <span class="tok-variable">o</span>) =&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-variable">o</span> -&gt; <span class="tok-variable">o</span>))</code></pre>



<a id="method-526f756e64696e674f7074696f6e732e776974682d6d6f6465"></a>

## `RoundingOptions.with-mode`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">with-mode</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">o</span> :: <span class="tok-constructor">Type</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Unit</span>.<span class="tok-constructor">RoundingOptions</span> <span class="tok-variable">o</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Unit</span>.<span class="tok-constructor">RoundMode</span> -&gt; (<span class="tok-variable">o</span> -&gt; <span class="tok-variable">o</span>))</code></pre>



# Values

<a id="value-6e616e6f732d7065722d636976696c2d646179"></a>

## `nanos-per-civil-day`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">nanos-per-civil-day</span> :: <span class="tok-constructor">I128</span></code></pre>

A civil day of exactly 24 hours, in nanoseconds. Used only where
Temporal says a day is 24 hours: civil date-time arithmetic (no
zone, so no transition can change a day's length) and the
conventions whose names say `days-24h`.

[Source](/reference-sources/Data.Time.Unit.cinch) · line 154

<a id="value-726f756e642d69313238"></a>

## `round-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">round-i128</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Unit</span>.<span class="tok-constructor">RoundMode</span> -&gt; (<span class="tok-constructor">I128</span> -&gt; (<span class="tok-constructor">I128</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">I128</span>))))</code></pre>

The kernel: `x` rounded to a multiple of `inc` under `mode`
(Temporal's RoundNumberToIncrement; the header gives the law).
`None` when `inc < 1`, when `x` is `I128`'s minimum (its magnitude
does not exist) or when the rounded value leaves `I128`.

The work is on the magnitude `a = |x| = q * inc + r`. `a` is at
most 2^127 - 1 and `inc >= 1`, so the division is exact machine
division with no overflow; `q + 1` cannot overflow because
`q <= a`, and it is only formed when `r > 0`, where `q < a`. The
final product is checked.

[Source](/reference-sources/Data.Time.Unit.cinch) · line 259

<a id="value-726f756e642d693132382d61732d69662d706f736974697665"></a>

## `round-i128-as-if-positive`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">round-i128-as-if-positive</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Unit</span>.<span class="tok-constructor">RoundMode</span> -&gt; (<span class="tok-constructor">I128</span> -&gt; (<span class="tok-constructor">I128</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">I128</span>))))</code></pre>

Temporal's RoundNumberToIncrementAsIfPositive: the direction-free
modes are read on the number line rather than relative to zero, so
an instant before the epoch rounds the same way as one after it.

[Source](/reference-sources/Data.Time.Unit.cinch) · line 282

<a id="value-726f756e642d6d6f64652d6e6567617465"></a>

## `round-mode-negate`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">round-mode-negate</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Unit</span>.<span class="tok-constructor">RoundMode</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Unit</span>.<span class="tok-constructor">RoundMode</span>)</code></pre>

Temporal's NegateRoundingMode: the mode that, applied to a negated
quantity, gives the negated result. Only the four modes that name a
direction on the number line change.

[Source](/reference-sources/Data.Time.Unit.cinch) · line 201

<a id="value-756e69742d636d70"></a>

## `unit-cmp`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unit-cmp</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Unit</span>.<span class="tok-constructor">TimeUnit</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Unit</span>.<span class="tok-constructor">TimeUnit</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Ordering</span>.<span class="tok-constructor">Ordering</span>))</code></pre>

[Source](/reference-sources/Data.Time.Unit.cinch)

<a id="value-756e69742d6571"></a>

## `unit-eq`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unit-eq</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Unit</span>.<span class="tok-constructor">TimeUnit</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Unit</span>.<span class="tok-constructor">TimeUnit</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Time.Unit.cinch)

<a id="value-756e69742d69732d63616c656e646172"></a>

## `unit-is-calendar`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unit-is-calendar</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Unit</span>.<span class="tok-constructor">TimeUnit</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Day and larger: units whose length is not a fixed number of
nanoseconds.

[Source](/reference-sources/Data.Time.Unit.cinch) · line 132

<a id="value-756e69742d6c61726765722d6f66"></a>

## `unit-larger-of`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unit-larger-of</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Unit</span>.<span class="tok-constructor">TimeUnit</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Unit</span>.<span class="tok-constructor">TimeUnit</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Unit</span>.<span class="tok-constructor">TimeUnit</span>))</code></pre>

[Source](/reference-sources/Data.Time.Unit.cinch)

<a id="value-756e69742d6e616e6f73"></a>

## `unit-nanos`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unit-nanos</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Unit</span>.<span class="tok-constructor">TimeUnit</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">I128</span>))</code></pre>

The length of a fixed-length unit in nanoseconds; `None` for `Day`
and larger.

[Source](/reference-sources/Data.Time.Unit.cinch) · line 137

<a id="value-756e69742d6e6578742d6c6172676572"></a>

## `unit-next-larger`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unit-next-larger</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Unit</span>.<span class="tok-constructor">TimeUnit</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Unit</span>.<span class="tok-constructor">TimeUnit</span>))</code></pre>

[Source](/reference-sources/Data.Time.Unit.cinch)

<a id="value-756e69742d6e6578742d736d616c6c6572"></a>

## `unit-next-smaller`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unit-next-smaller</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Unit</span>.<span class="tok-constructor">TimeUnit</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Unit</span>.<span class="tok-constructor">TimeUnit</span>))</code></pre>

[Source](/reference-sources/Data.Time.Unit.cinch)

<a id="value-756e69742d72616e6b"></a>

## `unit-rank`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unit-rank</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Unit</span>.<span class="tok-constructor">TimeUnit</span> -&gt; (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">0</span> <span class="tok-number">9</span> <span class="tok-constructor">I64</span>))</code></pre>

The order: Nanosecond is the smallest unit, Year the largest.

[Source](/reference-sources/Data.Time.Unit.cinch) · line 102

<a id="value-756e69742d726f756e642d696e6372656d656e742d6f6b"></a>

## `unit-round-increment-ok`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unit-round-increment-ok</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Unit</span>.<span class="tok-constructor">TimeUnit</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

For rounding a `TimeOfDay` or a `DateTime` (PlainTime.round,
PlainDateTime.round): the same maxima for hour and smaller; a day
increment must be exactly 1; week and larger are not rounding units
of a value.

[Source](/reference-sources/Data.Time.Unit.cinch) · line 333

<a id="value-756e69742d736d616c6c65722d6f66"></a>

## `unit-smaller-of`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unit-smaller-of</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Unit</span>.<span class="tok-constructor">TimeUnit</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Unit</span>.<span class="tok-constructor">TimeUnit</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Unit</span>.<span class="tok-constructor">TimeUnit</span>))</code></pre>

[Source](/reference-sources/Data.Time.Unit.cinch)

<a id="value-756e69742d7370616e2d696e6372656d656e742d6f6b"></a>

## `unit-span-increment-ok`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unit-span-increment-ok</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Unit</span>.<span class="tok-constructor">TimeUnit</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

For rounding a span and for `until`/`since`
(MaximumTemporalDurationRoundingIncrement, exclusive): hours must
divide 24 and be below it, minutes and seconds 60, milliseconds,
microseconds and nanoseconds 1000. Day and larger have no maximum:
any increment in 1..=1000000000.

[Source](/reference-sources/Data.Time.Unit.cinch) · line 316

<a id="value-756e69742d74696d657374616d702d696e6372656d656e742d6f6b"></a>

## `unit-timestamp-increment-ok`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unit-timestamp-increment-ok</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Unit</span>.<span class="tok-constructor">TimeUnit</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

For rounding a `Timestamp` (Instant.round): the increment, in its
unit, must divide one 24-hour day evenly, and may equal it
(inclusive): hours divide 24, minutes 1440, seconds 86400, and so
on, within the general bound of 1000000000. Day and larger are
refused.

[Source](/reference-sources/Data.Time.Unit.cinch) · line 351

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Data/Time/Unit.cinch`. Source SHA-256: `f2dfb80db9ec3788b0d698bb055050c8842b44d14bf47ee8a7c72762175cc8ab`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
