# Effect.Scanner.Split

Byte-preserving token splitters with explicit absent, empty and final token states.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Effect.Scanner.Split`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Effect.Scanner.Split.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Bytes.Utf8](Data.Bytes.Utf8.md), [Data.Char](Data.Char.md), [Data.Int](Data.Int.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.String](Data.String.md), [Data.Unicode.Kernel](Data.Unicode.Kernel.md), [Effect.Buffered.Core](Effect.Buffered.Core.md), [Effect.Buffered.Internal.Scan](Effect.Buffered.Internal.Scan.md).

Non-embedded source closure: [Data.Bytes.Utf8](Data.Bytes.Utf8.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Unicode.Kernel](Data.Unicode.Kernel.md), [Data.Unicode.Tables](Data.Unicode.Tables.md), [Effect.Buffered.Core](Effect.Buffered.Core.md), [Effect.Buffered.Internal.Cell](Effect.Buffered.Internal.Cell.md), [Effect.Buffered.Internal.Scan](Effect.Buffered.Internal.Scan.md), [Effect.Scanner.Split](Effect.Scanner.Split.md), [Effect.Stream](Effect.Stream.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Byte-preserving token splitters with explicit absent, empty and final token states.
Built-in tokens are fresh. Custom callbacks receive a private snapshot and their
returned tokens are snapshotted before retention. Empty nonterminal direct calls
request more input; they never inspect storage outside the logical byte span.

</details>

# Types and constructors

<a id="type-5363616e4572726f72"></a>

## `ScanError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">ScanError</span></code></pre>

A checked scanner failure, final-token sentinel, source failure or explicit custom diagnostic.

[Source](/reference-sources/Effect.Scanner.Split.cinch) · line 25

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ScanTooLong</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Scanner</span>.<span class="tok-constructor">Split</span>.<span class="tok-constructor">ScanError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ScanNegativeAdvance</span> :: (<span class="tok-constructor">Int</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Scanner</span>.<span class="tok-constructor">Split</span>.<span class="tok-constructor">ScanError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ScanAdvanceTooFar</span> :: (<span class="tok-constructor">Int</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Scanner</span>.<span class="tok-constructor">Split</span>.<span class="tok-constructor">ScanError</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ScanBadReadCount</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Int</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Scanner</span>.<span class="tok-constructor">Split</span>.<span class="tok-constructor">ScanError</span>)))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ScanFinalToken</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Scanner</span>.<span class="tok-constructor">Split</span>.<span class="tok-constructor">ScanError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ScanNoProgress</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Scanner</span>.<span class="tok-constructor">Split</span>.<span class="tok-constructor">ScanError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ScanEmptyTokens</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Scanner</span>.<span class="tok-constructor">Split</span>.<span class="tok-constructor">ScanError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ScanEmptyTokensAfter</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Scanner</span>.<span class="tok-constructor">Split</span>.<span class="tok-constructor">ScanError</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Scanner</span>.<span class="tok-constructor">Split</span>.<span class="tok-constructor">ScanError</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ScanStarted</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Scanner</span>.<span class="tok-constructor">Split</span>.<span class="tok-constructor">ScanError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ScanSourceError</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">BufferError</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Scanner</span>.<span class="tok-constructor">Split</span>.<span class="tok-constructor">ScanError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ScanCustomError</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Scanner</span>.<span class="tok-constructor">Split</span>.<span class="tok-constructor">ScanError</span>)</code></pre>

<a id="type-5363616e53706c6974"></a>

## `ScanSplit`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">ScanSplit</span></code></pre>

A pure split capability receiving owned bytes and a flag indicating any terminal source status.

[Source](/reference-sources/Effect.Scanner.Split.cinch) · line 70

<a id="type-5363616e53746570"></a>

## `ScanStep`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">ScanStep</span></code></pre>

Final-token errors take precedence over the advance count; other counts are checked by the scanner.

[Source](/reference-sources/Effect.Scanner.Split.cinch) · line 67

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ScanStep</span> :: (<span class="tok-constructor">Int</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Scanner</span>.<span class="tok-constructor">Split</span>.<span class="tok-constructor">ScanError</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Scanner</span>.<span class="tok-constructor">Split</span>.<span class="tok-constructor">ScanStep</span>)))</code></pre>

