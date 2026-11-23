# Text.TabWriter

Elastic tabstops: align tab-separated cells into padded columns, with escapes, HTML-aware widths, right alignment and incremental flushing.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Text.TabWriter`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.TabWriter.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bytes](Data.Bytes.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md).

Non-embedded source closure: [Data.Bytes.Build](Data.Bytes.Build.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Text.TabWriter](Text.TabWriter.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Elastic tabstops: align tab-separated cells into columns, the way terminal tables and aligned listings need.
Input is a sequence of cells, each terminated by a horizontal tab ('\t') or
a vertical tab ('\v'); a newline ends a line and a form feed ('\f') ends a
line and forces output. Consecutive lines whose cells reach the same column
form a column block; every cell in a block is padded to the block's widest
cell plus the configured padding. The last cell of each line is never
aligned. Text between two 0xff escape bytes is copied literally (its tabs and
newlines do not split cells), and with tab-filter-html, HTML tags have zero
width and entities a width of one. Cell width counts Unicode scalars, using
SWAR to skip eight bytes at a time; in malformed UTF-8 every byte that is not
a continuation byte counts as one column.

The writer is an immutable value: tab-write returns the output that became
final (blocks are released at a form feed or after a line with a single
cell) together with the advanced writer, and tab-flush releases everything
buffered. Retaining a writer branches the formatting.

</details>

# Types and constructors

<a id="type-546162436f6e666967"></a>

## `TabConfig`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">TabConfig</span></code></pre>

Formatting parameters: minimum cell width, tab width (for tab padding), padding added to the
widest cell, the padding byte, and the bitwise union of the flag constants.

[Source](/reference-sources/Text.TabWriter.cinch) · line 59

<a id="type-546162577269746572"></a>

## `TabWriter`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">TabWriter</span></code></pre>

Buffered lines (most recent first, each line's cells in order), the current line's finished
cells (most recent first), the current cell's chunks (most recent first) and width, and the lexer.

[Source](/reference-sources/Text.TabWriter.cinch) · line 94

# Values

<a id="value-7461622d616c69676e2d7269676874"></a>

## `tab-align-right`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tab-align-right</span> :: <span class="tok-constructor">Nat</span></code></pre>

Pad cells on the left, aligning their text to the right edge of the column.

[Source](/reference-sources/Text.TabWriter.cinch) · line 38

<a id="value-7461622d636f6e666967"></a>

## `tab-config`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tab-config</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; ((<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">0</span> <span class="tok-number">255</span> <span class="tok-constructor">Nat</span>) -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">TabWriter</span>.<span class="tok-constructor">TabConfig</span>)))))</code></pre>

Build a configuration. A padding byte of 9 (tab) pads with tabs of the given tab width, rounding
column widths up to multiples of it; any other byte pads with repetitions of that byte.
Tab padding implies left alignment, so tab-align-right is cleared when the padding byte is a tab.

[Source](/reference-sources/Text.TabWriter.cinch) · line 66

<a id="value-7461622d6465627567"></a>

## `tab-debug`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tab-debug</span> :: <span class="tok-constructor">Nat</span></code></pre>

Print a vertical bar between columns and a dashed line after each form feed.

[Source](/reference-sources/Text.TabWriter.cinch) · line 50

<a id="value-7461622d646973636172642d656d7074792d636f6c756d6e73"></a>

## `tab-discard-empty-columns`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tab-discard-empty-columns</span> :: <span class="tok-constructor">Nat</span></code></pre>

Give columns whose cells are all empty and soft-terminated a width of zero.

[Source](/reference-sources/Text.TabWriter.cinch) · line 42

<a id="value-7461622d657363617065"></a>

## `tab-escape`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tab-escape</span> :: <span class="tok-constructor">Nat</span></code></pre>

The escape byte, 0xff, that brackets literal text.

[Source](/reference-sources/Text.TabWriter.cinch) · line 54

<a id="value-7461622d66696c7465722d68746d6c"></a>

## `tab-filter-html`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tab-filter-html</span> :: <span class="tok-constructor">Nat</span></code></pre>

Treat HTML tags as zero width and entities as width one; both are copied unchanged.

[Source](/reference-sources/Text.TabWriter.cinch) · line 30

<a id="value-7461622d666c757368"></a>

## `tab-flush`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tab-flush</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">TabWriter</span>.<span class="tok-constructor">TabWriter</span> -&gt; (<span class="tok-constructor">Bytes</span>, <span class="tok-constructor">Text</span>.<span class="tok-constructor">TabWriter</span>.<span class="tok-constructor">TabWriter</span>))</code></pre>

Release every buffered line and the pending cell, ending any open escape, as final output.
The final line receives no newline of its own; the writer is empty afterwards.

[Source](/reference-sources/Text.TabWriter.cinch) · line 114

<a id="value-7461622d666f726d6174"></a>

## `tab-format`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tab-format</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">TabWriter</span>.<span class="tok-constructor">TabConfig</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Bytes</span>))</code></pre>

Format an entire input as one write followed by a flush.

[Source](/reference-sources/Text.TabWriter.cinch) · line 103

<a id="value-7461622d696e64656e74"></a>

## `tab-indent`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tab-indent</span> :: <span class="tok-constructor">Nat</span></code></pre>

Pad leading empty cells with tabs, whatever the padding byte, so indentation stays tab-based.

[Source](/reference-sources/Text.TabWriter.cinch) · line 46

<a id="value-7461622d73747269702d657363617065"></a>

## `tab-strip-escape`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tab-strip-escape</span> :: <span class="tok-constructor">Nat</span></code></pre>

Remove the 0xff escape bytes that bracket literal text instead of copying them.

[Source](/reference-sources/Text.TabWriter.cinch) · line 34

<a id="value-7461622d7772697465"></a>

## `tab-write`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tab-write</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">TabWriter</span>.<span class="tok-constructor">TabWriter</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span>, <span class="tok-constructor">Text</span>.<span class="tok-constructor">TabWriter</span>.<span class="tok-constructor">TabWriter</span>)))</code></pre>

Consume input and return the output that became final, with the advanced writer.

[Source](/reference-sources/Text.TabWriter.cinch) · line 108

<a id="value-7461622d777269746572"></a>

## `tab-writer`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tab-writer</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">TabWriter</span>.<span class="tok-constructor">TabConfig</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">TabWriter</span>.<span class="tok-constructor">TabWriter</span>)</code></pre>

A writer with nothing buffered.

[Source](/reference-sources/Text.TabWriter.cinch) · line 99

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Text/TabWriter.cinch`. Source SHA-256: `e13da1b41483f29493fe929ca885b7a91a5483f3a3b11ff56482fc034161acc5`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
