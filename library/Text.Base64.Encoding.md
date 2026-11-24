# Text.Base64.Encoding

Configurable Base64 byte alphabets, padding and strict trailing bits, SIMD codecs, persistent states and checked caller buffers.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Text.Base64.Encoding`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.Base64.Encoding.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bytes](Data.Bytes.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Data.Wide](Data.Wide.md).

Non-embedded source closure: [Text.Base64.Encoding](Text.Base64.Encoding.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Configurable Base64 byte encodings with portable 128-bit SIMD quanta and persistent incremental state.
Alphabets and padding preserve exact byte values, including non-UTF-8 values.
Decoders ignore CR/LF, preserve completed output before failures, and count
original input bytes in diagnostics. Strict mode rejects nonzero unused bits.
All returned buffers are fresh; no caller-owned buffer aliases retained state.

</details>

# Types and constructors

<a id="type-426173653634436f6e6669674572726f72"></a>

## `Base64ConfigError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Base64ConfigError</span></code></pre>

Invalid alphabet or padding configuration, returned without mutating caller storage.

[Source](/reference-sources/Text.Base64.Encoding.cinch) · line 31

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Base64AlphabetLength</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Base64</span>.<span class="tok-constructor">Encoding</span>.<span class="tok-constructor">Base64ConfigError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Base64AlphabetNewline</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Base64</span>.<span class="tok-constructor">Encoding</span>.<span class="tok-constructor">Base64ConfigError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Base64AlphabetDuplicate</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Base64</span>.<span class="tok-constructor">Encoding</span>.<span class="tok-constructor">Base64ConfigError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Base64PaddingValue</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Base64</span>.<span class="tok-constructor">Encoding</span>.<span class="tok-constructor">Base64ConfigError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Base64PaddingCollision</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Base64</span>.<span class="tok-constructor">Encoding</span>.<span class="tok-constructor">Base64ConfigError</span>)</code></pre>

<a id="type-4261736536344465636f646572"></a>

## `Base64Decoder`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Base64Decoder</span></code></pre>

Immutable decoder with a packed partial quantum, absolute offsets and sticky error.

[Source](/reference-sources/Text.Base64.Encoding.cinch) · line 335

<a id="type-426173653634456e636f646572"></a>

## `Base64Encoder`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Base64Encoder</span></code></pre>

Immutable encoding continuation with at most two copied pending input bytes.

[Source](/reference-sources/Text.Base64.Encoding.cinch) · line 314

<a id="type-426173653634456e636f64696e67"></a>

## `Base64Encoding`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Base64Encoding</span></code></pre>

Opaque immutable alphabet, optional padding and trailing-bit validation policy.

[Source](/reference-sources/Text.Base64.Encoding.cinch) · line 26

<a id="type-426173653634496e7075744572726f72"></a>

## `Base64InputError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Base64InputError</span></code></pre>

Precise original-byte failures and caller-buffer capacity failures.

[Source](/reference-sources/Text.Base64.Encoding.cinch) · line 278

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">InvalidBase64Byte</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Base64</span>.<span class="tok-constructor">Encoding</span>.<span class="tok-constructor">Base64InputError</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">TruncatedBase64Quantum</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Base64</span>.<span class="tok-constructor">Encoding</span>.<span class="tok-constructor">Base64InputError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">InvalidBase64TrailingBits</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Base64</span>.<span class="tok-constructor">Encoding</span>.<span class="tok-constructor">Base64InputError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Base64ShortDestination</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Base64</span>.<span class="tok-constructor">Encoding</span>.<span class="tok-constructor">Base64InputError</span>))</code></pre>

# Values

<a id="value-6261736536342d636f6e6669672d6572726f722d6d657373616765"></a>

## `base64-config-error-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">base64-config-error-message</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Base64</span>.<span class="tok-constructor">Encoding</span>.<span class="tok-constructor">Base64ConfigError</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Describe the first invalid alphabet position or rejected padding value.

[Source](/reference-sources/Text.Base64.Encoding.cinch) · line 44

<a id="value-6261736536342d6465636f64652d617070656e64"></a>

## `base64-decode-append`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">base64-decode-append</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Base64</span>.<span class="tok-constructor">Encoding</span>.<span class="tok-constructor">Base64Encoding</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span>, (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Base64</span>.<span class="tok-constructor">Encoding</span>.<span class="tok-constructor">Base64InputError</span>)))))</code></pre>

Append decoded bytes to a copied prefix, preserving any later input failure.

[Source](/reference-sources/Text.Base64.Encoding.cinch) · line 545

<a id="value-6261736536342d6465636f64652d6279746573"></a>

