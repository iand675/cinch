# Effect.Csv

Bounded CSV readers and writers with explicit continuation, record counts, position tracking and advanced handles after failures.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Effect.Csv`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Effect.Csv.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Char](Data.Char.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Data.Wide](Data.Wide.md), [Effect.Stream](Effect.Stream.md), [Text.Csv.Record](Text.Csv.Record.md).

Non-embedded source closure: [Data.Bytes.Word](Data.Bytes.Word.md), [Effect.Csv](Effect.Csv.md), [Effect.Stream](Effect.Stream.md), [Text.Csv.Record](Text.Csv.Record.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Incremental CSV record readers and inspectable synchronous writers.
Readers retain one unfinished record plus a bounded source chunk. Immutable
continuations preserve advanced sources/sinks on success and failure. Source
failures are sticky; CSV parse errors carry partial records and permit the
next record to be read. No operation closes or flushes a borrowed descriptor.

</details>

# Types and constructors

<a id="type-4373764279746553747265616d526561646572"></a>

## `CsvByteStreamReader`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">CsvByteStreamReader</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>)</code></pre>

A lossless byte-stream reader kept distinct from a validating UTF-8 text-stream reader.

[Source](/reference-sources/Effect.Csv.cinch) · line 70

<a id="type-43737653747265616d4661696c757265"></a>

## `CsvStreamFailure`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">CsvStreamFailure</span></code></pre>

A materializing read-all failure distinguishes CSV syntax from source protocol failures.

[Source](/reference-sources/Effect.Csv.cinch) · line 33

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">CsvParseFailure</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvRecordError</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">CsvStreamFailure</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">CsvSourceFailure</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">StreamError</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">CsvStreamFailure</span>)</code></pre>

<a id="type-43737653747265616d526561646572"></a>

## `CsvStreamReader`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">CsvStreamReader</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>)</code></pre>

Immutable CSV source continuation retaining at most one record and bounded source lookahead.

[Source](/reference-sources/Effect.Csv.cinch) · line 63

<a id="type-43737653747265616d577269746572"></a>

## `CsvStreamWriter`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">CsvStreamWriter</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>)</code></pre>

Immutable synchronous record writer with sticky output failure and an inspectable sink.

[Source](/reference-sources/Effect.Csv.cinch) · line 413

# Values

<a id="value-6373762d627974652d73747265616d2d696e7075742d6f6666736574"></a>

## `csv-byte-stream-input-offset`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">csv-byte-stream-input-offset</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">CsvByteStreamReader</span> <span class="tok-variable">r</span>) -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Logical original byte position after the last complete byte record or skipped comment/blank prefix.

[Source](/reference-sources/Effect.Csv.cinch) · line 112

<a id="value-6373762d627974652d73747265616d2d72656164"></a>

## `csv-byte-stream-read`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">csv-byte-stream-read</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">CsvByteStreamReader</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">StreamError</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvByteRead</span>), (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">CsvByteStreamReader</span> <span class="tok-variable">r</span>)))</code></pre>

Read one lossless arbitrary-byte record while preserving quote/count errors and completed fields.

[Source](/reference-sources/Effect.Csv.cinch) · line 380

<a id="value-6373762d627974652d73747265616d2d726561642d616c6c"></a>

## `csv-byte-stream-read-all`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">csv-byte-stream-read-all</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">CsvByteStreamReader</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">CsvStreamFailure</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvByteRecord</span>)), (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">CsvByteStreamReader</span> <span class="tok-variable">r</span>)))</code></pre>

Materialize remaining lossless byte records, returning the first parse/source failure and advanced reader.

[Source](/reference-sources/Effect.Csv.cinch) · line 386

<a id="value-6373762d627974652d73747265616d2d726561646572"></a>

## `csv-byte-stream-reader`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">csv-byte-stream-reader</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>). (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvOptions</span> -&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteReader</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">CsvByteStreamReader</span> <span class="tok-variable">r</span>)))</code></pre>

Begin lossless arbitrary-byte CSV reading with bounded 4096-byte source lookahead.
Delimiters/comments remain Unicode scalars; invalid UTF-8 field bytes are preserved rather than rejected.

[Source](/reference-sources/Effect.Csv.cinch) · line 81

<a id="value-6373762d627974652d73747265616d2d7265616465722d636f6e666967757265"></a>

## `csv-byte-stream-reader-configure`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">csv-byte-stream-reader-configure</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">CsvByteStreamReader</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvOptions</span> -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">CsvByteStreamReader</span> <span class="tok-variable">r</span>)))</code></pre>

Reconfigure the next lossless byte record while preserving its advanced source and unread lookahead.

[Source](/reference-sources/Effect.Csv.cinch) · line 122

<a id="value-6373762d627974652d73747265616d2d7265616465722d6572726f72"></a>

## `csv-byte-stream-reader-error`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">csv-byte-stream-reader-error</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">CsvByteStreamReader</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">StreamError</span>))</code></pre>

Observe a sticky underlying failure without discarding byte-record progress.

[Source](/reference-sources/Effect.Csv.cinch) · line 108

<a id="value-6373762d627974652d73747265616d2d7265616465722d736f75726365"></a>

## `csv-byte-stream-reader-source`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">csv-byte-stream-reader-source</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">CsvByteStreamReader</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteReader</span> <span class="tok-variable">r</span>))</code></pre>

Observe the advanced underlying byte source, including after a sticky source failure.

[Source](/reference-sources/Effect.Csv.cinch) · line 104

<a id="value-6373762d627974652d73747265616d2d7772697465"></a>

## `csv-byte-stream-write`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">csv-byte-stream-write</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">CsvStreamWriter</span> <span class="tok-variable">r</span>) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Bytes</span>) -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">StreamError</span> <span class="tok-constructor">Unit</span>), (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">CsvStreamWriter</span> <span class="tok-variable">r</span>))))</code></pre>

Render and synchronously write arbitrary byte fields, preserving invalid UTF-8 and embedded NUL bytes.

[Source](/reference-sources/Effect.Csv.cinch) · line 452

<a id="value-6373762d627974652d73747265616d2d77726974652d616c6c"></a>

## `csv-byte-stream-write-all`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">csv-byte-stream-write-all</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">CsvStreamWriter</span> <span class="tok-variable">r</span>) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Bytes</span>)) -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">StreamError</span> <span class="tok-constructor">Unit</span>), (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">CsvStreamWriter</span> <span class="tok-variable">r</span>))))</code></pre>

Write arbitrary-byte records in order, preserving the first sticky output failure and advanced sink.

[Source](/reference-sources/Effect.Csv.cinch) · line 475

<a id="value-6373762d73747265616d2d666c757368"></a>

## `csv-stream-flush`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">csv-stream-flush</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">CsvStreamWriter</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">StreamError</span> <span class="tok-constructor">Unit</span>), (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">CsvStreamWriter</span> <span class="tok-variable">r</span>)))</code></pre>

Observe synchronous completion or a sticky failure; preceding writes already reached the sink.
This does not flush or close the borrowed sink, whose protocol has no flush operation.

[Source](/reference-sources/Effect.Csv.cinch) · line 461

<a id="value-6373762d73747265616d2d696e7075742d6f6666736574"></a>

## `csv-stream-input-offset`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">csv-stream-input-offset</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">CsvStreamReader</span> <span class="tok-variable">r</span>) -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Original byte position after the last consumed CSV record or skipped blank/comment prefix.
Lookahead and an unfinished record do not advance this logical input offset.

[Source](/reference-sources/Effect.Csv.cinch) · line 98

<a id="value-6373762d73747265616d2d72656164"></a>

## `csv-stream-read`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">csv-stream-read</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">CsvStreamReader</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">StreamError</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvRead</span>), (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">CsvStreamReader</span> <span class="tok-variable">r</span>)))</code></pre>

Read one validated UTF-8 record with parse progress and an advanced source continuation.

[Source](/reference-sources/Effect.Csv.cinch) · line 370

<a id="value-6373762d73747265616d2d726561642d616c6c"></a>

## `csv-stream-read-all`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">csv-stream-read-all</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">CsvStreamReader</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">CsvStreamFailure</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvRecord</span>)), (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">CsvStreamReader</span> <span class="tok-variable">r</span>)))</code></pre>

Materialize remaining records, returning the first parse/source failure and the advanced reader.
Use csv-stream-read to bound application memory to one record or retain partial records.

[Source](/reference-sources/Effect.Csv.cinch) · line 400

<a id="value-6373762d73747265616d2d726561646572"></a>

## `csv-stream-reader`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">csv-stream-reader</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>). (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvOptions</span> -&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteReader</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">CsvStreamReader</span> <span class="tok-variable">r</span>)))</code></pre>

Begin incrementally reading UTF-8 CSV using bounded 4096-byte source reads.
Memory scales with the largest record, not the total source. Fields remain immutable text.

[Source](/reference-sources/Effect.Csv.cinch) · line 76

<a id="value-6373762d73747265616d2d7265616465722d636f6e666967757265"></a>

## `csv-stream-reader-configure`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">csv-stream-reader-configure</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">CsvStreamReader</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvOptions</span> -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">CsvStreamReader</span> <span class="tok-variable">r</span>)))</code></pre>

Select validated policies for the next record, preserving owned lookahead and any sticky source failure.

[Source](/reference-sources/Effect.Csv.cinch) · line 116

<a id="value-6373762d73747265616d2d7265616465722d6572726f72"></a>

## `csv-stream-reader-error`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">csv-stream-reader-error</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">CsvStreamReader</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">StreamError</span>))</code></pre>

Observe a sticky source or UTF-8 error; ordinary CSV parse errors accompany their partial record.

[Source](/reference-sources/Effect.Csv.cinch) · line 91

<a id="value-6373762d73747265616d2d7265616465722d736f75726365"></a>

## `csv-stream-reader-source`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">csv-stream-reader-source</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">CsvStreamReader</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteReader</span> <span class="tok-variable">r</span>))</code></pre>

Observe the advanced underlying source, including after a sticky read failure.

[Source](/reference-sources/Effect.Csv.cinch) · line 85

<a id="value-6373762d73747265616d2d7772697465"></a>

## `csv-stream-write`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">csv-stream-write</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">CsvStreamWriter</span> <span class="tok-variable">r</span>) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>) -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">StreamError</span> <span class="tok-constructor">Unit</span>), (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">CsvStreamWriter</span> <span class="tok-variable">r</span>))))</code></pre>

Render and write one complete record, retrying valid short writes and preserving failures.

[Source](/reference-sources/Effect.Csv.cinch) · line 442

<a id="value-6373762d73747265616d2d77726974652d616c6c"></a>

## `csv-stream-write-all`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">csv-stream-write-all</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">CsvStreamWriter</span> <span class="tok-variable">r</span>) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)) -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">StreamError</span> <span class="tok-constructor">Unit</span>), (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">CsvStreamWriter</span> <span class="tok-variable">r</span>))))</code></pre>

Write records in order, stopping at the first output failure with its advanced sink.

[Source](/reference-sources/Effect.Csv.cinch) · line 467

<a id="value-6373762d73747265616d2d777269746572"></a>

## `csv-stream-writer`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">csv-stream-writer</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>). (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvOptions</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteWriter</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">CsvStreamWriter</span> <span class="tok-variable">r</span>))))</code></pre>

Begin writing minimally quoted records without buffering or taking ownership of the sink.

[Source](/reference-sources/Effect.Csv.cinch) · line 420

<a id="value-6373762d73747265616d2d7772697465722d636f6e666967757265"></a>

## `csv-stream-writer-configure`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">csv-stream-writer-configure</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">CsvStreamWriter</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">Record</span>.<span class="tok-constructor">CsvOptions</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">CsvStreamWriter</span> <span class="tok-variable">r</span>))))</code></pre>

Select delimiter and newline policies for subsequent records without discarding a sticky output failure.

[Source](/reference-sources/Effect.Csv.cinch) · line 424

<a id="value-6373762d73747265616d2d7772697465722d6572726f72"></a>

## `csv-stream-writer-error`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">csv-stream-writer-error</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">CsvStreamWriter</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">StreamError</span>))</code></pre>

Observe a sticky write failure without performing more output.

[Source](/reference-sources/Effect.Csv.cinch) · line 436

<a id="value-6373762d73747265616d2d7772697465722d73696e6b"></a>

## `csv-stream-writer-sink`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">csv-stream-writer-sink</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Csv</span>.<span class="tok-constructor">CsvStreamWriter</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteWriter</span> <span class="tok-variable">r</span>))</code></pre>

Observe the advanced sink, including its accepted prefix after a failed write.

[Source](/reference-sources/Effect.Csv.cinch) · line 430

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Effect/Csv.cinch`. Source SHA-256: `5d387456fb56dbaeeb3a3e5077e01af6887b6041d155998c23370988e052612b`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
