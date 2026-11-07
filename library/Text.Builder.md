# Text.Builder

An efficient text-construction vocabulary for assembling many fragments before rendering.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Text.Builder`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.Builder.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bytes](Data.Bytes.md), [Data.Nat](Data.Nat.md), [Data.String](Data.String.md), [Text.Show](Text.Show.md).

Non-embedded source closure: [Text.Builder](Text.Builder.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Text.Builder: an append-cheap text and byte builder (the codegen rung's
accumulator).

The ladder's IR rung renders LLVM IR as TEXT with `string-concat`, which
copies its LEFT operand, so emitting a module piece by piece is quadratic in
the number of pieces: survivable for a toy function, hostile for the whole
prelude's 55 modules, and the same shape would bite the manifest writer and
every diagnostic renderer. This module is the efficient accumulator the text
homes did not have.

The representation is a SPINE of pending pieces (the `Bootstrap.Ast`
`*-spine` convention, D387's answer to a data type that cannot nest a `List`
of itself): each append allocates one cell and RETAINS its argument, so an
append of a string or a byte slice copies NOTHING and runs in constant time,
independent of the piece's length. The spine holds the pieces most-recent
first; the flatten lays them out in APPEND order by filling the output
backward from its end. `builder-run`/`builder-bytes` measure the whole spine
once (`builder-size`), allocate ONE output buffer, and copy each piece
exactly once: O(total bytes) time and one allocation, never quadratic.

`builder-size` is EXACT (the sum of each piece's byte length, computed
without flattening) and O(number of pieces). `builder-chunks` counts the
pending pieces, one per append (an append of a line is still one piece), so
a caller can assert that appends do not re-copy.

`builder-add-nat`/`builder-add-int` render through `Text.Show`'s
`nat-to-string`/`int-to-string` and append the resulting `String` as one
piece. `builder-add-line` appends the text and a single LF as one piece, so
the line semantics are the builder's, not a caller's.

`builder-run` decodes the flattened UTF-8. A builder built only through the
string, line, nat and int surface is always valid UTF-8, so the decode never
fails there; a raw byte chunk that is not valid UTF-8 (only
`builder-add-bytes` can insert one) still flattens exactly through
`builder-bytes`, and `builder-run` then yields the empty string. That is the
defined total fallback, the same shape `Text.Show`'s `wrap-bytes` uses.

Module scheme (design/base-library.md): `Text.*` is conversion to and from
String/Bytes, so the builder lives here.

</details>

# Types and constructors

<a id="type-4275696c646572"></a>

## `Builder`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Builder</span></code></pre>

A pending piece, each a retained argument with no copy made at append
time. `BEnd` is the spine's end marker; `BLine` carries the text before
its newline; `BBytes` carries a raw byte slice (which need not be UTF-8).

[Source](/reference-sources/Text.Builder.cinch) · line 53

# Values

<a id="value-6275696c6465722d6164642d6279746573"></a>

## `builder-add-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">builder-add-bytes</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Builder</span> -&gt; <span class="tok-constructor">Builder</span>))</code></pre>

Append a raw byte slice as one piece: O(1), no copy of the slice.

[Source](/reference-sources/Text.Builder.cinch) · line 78

<a id="value-6275696c6465722d6164642d696e74"></a>

## `builder-add-int`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">builder-add-int</span> :: (<span class="tok-constructor">Int</span> -&gt; (<span class="tok-constructor">Builder</span> -&gt; <span class="tok-constructor">Builder</span>))</code></pre>

Append a decimal `Int` (rendered by `Text.Show`).

[Source](/reference-sources/Text.Builder.cinch) · line 90

<a id="value-6275696c6465722d6164642d6c696e65"></a>

## `builder-add-line`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">builder-add-line</span> :: (<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Builder</span> -&gt; <span class="tok-constructor">Builder</span>))</code></pre>

Append the text and a single LF as one piece.

[Source](/reference-sources/Text.Builder.cinch) · line 82

<a id="value-6275696c6465722d6164642d6e6174"></a>

## `builder-add-nat`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">builder-add-nat</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Builder</span> -&gt; <span class="tok-constructor">Builder</span>))</code></pre>

Append a decimal `Nat` (rendered by `Text.Show`).

[Source](/reference-sources/Text.Builder.cinch) · line 86

<a id="value-6275696c6465722d6164642d737472696e67"></a>

## `builder-add-string`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">builder-add-string</span> :: (<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Builder</span> -&gt; <span class="tok-constructor">Builder</span>))</code></pre>

Append a string as one piece: O(1), no copy of the string's bytes.

[Source](/reference-sources/Text.Builder.cinch) · line 74

<a id="value-6275696c6465722d6279746573"></a>

## `builder-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">builder-bytes</span> :: (<span class="tok-constructor">Builder</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Exact size and fixed-length pieces make the checked copy pass succeed.
Preserve the original semantics if those private invariants ever disagree:
recompute with the range-checked fill, never return truncated text.

[Source](/reference-sources/Text.Builder.cinch) · line 184

<a id="value-6275696c6465722d6368756e6b73"></a>

## `builder-chunks`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">builder-chunks</span> :: (<span class="tok-constructor">Builder</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

How many pieces are pending: one per append, so a test can assert that
appends do not re-copy. O(number of pieces).

[Source](/reference-sources/Text.Builder.cinch) · line 95

<a id="value-6275696c6465722d69732d656d707479"></a>

## `builder-is-empty`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">builder-is-empty</span> :: (<span class="tok-constructor">Builder</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Whether a builder has no pending pieces. Structural: a builder is empty
exactly when it was never appended to, so an appended empty string counts
as a piece.

[Source](/reference-sources/Text.Builder.cinch) · line 66

<a id="value-6275696c6465722d6e6577"></a>

## `builder-new`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">builder-new</span> :: <span class="tok-constructor">Builder</span></code></pre>

The empty builder (no pending pieces).

[Source](/reference-sources/Text.Builder.cinch) · line 60

<a id="value-6275696c6465722d72756e"></a>

## `builder-run`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">builder-run</span> :: (<span class="tok-constructor">Builder</span> -&gt; <span class="tok-constructor">String</span>)</code></pre>

The flattened string: flatten once, then decode the UTF-8. A builder
built through the string, line, nat and int surface is always valid UTF-8;
a raw byte chunk that is not yields the empty string (see the header).

[Source](/reference-sources/Text.Builder.cinch) · line 193

<a id="value-6275696c6465722d73697a65"></a>

## `builder-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">builder-size</span> :: (<span class="tok-constructor">Builder</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

The exact total byte length of the flattened result, without flattening
(a line piece counts its LF). O(number of pieces).

[Source](/reference-sources/Text.Builder.cinch) · line 106

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Text/Builder.cinch`. Source SHA-256: `c089e433293b637862387c71967e800c5cb57f4f7011724ee4d21b18c5ece24b`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