## `base64-decode-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">base64-decode-bytes</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Base64</span>.<span class="tok-constructor">Encoding</span>.<span class="tok-constructor">Base64Encoding</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Base64</span>.<span class="tok-constructor">Encoding</span>.<span class="tok-constructor">Base64InputError</span> <span class="tok-constructor">Bytes</span>)))</code></pre>

Decode complete input atomically; use the prefix API when progress is needed on failure.

[Source](/reference-sources/Text.Base64.Encoding.cinch) · line 535

<a id="value-6261736536342d6465636f64652d696e746f"></a>

## `base64-decode-into`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">base64-decode-into</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Base64</span>.<span class="tok-constructor">Encoding</span>.<span class="tok-constructor">Base64Encoding</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span>, (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Base64</span>.<span class="tok-constructor">Encoding</span>.<span class="tok-constructor">Base64InputError</span>))))))</code></pre>

Decode into a checked capacity bound; complete decoding before copying makes overlap safe.
A short range fails before mutation, otherwise completed output is retained beside its input error.

[Source](/reference-sources/Text.Base64.Encoding.cinch) · line 550

<a id="value-6261736536342d6465636f64652d707265666978"></a>

## `base64-decode-prefix`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">base64-decode-prefix</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Base64</span>.<span class="tok-constructor">Encoding</span>.<span class="tok-constructor">Base64Encoding</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span>, (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Base64</span>.<span class="tok-constructor">Encoding</span>.<span class="tok-constructor">Base64InputError</span>))))</code></pre>

Decode all bytes while preserving the completed prefix beside a later input error.

[Source](/reference-sources/Text.Base64.Encoding.cinch) · line 530

<a id="value-6261736536342d6465636f64652d74657874"></a>

## `base64-decode-text`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">base64-decode-text</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Base64</span>.<span class="tok-constructor">Encoding</span>.<span class="tok-constructor">Base64Encoding</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Bytes</span>, (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Base64</span>.<span class="tok-constructor">Encoding</span>.<span class="tok-constructor">Base64InputError</span>))))</code></pre>

Decode text with original UTF-8 byte offsets while preserving completed output on error.

[Source](/reference-sources/Text.Base64.Encoding.cinch) · line 541

<a id="value-6261736536342d6465636f6465642d6c656e677468"></a>

## `base64-decoded-length`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">base64-decoded-length</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Base64</span>.<span class="tok-constructor">Encoding</span>.<span class="tok-constructor">Base64Encoding</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Nat</span>))</code></pre>

Maximum decoded byte count for an encoded byte count; this does not validate the bytes.

[Source](/reference-sources/Text.Base64.Encoding.cinch) · line 160

<a id="value-6261736536342d6465636f646572"></a>

## `base64-decoder`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">base64-decoder</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Base64</span>.<span class="tok-constructor">Encoding</span>.<span class="tok-constructor">Base64Encoding</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Base64</span>.<span class="tok-constructor">Encoding</span>.<span class="tok-constructor">Base64Decoder</span>)</code></pre>

Begin an independent decoder; CR/LF remain ignored even in strict mode.

[Source](/reference-sources/Text.Base64.Encoding.cinch) · line 340

<a id="value-6261736536342d6465636f6465722d6572726f72"></a>

## `base64-decoder-error`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">base64-decoder-error</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Base64</span>.<span class="tok-constructor">Encoding</span>.<span class="tok-constructor">Base64Decoder</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Base64</span>.<span class="tok-constructor">Encoding</span>.<span class="tok-constructor">Base64InputError</span>))</code></pre>

Observe a sticky failure after retaining all output returned with its feed.

[Source](/reference-sources/Text.Base64.Encoding.cinch) · line 344

<a id="value-6261736536342d6465636f6465722d66656564"></a>

## `base64-decoder-feed`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">base64-decoder-feed</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Base64</span>.<span class="tok-constructor">Encoding</span>.<span class="tok-constructor">Base64Decoder</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span>, <span class="tok-constructor">Text</span>.<span class="tok-constructor">Base64</span>.<span class="tok-constructor">Encoding</span>.<span class="tok-constructor">Base64Decoder</span>)))</code></pre>

Feed encoded bytes, preserving complete output before any malformed byte or strict-bit error.

[Source](/reference-sources/Text.Base64.Encoding.cinch) · line 361

<a id="value-6261736536342d6465636f6465722d66696e697368"></a>

## `base64-decoder-finish`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">base64-decoder-finish</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Base64</span>.<span class="tok-constructor">Encoding</span>.<span class="tok-constructor">Base64Decoder</span> -&gt; (<span class="tok-constructor">Bytes</span>, (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Base64</span>.<span class="tok-constructor">Encoding</span>.<span class="tok-constructor">Base64InputError</span>)))</code></pre>

