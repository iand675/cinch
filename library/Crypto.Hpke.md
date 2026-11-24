# Crypto.Hpke

Hybrid public key encryption (RFC 9180) over DHKEM X25519 and P-256, HKDF and AES-GCM or ChaCha20-Poly1305, in all four modes.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Crypto.Hpke</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Crypto.Hpke/source/">View source</a><a href="/packages/search/?q=%2BCrypto.Hpke">Search module</a><a href="/reference-sources/Crypto.Hpke.cinch" download="Crypto.Hpke.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Crypto.Aes](Crypto.Aes.md), [Crypto.ChaCha20Poly1305](Crypto.ChaCha20Poly1305.md), [Crypto.Cipher](Crypto.Cipher.md), [Crypto.Hmac](Crypto.Hmac.md), [Crypto.Kdf](Crypto.Kdf.md), [Crypto.P256](Crypto.P256.md), [Crypto.X25519](Crypto.X25519.md), [Data.Bytes](Data.Bytes.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Text.Sha512](Text.Sha512.md).

<details class="doc-notes"><summary>Module notes</summary>

Hybrid public key encryption (RFC 9180): a key encapsulation, a key schedule and an AEAD combined into
contexts that seal or open a sequence of messages and export secrets.

Suites combine DHKEM(X25519, HKDF-SHA256) or DHKEM(P-256, HKDF-SHA256); HKDF-SHA256, HKDF-SHA384 or
HKDF-SHA512; and AES-128-GCM, AES-256-GCM, ChaCha20-Poly1305 or export-only. All four modes are
supported: base, PSK (a pre-shared key and its identifier), auth (the sender's static key) and auth-PSK.
Key pairs derive deterministically from input keying material, and the sender's ephemeral key is an
explicit argument (Effect.Crypto supplies fresh ones), so every operation here is pure and repeatable.

A context is an immutable value: sealing or opening returns the next context with the sequence number
advanced, so a retained context can never reuse a nonce by accident. Opening never releases plaintext
before the AEAD tag verifies.

</details>

<details class="doc-synopsis"><summary>Synopsis (32 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-HpkeAead">HpkeAead</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">HpkeAead</strong></code></li><li><a class="syn-name" href="#t-HpkeContext">HpkeContext</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">HpkeContext</strong></code></li><li><a class="syn-name" href="#t-HpkeError">HpkeError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">HpkeError</strong></code></li><li><a class="syn-name" href="#t-HpkeKdf">HpkeKdf</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">HpkeKdf</strong></code></li><li><a class="syn-name" href="#t-HpkeKem">HpkeKem</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">HpkeKem</strong></code></li><li><a class="syn-name" href="#t-HpkePrivateKey">HpkePrivateKey</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">HpkePrivateKey</strong></code></li><li><a class="syn-name" href="#t-HpkePublicKey">HpkePublicKey</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">HpkePublicKey</strong></code></li><li><a class="syn-name" href="#t-HpkeSuite">HpkeSuite</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">HpkeSuite</strong></code></li><li><a class="syn-name" href="#v-hpke-aead-id">hpke-aead-id</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">hpke-aead-id</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeAead" title="Crypto.Hpke.HpkeAead">HpkeAead</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-hpke-derive-key-pair">hpke-derive-key-pair</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">hpke-derive-key-pair</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeKem" title="Crypto.Hpke.HpkeKem">HpkeKem</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeError" title="Crypto.Hpke.HpkeError">HpkeError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkePrivateKey" title="Crypto.Hpke.HpkePrivateKey">HpkePrivateKey</a></code></li><li><a class="syn-name" href="#v-hpke-enc-size">hpke-enc-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">hpke-enc-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeKem" title="Crypto.Hpke.HpkeKem">HpkeKem</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-hpke-error-message">hpke-error-message</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">hpke-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeError" title="Crypto.Hpke.HpkeError">HpkeError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-hpke-export">hpke-export</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">hpke-export</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeContext" title="Crypto.Hpke.HpkeContext">HpkeContext</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeError" title="Crypto.Hpke.HpkeError">HpkeError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-hpke-kdf-id">hpke-kdf-id</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">hpke-kdf-id</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeKdf" title="Crypto.Hpke.HpkeKdf">HpkeKdf</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-hpke-kem-id">hpke-kem-id</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">hpke-kem-id</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeKem" title="Crypto.Hpke.HpkeKem">HpkeKem</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-hpke-key-kem">hpke-key-kem</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">hpke-key-kem</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkePrivateKey" title="Crypto.Hpke.HpkePrivateKey">HpkePrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeKem" title="Crypto.Hpke.HpkeKem">HpkeKem</a></code></li><li><a class="syn-name" href="#v-hpke-open">hpke-open</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">hpke-open</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeContext" title="Crypto.Hpke.HpkeContext">HpkeContext</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeError" title="Crypto.Hpke.HpkeError">HpkeError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeContext" title="Crypto.Hpke.HpkeContext">HpkeContext</a>)</code></li><li><a class="syn-name" href="#v-hpke-open-once">hpke-open-once</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">hpke-open-once</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeSuite" title="Crypto.Hpke.HpkeSuite">HpkeSuite</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkePrivateKey" title="Crypto.Hpke.HpkePrivateKey">HpkePrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkePublicKey" title="Crypto.Hpke.HpkePublicKey">HpkePublicKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeError" title="Crypto.Hpke.HpkeError">HpkeError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-hpke-private-key">hpke-private-key</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">hpke-private-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeKem" title="Crypto.Hpke.HpkeKem">HpkeKem</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeError" title="Crypto.Hpke.HpkeError">HpkeError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkePrivateKey" title="Crypto.Hpke.HpkePrivateKey">HpkePrivateKey</a></code></li><li><a class="syn-name" href="#v-hpke-private-key-bytes">hpke-private-key-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">hpke-private-key-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkePrivateKey" title="Crypto.Hpke.HpkePrivateKey">HpkePrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-hpke-public-key">hpke-public-key</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">hpke-public-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeKem" title="Crypto.Hpke.HpkeKem">HpkeKem</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeError" title="Crypto.Hpke.HpkeError">HpkeError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkePublicKey" title="Crypto.Hpke.HpkePublicKey">HpkePublicKey</a></code></li><li><a class="syn-name" href="#v-hpke-public-key-bytes">hpke-public-key-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">hpke-public-key-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkePublicKey" title="Crypto.Hpke.HpkePublicKey">HpkePublicKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-hpke-public-key-of">hpke-public-key-of</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">hpke-public-key-of</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkePrivateKey" title="Crypto.Hpke.HpkePrivateKey">HpkePrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkePublicKey" title="Crypto.Hpke.HpkePublicKey">HpkePublicKey</a></code></li><li><a class="syn-name" href="#v-hpke-seal">hpke-seal</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">hpke-seal</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeContext" title="Crypto.Hpke.HpkeContext">HpkeContext</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeError" title="Crypto.Hpke.HpkeError">HpkeError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeContext" title="Crypto.Hpke.HpkeContext">HpkeContext</a>)</code></li><li><a class="syn-name" href="#v-hpke-seal-once">hpke-seal-once</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">hpke-seal-once</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeSuite" title="Crypto.Hpke.HpkeSuite">HpkeSuite</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkePublicKey" title="Crypto.Hpke.HpkePublicKey">HpkePublicKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkePrivateKey" title="Crypto.Hpke.HpkePrivateKey">HpkePrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkePrivateKey" title="Crypto.Hpke.HpkePrivateKey">HpkePrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeError" title="Crypto.Hpke.HpkeError">HpkeError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>)</code></li><li><a class="syn-name" href="#v-hpke-sequence">hpke-sequence</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">hpke-sequence</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeContext" title="Crypto.Hpke.HpkeContext">HpkeContext</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-hpke-setup-recipient">hpke-setup-recipient</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">hpke-setup-recipient</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeSuite" title="Crypto.Hpke.HpkeSuite">HpkeSuite</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkePrivateKey" title="Crypto.Hpke.HpkePrivateKey">HpkePrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkePublicKey" title="Crypto.Hpke.HpkePublicKey">HpkePublicKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeError" title="Crypto.Hpke.HpkeError">HpkeError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeContext" title="Crypto.Hpke.HpkeContext">HpkeContext</a></code></li><li><a class="syn-name" href="#v-hpke-setup-sender">hpke-setup-sender</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">hpke-setup-sender</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeSuite" title="Crypto.Hpke.HpkeSuite">HpkeSuite</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkePublicKey" title="Crypto.Hpke.HpkePublicKey">HpkePublicKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkePrivateKey" title="Crypto.Hpke.HpkePrivateKey">HpkePrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkePrivateKey" title="Crypto.Hpke.HpkePrivateKey">HpkePrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeError" title="Crypto.Hpke.HpkeError">HpkeError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeContext" title="Crypto.Hpke.HpkeContext">HpkeContext</a>)</code></li><li><a class="syn-name" href="#v-hpke-suite">hpke-suite</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">hpke-suite</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeKem" title="Crypto.Hpke.HpkeKem">HpkeKem</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeKdf" title="Crypto.Hpke.HpkeKdf">HpkeKdf</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeAead" title="Crypto.Hpke.HpkeAead">HpkeAead</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeSuite" title="Crypto.Hpke.HpkeSuite">HpkeSuite</a></code></li><li><a class="syn-name" href="#v-hpke-suite-aead">hpke-suite-aead</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">hpke-suite-aead</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeSuite" title="Crypto.Hpke.HpkeSuite">HpkeSuite</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeAead" title="Crypto.Hpke.HpkeAead">HpkeAead</a></code></li><li><a class="syn-name" href="#v-hpke-suite-kdf">hpke-suite-kdf</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">hpke-suite-kdf</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeSuite" title="Crypto.Hpke.HpkeSuite">HpkeSuite</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeKdf" title="Crypto.Hpke.HpkeKdf">HpkeKdf</a></code></li><li><a class="syn-name" href="#v-hpke-suite-kem">hpke-suite-kem</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">hpke-suite-kem</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeSuite" title="Crypto.Hpke.HpkeSuite">HpkeSuite</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeKem" title="Crypto.Hpke.HpkeKem">HpkeKem</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-HpkeAead">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">HpkeAead</strong></code><span class="doc-sig-links"><a href="#t-HpkeAead" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Hpke/source/#L62" title="View source at line 62">Source</a></span></div>
<div class="doc-body">

An authenticated cipher.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-HpkeAes128Gcm"><code><strong class="tok-constructor">HpkeAes128Gcm</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeAead" title="Crypto.Hpke.HpkeAead">HpkeAead</a></code></li><li id="k-HpkeAes256Gcm"><code><strong class="tok-constructor">HpkeAes256Gcm</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeAead" title="Crypto.Hpke.HpkeAead">HpkeAead</a></code></li><li id="k-HpkeChaCha20Poly1305"><code><strong class="tok-constructor">HpkeChaCha20Poly1305</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeAead" title="Crypto.Hpke.HpkeAead">HpkeAead</a></code></li><li id="k-HpkeExportOnly"><code><strong class="tok-constructor">HpkeExportOnly</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeAead" title="Crypto.Hpke.HpkeAead">HpkeAead</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-HpkeContext">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">HpkeContext</strong></code><span class="doc-sig-links"><a href="#t-HpkeContext" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Hpke/source/#L140" title="View source at line 140">Source</a></span></div>
<div class="doc-body">

Fields in the order listed above.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-HpkeContext"><code><strong class="tok-constructor">HpkeContext</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeSuite" title="Crypto.Hpke.HpkeSuite">HpkeSuite</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeContext" title="Crypto.Hpke.HpkeContext">HpkeContext</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-HpkeError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">HpkeError</strong></code><span class="doc-sig-links"><a href="#t-HpkeError" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Hpke/source/#L94" title="View source at line 94">Source</a></span></div>
<div class="doc-body">

Why an HPKE operation failed.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-HpkeInvalidKey"><code><strong class="tok-constructor">HpkeInvalidKey</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeError" title="Crypto.Hpke.HpkeError">HpkeError</a></code></li><li id="k-HpkeInvalidEncapsulation"><code><strong class="tok-constructor">HpkeInvalidEncapsulation</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeError" title="Crypto.Hpke.HpkeError">HpkeError</a></code></li><li id="k-HpkeDerivationFailed"><code><strong class="tok-constructor">HpkeDerivationFailed</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeError" title="Crypto.Hpke.HpkeError">HpkeError</a></code></li><li id="k-HpkeInconsistentPsk"><code><strong class="tok-constructor">HpkeInconsistentPsk</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeError" title="Crypto.Hpke.HpkeError">HpkeError</a></code></li><li id="k-HpkeExportOnlyAead"><code><strong class="tok-constructor">HpkeExportOnlyAead</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeError" title="Crypto.Hpke.HpkeError">HpkeError</a></code></li><li id="k-HpkeOpenFailed"><code><strong class="tok-constructor">HpkeOpenFailed</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeError" title="Crypto.Hpke.HpkeError">HpkeError</a></code></li><li id="k-HpkeMessageLimit"><code><strong class="tok-constructor">HpkeMessageLimit</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeError" title="Crypto.Hpke.HpkeError">HpkeError</a></code></li><li id="k-HpkeExportTooLong"><code><strong class="tok-constructor">HpkeExportTooLong</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeError" title="Crypto.Hpke.HpkeError">HpkeError</a></code></li><li id="k-HpkeKeyMismatch"><code><strong class="tok-constructor">HpkeKeyMismatch</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeError" title="Crypto.Hpke.HpkeError">HpkeError</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-HpkeKdf">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">HpkeKdf</strong></code><span class="doc-sig-links"><a href="#t-HpkeKdf" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Hpke/source/#L53" title="View source at line 53">Source</a></span></div>
<div class="doc-body">

A key derivation function.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-HpkeHkdfSha256"><code><strong class="tok-constructor">HpkeHkdfSha256</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeKdf" title="Crypto.Hpke.HpkeKdf">HpkeKdf</a></code></li><li id="k-HpkeHkdfSha384"><code><strong class="tok-constructor">HpkeHkdfSha384</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeKdf" title="Crypto.Hpke.HpkeKdf">HpkeKdf</a></code></li><li id="k-HpkeHkdfSha512"><code><strong class="tok-constructor">HpkeHkdfSha512</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeKdf" title="Crypto.Hpke.HpkeKdf">HpkeKdf</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-HpkeKem">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">HpkeKem</strong></code><span class="doc-sig-links"><a href="#t-HpkeKem" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Hpke/source/#L46" title="View source at line 46">Source</a></span></div>
<div class="doc-body">

A key encapsulation mechanism.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-HpkeDhkemX25519"><code><strong class="tok-constructor">HpkeDhkemX25519</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeKem" title="Crypto.Hpke.HpkeKem">HpkeKem</a></code></li><li id="k-HpkeDhkemP256"><code><strong class="tok-constructor">HpkeDhkemP256</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeKem" title="Crypto.Hpke.HpkeKem">HpkeKem</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-HpkePrivateKey">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">HpkePrivateKey</strong></code><span class="doc-sig-links"><a href="#t-HpkePrivateKey" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Hpke/source/#L130" title="View source at line 130">Source</a></span></div>
<div class="doc-body">

KEM and serialized key.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-HpkePrivateKey"><code><strong class="tok-constructor">HpkePrivateKey</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeKem" title="Crypto.Hpke.HpkeKem">HpkeKem</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkePrivateKey" title="Crypto.Hpke.HpkePrivateKey">HpkePrivateKey</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-HpkePublicKey">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">HpkePublicKey</strong></code><span class="doc-sig-links"><a href="#t-HpkePublicKey" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Hpke/source/#L135" title="View source at line 135">Source</a></span></div>
<div class="doc-body">

KEM and serialized key.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-HpkePublicKey"><code><strong class="tok-constructor">HpkePublicKey</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeKem" title="Crypto.Hpke.HpkeKem">HpkeKem</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkePublicKey" title="Crypto.Hpke.HpkePublicKey">HpkePublicKey</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-HpkeSuite">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">HpkeSuite</strong></code><span class="doc-sig-links"><a href="#t-HpkeSuite" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Hpke/source/#L75" title="View source at line 75">Source</a></span></div>
<div class="doc-body">

KEM, KDF and AEAD.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-HpkeSuite"><code><strong class="tok-constructor">HpkeSuite</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeKem" title="Crypto.Hpke.HpkeKem">HpkeKem</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeKdf" title="Crypto.Hpke.HpkeKdf">HpkeKdf</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeAead" title="Crypto.Hpke.HpkeAead">HpkeAead</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeSuite" title="Crypto.Hpke.HpkeSuite">HpkeSuite</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-hpke-aead-id">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">hpke-aead-id</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeAead" title="Crypto.Hpke.HpkeAead">HpkeAead</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-hpke-aead-id" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Hpke/source/#L160" title="View source at line 160">Source</a></span></div>
<div class="doc-body">

The AEAD identifier.

</div>
</article>

<article class="doc-entry" id="v-hpke-derive-key-pair">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">hpke-derive-key-pair</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeKem" title="Crypto.Hpke.HpkeKem">HpkeKem</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeError" title="Crypto.Hpke.HpkeError">HpkeError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkePrivateKey" title="Crypto.Hpke.HpkePrivateKey">HpkePrivateKey</a></code><span class="doc-sig-links"><a href="#v-hpke-derive-key-pair" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Hpke/source/#L294" title="View source at line 294">Source</a></span></div>
<div class="doc-body">

Derive a key pair from input keying material (DeriveKeyPair): X25519 expands the private key directly;
P-256 tries up to 256 candidates until one lies in [1, n).

</div>
</article>

<article class="doc-entry" id="v-hpke-enc-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">hpke-enc-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeKem" title="Crypto.Hpke.HpkeKem">HpkeKem</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-hpke-enc-size" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Hpke/source/#L168" title="View source at line 168">Source</a></span></div>
<div class="doc-body">

The encapsulated (and serialized public) key size.

</div>
</article>

<article class="doc-entry" id="v-hpke-error-message">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">hpke-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeError" title="Crypto.Hpke.HpkeError">HpkeError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-hpke-error-message" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Hpke/source/#L115" title="View source at line 115">Source</a></span></div>
<div class="doc-body">

A readable description of an error.

</div>
</article>

<article class="doc-entry" id="v-hpke-export">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">hpke-export</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeContext" title="Crypto.Hpke.HpkeContext">HpkeContext</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeError" title="Crypto.Hpke.HpkeError">HpkeError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-hpke-export" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Hpke/source/#L583" title="View source at line 583">Source</a></span></div>
<div class="doc-body">

Export a secret of a length for an exporter context (at most 255 hash outputs).

</div>
</article>

<article class="doc-entry" id="v-hpke-kdf-id">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">hpke-kdf-id</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeKdf" title="Crypto.Hpke.HpkeKdf">HpkeKdf</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-hpke-kdf-id" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Hpke/source/#L153" title="View source at line 153">Source</a></span></div>
<div class="doc-body">

The KDF identifier.

</div>
</article>

<article class="doc-entry" id="v-hpke-kem-id">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">hpke-kem-id</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeKem" title="Crypto.Hpke.HpkeKem">HpkeKem</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-hpke-kem-id" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Hpke/source/#L147" title="View source at line 147">Source</a></span></div>
<div class="doc-body">

The KEM identifier.

</div>
</article>

<article class="doc-entry" id="v-hpke-key-kem">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">hpke-key-kem</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkePrivateKey" title="Crypto.Hpke.HpkePrivateKey">HpkePrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeKem" title="Crypto.Hpke.HpkeKem">HpkeKem</a></code><span class="doc-sig-links"><a href="#v-hpke-key-kem" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Hpke/source/#L357" title="View source at line 357">Source</a></span></div>
<div class="doc-body">

The KEM a private key belongs to.

</div>
</article>

<article class="doc-entry" id="v-hpke-open">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">hpke-open</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeContext" title="Crypto.Hpke.HpkeContext">HpkeContext</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeError" title="Crypto.Hpke.HpkeError">HpkeError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeContext" title="Crypto.Hpke.HpkeContext">HpkeContext</a>)</code><span class="doc-sig-links"><a href="#v-hpke-open" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Hpke/source/#L546" title="View source at line 546">Source</a></span></div>
<div class="doc-body">

Open the next message: the plaintext and the context for the message after it.

</div>
</article>

<article class="doc-entry" id="v-hpke-open-once">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">hpke-open-once</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeSuite" title="Crypto.Hpke.HpkeSuite">HpkeSuite</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkePrivateKey" title="Crypto.Hpke.HpkePrivateKey">HpkePrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkePublicKey" title="Crypto.Hpke.HpkePublicKey">HpkePublicKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeError" title="Crypto.Hpke.HpkeError">HpkeError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-hpke-open-once" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Hpke/source/#L600" title="View source at line 600">Source</a></span></div>
<div class="doc-body">

Single-shot opening: set up a recipient context and open one message.

</div>
</article>

<article class="doc-entry" id="v-hpke-private-key">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">hpke-private-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeKem" title="Crypto.Hpke.HpkeKem">HpkeKem</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeError" title="Crypto.Hpke.HpkeError">HpkeError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkePrivateKey" title="Crypto.Hpke.HpkePrivateKey">HpkePrivateKey</a></code><span class="doc-sig-links"><a href="#v-hpke-private-key" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Hpke/source/#L316" title="View source at line 316">Source</a></span></div>
<div class="doc-body">

A private key from its serialized form.

</div>
</article>

<article class="doc-entry" id="v-hpke-private-key-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">hpke-private-key-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkePrivateKey" title="Crypto.Hpke.HpkePrivateKey">HpkePrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-hpke-private-key-bytes" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Hpke/source/#L349" title="View source at line 349">Source</a></span></div>
<div class="doc-body">

The serialized private key.

</div>
</article>

<article class="doc-entry" id="v-hpke-public-key">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">hpke-public-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeKem" title="Crypto.Hpke.HpkeKem">HpkeKem</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeError" title="Crypto.Hpke.HpkeError">HpkeError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkePublicKey" title="Crypto.Hpke.HpkePublicKey">HpkePublicKey</a></code><span class="doc-sig-links"><a href="#v-hpke-public-key" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Hpke/source/#L326" title="View source at line 326">Source</a></span></div>
<div class="doc-body">

A public key from its serialized form (P-256 keys must be uncompressed).

</div>
</article>

<article class="doc-entry" id="v-hpke-public-key-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">hpke-public-key-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkePublicKey" title="Crypto.Hpke.HpkePublicKey">HpkePublicKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-hpke-public-key-bytes" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Hpke/source/#L353" title="View source at line 353">Source</a></span></div>
<div class="doc-body">

The serialized public key.

</div>
</article>

<article class="doc-entry" id="v-hpke-public-key-of">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">hpke-public-key-of</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkePrivateKey" title="Crypto.Hpke.HpkePrivateKey">HpkePrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkePublicKey" title="Crypto.Hpke.HpkePublicKey">HpkePublicKey</a></code><span class="doc-sig-links"><a href="#v-hpke-public-key-of" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Hpke/source/#L334" title="View source at line 334">Source</a></span></div>
<div class="doc-body">

The public key of a private key.

</div>
</article>

<article class="doc-entry" id="v-hpke-seal">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">hpke-seal</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeContext" title="Crypto.Hpke.HpkeContext">HpkeContext</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeError" title="Crypto.Hpke.HpkeError">HpkeError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeContext" title="Crypto.Hpke.HpkeContext">HpkeContext</a>)</code><span class="doc-sig-links"><a href="#v-hpke-seal" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Hpke/source/#L535" title="View source at line 535">Source</a></span></div>
<div class="doc-body">

