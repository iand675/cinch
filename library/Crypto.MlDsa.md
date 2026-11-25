# Crypto.MlDsa

FIPS 204 ML-DSA-44, ML-DSA-65, and ML-DSA-87 post-quantum digital signatures: deterministic and hedged signing, verification, and key/signature codecs.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Crypto.MlDsa</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Crypto.MlDsa/source/">View source</a><a href="/packages/search/?q=%2BCrypto.MlDsa">Search module</a><a href="/reference-sources/Crypto.MlDsa.cinch" download="Crypto.MlDsa.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Crypto.ConstantTime](Crypto.ConstantTime.md), [Data.Bytes](Data.Bytes.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Text.Sha3](Text.Sha3.md), [Text.Show](Text.Show.md).

<details class="doc-notes"><summary>Module notes</summary>

ML-DSA (FIPS 204), the module-lattice digital signature algorithm, with the ML-DSA-44, ML-DSA-65, and ML-DSA-87 parameter sets.
Key pairs are derived deterministically from a 32-byte seed or loaded from
encoded FIPS 204 private and public key bytes. Signing supports both
deterministic mode and hedged mode with 32 bytes of caller-supplied
randomness, using the pure ML-DSA domain framing (0x00 || len(ctx) || ctx || M)
with context strings up to 255 bytes. Polynomials are stored as 1024-byte
buffers of 256 little-endian 32-bit coefficients reduced modulo q = 8380417,
using exact 64-bit modular arithmetic and the FIPS 204 256-point NTT.

</details>

