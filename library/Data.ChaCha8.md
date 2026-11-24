# Data.ChaCha8

The ChaCha8 pseudorandom generator: four interleaved eight-round blocks per vector kernel call, byte reads and state encoding.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Data.ChaCha8`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.ChaCha8.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bytes](Data.Bytes.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Data.Wide](Data.Wide.md).

Non-embedded source closure: [Data.Bytes.Build](Data.Bytes.Build.md), [Data.ChaCha8](Data.ChaCha8.md).

<details><summary>Behaviour and boundary notes from the source</summary>

ChaCha8: a fast, seekable-by-state, cryptographically strong pseudorandom generator producing 64-bit
values from a 32-byte seed, with byte reads and a compact binary encoding of its state.

The generator runs four ChaCha blocks with eight rounds at a time, word-interleaved so every quarter
round is one <4 x i32> vector operation per word via Data.Wide SIMD; one block call computes all sixteen
output vectors in a single pass and yields thirty-two 64-bit outputs. After four calls the last four
outputs of the fourth become the next seed instead of being returned, so earlier outputs cannot be
recovered from a captured state. A state is an immutable value: drawing returns the value and the next
state, so a retained state replays exactly. The encoding records the seed and the number of outputs used
within the current reseed period, plus any bytes a partial read left unread.

</details>

# Types and constructors

<a id="type-43686143686138"></a>

## `ChaCha8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">ChaCha8</span></code></pre>

Fields in the order listed above.

[Source](/reference-sources/Data.ChaCha8.cinch) · line 45

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ChaCha8</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">U128</span> -&gt; (<span class="tok-constructor">U128</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">ChaCha8</span>.<span class="tok-constructor">ChaCha8</span>)))))))</code></pre>

<a id="type-436861436861384572726f72"></a>

## `ChaCha8Error`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">ChaCha8Error</span></code></pre>

Why a seed or an encoding was refused.

[Source](/reference-sources/Data.ChaCha8.cinch) · line 25

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ChaCha8SeedSize</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">ChaCha8</span>.<span class="tok-constructor">ChaCha8Error</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ChaCha8BadEncoding</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">ChaCha8</span>.<span class="tok-constructor">ChaCha8Error</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ChaCha8BadReadBuffer</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">ChaCha8</span>.<span class="tok-constructor">ChaCha8Error</span></code></pre>

# Values

<a id="value-636861636861382d6572726f722d6d657373616765"></a>

## `chacha8-error-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">chacha8-error-message</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">ChaCha8</span>.<span class="tok-constructor">ChaCha8Error</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

A readable description of an error.

[Source](/reference-sources/Data.ChaCha8.cinch) · line 34

<a id="value-636861636861382d6d61727368616c"></a>

## `chacha8-marshal`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">chacha8-marshal</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">ChaCha8</span>.<span class="tok-constructor">ChaCha8</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

The state's encoding: "readbuf:", a length byte and the unread bytes when there are any, then "chacha8:",
the outputs used in this reseed period (big-endian 64-bit) and the seed (four little-endian 64-bit words).

[Source](/reference-sources/Data.ChaCha8.cinch) · line 264

<a id="value-636861636861382d6e6577"></a>

## `chacha8-new`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">chacha8-new</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">ChaCha8</span>.<span class="tok-constructor">ChaCha8Error</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">ChaCha8</span>.<span class="tok-constructor">ChaCha8</span>))</code></pre>

A generator from a 32-byte seed (four little-endian 64-bit words).

[Source](/reference-sources/Data.ChaCha8.cinch) · line 197

<a id="value-636861636861382d72656164"></a>

## `chacha8-read`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">chacha8-read</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">ChaCha8</span>.<span class="tok-constructor">ChaCha8</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Bytes</span>, <span class="tok-constructor">Data</span>.<span class="tok-constructor">ChaCha8</span>.<span class="tok-constructor">ChaCha8</span>)))</code></pre>

Read n pseudorandom bytes: unread bytes first, then whole values little-endian; a final partial value keeps
its remaining bytes for the next read.

[Source](/reference-sources/Data.ChaCha8.cinch) · line 224

<a id="value-636861636861382d726573656564"></a>

## `chacha8-reseed`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">chacha8-reseed</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">ChaCha8</span>.<span class="tok-constructor">ChaCha8</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">ChaCha8</span>.<span class="tok-constructor">ChaCha8Error</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">ChaCha8</span>.<span class="tok-constructor">ChaCha8</span>)))</code></pre>

Start over from a new 32-byte seed, dropping unread bytes.

[Source](/reference-sources/Data.ChaCha8.cinch) · line 201

<a id="value-636861636861382d75696e743634"></a>

## `chacha8-uint64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">chacha8-uint64</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">ChaCha8</span>.<span class="tok-constructor">ChaCha8</span> -&gt; (<span class="tok-constructor">U64</span>, <span class="tok-constructor">Data</span>.<span class="tok-constructor">ChaCha8</span>.<span class="tok-constructor">ChaCha8</span>))</code></pre>

The next 64-bit value and the following state.

[Source](/reference-sources/Data.ChaCha8.cinch) · line 205

<a id="value-636861636861382d756e6d61727368616c"></a>

## `chacha8-unmarshal`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">chacha8-unmarshal</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">ChaCha8</span>.<span class="tok-constructor">ChaCha8Error</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">ChaCha8</span>.<span class="tok-constructor">ChaCha8</span>))</code></pre>

Restore a state from its encoding.

[Source](/reference-sources/Data.ChaCha8.cinch) · line 308

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Data/ChaCha8.cinch`. Source SHA-256: `3755261e74bff330a00b4e7bddd7117bee17cc00281a6032dd30e7ffe5e88cba`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
