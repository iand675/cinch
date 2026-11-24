# Text.TextProto

Text-protocol framing: lines, canonical MIME headers, status responses and dot encoding for HTTP, SMTP and mail.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Text.TextProto`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.TextProto.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bytes](Data.Bytes.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Data.Wide](Data.Wide.md).

Non-embedded source closure: [Data.Bytes.Build](Data.Bytes.Build.md), [Text.TextProto](Text.TextProto.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Text protocol framing shared by HTTP, SMTP, NNTP and mail: lines, MIME headers, status responses and dot encoding.
Readers take a buffer and an offset and return the value and the offset
after it, so a caller can resume where a reader stopped. Lines end at LF
with an optional preceding CR. MIME headers are an ordered multimap keyed
by canonical header names: canonicalization lowercases eight bytes at a
time with SWAR, validates them as header-name bytes, and then uppercases the
first letter of every hyphen-separated word; keys with invalid bytes are
kept as given. Dot encoding stuffs leading dots and normalizes line endings
to CRLF, and dot decoding reverses it up to the terminating dot line.

</details>

# Types and constructors

<a id="type-4d696d65486561646572"></a>

## `MimeHeader`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">MimeHeader</span></code></pre>

Header fields in first-insertion order of their canonical keys, each with its values in order.

[Source](/reference-sources/Text.TextProto.cinch) · line 46

<a id="type-5465787450726f746f4572726f72"></a>

## `TextProtoError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">TextProtoError</span></code></pre>

A malformed or truncated protocol element; offsets locate the offending line.

[Source](/reference-sources/Text.TextProto.cinch) · line 27

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">UnexpectedEnd</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">TextProto</span>.<span class="tok-constructor">TextProtoError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MalformedHeader</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">TextProto</span>.<span class="tok-constructor">TextProtoError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MalformedCode</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">TextProto</span>.<span class="tok-constructor">TextProtoError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">UnexpectedCode</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">TextProto</span>.<span class="tok-constructor">TextProtoError</span>))</code></pre>

# Values

<a id="value-63616e6f6e6963616c2d6865616465722d6b6579"></a>

## `canonical-header-key`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">canonical-header-key</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Canonical form of a header name: "content-TYPE" becomes "Content-Type". A name containing a byte
that is not a header token, or a space, is returned unchanged.

[Source](/reference-sources/Text.TextProto.cinch) · line 173

<a id="value-646f742d656e636f6465"></a>

## `dot-encode`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">dot-encode</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Dot-encode a body: a dot doubled at the start of each line, a lone LF expanded to CRLF, existing
CRLF kept, and the body closed with CRLF (unless it already ended a line) and ".\r\n".

[Source](/reference-sources/Text.TextProto.cinch) · line 472

<a id="value-6865616465722d616464"></a>

## `header-add`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">header-add</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">TextProto</span>.<span class="tok-constructor">MimeHeader</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">TextProto</span>.<span class="tok-constructor">MimeHeader</span>)))</code></pre>

Append a value to a key's values; the key is canonicalized.

[Source](/reference-sources/Text.TextProto.cinch) · line 55

<a id="value-6865616465722d64656c657465"></a>

## `header-delete`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">header-delete</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">TextProto</span>.<span class="tok-constructor">MimeHeader</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">TextProto</span>.<span class="tok-constructor">MimeHeader</span>))</code></pre>

Remove a key and all its values.

[Source](/reference-sources/Text.TextProto.cinch) · line 88

<a id="value-6865616465722d656e7472696573"></a>

## `header-entries`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">header-entries</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">TextProto</span>.<span class="tok-constructor">MimeHeader</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-constructor">Bytes</span>, (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Bytes</span>))))</code></pre>

Every field with its values.

[Source](/reference-sources/Text.TextProto.cinch) · line 106

<a id="value-6865616465722d676574"></a>

## `header-get`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">header-get</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">TextProto</span>.<span class="tok-constructor">MimeHeader</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Bytes</span>)))</code></pre>

The first value for a key, if any.

[Source](/reference-sources/Text.TextProto.cinch) · line 74

<a id="value-6865616465722d6b657973"></a>

## `header-keys`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">header-keys</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">TextProto</span>.<span class="tok-constructor">MimeHeader</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Bytes</span>))</code></pre>

Canonical keys in order.

[Source](/reference-sources/Text.TextProto.cinch) · line 98

<a id="value-6865616465722d736574"></a>

## `header-set`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">header-set</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">TextProto</span>.<span class="tok-constructor">MimeHeader</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">TextProto</span>.<span class="tok-constructor">MimeHeader</span>)))</code></pre>

Replace a key's values with a single value.

[Source](/reference-sources/Text.TextProto.cinch) · line 70

