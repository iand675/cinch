# Text.Csv.Record

Configurable Unicode-delimited and lossless byte CSV records, checked partial results, positions and incremental parser state.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Text.Csv.Record`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.Csv.Record.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Char](Data.Char.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md).

Non-embedded source closure: [Data.Bytes.Word](Data.Bytes.Word.md), [Text.Csv.Record](Text.Csv.Record.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Configurable CSV records with immutable reader continuations and byte-accurate positions.
Delimiters and comments are Unicode scalars. CRLF becomes LF inside records,
blank physical lines and configured comment lines are skipped. Byte records
preserve arbitrary field bytes; text records validate UTF-8. Both interfaces
preserve completed fields alongside parse errors.
Open-prefix readers distinguish a complete row from input that may continue.

</details>

# Types and constructors

<a id="type-4373764279746552656164"></a>

## `CsvByteRead`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">CsvByteRead</span></code></pre>

A lossless byte reader step retains completed arbitrary-byte fields and parse errors together.

[Source](/reference-sources/Text.Csv.Record.cinch) · line 240

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">CsvByteEnd</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvByteRead</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">CsvByteNeedInput</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvByteRead</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">CsvReadByteRecord</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvByteRecord</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvRecordError</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvByteRead</span>))</code></pre>

<a id="type-43737642797465526561646572"></a>

## `CsvByteReader`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">CsvByteReader</span></code></pre>

An immutable lossless byte-input reader that cannot be mistaken for a validated text reader.

[Source](/reference-sources/Text.Csv.Record.cinch) · line 254

<a id="type-437376427974655265636f7264"></a>

## `CsvByteRecord`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">CsvByteRecord</span></code></pre>

Immutable arbitrary-byte fields with the same original source positions as text records.

[Source](/reference-sources/Text.Csv.Record.cinch) · line 111

<a id="type-437376436f6e6669674572726f72"></a>

## `CsvConfigError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">CsvConfigError</span></code></pre>

Invalid delimiter/comment configuration is rejected before reading or writing.

[Source](/reference-sources/Text.Csv.Record.cinch) · line 52

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">CsvInvalidDelimiter</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvConfigError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">CsvInvalidComment</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvConfigError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">CsvDelimiterCommentCollision</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvConfigError</span></code></pre>

<a id="type-4373764669656c64506f6c696379"></a>

## `CsvFieldPolicy`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">CsvFieldPolicy</span></code></pre>

A row's field-count rule, independent of text quoting and separators.

[Source](/reference-sources/Text.Csv.Record.cinch) · line 38

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">CsvAnyFields</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvFieldPolicy</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">CsvInferFields</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvFieldPolicy</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">CsvExactFields</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvFieldPolicy</span>)</code></pre>

<a id="type-4373764f7074696f6e73"></a>

## `CsvOptions`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">CsvOptions</span></code></pre>

A validated immutable CSV dialect and parsing policy.

[Source](/reference-sources/Text.Csv.Record.cinch) · line 47

<a id="type-43737652656164"></a>

## `CsvRead`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">CsvRead</span></code></pre>

One reader step distinguishes EOF, an unfinished open prefix and a row with optional error.

[Source](/reference-sources/Text.Csv.Record.cinch) · line 231

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">CsvEnd</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvRead</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">CsvNeedInput</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvRead</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">CsvReadRecord</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvRecord</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvRecordError</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvRead</span>))</code></pre>

<a id="type-437376526561646572"></a>

## `CsvReader`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">CsvReader</span></code></pre>

Immutable in-memory reader with an original byte/line origin and explicit EOF policy.

[Source](/reference-sources/Text.Csv.Record.cinch) · line 249

<a id="type-4373765265636f7264"></a>

## `CsvRecord`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">CsvRecord</span></code></pre>

A row's immutable fields, original starting line and one-based byte positions.

[Source](/reference-sources/Text.Csv.Record.cinch) · line 106

<a id="type-4373765265636f72644572726f72"></a>

## `CsvRecordError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">CsvRecordError</span></code></pre>

CSV row errors retain original one-based physical lines and byte columns.

[Source](/reference-sources/Text.Csv.Record.cinch) · line 156

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">CsvBareQuote</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvRecordError</span>)))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">CsvQuote</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvRecordError</span>)))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">CsvUnclosedQuote</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvRecordError</span>)))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">CsvFieldCount</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvRecordError</span>)))))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">CsvFieldEncoding</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvRecordError</span>))))</code></pre>

# Values

<a id="value-6373762d627974652d726561646572"></a>

