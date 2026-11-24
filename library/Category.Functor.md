# Category.Functor

Higher-kinded mapping vocabulary, distinct from the lexical do-sequencing provider mechanism.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">Embedded</span> <code>import Category.Functor</code></div><div class="doc-mod-bar-links"><a href="/library/source/Category.Functor/">View source</a><a href="/library/search/?q=%2BCategory.Functor">Search module</a><a href="/reference-sources/Category.Functor.cinch" download="Category.Functor.cinch">Download .cinch</a><a href="/library/00-index/">All modules</a></div></div>

**Dependencies:** [Function.Combinators](Function.Combinators.md).

<details class="doc-notes"><summary>Module notes</summary>

Category.Functor: the functor class over type constructors.

The method is `map` (not `fmap`; decision D152's lisp-style naming),
and `<$>` is its operator spelling (decision D2: operators are class
methods with per-module fixity declarations, so `infixl 4 <$>`
below is the declaration consumers import). Both methods have the
same type; a lawful instance defines them identically.

Class shape: `Functor (f :: Type -> Type)` with the methods at the
applied container `f a` (higher-kinded parameters, modules
specification section 1).

`do` notation and these classes (decision D22): `do` picks its
provider lexically per definition, through a `using` binder with a
DirectSequencing/ResultSequencing/OptionSequencing head (default:
DirectSequencing). A do-provider is a fixed instance choice: exactly
one container per definition, no polymorphism. This class abstracts
over containers for polymorphic code; its methods are ordinary class
methods resolved per use site. The two mechanisms do not interact:
`do` never consults the class, and a class constraint never selects
a do provider.

Instance homes. The ownership rule is "instances live with their
type; structural instances over functions and tuples live with the
class module" (design/base-library.md). The function instance below
is structural and lives here permanently. The List, Option and
Result instances got theirs at the prelude split: they live in
Data.List/Data.Option/Data.Result now.

</details>

<details class="doc-synopsis" open><summary>Synopsis (3 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#c-Functor">Functor</a><code class="syn-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">Functor</strong> <span class="tok-variable">f</span></code></li><li class="syn-sub"><a class="syn-name" href="#m-Functor-map">map</a><code class="syn-sig"><strong class="tok-variable">map</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">f</span> :: <span class="tok-constructor">Type</span> <span class="tok-operator">-&gt;</span> <span class="tok-constructor">Type</span>) <span class="tok-variable">a</span> <span class="tok-variable">b</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Category.Functor/#c-Functor" title="Category.Functor.Functor">Functor</a> <span class="tok-variable">f</span>) <span class="tok-operator">=&gt;</span> (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span>) <span class="tok-operator">-&gt;</span> <span class="tok-variable">f</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">f</span> <span class="tok-variable">b</span></code></li><li class="syn-sub"><a class="syn-name" href="#m-Functor-op-3c243e">&lt;$&gt;</a><code class="syn-sig"><strong class="tok-variable">&lt;$&gt;</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">f</span> :: <span class="tok-constructor">Type</span> <span class="tok-operator">-&gt;</span> <span class="tok-constructor">Type</span>) <span class="tok-variable">a</span> <span class="tok-variable">b</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Category.Functor/#c-Functor" title="Category.Functor.Functor">Functor</a> <span class="tok-variable">f</span>) <span class="tok-operator">=&gt;</span> (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span>) <span class="tok-operator">-&gt;</span> <span class="tok-variable">f</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">f</span> <span class="tok-variable">b</span></code></li></ul>
</details>

# Signatures

<article class="doc-entry" id="c-Functor">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">Functor</strong> <span class="tok-variable">f</span></code><span class="doc-sig-links"><a href="#c-Functor" title="Permalink">#</a><a href="/library/source/Category.Functor/#L33" title="View source at line 33">Source</a></span></div>
<div class="doc-body">
<div class="doc-children"><h4>Methods</h4><ul><li id="m-Functor-map"><div class="doc-method-row"><code><strong class="tok-variable">map</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">f</span> :: <span class="tok-constructor">Type</span> <span class="tok-operator">-&gt;</span> <span class="tok-constructor">Type</span>) <span class="tok-variable">a</span> <span class="tok-variable">b</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Category.Functor/#c-Functor" title="Category.Functor.Functor">Functor</a> <span class="tok-variable">f</span>) <span class="tok-operator">=&gt;</span> (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span>) <span class="tok-operator">-&gt;</span> <span class="tok-variable">f</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">f</span> <span class="tok-variable">b</span></code><span class="doc-sig-links"><a href="#m-Functor-map" title="Permalink">#</a><a href="/library/source/Category.Functor/#L34">Source</a></span></div></li><li id="m-Functor-op-3c243e"><div class="doc-method-row"><code><strong class="tok-variable">&lt;$&gt;</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">f</span> :: <span class="tok-constructor">Type</span> <span class="tok-operator">-&gt;</span> <span class="tok-constructor">Type</span>) <span class="tok-variable">a</span> <span class="tok-variable">b</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Category.Functor/#c-Functor" title="Category.Functor.Functor">Functor</a> <span class="tok-variable">f</span>) <span class="tok-operator">=&gt;</span> (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span>) <span class="tok-operator">-&gt;</span> <span class="tok-variable">f</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">f</span> <span class="tok-variable">b</span></code><span class="doc-sig-links"><a href="#m-Functor-op-3c243e" title="Permalink">#</a><a href="/library/source/Category.Functor/#L35">Source</a></span></div></li></ul></div>
<details class="doc-instances"><summary>Instances (7)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/library/Category.Functor/#c-Functor" title="Category.Functor.Functor">Functor</a> ((-&gt;) r)</code> <span class="doc-inst-meta">implicit · <a href="/library/Category.Functor/#instances">Category.Functor</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Category.Functor/#c-Functor" title="Category.Functor.Functor">Functor</a> (<a class="doc-type-link tok-constructor" href="/library/Data.Const/#t-Const" title="Data.Const.Const">Const</a> <span class="tok-variable">m</span>)</code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Const/#instances">Data.Const</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Category.Functor/#c-Functor" title="Category.Functor.Functor">Functor</a> <a class="doc-type-link tok-constructor" href="/library/Data.Identity/#t-Identity" title="Data.Identity.Identity">Identity</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Identity/#instances">Data.Identity</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Category.Functor/#c-Functor" title="Category.Functor.Functor">Functor</a> <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.List/#instances">Data.List</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Category.Functor/#c-Functor" title="Category.Functor.Functor">Functor</a> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Option/#instances">Data.Option</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Category.Functor/#c-Functor" title="Category.Functor.Functor">Functor</a> (<a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <span class="tok-variable">e</span>)</code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Result/#instances">Data.Result</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Category.Functor/#c-Functor" title="Category.Functor.Functor">Functor</a> <a class="doc-type-link tok-constructor" href="/library/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Effect.Stm/#instances">Effect.Stm</a></span></li></ul></details>
</div>
</article>

# Operator fixities

| Operator | Associativity | Precedence |
| --- | --- | --- |
| `<$>` | `infixl` | `4` |

# Instances

Exported checked instances declared in this module. For law checking, see [rewrites and optimization](../reference/12-rewrites-and-optimization.md).

<ul class="doc-module-instances">
<li id="i-Functor_Function"><code><a class="doc-type-link tok-constructor" href="/library/Category.Functor/#c-Functor" title="Category.Functor.Functor">Functor</a> ((-&gt;) r)</code> <span class="doc-inst-meta">Implicit · <code>Category.Functor.Functor_Function</code></span><div class="doc-child-prose">Structural: map over a function's result (the reader functor).</div></li>
</ul>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Category/Functor.cinch`](/library/source/Category.Functor/). Source SHA-256: `2b1b8078f2abdf140b110074f4f80f22ba2b6ea54d72baf15125ea8311639ba1`.
