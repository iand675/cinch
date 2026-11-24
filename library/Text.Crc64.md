# Text.Crc64

Immutable SIMD-built polynomial tables, slicing-by-eight CRC-64, persistent states, network-order digests and table-checked checkpoints.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Text.Crc64`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.Crc64.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bytes](Data.Bytes.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Data.Wide](Data.Wide.md), [Text.Binary](Text.Binary.md).

Non-embedded source closure: [Data.Bytes.Word](Data.Bytes.Word.md), [Text.Binary](Text.Binary.md), [Text.Crc64](Text.Crc64.md).

<details><summary>Behaviour and boundary notes from the source</summary>

CRC-64 polynomial tables, persistent accumulators and binary checkpoints.
Tables own immutable slicing-by-eight storage; construction generates pairs
of entries with 128-bit SIMD lanes. Updates combine eight packed input bytes per
step and handle exact tails. Digest and checkpoint words use network order.
Retaining a state branches the computation. This checksum does not authenticate data.

</details>

# Types and constructors

<a id="type-43726336345374617465"></a>

## `Crc64State`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Crc64State</span></code></pre>

An immutable checksum accumulator retaining its immutable prepared table.

[Source](/reference-sources/Text.Crc64.cinch) · line 136

<a id="type-437263363453746174654572726f72"></a>

## `Crc64StateError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Crc64StateError</span></code></pre>

A checkpoint's identifier, exact byte size or prepared table does not match.

[Source](/reference-sources/Text.Crc64.cinch) · line 191

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">InvalidCrc64StateIdentifier</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Crc64</span>.<span class="tok-constructor">Crc64StateError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">InvalidCrc64StateSize</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Crc64</span>.<span class="tok-constructor">Crc64StateError</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Crc64StateTableMismatch</span> :: (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Crc64</span>.<span class="tok-constructor">Crc64StateError</span>))</code></pre>

<a id="type-43726336345461626c65"></a>

## `Crc64Table`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Crc64Table</span></code></pre>

An immutable polynomial table with eight derived slices and a checkpoint fingerprint.
Retain and reuse a prepared table across updates to amortize its construction.

[Source](/reference-sources/Text.Crc64.cinch) · line 32

# Values

<a id="value-63726336342d65636d61"></a>

## `crc64-ecma`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">crc64-ecma</span> :: <span class="tok-constructor">U64</span></code></pre>

The reflected ECMA CRC-64 polynomial, 0xC96C5795D7870F42.

[Source](/reference-sources/Text.Crc64.cinch) · line 229

<a id="value-63726336342d69736f"></a>

## `crc64-iso`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">crc64-iso</span> :: <span class="tok-constructor">U64</span></code></pre>

The reflected ISO CRC-64 polynomial, 0xD800000000000000.

[Source](/reference-sources/Text.Crc64.cinch) · line 225

<a id="value-63726336342d73697a65"></a>

## `crc64-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">crc64-size</span> :: <span class="tok-constructor">Nat</span></code></pre>

Digest size in bytes, always eight.

[Source](/reference-sources/Text.Crc64.cinch) · line 27

<a id="value-63726336342d7374617465"></a>

## `crc64-state`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">crc64-state</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Crc64</span>.<span class="tok-constructor">Crc64Table</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Crc64</span>.<span class="tok-constructor">Crc64State</span>)</code></pre>

Start an accumulator at zero with the supplied prepared table.

[Source](/reference-sources/Text.Crc64.cinch) · line 141

<a id="value-63726336342d73746174652d617070656e64"></a>

## `crc64-state-append`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">crc64-state-append</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Crc64</span>.<span class="tok-constructor">Crc64State</span> -&gt; <span class="tok-constructor">Bytes</span>))</code></pre>

Append the network-order digest to a copied prefix without changing either argument.

[Source](/reference-sources/Text.Crc64.cinch) · line 176

<a id="value-63726336342d73746174652d617070656e642d73617665"></a>

## `crc64-state-append-save`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">crc64-state-append-save</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Crc64</span>.<span class="tok-constructor">Crc64State</span> -&gt; <span class="tok-constructor">Bytes</span>))</code></pre>

Append a complete checkpoint to a copied prefix, preserving the state.

[Source](/reference-sources/Text.Crc64.cinch) · line 187

<a id="value-63726336342d73746174652d626c6f636b2d73697a65"></a>

## `crc64-state-block-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">crc64-state-block-size</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Crc64</span>.<span class="tok-constructor">Crc64State</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Input block size in bytes, always one; updates accept any chunk length.

[Source](/reference-sources/Text.Crc64.cinch) · line 157

<a id="value-63726336342d73746174652d6572726f722d6d657373616765"></a>

## `crc64-state-error-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">crc64-state-error-message</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Crc64</span>.<span class="tok-constructor">Crc64StateError</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Render a checkpoint diagnostic, including complete expected and actual byte lengths.

[Source](/reference-sources/Text.Crc64.cinch) · line 200

<a id="value-63726336342d73746174652d6c6f6164"></a>

## `crc64-state-load`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">crc64-state-load</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Crc64</span>.<span class="tok-constructor">Crc64Table</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Crc64</span>.<span class="tok-constructor">Crc64StateError</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Crc64</span>.<span class="tok-constructor">Crc64State</span>)))</code></pre>

