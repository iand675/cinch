# Text.Punycode

Punycode (RFC 3492) encoding and decoding, and ASCII-compatible host names with xn-- labels.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Text.Punycode`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.Punycode.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bytes](Data.Bytes.md), [Data.List](Data.List.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md).

Non-embedded source closure: [Text.Punycode](Text.Punycode.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Punycode (RFC 3492): the bootstring encoding of Unicode labels in the ASCII letters, digits and hyphen
that internationalized domain names use, and the "xn--" per-label conversion of host names.

Encoding copies a label's ASCII code points, adds a hyphen when there were any, and then emits the
variable-length deltas that insert each remaining code point in increasing order, adapting the bias
after every insertion. Decoding runs the same state machine backwards and refuses digits that are not
base-36, deltas that overflow 32 bits and code points beyond U+10FFFF. Case folding and the other
IDNA mapping steps are not applied.

</details>

# Types and constructors

<a id="type-50756e79636f64654572726f72"></a>

## `PunycodeError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">PunycodeError</span></code></pre>

Why a label could not be converted.

[Source](/reference-sources/Text.Punycode.cinch) · line 21

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PunycodeOverflow</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Punycode</span>.<span class="tok-constructor">PunycodeError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PunycodeBadDigit</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Punycode</span>.<span class="tok-constructor">PunycodeError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PunycodeBadInput</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Punycode</span>.<span class="tok-constructor">PunycodeError</span></code></pre>

# Values

<a id="value-70756e79636f64652d6465636f6465"></a>

## `punycode-decode`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">punycode-decode</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Punycode</span>.<span class="tok-constructor">PunycodeError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

Decode a label (without the "xn--" prefix).

[Source](/reference-sources/Text.Punycode.cinch) · line 202

<a id="value-70756e79636f64652d656e636f6465"></a>

## `punycode-encode`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">punycode-encode</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Punycode</span>.<span class="tok-constructor">PunycodeError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

Encode a label (without the "xn--" prefix).

[Source](/reference-sources/Text.Punycode.cinch) · line 141

<a id="value-70756e79636f64652d6572726f722d6d657373616765"></a>

## `punycode-error-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">punycode-error-message</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Punycode</span>.<span class="tok-constructor">PunycodeError</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

A readable description of an error.

[Source](/reference-sources/Text.Punycode.cinch) · line 30

<a id="value-70756e79636f64652d746f2d6173636969"></a>

## `punycode-to-ascii`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">punycode-to-ascii</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Punycode</span>.<span class="tok-constructor">PunycodeError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

A host name with every non-ASCII label encoded as "xn--" and its Punycode.

[Source](/reference-sources/Text.Punycode.cinch) · line 267

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Text/Punycode.cinch`. Source SHA-256: `c27491d86e8dfe07722cadc036211354ecff896d53959e94b6d534e7a72da3b6`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
