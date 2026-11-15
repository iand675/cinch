# Text.Base32

RFC 4648 standard and extended-hex Base32, with padded and raw forms and named decoding errors.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Text.Base32`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.Base32.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Char](Data.Char.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md).

Non-embedded source closure: [Data.Bytes.Word](Data.Bytes.Word.md), [Text.Base32](Text.Base32.md).

<details><summary>Behaviour and boundary notes from the source</summary>

RFC 4648 Base32 with standard and extended-hex alphabets.
Both padded and raw encodings are available. Decoding ignores CR and LF, accepts
uppercase alphabet characters only, and validates the complete final quantum.
Unused low bits of the final symbol are discarded. No partial decoded buffer
is returned on error. Complete quanta use inline LLVM SIMD lanes; padding,
newlines, malformed input and short tails use the same scalar error rules.

</details>

# Types and constructors

<a id="type-426173653332416c706861626574"></a>

## `Base32Alphabet`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Base32Alphabet</span></code></pre>

The fixed 32-symbol alphabet for a Base32 encoding.

[Source](/reference-sources/Text.Base32.cinch) · line 25

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Base32Standard</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Base32</span>.<span class="tok-constructor">Base32Alphabet</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Base32Hex</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Base32</span>.<span class="tok-constructor">Base32Alphabet</span></code></pre>

<a id="type-4261736533324572726f72"></a>

## `Base32Error`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Base32Error</span></code></pre>

Malformed Base32 text; an error carries no partial decoded output.

[Source](/reference-sources/Text.Base32.cinch) · line 32

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">InvalidBase32Character</span> :: (<span class="tok-constructor">Char</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Base32</span>.<span class="tok-constructor">Base32Error</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">InvalidBase32Length</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Base32</span>.<span class="tok-constructor">Base32Error</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">InvalidBase32Padding</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Base32</span>.<span class="tok-constructor">Base32Error</span></code></pre>

# Values

<a id="value-6261736533322d6465636f6465"></a>

## `base32-decode`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">base32-decode</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Base32</span>.<span class="tok-constructor">Base32Alphabet</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Base32</span>.<span class="tok-constructor">Base32Error</span> <span class="tok-constructor">Bytes</span>)))</code></pre>

Decode padded Base32, ignoring CR/LF. Incomplete quanta, misplaced padding, lowercase and unknown symbols return named errors.

[Source](/reference-sources/Text.Base32.cinch) · line 135

<a id="value-6261736533322d656e636f6465"></a>

## `base32-encode`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">base32-encode</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Base32</span>.<span class="tok-constructor">Base32Alphabet</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

Encode with the selected alphabet and enough equals signs to complete an eight-character quantum. Empty input stays empty.

[Source](/reference-sources/Text.Base32.cinch) · line 90

<a id="value-6261736533322d7261772d6465636f6465"></a>

## `base32-raw-decode`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">base32-raw-decode</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Base32</span>.<span class="tok-constructor">Base32Alphabet</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Base32</span>.<span class="tok-constructor">Base32Error</span> <span class="tok-constructor">Bytes</span>)))</code></pre>

Decode unpadded Base32, ignoring CR/LF. Padding and impossible final-quantum lengths are errors.

[Source](/reference-sources/Text.Base32.cinch) · line 139

<a id="value-6261736533322d7261772d656e636f6465"></a>

## `base32-raw-encode`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">base32-raw-encode</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Base32</span>.<span class="tok-constructor">Base32Alphabet</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

Encode with the selected alphabet and no padding. The output length is ceil(8 * byte-count / 5).

[Source](/reference-sources/Text.Base32.cinch) · line 81

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Text/Base32.cinch`. Source SHA-256: `1497ed5e0adb806c86a2626c9719afdec1e83825331d6773d4e812e09fbd5569`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
