# Data.Time.Date

Civil calendar dates with validated construction, weekday and ordinal queries, and day arithmetic.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Data.Time.Date`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Time.Date.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Algebra.Arithmetic](Algebra.Arithmetic.md), [Data.Machine](Data.Machine.md), [Data.Option](Data.Option.md), [Data.Ordering](Data.Ordering.md), [Data.Time.Weekday](Data.Time.Weekday.md), [Relation.Eq](Relation.Eq.md), [Relation.Ord](Relation.Ord.md).

Non-embedded source closure: [Data.Time.Date](Data.Time.Date.md), [Data.Time.Weekday](Data.Time.Weekday.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Data.Time.Date: a civil calendar date.

A `Date` is a day in the proleptic Gregorian calendar, attached to no
place and no instant: a year in -9999..=9999 (astronomical numbering,
so year 0 exists and is a leap year), a month in 1..=12 and a day in
1..=that month's length in that year. There are no leap seconds and
no reform cutover (design/datetime.md T3).

Representation. One `I64` word behind a private constructor:

    word = year * 512 + month * 32 + day

The year is NOT biased: for a negative year the word is negative,
and because `month * 32 + day` is in 33..=415 (below 512) the year is
exactly the arithmetic shift `word >> 9` (which floors) and the low
nine bits are `month * 32 + day` in two's complement for either sign.
The encoding is order preserving: the year is the most significant
part, then the month, then the day, and each lower part is smaller
than the weight of the part above it, so comparing two dates is
comparing two words (`cmp`, `Eq`, `Ord` are one machine compare and
never run the calendar). Field access is a shift and a mask, so a
printer needs no division by 146097.

How the value is held (verified against the emitted LLVM, see NAMED
REFUSALS): the declaration carries `#[repr(transparent)]`, which the
compiler admits on a one-field `data` and records in the interface as
size 8, align 8, but the code generator still holds a `Date` as a
reference to a one-field heap object. The word is unboxed inside that
object; the `Date` itself is not yet a bare scalar.

The constructor is not exported, so every `Date` came through `date`,
`date-constrain`, `date-of`, `from-epoch-days` or arithmetic on a
valid date, and an invalid date (a 30 February, a month 13, a year
10000) has no representation.

