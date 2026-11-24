# Text.Gob

Wire-compatible Go gob binary encoder and decoder for primitives, slices, arrays, maps and structs with stateful type-descriptor reuse.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Text.Gob`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.Gob.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Data.Int](Data.Int.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Text.Show](Text.Show.md).

Non-embedded source closure: [Data.Bytes.Build](Data.Bytes.Build.md), [Text.Gob](Text.Gob.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Wire-compatible gob binary encoder and decoder over typed descriptors and values.
Implements the self-describing gob binary stream format: variable-width big-endian
unsigned integers, zigzag signed integers, byte-reversed binary64 floats, complex128
pairs, length-prefixed byte slices and UTF-8 strings, slices, fixed-length arrays,
maps, and delta-indexed structs with omitted zero fields. Stateful encoders and
decoders retain transmitted user type IDs across multi-message streams so repeated
descriptors are transmitted once. All decoding is bounds-checked and depth-bounded.

</details>

# Types and constructors

<a id="type-476f624465636f6465725374617465"></a>

## `GobDecoderState`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">GobDecoderState</span></code></pre>

Persistent decoder state mapping transmitted user type IDs to wire descriptors.

[Source](/reference-sources/Text.Gob.cinch) · line 1026

<a id="type-476f62456e636f6465725374617465"></a>

## `GobEncoderState`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">GobEncoderState</span></code></pre>

Persistent encoder state tracking already-transmitted user types and the next ID.

[Source](/reference-sources/Text.Gob.cinch) · line 619

<a id="type-476f624572726f72"></a>

## `GobError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">GobError</span></code></pre>

Named failures returned when encoding or decoding gob wire streams.

[Source](/reference-sources/Text.Gob.cinch) · line 126

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">GobTruncated</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">GobOverflow</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">GobUnknownTypeId</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">GobMalformedWire</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">GobTypeMismatch</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">GobDepthLimit</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobError</span></code></pre>

<a id="type-476f624669656c645479706573"></a>

## `GobFieldTypes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">GobFieldTypes</span></code></pre>

Field descriptors of a gob struct type, kept as a direct recursive spine.

[Source](/reference-sources/Text.Gob.cinch) · line 48

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">GobFieldTypesEnd</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobFieldTypes</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">GobFieldTypeCons</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobType</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobFieldTypes</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobFieldTypes</span>)))</code></pre>

<a id="type-476f624d6170456e7472696573"></a>

## `GobMapEntries`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">GobMapEntries</span></code></pre>

Key-value entries of a gob map, kept as a direct recursive spine.

[Source](/reference-sources/Text.Gob.cinch) · line 87

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">GobMapEnd</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobMapEntries</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">GobMapCons</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobValue</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobValue</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobMapEntries</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobMapEntries</span>)))</code></pre>

<a id="type-476f625374727563744669656c6473"></a>

## `GobStructFields`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">GobStructFields</span></code></pre>

Named field values of a gob struct, kept as a direct recursive spine.

[Source](/reference-sources/Text.Gob.cinch) · line 94

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">GobFieldEnd</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobStructFields</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">GobFieldCons</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobValue</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobStructFields</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobStructFields</span>)))</code></pre>

<a id="type-476f6254797065"></a>

## `GobType`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">GobType</span></code></pre>

A gob wire type descriptor.

[Source](/reference-sources/Text.Gob.cinch) · line 55

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">GobTBool</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobType</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">GobTInt</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobType</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">GobTUint</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobType</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">GobTFloat</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobType</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">GobTBytes</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobType</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">GobTString</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobType</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">GobTComplex</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobType</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">GobTSlice</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobType</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobType</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">GobTArray</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobType</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobType</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">GobTMap</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobType</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobType</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobType</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">GobTStruct</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobFieldTypes</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobType</span>))</code></pre>

<a id="type-476f6256616c7565"></a>

## `GobValue`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">GobValue</span></code></pre>

A typed gob value corresponding to a GobType descriptor.

[Source](/reference-sources/Text.Gob.cinch) · line 101

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">GobBool</span> :: (<span class="tok-constructor">Bool</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobValue</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">GobInt</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobValue</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">GobUint</span> :: (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobValue</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">GobFloat</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobValue</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">GobBytes</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobValue</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">GobString</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobValue</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">GobComplex</span> :: (<span class="tok-constructor">F64</span> -&gt; (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobValue</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">GobSlice</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobValueList</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobValue</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">GobArray</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobValueList</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobValue</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">GobMap</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobMapEntries</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobValue</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">GobStruct</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobStructFields</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobValue</span>))</code></pre>

