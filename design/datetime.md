# Datetime: machine time, civil time, zones and spans

Status: specification, 2026-11-17, with every lane built and tested the
same day (section 11 records what landed and where it departs from the
text above it). Written at the user's
request ("spec out what needs doing and delegate all of the work ... We do
need a machine time and so forth as well"). The model is Jiff 1.0
(<https://burntsushi.net/jiff/>), which is itself a Rust rendering of
TC39 Temporal. This note says where Cinch follows Jiff, where the
no-faults rule and the type system let it do better, and what is built in
which lane. Companions: `design/no-faults.md`, `design/refinement-types.md`,
`design/wide-scalars.md`, `design/base-library.md`,
`design/stdlib-parity-ruby.md` (P38). No em dashes.

Decision numbers are assigned when the lanes land; this note uses T1..Tn
for its own decisions so they can be cited meanwhile.

## 1 Where Cinch stands today

`Data.Date`, `Data.Time` and `Effect.Clock` exist (P38, D345, ported from
Ruby's `Date`/`Time`). They are sound and tested
(`seed/tests/lib_civil_time.rs`) but they bake in four things this
design removes:

1. `Data.Time.Time` is a civil date-time that is also treated as an
   instant (`to-epoch-nanos`, "UTC-explicit"). A civil reading and an
   instant are different things; the conversion between them is exactly
   what a time zone is. Conflating them is the mistake Jiff exists to
   prevent.
2. `Effect.Clock.Duration` is an unsigned millisecond count. `Data.Time.diff`
   therefore drops direction and the sub-millisecond part, and
   `add-duration` cannot move the nanoseconds `Time` stores.
3. `Date` is three boxed bignums (`Int`, `Nat`, `Nat`), and `==` runs
   days-from-civil on both sides.
4. Fields are bare `Nat`; an impossible arm "reads 0" (`civil-field`,
   `days-in-month` on month 13) instead of being ruled out by a type.

T1. The existing three modules are NOT edited by this work. They are in
the bootstrap's recorded results and `Effect.Clock` is embedded in the
compiler. The new family is added beside them under `Data.Time.*`,
`Text.Time.*` and `Effect.Time*`, on disk and not in `cinch-base.cinchpkg`
(the same status `Data.Date` has today). Retiring `Data.Date`/`Data.Time`
and moving `Effect.Clock.Duration` onto the new `Duration` is an open
decision (section 10), not part of these lanes.

## 2 Ontology

Five kinds of thing, in layers. Each layer only knows the ones above it.

| Layer | Thing | Meaning | Knows about |
| --- | --- | --- | --- |
| L0 machine | `Duration` | an exact signed length of elapsed time, in nanoseconds | nothing |
| L0 machine | `Monotonic` | a reading of the monotonic clock; only differences mean anything | `Duration` |
| L0 machine | `Timestamp` | an instant: nanoseconds since the Unix epoch, no leap seconds | `Duration` |
| L1 civil | `Date`, `TimeOfDay`, `DateTime`, `Weekday` | what a calendar and a wall clock show, attached to no place | nothing |
| L2 zone | `Offset`, `TimeZone`, `Ambiguous` | the two mappings instant to offset and civil to (possibly ambiguous) offset | L0, L1 |
| L2 zone | `Zoned` | an instant in a place: `Timestamp` + `TimeZone`, with the civil reading cached | L0, L1 |
| L3 span | `Unit`, `RoundMode`, `Span` | a calendar-aware duration that keeps each unit's value | L0, L1, L2 |
| L4 text | printers and parsers | RFC 3339, RFC 9557, ISO 8601, friendly durations, strftime | all |
| L5 effect | `Effect.Time`, `Effect.Time.Zone` | reading clocks, the system zone, the system tz database | all |

"Machine time" is L0 plus its L5 readers: what a program needs to
measure, time out, stamp and compare without any calendar. It is usable
with no other layer loaded, and none of its operations allocates.

The two questions Jiff's users confuse, and the types that answer them:

- "How long did that take?" `Monotonic` minus `Monotonic` is a `Duration`.
  There is no way to turn a `Monotonic` into a date.
- "When did that happen?" A `Timestamp`. Showing it to a person needs a
  `TimeZone`, and that conversion is the only road from L0 to L1.

## 3 The rules that differ from Jiff

T2. **No operation faults** (`design/no-faults.md`). Jiff's `+`, `1.hour()`
and `civil::date` panic out of range. Here every operation is one of:

1. total because its argument types carry the evidence (a refined field,
   so a literal discharges at compile time);
2. `Option`/`Result` with a named error type per module;
3. total by a named convention: `saturating-` and `constrain` spellings.

There is no operator instance for datetime arithmetic. Machine scalars
have no `Add` instance for the same reason: the name states the policy.

T3. **Ranges.** Years are `-9999..=9999` as in Jiff. `Timestamp` covers
`-9999-01-02T01:59:59Z` to `9999-12-30T22:00:00.999999999Z`, so every
timestamp has a civil reading in every offset. `Offset` is within
`+-25:59:59`. The bounded range is what lets every value be a machine
scalar; the price is that arithmetic which can leave the range returns
`Option`.

T4. **Ambiguity is a value, not a policy.** Civil to instant returns

```cinch
data Ambiguous where
  Unambiguous :: Offset -> Ambiguous
  Gap :: Offset -> Offset -> Ambiguous   -- before, after: the reading does not exist
  Fold :: Offset -> Offset -> Ambiguous  -- before, after: the reading occurs twice
```

and `compatible`, `earlier`, `later` and `reject` are ordinary eliminators.
No operation applies a strategy silently: every function that may need
one either takes a `Disambiguation` argument or has the strategy in its
name. Zoned arithmetic carries it in its options value (Jiff makes you
drop to civil time and come back).

T5. **Zone lookup is not zone use.** Jiff's `in_tz("America/New_York")`
reads a database behind a string and merges "unknown zone" with "out of
range". Here a lookup is an effect (or a pure function of an explicit
`TzDb` value) that returns a `TimeZone`; everything after is pure and
has a narrower error type.

T6. **A span says whether it needs a calendar.** Jiff fails at run time
when a span with days or larger is compared or totalled without a
relative date. `Span` carries one index, `SpanClass` (`Clock` for hours
and smaller only, `Calendar` otherwise). Operations that need no anchor
accept only `Span Clock`; the anchored forms accept any class. If the
stage compiler's indexed families cannot carry this, the lane falls back
to an unindexed `Span` with a named `Result` error and records the
refusal in the module header.

T7. **Options are values built by functions.** Records have no update
syntax and every field must be named, so Temporal's option bags and
Jiff's tuple arguments do not translate. Each options type is abstract,
has one `-default` value and one setter per knob
(`difference-default |> with-largest Month |> with-smallest Day`). The
common cases get their own named functions so most callers never build
an options value.

T8. **Balancing is not rounding.** Jiff's `Span::round` with only
`largest` set does unit balancing. Here `span-balance` and `span-round`
are separate functions.

T9. **Strict parsing by default.** Jiff's `FromStr` accepts a date-time
when a date is asked for and drops the time. Each parser here accepts
exactly its grammar; lenient forms are separate, named functions.

T10. **Errors are sums.** One error type per module, constructors named
for the refusal (the convention `Data.Date` already follows).

T11. **Laws and refusals in the header.** Every module opens with its
design law shape and its named refusals, as `Data.Date` does.

## 4 Representations (performance)

Values are word slots; field unpacking is not available yet, so the hot
types are each ONE unboxed scalar behind an abstract constructor
(`#[repr(transparent)]` where the compiler admits it; the lane verifies
with the layout report and records what it found).

| Type | Representation | Why |
| --- | --- | --- |
| `Duration` | `I128` nanoseconds (`Data.Wide`) | unboxed, never counted, covers the whole timestamp range squared; compare is one `i128` compare |
| `Timestamp` | `I128` nanoseconds since the epoch | same; `Timestamp - Timestamp` is a subtraction |
| `Monotonic` | `I64` nanoseconds since the runtime origin | what the runtime returns, unconverted |
| `Date` | one `I64`, `year * 512 + month * 32 + day` (order preserving) | compare is a word compare, field access is a shift and mask, formatting needs no division by 146097 |
| `TimeOfDay` | `I64` nanoseconds since midnight, `0..=86399999999999` | one word |
| `DateTime` | `Date` and `TimeOfDay` (two fields, boxed until unpacking lands) | |
| `Offset` | `I64` seconds east of UTC | one word |
| `TimeZone` | fixed offset, or transitions in unboxed arrays (`I64` instants, `U8` type indices) plus a POSIX rule for the tail | binary search, no per-transition allocation |
| `Zoned` | `Timestamp`, `TimeZone`, cached `Offset` and `DateTime` | the civil reading is computed once at construction |
| `Span` | sign plus ten `I64` unit magnitudes | Jiff's layout |

If `I128` proves unusable for a lane (a missing lowering), the fallback
is a pair of `I64` seconds and `I64` subsecond nanoseconds in
`0..=999999999`, with the public API unchanged. `Int` and `Nat`
(bignums) do not appear in any representation and appear in the API
only at conversion functions.

All calendar arithmetic uses machine integers with the checked or
wrapping operation the algorithm's bounds justify, stated in a comment
at the use. Parsers read `Bytes` by index as explicit state machines
(no `unpack` to `List Char`); printers write through `Text.Builder` or a
preallocated `Bytes`.

## 5 Module map

On disk under `seed/prelude/`, not in the base package.

| Module | Lane | Exports (summary) |
| --- | --- | --- |
| `Data.Time.Duration` | A | `Duration`; from and to nanos, micros, millis, seconds, minutes, hours (checked and saturating); `add`, `sub`, `neg`, `abs`, `mul`, `div` by `NonZero`; `cmp`; `Eq`/`Ord`; `zero`, `is-negative` |
| `Data.Time.Monotonic` | A | `Monotonic`; `elapsed-between`, `add`; `Eq`/`Ord` |
| `Data.Time.Timestamp` | A | `Timestamp`; `min`/`max`/`epoch`; from and to Unix seconds, millis, micros, nanos (checked); `add`/`sub` a `Duration` (`Option`, `saturating-`); `since`/`until` giving a `Duration`; `cmp`; `Eq`/`Ord` |
| `Effect.Time` | A | `now :: Timestamp`, `monotonic-now :: Monotonic`, `elapsed`, `sleep-for`, `deadline-after`, all under the `Io r` capability, built on `Effect.Clock`'s readers |
| `Data.Time.Weekday` | B | `Weekday` (seven constructors); numbering conversions (Monday one, Sunday zero, both ways); `next`/`previous`; `days-until` |
| `Data.Time.Date` | B | `Date`; `date` (`Option`), `date-constrain` (total, clamps the day), refined-field accessors; `is-leap-year`, `days-in-month`, `days-in-year`; epoch-day conversions; `add-days` (`Option`), `diff-days`; `weekday`, `day-of-year`; `first-of-month`/`last-of-month`/`first-of-year`/`last-of-year`; `tomorrow`/`yesterday`; `nth-weekday-of-month`, `nth-weekday`; `min`/`max`; `Eq`/`Ord` |
| `Data.Time.TimeOfDay` | B | `TimeOfDay`; `time-of-day` (total over refined fields), `from-nanos-of-day` (`Option`); accessors; `midnight`; `add-nanos-wrapping` returning the day carry; `cmp`; `Eq`/`Ord` |
| `Data.Time.DateTime` | B | `DateTime`; construct from parts; accessors; `add-seconds-nanos` (`Option`); `diff-nanos`; to and from "epoch nanoseconds as if UTC" for the zone layer; `Eq`/`Ord` |
| `Data.Time.Offset` | C | `Offset`; `utc`; from hours, from seconds (`Option`), refined constructor; `to-seconds`; `timestamp-to-datetime`, `datetime-to-timestamp` at an offset |
| `Data.Time.Zone` | C | `TimeZone`; `fixed`, `utc`; `Ambiguous` and `Disambiguation` with eliminators; `offset-at`, `to-ambiguous`; `OffsetInfo` (offset, DST flag, abbreviation); `next-transition`/`previous-transition`; `name` |
| `Data.Time.Zone.Tzif` | C | `parse-tzif :: Bytes -> Result TzifError TimeZone` (RFC 9636 versions 1 to 4) |
| `Data.Time.Zone.Posix` | C | `parse-posix-tz :: String -> Result PosixTzError PosixRule`, `posix-offset-at`, `posix-to-ambiguous` |
| `Data.Time.Zoned` | C | `Zoned`; `from-timestamp`, `from-datetime` (takes `Disambiguation`); accessors; `with-time-zone`; `start-of-day`; `Eq` on instant and zone |
| `Effect.Time.Zone` | C | `TzDb`; `system-tzdb`, `lookup :: TzDb -> String -> Result TzLookupError TimeZone`; `system-time-zone`; `zoned-now` |
| `Data.Time.Unit` | D | `Unit` (Year to Nanosecond), `RoundMode` (the nine Temporal modes), increment validation, the shared rounding kernel over `I128` |
| `Data.Time.Span` | D | `Span`, `SpanClass`; unit builders and checked setters; accessors; `negate`, `abs`, `is-zero`; `fieldwise-eq`; `to-duration`/`from-duration` for `Span Clock`; `span-balance`, `span-round`, `span-total`, `span-compare`, anchored and unanchored |
| `Data.Time.Arithmetic` | D | `add-span`/`sub-span` on `Date`, `DateTime`, `Timestamp`; `until`/`since` with `Difference` options; `round` on `TimeOfDay`, `DateTime`, `Timestamp`; `Difference` and `Round` option values |
| `Data.Time.Zoned.Arithmetic` | F | the same on `Zoned`, with DST-aware days and a `Disambiguation` in the options |
| `Text.Time.Iso` | E | print and parse `Date`, `TimeOfDay`, `DateTime` (ISO 8601) and `Timestamp` (RFC 3339, print with a chosen offset) |
| `Text.Time.Duration` | E | ISO 8601 durations and the Jiff "friendly" format, for `Span` and `Duration`; the friendly printer's options |
| `Text.Time.Strtime` | E | `strftime`/`strptime` for the POSIX conversions Jiff supports, via a `BrokenDown` value |
| `Text.Time.Rfc9557` | F | print and parse `Zoned` (needs a `TzDb` to resolve the annotation; offset and zone conflict policy as in Temporal) |

## 6 Semantics that are ported, not redesigned

Arithmetic and rounding follow Temporal exactly; that is where years of
specification work live. In particular:

- Adding months or years constrains the day to the target month's length
  (`2024-05-31 + 1 month = 2024-06-30`); units are applied largest first.
- `until`/`since` default `largest` to days for dates and civil
  date-times, hours for `Zoned`, seconds for `Timestamp`; truncation is
  the default rounding mode for differences and half-expand for `round`.
- For `Zoned`, calendar units are added on the civil reading and
  re-resolved with the options' disambiguation; clock units are added on
  the instant. A day is the time between two consecutive civil midnights
  in that zone.
- The "compatible" strategy is the earlier offset in a fold and the later
  reading in a gap.

Test vectors come from the Jiff announcement's examples and from
Temporal's documented cases; each lane lists the ones it encodes.

## 7 Time zone data

T12. The system database (`/usr/share/zoneinfo`, TZif files) is read
through `Effect.Time.Zone` under the file capability. `TZ` and
`/etc/localtime` select the system zone.

T13. A build-time snapshot of the database (the staging `snapshot`
mechanism, so the tzdb version is a recorded, hashed, replayable build
input) is specified but NOT built in these lanes: staged `readFile`
returns `String` and TZif is binary. It needs either a staged binary
read or a generated text table. See section 10.

Tests ship a small set of TZif files as fixtures (the IANA database is
public domain): `America/New_York`, `Europe/Amsterdam`, `Europe/London`,
`Australia/Sydney`, `Asia/Kolkata`, `Pacific/Apia`, `America/Santiago`,
`UTC`.

## 8 Lanes and order

Phase 1, in parallel: **A** machine time, **B** civil.
Phase 2, in parallel, after phase 1: **C** zones, **D** spans and
arithmetic, **E** text.
Phase 3: **F** zoned arithmetic and RFC 9557, then integration.

Each lane delivers its modules, a test file
`seed/tests/lib_time_<lane>.rs` in the shape of
`lib_civil_time.rs` (real module sources beside a `Main`, `check-*`
functions summed into an exit bitmask), and a short report of every place
the language or compiler refused what this note asks for.

## 9 Acceptance

- Every listed module compiles and its test file passes under
  `cargo test --test lib_time_<lane>`; `lib_civil_time` still passes.
- No module imports `Data.Date`, `Data.Time` or redefines `Duration`
  from `Effect.Clock`.
- No `Int`/`Nat` in a representation; no `List Char` in a parser.
- Each module header states its laws and named refusals.

## 10 Open decisions (for the user)

1. Retire `Data.Date` and `Data.Time` in favour of this family, and move
   `Effect.Clock.Duration`, `timeout` and `ioSleep` onto
   `Data.Time.Duration`? Both touch the bootstrap's recorded inputs.
2. Embed the family in the base package once it settles?
3. The tzdb snapshot (T13): staged binary read, or a generated table?
4. A staged date literal that checks `2024-02-29` at compile time, if the
   refinement vocabulary cannot decide a day bound that depends on the
   year and month.
5. Default field values or record update in the language would remove
   the setter boilerplate T7 requires; this family is evidence for it.
6. Locale-aware formatting and non-Gregorian calendars are out of scope,
   as in Jiff (which delegates both to ICU4X).
7. Leap seconds are not modelled (Unix time), as in Jiff.

## 11 As built (2026-11-17)

All lanes landed on `lane/datetime`: 26 modules, about 13,800 lines, 69
fixture programs, eight Rust suites (`seed/tests/lib_time_{a..h}.rs`),
all passing, as does the untouched `lib_civil_time.rs`. Where this section
and the sections above disagree, this section is what exists.

### 11.1 Modules beyond the map in section 5

| Module | Lane | Why |
| --- | --- | --- |
| `Data.Time.Zone.Types` | C | `OffsetInfo`, `Ambiguous`, `Disambiguation` and the resolvers, shared by `Zone` and `Zone.Posix`; re-exported whole by `Data.Time.Zone` |
| `Text.Time.Scan` | E | byte cursor, digit readers and writers shared by every text module |
| `Text.Time.Zoned` | G | `Offset`- and `Zoned`-typed wrappers over `Iso` and `Strtime`, zoned `strftime`, and the effectful parse, look up, resolve conveniences (so `Rfc9557` stays pure) |
| `Text.Time.Rfc2822` | G | RFC 2822 and 5322 date-times |

`Text.Time.Rfc9557` was built by lane G and `Text.Time.Duration` by lane H
(the map lists them under F and E).

### 11.2 Naming as delivered

- The unit type is `TimeUnit` (`Unit` is a builtin); the module is still
  `Data.Time.Unit`.
- Lanes A, C, D, F, G and H export type-prefixed names
  (`duration-from-seconds`, `zone-offset-at`, `span-negate`) and can be
  imported unqualified together. Lane B (`Weekday`, `Date`, `TimeOfDay`,
  `DateTime`) exports short names (`cmp`, `add-days`, `year`) and must be
  imported qualified. Unifying the two is a rename-only follow-up.
- Representation constructors are named `MkDuration`, `MkDate` and so
  on, and are not exported (11.4, item 1).

### 11.3 Departures from the specification

1. **Values are boxed.** Each hot type is one scalar as section 4 says,
   but a one-field `data` allocates today even under
   `#[repr(transparent)]`, so "none of its operations allocates" (section
   2) does not hold yet. The representations are kept so that they become
   free when field unpacking lands.
2. **Refined results use a clamp or branch that never fires**, because
   machine arithmetic is opaque to the refinement checker; and there is
   no total machine division, so private `quot`/`rem` helpers carry a
   dead `None` arm. Each is commented at the use.
3. **No `TZ`, `TZDIR` or `/etc/localtime` symlink target**: the prelude
   has no environment reader and no `readlink`. The system zone comes
   from `/etc/timezone`, else the bytes of `/etc/localtime` (named
   `Etc/Unknown`), else UTC.
4. **A day needs an anchor.** `Span` treats a day as a calendar unit
   everywhere (Jiff and T6); Temporal's 24-hour day without `relativeTo`
   is offered only as the named convention `span-to-duration-civil-days`.
5. **Temporal fidelity.** Lane D was checked against the reference
   polyfill 0.5.1 on about ten thousand generated vectors. The current
   specification text is newer than that polyfill in three places (the
   rounding window shift, the same-civil-date difference, and the day
   correction limit); both engines follow the current text, and
   `tests/fixtures/time/f/civil_agreement.cinch` checks that the civil and
   zoned engines agree in fixed-offset zones.
6. **Two deliberate departures from the letter of Temporal**, each
   documented at its function: NudgeToDayOrTime reports an expansion only
   when a day was crossed (so a whole-range difference is not refused),
   and the zoned nudge starts from the origin instant only when the whole
   start candidate is zero.
7. **Lossy ISO durations.** ISO 8601 has no sub-second designators, so
   2000 milliseconds prints `PT2S`. The friendly format is lossless and
   is the one to store.
8. **Not built:** `zoned-hours-in-day` (no floats in the family; use
   `zoned-day-length`), `months-between`, Jiff's `HumanTime` designator
   style, military RFC 2822 zones, and the tzdb snapshot (T13).

### 11.4 What the lanes found in the compiler and prelude

1. `export T` also exports a constructor named `T`, so `Data.Date`'s
   "private" constructor is not private. A `record` exported by type name
   is constructible and projectable by importers; there is no abstract
   record.
2. `#[repr(transparent)]` changes the recorded layout but not code
   generation (the interface says size 8, the IR allocates); over `I128`
   it is refused outright.
3. `fs-read` on a directory faults the process
   (`exhausted: heap (9223372036854779213 bytes)`), which breaks the
   no-faults rule; `Effect.Time.Zone` guards with `fs-metadata`.
4. A run-time segfault that compiles clean: an `I128` as the eighth of
   eleven parameters of a `tailcc` function (`fr-designator` in
   `Text.Time.Duration`); moving it earlier fixes it. `cinch run` reports
   the crash as a silent exit 1.
5. A `forall (c :: SpanClass).` index is passed at run time: callers
   build and drop a `Clock()` value nothing reads.
6. Top-level constants of constructor type are rebuilt at every use.
7. A literal in the first branch of an `if` inside a `let` defaults to
   `Int` even when used at `I64`.
8. Refined types inside tuple components check only for literals.
9. Missing prelude pieces this family wanted: `NonZero`-divisor division
   for machine integers and `I128`; an `I64`-indexed `bytes-get` and a
   read-only byte view of `String`; big-endian `Bytes` word readers; an
   `Option`-free array read; `Eq` for `Ordering` and for `Option`; `I64`
   clock readers on `Effect.Clock`; an environment reader and `readlink`.
10. Reserved words with misleading diagnostics when used as names: `to`,
    `narrow`, `target`, `end`. `name-123` does not lex. `&& case` does not
    parse. There is no `|>`.
11. `Text.Builder`, `Data.Array` and `Data.Array.Packed` are not in the
    base package, so an on-disk family cannot import them without
    shipping them; `Data.Time.Zone` binds the array runtime entry points
    privately.

### 11.5 Open decisions added by the build

8. One Temporal corner looks like a specification defect and both
   engines follow it: 2024-02-29T12:00 until 2025-03-01T11:00 with
   largest Year, smallest Month and `Ceil` is 1 year, 23 hours short of
   the destination (with largest Month it is 13 months). Follow the
   letter, or bubble differently?
9. The website's generated library reference
   (`website/scripts/generate-library-reference.py`) does not yet list
   this family.
