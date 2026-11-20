# Text.Base32

Configurable Base32 byte alphabets and padding, SIMD codecs, prefix/caller-buffer operations and persistent incremental states.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Text.Base32`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.Base32.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Char](Data.Char.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md).

Non-embedded source closure: [Data.Bytes.Word](Data.Bytes.Word.md), [Text.Base32](Text.Base32.md).

<details><summary>Behaviour and boundary notes from the source</summary>

RFC 4648 Base32 with fixed or custom byte alphabets and optional byte padding.
Fixed-alphabet text conveniences accept uppercase symbols and return atomic
errors. Configurable byte APIs preserve complete output before a later error,
count positions in original bytes and provide persistent encoder/decoder state.
CR and LF are ignored. Final lengths, padding and trailing data are validated;
unused low symbol bits are discarded. Complete quanta use inline LLVM SIMD
and SWAR; custom lookup tables are private, immutable copies of caller bytes.

</details>

# Types and constructors

<a id="type-426173653332416c706861626574"></a>

## `Base32Alphabet`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Base32Alphabet</span></code></pre>

The fixed 32-symbol alphabet for a Base32 encoding.

[Source](/reference-sources/Text.Base32.cinch) · line 34

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Base32Standard</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Base32</span>.<span class="tok-constructor">Base32Alphabet</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Base32Hex</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Base32</span>.<span class="tok-constructor">Base32Alphabet</span></code></pre>

<a id="type-426173653332436f6e6669674572726f72"></a>

## `Base32ConfigError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Base32ConfigError</span></code></pre>

Invalid alphabet or padding configuration; construction never changes caller storage.

[Source](/reference-sources/Text.Base32.cinch) · line 450

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Base32AlphabetLength</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Base32</span>.<span class="tok-constructor">Base32ConfigError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Base32AlphabetNewline</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Base32</span>.<span class="tok-constructor">Base32ConfigError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Base32AlphabetDuplicate</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Base32</span>.<span class="tok-constructor">Base32ConfigError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Base32PaddingValue</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Base32</span>.<span class="tok-constructor">Base32ConfigError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Base32PaddingCollision</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Base32</span>.<span class="tok-constructor">Base32ConfigError</span>)</code></pre>

<a id="type-4261736533324465636f646572"></a>

## `Base32Decoder`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Base32Decoder</span></code></pre>

Persistent decoding continuation with a packed unfinished quantum and sticky failure.

[Source](/reference-sources/Text.Base32.cinch) · line 714

<a id="type-426173653332456e636f646572"></a>

## `Base32Encoder`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Base32Encoder</span></code></pre>

Persistent encoding continuation owning fewer than five pending input bytes.

[Source](/reference-sources/Text.Base32.cinch) · line 690

<a id="type-426173653332456e636f64696e67"></a>

## `Base32Encoding`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Base32Encoding</span></code></pre>

An immutable byte alphabet, inverse lookup and optional one-byte padding policy.
Custom alphabets are copied. Fixed configurations use the existing SIMD classifiers.

[Source](/reference-sources/Text.Base32.cinch) · line 445

<a id="type-4261736533324572726f72"></a>

## `Base32Error`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Base32Error</span></code></pre>

Malformed input for the fixed-alphabet text conveniences; errors carry no partial output.

[Source](/reference-sources/Text.Base32.cinch) · line 41

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">InvalidBase32Character</span> :: (<span class="tok-constructor">Char</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Base32</span>.<span class="tok-constructor">Base32Error</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">InvalidBase32Length</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Base32</span>.<span class="tok-constructor">Base32Error</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">InvalidBase32Padding</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Base32</span>.<span class="tok-constructor">Base32Error</span></code></pre>

<a id="type-426173653332496e7075744572726f72"></a>

## `Base32InputError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Base32InputError</span></code></pre>

Invalid encoded bytes, an incomplete final quantum, or insufficient destination capacity.
Input positions count original bytes, including ignored CR and LF.

[Source](/reference-sources/Text.Base32.cinch) · line 657

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">InvalidBase32Byte</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Base32</span>.<span class="tok-constructor">Base32InputError</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">TruncatedBase32Quantum</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Base32</span>.<span class="tok-constructor">Base32InputError</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Base32ShortDestination</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Base32</span>.<span class="tok-constructor">Base32InputError</span>))</code></pre>

# Values

<a id="value-6261736533322d636f6e6669672d6572726f722d6d657373616765"></a>

