# Text.Multipart

Multipart wire primitives over arbitrary bytes, exact multi-value headers and explicit delimiter observations.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Text.Multipart`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.Multipart.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Ordering](Data.Ordering.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Data.Wide](Data.Wide.md), [Text.Mime](Text.Mime.md).

Non-embedded source closure: [Data.Bytes.Utf8](Data.Bytes.Utf8.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Unicode.Kernel](Data.Unicode.Kernel.md), [Data.Unicode.Tables](Data.Unicode.Tables.md), [Text.Base64.Encoding](Text.Base64.Encoding.md), [Text.Mime](Text.Mime.md), [Text.Multipart](Text.Multipart.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Multipart wire primitives over arbitrary bytes, exact multi-value headers and
explicit delimiter observations. Retained values and returned byte views are
independently owned snapshots. Boundary scans never read outside a checked span.

</details>

# Types and constructors

<a id="type-4d756c746970617274426f64795363616e"></a>

## `MultipartBodyScan`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">MultipartBodyScan</span></code></pre>

A body scan separates confirmed payload from incomplete delimiters and end.

[Source](/reference-sources/Text.Multipart.cinch) · line 405

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MultipartBodyPrefix</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartBodyScan</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MultipartBodyNeedMore</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartBodyScan</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MultipartBodyEnd</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartBodyScan</span>)</code></pre>

<a id="type-4d756c74697061727444656c696d69746572"></a>

## `MultipartDelimiter`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">MultipartDelimiter</span></code></pre>

Delimiter recognition retains the first ordinary line's newline policy.

[Source](/reference-sources/Text.Multipart.cinch) · line 536

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MultipartNoDelimiter</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartDelimiter</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MultipartNextDelimiter</span> :: (<span class="tok-constructor">Bool</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartDelimiter</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MultipartFinalDelimiter</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartDelimiter</span></code></pre>

<a id="type-4d756c74697061727448656164657273"></a>

## `MultipartHeaders`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">MultipartHeaders</span></code></pre>

Privately retained exact keys and ordered values.

[Source](/reference-sources/Text.Multipart.cinch) · line 114

<a id="type-4d756c74697061727453796e746178"></a>

## `MultipartSyntax`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">MultipartSyntax</span></code></pre>

Named protocol refusals retain the original offending wire bytes.

[Source](/reference-sources/Text.Multipart.cinch) · line 33

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MultipartBoundaryLength</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartSyntax</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MultipartBoundaryCharacter</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartSyntax</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MultipartHeaderInitial</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartSyntax</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MultipartHeaderMissingColon</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartSyntax</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MultipartHeaderMalformed</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartSyntax</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MultipartMessageTooLarge</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartSyntax</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MultipartEmptyBoundary</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartSyntax</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MultipartUnexpectedLine</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartSyntax</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MultipartExpectedPart</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartSyntax</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MultipartBufferFull</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartSyntax</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MultipartUnexpectedEnd</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartSyntax</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MultipartInputEnd</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartSyntax</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MultipartNoProgress</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartSyntax</span></code></pre>

# Values

<a id="value-6d756c7469706172742d626f64792d7363616e"></a>

## `multipart-body-scan`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">multipart-body-scan</span> :: <span class="tok-keyword">using</span> ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-variable">bytes-len</span> <span class="tok-variable">arg0</span>) <span class="tok-operator">&lt;=</span> <span class="tok-number">4096</span>) =&gt; ((<span class="tok-variable">arg0</span> :: <span class="tok-constructor">Bytes</span>) -&gt; ((<span class="tok-variable">arg1</span> :: <span class="tok-constructor">Bytes</span>) -&gt; ((<span class="tok-variable">arg2</span> :: <span class="tok-constructor">Bool</span>) -&gt; ((<span class="tok-variable">arg3</span> :: <span class="tok-constructor">Bool</span>) -&gt; ((<span class="tok-variable">arg4</span> :: <span class="tok-constructor">Bool</span>) -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartBodyScan</span>)))))</code></pre>

Observe one bounded raw-body span; terminal status decides ambiguous suffixes.

[Source](/reference-sources/Text.Multipart.cinch) · line 463

<a id="value-6d756c7469706172742d636865636b2d626f756e64617279"></a>

## `multipart-check-boundary`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">multipart-check-boundary</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartSyntax</span> <span class="tok-constructor">Unit</span>))</code></pre>

Validate a writer boundary without imposing this restriction on readers.

[Source](/reference-sources/Text.Multipart.cinch) · line 268

<a id="value-6d756c7469706172742d64656c696d697465722d6c696e65"></a>

## `multipart-delimiter-line`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">multipart-delimiter-line</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartDelimiter</span>))))</code></pre>

Classify a complete line without validating a reader's arbitrary boundary.

[Source](/reference-sources/Text.Multipart.cinch) · line 554

<a id="value-6d756c7469706172742d6669656c642d646973706f736974696f6e"></a>

## `multipart-field-disposition`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">multipart-field-disposition</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Render the exact disposition for a named value field.

[Source](/reference-sources/Text.Multipart.cinch) · line 311

<a id="value-6d756c7469706172742d66696c652d646973706f736974696f6e"></a>

## `multipart-file-disposition`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">multipart-file-disposition</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Bytes</span>))</code></pre>

Render the exact disposition for a named file field.

[Source](/reference-sources/Text.Multipart.cinch) · line 315

<a id="value-6d756c7469706172742d66696e642d62797465"></a>

## `multipart-find-byte`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">multipart-find-byte</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">Nat</span>)))</code></pre>

Return the first target byte at or after start, or the input length.

[Source](/reference-sources/Text.Multipart.cinch) · line 388

<a id="value-6d756c7469706172742d666f726d2d636f6e74656e742d74797065"></a>

## `multipart-form-content-type`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">multipart-form-content-type</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Render a form content type with exactly the required boundary quoting.

[Source](/reference-sources/Text.Multipart.cinch) · line 282

<a id="value-6d756c7469706172742d6865616465722d617070656e64"></a>

## `multipart-header-append`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">multipart-header-append</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartHeaders</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartHeaders</span>)))</code></pre>

Append a value under a canonicalized lookup key.

[Source](/reference-sources/Text.Multipart.cinch) · line 218

<a id="value-6d756c7469706172742d6865616465722d617070656e642d726177"></a>

## `multipart-header-append-raw`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">multipart-header-append-raw</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartHeaders</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartHeaders</span>)))</code></pre>

Append an independently retained raw-key value without changing key spelling.

[Source](/reference-sources/Text.Multipart.cinch) · line 206

<a id="value-6d756c7469706172742d6865616465722d64726f70"></a>

## `multipart-header-drop`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">multipart-header-drop</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartHeaders</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartHeaders</span>))</code></pre>

Delete a canonicalized lookup key without touching differently spelled raw keys.

[Source](/reference-sources/Text.Multipart.cinch) · line 222

<a id="value-6d756c7469706172742d6865616465722d64726f702d726177"></a>

## `multipart-header-drop-raw`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">multipart-header-drop-raw</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartHeaders</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartHeaders</span>))</code></pre>

Delete only the exact raw-key entry.

[Source](/reference-sources/Text.Multipart.cinch) · line 210

<a id="value-6d756c7469706172742d6865616465722d676574"></a>

## `multipart-header-get`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">multipart-header-get</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartHeaders</span> -&gt; <span class="tok-constructor">Bytes</span>))</code></pre>

Read the first canonical-key value, using an empty buffer for absent/empty lists.

[Source](/reference-sources/Text.Multipart.cinch) · line 198

<a id="value-6d756c7469706172742d6865616465722d6b6579"></a>

## `multipart-header-key`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">multipart-header-key</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Canonicalize token keys; invalid and space-containing raw keys stay exact.

[Source](/reference-sources/Text.Multipart.cinch) · line 188

<a id="value-6d756c7469706172742d6865616465722d7061697273"></a>

## `multipart-header-pairs`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">multipart-header-pairs</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartHeaders</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-constructor">Bytes</span>, (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Bytes</span>))))</code></pre>

Observe sorted raw keys and original per-key value order in fresh storage.

[Source](/reference-sources/Text.Multipart.cinch) · line 161

<a id="value-6d756c7469706172742d6865616465722d736574"></a>

## `multipart-header-set`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">multipart-header-set</span> :: (<span class="tok-constructor">Bytes</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Bytes</span>) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartHeaders</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartHeaders</span>)))</code></pre>

Replace the values under a canonicalized lookup key.

[Source](/reference-sources/Text.Multipart.cinch) · line 214

<a id="value-6d756c7469706172742d6865616465722d7365742d726177"></a>

## `multipart-header-set-raw`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">multipart-header-set-raw</span> :: (<span class="tok-constructor">Bytes</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Bytes</span>) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartHeaders</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartHeaders</span>)))</code></pre>

Replace an exact raw key with privately owned values.

[Source](/reference-sources/Text.Multipart.cinch) · line 202

<a id="value-6d756c7469706172742d6865616465722d73697a65"></a>

## `multipart-header-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">multipart-header-size</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartHeaders</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Exact form-budget charge: four hundred plus per-key and value overhead.

