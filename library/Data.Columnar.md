# Data.Columnar

Columnar leaves over typed arrays, zipped into nested pairs that share their buffers. A length mismatch is None.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Data.Columnar`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Columnar.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Array](Data.Array.md), [Data.Core](Data.Core.md).

Non-embedded source closure: [Data.Array](Data.Array.md), [Data.Columnar](Data.Columnar.md).

<details><summary>Behaviour and boundary notes from the source</summary>

A leaf is one typed array. Zip composes any number of heterogeneous
columns as nested pairs, sharing their buffers. Length mismatch is None.

</details>

# Types and constructors

This module exports no type declarations.

# Values

<a id="value-636f6c756d6e61722d6172726179"></a>

## `columnar-array`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">columnar-array</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . (<span class="tok-constructor">Columnar</span> <span class="tok-variable">a</span> -&gt; <span class="tok-constructor">Option</span> (<span class="tok-constructor">Array</span> <span class="tok-variable">a</span>))</code></pre>

[Source](/reference-sources/Data.Columnar.cinch)

<a id="value-636f6c756d6e61722d636f6c756d6e"></a>

## `columnar-column`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">columnar-column</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . (<span class="tok-constructor">Array</span> <span class="tok-variable">a</span> -&gt; <span class="tok-constructor">Columnar</span> <span class="tok-variable">a</span>)</code></pre>

[Source](/reference-sources/Data.Columnar.cinch)

<a id="value-636f6c756d6e61722d636f6c756d6e73"></a>

## `columnar-columns`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">columnar-columns</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . (<span class="tok-constructor">Columnar</span> <span class="tok-variable">a</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

[Source](/reference-sources/Data.Columnar.cinch)

<a id="value-636f6c756d6e61722d676574"></a>

## `columnar-get`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">columnar-get</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . (<span class="tok-constructor">Columnar</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-variable">a</span>))</code></pre>

[Source](/reference-sources/Data.Columnar.cinch)

<a id="value-636f6c756d6e61722d6c656674"></a>

## `columnar-left`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">columnar-left</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span> . (<span class="tok-constructor">Columnar</span> (<span class="tok-variable">a</span>, <span class="tok-variable">b</span>) -&gt; <span class="tok-constructor">Option</span> (<span class="tok-constructor">Columnar</span> <span class="tok-variable">a</span>))</code></pre>

[Source](/reference-sources/Data.Columnar.cinch)

<a id="value-636f6c756d6e61722d7269676874"></a>

## `columnar-right`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">columnar-right</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span> . (<span class="tok-constructor">Columnar</span> (<span class="tok-variable">a</span>, <span class="tok-variable">b</span>) -&gt; <span class="tok-constructor">Option</span> (<span class="tok-constructor">Columnar</span> <span class="tok-variable">b</span>))</code></pre>

[Source](/reference-sources/Data.Columnar.cinch)

<a id="value-636f6c756d6e61722d726f7773"></a>

## `columnar-rows`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">columnar-rows</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . (<span class="tok-constructor">Columnar</span> <span class="tok-variable">a</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

[Source](/reference-sources/Data.Columnar.cinch)

<a id="value-636f6c756d6e61722d7a6970"></a>

## `columnar-zip`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">columnar-zip</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span> . (<span class="tok-constructor">Columnar</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Columnar</span> <span class="tok-variable">b</span> -&gt; <span class="tok-constructor">Option</span> (<span class="tok-constructor">Columnar</span> (<span class="tok-variable">a</span>, <span class="tok-variable">b</span>))))</code></pre>

[Source](/reference-sources/Data.Columnar.cinch)

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Data/Columnar.cinch`. Source SHA-256: `20bf82a9abeddc35eb4abd3c3c6a91d9ee66b40d18848eb40ab674552bbc6506`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
