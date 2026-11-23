# Crypto.Rc4

The RC4 stream cipher as a persistent keystream state, for legacy data only.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Crypto.Rc4`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Crypto.Rc4.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bytes](Data.Bytes.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Text.Show](Text.Show.md).

Non-embedded source closure: [Crypto.Rc4](Crypto.Rc4.md).

<details><summary>Behaviour and boundary notes from the source</summary>

The RC4 stream cipher, for interoperation with legacy formats only.
RC4's keystream is biased and the cipher is broken for confidentiality
(RFC 7465 prohibits it in TLS); it is provided to read and write old data.
A stream is an explicit state machine: the 256-byte permutation and the two
indices. Every call returns the advanced stream with its own copy of the
permutation, so earlier streams remain usable and replay the same keystream.

</details>

# Types and constructors

<a id="type-526334"></a>

## `Rc4`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Rc4</span></code></pre>

Private fields in the order listed above.

[Source](/reference-sources/Crypto.Rc4.cinch) · line 30

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Rc4</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rc4</span>.<span class="tok-constructor">Rc4</span>)))</code></pre>

<a id="type-5263344572726f72"></a>

## `Rc4Error`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Rc4Error</span></code></pre>

A key outside 1 to 256 bytes; its length.

[Source](/reference-sources/Crypto.Rc4.cinch) · line 19

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Rc4InvalidKeySize</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rc4</span>.<span class="tok-constructor">Rc4Error</span>)</code></pre>

# Values

<a id="value-7263342d6572726f722d6d657373616765"></a>

## `rc4-error-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rc4-error-message</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rc4</span>.<span class="tok-constructor">Rc4Error</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Render an error.

[Source](/reference-sources/Crypto.Rc4.cinch) · line 24

<a id="value-7263342d6e6577"></a>

## `rc4-new`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rc4-new</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rc4</span>.<span class="tok-constructor">Rc4Error</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rc4</span>.<span class="tok-constructor">Rc4</span>))</code></pre>

Key a stream with 1 to 256 key bytes (the key-scheduling algorithm).

[Source](/reference-sources/Crypto.Rc4.cinch) · line 33

<a id="value-7263342d786f72"></a>

## `rc4-xor`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rc4-xor</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rc4</span>.<span class="tok-constructor">Rc4</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rc4</span>.<span class="tok-constructor">Rc4</span>, <span class="tok-constructor">Bytes</span>)))</code></pre>

XOR the input with the next keystream bytes, returning the advanced stream and the output.

[Source](/reference-sources/Crypto.Rc4.cinch) · line 71

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Crypto/Rc4.cinch`. Source SHA-256: `625745d9d911990be9c7d9f1efb7fcc21077b0b44fe9cdaef20c5052cefbf1e7`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
