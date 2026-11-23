# Crypto.Hmac

Prepared HMAC-SHA-256/SHA-512 keys, persistent messages and full-tag verification.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Crypto.Hmac`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Crypto.Hmac.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Crypto.ConstantTime](Crypto.ConstantTime.md), [Data.Bytes](Data.Bytes.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Text.Digest](Text.Digest.md).

Non-embedded source closure: [Crypto.ConstantTime](Crypto.ConstantTime.md), [Crypto.Hmac](Crypto.Hmac.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Text.Digest](Text.Digest.md).

<details><summary>Behaviour and boundary notes from the source</summary>

RFC 2104 HMAC with SHA-256 and SHA-512. A prepared key owns its pads.
Persistent states snapshot written chunks; finalization traverses the entire
retained message. Whole-tag verification uses public-length constant-time equality.

</details>

# Types and constructors

<a id="type-486d6163416c676f726974686d"></a>

## `HmacAlgorithm`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">HmacAlgorithm</span></code></pre>

Hash algorithm used for both inner and outer digests.

[Source](/reference-sources/Crypto.Hmac.cinch) · line 18

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">HmacSha256</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hmac</span>.<span class="tok-constructor">HmacAlgorithm</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">HmacSha512</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hmac</span>.<span class="tok-constructor">HmacAlgorithm</span></code></pre>

<a id="type-486d61634b6579"></a>

## `HmacKey`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">HmacKey</span></code></pre>

An owned pair of block-sized inner and outer key pads.

[Source](/reference-sources/Crypto.Hmac.cinch) · line 23

<a id="type-486d61635374617465"></a>

## `HmacState`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">HmacState</span></code></pre>

A prepared key and privately owned message fragments, newest first.

[Source](/reference-sources/Crypto.Hmac.cinch) · line 27

# Values

<a id="value-686d61632d617070656e64"></a>

## `hmac-append`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hmac-append</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hmac</span>.<span class="tok-constructor">HmacState</span> -&gt; <span class="tok-constructor">Bytes</span>))</code></pre>

Append the tag to a copied prefix.

[Source](/reference-sources/Crypto.Hmac.cinch) · line 87

<a id="value-686d61632d626c6f636b2d73697a65"></a>

## `hmac-block-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hmac-block-size</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hmac</span>.<span class="tok-constructor">HmacAlgorithm</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Digest compression block length in bytes.

[Source](/reference-sources/Crypto.Hmac.cinch) · line 35

<a id="value-686d61632d6b6579"></a>

## `hmac-key`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hmac-key</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hmac</span>.<span class="tok-constructor">HmacAlgorithm</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hmac</span>.<span class="tok-constructor">HmacKey</span>))</code></pre>

Normalize an arbitrary key and precompute its two pads. The caller
can mutate the original key after preparation.

[Source](/reference-sources/Crypto.Hmac.cinch) · line 45

<a id="value-686d61632d7265736574"></a>

## `hmac-reset`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hmac-reset</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hmac</span>.<span class="tok-constructor">HmacState</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hmac</span>.<span class="tok-constructor">HmacState</span>)</code></pre>

Start a new empty message while retaining the prepared key.

[Source](/reference-sources/Crypto.Hmac.cinch) · line 91

<a id="value-686d61632d73697a65"></a>

## `hmac-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hmac-size</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hmac</span>.<span class="tok-constructor">HmacAlgorithm</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Full authentication tag length in bytes.

[Source](/reference-sources/Crypto.Hmac.cinch) · line 31

<a id="value-686d61632d7374617465"></a>

## `hmac-state`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hmac-state</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hmac</span>.<span class="tok-constructor">HmacKey</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hmac</span>.<span class="tok-constructor">HmacState</span>)</code></pre>

Begin a persistent message under a prepared key.

[Source](/reference-sources/Crypto.Hmac.cinch) · line 74

<a id="value-686d61632d73756d"></a>

## `hmac-sum`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hmac-sum</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hmac</span>.<span class="tok-constructor">HmacState</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Return the full tag without consuming the state.

[Source](/reference-sources/Crypto.Hmac.cinch) · line 83

<a id="value-686d61632d746167"></a>

## `hmac-tag`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hmac-tag</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hmac</span>.<span class="tok-constructor">HmacKey</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Bytes</span>))</code></pre>

Compute an independent full-width tag with a reusable prepared key.

[Source](/reference-sources/Crypto.Hmac.cinch) · line 60

<a id="value-686d61632d757064617465"></a>

## `hmac-update`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hmac-update</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hmac</span>.<span class="tok-constructor">HmacState</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hmac</span>.<span class="tok-constructor">HmacState</span>))</code></pre>

Append a snapshot of one message chunk without changing earlier states.

[Source](/reference-sources/Crypto.Hmac.cinch) · line 78

<a id="value-686d61632d766572696679"></a>

## `hmac-verify`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hmac-verify</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hmac</span>.<span class="tok-constructor">HmacKey</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Bool</span>)))</code></pre>

Verify a full tag. Unequal lengths fail; equal-length comparisons visit every byte.

[Source](/reference-sources/Crypto.Hmac.cinch) · line 70

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Crypto/Hmac.cinch`. Source SHA-256: `e585b6638ff0452e24305d5452c798253e0ba4e72c6381a1fad738f929592408`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