## `base32-config-error-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">base32-config-error-message</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Base32</span>.<span class="tok-constructor">Base32ConfigError</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Describe the precise alphabet offset or rejected padding value.

[Source](/reference-sources/Text.Base32.cinch) · line 463

<a id="value-6261736533322d6465636f6465"></a>

## `base32-decode`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">base32-decode</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Base32</span>.<span class="tok-constructor">Base32Alphabet</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Base32</span>.<span class="tok-constructor">Base32Error</span> <span class="tok-constructor">Bytes</span>)))</code></pre>

Decode padded Base32, ignoring CR/LF. Incomplete quanta, misplaced padding, lowercase and unknown symbols return named errors.

[Source](/reference-sources/Text.Base32.cinch) · line 198

<a id="value-6261736533322d6465636f64652d617070656e64"></a>

## `base32-decode-append`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">base32-decode-append</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Base32</span>.<span class="tok-constructor">Base32Encoding</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span>, (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Base32</span>.<span class="tok-constructor">Base32InputError</span>)))))</code></pre>

Append completed decoded bytes to a copied prefix, retaining a later malformed-input error.

[Source](/reference-sources/Text.Base32.cinch) · line 831

<a id="value-6261736533322d6465636f64652d6279746573"></a>

## `base32-decode-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">base32-decode-bytes</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Base32</span>.<span class="tok-constructor">Base32Encoding</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Base32</span>.<span class="tok-constructor">Base32InputError</span> <span class="tok-constructor">Bytes</span>)))</code></pre>

Decode complete bytes atomically; use the prefix API when malformed-input progress is needed.

[Source](/reference-sources/Text.Base32.cinch) · line 821

<a id="value-6261736533322d6465636f64652d696e746f"></a>

## `base32-decode-into`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">base32-decode-into</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Base32</span>.<span class="tok-constructor">Base32Encoding</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span>, (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Base32</span>.<span class="tok-constructor">Base32InputError</span>))))))</code></pre>

Decode into a checked destination and return progress plus a possible input error.
The actual completed output must fit before mutation. Decoding completes before
copying, so overlap is safe and valid padding never requires excess capacity.

[Source](/reference-sources/Text.Base32.cinch) · line 838

<a id="value-6261736533322d6465636f64652d707265666978"></a>

## `base32-decode-prefix`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">base32-decode-prefix</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Base32</span>.<span class="tok-constructor">Base32Encoding</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span>, (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Base32</span>.<span class="tok-constructor">Base32InputError</span>))))</code></pre>

Decode bytes with precise original offsets and retain all preceding complete output on error.

[Source](/reference-sources/Text.Base32.cinch) · line 815

<a id="value-6261736533322d6465636f64652d74657874"></a>

## `base32-decode-text`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">base32-decode-text</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Base32</span>.<span class="tok-constructor">Base32Encoding</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Bytes</span>, (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Base32</span>.<span class="tok-constructor">Base32InputError</span>))))</code></pre>

Decode UTF-8 text while preserving complete output and counting original UTF-8 byte offsets.

[Source](/reference-sources/Text.Base32.cinch) · line 827

<a id="value-6261736533322d6465636f6465642d6c656e677468"></a>

## `base32-decoded-length`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">base32-decoded-length</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Base32</span>.<span class="tok-constructor">Base32Encoding</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Nat</span>))</code></pre>

Maximum decoded byte count from an encoded byte count; this does not validate input.
Newlines and padding can make the actual result shorter than this capacity bound.

[Source](/reference-sources/Text.Base32.cinch) · line 563

<a id="value-6261736533322d6465636f646572"></a>

## `base32-decoder`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">base32-decoder</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Base32</span>.<span class="tok-constructor">Base32Encoding</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Base32</span>.<span class="tok-constructor">Base32Decoder</span>)</code></pre>

Begin a strict byte decoder. CR/LF are ignored; padding and final lengths are validated.

[Source](/reference-sources/Text.Base32.cinch) · line 719

<a id="value-6261736533322d6465636f6465722d6572726f72"></a>

## `base32-decoder-error`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">base32-decoder-error</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Base32</span>.<span class="tok-constructor">Base32Decoder</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Base32</span>.<span class="tok-constructor">Base32InputError</span>))</code></pre>

Observe a sticky feed failure while retaining any output returned with that feed.

[Source](/reference-sources/Text.Base32.cinch) · line 723

<a id="value-6261736533322d6465636f6465722d66656564"></a>

