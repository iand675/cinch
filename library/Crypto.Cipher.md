# Crypto.Cipher

CTR, OFB, CFB, CBC and GCM over any 128-bit block cipher, with constant-time or carry-less-instruction GHASH and constant-time tag checks.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Crypto.Cipher`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Crypto.Cipher.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bytes](Data.Bytes.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Data.Wide](Data.Wide.md).

Non-embedded source closure: [Crypto.Cipher](Crypto.Cipher.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Block-cipher modes over any 128-bit block cipher: CTR, OFB and CFB keystreams, CBC, and GCM authenticated encryption.
A BlockCipher packages a cipher's forward and inverse block functions over
U128 blocks whose byte i occupies bits 8i..8i+7 (memory order). Keystream
modes share one explicit state machine, XorStream, whose register, buffered
keystream and consumption count persist across calls of any length; retaining
a stream branches it. CBC processes whole blocks and carries its chaining
value explicitly. GCM follows NIST SP 800-38D: GHASH uses a constant-time
portable multiplication or PCLMULQDQ after CPU feature detection, tags are
compared in constant time, and no plaintext is released before the tag
verifies. Every returned buffer is fresh; inputs are never retained or mutated.

</details>

# Types and constructors

<a id="type-426c6f636b436970686572"></a>

## `BlockCipher`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">BlockCipher</span></code></pre>

Encrypt and decrypt functions over canonical U128 blocks.

[Source](/reference-sources/Crypto.Cipher.cinch) · line 30

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">BlockCipher</span> :: ((<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">U128</span>) -&gt; ((<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">U128</span>) -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Cipher</span>.<span class="tok-constructor">BlockCipher</span>))</code></pre>

<a id="type-4362635374617465"></a>

## `CbcState`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">CbcState</span></code></pre>

CBC state: cipher, direction (True encrypts) and the chaining value for the next block.

[Source](/reference-sources/Crypto.Cipher.cinch) · line 240

<a id="type-4369706865724572726f72"></a>

## `CipherError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">CipherError</span></code></pre>

A cipher misuse or authentication failure; every operation reports one before producing output.

[Source](/reference-sources/Crypto.Cipher.cinch) · line 95

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">InvalidIvSize</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Cipher</span>.<span class="tok-constructor">CipherError</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">InvalidInputSize</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Cipher</span>.<span class="tok-constructor">CipherError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">InvalidNonceSize</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Cipher</span>.<span class="tok-constructor">CipherError</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">InvalidTagSize</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Cipher</span>.<span class="tok-constructor">CipherError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MessageTooLarge</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Cipher</span>.<span class="tok-constructor">CipherError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">AuthenticationFailed</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Cipher</span>.<span class="tok-constructor">CipherError</span></code></pre>

<a id="type-47636d"></a>

## `Gcm`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Gcm</span></code></pre>

A GCM instance: cipher, hash key H, nonce size, tag size and whether GHASH uses carry-less instructions.

[Source](/reference-sources/Crypto.Cipher.cinch) · line 288

<a id="type-586f7253747265616d"></a>

## `XorStream`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">XorStream</span></code></pre>

A keystream mode's state: cipher, mode (1 CTR, 2 OFB, 3 CFB encrypt, 4 CFB decrypt), register,
current keystream block, bytes of it already used (0 to 16), and the CFB ciphertext feedback gathered so far.

[Source](/reference-sources/Crypto.Cipher.cinch) · line 121

# Values

<a id="value-626c6f636b2d636970686572"></a>

## `block-cipher`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">block-cipher</span> :: ((<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">U128</span>) -&gt; ((<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">U128</span>) -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Cipher</span>.<span class="tok-constructor">BlockCipher</span>))</code></pre>

Package forward and inverse block functions; decrypt must invert encrypt.

[Source](/reference-sources/Crypto.Cipher.cinch) · line 33

<a id="value-626c6f636b2d64656372797074"></a>

## `block-decrypt`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">block-decrypt</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Cipher</span>.<span class="tok-constructor">BlockCipher</span> -&gt; (<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">U128</span>))</code></pre>

