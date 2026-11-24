# Text.Ascii85

ASCII85 text, caller-buffer and incremental codecs with consumed-input progress, checked source offsets, SIMD groups and control-run scanning.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Text.Ascii85`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.Ascii85.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Data.Wide](Data.Wide.md).

Non-embedded source closure: [Text.Ascii85](Text.Ascii85.md).

<details><summary>Behaviour and boundary notes from the source</summary>

ASCII85 byte encoding with incremental encoder and decoder state.
Four bytes become five symbols from ! through u; a complete zero word
becomes z. No document delimiters are added or accepted. Decoding ignores
bytes 0 through 32, counts errors in original input bytes, and accumulates
complete groups modulo 2^32, including noncanonical overflowing groups.
Complete groups use portable machine and SIMD digit arithmetic. Incremental calls
retain only an unfinished word; finishing emits the final short quantum.

</details>

# Types and constructors

<a id="type-417363696938354275666665724572726f72"></a>

## `Ascii85BufferError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Ascii85BufferError</span></code></pre>

A source offset or destination interval is outside its accessible buffer.

[Source](/reference-sources/Text.Ascii85.cinch) · line 278

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Ascii85ShortDestination</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Ascii85</span>.<span class="tok-constructor">Ascii85BufferError</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Ascii85InvalidSourceOffset</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Ascii85</span>.<span class="tok-constructor">Ascii85BufferError</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Ascii85InvalidDestinationOffset</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Ascii85</span>.<span class="tok-constructor">Ascii85BufferError</span>))</code></pre>

<a id="type-417363696938354465636f646572"></a>

## `Ascii85Decoder`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Ascii85Decoder</span></code></pre>

An immutable decoding continuation holding at most four pending symbols.

[Source](/reference-sources/Text.Ascii85.cinch) · line 53

<a id="type-41736369693835456e636f646572"></a>

## `Ascii85Encoder`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Ascii85Encoder</span></code></pre>

An immutable encoding continuation holding at most three pending bytes.

[Source](/reference-sources/Text.Ascii85.cinch) · line 48

<a id="type-417363696938354572726f72"></a>

## `Ascii85Error`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Ascii85Error</span></code></pre>

Malformed encoded input. An unsuccessful feed returns no partial output.

[Source](/reference-sources/Text.Ascii85.cinch) · line 29

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">InvalidAscii85Byte</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Ascii85</span>.<span class="tok-constructor">Ascii85Error</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">TruncatedAscii85Quantum</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Ascii85</span>.<span class="tok-constructor">Ascii85Error</span>)</code></pre>

# Values

<a id="value-617363696938352d6275666665722d6572726f722d6d657373616765"></a>

## `ascii85-buffer-error-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ascii85-buffer-error-message</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Ascii85</span>.<span class="tok-constructor">Ascii85BufferError</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Render an interval diagnostic with the requested size or offset and its available bound.

[Source](/reference-sources/Text.Ascii85.cinch) · line 287

<a id="value-617363696938352d6465636f6465"></a>

## `ascii85-decode`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ascii85-decode</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Ascii85</span>.<span class="tok-constructor">Ascii85Error</span> <span class="tok-constructor">Bytes</span>))</code></pre>

Decode complete text with error positions measured in UTF-8 input bytes.

[Source](/reference-sources/Text.Ascii85.cinch) · line 266

<a id="value-617363696938352d6465636f64652d6174"></a>

## `ascii85-decode-at`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ascii85-decode-at</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Ascii85</span>.<span class="tok-constructor">Ascii85BufferError</span> (<span class="tok-constructor">Nat</span>, <span class="tok-constructor">Nat</span>, (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Ascii85</span>.<span class="tok-constructor">Ascii85Error</span>))))))))</code></pre>

Decode from a checked source offset without copying an unconsumed suffix into a new input buffer.
Return written bytes and the number consumed from that offset. Errors retain their absolute
original-buffer positions. A caller can add the consumed count to its next source offset.
Four available destination bytes permit progress; final short output occupies only its actual bytes.
Overlapping intervals snapshot the source before writing; disjoint buffers keep their original owners.

[Source](/reference-sources/Text.Ascii85.cinch) · line 364

<a id="value-617363696938352d6465636f64652d6279746573"></a>

## `ascii85-decode-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ascii85-decode-bytes</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Ascii85</span>.<span class="tok-constructor">Ascii85Error</span> <span class="tok-constructor">Bytes</span>))</code></pre>

Decode a complete byte buffer. Control bytes through space are ignored; framing is invalid.

[Source](/reference-sources/Text.Ascii85.cinch) · line 260

<a id="value-617363696938352d6465636f64652d696e746f"></a>

## `ascii85-decode-into`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ascii85-decode-into</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Ascii85</span>.<span class="tok-constructor">Ascii85BufferError</span> (<span class="tok-constructor">Nat</span>, <span class="tok-constructor">Nat</span>, (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Ascii85</span>.<span class="tok-constructor">Ascii85Error</span>)))))))</code></pre>

Decode into an existing interval and return written bytes, consumed input and an optional error.
A full quantum requires four available bytes; insufficient space returns completed progress.
With finish False, consumed input ends at the last full group so its unfinished suffix can be retried.
With finish True, valid short tails are emitted and all input is consumed. On malformed input,
completed output and its consumed boundary remain available alongside the absolute byte error.
An offset beyond the destination is rejected before mutation; overlapping input is snapshotted.

[Source](/reference-sources/Text.Ascii85.cinch) · line 356

<a id="value-617363696938352d6465636f64652d707265666978"></a>

