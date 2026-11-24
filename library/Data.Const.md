# Data.Const

A constant container that retains its first parameter while ignoring the mapped value parameter.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Data.Const</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Data.Const/source/">View source</a><a href="/packages/search/?q=%2BData.Const">Search module</a><a href="/reference-sources/Data.Const.cinch" download="Data.Const.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Algebra.Monoid](Algebra.Monoid.md), [Algebra.Semigroup](Algebra.Semigroup.md), [Category.Applicative](Category.Applicative.md), [Category.Functor](Category.Functor.md).

<details class="doc-notes"><summary>Module notes</summary>

The constant functor: `Const a b` carries an `a` and erases the `b`.
It is the universal visitor for computations whose result type is
fixed, and the composing piece the Category.* classes use as their
lawful constant functor.

Module name: `Data.Const` (the module scheme's name,
design/base-library.md; the temporary single-segment header is gone
now that dotted module names resolve).

`do` notation and these classes (decision D22): `do` picks its
provider lexically per definition, through a `using` binder with a
DirectSequencing/ResultSequencing/OptionSequencing head (default:
DirectSequencing). A do-provider is a fixed instance choice: exactly
one container per definition, no polymorphism. The Category.*
classes (Functor/Applicative/Monad) abstract over containers for
polymorphic code; their `map`/`pure`/`>>=` are ordinary class
methods resolved per use site. The two mechanisms do not interact
and neither is changing: `do` never consults the classes, and a class
constraint never selects a do provider.

The functions and the instances: `map-const` is the Functor `map`
body for `Const` (the class name is `map`, decision D152), and the
Functor instance at the bottom is defined through it (instances live
with their type). `Const`'s Applicative lands with the prelude
split (D157's waiter is discharged): its `pure` fabricates the
`Monoid` payload and its `<*>` combines payloads under `Semigroup`,
so the instance states both constraints (a class constraint entails
nothing; the standing no-superbundle rule). The operator `<$>` is a
class method per decision D2 (operators are class methods with
per-module fixity declarations), so no per-type operator stand-in is
defined here.

</details>

<details class="doc-synopsis" open><summary>Synopsis (3 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-Const">Const</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Const</strong> <span class="tok-variable">a</span> <span class="tok-variable">b</span></code></li><li><a class="syn-name" href="#v-get-const">get-const</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">get-const</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Const/#t-Const" title="Data.Const.Const">Const</a> <span class="tok-variable">a</span> <span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-map-const">map-const</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">map-const</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">m</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span>. (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Const/#t-Const" title="Data.Const.Const">Const</a> <span class="tok-variable">m</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Const/#t-Const" title="Data.Const.Const">Const</a> <span class="tok-variable">m</span> <span class="tok-variable">b</span></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-Const">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Const</strong> <span class="tok-variable">a</span> <span class="tok-variable">b</span></code><span class="doc-sig-links"><a href="#t-Const" title="Permalink">#</a><a href="/packages/stdlib/Data.Const/source/#L40" title="View source at line 40">Source</a></span></div>
<div class="doc-body">
<div class="doc-children"><h4>Constructors</h4><ul><li id="k-Const"><code><strong class="tok-constructor">Const</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span>. <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Const/#t-Const" title="Data.Const.Const">Const</a> <span class="tok-variable">a</span> <span class="tok-variable">b</span></code></li></ul></div>
<details class="doc-instances"><summary>Instances (2)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Category.Applicative/#c-Applicative" title="Category.Applicative.Applicative">Applicative</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Const/#t-Const" title="Data.Const.Const">Const</a> <span class="tok-variable">m</span>)</code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Const/#instances">Data.Const</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Category.Functor/#c-Functor" title="Category.Functor.Functor">Functor</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Const/#t-Const" title="Data.Const.Const">Const</a> <span class="tok-variable">m</span>)</code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Const/#instances">Data.Const</a></span></li></ul></details>
</div>
</article>

# Values

<article class="doc-entry" id="v-get-const">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">get-const</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Const/#t-Const" title="Data.Const.Const">Const</a> <span class="tok-variable">a</span> <span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-get-const" title="Permalink">#</a><a href="/packages/stdlib/Data.Const/source/#L43" title="View source at line 43">Source</a></span></div>
<div class="doc-body">

Elimination: the carried value.

</div>
</article>

<article class="doc-entry" id="v-map-const">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">map-const</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">m</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span>. (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Const/#t-Const" title="Data.Const.Const">Const</a> <span class="tok-variable">m</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Const/#t-Const" title="Data.Const.Const">Const</a> <span class="tok-variable">m</span> <span class="tok-variable">b</span></code><span class="doc-sig-links"><a href="#v-map-const" title="Permalink">#</a><a href="/packages/stdlib/Data.Const/source/#L49" title="View source at line 49">Source</a></span></div>
<div class="doc-body">

Functor body: the carried value is unchanged and the function is
ignored (the constant functor).

</div>
</article>

# Instances

Exported checked instances declared in this module. For law checking, see [rewrites and optimization](../reference/12-rewrites-and-optimization.md).

<ul class="doc-module-instances">
<li id="i-Applicative_Const"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Category.Applicative/#c-Applicative" title="Category.Applicative.Applicative">Applicative</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Const/#t-Const" title="Data.Const.Const">Const</a> <span class="tok-variable">m</span>)</code> <span class="doc-inst-meta">Implicit · <code>Data.Const.Applicative_Const</code></span><div class="doc-child-prose">The applicative (D157's waiter is discharged at the prelude split): pure fabricates the Monoid payload and &lt;&gt; combines payloads under Semigroup.</div></li>
<li id="i-Functor_Const"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Category.Functor/#c-Functor" title="Category.Functor.Functor">Functor</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Const/#t-Const" title="Data.Const.Const">Const</a> <span class="tok-variable">m</span>)</code> <span class="doc-inst-meta">Implicit · <code>Data.Const.Functor_Const</code></span><div class="doc-child-prose">The class instance (instances live with their type), defined through the body above.</div></li>
</ul>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Data/Const.cinch`](/packages/stdlib/Data.Const/source/). Source SHA-256: `6ab8e9bf59d407a77e3a2f78ded46730596593352c4c948f9896d9fa0917a26c`.
