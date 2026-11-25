# Function.Combinators

Small reusable function operations such as identity, composition, argument manipulation, and application.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Function.Combinators</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Function.Combinators/source/">View source</a><a href="/packages/search/?q=%2BFunction.Combinators">Search module</a><a href="/reference-sources/Function.Combinators.cinch" download="Function.Combinators.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Algebra.Arithmetic](Algebra.Arithmetic.md), [Relation.Eq](Relation.Eq.md).

<details class="doc-notes"><summary>Module notes</summary>

Function.Combinators: combinators over plain functions.

Module scheme (design/base-library.md): `Function.*` is the kind of
combinators over plain functions. The pair projections (`fst`,
`snd`, `swap`, `curry`, `uncurry`) live here too: a tuple is a
language-level type with no home module of its own, and these are
function-level projections over a pair, the same kind of value as
the combinators around them, not list operations.

</details>

<details class="doc-synopsis" open><summary>Synopsis (13 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#v-apply-n-times">apply-n-times</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">apply-n-times</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span>) <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-apply-to">apply-to</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">apply-to</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span>. <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span>) <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span></code></li><li><a class="syn-name" href="#v-both">both</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">both</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span> <span class="tok-variable">c</span>. (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">c</span>) <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> (<span class="tok-variable">b</span>, <span class="tok-variable">c</span>)</code></li><li><a class="syn-name" href="#v-compose">compose</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">compose</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span> <span class="tok-variable">c</span>. (<span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">c</span>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span>) <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">c</span></code></li><li><a class="syn-name" href="#v-const">const</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">const</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span>. <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-curry">curry</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">curry</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span> <span class="tok-variable">c</span>. ((<span class="tok-variable">a</span>, <span class="tok-variable">b</span>) <span class="tok-operator">-&gt;</span> <span class="tok-variable">c</span>) <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">c</span></code></li><li><a class="syn-name" href="#v-flip">flip</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">flip</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span> <span class="tok-variable">c</span>. (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">c</span>) <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">c</span></code></li><li><a class="syn-name" href="#v-fst">fst</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">fst</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span>. (<span class="tok-variable">a</span>, <span class="tok-variable">b</span>) <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-identity">identity</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">identity</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-on">on</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">on</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span> <span class="tok-variable">c</span>. (<span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">c</span>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span>) <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">c</span></code></li><li><a class="syn-name" href="#v-snd">snd</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">snd</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span>. (<span class="tok-variable">a</span>, <span class="tok-variable">b</span>) <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span></code></li><li><a class="syn-name" href="#v-swap">swap</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">swap</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span>. (<span class="tok-variable">a</span>, <span class="tok-variable">b</span>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">b</span>, <span class="tok-variable">a</span>)</code></li><li><a class="syn-name" href="#v-uncurry">uncurry</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">uncurry</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span> <span class="tok-variable">c</span>. (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">c</span>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">a</span>, <span class="tok-variable">b</span>) <span class="tok-operator">-&gt;</span> <span class="tok-variable">c</span></code></li></ul>
</details>

# Values

<article class="doc-entry" id="v-apply-n-times">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">apply-n-times</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span>) <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-apply-n-times" title="Permalink">#</a><a href="/packages/stdlib/Function.Combinators/source/#L54" title="View source at line 54">Source</a></span></div>
<div class="doc-body">

`apply-n-times n f x` applies `f` to `x` `n` times: `f (f x)` at
`n` = 2 and `x` at `n` = 0. The count is a `Nat`, so the base case
is the zero test and the step decrements (the same `Nat` walk as
`Data.List`'s `take`).

</div>
</article>

<article class="doc-entry" id="v-apply-to">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">apply-to</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span>. <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span>) <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span></code><span class="doc-sig-links"><a href="#v-apply-to" title="Permalink">#</a><a href="/packages/stdlib/Function.Combinators/source/#L47" title="View source at line 47">Source</a></span></div>
<div class="doc-body">

Reverse application: `apply-to x f` is `f x` (the `Data.Function.(&)`
pipe). The name is `apply-to`, not an operator: `&` is bitwise AND
(Algebra.Bits) and `|>` is not in the base fixity table, so an
operator spelling would either collide or need a new table entry.

</div>
</article>

<article class="doc-entry" id="v-both">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">both</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span> <span class="tok-variable">c</span>. (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">c</span>) <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> (<span class="tok-variable">b</span>, <span class="tok-variable">c</span>)</code><span class="doc-sig-links"><a href="#v-both" title="Permalink">#</a><a href="/packages/stdlib/Function.Combinators/source/#L61" title="View source at line 61">Source</a></span></div>
<div class="doc-body">

`both f g x` is the pair of both projections of `x` (the fan-out
combinator, the `(&&&)` shape without the arrow machinery).

</div>
</article>

<article class="doc-entry" id="v-compose">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">compose</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span> <span class="tok-variable">c</span>. (<span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">c</span>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span>) <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">c</span></code><span class="doc-sig-links"><a href="#v-compose" title="Permalink">#</a><a href="/packages/stdlib/Function.Combinators/source/#L28" title="View source at line 28">Source</a></span></div>
<div class="doc-body">