## `base32-decoder-feed`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">base32-decoder-feed</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Base32</span>.<span class="tok-constructor">Base32Decoder</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span>, <span class="tok-constructor">Text</span>.<span class="tok-constructor">Base32</span>.<span class="tok-constructor">Base32Decoder</span>)))</code></pre>

Emit complete quanta, preserving their output alongside a later sticky decoding failure.
An incomplete padded quantum is held until every required pad is validated.

[Source](/reference-sources/Text.Base32.cinch) · line 728

<a id="value-6261736533322d6465636f6465722d66696e697368"></a>

## `base32-decoder-finish`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">base32-decoder-finish</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Base32</span>.<span class="tok-constructor">Base32Decoder</span> -&gt; (<span class="tok-constructor">Bytes</span>, (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Base32</span>.<span class="tok-constructor">Base32InputError</span>)))</code></pre>

Validate EOF and render an allowed unpadded tail. Lengths 1, 3 and 6 are rejected.
Padded input requires every final pad; any data after a padded final quantum is invalid.

[Source](/reference-sources/Text.Base32.cinch) · line 801

<a id="value-6261736533322d656e636f6465"></a>

## `base32-encode`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">base32-encode</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Base32</span>.<span class="tok-constructor">Base32Alphabet</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

Encode with the selected alphabet and enough equals signs to complete an eight-character quantum. Empty input stays empty.

[Source](/reference-sources/Text.Base32.cinch) · line 111

<a id="value-6261736533322d656e636f64652d617070656e64"></a>

## `base32-encode-append`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">base32-encode-append</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Base32</span>.<span class="tok-constructor">Base32Encoding</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Bytes</span>)))</code></pre>

Append encoded bytes to a copied prefix without mutating or retaining either caller buffer.

[Source](/reference-sources/Text.Base32.cinch) · line 647

<a id="value-6261736533322d656e636f64652d6279746573"></a>

## `base32-encode-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">base32-encode-bytes</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Base32</span>.<span class="tok-constructor">Base32Encoding</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Bytes</span>))</code></pre>

Encode directly into fresh byte storage; arbitrary alphabet and padding bytes are preserved.

[Source](/reference-sources/Text.Base32.cinch) · line 637

<a id="value-6261736533322d656e636f64652d696e746f"></a>

## `base32-encode-into`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">base32-encode-into</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Base32</span>.<span class="tok-constructor">Base32Encoding</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Base32</span>.<span class="tok-constructor">Base32InputError</span> <span class="tok-constructor">Nat</span>)))))</code></pre>

Encode into a complete checked destination interval, returning the written byte count.
Capacity is checked before mutation; a source snapshot makes overlap safe.

[Source](/reference-sources/Text.Base32.cinch) · line 681

<a id="value-6261736533322d656e636f64652d74657874"></a>

## `base32-encode-text`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">base32-encode-text</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Base32</span>.<span class="tok-constructor">Base32Encoding</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)))</code></pre>

Encode as text only when the exact encoded bytes form valid UTF-8.

[Source](/reference-sources/Text.Base32.cinch) · line 643

<a id="value-6261736533322d656e636f6465642d6c656e677468"></a>

## `base32-encoded-length`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">base32-encoded-length</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Base32</span>.<span class="tok-constructor">Base32Encoding</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Nat</span>))</code></pre>

Exact encoded byte count for an input length, including the selected padding policy.

[Source](/reference-sources/Text.Base32.cinch) · line 556

<a id="value-6261736533322d656e636f646572"></a>

## `base32-encoder`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">base32-encoder</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Base32</span>.<span class="tok-constructor">Base32Encoding</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Base32</span>.<span class="tok-constructor">Base32Encoder</span>)</code></pre>

Begin an independent encoding; finish explicitly to emit its final short quantum.

[Source](/reference-sources/Text.Base32.cinch) · line 695

<a id="value-6261736533322d656e636f6465722d66656564"></a>

## `base32-encoder-feed`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">base32-encoder-feed</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Base32</span>.<span class="tok-constructor">Base32Encoder</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span>, <span class="tok-constructor">Text</span>.<span class="tok-constructor">Base32</span>.<span class="tok-constructor">Base32Encoder</span>)))</code></pre>

Emit all complete five-byte quanta, retaining only a copied tail of at most four bytes.

[Source](/reference-sources/Text.Base32.cinch) · line 699

<a id="value-6261736533322d656e636f6465722d66696e697368"></a>

