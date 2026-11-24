# Data.Mutable.Var

A single mutable cell, written in place; not synchronized (TVar is the shared cell).

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Data.Mutable.Var</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Data.Mutable.Var/source/">View source</a><a href="/packages/search/?q=%2BData.Mutable.Var">Search module</a><a href="/reference-sources/Data.Mutable.Var.cinch" download="Data.Mutable.Var.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

<details class="doc-notes"><summary>Module notes</summary>

Data.Mutable.Var: the single mutable cell (Haskell's `IORef`/`STRef`,
OCaml's `ref`).

A `Var r a` always holds one value. `var-write` releases the value it
replaces; `var-modify` moves the value out and back, so a uniquely held
value is rewritten without a copy. It is not synchronized: the shared
cell between tasks is `Effect.Stm`'s `TVar`.

The trusted core is the private `cinch.runtime.slots*` imports (the
compiler's `slots.rs`); none of them leaves the module. The one slot is
written before the cell is returned.

</details>

<details class="doc-synopsis" open><summary>Synopsis (6 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-Var">Var</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Var</strong> <span class="tok-variable">r</span> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-var-modify">var-modify</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">var-modify</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Mutable/#t-Var" title="Data.Mutable.Var.Var">Var</a> <span class="tok-variable">r</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span>) <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/10-ownership-and-memory/" title="Mut">Mut</a> r} <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code></li><li><a class="syn-name" href="#v-var-new">var-new</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">var-new</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) <span class="tok-variable">a</span>. <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/10-ownership-and-memory/" title="Mut">Mut</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Mutable/#t-Var" title="Data.Mutable.Var.Var">Var</a> <span class="tok-variable">r</span> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-var-read">var-read</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">var-read</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Mutable/#t-Var" title="Data.Mutable.Var.Var">Var</a> <span class="tok-variable">r</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/10-ownership-and-memory/" title="Mut">Mut</a> r} <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-var-swap">var-swap</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">var-swap</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Mutable/#t-Var" title="Data.Mutable.Var.Var">Var</a> <span class="tok-variable">r</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/10-ownership-and-memory/" title="Mut">Mut</a> r} <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-var-write">var-write</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">var-write</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Mutable/#t-Var" title="Data.Mutable.Var.Var">Var</a> <span class="tok-variable">r</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/10-ownership-and-memory/" title="Mut">Mut</a> r} <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-Var">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Var</strong> <span class="tok-variable">r</span> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#t-Var" title="Permalink">#</a><a href="/packages/stdlib/Data.Mutable.Var/source/#L21" title="View source at line 21">Source</a></span></div>
</article>

# Values

<article class="doc-entry" id="v-var-modify">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">var-modify</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Mutable/#t-Var" title="Data.Mutable.Var.Var">Var</a> <span class="tok-variable">r</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span>) <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/10-ownership-and-memory/" title="Mut">Mut</a> r} <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code><span class="doc-sig-links"><a href="#v-var-modify" title="Permalink">#</a><a href="/packages/stdlib/Data.Mutable.Var/source/#L46" title="View source at line 46">Source</a></span></div>
</article>

<article class="doc-entry" id="v-var-new">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">var-new</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) <span class="tok-variable">a</span>. <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/10-ownership-and-memory/" title="Mut">Mut</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Mutable/#t-Var" title="Data.Mutable.Var.Var">Var</a> <span class="tok-variable">r</span> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-var-new" title="Permalink">#</a><a href="/packages/stdlib/Data.Mutable.Var/source/#L28" title="View source at line 28">Source</a></span></div>
</article>

<article class="doc-entry" id="v-var-read">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">var-read</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Mutable/#t-Var" title="Data.Mutable.Var.Var">Var</a> <span class="tok-variable">r</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/10-ownership-and-memory/" title="Mut">Mut</a> r} <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-var-read" title="Permalink">#</a><a href="/packages/stdlib/Data.Mutable.Var/source/#L34" title="View source at line 34">Source</a></span></div>
</article>

<article class="doc-entry" id="v-var-swap">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">var-swap</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Mutable/#t-Var" title="Data.Mutable.Var.Var">Var</a> <span class="tok-variable">r</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/10-ownership-and-memory/" title="Mut">Mut</a> r} <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-var-swap" title="Permalink">#</a><a href="/packages/stdlib/Data.Mutable.Var/source/#L54" title="View source at line 54">Source</a></span></div>
<div class="doc-body">

Store `x` and return the value it replaced.

</div>
</article>

<article class="doc-entry" id="v-var-write">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">var-write</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Mutable/#t-Var" title="Data.Mutable.Var.Var">Var</a> <span class="tok-variable">r</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/10-ownership-and-memory/" title="Mut">Mut</a> r} <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code><span class="doc-sig-links"><a href="#v-var-write" title="Permalink">#</a><a href="/packages/stdlib/Data.Mutable.Var/source/#L40" title="View source at line 40">Source</a></span></div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Data/Mutable/Var.cinch`](/packages/stdlib/Data.Mutable.Var/source/). Source SHA-256: `1940d102ebcfdd02df9b0f81279f991e5419a480937179f0a635b1de17f25b26`.
