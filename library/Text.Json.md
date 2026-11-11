# Text.Json

JSON parsing and rendering over Data.Json, with explicit errors and builder-based output.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Text.Json`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.Json.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Char](Data.Char.md), [Data.Int](Data.Int.md), [Data.Json](Data.Json.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Ordering](Data.Ordering.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Text.Builder](Text.Builder.md), [Text.Codec](Text.Codec.md), [Text.Lex](Text.Lex.md), [Text.Parse](Text.Parse.md), [Text.Show](Text.Show.md).

Non-embedded source closure: [Text.Builder](Text.Builder.md), [Text.Json](Text.Json.md), [Text.Lex](Text.Lex.md), [Text.Parse](Text.Parse.md), [Text.Scan](Text.Scan.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Text.Json: JSON text to `Json` values and back, canonically.

Module scheme (design/base-library.md): `Text.*` is conversion to and
from String/Bytes. This module is the P31 JSON family over the P30
parsing toolkit's error vocabulary and the `Json` value family
(decision D148): a real JSON parser (RFC 8259 section 2's grammar)
with megaparsec-style error bundles, a total printer, the canonical
form (sorted object keys, fixed number formatting, no insignificant
whitespace) for manifests and `.cinchif` round trips, a
SHA-256-strength deterministic content hash, and `JsonCodec`
instances in `Text.Codec`'s law shape over the A6 schema vocabulary.

The parser is built ON the P30 toolkit: the combinator vocabulary
(`parse-pure`, `parse-bind`, `attempt-p`, `or-else`, `label-p`,
`many-p`, `sep-by-p`, `choice-p`, `expect-eof`, ...) is the same
megaparsec shape as `Text.Parse`, carrying the toolkit's own error
types (`ParseError`, `ErrorItem`, `ParseErrorBundle`) and span model,
so `render-bundle` renders JSON errors exactly as it renders grammar
errors. Where the two modules export the same name they are distinct
types (`Text.Parse.Parser` over tokens, `Text.Json.JParser` over the
character stream); import renames or qualification (`import Text.Json
as Json`, `Json.or-else`) disambiguate at use (P19/D108).

WHY THE CHARACTER STREAM AND NOT `Text.Parse`'S TOKEN STREAM
(reported): JSON text cannot be lexed by `Text.Lex` first. The two
escape vocabularies are mutually exclusive in both directions (JSON
`\u0041` is not a Cinch escape, Cinch `\u{41}` is not a JSON
escape; JSON `"\/"` and `"\b"`/`"\f"` are unknown Cinch escapes),
and `Text.Lex` folds numeric spellings (hex folded, underscores
dropped, leading zeros stripped), so `007` and `0x1f` could not be
refused as the JSON grammar demands. A JSON tokenizer feeding
`Text.Parse`'s literal `Parser` would also need `Text.Lex` to export
its `LTok` constructor (module-private after D308's ctor rule); the
combinator shape is carried here over the raw bytes instead, which
keeps every grammar rule honest. When `LTok` is exported this
module's scanners can move onto `List LToken` without API change.

THE CANONICAL FORM (reported): object keys sorted by `string-compare`
(byte order), duplicate keys refused at parse (RFC 8785 requires
unique names; canonical identity needs it), no insignificant
whitespace, integers as canonical decimal (`int-to-string`), floats
as the shortest round-trip scientific text (`textF64ToDecimal`,
`-d.dddde[-]d+`, so `2.5` renders `2.5e0` and `0.5` renders `5e-1`),
strings with exactly the `\"`, `\\`, `\b`, `\f`, `\n`, `\r`, `\t` and
`\u00xx` escapes (other scalars raw UTF-8). Two prints of one value
are byte identical, and the canonical form is a fixed point of
parse-then-print. A non-finite `JFloat` renders `null` (aeson's
choice; JSON numbers are finite), the one rendering that is not a
round trip: the law covers the JSON-representable values.

Total printing (reported): the `Json` family documents the object
and array spines (`JObj`/`JArr` ended by `JObjEnd`/`JArrEnd`) but
cannot enforce them, so a spine tail outside its own family (for
example `JArr x JNull`) renders as one trailing item (arrays) or one
trailing entry under the empty key (objects). Parsing only produces
well-formed spines.

