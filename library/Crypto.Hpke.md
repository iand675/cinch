# Crypto.Hpke

Hybrid public key encryption (RFC 9180) over DHKEM X25519 and P-256, HKDF and AES-GCM or ChaCha20-Poly1305, in all four modes.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Crypto.Hpke`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Crypto.Hpke.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Crypto.Aes](Crypto.Aes.md), [Crypto.ChaCha20Poly1305](Crypto.ChaCha20Poly1305.md), [Crypto.Cipher](Crypto.Cipher.md), [Crypto.Hmac](Crypto.Hmac.md), [Crypto.Kdf](Crypto.Kdf.md), [Crypto.P256](Crypto.P256.md), [Crypto.X25519](Crypto.X25519.md), [Data.Bytes](Data.Bytes.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Text.Sha512](Text.Sha512.md).

Non-embedded source closure: [Crypto.Aes](Crypto.Aes.md), [Crypto.ChaCha20Poly1305](Crypto.ChaCha20Poly1305.md), [Crypto.Cipher](Crypto.Cipher.md), [Crypto.ConstantTime](Crypto.ConstantTime.md), [Crypto.Field25519](Crypto.Field25519.md), [Crypto.Hmac](Crypto.Hmac.md), [Crypto.Hpke](Crypto.Hpke.md), [Crypto.Kdf](Crypto.Kdf.md), [Crypto.P256](Crypto.P256.md), [Crypto.X25519](Crypto.X25519.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Text.Binary](Text.Binary.md), [Text.Digest](Text.Digest.md), [Text.Sha512](Text.Sha512.md).

<details><summary>Behaviour and boundary notes from the source</summary>

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

# Types and constructors

<a id="type-48706b6541656164"></a>

## `HpkeAead`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">HpkeAead</span></code></pre>

An authenticated cipher.

[Source](/reference-sources/Crypto.Hpke.cinch) · line 62

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">HpkeAes128Gcm</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkeAead</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">HpkeAes256Gcm</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkeAead</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">HpkeChaCha20Poly1305</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkeAead</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">HpkeExportOnly</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkeAead</span></code></pre>

<a id="type-48706b65436f6e74657874"></a>

## `HpkeContext`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">HpkeContext</span></code></pre>

Fields in the order listed above.

[Source](/reference-sources/Crypto.Hpke.cinch) · line 140

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">HpkeContext</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkeSuite</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkeContext</span>)))))</code></pre>

<a id="type-48706b654572726f72"></a>

## `HpkeError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">HpkeError</span></code></pre>

Why an HPKE operation failed.

[Source](/reference-sources/Crypto.Hpke.cinch) · line 94

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">HpkeInvalidKey</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkeError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">HpkeInvalidEncapsulation</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkeError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">HpkeDerivationFailed</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkeError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">HpkeInconsistentPsk</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkeError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">HpkeExportOnlyAead</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkeError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">HpkeOpenFailed</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkeError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">HpkeMessageLimit</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkeError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">HpkeExportTooLong</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkeError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">HpkeKeyMismatch</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkeError</span></code></pre>

<a id="type-48706b654b6466"></a>

## `HpkeKdf`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">HpkeKdf</span></code></pre>

A key derivation function.

[Source](/reference-sources/Crypto.Hpke.cinch) · line 53

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">HpkeHkdfSha256</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkeKdf</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">HpkeHkdfSha384</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkeKdf</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">HpkeHkdfSha512</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkeKdf</span></code></pre>

<a id="type-48706b654b656d"></a>

## `HpkeKem`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">HpkeKem</span></code></pre>

A key encapsulation mechanism.

[Source](/reference-sources/Crypto.Hpke.cinch) · line 46

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">HpkeDhkemX25519</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkeKem</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">HpkeDhkemP256</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkeKem</span></code></pre>

<a id="type-48706b65507269766174654b6579"></a>

## `HpkePrivateKey`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">HpkePrivateKey</span></code></pre>

KEM and serialized key.

[Source](/reference-sources/Crypto.Hpke.cinch) · line 130

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">HpkePrivateKey</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkeKem</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkePrivateKey</span>))</code></pre>

<a id="type-48706b655075626c69634b6579"></a>

## `HpkePublicKey`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">HpkePublicKey</span></code></pre>

KEM and serialized key.