Validate EOF and render an allowed unpadded tail, retaining strict trailing-bit failures.

[Source](/reference-sources/Text.Base64.Encoding.cinch) · line 516

<a id="value-6261736536342d656e636f64652d617070656e64"></a>

## `base64-encode-append`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">base64-encode-append</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Base64</span>.<span class="tok-constructor">Encoding</span>.<span class="tok-constructor">Base64Encoding</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Bytes</span>)))</code></pre>

Append encoded bytes to a copied prefix without changing either input buffer.

[Source](/reference-sources/Text.Base64.Encoding.cinch) · line 269

<a id="value-6261736536342d656e636f64652d6279746573"></a>

## `base64-encode-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">base64-encode-bytes</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Base64</span>.<span class="tok-constructor">Encoding</span>.<span class="tok-constructor">Base64Encoding</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Bytes</span>))</code></pre>

Encode into fresh bytes, preserving custom bytes without a UTF-8 conversion.

[Source](/reference-sources/Text.Base64.Encoding.cinch) · line 259

<a id="value-6261736536342d656e636f64652d696e746f"></a>

## `base64-encode-into`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">base64-encode-into</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Base64</span>.<span class="tok-constructor">Encoding</span>.<span class="tok-constructor">Base64Encoding</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Base64</span>.<span class="tok-constructor">Encoding</span>.<span class="tok-constructor">Base64InputError</span> <span class="tok-constructor">Nat</span>)))))</code></pre>

Encode into a complete destination range after checking capacity and snapshotting overlap.

[Source](/reference-sources/Text.Base64.Encoding.cinch) · line 305

<a id="value-6261736536342d656e636f64652d74657874"></a>

## `base64-encode-text`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">base64-encode-text</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Base64</span>.<span class="tok-constructor">Encoding</span>.<span class="tok-constructor">Base64Encoding</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)))</code></pre>

Return text only when the exact encoded byte sequence is valid UTF-8.

[Source](/reference-sources/Text.Base64.Encoding.cinch) · line 265

<a id="value-6261736536342d656e636f6465642d6c656e677468"></a>

## `base64-encoded-length`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">base64-encoded-length</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Base64</span>.<span class="tok-constructor">Encoding</span>.<span class="tok-constructor">Base64Encoding</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Nat</span>))</code></pre>

Exact output byte count, including any final padding.

[Source](/reference-sources/Text.Base64.Encoding.cinch) · line 154

<a id="value-6261736536342d656e636f646572"></a>

## `base64-encoder`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">base64-encoder</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Base64</span>.<span class="tok-constructor">Encoding</span>.<span class="tok-constructor">Base64Encoding</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Base64</span>.<span class="tok-constructor">Encoding</span>.<span class="tok-constructor">Base64Encoder</span>)</code></pre>

Begin an encoding that emits full quanta until explicitly finished.

[Source](/reference-sources/Text.Base64.Encoding.cinch) · line 319

<a id="value-6261736536342d656e636f6465722d66656564"></a>

## `base64-encoder-feed`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">base64-encoder-feed</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Base64</span>.<span class="tok-constructor">Encoding</span>.<span class="tok-constructor">Base64Encoder</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span>, <span class="tok-constructor">Text</span>.<span class="tok-constructor">Base64</span>.<span class="tok-constructor">Encoding</span>.<span class="tok-constructor">Base64Encoder</span>)))</code></pre>

Emit complete three-byte quanta and copy only the remaining zero to two input bytes.

[Source](/reference-sources/Text.Base64.Encoding.cinch) · line 323

<a id="value-6261736536342d656e636f6465722d66696e697368"></a>

## `base64-encoder-finish`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">base64-encoder-finish</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Base64</span>.<span class="tok-constructor">Encoding</span>.<span class="tok-constructor">Base64Encoder</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Render the final short quantum with the selected padding, preserving the original continuation.

[Source](/reference-sources/Text.Base64.Encoding.cinch) · line 331

<a id="value-6261736536342d656e636f64696e67"></a>

## `base64-encoding`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">base64-encoding</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Base64</span>.<span class="tok-constructor">Encoding</span>.<span class="tok-constructor">Base64ConfigError</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Base64</span>.<span class="tok-constructor">Encoding</span>.<span class="tok-constructor">Base64Encoding</span>))</code></pre>

Construct an independent encoding from 64 distinct non-newline bytes.
The default pad is '='. If that byte is in the custom alphabet, change or
disable padding before relying on an invertible final short quantum.

[Source](/reference-sources/Text.Base64.Encoding.cinch) · line 89