Seal the next message: the ciphertext (with tag) and the context for the message after it.

</div>
</article>

<article class="doc-entry" id="v-hpke-seal-once">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">hpke-seal-once</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeSuite" title="Crypto.Hpke.HpkeSuite">HpkeSuite</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkePublicKey" title="Crypto.Hpke.HpkePublicKey">HpkePublicKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkePrivateKey" title="Crypto.Hpke.HpkePrivateKey">HpkePrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkePrivateKey" title="Crypto.Hpke.HpkePrivateKey">HpkePrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeError" title="Crypto.Hpke.HpkeError">HpkeError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>)</code><span class="doc-sig-links"><a href="#v-hpke-seal-once" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Hpke/source/#L590" title="View source at line 590">Source</a></span></div>
<div class="doc-body">

Single-shot sealing: set up a sender context and seal one message; the encapsulated key and ciphertext.

</div>
</article>

<article class="doc-entry" id="v-hpke-sequence">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">hpke-sequence</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeContext" title="Crypto.Hpke.HpkeContext">HpkeContext</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-hpke-sequence" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Hpke/source/#L502" title="View source at line 502">Source</a></span></div>
<div class="doc-body">

The sequence number of the next message.

</div>
</article>

<article class="doc-entry" id="v-hpke-setup-recipient">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">hpke-setup-recipient</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeSuite" title="Crypto.Hpke.HpkeSuite">HpkeSuite</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkePrivateKey" title="Crypto.Hpke.HpkePrivateKey">HpkePrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkePublicKey" title="Crypto.Hpke.HpkePublicKey">HpkePublicKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeError" title="Crypto.Hpke.HpkeError">HpkeError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeContext" title="Crypto.Hpke.HpkeContext">HpkeContext</a></code><span class="doc-sig-links"><a href="#v-hpke-setup-recipient" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Hpke/source/#L465" title="View source at line 465">Source</a></span></div>
<div class="doc-body">

