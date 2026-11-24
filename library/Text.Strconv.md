# Text.Strconv

Explicit-base and width-aware integer codecs, exact decimal and hexadecimal float parsing and formatting, complex parsing/formatting and Go string/rune quoting.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Text.Strconv`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.Strconv.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Char](Data.Char.md), [Data.Float](Data.Float.md), [Data.Int](Data.Int.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Data.Unicode.Kernel](Data.Unicode.Kernel.md), [Relation.Eq](Relation.Eq.md), [Text.Builder](Text.Builder.md), [Text.Show](Text.Show.md).

Non-embedded source closure: [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Float](Data.Float.md), [Data.Unicode.Kernel](Data.Unicode.Kernel.md), [Data.Unicode.Tables](Data.Unicode.Tables.md), [Text.Builder](Text.Builder.md), [Text.Strconv](Text.Strconv.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Conversions between strings, byte buffers, and basic scalar values:
booleans, signed and unsigned machine integers in bases 2 through 36 (plus
base-0 prefix and underscore syntax), binary32 and binary64 floating-point
numbers (decimal, hexadecimal, and special values with exact IEEE-754
round-to-nearest-even), complex pairs, literal quoting and unquoting, and
Unicode printable/graphic rune classification.

Every fallible conversion returns an explicit Result with a named
StrconvError rather than panicking, and clamped-pair variants expose the
saturated boundary value alongside a range error when overflow occurs.
Byte-appending functions return fresh buffers without mutating caller
prefixes.

</details>

# Types and constructors

<a id="type-537472636f6e764572726f72"></a>

## `StrconvError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">StrconvError</span></code></pre>

Why a string conversion failed.

[Source](/reference-sources/Text.Strconv.cinch) · line 55

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">StrconvSyntax</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Strconv</span>.<span class="tok-constructor">StrconvError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">StrconvRange</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Strconv</span>.<span class="tok-constructor">StrconvError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">StrconvBadBase</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Strconv</span>.<span class="tok-constructor">StrconvError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">StrconvBadBitSize</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Strconv</span>.<span class="tok-constructor">StrconvError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">StrconvBadFormat</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Strconv</span>.<span class="tok-constructor">StrconvError</span></code></pre>

# Values

<a id="value-617070656e642d626f6f6c"></a>

## `append-bool`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">append-bool</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; <span class="tok-constructor">Bytes</span>))</code></pre>

Append "true" or "false" to a byte buffer, returning a fresh buffer.

[Source](/reference-sources/Text.Strconv.cinch) · line 219

<a id="value-617070656e642d666c6f61743332"></a>

## `append-float32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">append-float32</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">F32</span> -&gt; (<span class="tok-constructor">Char</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Strconv</span>.<span class="tok-constructor">StrconvError</span> <span class="tok-constructor">Bytes</span>)))))</code></pre>

Append the formatted binary32 string to a byte buffer.

[Source](/reference-sources/Text.Strconv.cinch) · line 726

<a id="value-617070656e642d666c6f61743634"></a>

## `append-float64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">append-float64</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">F64</span> -&gt; (<span class="tok-constructor">Char</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Strconv</span>.<span class="tok-constructor">StrconvError</span> <span class="tok-constructor">Bytes</span>)))))</code></pre>

Append the formatted binary64 string to a byte buffer.

[Source](/reference-sources/Text.Strconv.cinch) · line 720

<a id="value-617070656e642d696e74"></a>

## `append-int`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">append-int</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Strconv</span>.<span class="tok-constructor">StrconvError</span> <span class="tok-constructor">Bytes</span>))))</code></pre>

Append the base-b representation of a signed 64-bit integer to a byte buffer.

[Source](/reference-sources/Text.Strconv.cinch) · line 442

<a id="value-617070656e642d71756f7465"></a>

## `append-quote`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">append-quote</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Bytes</span>))</code></pre>

Append the double-quoted literal of a string to a byte buffer.

[Source](/reference-sources/Text.Strconv.cinch) · line 1552

<a id="value-617070656e642d71756f74652d72756e65"></a>

## `append-quote-rune`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">append-quote-rune</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Char</span> -&gt; <span class="tok-constructor">Bytes</span>))</code></pre>

Append the single-quoted literal of a character to a byte buffer.

[Source](/reference-sources/Text.Strconv.cinch) · line 1583

