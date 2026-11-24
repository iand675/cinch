# Effect.Coverage

Coverage collection/reporting utilities with explicit runtime and source metadata.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--source">Ship source</span> <code>import Effect.Coverage</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Effect.Coverage/source/">View source</a><a href="/packages/search/?q=%2BEffect.Coverage">Search module</a><a href="/reference-sources/Effect.Coverage.cinch" download="Effect.Coverage.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Algebra.Monoid](Algebra.Monoid.md), [Algebra.Semigroup](Algebra.Semigroup.md), [Data.List](Data.List.md), [Data.Map](Data.Map.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.String](Data.String.md), [Text.Show](Text.Show.md).

<details class="doc-closure"><summary>Ship-source closure (1 file)</summary>

Copy these modules beside your entry file before importing `Effect.Coverage`: [Effect.Coverage](Effect.Coverage.md).

```sh
cp seed/prelude/Effect/Coverage.cinch ./Effect.Coverage.cinch
```

</details>

<details class="doc-notes"><summary>Module notes</summary>

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
(seed/tests/lib_coverage.rs). Commutativity also holds (addition
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

<details class="doc-synopsis" open><summary>Synopsis (7 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-Coverage">Coverage</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Coverage</strong></code></li><li><a class="syn-name" href="#v-coverage-counts">coverage-counts</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">coverage-counts</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Coverage/#t-Coverage" title="Effect.Coverage.Coverage">Coverage</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>)</code></li><li><a class="syn-name" href="#v-coverage-hit">coverage-hit</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">coverage-hit</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Coverage/#t-Coverage" title="Effect.Coverage.Coverage">Coverage</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Coverage/#t-Coverage" title="Effect.Coverage.Coverage">Coverage</a></code></li><li><a class="syn-name" href="#v-coverage-merge">coverage-merge</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">coverage-merge</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Coverage/#t-Coverage" title="Effect.Coverage.Coverage">Coverage</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Coverage/#t-Coverage" title="Effect.Coverage.Coverage">Coverage</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Coverage/#t-Coverage" title="Effect.Coverage.Coverage">Coverage</a></code></li><li><a class="syn-name" href="#v-coverage-new">coverage-new</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">coverage-new</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Coverage/#t-Coverage" title="Effect.Coverage.Coverage">Coverage</a></code></li><li><a class="syn-name" href="#v-coverage-report">coverage-report</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">coverage-report</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Coverage/#t-Coverage" title="Effect.Coverage.Coverage">Coverage</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-coverage-total">coverage-total</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">coverage-total</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Coverage/#t-Coverage" title="Effect.Coverage.Coverage">Coverage</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-Coverage">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Coverage</strong></code><span class="doc-sig-links"><a href="#t-Coverage" title="Permalink">#</a><a href="/packages/stdlib/Effect.Coverage/source/#L93" title="View source at line 93">Source</a></span></div>
<div class="doc-body">

The recorder: hit counts keyed by site label. The tree is the
`Data.Map` ordered map, so a hit is a lookup plus an insert and
`coverage-counts` is the map's in-order walk.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-Coverage"><code><strong class="tok-constructor">Coverage</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Map/#t-Map" title="Data.Map.Map">Map</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Coverage/#t-Coverage" title="Effect.Coverage.Coverage">Coverage</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-coverage-counts">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">coverage-counts</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Coverage/#t-Coverage" title="Effect.Coverage.Coverage">Coverage</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>)</code><span class="doc-sig-links"><a href="#v-coverage-counts" title="Permalink">#</a><a href="/packages/stdlib/Effect.Coverage/source/#L116" title="View source at line 116">Source</a></span></div>
<div class="doc-body">

Every site with its count, in ascending label order: the in-order
walk of the search tree, so the order is byte-wise on the label
(the `Ord String` the map is built over), independent of the order
the hits arrived in.

</div>
</article>

<article class="doc-entry" id="v-coverage-hit">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">coverage-hit</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Coverage/#t-Coverage" title="Effect.Coverage.Coverage">Coverage</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Coverage/#t-Coverage" title="Effect.Coverage.Coverage">Coverage</a></code><span class="doc-sig-links"><a href="#v-coverage-hit" title="Permalink">#</a><a href="/packages/stdlib/Effect.Coverage/source/#L103" title="View source at line 103">Source</a></span></div>
<div class="doc-body">

Record one hit at `site`: the counter goes up by one, and a site
not seen before starts at one. This is the one call a generated
instrumentation makes per instrumented site.

</div>
</article>

<article class="doc-entry" id="v-coverage-merge">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">coverage-merge</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Coverage/#t-Coverage" title="Effect.Coverage.Coverage">Coverage</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Coverage/#t-Coverage" title="Effect.Coverage.Coverage">Coverage</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Coverage/#t-Coverage" title="Effect.Coverage.Coverage">Coverage</a></code><span class="doc-sig-links"><a href="#v-coverage-merge" title="Permalink">#</a><a href="/packages/stdlib/Effect.Coverage/source/#L159" title="View source at line 159">Source</a></span></div>
<div class="doc-body">

Combine two recorders: the counts at equal labels add, and a label
present in only one side keeps its count. The identity element is
`coverage-new` and the operation is associative (the header's law
shape; the test checks both over sample recorders).

</div>
</article>

<article class="doc-entry" id="v-coverage-new">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">coverage-new</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Coverage/#t-Coverage" title="Effect.Coverage.Coverage">Coverage</a></code><span class="doc-sig-links"><a href="#v-coverage-new" title="Permalink">#</a><a href="/packages/stdlib/Effect.Coverage/source/#L97" title="View source at line 97">Source</a></span></div>
<div class="doc-body">

A recorder with no hits (the merge identity).

</div>
</article>

<article class="doc-entry" id="v-coverage-report">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">coverage-report</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Coverage/#t-Coverage" title="Effect.Coverage.Coverage">Coverage</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-coverage-report" title="Permalink">#</a><a href="/packages/stdlib/Effect.Coverage/source/#L140" title="View source at line 140">Source</a></span></div>
<div class="doc-body">

The report: one line per site, `label` TAB count NEWLINE, then a
final `total` TAB total NEWLINE. An empty recorder is the total
line alone (`total\t0\n`), with no site lines.
The lines are combined once by `mconcat` (String's is one buffer);
nesting `<>` would re-copy the rest of the report at every site.

</div>
</article>

<article class="doc-entry" id="v-coverage-total">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">coverage-total</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Coverage/#t-Coverage" title="Effect.Coverage.Coverage">Coverage</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-coverage-total" title="Permalink">#</a><a href="/packages/stdlib/Effect.Coverage/source/#L126" title="View source at line 126">Source</a></span></div>
<div class="doc-body">

The sum of every count: the number of hits recorded.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Effect/Coverage.cinch`](/packages/stdlib/Effect.Coverage/source/). Source SHA-256: `b915528a23ac1a11bf8d7223ba1fbf87c1cc1bfd19ce2e38a0e59a1f8858ef62`.
