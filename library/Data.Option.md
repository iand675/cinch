# Data.Option

Optional values with None and Some, plus combinators for propagating or consuming absence.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Data.Option</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Data.Option/source/">View source</a><a href="/packages/search/?q=%2BData.Option">Search module</a><a href="/reference-sources/Data.Option.cinch" download="Data.Option.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Algebra.Monoid](Algebra.Monoid.md), [Algebra.Semigroup](Algebra.Semigroup.md), [Category.Alternative](Category.Alternative.md), [Category.Applicative](Category.Applicative.md), [Category.Foldable](Category.Foldable.md), [Category.Functor](Category.Functor.md), [Category.Monad](Category.Monad.md), [Category.Traversable](Category.Traversable.md), [Data.Core](Data.Core.md), [Effect.Sequencing](Effect.Sequencing.md).

<details class="doc-notes"><summary>Module notes</summary>

Data.Option: the optional value: `None` or `Some x`.

Module scheme (design/base-library.md): `Data.*` homes are the
concrete types; every instance lives with its type, so the instance
family for `Option` is here (the parked-instance deviation of D159
is over). The class modules own only their structural instances.

The D22 sequencing provider (`OptionSequencing`) is a property of
the container, so its provider instance is here too: one instance
per provider class keeps `do` selection order-independent (decision
D22; the classes live in Effect.Sequencing).

</details>

