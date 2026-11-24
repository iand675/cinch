# Crypto.Ed25519

Deterministic Ed25519 signatures with constant-time signing and strict verification.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--source">Ship source</span> <code>import Crypto.Ed25519</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Crypto.Ed25519/source/">View source</a><a href="/packages/search/?q=%2BCrypto.Ed25519">Search module</a><a href="/reference-sources/Crypto.Ed25519.cinch" download="Crypto.Ed25519.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Crypto.Field25519](Crypto.Field25519.md), [Data.Bytes](Data.Bytes.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Data.Wide](Data.Wide.md), [Text.Sha512](Text.Sha512.md).

<details class="doc-closure"><summary>Ship-source closure (6 files)</summary>

Copy these modules beside your entry file before importing `Crypto.Ed25519`: [Crypto.Ed25519](Crypto.Ed25519.md), [Crypto.Field25519](Crypto.Field25519.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Text.Binary](Text.Binary.md), [Text.Sha512](Text.Sha512.md).

```sh
cp seed/prelude/Crypto/Ed25519.cinch ./Crypto.Ed25519.cinch
cp seed/prelude/Crypto/Field25519.cinch ./Crypto.Field25519.cinch
cp seed/prelude/Data/Bytes/Build.cinch ./Data.Bytes.Build.cinch
cp seed/prelude/Data/Bytes/Word.cinch ./Data.Bytes.Word.cinch
cp seed/prelude/Text/Binary.cinch ./Text.Binary.cinch
cp seed/prelude/Text/Sha512.cinch ./Text.Sha512.cinch
```

</details>

<details class="doc-notes"><summary>Module notes</summary>

Ed25519 signatures (RFC 8032) with constant-time key handling and signing.
A private key is a 32-byte seed; signing hashes it with SHA-512 into a
clamped secret scalar and a nonce prefix, so signatures are deterministic.
Points use extended twisted Edwards coordinates with complete addition
formulas; scalar multiplication always doubles and adds and selects with
masks, and scalar arithmetic modulo the group order L is bit-serial with a
single masked conditional subtraction per bit, so secret values never drive
branches or memory addresses. Verification decodes the public key, rejects
signatures whose S is not below L, and compares the encoding of
[S]B - [k]A with R, the cofactorless check most implementations use.

</details>

