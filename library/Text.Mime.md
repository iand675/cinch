# Text.Mime

Byte-preserving media types, encoded words, immutable parameter maps and explicit extension registries.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Text.Mime`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.Mime.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Bytes.Utf8](Data.Bytes.Utf8.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Char](Data.Char.md), [Data.Hash](Data.Hash.md), [Data.HashMap](Data.HashMap.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Ordering](Data.Ordering.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Data.Unicode.Kernel](Data.Unicode.Kernel.md), [Relation.Eq](Relation.Eq.md), [Text.Base64.Encoding](Text.Base64.Encoding.md), [Text.Show](Text.Show.md).

Non-embedded source closure: [Data.Bytes.Utf8](Data.Bytes.Utf8.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Unicode.Kernel](Data.Unicode.Kernel.md), [Data.Unicode.Tables](Data.Unicode.Tables.md), [Text.Base64.Encoding](Text.Base64.Encoding.md), [Text.Mime](Text.Mime.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Media type values, raw encoded words and explicit immutable extension contexts.
Wire values and parameter values preserve arbitrary bytes. String conveniences
validate UTF-8 explicitly. Returned buffers and context observations are fresh;
input mutation cannot change retained parameters or registered associations.

</details>

# Types and constructors

<a id="type-4d696d654572726f72"></a>

## `MimeError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">MimeError</span></code></pre>

A precise syntax, registration or encoded-word failure without mutable payloads.

[Source](/reference-sources/Text.Mime.cinch) · line 41

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MimeNoMediaType</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Mime</span>.<span class="tok-constructor">MimeError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MimeExpectedSlash</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Mime</span>.<span class="tok-constructor">MimeError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MimeExpectedSubtype</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Mime</span>.<span class="tok-constructor">MimeError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MimeUnexpectedContent</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Mime</span>.<span class="tok-constructor">MimeError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MimeInvalidMediaParameter</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Mime</span>.<span class="tok-constructor">MimeError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MimeDuplicateParameter</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Mime</span>.<span class="tok-constructor">MimeError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MimeInvalidExtension</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Mime</span>.<span class="tok-constructor">MimeError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MimeInvalidWord</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Mime</span>.<span class="tok-constructor">MimeError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MimeInvalidHexByte</span> :: (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">0</span> <span class="tok-number">255</span> <span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Mime</span>.<span class="tok-constructor">MimeError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MimeBase64Corruption</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Mime</span>.<span class="tok-constructor">MimeError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MimeUnhandledCharset</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Mime</span>.<span class="tok-constructor">MimeError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MimeInvalidUtf8</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Mime</span>.<span class="tok-constructor">MimeError</span>)</code></pre>

<a id="type-4d696d6548656164657250617274"></a>

## `MimeHeaderPart`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">MimeHeaderPart</span></code></pre>

A single header scan observation, preserving literal prefixes and candidate spelling.

[Source](/reference-sources/Text.Mime.cinch) · line 1030

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MimeHeaderEnd</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Mime</span>.<span class="tok-constructor">MimeHeaderPart</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MimeHeaderWord</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Mime</span>.<span class="tok-constructor">MimeError</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Mime</span>.<span class="tok-constructor">MimeWordToken</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Mime</span>.<span class="tok-constructor">MimeHeaderPart</span>))))</code></pre>

<a id="type-4d696d654d65646961526573756c74"></a>

## `MimeMediaResult`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">MimeMediaResult</span></code></pre>

Media parsing observations preserve a valid base but no parameter map on parameter errors.

[Source](/reference-sources/Text.Mime.cinch) · line 201

<a id="type-4d696d65506172616d6574657273"></a>

## `MimeParameters`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">MimeParameters</span></code></pre>

Persistent exact-key parameter map with privately copied values.

[Source](/reference-sources/Text.Mime.cinch) · line 150

<a id="type-4d696d655265676973747279"></a>

## `MimeRegistry`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">MimeRegistry</span></code></pre>

Explicit persistent association context. Exact spellings and lowercase
fallbacks are distinct; inverse lists preserve historical registered types.

[Source](/reference-sources/Text.Mime.cinch) · line 1065

<a id="type-4d696d65576f7264456e636f646572"></a>

## `MimeWordEncoder`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">MimeWordEncoder</span></code></pre>

Preserve any U8 selector, including nonstandard values, without silently restricting it.

