# Text.QuotedPrintable

Quoted-printable byte encoding and decoding with checked line wrapping, escape and malformed-input policies.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Text.QuotedPrintable`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.QuotedPrintable.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Algebra.Semigroup](Algebra.Semigroup.md), [Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Text.Show](Text.Show.md).

Non-embedded source closure: [Data.Bytes.Word](Data.Bytes.Word.md), [Text.QuotedPrintable](Text.QuotedPrintable.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Quoted-printable byte codecs with explicit newline modes and continuation state.
Text mode normalizes CR, LF and CRLF to CRLF; binary mode escapes them.
Encoded lines contain at most 76 symbols, including a soft-break marker.
Continuations snapshot unfinished input. Returned byte buffers are fresh and
may be mutated without changing a continuation or another returned buffer.
Decoding accepts lowercase hex, raw high bytes and malformed non-newline
escapes as literals; precise errors retain the successfully decoded prefix.

</details>

# Types and constructors

<a id="type-51756f7465645072696e7461626c654465636f646572"></a>

## `QuotedPrintableDecoder`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">QuotedPrintableDecoder</span></code></pre>

Immutable decoder continuation owning its unfinished physical-line snapshot.
Feed results count consumed input separately from the failure's precise offset.

[Source](/reference-sources/Text.QuotedPrintable.cinch) · line 320

<a id="type-51756f7465645072696e7461626c65456e636f646572"></a>

## `QuotedPrintableEncoder`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">QuotedPrintableEncoder</span></code></pre>

Immutable encoding continuation. Feeds clone its private line before writing.

[Source](/reference-sources/Text.QuotedPrintable.cinch) · line 47

<a id="type-51756f7465645072696e7461626c654572726f72"></a>

## `QuotedPrintableError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">QuotedPrintableError</span></code></pre>

Malformed input or an overlong physical line. Prefix APIs retain all
decoded bytes preceding the failure; offsets count original input bytes.

[Source](/reference-sources/Text.QuotedPrintable.cinch) · line 264

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">QuotedPrintableInvalidByte</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">0</span> <span class="tok-number">255</span> <span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">QuotedPrintable</span>.<span class="tok-constructor">QuotedPrintableError</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">QuotedPrintableInvalidHex</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">0</span> <span class="tok-number">255</span> <span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">QuotedPrintable</span>.<span class="tok-constructor">QuotedPrintableError</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">QuotedPrintableIncompleteEscape</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">QuotedPrintable</span>.<span class="tok-constructor">QuotedPrintableError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">QuotedPrintableInvalidSoftBreak</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">QuotedPrintable</span>.<span class="tok-constructor">QuotedPrintableError</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">QuotedPrintableLineTooLong</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">QuotedPrintable</span>.<span class="tok-constructor">QuotedPrintableError</span>)</code></pre>

<a id="type-51756f7465645072696e7461626c654d6f6465"></a>

## `QuotedPrintableMode`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">QuotedPrintableMode</span></code></pre>

Newline policy for subsequent input; mode changes preserve pending bytes and CR carry.

[Source](/reference-sources/Text.QuotedPrintable.cinch) · line 37

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">QuotedPrintableText</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">QuotedPrintable</span>.<span class="tok-constructor">QuotedPrintableMode</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">QuotedPrintableBinary</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">QuotedPrintable</span>.<span class="tok-constructor">QuotedPrintableMode</span></code></pre>

# Values

<a id="value-71756f7465642d7072696e7461626c652d6465636f64652d6279746573"></a>

## `quoted-printable-decode-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">quoted-printable-decode-bytes</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">QuotedPrintable</span>.<span class="tok-constructor">QuotedPrintableError</span> <span class="tok-constructor">Bytes</span>))</code></pre>

Return fresh decoded bytes or the first failure under the decoder
acceptance rules. Lowercase hex and malformed non-newline escapes are accepted.
Use decode-prefix when the successfully decoded bytes before an error matter.

[Source](/reference-sources/Text.QuotedPrintable.cinch) · line 553

<a id="value-71756f7465642d7072696e7461626c652d6465636f64652d707265666978"></a>

## `quoted-printable-decode-prefix`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">quoted-printable-decode-prefix</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span>, (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">QuotedPrintable</span>.<span class="tok-constructor">QuotedPrintableError</span>)))</code></pre>

Decode a complete buffer to fresh bytes and an optional first error.

[Source](/reference-sources/Text.QuotedPrintable.cinch) · line 544

<a id="value-71756f7465642d7072696e7461626c652d6465636f646572"></a>

## `quoted-printable-decoder`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">quoted-printable-decoder</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">QuotedPrintable</span>.<span class="tok-constructor">QuotedPrintableDecoder</span></code></pre>

Begin independent decoding with original offsets starting at zero.

