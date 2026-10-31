# Effect.Benchmark

Benchmark case execution and timing/reporting helpers over runtime clocks.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Effect.Benchmark`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Effect.Benchmark.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.List](Data.List.md), [Data.Nat](Data.Nat.md), [Data.String](Data.String.md), [Effect.Clock](Effect.Clock.md), [Text.Show](Text.Show.md).

Non-embedded source closure: [Effect.Benchmark](Effect.Benchmark.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Effect.Benchmark: timing `IO r` actions against the runtime's
monotonic clock, the Ruby `Benchmark` surface (the tooling entry in
design/stdlib-parity-ruby.md, ported onto the P34 `Effect.Clock`).

The clock is the monotonic reading only (`clock-monotonic-ns`, never
`clock-wall-ns`), so an interval is a natural number of nanoseconds
and never runs backwards. As with Ruby's `Benchmark`, no attempt is
made to subtract the measurement overhead (the two clock reads and
the report record): an interval is the raw span around the action,
and two measurements are comparable because they pay the same
overhead.

Design law shape:
  value transparency: `benchmark-measure label work` returns
    `work`'s value unchanged (timing never consumes or alters the
    timed result);
  non-negativity: the monotonic clock is non-decreasing, so the
    elapsed nanoseconds (and `benchmark-realtime`'s `Duration`) are
    always non-negative;
  agreement: `benchmark-realtime` and `benchmark-measure` read the
    same interval around the same action (the former truncates to
    whole milliseconds), and `benchmark-table`'s total is the sum
    of its rows;
  ordering only: measurements are compared to each other, never to
    an absolute wall-clock budget.

Named refusals (boundaries this module deliberately does not cross):
  overhead subtraction: not attempted (Ruby does not either); the
    clock reads stay part of the interval;
  absolute-duration claims: the module offers no thresholds or
    budgets, and its tests order measurements rather than pin
    absolute times;
  wall-clock and calendar: only the monotonic clock is consulted;
    dates and formatting are the Time family's business
    (Effect.Clock's header), not here;
  statistics: no minimum/maximum/mean/deviation aggregation and no
    Ruby `Benchmark.bmbm` warm-up pass; a report is one interval and
    the table adds the rows up;
  compile-time readings: `Effect.Clock`'s refusal (D45): the timing
    operations need the `Io r` capability and cannot run in a
    `stage` body.

Module scheme (design/base-library.md): `Effect.*` is the effect
system's vocabulary, and timing is a runtime capability in the
`Io r` mold (D45), so the home is Effect.

</details>

# Types and constructors

<a id="type-42656e63686d61726b5265706f7274"></a>

## `BenchmarkReport`

```cinch
data BenchmarkReport
```

One measurement: the label the caller supplied and the elapsed
monotonic nanoseconds around the action. The constructor is the
smart surface's only source of reports, so a report's interval is
always one this module read.

[Source](/reference-sources/Effect.Benchmark.cinch) · line 62

```cinch
BenchmarkReport :: (String -> (Nat -> BenchmarkReport))
```

One measurement: the label the caller supplied and the elapsed
monotonic nanoseconds around the action. The constructor is the
smart surface's only source of reports, so a report's interval is
always one this module read.

# Values

<a id="value-62656e63686d61726b2d656c61707365642d6e73"></a>

## `benchmark-elapsed-ns`

```cinch
benchmark-elapsed-ns :: (BenchmarkReport -> Nat)
```

[Source](/reference-sources/Effect.Benchmark.cinch) · line 69

<a id="value-62656e63686d61726b2d6c6162656c"></a>

## `benchmark-label`

```cinch
benchmark-label :: (BenchmarkReport -> String)
```

[Source](/reference-sources/Effect.Benchmark.cinch) · line 65

<a id="value-62656e63686d61726b2d6d656173757265"></a>

## `benchmark-measure`

```cinch
benchmark-measure :: forall a r . using (io :: Io r) => (String -> ((Unit ->{IO r} a) ->{IO r} (BenchmarkReport, a)))
```

Run `work` and report the monotonic span around it, together with
`work`'s own value so a measurement never discards the result. The
elapsed is `t1 - t0`; the monotonic clock guarantees `t1 >= t0`,
but that is the OS's promise, not a fact the checker sees, so the
span clamps at zero (a duration is never negative).

[Source](/reference-sources/Effect.Benchmark.cinch) · line 79

<a id="value-62656e63686d61726b2d7265616c74696d65"></a>

## `benchmark-realtime`

```cinch
benchmark-realtime :: forall a r . using (io :: Io r) => ((Unit ->{IO r} a) ->{IO r} Duration)
```

The same interval as `benchmark-measure`, truncated to whole
milliseconds (`Effect.Clock`'s `monotonic-elapsed`); the action's
value is deliberately dropped, so callers wanting it use
`benchmark-measure`.

[Source](/reference-sources/Effect.Benchmark.cinch) · line 90

<a id="value-62656e63686d61726b2d7461626c65"></a>

## `benchmark-table`

```cinch
benchmark-table :: (List BenchmarkReport -> String)
```

The table: a header, one row per report in input order, and the
total row. Labels are the caller's, printed verbatim. The rows are
flattened once: nesting `string-concat` would re-copy the rest of the
table at every row.

[Source](/reference-sources/Effect.Benchmark.cinch) · line 102

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Effect/Benchmark.cinch`. Source SHA-256: `59fcce379631b8530523a7370db82c170c0e5d6e0a7b5961e80fd270883e8ba3`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