Restore a checkpoint against the supplied immutable table without retaining input bytes.
Identifier checks precede exact-size checks, which precede table matching. Every checksum word is accepted.

[Source](/reference-sources/Text.Crc64.cinch) · line 208

<a id="value-63726336342d73746174652d7265736574"></a>

## `crc64-state-reset`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">crc64-state-reset</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Crc64</span>.<span class="tok-constructor">Crc64State</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Crc64</span>.<span class="tok-constructor">Crc64State</span>)</code></pre>

Return a zero accumulator retaining the same prepared polynomial table.

[Source](/reference-sources/Text.Crc64.cinch) · line 149

<a id="value-63726336342d73746174652d73617665"></a>

## `crc64-state-save`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">crc64-state-save</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Crc64</span>.<span class="tok-constructor">Crc64State</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Return a twenty-byte checkpoint: ASCII crc, version 2, table fingerprint and checksum.
Both words use network order. The fingerprint identifies the complete base table.

[Source](/reference-sources/Text.Crc64.cinch) · line 181

<a id="value-63726336342d73746174652d73697a65"></a>

## `crc64-state-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">crc64-state-size</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Crc64</span>.<span class="tok-constructor">Crc64State</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Digest size in bytes, always eight.

[Source](/reference-sources/Text.Crc64.cinch) · line 153

<a id="value-63726336342d73746174652d73756d"></a>

## `crc64-state-sum`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">crc64-state-sum</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Crc64</span>.<span class="tok-constructor">Crc64State</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Return eight fresh digest bytes in network order without changing the state.

[Source](/reference-sources/Text.Crc64.cinch) · line 165

<a id="value-63726336342d73746174652d757064617465"></a>

## `crc64-state-update`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">crc64-state-update</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Crc64</span>.<span class="tok-constructor">Crc64State</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Crc64</span>.<span class="tok-constructor">Crc64State</span>))</code></pre>

Consume a chunk without changing any earlier state, input buffer or table.

[Source](/reference-sources/Text.Crc64.cinch) · line 145

<a id="value-63726336342d73746174652d776f7264"></a>

## `crc64-state-word`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">crc64-state-word</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Crc64</span>.<span class="tok-constructor">Crc64State</span> -&gt; <span class="tok-constructor">U64</span>)</code></pre>

Observe the complete numeric checksum without advancing or resetting.

[Source](/reference-sources/Text.Crc64.cinch) · line 161

<a id="value-63726336342d73756d"></a>

## `crc64-sum`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">crc64-sum</span> :: (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">U64</span>))</code></pre>

Compute CRC-64 with the supplied reflected polynomial. The empty checksum is zero.

[Source](/reference-sources/Text.Crc64.cinch) · line 233

<a id="value-63726336342d7461626c65"></a>

## `crc64-table`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">crc64-table</span> :: (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Crc64</span>.<span class="tok-constructor">Crc64Table</span>)</code></pre>

Prepare all 256 polynomial entries and seven derived slices in fresh owned storage.
The argument is a reflected polynomial; every U64 value is supported.

[Source](/reference-sources/Text.Crc64.cinch) · line 38

<a id="value-63726336342d7461626c652d656e7472696573"></a>

## `crc64-table-entries`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">crc64-table-entries</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Crc64</span>.<span class="tok-constructor">Crc64Table</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">U64</span>))</code></pre>

Return all 256 base entries in index order without exposing the owned backing bytes.

[Source](/reference-sources/Text.Crc64.cinch) · line 88

<a id="value-63726336342d7461626c652d656e747279"></a>

## `crc64-table-entry`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">crc64-table-entry</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Crc64</span>.<span class="tok-constructor">Crc64Table</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">U64</span>)))</code></pre>

Observe a base entry at an index in 0..255; other indices return None.

[Source](/reference-sources/Text.Crc64.cinch) · line 81

<a id="value-63726336342d7461626c652d73756d"></a>

## `crc64-table-sum`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">crc64-table-sum</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Crc64</span>.<span class="tok-constructor">Crc64Table</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">U64</span>))</code></pre>

Checksum all bytes from the initial zero state with a reusable prepared table.

[Source](/reference-sources/Text.Crc64.cinch) · line 100

<a id="value-63726336342d7461626c652d757064617465"></a>

## `crc64-table-update`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">crc64-table-update</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Crc64</span>.<span class="tok-constructor">Crc64Table</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">U64</span>)))</code></pre>

Continue any finished checksum using a prepared polynomial table.
Empty input preserves every supplied bit; inputs and table storage are never modified.

[Source](/reference-sources/Text.Crc64.cinch) · line 105

<a id="value-63726336342d757064617465"></a>

## `crc64-update`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">crc64-update</span> :: (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">U64</span>)))</code></pre>

Continue a finished CRC-64 with a reflected polynomial, preserving empty updates.
Chunks of at least 16384 bytes prepare a slicing table; shorter chunks use
the direct shift register. Reuse crc64-table-update to amortize table setup.

[Source](/reference-sources/Text.Crc64.cinch) · line 239

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Text/Crc64.cinch`. Source SHA-256: `ac34354165d37a111c795371ad88980c200e4c658ae19678c8d3cf1615ad3234`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