<a id="value-6865616465722d76616c756573"></a>

## `header-values`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">header-values</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">TextProto</span>.<span class="tok-constructor">MimeHeader</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Bytes</span>)))</code></pre>

Every value for a key, in order.

[Source](/reference-sources/Text.TextProto.cinch) · line 78

<a id="value-6d696d652d686561646572"></a>

## `mime-header`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mime-header</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">TextProto</span>.<span class="tok-constructor">MimeHeader</span></code></pre>

An empty header.

[Source](/reference-sources/Text.TextProto.cinch) · line 51

<a id="value-726561642d636f64652d6c696e65"></a>

## `read-code-line`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">read-code-line</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">TextProto</span>.<span class="tok-constructor">TextProtoError</span> (<span class="tok-constructor">Nat</span>, <span class="tok-constructor">Bytes</span>, <span class="tok-constructor">Nat</span>)))))</code></pre>

Read a single status line "ddd message" and check its code against an expected prefix (0 accepts any);
a continuation line "ddd-message" is malformed here. Returns the code, message and next offset.

[Source](/reference-sources/Text.TextProto.cinch) · line 374

<a id="value-726561642d636f6e74696e7565642d6c696e65"></a>

## `read-continued-line`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">read-continued-line</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">TextProto</span>.<span class="tok-constructor">TextProtoError</span> (<span class="tok-constructor">Bytes</span>, <span class="tok-constructor">Nat</span>))))</code></pre>

Read a line and any continuation lines (beginning with space or tab), joined by single spaces,
with surrounding whitespace removed.

[Source](/reference-sources/Text.TextProto.cinch) · line 304

<a id="value-726561642d646f742d6279746573"></a>

## `read-dot-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">read-dot-bytes</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">TextProto</span>.<span class="tok-constructor">TextProtoError</span> (<span class="tok-constructor">Bytes</span>, <span class="tok-constructor">Nat</span>))))</code></pre>

Decode a dot-encoded body up to the terminating ".\r\n" or ".\n": a leading dot is removed from each
line, CRLF becomes LF, and a CR not followed by LF is kept. Returns the body and the offset after the terminator.

[Source](/reference-sources/Text.TextProto.cinch) · line 426

<a id="value-726561642d6c696e65"></a>

## `read-line`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">read-line</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">TextProto</span>.<span class="tok-constructor">TextProtoError</span> (<span class="tok-constructor">Bytes</span>, <span class="tok-constructor">Nat</span>))))</code></pre>

Read one line without its line ending (LF or CRLF); returns the line and the offset after the LF.

[Source](/reference-sources/Text.TextProto.cinch) · line 297

<a id="value-726561642d6d696d652d686561646572"></a>

## `read-mime-header`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">read-mime-header</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">TextProto</span>.<span class="tok-constructor">TextProtoError</span> (<span class="tok-constructor">Text</span>.<span class="tok-constructor">TextProto</span>.<span class="tok-constructor">MimeHeader</span>, <span class="tok-constructor">Nat</span>))))</code></pre>

Read header lines up to and including the blank line that ends them.

[Source](/reference-sources/Text.TextProto.cinch) · line 319

<a id="value-726561642d726573706f6e7365"></a>

## `read-response`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">read-response</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">TextProto</span>.<span class="tok-constructor">TextProtoError</span> (<span class="tok-constructor">Nat</span>, <span class="tok-constructor">Bytes</span>, <span class="tok-constructor">Nat</span>)))))</code></pre>

Read a possibly multi-line response: lines "ddd-text" continue it, and a line "ddd text" with the same
code ends it; other lines inside are kept verbatim. Message lines are joined with LF.

[Source](/reference-sources/Text.TextProto.cinch) · line 386

<a id="value-72656e6465722d686561646572"></a>

## `render-header`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">render-header</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">TextProto</span>.<span class="tok-constructor">MimeHeader</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Render header fields as "Key: value\r\n" lines in order, followed by the blank line.

[Source](/reference-sources/Text.TextProto.cinch) · line 502

<a id="value-746578742d70726f746f2d6572726f722d6d657373616765"></a>

## `text-proto-error-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">text-proto-error-message</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">TextProto</span>.<span class="tok-constructor">TextProtoError</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Render a text protocol error.

[Source](/reference-sources/Text.TextProto.cinch) · line 38

<a id="value-7472696d2d6279746573"></a>

## `trim-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">trim-bytes</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Remove leading and trailing ASCII whitespace (space, tab, CR, LF).

[Source](/reference-sources/Text.TextProto.cinch) · line 275

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Text/TextProto.cinch`. Source SHA-256: `1a6cee0680bfb10043fd09c373100c091c829f4f65c8e6fbc648bb7766a1373f`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
