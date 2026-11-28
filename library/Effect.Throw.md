# Effect.Throw

Typed short-circuiting algebraic error effect (Throw e) with cross-call cooperative bubbling, throw, catch, and try-throw.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Effect.Throw</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Effect.Throw/source/">View source</a><a href="/packages/search/?q=%2BEffect.Throw">Search module</a><a href="/reference-sources/Effect.Throw.cinch" download="Effect.Throw.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Core](Data.Core.md).

<details class="doc-notes"><summary>Module notes</summary>

Effect.Throw: typed short-circuiting algebraic error effect.

`Throw e` is a type-parameterized algebraic effect lowered via Koka-style
evidence passing and cooperative bubbling:
  * Pure (`->{}`) functions pay zero overhead and retain `nounwind`/`willreturn`.
  * `throw err` unwinds cooperatively across arbitrarily deep call chains,
    running exact Perceus reference-count drops on every popped frame.
  * Distinct error types (`Throw ParseErr`, `Throw IoErr`) carry distinct
    canonical label hashes and compose cleanly with nested `catch` prompts.

</details>

<details class="doc-synopsis" open><summary>Synopsis (4 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-Throw">Throw</a><code class="syn-sig"><span class="tok-keyword">effect</span> <strong class="doc-decl-name tok-constructor">Throw</strong> <span class="tok-variable">e</span></code></li><li><a class="syn-name" href="#v-catch">catch</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">catch</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">e</span> <span class="tok-variable">a</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Row</span>). (<a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Throw/#t-Throw" title="Effect.Throw.Throw">Throw</a> e | r} <span class="tok-variable">a</span>) <span class="tok-operator">-&gt;</span>{|r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <span class="tok-variable">e</span> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-throw">throw</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">throw</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">e</span> <span class="tok-variable">a</span>. <span class="tok-variable">e</span> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Throw/#t-Throw" title="Effect.Throw.Throw">Throw</a> e} <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-try-throw">try-throw</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">try-throw</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">e</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <span class="tok-variable">e</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Throw/#t-Throw" title="Effect.Throw.Throw">Throw</a> e} <span class="tok-variable">a</span></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-Throw">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">effect</span> <strong class="doc-decl-name tok-constructor">Throw</strong> <span class="tok-variable">e</span></code><span class="doc-sig-links"><a href="#t-Throw" title="Permalink">#</a><a href="/packages/stdlib/Effect.Throw/source/" title="View source">Source</a></span></div>
<div class="doc-body">
<div class="doc-children"><h4>Fields</h4><ul><li><code>throw</code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-catch">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">catch</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">e</span> <span class="tok-variable">a</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Row</span>). (<a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Throw/#t-Throw" title="Effect.Throw.Throw">Throw</a> e | r} <span class="tok-variable">a</span>) <span class="tok-operator">-&gt;</span>{|r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <span class="tok-variable">e</span> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-catch" title="Permalink">#</a><a href="/packages/stdlib/Effect.Throw/source/#L19" title="View source at line 19">Source</a></span></div>
<div class="doc-body">

Run `body` under a `Throw e` handler, capturing any thrown `e` as `Err e`
and wrapping a normal completion as `Ok x`.

</div>
</article>

<article class="doc-entry" id="v-throw">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">throw</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">e</span> <span class="tok-variable">a</span>. <span class="tok-variable">e</span> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Throw/#t-Throw" title="Effect.Throw.Throw">Throw</a> e} <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-throw" title="Permalink">#</a><a href="/packages/stdlib/Effect.Throw/source/" title="View source">Source</a></span></div>
</article>

<article class="doc-entry" id="v-try-throw">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">try-throw</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">e</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <span class="tok-variable">e</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Throw/#t-Throw" title="Effect.Throw.Throw">Throw</a> e} <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-try-throw" title="Permalink">#</a><a href="/packages/stdlib/Effect.Throw/source/#L25" title="View source at line 25">Source</a></span></div>
<div class="doc-body">

Lift a `Result e a` value into the `Throw e` effect.

</div>
</article>
