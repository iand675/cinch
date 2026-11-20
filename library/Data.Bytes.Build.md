# Data.Bytes.Build

Fresh byte cloning, doubling repetition, concatenation and separator-aware joining.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Data.Bytes.Build`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Bytes.Build.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.List](Data.List.md), [Data.Nat](Data.Nat.md).

Non-embedded source closure: [Data.Bytes.Build](Data.Bytes.Build.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Bulk construction of owned mutable byte buffers with exact single-allocation results.
Clone, repetition, concatenation and joining never retain caller input aliases.
Complete copies use the runtime's accelerated checked range-copy operation;
list traversal and repeated-prefix expansion use ordinary bounded-stack loops.

</details>

# Types and constructors

This module exports no type declarations.

# Values

<a id="value-62797465732d636c6f6e65"></a>

## `bytes-clone`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bytes-clone</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Copy every source byte into independent mutable storage, including empty input.

[Source](/reference-sources/Data.Bytes.Build.cinch) · line 15

<a id="value-62797465732d636f6e636174"></a>

## `bytes-concat`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bytes-concat</span> :: ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Bytes</span>) -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Concatenate raw byte pieces into one fresh buffer without recursive return frames.

[Source](/reference-sources/Data.Bytes.Build.cinch) · line 79

<a id="value-62797465732d6a6f696e"></a>

## `bytes-join`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bytes-join</span> :: ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Bytes</span>) -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Bytes</span>))</code></pre>

Join raw byte pieces in list order with separators, into one fresh exact-size buffer.
Empty pieces still contribute their adjacent separators; caller buffers remain unchanged.

[Source](/reference-sources/Data.Bytes.Build.cinch) · line 85

<a id="value-62797465732d726570656174"></a>

## `bytes-repeat`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bytes-repeat</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Bytes</span>))</code></pre>

Repeat raw bytes the requested natural count into one independent buffer.
Empty inputs and zero counts need no fill traversal; byte values are never decoded.

[Source](/reference-sources/Data.Bytes.Build.cinch) · line 32

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Data/Bytes/Build.cinch`. Source SHA-256: `6c71c2b3807a54c6f168ea5acf4aa1f2486801501487f9494e6e18a8d9449de9`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