<a id="type-476f6256616c75654c697374"></a>

## `GobValueList`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">GobValueList</span></code></pre>

Elements of a gob slice or fixed array, kept as a direct recursive spine.

[Source](/reference-sources/Text.Gob.cinch) · line 80

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">GobValueEnd</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobValueList</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">GobValueCons</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobValue</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobValueList</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobValueList</span>))</code></pre>

# Values

<a id="value-676f622d6172726179"></a>

## `gob-array`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gob-array</span> :: ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobValue</span>) -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobValue</span>)</code></pre>

Construct a gob fixed-length array value from a list of elements.

[Source](/reference-sources/Text.Gob.cinch) · line 242

<a id="value-676f622d6465636f6465"></a>

## `gob-decode`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gob-decode</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobError</span> (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobType</span>, <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobValue</span>)))</code></pre>

Decode a single gob value occupying the entire input buffer.

[Source](/reference-sources/Text.Gob.cinch) · line 1588

<a id="value-676f622d6465636f64652d6173"></a>

## `gob-decode-as`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gob-decode-as</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobType</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobError</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobValue</span>)))</code></pre>

Decode a single gob value and verify that its wire type matches the expected GobType.

[Source](/reference-sources/Text.Gob.cinch) · line 1597

<a id="value-676f622d6465636f64652d666c6f6174"></a>

## `gob-decode-float`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gob-decode-float</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobError</span> (<span class="tok-constructor">F64</span>, <span class="tok-constructor">Nat</span>))))</code></pre>

Decode a binary64 float from a gob unsigned integer at byte offset pos.

[Source](/reference-sources/Text.Gob.cinch) · line 543

<a id="value-676f622d6465636f64652d696e74"></a>

## `gob-decode-int`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gob-decode-int</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobError</span> (<span class="tok-constructor">I64</span>, <span class="tok-constructor">Nat</span>))))</code></pre>

Decode a gob signed 64-bit integer at byte offset pos, returning (value, next-offset).

[Source](/reference-sources/Text.Gob.cinch) · line 529

<a id="value-676f622d6465636f64652d73747265616d"></a>

## `gob-decode-stream`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gob-decode-stream</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobType</span>, <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobValue</span>))))</code></pre>

Decode every value from a multi-message gob byte buffer.

[Source](/reference-sources/Text.Gob.cinch) · line 1630

<a id="value-676f622d6465636f64652d75696e74"></a>

## `gob-decode-uint`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gob-decode-uint</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobError</span> (<span class="tok-constructor">U64</span>, <span class="tok-constructor">Nat</span>))))</code></pre>

Decode a gob unsigned 64-bit integer at byte offset pos, returning (value, next-offset).

[Source](/reference-sources/Text.Gob.cinch) · line 504

<a id="value-676f622d6465636f6465722d6465636f64652d6174"></a>

## `gob-decoder-decode-at`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gob-decoder-decode-at</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobDecoderState</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobError</span> (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobType</span>, <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobValue</span>, <span class="tok-constructor">Nat</span>, <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobDecoderState</span>)))))</code></pre>

Decode one gob value starting at byte offset pos, consuming any preceding type definitions.

[Source](/reference-sources/Text.Gob.cinch) · line 1554

<a id="value-676f622d6465636f6465722d6465636f64652d6d657373616765"></a>

## `gob-decoder-decode-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gob-decoder-decode-message</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobDecoderState</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobError</span> ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobType</span>, <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobValue</span>)), <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobDecoderState</span>))))</code></pre>

Decode one already-deframed gob message payload (after its unsigned byte length prefix).
Returns None for a type-definition message (updating state) or Some (type, value) for a value message.

[Source](/reference-sources/Text.Gob.cinch) · line 1502

<a id="value-676f622d6465636f6465722d6e6577"></a>

## `gob-decoder-new`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gob-decoder-new</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobDecoderState</span></code></pre>

Create a fresh gob decoder state with no user type definitions yet received.

[Source](/reference-sources/Text.Gob.cinch) · line 1031

<a id="value-676f622d6465636f6465722d74797065"></a>

## `gob-decoder-type`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gob-decoder-type</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobDecoderState</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobError</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobType</span>)))</code></pre>

Look up and resolve a wire type ID in a GobDecoderState.

[Source](/reference-sources/Text.Gob.cinch) · line 1355

<a id="value-676f622d656e636f6465"></a>

## `gob-encode`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gob-encode</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobType</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobValue</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobError</span> <span class="tok-constructor">Bytes</span>)))</code></pre>