[Source](/reference-sources/Text.Mime.cinch) · line 769

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MimeWordEncoder</span> :: (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Mime</span>.<span class="tok-constructor">MimeWordEncoder</span>)</code></pre>

<a id="type-4d696d65576f7264546f6b656e"></a>

## `MimeWordToken`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">MimeWordToken</span></code></pre>

A parsed encoded-word charset label and decoded transfer body.

[Source](/reference-sources/Text.Mime.cinch) · line 873

# Values

<a id="value-6d696d652d622d656e636f64696e67"></a>

## `mime-b-encoding`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mime-b-encoding</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Mime</span>.<span class="tok-constructor">MimeWordEncoder</span></code></pre>

Standard Base64 encoded-word selector, with canonical byte value 98.

[Source](/reference-sources/Text.Mime.cinch) · line 772

<a id="value-6d696d652d636861727365742d6e616d65"></a>

## `mime-charset-name`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mime-charset-name</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Produce the lowercase callback label, repairing each malformed source byte separately.

[Source](/reference-sources/Text.Mime.cinch) · line 995

<a id="value-6d696d652d636f6e766572742d6b6e6f776e2d63686172736574"></a>

## `mime-convert-known-charset`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mime-convert-known-charset</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Bytes</span>)))</code></pre>

Convert the built-in UTF-8, Latin-1 and US-ASCII charsets. UTF-8-labelled
payload bytes pass through without validation; an unknown label returns None.

[Source](/reference-sources/Text.Mime.cinch) · line 987

<a id="value-6d696d652d6572726f722d6d657373616765"></a>

## `mime-error-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mime-error-message</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Mime</span>.<span class="tok-constructor">MimeError</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Render exact familiar wire diagnostics without discarding their error kind.

[Source](/reference-sources/Text.Mime.cinch) · line 83

<a id="value-6d696d652d666f726d61742d6d656469612d74797065"></a>

## `mime-format-media-type`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mime-format-media-type</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Mime</span>.<span class="tok-constructor">MimeParameters</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

Serialize a valid token or major/subtype pair with sorted parameter names.
Invalid type/name arguments return empty; arbitrary value bytes are preserved
through quoted or extended encoding, and all successful output is UTF-8 ASCII.

[Source](/reference-sources/Text.Mime.cinch) · line 573

<a id="value-6d696d652d6865616465722d73746570"></a>

## `mime-header-step`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mime-header-step</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Mime</span>.<span class="tok-constructor">MimeHeaderPart</span>))</code></pre>

Parse one header candidate using permissive header rules, which differ from
full-word rules: empty charsets and literal question marks in Q bodies remain observable.

[Source](/reference-sources/Text.Mime.cinch) · line 1044

<a id="value-6d696d652d6d656469612d62617365"></a>

## `mime-media-base`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mime-media-base</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Mime</span>.<span class="tok-constructor">MimeMediaResult</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Observe the normalized base, or empty when the base/duplicate policy failed.

[Source](/reference-sources/Text.Mime.cinch) · line 206

<a id="value-6d696d652d6d656469612d6572726f72"></a>

## `mime-media-error`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mime-media-error</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Mime</span>.<span class="tok-constructor">MimeMediaResult</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Mime</span>.<span class="tok-constructor">MimeError</span>))</code></pre>

Observe the exact parse failure without flattening it to text.

[Source](/reference-sources/Text.Mime.cinch) · line 214

<a id="value-6d696d652d6d656469612d706172616d6574657273"></a>

## `mime-media-parameters`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mime-media-parameters</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Mime</span>.<span class="tok-constructor">MimeMediaResult</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Mime</span>.<span class="tok-constructor">MimeParameters</span>))</code></pre>

Observe a non-null successful parameter context, including an empty context.

[Source](/reference-sources/Text.Mime.cinch) · line 210

<a id="value-6d696d652d706172616d657465722d66696e64"></a>

## `mime-parameter-find`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mime-parameter-find</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Mime</span>.<span class="tok-constructor">MimeParameters</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Bytes</span>)))</code></pre>

Observe one value through a fresh copy; absence remains distinct from an empty value.

[Source](/reference-sources/Text.Mime.cinch) · line 175

<a id="value-6d696d652d706172616d657465722d7061697273"></a>

## `mime-parameter-pairs`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mime-parameter-pairs</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Mime</span>.<span class="tok-constructor">MimeParameters</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-constructor">Bytes</span>, <span class="tok-constructor">Bytes</span>)))</code></pre>

