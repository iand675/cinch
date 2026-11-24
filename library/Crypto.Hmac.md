# Crypto.Hmac

Prepared HMAC-SHA-256/SHA-512 keys, persistent messages and full-tag verification.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Crypto.Hmac</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Crypto.Hmac/source/">View source</a><a href="/packages/search/?q=%2BCrypto.Hmac">Search module</a><a href="/reference-sources/Crypto.Hmac.cinch" download="Crypto.Hmac.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Crypto.ConstantTime](Crypto.ConstantTime.md), [Data.Bytes](Data.Bytes.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Text.Digest](Text.Digest.md).

<details class="doc-notes"><summary>Module notes</summary>

RFC 2104 HMAC with SHA-256 and SHA-512. A prepared key owns its pads.
Persistent states snapshot written chunks; finalization traverses the entire
retained message. Whole-tag verification uses public-length constant-time equality.

</details>

<details class="doc-synopsis" open><summary>Synopsis (13 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-HmacAlgorithm">HmacAlgorithm</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">HmacAlgorithm</strong></code></li><li><a class="syn-name" href="#t-HmacKey">HmacKey</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">HmacKey</strong></code></li><li><a class="syn-name" href="#t-HmacState">HmacState</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">HmacState</strong></code></li><li><a class="syn-name" href="#v-hmac-append">hmac-append</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">hmac-append</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hmac/#t-HmacState" title="Crypto.Hmac.HmacState">HmacState</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-hmac-block-size">hmac-block-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">hmac-block-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hmac/#t-HmacAlgorithm" title="Crypto.Hmac.HmacAlgorithm">HmacAlgorithm</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-hmac-key">hmac-key</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">hmac-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hmac/#t-HmacAlgorithm" title="Crypto.Hmac.HmacAlgorithm">HmacAlgorithm</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hmac/#t-HmacKey" title="Crypto.Hmac.HmacKey">HmacKey</a></code></li><li><a class="syn-name" href="#v-hmac-reset">hmac-reset</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">hmac-reset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hmac/#t-HmacState" title="Crypto.Hmac.HmacState">HmacState</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hmac/#t-HmacState" title="Crypto.Hmac.HmacState">HmacState</a></code></li><li><a class="syn-name" href="#v-hmac-size">hmac-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">hmac-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hmac/#t-HmacAlgorithm" title="Crypto.Hmac.HmacAlgorithm">HmacAlgorithm</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-hmac-state">hmac-state</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">hmac-state</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hmac/#t-HmacKey" title="Crypto.Hmac.HmacKey">HmacKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hmac/#t-HmacState" title="Crypto.Hmac.HmacState">HmacState</a></code></li><li><a class="syn-name" href="#v-hmac-sum">hmac-sum</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">hmac-sum</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hmac/#t-HmacState" title="Crypto.Hmac.HmacState">HmacState</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-hmac-tag">hmac-tag</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">hmac-tag</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hmac/#t-HmacKey" title="Crypto.Hmac.HmacKey">HmacKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-hmac-update">hmac-update</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">hmac-update</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hmac/#t-HmacState" title="Crypto.Hmac.HmacState">HmacState</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hmac/#t-HmacState" title="Crypto.Hmac.HmacState">HmacState</a></code></li><li><a class="syn-name" href="#v-hmac-verify">hmac-verify</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">hmac-verify</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hmac/#t-HmacKey" title="Crypto.Hmac.HmacKey">HmacKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-HmacAlgorithm">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">HmacAlgorithm</strong></code><span class="doc-sig-links"><a href="#t-HmacAlgorithm" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Hmac/source/#L18" title="View source at line 18">Source</a></span></div>
<div class="doc-body">

Hash algorithm used for both inner and outer digests.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-HmacSha256"><code><strong class="tok-constructor">HmacSha256</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hmac/#t-HmacAlgorithm" title="Crypto.Hmac.HmacAlgorithm">HmacAlgorithm</a></code></li><li id="k-HmacSha512"><code><strong class="tok-constructor">HmacSha512</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hmac/#t-HmacAlgorithm" title="Crypto.Hmac.HmacAlgorithm">HmacAlgorithm</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-HmacKey">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">HmacKey</strong></code><span class="doc-sig-links"><a href="#t-HmacKey" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Hmac/source/#L23" title="View source at line 23">Source</a></span></div>
<div class="doc-body">

An owned pair of block-sized inner and outer key pads.

</div>
</article>

<article class="doc-entry" id="t-HmacState">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">HmacState</strong></code><span class="doc-sig-links"><a href="#t-HmacState" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Hmac/source/#L27" title="View source at line 27">Source</a></span></div>
<div class="doc-body">

A prepared key and privately owned message fragments, newest first.

</div>
</article>

# Values

<article class="doc-entry" id="v-hmac-append">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">hmac-append</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hmac/#t-HmacState" title="Crypto.Hmac.HmacState">HmacState</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-hmac-append" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Hmac/source/#L87" title="View source at line 87">Source</a></span></div>
<div class="doc-body">

Append the tag to a copied prefix.

</div>
</article>

<article class="doc-entry" id="v-hmac-block-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">hmac-block-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hmac/#t-HmacAlgorithm" title="Crypto.Hmac.HmacAlgorithm">HmacAlgorithm</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-hmac-block-size" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Hmac/source/#L35" title="View source at line 35">Source</a></span></div>
<div class="doc-body">

Digest compression block length in bytes.

</div>
</article>

<article class="doc-entry" id="v-hmac-key">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">hmac-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hmac/#t-HmacAlgorithm" title="Crypto.Hmac.HmacAlgorithm">HmacAlgorithm</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hmac/#t-HmacKey" title="Crypto.Hmac.HmacKey">HmacKey</a></code><span class="doc-sig-links"><a href="#v-hmac-key" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Hmac/source/#L45" title="View source at line 45">Source</a></span></div>
<div class="doc-body">

Normalize an arbitrary key and precompute its two pads. The caller
can mutate the original key after preparation.

</div>
</article>

<article class="doc-entry" id="v-hmac-reset">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">hmac-reset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hmac/#t-HmacState" title="Crypto.Hmac.HmacState">HmacState</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hmac/#t-HmacState" title="Crypto.Hmac.HmacState">HmacState</a></code><span class="doc-sig-links"><a href="#v-hmac-reset" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Hmac/source/#L91" title="View source at line 91">Source</a></span></div>
<div class="doc-body">

Start a new empty message while retaining the prepared key.

</div>
</article>

<article class="doc-entry" id="v-hmac-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">hmac-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hmac/#t-HmacAlgorithm" title="Crypto.Hmac.HmacAlgorithm">HmacAlgorithm</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-hmac-size" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Hmac/source/#L31" title="View source at line 31">Source</a></span></div>
<div class="doc-body">

Full authentication tag length in bytes.

</div>
</article>

<article class="doc-entry" id="v-hmac-state">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">hmac-state</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hmac/#t-HmacKey" title="Crypto.Hmac.HmacKey">HmacKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hmac/#t-HmacState" title="Crypto.Hmac.HmacState">HmacState</a></code><span class="doc-sig-links"><a href="#v-hmac-state" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Hmac/source/#L74" title="View source at line 74">Source</a></span></div>
<div class="doc-body">

Begin a persistent message under a prepared key.

</div>
</article>

<article class="doc-entry" id="v-hmac-sum">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">hmac-sum</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hmac/#t-HmacState" title="Crypto.Hmac.HmacState">HmacState</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-hmac-sum" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Hmac/source/#L83" title="View source at line 83">Source</a></span></div>
<div class="doc-body">

Return the full tag without consuming the state.

</div>
</article>

<article class="doc-entry" id="v-hmac-tag">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">hmac-tag</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hmac/#t-HmacKey" title="Crypto.Hmac.HmacKey">HmacKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-hmac-tag" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Hmac/source/#L60" title="View source at line 60">Source</a></span></div>
<div class="doc-body">

Compute an independent full-width tag with a reusable prepared key.

</div>
</article>

<article class="doc-entry" id="v-hmac-update">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">hmac-update</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hmac/#t-HmacState" title="Crypto.Hmac.HmacState">HmacState</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hmac/#t-HmacState" title="Crypto.Hmac.HmacState">HmacState</a></code><span class="doc-sig-links"><a href="#v-hmac-update" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Hmac/source/#L78" title="View source at line 78">Source</a></span></div>
<div class="doc-body">

Append a snapshot of one message chunk without changing earlier states.

</div>
</article>

<article class="doc-entry" id="v-hmac-verify">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">hmac-verify</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hmac/#t-HmacKey" title="Crypto.Hmac.HmacKey">HmacKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-hmac-verify" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Hmac/source/#L70" title="View source at line 70">Source</a></span></div>
<div class="doc-body">

Verify a full tag. Unequal lengths fail; equal-length comparisons visit every byte.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Crypto/Hmac.cinch`](/packages/stdlib/Crypto.Hmac/source/). Source SHA-256: `e585b6638ff0452e24305d5452c798253e0ba4e72c6381a1fad738f929592408`.
