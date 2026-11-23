# Data.Time.Timestamp

Instants on the Unix time line with nanosecond precision and checked arithmetic.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Data.Time.Timestamp`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Time.Timestamp.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Machine](Data.Machine.md), [Data.Option](Data.Option.md), [Data.Ordering](Data.Ordering.md), [Data.Time.Duration](Data.Time.Duration.md), [Data.Wide](Data.Wide.md), [Relation.Eq](Relation.Eq.md), [Relation.Ord](Relation.Ord.md).

Non-embedded source closure: [Data.Time.Duration](Data.Time.Duration.md), [Data.Time.Timestamp](Data.Time.Timestamp.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Data.Time.Timestamp: an instant on the Unix time line
(design/datetime.md, layer L0 "machine time", lane A).

A `Timestamp` answers "when did that happen?": a count of
nanoseconds since 1970-01-01T00:00:00Z, with no leap seconds (Unix
time, as in Jiff and Temporal). It has no calendar fields and no
zone. Showing one to a person needs a `TimeZone`
(`Data.Time.Offset`, `Data.Time.Zone`), and that conversion is the
only road from machine time to civil time. The representation
constructor is not exported, so an out-of-range instant has no
representation.

REPRESENTATION. One `I128` nanosecond count behind the private
constructor `MkTimestamp` (the same shape and the same one-allocation
wrapper cost as `Data.Time.Duration`; see its header).

RANGE (T3). `-9999-01-02T01:59:59Z ..= 9999-12-30T22:00:00.999999999Z`
(`min-timestamp`, `max-timestamp`), Jiff's bounds. Derivation:
  - civil years are -9999..=9999 and offsets are within 25:59:59 of
    UTC (93599 seconds), so the range is the civil range pulled in
    by one maximal offset at each end; then every timestamp has a
    civil reading in every offset.
  - -9999-01-01 is epoch day -4371587: 0001-01-01 is day -719162,
    and the 10000 years -9999..=0 are 25 Gregorian cycles of 146097
    days (3652425 days) before it. In seconds, -377705116800; plus
    93599 is -377705023201; in nanoseconds
    -377705023201000000000.
  - 9999-12-31 is epoch day 2932896, so 9999-12-31T23:59:59 is
    253402300799 seconds; less 93599 is 253402207200
    (9999-12-30T22:00:00); the last nanosecond of that second is
    253402207200999999999.
Both bounds are past `I64` nanoseconds (about +-9.22e18), which is
why the representation is wide and why `timestamp-from-unix-nanos`
on an `I64` is total.

Design law shape. The nanosecond count is the value: equality and
`timestamp-cmp` are the integer order, which is the order of the
instants. The Unix conversions floor: `to-unix-seconds` is the
greatest whole second not after the instant and `subsec-nanos` is
the rest, always `0..=999999999`, so 1 ns before the epoch is second
-1 with 999999999 ns, and `from-unix-seconds-and-nanos` inverts
`to-unix-seconds-and-nanos` exactly. `timestamp-since a b` is
`a - b` and `timestamp-until a b` is `b - a`, so each is the other's
negation; `checked-add t (since u t)` is `u`; `checked-add` and
`checked-sub` of the same duration undo each other whenever both
exist. A `checked-` result is `Some` exactly when the true result is
in the range; the `saturating-` twin clamps to the bounds. The
checks are seed/tests/lib_time_a.rs.

NAMED REFUSALS (boundaries this module deliberately does not
cross):
  - instants outside the range above: construction and arithmetic
    answer `None` (or clamp, in the `saturating-` spellings).
  - leap seconds: not modelled (Unix time).
  - calendar fields, formatting and parsing: `Data.Time.Offset`,
    `Data.Time.Zone` and `Text.Time.Iso` own those.
  - reading the clock: `Effect.Time.now`.
  - operator instances for arithmetic (T2); only `Eq` and `Ord`.
  - `Int`/`Nat` (bignums): neither appears.
  - an allocation-free wrapper: as `Data.Time.Duration`.
  - a refined component in the pair `to-unix-seconds-and-nanos`
    returns: a tuple whose component type is refined accepts only a
    literal there ("type mismatch: expected I64, got InRange 0
    999999999 I64" for any computed value), so the pair is
    `(I64, I64)` and the bound is carried by `timestamp-subsec-nanos`.

Naming: every export is prefixed `timestamp-` (the bounds are
`min-timestamp`/`max-timestamp`); see `Data.Time.Duration`'s header.

</details>

# Types and constructors

<a id="type-54696d657374616d70"></a>

## `Timestamp`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Timestamp</span></code></pre>

Nanoseconds since the Unix epoch. Not exported (the range
invariant above); named apart from the type so that
`export Timestamp` does not export it.

[Source](/reference-sources/Data.Time.Timestamp.cinch) · line 98

# Values

<a id="value-6d61782d74696d657374616d70"></a>

## `max-timestamp`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">max-timestamp</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span></code></pre>

9999-12-30T22:00:00.999999999Z.

[Source](/reference-sources/Data.Time.Timestamp.cinch) · line 164

<a id="value-6d696e2d74696d657374616d70"></a>

## `min-timestamp`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">min-timestamp</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span></code></pre>

-9999-01-02T01:59:59Z.

[Source](/reference-sources/Data.Time.Timestamp.cinch) · line 160

<a id="value-74696d657374616d702d636865636b65642d616464"></a>

## `timestamp-checked-add`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">timestamp-checked-add</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">Duration</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span>)))</code></pre>

The instant `d` after `t` (before it for a negative `d`).

[Source](/reference-sources/Data.Time.Timestamp.cinch) · line 241

<a id="value-74696d657374616d702d636865636b65642d737562"></a>

## `timestamp-checked-sub`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">timestamp-checked-sub</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">Duration</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span>)))</code></pre>

The instant `d` before `t`.

[Source](/reference-sources/Data.Time.Timestamp.cinch) · line 247

<a id="value-74696d657374616d702d636d70"></a>

## `timestamp-cmp`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">timestamp-cmp</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Ordering</span>.<span class="tok-constructor">Ordering</span>))</code></pre>

Order.

[Source](/reference-sources/Data.Time.Timestamp.cinch) · line 274

<a id="value-74696d657374616d702d65706f6368"></a>

## `timestamp-epoch`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">timestamp-epoch</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span></code></pre>

1970-01-01T00:00:00Z.

[Source](/reference-sources/Data.Time.Timestamp.cinch) · line 156

<a id="value-74696d657374616d702d66726f6d2d756e69782d6d6963726f73"></a>

## `timestamp-from-unix-micros`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">timestamp-from-unix-micros</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span>))</code></pre>

[Source](/reference-sources/Data.Time.Timestamp.cinch)

<a id="value-74696d657374616d702d66726f6d2d756e69782d6d696c6c6973"></a>

## `timestamp-from-unix-millis`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">timestamp-from-unix-millis</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span>))</code></pre>

[Source](/reference-sources/Data.Time.Timestamp.cinch)

<a id="value-74696d657374616d702d66726f6d2d756e69782d6e616e6f73"></a>

## `timestamp-from-unix-nanos`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">timestamp-from-unix-nanos</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span>)</code></pre>

Total: every `I64` nanosecond count (1677-09-21 to 2262-04-11) is
inside the range, whose bounds are past `I64` on both sides.

[Source](/reference-sources/Data.Time.Timestamp.cinch) · line 179

<a id="value-74696d657374616d702d66726f6d2d756e69782d6e616e6f732d69313238"></a>

## `timestamp-from-unix-nanos-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">timestamp-from-unix-nanos-i128</span> :: (<span class="tok-constructor">I128</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span>))</code></pre>

