# Text.Sha256

Portable SHA-224/SHA-256 with generated SIMD schedules, persistent states, network-order digests and checked binary checkpoints.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Text.Sha256</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Text.Sha256/source/">View source</a><a href="/packages/search/?q=%2BText.Sha256">Search module</a><a href="/reference-sources/Text.Sha256.cinch" download="Text.Sha256.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bytes](Data.Bytes.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Data.Wide](Data.Wide.md), [Text.Binary](Text.Binary.md).

<details class="doc-notes"><summary>Module notes</summary>

SHA-224 and SHA-256 digests, persistent streaming states and binary checkpoints.
Compression expands four schedule words per step in portable 128-bit SIMD lanes
and evaluates all sixty-four ordered rounds in registers with native rotates.
Updates read checked 64-byte blocks directly from the input buffer and copy
pending tails into private storage. Retaining a state branches the computation.
Digest and checkpoint words use network order; byte counts wrap at 64 bits.
Checkpoint input and returned buffers never alias the state.

</details>

<details class="doc-synopsis"><summary>Synopsis (19 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-Sha256State">Sha256State</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Sha256State</strong></code></li><li><a class="syn-name" href="#t-Sha256StateError">Sha256StateError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Sha256StateError</strong></code></li><li><a class="syn-name" href="#v-sha224-size">sha224-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sha224-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-sha224-state">sha224-state</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sha224-state</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha256/#t-Sha256State" title="Text.Sha256.Sha256State">Sha256State</a></code></li><li><a class="syn-name" href="#v-sha224-sum">sha224-sum</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sha224-sum</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-sha256-block-size">sha256-block-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sha256-block-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-sha256-size">sha256-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sha256-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-sha256-state">sha256-state</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sha256-state</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha256/#t-Sha256State" title="Text.Sha256.Sha256State">Sha256State</a></code></li><li><a class="syn-name" href="#v-sha256-state-append">sha256-state-append</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sha256-state-append</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha256/#t-Sha256State" title="Text.Sha256.Sha256State">Sha256State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-sha256-state-append-save">sha256-state-append-save</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sha256-state-append-save</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha256/#t-Sha256State" title="Text.Sha256.Sha256State">Sha256State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-sha256-state-block-size">sha256-state-block-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sha256-state-block-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha256/#t-Sha256State" title="Text.Sha256.Sha256State">Sha256State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-sha256-state-error-message">sha256-state-error-message</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sha256-state-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha256/#t-Sha256StateError" title="Text.Sha256.Sha256StateError">Sha256StateError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-sha256-state-load">sha256-state-load</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sha256-state-load</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha256/#t-Sha256State" title="Text.Sha256.Sha256State">Sha256State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha256/#t-Sha256StateError" title="Text.Sha256.Sha256StateError">Sha256StateError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha256/#t-Sha256State" title="Text.Sha256.Sha256State">Sha256State</a></code></li><li><a class="syn-name" href="#v-sha256-state-reset">sha256-state-reset</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sha256-state-reset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha256/#t-Sha256State" title="Text.Sha256.Sha256State">Sha256State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha256/#t-Sha256State" title="Text.Sha256.Sha256State">Sha256State</a></code></li><li><a class="syn-name" href="#v-sha256-state-save">sha256-state-save</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sha256-state-save</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha256/#t-Sha256State" title="Text.Sha256.Sha256State">Sha256State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-sha256-state-size">sha256-state-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sha256-state-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha256/#t-Sha256State" title="Text.Sha256.Sha256State">Sha256State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 28 32 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-sha256-state-sum">sha256-state-sum</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sha256-state-sum</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha256/#t-Sha256State" title="Text.Sha256.Sha256State">Sha256State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-sha256-state-update">sha256-state-update</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sha256-state-update</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha256/#t-Sha256State" title="Text.Sha256.Sha256State">Sha256State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha256/#t-Sha256State" title="Text.Sha256.Sha256State">Sha256State</a></code></li><li><a class="syn-name" href="#v-sha256-sum">sha256-sum</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sha256-sum</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-Sha256State">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Sha256State</strong></code><span class="doc-sig-links"><a href="#t-Sha256State" title="Permalink">#</a><a href="/packages/stdlib/Text.Sha256/source/#L42" title="View source at line 42">Source</a></span></div>
<div class="doc-body">

An immutable variant, wrapping byte count, copied partial block and chaining value.

</div>
</article>

<article class="doc-entry" id="t-Sha256StateError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Sha256StateError</strong></code><span class="doc-sig-links"><a href="#t-Sha256StateError" title="Permalink">#</a><a href="/packages/stdlib/Text.Sha256/source/#L240" title="View source at line 240">Source</a></span></div>
<div class="doc-body">

A checkpoint's complete variant identifier or exact byte length is invalid.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-InvalidSha256StateIdentifier"><code><strong class="tok-constructor">InvalidSha256StateIdentifier</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha256/#t-Sha256StateError" title="Text.Sha256.Sha256StateError">Sha256StateError</a></code></li><li id="k-InvalidSha256StateSize"><code><strong class="tok-constructor">InvalidSha256StateSize</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha256/#t-Sha256StateError" title="Text.Sha256.Sha256StateError">Sha256StateError</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-sha224-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sha224-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-sha224-size" title="Permalink">#</a><a href="/packages/stdlib/Text.Sha256/source/#L29" title="View source at line 29">Source</a></span></div>
<div class="doc-body">

SHA-224 digest size in bytes, always 28.

</div>
</article>

<article class="doc-entry" id="v-sha224-state">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sha224-state</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha256/#t-Sha256State" title="Text.Sha256.Sha256State">Sha256State</a></code><span class="doc-sig-links"><a href="#v-sha224-state" title="Permalink">#</a><a href="/packages/stdlib/Text.Sha256/source/#L51" title="View source at line 51">Source</a></span></div>
<div class="doc-body">

Initial SHA-224 state with its distinct FIPS 180-4 chaining value and no pending bytes.

</div>
</article>

<article class="doc-entry" id="v-sha224-sum">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sha224-sum</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-sha224-sum" title="Permalink">#</a><a href="/packages/stdlib/Text.Sha256/source/#L59" title="View source at line 59">Source</a></span></div>
<div class="doc-body">

Compute a fresh 28-byte network-order SHA-224 digest of a complete input.

</div>
</article>

<article class="doc-entry" id="v-sha256-block-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sha256-block-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-sha256-block-size" title="Permalink">#</a><a href="/packages/stdlib/Text.Sha256/source/#L33" title="View source at line 33">Source</a></span></div>
<div class="doc-body">

Compression block size in bytes for both variants, always 64.

</div>
</article>

<article class="doc-entry" id="v-sha256-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sha256-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-sha256-size" title="Permalink">#</a><a href="/packages/stdlib/Text.Sha256/source/#L25" title="View source at line 25">Source</a></span></div>
<div class="doc-body">

SHA-256 digest size in bytes, always 32.

</div>
</article>

<article class="doc-entry" id="v-sha256-state">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sha256-state</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha256/#t-Sha256State" title="Text.Sha256.Sha256State">Sha256State</a></code><span class="doc-sig-links"><a href="#v-sha256-state" title="Permalink">#</a><a href="/packages/stdlib/Text.Sha256/source/#L47" title="View source at line 47">Source</a></span></div>
<div class="doc-body">

Initial SHA-256 state with the FIPS 180-4 chaining value and no pending bytes.

</div>
</article>

<article class="doc-entry" id="v-sha256-state-append">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sha256-state-append</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha256/#t-Sha256State" title="Text.Sha256.Sha256State">Sha256State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-sha256-state-append" title="Permalink">#</a><a href="/packages/stdlib/Text.Sha256/source/#L220" title="View source at line 220">Source</a></span></div>
<div class="doc-body">

Append the selected digest to a copied prefix, preserving the state and caller buffer.

</div>
</article>

<article class="doc-entry" id="v-sha256-state-append-save">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sha256-state-append-save</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha256/#t-Sha256State" title="Text.Sha256.Sha256State">Sha256State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-sha256-state-append-save" title="Permalink">#</a><a href="/packages/stdlib/Text.Sha256/source/#L236" title="View source at line 236">Source</a></span></div>
<div class="doc-body">

Append the complete checkpoint to a copied prefix without mutating either input.

</div>
</article>

<article class="doc-entry" id="v-sha256-state-block-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sha256-state-block-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha256/#t-Sha256State" title="Text.Sha256.Sha256State">Sha256State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-sha256-state-block-size" title="Permalink">#</a><a href="/packages/stdlib/Text.Sha256/source/#L137" title="View source at line 137">Source</a></span></div>
<div class="doc-body">

Compression block size in bytes, always 64 for either variant.

</div>
</article>

<article class="doc-entry" id="v-sha256-state-error-message">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sha256-state-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha256/#t-Sha256StateError" title="Text.Sha256.Sha256StateError">Sha256StateError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-sha256-state-error-message" title="Permalink">#</a><a href="/packages/stdlib/Text.Sha256/source/#L247" title="View source at line 247">Source</a></span></div>
<div class="doc-body">

Render identifier errors or complete required and actual checkpoint byte lengths.

</div>
</article>

<article class="doc-entry" id="v-sha256-state-load">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sha256-state-load</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha256/#t-Sha256State" title="Text.Sha256.Sha256State">Sha256State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha256/#t-Sha256StateError" title="Text.Sha256.Sha256StateError">Sha256StateError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha256/#t-Sha256State" title="Text.Sha256.Sha256State">Sha256State</a></code><span class="doc-sig-links"><a href="#v-sha256-state-load" title="Permalink">#</a><a href="/packages/stdlib/Text.Sha256/source/#L259" title="View source at line 259">Source</a></span></div>
<div class="doc-body">

Restore every chaining/count bit, copying only the live partial block into private storage.
The template selects the variant. Identifier checks precede exact-size checks;
unused checkpoint tail bytes are ignored and serialized as zeros on the next save.

</div>
</article>

<article class="doc-entry" id="v-sha256-state-reset">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sha256-state-reset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha256/#t-Sha256State" title="Text.Sha256.Sha256State">Sha256State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha256/#t-Sha256State" title="Text.Sha256.Sha256State">Sha256State</a></code><span class="doc-sig-links"><a href="#v-sha256-state-reset" title="Permalink">#</a><a href="/packages/stdlib/Text.Sha256/source/#L125" title="View source at line 125">Source</a></span></div>
<div class="doc-body">

Return the initial state for the same variant, preserving the supplied state.

</div>
</article>

<article class="doc-entry" id="v-sha256-state-save">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sha256-state-save</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha256/#t-Sha256State" title="Text.Sha256.Sha256State">Sha256State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-sha256-state-save" title="Permalink">#</a><a href="/packages/stdlib/Text.Sha256/source/#L225" title="View source at line 225">Source</a></span></div>
<div class="doc-body">

Return 108 checkpoint bytes: sha, variant byte 2/3, chaining words, zero-padded tail and count.
Variant 2 is SHA-224 and variant 3 is SHA-256. Every numeric field uses network order.

</div>
</article>

<article class="doc-entry" id="v-sha256-state-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sha256-state-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha256/#t-Sha256State" title="Text.Sha256.Sha256State">Sha256State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 28 32 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-sha256-state-size" title="Permalink">#</a><a href="/packages/stdlib/Text.Sha256/source/#L129" title="View source at line 129">Source</a></span></div>
<div class="doc-body">

Digest size in bytes, bounded by 28 and 32 and fixed by the state's selected variant.

</div>
</article>

<article class="doc-entry" id="v-sha256-state-sum">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sha256-state-sum</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha256/#t-Sha256State" title="Text.Sha256.Sha256State">Sha256State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-sha256-state-sum" title="Permalink">#</a><a href="/packages/stdlib/Text.Sha256/source/#L184" title="View source at line 184">Source</a></span></div>
<div class="doc-body">

Return the selected network-order digest in fresh storage without changing the state.

</div>
</article>

<article class="doc-entry" id="v-sha256-state-update">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sha256-state-update</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha256/#t-Sha256State" title="Text.Sha256.Sha256State">Sha256State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha256/#t-Sha256State" title="Text.Sha256.Sha256State">Sha256State</a></code><span class="doc-sig-links"><a href="#v-sha256-state-update" title="Permalink">#</a><a href="/packages/stdlib/Text.Sha256/source/#L76" title="View source at line 76">Source</a></span></div>
<div class="doc-body">

Consume a complete chunk without mutating any previous state or retaining the input buffer.
Empty chunks preserve the supplied state. Counts use explicit unsigned 64-bit wrapping.

</div>
</article>

<article class="doc-entry" id="v-sha256-sum">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sha256-sum</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-sha256-sum" title="Permalink">#</a><a href="/packages/stdlib/Text.Sha256/source/#L55" title="View source at line 55">Source</a></span></div>
<div class="doc-body">

Compute a fresh 32-byte network-order SHA-256 digest of a complete input.

</div>
</article>