<details class="doc-synopsis"><summary>Synopsis (20 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-MlDsaError">MlDsaError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MlDsaError</strong></code></li><li><a class="syn-name" href="#t-MlDsaParameters">MlDsaParameters</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MlDsaParameters</strong></code></li><li><a class="syn-name" href="#t-MlDsaPrivateKey">MlDsaPrivateKey</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MlDsaPrivateKey</strong></code></li><li><a class="syn-name" href="#t-MlDsaPublicKey">MlDsaPublicKey</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MlDsaPublicKey</strong></code></li><li><a class="syn-name" href="#v-mldsa-error-message">mldsa-error-message</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mldsa-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlDsa/#t-MlDsaError" title="Crypto.MlDsa.MlDsaError">MlDsaError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-mldsa-key-from-seed">mldsa-key-from-seed</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mldsa-key-from-seed</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlDsa/#t-MlDsaParameters" title="Crypto.MlDsa.MlDsaParameters">MlDsaParameters</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlDsa/#t-MlDsaError" title="Crypto.MlDsa.MlDsaError">MlDsaError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlDsa/#t-MlDsaPrivateKey" title="Crypto.MlDsa.MlDsaPrivateKey">MlDsaPrivateKey</a></code></li><li><a class="syn-name" href="#v-mldsa-private-key">mldsa-private-key</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mldsa-private-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlDsa/#t-MlDsaParameters" title="Crypto.MlDsa.MlDsaParameters">MlDsaParameters</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlDsa/#t-MlDsaError" title="Crypto.MlDsa.MlDsaError">MlDsaError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlDsa/#t-MlDsaPrivateKey" title="Crypto.MlDsa.MlDsaPrivateKey">MlDsaPrivateKey</a></code></li><li><a class="syn-name" href="#v-mldsa-private-key-bytes">mldsa-private-key-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mldsa-private-key-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlDsa/#t-MlDsaPrivateKey" title="Crypto.MlDsa.MlDsaPrivateKey">MlDsaPrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-mldsa-private-key-parameters">mldsa-private-key-parameters</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mldsa-private-key-parameters</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlDsa/#t-MlDsaPrivateKey" title="Crypto.MlDsa.MlDsaPrivateKey">MlDsaPrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlDsa/#t-MlDsaParameters" title="Crypto.MlDsa.MlDsaParameters">MlDsaParameters</a></code></li><li><a class="syn-name" href="#v-mldsa-private-key-size">mldsa-private-key-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mldsa-private-key-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlDsa/#t-MlDsaParameters" title="Crypto.MlDsa.MlDsaParameters">MlDsaParameters</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-mldsa-public-key">mldsa-public-key</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mldsa-public-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlDsa/#t-MlDsaParameters" title="Crypto.MlDsa.MlDsaParameters">MlDsaParameters</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlDsa/#t-MlDsaError" title="Crypto.MlDsa.MlDsaError">MlDsaError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlDsa/#t-MlDsaPublicKey" title="Crypto.MlDsa.MlDsaPublicKey">MlDsaPublicKey</a></code></li><li><a class="syn-name" href="#v-mldsa-public-key-bytes">mldsa-public-key-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mldsa-public-key-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlDsa/#t-MlDsaPublicKey" title="Crypto.MlDsa.MlDsaPublicKey">MlDsaPublicKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-mldsa-public-key-of">mldsa-public-key-of</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mldsa-public-key-of</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlDsa/#t-MlDsaPrivateKey" title="Crypto.MlDsa.MlDsaPrivateKey">MlDsaPrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlDsa/#t-MlDsaPublicKey" title="Crypto.MlDsa.MlDsaPublicKey">MlDsaPublicKey</a></code></li><li><a class="syn-name" href="#v-mldsa-public-key-parameters">mldsa-public-key-parameters</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mldsa-public-key-parameters</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlDsa/#t-MlDsaPublicKey" title="Crypto.MlDsa.MlDsaPublicKey">MlDsaPublicKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlDsa/#t-MlDsaParameters" title="Crypto.MlDsa.MlDsaParameters">MlDsaParameters</a></code></li><li><a class="syn-name" href="#v-mldsa-public-key-size">mldsa-public-key-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mldsa-public-key-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlDsa/#t-MlDsaParameters" title="Crypto.MlDsa.MlDsaParameters">MlDsaParameters</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-mldsa-seed-size">mldsa-seed-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mldsa-seed-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-mldsa-sign-deterministic">mldsa-sign-deterministic</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mldsa-sign-deterministic</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlDsa/#t-MlDsaPrivateKey" title="Crypto.MlDsa.MlDsaPrivateKey">MlDsaPrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlDsa/#t-MlDsaError" title="Crypto.MlDsa.MlDsaError">MlDsaError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-mldsa-sign-with">mldsa-sign-with</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mldsa-sign-with</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlDsa/#t-MlDsaPrivateKey" title="Crypto.MlDsa.MlDsaPrivateKey">MlDsaPrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlDsa/#t-MlDsaError" title="Crypto.MlDsa.MlDsaError">MlDsaError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-mldsa-signature-size">mldsa-signature-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mldsa-signature-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlDsa/#t-MlDsaParameters" title="Crypto.MlDsa.MlDsaParameters">MlDsaParameters</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-mldsa-verify">mldsa-verify</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mldsa-verify</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlDsa/#t-MlDsaPublicKey" title="Crypto.MlDsa.MlDsaPublicKey">MlDsaPublicKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-MlDsaError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MlDsaError</strong></code><span class="doc-sig-links"><a href="#t-MlDsaError" title="Permalink">#</a><a href="/packages/stdlib/Crypto.MlDsa/source/#L104" title="View source at line 104">Source</a></span></div>
<div class="doc-body">

A rejected ML-DSA input or signing failure.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-MlDsaInvalidSeedSize"><code><strong class="tok-constructor">MlDsaInvalidSeedSize</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlDsa/#t-MlDsaError" title="Crypto.MlDsa.MlDsaError">MlDsaError</a></code></li><li id="k-MlDsaInvalidPublicKey"><code><strong class="tok-constructor">MlDsaInvalidPublicKey</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlDsa/#t-MlDsaError" title="Crypto.MlDsa.MlDsaError">MlDsaError</a></code></li><li id="k-MlDsaInvalidPrivateKey"><code><strong class="tok-constructor">MlDsaInvalidPrivateKey</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlDsa/#t-MlDsaError" title="Crypto.MlDsa.MlDsaError">MlDsaError</a></code></li><li id="k-MlDsaInvalidSignature"><code><strong class="tok-constructor">MlDsaInvalidSignature</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlDsa/#t-MlDsaError" title="Crypto.MlDsa.MlDsaError">MlDsaError</a></code></li><li id="k-MlDsaInvalidRandomnessSize"><code><strong class="tok-constructor">MlDsaInvalidRandomnessSize</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlDsa/#t-MlDsaError" title="Crypto.MlDsa.MlDsaError">MlDsaError</a></code></li><li id="k-MlDsaContextTooLong"><code><strong class="tok-constructor">MlDsaContextTooLong</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlDsa/#t-MlDsaError" title="Crypto.MlDsa.MlDsaError">MlDsaError</a></code></li><li id="k-MlDsaSigningFailed"><code><strong class="tok-constructor">MlDsaSigningFailed</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlDsa/#t-MlDsaError" title="Crypto.MlDsa.MlDsaError">MlDsaError</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-MlDsaParameters">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MlDsaParameters</strong></code><span class="doc-sig-links"><a href="#t-MlDsaParameters" title="Permalink">#</a><a href="/packages/stdlib/Crypto.MlDsa/source/#L35" title="View source at line 35">Source</a></span></div>
<div class="doc-body">

An ML-DSA parameter set from FIPS 204 Table 1.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-MlDsa44"><code><strong class="tok-constructor">MlDsa44</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlDsa/#t-MlDsaParameters" title="Crypto.MlDsa.MlDsaParameters">MlDsaParameters</a></code></li><li id="k-MlDsa65"><code><strong class="tok-constructor">MlDsa65</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlDsa/#t-MlDsaParameters" title="Crypto.MlDsa.MlDsaParameters">MlDsaParameters</a></code></li><li id="k-MlDsa87"><code><strong class="tok-constructor">MlDsa87</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlDsa/#t-MlDsaParameters" title="Crypto.MlDsa.MlDsaParameters">MlDsaParameters</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-MlDsaPrivateKey">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MlDsaPrivateKey</strong></code><span class="doc-sig-links"><a href="#t-MlDsaPrivateKey" title="Permalink">#</a><a href="/packages/stdlib/Crypto.MlDsa/source/#L143" title="View source at line 143">Source</a></span></div>
<div class="doc-body">

Fields in the order listed above.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-MlDsaPrivateKey"><code><strong class="tok-constructor">MlDsaPrivateKey</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlDsa/#t-MlDsaPublicKey" title="Crypto.MlDsa.MlDsaPublicKey">MlDsaPublicKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlDsa/#t-MlDsaPrivateKey" title="Crypto.MlDsa.MlDsaPrivateKey">MlDsaPrivateKey</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-MlDsaPublicKey">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MlDsaPublicKey</strong></code><span class="doc-sig-links"><a href="#t-MlDsaPublicKey" title="Permalink">#</a><a href="/packages/stdlib/Crypto.MlDsa/source/#L138" title="View source at line 138">Source</a></span></div>
<div class="doc-body">

Fields in the order listed above.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-MlDsaPublicKey"><code><strong class="tok-constructor">MlDsaPublicKey</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlDsa/#t-MlDsaParameters" title="Crypto.MlDsa.MlDsaParameters">MlDsaParameters</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlDsa/#t-MlDsaPublicKey" title="Crypto.MlDsa.MlDsaPublicKey">MlDsaPublicKey</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-mldsa-error-message">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mldsa-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlDsa/#t-MlDsaError" title="Crypto.MlDsa.MlDsaError">MlDsaError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-mldsa-error-message" title="Permalink">#</a><a href="/packages/stdlib/Crypto.MlDsa/source/#L121" title="View source at line 121">Source</a></span></div>
<div class="doc-body">

Render an ML-DSA error as a diagnostic message.

</div>
</article>

<article class="doc-entry" id="v-mldsa-key-from-seed">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mldsa-key-from-seed</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlDsa/#t-MlDsaParameters" title="Crypto.MlDsa.MlDsaParameters">MlDsaParameters</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlDsa/#t-MlDsaError" title="Crypto.MlDsa.MlDsaError">MlDsaError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlDsa/#t-MlDsaPrivateKey" title="Crypto.MlDsa.MlDsaPrivateKey">MlDsaPrivateKey</a></code><span class="doc-sig-links"><a href="#v-mldsa-key-from-seed" title="Permalink">#</a><a href="/packages/stdlib/Crypto.MlDsa/source/#L166" title="View source at line 166">Source</a></span></div>
<div class="doc-body">

Derive an ML-DSA key pair from a 32-byte seed xi (FIPS 204 Algorithm 6).

</div>
</article>

<article class="doc-entry" id="v-mldsa-private-key">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mldsa-private-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlDsa/#t-MlDsaParameters" title="Crypto.MlDsa.MlDsaParameters">MlDsaParameters</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlDsa/#t-MlDsaError" title="Crypto.MlDsa.MlDsaError">MlDsaError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlDsa/#t-MlDsaPrivateKey" title="Crypto.MlDsa.MlDsaPrivateKey">MlDsaPrivateKey</a></code><span class="doc-sig-links"><a href="#v-mldsa-private-key" title="Permalink">#</a><a href="/packages/stdlib/Crypto.MlDsa/source/#L208" title="View source at line 208">Source</a></span></div>
<div class="doc-body">

Parse and validate an encoded ML-DSA private key (FIPS 204 skDecode), checking secret bounds and public-key consistency.

</div>
</article>

<article class="doc-entry" id="v-mldsa-private-key-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mldsa-private-key-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlDsa/#t-MlDsaPrivateKey" title="Crypto.MlDsa.MlDsaPrivateKey">MlDsaPrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-mldsa-private-key-bytes" title="Permalink">#</a><a href="/packages/stdlib/Crypto.MlDsa/source/#L154" title="View source at line 154">Source</a></span></div>
<div class="doc-body">

Return a fresh copy of an ML-DSA private key's encoded bytes.

</div>
</article>

<article class="doc-entry" id="v-mldsa-private-key-parameters">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mldsa-private-key-parameters</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlDsa/#t-MlDsaPrivateKey" title="Crypto.MlDsa.MlDsaPrivateKey">MlDsaPrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlDsa/#t-MlDsaParameters" title="Crypto.MlDsa.MlDsaParameters">MlDsaParameters</a></code><span class="doc-sig-links"><a href="#v-mldsa-private-key-parameters" title="Permalink">#</a><a href="/packages/stdlib/Crypto.MlDsa/source/#L158" title="View source at line 158">Source</a></span></div>
<div class="doc-body">

Return the parameter set of an ML-DSA private key.

</div>
</article>

<article class="doc-entry" id="v-mldsa-private-key-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mldsa-private-key-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlDsa/#t-MlDsaParameters" title="Crypto.MlDsa.MlDsaParameters">MlDsaParameters</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-mldsa-private-key-size" title="Permalink">#</a><a href="/packages/stdlib/Crypto.MlDsa/source/#L96" title="View source at line 96">Source</a></span></div>
<div class="doc-body">

Encoded private key size in bytes: 2560 for ML-DSA-44, 4032 for ML-DSA-65, 4896 for ML-DSA-87.

</div>
</article>

<article class="doc-entry" id="v-mldsa-public-key">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mldsa-public-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlDsa/#t-MlDsaParameters" title="Crypto.MlDsa.MlDsaParameters">MlDsaParameters</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlDsa/#t-MlDsaError" title="Crypto.MlDsa.MlDsaError">MlDsaError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlDsa/#t-MlDsaPublicKey" title="Crypto.MlDsa.MlDsaPublicKey">MlDsaPublicKey</a></code><span class="doc-sig-links"><a href="#v-mldsa-public-key" title="Permalink">#</a><a href="/packages/stdlib/Crypto.MlDsa/source/#L194" title="View source at line 194">Source</a></span></div>
<div class="doc-body">

Parse and validate an encoded ML-DSA public key (FIPS 204 pkDecode).

</div>
</article>

<article class="doc-entry" id="v-mldsa-public-key-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mldsa-public-key-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlDsa/#t-MlDsaPublicKey" title="Crypto.MlDsa.MlDsaPublicKey">MlDsaPublicKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-mldsa-public-key-bytes" title="Permalink">#</a><a href="/packages/stdlib/Crypto.MlDsa/source/#L150" title="View source at line 150">Source</a></span></div>
<div class="doc-body">

Return a fresh copy of an ML-DSA public key's encoded bytes.

</div>
</article>

<article class="doc-entry" id="v-mldsa-public-key-of">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mldsa-public-key-of</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlDsa/#t-MlDsaPrivateKey" title="Crypto.MlDsa.MlDsaPrivateKey">MlDsaPrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlDsa/#t-MlDsaPublicKey" title="Crypto.MlDsa.MlDsaPublicKey">MlDsaPublicKey</a></code><span class="doc-sig-links"><a href="#v-mldsa-public-key-of" title="Permalink">#</a><a href="/packages/stdlib/Crypto.MlDsa/source/#L162" title="View source at line 162">Source</a></span></div>
<div class="doc-body">

Return the public key corresponding to an ML-DSA private key.

</div>
</article>

<article class="doc-entry" id="v-mldsa-public-key-parameters">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mldsa-public-key-parameters</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlDsa/#t-MlDsaPublicKey" title="Crypto.MlDsa.MlDsaPublicKey">MlDsaPublicKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlDsa/#t-MlDsaParameters" title="Crypto.MlDsa.MlDsaParameters">MlDsaParameters</a></code><span class="doc-sig-links"><a href="#v-mldsa-public-key-parameters" title="Permalink">#</a><a href="/packages/stdlib/Crypto.MlDsa/source/#L146" title="View source at line 146">Source</a></span></div>
<div class="doc-body">

Return the parameter set of an ML-DSA public key.

</div>
</article>

<article class="doc-entry" id="v-mldsa-public-key-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mldsa-public-key-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlDsa/#t-MlDsaParameters" title="Crypto.MlDsa.MlDsaParameters">MlDsaParameters</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-mldsa-public-key-size" title="Permalink">#</a><a href="/packages/stdlib/Crypto.MlDsa/source/#L92" title="View source at line 92">Source</a></span></div>
<div class="doc-body">

Encoded public key size in bytes: 1312 for ML-DSA-44, 1952 for ML-DSA-65, 2592 for ML-DSA-87.

</div>
</article>

<article class="doc-entry" id="v-mldsa-seed-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mldsa-seed-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-mldsa-seed-size" title="Permalink">#</a><a href="/packages/stdlib/Crypto.MlDsa/source/#L88" title="View source at line 88">Source</a></span></div>
<div class="doc-body">

Key-generation seed size in bytes (32).

</div>
</article>

<article class="doc-entry" id="v-mldsa-sign-deterministic">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mldsa-sign-deterministic</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlDsa/#t-MlDsaPrivateKey" title="Crypto.MlDsa.MlDsaPrivateKey">MlDsaPrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlDsa/#t-MlDsaError" title="Crypto.MlDsa.MlDsaError">MlDsaError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-mldsa-sign-deterministic" title="Permalink">#</a><a href="/packages/stdlib/Crypto.MlDsa/source/#L255" title="View source at line 255">Source</a></span></div>
<div class="doc-body">

Produce a deterministic pure ML-DSA signature (FIPS 204 Algorithms 2 and 7 with rnd = 0^32).

</div>
</article>

<article class="doc-entry" id="v-mldsa-sign-with">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mldsa-sign-with</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlDsa/#t-MlDsaPrivateKey" title="Crypto.MlDsa.MlDsaPrivateKey">MlDsaPrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlDsa/#t-MlDsaError" title="Crypto.MlDsa.MlDsaError">MlDsaError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-mldsa-sign-with" title="Permalink">#</a><a href="/packages/stdlib/Crypto.MlDsa/source/#L259" title="View source at line 259">Source</a></span></div>
<div class="doc-body">

Produce a pure ML-DSA signature using 32 bytes of explicit randomness (FIPS 204 Algorithms 2 and 7).

</div>
</article>

<article class="doc-entry" id="v-mldsa-signature-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mldsa-signature-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlDsa/#t-MlDsaParameters" title="Crypto.MlDsa.MlDsaParameters">MlDsaParameters</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-mldsa-signature-size" title="Permalink">#</a><a href="/packages/stdlib/Crypto.MlDsa/source/#L100" title="View source at line 100">Source</a></span></div>
<div class="doc-body">

Encoded signature size in bytes: 2420 for ML-DSA-44, 3309 for ML-DSA-65, 4627 for ML-DSA-87.

</div>
</article>

<article class="doc-entry" id="v-mldsa-verify">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mldsa-verify</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlDsa/#t-MlDsaPublicKey" title="Crypto.MlDsa.MlDsaPublicKey">MlDsaPublicKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-mldsa-verify" title="Permalink">#</a><a href="/packages/stdlib/Crypto.MlDsa/source/#L309" title="View source at line 309">Source</a></span></div>
<div class="doc-body">

Verify a pure ML-DSA signature over msg and ctx (FIPS 204 Algorithms 3 and 8).

</div>
</article>