<a id="value-6261736536342d656e636f64696e672d616c706861626574"></a>

## `base64-encoding-alphabet`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">base64-encoding-alphabet</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Base64</span>.<span class="tok-constructor">Encoding</span>.<span class="tok-constructor">Base64Encoding</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Return an independent copy of the complete byte alphabet.

[Source](/reference-sources/Text.Base64.Encoding.cinch) · line 109

<a id="value-6261736536342d656e636f64696e672d70616464696e67"></a>

## `base64-encoding-padding`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">base64-encoding-padding</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Base64</span>.<span class="tok-constructor">Encoding</span>.<span class="tok-constructor">Base64Encoding</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Nat</span>))</code></pre>

Observe the optional one-byte padding policy.

[Source](/reference-sources/Text.Base64.Encoding.cinch) · line 117

<a id="value-6261736536342d696e7075742d6572726f722d6d657373616765"></a>

## `base64-input-error-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">base64-input-error-message</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Base64</span>.<span class="tok-constructor">Encoding</span>.<span class="tok-constructor">Base64InputError</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Render a diagnostic with an original byte position or complete destination capacity.

[Source](/reference-sources/Text.Base64.Encoding.cinch) · line 297

<a id="value-6261736536342d696e7075742d6572726f722d6f6666736574"></a>

## `base64-input-error-offset`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">base64-input-error-offset</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Base64</span>.<span class="tok-constructor">Encoding</span>.<span class="tok-constructor">Base64InputError</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Original byte position for an input failure; capacity failures return zero.

[Source](/reference-sources/Text.Base64.Encoding.cinch) · line 289

<a id="value-6261736536342d7261772d7374616e646172642d656e636f64696e67"></a>

## `base64-raw-standard-encoding`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">base64-raw-standard-encoding</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Base64</span>.<span class="tok-constructor">Encoding</span>.<span class="tok-constructor">Base64Encoding</span></code></pre>

Standard alphabet without padding.

[Source](/reference-sources/Text.Base64.Encoding.cinch) · line 61

<a id="value-6261736536342d7261772d75726c2d656e636f64696e67"></a>

## `base64-raw-url-encoding`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">base64-raw-url-encoding</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Base64</span>.<span class="tok-constructor">Encoding</span>.<span class="tok-constructor">Base64Encoding</span></code></pre>

URL alphabet without padding.

[Source](/reference-sources/Text.Base64.Encoding.cinch) · line 65

<a id="value-6261736536342d7374616e646172642d656e636f64696e67"></a>

## `base64-standard-encoding`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">base64-standard-encoding</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Base64</span>.<span class="tok-constructor">Encoding</span>.<span class="tok-constructor">Base64Encoding</span></code></pre>

Standard alphabet with equals-sign padding and permissive unused trailing bits.

[Source](/reference-sources/Text.Base64.Encoding.cinch) · line 53

<a id="value-6261736536342d737472696374"></a>

## `base64-strict`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">base64-strict</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Base64</span>.<span class="tok-constructor">Encoding</span>.<span class="tok-constructor">Base64Encoding</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Base64</span>.<span class="tok-constructor">Encoding</span>.<span class="tok-constructor">Base64Encoding</span>)</code></pre>

Copy the encoding with unused trailing-bit validation enabled; encoding output is unchanged.

[Source](/reference-sources/Text.Base64.Encoding.cinch) · line 150

<a id="value-6261736536342d75726c2d656e636f64696e67"></a>

## `base64-url-encoding`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">base64-url-encoding</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Base64</span>.<span class="tok-constructor">Encoding</span>.<span class="tok-constructor">Base64Encoding</span></code></pre>

URL alphabet with hyphen/underscore symbols and equals-sign padding.

[Source](/reference-sources/Text.Base64.Encoding.cinch) · line 57

<a id="value-6261736536342d776974682d70616464696e67"></a>

## `base64-with-padding`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">base64-with-padding</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Base64</span>.<span class="tok-constructor">Encoding</span>.<span class="tok-constructor">Base64Encoding</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Nat</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Base64</span>.<span class="tok-constructor">Encoding</span>.<span class="tok-constructor">Base64ConfigError</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Base64</span>.<span class="tok-constructor">Encoding</span>.<span class="tok-constructor">Base64Encoding</span>)))</code></pre>

Select a validated padding byte, or None for unpadded output.

[Source](/reference-sources/Text.Base64.Encoding.cinch) · line 141

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Text/Base64/Encoding.cinch`. Source SHA-256: `fa7eb02f456cc9bf8edd08b19e96feaf29b6a1d69b848ca9fc2c68813e0d8f01`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
