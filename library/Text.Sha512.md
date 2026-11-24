# Text.Sha512

Portable SHA-384, SHA-512, SHA-512/224 and SHA-512/256 with two-lane SIMD schedules, native rotates, persistent states and checked binary checkpoints.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Text.Sha512</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Text.Sha512/source/">View source</a><a href="/packages/search/?q=%2BText.Sha512">Search module</a><a href="/reference-sources/Text.Sha512.cinch" download="Text.Sha512.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bytes](Data.Bytes.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Text.Binary](Text.Binary.md).

<details class="doc-notes"><summary>Module notes</summary>

SHA-384, SHA-512, SHA-512/224 and SHA-512/256 digests, persistent streaming states and binary checkpoints.
Compression expands two schedule words per step in independent SIMD lanes and
evaluates every one of the eighty ordered rounds exactly once with native
64-bit rotates. Updates read exact 128-byte blocks while retaining their input
owner, and copy pending tails into private storage. Retaining a state branches
the computation. Digest and checkpoint words use network order. Byte counts
wrap at 64 bits, so the encoded 128-bit message length is exact for every
input shorter than 2^64 bytes. Checkpoint input and returned buffers never
alias the state.

</details>

<details class="doc-synopsis"><summary>Synopsis (25 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-Sha512State">Sha512State</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Sha512State</strong></code></li><li><a class="syn-name" href="#t-Sha512StateError">Sha512StateError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Sha512StateError</strong></code></li><li><a class="syn-name" href="#v-sha384-size">sha384-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sha384-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-sha384-state">sha384-state</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sha384-state</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha512/#t-Sha512State" title="Text.Sha512.Sha512State">Sha512State</a></code></li><li><a class="syn-name" href="#v-sha384-sum">sha384-sum</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sha384-sum</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-sha512-block-size">sha512-block-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sha512-block-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-sha512-size">sha512-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sha512-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-sha512-state">sha512-state</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sha512-state</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha512/#t-Sha512State" title="Text.Sha512.Sha512State">Sha512State</a></code></li><li><a class="syn-name" href="#v-sha512-state-append">sha512-state-append</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sha512-state-append</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha512/#t-Sha512State" title="Text.Sha512.Sha512State">Sha512State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-sha512-state-append-save">sha512-state-append-save</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sha512-state-append-save</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha512/#t-Sha512State" title="Text.Sha512.Sha512State">Sha512State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-sha512-state-block-size">sha512-state-block-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sha512-state-block-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha512/#t-Sha512State" title="Text.Sha512.Sha512State">Sha512State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-sha512-state-error-message">sha512-state-error-message</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sha512-state-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha512/#t-Sha512StateError" title="Text.Sha512.Sha512StateError">Sha512StateError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-sha512-state-load">sha512-state-load</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sha512-state-load</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha512/#t-Sha512State" title="Text.Sha512.Sha512State">Sha512State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha512/#t-Sha512StateError" title="Text.Sha512.Sha512StateError">Sha512StateError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha512/#t-Sha512State" title="Text.Sha512.Sha512State">Sha512State</a></code></li><li><a class="syn-name" href="#v-sha512-state-reset">sha512-state-reset</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sha512-state-reset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha512/#t-Sha512State" title="Text.Sha512.Sha512State">Sha512State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha512/#t-Sha512State" title="Text.Sha512.Sha512State">Sha512State</a></code></li><li><a class="syn-name" href="#v-sha512-state-save">sha512-state-save</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sha512-state-save</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha512/#t-Sha512State" title="Text.Sha512.Sha512State">Sha512State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-sha512-state-size">sha512-state-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sha512-state-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha512/#t-Sha512State" title="Text.Sha512.Sha512State">Sha512State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-sha512-state-sum">sha512-state-sum</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sha512-state-sum</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha512/#t-Sha512State" title="Text.Sha512.Sha512State">Sha512State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-sha512-state-update">sha512-state-update</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sha512-state-update</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha512/#t-Sha512State" title="Text.Sha512.Sha512State">Sha512State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha512/#t-Sha512State" title="Text.Sha512.Sha512State">Sha512State</a></code></li><li><a class="syn-name" href="#v-sha512-sum">sha512-sum</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sha512-sum</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-sha512t224-size">sha512t224-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sha512t224-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-sha512t224-state">sha512t224-state</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sha512t224-state</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha512/#t-Sha512State" title="Text.Sha512.Sha512State">Sha512State</a></code></li><li><a class="syn-name" href="#v-sha512t224-sum">sha512t224-sum</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sha512t224-sum</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-sha512t256-size">sha512t256-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sha512t256-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-sha512t256-state">sha512t256-state</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sha512t256-state</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha512/#t-Sha512State" title="Text.Sha512.Sha512State">Sha512State</a></code></li><li><a class="syn-name" href="#v-sha512t256-sum">sha512t256-sum</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sha512t256-sum</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-Sha512State">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Sha512State</strong></code><span class="doc-sig-links"><a href="#t-Sha512State" title="Permalink">#</a><a href="/packages/stdlib/Text.Sha512/source/#L53" title="View source at line 53">Source</a></span></div>
<div class="doc-body">

An immutable variant code, wrapping byte count, copied partial block and chaining value.

</div>
</article>

<article class="doc-entry" id="t-Sha512StateError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Sha512StateError</strong></code><span class="doc-sig-links"><a href="#t-Sha512StateError" title="Permalink">#</a><a href="/packages/stdlib/Text.Sha512/source/#L293" title="View source at line 293">Source</a></span></div>
<div class="doc-body">

A checkpoint's complete variant identifier or exact byte length is invalid.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-InvalidSha512StateIdentifier"><code><strong class="tok-constructor">InvalidSha512StateIdentifier</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha512/#t-Sha512StateError" title="Text.Sha512.Sha512StateError">Sha512StateError</a></code></li><li id="k-InvalidSha512StateSize"><code><strong class="tok-constructor">InvalidSha512StateSize</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha512/#t-Sha512StateError" title="Text.Sha512.Sha512StateError">Sha512StateError</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-sha384-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sha384-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-sha384-size" title="Permalink">#</a><a href="/packages/stdlib/Text.Sha512/source/#L32" title="View source at line 32">Source</a></span></div>
<div class="doc-body">

SHA-384 digest size in bytes, always 48.

</div>
</article>

<article class="doc-entry" id="v-sha384-state">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sha384-state</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha512/#t-Sha512State" title="Text.Sha512.Sha512State">Sha512State</a></code><span class="doc-sig-links"><a href="#v-sha384-state" title="Permalink">#</a><a href="/packages/stdlib/Text.Sha512/source/#L67" title="View source at line 67">Source</a></span></div>
<div class="doc-body">

Initial SHA-384 state, with its distinct chaining value.

</div>
</article>

<article class="doc-entry" id="v-sha384-sum">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sha384-sum</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-sha384-sum" title="Permalink">#</a><a href="/packages/stdlib/Text.Sha512/source/#L83" title="View source at line 83">Source</a></span></div>
<div class="doc-body">

Compute a fresh 48-byte network-order SHA-384 digest of a complete input.

</div>
</article>

<article class="doc-entry" id="v-sha512-block-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sha512-block-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-sha512-block-size" title="Permalink">#</a><a href="/packages/stdlib/Text.Sha512/source/#L44" title="View source at line 44">Source</a></span></div>
<div class="doc-body">

Compression block size in bytes for every variant, always 128.

</div>
</article>

<article class="doc-entry" id="v-sha512-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sha512-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-sha512-size" title="Permalink">#</a><a href="/packages/stdlib/Text.Sha512/source/#L28" title="View source at line 28">Source</a></span></div>
<div class="doc-body">

SHA-512 digest size in bytes, always 64.

</div>
</article>

<article class="doc-entry" id="v-sha512-state">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sha512-state</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha512/#t-Sha512State" title="Text.Sha512.Sha512State">Sha512State</a></code><span class="doc-sig-links"><a href="#v-sha512-state" title="Permalink">#</a><a href="/packages/stdlib/Text.Sha512/source/#L63" title="View source at line 63">Source</a></span></div>
<div class="doc-body">

Initial SHA-512 state.

</div>
</article>

<article class="doc-entry" id="v-sha512-state-append">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sha512-state-append</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha512/#t-Sha512State" title="Text.Sha512.Sha512State">Sha512State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-sha512-state-append" title="Permalink">#</a><a href="/packages/stdlib/Text.Sha512/source/#L269" title="View source at line 269">Source</a></span></div>
<div class="doc-body">

Append the selected digest to a copied prefix, preserving the state and caller buffer.

</div>
</article>

<article class="doc-entry" id="v-sha512-state-append-save">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sha512-state-append-save</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha512/#t-Sha512State" title="Text.Sha512.Sha512State">Sha512State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-sha512-state-append-save" title="Permalink">#</a><a href="/packages/stdlib/Text.Sha512/source/#L289" title="View source at line 289">Source</a></span></div>
<div class="doc-body">

Append the complete checkpoint to a copied prefix without mutating either input.

</div>
</article>

<article class="doc-entry" id="v-sha512-state-block-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sha512-state-block-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha512/#t-Sha512State" title="Text.Sha512.Sha512State">Sha512State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-sha512-state-block-size" title="Permalink">#</a><a href="/packages/stdlib/Text.Sha512/source/#L208" title="View source at line 208">Source</a></span></div>
<div class="doc-body">

Compression block size in bytes, always 128 for every variant.

</div>
</article>

<article class="doc-entry" id="v-sha512-state-error-message">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sha512-state-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha512/#t-Sha512StateError" title="Text.Sha512.Sha512StateError">Sha512StateError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-sha512-state-error-message" title="Permalink">#</a><a href="/packages/stdlib/Text.Sha512/source/#L300" title="View source at line 300">Source</a></span></div>
<div class="doc-body">

Render identifier errors or complete required and actual checkpoint byte lengths.

</div>
</article>

<article class="doc-entry" id="v-sha512-state-load">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sha512-state-load</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha512/#t-Sha512State" title="Text.Sha512.Sha512State">Sha512State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha512/#t-Sha512StateError" title="Text.Sha512.Sha512StateError">Sha512StateError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha512/#t-Sha512State" title="Text.Sha512.Sha512State">Sha512State</a></code><span class="doc-sig-links"><a href="#v-sha512-state-load" title="Permalink">#</a><a href="/packages/stdlib/Text.Sha512/source/#L312" title="View source at line 312">Source</a></span></div>
<div class="doc-body">

Restore every chaining and count bit, copying only the live partial block into private storage.
The template selects the variant. Identifier checks precede exact-size checks; unused
checkpoint tail bytes are ignored and serialized as zeros on the next save.

</div>
</article>

<article class="doc-entry" id="v-sha512-state-reset">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sha512-state-reset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha512/#t-Sha512State" title="Text.Sha512.Sha512State">Sha512State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha512/#t-Sha512State" title="Text.Sha512.Sha512State">Sha512State</a></code><span class="doc-sig-links"><a href="#v-sha512-state-reset" title="Permalink">#</a><a href="/packages/stdlib/Text.Sha512/source/#L195" title="View source at line 195">Source</a></span></div>
<div class="doc-body">

Return the initial state for the same variant, preserving the supplied state.

</div>
</article>

<article class="doc-entry" id="v-sha512-state-save">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sha512-state-save</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha512/#t-Sha512State" title="Text.Sha512.Sha512State">Sha512State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-sha512-state-save" title="Permalink">#</a><a href="/packages/stdlib/Text.Sha512/source/#L274" title="View source at line 274">Source</a></span></div>
<div class="doc-body">

Return 204 checkpoint bytes: ASCII sha, the variant byte, chaining words, zero-padded tail and count.
Variant bytes 4, 5, 6 and 7 are SHA-384, SHA-512/224, SHA-512/256 and SHA-512. Numbers use network order.

</div>
</article>

<article class="doc-entry" id="v-sha512-state-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sha512-state-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha512/#t-Sha512State" title="Text.Sha512.Sha512State">Sha512State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-sha512-state-size" title="Permalink">#</a><a href="/packages/stdlib/Text.Sha512/source/#L199" title="View source at line 199">Source</a></span></div>
<div class="doc-body">

Digest size in bytes for the state's variant: 48, 28, 32 or 64.

</div>
</article>

<article class="doc-entry" id="v-sha512-state-sum">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sha512-state-sum</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha512/#t-Sha512State" title="Text.Sha512.Sha512State">Sha512State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-sha512-state-sum" title="Permalink">#</a><a href="/packages/stdlib/Text.Sha512/source/#L263" title="View source at line 263">Source</a></span></div>
<div class="doc-body">

Return the selected network-order digest in fresh storage without changing the state.

</div>
</article>

<article class="doc-entry" id="v-sha512-state-update">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sha512-state-update</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha512/#t-Sha512State" title="Text.Sha512.Sha512State">Sha512State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha512/#t-Sha512State" title="Text.Sha512.Sha512State">Sha512State</a></code><span class="doc-sig-links"><a href="#v-sha512-state-update" title="Permalink">#</a><a href="/packages/stdlib/Text.Sha512/source/#L106" title="View source at line 106">Source</a></span></div>
<div class="doc-body">

Consume a complete chunk without mutating any previous state or retaining the input buffer.
Empty chunks preserve the supplied state. Counts use explicit unsigned 64-bit wrapping.

</div>
</article>

<article class="doc-entry" id="v-sha512-sum">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sha512-sum</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-sha512-sum" title="Permalink">#</a><a href="/packages/stdlib/Text.Sha512/source/#L79" title="View source at line 79">Source</a></span></div>
<div class="doc-body">

Compute a fresh 64-byte network-order SHA-512 digest of a complete input.

</div>
</article>

<article class="doc-entry" id="v-sha512t224-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sha512t224-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-sha512t224-size" title="Permalink">#</a><a href="/packages/stdlib/Text.Sha512/source/#L36" title="View source at line 36">Source</a></span></div>
<div class="doc-body">

SHA-512/224 digest size in bytes, always 28.

</div>
</article>

<article class="doc-entry" id="v-sha512t224-state">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sha512t224-state</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha512/#t-Sha512State" title="Text.Sha512.Sha512State">Sha512State</a></code><span class="doc-sig-links"><a href="#v-sha512t224-state" title="Permalink">#</a><a href="/packages/stdlib/Text.Sha512/source/#L71" title="View source at line 71">Source</a></span></div>
<div class="doc-body">

Initial SHA-512/224 state, whose chaining value comes from the standard generation function.

</div>
</article>

<article class="doc-entry" id="v-sha512t224-sum">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sha512t224-sum</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-sha512t224-sum" title="Permalink">#</a><a href="/packages/stdlib/Text.Sha512/source/#L87" title="View source at line 87">Source</a></span></div>
<div class="doc-body">

Compute a fresh 28-byte network-order SHA-512/224 digest of a complete input.

</div>
</article>

<article class="doc-entry" id="v-sha512t256-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sha512t256-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-sha512t256-size" title="Permalink">#</a><a href="/packages/stdlib/Text.Sha512/source/#L40" title="View source at line 40">Source</a></span></div>
<div class="doc-body">

SHA-512/256 digest size in bytes, always 32.

</div>
</article>

<article class="doc-entry" id="v-sha512t256-state">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sha512t256-state</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha512/#t-Sha512State" title="Text.Sha512.Sha512State">Sha512State</a></code><span class="doc-sig-links"><a href="#v-sha512t256-state" title="Permalink">#</a><a href="/packages/stdlib/Text.Sha512/source/#L75" title="View source at line 75">Source</a></span></div>
<div class="doc-body">

Initial SHA-512/256 state, whose chaining value comes from the standard generation function.

</div>
</article>

<article class="doc-entry" id="v-sha512t256-sum">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sha512t256-sum</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-sha512t256-sum" title="Permalink">#</a><a href="/packages/stdlib/Text.Sha512/source/#L91" title="View source at line 91">Source</a></span></div>
<div class="doc-body">

Compute a fresh 32-byte network-order SHA-512/256 digest of a complete input.

</div>
</article>
