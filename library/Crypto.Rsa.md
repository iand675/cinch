# Crypto.Rsa

RSA PKCS#1 v1.5 and PSS signing/verification, OAEP and PKCS#1 v1.5 encryption/decryption, PKCS#1 DER key codecs and seeded key generation.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Crypto.Rsa`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Crypto.Rsa.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Crypto.ConstantTime](Crypto.ConstantTime.md), [Data.Bytes](Data.Bytes.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Data.Int](Data.Int.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Text.Asn1](Text.Asn1.md), [Text.Md5](Text.Md5.md), [Text.Sha1](Text.Sha1.md), [Text.Sha256](Text.Sha256.md), [Text.Sha512](Text.Sha512.md).

Non-embedded source closure: [Crypto.ConstantTime](Crypto.ConstantTime.md), [Crypto.Rsa](Crypto.Rsa.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Text.Asn1](Text.Asn1.md), [Text.Binary](Text.Binary.md), [Text.Md5](Text.Md5.md), [Text.Sha1](Text.Sha1.md), [Text.Sha256](Text.Sha256.md), [Text.Sha512](Text.Sha512.md).

<details><summary>Behaviour and boundary notes from the source</summary>

RSA public-key cryptography (RFC 8017): PKCS#1 v1.5 and RSASSA-PSS signatures,
RSAES-OAEP and RSAES-PKCS1-v1_5 encryption, PKCS#1 DER key serialization, key
validation and deterministic seeded key generation.

Private-key modular exponentiation uses Chinese Remainder Theorem (CRT)
recombination with a public-exponent consistency check against faults. Padding
checks in decryption and signature verification use constant-time comparisons
from Crypto.ConstantTime; underlying arbitrary-precision arithmetic uses Data.Nat
and is not constant-time in operand values.

</details>

# Types and constructors

<a id="type-50737353616c744c656e677468"></a>

## `PssSaltLength`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">PssSaltLength</span></code></pre>

Salt length policy for RSASSA-PSS signing and verification.

[Source](/reference-sources/Crypto.Rsa.cinch) · line 70

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PssSaltAuto</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">PssSaltLength</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PssSaltEqualsHash</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">PssSaltLength</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PssSaltExact</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">PssSaltLength</span>)</code></pre>

<a id="type-5273614572726f72"></a>

## `RsaError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">RsaError</span></code></pre>

Errors produced by RSA key handling, encoding, signing, verification and encryption.

[Source](/reference-sources/Crypto.Rsa.cinch) · line 79

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">RsaMessageTooLong</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">RsaDecryptionError</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">RsaVerificationError</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">RsaInvalidPublicKey</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">RsaInvalidPrivateKey</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">RsaInvalidDigestLength</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">RsaUnsupportedHash</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">RsaInvalidParameter</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">RsaInvalidEncoding</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaError</span></code></pre>

<a id="type-52736148617368"></a>

## `RsaHash`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">RsaHash</span></code></pre>

Hash algorithm used by RSA signature and OAEP padding schemes.

[Source](/reference-sources/Crypto.Rsa.cinch) · line 49

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">RsaNoHash</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaHash</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">RsaMd5</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaHash</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">RsaSha1</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaHash</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">RsaSha224</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaHash</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">RsaSha256</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaHash</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">RsaSha384</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaHash</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">RsaSha512</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaHash</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">RsaSha512t224</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaHash</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">RsaSha512t256</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaHash</span></code></pre>

<a id="type-527361507269766174654b6579"></a>

## `RsaPrivateKey`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">RsaPrivateKey</span></code></pre>

A validated two-prime RSA private key with precomputed CRT values.

[Source](/reference-sources/Crypto.Rsa.cinch) · line 123

<a id="type-5273615075626c69634b6579"></a>

## `RsaPublicKey`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">RsaPublicKey</span></code></pre>

A validated RSA public key holding modulus n and public exponent e.

[Source](/reference-sources/Crypto.Rsa.cinch) · line 113

# Values

<a id="value-7273612d646563727970742d6f616570"></a>

## `rsa-decrypt-oaep`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rsa-decrypt-oaep</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaPrivateKey</span> -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaHash</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaError</span> <span class="tok-constructor">Bytes</span>)))))</code></pre>

Decrypt an RSAES-OAEP ciphertext using the same hash for the label and MGF1, with constant-time padding checks.

[Source](/reference-sources/Crypto.Rsa.cinch) · line 954

<a id="value-7273612d646563727970742d6f6165702d6d6766"></a>