[Source](/reference-sources/Text.Multipart.cinch) · line 234

<a id="value-6d756c7469706172742d6865616465722d76616c756573"></a>

## `multipart-header-values`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">multipart-header-values</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartHeaders</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Bytes</span>)))</code></pre>

Read every value for a canonicalized lookup key into fresh snapshots.

[Source](/reference-sources/Text.Multipart.cinch) · line 194

<a id="value-6d756c7469706172742d6865616465722d77697265"></a>

## `multipart-header-wire`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">multipart-header-wire</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartHeaders</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Serialize a complete header block with sorted raw keys and a final blank line.

[Source](/reference-sources/Text.Multipart.cinch) · line 252

<a id="value-6d756c7469706172742d68656164657273"></a>

## `multipart-headers`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">multipart-headers</span> :: ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-constructor">Bytes</span>, (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Bytes</span>))) -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartHeaders</span>)</code></pre>

Construct exact-key headers, including empty lists and arbitrary raw keys.

[Source](/reference-sources/Text.Multipart.cinch) · line 148

<a id="value-6d756c7469706172742d70617273652d6865616465722d6c696e65"></a>

## `multipart-parse-header-line`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">multipart-parse-header-line</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartSyntax</span> (<span class="tok-constructor">Bytes</span>, <span class="tok-constructor">Bytes</span>)))</code></pre>

