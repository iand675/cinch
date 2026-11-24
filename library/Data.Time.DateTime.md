# Data.Time.DateTime

Civil date and wall-clock time pairs without a time zone.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Data.Time.DateTime`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Time.DateTime.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Algebra.Arithmetic](Algebra.Arithmetic.md), [Data.Machine](Data.Machine.md), [Data.Option](Data.Option.md), [Data.Ordering](Data.Ordering.md), [Data.Time.Date](Data.Time.Date.md), [Data.Time.TimeOfDay](Data.Time.TimeOfDay.md), [Data.Time.Weekday](Data.Time.Weekday.md), [Data.Wide](Data.Wide.md), [Relation.Eq](Relation.Eq.md), [Relation.Ord](Relation.Ord.md).

Non-embedded source closure: [Data.Time.Date](Data.Time.Date.md), [Data.Time.DateTime](Data.Time.DateTime.md), [Data.Time.TimeOfDay](Data.Time.TimeOfDay.md), [Data.Time.Weekday](Data.Time.Weekday.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Data.Time.DateTime: a civil date and wall-clock time.

A `DateTime` is a `Date` and a `TimeOfDay` together: what a calendar
and a wall clock show at once, attached to no place. It is NOT an
instant. Turning a civil reading into an instant needs an offset or
a time zone, and that conversion is the zone layer's
(Data.Time.Offset, Data.Time.Zone); the two `-utc` functions here
are the arithmetic that layer is built from, and their names say
what they assume.

Representation. A two-field `data` behind a private constructor
holding the `Date` and the `TimeOfDay` (design/datetime.md section
4: "two fields, boxed until unpacking lands"). As held today
(verified against the emitted LLVM): one heap object with two
reference fields, each pointing at the one-word object of its part,
so `date-part` and `time-part` are a field read and build nothing.
Storing the two `I64` words inline instead would save two objects
per value but needs an unchecked `Date`-from-word constructor
exported from Data.Time.Date, which would let any importer forge an
invalid date; the invariant was kept.

Design law shape. `datetime d t` followed by `date-part` and
`time-part` returns `d` and `t`. The order is lexicographic, date
then time, and agrees with the "as if UTC" line:
`cmp a b` is the sign of `diff-nanos a b` reversed in argument order
(`diff-nanos start finish` is finish minus start) and the order of
the `(seconds, nanoseconds)` pairs of `to-epoch-seconds-nanos-utc`.
`from-epoch-seconds-nanos-utc` inverts `to-epoch-seconds-nanos-utc`
on the whole range. `add-seconds-nanos s n x` is the value whose
"as if UTC" line position is `x`'s plus `s` seconds and `n`
nanoseconds, or `None` when the date leaves -9999-01-01..=9999-12-31;
it carries through midnight, month ends and year ends in either
direction. `add-days`, `add-months-constrain` and
`add-years-constrain` move the date exactly as Data.Time.Date's do
and keep the time.

No operation faults (T2): everything is total or answers `Option`.

NAMED REFUSALS:
  - an inline two-word value: see Representation above.
  - instants, offsets and zones: a `DateTime` has no position on the
    timeline; "epoch seconds as if UTC" is a civil-line coordinate
    for the zone layer, not a `Timestamp`.
  - a `Duration`- or `Span`-typed surface, `until`/`since` in
    calendar units, and rounding: Data.Time.Arithmetic owns those;
    this module is plain `I64` (and one `I128` for the full-range
    nanosecond difference, which does not fit `I64`: the range spans
    about 6.3e20 nanoseconds).
  - text (Text.Time.Iso).
  - a total machine division: `Data.Machine` has only
    `checked-div-i64`/`checked-rem-i64`; the private `quot`/`rem`
    take a divisor at `InRange 2 9223372036854775807 I64` (which
    excludes both `None` cases) and still must match the `None` arm.

</details>

# Types and constructors

<a id="type-4461746554696d65"></a>

## `DateTime`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">DateTime</span></code></pre>

The date and the time (header). Not exported.

[Source](/reference-sources/Data.Time.DateTime.cinch) · line 81

# Values

<a id="value-6164642d64617973"></a>

## `add-days`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">add-days</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span>)))</code></pre>

`n` days later (earlier when negative), same time.

[Source](/reference-sources/Data.Time.DateTime.cinch) · line 187

<a id="value-6164642d6d6f6e7468732d636f6e73747261696e"></a>

## `add-months-constrain`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">add-months-constrain</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span>)))</code></pre>

`n` months later, the day clamped to the target month's length
(Temporal's "constrain"), same time.

[Source](/reference-sources/Data.Time.DateTime.cinch) · line 194

<a id="value-6164642d7365636f6e64732d6e616e6f73"></a>

## `add-seconds-nanos`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">add-seconds-nanos</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span>))))</code></pre>

Adds `secs` seconds and `nanos` nanoseconds, each signed and each
any `I64`, carrying into the date. `None` when the date leaves the
range. The time wraps with `T.add-seconds-nanos-wrapping` (total
for every pair of `I64`s; its overflow argument is there), and the
whole-day carry it reports (within +-1.07e14) moves the date
through `D.add-days`, whose sum is checked.

[Source](/reference-sources/Data.Time.DateTime.cinch) · line 180

<a id="value-6164642d79656172732d636f6e73747261696e"></a>

## `add-years-constrain`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">add-years-constrain</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span>)))</code></pre>

`n` years later, 29 February clamped to 28 February where the
target year is not a leap year, same time.

[Source](/reference-sources/Data.Time.DateTime.cinch) · line 201

<a id="value-636d70"></a>

## `cmp`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cmp</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Ordering</span>.<span class="tok-constructor">Ordering</span>))</code></pre>

Order: lexicographic, date then time.

[Source](/reference-sources/Data.Time.DateTime.cinch) · line 258

<a id="value-646174652d70617274"></a>

## `date-part`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">date-part</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Date</span>.<span class="tok-constructor">Date</span>)</code></pre>

[Source](/reference-sources/Data.Time.DateTime.cinch)

<a id="value-6461746574696d65"></a>

## `datetime`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">datetime</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Date</span>.<span class="tok-constructor">Date</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">TimeOfDay</span>.<span class="tok-constructor">TimeOfDay</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span>))</code></pre>

Construction and parts. Any `Date` and any `TimeOfDay` make a
`DateTime`, so construction is total.

[Source](/reference-sources/Data.Time.DateTime.cinch) · line 89

<a id="value-646179"></a>

## `day`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">day</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span> -&gt; (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">1</span> <span class="tok-number">31</span> <span class="tok-constructor">I64</span>))</code></pre>

[Source](/reference-sources/Data.Time.DateTime.cinch)

<a id="value-6461792d6f662d79656172"></a>

## `day-of-year`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">day-of-year</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span> -&gt; (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">1</span> <span class="tok-number">366</span> <span class="tok-constructor">I64</span>))</code></pre>

[Source](/reference-sources/Data.Time.DateTime.cinch)

<a id="value-646966662d6e616e6f73"></a>

## `diff-nanos`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">diff-nanos</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span> -&gt; <span class="tok-constructor">I128</span>))</code></pre>

`diff-nanos start finish`: finish minus start in nanoseconds on
the civil line (every day is 86400 seconds), negative when
`finish` is earlier. Total: the day difference is within
+-7304483 and the time difference within +-86399999999999, so the
result is within +-6.4e20, which needs `I128` (it exceeds `I64`'s
9.2e18) and is nowhere near wrapping it.

[Source](/reference-sources/Data.Time.DateTime.cinch) · line 212

<a id="value-656e642d6f662d646179"></a>

## `end-of-day`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">end-of-day</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span>)</code></pre>

The same date at 23:59:59.999999999, the last representable time
of the civil day.

[Source](/reference-sources/Data.Time.DateTime.cinch) · line 161

<a id="value-66726f6d2d64617465"></a>

## `from-date`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">from-date</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Date</span>.<span class="tok-constructor">Date</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span>)</code></pre>

The date at 00:00:00.

[Source](/reference-sources/Data.Time.DateTime.cinch) · line 93

<a id="value-66726f6d2d65706f63682d7365636f6e64732d6e616e6f732d757463"></a>

## `from-epoch-seconds-nanos-utc`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">from-epoch-seconds-nanos-utc</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span>)))</code></pre>

The inverse. `None` when `nanos` is outside 0..=999999999 (the
pair is not normalised) or the date leaves the range. The seconds
are any `I64`: they are split by truncated division into days and
a remainder that is moved into 0..=86399, so nothing wraps, and
the day count (within +-1.07e14) is refused by `D.from-epoch-days`
when out of range.

[Source](/reference-sources/Data.Time.DateTime.cinch) · line 240

<a id="value-686f7572"></a>

## `hour`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hour</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span> -&gt; (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">0</span> <span class="tok-number">23</span> <span class="tok-constructor">I64</span>))</code></pre>

[Source](/reference-sources/Data.Time.DateTime.cinch)

<a id="value-6d61782d6461746574696d65"></a>

## `max-datetime`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">max-datetime</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span></code></pre>

[Source](/reference-sources/Data.Time.DateTime.cinch)

<a id="value-6d6963726f7365636f6e64"></a>

## `microsecond`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">microsecond</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span> -&gt; (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">0</span> <span class="tok-number">999</span> <span class="tok-constructor">I64</span>))</code></pre>

[Source](/reference-sources/Data.Time.DateTime.cinch)

<a id="value-6d696c6c697365636f6e64"></a>

## `millisecond`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">millisecond</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span> -&gt; (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">0</span> <span class="tok-number">999</span> <span class="tok-constructor">I64</span>))</code></pre>

[Source](/reference-sources/Data.Time.DateTime.cinch)

<a id="value-6d696e2d6461746574696d65"></a>

## `min-datetime`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">min-datetime</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span></code></pre>

-9999-01-01T00:00:00 and 9999-12-31T23:59:59.999999999.

[Source](/reference-sources/Data.Time.DateTime.cinch) · line 105

<a id="value-6d696e757465"></a>

## `minute`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">minute</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span> -&gt; (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">0</span> <span class="tok-number">59</span> <span class="tok-constructor">I64</span>))</code></pre>

[Source](/reference-sources/Data.Time.DateTime.cinch)

<a id="value-6d6f6e7468"></a>

## `month`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">month</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span> -&gt; (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">1</span> <span class="tok-number">12</span> <span class="tok-constructor">I64</span>))</code></pre>

[Source](/reference-sources/Data.Time.DateTime.cinch)

<a id="value-6e616e6f7365636f6e64"></a>

## `nanosecond`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">nanosecond</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span> -&gt; (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">0</span> <span class="tok-number">999</span> <span class="tok-constructor">I64</span>))</code></pre>

[Source](/reference-sources/Data.Time.DateTime.cinch)

<a id="value-7365636f6e64"></a>

## `second`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">second</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span> -&gt; (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">0</span> <span class="tok-number">59</span> <span class="tok-constructor">I64</span>))</code></pre>

[Source](/reference-sources/Data.Time.DateTime.cinch)

<a id="value-73746172742d6f662d646179"></a>

## `start-of-day`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">start-of-day</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span>)</code></pre>

The same date at 00:00:00.

[Source](/reference-sources/Data.Time.DateTime.cinch) · line 156

<a id="value-7375627365632d6e616e6f7365636f6e64"></a>

## `subsec-nanosecond`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">subsec-nanosecond</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span> -&gt; (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">0</span> <span class="tok-number">999999999</span> <span class="tok-constructor">I64</span>))</code></pre>

[Source](/reference-sources/Data.Time.DateTime.cinch)

<a id="value-74696d652d70617274"></a>

## `time-part`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">time-part</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">TimeOfDay</span>.<span class="tok-constructor">TimeOfDay</span>)</code></pre>

[Source](/reference-sources/Data.Time.DateTime.cinch)

<a id="value-746f2d65706f63682d7365636f6e64732d6e616e6f732d757463"></a>

## `to-epoch-seconds-nanos-utc`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">to-epoch-seconds-nanos-utc</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span> -&gt; (<span class="tok-constructor">I64</span>, <span class="tok-constructor">I64</span>))</code></pre>

Whole seconds since 1970-01-01T00:00:00 reading the civil value as
if it were UTC, floored, and the subsecond nanoseconds in
0..=999999999. One nanosecond before the epoch is `(-1, 999999999)`.
The seconds are `epoch-day * 86400 + second-of-day`, within
+-3.8e11, so nothing wraps; the floor is automatic because the
time of day is never negative.

[Source](/reference-sources/Data.Time.DateTime.cinch) · line 228

<a id="value-7765656b646179"></a>

## `weekday`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">weekday</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Weekday</span>.<span class="tok-constructor">Weekday</span>)</code></pre>

[Source](/reference-sources/Data.Time.DateTime.cinch)

<a id="value-776974682d64617465"></a>

## `with-date`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">with-date</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Date</span>.<span class="tok-constructor">Date</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span>))</code></pre>

[Source](/reference-sources/Data.Time.DateTime.cinch)

<a id="value-776974682d74696d65"></a>

## `with-time`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">with-time</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">TimeOfDay</span>.<span class="tok-constructor">TimeOfDay</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span>))</code></pre>

[Source](/reference-sources/Data.Time.DateTime.cinch)

<a id="value-79656172"></a>

## `year`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">year</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span> -&gt; (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">-9999</span> <span class="tok-number">9999</span> <span class="tok-constructor">I64</span>))</code></pre>

Fields, forwarded to the parts with their refined ranges.

[Source](/reference-sources/Data.Time.DateTime.cinch) · line 115

# Instances

These are checked exported instance entries. Law/sample-check status is separate from the existence of an instance; see [laws and optimization](../reference/12-rewrites-and-optimization.md).

| Instance | Target | Resolution |
| --- | --- | --- |
| `Data.Time.DateTime.Eq_DateTime` | `Relation.Eq.Eq Data.Time.DateTime.DateTime` | Implicit |
| `Data.Time.DateTime.Ord_DateTime` | `Relation.Ord.Ord Data.Time.DateTime.DateTime` | Implicit |

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Data/Time/DateTime.cinch`. Source SHA-256: `777d7c3bf5a8ecc3ed4f5c357eadd07019d85f5a77898a1c163e1ca33c9619ce`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
