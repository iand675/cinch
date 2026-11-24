# Text.Sha1

SHA-1 for legacy interoperability, with a four-lane SIMD schedule, persistent states and checked binary checkpoints.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--source">Ship source</span> <code>import Text.Sha1</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Text.Sha1/source/">View source</a><a href="/packages/search/?q=%2BText.Sha1">Search module</a><a href="/reference-sources/Text.Sha1.cinch" download="Text.Sha1.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bytes](Data.Bytes.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Text.Binary](Text.Binary.md).

<details class="doc-closure"><summary>Ship-source closure (3 files)</summary>

Copy these modules beside your entry file before importing `Text.Sha1`: [Data.Bytes.Word](Data.Bytes.Word.md), [Text.Binary](Text.Binary.md), [Text.Sha1](Text.Sha1.md).

```sh
cp seed/prelude/Data/Bytes/Word.cinch ./Data.Bytes.Word.cinch
cp seed/prelude/Text/Binary.cinch ./Text.Binary.cinch
cp seed/prelude/Text/Sha1.cinch ./Text.Sha1.cinch
```

</details>

<details class="doc-notes"><summary>Module notes</summary>

SHA-1 digests, persistent streaming states and binary checkpoints.
SHA-1 is broken for collision resistance: use it only to interoperate with
existing formats and protocols, never for new signatures or integrity checks.
Compression expands four schedule words per step in SIMD lanes and evaluates
each of the eighty ordered rounds once with native rotates. Updates read exact
64-byte blocks while retaining their input owner and copy pending tails into
private storage. Retaining a state branches the computation. Digest and
checkpoint words use network order; byte counts wrap at 64 bits.

</details>