Encode a single typed value into a complete self-contained gob message buffer.

[Source](/reference-sources/Text.Gob.cinch) · line 1581

<a id="value-676f622d656e636f64652d666c6f6174"></a>

## `gob-encode-float`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gob-encode-float</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Encode a binary64 float using byte-reversed IEEE-754 bits as a gob unsigned integer.

[Source](/reference-sources/Text.Gob.cinch) · line 539

<a id="value-676f622d656e636f64652d696e74"></a>

## `gob-encode-int`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gob-encode-int</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Encode a signed 64-bit integer using gob zigzag encoding.

[Source](/reference-sources/Text.Gob.cinch) · line 520

<a id="value-676f622d656e636f64652d73747265616d"></a>

## `gob-encode-stream`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gob-encode-stream</span> :: ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobType</span>, <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobValue</span>)) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobError</span> <span class="tok-constructor">Bytes</span>))</code></pre>

Encode a sequence of (GobType, GobValue) pairs into one gob stream sharing type IDs.

[Source](/reference-sources/Text.Gob.cinch) · line 1616

<a id="value-676f622d656e636f64652d75696e74"></a>

## `gob-encode-uint`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gob-encode-uint</span> :: (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Encode an unsigned 64-bit integer in gob wire format.
Values 0..127 emit one byte; larger values emit (256 - width) followed by big-endian bytes.

[Source](/reference-sources/Text.Gob.cinch) · line 479

<a id="value-676f622d656e636f6465722d656e636f6465"></a>

## `gob-encoder-encode`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gob-encoder-encode</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobEncoderState</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobType</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobValue</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobError</span> (<span class="tok-constructor">Bytes</span>, <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobEncoderState</span>)))))</code></pre>

Encode one value using persistent encoder state, transmitting any unseen user type descriptors.
Returns the wire bytes for this value (preceded by any new type definitions) and updated state.

[Source](/reference-sources/Text.Gob.cinch) · line 988

<a id="value-676f622d656e636f6465722d6e6577"></a>

## `gob-encoder-new`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gob-encoder-new</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobEncoderState</span></code></pre>

Create a fresh gob encoder state with no transmitted user types.

[Source](/reference-sources/Text.Gob.cinch) · line 624

<a id="value-676f622d656e636f6465722d747970652d6964"></a>

## `gob-encoder-type-id`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gob-encoder-type-id</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobEncoderState</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobType</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">I64</span>)))</code></pre>

Query the wire type ID of a GobType if it is builtin or already transmitted.

[Source](/reference-sources/Text.Gob.cinch) · line 635

<a id="value-676f622d6572726f722d6d657373616765"></a>

## `gob-error-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gob-error-message</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobError</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Render a human-readable diagnostic message for a GobError.

[Source](/reference-sources/Text.Gob.cinch) · line 141

<a id="value-676f622d6669656c642d626f6f6c"></a>

## `gob-field-bool`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gob-field-bool</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobValue</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobError</span> <span class="tok-constructor">Bool</span>)))</code></pre>

Extract a typed Bool field from a GobStruct value.

[Source](/reference-sources/Text.Gob.cinch) · line 287

<a id="value-676f622d6669656c642d6279746573"></a>

## `gob-field-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gob-field-bytes</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobValue</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobError</span> <span class="tok-constructor">Bytes</span>)))</code></pre>

Extract a typed Bytes field from a GobStruct value.

[Source](/reference-sources/Text.Gob.cinch) · line 319

<a id="value-676f622d6669656c642d636f6d706c6578"></a>

## `gob-field-complex`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gob-field-complex</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobValue</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobError</span> (<span class="tok-constructor">F64</span>, <span class="tok-constructor">F64</span>))))</code></pre>

Extract a typed complex128 (real, imag) field from a GobStruct value.

[Source](/reference-sources/Text.Gob.cinch) · line 335

<a id="value-676f622d6669656c642d666c6f6174"></a>

## `gob-field-float`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gob-field-float</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobValue</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobError</span> <span class="tok-constructor">F64</span>)))</code></pre>

Extract a typed F64 field from a GobStruct value.

[Source](/reference-sources/Text.Gob.cinch) · line 311

<a id="value-676f622d6669656c642d696e74"></a>

## `gob-field-int`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gob-field-int</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobValue</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobError</span> <span class="tok-constructor">I64</span>)))</code></pre>

Extract a typed I64 field from a GobStruct value.

[Source](/reference-sources/Text.Gob.cinch) · line 295

