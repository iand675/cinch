# Effect.Coverage

Coverage collection/reporting utilities with explicit runtime and source metadata.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Effect.Coverage`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Effect.Coverage.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Algebra.Monoid](Algebra.Monoid.md), [Algebra.Semigroup](Algebra.Semigroup.md), [Data.List](Data.List.md), [Data.Map](Data.Map.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.String](Data.String.md), [Text.Show](Text.Show.md).

Non-embedded source closure: [Effect.Coverage](Effect.Coverage.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Effect.Coverage: the coverage recorder (Ruby's `Coverage`, the
recorder half).

Module scheme (design/base-library.md): `Effect.*` is the effect
system's vocabulary. This is the tooling member of that home, the
counterpart of `Effect.Log`/`Effect.Trace`: a pure value the
instrumentation stage accumulates into. The Ruby item
(design/stdlib-parity-ruby.md, "Tooling: `OptionParser`, `Benchmark`,
`Coverage`, `Minitest`") is `Coverage.peek_result`, a map from site
to hit count; the value half of that pair is the recorder here, the
reporting half is `coverage-report`.

WHAT THIS MODULE IS NOT. There is no source-level instrumentation in
this compiler: nothing in the pipeline emits per-site calls yet, and
this module does not add one. It is the RECORDER ONLY, the state a
generated instrumentation would accumulate into plus the reporting
over that state, with the call shape fixed so the eventual pass has a
target. An instrumenting pass must supply, per instrumented site,
exactly two things: a stable LABEL (a `String` naming the site, say
a source path with a line and column) and one `coverage-hit label`
call at that site. Nothing else: the recorder treats labels as
opaque, orders them by byte value only (`Ord String`), and merging
two recorders sums the counts at equal labels. The pass chooses its
own granularity (one label per function, per branch, or per basic
block) because granularity is a property of the labels it invents,
never of a type here. It is deliberately NOT the instrumentation: the
compiler does not know the set of "all the sites", so the recorder
can only report the labels it was hit with (see the refusals).

THE INTENDED CALL SHAPE (what the generated code does). One call per
site, threading the recorder:

  let c = coverage-hit "examples/fib.cinch:12:3" c in
  let c = coverage-hit "examples/fib.cinch:12:3" c in
  ...

Where the recorder lives is the pass's choice: as a pure value it
threads through the instrumented functions, and a caller wanting
per-run totals threads one recorder from `coverage-new` and reads
`coverage-report` when the run ends.

THE LAW SHAPE. `Coverage` is the hit multiset: `coverage-merge` is
keywise addition of the counts, so
  identity:      `coverage-merge coverage-new c` and
                 `coverage-merge c coverage-new` have the same
                 `coverage-counts` as `c` (the empty recorder is the
                 identity element);
  associativity: `coverage-merge (coverage-merge a b) c` and
                 `coverage-merge a (coverage-merge b c)` have the
                 same `coverage-counts`;
  agreement:     `coverage-total c` is the sum of the counts
                 `coverage-counts` reports.
The test asserts all three over sample recorders
(compiler/tests/lib_coverage.rs). Commutativity also holds (addition
and the map union are symmetric) and is checked there, but it is not
the law anything depends on: only the keywise sum is observable.

NAMED REFUSALS (boundaries this module deliberately does not cross):
  instrumentation: the module is the recorder, not the pass; it never
    decides which sites exist and never emits a call;
  un-hit sites: a recorder reports exactly the labels it was hit
    with, so a site that never ran cannot appear (the "all the sites"
    list is the pass's, not the recorder's);
  granularity and label syntax: the label is an opaque `String`, so
    line-level vs statement-level, and any path/line/column encoding,
    are the pass's decisions, never a type or a parser here;
  representation exposure: the label tree is module-private (the
    `Data.Map` ordered map, decision D105), so every reachable
    recorder is a correct search tree and `coverage-counts` is always
    in ascending label order (D143's surface rule: the export list is
    exactly the named set);
  persistence and output: no file format and no `Coverage.result`
    serialization; `coverage-report` is a `String`, and writing it
    somewhere is the caller's (the `Effect.Fs` home's) business.

</details>

# Types and constructors

<a id="type-436f766572616765"></a>

## `Coverage`

```cinch
data Coverage
```

The recorder: hit counts keyed by site label. The tree is the
`Data.Map` ordered map, so a hit is a lookup plus an insert and
`coverage-counts` is the map's in-order walk.

[Source](/reference-sources/Effect.Coverage.cinch) · line 93

```cinch
Coverage :: (Map String Nat -> Coverage)
```

The recorder: hit counts keyed by site label. The tree is the
`Data.Map` ordered map, so a hit is a lookup plus an insert and
`coverage-counts` is the map's in-order walk.

# Values

<a id="value-636f7665726167652d636f756e7473"></a>

## `coverage-counts`

```cinch
coverage-counts :: (Coverage -> List (String, Nat))
```

Every site with its count, in ascending label order: the in-order
walk of the search tree, so the order is byte-wise on the label
(the `Ord String` the map is built over), independent of the order
the hits arrived in.

[Source](/reference-sources/Effect.Coverage.cinch) · line 116

<a id="value-636f7665726167652d686974"></a>

## `coverage-hit`

```cinch
coverage-hit :: (String -> (Coverage -> Coverage))
```

Record one hit at `site`: the counter goes up by one, and a site
not seen before starts at one. This is the one call a generated
instrumentation makes per instrumented site.

[Source](/reference-sources/Effect.Coverage.cinch) · line 103

<a id="value-636f7665726167652d6d65726765"></a>

## `coverage-merge`

```cinch
coverage-merge :: (Coverage -> (Coverage -> Coverage))
```

Combine two recorders: the counts at equal labels add, and a label
present in only one side keeps its count. The identity element is
`coverage-new` and the operation is associative (the header's law
shape; the test checks both over sample recorders).

[Source](/reference-sources/Effect.Coverage.cinch) · line 159

<a id="value-636f7665726167652d6e6577"></a>

## `coverage-new`

```cinch
coverage-new :: Coverage
```

A recorder with no hits (the merge identity).

[Source](/reference-sources/Effect.Coverage.cinch) · line 97

<a id="value-636f7665726167652d7265706f7274"></a>

## `coverage-report`

```cinch
coverage-report :: (Coverage -> String)
```

The report: one line per site, `label` TAB count NEWLINE, then a
final `total` TAB total NEWLINE. An empty recorder is the total
line alone (`total\t0\n`), with no site lines.
The lines are combined once by `mconcat` (String's is one buffer);
nesting `<>` would re-copy the rest of the report at every site.

[Source](/reference-sources/Effect.Coverage.cinch) · line 140

<a id="value-636f7665726167652d746f74616c"></a>

## `coverage-total`

```cinch
coverage-total :: (Coverage -> Nat)
```

The sum of every count: the number of hits recorded.

[Source](/reference-sources/Effect.Coverage.cinch) · line 126

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Effect/Coverage.cinch`. Source SHA-256: `62f842a0e3043f00dbfbbbfa26b4c53df3392bd32a44681ad953a1344c21efef`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
