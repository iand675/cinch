# Data.Result

Success and named failure values, error-aware transformations, partitioning, and sequencing instances.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Data.Result</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Data.Result/source/">View source</a><a href="/packages/search/?q=%2BData.Result">Search module</a><a href="/reference-sources/Data.Result.cinch" download="Data.Result.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Category.Applicative](Category.Applicative.md), [Category.Functor](Category.Functor.md), [Category.Monad](Category.Monad.md), [Data.Core](Data.Core.md), [Effect.Sequencing](Effect.Sequencing.md).

<details class="doc-notes"><summary>Module notes</summary>

Data.Result: a computation's outcome: `Err e` or `Ok x`.

Module scheme (design/base-library.md): the type's home carries its
instance family (the parked-instance deviation of D159 is over) and
its D22 sequencing provider instance (`ResultSequencing`: `do`
short-circuits on the first `Err`; the class lives in
Effect.Sequencing).

</details>

<details class="doc-synopsis" open><summary>Synopsis (12 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-Result">Result</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Result</strong> <span class="tok-variable">e</span> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-either">either</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">either</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">e</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span>. (<span class="tok-variable">e</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <span class="tok-variable">e</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span></code></li><li><a class="syn-name" href="#v-errors">errors</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">errors</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">e</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <span class="tok-variable">e</span> <span class="tok-variable">a</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">e</span></code></li><li><a class="syn-name" href="#v-from-result">from-result</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">from-result</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">e</span> <span class="tok-variable">a</span>. <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <span class="tok-variable">e</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-is-err">is-err</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">is-err</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">e</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <span class="tok-variable">e</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-is-ok">is-ok</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">is-ok</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">e</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <span class="tok-variable">e</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-map-err">map-err</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">map-err</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">e</span> <span class="tok-variable">f</span> <span class="tok-variable">a</span>. (<span class="tok-variable">e</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">f</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <span class="tok-variable">e</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <span class="tok-variable">f</span> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-map-result">map-result</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">map-result</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">e</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span>. (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <span class="tok-variable">e</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <span class="tok-variable">e</span> <span class="tok-variable">b</span></code></li><li><a class="syn-name" href="#v-option-to-result">option-to-result</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">option-to-result</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">e</span> <span class="tok-variable">a</span>. <span class="tok-variable">e</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <span class="tok-variable">e</span> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-partition-results">partition-results</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">partition-results</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">e</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <span class="tok-variable">e</span> <span class="tok-variable">a</span>) <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">e</span>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span>)</code></li><li><a class="syn-name" href="#v-result-to-option">result-to-option</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">result-to-option</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">e</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <span class="tok-variable">e</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-results">results</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">results</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">e</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <span class="tok-variable">e</span> <span class="tok-variable">a</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-Result">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Result</strong> <span class="tok-variable">e</span> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#t-Result" title="Permalink">#</a><a href="/packages/stdlib/Data.Core/source/#L37" title="View source at line 37">Source</a></span></div>
<div class="doc-body">

`Result` is here for the same reason: `Data.Result`'s
`partition-results` and `results` consume a `List`, and it too is
checked before `Data.List`.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-Err"><code><strong class="tok-constructor">Err</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">e</span> <span class="tok-variable">a</span>. <span class="tok-variable">e</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <span class="tok-variable">e</span> <span class="tok-variable">a</span></code></li><li id="k-Ok"><code><strong class="tok-constructor">Ok</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">e</span> <span class="tok-variable">a</span>. <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <span class="tok-variable">e</span> <span class="tok-variable">a</span></code></li></ul></div>
<details class="doc-instances"><summary>Instances (4)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Finite/#c-Finite" title="Data.Finite.Finite">Finite</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <span class="tok-variable">e</span> <span class="tok-variable">a</span>)</code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Finite/#instances">Data.Finite</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Category.Applicative/#c-Applicative" title="Category.Applicative.Applicative">Applicative</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <span class="tok-variable">e</span>)</code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Result/#instances">Data.Result</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Category.Functor/#c-Functor" title="Category.Functor.Functor">Functor</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <span class="tok-variable">e</span>)</code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Result/#instances">Data.Result</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Category.Monad/#c-Monad" title="Category.Monad.Monad">Monad</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <span class="tok-variable">e</span>)</code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Result/#instances">Data.Result</a></span></li></ul></details>
</div>
</article>

# Values

<article class="doc-entry" id="v-either">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">either</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">e</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span>. (<span class="tok-variable">e</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <span class="tok-variable">e</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span></code><span class="doc-sig-links"><a href="#v-either" title="Permalink">#</a><a href="/packages/stdlib/Data.Result/source/#L41" title="View source at line 41">Source</a></span></div>
<div class="doc-body">

The two-branch fold: `f` consumes an error, `g` a value. The error
branch leads, matching Haskell's `either` and this type's
constructor order (the error is the first argument everywhere,
including `Result e a` itself).

</div>
</article>

<article class="doc-entry" id="v-errors">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">errors</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">e</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <span class="tok-variable">e</span> <span class="tok-variable">a</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">e</span></code><span class="doc-sig-links"><a href="#v-errors" title="Permalink">#</a><a href="/packages/stdlib/Data.Result/source/#L104" title="View source at line 104">Source</a></span></div>
<div class="doc-body">

The error projection, the values dropped (`Data.Either.lefts`).

</div>
</article>

<article class="doc-entry" id="v-from-result">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">from-result</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">e</span> <span class="tok-variable">a</span>. <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <span class="tok-variable">e</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-from-result" title="Permalink">#</a><a href="/packages/stdlib/Data.Result/source/#L75" title="View source at line 75">Source</a></span></div>
<div class="doc-body">

The value, or the default when there is an error
(`Data.Either.fromRight`); the default leads, like `from-option`'s.

</div>
</article>

<article class="doc-entry" id="v-is-err">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">is-err</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">e</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <span class="tok-variable">e</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-is-err" title="Permalink">#</a><a href="/packages/stdlib/Data.Result/source/#L52" title="View source at line 52">Source</a></span></div>
</article>

<article class="doc-entry" id="v-is-ok">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">is-ok</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">e</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <span class="tok-variable">e</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-is-ok" title="Permalink">#</a><a href="/packages/stdlib/Data.Result/source/#L47" title="View source at line 47">Source</a></span></div>
<div class="doc-body">

The branch tests: the tag read, no payload work, no constraint.

</div>
</article>

<article class="doc-entry" id="v-map-err">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">map-err</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">e</span> <span class="tok-variable">f</span> <span class="tok-variable">a</span>. (<span class="tok-variable">e</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">f</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <span class="tok-variable">e</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <span class="tok-variable">f</span> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-map-err" title="Permalink">#</a><a href="/packages/stdlib/Data.Result/source/#L68" title="View source at line 68">Source</a></span></div>
<div class="doc-body">

The error's map (`Data.Either`'s `first`, `mapLeft` in ML-family
spellings: it maps the LEFT type). The value side is untouched.

</div>
</article>

<article class="doc-entry" id="v-map-result">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">map-result</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">e</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span>. (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <span class="tok-variable">e</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <span class="tok-variable">e</span> <span class="tok-variable">b</span></code><span class="doc-sig-links"><a href="#v-map-result" title="Permalink">#</a><a href="/packages/stdlib/Data.Result/source/#L61" title="View source at line 61">Source</a></span></div>
<div class="doc-body">

The value's own map, for a caller who has only the prelude: the
`Functor` method of the same name (`map` in Data.List's surface) is
reached through its constraint or with `import Category.Functor`.
The error side is untouched.

</div>
</article>

<article class="doc-entry" id="v-option-to-result">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">option-to-result</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">e</span> <span class="tok-variable">a</span>. <span class="tok-variable">e</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <span class="tok-variable">e</span> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-option-to-result" title="Permalink">#</a><a href="/packages/stdlib/Data.Result/source/#L123" title="View source at line 123">Source</a></span></div>
<div class="doc-body">

The option as a result, with the error value supplied for `None`
(`Data.Either.note`); the error leads, as everywhere in this module.

</div>
</article>

<article class="doc-entry" id="v-partition-results">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">partition-results</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">e</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <span class="tok-variable">e</span> <span class="tok-variable">a</span>) <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">e</span>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span>)</code><span class="doc-sig-links"><a href="#v-partition-results" title="Permalink">#</a><a href="/packages/stdlib/Data.Result/source/#L84" title="View source at line 84">Source</a></span></div>
<div class="doc-body">

The classic `partitionEithers`: every error and every value, each
side in the input order with its relative order preserved. One pass,
and the recursion happens once per element (the returned tuple is
taken apart by pattern, so neither side is walked twice).

</div>
</article>

<article class="doc-entry" id="v-result-to-option">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">result-to-option</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">e</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <span class="tok-variable">e</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-result-to-option" title="Permalink">#</a><a href="/packages/stdlib/Data.Result/source/#L116" title="View source at line 116">Source</a></span></div>
<div class="doc-body">

The value as an option, the error dropped. This is the projection a
caller reaches for in this repo because `Ok` is the success; the
error-side twin is `Data.Either`'s `hush` and is deliberately not
added (its name would have to be spelled for the error side, and no
caller has asked for it).

</div>
</article>

<article class="doc-entry" id="v-results">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">results</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">e</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <span class="tok-variable">e</span> <span class="tok-variable">a</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-results" title="Permalink">#</a><a href="/packages/stdlib/Data.Result/source/#L96" title="View source at line 96">Source</a></span></div>
<div class="doc-body">

The value projection, the errors dropped (`Data.Either.rights`).
`partition-results` is the pair form; these two are the single-side
walks a caller reaches for when it wants one side only.

</div>
</article>

# Instances

Exported checked instances declared in this module. For law checking, see [rewrites and optimization](../reference/12-rewrites-and-optimization.md).

<ul class="doc-module-instances">
<li id="i-Applicative_Result"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Category.Applicative/#c-Applicative" title="Category.Applicative.Applicative">Applicative</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <span class="tok-variable">e</span>)</code> <span class="doc-inst-meta">Implicit · <code>Data.Result.Applicative_Result</code></span><div class="doc-child-prose">pure is Ok; application propagates the FIRST Err (the left operand's error wins), which is what makes traverse into Result short-circuit on the first error in traversal order (moved from Category.Applicative).</div></li>
<li id="i-Functor_Result"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Category.Functor/#c-Functor" title="Category.Functor.Functor">Functor</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <span class="tok-variable">e</span>)</code> <span class="doc-inst-meta">Implicit · <code>Data.Result.Functor_Result</code></span><div class="doc-child-prose">The error side is untouched: map f (Err e) is Err e (moved from Category.Functor at the prelude split).</div></li>
<li id="i-Monad_Result"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Category.Monad/#c-Monad" title="Category.Monad.Monad">Monad</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <span class="tok-variable">e</span>)</code> <span class="doc-inst-meta">Implicit · <code>Data.Result.Monad_Result</code></span><div class="doc-child-prose">The error case short-circuits: Err e &gt;&gt;= k is Err e (moved from Category.Monad).</div></li>
<li id="i-SeqResult"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Sequencing/#c-ResultSequencing" title="Effect.Sequencing.ResultSequencing">ResultSequencing</a> <span class="tok-variable">e</span></code> <span class="doc-inst-meta">Implicit · <code>Data.Result.SeqResult</code></span></li>
</ul>
