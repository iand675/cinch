# Data.Uuid

Immutable 128-bit identifiers with RFC 9562 text, ordering and explicit version 7 state.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Data.Uuid`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Uuid.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bytes](Data.Bytes.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Core](Data.Core.md), [Data.Hash](Data.Hash.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Ordering](Data.Ordering.md), [Data.String](Data.String.md), [Relation.Eq](Relation.Eq.md), [Relation.Ord](Relation.Ord.md), [Text.Hex](Text.Hex.md).

Non-embedded source closure: [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Uuid](Data.Uuid.md), [Text.Hex](Text.Hex.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Immutable 128-bit identifiers with RFC 9562 text, ordering and explicit version 7 state.
Every bit pattern is a valid value. Parsing validates only the accepted text
forms; version and variant bits are set only by generation functions. Values
contain two network-order words and never retain a mutable byte-buffer alias.

</details>

# Types and constructors

<a id="type-50617273656455756964"></a>

## `ParsedUuid`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">ParsedUuid</span></code></pre>

A successfully checked text value, retaining the decoded identifier rather than its input buffer.

[Source](/reference-sources/Data.Uuid.cinch) · line 44

<a id="type-55756964"></a>

## `Uuid`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Uuid</span></code></pre>

Two unsigned words in network significance order, with no mutable backing store.

[Source](/reference-sources/Data.Uuid.cinch) · line 26

<a id="type-557569644572726f72"></a>

## `UuidError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">UuidError</span></code></pre>

A checked text, buffer, time or logical-clock refusal.

[Source](/reference-sources/Data.Uuid.cinch) · line 31

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">UuidInvalidLength</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Uuid</span>.<span class="tok-constructor">UuidError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">UuidInvalidText</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Uuid</span>.<span class="tok-constructor">UuidError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">UuidOutputTooLarge</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Uuid</span>.<span class="tok-constructor">UuidError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">UuidClockOutOfRange</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Uuid</span>.<span class="tok-constructor">UuidError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">UuidClockExhausted</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Uuid</span>.<span class="tok-constructor">UuidError</span></code></pre>

<a id="type-5575696456375374617465"></a>

## `UuidV7State`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">UuidV7State</span></code></pre>

Explicit version 7 ordering history for one independent generation sequence.

[Source](/reference-sources/Data.Uuid.cinch) · line 49

# Values

<a id="value-757569642d617070656e642d74657874"></a>

## `uuid-append-text`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">uuid-append-text</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Uuid</span>.<span class="tok-constructor">Uuid</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Uuid</span>.<span class="tok-constructor">UuidError</span> <span class="tok-constructor">Bytes</span>)))</code></pre>

Copy an arbitrary prefix and append canonical text without exposing mutable aliases or partial output.

[Source](/reference-sources/Data.Uuid.cinch) · line 183

<a id="value-757569642d636f6d70617265"></a>

## `uuid-compare`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">uuid-compare</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Uuid</span>.<span class="tok-constructor">Uuid</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Uuid</span>.<span class="tok-constructor">Uuid</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Ordering</span>.<span class="tok-constructor">Ordering</span>))</code></pre>

Compare the full 128-bit network order, including reserved and unfamiliar version bits.

[Source](/reference-sources/Data.Uuid.cinch) · line 79

<a id="value-757569642d657175616c"></a>

## `uuid-equal`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">uuid-equal</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Uuid</span>.<span class="tok-constructor">Uuid</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Uuid</span>.<span class="tok-constructor">Uuid</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

Test complete identifier equality without pointer identity shortcuts.

[Source](/reference-sources/Data.Uuid.cinch) · line 88

<a id="value-757569642d666f726d6174"></a>

## `uuid-format`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">uuid-format</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Uuid</span>.<span class="tok-constructor">Uuid</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Return the canonical 36-byte lowercase string; all generated bytes are valid ASCII.

