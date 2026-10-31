# Effect.Test

A subprocess-based test runner and named test-result/report vocabulary.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Effect.Test`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Effect.Test.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bool](Data.Bool.md), [Data.List](Data.List.md), [Data.Option](Data.Option.md), [Data.String](Data.String.md), [Relation.Eq](Relation.Eq.md), [Text.Show](Text.Show.md).

Non-embedded source closure: [Effect.Test](Effect.Test.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Effect.Test: the Minitest-shaped test runner (the bootstrap's own
test vocabulary).

What this module is: the runner the bootstrap needs to run its own
tests (design/stdlib-and-bootstrap.md, "Also needed for
bootstrapping": "a Cinch test runner"). Ruby's `Minitest` is the
design reference (design/stdlib-parity-ruby.md, P40: "the Cinch
test runner (Minitest-shaped)"), and this module ports its spirit
as a library: a `TestCase` carries a name and a pure body returning
the case's checks, `assert`/`assert-eq`/`refute`/`refute-eq` build
NAMED checks (`Pass`/`Fail`) as values, `run-suite` runs a list of
cases, writes one progress line per case and returns the `Report`,
and `render-report` renders the summary Minitest prints (one block
per failure, then the counts line).

Module scheme (design/base-library.md): `Effect.*` is the effect
system's vocabulary, and the runner is here rather than in a
`Data.*` home because its one effectful operation IS the report.
REPORTING is `Effect.Io`'s capability: each case's line goes to the
runtime's output sink through the `Io r` capability's `ioWrite`
(the runtime's own operation, D81; capabilities are lexical and
never ambient, D21), so the suite's row is exactly the reporting
row `{IO r}` and the caller must hold the capability (`with (io =
newIO ()) in`). The choice is stated against the alternative this
repo has: `Effect.Log`'s `log` carries structured `Json` records
for the P32 observability story and lives outside the embedded
prelude family, while a runner's output is plain text lines, one
per case, which is what Minitest prints and what `ioWrite` takes.

THE LAW SHAPE (what the tests pin):
  1. counts. `report-runs` is the number of cases run,
     `report-assertions` is the sum of the per-case counts, and
     `report-failures` holds one entry per failing case, in run
     order.
  2. order. Outcomes follow the case list's order, and the suite is
     that list's catamorphism: `run-suite (xs <> ys)` reports xs's
     outcomes then ys's, with the counts added and the failure list
     concatenated.
  3. the observation law for assertions. A case's assertion count is
     the length of the prefix of its checks up to and including the
     first `Fail` (every `Pass` before it, and nothing after), so
     the count is exactly the number of assertions a runner that
     aborted the body would have executed.
  4. rendering. `assert-eq` and `refute-eq` render BOTH values
     through `Text.Show` (the message names the expectation and the
     observed rendering), and `render-report`'s totals line is the
     Minitest format `N cases, M assertions, K failures` (always
     plural: the line has one shape for every report).

NAMED REFUSALS (what this runner will not do):
  faults are not caught. An assertion never faults: a failing
    assertion is a `Fail` value. A body that faults anyway
    (division by zero, a byte write out of range) takes the
    runtime's fault path, so there is no Minitest `errors` count;
    catching the fault would need a handler over it, and no effect
    label here carries a catchable failure (the `Diverge` label is
    productivity, not an exception).
  no abort. A body's checks are a strict `List`, so the body
    evaluates all of them before the runner sees the first `Fail`;
    the runner OBSERVES the prefix (law 3) and cannot stop the
    body. A short-circuit body would need the checks to be an
    effect or a codata producer, neither of which the case type
    spends.
  no effectful bodies. A body is `Unit -> List Check`, pure, because
    the suite's row is exactly the reporting row; a body that wants
    the runtime capability would widen the suite's type and is out
    of this slice.
  no skips, no expected failures, no filtering or shuffling. The
    runner runs the cases it is given, in the order it is given
    them; Minitest's `skip`/`flunk`, its name filter and its seed
    are not ported.

</details>

# Types and constructors

<a id="type-436173654661696c757265"></a>

## `CaseFailure`

```cinch
data CaseFailure
```

A failing case's detail: the case's name, the failing assertion's
label, and the assertion's rendered message. `render-report`
prints exactly these three, and they are the three Minitest's own
failure list carries (test name, failing assertion, message).

[Source](/reference-sources/Effect.Test.cinch) · line 156

```cinch
CaseFailure :: (String -> (String -> (String -> CaseFailure)))
```

A failing case's detail: the case's name, the failing assertion's
label, and the assertion's rendered message. `render-report`
prints exactly these three, and they are the three Minitest's own
failure list carries (test name, failing assertion, message).

<a id="type-436173654f7574636f6d65"></a>

## `CaseOutcome`

```cinch
data CaseOutcome
```

One case's outcome: its name and the number of assertions
observed (law 3), plus the failure detail when it failed.

[Source](/reference-sources/Effect.Test.cinch) · line 173

```cinch
CasePassed :: (String -> (Int -> CaseOutcome))
```

```cinch
CaseFailed :: (Int -> (CaseFailure -> CaseOutcome))
```

<a id="type-436865636b"></a>

## `Check`

```cinch
data Check
```

The assertion vocabulary: assertions are values.

One assertion's outcome. The first field is the assertion's NAME
(the label the author gave it, so a failure reads as the
sentence it is); a `Fail` carries the rendered message after it.
An assertion is a value, never a fault: that is the whole reason
a failing case is reportable rather than fatal.

[Source](/reference-sources/Effect.Test.cinch) · line 99

```cinch
Pass :: (String -> Check)
```

```cinch
Fail :: (String -> (String -> Check))
```

<a id="type-5265706f7274"></a>

## `Report`

```cinch
data Report
```

A run of a suite: what Minitest's `Minitest::Result` list is, one
outcome per case in run order.

[Source](/reference-sources/Effect.Test.cinch) · line 194

```cinch
Report :: (List CaseOutcome -> Report)
```

A run of a suite: what Minitest's `Minitest::Result` list is, one
outcome per case in run order.

<a id="type-5465737443617365"></a>

## `TestCase`

```cinch
data TestCase
```

Cases, outcomes and the report.

A named case and its body. The body is a function of `Unit` so a
suite re-evaluates it per run instead of sharing one evaluation
between runs, and it is pure: the runner's only effect is the
reporting (see the refusals).

[Source](/reference-sources/Effect.Test.cinch) · line 141

```cinch
TestCase :: (String -> ((Unit -> List Check) -> TestCase))
```

Cases, outcomes and the report.

A named case and its body. The body is a function of `Unit` so a
suite re-evaluates it per run instead of sharing one evaluation
between runs, and it is pure: the runner's only effect is the
reporting (see the refusals).

# Values

<a id="value-617373657274"></a>

## `assert`

```cinch
assert :: (String -> (Bool -> Check))
```

The condition holds. `assert "one-plus-one" (1 + 1 == 2)`.

[Source](/reference-sources/Effect.Test.cinch) · line 104

<a id="value-6173736572742d6571"></a>

## `assert-eq`

```cinch
assert-eq :: forall a . using (ea :: Eq a, sa :: Show a) => (String -> (a -> (a -> Check)))
```

The two values are equal, rendered through `Show` so the message
names both. `assert-eq "sum" (1 + 1) 2`.

[Source](/reference-sources/Effect.Test.cinch) · line 109

<a id="value-636173652d626f6479"></a>

## `case-body`

```cinch
case-body :: (TestCase -> (Unit -> List Check))
```

[Source](/reference-sources/Effect.Test.cinch) · line 148

<a id="value-636173652d6e616d65"></a>

## `case-name`

```cinch
case-name :: (TestCase -> String)
```

[Source](/reference-sources/Effect.Test.cinch) · line 144

<a id="value-6661696c7572652d617373657274696f6e"></a>

## `failure-assertion`

```cinch
failure-assertion :: (CaseFailure -> String)
```

[Source](/reference-sources/Effect.Test.cinch) · line 163

<a id="value-6661696c7572652d63617365"></a>

## `failure-case`

```cinch
failure-case :: (CaseFailure -> String)
```

[Source](/reference-sources/Effect.Test.cinch) · line 159

<a id="value-6661696c7572652d6d657373616765"></a>

## `failure-message`

```cinch
failure-message :: (CaseFailure -> String)
```

[Source](/reference-sources/Effect.Test.cinch) · line 167

<a id="value-6f7574636f6d652d617373657274696f6e73"></a>

## `outcome-assertions`

```cinch
outcome-assertions :: (CaseOutcome -> Int)
```

[Source](/reference-sources/Effect.Test.cinch) · line 187

<a id="value-6f7574636f6d652d6e616d65"></a>

## `outcome-name`

```cinch
outcome-name :: (CaseOutcome -> String)
```

[Source](/reference-sources/Effect.Test.cinch) · line 177

<a id="value-6f7574636f6d652d6f6b"></a>

## `outcome-ok`

```cinch
outcome-ok :: (CaseOutcome -> Bool)
```

[Source](/reference-sources/Effect.Test.cinch) · line 182

<a id="value-726566757465"></a>

## `refute`

```cinch
refute :: (String -> (Bool -> Check))
```

The condition fails. `refute "not-two" (1 == 2)`.

[Source](/reference-sources/Effect.Test.cinch) · line 119

<a id="value-7265667574652d6571"></a>

## `refute-eq`

```cinch
refute-eq :: forall a . using (ea :: Eq a, sa :: Show a) => (String -> (a -> (a -> Check)))
```

The two values differ (Minitest's `refute_equal`), rendered
through `Show`.

[Source](/reference-sources/Effect.Test.cinch) · line 124

<a id="value-72656e6465722d7265706f7274"></a>

## `render-report`

```cinch
render-report :: (Report -> String)
```

Rendering the report.

The summary Minitest prints: one block per failure (its number,
the case, the failing assertion and the message), a blank line,
then the totals line. A report with no failures is the totals
line alone.
The blocks are flattened once: nesting `string-concat` would re-copy
the rest of the report at every failure.

[Source](/reference-sources/Effect.Test.cinch) · line 271

<a id="value-7265706f72742d617373657274696f6e73"></a>

## `report-assertions`

```cinch
report-assertions :: (Report -> Int)
```

[Source](/reference-sources/Effect.Test.cinch) · line 204

<a id="value-7265706f72742d6661696c75726573"></a>

## `report-failures`

```cinch
report-failures :: (Report -> List CaseFailure)
```

The failing cases, in run order.

[Source](/reference-sources/Effect.Test.cinch) · line 208

<a id="value-7265706f72742d6f6b"></a>

## `report-ok`

```cinch
report-ok :: (Report -> Bool)
```

True when no case failed (an empty report is ok: nothing ran,
nothing failed).

[Source](/reference-sources/Effect.Test.cinch) · line 213

<a id="value-7265706f72742d6f7574636f6d6573"></a>

## `report-outcomes`

```cinch
report-outcomes :: (Report -> List CaseOutcome)
```

[Source](/reference-sources/Effect.Test.cinch) · line 197

<a id="value-7265706f72742d72756e73"></a>

## `report-runs`

```cinch
report-runs :: (Report -> Int)
```

[Source](/reference-sources/Effect.Test.cinch) · line 201

<a id="value-72756e2d7375697465"></a>

## `run-suite`

```cinch
run-suite :: forall r . using (io :: Io r) => (List TestCase ->{IO r} Report)
```

Running a suite.

Run every case, reporting each one through the `Io r` capability
as it finishes (one line: `<name>` prefixed by its verdict), and
return the report. The progress lines are what a caller watches
while the suite runs; `render-report` is the summary after it.

[Source](/reference-sources/Effect.Test.cinch) · line 224

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Effect/Test.cinch`. Source SHA-256: `25f23bc38d56b5f178e7c80980cf4f29a8e84544dee3c883d8c857cb462478e0`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