Apply the inverse permutation to one block.

[Source](/reference-sources/Crypto.Cipher.cinch) · line 41

<a id="value-626c6f636b2d656e6372797074"></a>

## `block-encrypt`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">block-encrypt</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Cipher</span>.<span class="tok-constructor">BlockCipher</span> -&gt; (<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">U128</span>))</code></pre>

Apply the forward permutation to one block.

[Source](/reference-sources/Crypto.Cipher.cinch) · line 37

<a id="value-626c6f636b2d66726f6d2d6279746573"></a>

## `block-from-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">block-from-bytes</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">U128</span>)))</code></pre>

Read sixteen bytes at a position as a canonical block, or None when they are not all present.

[Source](/reference-sources/Crypto.Cipher.cinch) · line 49

<a id="value-626c6f636b2d746f2d6279746573"></a>

## `block-to-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">block-to-bytes</span> :: (<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Return a block's sixteen bytes in fresh storage.

[Source](/reference-sources/Crypto.Cipher.cinch) · line 53

<a id="value-6362632d63727970742d626c6f636b73"></a>

## `cbc-crypt-blocks`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cbc-crypt-blocks</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Cipher</span>.<span class="tok-constructor">CbcState</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Cipher</span>.<span class="tok-constructor">CipherError</span> (<span class="tok-constructor">Bytes</span>, <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Cipher</span>.<span class="tok-constructor">CbcState</span>))))</code></pre>

Encrypt or decrypt whole blocks, returning fresh output and the state chained to the last block.
Input that is not a multiple of sixteen bytes is rejected before any work; CBC applies no padding.

[Source](/reference-sources/Crypto.Cipher.cinch) · line 254

<a id="value-6362632d64656372797074"></a>

## `cbc-decrypt`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cbc-decrypt</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Cipher</span>.<span class="tok-constructor">BlockCipher</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Cipher</span>.<span class="tok-constructor">CipherError</span> <span class="tok-constructor">Bytes</span>))))</code></pre>

One-shot CBC decryption of whole blocks.

[Source](/reference-sources/Crypto.Cipher.cinch) · line 282

<a id="value-6362632d646563727970746572"></a>

## `cbc-decrypter`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cbc-decrypter</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Cipher</span>.<span class="tok-constructor">BlockCipher</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Cipher</span>.<span class="tok-constructor">CipherError</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Cipher</span>.<span class="tok-constructor">CbcState</span>)))</code></pre>

Start CBC decryption from a sixteen-byte IV.

[Source](/reference-sources/Crypto.Cipher.cinch) · line 249

<a id="value-6362632d656e6372797074"></a>

## `cbc-encrypt`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cbc-encrypt</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Cipher</span>.<span class="tok-constructor">BlockCipher</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Cipher</span>.<span class="tok-constructor">CipherError</span> <span class="tok-constructor">Bytes</span>))))</code></pre>

One-shot CBC encryption of whole blocks.

[Source](/reference-sources/Crypto.Cipher.cinch) · line 276

<a id="value-6362632d656e63727970746572"></a>

## `cbc-encrypter`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cbc-encrypter</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Cipher</span>.<span class="tok-constructor">BlockCipher</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Cipher</span>.<span class="tok-constructor">CipherError</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Cipher</span>.<span class="tok-constructor">CbcState</span>)))</code></pre>

Start CBC encryption from a sixteen-byte IV.

[Source](/reference-sources/Crypto.Cipher.cinch) · line 245

<a id="value-6366622d64656372797074"></a>

## `cfb-decrypt`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cfb-decrypt</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Cipher</span>.<span class="tok-constructor">BlockCipher</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Cipher</span>.<span class="tok-constructor">CipherError</span> <span class="tok-constructor">Bytes</span>))))</code></pre>

One-shot cipher-feedback decryption from an IV.

[Source](/reference-sources/Crypto.Cipher.cinch) · line 234

<a id="value-6366622d646563727970742d73747265616d"></a>