## `rsa-decrypt-oaep-mgf`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rsa-decrypt-oaep-mgf</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaPrivateKey</span> -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaHash</span> -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaHash</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaError</span> <span class="tok-constructor">Bytes</span>))))))</code></pre>

Decrypt an RSAES-OAEP ciphertext with separate label and MGF1 hashes and constant-time padding validation.

[Source](/reference-sources/Crypto.Rsa.cinch) · line 958

<a id="value-7273612d646563727970742d706b637331763135"></a>

## `rsa-decrypt-pkcs1v15`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rsa-decrypt-pkcs1v15</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaPrivateKey</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaError</span> <span class="tok-constructor">Bytes</span>)))</code></pre>

Decrypt an RSAES-PKCS1-v1_5 ciphertext with constant-time padding validation.

[Source](/reference-sources/Crypto.Rsa.cinch) · line 1058

<a id="value-7273612d646563727970742d706b6373317631352d73657373696f6e2d6b6579"></a>

## `rsa-decrypt-pkcs1v15-session-key`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rsa-decrypt-pkcs1v15-session-key</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaPrivateKey</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaError</span> <span class="tok-constructor">Bytes</span>))))</code></pre>

Decrypt an RSAES-PKCS1-v1_5 session key of fixed length, returning fallback if padding or length is invalid.

[Source](/reference-sources/Crypto.Rsa.cinch) · line 1068

<a id="value-7273612d656e63727970742d6f6165702d6d6766"></a>

## `rsa-encrypt-oaep-mgf`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rsa-encrypt-oaep-mgf</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaPublicKey</span> -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaHash</span> -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaHash</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaError</span> <span class="tok-constructor">Bytes</span>)))))))</code></pre>

Encrypt a message with RSAES-OAEP using separate label and MGF1 hashes, and explicit seed bytes.

[Source](/reference-sources/Crypto.Rsa.cinch) · line 924

<a id="value-7273612d656e63727970742d6f6165702d776974682d73656564"></a>

## `rsa-encrypt-oaep-with-seed`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rsa-encrypt-oaep-with-seed</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaPublicKey</span> -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaHash</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaError</span> <span class="tok-constructor">Bytes</span>))))))</code></pre>

Encrypt a message with RSAES-OAEP using the same hash for the label and MGF1, and explicit seed bytes.

[Source](/reference-sources/Crypto.Rsa.cinch) · line 920

<a id="value-7273612d656e63727970742d706b6373317631352d66726f6d2d73656564"></a>

## `rsa-encrypt-pkcs1v15-from-seed`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rsa-encrypt-pkcs1v15-from-seed</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaPublicKey</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaError</span> <span class="tok-constructor">Bytes</span>))))</code></pre>

Encrypt a message with RSAES-PKCS1-v1_5, deriving non-zero padding bytes deterministically from seed.

[Source](/reference-sources/Crypto.Rsa.cinch) · line 1039

<a id="value-7273612d656e63727970742d706b6373317631352d776974682d706164"></a>

## `rsa-encrypt-pkcs1v15-with-pad`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rsa-encrypt-pkcs1v15-with-pad</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaPublicKey</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaError</span> <span class="tok-constructor">Bytes</span>))))</code></pre>

Encrypt a message with RSAES-PKCS1-v1_5 using explicit non-zero padding bytes of length k - len(msg) - 3.

[Source](/reference-sources/Crypto.Rsa.cinch) · line 1020

<a id="value-7273612d6572726f722d6d657373616765"></a>

## `rsa-error-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rsa-error-message</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaError</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Render an RSA error as a human-readable message.

[Source](/reference-sources/Crypto.Rsa.cinch) · line 100

<a id="value-7273612d67656e65726174652d6b65792d66726f6d2d73656564"></a>

## `rsa-generate-key-from-seed`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rsa-generate-key-from-seed</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaError</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaPrivateKey</span>)))</code></pre>

Generate a validated two-prime RSA private key of the requested bit size (64..8192) deterministically from seed.

[Source](/reference-sources/Crypto.Rsa.cinch) · line 1186

<a id="value-7273612d686173682d73697a65"></a>

## `rsa-hash-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rsa-hash-size</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaHash</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Output digest length in bytes for an RSA hash algorithm (0 for RsaNoHash).

[Source](/reference-sources/Crypto.Rsa.cinch) · line 418

<a id="value-7273612d686173682d73756d"></a>

## `rsa-hash-sum`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rsa-hash-sum</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaHash</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaError</span> <span class="tok-constructor">Bytes</span>)))</code></pre>

Compute a hash digest of a message using the specified RSA hash algorithm.

