# Effect.Scanner

Advanced byte-stream tokenization with bounded buffers, fresh tokens and explicit checked protocol failures.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Effect.Scanner`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Effect.Scanner.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Int](Data.Int.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Effect.Buffered.Core](Effect.Buffered.Core.md), [Effect.Io](Effect.Io.md), [Effect.Scanner.Split](Effect.Scanner.Split.md), [Effect.Stream](Effect.Stream.md).

Non-embedded source closure: [Data.Bytes.Utf8](Data.Bytes.Utf8.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Unicode.Kernel](Data.Unicode.Kernel.md), [Data.Unicode.Tables](Data.Unicode.Tables.md), [Effect.Buffered.Core](Effect.Buffered.Core.md), [Effect.Buffered.Internal.Cell](Effect.Buffered.Internal.Cell.md), [Effect.Buffered.Internal.Scan](Effect.Buffered.Internal.Scan.md), [Effect.Scanner](Effect.Scanner.md), [Effect.Scanner.Split](Effect.Scanner.Split.md), [Effect.Stream](Effect.Stream.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Advanced byte-stream tokenization with bounded buffers, fresh tokens and explicit checked protocol failures.
Configuration is immutable and becomes fixed after the first scan. Custom
split callbacks receive snapshots; token getters never expose private backing.

</details>

# Types and constructors

<a id="type-5363616e6e6572"></a>

## `Scanner`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Scanner</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>)</code></pre>

An advanced tokenizer retains its authoritative source, split capability and last token independently.

[Source](/reference-sources/Effect.Scanner.cinch) · line 27

# Values

<a id="value-7363616e6e6572"></a>

## `scanner`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">scanner</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">BufferSource</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Scanner</span>.<span class="tok-constructor">Scanner</span> <span class="tok-variable">r</span>))</code></pre>

Construct an unstarted line scanner with lazy initial buffer allocation and the default token maximum.

[Source](/reference-sources/Effect.Scanner.cinch) · line 36

<a id="value-7363616e6e65722d627566666572"></a>

## `scanner-buffer`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">scanner-buffer</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Scanner</span>.<span class="tok-constructor">Scanner</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Int</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Scanner</span>.<span class="tok-constructor">Split</span>.<span class="tok-constructor">ScanError</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Scanner</span>.<span class="tok-constructor">Scanner</span> <span class="tok-variable">r</span>)))))</code></pre>

Configure an initial private buffer length and a signed growth maximum before scanning begins.
The supplied byte contents are ignored and never retained. Existing initial capacity may exceed the growth maximum.

[Source](/reference-sources/Effect.Scanner.cinch) · line 45

<a id="value-7363616e6e65722d6275666665722d73697a65"></a>

## `scanner-buffer-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">scanner-buffer-size</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Scanner</span>.<span class="tok-constructor">Scanner</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Int</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Scanner</span>.<span class="tok-constructor">Split</span>.<span class="tok-constructor">ScanError</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Scanner</span>.<span class="tok-constructor">Scanner</span> <span class="tok-variable">r</span>)))))</code></pre>

Select explicit initial buffer capacity without allocating or exposing caller-owned backing storage.

[Source](/reference-sources/Effect.Scanner.cinch) · line 49

<a id="value-7363616e6e65722d6279746573"></a>

## `scanner-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">scanner-bytes</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Scanner</span>.<span class="tok-constructor">Scanner</span> <span class="tok-variable">r</span>) -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Observe fresh last-token bytes; an absent token is represented by fresh empty bytes.

[Source](/reference-sources/Effect.Scanner.cinch) · line 74

<a id="value-7363616e6e65722d6572726f72"></a>

## `scanner-error`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">scanner-error</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Scanner</span>.<span class="tok-constructor">Scanner</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Scanner</span>.<span class="tok-constructor">Split</span>.<span class="tok-constructor">ScanError</span>))</code></pre>

Observe the first retained non-end error; ordinary source end-of-input is successful termination.

[Source](/reference-sources/Effect.Scanner.cinch) · line 83

<a id="value-7363616e6e65722d66726f6d2d73747265616d"></a>

## `scanner-from-stream`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">scanner-from-stream</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteReader</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Scanner</span>.<span class="tok-constructor">Scanner</span> <span class="tok-variable">r</span>))</code></pre>

Adapt an existing stream reader under its established positive-empty-success end-of-input contract.

[Source](/reference-sources/Effect.Scanner.cinch) · line 40

<a id="value-7363616e6e65722d6d61782d746f6b656e2d73697a65"></a>

## `scanner-max-token-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">scanner-max-token-size</span> :: <span class="tok-constructor">Nat</span></code></pre>

The default maximum allocated token buffer is 65,536 bytes, including any required delimiter.

[Source](/reference-sources/Effect.Scanner.cinch) · line 32

<a id="value-7363616e6e65722d6e657874"></a>

## `scanner-next`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">scanner-next</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Scanner</span>.<span class="tok-constructor">Scanner</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Bool</span>, (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Scanner</span>.<span class="tok-constructor">Scanner</span> <span class="tok-variable">r</span>)))</code></pre>

Advance to the next token, returning an explicit authoritative scanner and a Boolean indicating token presence.
Final empty tokens are delivered once. Invalid advances, counts, excessive
terminal empty tokens and later configuration changes are checked named failures.

[Source](/reference-sources/Effect.Scanner.cinch) · line 218

<a id="value-7363616e6e65722d73706c6974"></a>

## `scanner-split`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">scanner-split</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Scanner</span>.<span class="tok-constructor">Scanner</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Scanner</span>.<span class="tok-constructor">Split</span>.<span class="tok-constructor">ScanSplit</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Scanner</span>.<span class="tok-constructor">Split</span>.<span class="tok-constructor">ScanError</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Scanner</span>.<span class="tok-constructor">Scanner</span> <span class="tok-variable">r</span>))))</code></pre>

Select a pure custom split capability before the first scan; later changes return a checked refusal.

[Source](/reference-sources/Effect.Scanner.cinch) · line 55

<a id="value-7363616e6e65722d74657874"></a>

## `scanner-text`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">scanner-text</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Scanner</span>.<span class="tok-constructor">Scanner</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

Check the complete last-token bytes for UTF-8 validity without discarding malformed bytes from scanner-bytes.
An absent or empty token yields Some empty text; malformed bytes yield None.

[Source](/reference-sources/Effect.Scanner.cinch) · line 79

<a id="value-7363616e6e65722d746f6b656e"></a>

## `scanner-token`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">scanner-token</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Scanner</span>.<span class="tok-constructor">Scanner</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Bytes</span>))</code></pre>

Observe a fresh optional last token, preserving absence separately from an explicitly empty token.

[Source](/reference-sources/Effect.Scanner.cinch) · line 67

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Effect/Scanner.cinch`. Source SHA-256: `dd6d190f2b53e1fb80b10e48d64a29363b82041895660675d2b383c637be06aa`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
