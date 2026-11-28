# Effect.Arena

Region-delimited bump arenas with O(1) fast escape (with-arena, arena-abort) and typed byte-budget boundaries (with-arena-budget, BudgetError).

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Effect.Arena</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Effect.Arena/source/">View source</a><a href="/packages/search/?q=%2BEffect.Arena">Search module</a><a href="/reference-sources/Effect.Arena.cinch" download="Effect.Arena.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Core](Data.Core.md).

<details class="doc-notes"><summary>Module notes</summary>

Effect.Arena: region-delimited O(1) fast escape and typed heap budget boundary.

`with-arena` and `with-arena-budget` execute a pure computation inside a
dedicated bump arena scoped to an unforgeable region `r`. Calling
`arena-abort tok err` (or exceeding the byte cap in `with-arena-budget`)
jumps straight back to the prompt in O(1), reclaiming every arena chunk and
tracked allocation without leaking.

Static safety invariants enforced by the compiler:
  1. Region `r` is Rank-2 universally quantified, so `ArenaToken r e` cannot
     escape the arena scope.
  2. Return type `a` and error type `e` must be arena-escape-safe (scalars,
     `Bool`, `Unit`, `Int`, `Nat`, or nullary enums) so no arena pointer can
     outlive the arena.
  3. The callback closure cannot capture any linear (`%1`) binder.
  4. The callback arrow is pure (`->{}`), forbidding `Async`/`Spawn`/`IO`
     inside the O(1) jump extent.

</details>

<details class="doc-synopsis" open><summary>Synopsis (5 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-ArenaToken">ArenaToken</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ArenaToken</strong> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) <span class="tok-variable">e</span></code></li><li><a class="syn-name" href="#t-BudgetError">BudgetError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">BudgetError</strong></code></li><li><a class="syn-name" href="#v-arena-abort">arena-abort</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">arena-abort</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) <span class="tok-variable">e</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Arena/#t-ArenaToken" title="Effect.Arena.ArenaToken">ArenaToken</a> <span class="tok-variable">r</span> <span class="tok-variable">e</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">e</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-with-arena">with-arena</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">with-arena</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">e</span> <span class="tok-variable">a</span>. (<span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Arena/#t-ArenaToken" title="Effect.Arena.ArenaToken">ArenaToken</a> <span class="tok-variable">r</span> <span class="tok-variable">e</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <span class="tok-variable">e</span> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-with-arena-budget">with-arena-budget</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">with-arena-budget</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> (<span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Arena/#t-ArenaToken" title="Effect.Arena.ArenaToken">ArenaToken</a> <span class="tok-variable">r</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Arena/#t-BudgetError" title="Effect.Arena.BudgetError">BudgetError</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Arena/#t-BudgetError" title="Effect.Arena.BudgetError">BudgetError</a> <span class="tok-variable">a</span></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-ArenaToken">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ArenaToken</strong> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) <span class="tok-variable">e</span></code><span class="doc-sig-links"><a href="#t-ArenaToken" title="Permalink">#</a><a href="/packages/stdlib/Effect.Arena/source/#L28" title="View source at line 28">Source</a></span></div>
<div class="doc-body">

Unforgeable region-indexed escape prompt token.

</div>
</article>

<article class="doc-entry" id="t-BudgetError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">BudgetError</strong></code><span class="doc-sig-links"><a href="#t-BudgetError" title="Permalink">#</a><a href="/packages/stdlib/Effect.Arena/source/#L24" title="View source at line 24">Source</a></span></div>
<div class="doc-body">

Typed error returned when an arena exceeds its byte budget.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-Exhausted"><code><strong class="tok-constructor">Exhausted</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Arena/#t-BudgetError" title="Effect.Arena.BudgetError">BudgetError</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-arena-abort">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">arena-abort</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) <span class="tok-variable">e</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Arena/#t-ArenaToken" title="Effect.Arena.ArenaToken">ArenaToken</a> <span class="tok-variable">r</span> <span class="tok-variable">e</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">e</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-arena-abort" title="Permalink">#</a><a href="/packages/stdlib/Effect.Arena/source/#L49" title="View source at line 49">Source</a></span></div>
<div class="doc-body">

Escape in O(1) to the prompt owning `tok` with error value `err`.

</div>
</article>

<article class="doc-entry" id="v-with-arena">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">with-arena</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">e</span> <span class="tok-variable">a</span>. (<span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Arena/#t-ArenaToken" title="Effect.Arena.ArenaToken">ArenaToken</a> <span class="tok-variable">r</span> <span class="tok-variable">e</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <span class="tok-variable">e</span> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-with-arena" title="Permalink">#</a><a href="/packages/stdlib/Effect.Arena/source/#L38" title="View source at line 38">Source</a></span></div>
<div class="doc-body">

Run a pure computation in a fresh bump arena with O(1) `arena-abort`.

</div>
</article>

<article class="doc-entry" id="v-with-arena-budget">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">with-arena-budget</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> (<span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Arena/#t-ArenaToken" title="Effect.Arena.ArenaToken">ArenaToken</a> <span class="tok-variable">r</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Arena/#t-BudgetError" title="Effect.Arena.BudgetError">BudgetError</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Arena/#t-BudgetError" title="Effect.Arena.BudgetError">BudgetError</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-with-arena-budget" title="Permalink">#</a><a href="/packages/stdlib/Effect.Arena/source/#L44" title="View source at line 44">Source</a></span></div>
<div class="doc-body">

Run a pure computation in a bump arena capped at `budget-bytes` bytes.
Exceeding the budget escapes in O(1) with `Err Exhausted`.

</div>
</article>