Named refusals (unproductions this module deliberately does not
implement; each is a named boundary, never a silent gap):
  parse recovery and soft notes (`parse-note`, `with-recovery-p`):
    JSON documents report their first error as the bundle; the
    toolkit's recovery vocabulary composes on top when a caller
    wants it;
  comments, `NaN`/`Infinity`, trailing commas and unquoted keys
    (JSON5/JSONC extensions): not RFC 8259;
  lone surrogate escapes: refused with a named error (RFC 8259
    leaves them unpredictable; UTF-8 output cannot carry them);
  streaming/incremental parsing: the document is parsed whole
    (the P30 runner's shape).

</details>

# Types and constructors

<a id="type-4a506172736572"></a>

## `JParser`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">JParser</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>)</code></pre>

A parser: a function from cursor to reply (the P30 `Parser`
shape). Recursive parsers wrap in `MkJParser` so their references
sit under a lambda (the D302 top-level value discipline).

[Source](/reference-sources/Text.Json.cinch) · line 194

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MkJParser</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">JState</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">JReply</span> <span class="tok-variable">a</span>) -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">JParser</span> <span class="tok-variable">a</span>)</code></pre>

<a id="type-4a5265706c79"></a>

## `JReply`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">JReply</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>)</code></pre>

A reply: the consumed/empty success and failure distinction of
`Text.Parse`'s `Reply` (the expected-set hint merges exactly as
there).

[Source](/reference-sources/Text.Json.cinch) · line 187

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">JOk</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . (<span class="tok-constructor">Bool</span> -&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">JState</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">ErrorItem</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">JReply</span> <span class="tok-variable">a</span>))))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">JErr</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . (<span class="tok-constructor">Bool</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">ParseError</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">JReply</span> <span class="tok-variable">a</span>))</code></pre>

<a id="type-4a5374617465"></a>

## `JState`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">JState</span></code></pre>

The cursor into the source: the buffer, the byte offset, and the
position as line and character column (a UTF-8 continuation byte
does not advance the column, so columns count scalars like the
compiler's spans).

[Source](/reference-sources/Text.Json.cinch) · line 165

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">JState</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">JState</span>))))</code></pre>

# Values

<a id="value-617474656d70742d70"></a>

## `attempt-p`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">attempt-p</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">JParser</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">JParser</span> <span class="tok-variable">a</span>))</code></pre>

[Source](/reference-sources/Text.Json.cinch)

<a id="value-6265747765656e2d70"></a>

## `between-p`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">between-p</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">open</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">close</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">JParser</span> <span class="tok-variable">open</span>) -&gt; ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">JParser</span> <span class="tok-variable">close</span>) -&gt; ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">JParser</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">JParser</span> <span class="tok-variable">a</span>))))</code></pre>

[Source](/reference-sources/Text.Json.cinch)

<a id="value-63616e6f6e6963616c2d6279746573"></a>

## `canonical-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">canonical-bytes</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Json</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

[Source](/reference-sources/Text.Json.cinch)

<a id="value-63616e6f6e6963616c2d68617368"></a>

## `canonical-hash`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">canonical-hash</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Json</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

[Source](/reference-sources/Text.Json.cinch)

<a id="value-63616e6f6e6963616c2d6a736f6e"></a>

## `canonical-json`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">canonical-json</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Json</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

[Source](/reference-sources/Text.Json.cinch)

<a id="value-63686f6963652d70"></a>

## `choice-p`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">choice-p</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">JParser</span> <span class="tok-variable">a</span>)) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">JParser</span> <span class="tok-variable">a</span>))</code></pre>

[Source](/reference-sources/Text.Json.cinch)

<a id="value-6469676573742d686578"></a>

## `digest-hex`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">digest-hex</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

[Source](/reference-sources/Text.Json.cinch)

<a id="value-6578706563742d656f66"></a>

## `expect-eof`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">expect-eof</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">JParser</span> <span class="tok-constructor">Unit</span>)</code></pre>

[Source](/reference-sources/Text.Json.cinch)

<a id="value-6661696c2d70"></a>

## `fail-p`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">fail-p</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">JParser</span> <span class="tok-variable">a</span>))</code></pre>

[Source](/reference-sources/Text.Json.cinch)

<a id="value-6a736f6e2d6571"></a>

## `json-eq`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-eq</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Json</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Json</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Text.Json.cinch)

<a id="value-6a736f6e2d76616c7565"></a>

## `json-value`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-value</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">JParser</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Json</span>)</code></pre>

[Source](/reference-sources/Text.Json.cinch)

