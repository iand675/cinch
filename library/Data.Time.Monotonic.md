# Data.Time.Monotonic

Monotonic clock readings for measuring elapsed time; never related to the civil calendar.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Data.Time.Monotonic`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Time.Monotonic.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Machine](Data.Machine.md), [Data.Option](Data.Option.md), [Data.Ordering](Data.Ordering.md), [Data.Time.Duration](Data.Time.Duration.md), [Data.Wide](Data.Wide.md), [Relation.Eq](Relation.Eq.md), [Relation.Ord](Relation.Ord.md).

Non-embedded source closure: [Data.Time.Duration](Data.Time.Duration.md), [Data.Time.Monotonic](Data.Time.Monotonic.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Data.Time.Monotonic: a reading of the monotonic clock
(design/datetime.md, layer L0 "machine time", lane A).

A `Monotonic` answers "how long did that take?" and nothing else. It
is a count of nanoseconds since an origin the runtime picks at
start-up, so a single reading means nothing and only the difference
of two readings from the same process does. There is deliberately no
way to turn one into a `Timestamp` or a date: "when did that
happen?" is `Data.Time.Timestamp`'s question.

REPRESENTATION. One `I64` (what `cinch_clock_monotonic_ns` returns,
unconverted) behind the private constructor `MkMonotonic`. As for
`Duration`, the scalar is unboxed and the single-constructor `data`
wrapper is one 8-byte heap object per value (`cinch_alloc_ctor(8,
0)`); `#[repr(transparent)] record` does not remove the box in
native code and would make the type constructible by importers.

Design law shape. `monotonic-elapsed-between a b` is `b - a`, signed
and exact (the difference of two `I64`s always fits a `Duration`),
so it is antisymmetric (`elapsed-between b a` is its negation) and
zero on equal readings. `monotonic-checked-add m d` is the reading
`d` later, so `elapsed-between m (add m d)` is `d` whenever the sum
exists, and `checked-sub m d` is `checked-add m (neg d)`.
`monotonic-cmp` is the integer order, and it is the order of the
instants when both readings come from one process. The checks are
seed/tests/lib_time_a.rs.

NAMED REFUSALS (boundaries this module deliberately does not
cross):
  - conversion to or from wall-clock time: no function relates a
    `Monotonic` to a `Timestamp`.
  - readings from different processes or boots: the origin is
    per-process; comparing across processes is meaningless and the
    type cannot detect it.
  - a friend-only constructor: Cinch has no friend modules, so the
    raw constructor `monotonic-from-raw-nanos` is exported for
    `Effect.Time`. Only a clock reader should call it (tests use it
    to build fixed readings).
  - sums outside `I64` nanoseconds (about 292 years from the
    origin): `monotonic-checked-add` answers `None`, the
    `saturating-` twin clamps.
  - `Int`/`Nat` (bignums): neither appears.

Naming: every export is prefixed `monotonic-` (see
`Data.Time.Duration`'s header).

</details>

# Types and constructors

<a id="type-4d6f6e6f746f6e6963"></a>

## `Monotonic`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Monotonic</span></code></pre>

Nanoseconds since the runtime's origin. Not exported (named
apart from the type so that `export Monotonic` does not export it).

[Source](/reference-sources/Data.Time.Monotonic.cinch) · line 66

# Values

<a id="value-6d6f6e6f746f6e69632d636865636b65642d616464"></a>

## `monotonic-checked-add`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">monotonic-checked-add</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Monotonic</span>.<span class="tok-constructor">Monotonic</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">Duration</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Monotonic</span>.<span class="tok-constructor">Monotonic</span>)))</code></pre>

The reading `d` after `m` (before it for a negative `d`); `None`
when that leaves `I64` nanoseconds.

[Source](/reference-sources/Data.Time.Monotonic.cinch) · line 99

<a id="value-6d6f6e6f746f6e69632d636865636b65642d737562"></a>

