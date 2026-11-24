# Crypto.Field25519

Constant-time arithmetic modulo 2^255 - 19 with 128-bit column products, shared by X25519 and Ed25519.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--source">Ship source</span> <code>import Crypto.Field25519</code></div><div class="doc-mod-bar-links"><a href="/library/source/Crypto.Field25519/">View source</a><a href="/library/search/?q=%2BCrypto.Field25519">Search module</a><a href="/reference-sources/Crypto.Field25519.cinch" download="Crypto.Field25519.cinch">Download .cinch</a><a href="/library/00-index/">All modules</a></div></div>

**Dependencies:** [Data.Bytes](Data.Bytes.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Machine](Data.Machine.md), [Data.Wide](Data.Wide.md).

<details class="doc-closure"><summary>Ship-source closure (2 files)</summary>

Copy these modules beside your entry file before importing `Crypto.Field25519`: [Crypto.Field25519](Crypto.Field25519.md), [Data.Bytes.Word](Data.Bytes.Word.md).

```sh
cp seed/prelude/Crypto/Field25519.cinch ./Crypto.Field25519.cinch
cp seed/prelude/Data/Bytes/Word.cinch ./Data.Bytes.Word.cinch
```

</details>

<details class="doc-notes"><summary>Module notes</summary>

Constant-time arithmetic in the field of integers modulo 2^255 - 19, shared by X25519 and Ed25519.
An element is five 51-bit limbs (little-endian radix 2^51). Each product
column is one primitive that accumulates its five 64x64-bit products in a
128-bit register, so a multiplication is five independent wide columns and
one carry chain; additions and subtractions carry immediately, keeping every
limb below 2^52 so the next product cannot overflow. Every operation is free
of data-dependent branches and memory access: conditional swaps and
selections use masks, inversion is a fixed addition chain, and comparisons
reduce to the canonical encoding first.

</details>

