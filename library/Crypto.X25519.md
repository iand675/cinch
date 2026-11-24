# Crypto.X25519

Constant-time X25519 Diffie-Hellman key agreement with low-order peer rejection.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--source">Ship source</span> <code>import Crypto.X25519</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Crypto.X25519/source/">View source</a><a href="/packages/search/?q=%2BCrypto.X25519">Search module</a><a href="/reference-sources/Crypto.X25519.cinch" download="Crypto.X25519.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Crypto.Field25519](Crypto.Field25519.md), [Data.Bytes](Data.Bytes.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md).

<details class="doc-closure"><summary>Ship-source closure (3 files)</summary>

Copy these modules beside your entry file before importing `Crypto.X25519`: [Crypto.Field25519](Crypto.Field25519.md), [Crypto.X25519](Crypto.X25519.md), [Data.Bytes.Word](Data.Bytes.Word.md).

```sh
cp seed/prelude/Crypto/Field25519.cinch ./Crypto.Field25519.cinch
cp seed/prelude/Crypto/X25519.cinch ./Crypto.X25519.cinch
cp seed/prelude/Data/Bytes/Word.cinch ./Data.Bytes.Word.cinch
```

</details>

<details class="doc-notes"><summary>Module notes</summary>

X25519 Diffie-Hellman key agreement (RFC 7748) in constant time.
A private key is 32 secret bytes, clamped as the function requires; the
public key is the private key applied to the base point u = 9. Scalar
multiplication is the Montgomery ladder over the u-coordinate with masked
conditional swaps and a fixed iteration count, so its timing and memory
accesses do not depend on the secret. A shared secret of all zero bytes,
produced by low-order peer keys, is rejected. Private keys should come from
a cryptographic random source such as Effect.Random.

</details>