<a id="value-617070656e642d71756f74652d72756e652d746f2d6173636969"></a>

## `append-quote-rune-to-ascii`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">append-quote-rune-to-ascii</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Char</span> -&gt; <span class="tok-constructor">Bytes</span>))</code></pre>

Append the ASCII-only single-quoted literal of a character to a byte buffer.

[Source](/reference-sources/Text.Strconv.cinch) · line 1587

<a id="value-617070656e642d71756f74652d72756e652d746f2d67726170686963"></a>

## `append-quote-rune-to-graphic`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">append-quote-rune-to-graphic</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Char</span> -&gt; <span class="tok-constructor">Bytes</span>))</code></pre>

Append the graphic-preserving single-quoted literal of a character to a byte buffer.

[Source](/reference-sources/Text.Strconv.cinch) · line 1591

<a id="value-617070656e642d71756f74652d746f2d6173636969"></a>

## `append-quote-to-ascii`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">append-quote-to-ascii</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Bytes</span>))</code></pre>

Append the ASCII-only double-quoted literal of a string to a byte buffer.

[Source](/reference-sources/Text.Strconv.cinch) · line 1556

<a id="value-617070656e642d71756f74652d746f2d67726170686963"></a>

## `append-quote-to-graphic`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">append-quote-to-graphic</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Bytes</span>))</code></pre>

Append the graphic-preserving double-quoted literal of a string to a byte buffer.

[Source](/reference-sources/Text.Strconv.cinch) · line 1560

<a id="value-617070656e642d75696e74"></a>

## `append-uint`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">append-uint</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Strconv</span>.<span class="tok-constructor">StrconvError</span> <span class="tok-constructor">Bytes</span>))))</code></pre>

Append the base-b representation of an unsigned 64-bit integer to a byte buffer.

[Source](/reference-sources/Text.Strconv.cinch) · line 451

<a id="value-61746f69"></a>

## `atoi`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">atoi</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Strconv</span>.<span class="tok-constructor">StrconvError</span> <span class="tok-constructor">I64</span>))</code></pre>

Parse a base-10 64-bit signed integer.

[Source](/reference-sources/Text.Strconv.cinch) · line 392

<a id="value-61746f692d776974682d636c616d70"></a>

## `atoi-with-clamp`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">atoi-with-clamp</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">I64</span>, (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Strconv</span>.<span class="tok-constructor">StrconvError</span>)))</code></pre>

Parse a base-10 64-bit signed integer, returning the clamped value alongside any error.

[Source](/reference-sources/Text.Strconv.cinch) · line 388

<a id="value-63616e2d6261636b71756f7465"></a>

## `can-backquote`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">can-backquote</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Report whether a string can be represented unchanged as a single-line backquoted literal without control characters other than tab.

[Source](/reference-sources/Text.Strconv.cinch) · line 1595

<a id="value-666f726d61742d626f6f6c"></a>

## `format-bool`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">format-bool</span> :: (<span class="tok-constructor">Bool</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Format a boolean as "true" or "false".

[Source](/reference-sources/Text.Strconv.cinch) · line 215

<a id="value-666f726d61742d636f6d706c6578313238"></a>

## `format-complex128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">format-complex128</span> :: ((<span class="tok-constructor">F64</span>, <span class="tok-constructor">F64</span>) -&gt; (<span class="tok-constructor">Char</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Strconv</span>.<span class="tok-constructor">StrconvError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))))</code></pre>

Format a 128-bit complex number (real, imag) as "(a+bi)".

[Source](/reference-sources/Text.Strconv.cinch) · line 1406

<a id="value-666f726d61742d636f6d706c65783634"></a>

## `format-complex64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">format-complex64</span> :: ((<span class="tok-constructor">F32</span>, <span class="tok-constructor">F32</span>) -&gt; (<span class="tok-constructor">Char</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Strconv</span>.<span class="tok-constructor">StrconvError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))))</code></pre>

Format a 64-bit complex number (real, imag) as "(a+bi)".

[Source](/reference-sources/Text.Strconv.cinch) · line 1415

<a id="value-666f726d61742d666c6f61743332"></a>

## `format-float32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">format-float32</span> :: (<span class="tok-constructor">F32</span> -&gt; (<span class="tok-constructor">Char</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Strconv</span>.<span class="tok-constructor">StrconvError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))))</code></pre>

Format a binary32 floating-point value according to format specifier and precision.