<details class="doc-synopsis"><summary>Synopsis (20 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-Fe">Fe</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Fe</strong></code></li><li><a class="syn-name" href="#v-fe-add">fe-add</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">fe-add</strong> :: <a class="doc-type-link tok-constructor" href="/library/Crypto.Field25519/#t-Fe" title="Crypto.Field25519.Fe">Fe</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Crypto.Field25519/#t-Fe" title="Crypto.Field25519.Fe">Fe</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Crypto.Field25519/#t-Fe" title="Crypto.Field25519.Fe">Fe</a></code></li><li><a class="syn-name" href="#v-fe-equal">fe-equal</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">fe-equal</strong> :: <a class="doc-type-link tok-constructor" href="/library/Crypto.Field25519/#t-Fe" title="Crypto.Field25519.Fe">Fe</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Crypto.Field25519/#t-Fe" title="Crypto.Field25519.Fe">Fe</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U64">U64</a></code></li><li><a class="syn-name" href="#v-fe-from-bytes">fe-from-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">fe-from-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Crypto.Field25519/#t-Fe" title="Crypto.Field25519.Fe">Fe</a></code></li><li><a class="syn-name" href="#v-fe-from-u64">fe-from-u64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">fe-from-u64</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Crypto.Field25519/#t-Fe" title="Crypto.Field25519.Fe">Fe</a></code></li><li><a class="syn-name" href="#v-fe-invert">fe-invert</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">fe-invert</strong> :: <a class="doc-type-link tok-constructor" href="/library/Crypto.Field25519/#t-Fe" title="Crypto.Field25519.Fe">Fe</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Crypto.Field25519/#t-Fe" title="Crypto.Field25519.Fe">Fe</a></code></li><li><a class="syn-name" href="#v-fe-is-negative">fe-is-negative</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">fe-is-negative</strong> :: <a class="doc-type-link tok-constructor" href="/library/Crypto.Field25519/#t-Fe" title="Crypto.Field25519.Fe">Fe</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U64">U64</a></code></li><li><a class="syn-name" href="#v-fe-is-zero">fe-is-zero</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">fe-is-zero</strong> :: <a class="doc-type-link tok-constructor" href="/library/Crypto.Field25519/#t-Fe" title="Crypto.Field25519.Fe">Fe</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U64">U64</a></code></li><li><a class="syn-name" href="#v-fe-limbs">fe-limbs</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">fe-limbs</strong> :: <a class="doc-type-link tok-constructor" href="/library/Crypto.Field25519/#t-Fe" title="Crypto.Field25519.Fe">Fe</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U64">U64</a>, <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U64">U64</a>, <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U64">U64</a>, <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U64">U64</a>, <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U64">U64</a>)</code></li><li><a class="syn-name" href="#v-fe-mul">fe-mul</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">fe-mul</strong> :: <a class="doc-type-link tok-constructor" href="/library/Crypto.Field25519/#t-Fe" title="Crypto.Field25519.Fe">Fe</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Crypto.Field25519/#t-Fe" title="Crypto.Field25519.Fe">Fe</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Crypto.Field25519/#t-Fe" title="Crypto.Field25519.Fe">Fe</a></code></li><li><a class="syn-name" href="#v-fe-mul-small">fe-mul-small</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">fe-mul-small</strong> :: <a class="doc-type-link tok-constructor" href="/library/Crypto.Field25519/#t-Fe" title="Crypto.Field25519.Fe">Fe</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Crypto.Field25519/#t-Fe" title="Crypto.Field25519.Fe">Fe</a></code></li><li><a class="syn-name" href="#v-fe-neg">fe-neg</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">fe-neg</strong> :: <a class="doc-type-link tok-constructor" href="/library/Crypto.Field25519/#t-Fe" title="Crypto.Field25519.Fe">Fe</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Crypto.Field25519/#t-Fe" title="Crypto.Field25519.Fe">Fe</a></code></li><li><a class="syn-name" href="#v-fe-one">fe-one</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">fe-one</strong> :: <a class="doc-type-link tok-constructor" href="/library/Crypto.Field25519/#t-Fe" title="Crypto.Field25519.Fe">Fe</a></code></li><li><a class="syn-name" href="#v-fe-pow22523">fe-pow22523</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">fe-pow22523</strong> :: <a class="doc-type-link tok-constructor" href="/library/Crypto.Field25519/#t-Fe" title="Crypto.Field25519.Fe">Fe</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Crypto.Field25519/#t-Fe" title="Crypto.Field25519.Fe">Fe</a></code></li><li><a class="syn-name" href="#v-fe-select">fe-select</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">fe-select</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Crypto.Field25519/#t-Fe" title="Crypto.Field25519.Fe">Fe</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Crypto.Field25519/#t-Fe" title="Crypto.Field25519.Fe">Fe</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Crypto.Field25519/#t-Fe" title="Crypto.Field25519.Fe">Fe</a></code></li><li><a class="syn-name" href="#v-fe-square">fe-square</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">fe-square</strong> :: <a class="doc-type-link tok-constructor" href="/library/Crypto.Field25519/#t-Fe" title="Crypto.Field25519.Fe">Fe</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Crypto.Field25519/#t-Fe" title="Crypto.Field25519.Fe">Fe</a></code></li><li><a class="syn-name" href="#v-fe-sub">fe-sub</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">fe-sub</strong> :: <a class="doc-type-link tok-constructor" href="/library/Crypto.Field25519/#t-Fe" title="Crypto.Field25519.Fe">Fe</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Crypto.Field25519/#t-Fe" title="Crypto.Field25519.Fe">Fe</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Crypto.Field25519/#t-Fe" title="Crypto.Field25519.Fe">Fe</a></code></li><li><a class="syn-name" href="#v-fe-swap">fe-swap</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">fe-swap</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Crypto.Field25519/#t-Fe" title="Crypto.Field25519.Fe">Fe</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Crypto.Field25519/#t-Fe" title="Crypto.Field25519.Fe">Fe</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/library/Crypto.Field25519/#t-Fe" title="Crypto.Field25519.Fe">Fe</a>, <a class="doc-type-link tok-constructor" href="/library/Crypto.Field25519/#t-Fe" title="Crypto.Field25519.Fe">Fe</a>)</code></li><li><a class="syn-name" href="#v-fe-to-bytes">fe-to-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">fe-to-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/library/Crypto.Field25519/#t-Fe" title="Crypto.Field25519.Fe">Fe</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-fe-zero">fe-zero</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">fe-zero</strong> :: <a class="doc-type-link tok-constructor" href="/library/Crypto.Field25519/#t-Fe" title="Crypto.Field25519.Fe">Fe</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-Fe">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Fe</strong></code><span class="doc-sig-links"><a href="#t-Fe" title="Permalink">#</a><a href="/library/source/Crypto.Field25519/#L23" title="View source at line 23">Source</a></span></div>
<div class="doc-body">

Limbs from least to most significant, radix 2^51.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-Fe"><code><strong class="tok-constructor">Fe</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Crypto.Field25519/#t-Fe" title="Crypto.Field25519.Fe">Fe</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-fe-add">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">fe-add</strong> :: <a class="doc-type-link tok-constructor" href="/library/Crypto.Field25519/#t-Fe" title="Crypto.Field25519.Fe">Fe</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Crypto.Field25519/#t-Fe" title="Crypto.Field25519.Fe">Fe</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Crypto.Field25519/#t-Fe" title="Crypto.Field25519.Fe">Fe</a></code><span class="doc-sig-links"><a href="#v-fe-add" title="Permalink">#</a><a href="/library/source/Crypto.Field25519/#L57" title="View source at line 57">Source</a></span></div>
<div class="doc-body">

Sum with an immediate carry.

</div>
</article>

<article class="doc-entry" id="v-fe-equal">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">fe-equal</strong> :: <a class="doc-type-link tok-constructor" href="/library/Crypto.Field25519/#t-Fe" title="Crypto.Field25519.Fe">Fe</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Crypto.Field25519/#t-Fe" title="Crypto.Field25519.Fe">Fe</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U64">U64</a></code><span class="doc-sig-links"><a href="#v-fe-equal" title="Permalink">#</a><a href="/library/source/Crypto.Field25519/#L377" title="View source at line 377">Source</a></span></div>
<div class="doc-body">

One when the two values are equal modulo p, otherwise zero.

</div>
</article>

<article class="doc-entry" id="v-fe-from-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">fe-from-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Crypto.Field25519/#t-Fe" title="Crypto.Field25519.Fe">Fe</a></code><span class="doc-sig-links"><a href="#v-fe-from-bytes" title="Permalink">#</a><a href="/library/source/Crypto.Field25519/#L314" title="View source at line 314">Source</a></span></div>
<div class="doc-body">

Decode 32 little-endian bytes at an offset, ignoring the top bit. Out-of-range input reads as zero.

</div>
</article>

<article class="doc-entry" id="v-fe-from-u64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">fe-from-u64</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Crypto.Field25519/#t-Fe" title="Crypto.Field25519.Fe">Fe</a></code><span class="doc-sig-links"><a href="#v-fe-from-u64" title="Permalink">#</a><a href="/library/source/Crypto.Field25519/#L34" title="View source at line 34">Source</a></span></div>
<div class="doc-body">

A small element below 2^51.

</div>
</article>

<article class="doc-entry" id="v-fe-invert">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">fe-invert</strong> :: <a class="doc-type-link tok-constructor" href="/library/Crypto.Field25519/#t-Fe" title="Crypto.Field25519.Fe">Fe</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Crypto.Field25519/#t-Fe" title="Crypto.Field25519.Fe">Fe</a></code><span class="doc-sig-links"><a href="#v-fe-invert" title="Permalink">#</a><a href="/library/source/Crypto.Field25519/#L286" title="View source at line 286">Source</a></span></div>
<div class="doc-body">

Multiplicative inverse a^(p - 2) by the fixed addition chain; the inverse of zero is zero.

</div>
</article>

<article class="doc-entry" id="v-fe-is-negative">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">fe-is-negative</strong> :: <a class="doc-type-link tok-constructor" href="/library/Crypto.Field25519/#t-Fe" title="Crypto.Field25519.Fe">Fe</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U64">U64</a></code><span class="doc-sig-links"><a href="#v-fe-is-negative" title="Permalink">#</a><a href="/library/source/Crypto.Field25519/#L366" title="View source at line 366">Source</a></span></div>
<div class="doc-body">

Whether the canonical value is odd (the sign used by point encodings).

</div>
</article>

<article class="doc-entry" id="v-fe-is-zero">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">fe-is-zero</strong> :: <a class="doc-type-link tok-constructor" href="/library/Crypto.Field25519/#t-Fe" title="Crypto.Field25519.Fe">Fe</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U64">U64</a></code><span class="doc-sig-links"><a href="#v-fe-is-zero" title="Permalink">#</a><a href="/library/source/Crypto.Field25519/#L370" title="View source at line 370">Source</a></span></div>
<div class="doc-body">

One when the value is zero modulo p, otherwise zero, computed without branching.

</div>
</article>

<article class="doc-entry" id="v-fe-limbs">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">fe-limbs</strong> :: <a class="doc-type-link tok-constructor" href="/library/Crypto.Field25519/#t-Fe" title="Crypto.Field25519.Fe">Fe</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U64">U64</a>, <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U64">U64</a>, <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U64">U64</a>, <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U64">U64</a>, <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U64">U64</a>)</code><span class="doc-sig-links"><a href="#v-fe-limbs" title="Permalink">#</a><a href="/library/source/Crypto.Field25519/#L38" title="View source at line 38">Source</a></span></div>
<div class="doc-body">

The five limbs, for constant tables and tests.

</div>
</article>

<article class="doc-entry" id="v-fe-mul">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">fe-mul</strong> :: <a class="doc-type-link tok-constructor" href="/library/Crypto.Field25519/#t-Fe" title="Crypto.Field25519.Fe">Fe</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Crypto.Field25519/#t-Fe" title="Crypto.Field25519.Fe">Fe</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Crypto.Field25519/#t-Fe" title="Crypto.Field25519.Fe">Fe</a></code><span class="doc-sig-links"><a href="#v-fe-mul" title="Permalink">#</a><a href="/library/source/Crypto.Field25519/#L76" title="View source at line 76">Source</a></span></div>
<div class="doc-body">

Product: five 128-bit columns, then one carry chain.

</div>
</article>

<article class="doc-entry" id="v-fe-mul-small">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">fe-mul-small</strong> :: <a class="doc-type-link tok-constructor" href="/library/Crypto.Field25519/#t-Fe" title="Crypto.Field25519.Fe">Fe</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Crypto.Field25519/#t-Fe" title="Crypto.Field25519.Fe">Fe</a></code><span class="doc-sig-links"><a href="#v-fe-mul-small" title="Permalink">#</a><a href="/library/source/Crypto.Field25519/#L252" title="View source at line 252">Source</a></span></div>
<div class="doc-body">

Multiply by a constant below 2^13 (such as 121665): one carry chain over 128-bit limbs.

</div>
</article>

<article class="doc-entry" id="v-fe-neg">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">fe-neg</strong> :: <a class="doc-type-link tok-constructor" href="/library/Crypto.Field25519/#t-Fe" title="Crypto.Field25519.Fe">Fe</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Crypto.Field25519/#t-Fe" title="Crypto.Field25519.Fe">Fe</a></code><span class="doc-sig-links"><a href="#v-fe-neg" title="Permalink">#</a><a href="/library/source/Crypto.Field25519/#L72" title="View source at line 72">Source</a></span></div>
<div class="doc-body">

Negation.

</div>
</article>

<article class="doc-entry" id="v-fe-one">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">fe-one</strong> :: <a class="doc-type-link tok-constructor" href="/library/Crypto.Field25519/#t-Fe" title="Crypto.Field25519.Fe">Fe</a></code><span class="doc-sig-links"><a href="#v-fe-one" title="Permalink">#</a><a href="/library/source/Crypto.Field25519/#L30" title="View source at line 30">Source</a></span></div>
<div class="doc-body">

One.

</div>
</article>

<article class="doc-entry" id="v-fe-pow22523">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">fe-pow22523</strong> :: <a class="doc-type-link tok-constructor" href="/library/Crypto.Field25519/#t-Fe" title="Crypto.Field25519.Fe">Fe</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Crypto.Field25519/#t-Fe" title="Crypto.Field25519.Fe">Fe</a></code><span class="doc-sig-links"><a href="#v-fe-pow22523" title="Permalink">#</a><a href="/library/source/Crypto.Field25519/#L290" title="View source at line 290">Source</a></span></div>
<div class="doc-body">

a^((p - 5) / 8), used to compute square roots.

</div>
</article>

<article class="doc-entry" id="v-fe-select">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">fe-select</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Crypto.Field25519/#t-Fe" title="Crypto.Field25519.Fe">Fe</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Crypto.Field25519/#t-Fe" title="Crypto.Field25519.Fe">Fe</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Crypto.Field25519/#t-Fe" title="Crypto.Field25519.Fe">Fe</a></code><span class="doc-sig-links"><a href="#v-fe-select" title="Permalink">#</a><a href="/library/source/Crypto.Field25519/#L310" title="View source at line 310">Source</a></span></div>
<div class="doc-body">

Choose b when the low bit of choice is set and a otherwise, without branching.

</div>
</article>

<article class="doc-entry" id="v-fe-square">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">fe-square</strong> :: <a class="doc-type-link tok-constructor" href="/library/Crypto.Field25519/#t-Fe" title="Crypto.Field25519.Fe">Fe</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Crypto.Field25519/#t-Fe" title="Crypto.Field25519.Fe">Fe</a></code><span class="doc-sig-links"><a href="#v-fe-square" title="Permalink">#</a><a href="/library/source/Crypto.Field25519/#L88" title="View source at line 88">Source</a></span></div>
<div class="doc-body">

Square (the general product with both operands equal; LLVM merges the symmetric terms).

</div>
</article>

<article class="doc-entry" id="v-fe-sub">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">fe-sub</strong> :: <a class="doc-type-link tok-constructor" href="/library/Crypto.Field25519/#t-Fe" title="Crypto.Field25519.Fe">Fe</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Crypto.Field25519/#t-Fe" title="Crypto.Field25519.Fe">Fe</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Crypto.Field25519/#t-Fe" title="Crypto.Field25519.Fe">Fe</a></code><span class="doc-sig-links"><a href="#v-fe-sub" title="Permalink">#</a><a href="/library/source/Crypto.Field25519/#L63" title="View source at line 63">Source</a></span></div>
<div class="doc-body">

Difference, computed as a + 4p - b so no limb underflows, with an immediate carry.

</div>
</article>

<article class="doc-entry" id="v-fe-swap">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">fe-swap</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Crypto.Field25519/#t-Fe" title="Crypto.Field25519.Fe">Fe</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Crypto.Field25519/#t-Fe" title="Crypto.Field25519.Fe">Fe</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/library/Crypto.Field25519/#t-Fe" title="Crypto.Field25519.Fe">Fe</a>, <a class="doc-type-link tok-constructor" href="/library/Crypto.Field25519/#t-Fe" title="Crypto.Field25519.Fe">Fe</a>)</code><span class="doc-sig-links"><a href="#v-fe-swap" title="Permalink">#</a><a href="/library/source/Crypto.Field25519/#L298" title="View source at line 298">Source</a></span></div>
<div class="doc-body">

Swap two elements when the low bit of choice is set, without branching.

</div>
</article>

<article class="doc-entry" id="v-fe-to-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">fe-to-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/library/Crypto.Field25519/#t-Fe" title="Crypto.Field25519.Fe">Fe</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-fe-to-bytes" title="Permalink">#</a><a href="/library/source/Crypto.Field25519/#L355" title="View source at line 355">Source</a></span></div>
<div class="doc-body">

Encode the canonical value as 32 little-endian bytes in fresh storage.

</div>
</article>

<article class="doc-entry" id="v-fe-zero">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">fe-zero</strong> :: <a class="doc-type-link tok-constructor" href="/library/Crypto.Field25519/#t-Fe" title="Crypto.Field25519.Fe">Fe</a></code><span class="doc-sig-links"><a href="#v-fe-zero" title="Permalink">#</a><a href="/library/source/Crypto.Field25519/#L26" title="View source at line 26">Source</a></span></div>
<div class="doc-body">

Zero.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Crypto/Field25519.cinch`](/library/source/Crypto.Field25519/). Source SHA-256: `bbbcd693d8fbf35589b30e4837955da8c04f69884a85a55d6d5f613a0e62609d`.
