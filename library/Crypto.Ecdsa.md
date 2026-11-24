# Crypto.Ecdsa

Deterministic (RFC 6979) and hedged P-256 ECDSA signatures in raw and DER form with strict verification.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Crypto.Ecdsa</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Crypto.Ecdsa/source/">View source</a><a href="/packages/search/?q=%2BCrypto.Ecdsa">Search module</a><a href="/reference-sources/Crypto.Ecdsa.cinch" download="Crypto.Ecdsa.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Crypto.Hmac](Crypto.Hmac.md), [Crypto.P256](Crypto.P256.md), [Data.Bytes](Data.Bytes.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Data.List](Data.List.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md).

<details class="doc-notes"><summary>Module notes</summary>

ECDSA signatures over P-256 (FIPS 186-5), with deterministic nonces (RFC 6979) and DER encoding.
A signature over a message digest is the pair (r, s): r is the x-coordinate
of k * G reduced modulo n and s = (e + r * d) / k modulo n, where e is the
digest's leftmost 256 bits. The nonce k comes from the RFC 6979 HMAC-DRBG
over the private key and digest (HMAC-SHA-256), optionally hedged with extra
entropy; Effect.Crypto supplies fresh entropy for randomized signing.
Verification checks both values are in [1, n - 1] and that the x-coordinate of
(e / s) * G + (r / s) * Q matches r. Signatures are 64 fixed-length bytes
(r || s) or a DER SEQUENCE of two INTEGERs, parsed strictly.

</details>