Return fresh key/value buffers in original raw-key order, before case folding.

[Source](/reference-sources/Text.Mime.cinch) · line 196

<a id="value-6d696d652d706172616d657465722d736574"></a>

## `mime-parameter-set`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mime-parameter-set</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Mime</span>.<span class="tok-constructor">MimeParameters</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Mime</span>.<span class="tok-constructor">MimeParameters</span>)))</code></pre>

Insert one exact key/value snapshot; later equal exact keys replace earlier values.

[Source](/reference-sources/Text.Mime.cinch) · line 159

<a id="value-6d696d652d706172616d6574657273"></a>

## `mime-parameters`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mime-parameters</span> :: ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-constructor">Bytes</span>, <span class="tok-constructor">Bytes</span>)) -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Mime</span>.<span class="tok-constructor">MimeParameters</span>)</code></pre>

Construct a context in source order, so the last equal exact key wins.

[Source](/reference-sources/Text.Mime.cinch) · line 164

<a id="value-6d696d652d706172616d65746572732d656d707479"></a>

## `mime-parameters-empty`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mime-parameters-empty</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Mime</span>.<span class="tok-constructor">MimeParameters</span></code></pre>

Begin with an empty, non-null parameter context.

[Source](/reference-sources/Text.Mime.cinch) · line 155

<a id="value-6d696d652d70617273652d6d656469612d74797065"></a>

## `mime-parse-media-type`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mime-parse-media-type</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Mime</span>.<span class="tok-constructor">MimeMediaResult</span>)</code></pre>

Parse media syntax and RFC 2231 continuations. Successful parameters are non-null;
parameter errors retain the valid base, while conflicting duplicates clear it.

[Source](/reference-sources/Text.Mime.cinch) · line 516

<a id="value-6d696d652d712d656e636f64696e67"></a>

## `mime-q-encoding`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mime-q-encoding</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Mime</span>.<span class="tok-constructor">MimeWordEncoder</span></code></pre>

Standard Q encoded-word selector, with canonical byte value 113.

[Source](/reference-sources/Text.Mime.cinch) · line 776

<a id="value-6d696d652d71756f74652d6279746573"></a>

## `mime-quote-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mime-quote-bytes</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Return an immutable quoted diagnostic spelling of arbitrary bytes.

[Source](/reference-sources/Text.Mime.cinch) · line 1026

<a id="value-6d696d652d72656769737472792d616464"></a>

## `mime-registry-add`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mime-registry-add</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Mime</span>.<span class="tok-constructor">MimeRegistry</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Mime</span>.<span class="tok-constructor">MimeError</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Mime</span>.<span class="tok-constructor">MimeRegistry</span>))))</code></pre>

Register one exact association atomically. Text defaults apply only to a
literal lowercase text/ prefix, and missing or empty charsets become utf-8.

[Source](/reference-sources/Text.Mime.cinch) · line 1085

<a id="value-6d696d652d72656769737472792d6173736f63696174696f6e73"></a>

## `mime-registry-associations`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mime-registry-associations</span> :: ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-constructor">Bytes</span>, <span class="tok-constructor">Bytes</span>)) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Mime</span>.<span class="tok-constructor">MimeRegistry</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Mime</span>.<span class="tok-constructor">MimeError</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Mime</span>.<span class="tok-constructor">MimeRegistry</span>)))</code></pre>

Import associations explicitly in order, returning the first registration
error without modifying the supplied context. Host-specific data is supplied
as ordinary associations rather than read through ambient initialization.

[Source](/reference-sources/Text.Mime.cinch) · line 1140

<a id="value-6d696d652d72656769737472792d64656661756c74"></a>

## `mime-registry-default`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mime-registry-default</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Mime</span>.<span class="tok-constructor">MimeRegistry</span></code></pre>

Stable built-in associations. Additional host-specific records are imported
through mime-registry-associations, keeping configuration explicit and reproducible.

[Source](/reference-sources/Text.Mime.cinch) · line 1150

<a id="value-6d696d652d72656769737472792d656d707479"></a>

## `mime-registry-empty`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mime-registry-empty</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Mime</span>.<span class="tok-constructor">MimeRegistry</span></code></pre>

Begin with no hidden host or process-wide registry state.

[Source](/reference-sources/Text.Mime.cinch) · line 1070

<a id="value-6d696d652d72656769737472792d657874656e73696f6e73"></a>

