# Data.String

Immutable UTF-8 text, encoding/decoding, character access, slicing, comparison, and concatenation. Byte conversions copy to preserve immutability.

<span class="support-tag support-tag--current">Embedded</span> Available from the compiler's embedded base family.

Import with `import Data.String`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.String.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Algebra.Arithmetic](Algebra.Arithmetic.md), [Algebra.Monoid](Algebra.Monoid.md), [Algebra.Semigroup](Algebra.Semigroup.md), [Data.Bytes](Data.Bytes.md), [Data.Char](Data.Char.md), [Data.Hash](Data.Hash.md), [Data.List](Data.List.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Data.String: runtime strings, UTF-8 over Bytes.

`String` is the runtime text type (decision D88). It is distinct
from the staging interpreter's compile-time string values, which
are the same type observed at compile time (D43's deferral
resolved). A `"..."` literal in a term is a runtime `String`: the
lexer's decoded bytes in one shared read-only data-section constant
(immutability makes the sharing semantics-transparent: every
evaluation references the same object and nothing allocates).

THE IMMUTABILITY CONTRACT: a `String` never changes and no API ever
exposes its backing mutably. Conversions copy on the way in
(`string-from-bytes`) and on the way out (`string-to-bytes`);
mutation exists only on plain `Bytes`. The representation
constructor and the backing reader are module-private, so once a
buffer is wrapped no alias can mutate it, and a literal's backing
(the read-only data-section constant itself) never reaches a write:
outside this module it cannot be named, and in it every use of a
backing is a read or the source of a copy.

The functions here are the REFERENCE SEMANTICS for UTF-8. The
runtime provides SIMD-accelerated implementations of the same rules
(`cinch_utf8_*` in `compiler/runtime/src/string.rs`), tested to agree
with this code; the acceleration is a representation detail behind
the primitive operations chapter's justified bodies (D89).

Module scheme (design/base-library.md): the type's home carries its
instance family (the parked-instance deviation of D159 is over).

</details>

# Types and constructors

<a id="type-537472696e67"></a>

## `String`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">String</span></code></pre>

The representation constructor: module-private (the immutability
contract: no API hands out a string's backing). A `StrBuf` wraps a
buffer that is either fresh (`bytes-new` filled here) or a copy,
never one a caller retains.

[Source](/reference-sources/Data.String.cinch) · line 70

# Values

<a id="value-636f6d706172652d62797465732d66726f6d"></a>

## `compare-bytes-from`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">compare-bytes-from</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">I64</span>)))</code></pre>

Byte-wise comparison; UTF-8 preserves scalar value order under
byte-wise lexicographic order, so this is also code point order.

[Source](/reference-sources/Data.String.cinch) · line 341

<a id="value-636f70792d62797465732d696e746f"></a>

## `copy-bytes-into`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">copy-bytes-into</span> :: <span class="tok-keyword">using</span> ((<span class="tok-variable">arg2</span> <span class="tok-operator">+</span> <span class="tok-variable">arg4</span>) <span class="tok-operator">&lt;=</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-variable">bytes-len</span> <span class="tok-variable">arg0</span>), (<span class="tok-variable">arg3</span> <span class="tok-operator">+</span> <span class="tok-variable">arg4</span>) <span class="tok-operator">&lt;=</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-variable">bytes-len</span> <span class="tok-variable">arg1</span>)) =&gt; ((<span class="tok-variable">arg0</span> :: <span class="tok-constructor">Bytes</span>) -&gt; ((<span class="tok-variable">arg1</span> :: <span class="tok-constructor">Bytes</span>) -&gt; ((<span class="tok-variable">arg2</span> :: <span class="tok-constructor">Nat</span>) -&gt; ((<span class="tok-variable">arg3</span> :: <span class="tok-constructor">Nat</span>) -&gt; ((<span class="tok-variable">arg4</span> :: <span class="tok-constructor">Nat</span>) -&gt; <span class="tok-constructor">Unit</span>)))))</code></pre>

Concatenation (O(n + m) copies). The range copy is the runtime's
`memmove` (`bytes-copy-range`, with its two range obligations): a
loop here boxed a `Nat` for every index step.

[Source](/reference-sources/Data.String.cinch) · line 233

<a id="value-656d7074792d737472696e67"></a>

## `empty-string`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">empty-string</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span></code></pre>

[Source](/reference-sources/Data.String.cinch)

<a id="value-7061636b"></a>

## `pack`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pack</span> :: ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Char</span>) -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Haskell's `Data.String` pair: `pack` is the `List Char` view of a
string, `unpack` its inverse (new conversions: no `String`/`List
Char` pair existed before). `pack` constructs a fresh string (the
wrap/copy discipline: the buffer is fresh and never aliased).

[Source](/reference-sources/Data.String.cinch) · line 312

<a id="value-737472696e672d627974652d696e6465782d66726f6d"></a>

## `string-byte-index-from`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">string-byte-index-from</span> :: <span class="tok-keyword">using</span> (<span class="tok-variable">arg1</span> <span class="tok-operator">&lt;=</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-variable">bytes-len</span> <span class="tok-variable">arg0</span>)) =&gt; ((<span class="tok-variable">arg0</span> :: <span class="tok-constructor">Bytes</span>) -&gt; ((<span class="tok-variable">arg1</span> :: <span class="tok-constructor">Nat</span>) -&gt; ((<span class="tok-variable">arg2</span> :: <span class="tok-constructor">Nat</span>) -&gt; <span class="tok-variable">refined</span> ((<span class="tok-variable">arg1</span> <span class="tok-operator">&lt;=</span> <span class="tok-variable">r</span> <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">&lt;=</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-variable">bytes-len</span> <span class="tok-variable">arg0</span>))) <span class="tok-constructor">Nat</span>)))</code></pre>

Byte offset of code point `n` from offset `i`; the byte length when
`n` reaches past the end. The offset never moves backwards and never
passes the end, and the result says so. A sequence whose width runs
past the end is malformed, as `utf8-at` decides; the step restates
that end test, so it answers `i` as a malformed sequence does.

[Source](/reference-sources/Data.String.cinch) · line 286

<a id="value-737472696e672d627974652d6c656e677468"></a>

## `string-byte-length`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">string-byte-length</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

[Source](/reference-sources/Data.String.cinch)

<a id="value-737472696e672d636861722d6174"></a>

## `string-char-at`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">string-char-at</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Char</span>)))</code></pre>

[Source](/reference-sources/Data.String.cinch)

<a id="value-737472696e672d636861722d61742d66726f6d"></a>

## `string-char-at-from`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">string-char-at-from</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Char</span>))))</code></pre>

The scalar value at code point index `n`, by scan (O(n)).

[Source](/reference-sources/Data.String.cinch) · line 185

<a id="value-737472696e672d636f6d70617265"></a>

## `string-compare`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">string-compare</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">I64</span>))</code></pre>

Compares the two backings in place: `string-to-bytes` would copy
both operands first, and the lexer's keyword table compares every
identifier against dozens of literals.

[Source](/reference-sources/Data.String.cinch) · line 353

<a id="value-737472696e672d636f6e636174"></a>

## `string-concat`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">string-concat</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

[Source](/reference-sources/Data.String.cinch)

<a id="value-737472696e672d636f6e6361742d616c6c"></a>

## `string-concat-all`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">string-concat-all</span> :: ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>) -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Every string in the list, in order, in one pass: the buffer is made
once, at the end of the list, sized to the pieces' total, and each
piece is copied once on the way back. A right fold of
`string-concat` would re-copy the whole built suffix at every step,
quadratic in the result's length. The concatenation of valid UTF-8
is valid UTF-8, so the buffer wraps without a decode. This is the
`Monoid String` instance's `mconcat`.

[Source](/reference-sources/Data.String.cinch) · line 252

<a id="value-737472696e672d66726f6d2d6279746573"></a>

## `string-from-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">string-from-bytes</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

[Source](/reference-sources/Data.String.cinch)

<a id="value-737472696e672d6c656e677468"></a>

## `string-length`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">string-length</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

[Source](/reference-sources/Data.String.cinch)

<a id="value-737472696e672d6c656e6774682d66726f6d"></a>

## `string-length-from`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">string-length-from</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Nat</span>)))</code></pre>

Length in Unicode scalar values (O(n) scan).

[Source](/reference-sources/Data.String.cinch) · line 173

<a id="value-737472696e672d726570656174"></a>

## `string-repeat`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">string-repeat</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

`s` repeated `n` times (the run-building helper the padding and
numeric-format sites spelled out as fixed byte runs): one buffer of
`n` copies, made once and filled in place.

[Source](/reference-sources/Data.String.cinch) · line 269

<a id="value-737472696e672d736c696365"></a>

## `string-slice`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">string-slice</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)))</code></pre>

The code point range [start, end); indices past the end clamp and a
reversed range yields the empty string (the span's subtraction
takes its `start < end` evidence from the test).

[Source](/reference-sources/Data.String.cinch) · line 298

<a id="value-737472696e672d746f2d6279746573"></a>

## `string-to-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">string-to-bytes</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

A copy of the backing (the conversion out copies: the caller may
mutate its copy freely and the `String` never changes).

[Source](/reference-sources/Data.String.cinch) · line 89

<a id="value-756e7061636b"></a>

## `unpack`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unpack</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Char</span>))</code></pre>

[Source](/reference-sources/Data.String.cinch)

<a id="value-757466382d6174"></a>

## `utf8-at`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">utf8-at</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Char</span>, <span class="tok-constructor">Nat</span>))))</code></pre>

[Source](/reference-sources/Data.String.cinch)

<a id="value-757466382d63686172"></a>

## `utf8-char`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">utf8-char</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Char</span>, <span class="tok-constructor">Nat</span>))))</code></pre>

[Source](/reference-sources/Data.String.cinch)

<a id="value-757466382d636f6e74"></a>

## `utf8-cont`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">utf8-cont</span> :: <span class="tok-keyword">using</span> ((<span class="tok-variable">arg1</span> <span class="tok-operator">+</span> <span class="tok-variable">arg2</span>) <span class="tok-operator">&lt;=</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-variable">bytes-len</span> <span class="tok-variable">arg0</span>)) =&gt; ((<span class="tok-variable">arg0</span> :: <span class="tok-constructor">Bytes</span>) -&gt; ((<span class="tok-variable">arg1</span> :: <span class="tok-constructor">Nat</span>) -&gt; ((<span class="tok-variable">arg2</span> :: <span class="tok-constructor">Nat</span>) -&gt; ((<span class="tok-variable">arg3</span> :: <span class="tok-constructor">Nat</span>) -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">Nat</span>))))</code></pre>

The `k` continuation bytes from `i`: the caller has checked that
they lie inside the buffer (`i + k <= bytes-len b`), so each read
is in bounds and the obligation steps with the index.

[Source](/reference-sources/Data.String.cinch) · line 114

<a id="value-757466382d656e636f64652d696e746f"></a>

## `utf8-encode-into`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">utf8-encode-into</span> :: <span class="tok-keyword">using</span> ((<span class="tok-variable">arg1</span> <span class="tok-operator">+</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-variable">utf8-encode-width</span> <span class="tok-variable">arg2</span>)) <span class="tok-operator">&lt;=</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-variable">bytes-len</span> <span class="tok-variable">arg0</span>)) =&gt; ((<span class="tok-variable">arg0</span> :: <span class="tok-constructor">Bytes</span>) -&gt; ((<span class="tok-variable">arg1</span> :: <span class="tok-constructor">Nat</span>) -&gt; ((<span class="tok-variable">arg2</span> :: <span class="tok-constructor">Nat</span>) -&gt; <span class="tok-constructor">Nat</span>)))</code></pre>

Encode one scalar value at `pos` (bytes are raw values; division is
the `nat-div`/`nat-rem` function family). The caller proves the
encoding fits (`pos + utf8-encode-width cp <= bytes-len b`, a
buffer sized with `utf8-encode-width`), and the branches test that
same width, so each one knows how many bytes it may write. Every
byte is taken modulo its field width, which is the same number for
every value its width admits (`cp < 128` in the first branch, `cp /
64 < 32` in the second, `cp / 4096 < 16` in the third, and `cp <
2^22` in the last, past which the unmasked lead byte did not fit a
byte at all).

[Source](/reference-sources/Data.String.cinch) · line 213

<a id="value-757466382d656e636f64652d7769647468"></a>

## `utf8-encode-width`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">utf8-encode-width</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">1</span> <span class="tok-number">4</span> <span class="tok-constructor">Nat</span>))</code></pre>

[Source](/reference-sources/Data.String.cinch)

<a id="value-757466382d6c6561642d7061796c6f6164"></a>

## `utf8-lead-payload`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">utf8-lead-payload</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Nat</span>))</code></pre>

The lead byte's payload bits: `x - 192`, `x - 224`, `x - 240` for
the lead byte of a two-, three- and four-byte sequence, written as
the low bits, which is the same number for every lead byte
`utf8-width` admits and needs no `x >= 192` evidence.

[Source](/reference-sources/Data.String.cinch) · line 127

<a id="value-757466382d6d696e"></a>

## `utf8-min`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">utf8-min</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

[Source](/reference-sources/Data.String.cinch)

<a id="value-757466382d76616c69642d66726f6d"></a>

## `utf8-valid-from`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">utf8-valid-from</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.String.cinch)

<a id="value-757466382d7769647468"></a>

## `utf8-width`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">utf8-width</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

UTF-8 decoding rules: a lead byte gives the width; continuation
bytes are 10xxxxxx; overlong encodings, surrogates, and values above
0x10FFFF are malformed.

[Source](/reference-sources/Data.String.cinch) · line 95

# Instances

These are checked exported instance entries. Law/sample-check status is separate from the existence of an instance; see [laws and optimization](../reference/12-rewrites-and-optimization.md).

| Instance | Target | Resolution |
| --- | --- | --- |
| `Data.String.Eq_String` | `Relation.Eq.Eq Data.String.String` | Implicit |
| `Data.String.Hash_String` | `Data.Hash.Hash Data.String.String` | Implicit |
| `Data.String.Monoid_String` | `Algebra.Monoid.Monoid Data.String.String` | Implicit |
| `Data.String.Ord_String` | `Relation.Ord.Ord Data.String.String` | Implicit |
| `Data.String.Semigroup_String` | `Algebra.Semigroup.Semigroup Data.String.String` | Implicit |

**`Data.String.Hash_String`**: The digest of the UTF-8 backing through the `Bytes` instance
(Data.Bytes owns the byte fold, so the two instances agree by
construction instead of keeping two copies of one algorithm).
`string-bytes-raw` reads the backing in place: hashing copies
nothing.
**`Data.String.Monoid_String`**: `empty-string` is the identity (moved from Algebra.Monoid); `mconcat`
overrides the class's balanced fold with the one-pass concatenation.
**`Data.String.Semigroup_String`**: Strings under concatenation over the UTF-8 Bytes backing (moved
from Algebra.Semigroup at the prelude split; the instance lives
with its type).

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Data/String.cinch`. Source SHA-256: `27a381efd32575a2b0ca0c4a0d9f3a9c982b64b731d672caad03ba04a997ae79`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
