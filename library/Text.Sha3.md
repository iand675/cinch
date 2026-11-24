# Text.Sha3

SHA3-224/256/384/512, SHAKE128/256 and cSHAKE128/256 over a register-resident Keccak permutation, with persistent sponges, output readers and checkpoints.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Text.Sha3</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Text.Sha3/source/">View source</a><a href="/packages/search/?q=%2BText.Sha3">Search module</a><a href="/reference-sources/Text.Sha3.cinch" download="Text.Sha3.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bytes](Data.Bytes.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Text.Binary](Text.Binary.md).

<details class="doc-notes"><summary>Module notes</summary>

SHA3-224/256/384/512 digests, SHAKE128/256 and cSHAKE128/256 extendable output, persistent sponges and checkpoints.
Keccak-f[1600] runs as generated straight-line code over twenty-five private
64-bit lanes with native rotates; every round's theta, rho, pi and chi are
fused per output lane so the lanes stay in registers. A sponge is sequential
across rounds, so single-message hashing is scalar rather than SIMD. Updates
absorb whole rate-sized blocks directly from the retained input owner and copy
pending tails into private storage. Retaining a state or reader branches the
computation; buffers returned or accepted are never aliased by a state.

</details>

<details class="doc-synopsis"><summary>Synopsis (29 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-Sha3State">Sha3State</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Sha3State</strong></code></li><li><a class="syn-name" href="#t-Sha3StateError">Sha3StateError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Sha3StateError</strong></code></li><li><a class="syn-name" href="#t-ShakeReader">ShakeReader</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ShakeReader</strong></code></li><li><a class="syn-name" href="#v-cshake128-state">cshake128-state</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cshake128-state</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha3/#t-Sha3State" title="Text.Sha3.Sha3State">Sha3State</a></code></li><li><a class="syn-name" href="#v-cshake256-state">cshake256-state</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cshake256-state</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha3/#t-Sha3State" title="Text.Sha3.Sha3State">Sha3State</a></code></li><li><a class="syn-name" href="#v-sha3-state-append">sha3-state-append</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sha3-state-append</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha3/#t-Sha3State" title="Text.Sha3.Sha3State">Sha3State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-sha3-state-append-save">sha3-state-append-save</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sha3-state-append-save</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha3/#t-Sha3State" title="Text.Sha3.Sha3State">Sha3State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-sha3-state-block-size">sha3-state-block-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sha3-state-block-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha3/#t-Sha3State" title="Text.Sha3.Sha3State">Sha3State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-sha3-state-error-message">sha3-state-error-message</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sha3-state-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha3/#t-Sha3StateError" title="Text.Sha3.Sha3StateError">Sha3StateError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-sha3-state-load">sha3-state-load</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sha3-state-load</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha3/#t-Sha3State" title="Text.Sha3.Sha3State">Sha3State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha3/#t-Sha3StateError" title="Text.Sha3.Sha3StateError">Sha3StateError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha3/#t-Sha3State" title="Text.Sha3.Sha3State">Sha3State</a></code></li><li><a class="syn-name" href="#v-sha3-state-reset">sha3-state-reset</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sha3-state-reset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha3/#t-Sha3State" title="Text.Sha3.Sha3State">Sha3State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha3/#t-Sha3State" title="Text.Sha3.Sha3State">Sha3State</a></code></li><li><a class="syn-name" href="#v-sha3-state-save">sha3-state-save</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sha3-state-save</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha3/#t-Sha3State" title="Text.Sha3.Sha3State">Sha3State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-sha3-state-size">sha3-state-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sha3-state-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha3/#t-Sha3State" title="Text.Sha3.Sha3State">Sha3State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-sha3-state-sum">sha3-state-sum</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sha3-state-sum</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha3/#t-Sha3State" title="Text.Sha3.Sha3State">Sha3State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-sha3-state-update">sha3-state-update</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sha3-state-update</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha3/#t-Sha3State" title="Text.Sha3.Sha3State">Sha3State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha3/#t-Sha3State" title="Text.Sha3.Sha3State">Sha3State</a></code></li><li><a class="syn-name" href="#v-sha3-state224">sha3-state224</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sha3-state224</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha3/#t-Sha3State" title="Text.Sha3.Sha3State">Sha3State</a></code></li><li><a class="syn-name" href="#v-sha3-state256">sha3-state256</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sha3-state256</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha3/#t-Sha3State" title="Text.Sha3.Sha3State">Sha3State</a></code></li><li><a class="syn-name" href="#v-sha3-state384">sha3-state384</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sha3-state384</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha3/#t-Sha3State" title="Text.Sha3.Sha3State">Sha3State</a></code></li><li><a class="syn-name" href="#v-sha3-state512">sha3-state512</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sha3-state512</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha3/#t-Sha3State" title="Text.Sha3.Sha3State">Sha3State</a></code></li><li><a class="syn-name" href="#v-sha3-sum224">sha3-sum224</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sha3-sum224</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-sha3-sum256">sha3-sum256</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sha3-sum256</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-sha3-sum384">sha3-sum384</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sha3-sum384</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-sha3-sum512">sha3-sum512</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sha3-sum512</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-shake-read">shake-read</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">shake-read</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha3/#t-Sha3State" title="Text.Sha3.Sha3State">Sha3State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha3/#t-ShakeReader" title="Text.Sha3.ShakeReader">ShakeReader</a>)</code></li><li><a class="syn-name" href="#v-shake-reader-read">shake-reader-read</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">shake-reader-read</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha3/#t-ShakeReader" title="Text.Sha3.ShakeReader">ShakeReader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha3/#t-ShakeReader" title="Text.Sha3.ShakeReader">ShakeReader</a>)</code></li><li><a class="syn-name" href="#v-shake128-state">shake128-state</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">shake128-state</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha3/#t-Sha3State" title="Text.Sha3.Sha3State">Sha3State</a></code></li><li><a class="syn-name" href="#v-shake128-sum">shake128-sum</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">shake128-sum</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-shake256-state">shake256-state</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">shake256-state</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha3/#t-Sha3State" title="Text.Sha3.Sha3State">Sha3State</a></code></li><li><a class="syn-name" href="#v-shake256-sum">shake256-sum</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">shake256-sum</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-Sha3State">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Sha3State</strong></code><span class="doc-sig-links"><a href="#t-Sha3State" title="Permalink">#</a><a href="/packages/stdlib/Text.Sha3/source/#L35" title="View source at line 35">Source</a></span></div>
<div class="doc-body">

An absorbing sponge: variant code, lanes to restore on reset, copied pending bytes and current lanes.
Codes 1-4 are SHA3-224/256/384/512, 5-6 SHAKE128/256 and 7-8 cSHAKE128/256.
Pending bytes are always fewer than one rate-sized block.

</div>
</article>

<article class="doc-entry" id="t-Sha3StateError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Sha3StateError</strong></code><span class="doc-sig-links"><a href="#t-Sha3StateError" title="Permalink">#</a><a href="/packages/stdlib/Text.Sha3/source/#L341" title="View source at line 341">Source</a></span></div>
<div class="doc-body">

A checkpoint's identifier, exact byte length or pending length is invalid.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-InvalidSha3StateIdentifier"><code><strong class="tok-constructor">InvalidSha3StateIdentifier</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha3/#t-Sha3StateError" title="Text.Sha3.Sha3StateError">Sha3StateError</a></code></li><li id="k-InvalidSha3StateSize"><code><strong class="tok-constructor">InvalidSha3StateSize</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha3/#t-Sha3StateError" title="Text.Sha3.Sha3StateError">Sha3StateError</a></code></li><li id="k-InvalidSha3StateTail"><code><strong class="tok-constructor">InvalidSha3StateTail</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha3/#t-Sha3StateError" title="Text.Sha3.Sha3StateError">Sha3StateError</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-ShakeReader">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ShakeReader</strong></code><span class="doc-sig-links"><a href="#t-ShakeReader" title="Permalink">#</a><a href="/packages/stdlib/Text.Sha3/source/#L40" title="View source at line 40">Source</a></span></div>
<div class="doc-body">

A squeezing sponge for extendable output: variant code, bytes already read from the current block, lanes.

</div>
</article>

# Values

<article class="doc-entry" id="v-cshake128-state">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cshake128-state</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha3/#t-Sha3State" title="Text.Sha3.Sha3State">Sha3State</a></code><span class="doc-sig-links"><a href="#v-cshake128-state" title="Permalink">#</a><a href="/packages/stdlib/Text.Sha3/source/#L93" title="View source at line 93">Source</a></span></div>
<div class="doc-body">

cSHAKE128 with a function name and customization string. With both empty it is exactly SHAKE128.

</div>
</article>

<article class="doc-entry" id="v-cshake256-state">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cshake256-state</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha3/#t-Sha3State" title="Text.Sha3.Sha3State">Sha3State</a></code><span class="doc-sig-links"><a href="#v-cshake256-state" title="Permalink">#</a><a href="/packages/stdlib/Text.Sha3/source/#L97" title="View source at line 97">Source</a></span></div>
<div class="doc-body">

cSHAKE256 with a function name and customization string. With both empty it is exactly SHAKE256.

</div>
</article>

<article class="doc-entry" id="v-sha3-state-append">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sha3-state-append</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha3/#t-Sha3State" title="Text.Sha3.Sha3State">Sha3State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-sha3-state-append" title="Permalink">#</a><a href="/packages/stdlib/Text.Sha3/source/#L276" title="View source at line 276">Source</a></span></div>
<div class="doc-body">

Append the digest to a copied prefix, preserving the state and caller buffer.

</div>
</article>

<article class="doc-entry" id="v-sha3-state-append-save">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sha3-state-append-save</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha3/#t-Sha3State" title="Text.Sha3.Sha3State">Sha3State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-sha3-state-append-save" title="Permalink">#</a><a href="/packages/stdlib/Text.Sha3/source/#L337" title="View source at line 337">Source</a></span></div>
<div class="doc-body">

Append the complete checkpoint to a copied prefix without mutating either input.

</div>
</article>

<article class="doc-entry" id="v-sha3-state-block-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sha3-state-block-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha3/#t-Sha3State" title="Text.Sha3.Sha3State">Sha3State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-sha3-state-block-size" title="Permalink">#</a><a href="/packages/stdlib/Text.Sha3/source/#L240" title="View source at line 240">Source</a></span></div>
<div class="doc-body">

Rate in bytes: the block size absorbed per permutation.

</div>
</article>

<article class="doc-entry" id="v-sha3-state-error-message">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sha3-state-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha3/#t-Sha3StateError" title="Text.Sha3.Sha3StateError">Sha3StateError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-sha3-state-error-message" title="Permalink">#</a><a href="/packages/stdlib/Text.Sha3/source/#L350" title="View source at line 350">Source</a></span></div>
<div class="doc-body">

Render identifier, size and pending-length errors.

</div>
</article>

<article class="doc-entry" id="v-sha3-state-load">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sha3-state-load</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha3/#t-Sha3State" title="Text.Sha3.Sha3State">Sha3State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha3/#t-Sha3StateError" title="Text.Sha3.Sha3StateError">Sha3StateError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha3/#t-Sha3State" title="Text.Sha3.Sha3State">Sha3State</a></code><span class="doc-sig-links"><a href="#v-sha3-state-load" title="Permalink">#</a><a href="/packages/stdlib/Text.Sha3/source/#L371" title="View source at line 371">Source</a></span></div>
<div class="doc-body">

Restore a checkpoint for the template's variant, copying only live pending bytes.
Identifier checks precede the exact-size check, which precedes the pending-length check.

</div>
</article>

<article class="doc-entry" id="v-sha3-state-reset">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sha3-state-reset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha3/#t-Sha3State" title="Text.Sha3.Sha3State">Sha3State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha3/#t-Sha3State" title="Text.Sha3.Sha3State">Sha3State</a></code><span class="doc-sig-links"><a href="#v-sha3-state-reset" title="Permalink">#</a><a href="/packages/stdlib/Text.Sha3/source/#L232" title="View source at line 232">Source</a></span></div>
<div class="doc-body">

Return to the variant's initial lanes, keeping any cSHAKE customization; the supplied state is unchanged.

</div>
</article>

<article class="doc-entry" id="v-sha3-state-save">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sha3-state-save</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha3/#t-Sha3State" title="Text.Sha3.Sha3State">Sha3State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-sha3-state-save" title="Permalink">#</a><a href="/packages/stdlib/Text.Sha3/source/#L314" title="View source at line 314">Source</a></span></div>
<div class="doc-body">

Return 573 checkpoint bytes: ASCII kec, the variant byte, initial lanes, current lanes,
the pending length byte and the pending bytes zero-padded to the largest rate (168).

</div>
</article>

<article class="doc-entry" id="v-sha3-state-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sha3-state-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha3/#t-Sha3State" title="Text.Sha3.Sha3State">Sha3State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-sha3-state-size" title="Permalink">#</a><a href="/packages/stdlib/Text.Sha3/source/#L236" title="View source at line 236">Source</a></span></div>
<div class="doc-body">

Digest size of a fixed variant, or the default output length (32 or 64) of an extendable one.

</div>
</article>

<article class="doc-entry" id="v-sha3-state-sum">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sha3-state-sum</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha3/#t-Sha3State" title="Text.Sha3.Sha3State">Sha3State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-sha3-state-sum" title="Permalink">#</a><a href="/packages/stdlib/Text.Sha3/source/#L272" title="View source at line 272">Source</a></span></div>
<div class="doc-body">

Return the fixed-size digest, or the default-length output of an extendable variant, without changing the state.

</div>
</article>

<article class="doc-entry" id="v-sha3-state-update">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sha3-state-update</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha3/#t-Sha3State" title="Text.Sha3.Sha3State">Sha3State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha3/#t-Sha3State" title="Text.Sha3.Sha3State">Sha3State</a></code><span class="doc-sig-links"><a href="#v-sha3-state-update" title="Permalink">#</a><a href="/packages/stdlib/Text.Sha3/source/#L152" title="View source at line 152">Source</a></span></div>
<div class="doc-body">

Absorb a complete chunk without mutating any previous state or retaining the input buffer.
Empty chunks preserve the supplied state.

</div>
</article>

<article class="doc-entry" id="v-sha3-state224">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sha3-state224</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha3/#t-Sha3State" title="Text.Sha3.Sha3State">Sha3State</a></code><span class="doc-sig-links"><a href="#v-sha3-state224" title="Permalink">#</a><a href="/packages/stdlib/Text.Sha3/source/#L69" title="View source at line 69">Source</a></span></div>
<div class="doc-body">

Initial SHA3-224 state.

</div>
</article>

<article class="doc-entry" id="v-sha3-state256">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sha3-state256</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha3/#t-Sha3State" title="Text.Sha3.Sha3State">Sha3State</a></code><span class="doc-sig-links"><a href="#v-sha3-state256" title="Permalink">#</a><a href="/packages/stdlib/Text.Sha3/source/#L73" title="View source at line 73">Source</a></span></div>
<div class="doc-body">

Initial SHA3-256 state.

</div>
</article>

<article class="doc-entry" id="v-sha3-state384">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sha3-state384</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha3/#t-Sha3State" title="Text.Sha3.Sha3State">Sha3State</a></code><span class="doc-sig-links"><a href="#v-sha3-state384" title="Permalink">#</a><a href="/packages/stdlib/Text.Sha3/source/#L77" title="View source at line 77">Source</a></span></div>
<div class="doc-body">

Initial SHA3-384 state.

</div>
</article>

<article class="doc-entry" id="v-sha3-state512">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sha3-state512</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha3/#t-Sha3State" title="Text.Sha3.Sha3State">Sha3State</a></code><span class="doc-sig-links"><a href="#v-sha3-state512" title="Permalink">#</a><a href="/packages/stdlib/Text.Sha3/source/#L81" title="View source at line 81">Source</a></span></div>
<div class="doc-body">

Initial SHA3-512 state.

</div>
</article>

<article class="doc-entry" id="v-sha3-sum224">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sha3-sum224</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-sha3-sum224" title="Permalink">#</a><a href="/packages/stdlib/Text.Sha3/source/#L127" title="View source at line 127">Source</a></span></div>
<div class="doc-body">

Compute a fresh 28-byte SHA3-224 digest.

</div>
</article>

<article class="doc-entry" id="v-sha3-sum256">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sha3-sum256</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-sha3-sum256" title="Permalink">#</a><a href="/packages/stdlib/Text.Sha3/source/#L131" title="View source at line 131">Source</a></span></div>
<div class="doc-body">

Compute a fresh 32-byte SHA3-256 digest.

</div>
</article>

<article class="doc-entry" id="v-sha3-sum384">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sha3-sum384</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-sha3-sum384" title="Permalink">#</a><a href="/packages/stdlib/Text.Sha3/source/#L135" title="View source at line 135">Source</a></span></div>
<div class="doc-body">

Compute a fresh 48-byte SHA3-384 digest.

</div>
</article>

<article class="doc-entry" id="v-sha3-sum512">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sha3-sum512</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-sha3-sum512" title="Permalink">#</a><a href="/packages/stdlib/Text.Sha3/source/#L139" title="View source at line 139">Source</a></span></div>
<div class="doc-body">

Compute a fresh 64-byte SHA3-512 digest.

</div>
</article>

<article class="doc-entry" id="v-shake-read">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">shake-read</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha3/#t-Sha3State" title="Text.Sha3.Sha3State">Sha3State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha3/#t-ShakeReader" title="Text.Sha3.ShakeReader">ShakeReader</a>)</code><span class="doc-sig-links"><a href="#v-shake-read" title="Permalink">#</a><a href="/packages/stdlib/Text.Sha3/source/#L281" title="View source at line 281">Source</a></span></div>
<div class="doc-body">

Finalize a copy of the sponge and read its first output bytes; the reader continues the stream.
Fixed-size variants may be read past their digest size, which yields their underlying sponge output.

</div>
</article>

<article class="doc-entry" id="v-shake-reader-read">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">shake-reader-read</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha3/#t-ShakeReader" title="Text.Sha3.ShakeReader">ShakeReader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha3/#t-ShakeReader" title="Text.Sha3.ShakeReader">ShakeReader</a>)</code><span class="doc-sig-links"><a href="#v-shake-reader-read" title="Permalink">#</a><a href="/packages/stdlib/Text.Sha3/source/#L286" title="View source at line 286">Source</a></span></div>
<div class="doc-body">

Read the next output bytes, permuting whenever a rate-sized block is exhausted.

</div>
</article>

<article class="doc-entry" id="v-shake128-state">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">shake128-state</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha3/#t-Sha3State" title="Text.Sha3.Sha3State">Sha3State</a></code><span class="doc-sig-links"><a href="#v-shake128-state" title="Permalink">#</a><a href="/packages/stdlib/Text.Sha3/source/#L85" title="View source at line 85">Source</a></span></div>
<div class="doc-body">

Initial SHAKE128 state, with 128-bit security and a 168-byte rate.

</div>
</article>

<article class="doc-entry" id="v-shake128-sum">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">shake128-sum</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-shake128-sum" title="Permalink">#</a><a href="/packages/stdlib/Text.Sha3/source/#L143" title="View source at line 143">Source</a></span></div>
<div class="doc-body">

Read exactly the requested number of SHAKE128 output bytes for a complete input.

</div>
</article>

<article class="doc-entry" id="v-shake256-state">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">shake256-state</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha3/#t-Sha3State" title="Text.Sha3.Sha3State">Sha3State</a></code><span class="doc-sig-links"><a href="#v-shake256-state" title="Permalink">#</a><a href="/packages/stdlib/Text.Sha3/source/#L89" title="View source at line 89">Source</a></span></div>
<div class="doc-body">

Initial SHAKE256 state, with 256-bit security and a 136-byte rate.

</div>
</article>

<article class="doc-entry" id="v-shake256-sum">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">shake256-sum</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-shake256-sum" title="Permalink">#</a><a href="/packages/stdlib/Text.Sha3/source/#L147" title="View source at line 147">Source</a></span></div>
<div class="doc-body">

Read exactly the requested number of SHAKE256 output bytes for a complete input.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Text/Sha3.cinch`](/packages/stdlib/Text.Sha3/source/). Source SHA-256: `26c3d9fe682494d2a244a66098253c9a9a0871eb5bbc1f2405c8b9f99c56b30c`.
