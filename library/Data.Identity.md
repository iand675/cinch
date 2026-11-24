# Data.Identity

The identity container and its generic abstraction instances.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">Embedded</span> <code>import Data.Identity</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Data.Identity/source/">View source</a><a href="/packages/search/?q=%2BData.Identity">Search module</a><a href="/reference-sources/Data.Identity.cinch" download="Data.Identity.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Category.Applicative](Category.Applicative.md), [Category.Functor](Category.Functor.md), [Category.Monad](Category.Monad.md).

<details class="doc-notes"><summary>Module notes</summary>

The identity type: the universal visitor that changes nothing. It is
the unit of the category family (the identity functor and the trivial
monad), so its operations below are the instance method bodies the
Category.* classes carry as this type's instances.

Module name: `Data.Identity` (the module scheme's name,
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

The functions and the instances: the functions below are the
instance method bodies (Functor `map`, Applicative `pure`/`<*>`/
`lift-a2`, Monad `>>=`/`join`; there is no `return`, decision D152).
The classes have landed (higher-kinded parameters), so the instances
at the bottom are defined through these bodies: the plain functions
stay as the type's direct API and the class methods resolve to them.
The operators `<$>`, `<*>` and `>>=` are class methods per decision
D2 (operators are class methods with per-module fixity
declarations), so no per-type operator stand-ins are defined here.

</details>

<details class="doc-synopsis" open><summary>Synopsis (8 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-Identity">Identity</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Identity</strong> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-ap-identity">ap-identity</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ap-identity</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Identity/#t-Identity" title="Data.Identity.Identity">Identity</a> (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Identity/#t-Identity" title="Data.Identity.Identity">Identity</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Identity/#t-Identity" title="Data.Identity.Identity">Identity</a> <span class="tok-variable">b</span></code></li><li><a class="syn-name" href="#v-bind-identity">bind-identity</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">bind-identity</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Identity/#t-Identity" title="Data.Identity.Identity">Identity</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Identity/#t-Identity" title="Data.Identity.Identity">Identity</a> <span class="tok-variable">b</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Identity/#t-Identity" title="Data.Identity.Identity">Identity</a> <span class="tok-variable">b</span></code></li><li><a class="syn-name" href="#v-join-identity">join-identity</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">join-identity</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Identity/#t-Identity" title="Data.Identity.Identity">Identity</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Identity/#t-Identity" title="Data.Identity.Identity">Identity</a> <span class="tok-variable">a</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Identity/#t-Identity" title="Data.Identity.Identity">Identity</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-lift-a2-identity">lift-a2-identity</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">lift-a2-identity</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span> <span class="tok-variable">c</span>. (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">c</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Identity/#t-Identity" title="Data.Identity.Identity">Identity</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Identity/#t-Identity" title="Data.Identity.Identity">Identity</a> <span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Identity/#t-Identity" title="Data.Identity.Identity">Identity</a> <span class="tok-variable">c</span></code></li><li><a class="syn-name" href="#v-map-identity">map-identity</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">map-identity</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span>. (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Identity/#t-Identity" title="Data.Identity.Identity">Identity</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Identity/#t-Identity" title="Data.Identity.Identity">Identity</a> <span class="tok-variable">b</span></code></li><li><a class="syn-name" href="#v-pure-identity">pure-identity</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pure-identity</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Identity/#t-Identity" title="Data.Identity.Identity">Identity</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-run-identity">run-identity</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">run-identity</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Identity/#t-Identity" title="Data.Identity.Identity">Identity</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-Identity">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Identity</strong> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#t-Identity" title="Permalink">#</a><a href="/packages/stdlib/Data.Identity/source/#L37" title="View source at line 37">Source</a></span></div>
<div class="doc-body">
<div class="doc-children"><h4>Constructors</h4><ul><li id="k-Identity"><code><strong class="tok-constructor">Identity</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Identity/#t-Identity" title="Data.Identity.Identity">Identity</a> <span class="tok-variable">a</span></code></li></ul></div>
<details class="doc-instances"><summary>Instances (3)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Category.Applicative/#c-Applicative" title="Category.Applicative.Applicative">Applicative</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Identity/#t-Identity" title="Data.Identity.Identity">Identity</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Identity/#instances">Data.Identity</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Category.Functor/#c-Functor" title="Category.Functor.Functor">Functor</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Identity/#t-Identity" title="Data.Identity.Identity">Identity</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Identity/#instances">Data.Identity</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Category.Monad/#c-Monad" title="Category.Monad.Monad">Monad</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Identity/#t-Identity" title="Data.Identity.Identity">Identity</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Identity/#instances">Data.Identity</a></span></li></ul></details>
</div>
</article>

# Values

<article class="doc-entry" id="v-ap-identity">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ap-identity</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Identity/#t-Identity" title="Data.Identity.Identity">Identity</a> (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Identity/#t-Identity" title="Data.Identity.Identity">Identity</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Identity/#t-Identity" title="Data.Identity.Identity">Identity</a> <span class="tok-variable">b</span></code><span class="doc-sig-links"><a href="#v-ap-identity" title="Permalink">#</a><a href="/packages/stdlib/Data.Identity/source/#L54" title="View source at line 54">Source</a></span></div>
<div class="doc-body">

Applicative body: apply a wrapped function to a wrapped value.

</div>
</article>

<article class="doc-entry" id="v-bind-identity">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">bind-identity</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Identity/#t-Identity" title="Data.Identity.Identity">Identity</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Identity/#t-Identity" title="Data.Identity.Identity">Identity</a> <span class="tok-variable">b</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Identity/#t-Identity" title="Data.Identity.Identity">Identity</a> <span class="tok-variable">b</span></code><span class="doc-sig-links"><a href="#v-bind-identity" title="Permalink">#</a><a href="/packages/stdlib/Data.Identity/source/#L63" title="View source at line 63">Source</a></span></div>
<div class="doc-body">

Monad body: sequence a dependent computation.

</div>
</article>

<article class="doc-entry" id="v-join-identity">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">join-identity</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Identity/#t-Identity" title="Data.Identity.Identity">Identity</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Identity/#t-Identity" title="Data.Identity.Identity">Identity</a> <span class="tok-variable">a</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Identity/#t-Identity" title="Data.Identity.Identity">Identity</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-join-identity" title="Permalink">#</a><a href="/packages/stdlib/Data.Identity/source/#L68" title="View source at line 68">Source</a></span></div>
<div class="doc-body">

Monad body: collapse one layer.

</div>
</article>

<article class="doc-entry" id="v-lift-a2-identity">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">lift-a2-identity</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span> <span class="tok-variable">c</span>. (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">c</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Identity/#t-Identity" title="Data.Identity.Identity">Identity</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Identity/#t-Identity" title="Data.Identity.Identity">Identity</a> <span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Identity/#t-Identity" title="Data.Identity.Identity">Identity</a> <span class="tok-variable">c</span></code><span class="doc-sig-links"><a href="#v-lift-a2-identity" title="Permalink">#</a><a href="/packages/stdlib/Data.Identity/source/#L59" title="View source at line 59">Source</a></span></div>
<div class="doc-body">

Applicative body: lift a two-argument function.

</div>
</article>

<article class="doc-entry" id="v-map-identity">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">map-identity</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span>. (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Identity/#t-Identity" title="Data.Identity.Identity">Identity</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Identity/#t-Identity" title="Data.Identity.Identity">Identity</a> <span class="tok-variable">b</span></code><span class="doc-sig-links"><a href="#v-map-identity" title="Permalink">#</a><a href="/packages/stdlib/Data.Identity/source/#L45" title="View source at line 45">Source</a></span></div>
<div class="doc-body">

Functor body: map over the wrapped value.

</div>
</article>

<article class="doc-entry" id="v-pure-identity">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pure-identity</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Identity/#t-Identity" title="Data.Identity.Identity">Identity</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-pure-identity" title="Permalink">#</a><a href="/packages/stdlib/Data.Identity/source/#L50" title="View source at line 50">Source</a></span></div>
<div class="doc-body">

Applicative body.

</div>
</article>

<article class="doc-entry" id="v-run-identity">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">run-identity</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Identity/#t-Identity" title="Data.Identity.Identity">Identity</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-run-identity" title="Permalink">#</a><a href="/packages/stdlib/Data.Identity/source/#L40" title="View source at line 40">Source</a></span></div>
<div class="doc-body">

Elimination: the wrapped value.

</div>
</article>

# Instances

Exported checked instances declared in this module. For law checking, see [rewrites and optimization](../reference/12-rewrites-and-optimization.md).

<ul class="doc-module-instances">
<li id="i-Applicative_Identity"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Category.Applicative/#c-Applicative" title="Category.Applicative.Applicative">Applicative</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Identity/#t-Identity" title="Data.Identity.Identity">Identity</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Identity.Applicative_Identity</code></span></li>
<li id="i-Functor_Identity"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Category.Functor/#c-Functor" title="Category.Functor.Functor">Functor</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Identity/#t-Identity" title="Data.Identity.Identity">Identity</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Identity.Functor_Identity</code></span><div class="doc-child-prose">The class instances (instances live with their type), defined through the bodies above.</div></li>
<li id="i-Monad_Identity"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Category.Monad/#c-Monad" title="Category.Monad.Monad">Monad</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Identity/#t-Identity" title="Data.Identity.Identity">Identity</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Identity.Monad_Identity</code></span></li>
</ul>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Data/Identity.cinch`](/packages/stdlib/Data.Identity/source/). Source SHA-256: `2429ddda85cbdaaef07d640c7df7cb7e78e6537c30ff3920ac0ca1675a5922e9`.
