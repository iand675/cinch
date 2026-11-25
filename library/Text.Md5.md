# Text.Md5

MD5 for legacy interoperability, with persistent states, little-endian digests and checked binary checkpoints.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Text.Md5</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Text.Md5/source/">View source</a><a href="/packages/search/?q=%2BText.Md5">Search module</a><a href="/reference-sources/Text.Md5.cinch" download="Text.Md5.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bytes](Data.Bytes.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Data.Wide](Data.Wide.md), [Text.Binary](Text.Binary.md).

<details class="doc-notes"><summary>Module notes</summary>

MD5 digests, persistent streaming states and binary checkpoints.
MD5 is broken for collision resistance: use it only to interoperate with
existing formats and protocols, never for signatures or integrity checks.
Compression loads checked 64-byte little-endian blocks directly from the input
buffer and evaluates all sixty-four sequential rounds in registers with native
rotates. Updates copy pending tails into private storage. Retaining a state
branches the computation. Digest words and the length field use little-endian
order as the algorithm defines; checkpoint fields use network order.

</details>

<details class="doc-synopsis" open><summary>Synopsis (16 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-Md5State">Md5State</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Md5State</strong></code></li><li><a class="syn-name" href="#t-Md5StateError">Md5StateError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Md5StateError</strong></code></li><li><a class="syn-name" href="#v-md5-block-size">md5-block-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">md5-block-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-md5-size">md5-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">md5-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-md5-state">md5-state</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">md5-state</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Md5/#t-Md5State" title="Text.Md5.Md5State">Md5State</a></code></li><li><a class="syn-name" href="#v-md5-state-append">md5-state-append</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">md5-state-append</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Md5/#t-Md5State" title="Text.Md5.Md5State">Md5State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-md5-state-append-save">md5-state-append-save</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">md5-state-append-save</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Md5/#t-Md5State" title="Text.Md5.Md5State">Md5State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-md5-state-block-size">md5-state-block-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">md5-state-block-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Md5/#t-Md5State" title="Text.Md5.Md5State">Md5State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-md5-state-error-message">md5-state-error-message</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">md5-state-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Md5/#t-Md5StateError" title="Text.Md5.Md5StateError">Md5StateError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-md5-state-load">md5-state-load</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">md5-state-load</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Md5/#t-Md5StateError" title="Text.Md5.Md5StateError">Md5StateError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Md5/#t-Md5State" title="Text.Md5.Md5State">Md5State</a></code></li><li><a class="syn-name" href="#v-md5-state-reset">md5-state-reset</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">md5-state-reset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Md5/#t-Md5State" title="Text.Md5.Md5State">Md5State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Md5/#t-Md5State" title="Text.Md5.Md5State">Md5State</a></code></li><li><a class="syn-name" href="#v-md5-state-save">md5-state-save</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">md5-state-save</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Md5/#t-Md5State" title="Text.Md5.Md5State">Md5State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-md5-state-size">md5-state-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">md5-state-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Md5/#t-Md5State" title="Text.Md5.Md5State">Md5State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-md5-state-sum">md5-state-sum</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">md5-state-sum</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Md5/#t-Md5State" title="Text.Md5.Md5State">Md5State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-md5-state-update">md5-state-update</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">md5-state-update</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Md5/#t-Md5State" title="Text.Md5.Md5State">Md5State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Md5/#t-Md5State" title="Text.Md5.Md5State">Md5State</a></code></li><li><a class="syn-name" href="#v-md5-sum">md5-sum</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">md5-sum</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-Md5State">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Md5State</strong></code><span class="doc-sig-links"><a href="#t-Md5State" title="Permalink">#</a><a href="/packages/stdlib/Text.Md5/source/#L38" title="View source at line 38">Source</a></span></div>
<div class="doc-body">

An immutable wrapping byte count, copied partial block and chaining value.

</div>
</article>

<article class="doc-entry" id="t-Md5StateError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Md5StateError</strong></code><span class="doc-sig-links"><a href="#t-Md5StateError" title="Permalink">#</a><a href="/packages/stdlib/Text.Md5/source/#L183" title="View source at line 183">Source</a></span></div>
<div class="doc-body">

A checkpoint's identifier or exact byte length is invalid.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-InvalidMd5StateIdentifier"><code><strong class="tok-constructor">InvalidMd5StateIdentifier</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Md5/#t-Md5StateError" title="Text.Md5.Md5StateError">Md5StateError</a></code></li><li id="k-InvalidMd5StateSize"><code><strong class="tok-constructor">InvalidMd5StateSize</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Md5/#t-Md5StateError" title="Text.Md5.Md5StateError">Md5StateError</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-md5-block-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">md5-block-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-md5-block-size" title="Permalink">#</a><a href="/packages/stdlib/Text.Md5/source/#L29" title="View source at line 29">Source</a></span></div>
<div class="doc-body">

Compression block size in bytes, always 64.

</div>
</article>

<article class="doc-entry" id="v-md5-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">md5-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-md5-size" title="Permalink">#</a><a href="/packages/stdlib/Text.Md5/source/#L25" title="View source at line 25">Source</a></span></div>
<div class="doc-body">

Digest size in bytes, always 16.

</div>
</article>

<article class="doc-entry" id="v-md5-state">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">md5-state</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Md5/#t-Md5State" title="Text.Md5.Md5State">Md5State</a></code><span class="doc-sig-links"><a href="#v-md5-state" title="Permalink">#</a><a href="/packages/stdlib/Text.Md5/source/#L47" title="View source at line 47">Source</a></span></div>
<div class="doc-body">

Initial state with no pending bytes.

</div>
</article>

<article class="doc-entry" id="v-md5-state-append">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">md5-state-append</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Md5/#t-Md5State" title="Text.Md5.Md5State">Md5State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-md5-state-append" title="Permalink">#</a><a href="/packages/stdlib/Text.Md5/source/#L163" title="View source at line 163">Source</a></span></div>
<div class="doc-body">

Append the digest to a copied prefix, preserving the state and caller buffer.

</div>
</article>

<article class="doc-entry" id="v-md5-state-append-save">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">md5-state-append-save</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Md5/#t-Md5State" title="Text.Md5.Md5State">Md5State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-md5-state-append-save" title="Permalink">#</a><a href="/packages/stdlib/Text.Md5/source/#L179" title="View source at line 179">Source</a></span></div>
<div class="doc-body">

Append the complete checkpoint to a copied prefix without mutating either input.

</div>
</article>

<article class="doc-entry" id="v-md5-state-block-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">md5-state-block-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Md5/#t-Md5State" title="Text.Md5.Md5State">Md5State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-md5-state-block-size" title="Permalink">#</a><a href="/packages/stdlib/Text.Md5/source/#L116" title="View source at line 116">Source</a></span></div>
<div class="doc-body">

Compression block size in bytes, always 64.

</div>
</article>

<article class="doc-entry" id="v-md5-state-error-message">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">md5-state-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Md5/#t-Md5StateError" title="Text.Md5.Md5StateError">Md5StateError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-md5-state-error-message" title="Permalink">#</a><a href="/packages/stdlib/Text.Md5/source/#L190" title="View source at line 190">Source</a></span></div>
<div class="doc-body">

Render identifier errors or complete required and actual checkpoint byte lengths.

</div>
</article>

<article class="doc-entry" id="v-md5-state-load">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">md5-state-load</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Md5/#t-Md5StateError" title="Text.Md5.Md5StateError">Md5StateError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Md5/#t-Md5State" title="Text.Md5.Md5State">Md5State</a></code><span class="doc-sig-links"><a href="#v-md5-state-load" title="Permalink">#</a><a href="/packages/stdlib/Text.Md5/source/#L201" title="View source at line 201">Source</a></span></div>
<div class="doc-body">

Restore every chaining and count bit, copying only the live partial block into private storage.
Identifier checks precede exact-size checks; unused tail bytes are ignored and saved as zeros.

</div>
</article>

<article class="doc-entry" id="v-md5-state-reset">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">md5-state-reset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Md5/#t-Md5State" title="Text.Md5.Md5State">Md5State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Md5/#t-Md5State" title="Text.Md5.Md5State">Md5State</a></code><span class="doc-sig-links"><a href="#v-md5-state-reset" title="Permalink">#</a><a href="/packages/stdlib/Text.Md5/source/#L108" title="View source at line 108">Source</a></span></div>
<div class="doc-body">

Return the initial state, preserving the supplied state.

</div>
</article>

<article class="doc-entry" id="v-md5-state-save">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">md5-state-save</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Md5/#t-Md5State" title="Text.Md5.Md5State">Md5State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-md5-state-save" title="Permalink">#</a><a href="/packages/stdlib/Text.Md5/source/#L168" title="View source at line 168">Source</a></span></div>
<div class="doc-body">

Return 92 checkpoint bytes: ASCII md5, version byte 1, chaining words, zero-padded tail and count.
Every numeric field uses network order.

</div>
</article>

<article class="doc-entry" id="v-md5-state-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">md5-state-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Md5/#t-Md5State" title="Text.Md5.Md5State">Md5State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-md5-state-size" title="Permalink">#</a><a href="/packages/stdlib/Text.Md5/source/#L112" title="View source at line 112">Source</a></span></div>
<div class="doc-body">

Digest size in bytes, always 16.

</div>
</article>

<article class="doc-entry" id="v-md5-state-sum">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">md5-state-sum</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Md5/#t-Md5State" title="Text.Md5.Md5State">Md5State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-md5-state-sum" title="Permalink">#</a><a href="/packages/stdlib/Text.Md5/source/#L158" title="View source at line 158">Source</a></span></div>
<div class="doc-body">

Return the digest in fresh storage without changing the state.

</div>
</article>

<article class="doc-entry" id="v-md5-state-update">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">md5-state-update</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Md5/#t-Md5State" title="Text.Md5.Md5State">Md5State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Md5/#t-Md5State" title="Text.Md5.Md5State">Md5State</a></code><span class="doc-sig-links"><a href="#v-md5-state-update" title="Permalink">#</a><a href="/packages/stdlib/Text.Md5/source/#L61" title="View source at line 61">Source</a></span></div>
<div class="doc-body">

Consume a complete chunk without mutating any previous state or retaining the input buffer.
Empty chunks preserve the supplied state. Counts use explicit unsigned 64-bit wrapping.

</div>
</article>

<article class="doc-entry" id="v-md5-sum">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">md5-sum</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-md5-sum" title="Permalink">#</a><a href="/packages/stdlib/Text.Md5/source/#L51" title="View source at line 51">Source</a></span></div>
<div class="doc-body">

Compute a fresh 16-byte digest of a complete input.

</div>
</article>
