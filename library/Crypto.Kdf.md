# Crypto.Kdf

HKDF and PBKDF2 key derivation with checked lengths, binary inputs and HMAC-SHA-256.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Crypto.Kdf`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Crypto.Kdf.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bytes](Data.Bytes.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Text.Digest](Text.Digest.md).

Non-embedded source closure: [Crypto.Kdf](Crypto.Kdf.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Text.Digest](Text.Digest.md).

<details><summary>Behaviour and boundary notes from the source</summary>

HKDF (RFC 5869) and PBKDF2 (RFC 8018) over a fixed-width keyed PRF.
The supplied default is HMAC-SHA-256. Passwords, keys, salt and info are
arbitrary bytes. Intermediate and returned buffers use ordinary Bytes storage.

</details>

# Types and constructors

<a id="type-4b64664572726f72"></a>

## `KdfError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">KdfError</span></code></pre>

Parameter and callback failures. No partially derived key is returned.

[Source](/reference-sources/Crypto.Kdf.cinch) · line 31

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">KdfInvalidLength</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Kdf</span>.<span class="tok-constructor">KdfError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">KdfInvalidIterations</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Kdf</span>.<span class="tok-constructor">KdfError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">KdfShortKey</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Kdf</span>.<span class="tok-constructor">KdfError</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">KdfDigestSize</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Kdf</span>.<span class="tok-constructor">KdfError</span>))</code></pre>

<a id="type-4b6466507266"></a>

## `KdfPrf`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">KdfPrf</span></code></pre>

A deterministic keyed function with a nonzero, fixed output width.

[Source](/reference-sources/Crypto.Kdf.cinch) · line 18

# Values

<a id="value-686b64662d657870616e64"></a>

## `hkdf-expand`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hkdf-expand</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Kdf</span>.<span class="tok-constructor">KdfPrf</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Kdf</span>.<span class="tok-constructor">KdfError</span> <span class="tok-constructor">Bytes</span>)))))</code></pre>

Expand an extracted key using arbitrary context bytes. Output is limited
to 255 digest blocks; a zero-byte request returns a fresh empty buffer.

[Source](/reference-sources/Crypto.Kdf.cinch) · line 53

<a id="value-686b64662d65787472616374"></a>

## `hkdf-extract`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hkdf-extract</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Kdf</span>.<span class="tok-constructor">KdfPrf</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Kdf</span>.<span class="tok-constructor">KdfError</span> <span class="tok-constructor">Bytes</span>))))</code></pre>

Extract a pseudorandom key from secret input material. An empty salt
selects the RFC default of one digest-width block of zero bytes.

[Source](/reference-sources/Crypto.Kdf.cinch) · line 46

<a id="value-686b64662d6b6579"></a>

## `hkdf-key`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hkdf-key</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Kdf</span>.<span class="tok-constructor">KdfPrf</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Kdf</span>.<span class="tok-constructor">KdfError</span> <span class="tok-constructor">Bytes</span>))))))</code></pre>

Extract then expand. Reject excessive output before invoking the PRF.

[Source](/reference-sources/Crypto.Kdf.cinch) · line 69

<a id="value-6b64662d686d61632d736861323536"></a>

## `kdf-hmac-sha256`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">kdf-hmac-sha256</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Kdf</span>.<span class="tok-constructor">KdfPrf</span></code></pre>

The runtime's RFC 2104 HMAC with SHA-256.

[Source](/reference-sources/Crypto.Kdf.cinch) · line 27

<a id="value-6b64662d707266"></a>

## `kdf-prf`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">kdf-prf</span> :: (<span class="tok-constructor">Nat</span> -&gt; ((<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Bytes</span>)) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Kdf</span>.<span class="tok-constructor">KdfPrf</span>)))</code></pre>

The callback takes key then message and must return a fresh digest while
preserving its inputs. A zero output width is rejected at construction.

[Source](/reference-sources/Crypto.Kdf.cinch) · line 23

<a id="value-70626b6466322d6b6579"></a>

## `pbkdf2-key`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pbkdf2-key</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Kdf</span>.<span class="tok-constructor">KdfPrf</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Kdf</span>.<span class="tok-constructor">KdfError</span> <span class="tok-constructor">Bytes</span>))))))</code></pre>

Derive a positive-length key with at least one iteration. At most
2^32-1 digest blocks can be numbered by PBKDF2's big-endian counter.

[Source](/reference-sources/Crypto.Kdf.cinch) · line 78

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Crypto/Kdf.cinch`. Source SHA-256: `ee877d5b4b05c5425637d29b05dd35b3012f291e16bb67d159fe820484033291`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