# Values

<a id="value-7363616e2d6170706c79"></a>

## `scan-apply`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">scan-apply</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Scanner</span>.<span class="tok-constructor">Split</span>.<span class="tok-constructor">ScanSplit</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Scanner</span>.<span class="tok-constructor">Split</span>.<span class="tok-constructor">ScanStep</span>)))</code></pre>

Invoke a split capability with a fresh input snapshot and snapshot any mutable returned token.

[Source](/reference-sources/Effect.Scanner.Split.cinch) · line 85

<a id="value-7363616e2d6170706c792d6265747765656e"></a>

## `scan-apply-between`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">scan-apply-between</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Scanner</span>.<span class="tok-constructor">Split</span>.<span class="tok-constructor">ScanSplit</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Scanner</span>.<span class="tok-constructor">Split</span>.<span class="tok-constructor">ScanStep</span>)))))</code></pre>

Invoke a splitter over a checked owner span with exactly one callback-input snapshot and one retained-token snapshot.
Offsets are absolute and invalid bounds return a checked error without invoking the callback.

[Source](/reference-sources/Effect.Scanner.Split.cinch) · line 90

<a id="value-7363616e2d6279746573"></a>

## `scan-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">scan-bytes</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Scanner</span>.<span class="tok-constructor">Split</span>.<span class="tok-constructor">ScanStep</span>))</code></pre>

Split one arbitrary byte; an empty input safely requests more data or completes at end-of-input.

[Source](/reference-sources/Effect.Scanner.Split.cinch) · line 101

<a id="value-7363616e2d6572726f722d6d657373616765"></a>

## `scan-error-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">scan-error-message</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Scanner</span>.<span class="tok-constructor">Split</span>.<span class="tok-constructor">ScanError</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Explain a scanner failure while retaining exact counts and original causes in its structured value.

[Source](/reference-sources/Effect.Scanner.Split.cinch) · line 50

<a id="value-7363616e2d6c696e6573"></a>

## `scan-lines`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">scan-lines</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Scanner</span>.<span class="tok-constructor">Split</span>.<span class="tok-constructor">ScanStep</span>))</code></pre>

Split LF-terminated lines, remove one optional preceding CR, and return a final nonempty unterminated line.

[Source](/reference-sources/Effect.Scanner.Split.cinch) · line 123

<a id="value-7363616e2d72756e6573"></a>

## `scan-runes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">scan-runes</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Scanner</span>.<span class="tok-constructor">Split</span>.<span class="tok-constructor">ScanStep</span>))</code></pre>

Split one shortest-form Unicode scalar, replacing each malformed input byte with encoded U+FFFD.
An incomplete nonterminal encoding requests more input; invalid and truncated terminal bytes consume one byte each.

[Source](/reference-sources/Effect.Scanner.Split.cinch) · line 107

<a id="value-7363616e2d73706c6974"></a>

## `scan-split`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">scan-split</span> :: ((<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Scanner</span>.<span class="tok-constructor">Split</span>.<span class="tok-constructor">ScanStep</span>)) -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Scanner</span>.<span class="tok-constructor">Split</span>.<span class="tok-constructor">ScanSplit</span>)</code></pre>

Define a custom tokenization capability without exposing the scanner's private buffer.

[Source](/reference-sources/Effect.Scanner.Split.cinch) · line 75

<a id="value-7363616e2d776f726473"></a>

## `scan-words`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">scan-words</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Scanner</span>.<span class="tok-constructor">Split</span>.<span class="tok-constructor">ScanStep</span>))</code></pre>

Split nonempty words on the Unicode White_Space property, retaining arbitrary non-whitespace bytes exactly.

[Source](/reference-sources/Effect.Scanner.Split.cinch) · line 152

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Effect/Scanner/Split.cinch`. Source SHA-256: `5bcdaea29e3d7bcc9a7f77aaaad39e5fcb8af0504862e891a4d853f2e8e2c48e`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