## `ascii85-decode-prefix`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ascii85-decode-prefix</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; (<span class="tok-constructor">Bytes</span>, <span class="tok-constructor">Nat</span>, (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Ascii85</span>.<span class="tok-constructor">Ascii85Error</span>))))</code></pre>

Decode to fresh bytes while exposing completed output, consumed input and any malformed suffix.
A non-final call leaves an unfinished quantum in the input suffix rather than hidden state.

[Source](/reference-sources/Text.Ascii85.cinch) · line 376

<a id="value-617363696938352d6465636f646572"></a>

## `ascii85-decoder`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ascii85-decoder</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Ascii85</span>.<span class="tok-constructor">Ascii85Decoder</span></code></pre>

Begin an independent decoding with absolute error offsets starting at zero.

[Source](/reference-sources/Text.Ascii85.cinch) · line 66

<a id="value-617363696938352d6465636f6465722d66656564"></a>

## `ascii85-decoder-feed`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ascii85-decoder-feed</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Ascii85</span>.<span class="tok-constructor">Ascii85Decoder</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Ascii85</span>.<span class="tok-constructor">Ascii85Error</span> (<span class="tok-constructor">Bytes</span>, <span class="tok-constructor">Text</span>.<span class="tok-constructor">Ascii85</span>.<span class="tok-constructor">Ascii85Decoder</span>))))</code></pre>

Decode complete groups in arbitrary byte chunks, retaining unfinished symbols.
Invalid bytes return an absolute byte offset and no output for this feed.

[Source](/reference-sources/Text.Ascii85.cinch) · line 236

<a id="value-617363696938352d6465636f6465722d66696e697368"></a>

## `ascii85-decoder-finish`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ascii85-decoder-finish</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Ascii85</span>.<span class="tok-constructor">Ascii85Decoder</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Ascii85</span>.<span class="tok-constructor">Ascii85Error</span> <span class="tok-constructor">Bytes</span>))</code></pre>

Finish decoding: a single remaining symbol is invalid; two through four emit a byte tail.

[Source](/reference-sources/Text.Ascii85.cinch) · line 249

<a id="value-617363696938352d656e636f6465"></a>

## `ascii85-encode`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ascii85-encode</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Encode a complete buffer without document delimiters or inserted whitespace.

[Source](/reference-sources/Text.Ascii85.cinch) · line 160

<a id="value-617363696938352d656e636f64652d6279746573"></a>

## `ascii85-encode-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ascii85-encode-bytes</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Encode a complete buffer to fresh ASCII bytes, including an exact short final word.

[Source](/reference-sources/Text.Ascii85.cinch) · line 294

<a id="value-617363696938352d656e636f64652d696e746f"></a>

## `ascii85-encode-into`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ascii85-encode-into</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Ascii85</span>.<span class="tok-constructor">Ascii85BufferError</span> <span class="tok-constructor">Nat</span>))))</code></pre>

Encode at a destination offset and return the actual byte count, including zero-word shorthand.
Capacity must hold ascii85-max-encoded-length; rejection precedes all writes.
Source and destination may overlap: overlapping input is snapshotted before mutation.

[Source](/reference-sources/Text.Ascii85.cinch) · line 302

<a id="value-617363696938352d656e636f646572"></a>

## `ascii85-encoder`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ascii85-encoder</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Ascii85</span>.<span class="tok-constructor">Ascii85Encoder</span></code></pre>

Begin an independent encoding. Empty feeds preserve the continuation.

[Source](/reference-sources/Text.Ascii85.cinch) · line 62

<a id="value-617363696938352d656e636f6465722d66656564"></a>

## `ascii85-encoder-feed`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ascii85-encoder-feed</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Ascii85</span>.<span class="tok-constructor">Ascii85Encoder</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span>, <span class="tok-constructor">Text</span>.<span class="tok-constructor">Ascii85</span>.<span class="tok-constructor">Ascii85Encoder</span>)))</code></pre>

Encode all complete words in a chunk and return their bytes plus the advanced state.
The returned output is independent of subsequent mutation of the input.

[Source](/reference-sources/Text.Ascii85.cinch) · line 141

<a id="value-617363696938352d656e636f6465722d66696e697368"></a>

## `ascii85-encoder-finish`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ascii85-encoder-finish</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Ascii85</span>.<span class="tok-constructor">Ascii85Encoder</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Finish an encoding, emitting two through four symbols for any pending bytes.
A partial zero word is never shortened to z. Finishing an empty state is empty.

[Source](/reference-sources/Text.Ascii85.cinch) · line 149

<a id="value-617363696938352d6572726f722d6d657373616765"></a>

## `ascii85-error-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ascii85-error-message</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Ascii85</span>.<span class="tok-constructor">Ascii85Error</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Render a decoding diagnostic, including the offending byte or incomplete tail position.

[Source](/reference-sources/Text.Ascii85.cinch) · line 42

<a id="value-617363696938352d6572726f722d6f6666736574"></a>

## `ascii85-error-offset`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ascii85-error-offset</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Ascii85</span>.<span class="tok-constructor">Ascii85Error</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

The absolute original input byte offset associated with a decoding failure.

[Source](/reference-sources/Text.Ascii85.cinch) · line 36

<a id="value-617363696938352d6d61782d656e636f6465642d6c656e677468"></a>

## `ascii85-max-encoded-length`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ascii85-max-encoded-length</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

A safe encoded byte bound, including a full five-symbol final allocation.

[Source](/reference-sources/Text.Ascii85.cinch) · line 58

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Text/Ascii85.cinch`. Source SHA-256: `f92df60216bcd1b1e78d4f7ba90b3b69066a457f83aa3e0fda21baefd26cb037`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
