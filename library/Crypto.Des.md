# Crypto.Des

DES and Triple DES block ciphers with ECB, CBC and CTR for legacy interoperation, from a generated straight-line round kernel.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Crypto.Des`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Crypto.Des.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bytes](Data.Bytes.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Text.Hex](Text.Hex.md), [Text.Show](Text.Show.md).

Non-embedded source closure: [Crypto.Des](Crypto.Des.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Text.Hex](Text.Hex.md).

<details><summary>Behaviour and boundary notes from the source</summary>

DES and Triple DES (FIPS 46-3, SP 800-67) block ciphers with ECB, CBC and CTR over 64-bit blocks.
These ciphers exist for interoperation with legacy formats and protocols: DES
keys are far too short, and Triple DES has a 64-bit block, so neither should
protect new data. A whole block is one straight-line kernel: permutation
networks for the initial and final permutations, rotation windows for the
expansion, and combined S-box and P-permutation tables addressed through the
key schedule. Table lookups depend on data, so timing is not constant. Triple
DES is encrypt-decrypt-encrypt with three independent keys (a 24-byte key).
Every returned buffer is fresh; inputs are never retained or mutated.

</details>

# Types and constructors

<a id="type-446573436970686572"></a>

## `DesCipher`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">DesCipher</span></code></pre>

A keyed cipher: one schedule for DES, three for Triple DES (applied encrypt, decrypt, encrypt).
A schedule holds the SP tables (2048 bytes), the encryption round keys and the decryption round keys (128 bytes each).

[Source](/reference-sources/Crypto.Des.cinch) · line 45

<a id="type-4465734572726f72"></a>

## `DesError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">DesError</span></code></pre>

A rejected key, input or initialization vector.

[Source](/reference-sources/Crypto.Des.cinch) · line 28

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">DesInvalidKeySize</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Des</span>.<span class="tok-constructor">DesError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">DesInvalidInputSize</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Des</span>.<span class="tok-constructor">DesError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">DesInvalidIvSize</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Des</span>.<span class="tok-constructor">DesError</span>)</code></pre>

# Values

<a id="value-6465732d626c6f636b2d73697a65"></a>

## `des-block-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">des-block-size</span> :: <span class="tok-constructor">Nat</span></code></pre>

The block size in bytes.

[Source](/reference-sources/Crypto.Des.cinch) · line 52

<a id="value-6465732d6362632d64656372797074"></a>

## `des-cbc-decrypt`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">des-cbc-decrypt</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Des</span>.<span class="tok-constructor">DesCipher</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Des</span>.<span class="tok-constructor">DesError</span> <span class="tok-constructor">Bytes</span>))))</code></pre>

CBC decryption.

[Source](/reference-sources/Crypto.Des.cinch) · line 172

<a id="value-6465732d6362632d656e6372797074"></a>

## `des-cbc-encrypt`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">des-cbc-encrypt</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Des</span>.<span class="tok-constructor">DesCipher</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Des</span>.<span class="tok-constructor">DesError</span> <span class="tok-constructor">Bytes</span>))))</code></pre>

CBC encryption: each plaintext block is XORed with the previous ciphertext block (the IV first).

[Source](/reference-sources/Crypto.Des.cinch) · line 157

<a id="value-6465732d636970686572"></a>

## `des-cipher`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">des-cipher</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Des</span>.<span class="tok-constructor">DesError</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Des</span>.<span class="tok-constructor">DesCipher</span>))</code></pre>

Key DES with an 8-byte key (parity bits are ignored).

[Source](/reference-sources/Crypto.Des.cinch) · line 56

<a id="value-6465732d6374722d786f72"></a>

## `des-ctr-xor`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">des-ctr-xor</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Des</span>.<span class="tok-constructor">DesCipher</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Des</span>.<span class="tok-constructor">DesError</span> <span class="tok-constructor">Bytes</span>))))</code></pre>

CTR mode: XOR the input (any length) with encryptions of the counter block, which increments as a
big-endian 64-bit integer. Encryption and decryption are the same operation.

[Source](/reference-sources/Crypto.Des.cinch) · line 188

<a id="value-6465732d646563727970742d626c6f636b"></a>

## `des-decrypt-block`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">des-decrypt-block</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Des</span>.<span class="tok-constructor">DesCipher</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Des</span>.<span class="tok-constructor">DesError</span> <span class="tok-constructor">Bytes</span>)))</code></pre>

Decrypt exactly one 8-byte block.

[Source](/reference-sources/Crypto.Des.cinch) · line 128

<a id="value-6465732d646563727970742d656362"></a>

## `des-decrypt-ecb`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">des-decrypt-ecb</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Des</span>.<span class="tok-constructor">DesCipher</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Des</span>.<span class="tok-constructor">DesError</span> <span class="tok-constructor">Bytes</span>)))</code></pre>

Decrypt each block independently.

[Source](/reference-sources/Crypto.Des.cinch) · line 140

<a id="value-6465732d656e63727970742d626c6f636b"></a>

## `des-encrypt-block`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">des-encrypt-block</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Des</span>.<span class="tok-constructor">DesCipher</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Des</span>.<span class="tok-constructor">DesError</span> <span class="tok-constructor">Bytes</span>)))</code></pre>

Encrypt exactly one 8-byte block.

[Source](/reference-sources/Crypto.Des.cinch) · line 124

<a id="value-6465732d656e63727970742d656362"></a>

## `des-encrypt-ecb`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">des-encrypt-ecb</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Des</span>.<span class="tok-constructor">DesCipher</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Des</span>.<span class="tok-constructor">DesError</span> <span class="tok-constructor">Bytes</span>)))</code></pre>

Encrypt each block independently.

[Source](/reference-sources/Crypto.Des.cinch) · line 136

<a id="value-6465732d6572726f722d6d657373616765"></a>

## `des-error-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">des-error-message</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Des</span>.<span class="tok-constructor">DesError</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Render an error.

[Source](/reference-sources/Crypto.Des.cinch) · line 37

<a id="value-747269706c652d6465732d636970686572"></a>

## `triple-des-cipher`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">triple-des-cipher</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Des</span>.<span class="tok-constructor">DesError</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Des</span>.<span class="tok-constructor">DesCipher</span>))</code></pre>

Key Triple DES with a 24-byte key: three DES keys in order.

[Source](/reference-sources/Crypto.Des.cinch) · line 60

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Crypto/Des.cinch`. Source SHA-256: `35f5c4f7cb8a474bae4e438b2085d9e05fc7a3b290798826588883381dc55fd9`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
