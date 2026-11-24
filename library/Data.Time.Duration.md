# Data.Time.Duration

Exact signed elapsed-time durations with nanosecond precision and checked arithmetic.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Data.Time.Duration</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Data.Time.Duration/source/">View source</a><a href="/packages/search/?q=%2BData.Time.Duration">Search module</a><a href="/reference-sources/Data.Time.Duration.cinch" download="Data.Time.Duration.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Algebra.Arithmetic](Algebra.Arithmetic.md), [Data.Machine](Data.Machine.md), [Data.Option](Data.Option.md), [Data.Ordering](Data.Ordering.md), [Data.Wide](Data.Wide.md), [Relation.Eq](Relation.Eq.md), [Relation.Ord](Relation.Ord.md).

<details class="doc-notes"><summary>Module notes</summary>

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

<details class="doc-synopsis"><summary>Synopsis (43 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-Duration">Duration</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Duration</strong></code></li><li><a class="syn-name" href="#v-duration-abs">duration-abs</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">duration-abs</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a></code></li><li><a class="syn-name" href="#v-duration-checked-add">duration-checked-add</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">duration-checked-add</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a></code></li><li><a class="syn-name" href="#v-duration-checked-mul">duration-checked-mul</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">duration-checked-mul</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a></code></li><li><a class="syn-name" href="#v-duration-checked-sub">duration-checked-sub</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">duration-checked-sub</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a></code></li><li><a class="syn-name" href="#v-duration-cmp">duration-cmp</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">duration-cmp</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Ordering/#t-Ordering" title="Data.Ordering.Ordering">Ordering</a></code></li><li><a class="syn-name" href="#v-duration-div">duration-div</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">duration-div</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-NonZero" title="Algebra.Arithmetic.NonZero">NonZero</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a></code></li><li><a class="syn-name" href="#v-duration-div-floor">duration-div-floor</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">duration-div-floor</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-NonZero" title="Algebra.Arithmetic.NonZero">NonZero</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a></code></li><li><a class="syn-name" href="#v-duration-from-hours">duration-from-hours</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">duration-from-hours</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a></code></li><li><a class="syn-name" href="#v-duration-from-i128-nanos">duration-from-i128-nanos</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">duration-from-i128-nanos</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="I128">I128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a></code></li><li><a class="syn-name" href="#v-duration-from-micros">duration-from-micros</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">duration-from-micros</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a></code></li><li><a class="syn-name" href="#v-duration-from-millis">duration-from-millis</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">duration-from-millis</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a></code></li><li><a class="syn-name" href="#v-duration-from-minutes">duration-from-minutes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">duration-from-minutes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a></code></li><li><a class="syn-name" href="#v-duration-from-nanos">duration-from-nanos</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">duration-from-nanos</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a></code></li><li><a class="syn-name" href="#v-duration-from-seconds">duration-from-seconds</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">duration-from-seconds</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a></code></li><li><a class="syn-name" href="#v-duration-from-seconds-and-nanos">duration-from-seconds-and-nanos</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">duration-from-seconds-and-nanos</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a></code></li><li><a class="syn-name" href="#v-duration-is-negative">duration-is-negative</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">duration-is-negative</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-duration-is-positive">duration-is-positive</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">duration-is-positive</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-duration-is-zero">duration-is-zero</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">duration-is-zero</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-duration-max-of">duration-max-of</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">duration-max-of</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a></code></li><li><a class="syn-name" href="#v-duration-min-of">duration-min-of</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">duration-min-of</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a></code></li><li><a class="syn-name" href="#v-duration-neg">duration-neg</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">duration-neg</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a></code></li><li><a class="syn-name" href="#v-duration-saturating-add">duration-saturating-add</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">duration-saturating-add</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a></code></li><li><a class="syn-name" href="#v-duration-saturating-from-i128-nanos">duration-saturating-from-i128-nanos</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">duration-saturating-from-i128-nanos</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="I128">I128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a></code></li><li><a class="syn-name" href="#v-duration-saturating-mul">duration-saturating-mul</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">duration-saturating-mul</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a></code></li><li><a class="syn-name" href="#v-duration-saturating-sub">duration-saturating-sub</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">duration-saturating-sub</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a></code></li><li><a class="syn-name" href="#v-duration-saturating-to-hours">duration-saturating-to-hours</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">duration-saturating-to-hours</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-duration-saturating-to-micros">duration-saturating-to-micros</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">duration-saturating-to-micros</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-duration-saturating-to-millis">duration-saturating-to-millis</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">duration-saturating-to-millis</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-duration-saturating-to-minutes">duration-saturating-to-minutes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">duration-saturating-to-minutes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-duration-saturating-to-nanos">duration-saturating-to-nanos</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">duration-saturating-to-nanos</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-duration-saturating-to-seconds">duration-saturating-to-seconds</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">duration-saturating-to-seconds</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-duration-subsec-nanos">duration-subsec-nanos</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">duration-subsec-nanos</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> - 999999999 999999999 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-duration-to-hours">duration-to-hours</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">duration-to-hours</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-duration-to-micros">duration-to-micros</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">duration-to-micros</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-duration-to-millis">duration-to-millis</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">duration-to-millis</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-duration-to-minutes">duration-to-minutes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">duration-to-minutes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-duration-to-nanos">duration-to-nanos</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">duration-to-nanos</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-duration-to-seconds">duration-to-seconds</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">duration-to-seconds</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-duration-total-nanos">duration-total-nanos</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">duration-total-nanos</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="I128">I128</a></code></li><li><a class="syn-name" href="#v-duration-zero">duration-zero</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">duration-zero</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a></code></li><li><a class="syn-name" href="#v-max-duration">max-duration</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">max-duration</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a></code></li><li><a class="syn-name" href="#v-min-duration">min-duration</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">min-duration</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-Duration">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Duration</strong></code><span class="doc-sig-links"><a href="#t-Duration" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Duration/source/#L110" title="View source at line 110">Source</a></span></div>
<div class="doc-body">

The nanosecond count. Not exported (the range invariant above).
The constructor's name differs from the type's on purpose:
`export Duration` exports the type AND a constructor of the same
name, so a constructor called `Duration` would be public.

<details class="doc-instances"><summary>Instances (2)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Time.Duration/#instances">Data.Time.Duration</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Time.Duration/#instances">Data.Time.Duration</a></span></li></ul></details>
</div>
</article>

# Values

<article class="doc-entry" id="v-duration-abs">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">duration-abs</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a></code><span class="doc-sig-links"><a href="#v-duration-abs" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Duration/source/#L297" title="View source at line 297">Source</a></span></div>
<div class="doc-body">

Total, by the same argument.

</div>
</article>

<article class="doc-entry" id="v-duration-checked-add">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">duration-checked-add</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a></code><span class="doc-sig-links"><a href="#v-duration-checked-add" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Duration/source/#L300" title="View source at line 300">Source</a></span></div>
</article>

<article class="doc-entry" id="v-duration-checked-mul">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">duration-checked-mul</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a></code><span class="doc-sig-links"><a href="#v-duration-checked-mul" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Duration/source/#L317" title="View source at line 317">Source</a></span></div>
<div class="doc-body">

The duration `k` times over.

</div>
</article>

<article class="doc-entry" id="v-duration-checked-sub">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">duration-checked-sub</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a></code><span class="doc-sig-links"><a href="#v-duration-checked-sub" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Duration/source/#L305" title="View source at line 305">Source</a></span></div>
</article>

<article class="doc-entry" id="v-duration-cmp">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">duration-cmp</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Ordering/#t-Ordering" title="Data.Ordering.Ordering">Ordering</a></code><span class="doc-sig-links"><a href="#v-duration-cmp" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Duration/source/#L349" title="View source at line 349">Source</a></span></div>
<div class="doc-body">

Order.

</div>
</article>

<article class="doc-entry" id="v-duration-div">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">duration-div</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-NonZero" title="Algebra.Arithmetic.NonZero">NonZero</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a></code><span class="doc-sig-links"><a href="#v-duration-div" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Duration/source/#L328" title="View source at line 328">Source</a></span></div>
<div class="doc-body">

One `k`th of the duration, truncated toward zero. Total: the
divisor is not zero by its type and the quotient's magnitude never
exceeds the dividend's.

</div>
</article>

<article class="doc-entry" id="v-duration-div-floor">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">duration-div-floor</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-NonZero" title="Algebra.Arithmetic.NonZero">NonZero</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a></code><span class="doc-sig-links"><a href="#v-duration-div-floor" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Duration/source/#L336" title="View source at line 336">Source</a></span></div>
<div class="doc-body">

One `k`th of the duration, rounded toward negative infinity: the
truncated quotient, less one when the division is inexact and the
exact quotient is negative (remainder and divisor of opposite
signs). The decrement cannot leave the range: an inexact division
has `|k| >= 2`, so the quotient's magnitude is at most 2^126.

</div>
</article>

<article class="doc-entry" id="v-duration-from-hours">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">duration-from-hours</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a></code><span class="doc-sig-links"><a href="#v-duration-from-hours" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Duration/source/#L192" title="View source at line 192">Source</a></span></div>
</article>

<article class="doc-entry" id="v-duration-from-i128-nanos">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">duration-from-i128-nanos</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="I128">I128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a></code><span class="doc-sig-links"><a href="#v-duration-from-i128-nanos" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Duration/source/#L204" title="View source at line 204">Source</a></span></div>
<div class="doc-body">

A raw nanosecond count: `None` for `I128`'s minimum only.

</div>
</article>

<article class="doc-entry" id="v-duration-from-micros">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">duration-from-micros</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a></code><span class="doc-sig-links"><a href="#v-duration-from-micros" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Duration/source/#L180" title="View source at line 180">Source</a></span></div>
</article>

<article class="doc-entry" id="v-duration-from-millis">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">duration-from-millis</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a></code><span class="doc-sig-links"><a href="#v-duration-from-millis" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Duration/source/#L183" title="View source at line 183">Source</a></span></div>
</article>

<article class="doc-entry" id="v-duration-from-minutes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">duration-from-minutes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a></code><span class="doc-sig-links"><a href="#v-duration-from-minutes" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Duration/source/#L189" title="View source at line 189">Source</a></span></div>
</article>

<article class="doc-entry" id="v-duration-from-nanos">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">duration-from-nanos</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a></code><span class="doc-sig-links"><a href="#v-duration-from-nanos" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Duration/source/#L177" title="View source at line 177">Source</a></span></div>
<div class="doc-body">

The unit constructors are total: every `I64` count of every unit
is in the range (the header's RANGE argument).

</div>
</article>

<article class="doc-entry" id="v-duration-from-seconds">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">duration-from-seconds</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a></code><span class="doc-sig-links"><a href="#v-duration-from-seconds" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Duration/source/#L186" title="View source at line 186">Source</a></span></div>
</article>

<article class="doc-entry" id="v-duration-from-seconds-and-nanos">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">duration-from-seconds-and-nanos</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a></code><span class="doc-sig-links"><a href="#v-duration-from-seconds-and-nanos" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Duration/source/#L199" title="View source at line 199">Source</a></span></div>
<div class="doc-body">

`secs` seconds plus `nanos` nanoseconds, any signs (the two parts
are added, so `(-1, 999999999)` is -1 ns). Total: the seconds term
is below 2^93 and the nanosecond term below 2^63, so the wrapping
sum is exact.

</div>
</article>

<article class="doc-entry" id="v-duration-is-negative">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">duration-is-negative</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-duration-is-negative" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Duration/source/#L281" title="View source at line 281">Source</a></span></div>
</article>

<article class="doc-entry" id="v-duration-is-positive">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">duration-is-positive</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-duration-is-positive" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Duration/source/#L284" title="View source at line 284">Source</a></span></div>
</article>

<article class="doc-entry" id="v-duration-is-zero">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">duration-is-zero</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-duration-is-zero" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Duration/source/#L278" title="View source at line 278">Source</a></span></div>
</article>

<article class="doc-entry" id="v-duration-max-of">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">duration-max-of</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a></code><span class="doc-sig-links"><a href="#v-duration-max-of" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Duration/source/#L360" title="View source at line 360">Source</a></span></div>
<div class="doc-body">

The longer (more positive) of two durations.

</div>
</article>

<article class="doc-entry" id="v-duration-min-of">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">duration-min-of</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a></code><span class="doc-sig-links"><a href="#v-duration-min-of" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Duration/source/#L356" title="View source at line 356">Source</a></span></div>
<div class="doc-body">

The shorter (more negative) of two durations.

</div>
</article>

<article class="doc-entry" id="v-duration-neg">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">duration-neg</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a></code><span class="doc-sig-links"><a href="#v-duration-neg" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Duration/source/#L293" title="View source at line 293">Source</a></span></div>
<div class="doc-body">

Total: the range is symmetric, so the wrapping negation is exact
and stays in the range.

</div>
</article>

<article class="doc-entry" id="v-duration-saturating-add">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">duration-saturating-add</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a></code><span class="doc-sig-links"><a href="#v-duration-saturating-add" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Duration/source/#L310" title="View source at line 310">Source</a></span></div>
</article>

<article class="doc-entry" id="v-duration-saturating-from-i128-nanos">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">duration-saturating-from-i128-nanos</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="I128">I128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a></code><span class="doc-sig-links"><a href="#v-duration-saturating-from-i128-nanos" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Duration/source/#L208" title="View source at line 208">Source</a></span></div>
<div class="doc-body">

The same, with the minimum clamped to `min-duration`.

</div>
</article>

<article class="doc-entry" id="v-duration-saturating-mul">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">duration-saturating-mul</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a></code><span class="doc-sig-links"><a href="#v-duration-saturating-mul" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Duration/source/#L322" title="View source at line 322">Source</a></span></div>
</article>

<article class="doc-entry" id="v-duration-saturating-sub">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">duration-saturating-sub</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a></code><span class="doc-sig-links"><a href="#v-duration-saturating-sub" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Duration/source/#L313" title="View source at line 313">Source</a></span></div>
</article>

<article class="doc-entry" id="v-duration-saturating-to-hours">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">duration-saturating-to-hours</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-duration-saturating-to-hours" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Duration/source/#L275" title="View source at line 275">Source</a></span></div>
</article>

<article class="doc-entry" id="v-duration-saturating-to-micros">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">duration-saturating-to-micros</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-duration-saturating-to-micros" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Duration/source/#L263" title="View source at line 263">Source</a></span></div>
</article>

<article class="doc-entry" id="v-duration-saturating-to-millis">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">duration-saturating-to-millis</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-duration-saturating-to-millis" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Duration/source/#L266" title="View source at line 266">Source</a></span></div>
</article>

<article class="doc-entry" id="v-duration-saturating-to-minutes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">duration-saturating-to-minutes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-duration-saturating-to-minutes" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Duration/source/#L272" title="View source at line 272">Source</a></span></div>
</article>

<article class="doc-entry" id="v-duration-saturating-to-nanos">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">duration-saturating-to-nanos</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-duration-saturating-to-nanos" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Duration/source/#L260" title="View source at line 260">Source</a></span></div>
</article>

<article class="doc-entry" id="v-duration-saturating-to-seconds">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">duration-saturating-to-seconds</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-duration-saturating-to-seconds" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Duration/source/#L269" title="View source at line 269">Source</a></span></div>
</article>

<article class="doc-entry" id="v-duration-subsec-nanos">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">duration-subsec-nanos</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> - 999999999 999999999 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-duration-subsec-nanos" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Duration/source/#L225" title="View source at line 225">Source</a></span></div>
<div class="doc-body">

The part below one second, with the duration's sign:
`-999999999..=999999999`, and zero or the sign of the duration.
The remainder of an `I128` by 10^9 is in that interval, so the
wrapping narrowing is exact; the guard restates the bound for the
checker (a wide scalar has no solver sort) and its `else` arm is
unreachable.

</div>
</article>

<article class="doc-entry" id="v-duration-to-hours">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">duration-to-hours</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-duration-to-hours" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Duration/source/#L257" title="View source at line 257">Source</a></span></div>
</article>

<article class="doc-entry" id="v-duration-to-micros">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">duration-to-micros</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-duration-to-micros" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Duration/source/#L243" title="View source at line 243">Source</a></span></div>
</article>

<article class="doc-entry" id="v-duration-to-millis">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">duration-to-millis</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-duration-to-millis" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Duration/source/#L246" title="View source at line 246">Source</a></span></div>
</article>

<article class="doc-entry" id="v-duration-to-minutes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">duration-to-minutes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-duration-to-minutes" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Duration/source/#L254" title="View source at line 254">Source</a></span></div>
</article>

<article class="doc-entry" id="v-duration-to-nanos">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">duration-to-nanos</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-duration-to-nanos" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Duration/source/#L240" title="View source at line 240">Source</a></span></div>
<div class="doc-body">

`None` outside `I64` (about 292 years of nanoseconds).

</div>
</article>

<article class="doc-entry" id="v-duration-to-seconds">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">duration-to-seconds</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-duration-to-seconds" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Duration/source/#L251" title="View source at line 251">Source</a></span></div>
<div class="doc-body">

The whole seconds (truncated toward zero; pair with
`duration-subsec-nanos`).

</div>
</article>

<article class="doc-entry" id="v-duration-total-nanos">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">duration-total-nanos</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="I128">I128</a></code><span class="doc-sig-links"><a href="#v-duration-total-nanos" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Duration/source/#L216" title="View source at line 216">Source</a></span></div>
<div class="doc-body">

The whole count, exactly.

</div>
</article>

<article class="doc-entry" id="v-duration-zero">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">duration-zero</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a></code><span class="doc-sig-links"><a href="#v-duration-zero" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Duration/source/#L164" title="View source at line 164">Source</a></span></div>
<div class="doc-body">

Bounds and construction.

</div>
</article>

<article class="doc-entry" id="v-max-duration">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">max-duration</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a></code><span class="doc-sig-links"><a href="#v-max-duration" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Duration/source/#L172" title="View source at line 172">Source</a></span></div>
<div class="doc-body">

2^127 - 1 nanoseconds.

</div>
</article>

<article class="doc-entry" id="v-min-duration">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">min-duration</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a></code><span class="doc-sig-links"><a href="#v-min-duration" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Duration/source/#L168" title="View source at line 168">Source</a></span></div>
<div class="doc-body">

-(2^127 - 1) nanoseconds.

</div>
</article>

# Instances

Exported checked instances declared in this module. For law checking, see [rewrites and optimization](../reference/12-rewrites-and-optimization.md).

<ul class="doc-module-instances">
<li id="i-Eq_Duration"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Time.Duration.Eq_Duration</code></span><div class="doc-child-prose">Instances (the class spellings of the integer order; instances live with their type per design/base-library.md).</div></li>
<li id="i-Ord_Duration"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Time.Duration.Ord_Duration</code></span></li>
</ul>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Data/Time/Duration.cinch`](/packages/stdlib/Data.Time.Duration/source/). Source SHA-256: `65b41d25ea1aafb71542227a9456ccb996d696089b1af1993ff90ba80fd506cc`.