Set up a recipient context from the encapsulated key; a sender public key selects an authenticated mode and
a (key, identifier) pair a PSK mode.

</div>
</article>

<article class="doc-entry" id="v-hpke-setup-sender">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">hpke-setup-sender</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeSuite" title="Crypto.Hpke.HpkeSuite">HpkeSuite</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkePublicKey" title="Crypto.Hpke.HpkePublicKey">HpkePublicKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkePrivateKey" title="Crypto.Hpke.HpkePrivateKey">HpkePrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkePrivateKey" title="Crypto.Hpke.HpkePrivateKey">HpkePrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeError" title="Crypto.Hpke.HpkeError">HpkeError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeContext" title="Crypto.Hpke.HpkeContext">HpkeContext</a>)</code><span class="doc-sig-links"><a href="#v-hpke-setup-sender" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Hpke/source/#L449" title="View source at line 449">Source</a></span></div>
<div class="doc-body">

Set up a sender context to a recipient's public key with an ephemeral private key; a sender key selects an
authenticated mode and a (key, identifier) pair a PSK mode. Returns the encapsulated key and the context.

</div>
</article>

<article class="doc-entry" id="v-hpke-suite">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">hpke-suite</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeKem" title="Crypto.Hpke.HpkeKem">HpkeKem</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeKdf" title="Crypto.Hpke.HpkeKdf">HpkeKdf</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeAead" title="Crypto.Hpke.HpkeAead">HpkeAead</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeSuite" title="Crypto.Hpke.HpkeSuite">HpkeSuite</a></code><span class="doc-sig-links"><a href="#v-hpke-suite" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Hpke/source/#L78" title="View source at line 78">Source</a></span></div>
<div class="doc-body">