Parse one unfolded logical header line, preserving exact refused bytes.

[Source](/reference-sources/Text.Multipart.cinch) · line 505

<a id="value-6d756c7469706172742d706f7369782d66696c656e616d65"></a>

## `multipart-posix-filename`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">multipart-posix-filename</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Apply the POSIX basename byte policy, including empty and all-separator cases.

[Source](/reference-sources/Text.Multipart.cinch) · line 528

<a id="value-6d756c7469706172742d707265666978"></a>

## `multipart-prefix`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">multipart-prefix</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

Test a complete raw prefix without exposing retained bytes.

[Source](/reference-sources/Text.Multipart.cinch) · line 401

<a id="value-6d756c7469706172742d73796e7461782d6d657373616765"></a>

## `multipart-syntax-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">multipart-syntax-message</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartSyntax</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Render exact wire diagnostics while retaining the typed refusal.

[Source](/reference-sources/Text.Multipart.cinch) · line 62

<a id="value-6d756c7469706172742d7472696d2d6865616465722d6c696e65"></a>

## `multipart-trim-header-line`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">multipart-trim-header-line</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Trim only header space and tab, never Unicode whitespace or body bytes.

[Source](/reference-sources/Text.Multipart.cinch) · line 486

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Text/Multipart.cinch`. Source SHA-256: `bbbdb0e48e1c8a907097624dd570a5394fe8866f97603d43a49400da85283d75`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