Design law shape. One predicate, `is-leap-year` (divisible by 4 and
not by 100 unless by 400), fixes every month length
(`days-in-month`) and every year length (`days-in-year`). On top of
it:
  - `to-epoch-days` is strictly increasing in the word order and
    `from-epoch-days` inverts it on `min-epoch-day..=max-epoch-day`
    (1970-01-01 is day 0); outside that interval `from-epoch-days` is
    `None`.
  - `add-days n d` is `from-epoch-days (to-epoch-days d + n)`, so
    `diff-days d e` (finish minus start) is the `n` with
    `add-days n d == Some e`, and adding m then n days is adding
    m + n whenever every step stays in range.
  - `tomorrow`/`yesterday` are `add-days 1`/`add-days (-1)`.
  - `weekday` is the epoch day read modulo 7 with 1970-01-01 a
    Thursday, so consecutive days have consecutive weekdays.
  - `add-months-constrain` and `add-years-constrain` follow Temporal's
    "constrain" overflow: the year and month move exactly, and the day
    is clamped down to the target month's length
    (2024-05-31 + 1 month = 2024-06-30, 2024-02-29 + 1 year =
    2025-02-28). They are `None` only when the year leaves the range.
  - `day-of-year` is 1 on 1 January and `days-in-year` on 31 December.
  - `nth-weekday-of-month` and `nth-weekday` are Jiff's
    `nth_weekday_of_month` and `nth_weekday`.
  - `iso-week-year`/`iso-week` are ISO 8601 week dates (week 1 is the
    week holding the year's first Thursday).

No operation faults (T2): each is total over refined arguments
(`date-of`, `date-constrain`, `days-in-month`), total by a named
convention (`-constrain`), or answers `Option`.

NAMED REFUSALS:
  - an unboxed `Date`: `#[repr(transparent)]` is accepted and
    recorded, but the emitted code allocates a one-field object for
    the constructor and loads the word through it (`#[no-alloc]` on
    a function that builds a `Date` is refused with "the constructor
    `MkDate` is constructed here"). A `record` is held the same way
    and, unlike a `data`, lets any importer write
    `Date { word = ... }`, so a `data` with a private constructor is
    used. Nothing in the API depends on the box.
  - a total literal constructor for days 29..=31: the refinement
    vocabulary is comparisons over one value, so "day <= the length of
    this month in this year" cannot be a parameter's type. `date-of`
    is total for days 1..=28 (valid in every month); other days go
    through `date` (`Option`) or `date-constrain` (clamps). A staged
    date literal is design/datetime.md's open decision 4.
  - refined results by proof: machine integer arithmetic is opaque to
    the checker, so `year`, `month`, `day`, `day-of-year` and
    `iso-week` establish their declared ranges with a clamp. The
    clamp never changes a value (the representation invariant puts
    every field in range already); it is how the range reaches the
    type.
  - a total machine division: `Data.Machine` has only
    `checked-div-i64`/`checked-rem-i64`. The private `quot`/`rem`
    take the divisor at `InRange 2 9223372036854775807 I64`, which
    excludes both `None` cases (zero, and `min / -1`), and still must
    match the `None` arm.
  - text: no printer or parser lives here (Text.Time.Iso).
  - `Int`/`Nat`: every quantity is an `I64`.

</details>

# Types and constructors

<a id="type-44617465"></a>

## `Date`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Date</span></code></pre>

The packed word (header). Not exported: the validity invariant.

[Source](/reference-sources/Data.Time.Date.cinch) · line 121

# Values

<a id="value-6164642d64617973"></a>

## `add-days`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">add-days</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Date</span>.<span class="tok-constructor">Date</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Date</span>.<span class="tok-constructor">Date</span>)))</code></pre>

`n` days later (earlier for a negative `n`). `None` when the result
leaves the range. The sum is a checked addition because `n` is any
`I64`; an overflowing sum is out of range a fortiori.

[Source](/reference-sources/Data.Time.Date.cinch) · line 240

<a id="value-6164642d6d6f6e7468732d636f6e73747261696e"></a>

## `add-months-constrain`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">add-months-constrain</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Date</span>.<span class="tok-constructor">Date</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Date</span>.<span class="tok-constructor">Date</span>)))</code></pre>

`n` months later (earlier for a negative `n`): the year and month
move exactly, and the day is clamped down to the target month's
length. `None` only when the year leaves -9999..=9999.

Overflow argument: the date's month index `year * 12 + month - 1`
is within +-120000. `n` is any `I64`, so the sum is a checked
addition, and the sum is then tested against the range's own month
indexes (-119988..=119999) before anything else is computed from
it; after the test every intermediate is below 2^18.

[Source](/reference-sources/Data.Time.Date.cinch) · line 288

<a id="value-6164642d79656172732d636f6e73747261696e"></a>

## `add-years-constrain`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">add-years-constrain</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Date</span>.<span class="tok-constructor">Date</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Date</span>.<span class="tok-constructor">Date</span>)))</code></pre>

`n` years later (earlier for a negative `n`), same month; the day
is clamped down to that month's length in the target year, which
only ever turns 29 February into 28 February. `None` only when the
year leaves the range (a checked addition, because `n` is any
`I64`).

[Source](/reference-sources/Data.Time.Date.cinch) · line 305

<a id="value-636d70"></a>

## `cmp`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cmp</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Date</span>.<span class="tok-constructor">Date</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Date</span>.<span class="tok-constructor">Date</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Ordering</span>.<span class="tok-constructor">Ordering</span>))</code></pre>

Order: a word compare (see the header).

[Source](/reference-sources/Data.Time.Date.cinch) · line 416

<a id="value-64617465"></a>

## `date`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">date</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Date</span>.<span class="tok-constructor">Date</span>))))</code></pre>

The checked constructor: year, month, day. `None` for a year
outside -9999..=9999, a month outside 1..=12, or a day outside
1..=the month's length in that year.

[Source](/reference-sources/Data.Time.Date.cinch) · line 131

<a id="value-646174652d636f6e73747261696e"></a>

## `date-constrain`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">date-constrain</span> :: ((<span class="tok-variable">y</span> :: (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">-9999</span> <span class="tok-number">9999</span> <span class="tok-constructor">I64</span>)) -&gt; ((<span class="tok-variable">m</span> :: (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">1</span> <span class="tok-number">12</span> <span class="tok-constructor">I64</span>)) -&gt; (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Date</span>.<span class="tok-constructor">Date</span>)))</code></pre>

Temporal's "constrain" overflow for the day: the year and month are
taken as given (their ranges are in their types, so a literal
discharges at compile time) and the day is clamped into
1..=the month's length. `date-constrain 2023 2 31` is 2023-02-28;
a day below 1 is the 1st.

[Source](/reference-sources/Data.Time.Date.cinch) · line 143

<a id="value-646174652d6f66"></a>

## `date-of`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">date-of</span> :: ((<span class="tok-variable">y</span> :: (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">-9999</span> <span class="tok-number">9999</span> <span class="tok-constructor">I64</span>)) -&gt; ((<span class="tok-variable">m</span> :: (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">1</span> <span class="tok-number">12</span> <span class="tok-constructor">I64</span>)) -&gt; ((<span class="tok-variable">d</span> :: (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">1</span> <span class="tok-number">28</span> <span class="tok-constructor">I64</span>)) -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Date</span>.<span class="tok-constructor">Date</span>)))</code></pre>

The total constructor for the days every month has. All three
ranges are in the argument types, so `date-of 2024 2 15` checks
with nothing else written and a bad literal is refused at compile
time.

[Source](/reference-sources/Data.Time.Date.cinch) · line 154

<a id="value-646179"></a>

## `day`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">day</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Date</span>.<span class="tok-constructor">Date</span> -&gt; (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">1</span> <span class="tok-number">31</span> <span class="tok-constructor">I64</span>))</code></pre>

[Source](/reference-sources/Data.Time.Date.cinch)

<a id="value-6461792d6f662d79656172"></a>

## `day-of-year`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">day-of-year</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Date</span>.<span class="tok-constructor">Date</span> -&gt; (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">1</span> <span class="tok-number">366</span> <span class="tok-constructor">I64</span>))</code></pre>

1 on 1 January, `days-in-year` on 31 December.

[Source](/reference-sources/Data.Time.Date.cinch) · line 323

<a id="value-646179732d696e2d6d6f6e7468"></a>

## `days-in-month`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">days-in-month</span> :: (<span class="tok-constructor">I64</span> -&gt; ((<span class="tok-variable">m</span> :: (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">1</span> <span class="tok-number">12</span> <span class="tok-constructor">I64</span>)) -&gt; (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">28</span> <span class="tok-number">31</span> <span class="tok-constructor">I64</span>)))</code></pre>

The length of month `m` in year `y`: 28, 29, 30 or 31. The month's
range is in its type, so there is no out-of-calendar case.

[Source](/reference-sources/Data.Time.Date.cinch) · line 200

<a id="value-646179732d696e2d6d6f6e74682d6f66"></a>

## `days-in-month-of`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">days-in-month-of</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Date</span>.<span class="tok-constructor">Date</span> -&gt; (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">28</span> <span class="tok-number">31</span> <span class="tok-constructor">I64</span>))</code></pre>

[Source](/reference-sources/Data.Time.Date.cinch)

<a id="value-646179732d696e2d79656172"></a>

## `days-in-year`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">days-in-year</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">365</span> <span class="tok-number">366</span> <span class="tok-constructor">I64</span>))</code></pre>

[Source](/reference-sources/Data.Time.Date.cinch)

<a id="value-646179732d696e2d796561722d6f66"></a>

## `days-in-year-of`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">days-in-year-of</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Date</span>.<span class="tok-constructor">Date</span> -&gt; (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">365</span> <span class="tok-number">366</span> <span class="tok-constructor">I64</span>))</code></pre>

[Source](/reference-sources/Data.Time.Date.cinch)

<a id="value-646966662d64617973"></a>

## `diff-days`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">diff-days</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Date</span>.<span class="tok-constructor">Date</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Date</span>.<span class="tok-constructor">Date</span> -&gt; <span class="tok-constructor">I64</span>))</code></pre>

`diff-days start finish`: the days from `start` to `finish`,
finish minus start, negative when `finish` is earlier. Both epoch
days are within +-4371588, so the difference is within +-7304484
and the wrapping subtraction never wraps.

[Source](/reference-sources/Data.Time.Date.cinch) · line 249

<a id="value-66697273742d6f662d6d6f6e7468"></a>

## `first-of-month`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">first-of-month</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Date</span>.<span class="tok-constructor">Date</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Date</span>.<span class="tok-constructor">Date</span>)</code></pre>

[Source](/reference-sources/Data.Time.Date.cinch)

<a id="value-66697273742d6f662d79656172"></a>

## `first-of-year`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">first-of-year</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Date</span>.<span class="tok-constructor">Date</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Date</span>.<span class="tok-constructor">Date</span>)</code></pre>

[Source](/reference-sources/Data.Time.Date.cinch)

<a id="value-66726f6d2d65706f63682d64617973"></a>

## `from-epoch-days`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">from-epoch-days</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Date</span>.<span class="tok-constructor">Date</span>))</code></pre>

The inverse: `None` outside `min-epoch-day..=max-epoch-day`.

[Source](/reference-sources/Data.Time.Date.cinch) · line 232

<a id="value-696e2d6c6561702d79656172"></a>

## `in-leap-year`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">in-leap-year</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Date</span>.<span class="tok-constructor">Date</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

[Source](/reference-sources/Data.Time.Date.cinch)

<a id="value-69732d6c6561702d79656172"></a>

## `is-leap-year`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">is-leap-year</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

The Gregorian leap rule, total over `I64` (the proleptic calendar
defines it for every year, including 0 and the negative ones).
Divisibility does not depend on the remainder's sign, so the
truncated remainder serves for negative years too.

[Source](/reference-sources/Data.Time.Date.cinch) · line 192

<a id="value-69736f2d7765656b"></a>

## `iso-week`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">iso-week</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Date</span>.<span class="tok-constructor">Date</span> -&gt; (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">1</span> <span class="tok-number">53</span> <span class="tok-constructor">I64</span>))</code></pre>

The ISO week number, 1..=53.

[Source](/reference-sources/Data.Time.Date.cinch) · line 404

<a id="value-69736f2d7765656b2d79656172"></a>

## `iso-week-year`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">iso-week-year</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Date</span>.<span class="tok-constructor">Date</span> -&gt; (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">-9999</span> <span class="tok-number">9999</span> <span class="tok-constructor">I64</span>))</code></pre>

The ISO week-numbering year. It differs from `year` only in the
first days of January and the last days of December. It stays in
-9999..=9999 because the range begins on a Monday (-9999-01-01
opens week 1 of -9999) and ends on a Friday (9999-12-31 is in week
52 of 9999), so neither end borrows a week from a year outside the
range; the clamp states that in the type and never fires.

[Source](/reference-sources/Data.Time.Date.cinch) · line 398

<a id="value-6c6173742d6f662d6d6f6e7468"></a>

## `last-of-month`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">last-of-month</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Date</span>.<span class="tok-constructor">Date</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Date</span>.<span class="tok-constructor">Date</span>)</code></pre>

[Source](/reference-sources/Data.Time.Date.cinch)

<a id="value-6c6173742d6f662d79656172"></a>

## `last-of-year`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">last-of-year</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Date</span>.<span class="tok-constructor">Date</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Date</span>.<span class="tok-constructor">Date</span>)</code></pre>

[Source](/reference-sources/Data.Time.Date.cinch)

<a id="value-6d61782d64617465"></a>

## `max-date`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">max-date</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Date</span>.<span class="tok-constructor">Date</span></code></pre>

[Source](/reference-sources/Data.Time.Date.cinch)

<a id="value-6d61782d65706f63682d646179"></a>

## `max-epoch-day`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">max-epoch-day</span> :: <span class="tok-constructor">I64</span></code></pre>

[Source](/reference-sources/Data.Time.Date.cinch)

<a id="value-6d696e2d64617465"></a>

## `min-date`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">min-date</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Date</span>.<span class="tok-constructor">Date</span></code></pre>

-9999-01-01 and 9999-12-31.

[Source](/reference-sources/Data.Time.Date.cinch) · line 158

<a id="value-6d696e2d65706f63682d646179"></a>

## `min-epoch-day`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">min-epoch-day</span> :: <span class="tok-constructor">I64</span></code></pre>

to-epoch-days min-date and to-epoch-days max-date.

[Source](/reference-sources/Data.Time.Date.cinch) · line 220

<a id="value-6d6f6e7468"></a>

## `month`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">month</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Date</span>.<span class="tok-constructor">Date</span> -&gt; (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">1</span> <span class="tok-number">12</span> <span class="tok-constructor">I64</span>))</code></pre>

[Source](/reference-sources/Data.Time.Date.cinch)

<a id="value-6e74682d7765656b646179"></a>

## `nth-weekday`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">nth-weekday</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Weekday</span>.<span class="tok-constructor">Weekday</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Date</span>.<span class="tok-constructor">Date</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Date</span>.<span class="tok-constructor">Date</span>))))</code></pre>

The `n`th `wd` strictly after the date for a positive `n`, strictly
before it for a negative `n` (Jiff's `nth_weekday`): from Wednesday
2024-07-10, `nth-weekday 1 Wednesday` is 2024-07-17 and
`nth-weekday (-1) Wednesday` is 2024-07-03. `None` when `n` is 0 or
the answer leaves the range.

Overflow argument: the whole range holds 7304484 days, fewer than
1043500 weeks, so an `n` beyond +-1043500 cannot land in range and
is refused before any multiplication; within that bound
`(|n| - 1) * 7 + 7` is below 2^23.

[Source](/reference-sources/Data.Time.Date.cinch) · line 373

<a id="value-6e74682d7765656b6461792d6f662d6d6f6e7468"></a>

## `nth-weekday-of-month`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">nth-weekday-of-month</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Weekday</span>.<span class="tok-constructor">Weekday</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Date</span>.<span class="tok-constructor">Date</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Date</span>.<span class="tok-constructor">Date</span>))))</code></pre>

The `n`th `wd` of the date's month (Jiff's `nth_weekday_of_month`).
`n` in 1..=5 counts from the start of the month (1 is the first
`wd`), `n` in -5..=-1 from its end (-1 is the last `wd`). `None`
when `n` is 0 or outside +-5, or when the month has no such
occurrence (a fifth Monday in a month with four).

[Source](/reference-sources/Data.Time.Date.cinch) · line 348

<a id="value-746f2d65706f63682d64617973"></a>

## `to-epoch-days`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">to-epoch-days</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Date</span>.<span class="tok-constructor">Date</span> -&gt; <span class="tok-constructor">I64</span>)</code></pre>

The day number counted from 1970-01-01, negative before it; in
`min-epoch-day..=max-epoch-day`.

[Source](/reference-sources/Data.Time.Date.cinch) · line 228

<a id="value-746f6d6f72726f77"></a>

## `tomorrow`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tomorrow</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Date</span>.<span class="tok-constructor">Date</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Date</span>.<span class="tok-constructor">Date</span>))</code></pre>

The next day; `None` only after 9999-12-31. Within a month this is
the word plus one (the day is the word's lowest field).

[Source](/reference-sources/Data.Time.Date.cinch) · line 254

<a id="value-7765656b646179"></a>

## `weekday`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">weekday</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Date</span>.<span class="tok-constructor">Date</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Weekday</span>.<span class="tok-constructor">Weekday</span>)</code></pre>

1970-01-01 (day 0) is a Thursday, three days after a Monday, so
the weekday is Monday moved by `epoch-day + 3` (within +-4371591;
`W.add-days` reduces modulo 7 for either sign).

[Source](/reference-sources/Data.Time.Date.cinch) · line 319

<a id="value-79656172"></a>

## `year`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">year</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Date</span>.<span class="tok-constructor">Date</span> -&gt; (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">-9999</span> <span class="tok-number">9999</span> <span class="tok-constructor">I64</span>))</code></pre>

Fields. Each is a shift or a mask of the word; the clamp states
the range in the type and never fires (see NAMED REFUSALS).

[Source](/reference-sources/Data.Time.Date.cinch) · line 169

<a id="value-796573746572646179"></a>

## `yesterday`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">yesterday</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Date</span>.<span class="tok-constructor">Date</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Date</span>.<span class="tok-constructor">Date</span>))</code></pre>

The previous day; `None` only before -9999-01-01.

[Source](/reference-sources/Data.Time.Date.cinch) · line 265

# Instances

These are checked exported instance entries. Law/sample-check status is separate from the existence of an instance; see [laws and optimization](../reference/12-rewrites-and-optimization.md).

| Instance | Target | Resolution |
| --- | --- | --- |
| `Data.Time.Date.Eq_Date` | `Relation.Eq.Eq Data.Time.Date.Date` | Implicit |
| `Data.Time.Date.Ord_Date` | `Relation.Ord.Ord Data.Time.Date.Date` | Implicit |

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Data/Time/Date.cinch`. Source SHA-256: `dd1b92039b8c4d39ee8c82843e5eff46a1b324e51b3c8b917c309733227bfc0d`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