[Source](/reference-sources/Text.QuotedPrintable.cinch) · line 330

<a id="value-71756f7465642d7072696e7461626c652d6465636f6465722d6361706163697479"></a>

## `quoted-printable-decoder-capacity`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">quoted-printable-decoder-capacity</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">QuotedPrintable</span>.<span class="tok-constructor">QuotedPrintableDecoder</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Number of bytes that may be fetched without reading past the decoder's
current physical-line buffer capacity. Failed decoders request no input.

[Source](/reference-sources/Text.QuotedPrintable.cinch) · line 340

<a id="value-71756f7465642d7072696e7461626c652d6465636f6465722d656e64"></a>

## `quoted-printable-decoder-end`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">quoted-printable-decoder-end</span> :: (<span class="tok-constructor">Bool</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">QuotedPrintable</span>.<span class="tok-constructor">QuotedPrintableDecoder</span> -&gt; (<span class="tok-constructor">Bytes</span>, (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">QuotedPrintable</span>.<span class="tok-constructor">QuotedPrintableError</span>))))</code></pre>

End an unfinished physical line, distinguishing genuine EOF from an
interrupted source. A codec error may supersede a pending source failure.

[Source](/reference-sources/Text.QuotedPrintable.cinch) · line 533

<a id="value-71756f7465642d7072696e7461626c652d6465636f6465722d6572726f72"></a>

## `quoted-printable-decoder-error`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">quoted-printable-decoder-error</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">QuotedPrintable</span>.<span class="tok-constructor">QuotedPrintableDecoder</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">QuotedPrintable</span>.<span class="tok-constructor">QuotedPrintableError</span>))</code></pre>

Observe the first sticky codec failure without advancing the continuation.

[Source](/reference-sources/Text.QuotedPrintable.cinch) · line 334

<a id="value-71756f7465642d7072696e7461626c652d6465636f6465722d66656564"></a>

## `quoted-printable-decoder-feed`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">quoted-printable-decoder-feed</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">QuotedPrintable</span>.<span class="tok-constructor">QuotedPrintableDecoder</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span>, (<span class="tok-constructor">Nat</span>, <span class="tok-constructor">Text</span>.<span class="tok-constructor">QuotedPrintable</span>.<span class="tok-constructor">QuotedPrintableDecoder</span>))))</code></pre>

Decode complete physical lines from a feed, returning fresh prefix bytes,
the consumed input count, and an advanced immutable continuation. Input after
a terminal error remains unconsumed; previously returned buffers never alias state.

[Source](/reference-sources/Text.QuotedPrintable.cinch) · line 525

<a id="value-71756f7465642d7072696e7461626c652d6465636f6465722d66696e697368"></a>

## `quoted-printable-decoder-finish`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">quoted-printable-decoder-finish</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">QuotedPrintable</span>.<span class="tok-constructor">QuotedPrintableDecoder</span> -&gt; (<span class="tok-constructor">Bytes</span>, (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">QuotedPrintable</span>.<span class="tok-constructor">QuotedPrintableError</span>)))</code></pre>

Finish at genuine EOF, preserving decoded prefix bytes before a final error.

[Source](/reference-sources/Text.QuotedPrintable.cinch) · line 540

<a id="value-71756f7465642d7072696e7461626c652d6465636f6465722d706f736974696f6e"></a>

## `quoted-printable-decoder-position`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">quoted-printable-decoder-position</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">QuotedPrintable</span>.<span class="tok-constructor">QuotedPrintableDecoder</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Original input bytes consumed so far, including the unfinished physical line.

[Source](/reference-sources/Text.QuotedPrintable.cinch) · line 347

<a id="value-71756f7465642d7072696e7461626c652d656e636f6465"></a>

## `quoted-printable-encode`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">quoted-printable-encode</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">QuotedPrintable</span>.<span class="tok-constructor">QuotedPrintableMode</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

Encode a complete buffer as ASCII text; all possible input bytes are representable.

[Source](/reference-sources/Text.QuotedPrintable.cinch) · line 257

<a id="value-71756f7465642d7072696e7461626c652d656e636f64652d6279746573"></a>

## `quoted-printable-encode-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">quoted-printable-encode-bytes</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">QuotedPrintable</span>.<span class="tok-constructor">QuotedPrintableMode</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Bytes</span>))</code></pre>

Encode a complete buffer as fresh ASCII bytes using an explicit newline policy.

[Source](/reference-sources/Text.QuotedPrintable.cinch) · line 252

<a id="value-71756f7465642d7072696e7461626c652d656e636f646572"></a>

## `quoted-printable-encoder`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">quoted-printable-encoder</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">QuotedPrintable</span>.<span class="tok-constructor">QuotedPrintableMode</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">QuotedPrintable</span>.<span class="tok-constructor">QuotedPrintableEncoder</span>)</code></pre>