<details class="doc-synopsis" open><summary>Synopsis (11 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-Ed25519Error">Ed25519Error</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Ed25519Error</strong></code></li><li><a class="syn-name" href="#t-Ed25519PrivateKey">Ed25519PrivateKey</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Ed25519PrivateKey</strong></code></li><li><a class="syn-name" href="#v-ed25519-error-message">ed25519-error-message</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ed25519-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Ed25519/#t-Ed25519Error" title="Crypto.Ed25519.Ed25519Error">Ed25519Error</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-ed25519-private-key">ed25519-private-key</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ed25519-private-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Ed25519/#t-Ed25519Error" title="Crypto.Ed25519.Ed25519Error">Ed25519Error</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Ed25519/#t-Ed25519PrivateKey" title="Crypto.Ed25519.Ed25519PrivateKey">Ed25519PrivateKey</a></code></li><li><a class="syn-name" href="#v-ed25519-public-key">ed25519-public-key</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ed25519-public-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Ed25519/#t-Ed25519PrivateKey" title="Crypto.Ed25519.Ed25519PrivateKey">Ed25519PrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-ed25519-public-key-size">ed25519-public-key-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ed25519-public-key-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-ed25519-seed">ed25519-seed</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ed25519-seed</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Ed25519/#t-Ed25519PrivateKey" title="Crypto.Ed25519.Ed25519PrivateKey">Ed25519PrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-ed25519-seed-size">ed25519-seed-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ed25519-seed-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-ed25519-sig-size">ed25519-sig-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ed25519-sig-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-ed25519-sign">ed25519-sign</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ed25519-sign</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Ed25519/#t-Ed25519PrivateKey" title="Crypto.Ed25519.Ed25519PrivateKey">Ed25519PrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-ed25519-verify">ed25519-verify</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ed25519-verify</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-Ed25519Error">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Ed25519Error</strong></code><span class="doc-sig-links"><a href="#t-Ed25519Error" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Ed25519/source/#L40" title="View source at line 40">Source</a></span></div>
<div class="doc-body">

A rejected seed.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-InvalidEd25519SeedSize"><code><strong class="tok-constructor">InvalidEd25519SeedSize</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Ed25519/#t-Ed25519Error" title="Crypto.Ed25519.Ed25519Error">Ed25519Error</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-Ed25519PrivateKey">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Ed25519PrivateKey</strong></code><span class="doc-sig-links"><a href="#t-Ed25519PrivateKey" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Ed25519/source/#L50" title="View source at line 50">Source</a></span></div>
<div class="doc-body">

An expanded private key: the seed, the clamped secret scalar bytes, the nonce prefix and the public key.

</div>
</article>

# Values

<article class="doc-entry" id="v-ed25519-error-message">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ed25519-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Ed25519/#t-Ed25519Error" title="Crypto.Ed25519.Ed25519Error">Ed25519Error</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-ed25519-error-message" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Ed25519/source/#L45" title="View source at line 45">Source</a></span></div>
<div class="doc-body">

Render an Ed25519 error.

</div>
</article>

<article class="doc-entry" id="v-ed25519-private-key">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ed25519-private-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Ed25519/#t-Ed25519Error" title="Crypto.Ed25519.Ed25519Error">Ed25519Error</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Ed25519/#t-Ed25519PrivateKey" title="Crypto.Ed25519.Ed25519PrivateKey">Ed25519PrivateKey</a></code><span class="doc-sig-links"><a href="#v-ed25519-private-key" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Ed25519/source/#L55" title="View source at line 55">Source</a></span></div>
<div class="doc-body">

Expand a 32-byte seed into a private key and derive its public key.

</div>
</article>

<article class="doc-entry" id="v-ed25519-public-key">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ed25519-public-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Ed25519/#t-Ed25519PrivateKey" title="Crypto.Ed25519.Ed25519PrivateKey">Ed25519PrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-ed25519-public-key" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Ed25519/source/#L68" title="View source at line 68">Source</a></span></div>
<div class="doc-body">

The 32-byte public key, in fresh storage.

</div>
</article>

<article class="doc-entry" id="v-ed25519-public-key-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ed25519-public-key-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-ed25519-public-key-size" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Ed25519/source/#L32" title="View source at line 32">Source</a></span></div>
<div class="doc-body">

Public key size in bytes, 32.

</div>
</article>

<article class="doc-entry" id="v-ed25519-seed">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ed25519-seed</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Ed25519/#t-Ed25519PrivateKey" title="Crypto.Ed25519.Ed25519PrivateKey">Ed25519PrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-ed25519-seed" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Ed25519/source/#L64" title="View source at line 64">Source</a></span></div>
<div class="doc-body">

The seed, in fresh storage.

</div>
</article>

<article class="doc-entry" id="v-ed25519-seed-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ed25519-seed-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-ed25519-seed-size" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Ed25519/source/#L28" title="View source at line 28">Source</a></span></div>
<div class="doc-body">

Seed size in bytes, 32.

</div>
</article>

<article class="doc-entry" id="v-ed25519-sig-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ed25519-sig-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-ed25519-sig-size" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Ed25519/source/#L36" title="View source at line 36">Source</a></span></div>
<div class="doc-body">

Signature size in bytes, 64.

</div>
</article>

<article class="doc-entry" id="v-ed25519-sign">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ed25519-sign</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Ed25519/#t-Ed25519PrivateKey" title="Crypto.Ed25519.Ed25519PrivateKey">Ed25519PrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-ed25519-sign" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Ed25519/source/#L73" title="View source at line 73">Source</a></span></div>
<div class="doc-body">

Sign a message: R = [r]B for r = SHA-512(prefix || message) mod L, and S = r + k s mod L
for k = SHA-512(R || public key || message) mod L. Returns the 64-byte signature R || S.

</div>
</article>

<article class="doc-entry" id="v-ed25519-verify">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ed25519-verify</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-ed25519-verify" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Ed25519/source/#L83" title="View source at line 83">Source</a></span></div>
<div class="doc-body">

Verify a 64-byte signature of a message under a 32-byte public key.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Crypto/Ed25519.cinch`](/packages/stdlib/Crypto.Ed25519/source/). Source SHA-256: `c9653a1c9514393759e3c07a4a826ff1fdf856d9dcedd5eb4ac509130350c707`.
