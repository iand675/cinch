# Text.Crc32

Prepared reflected polynomial tables, persistent CRC-32 states and table-checked binary checkpoints.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Text.Crc32`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.Crc32.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bytes](Data.Bytes.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Text.Binary](Text.Binary.md).

Non-embedded source closure: [Data.Bytes.Word](Data.Bytes.Word.md), [Text.Binary](Text.Binary.md), [Text.Crc32](Text.Crc32.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Prepared reflected CRC-32 tables, immutable accumulators and portable binary checkpoints.

</details>

# Types and constructors

<a id="type-43726333325374617465"></a>

## `Crc32State`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Crc32State</span></code></pre>

An immutable checksum accumulator retaining its immutable prepared table.

[Source](/reference-sources/Text.Crc32.cinch) · line 104

<a id="type-437263333253746174654572726f72"></a>

## `Crc32StateError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Crc32StateError</span></code></pre>

A checkpoint's identifier, exact byte size or prepared table does not match.

[Source](/reference-sources/Text.Crc32.cinch) · line 159

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">InvalidCrc32StateIdentifier</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Crc32</span>.<span class="tok-constructor">Crc32StateError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">InvalidCrc32StateSize</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Crc32</span>.<span class="tok-constructor">Crc32StateError</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Crc32StateTableMismatch</span> :: (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Crc32</span>.<span class="tok-constructor">Crc32StateError</span>))</code></pre>

<a id="type-43726333325461626c65"></a>

## `Crc32Table`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Crc32Table</span></code></pre>

Private little-endian base entries and their IEEE fingerprint.

[Source](/reference-sources/Text.Crc32.cinch) · line 27

# Values

<a id="value-63726333322d73697a65"></a>

## `crc32-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">crc32-size</span> :: <span class="tok-constructor">Nat</span></code></pre>

Number of bytes in a CRC-32 digest.

[Source](/reference-sources/Text.Crc32.cinch) · line 23

<a id="value-63726333322d7374617465"></a>

## `crc32-state`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">crc32-state</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Crc32</span>.<span class="tok-constructor">Crc32Table</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Crc32</span>.<span class="tok-constructor">Crc32State</span>)</code></pre>

Start an accumulator at zero with the supplied prepared table.

[Source](/reference-sources/Text.Crc32.cinch) · line 109

<a id="value-63726333322d73746174652d617070656e64"></a>

## `crc32-state-append`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">crc32-state-append</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Crc32</span>.<span class="tok-constructor">Crc32State</span> -&gt; <span class="tok-constructor">Bytes</span>))</code></pre>

Append the network-order digest to a copied prefix without changing either argument.

[Source](/reference-sources/Text.Crc32.cinch) · line 144

<a id="value-63726333322d73746174652d617070656e642d73617665"></a>

## `crc32-state-append-save`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">crc32-state-append-save</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Crc32</span>.<span class="tok-constructor">Crc32State</span> -&gt; <span class="tok-constructor">Bytes</span>))</code></pre>

Append a complete checkpoint to a copied prefix, preserving the state.

[Source](/reference-sources/Text.Crc32.cinch) · line 155

<a id="value-63726333322d73746174652d626c6f636b2d73697a65"></a>

## `crc32-state-block-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">crc32-state-block-size</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Crc32</span>.<span class="tok-constructor">Crc32State</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Input block size in bytes, always one; updates accept any chunk length.

[Source](/reference-sources/Text.Crc32.cinch) · line 125

<a id="value-63726333322d73746174652d6572726f722d6d657373616765"></a>

## `crc32-state-error-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">crc32-state-error-message</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Crc32</span>.<span class="tok-constructor">Crc32StateError</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Render a checkpoint diagnostic, including complete expected and actual byte lengths.

[Source](/reference-sources/Text.Crc32.cinch) · line 168

<a id="value-63726333322d73746174652d6c6f6164"></a>

## `crc32-state-load`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">crc32-state-load</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Crc32</span>.<span class="tok-constructor">Crc32Table</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Crc32</span>.<span class="tok-constructor">Crc32StateError</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Crc32</span>.<span class="tok-constructor">Crc32State</span>)))</code></pre>

Restore a checkpoint against the supplied immutable table without retaining input bytes.
Identifier checks precede exact-size checks, which precede table matching. Every checksum word is accepted.