<a id="value-676f622d6669656c642d737472696e67"></a>

## `gob-field-string`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gob-field-string</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobValue</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)))</code></pre>

Extract a typed String field from a GobStruct value.

[Source](/reference-sources/Text.Gob.cinch) · line 327

<a id="value-676f622d6669656c642d75696e74"></a>

## `gob-field-uint`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gob-field-uint</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobValue</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobError</span> <span class="tok-constructor">U64</span>)))</code></pre>

Extract a typed U64 field from a GobStruct value.

[Source](/reference-sources/Text.Gob.cinch) · line 303

<a id="value-676f622d69732d7a65726f"></a>

## `gob-is-zero`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gob-is-zero</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobValue</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Whether a GobValue is zero for struct field omission rules.
Fixed arrays and nested structs always return False because they are always transmitted.

[Source](/reference-sources/Text.Gob.cinch) · line 420

<a id="value-676f622d6d6170"></a>

## `gob-map`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gob-map</span> :: ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobValue</span>, <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobValue</span>)) -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobValue</span>)</code></pre>

Construct a gob map value from a list of (key, value) pairs.

[Source](/reference-sources/Text.Gob.cinch) · line 246

<a id="value-676f622d6d61702d656e7472696573"></a>

## `gob-map-entries`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gob-map-entries</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobValue</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobValue</span>, <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobValue</span>)))</code></pre>

Extract the key-value entries of a gob map; other values return Nil.

[Source](/reference-sources/Text.Gob.cinch) · line 261

<a id="value-676f622d736c696365"></a>

## `gob-slice`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gob-slice</span> :: ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobValue</span>) -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobValue</span>)</code></pre>

Construct a gob slice value from a list of elements.

[Source](/reference-sources/Text.Gob.cinch) · line 238

<a id="value-676f622d736c6963652d6974656d73"></a>

## `gob-slice-items`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gob-slice-items</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobValue</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobValue</span>))</code></pre>

Extract the elements of a gob slice or fixed array; other values return Nil.

[Source](/reference-sources/Text.Gob.cinch) · line 254

<a id="value-676f622d737472756374"></a>

## `gob-struct`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gob-struct</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>, <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobValue</span>)) -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobValue</span>))</code></pre>

Construct a named gob struct value from a list of (field, value) pairs.

[Source](/reference-sources/Text.Gob.cinch) · line 250

<a id="value-676f622d7374727563742d6669656c64"></a>

## `gob-struct-field`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gob-struct-field</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobValue</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobValue</span>)))</code></pre>

Look up a named field on a GobStruct value.

[Source](/reference-sources/Text.Gob.cinch) · line 281

<a id="value-676f622d7374727563742d6669656c6473"></a>

## `gob-struct-fields`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gob-struct-fields</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobValue</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>, <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobValue</span>)))</code></pre>

Extract the named fields of a gob struct; other values return Nil.

[Source](/reference-sources/Text.Gob.cinch) · line 267

<a id="value-676f622d747970652d6571"></a>

## `gob-type-eq`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gob-type-eq</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobType</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobType</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

Structural equality between two gob type descriptors.

[Source](/reference-sources/Text.Gob.cinch) · line 183

<a id="value-676f622d747970652d6669656c6473"></a>

## `gob-type-fields`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gob-type-fields</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobFieldTypes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>, <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobType</span>)))</code></pre>

Extract the ordered field descriptors of a GobFieldTypes spine as a list.

[Source](/reference-sources/Text.Gob.cinch) · line 166

<a id="value-676f622d747970652d737472756374"></a>

## `gob-type-struct`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gob-type-struct</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>, <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobType</span>)) -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobType</span>))</code></pre>

Construct a named gob struct type from an ordered list of field descriptors.

[Source](/reference-sources/Text.Gob.cinch) · line 162

<a id="value-676f622d76616c75652d6571"></a>

## `gob-value-eq`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gob-value-eq</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobValue</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobValue</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

Exact structural equality on GobValue, using bit-exact IEEE-754 comparison for floats.

[Source](/reference-sources/Text.Gob.cinch) · line 368

<a id="value-676f622d7a65726f2d76616c7565"></a>

## `gob-zero-value`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gob-zero-value</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobType</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobValue</span>)</code></pre>

Construct the canonical zero value for any GobType descriptor.

[Source](/reference-sources/Text.Gob.cinch) · line 404

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Text/Gob.cinch`. Source SHA-256: `f0fb3e24362574e84fa15cd9312892ab2200aac7cc2a9c3e2258b3235e88543d`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
