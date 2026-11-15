# Text.Fnv

Incremental 32-, 64- and 128-bit FNV-1 and FNV-1a hashing with explicit wrapping arithmetic.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Text.Fnv`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.Fnv.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md).

Non-embedded source closure: [Data.Bytes.Word](Data.Bytes.Word.md), [Text.Fnv](Text.Fnv.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Deterministic 32-, 64- and 128-bit FNV-1 and FNV-1a hashing over bytes.
Overflow wraps explicitly. Update functions accept a finished hash, so chunked
and concatenated input agree. Empty input yields the offset basis, and empty
updates leave the state unchanged. Inline LLVM unrolls eight ordered steps at
a time; the recurrence stays sequential. These non-cryptographic hashes are
separate from the collection hashing vocabulary in Data.Hash.

</details>

# Types and constructors

This module exports no type declarations.

# Values

<a id="value-666e76313238"></a>

## `fnv128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">fnv128</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">U64</span>, <span class="tok-constructor">U64</span>))</code></pre>

Compute 128-bit FNV-1 with the standard offset basis, returning high word then low word.
Two U64 words preserve the full hash on every target without a wide-scalar ABI dependency.

[Source](/reference-sources/Text.Fnv.cinch) · line 208

<a id="value-666e763132382d757064617465"></a>

## `fnv128-update`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">fnv128-update</span> :: ((<span class="tok-constructor">U64</span>, <span class="tok-constructor">U64</span>) -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">U64</span>, <span class="tok-constructor">U64</span>)))</code></pre>

Continue a 128-bit FNV-1 state given as high and low words. Empty input preserves both words.

[Source](/reference-sources/Text.Fnv.cinch) · line 217

<a id="value-666e7631323861"></a>

## `fnv128a`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">fnv128a</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">U64</span>, <span class="tok-constructor">U64</span>))</code></pre>

Compute 128-bit FNV-1a, returning high word then low word.
This deterministic hash is non-cryptographic; serialization byte order is chosen separately.

[Source](/reference-sources/Text.Fnv.cinch) · line 213

<a id="value-666e76313238612d757064617465"></a>

## `fnv128a-update`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">fnv128a-update</span> :: ((<span class="tok-constructor">U64</span>, <span class="tok-constructor">U64</span>) -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">U64</span>, <span class="tok-constructor">U64</span>)))</code></pre>

Continue a high/low 128-bit FNV-1a state; arbitrary input chunking preserves the final result.

[Source](/reference-sources/Text.Fnv.cinch) · line 221

<a id="value-666e763332"></a>

## `fnv32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">fnv32</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Compute 32-bit FNV-1, multiplying before xor, with offset basis 0x811C9DC5.

[Source](/reference-sources/Text.Fnv.cinch) · line 19

<a id="value-666e7633322d757064617465"></a>

## `fnv32-update`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">fnv32-update</span> :: (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">U32</span>))</code></pre>

Continue a finished 32-bit FNV-1 hash with more bytes; an empty update preserves the hash.

[Source](/reference-sources/Text.Fnv.cinch) · line 35

<a id="value-666e76333261"></a>

## `fnv32a`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">fnv32a</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Compute 32-bit FNV-1a, xor before multiplying, with offset basis 0x811C9DC5.

[Source](/reference-sources/Text.Fnv.cinch) · line 23

<a id="value-666e763332612d757064617465"></a>

## `fnv32a-update`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">fnv32a-update</span> :: (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">U32</span>))</code></pre>

Continue a finished 32-bit FNV-1a hash with more bytes; an empty update preserves the hash.

[Source](/reference-sources/Text.Fnv.cinch) · line 39

<a id="value-666e763634"></a>

## `fnv64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">fnv64</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">U64</span>)</code></pre>

Compute 64-bit FNV-1, multiplying before xor, with offset basis 0xCBF29CE484222325.

[Source](/reference-sources/Text.Fnv.cinch) · line 27

<a id="value-666e7636342d757064617465"></a>

## `fnv64-update`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">fnv64-update</span> :: (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">U64</span>))</code></pre>

Continue a finished 64-bit FNV-1 hash with more bytes; an empty update preserves the hash.

[Source](/reference-sources/Text.Fnv.cinch) · line 43

<a id="value-666e76363461"></a>

## `fnv64a`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">fnv64a</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">U64</span>)</code></pre>

Compute 64-bit FNV-1a, xor before multiplying, with offset basis 0xCBF29CE484222325.

[Source](/reference-sources/Text.Fnv.cinch) · line 31

<a id="value-666e763634612d757064617465"></a>

## `fnv64a-update`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">fnv64a-update</span> :: (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">U64</span>))</code></pre>

Continue a finished 64-bit FNV-1a hash with more bytes; an empty update preserves the hash.

[Source](/reference-sources/Text.Fnv.cinch) · line 47

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Text/Fnv.cinch`. Source SHA-256: `54f79c14c3b528db170df4b405e0b00c111b2e7968ec892d4d5cc20cc2a71670`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