[Source](/reference-sources/Text.Strconv.cinch) · line 702

<a id="value-666f726d61742d666c6f61743634"></a>

## `format-float64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">format-float64</span> :: (<span class="tok-constructor">F64</span> -&gt; (<span class="tok-constructor">Char</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Strconv</span>.<span class="tok-constructor">StrconvError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))))</code></pre>

Format a binary64 floating-point value according to format specifier and precision.
Supported formats: 'b', 'e', 'E', 'f', 'g', 'G', 'x', 'X'. Negative precision selects shortest round-trip digits.

[Source](/reference-sources/Text.Strconv.cinch) · line 684

<a id="value-666f726d61742d696e74"></a>

## `format-int`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">format-int</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Strconv</span>.<span class="tok-constructor">StrconvError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)))</code></pre>

Format a signed 64-bit integer in the given base (2..36) using lowercase letters a..z.

[Source](/reference-sources/Text.Strconv.cinch) · line 426

<a id="value-666f726d61742d75696e74"></a>

## `format-uint`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">format-uint</span> :: (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Strconv</span>.<span class="tok-constructor">StrconvError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)))</code></pre>

Format an unsigned 64-bit integer in the given base (2..36) using lowercase letters a..z.

[Source](/reference-sources/Text.Strconv.cinch) · line 420

<a id="value-696e742d73697a65"></a>

## `int-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">int-size</span> :: <span class="tok-constructor">Nat</span></code></pre>

Size in bits of the default machine integer width used when bit size 0 is requested.

[Source](/reference-sources/Text.Strconv.cinch) · line 51

<a id="value-69732d67726170686963"></a>

## `is-graphic`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">is-graphic</span> :: (<span class="tok-constructor">Char</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Report whether a Unicode scalar is a Unicode graphic character (categories L, M, N, P, S, and Zs).

[Source](/reference-sources/Text.Strconv.cinch) · line 1432

<a id="value-69732d7072696e74"></a>

## `is-print`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">is-print</span> :: (<span class="tok-constructor">Char</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Report whether a Unicode scalar is printable (letters, marks, numbers, punctuation, symbols, and ASCII space).

[Source](/reference-sources/Text.Strconv.cinch) · line 1428

<a id="value-69746f61"></a>

## `itoa`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">itoa</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Format a signed 64-bit integer in base 10.

[Source](/reference-sources/Text.Strconv.cinch) · line 435

<a id="value-70617273652d626f6f6c"></a>

## `parse-bool`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">parse-bool</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Strconv</span>.<span class="tok-constructor">StrconvError</span> <span class="tok-constructor">Bool</span>))</code></pre>

Parse a boolean string accepting 1, t, T, TRUE, true, True, 0, f, F, FALSE, false, False.

[Source](/reference-sources/Text.Strconv.cinch) · line 208

<a id="value-70617273652d636f6d706c6578313238"></a>

## `parse-complex128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">parse-complex128</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Strconv</span>.<span class="tok-constructor">StrconvError</span> (<span class="tok-constructor">F64</span>, <span class="tok-constructor">F64</span>)))</code></pre>

Parse a 128-bit complex number of the form N, Ni, or N±Ni (optionally parenthesized).

[Source](/reference-sources/Text.Strconv.cinch) · line 1354

<a id="value-70617273652d636f6d706c65783132382d776974682d636c616d70"></a>

## `parse-complex128-with-clamp`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">parse-complex128-with-clamp</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; ((<span class="tok-constructor">F64</span>, <span class="tok-constructor">F64</span>), (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Strconv</span>.<span class="tok-constructor">StrconvError</span>)))</code></pre>

Parse a 128-bit complex number (pair of F64), returning clamped components alongside StrconvRange on overflow.

[Source](/reference-sources/Text.Strconv.cinch) · line 1326

<a id="value-70617273652d636f6d706c65783634"></a>

## `parse-complex64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">parse-complex64</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Strconv</span>.<span class="tok-constructor">StrconvError</span> (<span class="tok-constructor">F32</span>, <span class="tok-constructor">F32</span>)))</code></pre>

Parse a 64-bit complex number of the form N, Ni, or N±Ni (optionally parenthesized).

[Source](/reference-sources/Text.Strconv.cinch) · line 1388

<a id="value-70617273652d636f6d706c657836342d776974682d636c616d70"></a>