## `csv-byte-reader`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">csv-byte-reader</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvOptions</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvByteReader</span>))</code></pre>

Begin lossless reading of arbitrary CSV bytes, snapshotting the caller's mutable buffer.

[Source](/reference-sources/Text.Csv.Record.cinch) · line 259

<a id="value-6373762d627974652d7265616465722d6174"></a>

## `csv-byte-reader-at`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">csv-byte-reader-at</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvOptions</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvByteReader</span>)))))</code></pre>

Begin reading an arbitrary byte prefix with an original byte/line origin and explicit EOF status.

[Source](/reference-sources/Text.Csv.Record.cinch) · line 263

<a id="value-6373762d627974652d7265616465722d636f6e666967757265"></a>

## `csv-byte-reader-configure`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">csv-byte-reader-configure</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvByteReader</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvOptions</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvByteReader</span>))</code></pre>

Reconfigure the next lossless byte record without discarding unread bytes or changing an earlier reader.

[Source](/reference-sources/Text.Csv.Record.cinch) · line 306

<a id="value-6373762d627974652d7265616465722d696e7075742d6f6666736574"></a>

## `csv-byte-reader-input-offset`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">csv-byte-reader-input-offset</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvByteReader</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Observe the original input offset after the last consumed raw physical line.

[Source](/reference-sources/Text.Csv.Record.cinch) · line 267

<a id="value-6373762d627974652d7265616465722d6e6578742d6c696e65"></a>

## `csv-byte-reader-next-line`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">csv-byte-reader-next-line</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvByteReader</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Observe the next physical line in an immutable byte-reader continuation.

[Source](/reference-sources/Text.Csv.Record.cinch) · line 271

<a id="value-6373762d627974652d7265616465722d6f7074696f6e73"></a>

## `csv-byte-reader-options`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">csv-byte-reader-options</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvByteReader</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvOptions</span>)</code></pre>

Observe validated options and the field count inferred by prior byte records.

[Source](/reference-sources/Text.Csv.Record.cinch) · line 275

<a id="value-6373762d627974652d7265616465722d72656164"></a>

## `csv-byte-reader-read`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">csv-byte-reader-read</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvByteReader</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvByteRead</span>, <span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvByteReader</span>))</code></pre>

Read the next lossless byte record, retaining complete fields beside later quote/count errors.

[Source](/reference-sources/Text.Csv.Record.cinch) · line 677

<a id="value-6373762d627974652d7265616465722d726561642d616c6c"></a>

## `csv-byte-reader-read-all`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">csv-byte-reader-read-all</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvByteReader</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvRecordError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvByteRecord</span>)))</code></pre>

Read all remaining byte records atomically, preserving arbitrary field bytes on success.

[Source](/reference-sources/Text.Csv.Record.cinch) · line 683

<a id="value-6373762d627974652d7265636f72642d6669656c642d706f736974696f6e"></a>

## `csv-byte-record-field-position`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">csv-byte-record-field-position</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvByteRecord</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Nat</span>, <span class="tok-constructor">Nat</span>))))</code></pre>

Safely observe the one-based physical line and byte column of a zero-based raw field index.

[Source](/reference-sources/Text.Csv.Record.cinch) · line 134

<a id="value-6373762d627974652d7265636f72642d6669656c6473"></a>

## `csv-byte-record-fields`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">csv-byte-record-fields</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvByteRecord</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Bytes</span>))</code></pre>

Return independent copies of every field, preserving invalid UTF-8 and embedded NUL bytes.

[Source](/reference-sources/Text.Csv.Record.cinch) · line 116

<a id="value-6373762d627974652d7265636f72642d73746172742d6c696e65"></a>

## `csv-byte-record-start-line`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">csv-byte-record-start-line</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvByteRecord</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Observe the starting physical line without decoding a field's bytes.

[Source](/reference-sources/Text.Csv.Record.cinch) · line 130

<a id="value-6373762d627974652d7265636f72642d74657874"></a>

## `csv-byte-record-text`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">csv-byte-record-text</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvByteRecord</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvRecord</span>, (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvRecordError</span>)))</code></pre>

Convert raw fields to immutable text, preserving the valid field prefix on an encoding error.
A failed conversion points at the offending field's original start and leaves the byte record unchanged.

[Source](/reference-sources/Text.Csv.Record.cinch) · line 206

<a id="value-6373762d636f6e6669672d6572726f722d6d657373616765"></a>

## `csv-config-error-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">csv-config-error-message</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvConfigError</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Render a configuration diagnostic without accessing input or output storage.

[Source](/reference-sources/Text.Csv.Record.cinch) · line 61

