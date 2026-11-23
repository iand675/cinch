# Data.Time.Duration

Exact signed elapsed-time durations with nanosecond precision and checked arithmetic.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Data.Time.Duration`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Time.Duration.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Algebra.Arithmetic](Algebra.Arithmetic.md), [Data.Machine](Data.Machine.md), [Data.Option](Data.Option.md), [Data.Ordering](Data.Ordering.md), [Data.Wide](Data.Wide.md), [Relation.Eq](Relation.Eq.md), [Relation.Ord](Relation.Ord.md).

Non-embedded source closure: [Data.Time.Duration](Data.Time.Duration.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Data.Time.Duration: an exact signed length of elapsed time
(design/datetime.md, layer L0 "machine time", lane A).

A `Duration` is a signed count of nanoseconds. It knows nothing of
calendars, zones or clocks: it is what `Monotonic - Monotonic` and
`Timestamp - Timestamp` are, and what a `Timestamp` moves by. The
representation constructor is not exported; the functions below are
the only way in, so the range invariant cannot be broken.

REPRESENTATION. One `I128` nanosecond count (`Data.Wide`) behind the
private constructor `MkDuration`. The scalar itself is unboxed (an
LLVM `i128` in locals, parameters and the constructor's payload),
so every operation is `i128` arithmetic and a compare is one `i128`
compare. The WRAPPER is not free today: a single-constructor `data`
is a 16-byte heap object (`cinch_alloc_ctor(16, 0)` in the emitted
IR), so each operation that returns a `Duration` allocates once.
See the named refusals.

RANGE. `-(2^127 - 1) ..= 2^127 - 1` nanoseconds (`min-duration`,
`max-duration`): the whole `I128` except its minimum. Dropping that
one value makes the range symmetric, which is what makes
`duration-neg`, `duration-abs` and both divisions total (the only
`I128` negation or division that overflows is the minimum's), at the
price of one refusal in `duration-from-i128-nanos`. About 5.39e21
years either way, so every unit constructor from an `I64` is total:
`|n| <= 2^63` and the largest unit is 3.6e12 < 2^42 nanoseconds, so
the product is below 2^105.

Design law shape. The nanosecond count is the value: two durations
are equal exactly when their counts are, and `duration-cmp` is the
integer order. `duration-zero` is the identity of `checked-add`;
`duration-neg` is an involution and `checked-add d (neg d)` is zero;
`checked-sub a b` is `checked-add a (neg b)`; `duration-abs` is
never negative. A `checked-` operation is `Some` of the exact
integer result when that lies in the range and `None` otherwise; the
`saturating-` twin is the exact result clamped to `min-duration` or
`max-duration`. Conversions to a coarser unit truncate toward zero
(`-1500 ms` is `-1 s`, as in Rust and Jiff), and
`whole-seconds * 10^9 + subsec-nanos` is the count again, so the
subsecond part carries the duration's sign. `duration-div` truncates
toward zero and `duration-div-floor` rounds toward negative
infinity. The checks are seed/tests/lib_time_a.rs.

NAMED REFUSALS (boundaries this module deliberately does not
cross):
  - operator instances for arithmetic (`+`, `-`, `*`): the name of
    each operation states its overflow policy (T2), as for the
    machine scalars; only `Eq` and `Ord` are instances.
  - an allocation-free wrapper: `#[repr(transparent)]` over an
    `I128` field is refused by code generation ("holds an `I128` or
    `U128` value in a boundary-layout (`repr(C)`) field, which code
    generation does not lower yet"), a transparent record is still a
    heap object in native code, and a `record` exported by type name
    can be built and projected by an importer, so it is not
    abstract. The private `data` constructor is the nearest design:
    abstract, one unboxed `i128` payload, one allocation per result.
  - `I128`'s minimum: not a duration (see RANGE).
  - calendar units (days and larger): a day is not a fixed number of
    nanoseconds once a zone is involved; that is `Data.Time.Span`.
  - floating-point seconds: no `F64` constructor or observer, since
    neither is exact.
  - `Int`/`Nat` (bignums): neither appears, in the representation or
    the API.
  - the millisecond `Effect.Clock.Duration`: the bridge lives in
    `Effect.Time` (`to-clock-duration`, `from-clock-duration`) so
    this module imports no effect vocabulary.

Naming. Every export is prefixed `duration-` (the bounds are
`min-duration`/`max-duration`, after `min-bound-i64`), because an
import clash is an error in Cinch and this module is imported beside
`Data.Time.Timestamp` and `Data.Time.Monotonic` unqualified.

Home (design/base-library.md): `Data.*` holds the concrete types, so
the `Eq`/`Ord` instances live with `Duration` here.

</details>

# Types and constructors

<a id="type-4475726174696f6e"></a>

## `Duration`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Duration</span></code></pre>

The nanosecond count. Not exported (the range invariant above).
The constructor's name differs from the type's on purpose:
`export Duration` exports the type AND a constructor of the same
name, so a constructor called `Duration` would be public.

[Source](/reference-sources/Data.Time.Duration.cinch) · line 110

# Values

<a id="value-6475726174696f6e2d616273"></a>

## `duration-abs`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">duration-abs</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">Duration</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">Duration</span>)</code></pre>

Total, by the same argument.

[Source](/reference-sources/Data.Time.Duration.cinch) · line 297

<a id="value-6475726174696f6e2d636865636b65642d616464"></a>

## `duration-checked-add`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">duration-checked-add</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">Duration</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">Duration</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">Duration</span>)))</code></pre>

[Source](/reference-sources/Data.Time.Duration.cinch)

<a id="value-6475726174696f6e2d636865636b65642d6d756c"></a>

## `duration-checked-mul`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">duration-checked-mul</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">Duration</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">Duration</span>)))</code></pre>

The duration `k` times over.

[Source](/reference-sources/Data.Time.Duration.cinch) · line 317

<a id="value-6475726174696f6e2d636865636b65642d737562"></a>

## `duration-checked-sub`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">duration-checked-sub</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">Duration</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">Duration</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">Duration</span>)))</code></pre>

[Source](/reference-sources/Data.Time.Duration.cinch)

<a id="value-6475726174696f6e2d636d70"></a>

## `duration-cmp`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">duration-cmp</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">Duration</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">Duration</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Ordering</span>.<span class="tok-constructor">Ordering</span>))</code></pre>

Order.

[Source](/reference-sources/Data.Time.Duration.cinch) · line 349

<a id="value-6475726174696f6e2d646976"></a>

## `duration-div`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">duration-div</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">Duration</span> -&gt; ((<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">NonZero</span> <span class="tok-constructor">I64</span>) -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">Duration</span>))</code></pre>

One `k`th of the duration, truncated toward zero. Total: the
divisor is not zero by its type and the quotient's magnitude never
exceeds the dividend's.

[Source](/reference-sources/Data.Time.Duration.cinch) · line 328

<a id="value-6475726174696f6e2d6469762d666c6f6f72"></a>

## `duration-div-floor`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">duration-div-floor</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">Duration</span> -&gt; ((<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">NonZero</span> <span class="tok-constructor">I64</span>) -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">Duration</span>))</code></pre>

One `k`th of the duration, rounded toward negative infinity: the
truncated quotient, less one when the division is inexact and the
exact quotient is negative (remainder and divisor of opposite
signs). The decrement cannot leave the range: an inexact division
has `|k| >= 2`, so the quotient's magnitude is at most 2^126.

[Source](/reference-sources/Data.Time.Duration.cinch) · line 336

<a id="value-6475726174696f6e2d66726f6d2d686f757273"></a>

## `duration-from-hours`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">duration-from-hours</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">Duration</span>)</code></pre>

[Source](/reference-sources/Data.Time.Duration.cinch)

<a id="value-6475726174696f6e2d66726f6d2d693132382d6e616e6f73"></a>

## `duration-from-i128-nanos`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">duration-from-i128-nanos</span> :: (<span class="tok-constructor">I128</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">Duration</span>))</code></pre>

A raw nanosecond count: `None` for `I128`'s minimum only.

[Source](/reference-sources/Data.Time.Duration.cinch) · line 204

<a id="value-6475726174696f6e2d66726f6d2d6d6963726f73"></a>

## `duration-from-micros`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">duration-from-micros</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">Duration</span>)</code></pre>

[Source](/reference-sources/Data.Time.Duration.cinch)

<a id="value-6475726174696f6e2d66726f6d2d6d696c6c6973"></a>

## `duration-from-millis`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">duration-from-millis</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">Duration</span>)</code></pre>

[Source](/reference-sources/Data.Time.Duration.cinch)

<a id="value-6475726174696f6e2d66726f6d2d6d696e75746573"></a>

## `duration-from-minutes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">duration-from-minutes</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">Duration</span>)</code></pre>

[Source](/reference-sources/Data.Time.Duration.cinch)

<a id="value-6475726174696f6e2d66726f6d2d6e616e6f73"></a>

## `duration-from-nanos`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">duration-from-nanos</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">Duration</span>)</code></pre>

The unit constructors are total: every `I64` count of every unit
is in the range (the header's RANGE argument).

[Source](/reference-sources/Data.Time.Duration.cinch) · line 177

<a id="value-6475726174696f6e2d66726f6d2d7365636f6e6473"></a>

## `duration-from-seconds`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">duration-from-seconds</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">Duration</span>)</code></pre>

[Source](/reference-sources/Data.Time.Duration.cinch)

<a id="value-6475726174696f6e2d66726f6d2d7365636f6e64732d616e642d6e616e6f73"></a>

## `duration-from-seconds-and-nanos`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">duration-from-seconds-and-nanos</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">Duration</span>))</code></pre>

`secs` seconds plus `nanos` nanoseconds, any signs (the two parts
are added, so `(-1, 999999999)` is -1 ns). Total: the seconds term
is below 2^93 and the nanosecond term below 2^63, so the wrapping
sum is exact.

[Source](/reference-sources/Data.Time.Duration.cinch) · line 199

<a id="value-6475726174696f6e2d69732d6e65676174697665"></a>

## `duration-is-negative`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">duration-is-negative</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">Duration</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

[Source](/reference-sources/Data.Time.Duration.cinch)

<a id="value-6475726174696f6e2d69732d706f736974697665"></a>

## `duration-is-positive`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">duration-is-positive</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">Duration</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

[Source](/reference-sources/Data.Time.Duration.cinch)

<a id="value-6475726174696f6e2d69732d7a65726f"></a>

## `duration-is-zero`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">duration-is-zero</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">Duration</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

[Source](/reference-sources/Data.Time.Duration.cinch)

<a id="value-6475726174696f6e2d6d61782d6f66"></a>

## `duration-max-of`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">duration-max-of</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">Duration</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">Duration</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">Duration</span>))</code></pre>

The longer (more positive) of two durations.

[Source](/reference-sources/Data.Time.Duration.cinch) · line 360

<a id="value-6475726174696f6e2d6d696e2d6f66"></a>

## `duration-min-of`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">duration-min-of</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">Duration</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">Duration</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">Duration</span>))</code></pre>

The shorter (more negative) of two durations.

[Source](/reference-sources/Data.Time.Duration.cinch) · line 356

<a id="value-6475726174696f6e2d6e6567"></a>

## `duration-neg`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">duration-neg</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">Duration</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">Duration</span>)</code></pre>

Total: the range is symmetric, so the wrapping negation is exact
and stays in the range.

[Source](/reference-sources/Data.Time.Duration.cinch) · line 293

<a id="value-6475726174696f6e2d73617475726174696e672d616464"></a>

## `duration-saturating-add`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">duration-saturating-add</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">Duration</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">Duration</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">Duration</span>))</code></pre>

[Source](/reference-sources/Data.Time.Duration.cinch)

<a id="value-6475726174696f6e2d73617475726174696e672d66726f6d2d693132382d6e616e6f73"></a>

## `duration-saturating-from-i128-nanos`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">duration-saturating-from-i128-nanos</span> :: (<span class="tok-constructor">I128</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">Duration</span>)</code></pre>

The same, with the minimum clamped to `min-duration`.

[Source](/reference-sources/Data.Time.Duration.cinch) · line 208

<a id="value-6475726174696f6e2d73617475726174696e672d6d756c"></a>

## `duration-saturating-mul`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">duration-saturating-mul</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">Duration</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">Duration</span>))</code></pre>

[Source](/reference-sources/Data.Time.Duration.cinch)

<a id="value-6475726174696f6e2d73617475726174696e672d737562"></a>

## `duration-saturating-sub`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">duration-saturating-sub</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">Duration</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">Duration</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">Duration</span>))</code></pre>

[Source](/reference-sources/Data.Time.Duration.cinch)

<a id="value-6475726174696f6e2d73617475726174696e672d746f2d686f757273"></a>

## `duration-saturating-to-hours`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">duration-saturating-to-hours</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">Duration</span> -&gt; <span class="tok-constructor">I64</span>)</code></pre>

[Source](/reference-sources/Data.Time.Duration.cinch)

<a id="value-6475726174696f6e2d73617475726174696e672d746f2d6d6963726f73"></a>

## `duration-saturating-to-micros`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">duration-saturating-to-micros</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">Duration</span> -&gt; <span class="tok-constructor">I64</span>)</code></pre>

[Source](/reference-sources/Data.Time.Duration.cinch)

<a id="value-6475726174696f6e2d73617475726174696e672d746f2d6d696c6c6973"></a>

## `duration-saturating-to-millis`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">duration-saturating-to-millis</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">Duration</span> -&gt; <span class="tok-constructor">I64</span>)</code></pre>

[Source](/reference-sources/Data.Time.Duration.cinch)

<a id="value-6475726174696f6e2d73617475726174696e672d746f2d6d696e75746573"></a>

## `duration-saturating-to-minutes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">duration-saturating-to-minutes</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">Duration</span> -&gt; <span class="tok-constructor">I64</span>)</code></pre>

[Source](/reference-sources/Data.Time.Duration.cinch)

<a id="value-6475726174696f6e2d73617475726174696e672d746f2d6e616e6f73"></a>

## `duration-saturating-to-nanos`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">duration-saturating-to-nanos</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">Duration</span> -&gt; <span class="tok-constructor">I64</span>)</code></pre>

[Source](/reference-sources/Data.Time.Duration.cinch)

<a id="value-6475726174696f6e2d73617475726174696e672d746f2d7365636f6e6473"></a>

## `duration-saturating-to-seconds`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">duration-saturating-to-seconds</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">Duration</span> -&gt; <span class="tok-constructor">I64</span>)</code></pre>

[Source](/reference-sources/Data.Time.Duration.cinch)

<a id="value-6475726174696f6e2d7375627365632d6e616e6f73"></a>

## `duration-subsec-nanos`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">duration-subsec-nanos</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">Duration</span> -&gt; (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">-999999999</span> <span class="tok-number">999999999</span> <span class="tok-constructor">I64</span>))</code></pre>

The part below one second, with the duration's sign:
`-999999999..=999999999`, and zero or the sign of the duration.
The remainder of an `I128` by 10^9 is in that interval, so the
wrapping narrowing is exact; the guard restates the bound for the
checker (a wide scalar has no solver sort) and its `else` arm is
unreachable.

[Source](/reference-sources/Data.Time.Duration.cinch) · line 225

<a id="value-6475726174696f6e2d746f2d686f757273"></a>

## `duration-to-hours`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">duration-to-hours</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">Duration</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">I64</span>))</code></pre>

[Source](/reference-sources/Data.Time.Duration.cinch)

<a id="value-6475726174696f6e2d746f2d6d6963726f73"></a>

## `duration-to-micros`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">duration-to-micros</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">Duration</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">I64</span>))</code></pre>

[Source](/reference-sources/Data.Time.Duration.cinch)

<a id="value-6475726174696f6e2d746f2d6d696c6c6973"></a>

## `duration-to-millis`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">duration-to-millis</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">Duration</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">I64</span>))</code></pre>

[Source](/reference-sources/Data.Time.Duration.cinch)

<a id="value-6475726174696f6e2d746f2d6d696e75746573"></a>

## `duration-to-minutes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">duration-to-minutes</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">Duration</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">I64</span>))</code></pre>

[Source](/reference-sources/Data.Time.Duration.cinch)

<a id="value-6475726174696f6e2d746f2d6e616e6f73"></a>

## `duration-to-nanos`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">duration-to-nanos</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">Duration</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">I64</span>))</code></pre>

`None` outside `I64` (about 292 years of nanoseconds).

[Source](/reference-sources/Data.Time.Duration.cinch) · line 240

<a id="value-6475726174696f6e2d746f2d7365636f6e6473"></a>

## `duration-to-seconds`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">duration-to-seconds</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">Duration</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">I64</span>))</code></pre>

The whole seconds (truncated toward zero; pair with
`duration-subsec-nanos`).

[Source](/reference-sources/Data.Time.Duration.cinch) · line 251

<a id="value-6475726174696f6e2d746f74616c2d6e616e6f73"></a>

## `duration-total-nanos`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">duration-total-nanos</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">Duration</span> -&gt; <span class="tok-constructor">I128</span>)</code></pre>

The whole count, exactly.

[Source](/reference-sources/Data.Time.Duration.cinch) · line 216

<a id="value-6475726174696f6e2d7a65726f"></a>

## `duration-zero`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">duration-zero</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">Duration</span></code></pre>

Bounds and construction.

[Source](/reference-sources/Data.Time.Duration.cinch) · line 164

<a id="value-6d61782d6475726174696f6e"></a>

## `max-duration`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">max-duration</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">Duration</span></code></pre>

2^127 - 1 nanoseconds.

[Source](/reference-sources/Data.Time.Duration.cinch) · line 172

<a id="value-6d696e2d6475726174696f6e"></a>

## `min-duration`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">min-duration</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">Duration</span></code></pre>

-(2^127 - 1) nanoseconds.

[Source](/reference-sources/Data.Time.Duration.cinch) · line 168

# Instances

These are checked exported instance entries. Law/sample-check status is separate from the existence of an instance; see [laws and optimization](../reference/12-rewrites-and-optimization.md).

| Instance | Target | Resolution |
| --- | --- | --- |
| `Data.Time.Duration.Eq_Duration` | `Relation.Eq.Eq Data.Time.Duration.Duration` | Implicit |
| `Data.Time.Duration.Ord_Duration` | `Relation.Ord.Ord Data.Time.Duration.Duration` | Implicit |

**`Data.Time.Duration.Eq_Duration`**: Instances (the class spellings of the integer order; instances
live with their type per design/base-library.md).

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Data/Time/Duration.cinch`. Source SHA-256: `65b41d25ea1aafb71542227a9456ccb996d696089b1af1993ff90ba80fd506cc`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
