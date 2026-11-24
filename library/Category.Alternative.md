# Category.Alternative

Empty and choice operations for admitted higher-kinded containers.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Category.Alternative</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Category.Alternative/source/">View source</a><a href="/packages/search/?q=%2BCategory.Alternative">Search module</a><a href="/reference-sources/Category.Alternative.cinch" download="Category.Alternative.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Category.Applicative](Category.Applicative.md).

<details class="doc-notes"><summary>Module notes</summary>

Category.Alternative: the class of containers with an identity for
choice.

`empty` is the nullary method (the dictionary carries the value
itself) and `<|>` is the choice operator (infixl 3, decision D2's
per-module fixity declarations). As in Haskell the class sits over
Applicative (the import below).

`Alternative (Result e)` is omitted by design: its `empty` needs a
distinguished error value, which is a Monoid on `e`'s error type;
it belongs with the Monoid integration, not here.

Instance homes: the List and Option instances got theirs at the
prelude split (Data.List/Data.Option).

</details>

<details class="doc-synopsis" open><summary>Synopsis (3 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#c-Alternative">Alternative</a><code class="syn-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">Alternative</strong> <span class="tok-variable">f</span></code></li><li class="syn-sub"><a class="syn-name" href="#m-Alternative-empty">empty</a><code class="syn-sig"><strong class="tok-variable">empty</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">f</span> :: <span class="tok-constructor">Type</span> <span class="tok-operator">-&gt;</span> <span class="tok-constructor">Type</span>) <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Category.Alternative/#c-Alternative" title="Category.Alternative.Alternative">Alternative</a> <span class="tok-variable">f</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">f</span> <span class="tok-variable">a</span></code></li><li class="syn-sub"><a class="syn-name" href="#m-Alternative-op-3c7c3e">&lt;|&gt;</a><code class="syn-sig"><strong class="tok-variable">&lt;|&gt;</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">f</span> :: <span class="tok-constructor">Type</span> <span class="tok-operator">-&gt;</span> <span class="tok-constructor">Type</span>) <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Category.Alternative/#c-Alternative" title="Category.Alternative.Alternative">Alternative</a> <span class="tok-variable">f</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">f</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">f</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">f</span> <span class="tok-variable">a</span></code></li></ul>
</details>

# Signatures

<article class="doc-entry" id="c-Alternative">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">Alternative</strong> <span class="tok-variable">f</span></code><span class="doc-sig-links"><a href="#c-Alternative" title="Permalink">#</a><a href="/packages/stdlib/Category.Alternative/source/#L19" title="View source at line 19">Source</a></span></div>
<div class="doc-body">
<div class="doc-children"><h4>Methods</h4><ul><li id="m-Alternative-empty"><div class="doc-method-row"><code><strong class="tok-variable">empty</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">f</span> :: <span class="tok-constructor">Type</span> <span class="tok-operator">-&gt;</span> <span class="tok-constructor">Type</span>) <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Category.Alternative/#c-Alternative" title="Category.Alternative.Alternative">Alternative</a> <span class="tok-variable">f</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">f</span> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#m-Alternative-empty" title="Permalink">#</a><a href="/packages/stdlib/Category.Alternative/source/#L20">Source</a></span></div></li><li id="m-Alternative-op-3c7c3e"><div class="doc-method-row"><code><strong class="tok-variable">&lt;|&gt;</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">f</span> :: <span class="tok-constructor">Type</span> <span class="tok-operator">-&gt;</span> <span class="tok-constructor">Type</span>) <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Category.Alternative/#c-Alternative" title="Category.Alternative.Alternative">Alternative</a> <span class="tok-variable">f</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">f</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">f</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">f</span> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#m-Alternative-op-3c7c3e" title="Permalink">#</a><a href="/packages/stdlib/Category.Alternative/source/#L21">Source</a></span></div></li></ul></div>
<details class="doc-instances"><summary>Instances (3)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Category.Alternative/#c-Alternative" title="Category.Alternative.Alternative">Alternative</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.List/#instances">Data.List</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Category.Alternative/#c-Alternative" title="Category.Alternative.Alternative">Alternative</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Option/#instances">Data.Option</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Category.Alternative/#c-Alternative" title="Category.Alternative.Alternative">Alternative</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Effect.Stm/#instances">Effect.Stm</a></span></li></ul></details>
</div>
</article>

# Operator fixities

| Operator | Associativity | Precedence |
| --- | --- | --- |
| `<\|>` | `infixl` | `3` |