[Source](/reference-sources/Crypto.Rsa.cinch) · line 431

<a id="value-7273612d69326f7370"></a>

## `rsa-i2osp`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rsa-i2osp</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Bytes</span>))</code></pre>

Convert a Nat to a fixed-length big-endian byte string, zero-padded on the left (RFC 8017 I2OSP).

[Source](/reference-sources/Crypto.Rsa.cinch) · line 250

<a id="value-7273612d6b65792d62697473"></a>

## `rsa-key-bits`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rsa-key-bits</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaPublicKey</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Length of the RSA modulus in bits.

[Source](/reference-sources/Crypto.Rsa.cinch) · line 529

<a id="value-7273612d6b65792d73697a65"></a>

## `rsa-key-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rsa-key-size</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaPublicKey</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Length of the RSA modulus in bytes.

[Source](/reference-sources/Crypto.Rsa.cinch) · line 525

<a id="value-7273612d6d61727368616c2d706b6373312d707269766174652d6b6579"></a>

## `rsa-marshal-pkcs1-private-key`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rsa-marshal-pkcs1-private-key</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaPrivateKey</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Marshal an RSA private key to PKCS#1 ASN.1 DER (RFC 8017 Appendix A.1.2).

[Source](/reference-sources/Crypto.Rsa.cinch) · line 588

<a id="value-7273612d6d61727368616c2d706b6373312d7075626c69632d6b6579"></a>

## `rsa-marshal-pkcs1-public-key`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rsa-marshal-pkcs1-public-key</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaPublicKey</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Marshal an RSA public key to PKCS#1 ASN.1 DER (RFC 8017 Appendix A.1.1).

[Source](/reference-sources/Crypto.Rsa.cinch) · line 571

<a id="value-7273612d6f73326970"></a>

## `rsa-os2ip`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rsa-os2ip</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Convert an unsigned big-endian byte string to a Nat (RFC 8017 OS2IP).

[Source](/reference-sources/Crypto.Rsa.cinch) · line 232

<a id="value-7273612d70617273652d706b6373312d707269766174652d6b6579"></a>

## `rsa-parse-pkcs1-private-key`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rsa-parse-pkcs1-private-key</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaError</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaPrivateKey</span>))</code></pre>

Parse a two-prime RSA private key from PKCS#1 ASN.1 DER.

[Source](/reference-sources/Crypto.Rsa.cinch) · line 604

<a id="value-7273612d70617273652d706b6373312d7075626c69632d6b6579"></a>

## `rsa-parse-pkcs1-public-key`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rsa-parse-pkcs1-public-key</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaError</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaPublicKey</span>))</code></pre>

Parse an RSA public key from PKCS#1 ASN.1 DER.

[Source](/reference-sources/Crypto.Rsa.cinch) · line 576

<a id="value-7273612d707269766174652d64"></a>

## `rsa-private-d`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rsa-private-d</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaPrivateKey</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Private exponent d of an RSA private key.

[Source](/reference-sources/Crypto.Rsa.cinch) · line 501

<a id="value-7273612d707269766174652d6470"></a>

## `rsa-private-dp`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rsa-private-dp</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaPrivateKey</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Precomputed CRT exponent dp = d mod (p - 1).

[Source](/reference-sources/Crypto.Rsa.cinch) · line 513

<a id="value-7273612d707269766174652d6471"></a>

## `rsa-private-dq`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rsa-private-dq</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaPrivateKey</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Precomputed CRT exponent dq = d mod (q - 1).

[Source](/reference-sources/Crypto.Rsa.cinch) · line 517

<a id="value-7273612d707269766174652d6b6579"></a>

## `rsa-private-key`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rsa-private-key</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaError</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaPrivateKey</span>))))))</code></pre>

Construct, validate and precompute CRT parameters for a two-prime RSA private key.

[Source](/reference-sources/Crypto.Rsa.cinch) · line 469

<a id="value-7273612d707269766174652d6b65792d657175616c"></a>

## `rsa-private-key-equal`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rsa-private-key-equal</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaPrivateKey</span> -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaPrivateKey</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

Compare two RSA private keys for equality.

[Source](/reference-sources/Crypto.Rsa.cinch) · line 553

<a id="value-7273612d707269766174652d70"></a>

## `rsa-private-p`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rsa-private-p</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaPrivateKey</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

First prime factor p of an RSA private key.

[Source](/reference-sources/Crypto.Rsa.cinch) · line 505

<a id="value-7273612d707269766174652d71"></a>

## `rsa-private-q`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rsa-private-q</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaPrivateKey</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Second prime factor q of an RSA private key.