[Source](/reference-sources/Crypto.Hpke.cinch) · line 135

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">HpkePublicKey</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkeKem</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkePublicKey</span>))</code></pre>

<a id="type-48706b655375697465"></a>

## `HpkeSuite`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">HpkeSuite</span></code></pre>

KEM, KDF and AEAD.

[Source](/reference-sources/Crypto.Hpke.cinch) · line 75

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">HpkeSuite</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkeKem</span> -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkeKdf</span> -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkeAead</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkeSuite</span>)))</code></pre>

# Values

<a id="value-68706b652d616561642d6964"></a>

## `hpke-aead-id`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hpke-aead-id</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkeAead</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

The AEAD identifier.

[Source](/reference-sources/Crypto.Hpke.cinch) · line 160

<a id="value-68706b652d6465726976652d6b65792d70616972"></a>

## `hpke-derive-key-pair`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hpke-derive-key-pair</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkeKem</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkeError</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkePrivateKey</span>)))</code></pre>

Derive a key pair from input keying material (DeriveKeyPair): X25519 expands the private key directly;
P-256 tries up to 256 candidates until one lies in [1, n).

[Source](/reference-sources/Crypto.Hpke.cinch) · line 294

<a id="value-68706b652d656e632d73697a65"></a>

## `hpke-enc-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hpke-enc-size</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkeKem</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

The encapsulated (and serialized public) key size.

[Source](/reference-sources/Crypto.Hpke.cinch) · line 168

<a id="value-68706b652d6572726f722d6d657373616765"></a>

## `hpke-error-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hpke-error-message</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkeError</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

A readable description of an error.

[Source](/reference-sources/Crypto.Hpke.cinch) · line 115

<a id="value-68706b652d6578706f7274"></a>

## `hpke-export`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hpke-export</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkeContext</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkeError</span> <span class="tok-constructor">Bytes</span>))))</code></pre>

Export a secret of a length for an exporter context (at most 255 hash outputs).

[Source](/reference-sources/Crypto.Hpke.cinch) · line 583

<a id="value-68706b652d6b64662d6964"></a>

## `hpke-kdf-id`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hpke-kdf-id</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkeKdf</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

The KDF identifier.

[Source](/reference-sources/Crypto.Hpke.cinch) · line 153

<a id="value-68706b652d6b656d2d6964"></a>

## `hpke-kem-id`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hpke-kem-id</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkeKem</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

The KEM identifier.

[Source](/reference-sources/Crypto.Hpke.cinch) · line 147

<a id="value-68706b652d6b65792d6b656d"></a>

## `hpke-key-kem`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hpke-key-kem</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkePrivateKey</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkeKem</span>)</code></pre>

The KEM a private key belongs to.

[Source](/reference-sources/Crypto.Hpke.cinch) · line 357

<a id="value-68706b652d6f70656e"></a>

## `hpke-open`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hpke-open</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkeContext</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkeError</span> (<span class="tok-constructor">Bytes</span>, <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkeContext</span>)))))</code></pre>

Open the next message: the plaintext and the context for the message after it.

[Source](/reference-sources/Crypto.Hpke.cinch) · line 546

<a id="value-68706b652d6f70656e2d6f6e6365"></a>

## `hpke-open-once`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hpke-open-once</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkeSuite</span> -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkePrivateKey</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkePublicKey</span>) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Bytes</span>, <span class="tok-constructor">Bytes</span>)) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkeError</span> <span class="tok-constructor">Bytes</span>)))))))))</code></pre>

Single-shot opening: set up a recipient context and open one message.

[Source](/reference-sources/Crypto.Hpke.cinch) · line 600

<a id="value-68706b652d707269766174652d6b6579"></a>

## `hpke-private-key`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hpke-private-key</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkeKem</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkeError</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkePrivateKey</span>)))</code></pre>

A private key from its serialized form.

[Source](/reference-sources/Crypto.Hpke.cinch) · line 316

<a id="value-68706b652d707269766174652d6b65792d6279746573"></a>

## `hpke-private-key-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hpke-private-key-bytes</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkePrivateKey</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

The serialized private key.

[Source](/reference-sources/Crypto.Hpke.cinch) · line 349

<a id="value-68706b652d7075626c69632d6b6579"></a>

## `hpke-public-key`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hpke-public-key</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkeKem</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkeError</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkePublicKey</span>)))</code></pre>

A public key from its serialized form (P-256 keys must be uncompressed).

