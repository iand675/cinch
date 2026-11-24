# Crypto.Rc4

The RC4 stream cipher as a persistent keystream state, for legacy data only.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Crypto.Rc4</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Crypto.Rc4/source/">View source</a><a href="/packages/search/?q=%2BCrypto.Rc4">Search module</a><a href="/reference-sources/Crypto.Rc4.cinch" download="Crypto.Rc4.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bytes](Data.Bytes.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Text.Show](Text.Show.md).

<details class="doc-notes"><summary>Module notes</summary>

The RC4 stream cipher, for interoperation with legacy formats only.
RC4's keystream is biased and the cipher is broken for confidentiality
(RFC 7465 prohibits it in TLS); it is provided to read and write old data.
A stream is an explicit state machine: the 256-byte permutation and the two
indices. Every call returns the advanced stream with its own copy of the
permutation, so earlier streams remain usable and replay the same keystream.

</details>

<details class="doc-synopsis" open><summary>Synopsis (5 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-Rc4">Rc4</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Rc4</strong></code></li><li><a class="syn-name" href="#t-Rc4Error">Rc4Error</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Rc4Error</strong></code></li><li><a class="syn-name" href="#v-rc4-error-message">rc4-error-message</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rc4-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rc4/#t-Rc4Error" title="Crypto.Rc4.Rc4Error">Rc4Error</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-rc4-new">rc4-new</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rc4-new</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rc4/#t-Rc4Error" title="Crypto.Rc4.Rc4Error">Rc4Error</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rc4/#t-Rc4" title="Crypto.Rc4.Rc4">Rc4</a></code></li><li><a class="syn-name" href="#v-rc4-xor">rc4-xor</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rc4-xor</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rc4/#t-Rc4" title="Crypto.Rc4.Rc4">Rc4</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rc4/#t-Rc4" title="Crypto.Rc4.Rc4">Rc4</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>)</code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-Rc4">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Rc4</strong></code><span class="doc-sig-links"><a href="#t-Rc4" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Rc4/source/#L30" title="View source at line 30">Source</a></span></div>
<div class="doc-body">

Private fields in the order listed above.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-Rc4"><code><strong class="tok-constructor">Rc4</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rc4/#t-Rc4" title="Crypto.Rc4.Rc4">Rc4</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-Rc4Error">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Rc4Error</strong></code><span class="doc-sig-links"><a href="#t-Rc4Error" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Rc4/source/#L19" title="View source at line 19">Source</a></span></div>
<div class="doc-body">

A key outside 1 to 256 bytes; its length.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-Rc4InvalidKeySize"><code><strong class="tok-constructor">Rc4InvalidKeySize</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rc4/#t-Rc4Error" title="Crypto.Rc4.Rc4Error">Rc4Error</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-rc4-error-message">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rc4-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rc4/#t-Rc4Error" title="Crypto.Rc4.Rc4Error">Rc4Error</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-rc4-error-message" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Rc4/source/#L24" title="View source at line 24">Source</a></span></div>
<div class="doc-body">

Render an error.

</div>
</article>

<article class="doc-entry" id="v-rc4-new">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rc4-new</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rc4/#t-Rc4Error" title="Crypto.Rc4.Rc4Error">Rc4Error</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rc4/#t-Rc4" title="Crypto.Rc4.Rc4">Rc4</a></code><span class="doc-sig-links"><a href="#v-rc4-new" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Rc4/source/#L33" title="View source at line 33">Source</a></span></div>
<div class="doc-body">

Key a stream with 1 to 256 key bytes (the key-scheduling algorithm).

</div>
</article>

<article class="doc-entry" id="v-rc4-xor">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rc4-xor</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rc4/#t-Rc4" title="Crypto.Rc4.Rc4">Rc4</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rc4/#t-Rc4" title="Crypto.Rc4.Rc4">Rc4</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>)</code><span class="doc-sig-links"><a href="#v-rc4-xor" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Rc4/source/#L71" title="View source at line 71">Source</a></span></div>
<div class="doc-body">

XOR the input with the next keystream bytes, returning the advanced stream and the output.

</div>
</article>