<details class="doc-synopsis" open><summary>Synopsis (16 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-Sha1State">Sha1State</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Sha1State</strong></code></li><li><a class="syn-name" href="#t-Sha1StateError">Sha1StateError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Sha1StateError</strong></code></li><li><a class="syn-name" href="#v-sha1-block-size">sha1-block-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sha1-block-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-sha1-size">sha1-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sha1-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-sha1-state">sha1-state</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sha1-state</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha1/#t-Sha1State" title="Text.Sha1.Sha1State">Sha1State</a></code></li><li><a class="syn-name" href="#v-sha1-state-append">sha1-state-append</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sha1-state-append</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha1/#t-Sha1State" title="Text.Sha1.Sha1State">Sha1State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-sha1-state-append-save">sha1-state-append-save</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sha1-state-append-save</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha1/#t-Sha1State" title="Text.Sha1.Sha1State">Sha1State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-sha1-state-block-size">sha1-state-block-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sha1-state-block-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha1/#t-Sha1State" title="Text.Sha1.Sha1State">Sha1State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-sha1-state-error-message">sha1-state-error-message</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sha1-state-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha1/#t-Sha1StateError" title="Text.Sha1.Sha1StateError">Sha1StateError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-sha1-state-load">sha1-state-load</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sha1-state-load</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha1/#t-Sha1StateError" title="Text.Sha1.Sha1StateError">Sha1StateError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha1/#t-Sha1State" title="Text.Sha1.Sha1State">Sha1State</a></code></li><li><a class="syn-name" href="#v-sha1-state-reset">sha1-state-reset</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sha1-state-reset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha1/#t-Sha1State" title="Text.Sha1.Sha1State">Sha1State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha1/#t-Sha1State" title="Text.Sha1.Sha1State">Sha1State</a></code></li><li><a class="syn-name" href="#v-sha1-state-save">sha1-state-save</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sha1-state-save</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha1/#t-Sha1State" title="Text.Sha1.Sha1State">Sha1State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-sha1-state-size">sha1-state-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sha1-state-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha1/#t-Sha1State" title="Text.Sha1.Sha1State">Sha1State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-sha1-state-sum">sha1-state-sum</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sha1-state-sum</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha1/#t-Sha1State" title="Text.Sha1.Sha1State">Sha1State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-sha1-state-update">sha1-state-update</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sha1-state-update</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha1/#t-Sha1State" title="Text.Sha1.Sha1State">Sha1State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha1/#t-Sha1State" title="Text.Sha1.Sha1State">Sha1State</a></code></li><li><a class="syn-name" href="#v-sha1-sum">sha1-sum</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sha1-sum</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-Sha1State">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Sha1State</strong></code><span class="doc-sig-links"><a href="#t-Sha1State" title="Permalink">#</a><a href="/packages/stdlib/Text.Sha1/source/#L38" title="View source at line 38">Source</a></span></div>
<div class="doc-body">

An immutable wrapping byte count, copied partial block and chaining value.

</div>
</article>

<article class="doc-entry" id="t-Sha1StateError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Sha1StateError</strong></code><span class="doc-sig-links"><a href="#t-Sha1StateError" title="Permalink">#</a><a href="/packages/stdlib/Text.Sha1/source/#L211" title="View source at line 211">Source</a></span></div>
<div class="doc-body">

A checkpoint's identifier or exact byte length is invalid.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-InvalidSha1StateIdentifier"><code><strong class="tok-constructor">InvalidSha1StateIdentifier</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha1/#t-Sha1StateError" title="Text.Sha1.Sha1StateError">Sha1StateError</a></code></li><li id="k-InvalidSha1StateSize"><code><strong class="tok-constructor">InvalidSha1StateSize</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha1/#t-Sha1StateError" title="Text.Sha1.Sha1StateError">Sha1StateError</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-sha1-block-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sha1-block-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-sha1-block-size" title="Permalink">#</a><a href="/packages/stdlib/Text.Sha1/source/#L29" title="View source at line 29">Source</a></span></div>
<div class="doc-body">

Compression block size in bytes, always 64.

</div>
</article>

<article class="doc-entry" id="v-sha1-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sha1-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-sha1-size" title="Permalink">#</a><a href="/packages/stdlib/Text.Sha1/source/#L25" title="View source at line 25">Source</a></span></div>
<div class="doc-body">

Digest size in bytes, always 20.

</div>
</article>

<article class="doc-entry" id="v-sha1-state">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sha1-state</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha1/#t-Sha1State" title="Text.Sha1.Sha1State">Sha1State</a></code><span class="doc-sig-links"><a href="#v-sha1-state" title="Permalink">#</a><a href="/packages/stdlib/Text.Sha1/source/#L47" title="View source at line 47">Source</a></span></div>
<div class="doc-body">

Initial state with no pending bytes.

</div>
</article>

<article class="doc-entry" id="v-sha1-state-append">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sha1-state-append</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha1/#t-Sha1State" title="Text.Sha1.Sha1State">Sha1State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-sha1-state-append" title="Permalink">#</a><a href="/packages/stdlib/Text.Sha1/source/#L191" title="View source at line 191">Source</a></span></div>
<div class="doc-body">

Append the digest to a copied prefix, preserving the state and caller buffer.

</div>
</article>

<article class="doc-entry" id="v-sha1-state-append-save">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sha1-state-append-save</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha1/#t-Sha1State" title="Text.Sha1.Sha1State">Sha1State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-sha1-state-append-save" title="Permalink">#</a><a href="/packages/stdlib/Text.Sha1/source/#L207" title="View source at line 207">Source</a></span></div>
<div class="doc-body">

Append the complete checkpoint to a copied prefix without mutating either input.

</div>
</article>

<article class="doc-entry" id="v-sha1-state-block-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sha1-state-block-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha1/#t-Sha1State" title="Text.Sha1.Sha1State">Sha1State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-sha1-state-block-size" title="Permalink">#</a><a href="/packages/stdlib/Text.Sha1/source/#L152" title="View source at line 152">Source</a></span></div>
<div class="doc-body">

Compression block size in bytes, always 64.

</div>
</article>

<article class="doc-entry" id="v-sha1-state-error-message">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sha1-state-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha1/#t-Sha1StateError" title="Text.Sha1.Sha1StateError">Sha1StateError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-sha1-state-error-message" title="Permalink">#</a><a href="/packages/stdlib/Text.Sha1/source/#L218" title="View source at line 218">Source</a></span></div>
<div class="doc-body">

Render identifier errors or complete required and actual checkpoint byte lengths.

</div>
</article>

<article class="doc-entry" id="v-sha1-state-load">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sha1-state-load</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha1/#t-Sha1StateError" title="Text.Sha1.Sha1StateError">Sha1StateError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha1/#t-Sha1State" title="Text.Sha1.Sha1State">Sha1State</a></code><span class="doc-sig-links"><a href="#v-sha1-state-load" title="Permalink">#</a><a href="/packages/stdlib/Text.Sha1/source/#L229" title="View source at line 229">Source</a></span></div>
<div class="doc-body">

Restore every chaining and count bit, copying only the live partial block into private storage.
Identifier checks precede exact-size checks; unused tail bytes are ignored and saved as zeros.

</div>
</article>

<article class="doc-entry" id="v-sha1-state-reset">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sha1-state-reset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha1/#t-Sha1State" title="Text.Sha1.Sha1State">Sha1State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha1/#t-Sha1State" title="Text.Sha1.Sha1State">Sha1State</a></code><span class="doc-sig-links"><a href="#v-sha1-state-reset" title="Permalink">#</a><a href="/packages/stdlib/Text.Sha1/source/#L144" title="View source at line 144">Source</a></span></div>
<div class="doc-body">

Return the initial state, preserving the supplied state.

</div>
</article>

<article class="doc-entry" id="v-sha1-state-save">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sha1-state-save</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha1/#t-Sha1State" title="Text.Sha1.Sha1State">Sha1State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-sha1-state-save" title="Permalink">#</a><a href="/packages/stdlib/Text.Sha1/source/#L196" title="View source at line 196">Source</a></span></div>
<div class="doc-body">

Return 96 checkpoint bytes: ASCII sha, version byte 1, chaining words, zero-padded tail and count.
Every numeric field uses network order.

</div>
</article>

<article class="doc-entry" id="v-sha1-state-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sha1-state-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha1/#t-Sha1State" title="Text.Sha1.Sha1State">Sha1State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-sha1-state-size" title="Permalink">#</a><a href="/packages/stdlib/Text.Sha1/source/#L148" title="View source at line 148">Source</a></span></div>
<div class="doc-body">

Digest size in bytes, always 20.

</div>
</article>

<article class="doc-entry" id="v-sha1-state-sum">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sha1-state-sum</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha1/#t-Sha1State" title="Text.Sha1.Sha1State">Sha1State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-sha1-state-sum" title="Permalink">#</a><a href="/packages/stdlib/Text.Sha1/source/#L186" title="View source at line 186">Source</a></span></div>
<div class="doc-body">

Return the network-order digest in fresh storage without changing the state.

</div>
</article>

<article class="doc-entry" id="v-sha1-state-update">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sha1-state-update</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha1/#t-Sha1State" title="Text.Sha1.Sha1State">Sha1State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Sha1/#t-Sha1State" title="Text.Sha1.Sha1State">Sha1State</a></code><span class="doc-sig-links"><a href="#v-sha1-state-update" title="Permalink">#</a><a href="/packages/stdlib/Text.Sha1/source/#L61" title="View source at line 61">Source</a></span></div>
<div class="doc-body">

Consume a complete chunk without mutating any previous state or retaining the input buffer.
Empty chunks preserve the supplied state. Counts use explicit unsigned 64-bit wrapping.

</div>
</article>

<article class="doc-entry" id="v-sha1-sum">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sha1-sum</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-sha1-sum" title="Permalink">#</a><a href="/packages/stdlib/Text.Sha1/source/#L51" title="View source at line 51">Source</a></span></div>
<div class="doc-body">

Compute a fresh 20-byte network-order digest of a complete input.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Text/Sha1.cinch`](/packages/stdlib/Text.Sha1/source/). Source SHA-256: `7c4c6ab9c67ff50be3b4273fd01b709a4ad61b5a0943835f38b438f1142ea095`.