Combine algorithms into a suite.

</div>
</article>

<article class="doc-entry" id="v-hpke-suite-aead">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">hpke-suite-aead</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeSuite" title="Crypto.Hpke.HpkeSuite">HpkeSuite</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeAead" title="Crypto.Hpke.HpkeAead">HpkeAead</a></code><span class="doc-sig-links"><a href="#v-hpke-suite-aead" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Hpke/source/#L90" title="View source at line 90">Source</a></span></div>
<div class="doc-body">

The suite's AEAD.

</div>
</article>

<article class="doc-entry" id="v-hpke-suite-kdf">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">hpke-suite-kdf</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeSuite" title="Crypto.Hpke.HpkeSuite">HpkeSuite</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeKdf" title="Crypto.Hpke.HpkeKdf">HpkeKdf</a></code><span class="doc-sig-links"><a href="#v-hpke-suite-kdf" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Hpke/source/#L86" title="View source at line 86">Source</a></span></div>
<div class="doc-body">

The suite's KDF.

</div>
</article>

<article class="doc-entry" id="v-hpke-suite-kem">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">hpke-suite-kem</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeSuite" title="Crypto.Hpke.HpkeSuite">HpkeSuite</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Hpke/#t-HpkeKem" title="Crypto.Hpke.HpkeKem">HpkeKem</a></code><span class="doc-sig-links"><a href="#v-hpke-suite-kem" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Hpke/source/#L82" title="View source at line 82">Source</a></span></div>
<div class="doc-body">

The suite's KEM.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Crypto/Hpke.cinch`](/packages/stdlib/Crypto.Hpke/source/). Source SHA-256: `02fa628f945bc098ffe21f179980f676c0862c2c3a6f1f8e3655bdd89d116abd`.