<a id="value-6373762d64656661756c742d6f7074696f6e73"></a>

## `csv-default-options`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">csv-default-options</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvOptions</span></code></pre>

Comma-separated records with inferred field count, strict quotes and no space trimming or comments.

[Source](/reference-sources/Text.Csv.Record.cinch) · line 82

<a id="value-6373762d6f7074696f6e73"></a>

## `csv-options`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">csv-options</span> :: (<span class="tok-constructor">Char</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Char</span>) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvFieldPolicy</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvConfigError</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvOptions</span>))))))</code></pre>

Validate delimiter/comment scalars and choose the row's field/quote policies.

[Source](/reference-sources/Text.Csv.Record.cinch) · line 72

<a id="value-6373762d6f7074696f6e732d636f6d6d656e74"></a>

## `csv-options-comment`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">csv-options-comment</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvOptions</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Char</span>))</code></pre>

Observe the optional comment marker, which is recognized only at physical column one.

[Source](/reference-sources/Text.Csv.Record.cinch) · line 94

<a id="value-6373762d6f7074696f6e732d64656c696d69746572"></a>

## `csv-options-delimiter`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">csv-options-delimiter</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvOptions</span> -&gt; <span class="tok-constructor">Char</span>)</code></pre>

Observe the validated delimiter scalar.

[Source](/reference-sources/Text.Csv.Record.cinch) · line 86

<a id="value-6373762d6f7074696f6e732d6669656c642d706f6c696379"></a>

## `csv-options-field-policy`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">csv-options-field-policy</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvOptions</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvFieldPolicy</span>)</code></pre>

Observe the current field-count rule, including an inferred count after reading.

[Source](/reference-sources/Text.Csv.Record.cinch) · line 90

<a id="value-6373762d6f7074696f6e732d6c617a792d71756f746573"></a>

## `csv-options-lazy-quotes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">csv-options-lazy-quotes</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvOptions</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Whether quotation marks inside unquoted fields or non-escaped quoted fields are accepted literally.

[Source](/reference-sources/Text.Csv.Record.cinch) · line 98

<a id="value-6373762d6f7074696f6e732d7472696d2d6c656164696e672d7370616365"></a>

## `csv-options-trim-leading-space`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">csv-options-trim-leading-space</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvOptions</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Whether leading Unicode White_Space is removed before deciding each field's quoted form.

[Source](/reference-sources/Text.Csv.Record.cinch) · line 102

<a id="value-6373762d7265616465722d636f6e666967757265"></a>

## `csv-reader-configure`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">csv-reader-configure</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvReader</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvOptions</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvReader</span>))</code></pre>

Select validated policies for the next record while preserving the input cursor and original positions.
Retaining the previous reader preserves its previous policies and continuation.

[Source](/reference-sources/Text.Csv.Record.cinch) · line 302

<a id="value-6373762d7265616465722d696e7075742d6f6666736574"></a>

## `csv-reader-input-offset`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">csv-reader-input-offset</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvReader</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Original input byte offset immediately after the last consumed physical line.

[Source](/reference-sources/Text.Csv.Record.cinch) · line 289

<a id="value-6373762d7265616465722d6e6578742d6c696e65"></a>

## `csv-reader-next-line`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">csv-reader-next-line</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvReader</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Physical line number that the next read will begin with.

[Source](/reference-sources/Text.Csv.Record.cinch) · line 293

<a id="value-6373762d7265616465722d6f7074696f6e73"></a>

## `csv-reader-options`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">csv-reader-options</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvReader</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvOptions</span>)</code></pre>

Observe the current options, including any field count learned from a previous row.

[Source](/reference-sources/Text.Csv.Record.cinch) · line 297

<a id="value-6373762d7265616465722d72656164"></a>

## `csv-reader-read`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">csv-reader-read</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvReader</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvRead</span>, <span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvReader</span>))</code></pre>

Read the next validated text record with completed-field progress and an advanced reader.
Input originated as immutable UTF-8 text; byte fields are converted after common CSV parsing.

[Source](/reference-sources/Text.Csv.Record.cinch) · line 668

<a id="value-6373762d7265616465722d726561642d616c6c"></a>

## `csv-reader-read-all`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">csv-reader-read-all</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvReader</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvRecordError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvRecord</span>)))</code></pre>

Read the remaining records atomically, returning the first parse error without a partial document.
An open unfinished prefix returns the same unfinished-quote error vocabulary at its next line.

[Source](/reference-sources/Text.Csv.Record.cinch) · line 696

<a id="value-6373762d7265636f72642d6572726f722d636f6c756d6e"></a>

