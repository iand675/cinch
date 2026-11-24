# Crypto.Ecdsa

Deterministic (RFC 6979) and hedged P-256 ECDSA signatures in raw and DER form with strict verification.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Crypto.Ecdsa`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Crypto.Ecdsa.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Crypto.Hmac](Crypto.Hmac.md), [Crypto.P256](Crypto.P256.md), [Data.Bytes](Data.Bytes.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Data.List](Data.List.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md).

Non-embedded source closure: [Crypto.ConstantTime](Crypto.ConstantTime.md), [Crypto.Ecdsa](Crypto.Ecdsa.md), [Crypto.Hmac](Crypto.Hmac.md), [Crypto.P256](Crypto.P256.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Text.Digest](Text.Digest.md).

<details><summary>Behaviour and boundary notes from the source</summary>

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

# Types and constructors

<a id="type-45636473614572726f72"></a>

## `EcdsaError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">EcdsaError</span></code></pre>

A signature that cannot be produced or parsed.

[Source](/reference-sources/Crypto.Ecdsa.cinch) · line 25

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">EcdsaInvalidSignature</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Ecdsa</span>.<span class="tok-constructor">EcdsaError</span></code></pre>

# Values

<a id="value-65636473612d6572726f722d6d657373616765"></a>

## `ecdsa-error-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ecdsa-error-message</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Ecdsa</span>.<span class="tok-constructor">EcdsaError</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Render an error.

[Source](/reference-sources/Crypto.Ecdsa.cinch) · line 30

<a id="value-65636473612d726663363937392d6e6f6e6365"></a>

## `ecdsa-rfc6979-nonce`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ecdsa-rfc6979-nonce</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">P256</span>.<span class="tok-constructor">P256PrivateKey</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">P256</span>.<span class="tok-constructor">P256Scalar</span>)))</code></pre>

The RFC 6979 nonce for a key and digest with HMAC-SHA-256, with optional extra entropy appended to the seed.

[Source](/reference-sources/Crypto.Ecdsa.cinch) · line 78

<a id="value-65636473612d7369676e"></a>

## `ecdsa-sign`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ecdsa-sign</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">P256</span>.<span class="tok-constructor">P256PrivateKey</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Ecdsa</span>.<span class="tok-constructor">EcdsaError</span> <span class="tok-constructor">Bytes</span>)))</code></pre>

Sign a digest with a deterministic RFC 6979 nonce: 64 bytes r || s.

[Source](/reference-sources/Crypto.Ecdsa.cinch) · line 34

<a id="value-65636473612d7369676e2d646572"></a>

## `ecdsa-sign-der`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ecdsa-sign-der</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">P256</span>.<span class="tok-constructor">P256PrivateKey</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Ecdsa</span>.<span class="tok-constructor">EcdsaError</span> <span class="tok-constructor">Bytes</span>)))</code></pre>

Sign and DER-encode: SEQUENCE { INTEGER r, INTEGER s }.

[Source](/reference-sources/Crypto.Ecdsa.cinch) · line 66

<a id="value-65636473612d7369676e2d6465722d686564676564"></a>

## `ecdsa-sign-der-hedged`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ecdsa-sign-der-hedged</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">P256</span>.<span class="tok-constructor">P256PrivateKey</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Ecdsa</span>.<span class="tok-constructor">EcdsaError</span> <span class="tok-constructor">Bytes</span>))))</code></pre>

Hedged signing, DER-encoded.

[Source](/reference-sources/Crypto.Ecdsa.cinch) · line 70

<a id="value-65636473612d7369676e2d686564676564"></a>

## `ecdsa-sign-hedged`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ecdsa-sign-hedged</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">P256</span>.<span class="tok-constructor">P256PrivateKey</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Ecdsa</span>.<span class="tok-constructor">EcdsaError</span> <span class="tok-constructor">Bytes</span>))))</code></pre>

Sign with the RFC 6979 nonce hedged by extra entropy (section 3.6); empty entropy gives ecdsa-sign.

[Source](/reference-sources/Crypto.Ecdsa.cinch) · line 38

<a id="value-65636473612d766572696679"></a>

## `ecdsa-verify`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ecdsa-verify</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">P256</span>.<span class="tok-constructor">P256PublicKey</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Bool</span>)))</code></pre>

Verify a 64-byte r || s signature over a digest.

[Source](/reference-sources/Crypto.Ecdsa.cinch) · line 51

<a id="value-65636473612d7665726966792d646572"></a>

## `ecdsa-verify-der`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ecdsa-verify-der</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">P256</span>.<span class="tok-constructor">P256PublicKey</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Bool</span>)))</code></pre>

Verify a DER signature; any encoding that is not strict DER fails.

[Source](/reference-sources/Crypto.Ecdsa.cinch) · line 74

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Crypto/Ecdsa.cinch`. Source SHA-256: `6f15ab42fb1c78de630051e078c31260bdc5b3aa416ac90423471f79e171ffa1`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