[Source](/reference-sources/Data.Uuid.cinch) · line 175

<a id="value-757569642d66726f6d2d6279746573"></a>

## `uuid-from-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">uuid-from-bytes</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Uuid</span>.<span class="tok-constructor">UuidError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Uuid</span>.<span class="tok-constructor">Uuid</span>))</code></pre>

Read exactly 16 bytes and copy their bits into an immutable value.

[Source](/reference-sources/Data.Uuid.cinch) · line 92

<a id="value-757569642d66726f6d2d776f726473"></a>

## `uuid-from-words`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">uuid-from-words</span> :: (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Uuid</span>.<span class="tok-constructor">Uuid</span>))</code></pre>

Construct any identifier value directly from its two network-order words.

[Source](/reference-sources/Data.Uuid.cinch) · line 54

<a id="value-757569642d6d61727368616c2d74657874"></a>

## `uuid-marshal-text`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">uuid-marshal-text</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Uuid</span>.<span class="tok-constructor">Uuid</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Encode all bits with existing SIMD hex lanes and fixed lowercase canonical punctuation.

[Source](/reference-sources/Data.Uuid.cinch) · line 168

<a id="value-757569642d6d6178"></a>

## `uuid-max`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">uuid-max</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Uuid</span>.<span class="tok-constructor">Uuid</span></code></pre>

The all-one identifier; no version restriction excludes this sentinel value.

[Source](/reference-sources/Data.Uuid.cinch) · line 62

<a id="value-757569642d6e696c"></a>

## `uuid-nil`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">uuid-nil</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Uuid</span>.<span class="tok-constructor">Uuid</span></code></pre>

The all-zero identifier, independent of any optional or absent carrier.

[Source](/reference-sources/Data.Uuid.cinch) · line 58

<a id="value-757569642d7061727365"></a>

## `uuid-parse`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">uuid-parse</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Uuid</span>.<span class="tok-constructor">UuidError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Uuid</span>.<span class="tok-constructor">Uuid</span>))</code></pre>

Parse text atomically; hexadecimal letter case is accepted without validating version or variant bits.

[Source](/reference-sources/Data.Uuid.cinch) · line 137

<a id="value-757569642d70617273652d6279746573"></a>

## `uuid-parse-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">uuid-parse-bytes</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Uuid</span>.<span class="tok-constructor">UuidError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Uuid</span>.<span class="tok-constructor">Uuid</span>))</code></pre>

Parse canonical, brace-wrapped, exact lowercase URN or compact ASCII forms without trimming.

[Source](/reference-sources/Data.Uuid.cinch) · line 125

<a id="value-757569642d7265717569726564"></a>

## `uuid-required`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">uuid-required</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Uuid</span>.<span class="tok-constructor">ParsedUuid</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Uuid</span>.<span class="tok-constructor">Uuid</span>)</code></pre>

Extract a previously checked identifier without a partial parser or runtime panic.

[Source](/reference-sources/Data.Uuid.cinch) · line 151

<a id="value-757569642d746f2d6279746573"></a>

## `uuid-to-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">uuid-to-bytes</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Uuid</span>.<span class="tok-constructor">Uuid</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Return a fresh 16-byte network-order buffer; mutating it cannot change the identifier.

[Source](/reference-sources/Data.Uuid.cinch) · line 99

<a id="value-757569642d756e6d61727368616c2d74657874"></a>

## `uuid-unmarshal-text`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">uuid-unmarshal-text</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Uuid</span>.<span class="tok-constructor">Uuid</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Uuid</span>.<span class="tok-constructor">Uuid</span>, (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Uuid</span>.<span class="tok-constructor">UuidError</span>))))</code></pre>

Replace an immutable receiver on success and preserve every original bit on malformed text.

[Source](/reference-sources/Data.Uuid.cinch) · line 141

<a id="value-757569642d76342d66726f6d2d72616e646f6d"></a>