## `monotonic-checked-sub`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">monotonic-checked-sub</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Monotonic</span>.<span class="tok-constructor">Monotonic</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">Duration</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Monotonic</span>.<span class="tok-constructor">Monotonic</span>)))</code></pre>

The reading `d` before `m`. Negation is total on durations.

[Source](/reference-sources/Data.Time.Monotonic.cinch) · line 107

<a id="value-6d6f6e6f746f6e69632d636d70"></a>

## `monotonic-cmp`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">monotonic-cmp</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Monotonic</span>.<span class="tok-constructor">Monotonic</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Monotonic</span>.<span class="tok-constructor">Monotonic</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Ordering</span>.<span class="tok-constructor">Ordering</span>))</code></pre>

[Source](/reference-sources/Data.Time.Monotonic.cinch)

<a id="value-6d6f6e6f746f6e69632d656c61707365642d6265747765656e"></a>

## `monotonic-elapsed-between`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">monotonic-elapsed-between</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Monotonic</span>.<span class="tok-constructor">Monotonic</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Monotonic</span>.<span class="tok-constructor">Monotonic</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">Duration</span>))</code></pre>

The time from `earlier` to `later`: `later - earlier`, negative
when the arguments are the other way round. Total: the difference
of two `I64`s has magnitude below 2^64, so the wrapping `I128`
subtraction is exact and the result is a duration.

[Source](/reference-sources/Data.Time.Monotonic.cinch) · line 89

<a id="value-6d6f6e6f746f6e69632d66726f6d2d7261772d6e616e6f73"></a>

## `monotonic-from-raw-nanos`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">monotonic-from-raw-nanos</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Monotonic</span>.<span class="tok-constructor">Monotonic</span>)</code></pre>

RAW: wrap a monotonic clock word. For clock readers
(`Effect.Time.monotonic-now`) and tests only; a number that did
not come from this process's monotonic clock makes a reading that
compares meaninglessly with real ones.

[Source](/reference-sources/Data.Time.Monotonic.cinch) · line 77

<a id="value-6d6f6e6f746f6e69632d73617475726174696e672d616464"></a>

## `monotonic-saturating-add`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">monotonic-saturating-add</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Monotonic</span>.<span class="tok-constructor">Monotonic</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">Duration</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Monotonic</span>.<span class="tok-constructor">Monotonic</span>))</code></pre>

The same, clamped to the `I64` bounds.

[Source](/reference-sources/Data.Time.Monotonic.cinch) · line 111

<a id="value-6d6f6e6f746f6e69632d73617475726174696e672d737562"></a>

## `monotonic-saturating-sub`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">monotonic-saturating-sub</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Monotonic</span>.<span class="tok-constructor">Monotonic</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">Duration</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Monotonic</span>.<span class="tok-constructor">Monotonic</span>))</code></pre>

[Source](/reference-sources/Data.Time.Monotonic.cinch)

<a id="value-6d6f6e6f746f6e69632d746f2d7261772d6e616e6f73"></a>

## `monotonic-to-raw-nanos`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">monotonic-to-raw-nanos</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Monotonic</span>.<span class="tok-constructor">Monotonic</span> -&gt; <span class="tok-constructor">I64</span>)</code></pre>

RAW: the clock word back (for logging and tracing; it is not a
time of day and not comparable across processes).

[Source](/reference-sources/Data.Time.Monotonic.cinch) · line 82

# Instances

These are checked exported instance entries. Law/sample-check status is separate from the existence of an instance; see [laws and optimization](../reference/12-rewrites-and-optimization.md).

| Instance | Target | Resolution |
| --- | --- | --- |
| `Data.Time.Monotonic.Eq_Monotonic` | `Relation.Eq.Eq Data.Time.Monotonic.Monotonic` | Implicit |
| `Data.Time.Monotonic.Ord_Monotonic` | `Relation.Ord.Ord Data.Time.Monotonic.Monotonic` | Implicit |

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Data/Time/Monotonic.cinch`. Source SHA-256: `35ad9089126831a35cefed441b34eec60c6b807e9770bd8185dc3eec72c63ca6`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
