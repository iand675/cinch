# Data.Bytes.Search

Exact first, last, containment and count queries over raw bytes with bounded SWAR traversals.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Data.Bytes.Search`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Bytes.Search.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md).

Non-embedded source closure: [Data.Bytes.Search](Data.Bytes.Search.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Exact raw-byte searching with bounded machine loops and broad-word classification.
Complete eight-byte blocks use private straight-line SWAR kernels. Scalar
fringes read only their available bytes. Public results are zero-based byte
offsets; every byte value, including malformed UTF-8, is compared exactly.
Backing owners remain live around complete private traversals.

</details>

# Types and constructors

This module exports no type declarations.

# Values

<a id="value-62797465732d636f6e7461696e732d62797465"></a>

## `bytes-contains-byte`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bytes-contains-byte</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

Test whether any exact byte value occurs, including zero and high-bit byte values.

[Source](/reference-sources/Data.Bytes.Search.cinch) · line 232

<a id="value-62797465732d636f756e742d62797465"></a>

## `bytes-count-byte`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bytes-count-byte</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">Nat</span>))</code></pre>

Count every exact matching byte using eight-lane masks and a bounded scalar fringe.

[Source](/reference-sources/Data.Bytes.Search.cinch) · line 239

<a id="value-62797465732d696e6465782d62797465"></a>

## `bytes-index-byte`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bytes-index-byte</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Nat</span>)))</code></pre>

Find the first exact byte value, returning no offset when the input contains no match.

[Source](/reference-sources/Data.Bytes.Search.cinch) · line 218

<a id="value-62797465732d6c6173742d696e6465782d62797465"></a>

## `bytes-last-index-byte`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bytes-last-index-byte</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Nat</span>)))</code></pre>

Find the last exact byte value without decoding or changing the input.

[Source](/reference-sources/Data.Bytes.Search.cinch) · line 225

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Data/Bytes/Search.cinch`. Source SHA-256: `65b8c75745399b692031aff515386ffaed14ad9f50b4e4a644a8f3ce259c5256`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
