# Effect.Time

Typed readers for the system and monotonic clocks.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Effect.Time`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Effect.Time.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Time.Duration](Data.Time.Duration.md), [Data.Time.Monotonic](Data.Time.Monotonic.md), [Data.Time.Timestamp](Data.Time.Timestamp.md), [Effect.Clock](Effect.Clock.md), [Relation.Eq](Relation.Eq.md), [Relation.Ord](Relation.Ord.md).

Non-embedded source closure: [Data.Time.Duration](Data.Time.Duration.md), [Data.Time.Monotonic](Data.Time.Monotonic.md), [Data.Time.Timestamp](Data.Time.Timestamp.md), [Effect.Time](Effect.Time.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Effect.Time: the typed clock readers (design/datetime.md, layer L5,
lane A).

`Effect.Clock` reads the runtime clocks as bare `Nat` nanosecond
counts and measures in unsigned milliseconds. This module is the
same two clocks with the machine-time types: the wall clock reads a
`Timestamp` ("when did that happen?"), the monotonic clock reads a
`Monotonic` ("how long did that take?"), and everything between is
a signed, nanosecond `Duration`. Every reading requires the `Io r`
capability and performs `IO r`, exactly as `Effect.Clock`'s do, so
nothing ambient exists and no reading can enter a build product
(D45; a `stage` body cannot call a foreign reader).

Design law shape. Two `monotonic-now` readings in one process never
go backwards, so `elapsed-since m` is never negative for an `m` that
`monotonic-now` returned. `now` is the wall clock, which can step
(NTP, the operator), so it orders nothing and is never used to
measure here. A `Deadline` is a monotonic reading in the future:
`deadline-remaining` is the signed time to it (negative once it has
passed), `deadline-expired` is `remaining <= 0`, and neither is
affected by a wall-clock step. `sleep-for d` suspends for at least
`d` (rounded UP to a whole millisecond, the unit of the runtime's
`ioSleep`), and for zero milliseconds when `d` is zero or negative;
it is always a suspension point, so it always observes cancellation.
`from-clock-duration (to-clock-duration d)` is `d` rounded up to a
millisecond for a non-negative `d` within `I64` milliseconds. The
checks are seed/tests/lib_time_a.rs.

NAMED REFUSALS (boundaries this module deliberately does not
cross):
  - compile-time readings: as `Effect.Clock` (D45).
  - wall times before the epoch or past `I64` nanoseconds (the year
    2262): the runtime clamps `cinch_clock_wall_ns` to
    `0..=i64::MAX`, so `now` reads within 1970..=2262.
  - sub-millisecond sleeps: `ioSleep` takes milliseconds, so
    `sleep-for` rounds up; a sleep longer than `I64` milliseconds
    (292 million years) is clamped to that.
  - a direct `I64` clock word: `Effect.Clock` exports its readers as
    `Nat` only (its `I64` foreign imports are private and the module
    is not edited by this work, T1), so each reading converts
    `I64 -> Nat -> I64`. The `Nat` is transient and never stored;
    both conversions are the named ones (`nat-from-i64` there,
    `checked-nat-to-i64` here). A reading above `I64` cannot occur
    (the runtime's clamp) and would saturate.
  - killing a timed-out computation, and work rows wider than
    `{IO r, Cancel r}`: `timeout-after` is `Effect.Clock.timeout`
    with the deadline given as a `Duration`, and inherits both
    boundaries.
  - the system time zone and zoned "now": `Effect.Time.Zone`.

Naming. The readers keep the short names the specification gives
them (`now`, `monotonic-now`, `sleep-for`); the deadline operations
are prefixed `deadline-`. `Effect.Clock` is imported qualified
because its millisecond type is also called `Duration` (and it
exports its own `duration-from-millis`); a program that imports
`Effect.Clock` and `Data.Time.Duration` together must select or
alias one of them, and one that imports this module needs neither
for sleeping, deadlines or timeouts.

Module scheme (design/base-library.md): `Effect.*` is the effect
system's vocabulary.

</details>

# Types and constructors

<a id="type-446561646c696e65"></a>

## `Deadline`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Deadline</span></code></pre>

A point on the monotonic clock to finish by: the reading's `I64`
nanosecond word, held directly (one object, not a box around a
`Monotonic` box). The constructor is private; `deadline-at` and
`deadline-instant` are its public spellings.

[Source](/reference-sources/Effect.Time.cinch) · line 149

<a id="type-54696d656f7574"></a>

## `Timeout`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Timeout</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>)</code></pre>

The outcome of a computation raced against its deadline: it
finished (`Finished x`), or the deadline fired first and cancelled
it (`TimedOut`).

[Source](/reference-sources/Effect.Clock.cinch) · line 88

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">TimedOut</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Clock</span>.<span class="tok-constructor">Timeout</span> <span class="tok-variable">a</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Finished</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . (<span class="tok-variable">a</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Clock</span>.<span class="tok-constructor">Timeout</span> <span class="tok-variable">a</span>)</code></pre>

# Values

<a id="value-646561646c696e652d6166746572"></a>

## `deadline-after`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">deadline-after</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">Duration</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Deadline</span>)</code></pre>

The deadline `d` from now. Saturating: a `d` past the clock's
`I64` range gives the farthest representable deadline (292 years
from the origin), and a negative `d` one that has already passed.

[Source](/reference-sources/Effect.Time.cinch) · line 164

<a id="value-646561646c696e652d6174"></a>

## `deadline-at`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">deadline-at</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Monotonic</span>.<span class="tok-constructor">Monotonic</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Deadline</span>)</code></pre>

The deadline at a known monotonic reading.

[Source](/reference-sources/Effect.Time.cinch) · line 153

<a id="value-646561646c696e652d65787069726564"></a>

## `deadline-expired`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">deadline-expired</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Deadline</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Bool</span>)</code></pre>

Whether the deadline has been reached, read now.

[Source](/reference-sources/Effect.Time.cinch) · line 173

<a id="value-646561646c696e652d696e7374616e74"></a>

## `deadline-instant`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">deadline-instant</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Deadline</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Monotonic</span>.<span class="tok-constructor">Monotonic</span>)</code></pre>

The monotonic reading the deadline falls on.

[Source](/reference-sources/Effect.Time.cinch) · line 157

<a id="value-646561646c696e652d72656d61696e696e67"></a>

## `deadline-remaining`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">deadline-remaining</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Deadline</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">Duration</span>)</code></pre>

The time left, read now: positive before the deadline, zero on it,
negative after it (`sleep-for` treats that as zero).

[Source](/reference-sources/Effect.Time.cinch) · line 169

<a id="value-656c61707365642d73696e6365"></a>

## `elapsed-since`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elapsed-since</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Monotonic</span>.<span class="tok-constructor">Monotonic</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">Duration</span>)</code></pre>

The time since the reading `start`, read now.

[Source](/reference-sources/Effect.Time.cinch) · line 107

<a id="value-66726f6d2d636c6f636b2d6475726174696f6e"></a>

## `from-clock-duration`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">from-clock-duration</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Clock</span>.<span class="tok-constructor">Duration</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">Duration</span>)</code></pre>

Exact for every count within `I64` milliseconds; saturating at
that bound for a larger `Nat`.

[Source](/reference-sources/Effect.Time.cinch) · line 132

<a id="value-6d6f6e6f746f6e69632d6e6f77"></a>

## `monotonic-now`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">monotonic-now</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Unit</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Monotonic</span>.<span class="tok-constructor">Monotonic</span>)</code></pre>

The monotonic clock: non-decreasing across calls in one process.

[Source](/reference-sources/Effect.Time.cinch) · line 103

<a id="value-6e6f77"></a>

## `now`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">now</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Unit</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span>)</code></pre>

The wall clock: the current instant. It can step backwards or
forwards when the system clock is set; measure with
`monotonic-now`.

[Source](/reference-sources/Effect.Time.cinch) · line 99

<a id="value-736c6565702d666f72"></a>

## `sleep-for`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sleep-for</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">Duration</span> -&gt;{<span class="tok-constructor">Cancel</span> <span class="tok-variable">r</span>, <span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>)</code></pre>

Suspend for at least `d`: rounded up to a whole millisecond, and
zero milliseconds for a zero or negative `d` (never an error and
never a negative sleep). Always a suspension point.

[Source](/reference-sources/Effect.Time.cinch) · line 138

<a id="value-736c6565702d756e74696c"></a>

## `sleep-until`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sleep-until</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Deadline</span> -&gt;{<span class="tok-constructor">Cancel</span> <span class="tok-variable">r</span>, <span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>)</code></pre>

Suspend until the deadline (not at all, beyond the suspension
point itself, when it has passed).

[Source](/reference-sources/Effect.Time.cinch) · line 178

<a id="value-74696d656f75742d6166746572"></a>

## `timeout-after`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">timeout-after</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">Duration</span> -&gt; ((<span class="tok-constructor">Unit</span> -&gt;{<span class="tok-constructor">Cancel</span> <span class="tok-variable">r</span>, <span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} <span class="tok-variable">a</span>) %1 -&gt;{<span class="tok-constructor">Async</span> <span class="tok-variable">r</span>, <span class="tok-constructor">Cancel</span> <span class="tok-variable">r</span>, <span class="tok-constructor">Diverge</span>, <span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Clock</span>.<span class="tok-constructor">Timeout</span> <span class="tok-variable">a</span>)))</code></pre>

`Effect.Clock.timeout` with the limit as a `Duration` (rounded up
to a millisecond; zero for a negative one). The result type is
`Effect.Clock`'s `Timeout` (`TimedOut`, `Finished`), re-exported
from this module so a caller needs no `Effect.Clock` import.

[Source](/reference-sources/Effect.Time.cinch) · line 189

<a id="value-746f2d636c6f636b2d6475726174696f6e"></a>

## `to-clock-duration`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">to-clock-duration</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">Duration</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Clock</span>.<span class="tok-constructor">Duration</span>)</code></pre>

The millisecond `Effect.Clock.Duration` not shorter than `d`
(rounded up to a millisecond, so a deadline is never early);
saturating: zero for a negative `d`, `I64` milliseconds at most.

[Source](/reference-sources/Effect.Time.cinch) · line 127

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Effect/Time.cinch`. Source SHA-256: `55fb18e3413088cd5db3e41302ae0984f1f0039c0e386ae8ca72e4e4fe7be4e4`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
