# Text.Url.Scan

Private portable SIMD byte scanning shared by URL parsing, queries and path traversal.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Text.Url.Scan`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.Url.Scan.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Wide](Data.Wide.md).

Non-embedded source closure: [Text.Url.Scan](Text.Url.Scan.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Checked byte intervals and exact-load portable 16-byte SIMD scans used by URL grammars.
Every returned interval is a fresh buffer. Searches return the interval end
when absent; malformed bounds are rejected by returning the complete input end.

</details>

# Types and constructors

This module exports no type declarations.

# Values

<a id="value-75726c2d636f6e636174"></a>

## `url-concat`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-concat</span> :: ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Bytes</span>) -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Concatenate chunks with one exact allocation and one checked copy per chunk.

[Source](/reference-sources/Text.Url.Scan.cinch) · line 39

<a id="value-75726c2d636f7079"></a>

## `url-copy`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-copy</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Copy a complete input without retaining a mutable caller alias.

[Source](/reference-sources/Text.Url.Scan.cinch) · line 15

<a id="value-75726c2d636f756e742d62797465"></a>

## `url-count-byte`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-count-byte</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Nat</span>))))</code></pre>

Count exact wanted-byte occurrences in a half-open interval; invalid bounds yield zero.

[Source](/reference-sources/Text.Url.Scan.cinch) · line 271

<a id="value-75726c2d66696e642d62797465"></a>

## `url-find-byte`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-find-byte</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Nat</span>))))</code></pre>

Find the first wanted byte in a half-open interval, or its end when absent.

[Source](/reference-sources/Text.Url.Scan.cinch) · line 253

<a id="value-75726c2d66696e642d636f6e74726f6c"></a>

## `url-find-control`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-find-control</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Nat</span>)))</code></pre>

Find the first ASCII control or DEL in a half-open interval, or its end.

[Source](/reference-sources/Text.Url.Scan.cinch) · line 257

<a id="value-75726c2d66696e642d696e76616c69642d75736572"></a>

## `url-find-invalid-user`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-find-invalid-user</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Nat</span>)))</code></pre>

Find the first unsupported raw ASCII user-information byte, or the interval end.

[Source](/reference-sources/Text.Url.Scan.cinch) · line 290

<a id="value-75726c2d6c6173742d62797465"></a>

## `url-last-byte`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-last-byte</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Nat</span>))))</code></pre>

Find the last wanted byte in a half-open interval, or its end when absent.

[Source](/reference-sources/Text.Url.Scan.cinch) · line 261

<a id="value-75726c2d736368656d652d636f6c6f6e"></a>

## `url-scheme-colon`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-scheme-colon</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Find an ASCII scheme colon with SIMD grammar scanning; return input length when absent or invalid.

[Source](/reference-sources/Text.Url.Scan.cinch) · line 279

<a id="value-75726c2d736c696365"></a>

## `url-slice`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-slice</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Bytes</span>)))</code></pre>

Copy a half-open interval, returning empty bytes for invalid bounds.

[Source](/reference-sources/Text.Url.Scan.cinch) · line 20

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Text/Url/Scan.cinch`. Source SHA-256: `e76c8881d3b3527986849dbb51ec8e21afa198bc9010545000f33198baf83d8b`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
