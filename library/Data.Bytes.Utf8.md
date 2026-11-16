# Data.Bytes.Utf8

Raw UTF-8 validation, forward and reverse decoding, scalar counting, fresh repair and atomic encoding.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Data.Bytes.Utf8`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Bytes.Utf8.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Char](Data.Char.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md).

Non-embedded source closure: [Data.Bytes.Utf8](Data.Bytes.Utf8.md), [Data.Bytes.Word](Data.Bytes.Word.md).

<details><summary>Behaviour and boundary notes from the source</summary>

UTF-8 decoding, validation, scalar encoding and fresh byte repair.
Invalid or truncated encodings consume exactly one byte as U+FFFD. End and
out-of-bounds positions consume zero bytes and have distinct statuses. Numeric
rune operations accept scalar values only; encoding invalid values emits U+FFFD.
Repair groups consecutive malformed bytes by default; repair-each replaces each
malformed byte. Replacements are arbitrary bytes and every returned buffer is
fresh, including empty or already valid input. Complete ASCII blocks use SWAR;
non-ASCII fringes enforce shortest encoding, scalar bounds and exact input loads.

</details>

# Types and constructors

<a id="type-55746638537461747573"></a>

## `Utf8Status`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Utf8Status</span></code></pre>

Whether a decoding position is valid, malformed, exactly at the end, or beyond the buffer.

[Source](/reference-sources/Data.Bytes.Utf8.cinch) · line 25

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Utf8End</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">Utf8</span>.<span class="tok-constructor">Utf8Status</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Utf8Valid</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">Utf8</span>.<span class="tok-constructor">Utf8Status</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Utf8Invalid</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">Utf8</span>.<span class="tok-constructor">Utf8Status</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Utf8OutOfBounds</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">Utf8</span>.<span class="tok-constructor">Utf8Status</span></code></pre>

<a id="type-5574663853746570"></a>

## `Utf8Step`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Utf8Step</span></code></pre>

A decoded scalar, exact consumed byte width and explicit position status.

[Source](/reference-sources/Data.Bytes.Utf8.cinch) · line 36

# Values

<a id="value-757466382d617070656e642d72756e65"></a>

## `utf8-append-rune`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">utf8-append-rune</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">Bytes</span>))</code></pre>

Append one encoded numeric rune to a fresh copy of arbitrary source bytes.

[Source](/reference-sources/Data.Bytes.Utf8.cinch) · line 421

<a id="value-757466382d636f756e74"></a>

## `utf8-count`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">utf8-count</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Count scalars with exact one-byte progress on all malformed input and SWAR ASCII blocks.

[Source](/reference-sources/Data.Bytes.Utf8.cinch) · line 325

<a id="value-757466382d6465636f64652d6174"></a>

## `utf8-decode-at`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">utf8-decode-at</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">Utf8</span>.<span class="tok-constructor">Utf8Step</span>))</code></pre>

Decode at a byte offset; a malformed nonempty suffix always consumes exactly one byte.

[Source](/reference-sources/Data.Bytes.Utf8.cinch) · line 251

<a id="value-757466382d656e636f64652d696e746f"></a>

## `utf8-encode-into`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">utf8-encode-into</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">I32</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Nat</span>))))</code></pre>

Encode into an existing buffer, returning written width; insufficient capacity or an invalid offset writes nothing.

[Source](/reference-sources/Data.Bytes.Utf8.cinch) · line 411

<a id="value-757466382d656e636f64652d72756e65"></a>

## `utf8-encode-rune`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">utf8-encode-rune</span> :: (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Encode a numeric rune into fresh bytes, replacing invalid scalar inputs with U+FFFD.

[Source](/reference-sources/Data.Bytes.Utf8.cinch) · line 416

<a id="value-757466382d6669727374"></a>

## `utf8-first`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">utf8-first</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">Utf8</span>.<span class="tok-constructor">Utf8Step</span>)</code></pre>

Decode the first scalar or one malformed byte; an empty buffer has Utf8End status.

[Source](/reference-sources/Data.Bytes.Utf8.cinch) · line 259

<a id="value-757466382d66697273742d696e76616c6964"></a>

## `utf8-first-invalid`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">utf8-first-invalid</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Nat</span>))</code></pre>

Locate the first malformed byte, preserving end as None and never copying input.

[Source](/reference-sources/Data.Bytes.Utf8.cinch) · line 301

<a id="value-757466382d66756c6c2d72756e65"></a>

## `utf8-full-rune`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">utf8-full-rune</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Whether a nonempty prefix already determines a decoding step, valid or malformed.
A potentially valid but incomplete sequence is false; an already invalid prefix is true.

[Source](/reference-sources/Data.Bytes.Utf8.cinch) · line 331

<a id="value-757466382d6c617374"></a>

## `utf8-last`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">utf8-last</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">Utf8</span>.<span class="tok-constructor">Utf8Step</span>)</code></pre>

Decode the final scalar or one final malformed byte; empty input has Utf8End status.

[Source](/reference-sources/Data.Bytes.Utf8.cinch) · line 283

<a id="value-757466382d6c6173742d6265666f7265"></a>

## `utf8-last-before`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">utf8-last-before</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">Utf8</span>.<span class="tok-constructor">Utf8Step</span>))</code></pre>

Decode immediately before a byte offset; malformed endings consume exactly the final byte.

[Source](/reference-sources/Data.Bytes.Utf8.cinch) · line 274

<a id="value-757466382d6d61782d72756e65"></a>

## `utf8-max-rune`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">utf8-max-rune</span> :: <span class="tok-constructor">U32</span></code></pre>

The greatest Unicode code point, which is a valid scalar.

[Source](/reference-sources/Data.Bytes.Utf8.cinch) · line 61

<a id="value-757466382d6d61782d7769647468"></a>

## `utf8-max-width`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">utf8-max-width</span> :: <span class="tok-constructor">Nat</span></code></pre>

Maximum bytes in a scalar's shortest UTF-8 encoding.

[Source](/reference-sources/Data.Bytes.Utf8.cinch) · line 65

<a id="value-757466382d726570616972"></a>

## `utf8-repair`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">utf8-repair</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Bytes</span>))</code></pre>

Replace each maximal run of malformed bytes with one arbitrary byte replacement; output is always fresh.

[Source](/reference-sources/Data.Bytes.Utf8.cinch) · line 464

<a id="value-757466382d7265706169722d65616368"></a>

## `utf8-repair-each`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">utf8-repair-each</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Bytes</span>))</code></pre>

Replace every malformed byte separately, retaining exact one-byte decoding progress; output is always fresh.

[Source](/reference-sources/Data.Bytes.Utf8.cinch) · line 468

<a id="value-757466382d7265706c6163652d696e76616c6964"></a>

## `utf8-replace-invalid`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">utf8-replace-invalid</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Repair every malformed byte as U+FFFD, producing fresh valid UTF-8 bytes.

[Source](/reference-sources/Data.Bytes.Utf8.cinch) · line 472

<a id="value-757466382d72756e652d6572726f72"></a>

## `utf8-rune-error`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">utf8-rune-error</span> :: <span class="tok-constructor">Char</span></code></pre>

The scalar emitted for a malformed byte or invalid numeric encoding input.

[Source](/reference-sources/Data.Bytes.Utf8.cinch) · line 53

<a id="value-757466382d72756e652d6c656e677468"></a>

## `utf8-rune-length`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">utf8-rune-length</span> :: (<span class="tok-constructor">I32</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Nat</span>))</code></pre>

Return the exact encoded byte width, or None for an invalid numeric rune.

[Source](/reference-sources/Data.Bytes.Utf8.cinch) · line 343

<a id="value-757466382d72756e652d73656c66"></a>

## `utf8-rune-self`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">utf8-rune-self</span> :: <span class="tok-constructor">U32</span></code></pre>

The first byte requiring a multibyte encoding.

[Source](/reference-sources/Data.Bytes.Utf8.cinch) · line 57

<a id="value-757466382d72756e652d7374617274"></a>

## `utf8-rune-start`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">utf8-rune-start</span> :: (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Test whether a byte can start an encoding, including malformed leading bytes.

[Source](/reference-sources/Data.Bytes.Utf8.cinch) · line 263

<a id="value-757466382d737465702d63686172"></a>

## `utf8-step-char`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">utf8-step-char</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">Utf8</span>.<span class="tok-constructor">Utf8Step</span> -&gt; <span class="tok-constructor">Char</span>)</code></pre>

Observe the decoded scalar, or U+FFFD for an invalid/end/out-of-bounds step.

[Source](/reference-sources/Data.Bytes.Utf8.cinch) · line 41

<a id="value-757466382d737465702d737461747573"></a>

## `utf8-step-status`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">utf8-step-status</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">Utf8</span>.<span class="tok-constructor">Utf8Step</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">Utf8</span>.<span class="tok-constructor">Utf8Status</span>)</code></pre>

Observe whether the position represents valid input, a malformed byte, end, or an invalid offset.

[Source](/reference-sources/Data.Bytes.Utf8.cinch) · line 49

<a id="value-757466382d737465702d7769647468"></a>

## `utf8-step-width`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">utf8-step-width</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">Utf8</span>.<span class="tok-constructor">Utf8Step</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Observe consumed bytes: one through four for valid input, one for malformed input, zero at end or beyond.

[Source](/reference-sources/Data.Bytes.Utf8.cinch) · line 45

<a id="value-757466382d76616c6964"></a>

## `utf8-valid`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">utf8-valid</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Whether every byte participates in a shortest-form scalar encoding; empty input is valid.

[Source](/reference-sources/Data.Bytes.Utf8.cinch) · line 307

<a id="value-757466382d76616c69642d72756e65"></a>

## `utf8-valid-rune`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">utf8-valid-rune</span> :: (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Whether a numeric rune is a Unicode scalar; negatives, surrogates and values above U+10FFFF are false.

[Source](/reference-sources/Data.Bytes.Utf8.cinch) · line 339

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Data/Bytes/Utf8.cinch`. Source SHA-256: `a5f6a39e59b512862fe88e984a8fb5bef0cc6fc47516f59cc488d4493a523b9a`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