<details class="doc-synopsis" open><summary>Synopsis (9 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-EcdsaError">EcdsaError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">EcdsaError</strong></code></li><li><a class="syn-name" href="#v-ecdsa-error-message">ecdsa-error-message</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ecdsa-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Ecdsa/#t-EcdsaError" title="Crypto.Ecdsa.EcdsaError">EcdsaError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-ecdsa-rfc6979-nonce">ecdsa-rfc6979-nonce</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ecdsa-rfc6979-nonce</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256PrivateKey" title="Crypto.P256.P256PrivateKey">P256PrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256Scalar" title="Crypto.P256.P256Scalar">P256Scalar</a></code></li><li><a class="syn-name" href="#v-ecdsa-sign">ecdsa-sign</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ecdsa-sign</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256PrivateKey" title="Crypto.P256.P256PrivateKey">P256PrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Ecdsa/#t-EcdsaError" title="Crypto.Ecdsa.EcdsaError">EcdsaError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-ecdsa-sign-der">ecdsa-sign-der</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ecdsa-sign-der</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256PrivateKey" title="Crypto.P256.P256PrivateKey">P256PrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Ecdsa/#t-EcdsaError" title="Crypto.Ecdsa.EcdsaError">EcdsaError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-ecdsa-sign-der-hedged">ecdsa-sign-der-hedged</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ecdsa-sign-der-hedged</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256PrivateKey" title="Crypto.P256.P256PrivateKey">P256PrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Ecdsa/#t-EcdsaError" title="Crypto.Ecdsa.EcdsaError">EcdsaError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-ecdsa-sign-hedged">ecdsa-sign-hedged</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ecdsa-sign-hedged</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256PrivateKey" title="Crypto.P256.P256PrivateKey">P256PrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Ecdsa/#t-EcdsaError" title="Crypto.Ecdsa.EcdsaError">EcdsaError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-ecdsa-verify">ecdsa-verify</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ecdsa-verify</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256PublicKey" title="Crypto.P256.P256PublicKey">P256PublicKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-ecdsa-verify-der">ecdsa-verify-der</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ecdsa-verify-der</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256PublicKey" title="Crypto.P256.P256PublicKey">P256PublicKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-EcdsaError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">EcdsaError</strong></code><span class="doc-sig-links"><a href="#t-EcdsaError" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Ecdsa/source/#L25" title="View source at line 25">Source</a></span></div>
<div class="doc-body">

A signature that cannot be produced or parsed.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-EcdsaInvalidSignature"><code><strong class="tok-constructor">EcdsaInvalidSignature</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Ecdsa/#t-EcdsaError" title="Crypto.Ecdsa.EcdsaError">EcdsaError</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-ecdsa-error-message">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ecdsa-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Ecdsa/#t-EcdsaError" title="Crypto.Ecdsa.EcdsaError">EcdsaError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-ecdsa-error-message" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Ecdsa/source/#L30" title="View source at line 30">Source</a></span></div>
<div class="doc-body">

Render an error.

</div>
</article>

<article class="doc-entry" id="v-ecdsa-rfc6979-nonce">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ecdsa-rfc6979-nonce</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256PrivateKey" title="Crypto.P256.P256PrivateKey">P256PrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256Scalar" title="Crypto.P256.P256Scalar">P256Scalar</a></code><span class="doc-sig-links"><a href="#v-ecdsa-rfc6979-nonce" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Ecdsa/source/#L78" title="View source at line 78">Source</a></span></div>
<div class="doc-body">

The RFC 6979 nonce for a key and digest with HMAC-SHA-256, with optional extra entropy appended to the seed.

</div>
</article>

<article class="doc-entry" id="v-ecdsa-sign">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ecdsa-sign</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256PrivateKey" title="Crypto.P256.P256PrivateKey">P256PrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Ecdsa/#t-EcdsaError" title="Crypto.Ecdsa.EcdsaError">EcdsaError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-ecdsa-sign" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Ecdsa/source/#L34" title="View source at line 34">Source</a></span></div>
<div class="doc-body">

Sign a digest with a deterministic RFC 6979 nonce: 64 bytes r || s.

</div>
</article>

<article class="doc-entry" id="v-ecdsa-sign-der">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ecdsa-sign-der</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256PrivateKey" title="Crypto.P256.P256PrivateKey">P256PrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Ecdsa/#t-EcdsaError" title="Crypto.Ecdsa.EcdsaError">EcdsaError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-ecdsa-sign-der" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Ecdsa/source/#L66" title="View source at line 66">Source</a></span></div>
<div class="doc-body">

Sign and DER-encode: SEQUENCE { INTEGER r, INTEGER s }.

</div>
</article>

<article class="doc-entry" id="v-ecdsa-sign-der-hedged">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ecdsa-sign-der-hedged</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256PrivateKey" title="Crypto.P256.P256PrivateKey">P256PrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Ecdsa/#t-EcdsaError" title="Crypto.Ecdsa.EcdsaError">EcdsaError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-ecdsa-sign-der-hedged" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Ecdsa/source/#L70" title="View source at line 70">Source</a></span></div>
<div class="doc-body">

Hedged signing, DER-encoded.

</div>
</article>

<article class="doc-entry" id="v-ecdsa-sign-hedged">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ecdsa-sign-hedged</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256PrivateKey" title="Crypto.P256.P256PrivateKey">P256PrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Ecdsa/#t-EcdsaError" title="Crypto.Ecdsa.EcdsaError">EcdsaError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-ecdsa-sign-hedged" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Ecdsa/source/#L38" title="View source at line 38">Source</a></span></div>
<div class="doc-body">

Sign with the RFC 6979 nonce hedged by extra entropy (section 3.6); empty entropy gives ecdsa-sign.

</div>
</article>

<article class="doc-entry" id="v-ecdsa-verify">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ecdsa-verify</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256PublicKey" title="Crypto.P256.P256PublicKey">P256PublicKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-ecdsa-verify" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Ecdsa/source/#L51" title="View source at line 51">Source</a></span></div>
<div class="doc-body">

Verify a 64-byte r || s signature over a digest.

</div>
</article>

<article class="doc-entry" id="v-ecdsa-verify-der">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ecdsa-verify-der</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256PublicKey" title="Crypto.P256.P256PublicKey">P256PublicKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-ecdsa-verify-der" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Ecdsa/source/#L74" title="View source at line 74">Source</a></span></div>
<div class="doc-body">

Verify a DER signature; any encoding that is not strict DER fails.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Crypto/Ecdsa.cinch`](/packages/stdlib/Crypto.Ecdsa/source/). Source SHA-256: `6f15ab42fb1c78de630051e078c31260bdc5b3aa416ac90423471f79e171ffa1`.