## `cfb-decrypt-stream`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cfb-decrypt-stream</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Cipher</span>.<span class="tok-constructor">BlockCipher</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Cipher</span>.<span class="tok-constructor">CipherError</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Cipher</span>.<span class="tok-constructor">XorStream</span>)))</code></pre>

Full-block cipher feedback decryption, the inverse of cfb-encrypt-stream for the same IV.

[Source](/reference-sources/Crypto.Cipher.cinch) · line 145

<a id="value-6366622d656e6372797074"></a>

## `cfb-encrypt`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cfb-encrypt</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Cipher</span>.<span class="tok-constructor">BlockCipher</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Cipher</span>.<span class="tok-constructor">CipherError</span> <span class="tok-constructor">Bytes</span>))))</code></pre>

One-shot cipher-feedback encryption from an IV.

[Source](/reference-sources/Crypto.Cipher.cinch) · line 228

<a id="value-6366622d656e63727970742d73747265616d"></a>

## `cfb-encrypt-stream`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cfb-encrypt-stream</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Cipher</span>.<span class="tok-constructor">BlockCipher</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Cipher</span>.<span class="tok-constructor">CipherError</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Cipher</span>.<span class="tok-constructor">XorStream</span>)))</code></pre>

Full-block cipher feedback encryption: each keystream block encrypts the previous ciphertext block.

[Source](/reference-sources/Crypto.Cipher.cinch) · line 141

<a id="value-6369706865722d626c6f636b2d73697a65"></a>

## `cipher-block-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cipher-block-size</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Cipher</span>.<span class="tok-constructor">BlockCipher</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Block size in bytes, always 16.

[Source](/reference-sources/Crypto.Cipher.cinch) · line 45

<a id="value-6369706865722d6572726f722d6d657373616765"></a>

## `cipher-error-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cipher-error-message</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Cipher</span>.<span class="tok-constructor">CipherError</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Render a cipher error.

[Source](/reference-sources/Crypto.Cipher.cinch) · line 110

<a id="value-6374722d73747265616d"></a>

## `ctr-stream`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ctr-stream</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Cipher</span>.<span class="tok-constructor">BlockCipher</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Cipher</span>.<span class="tok-constructor">CipherError</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Cipher</span>.<span class="tok-constructor">XorStream</span>)))</code></pre>

Counter mode: the IV is a 128-bit big-endian counter incremented once per block, wrapping.

[Source](/reference-sources/Crypto.Cipher.cinch) · line 133

<a id="value-6374722d786f72"></a>

## `ctr-xor`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ctr-xor</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Cipher</span>.<span class="tok-constructor">BlockCipher</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Cipher</span>.<span class="tok-constructor">CipherError</span> <span class="tok-constructor">Bytes</span>))))</code></pre>

One-shot counter-mode XOR from an IV.

[Source](/reference-sources/Crypto.Cipher.cinch) · line 216

<a id="value-67636d"></a>

## `gcm`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gcm</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Cipher</span>.<span class="tok-constructor">BlockCipher</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Cipher</span>.<span class="tok-constructor">Gcm</span>)</code></pre>

GCM with the standard 12-byte nonce and 16-byte tag.

[Source](/reference-sources/Crypto.Cipher.cinch) · line 293

<a id="value-67636d2d6861726477617265"></a>

## `gcm-hardware`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gcm-hardware</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Cipher</span>.<span class="tok-constructor">Gcm</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Whether GHASH runs on carry-less multiplication instructions.

[Source](/reference-sources/Crypto.Cipher.cinch) · line 320

<a id="value-67636d2d6e6f6e63652d73697a65"></a>

## `gcm-nonce-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gcm-nonce-size</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Cipher</span>.<span class="tok-constructor">Gcm</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Required nonce size in bytes.

[Source](/reference-sources/Crypto.Cipher.cinch) · line 308

<a id="value-67636d2d6f70656e"></a>

## `gcm-open`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gcm-open</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Cipher</span>.<span class="tok-constructor">Gcm</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Cipher</span>.<span class="tok-constructor">CipherError</span> <span class="tok-constructor">Bytes</span>)))))</code></pre>