## `parse-complex64-with-clamp`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">parse-complex64-with-clamp</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; ((<span class="tok-constructor">F32</span>, <span class="tok-constructor">F32</span>), (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Strconv</span>.<span class="tok-constructor">StrconvError</span>)))</code></pre>

Parse a 64-bit complex number (pair of F32), returning clamped components alongside StrconvRange on overflow.

[Source](/reference-sources/Text.Strconv.cinch) · line 1360

<a id="value-70617273652d666c6f61743332"></a>

## `parse-float32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">parse-float32</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Strconv</span>.<span class="tok-constructor">StrconvError</span> <span class="tok-constructor">F32</span>))</code></pre>

Parse a binary32 floating-point number with exact single-rounding IEEE-754 semantics.

[Source](/reference-sources/Text.Strconv.cinch) · line 1316

<a id="value-70617273652d666c6f617433322d776974682d636c616d70"></a>

## `parse-float32-with-clamp`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">parse-float32-with-clamp</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">F32</span>, (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Strconv</span>.<span class="tok-constructor">StrconvError</span>)))</code></pre>

Parse a binary32 float, returning ±Inf alongside StrconvRange on overflow.

[Source](/reference-sources/Text.Strconv.cinch) · line 1304

<a id="value-70617273652d666c6f61743634"></a>

## `parse-float64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">parse-float64</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Strconv</span>.<span class="tok-constructor">StrconvError</span> <span class="tok-constructor">F64</span>))</code></pre>

Parse a binary64 floating-point number (decimal, hexadecimal, NaN, or ±Inf/Infinity).

[Source](/reference-sources/Text.Strconv.cinch) · line 1298

<a id="value-70617273652d666c6f617436342d776974682d636c616d70"></a>

## `parse-float64-with-clamp`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">parse-float64-with-clamp</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">F64</span>, (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Strconv</span>.<span class="tok-constructor">StrconvError</span>)))</code></pre>

Parse a binary64 float, returning ±Inf alongside StrconvRange on overflow.

[Source](/reference-sources/Text.Strconv.cinch) · line 1286

<a id="value-70617273652d696e74"></a>

## `parse-int`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">parse-int</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Strconv</span>.<span class="tok-constructor">StrconvError</span> <span class="tok-constructor">I64</span>))))</code></pre>

Parse a signed integer in the given base (0, or 2..36) and bit size (0..64).

[Source](/reference-sources/Text.Strconv.cinch) · line 382

<a id="value-70617273652d696e742d776974682d636c616d70"></a>

## `parse-int-with-clamp`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">parse-int-with-clamp</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span>, (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Strconv</span>.<span class="tok-constructor">StrconvError</span>)))))</code></pre>

Parse a signed integer, returning both the value (clamped to the bit-size minimum or maximum on overflow) and any error.

[Source](/reference-sources/Text.Strconv.cinch) · line 350

<a id="value-70617273652d75696e74"></a>

## `parse-uint`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">parse-uint</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Strconv</span>.<span class="tok-constructor">StrconvError</span> <span class="tok-constructor">U64</span>))))</code></pre>

Parse an unsigned integer in the given base (0, or 2..36) and bit size (0..64).

[Source](/reference-sources/Text.Strconv.cinch) · line 344

<a id="value-70617273652d75696e742d776974682d636c616d70"></a>

## `parse-uint-with-clamp`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">parse-uint-with-clamp</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">U64</span>, (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Strconv</span>.<span class="tok-constructor">StrconvError</span>)))))</code></pre>

Parse an unsigned integer, returning both the value (clamped to the bit-size maximum on overflow) and any error.

[Source](/reference-sources/Text.Strconv.cinch) · line 338

<a id="value-71756f7465"></a>

## `quote`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">quote</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Return a double-quoted string literal escaping control and non-printable characters.

[Source](/reference-sources/Text.Strconv.cinch) · line 1540

<a id="value-71756f74652d6279746573"></a>

## `quote-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">quote-bytes</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Return a double-quoted literal representing raw bytes, escaping control, non-printable, and invalid UTF-8 bytes.

[Source](/reference-sources/Text.Strconv.cinch) · line 1528

<a id="value-71756f74652d62797465732d746f2d6173636969"></a>

## `quote-bytes-to-ascii`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">quote-bytes-to-ascii</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Return an ASCII-only double-quoted literal representing raw bytes.