## `csv-record-error-column`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">csv-record-error-column</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvRecordError</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

One-based original byte column containing the error.

[Source](/reference-sources/Text.Csv.Record.cinch) · line 187

<a id="value-6373762d7265636f72642d6572726f722d6c696e65"></a>

## `csv-record-error-line`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">csv-record-error-line</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvRecordError</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Physical line containing the error, which can differ from its row's starting line.

[Source](/reference-sources/Text.Csv.Record.cinch) · line 178

<a id="value-6373762d7265636f72642d6572726f722d6d657373616765"></a>

## `csv-record-error-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">csv-record-error-message</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvRecordError</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Render a stable semantic diagnostic; structured accessors retain the full source position.

[Source](/reference-sources/Text.Csv.Record.cinch) · line 196

<a id="value-6373762d7265636f72642d6572726f722d73746172742d6c696e65"></a>

## `csv-record-error-start-line`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">csv-record-error-start-line</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvRecordError</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Starting physical line of the row associated with an error.

[Source](/reference-sources/Text.Csv.Record.cinch) · line 169

<a id="value-6373762d7265636f72642d6669656c642d706f736974696f6e"></a>

## `csv-record-field-position`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">csv-record-field-position</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvRecord</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Nat</span>, <span class="tok-constructor">Nat</span>))))</code></pre>

Safely observe a field's one-based physical line and byte column by zero-based field index.

[Source](/reference-sources/Text.Csv.Record.cinch) · line 146

<a id="value-6373762d7265636f72642d6669656c6473"></a>

## `csv-record-fields`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">csv-record-fields</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvRecord</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

Observe all fields, including completed fields returned with a later parse error.

[Source](/reference-sources/Text.Csv.Record.cinch) · line 138

<a id="value-6373762d7265636f72642d726561646572"></a>

## `csv-record-reader`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">csv-record-reader</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvOptions</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvReader</span>))</code></pre>

Begin reading a complete immutable UTF-8 document at byte zero and physical line one.

[Source](/reference-sources/Text.Csv.Record.cinch) · line 279

<a id="value-6373762d7265636f72642d7265616465722d6174"></a>

## `csv-record-reader-at`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">csv-record-reader-at</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvOptions</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvReader</span>)))))</code></pre>

Read a UTF-8 prefix with an original byte origin and first physical line.
A false closed flag returns CsvNeedInput when the last row may continue.
This permits stream adapters to retain one unfinished row rather than a whole source.

[Source](/reference-sources/Text.Csv.Record.cinch) · line 285

<a id="value-6373762d7265636f72642d73746172742d6c696e65"></a>

## `csv-record-start-line`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">csv-record-start-line</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvRecord</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

The one-based physical line where the row began, before any multiline quoted field.

[Source](/reference-sources/Text.Csv.Record.cinch) · line 142

<a id="value-6373762d72656e6465722d627974652d7265636f7264"></a>

## `csv-render-byte-record`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">csv-render-byte-record</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvOptions</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Bytes</span>) -&gt; <span class="tok-constructor">Bytes</span>)))</code></pre>

Render arbitrary byte fields with minimal quoting and an explicit LF or CRLF terminator.
Invalid UTF-8 and embedded NUL bytes are preserved; quoted CR is omitted and LF becomes CRLF when selected.

[Source](/reference-sources/Text.Csv.Record.cinch) · line 844

<a id="value-6373762d72656e6465722d627974652d7265636f726473"></a>

## `csv-render-byte-records`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">csv-render-byte-records</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvOptions</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Bytes</span>)) -&gt; <span class="tok-constructor">Bytes</span>)))</code></pre>

Render multiple arbitrary-byte records in order without requiring UTF-8 text conversion.

[Source](/reference-sources/Text.Csv.Record.cinch) · line 859

<a id="value-6373762d72656e6465722d7265636f7264"></a>

## `csv-render-record`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">csv-render-record</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvOptions</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>) -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)))</code></pre>

Render one immutable text record with the same minimal quoting and newline policy as byte records.

[Source](/reference-sources/Text.Csv.Record.cinch) · line 871

<a id="value-6373762d72656e6465722d7265636f726473"></a>

## `csv-render-records`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">csv-render-records</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvOptions</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)) -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)))</code></pre>

Render text records in order with a consistent delimiter and LF/CRLF policy.

[Source](/reference-sources/Text.Csv.Record.cinch) · line 883

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Text/Csv/Record.cinch`. Source SHA-256: `a9f4220c17a27ef09808c2835109cf7b5e9e83fddae72d23bc649e0923c11806`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