## `base32-encoder-finish`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">base32-encoder-finish</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Base32</span>.<span class="tok-constructor">Base32Encoder</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Render the final short quantum and selected padding without changing the retained state.

[Source](/reference-sources/Text.Base32.cinch) · line 710

<a id="value-6261736533322d656e636f64696e67"></a>

## `base32-encoding`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">base32-encoding</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Base32</span>.<span class="tok-constructor">Base32ConfigError</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Base32</span>.<span class="tok-constructor">Base32Encoding</span>))</code></pre>

Validate 32 distinct non-newline bytes and create an independent padded encoding.
Bytes above 127 retain their exact values. An equals-sign symbol is allowed;
select another padding byte or no padding before requiring an invertible tail.

[Source](/reference-sources/Text.Base32.cinch) · line 495

<a id="value-6261736533322d656e636f64696e672d616c706861626574"></a>

## `base32-encoding-alphabet`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">base32-encoding-alphabet</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Base32</span>.<span class="tok-constructor">Base32Encoding</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Return a fresh complete alphabet buffer, including for custom non-UTF-8 alphabets.

[Source](/reference-sources/Text.Base32.cinch) · line 515

<a id="value-6261736533322d656e636f64696e672d70616464696e67"></a>

## `base32-encoding-padding`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">base32-encoding-padding</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Base32</span>.<span class="tok-constructor">Base32Encoding</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Nat</span>))</code></pre>

Observe the optional padding byte; None selects an unpadded encoding.

[Source](/reference-sources/Text.Base32.cinch) · line 523

<a id="value-6261736533322d6865782d656e636f64696e67"></a>

## `base32-hex-encoding`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">base32-hex-encoding</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Base32</span>.<span class="tok-constructor">Base32Encoding</span></code></pre>

The extended-hex uppercase byte alphabet with equals-sign padding.

[Source](/reference-sources/Text.Base32.cinch) · line 476

<a id="value-6261736533322d696e7075742d6572726f722d6d657373616765"></a>

## `base32-input-error-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">base32-input-error-message</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Base32</span>.<span class="tok-constructor">Base32InputError</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Describe original-byte decoding failures or the complete required destination capacity.

[Source](/reference-sources/Text.Base32.cinch) · line 673

<a id="value-6261736533322d696e7075742d6572726f722d6f6666736574"></a>

## `base32-input-error-offset`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">base32-input-error-offset</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Base32</span>.<span class="tok-constructor">Base32InputError</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Original input position associated with a malformed quantum; capacity errors return zero.

[Source](/reference-sources/Text.Base32.cinch) · line 666

<a id="value-6261736533322d7261772d6465636f6465"></a>

## `base32-raw-decode`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">base32-raw-decode</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Base32</span>.<span class="tok-constructor">Base32Alphabet</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Base32</span>.<span class="tok-constructor">Base32Error</span> <span class="tok-constructor">Bytes</span>)))</code></pre>

Decode unpadded Base32, ignoring CR/LF. Padding and impossible final-quantum lengths are errors.

[Source](/reference-sources/Text.Base32.cinch) · line 202

<a id="value-6261736533322d7261772d656e636f6465"></a>

## `base32-raw-encode`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">base32-raw-encode</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Base32</span>.<span class="tok-constructor">Base32Alphabet</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

Encode with the selected alphabet and no padding. The output length is ceil(8 * byte-count / 5).

[Source](/reference-sources/Text.Base32.cinch) · line 102

<a id="value-6261736533322d7374616e646172642d656e636f64696e67"></a>

## `base32-standard-encoding`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">base32-standard-encoding</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Base32</span>.<span class="tok-constructor">Base32Encoding</span></code></pre>

The standard uppercase byte alphabet with equals-sign padding.

[Source](/reference-sources/Text.Base32.cinch) · line 472

<a id="value-6261736533322d776974682d70616464696e67"></a>

## `base32-with-padding`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">base32-with-padding</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Base32</span>.<span class="tok-constructor">Base32Encoding</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Nat</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Base32</span>.<span class="tok-constructor">Base32ConfigError</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Base32</span>.<span class="tok-constructor">Base32Encoding</span>)))</code></pre>

Copy an encoding with validated one-byte padding, or disable padding with None.
Newline values, values above 255 and alphabet collisions return named errors.

[Source](/reference-sources/Text.Base32.cinch) · line 542

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Text/Base32.cinch`. Source SHA-256: `e901c7d08842d6580c765eefac5ad5d7ab48078c6fc46dbb963e3abafd55acbd`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
