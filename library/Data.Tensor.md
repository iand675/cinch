# Data.Tensor

Immutable row-major tensors with checked runtime shapes of any rank.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--source">Ship source</span> <code>import Data.Tensor</code></div><div class="doc-mod-bar-links"><a href="/library/source/Data.Tensor/">View source</a><a href="/library/search/?q=%2BData.Tensor">Search module</a><a href="/reference-sources/Data.Tensor.cinch" download="Data.Tensor.cinch">Download .cinch</a><a href="/library/00-index/">All modules</a></div></div>

**Dependencies:** [Data.Array](Data.Array.md), [Data.Core](Data.Core.md).

<details class="doc-closure"><summary>Ship-source closure (2 files)</summary>

Copy these modules beside your entry file before importing `Data.Tensor`: [Data.Array](Data.Array.md), [Data.Tensor](Data.Tensor.md).

```sh
cp seed/prelude/Data/Array.cinch ./Data.Array.cinch
cp seed/prelude/Data/Tensor.cinch ./Data.Tensor.cinch
```

</details>

<details class="doc-notes"><summary>Module notes</summary>

Immutable row-major arrays, with checked runtime shapes of any rank.

</details>

<details class="doc-synopsis" open><summary>Synopsis (5 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#v-tensor-data">tensor-data</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tensor-data</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-constructor">Tensor</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Array/" title="Array">Array</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-tensor-get">tensor-get</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tensor-get</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-constructor">Tensor</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Array/" title="Array">Array</a> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-tensor-reshape">tensor-reshape</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tensor-reshape</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/library/Data.Array/" title="Array">Array</a> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Array/" title="Array">Array</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<span class="tok-constructor">Tensor</span> <span class="tok-variable">a</span>)</code></li><li><a class="syn-name" href="#v-tensor-set">tensor-set</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tensor-set</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-constructor">Tensor</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Array/" title="Array">Array</a> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<span class="tok-constructor">Tensor</span> <span class="tok-variable">a</span>)</code></li><li><a class="syn-name" href="#v-tensor-shape">tensor-shape</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tensor-shape</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-constructor">Tensor</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Array/" title="Array">Array</a> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code></li></ul>
</details>

# Values

<article class="doc-entry" id="v-tensor-data">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tensor-data</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-constructor">Tensor</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Array/" title="Array">Array</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-tensor-data" title="Permalink">#</a><a href="/library/source/Data.Tensor/#L9" title="View source at line 9">Source</a></span></div>
</article>

<article class="doc-entry" id="v-tensor-get">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tensor-get</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-constructor">Tensor</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Array/" title="Array">Array</a> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-tensor-get" title="Permalink">#</a><a href="/library/source/Data.Tensor/#L10" title="View source at line 10">Source</a></span></div>
</article>

<article class="doc-entry" id="v-tensor-reshape">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tensor-reshape</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/library/Data.Array/" title="Array">Array</a> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Array/" title="Array">Array</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<span class="tok-constructor">Tensor</span> <span class="tok-variable">a</span>)</code><span class="doc-sig-links"><a href="#v-tensor-reshape" title="Permalink">#</a><a href="/library/source/Data.Tensor/#L7" title="View source at line 7">Source</a></span></div>
</article>

<article class="doc-entry" id="v-tensor-set">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tensor-set</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-constructor">Tensor</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Array/" title="Array">Array</a> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<span class="tok-constructor">Tensor</span> <span class="tok-variable">a</span>)</code><span class="doc-sig-links"><a href="#v-tensor-set" title="Permalink">#</a><a href="/library/source/Data.Tensor/#L11" title="View source at line 11">Source</a></span></div>
</article>

<article class="doc-entry" id="v-tensor-shape">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tensor-shape</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-constructor">Tensor</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Array/" title="Array">Array</a> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-tensor-shape" title="Permalink">#</a><a href="/library/source/Data.Tensor/#L8" title="View source at line 8">Source</a></span></div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Data/Tensor.cinch`](/library/source/Data.Tensor/). Source SHA-256: `caeef1f823e842a73b16519af6e7baa9de0ac0ddb00d31472ed9461116836eb7`.
