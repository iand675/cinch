# Crypto.ConstantTime

Branch-free scalar comparison and selection, full-input byte equality, conditional copying and in-place XOR with public bounds and atomic length errors.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Crypto.ConstantTime`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Crypto.ConstantTime.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Wide](Data.Wide.md).

Non-embedded source closure: [Crypto.ConstantTime](Crypto.ConstantTime.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Branch-free scalar selection and byte operations with public-length traversal.
Byte equality returns early only for unequal lengths. Conditional copies visit
every destination byte for either selection value. Buffers must not be mutated
concurrently. Whole Bytes handles may alias exactly; partial raw-pointer aliases
are outside this interface. Data-independent instruction timing is a separate
processor/runtime capability and is not enabled by these operations.

</details>

# Types and constructors

<a id="type-436f6e7374616e7454696d654572726f72"></a>

## `ConstantTimeError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">ConstantTimeError</span></code></pre>

Public buffer dimensions cannot satisfy a complete operation.

[Source](/reference-sources/Crypto.ConstantTime.cinch) · line 19

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ConstantTimeLengthMismatch</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">ConstantTime</span>.<span class="tok-constructor">ConstantTimeError</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ConstantTimeShortDestination</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">ConstantTime</span>.<span class="tok-constructor">ConstantTimeError</span>))</code></pre>

# Values

<a id="value-636f6e7374616e742d627974652d657175616c"></a>

## `constant-byte-equal`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">constant-byte-equal</span> :: (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

Compare two bytes without branching on their contents.

[Source](/reference-sources/Crypto.ConstantTime.cinch) · line 26

<a id="value-636f6e7374616e742d62797465732d657175616c"></a>

## `constant-bytes-equal`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">constant-bytes-equal</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

Compare equal-length buffers using complete 16-byte blocks and an exact scalar tail.
Every byte is read even when an earlier byte differs; lengths are public.

[Source](/reference-sources/Crypto.ConstantTime.cinch) · line 61

<a id="value-636f6e7374616e742d636f7079"></a>

## `constant-copy`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">constant-copy</span> :: (<span class="tok-constructor">Bool</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">ConstantTime</span>.<span class="tok-constructor">ConstantTimeError</span> <span class="tok-constructor">Unit</span>))))</code></pre>

Conditionally copy a complete source, preserving the destination for False.
A length mismatch is returned before any byte is written.

[Source](/reference-sources/Crypto.ConstantTime.cinch) · line 69

<a id="value-636f6e7374616e742d6933322d657175616c"></a>

## `constant-i32-equal`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">constant-i32-equal</span> :: (<span class="tok-constructor">I32</span> -&gt; (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

Compare signed 32-bit words without a content-dependent branch.

[Source](/reference-sources/Crypto.ConstantTime.cinch) · line 30

<a id="value-636f6e7374616e742d6c6573732d6f722d657175616c"></a>

## `constant-less-or-equal`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">constant-less-or-equal</span> :: ((<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">0</span> <span class="tok-number">2147483647</span> <span class="tok-constructor">I64</span>) -&gt; ((<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">0</span> <span class="tok-number">2147483647</span> <span class="tok-constructor">I64</span>) -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

Compare nonnegative 31-bit values; invalid mathematical inputs are excluded by their types.

[Source](/reference-sources/Crypto.ConstantTime.cinch) · line 34

<a id="value-636f6e7374616e742d73656c656374"></a>

## `constant-select`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">constant-select</span> :: (<span class="tok-constructor">Bool</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">I64</span>)))</code></pre>

Select the first word for True and the second for False without branching on the flag.

[Source](/reference-sources/Crypto.ConstantTime.cinch) · line 54

<a id="value-786f722d6279746573"></a>

## `xor-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">xor-bytes</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">ConstantTime</span>.<span class="tok-constructor">ConstantTimeError</span> <span class="tok-constructor">Nat</span>))))</code></pre>

XOR the shorter input into the destination, returning the complete written length.
A short destination is returned before any write; untouched trailing bytes remain unchanged.

[Source](/reference-sources/Crypto.ConstantTime.cinch) · line 83

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Crypto/ConstantTime.cinch`. Source SHA-256: `698448407f9e0ff912ec4f0b515c7787efc7475bd0584d3229dfa5c105c5b9de`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