[Source](/reference-sources/Data.Time.Timestamp.cinch)

<a id="value-74696d657374616d702d66726f6d2d756e69782d7365636f6e6473"></a>

## `timestamp-from-unix-seconds`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">timestamp-from-unix-seconds</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span>))</code></pre>

`None` outside -377705023201..=253402207200.

[Source](/reference-sources/Data.Time.Timestamp.cinch) · line 168

<a id="value-74696d657374616d702d66726f6d2d756e69782d7365636f6e64732d616e642d6e616e6f73"></a>

## `timestamp-from-unix-seconds-and-nanos`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">timestamp-from-unix-seconds-and-nanos</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span>)))</code></pre>

`secs` whole seconds plus `nanos` nanoseconds. The two are added,
so any `I64` nanosecond part is accepted and normalised (the pair
`(-1, 999999999)` is 1 ns before the epoch, the pair
`to-unix-seconds-and-nanos` gives for it). The wrapping sum is
exact: the seconds term is below 2^93 and the other below 2^63.

[Source](/reference-sources/Data.Time.Timestamp.cinch) · line 190

<a id="value-74696d657374616d702d73617475726174696e672d616464"></a>

## `timestamp-saturating-add`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">timestamp-saturating-add</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">Duration</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span>))</code></pre>

The same, clamped to `min-timestamp`/`max-timestamp`.

[Source](/reference-sources/Data.Time.Timestamp.cinch) · line 253

<a id="value-74696d657374616d702d73617475726174696e672d737562"></a>

## `timestamp-saturating-sub`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">timestamp-saturating-sub</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">Duration</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span>))</code></pre>

[Source](/reference-sources/Data.Time.Timestamp.cinch)

