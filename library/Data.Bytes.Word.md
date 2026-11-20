# Data.Bytes.Word

Bounds-proved unaligned little-endian machine-word reads and writes for SIMD and SWAR processing.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Data.Bytes.Word`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Bytes.Word.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bytes](Data.Bytes.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md).

Non-embedded source closure: [Data.Bytes.Word](Data.Bytes.Word.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Unaligned little-endian machine-word access to byte buffers.
Bounds are explicit preconditions in the public signatures. Reads and writes
access exactly the proved byte range. Private LLVM loads retain the backing
buffer until completion; raw pointers never enter the public interface. The
packed words feed SIMD and SWAR kernels. Wire order is independent of host
endianness. Concurrent mutation follows the same rules as Data.Bytes.

</details>

# Types and constructors

This module exports no type declarations.

# Values

<a id="value-62797465732d726561642d6c653332"></a>

## `bytes-read-le32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bytes-read-le32</span> :: <span class="tok-keyword">using</span> ((<span class="tok-variable">arg1</span> <span class="tok-operator">+</span> <span class="tok-number">4</span>) <span class="tok-operator">&lt;=</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-variable">bytes-len</span> <span class="tok-variable">arg0</span>)) =&gt; ((<span class="tok-variable">arg0</span> :: <span class="tok-constructor">Bytes</span>) -&gt; ((<span class="tok-variable">arg1</span> :: <span class="tok-constructor">Nat</span>) -&gt; <span class="tok-constructor">U32</span>))</code></pre>

Read exactly 4 bytes as a little-endian U32. Alignment is unrestricted. The caller must prove pos + 4 <= bytes-len b; no tail bytes are read.

[Source](/reference-sources/Data.Bytes.Word.cinch) · line 16

<a id="value-62797465732d726561642d6c653634"></a>

## `bytes-read-le64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bytes-read-le64</span> :: <span class="tok-keyword">using</span> ((<span class="tok-variable">arg1</span> <span class="tok-operator">+</span> <span class="tok-number">8</span>) <span class="tok-operator">&lt;=</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-variable">bytes-len</span> <span class="tok-variable">arg0</span>)) =&gt; ((<span class="tok-variable">arg0</span> :: <span class="tok-constructor">Bytes</span>) -&gt; ((<span class="tok-variable">arg1</span> :: <span class="tok-constructor">Nat</span>) -&gt; <span class="tok-constructor">U64</span>))</code></pre>

Read exactly 8 bytes as a little-endian U64. Alignment is unrestricted. The caller must prove pos + 8 <= bytes-len b; no tail bytes are read.

[Source](/reference-sources/Data.Bytes.Word.cinch) · line 28

<a id="value-62797465732d77726974652d6c653332"></a>

## `bytes-write-le32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bytes-write-le32</span> :: <span class="tok-keyword">using</span> ((<span class="tok-variable">arg1</span> <span class="tok-operator">+</span> <span class="tok-number">4</span>) <span class="tok-operator">&lt;=</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-variable">bytes-len</span> <span class="tok-variable">arg0</span>)) =&gt; ((<span class="tok-variable">arg0</span> :: <span class="tok-constructor">Bytes</span>) -&gt; ((<span class="tok-variable">arg1</span> :: <span class="tok-constructor">Nat</span>) -&gt; ((<span class="tok-variable">arg2</span> :: <span class="tok-constructor">U32</span>) -&gt; <span class="tok-constructor">Unit</span>)))</code></pre>

Write exactly 4 low-to-high bytes of a U32. The caller must prove pos + 4 <= bytes-len b; bytes outside that range are preserved.

[Source](/reference-sources/Data.Bytes.Word.cinch) · line 20

<a id="value-62797465732d77726974652d6c653634"></a>

## `bytes-write-le64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bytes-write-le64</span> :: <span class="tok-keyword">using</span> ((<span class="tok-variable">arg1</span> <span class="tok-operator">+</span> <span class="tok-number">8</span>) <span class="tok-operator">&lt;=</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-variable">bytes-len</span> <span class="tok-variable">arg0</span>)) =&gt; ((<span class="tok-variable">arg0</span> :: <span class="tok-constructor">Bytes</span>) -&gt; ((<span class="tok-variable">arg1</span> :: <span class="tok-constructor">Nat</span>) -&gt; ((<span class="tok-variable">arg2</span> :: <span class="tok-constructor">U64</span>) -&gt; <span class="tok-constructor">Unit</span>)))</code></pre>

Write exactly 8 low-to-high bytes of a U64. The caller must prove pos + 8 <= bytes-len b; bytes outside that range are preserved.

[Source](/reference-sources/Data.Bytes.Word.cinch) · line 32

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Data/Bytes/Word.cinch`. Source SHA-256: `a67d8ecfa74d6b11039d13408f3efe7e8222de631b86a4e455f0976d8104f7d8`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