Begin encoding with empty line state under an explicit newline policy.

[Source](/reference-sources/Text.QuotedPrintable.cinch) · line 57

<a id="value-71756f7465642d7072696e7461626c652d656e636f6465722d6576656e7473"></a>

## `quoted-printable-encoder-events`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">quoted-printable-encoder-events</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">QuotedPrintable</span>.<span class="tok-constructor">QuotedPrintableEncoder</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-constructor">Nat</span>, <span class="tok-constructor">Bytes</span>)), <span class="tok-constructor">Text</span>.<span class="tok-constructor">QuotedPrintable</span>.<span class="tok-constructor">QuotedPrintableEncoder</span>)))</code></pre>

Encode a feed as complete sink-write events and an advanced continuation.
Each event carries the input prefix reportable if its sink write fails.
No event or retained state aliases the caller's mutable input.

[Source](/reference-sources/Text.QuotedPrintable.cinch) · line 205

<a id="value-71756f7465642d7072696e7461626c652d656e636f6465722d66656564"></a>

## `quoted-printable-encoder-feed`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">quoted-printable-encoder-feed</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">QuotedPrintable</span>.<span class="tok-constructor">QuotedPrintableEncoder</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span>, <span class="tok-constructor">Text</span>.<span class="tok-constructor">QuotedPrintable</span>.<span class="tok-constructor">QuotedPrintableEncoder</span>)))</code></pre>

Encode complete lines from one feed and retain only the unfinished private line.

[Source](/reference-sources/Text.QuotedPrintable.cinch) · line 243

<a id="value-71756f7465642d7072696e7461626c652d656e636f6465722d66696e697368"></a>

## `quoted-printable-encoder-finish`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">quoted-printable-encoder-finish</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">QuotedPrintable</span>.<span class="tok-constructor">QuotedPrintableEncoder</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Finish an immutable continuation and return the independent final encoded bytes.

[Source](/reference-sources/Text.QuotedPrintable.cinch) · line 248

<a id="value-71756f7465642d7072696e7461626c652d656e636f6465722d66696e6973682d6576656e7473"></a>

## `quoted-printable-encoder-finish-events`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">quoted-printable-encoder-finish-events</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">QuotedPrintable</span>.<span class="tok-constructor">QuotedPrintableEncoder</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-constructor">Nat</span>, <span class="tok-constructor">Bytes</span>)))</code></pre>

Flush the final line as fresh events, quoting its last space or tab.
The zero-length final event records a logical flush; byte streams may
treat it as an inert empty write under their existing protocol.

[Source](/reference-sources/Text.QuotedPrintable.cinch) · line 212

<a id="value-71756f7465642d7072696e7461626c652d656e636f6465722d6d6f6465"></a>

## `quoted-printable-encoder-mode`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">quoted-printable-encoder-mode</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">QuotedPrintable</span>.<span class="tok-constructor">QuotedPrintableMode</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">QuotedPrintable</span>.<span class="tok-constructor">QuotedPrintableEncoder</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">QuotedPrintable</span>.<span class="tok-constructor">QuotedPrintableEncoder</span>))</code></pre>

Change the policy for later feeds without flushing or changing pending input.

[Source](/reference-sources/Text.QuotedPrintable.cinch) · line 61

<a id="value-71756f7465642d7072696e7461626c652d656e636f6465722d706f6c696379"></a>

## `quoted-printable-encoder-policy`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">quoted-printable-encoder-policy</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">QuotedPrintable</span>.<span class="tok-constructor">QuotedPrintableEncoder</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">QuotedPrintable</span>.<span class="tok-constructor">QuotedPrintableMode</span>)</code></pre>

Observe the policy used for subsequent feeds without advancing or copying state.

[Source](/reference-sources/Text.QuotedPrintable.cinch) · line 66

<a id="value-71756f7465642d7072696e7461626c652d6572726f722d6d657373616765"></a>

## `quoted-printable-error-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">quoted-printable-error-message</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">QuotedPrintable</span>.<span class="tok-constructor">QuotedPrintableError</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Human-readable diagnostics preserving the exact offending byte or transport suffix.

[Source](/reference-sources/Text.QuotedPrintable.cinch) · line 304

<a id="value-71756f7465642d7072696e7461626c652d6572726f722d6f6666736574"></a>

## `quoted-printable-error-offset`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">quoted-printable-error-offset</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">QuotedPrintable</span>.<span class="tok-constructor">QuotedPrintableError</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

The absolute original input position associated with a codec failure.

[Source](/reference-sources/Text.QuotedPrintable.cinch) · line 277

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Text/QuotedPrintable.cinch`. Source SHA-256: `ae2def9c5e872f55fddb1fb908801d71263bd499e4882f6dc9bd205f2b816e00`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