## `mime-registry-extensions`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mime-registry-extensions</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Mime</span>.<span class="tok-constructor">MimeRegistry</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Mime</span>.<span class="tok-constructor">MimeError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Bytes</span>))))</code></pre>

Return sorted historical lowercase extensions after full media-type validation.
Parameters do not affect the inverse lookup, but parameter errors remain visible.

[Source](/reference-sources/Text.Mime.cinch) · line 1127

<a id="value-6d696d652d72656769737472792d74797065"></a>

## `mime-registry-type`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mime-registry-type</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Mime</span>.<span class="tok-constructor">MimeRegistry</span> -&gt; <span class="tok-constructor">Bytes</span>))</code></pre>

Resolve the exact spelling first, then its Unicode lowercase fallback.
Unknown extensions return a fresh empty buffer. Observations never alias a context.

[Source](/reference-sources/Text.Mime.cinch) · line 1108

<a id="value-6d696d652d776f72642d656e636f64652d6279746573"></a>

## `mime-word-encode-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mime-word-encode-bytes</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Mime</span>.<span class="tok-constructor">MimeWordEncoder</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Bytes</span>)))</code></pre>

Encode arbitrary source bytes. Plain ASCII including tab is returned freshly
unchanged. Only UTF-8-labelled encoded words split at scalar boundaries;
other labels remain unvalidated and unsplit, with no implicit transcoding.

[Source](/reference-sources/Text.Mime.cinch) · line 856

<a id="value-6d696d652d776f72642d656e636f64652d74657874"></a>

## `mime-word-encode-text`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mime-word-encode-text</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Mime</span>.<span class="tok-constructor">MimeWordEncoder</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Mime</span>.<span class="tok-constructor">MimeError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))))</code></pre>

Checked String convenience retaining exact byte-valued selector behavior.
Use the byte API when a nonstandard selector creates non-UTF-8 header bytes.

[Source](/reference-sources/Text.Mime.cinch) · line 864

<a id="value-6d696d652d776f72642d656e636f6465722d62797465"></a>

## `mime-word-encoder-byte`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mime-word-encoder-byte</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Mime</span>.<span class="tok-constructor">MimeWordEncoder</span> -&gt; <span class="tok-constructor">U8</span>)</code></pre>

Observe the exact byte-valued selector used in an encoded-word header.

[Source](/reference-sources/Text.Mime.cinch) · line 780

<a id="value-6d696d652d776f72642d746f6b656e"></a>

## `mime-word-token`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mime-word-token</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Mime</span>.<span class="tok-constructor">MimeError</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Mime</span>.<span class="tok-constructor">MimeWordToken</span>))</code></pre>

Parse a complete single encoded word with exactly four question delimiters.
Empty transfer bodies are accepted; charset labels remain unvalidated bytes.

[Source](/reference-sources/Text.Mime.cinch) · line 958

<a id="value-6d696d652d776f72642d746f6b656e2d63686172736574"></a>

## `mime-word-token-charset`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mime-word-token-charset</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Mime</span>.<span class="tok-constructor">MimeWordToken</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Observe an independent copy of the exact unnormalized charset label.

[Source](/reference-sources/Text.Mime.cinch) · line 878

<a id="value-6d696d652d776f72642d746f6b656e2d636f6e74656e74"></a>

## `mime-word-token-content`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mime-word-token-content</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Mime</span>.<span class="tok-constructor">MimeWordToken</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Observe independent decoded transfer bytes before any charset conversion.

[Source](/reference-sources/Text.Mime.cinch) · line 882

# Instances

These are checked exported instance entries. Law/sample-check status is separate from the existence of an instance; see [laws and optimization](../reference/12-rewrites-and-optimization.md).

| Instance | Target | Resolution |
| --- | --- | --- |
| `Text.Mime.Eq_MimeKey` | `Relation.Eq.Eq Text.Mime.MimeKey` | Implicit |
| `Text.Mime.Hash_MimeKey` | `Data.Hash.Hash Text.Mime.MimeKey` | Implicit |

**`Text.Mime.Eq_MimeKey`**: Compare only private immutable key contents, without changing buffer equality policies.
**`Text.Mime.Hash_MimeKey`**: Reuse stable byte hashing only over immutable privately owned snapshots.

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Text/Mime.cinch`. Source SHA-256: `399fb3c576ab4f4c13a0749dfa25dc72af00078636758a86e73c942018fd46a6`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