Verify and decrypt ciphertext followed by its tag. Verification is constant time, and
AuthenticationFailed is returned without any plaintext when the tag does not match.

[Source](/reference-sources/Crypto.Cipher.cinch) · line 399

<a id="value-67636d2d6f76657268656164"></a>

## `gcm-overhead`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gcm-overhead</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Cipher</span>.<span class="tok-constructor">Gcm</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Bytes a sealed message adds to its plaintext: the tag size.

[Source](/reference-sources/Crypto.Cipher.cinch) · line 316

<a id="value-67636d2d7365616c"></a>

## `gcm-seal`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gcm-seal</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Cipher</span>.<span class="tok-constructor">Gcm</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Cipher</span>.<span class="tok-constructor">CipherError</span> <span class="tok-constructor">Bytes</span>)))))</code></pre>

Encrypt and authenticate: returns ciphertext followed by the tag in fresh storage.
Nonces must never repeat under one key; repetition destroys both confidentiality and authenticity.

[Source](/reference-sources/Crypto.Cipher.cinch) · line 382

<a id="value-67636d2d7461672d73697a65"></a>

## `gcm-tag-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gcm-tag-size</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Cipher</span>.<span class="tok-constructor">Gcm</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Tag size in bytes.

[Source](/reference-sources/Crypto.Cipher.cinch) · line 312

<a id="value-67636d2d776974682d6e6f6e63652d73697a65"></a>

## `gcm-with-nonce-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gcm-with-nonce-size</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Cipher</span>.<span class="tok-constructor">BlockCipher</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Cipher</span>.<span class="tok-constructor">CipherError</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Cipher</span>.<span class="tok-constructor">Gcm</span>)))</code></pre>

GCM with a non-standard nonce size; sizes other than 12 derive the initial counter through GHASH.

[Source](/reference-sources/Crypto.Cipher.cinch) · line 297

<a id="value-67636d2d776974682d7461672d73697a65"></a>

## `gcm-with-tag-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gcm-with-tag-size</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Cipher</span>.<span class="tok-constructor">BlockCipher</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Cipher</span>.<span class="tok-constructor">CipherError</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Cipher</span>.<span class="tok-constructor">Gcm</span>)))</code></pre>

GCM with a truncated tag of 12 to 16 bytes.

[Source](/reference-sources/Crypto.Cipher.cinch) · line 302

<a id="value-6f66622d73747265616d"></a>

## `ofb-stream`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ofb-stream</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Cipher</span>.<span class="tok-constructor">BlockCipher</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Cipher</span>.<span class="tok-constructor">CipherError</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Cipher</span>.<span class="tok-constructor">XorStream</span>)))</code></pre>

Output feedback mode: each keystream block encrypts the previous one, starting from the IV.

[Source](/reference-sources/Crypto.Cipher.cinch) · line 137

<a id="value-6f66622d786f72"></a>

## `ofb-xor`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ofb-xor</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Cipher</span>.<span class="tok-constructor">BlockCipher</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Cipher</span>.<span class="tok-constructor">CipherError</span> <span class="tok-constructor">Bytes</span>))))</code></pre>

One-shot output-feedback XOR from an IV.

[Source](/reference-sources/Crypto.Cipher.cinch) · line 222

<a id="value-73747265616d2d786f72"></a>

## `stream-xor`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">stream-xor</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Cipher</span>.<span class="tok-constructor">XorStream</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span>, <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Cipher</span>.<span class="tok-constructor">XorStream</span>)))</code></pre>

XOR an input of any length with the keystream, returning fresh output and the advanced stream.
Applying the same stream to the output reverses it for CTR and OFB; CFB uses its matching decrypt stream.

[Source](/reference-sources/Crypto.Cipher.cinch) · line 165

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Crypto/Cipher.cinch`. Source SHA-256: `85f5838972cd8debab7f25257b2b174d2335f40a0adeab4b42c7e60788fffba1`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