<details class="doc-synopsis" open><summary>Synopsis (10 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-X25519Error">X25519Error</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">X25519Error</strong></code></li><li><a class="syn-name" href="#t-X25519PrivateKey">X25519PrivateKey</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">X25519PrivateKey</strong></code></li><li><a class="syn-name" href="#v-x25519">x25519</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">x25519</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X25519/#t-X25519Error" title="Crypto.X25519.X25519Error">X25519Error</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-x25519-base">x25519-base</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">x25519-base</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X25519/#t-X25519Error" title="Crypto.X25519.X25519Error">X25519Error</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-x25519-error-message">x25519-error-message</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">x25519-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X25519/#t-X25519Error" title="Crypto.X25519.X25519Error">X25519Error</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-x25519-key-size">x25519-key-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">x25519-key-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-x25519-private-bytes">x25519-private-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">x25519-private-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X25519/#t-X25519PrivateKey" title="Crypto.X25519.X25519PrivateKey">X25519PrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-x25519-private-key">x25519-private-key</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">x25519-private-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X25519/#t-X25519Error" title="Crypto.X25519.X25519Error">X25519Error</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X25519/#t-X25519PrivateKey" title="Crypto.X25519.X25519PrivateKey">X25519PrivateKey</a></code></li><li><a class="syn-name" href="#v-x25519-public-key">x25519-public-key</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">x25519-public-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X25519/#t-X25519PrivateKey" title="Crypto.X25519.X25519PrivateKey">X25519PrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-x25519-shared">x25519-shared</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">x25519-shared</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X25519/#t-X25519PrivateKey" title="Crypto.X25519.X25519PrivateKey">X25519PrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X25519/#t-X25519Error" title="Crypto.X25519.X25519Error">X25519Error</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-X25519Error">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">X25519Error</strong></code><span class="doc-sig-links"><a href="#t-X25519Error" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X25519/source/#L27" title="View source at line 27">Source</a></span></div>
<div class="doc-body">

A rejected key or a degenerate shared secret.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-InvalidX25519KeySize"><code><strong class="tok-constructor">InvalidX25519KeySize</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X25519/#t-X25519Error" title="Crypto.X25519.X25519Error">X25519Error</a></code></li><li id="k-InvalidX25519PeerSize"><code><strong class="tok-constructor">InvalidX25519PeerSize</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X25519/#t-X25519Error" title="Crypto.X25519.X25519Error">X25519Error</a></code></li><li id="k-LowOrderPeer"><code><strong class="tok-constructor">LowOrderPeer</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X25519/#t-X25519Error" title="Crypto.X25519.X25519Error">X25519Error</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-X25519PrivateKey">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">X25519PrivateKey</strong></code><span class="doc-sig-links"><a href="#t-X25519PrivateKey" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X25519/source/#L43" title="View source at line 43">Source</a></span></div>
<div class="doc-body">

A private key: its 32 secret bytes (copied) and the derived public key.

</div>
</article>

# Values

<article class="doc-entry" id="v-x25519">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">x25519</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X25519/#t-X25519Error" title="Crypto.X25519.X25519Error">X25519Error</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-x25519" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X25519/source/#L67" title="View source at line 67">Source</a></span></div>
<div class="doc-body">

The raw function X25519(scalar, u) with clamping, rejecting an all-zero result.

</div>
</article>

<article class="doc-entry" id="v-x25519-base">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">x25519-base</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X25519/#t-X25519Error" title="Crypto.X25519.X25519Error">X25519Error</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-x25519-base" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X25519/source/#L76" title="View source at line 76">Source</a></span></div>
<div class="doc-body">

X25519(scalar, 9): the public key for a private scalar.

</div>
</article>

<article class="doc-entry" id="v-x25519-error-message">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">x25519-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X25519/#t-X25519Error" title="Crypto.X25519.X25519Error">X25519Error</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-x25519-error-message" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X25519/source/#L36" title="View source at line 36">Source</a></span></div>
<div class="doc-body">

Render an X25519 error.

</div>
</article>

<article class="doc-entry" id="v-x25519-key-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">x25519-key-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-x25519-key-size" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X25519/source/#L23" title="View source at line 23">Source</a></span></div>
<div class="doc-body">

Key, scalar and coordinate size in bytes, always 32.

</div>
</article>

<article class="doc-entry" id="v-x25519-private-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">x25519-private-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X25519/#t-X25519PrivateKey" title="Crypto.X25519.X25519PrivateKey">X25519PrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-x25519-private-bytes" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X25519/source/#L55" title="View source at line 55">Source</a></span></div>
<div class="doc-body">

The private key's secret bytes, in fresh storage.

</div>
</article>

<article class="doc-entry" id="v-x25519-private-key">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">x25519-private-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X25519/#t-X25519Error" title="Crypto.X25519.X25519Error">X25519Error</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X25519/#t-X25519PrivateKey" title="Crypto.X25519.X25519PrivateKey">X25519PrivateKey</a></code><span class="doc-sig-links"><a href="#v-x25519-private-key" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X25519/source/#L48" title="View source at line 48">Source</a></span></div>
<div class="doc-body">

Accept 32 secret bytes as a private key and derive its public key.

</div>
</article>

<article class="doc-entry" id="v-x25519-public-key">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">x25519-public-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X25519/#t-X25519PrivateKey" title="Crypto.X25519.X25519PrivateKey">X25519PrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-x25519-public-key" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X25519/source/#L59" title="View source at line 59">Source</a></span></div>
<div class="doc-body">

The 32-byte public key, in fresh storage.

</div>
</article>

<article class="doc-entry" id="v-x25519-shared">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">x25519-shared</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X25519/#t-X25519PrivateKey" title="Crypto.X25519.X25519PrivateKey">X25519PrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X25519/#t-X25519Error" title="Crypto.X25519.X25519Error">X25519Error</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-x25519-shared" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X25519/source/#L63" title="View source at line 63">Source</a></span></div>
<div class="doc-body">

The shared secret with a peer's 32-byte public key; low-order peers are rejected.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Crypto/X25519.cinch`](/packages/stdlib/Crypto.X25519/source/). Source SHA-256: `84f0ecaa26c3903f7cc622b5552258f3fd3736112d773307ed8ccd3e73958ca3`.