[Source](/reference-sources/Text.Crc32.cinch) · line 176

<a id="value-63726333322d73746174652d7265736574"></a>

## `crc32-state-reset`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">crc32-state-reset</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Crc32</span>.<span class="tok-constructor">Crc32State</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Crc32</span>.<span class="tok-constructor">Crc32State</span>)</code></pre>

Return a zero accumulator retaining the same prepared polynomial table.

[Source](/reference-sources/Text.Crc32.cinch) · line 117

<a id="value-63726333322d73746174652d73617665"></a>

## `crc32-state-save`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">crc32-state-save</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Crc32</span>.<span class="tok-constructor">Crc32State</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Return a twelve-byte checkpoint: ASCII crc, version 1, table fingerprint and checksum.
Both words use network order. The fingerprint identifies the complete base table.

[Source](/reference-sources/Text.Crc32.cinch) · line 149

<a id="value-63726333322d73746174652d73697a65"></a>

## `crc32-state-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">crc32-state-size</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Crc32</span>.<span class="tok-constructor">Crc32State</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Digest size in bytes, always four.

[Source](/reference-sources/Text.Crc32.cinch) · line 121

<a id="value-63726333322d73746174652d73756d"></a>

## `crc32-state-sum`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">crc32-state-sum</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Crc32</span>.<span class="tok-constructor">Crc32State</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Return four fresh digest bytes in network order without changing the state.

[Source](/reference-sources/Text.Crc32.cinch) · line 133

<a id="value-63726333322d73746174652d757064617465"></a>

## `crc32-state-update`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">crc32-state-update</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Crc32</span>.<span class="tok-constructor">Crc32State</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Crc32</span>.<span class="tok-constructor">Crc32State</span>))</code></pre>

Consume a chunk without changing any earlier state, input buffer or table.

[Source](/reference-sources/Text.Crc32.cinch) · line 113

<a id="value-63726333322d73746174652d776f7264"></a>

## `crc32-state-word`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">crc32-state-word</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Crc32</span>.<span class="tok-constructor">Crc32State</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Observe the complete numeric checksum without advancing or resetting.

[Source](/reference-sources/Text.Crc32.cinch) · line 129

<a id="value-63726333322d7461626c65"></a>

## `crc32-table`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">crc32-table</span> :: (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Crc32</span>.<span class="tok-constructor">Crc32Table</span>)</code></pre>

Prepare all 256 entries for any reflected polynomial.

[Source](/reference-sources/Text.Crc32.cinch) · line 31

<a id="value-63726333322d7461626c652d656e7472696573"></a>

## `crc32-table-entries`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">crc32-table-entries</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Crc32</span>.<span class="tok-constructor">Crc32Table</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">U32</span>))</code></pre>

Return all 256 base entries in index order without exposing the owned backing bytes.

[Source](/reference-sources/Text.Crc32.cinch) · line 73

<a id="value-63726333322d7461626c652d656e747279"></a>

## `crc32-table-entry`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">crc32-table-entry</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Crc32</span>.<span class="tok-constructor">Crc32Table</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">U32</span>)))</code></pre>

Observe a base entry at an index in 0..255; other indices return None.

[Source](/reference-sources/Text.Crc32.cinch) · line 66

<a id="value-63726333322d7461626c652d73756d"></a>

## `crc32-table-sum`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">crc32-table-sum</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Crc32</span>.<span class="tok-constructor">Crc32Table</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">U32</span>))</code></pre>

Checksum all bytes from the initial zero state with a reusable prepared table.

[Source](/reference-sources/Text.Crc32.cinch) · line 85

<a id="value-63726333322d7461626c652d757064617465"></a>

## `crc32-table-update`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">crc32-table-update</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Crc32</span>.<span class="tok-constructor">Crc32Table</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">U32</span>)))</code></pre>

Continue any finished checksum using a prepared polynomial table.
Empty input preserves every supplied bit; inputs and table storage are never modified.

[Source](/reference-sources/Text.Crc32.cinch) · line 90

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Text/Crc32.cinch`. Source SHA-256: `10edb92937712606bbe37c1a25e9adc63f8847507cd968ad3a28f553c0baf4bc`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
