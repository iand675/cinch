# Text.Html

HTML text/byte escaping, complete HTML5 character references, exact escaped sizing and SIMD/SWAR scanning.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Text.Html`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.Html.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bytes](Data.Bytes.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.String](Data.String.md).

Non-embedded source closure: [Data.Bytes.Word](Data.Bytes.Word.md), [Text.Html](Text.Html.md).

<details><summary>Behaviour and boundary notes from the source</summary>

HTML text escaping and HTML5 named and numeric character references.
Escaping replaces only ampersand, both quotation marks and angle brackets.
Unescaping recognizes semicolon-terminated names, the documented legacy
semicolon-free names, decimal/hexadecimal numeric references and Windows-1252
numeric replacements. Unknown references remain unchanged. Reference expansion
is a single pass: newly decoded ampersands are not decoded again.
Byte APIs preserve unrelated arbitrary bytes; text APIs preserve valid UTF-8.
All returned byte buffers are fresh. Complete scanning blocks use LLVM SIMD;
private machine-counter loops retain their source until exact loads finish.

</details>

# Types and constructors

This module exports no type declarations.

# Values

<a id="value-68746d6c2d657363617065"></a>

## `html-escape`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">html-escape</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Escape ampersand, apostrophe, double quotation mark and angle brackets in UTF-8 text.
Apostrophes use &#39; and double quotation marks use &#34;.

[Source](/reference-sources/Text.Html.cinch) · line 226

<a id="value-68746d6c2d6573636170652d6279746573"></a>

## `html-escape-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">html-escape-bytes</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Escape arbitrary input bytes into fresh storage. Unrelated non-UTF-8 bytes are preserved exactly.

[Source](/reference-sources/Text.Html.cinch) · line 221

<a id="value-68746d6c2d657363617065642d6c656e677468"></a>

## `html-escaped-length`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">html-escaped-length</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Exact escaped byte count. Only five ASCII bytes expand; all other bytes retain their size.

[Source](/reference-sources/Text.Html.cinch) · line 125

<a id="value-68746d6c2d756e657363617065"></a>

## `html-unescape`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">html-unescape</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Decode named and numeric HTML references once, preserving unknown reference text.
Legacy names can omit a semicolon; invalid numeric scalars become U+FFFD.
Numeric accumulation wraps at 32 bits before scalar replacement.

[Source](/reference-sources/Text.Html.cinch) · line 452

<a id="value-68746d6c2d756e6573636170652d6279746573"></a>

## `html-unescape-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">html-unescape-bytes</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Decode HTML5 references into fresh arbitrary bytes, retaining unrelated bytes exactly.
Complete named references may expand: for example, &nLt; produces two scalars and six UTF-8 bytes.

[Source](/reference-sources/Text.Html.cinch) · line 446

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Text/Html.cinch`. Source SHA-256: `a22d0acc3c1f391ed0207a94984a8b6c93036239ac88f449f84324dd7ee8d478`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