`compose h g` applies `g` then `h` (the functor composition law's
named composition: a law equation states total function
applications, never a lambda; primitive operations and inline
target IR specification section 5).

</div>
</article>

<article class="doc-entry" id="v-const">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">const</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span>. <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-const" title="Permalink">#</a><a href="/packages/stdlib/Function.Combinators/source/#L21" title="View source at line 21">Source</a></span></div>
</article>

<article class="doc-entry" id="v-curry">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">curry</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span> <span class="tok-variable">c</span>. ((<span class="tok-variable">a</span>, <span class="tok-variable">b</span>) <span class="tok-operator">-&gt;</span> <span class="tok-variable">c</span>) <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">c</span></code><span class="doc-sig-links"><a href="#v-curry" title="Permalink">#</a><a href="/packages/stdlib/Function.Combinators/source/#L76" title="View source at line 76">Source</a></span></div>
</article>

<article class="doc-entry" id="v-flip">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">flip</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span> <span class="tok-variable">c</span>. (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">c</span>) <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">c</span></code><span class="doc-sig-links"><a href="#v-flip" title="Permalink">#</a><a href="/packages/stdlib/Function.Combinators/source/#L33" title="View source at line 33">Source</a></span></div>
<div class="doc-body">

`flip f` swaps `f`'s two arguments, so `flip (-) x y` is `y - x`
and `flip const x y` is `y` (`Data.Function.flip`).

</div>
</article>

<article class="doc-entry" id="v-fst">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">fst</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span>. (<span class="tok-variable">a</span>, <span class="tok-variable">b</span>) <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-fst" title="Permalink">#</a><a href="/packages/stdlib/Function.Combinators/source/#L64" title="View source at line 64">Source</a></span></div>
</article>

<article class="doc-entry" id="v-identity">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">identity</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-identity" title="Permalink">#</a><a href="/packages/stdlib/Function.Combinators/source/#L18" title="View source at line 18">Source</a></span></div>
</article>

<article class="doc-entry" id="v-on">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">on</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span> <span class="tok-variable">c</span>. (<span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">c</span>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span>) <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">c</span></code><span class="doc-sig-links"><a href="#v-on" title="Permalink">#</a><a href="/packages/stdlib/Function.Combinators/source/#L40" title="View source at line 40">Source</a></span></div>
<div class="doc-body">

`on g k` compares two values through `k`'s projections: `on (<)
fst` is the comparator on first components (the `Data.Function.on`
combinator, prefix here since the surface declares no `on`
fixity).

</div>
</article>

<article class="doc-entry" id="v-snd">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">snd</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span>. (<span class="tok-variable">a</span>, <span class="tok-variable">b</span>) <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span></code><span class="doc-sig-links"><a href="#v-snd" title="Permalink">#</a><a href="/packages/stdlib/Function.Combinators/source/#L68" title="View source at line 68">Source</a></span></div>
</article>

<article class="doc-entry" id="v-swap">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">swap</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span>. (<span class="tok-variable">a</span>, <span class="tok-variable">b</span>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">b</span>, <span class="tok-variable">a</span>)</code><span class="doc-sig-links"><a href="#v-swap" title="Permalink">#</a><a href="/packages/stdlib/Function.Combinators/source/#L72" title="View source at line 72">Source</a></span></div>
</article>

<article class="doc-entry" id="v-uncurry">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">uncurry</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span> <span class="tok-variable">c</span>. (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">c</span>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">a</span>, <span class="tok-variable">b</span>) <span class="tok-operator">-&gt;</span> <span class="tok-variable">c</span></code><span class="doc-sig-links"><a href="#v-uncurry" title="Permalink">#</a><a href="/packages/stdlib/Function.Combinators/source/#L79" title="View source at line 79">Source</a></span></div>
</article>