<details class="doc-synopsis" open><summary>Synopsis (10 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-Option">Option</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Option</strong> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-cat-options">cat-options</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cat-options</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">a</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-from-option">from-option</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">from-option</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-is-none">is-none</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">is-none</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-is-some">is-some</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">is-some</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-list-to-option">list-to-option</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">list-to-option</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-map-option">map-option</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">map-option</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span>. (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">b</span></code></li><li><a class="syn-name" href="#v-maybe">maybe</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">maybe</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span>. <span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span> (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span></code></li><li><a class="syn-name" href="#v-option-to-list">option-to-list</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">option-to-list</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-or-else">or-else</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">or-else</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">a</span></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-Option">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Option</strong> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#t-Option" title="Permalink">#</a><a href="/packages/stdlib/Data.Core/source/#L30" title="View source at line 30">Source</a></span></div>
<div class="doc-body">
<div class="doc-children"><h4>Constructors</h4><ul><li id="k-None"><code><strong class="tok-constructor">None</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">a</span></code></li><li id="k-Some"><code><strong class="tok-constructor">Some</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">a</span></code></li></ul></div>
<details class="doc-instances"><summary>Instances (11)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Finite/#c-Finite" title="Data.Finite.Finite">Finite</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">a</span>)</code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Finite/#instances">Data.Finite</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#c-MapHashable" title="Data.MapHash.MapHashable">MapHashable</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">a</span>)</code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.MapHash/#instances">Data.MapHash</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Category.Alternative/#c-Alternative" title="Category.Alternative.Alternative">Alternative</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Option/#instances">Data.Option</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Category.Applicative/#c-Applicative" title="Category.Applicative.Applicative">Applicative</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Option/#instances">Data.Option</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Category.Foldable/#c-Foldable" title="Category.Foldable.Foldable">Foldable</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Option/#instances">Data.Option</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Category.Functor/#c-Functor" title="Category.Functor.Functor">Functor</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Option/#instances">Data.Option</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Category.Monad/#c-Monad" title="Category.Monad.Monad">Monad</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Option/#instances">Data.Option</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Monoid/#c-Monoid" title="Algebra.Monoid.Monoid">Monoid</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">t</span>)</code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Option/#instances">Data.Option</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Semigroup/#c-Semigroup" title="Algebra.Semigroup.Semigroup">Semigroup</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">t</span>)</code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Option/#instances">Data.Option</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Category.Traversable/#c-Traversable" title="Category.Traversable.Traversable">Traversable</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Option/#instances">Data.Option</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test.Quick/#c-Arbitrary" title="Effect.Test.Quick.Arbitrary">Arbitrary</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">a</span>)</code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Effect.Test.Quick/#instances">Effect.Test.Quick</a></span></li></ul></details>
</div>
</article>

# Values

<article class="doc-entry" id="v-cat-options">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cat-options</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">a</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-cat-options" title="Permalink">#</a><a href="/packages/stdlib/Data.Option/source/#L76" title="View source at line 76">Source</a></span></div>
<div class="doc-body">

The present values, in order; `None`s drop (`Data.Maybe.catMaybes`).
`List` is nameable here because `Data.Core` declares it ahead of
every home (this function's signature is exactly the cycle that
module exists to break).

</div>
</article>

<article class="doc-entry" id="v-from-option">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">from-option</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-from-option" title="Permalink">#</a><a href="/packages/stdlib/Data.Option/source/#L56" title="View source at line 56">Source</a></span></div>
<div class="doc-body">

The value, or the default when there is none
(`Data.Maybe.fromMaybe`); the default leads for the same reason, and
because the result's type is the default's type.

</div>
</article>

<article class="doc-entry" id="v-is-none">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">is-none</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-is-none" title="Permalink">#</a><a href="/packages/stdlib/Data.Option/source/#L67" title="View source at line 67">Source</a></span></div>
</article>

<article class="doc-entry" id="v-is-some">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">is-some</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-is-some" title="Permalink">#</a><a href="/packages/stdlib/Data.Option/source/#L62" title="View source at line 62">Source</a></span></div>
<div class="doc-body">

The presence tests: the tag read, no payload work, no constraint.

</div>
</article>

<article class="doc-entry" id="v-list-to-option">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">list-to-option</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-list-to-option" title="Permalink">#</a><a href="/packages/stdlib/Data.Option/source/#L111" title="View source at line 111">Source</a></span></div>
<div class="doc-body">

The first element, or `None` for the empty list
(`Data.Maybe.listToMaybe`); the tail is deliberately not inspected,
so this is O(1) on any list.

</div>
</article>

<article class="doc-entry" id="v-map-option">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">map-option</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span>. (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">b</span></code><span class="doc-sig-links"><a href="#v-map-option" title="Permalink">#</a><a href="/packages/stdlib/Data.Option/source/#L96" title="View source at line 96">Source</a></span></div>
<div class="doc-body">

The option's own map, for a caller who has only the prelude: the
`Functor` method of the same name (`map` in Data.List's surface) is
reached through its constraint or with `import Category.Functor`.

```cinch test name=map-some
--- input
main :: Int
main = case map-option (\x -> x + 1) (Some 1) of
  Some n -> n - 2
  None -> 1
--- exit
0
```

</div>
</article>

<article class="doc-entry" id="v-maybe">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">maybe</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span>. <span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span> (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span></code><span class="doc-sig-links"><a href="#v-maybe" title="Permalink">#</a><a href="/packages/stdlib/Data.Option/source/#L48" title="View source at line 48">Source</a></span></div>
<div class="doc-body">

The fold: the default `d` unless there is a value, which `f`
consumes. The default leads and the function follows, matching
Haskell's `maybe` (`maybe d f o`); parity of argument order is the
point of these combinators, and the option is the thing opened last.

</div>
</article>

<article class="doc-entry" id="v-option-to-list">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">option-to-list</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-option-to-list" title="Permalink">#</a><a href="/packages/stdlib/Data.Option/source/#L103" title="View source at line 103">Source</a></span></div>
<div class="doc-body">

A `Some` is a one-element list and a `None` the empty list
(`Data.Maybe.maybeToList`); `list-to-option` is its left inverse.

</div>
</article>

<article class="doc-entry" id="v-or-else">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">or-else</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-or-else" title="Permalink">#</a><a href="/packages/stdlib/Data.Option/source/#L123" title="View source at line 123">Source</a></span></div>
<div class="doc-body">

First success: `None` falls through to the second option, `Some`
keeps the first (`Control.Applicative`'s `(<|>)` for `Option`, which
a prelude-only caller cannot reach: `Alternative` is not in the
implicit surface). The name is the value-fallback one, as in Scala's
`orElse` and this repo's `Text.Json.or-else`; Rust's closure-taking
`or_else` is deliberately not the shape, so an eager second option
is never built.

</div>
</article>

# Instances

Exported checked instances declared in this module. For law checking, see [rewrites and optimization](../reference/12-rewrites-and-optimization.md).

<ul class="doc-module-instances">
<li id="i-Alternative_Option"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Category.Alternative/#c-Alternative" title="Category.Alternative.Alternative">Alternative</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Option.Alternative_Option</code></span><div class="doc-child-prose">First success wins: None &lt;|&gt; y is y, Some x &lt;|&gt; y keeps x (moved from Category.Alternative).</div></li>
<li id="i-Applicative_Option"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Category.Applicative/#c-Applicative" title="Category.Applicative.Applicative">Applicative</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Option.Applicative_Option</code></span><div class="doc-child-prose">pure is Some; application is None if either side is (moved from Category.Applicative).</div></li>
<li id="i-Foldable_Option"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Category.Foldable/#c-Foldable" title="Category.Foldable.Foldable">Foldable</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Option.Foldable_Option</code></span><div class="doc-child-prose">A Some is a one-element container; None folds as empty (moved from Category.Foldable).</div></li>
<li id="i-Functor_Option"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Category.Functor/#c-Functor" title="Category.Functor.Functor">Functor</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Option.Functor_Option</code></span><div class="doc-child-prose">map is Some under the function; None is fixed (moved from Category.Functor).</div></li>
<li id="i-Monad_Option"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Category.Monad/#c-Monad" title="Category.Monad.Monad">Monad</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Option.Monad_Option</code></span><div class="doc-child-prose">The failing case short-circuits: None &gt;&gt;= k is None (moved from Category.Monad).</div></li>
<li id="i-Monoid_Option"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Monoid/#c-Monoid" title="Algebra.Monoid.Monoid">Monoid</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">t</span>)</code> <span class="doc-inst-meta">Implicit · <code>Data.Option.Monoid_Option</code></span><div class="doc-child-prose">None is the identity (moved from Algebra.Monoid).</div></li>
<li id="i-Semigroup_Option"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Semigroup/#c-Semigroup" title="Algebra.Semigroup.Semigroup">Semigroup</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">t</span>)</code> <span class="doc-inst-meta">Implicit · <code>Data.Option.Semigroup_Option</code></span><div class="doc-child-prose">Option first-some: the leftmost Some wins, None is ignored (moved from Algebra.Semigroup at the prelude split; the instance lives with its type).</div></li>
<li id="i-SeqOption"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Sequencing/#c-OptionSequencing" title="Effect.Sequencing.OptionSequencing">OptionSequencing</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Option.SeqOption</code></span></li>
<li id="i-Traversable_Option"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Category.Traversable/#c-Traversable" title="Category.Traversable.Traversable">Traversable</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Option.Traversable_Option</code></span><div class="doc-child-prose">None traverses to pure None; Some carries one effect (moved from Category.Traversable).</div></li>
</ul>