## `uuid-v4-from-random`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">uuid-v4-from-random</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Uuid</span>.<span class="tok-constructor">UuidError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Uuid</span>.<span class="tok-constructor">Uuid</span>))</code></pre>

Set version 4 and RFC variant10 while preserving the other 122 supplied entropy bits.

[Source](/reference-sources/Data.Uuid.cinch) · line 194

<a id="value-757569642d76372d616476616e6365"></a>

## `uuid-v7-advance`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">uuid-v7-advance</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Uuid</span>.<span class="tok-constructor">UuidV7State</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Uuid</span>.<span class="tok-constructor">UuidError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Uuid</span>.<span class="tok-constructor">UuidV7State</span>, <span class="tok-constructor">U64</span>))))</code></pre>

Advance the 60-bit timestamp, retaining same-second rollback ordering and resetting only on whole-second rollback.

[Source](/reference-sources/Data.Uuid.cinch) · line 205

<a id="value-757569642d76372d66726f6d2d7469636b"></a>

## `uuid-v7-from-tick`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">uuid-v7-from-tick</span> :: (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Uuid</span>.<span class="tok-constructor">UuidError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Uuid</span>.<span class="tok-constructor">Uuid</span>)))</code></pre>

Combine a checked logical timestamp with eight supplied random bytes, retaining 62 random bits.

[Source](/reference-sources/Data.Uuid.cinch) · line 220

<a id="value-757569642d76372d696e697469616c"></a>

## `uuid-v7-initial`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">uuid-v7-initial</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Uuid</span>.<span class="tok-constructor">UuidV7State</span></code></pre>

Begin one independent version 7 sequence without a hidden shared process-global counter.

[Source](/reference-sources/Data.Uuid.cinch) · line 201

<a id="value-757569642d76372d73746570"></a>

## `uuid-v7-step`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">uuid-v7-step</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Uuid</span>.<span class="tok-constructor">UuidV7State</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Uuid</span>.<span class="tok-constructor">UuidError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Uuid</span>.<span class="tok-constructor">UuidV7State</span>, <span class="tok-constructor">Data</span>.<span class="tok-constructor">Uuid</span>.<span class="tok-constructor">Uuid</span>)))))</code></pre>

Run a pure version 7 transition with explicit time and entropy; refusal returns no replacement state.

[Source](/reference-sources/Data.Uuid.cinch) · line 231

<a id="value-757569642d76616c69646174652d74657874"></a>

## `uuid-validate-text`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">uuid-validate-text</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Uuid</span>.<span class="tok-constructor">UuidError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Uuid</span>.<span class="tok-constructor">ParsedUuid</span>))</code></pre>

Check a trusted-text witness explicitly, retaining ordinary parse errors for invalid input.

[Source](/reference-sources/Data.Uuid.cinch) · line 147

# Instances

These are checked exported instance entries. Law/sample-check status is separate from the existence of an instance; see [laws and optimization](../reference/12-rewrites-and-optimization.md).

| Instance | Target | Resolution |
| --- | --- | --- |
| `Data.Uuid.Eq_Uuid` | `Relation.Eq.Eq Data.Uuid.Uuid` | Implicit |
| `Data.Uuid.Hash_Uuid` | `Data.Hash.Hash Data.Uuid.Uuid` | Implicit |
| `Data.Uuid.Ord_Uuid` | `Relation.Ord.Ord Data.Uuid.Uuid` | Implicit |

**`Data.Uuid.Eq_Uuid`**: Complete-bit equality for identifier values.
**`Data.Uuid.Hash_Uuid`**: Hash both complete words; hash collisions never replace identifier equality.
**`Data.Uuid.Ord_Uuid`**: Unsigned network-order comparison, consistent with complete-bit equality.

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Data/Uuid.cinch`. Source SHA-256: `97dfcba5728a8e24a81fd3fc12ef37cccb5f111b7a228c970c33a3526e64e94d`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