<a id="value-74696d657374616d702d73696e6365"></a>

## `timestamp-since`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">timestamp-since</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">Duration</span>))</code></pre>

`timestamp-since a b` is `a - b`: the time from `b` to `a`,
positive when `a` is the later. Total: two counts in the range
differ by at most 631107230401999999999 (below 2^70), so the
wrapping subtraction is exact and far inside a duration's range.

[Source](/reference-sources/Data.Time.Timestamp.cinch) · line 263

<a id="value-74696d657374616d702d7375627365632d6e616e6f73"></a>

## `timestamp-subsec-nanos`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">timestamp-subsec-nanos</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span> -&gt; (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">0</span> <span class="tok-number">999999999</span> <span class="tok-constructor">I64</span>))</code></pre>

The nanoseconds past the floored second: `0..=999999999`. The
value is `n - floor(n / 10^9) * 10^9`, which is in that interval,
so both wrapping steps are exact; the guard restates the bound for
the checker (a wide scalar has no solver sort) and its `else` arm
is unreachable.

[Source](/reference-sources/Data.Time.Timestamp.cinch) · line 222

<a id="value-74696d657374616d702d746f2d756e69782d6d6963726f73"></a>

## `timestamp-to-unix-micros`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">timestamp-to-unix-micros</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span> -&gt; <span class="tok-constructor">I64</span>)</code></pre>

[Source](/reference-sources/Data.Time.Timestamp.cinch)

<a id="value-74696d657374616d702d746f2d756e69782d6d696c6c6973"></a>

## `timestamp-to-unix-millis`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">timestamp-to-unix-millis</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span> -&gt; <span class="tok-constructor">I64</span>)</code></pre>

[Source](/reference-sources/Data.Time.Timestamp.cinch)

<a id="value-74696d657374616d702d746f2d756e69782d6e616e6f73"></a>

## `timestamp-to-unix-nanos`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">timestamp-to-unix-nanos</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span> -&gt; <span class="tok-constructor">I128</span>)</code></pre>

The whole count, exactly.

[Source](/reference-sources/Data.Time.Timestamp.cinch) · line 209

<a id="value-74696d657374616d702d746f2d756e69782d6e616e6f732d693634"></a>

## `timestamp-to-unix-nanos-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">timestamp-to-unix-nanos-i64</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">I64</span>))</code></pre>

`None` outside `I64` nanoseconds (before 1677-09-21T00:12:43.145224192Z
or after 2262-04-11T23:47:16.854775807Z).

[Source](/reference-sources/Data.Time.Timestamp.cinch) · line 214

<a id="value-74696d657374616d702d746f2d756e69782d7365636f6e6473"></a>

## `timestamp-to-unix-seconds`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">timestamp-to-unix-seconds</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span> -&gt; <span class="tok-constructor">I64</span>)</code></pre>

Observation. Every conversion to a coarser unit floors (rounds
toward the past), before the epoch as after it.

[Source](/reference-sources/Data.Time.Timestamp.cinch) · line 199

<a id="value-74696d657374616d702d746f2d756e69782d7365636f6e64732d616e642d6e616e6f73"></a>

## `timestamp-to-unix-seconds-and-nanos`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">timestamp-to-unix-seconds-and-nanos</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span> -&gt; (<span class="tok-constructor">I64</span>, <span class="tok-constructor">I64</span>))</code></pre>

The floored second and the nanoseconds past it (`0..=999999999`),
together: what the zone layer splits an instant into. The second
component is a plain `I64` here (named refusal in the header);
`timestamp-subsec-nanos` is the same number with its bound in the
type.

[Source](/reference-sources/Data.Time.Timestamp.cinch) · line 233

<a id="value-74696d657374616d702d756e74696c"></a>

## `timestamp-until`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">timestamp-until</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">Duration</span>))</code></pre>

`timestamp-until a b` is `b - a`: the time from `a` to `b`.

[Source](/reference-sources/Data.Time.Timestamp.cinch) · line 267

# Instances

These are checked exported instance entries. Law/sample-check status is separate from the existence of an instance; see [laws and optimization](../reference/12-rewrites-and-optimization.md).

| Instance | Target | Resolution |
| --- | --- | --- |
| `Data.Time.Timestamp.Eq_Timestamp` | `Relation.Eq.Eq Data.Time.Timestamp.Timestamp` | Implicit |
| `Data.Time.Timestamp.Ord_Timestamp` | `Relation.Ord.Ord Data.Time.Timestamp.Timestamp` | Implicit |

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Data/Time/Timestamp.cinch`. Source SHA-256: `c4ac068fadce1317eb55fe3e867e7c85e2e98449390bd95c01eb7c7982380614`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