<a id="value-6c6162656c2d70"></a>

## `label-p`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">label-p</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">JParser</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">JParser</span> <span class="tok-variable">a</span>)))</code></pre>

[Source](/reference-sources/Text.Json.cinch)

<a id="value-6c6f6f6b2d61686561642d70"></a>

## `look-ahead-p`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">look-ahead-p</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">JParser</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">JParser</span> <span class="tok-variable">a</span>))</code></pre>

[Source](/reference-sources/Text.Json.cinch)

<a id="value-6d616e792d70"></a>

## `many-p`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">many-p</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">JParser</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">JParser</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>)))</code></pre>

[Source](/reference-sources/Text.Json.cinch)

<a id="value-6f7074696f6e616c2d70"></a>

## `optional-p`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">optional-p</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">JParser</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">JParser</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">a</span>)))</code></pre>

[Source](/reference-sources/Text.Json.cinch)

<a id="value-6f722d656c7365"></a>

## `or-else`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">or-else</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">JParser</span> <span class="tok-variable">a</span>) -&gt; ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">JParser</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">JParser</span> <span class="tok-variable">a</span>)))</code></pre>

[Source](/reference-sources/Text.Json.cinch)

<a id="value-70617273652d62696e64"></a>

## `parse-bind`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">parse-bind</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">b</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">JParser</span> <span class="tok-variable">a</span>) -&gt; ((<span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">JParser</span> <span class="tok-variable">b</span>)) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">JParser</span> <span class="tok-variable">b</span>)))</code></pre>

[Source](/reference-sources/Text.Json.cinch)

<a id="value-70617273652d666d6170"></a>

## `parse-fmap`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">parse-fmap</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">b</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-variable">a</span> -&gt; <span class="tok-variable">b</span>) -&gt; ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">JParser</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">JParser</span> <span class="tok-variable">b</span>)))</code></pre>

[Source](/reference-sources/Text.Json.cinch)

<a id="value-70617273652d6a736f6e"></a>

## `parse-json`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">parse-json</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">ParseOutcome</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Json</span>)))</code></pre>

[Source](/reference-sources/Text.Json.cinch)

<a id="value-70617273652d70757265"></a>

## `parse-pure`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">parse-pure</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). (<span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">JParser</span> <span class="tok-variable">a</span>))</code></pre>

[Source](/reference-sources/Text.Json.cinch)

<a id="value-7072696e742d6a736f6e"></a>

## `print-json`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">print-json</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Json</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

[Source](/reference-sources/Text.Json.cinch)

<a id="value-72756e2d6a7061727365722d666e"></a>

## `run-jparser-fn`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">run-jparser-fn</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">JParser</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">JState</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">JReply</span> <span class="tok-variable">a</span>)))</code></pre>

[Source](/reference-sources/Text.Json.cinch)

<a id="value-736174697366792d62797465"></a>

## `satisfy-byte`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">satisfy-byte</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">a</span>)) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">ErrorItem</span>) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">JParser</span> <span class="tok-variable">a</span>)))</code></pre>

[Source](/reference-sources/Text.Json.cinch)

<a id="value-7365702d62792d70"></a>

## `sep-by-p`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sep-by-p</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">s</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">JParser</span> <span class="tok-variable">a</span>) -&gt; ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">JParser</span> <span class="tok-variable">s</span>) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">JParser</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>))))</code></pre>

[Source](/reference-sources/Text.Json.cinch)

<a id="value-736f6d652d70"></a>

## `some-p`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">some-p</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">JParser</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">JParser</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>)))</code></pre>

[Source](/reference-sources/Text.Json.cinch)

# Instances

These are checked exported instance entries. Law/sample-check status is separate from the existence of an instance; see [laws and optimization](../reference/12-rewrites-and-optimization.md).

| Instance | Target | Resolution |
| --- | --- | --- |
| `Text.Json.JsonCodec_Json` | `Text.Codec.JsonCodec Data.Json.Json` | Implicit |

**`Text.Json.JsonCodec_Json`**: JsonCodec's identity column (the class and its decode-error
vocabulary live in `Text.Codec`; the `I64`/`Bool` instances live
with their types in `Data.Machine`/`Data.Bool`). A `Json` round
trips as itself.

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Text/Json.cinch`. Source SHA-256: `461861b69d963202733c7016f571b393cfc048e45c04eaf92cef036fcfaeb47b`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
