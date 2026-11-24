# Crypto.Aes

AES-128/192/256 with AES-NI or ARMv8 instructions after detection and a constant-time, table-free SIMD fallback.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Crypto.Aes`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Crypto.Aes.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Crypto.Cipher](Crypto.Cipher.md), [Data.Bytes](Data.Bytes.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Data.Wide](Data.Wide.md).

Non-embedded source closure: [Crypto.Aes](Crypto.Aes.md), [Crypto.Cipher](Crypto.Cipher.md), [Data.Bytes.Word](Data.Bytes.Word.md).

<details><summary>Behaviour and boundary notes from the source</summary>

AES-128, AES-192 and AES-256 block encryption with hardware instructions and a constant-time portable fallback.
Keys are expanded once into immutable round-key schedules. On x86-64 the
AES-NI instructions are used after CPUID detection; on AArch64 the ARMv8
cryptography extension is used when the target profile supplies ArmAes
evidence. Otherwise every round runs in pure Cinch across all sixteen state
bytes in parallel using SIMD lane operations and bit-sliced tower-field
GF(2^8) inversion plus the affine map, so the fallback performs no table
lookups and no data-dependent branches. Blocks are U128 values with byte i in
bits 8i..8i+7; byte-buffer operations return fresh storage. aes-cipher adapts
a key to Crypto.Cipher's CTR, CBC, OFB, CFB and GCM modes.

</details>

# Types and constructors

<a id="type-4165734b6579"></a>

## `AesKey`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">AesKey</span></code></pre>

An expanded key: key size in bytes, whether hardware rounds are used, and encryption and decryption schedules.
With hardware rounds the decryption schedule holds the equivalent inverse cipher's keys.

[Source](/reference-sources/Crypto.Aes.cinch) · line 49

<a id="type-4165734b65794572726f72"></a>

## `AesKeyError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">AesKeyError</span></code></pre>

A key or block of the wrong length.

[Source](/reference-sources/Crypto.Aes.cinch) · line 54

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">InvalidAesKeySize</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Aes</span>.<span class="tok-constructor">AesKeyError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">InvalidAesBlockSize</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Aes</span>.<span class="tok-constructor">AesKeyError</span>)</code></pre>

# Signatures and methods

<a id="class-41726d416573"></a>

## `ArmAes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">signature</span> <span class="tok-constructor">ArmAes</span> </code></pre>

Target-profile evidence that the ARMv8 AES instructions may execute.
Only profiles that guarantee the cryptography extension should name this capability.

[Source](/reference-sources/Crypto.Aes.cinch) · line 30

# Values

<a id="value-6165732d626c6f636b2d73697a65"></a>

## `aes-block-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">aes-block-size</span> :: <span class="tok-constructor">Nat</span></code></pre>

AES block size in bytes, always 16.

[Source](/reference-sources/Crypto.Aes.cinch) · line 35

<a id="value-6165732d636970686572"></a>

## `aes-cipher`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">aes-cipher</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Aes</span>.<span class="tok-constructor">AesKey</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Cipher</span>.<span class="tok-constructor">BlockCipher</span>)</code></pre>

Adapt a key to the generic block-cipher modes.

[Source](/reference-sources/Crypto.Aes.cinch) · line 209

<a id="value-6165732d646563727970742d626c6f636b"></a>

## `aes-decrypt-block`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">aes-decrypt-block</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Aes</span>.<span class="tok-constructor">AesKey</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Aes</span>.<span class="tok-constructor">AesKeyError</span> <span class="tok-constructor">Bytes</span>)))</code></pre>

Decrypt exactly sixteen bytes into fresh storage.

[Source](/reference-sources/Crypto.Aes.cinch) · line 203

<a id="value-6165732d646563727970742d776f7264"></a>

## `aes-decrypt-word`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">aes-decrypt-word</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Aes</span>.<span class="tok-constructor">AesKey</span> -&gt; (<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">U128</span>))</code></pre>

Decrypt one canonical block.

[Source](/reference-sources/Crypto.Aes.cinch) · line 183

<a id="value-6165732d656e63727970742d626c6f636b"></a>

## `aes-encrypt-block`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">aes-encrypt-block</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Aes</span>.<span class="tok-constructor">AesKey</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Aes</span>.<span class="tok-constructor">AesKeyError</span> <span class="tok-constructor">Bytes</span>)))</code></pre>

Encrypt exactly sixteen bytes into fresh storage.

[Source](/reference-sources/Crypto.Aes.cinch) · line 197

<a id="value-6165732d656e63727970742d776f7264"></a>

## `aes-encrypt-word`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">aes-encrypt-word</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Aes</span>.<span class="tok-constructor">AesKey</span> -&gt; (<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">U128</span>))</code></pre>

Encrypt one canonical block.

[Source](/reference-sources/Crypto.Aes.cinch) · line 169

<a id="value-6165732d6b6579"></a>

## `aes-key`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">aes-key</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Aes</span>.<span class="tok-constructor">AesKeyError</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Aes</span>.<span class="tok-constructor">AesKey</span>))</code></pre>

Expand a 16-, 24- or 32-byte key, selecting hardware rounds when the processor supports them.

[Source](/reference-sources/Crypto.Aes.cinch) · line 67

<a id="value-6165732d6b65792d6572726f722d6d657373616765"></a>

## `aes-key-error-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">aes-key-error-message</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Aes</span>.<span class="tok-constructor">AesKeyError</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Render a key or block size error.

[Source](/reference-sources/Crypto.Aes.cinch) · line 61

<a id="value-6165732d6b65792d6861726477617265"></a>

## `aes-key-hardware`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">aes-key-hardware</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Aes</span>.<span class="tok-constructor">AesKey</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Whether this key's rounds run on hardware AES instructions.

[Source](/reference-sources/Crypto.Aes.cinch) · line 79

<a id="value-6165732d6b65792d73697a65"></a>

## `aes-key-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">aes-key-size</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Aes</span>.<span class="tok-constructor">AesKey</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Key size in bytes: 16, 24 or 32.

[Source](/reference-sources/Crypto.Aes.cinch) · line 75

<a id="value-6165732d706f727461626c652d6b6579"></a>

## `aes-portable-key`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">aes-portable-key</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Aes</span>.<span class="tok-constructor">AesKeyError</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Aes</span>.<span class="tok-constructor">AesKey</span>))</code></pre>

Expand a key that always uses the constant-time portable rounds, whatever the processor supports.

[Source](/reference-sources/Crypto.Aes.cinch) · line 71

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Crypto/Aes.cinch`. Source SHA-256: `67cbeaad7f50e32f07b2447c843ecf34549d1983b3bff6d341eff449eb7b8cba`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
