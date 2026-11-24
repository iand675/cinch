# Data.Bool

Boolean operations and their comparison instances.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Data.Bool</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Data.Bool/source/">View source</a><a href="/packages/search/?q=%2BData.Bool">Search module</a><a href="/reference-sources/Data.Bool.cinch" download="Data.Bool.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Hash](Data.Hash.md), [Text.Codec](Text.Codec.md).

<details class="doc-notes"><summary>Module notes</summary>

Data.Bool: the booleans and their strict operations.

`Bool` is a core runtime type; the `cinch.prim.*Bool` foreign
imports are the machine operations. `&&` and `||` are strict
functions in the bootstrap slice; the surface short-circuiting form
lands with the effects correction (primitive types specification
section 6).

Module scheme (design/base-library.md): the type's home carries its
instance family, including the `Text.Codec` instance (the A6
codec generator's BOOLEAN column is exactly one byte, 0 or 1).

</details>

<details class="doc-synopsis" open><summary>Synopsis (12 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#v-op-2626">&amp;&amp;</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">&amp;&amp;</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-and">and</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">and</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-bool">bool</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">bool</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-not">not</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">not</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-or">or</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">or</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-prim-and-bool">prim-and-bool</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">prim-and-bool</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-prim-eq-bool">prim-eq-bool</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">prim-eq-bool</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-prim-ne-bool">prim-ne-bool</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">prim-ne-bool</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-prim-not-bool">prim-not-bool</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">prim-not-bool</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-prim-or-bool">prim-or-bool</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">prim-or-bool</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-xor">xor</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">xor</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-op-7c7c">||</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">||</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li></ul>
</details>

# Values

<article class="doc-entry" id="v-op-2626">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">&amp;&amp;</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-op-2626" title="Permalink">#</a><a href="/packages/stdlib/Data.Bool/source/#L32" title="View source at line 32">Source</a></span></div>
<div class="doc-body">

These are strict functions in the bootstrap slice; the surface `&&`
and `||` short-circuit once that deviation is corrected (primitive
types specification, section 6).

</div>
</article>

<article class="doc-entry" id="v-and">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">and</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-and" title="Permalink">#</a><a href="/packages/stdlib/Data.Bool/source/#L42" title="View source at line 42">Source</a></span></div>
<div class="doc-body">

The strict boolean algebra (primitive types specification, section 6).

</div>
</article>

<article class="doc-entry" id="v-bool">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">bool</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-bool" title="Permalink">#</a><a href="/packages/stdlib/Data.Bool/source/#L54" title="View source at line 54">Source</a></span></div>
<div class="doc-body">

The boolean fold: `bool f t b` is `t` when `b` is `True` and `f`
otherwise (`Data.Bool.bool`), the elimination form matching the
constructors' order.

</div>
</article>

<article class="doc-entry" id="v-not">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">not</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-not" title="Permalink">#</a><a href="/packages/stdlib/Data.Bool/source/#L38" title="View source at line 38">Source</a></span></div>
</article>

<article class="doc-entry" id="v-or">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">or</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-or" title="Permalink">#</a><a href="/packages/stdlib/Data.Bool/source/#L45" title="View source at line 45">Source</a></span></div>
</article>

<article class="doc-entry" id="v-prim-and-bool">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">prim-and-bool</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-prim-and-bool" title="Permalink">#</a><a href="/packages/stdlib/Data.Bool/source/#L26" title="View source at line 26">Source</a></span></div>
</article>

<article class="doc-entry" id="v-prim-eq-bool">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">prim-eq-bool</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-prim-eq-bool" title="Permalink">#</a><a href="/packages/stdlib/Data.Bool/source/#L23" title="View source at line 23">Source</a></span></div>
</article>

<article class="doc-entry" id="v-prim-ne-bool">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">prim-ne-bool</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-prim-ne-bool" title="Permalink">#</a><a href="/packages/stdlib/Data.Bool/source/#L24" title="View source at line 24">Source</a></span></div>
</article>

<article class="doc-entry" id="v-prim-not-bool">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">prim-not-bool</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-prim-not-bool" title="Permalink">#</a><a href="/packages/stdlib/Data.Bool/source/#L25" title="View source at line 25">Source</a></span></div>
</article>

<article class="doc-entry" id="v-prim-or-bool">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">prim-or-bool</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-prim-or-bool" title="Permalink">#</a><a href="/packages/stdlib/Data.Bool/source/#L27" title="View source at line 27">Source</a></span></div>
</article>

<article class="doc-entry" id="v-xor">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">xor</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-xor" title="Permalink">#</a><a href="/packages/stdlib/Data.Bool/source/#L48" title="View source at line 48">Source</a></span></div>
</article>

<article class="doc-entry" id="v-op-7c7c">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">||</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-op-7c7c" title="Permalink">#</a><a href="/packages/stdlib/Data.Bool/source/#L35" title="View source at line 35">Source</a></span></div>
</article>

# Operator fixities

| Operator | Associativity | Precedence |
| --- | --- | --- |
| `&&` | `infixr` | `2` |
| `\|\|` | `infixr` | `1` |

# Instances

Exported checked instances declared in this module. For law checking, see [rewrites and optimization](../reference/12-rewrites-and-optimization.md).

<ul class="doc-module-instances">
<li id="i-Codec_Bool"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Codec/#c-Codec" title="Text.Codec.Codec">Codec</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Bool.Codec_Bool</code></span><div class="doc-child-prose">The A6 column codec for booleans (modules specification section 11's CodecGenerator.derive vocabulary, decision D65's covered set): one byte, 0 for False and 1 for True.</div></li>
<li id="i-Eq_Bool"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Bool.Eq_Bool</code></span></li>
<li id="i-Hash_Bool"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Hash/#c-Hash" title="Data.Hash.Hash">Hash</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Bool.Hash_Bool</code></span><div class="doc-child-prose">Two buckets (instances live with their type).</div></li>
<li id="i-JsonCodec_Bool"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Codec/#c-JsonCodec" title="Text.Codec.JsonCodec">JsonCodec</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Bool.JsonCodec_Bool</code></span><div class="doc-child-prose">The A6 JSON column codec for booleans (decision D65's covered set): JsonCodec's round-trip law (decode-json (encode-json x) is Ok x).</div></li>
<li id="i-Ord_Bool"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Bool.Ord_Bool</code></span></li>
</ul>
