# Effect.Test

A subprocess-based test runner and named test-result/report vocabulary.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--source">Ship source</span> <code>import Effect.Test</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Effect.Test/source/">View source</a><a href="/packages/search/?q=%2BEffect.Test">Search module</a><a href="/reference-sources/Effect.Test.cinch" download="Effect.Test.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bool](Data.Bool.md), [Data.List](Data.List.md), [Data.Option](Data.Option.md), [Data.String](Data.String.md), [Relation.Eq](Relation.Eq.md), [Text.Show](Text.Show.md).

<details class="doc-closure"><summary>Ship-source closure (1 file)</summary>

Copy these modules beside your entry file before importing `Effect.Test`: [Effect.Test](Effect.Test.md).

```sh
cp seed/prelude/Effect/Test.cinch ./Effect.Test.cinch
```

</details>

<details class="doc-notes"><summary>Module notes</summary>

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

<details class="doc-synopsis"><summary>Synopsis (24 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-CaseFailure">CaseFailure</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">CaseFailure</strong></code></li><li><a class="syn-name" href="#t-CaseOutcome">CaseOutcome</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">CaseOutcome</strong></code></li><li><a class="syn-name" href="#t-Check">Check</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Check</strong></code></li><li><a class="syn-name" href="#t-Report">Report</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Report</strong></code></li><li><a class="syn-name" href="#t-TestCase">TestCase</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">TestCase</strong></code></li><li><a class="syn-name" href="#v-assert">assert</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">assert</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test/#t-Check" title="Effect.Test.Check">Check</a></code></li><li><a class="syn-name" href="#v-assert-eq">assert-eq</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">assert-eq</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">ea</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <span class="tok-variable">a</span>, <span class="tok-variable">sa</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Show/#c-Show" title="Text.Show.Show">Show</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test/#t-Check" title="Effect.Test.Check">Check</a></code></li><li><a class="syn-name" href="#v-case-body">case-body</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">case-body</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test/#t-TestCase" title="Effect.Test.TestCase">TestCase</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test/#t-Check" title="Effect.Test.Check">Check</a></code></li><li><a class="syn-name" href="#v-case-name">case-name</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">case-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test/#t-TestCase" title="Effect.Test.TestCase">TestCase</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-failure-assertion">failure-assertion</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">failure-assertion</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test/#t-CaseFailure" title="Effect.Test.CaseFailure">CaseFailure</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-failure-case">failure-case</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">failure-case</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test/#t-CaseFailure" title="Effect.Test.CaseFailure">CaseFailure</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-failure-message">failure-message</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">failure-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test/#t-CaseFailure" title="Effect.Test.CaseFailure">CaseFailure</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-outcome-assertions">outcome-assertions</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">outcome-assertions</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test/#t-CaseOutcome" title="Effect.Test.CaseOutcome">CaseOutcome</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a></code></li><li><a class="syn-name" href="#v-outcome-name">outcome-name</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">outcome-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test/#t-CaseOutcome" title="Effect.Test.CaseOutcome">CaseOutcome</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-outcome-ok">outcome-ok</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">outcome-ok</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test/#t-CaseOutcome" title="Effect.Test.CaseOutcome">CaseOutcome</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-refute">refute</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">refute</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test/#t-Check" title="Effect.Test.Check">Check</a></code></li><li><a class="syn-name" href="#v-refute-eq">refute-eq</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">refute-eq</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">ea</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <span class="tok-variable">a</span>, <span class="tok-variable">sa</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Show/#c-Show" title="Text.Show.Show">Show</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test/#t-Check" title="Effect.Test.Check">Check</a></code></li><li><a class="syn-name" href="#v-render-report">render-report</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">render-report</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test/#t-Report" title="Effect.Test.Report">Report</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-report-assertions">report-assertions</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">report-assertions</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test/#t-Report" title="Effect.Test.Report">Report</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a></code></li><li><a class="syn-name" href="#v-report-failures">report-failures</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">report-failures</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test/#t-Report" title="Effect.Test.Report">Report</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test/#t-CaseFailure" title="Effect.Test.CaseFailure">CaseFailure</a></code></li><li><a class="syn-name" href="#v-report-ok">report-ok</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">report-ok</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test/#t-Report" title="Effect.Test.Report">Report</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-report-outcomes">report-outcomes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">report-outcomes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test/#t-Report" title="Effect.Test.Report">Report</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test/#t-CaseOutcome" title="Effect.Test.CaseOutcome">CaseOutcome</a></code></li><li><a class="syn-name" href="#v-report-runs">report-runs</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">report-runs</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test/#t-Report" title="Effect.Test.Report">Report</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a></code></li><li><a class="syn-name" href="#v-run-suite">run-suite</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">run-suite</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test/#t-TestCase" title="Effect.Test.TestCase">TestCase</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test/#t-Report" title="Effect.Test.Report">Report</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-CaseFailure">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">CaseFailure</strong></code><span class="doc-sig-links"><a href="#t-CaseFailure" title="Permalink">#</a><a href="/packages/stdlib/Effect.Test/source/#L156" title="View source at line 156">Source</a></span></div>
<div class="doc-body">

A failing case's detail: the case's name, the failing assertion's
label, and the assertion's rendered message. `render-report`
prints exactly these three, and they are the three Minitest's own
failure list carries (test name, failing assertion, message).

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-CaseFailure"><code><strong class="tok-constructor">CaseFailure</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test/#t-CaseFailure" title="Effect.Test.CaseFailure">CaseFailure</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-CaseOutcome">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">CaseOutcome</strong></code><span class="doc-sig-links"><a href="#t-CaseOutcome" title="Permalink">#</a><a href="/packages/stdlib/Effect.Test/source/#L173" title="View source at line 173">Source</a></span></div>
<div class="doc-body">

One case's outcome: its name and the number of assertions
observed (law 3), plus the failure detail when it failed.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-CasePassed"><code><strong class="tok-constructor">CasePassed</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test/#t-CaseOutcome" title="Effect.Test.CaseOutcome">CaseOutcome</a></code></li><li id="k-CaseFailed"><code><strong class="tok-constructor">CaseFailed</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test/#t-CaseFailure" title="Effect.Test.CaseFailure">CaseFailure</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test/#t-CaseOutcome" title="Effect.Test.CaseOutcome">CaseOutcome</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-Check">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Check</strong></code><span class="doc-sig-links"><a href="#t-Check" title="Permalink">#</a><a href="/packages/stdlib/Effect.Test/source/#L99" title="View source at line 99">Source</a></span></div>
<div class="doc-body">

One assertion's outcome. The first field is the assertion's NAME
(the label the author gave it, so a failure reads as the
sentence it is); a `Fail` carries the rendered message after it.
An assertion is a value, never a fault: that is the whole reason
a failing case is reportable rather than fatal.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-Pass"><code><strong class="tok-constructor">Pass</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test/#t-Check" title="Effect.Test.Check">Check</a></code></li><li id="k-Fail"><code><strong class="tok-constructor">Fail</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test/#t-Check" title="Effect.Test.Check">Check</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-Report">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Report</strong></code><span class="doc-sig-links"><a href="#t-Report" title="Permalink">#</a><a href="/packages/stdlib/Effect.Test/source/#L194" title="View source at line 194">Source</a></span></div>
<div class="doc-body">

A run of a suite: what Minitest's `Minitest::Result` list is, one
outcome per case in run order.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-Report"><code><strong class="tok-constructor">Report</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test/#t-CaseOutcome" title="Effect.Test.CaseOutcome">CaseOutcome</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test/#t-Report" title="Effect.Test.Report">Report</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-TestCase">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">TestCase</strong></code><span class="doc-sig-links"><a href="#t-TestCase" title="Permalink">#</a><a href="/packages/stdlib/Effect.Test/source/#L141" title="View source at line 141">Source</a></span></div>
<div class="doc-body">

A named case and its body. The body is a function of `Unit` so a
suite re-evaluates it per run instead of sharing one evaluation
between runs, and it is pure: the runner's only effect is the
reporting (see the refusals).

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-TestCase"><code><strong class="tok-constructor">TestCase</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test/#t-Check" title="Effect.Test.Check">Check</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test/#t-TestCase" title="Effect.Test.TestCase">TestCase</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-assert">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">assert</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test/#t-Check" title="Effect.Test.Check">Check</a></code><span class="doc-sig-links"><a href="#v-assert" title="Permalink">#</a><a href="/packages/stdlib/Effect.Test/source/#L104" title="View source at line 104">Source</a></span></div>
<div class="doc-body">

The condition holds. `assert "one-plus-one" (1 + 1 == 2)`.

</div>
</article>

<article class="doc-entry" id="v-assert-eq">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">assert-eq</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">ea</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <span class="tok-variable">a</span>, <span class="tok-variable">sa</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Show/#c-Show" title="Text.Show.Show">Show</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test/#t-Check" title="Effect.Test.Check">Check</a></code><span class="doc-sig-links"><a href="#v-assert-eq" title="Permalink">#</a><a href="/packages/stdlib/Effect.Test/source/#L109" title="View source at line 109">Source</a></span></div>
<div class="doc-body">

The two values are equal, rendered through `Show` so the message
names both. `assert-eq "sum" (1 + 1) 2`.

</div>
</article>

<article class="doc-entry" id="v-case-body">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">case-body</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test/#t-TestCase" title="Effect.Test.TestCase">TestCase</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test/#t-Check" title="Effect.Test.Check">Check</a></code><span class="doc-sig-links"><a href="#v-case-body" title="Permalink">#</a><a href="/packages/stdlib/Effect.Test/source/#L148" title="View source at line 148">Source</a></span></div>
</article>

<article class="doc-entry" id="v-case-name">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">case-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test/#t-TestCase" title="Effect.Test.TestCase">TestCase</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-case-name" title="Permalink">#</a><a href="/packages/stdlib/Effect.Test/source/#L144" title="View source at line 144">Source</a></span></div>
</article>

<article class="doc-entry" id="v-failure-assertion">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">failure-assertion</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test/#t-CaseFailure" title="Effect.Test.CaseFailure">CaseFailure</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-failure-assertion" title="Permalink">#</a><a href="/packages/stdlib/Effect.Test/source/#L163" title="View source at line 163">Source</a></span></div>
</article>

<article class="doc-entry" id="v-failure-case">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">failure-case</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test/#t-CaseFailure" title="Effect.Test.CaseFailure">CaseFailure</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-failure-case" title="Permalink">#</a><a href="/packages/stdlib/Effect.Test/source/#L159" title="View source at line 159">Source</a></span></div>
</article>

<article class="doc-entry" id="v-failure-message">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">failure-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test/#t-CaseFailure" title="Effect.Test.CaseFailure">CaseFailure</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-failure-message" title="Permalink">#</a><a href="/packages/stdlib/Effect.Test/source/#L167" title="View source at line 167">Source</a></span></div>
</article>

<article class="doc-entry" id="v-outcome-assertions">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">outcome-assertions</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test/#t-CaseOutcome" title="Effect.Test.CaseOutcome">CaseOutcome</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a></code><span class="doc-sig-links"><a href="#v-outcome-assertions" title="Permalink">#</a><a href="/packages/stdlib/Effect.Test/source/#L187" title="View source at line 187">Source</a></span></div>
</article>

<article class="doc-entry" id="v-outcome-name">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">outcome-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test/#t-CaseOutcome" title="Effect.Test.CaseOutcome">CaseOutcome</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-outcome-name" title="Permalink">#</a><a href="/packages/stdlib/Effect.Test/source/#L177" title="View source at line 177">Source</a></span></div>
</article>

<article class="doc-entry" id="v-outcome-ok">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">outcome-ok</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test/#t-CaseOutcome" title="Effect.Test.CaseOutcome">CaseOutcome</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-outcome-ok" title="Permalink">#</a><a href="/packages/stdlib/Effect.Test/source/#L182" title="View source at line 182">Source</a></span></div>
</article>

<article class="doc-entry" id="v-refute">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">refute</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test/#t-Check" title="Effect.Test.Check">Check</a></code><span class="doc-sig-links"><a href="#v-refute" title="Permalink">#</a><a href="/packages/stdlib/Effect.Test/source/#L119" title="View source at line 119">Source</a></span></div>
<div class="doc-body">

The condition fails. `refute "not-two" (1 == 2)`.

</div>
</article>

<article class="doc-entry" id="v-refute-eq">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">refute-eq</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">ea</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <span class="tok-variable">a</span>, <span class="tok-variable">sa</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Show/#c-Show" title="Text.Show.Show">Show</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test/#t-Check" title="Effect.Test.Check">Check</a></code><span class="doc-sig-links"><a href="#v-refute-eq" title="Permalink">#</a><a href="/packages/stdlib/Effect.Test/source/#L124" title="View source at line 124">Source</a></span></div>
<div class="doc-body">

The two values differ (Minitest's `refute_equal`), rendered
through `Show`.

</div>
</article>

<article class="doc-entry" id="v-render-report">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">render-report</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test/#t-Report" title="Effect.Test.Report">Report</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-render-report" title="Permalink">#</a><a href="/packages/stdlib/Effect.Test/source/#L271" title="View source at line 271">Source</a></span></div>
<div class="doc-body">

The summary Minitest prints: one block per failure (its number,
the case, the failing assertion and the message), a blank line,
then the totals line. A report with no failures is the totals
line alone.
The blocks are flattened once: nesting `string-concat` would re-copy
the rest of the report at every failure.

</div>
</article>

<article class="doc-entry" id="v-report-assertions">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">report-assertions</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test/#t-Report" title="Effect.Test.Report">Report</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a></code><span class="doc-sig-links"><a href="#v-report-assertions" title="Permalink">#</a><a href="/packages/stdlib/Effect.Test/source/#L204" title="View source at line 204">Source</a></span></div>
</article>

<article class="doc-entry" id="v-report-failures">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">report-failures</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test/#t-Report" title="Effect.Test.Report">Report</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test/#t-CaseFailure" title="Effect.Test.CaseFailure">CaseFailure</a></code><span class="doc-sig-links"><a href="#v-report-failures" title="Permalink">#</a><a href="/packages/stdlib/Effect.Test/source/#L208" title="View source at line 208">Source</a></span></div>
<div class="doc-body">

The failing cases, in run order.

</div>
</article>

<article class="doc-entry" id="v-report-ok">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">report-ok</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test/#t-Report" title="Effect.Test.Report">Report</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-report-ok" title="Permalink">#</a><a href="/packages/stdlib/Effect.Test/source/#L213" title="View source at line 213">Source</a></span></div>
<div class="doc-body">

True when no case failed (an empty report is ok: nothing ran,
nothing failed).

</div>
</article>

<article class="doc-entry" id="v-report-outcomes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">report-outcomes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test/#t-Report" title="Effect.Test.Report">Report</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test/#t-CaseOutcome" title="Effect.Test.CaseOutcome">CaseOutcome</a></code><span class="doc-sig-links"><a href="#v-report-outcomes" title="Permalink">#</a><a href="/packages/stdlib/Effect.Test/source/#L197" title="View source at line 197">Source</a></span></div>
</article>

<article class="doc-entry" id="v-report-runs">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">report-runs</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test/#t-Report" title="Effect.Test.Report">Report</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a></code><span class="doc-sig-links"><a href="#v-report-runs" title="Permalink">#</a><a href="/packages/stdlib/Effect.Test/source/#L201" title="View source at line 201">Source</a></span></div>
</article>

<article class="doc-entry" id="v-run-suite">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">run-suite</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test/#t-TestCase" title="Effect.Test.TestCase">TestCase</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test/#t-Report" title="Effect.Test.Report">Report</a></code><span class="doc-sig-links"><a href="#v-run-suite" title="Permalink">#</a><a href="/packages/stdlib/Effect.Test/source/#L224" title="View source at line 224">Source</a></span></div>
<div class="doc-body">

Run every case, reporting each one through the `Io r` capability
as it finishes (one line: `<name>` prefixed by its verdict), and
return the report. The progress lines are what a caller watches
while the suite runs; `render-report` is the summary after it.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Effect/Test.cinch`](/packages/stdlib/Effect.Test/source/). Source SHA-256: `25f23bc38d56b5f178e7c80980cf4f29a8e84544dee3c883d8c857cb462478e0`.
