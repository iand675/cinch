# Effect.Hex

Hexadecimal encoding writers, decoding readers and formatted dump sinks with explicit continuations and sticky errors.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Effect.Hex`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Effect.Hex.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bytes](Data.Bytes.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Effect.Stream](Effect.Stream.md), [Text.Hex](Text.Hex.md).

Non-embedded source closure: [Effect.Hex](Effect.Hex.md), [Effect.Stream](Effect.Stream.md), [Text.Hex](Text.Hex.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Hexadecimal encoding writers, decoding readers and formatted dump sinks.
Every operation returns its advanced continuation, including after failure.
Decoder errors are delayed until completed output has been read; a final
unpaired digit is StreamUnexpectedEnd. All stream failures are sticky.
Dump sinks buffer a partial sixteen-byte row until explicitly finished.
No adapter closes or flushes its underlying descriptor or sink.

</details>

# Types and constructors

<a id="type-48657844756d706572"></a>

## `HexDumper`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">HexDumper</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>)</code></pre>

A formatted dump continuation with at most fifteen pending input bytes.

[Source](/reference-sources/Effect.Hex.cinch) · line 123

<a id="type-486578577269746572"></a>

## `HexWriter`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">HexWriter</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>)</code></pre>

An explicitly advanced hexadecimal encoding sink with sticky failure state.

[Source](/reference-sources/Effect.Hex.cinch) · line 23

# Values

<a id="value-6865782d64756d702d66696e697368"></a>

## `hex-dump-finish`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hex-dump-finish</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Hex</span>.<span class="tok-constructor">HexDumper</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">StreamError</span> <span class="tok-constructor">Unit</span>), (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Hex</span>.<span class="tok-constructor">HexDumper</span> <span class="tok-variable">r</span>)))</code></pre>

Flush the final partial row and retain a finished handle. This operation is idempotent.
It does not flush or close the underlying sink; inspect that sink separately when needed.

[Source](/reference-sources/Effect.Hex.cinch) · line 167

<a id="value-6865782d64756d702d7772697465"></a>

## `hex-dump-write`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hex-dump-write</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Hex</span>.<span class="tok-constructor">HexDumper</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Bytes</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">StreamError</span> <span class="tok-constructor">Nat</span>), (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Hex</span>.<span class="tok-constructor">HexDumper</span> <span class="tok-variable">r</span>))))</code></pre>

Emit completed rows and snapshot the unfinished tail. Report accepted input bytes on success.
Finished handles reject writes with StreamClosed; output failures preserve their advanced sink.

[Source](/reference-sources/Effect.Hex.cinch) · line 144

<a id="value-6865782d64756d706572"></a>

## `hex-dumper`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hex-dumper</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteWriter</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Hex</span>.<span class="tok-constructor">HexDumper</span> <span class="tok-variable">r</span>))</code></pre>

Begin a hexadecimal dump whose first row starts at offset zero.

[Source](/reference-sources/Effect.Hex.cinch) · line 132

<a id="value-6865782d64756d7065722d73696e6b"></a>

## `hex-dumper-sink`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hex-dumper-sink</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Hex</span>.<span class="tok-constructor">HexDumper</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteWriter</span> <span class="tok-variable">r</span>))</code></pre>

Inspect output already accepted by the underlying dump sink, including after errors or finish.

[Source](/reference-sources/Effect.Hex.cinch) · line 136

<a id="value-6865782d656e636f64696e672d777269746572"></a>

## `hex-encoding-writer`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hex-encoding-writer</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteWriter</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteWriter</span> <span class="tok-variable">r</span>))</code></pre>

Expose an encoding sink through the generic byte-writer protocol for composition.
Use the explicit HexWriter interface when the advanced underlying sink must be inspected.

[Source](/reference-sources/Effect.Hex.cinch) · line 50

<a id="value-6865782d726561646572"></a>

## `hex-reader`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hex-reader</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteReader</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteReader</span> <span class="tok-variable">r</span>))</code></pre>

Decode a source in bounded 4096-byte input chunks, accepting either hexadecimal case.
Completed bytes precede any error. Zero reads do not advance the source; an unmatched
final digit reports StreamUnexpectedEnd 2 1. Invalid offsets count original input bytes.

[Source](/reference-sources/Effect.Hex.cinch) · line 87

<a id="value-6865782d7772697465"></a>

## `hex-write`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hex-write</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Hex</span>.<span class="tok-constructor">HexWriter</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Bytes</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">StreamError</span> <span class="tok-constructor">Nat</span>), (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Hex</span>.<span class="tok-constructor">HexWriter</span> <span class="tok-variable">r</span>))))</code></pre>

Write all encoded output, retrying short writes, and report accepted input bytes.
On error the returned writer preserves the advanced sink and will not replay output.

[Source](/reference-sources/Effect.Hex.cinch) · line 41

<a id="value-6865782d777269746572"></a>

## `hex-writer`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hex-writer</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteWriter</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Hex</span>.<span class="tok-constructor">HexWriter</span> <span class="tok-variable">r</span>))</code></pre>

Begin writing lowercase hexadecimal bytes to a sink. No final flush is required.

[Source](/reference-sources/Effect.Hex.cinch) · line 30

<a id="value-6865782d7772697465722d73696e6b"></a>

## `hex-writer-sink`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hex-writer-sink</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Hex</span>.<span class="tok-constructor">HexWriter</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteWriter</span> <span class="tok-variable">r</span>))</code></pre>

Inspect the advanced underlying sink, including after a failed encoded write.

[Source](/reference-sources/Effect.Hex.cinch) · line 34

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Effect/Hex.cinch`. Source SHA-256: `e53cb32a3da3f471f746fe04bcd8ef54d4fc774608d236b60da57a9af60c550b`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
