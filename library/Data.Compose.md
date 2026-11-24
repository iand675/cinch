# Data.Compose

Composition of higher-kinded containers with the admitted generic instances.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Data.Compose</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Data.Compose/source/">View source</a><a href="/packages/search/?q=%2BData.Compose">Search module</a><a href="/reference-sources/Data.Compose.cinch" download="Data.Compose.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

<details class="doc-notes"><summary>Module notes</summary>

Data.Compose: composition of type constructors, the composing piece
of the category family.

`Compose f g a` wraps `f (g a)`: the composite of `f` after `g` at
`a`. `get-compose` is the elimination.

Instance homes: the Functor/Applicative/Monad instances of the
composite arrive with the prelude split's instance homes (they need
the component instances as constraints); the type ships first so the
family's shape is complete.

</details>

<details class="doc-synopsis" open><summary>Synopsis (2 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-Compose">Compose</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Compose</strong> <span class="tok-variable">f</span> <span class="tok-variable">g</span> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-get-compose">get-compose</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">get-compose</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">f</span> :: <span class="tok-constructor">Type</span> <span class="tok-operator">-&gt;</span> <span class="tok-constructor">Type</span>) (<span class="tok-variable">g</span> :: <span class="tok-constructor">Type</span> <span class="tok-operator">-&gt;</span> <span class="tok-constructor">Type</span>) <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Compose/#t-Compose" title="Data.Compose.Compose">Compose</a> <span class="tok-variable">f</span> <span class="tok-variable">g</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">f</span> (<span class="tok-variable">g</span> <span class="tok-variable">a</span>)</code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-Compose">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Compose</strong> <span class="tok-variable">f</span> <span class="tok-variable">g</span> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#t-Compose" title="Permalink">#</a><a href="/packages/stdlib/Data.Compose/source/#L14" title="View source at line 14">Source</a></span></div>
<div class="doc-body">
<div class="doc-children"><h4>Constructors</h4><ul><li id="k-Compose"><code><strong class="tok-constructor">Compose</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">f</span> <span class="tok-variable">g</span> <span class="tok-variable">a</span>. <span class="tok-variable">f</span> (<span class="tok-variable">g</span> <span class="tok-variable">a</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Compose/#t-Compose" title="Data.Compose.Compose">Compose</a> <span class="tok-variable">f</span> <span class="tok-variable">g</span> <span class="tok-variable">a</span></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-get-compose">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">get-compose</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">f</span> :: <span class="tok-constructor">Type</span> <span class="tok-operator">-&gt;</span> <span class="tok-constructor">Type</span>) (<span class="tok-variable">g</span> :: <span class="tok-constructor">Type</span> <span class="tok-operator">-&gt;</span> <span class="tok-constructor">Type</span>) <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Compose/#t-Compose" title="Data.Compose.Compose">Compose</a> <span class="tok-variable">f</span> <span class="tok-variable">g</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">f</span> (<span class="tok-variable">g</span> <span class="tok-variable">a</span>)</code><span class="doc-sig-links"><a href="#v-get-compose" title="Permalink">#</a><a href="/packages/stdlib/Data.Compose/source/#L16" title="View source at line 16">Source</a></span></div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Data/Compose.cinch`](/packages/stdlib/Data.Compose/source/). Source SHA-256: `e3ccc746196d71bfa0a30c6e77e7faaef88b3373244beee3a051ffbe1b205b80`.