[Source](/reference-sources/Text.Strconv.cinch) · line 1532

<a id="value-71756f74652d62797465732d746f2d67726170686963"></a>

## `quote-bytes-to-graphic`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">quote-bytes-to-graphic</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Return a double-quoted literal preserving Unicode graphic characters and escaping non-graphic or invalid bytes.

[Source](/reference-sources/Text.Strconv.cinch) · line 1536

<a id="value-71756f74652d72756e65"></a>

## `quote-rune`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">quote-rune</span> :: (<span class="tok-constructor">Char</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Return a single-quoted character literal representing a Unicode scalar.

[Source](/reference-sources/Text.Strconv.cinch) · line 1571

<a id="value-71756f74652d72756e652d746f2d6173636969"></a>

## `quote-rune-to-ascii`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">quote-rune-to-ascii</span> :: (<span class="tok-constructor">Char</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Return an ASCII-only single-quoted character literal representing a Unicode scalar.

[Source](/reference-sources/Text.Strconv.cinch) · line 1575

<a id="value-71756f74652d72756e652d746f2d67726170686963"></a>

## `quote-rune-to-graphic`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">quote-rune-to-graphic</span> :: (<span class="tok-constructor">Char</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Return a single-quoted character literal preserving Unicode graphic scalars.

[Source](/reference-sources/Text.Strconv.cinch) · line 1579

<a id="value-71756f74652d746f2d6173636969"></a>

## `quote-to-ascii`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">quote-to-ascii</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Return a double-quoted ASCII-only string literal escaping non-ASCII and non-printable characters.

[Source](/reference-sources/Text.Strconv.cinch) · line 1544

<a id="value-71756f74652d746f2d67726170686963"></a>

## `quote-to-graphic`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">quote-to-graphic</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Return a double-quoted string literal preserving Unicode graphic characters and escaping non-graphic characters.

[Source](/reference-sources/Text.Strconv.cinch) · line 1548

<a id="value-71756f7465642d707265666978"></a>

## `quoted-prefix`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">quoted-prefix</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Strconv</span>.<span class="tok-constructor">StrconvError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

Return the verbatim quoted string literal at the prefix of `s`.

[Source](/reference-sources/Text.Strconv.cinch) · line 1757

<a id="value-737472636f6e762d6572726f722d6d657373616765"></a>

## `strconv-error-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">strconv-error-message</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Strconv</span>.<span class="tok-constructor">StrconvError</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Human-readable message describing a conversion error.

[Source](/reference-sources/Text.Strconv.cinch) · line 82

<a id="value-756e71756f7465"></a>

## `unquote`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unquote</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Strconv</span>.<span class="tok-constructor">StrconvError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

Interpret a single-quoted, double-quoted, or backquoted literal as a UTF-8 string.

[Source](/reference-sources/Text.Strconv.cinch) · line 1778

<a id="value-756e71756f74652d6279746573"></a>

## `unquote-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unquote-bytes</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Strconv</span>.<span class="tok-constructor">StrconvError</span> <span class="tok-constructor">Bytes</span>))</code></pre>

Interpret a single-quoted, double-quoted, or backquoted literal, returning its exact unescaped bytes.

[Source](/reference-sources/Text.Strconv.cinch) · line 1768

<a id="value-756e71756f74652d63686172"></a>

## `unquote-char`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unquote-char</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Char</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Strconv</span>.<span class="tok-constructor">StrconvError</span> (<span class="tok-constructor">Char</span>, <span class="tok-constructor">Bool</span>, <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))))</code></pre>

Decode the first character or byte escape in a quoted literal body.
Returns the decoded character, whether it requires multibyte UTF-8 encoding, and the remaining tail string.

[Source](/reference-sources/Text.Strconv.cinch) · line 1684

# Instances

These are checked exported instance entries. Law/sample-check status is separate from the existence of an instance; see [laws and optimization](../reference/12-rewrites-and-optimization.md).

| Instance | Target | Resolution |
| --- | --- | --- |
| `Text.Strconv.Eq_StrconvError` | `Relation.Eq.Eq Text.Strconv.StrconvError` | Implicit |

**`Text.Strconv.Eq_StrconvError`**: Structural equality for conversion errors.

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Text/Strconv.cinch`. Source SHA-256: `ef4be6a731b2d78ca1dfcdb09fde897400ebb27958d36ba4c7d77e7b8ec5edeb`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