[Source](/reference-sources/Crypto.Hpke.cinch) · line 326

<a id="value-68706b652d7075626c69632d6b65792d6279746573"></a>

## `hpke-public-key-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hpke-public-key-bytes</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkePublicKey</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

The serialized public key.

[Source](/reference-sources/Crypto.Hpke.cinch) · line 353

<a id="value-68706b652d7075626c69632d6b65792d6f66"></a>

## `hpke-public-key-of`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hpke-public-key-of</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkePrivateKey</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkePublicKey</span>)</code></pre>

The public key of a private key.

[Source](/reference-sources/Crypto.Hpke.cinch) · line 334

<a id="value-68706b652d7365616c"></a>

## `hpke-seal`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hpke-seal</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkeContext</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkeError</span> (<span class="tok-constructor">Bytes</span>, <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkeContext</span>)))))</code></pre>

Seal the next message: the ciphertext (with tag) and the context for the message after it.

[Source](/reference-sources/Crypto.Hpke.cinch) · line 535

<a id="value-68706b652d7365616c2d6f6e6365"></a>

## `hpke-seal-once`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hpke-seal-once</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkeSuite</span> -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkePublicKey</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkePrivateKey</span>) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Bytes</span>, <span class="tok-constructor">Bytes</span>)) -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkePrivateKey</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkeError</span> (<span class="tok-constructor">Bytes</span>, <span class="tok-constructor">Bytes</span>))))))))))</code></pre>

Single-shot sealing: set up a sender context and seal one message; the encapsulated key and ciphertext.

[Source](/reference-sources/Crypto.Hpke.cinch) · line 590

<a id="value-68706b652d73657175656e6365"></a>

## `hpke-sequence`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hpke-sequence</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkeContext</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

The sequence number of the next message.

[Source](/reference-sources/Crypto.Hpke.cinch) · line 502

<a id="value-68706b652d73657475702d726563697069656e74"></a>

## `hpke-setup-recipient`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hpke-setup-recipient</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkeSuite</span> -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkePrivateKey</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkePublicKey</span>) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Bytes</span>, <span class="tok-constructor">Bytes</span>)) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkeError</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkeContext</span>)))))))</code></pre>

Set up a recipient context from the encapsulated key; a sender public key selects an authenticated mode and
a (key, identifier) pair a PSK mode.

[Source](/reference-sources/Crypto.Hpke.cinch) · line 465

<a id="value-68706b652d73657475702d73656e646572"></a>

## `hpke-setup-sender`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hpke-setup-sender</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkeSuite</span> -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkePublicKey</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkePrivateKey</span>) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Bytes</span>, <span class="tok-constructor">Bytes</span>)) -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkePrivateKey</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkeError</span> (<span class="tok-constructor">Bytes</span>, <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkeContext</span>))))))))</code></pre>

Set up a sender context to a recipient's public key with an ephemeral private key; a sender key selects an
authenticated mode and a (key, identifier) pair a PSK mode. Returns the encapsulated key and the context.

[Source](/reference-sources/Crypto.Hpke.cinch) · line 449

<a id="value-68706b652d7375697465"></a>

## `hpke-suite`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hpke-suite</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkeKem</span> -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkeKdf</span> -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkeAead</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkeSuite</span>)))</code></pre>

Combine algorithms into a suite.

[Source](/reference-sources/Crypto.Hpke.cinch) · line 78

<a id="value-68706b652d73756974652d61656164"></a>

## `hpke-suite-aead`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hpke-suite-aead</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkeSuite</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkeAead</span>)</code></pre>

The suite's AEAD.

[Source](/reference-sources/Crypto.Hpke.cinch) · line 90

<a id="value-68706b652d73756974652d6b6466"></a>

## `hpke-suite-kdf`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hpke-suite-kdf</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkeSuite</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkeKdf</span>)</code></pre>

The suite's KDF.

[Source](/reference-sources/Crypto.Hpke.cinch) · line 86

<a id="value-68706b652d73756974652d6b656d"></a>

## `hpke-suite-kem`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hpke-suite-kem</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkeSuite</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkeKem</span>)</code></pre>

The suite's KEM.

[Source](/reference-sources/Crypto.Hpke.cinch) · line 82

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Crypto/Hpke.cinch`. Source SHA-256: `02fa628f945bc098ffe21f179980f676c0862c2c3a6f1f8e3655bdd89d116abd`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