[Source](/reference-sources/Crypto.Rsa.cinch) · line 509

<a id="value-7273612d707269766174652d71696e76"></a>

## `rsa-private-qinv`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rsa-private-qinv</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaPrivateKey</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Precomputed CRT coefficient qinv = q^(-1) mod p.

[Source](/reference-sources/Crypto.Rsa.cinch) · line 521

<a id="value-7273612d7075626c69632d65"></a>

## `rsa-public-e`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rsa-public-e</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaPublicKey</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Public exponent e of an RSA public key.

[Source](/reference-sources/Crypto.Rsa.cinch) · line 497

<a id="value-7273612d7075626c69632d6b6579"></a>

## `rsa-public-key`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rsa-public-key</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaError</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaPublicKey</span>)))</code></pre>

Construct and validate an RSA public key from modulus n and public exponent e.

[Source](/reference-sources/Crypto.Rsa.cinch) · line 461

<a id="value-7273612d7075626c69632d6b65792d657175616c"></a>

## `rsa-public-key-equal`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rsa-public-key-equal</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaPublicKey</span> -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaPublicKey</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

Compare two RSA public keys for equality.

[Source](/reference-sources/Crypto.Rsa.cinch) · line 549

<a id="value-7273612d7075626c69632d6b65792d6f66"></a>

## `rsa-public-key-of`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rsa-public-key-of</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaPrivateKey</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaPublicKey</span>)</code></pre>

Extract the public key corresponding to an RSA private key.

[Source](/reference-sources/Crypto.Rsa.cinch) · line 489

<a id="value-7273612d7075626c69632d6e"></a>

## `rsa-public-n`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rsa-public-n</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaPublicKey</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Modulus n of an RSA public key.

[Source](/reference-sources/Crypto.Rsa.cinch) · line 493

<a id="value-7273612d7369676e2d706b637331763135"></a>

## `rsa-sign-pkcs1v15`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rsa-sign-pkcs1v15</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaPrivateKey</span> -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaHash</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaError</span> <span class="tok-constructor">Bytes</span>))))</code></pre>

Sign a pre-computed message digest with RSASSA-PKCS1-v1_5.

[Source](/reference-sources/Crypto.Rsa.cinch) · line 723

<a id="value-7273612d7369676e2d707373"></a>

## `rsa-sign-pss`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rsa-sign-pss</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaPrivateKey</span> -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaHash</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">PssSaltLength</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaError</span> <span class="tok-constructor">Bytes</span>))))))</code></pre>

Sign a pre-computed message digest with RSASSA-PSS, deriving salt bytes deterministically from seed.

[Source](/reference-sources/Crypto.Rsa.cinch) · line 801

<a id="value-7273612d7369676e2d7073732d64657465726d696e6973746963"></a>

## `rsa-sign-pss-deterministic`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rsa-sign-pss-deterministic</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaPrivateKey</span> -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaHash</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaError</span> <span class="tok-constructor">Bytes</span>)))))</code></pre>

Sign a pre-computed message digest with RSASSA-PSS using explicit salt bytes.

[Source](/reference-sources/Crypto.Rsa.cinch) · line 790

<a id="value-7273612d76616c69646174652d707269766174652d6b6579"></a>

## `rsa-validate-private-key`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rsa-validate-private-key</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaPrivateKey</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaError</span> <span class="tok-constructor">Unit</span>))</code></pre>

Re-validate all structural and mathematical invariants of an RSA private key.

[Source](/reference-sources/Crypto.Rsa.cinch) · line 533

<a id="value-7273612d7665726966792d706b637331763135"></a>

## `rsa-verify-pkcs1v15`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rsa-verify-pkcs1v15</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaPublicKey</span> -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaHash</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaError</span> <span class="tok-constructor">Unit</span>)))))</code></pre>

Verify an RSASSA-PKCS1-v1_5 signature over a pre-computed message digest.

[Source](/reference-sources/Crypto.Rsa.cinch) · line 733

<a id="value-7273612d7665726966792d707373"></a>

## `rsa-verify-pss`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rsa-verify-pss</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaPublicKey</span> -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaHash</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">PssSaltLength</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaError</span> <span class="tok-constructor">Unit</span>))))))</code></pre>

Verify an RSASSA-PSS signature over a pre-computed message digest.

[Source](/reference-sources/Crypto.Rsa.cinch) · line 831

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Crypto/Rsa.cinch`. Source SHA-256: `48471c98c2a0660a5960461bb7e07be1fa1d9141f6c4275b54303c3a28a7a299`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
