# Text.Lzw

Variable-width-code LZW compression and decompression in either bit order, as used by GIF, TIFF and PDF.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Text.Lzw`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.Lzw.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bytes](Data.Bytes.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md).

Non-embedded source closure: [Data.Bytes.Build](Data.Bytes.Build.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Text.Lzw](Text.Lzw.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Lempel-Ziv-Welch compression with variable-width codes, as used by GIF, TIFF and PDF.
Codes start one bit wider than the literal width (2 to 8 bits) and grow to
twelve bits. Code 2^w clears the dictionary and code 2^w + 1 ends the data.
Compression always begins with a clear code and emits another clear code
whenever all 4096 codes are in use. Codes are packed least-significant bit
first (GIF) or most-significant bit first (TIFF and PDF); the "early change"
variant some TIFF writers use is not supported. Dictionaries live in private
byte tables, so compression is a single hashed pass and decompression
expands each code directly into its final output position.

</details>

# Types and constructors

<a id="type-4c7a77456e64"></a>

## `LzwEnd`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">LzwEnd</span></code></pre>

How a bounded decompression stopped.

[Source](/reference-sources/Text.Lzw.cinch) · line 239

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">LzwEnded</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Lzw</span>.<span class="tok-constructor">LzwEnd</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">LzwExhausted</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Lzw</span>.<span class="tok-constructor">LzwEnd</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">LzwOverflow</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Lzw</span>.<span class="tok-constructor">LzwEnd</span></code></pre>

<a id="type-4c7a774572726f72"></a>

## `LzwError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">LzwError</span></code></pre>

A rejected parameter or malformed compressed stream.

[Source](/reference-sources/Text.Lzw.cinch) · line 33

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">InvalidLiteralWidth</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Lzw</span>.<span class="tok-constructor">LzwError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">LiteralTooLarge</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Lzw</span>.<span class="tok-constructor">LzwError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">InvalidLzwCode</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Lzw</span>.<span class="tok-constructor">LzwError</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">UnexpectedLzwEnd</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Lzw</span>.<span class="tok-constructor">LzwError</span></code></pre>

<a id="type-4c7a774f72646572"></a>

## `LzwOrder`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">LzwOrder</span></code></pre>

Bit order used to pack codes into bytes.

[Source](/reference-sources/Text.Lzw.cinch) · line 26

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">LzwLsb</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Lzw</span>.<span class="tok-constructor">LzwOrder</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">LzwMsb</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Lzw</span>.<span class="tok-constructor">LzwOrder</span></code></pre>

# Values

<a id="value-6c7a772d636f6d7072657373"></a>

## `lzw-compress`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">lzw-compress</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Lzw</span>.<span class="tok-constructor">LzwOrder</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Lzw</span>.<span class="tok-constructor">LzwError</span> <span class="tok-constructor">Bytes</span>))))</code></pre>

Compress a complete input. Every byte must be below 2^width.

[Source](/reference-sources/Text.Lzw.cinch) · line 116

<a id="value-6c7a772d6465636f6d7072657373"></a>

## `lzw-decompress`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">lzw-decompress</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Lzw</span>.<span class="tok-constructor">LzwOrder</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Lzw</span>.<span class="tok-constructor">LzwError</span> <span class="tok-constructor">Bytes</span>))))</code></pre>

Decompress a complete stream through its end-of-data code.

[Source](/reference-sources/Text.Lzw.cinch) · line 231

<a id="value-6c7a772d6465636f6d70726573732d626f756e646564"></a>

## `lzw-decompress-bounded`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">lzw-decompress-bounded</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Lzw</span>.<span class="tok-constructor">LzwOrder</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Lzw</span>.<span class="tok-constructor">LzwError</span> (<span class="tok-constructor">Bytes</span>, <span class="tok-constructor">Text</span>.<span class="tok-constructor">Lzw</span>.<span class="tok-constructor">LzwEnd</span>))))))</code></pre>

Decompress until the end-of-data code or until limit bytes have been produced, whichever comes first.
After the limit only clear codes are consumed: a data code stops with LzwOverflow, the end of the input
with LzwExhausted. Running out of input before the limit is still UnexpectedLzwEnd.

[Source](/reference-sources/Text.Lzw.cinch) · line 250

<a id="value-6c7a772d6572726f722d6d657373616765"></a>

## `lzw-error-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">lzw-error-message</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Lzw</span>.<span class="tok-constructor">LzwError</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Render an LZW error.

[Source](/reference-sources/Text.Lzw.cinch) · line 44

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Text/Lzw.cinch`. Source SHA-256: `afc9d6247c42c1a0e7acd3cedbd0febb7fdfe91b87de5092078a9d731ec734c4`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
