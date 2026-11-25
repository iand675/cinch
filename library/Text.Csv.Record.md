# Text.Csv.Record

Configurable Unicode-delimited and lossless byte CSV records, checked partial results, positions and incremental parser state.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Text.Csv.Record</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Text.Csv.Record/source/">View source</a><a href="/packages/search/?q=%2BText.Csv.Record">Search module</a><a href="/reference-sources/Text.Csv.Record.cinch" download="Text.Csv.Record.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Char](Data.Char.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Data.Wide](Data.Wide.md).

<details class="doc-notes"><summary>Module notes</summary>

Configurable CSV records with immutable reader continuations and byte-accurate positions.
Delimiters and comments are Unicode scalars. CRLF becomes LF inside records,
blank physical lines and configured comment lines are skipped. Byte records
preserve arbitrary field bytes; text records validate UTF-8. Both interfaces
preserve completed fields alongside parse errors.
Open-prefix readers distinguish a complete row from input that may continue.

</details>

<details class="doc-synopsis"><summary>Synopsis (49 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-CsvByteRead">CsvByteRead</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">CsvByteRead</strong></code></li><li><a class="syn-name" href="#t-CsvByteReader">CsvByteReader</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">CsvByteReader</strong></code></li><li><a class="syn-name" href="#t-CsvByteRecord">CsvByteRecord</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">CsvByteRecord</strong></code></li><li><a class="syn-name" href="#t-CsvConfigError">CsvConfigError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">CsvConfigError</strong></code></li><li><a class="syn-name" href="#t-CsvFieldPolicy">CsvFieldPolicy</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">CsvFieldPolicy</strong></code></li><li><a class="syn-name" href="#t-CsvOptions">CsvOptions</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">CsvOptions</strong></code></li><li><a class="syn-name" href="#t-CsvRead">CsvRead</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">CsvRead</strong></code></li><li><a class="syn-name" href="#t-CsvReader">CsvReader</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">CsvReader</strong></code></li><li><a class="syn-name" href="#t-CsvRecord">CsvRecord</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">CsvRecord</strong></code></li><li><a class="syn-name" href="#t-CsvRecordError">CsvRecordError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">CsvRecordError</strong></code></li><li><a class="syn-name" href="#v-csv-byte-reader">csv-byte-reader</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">csv-byte-reader</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvOptions" title="Text.Csv.Record.CsvOptions">CsvOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvByteReader" title="Text.Csv.Record.CsvByteReader">CsvByteReader</a></code></li><li><a class="syn-name" href="#v-csv-byte-reader-at">csv-byte-reader-at</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">csv-byte-reader-at</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvOptions" title="Text.Csv.Record.CsvOptions">CsvOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvByteReader" title="Text.Csv.Record.CsvByteReader">CsvByteReader</a></code></li><li><a class="syn-name" href="#v-csv-byte-reader-configure">csv-byte-reader-configure</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">csv-byte-reader-configure</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvByteReader" title="Text.Csv.Record.CsvByteReader">CsvByteReader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvOptions" title="Text.Csv.Record.CsvOptions">CsvOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvByteReader" title="Text.Csv.Record.CsvByteReader">CsvByteReader</a></code></li><li><a class="syn-name" href="#v-csv-byte-reader-input-offset">csv-byte-reader-input-offset</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">csv-byte-reader-input-offset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvByteReader" title="Text.Csv.Record.CsvByteReader">CsvByteReader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-csv-byte-reader-next-line">csv-byte-reader-next-line</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">csv-byte-reader-next-line</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvByteReader" title="Text.Csv.Record.CsvByteReader">CsvByteReader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-csv-byte-reader-options">csv-byte-reader-options</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">csv-byte-reader-options</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvByteReader" title="Text.Csv.Record.CsvByteReader">CsvByteReader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvOptions" title="Text.Csv.Record.CsvOptions">CsvOptions</a></code></li><li><a class="syn-name" href="#v-csv-byte-reader-read">csv-byte-reader-read</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">csv-byte-reader-read</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvByteReader" title="Text.Csv.Record.CsvByteReader">CsvByteReader</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvByteRead" title="Text.Csv.Record.CsvByteRead">CsvByteRead</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvByteReader" title="Text.Csv.Record.CsvByteReader">CsvByteReader</a>)</code></li><li><a class="syn-name" href="#v-csv-byte-reader-read-all">csv-byte-reader-read-all</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">csv-byte-reader-read-all</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvByteReader" title="Text.Csv.Record.CsvByteReader">CsvByteReader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvRecordError" title="Text.Csv.Record.CsvRecordError">CsvRecordError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvByteRecord" title="Text.Csv.Record.CsvByteRecord">CsvByteRecord</a>)</code></li><li><a class="syn-name" href="#v-csv-byte-record-field-position">csv-byte-record-field-position</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">csv-byte-record-field-position</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvByteRecord" title="Text.Csv.Record.CsvByteRecord">CsvByteRecord</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>)</code></li><li><a class="syn-name" href="#v-csv-byte-record-fields">csv-byte-record-fields</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">csv-byte-record-fields</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvByteRecord" title="Text.Csv.Record.CsvByteRecord">CsvByteRecord</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-csv-byte-record-start-line">csv-byte-record-start-line</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">csv-byte-record-start-line</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvByteRecord" title="Text.Csv.Record.CsvByteRecord">CsvByteRecord</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-csv-byte-record-text">csv-byte-record-text</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">csv-byte-record-text</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvByteRecord" title="Text.Csv.Record.CsvByteRecord">CsvByteRecord</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvRecord" title="Text.Csv.Record.CsvRecord">CsvRecord</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvRecordError" title="Text.Csv.Record.CsvRecordError">CsvRecordError</a>)</code></li><li><a class="syn-name" href="#v-csv-config-error-message">csv-config-error-message</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">csv-config-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvConfigError" title="Text.Csv.Record.CsvConfigError">CsvConfigError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-csv-default-options">csv-default-options</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">csv-default-options</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvOptions" title="Text.Csv.Record.CsvOptions">CsvOptions</a></code></li><li><a class="syn-name" href="#v-csv-options">csv-options</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">csv-options</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvFieldPolicy" title="Text.Csv.Record.CsvFieldPolicy">CsvFieldPolicy</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvConfigError" title="Text.Csv.Record.CsvConfigError">CsvConfigError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvOptions" title="Text.Csv.Record.CsvOptions">CsvOptions</a></code></li><li><a class="syn-name" href="#v-csv-options-comment">csv-options-comment</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">csv-options-comment</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvOptions" title="Text.Csv.Record.CsvOptions">CsvOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a></code></li><li><a class="syn-name" href="#v-csv-options-delimiter">csv-options-delimiter</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">csv-options-delimiter</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvOptions" title="Text.Csv.Record.CsvOptions">CsvOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a></code></li><li><a class="syn-name" href="#v-csv-options-field-policy">csv-options-field-policy</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">csv-options-field-policy</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvOptions" title="Text.Csv.Record.CsvOptions">CsvOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvFieldPolicy" title="Text.Csv.Record.CsvFieldPolicy">CsvFieldPolicy</a></code></li><li><a class="syn-name" href="#v-csv-options-lazy-quotes">csv-options-lazy-quotes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">csv-options-lazy-quotes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvOptions" title="Text.Csv.Record.CsvOptions">CsvOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-csv-options-trim-leading-space">csv-options-trim-leading-space</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">csv-options-trim-leading-space</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvOptions" title="Text.Csv.Record.CsvOptions">CsvOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-csv-reader-configure">csv-reader-configure</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">csv-reader-configure</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvReader" title="Text.Csv.Record.CsvReader">CsvReader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvOptions" title="Text.Csv.Record.CsvOptions">CsvOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvReader" title="Text.Csv.Record.CsvReader">CsvReader</a></code></li><li><a class="syn-name" href="#v-csv-reader-input-offset">csv-reader-input-offset</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">csv-reader-input-offset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvReader" title="Text.Csv.Record.CsvReader">CsvReader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-csv-reader-next-line">csv-reader-next-line</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">csv-reader-next-line</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvReader" title="Text.Csv.Record.CsvReader">CsvReader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-csv-reader-options">csv-reader-options</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">csv-reader-options</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvReader" title="Text.Csv.Record.CsvReader">CsvReader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvOptions" title="Text.Csv.Record.CsvOptions">CsvOptions</a></code></li><li><a class="syn-name" href="#v-csv-reader-read">csv-reader-read</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">csv-reader-read</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvReader" title="Text.Csv.Record.CsvReader">CsvReader</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvRead" title="Text.Csv.Record.CsvRead">CsvRead</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvReader" title="Text.Csv.Record.CsvReader">CsvReader</a>)</code></li><li><a class="syn-name" href="#v-csv-reader-read-all">csv-reader-read-all</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">csv-reader-read-all</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvReader" title="Text.Csv.Record.CsvReader">CsvReader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvRecordError" title="Text.Csv.Record.CsvRecordError">CsvRecordError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvRecord" title="Text.Csv.Record.CsvRecord">CsvRecord</a>)</code></li><li><a class="syn-name" href="#v-csv-record-error-column">csv-record-error-column</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">csv-record-error-column</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvRecordError" title="Text.Csv.Record.CsvRecordError">CsvRecordError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-csv-record-error-line">csv-record-error-line</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">csv-record-error-line</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvRecordError" title="Text.Csv.Record.CsvRecordError">CsvRecordError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-csv-record-error-message">csv-record-error-message</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">csv-record-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvRecordError" title="Text.Csv.Record.CsvRecordError">CsvRecordError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-csv-record-error-start-line">csv-record-error-start-line</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">csv-record-error-start-line</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvRecordError" title="Text.Csv.Record.CsvRecordError">CsvRecordError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-csv-record-field-position">csv-record-field-position</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">csv-record-field-position</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvRecord" title="Text.Csv.Record.CsvRecord">CsvRecord</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>)</code></li><li><a class="syn-name" href="#v-csv-record-fields">csv-record-fields</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">csv-record-fields</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvRecord" title="Text.Csv.Record.CsvRecord">CsvRecord</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-csv-record-reader">csv-record-reader</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">csv-record-reader</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvOptions" title="Text.Csv.Record.CsvOptions">CsvOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvReader" title="Text.Csv.Record.CsvReader">CsvReader</a></code></li><li><a class="syn-name" href="#v-csv-record-reader-at">csv-record-reader-at</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">csv-record-reader-at</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvOptions" title="Text.Csv.Record.CsvOptions">CsvOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvReader" title="Text.Csv.Record.CsvReader">CsvReader</a></code></li><li><a class="syn-name" href="#v-csv-record-start-line">csv-record-start-line</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">csv-record-start-line</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvRecord" title="Text.Csv.Record.CsvRecord">CsvRecord</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-csv-render-byte-record">csv-render-byte-record</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">csv-render-byte-record</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvOptions" title="Text.Csv.Record.CsvOptions">CsvOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-csv-render-byte-records">csv-render-byte-records</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">csv-render-byte-records</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvOptions" title="Text.Csv.Record.CsvOptions">CsvOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-csv-render-record">csv-render-record</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">csv-render-record</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvOptions" title="Text.Csv.Record.CsvOptions">CsvOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-csv-render-records">csv-render-records</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">csv-render-records</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvOptions" title="Text.Csv.Record.CsvOptions">CsvOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-CsvByteRead">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">CsvByteRead</strong></code><span class="doc-sig-links"><a href="#t-CsvByteRead" title="Permalink">#</a><a href="/packages/stdlib/Text.Csv.Record/source/#L241" title="View source at line 241">Source</a></span></div>
<div class="doc-body">

A lossless byte reader step retains completed arbitrary-byte fields and parse errors together.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-CsvByteEnd"><code><strong class="tok-constructor">CsvByteEnd</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvByteRead" title="Text.Csv.Record.CsvByteRead">CsvByteRead</a></code></li><li id="k-CsvByteNeedInput"><code><strong class="tok-constructor">CsvByteNeedInput</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvByteRead" title="Text.Csv.Record.CsvByteRead">CsvByteRead</a></code></li><li id="k-CsvReadByteRecord"><code><strong class="tok-constructor">CsvReadByteRecord</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvByteRecord" title="Text.Csv.Record.CsvByteRecord">CsvByteRecord</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvRecordError" title="Text.Csv.Record.CsvRecordError">CsvRecordError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvByteRead" title="Text.Csv.Record.CsvByteRead">CsvByteRead</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-CsvByteReader">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">CsvByteReader</strong></code><span class="doc-sig-links"><a href="#t-CsvByteReader" title="Permalink">#</a><a href="/packages/stdlib/Text.Csv.Record/source/#L255" title="View source at line 255">Source</a></span></div>
<div class="doc-body">

An immutable lossless byte-input reader that cannot be mistaken for a validated text reader.

</div>
</article>

<article class="doc-entry" id="t-CsvByteRecord">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">CsvByteRecord</strong></code><span class="doc-sig-links"><a href="#t-CsvByteRecord" title="Permalink">#</a><a href="/packages/stdlib/Text.Csv.Record/source/#L112" title="View source at line 112">Source</a></span></div>
<div class="doc-body">

Immutable arbitrary-byte fields with the same original source positions as text records.

</div>
</article>

<article class="doc-entry" id="t-CsvConfigError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">CsvConfigError</strong></code><span class="doc-sig-links"><a href="#t-CsvConfigError" title="Permalink">#</a><a href="/packages/stdlib/Text.Csv.Record/source/#L53" title="View source at line 53">Source</a></span></div>
<div class="doc-body">

Invalid delimiter/comment configuration is rejected before reading or writing.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-CsvInvalidDelimiter"><code><strong class="tok-constructor">CsvInvalidDelimiter</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvConfigError" title="Text.Csv.Record.CsvConfigError">CsvConfigError</a></code></li><li id="k-CsvInvalidComment"><code><strong class="tok-constructor">CsvInvalidComment</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvConfigError" title="Text.Csv.Record.CsvConfigError">CsvConfigError</a></code></li><li id="k-CsvDelimiterCommentCollision"><code><strong class="tok-constructor">CsvDelimiterCommentCollision</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvConfigError" title="Text.Csv.Record.CsvConfigError">CsvConfigError</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-CsvFieldPolicy">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">CsvFieldPolicy</strong></code><span class="doc-sig-links"><a href="#t-CsvFieldPolicy" title="Permalink">#</a><a href="/packages/stdlib/Text.Csv.Record/source/#L39" title="View source at line 39">Source</a></span></div>
<div class="doc-body">

A row's field-count rule, independent of text quoting and separators.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-CsvAnyFields"><code><strong class="tok-constructor">CsvAnyFields</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvFieldPolicy" title="Text.Csv.Record.CsvFieldPolicy">CsvFieldPolicy</a></code></li><li id="k-CsvInferFields"><code><strong class="tok-constructor">CsvInferFields</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvFieldPolicy" title="Text.Csv.Record.CsvFieldPolicy">CsvFieldPolicy</a></code></li><li id="k-CsvExactFields"><code><strong class="tok-constructor">CsvExactFields</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvFieldPolicy" title="Text.Csv.Record.CsvFieldPolicy">CsvFieldPolicy</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-CsvOptions">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">CsvOptions</strong></code><span class="doc-sig-links"><a href="#t-CsvOptions" title="Permalink">#</a><a href="/packages/stdlib/Text.Csv.Record/source/#L48" title="View source at line 48">Source</a></span></div>
<div class="doc-body">

A validated immutable CSV dialect and parsing policy.

</div>
</article>

<article class="doc-entry" id="t-CsvRead">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">CsvRead</strong></code><span class="doc-sig-links"><a href="#t-CsvRead" title="Permalink">#</a><a href="/packages/stdlib/Text.Csv.Record/source/#L232" title="View source at line 232">Source</a></span></div>
<div class="doc-body">

One reader step distinguishes EOF, an unfinished open prefix and a row with optional error.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-CsvEnd"><code><strong class="tok-constructor">CsvEnd</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvRead" title="Text.Csv.Record.CsvRead">CsvRead</a></code></li><li id="k-CsvNeedInput"><code><strong class="tok-constructor">CsvNeedInput</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvRead" title="Text.Csv.Record.CsvRead">CsvRead</a></code></li><li id="k-CsvReadRecord"><code><strong class="tok-constructor">CsvReadRecord</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvRecord" title="Text.Csv.Record.CsvRecord">CsvRecord</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvRecordError" title="Text.Csv.Record.CsvRecordError">CsvRecordError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvRead" title="Text.Csv.Record.CsvRead">CsvRead</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-CsvReader">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">CsvReader</strong></code><span class="doc-sig-links"><a href="#t-CsvReader" title="Permalink">#</a><a href="/packages/stdlib/Text.Csv.Record/source/#L250" title="View source at line 250">Source</a></span></div>
<div class="doc-body">

Immutable in-memory reader with an original byte/line origin and explicit EOF policy.

</div>
</article>

<article class="doc-entry" id="t-CsvRecord">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">CsvRecord</strong></code><span class="doc-sig-links"><a href="#t-CsvRecord" title="Permalink">#</a><a href="/packages/stdlib/Text.Csv.Record/source/#L107" title="View source at line 107">Source</a></span></div>
<div class="doc-body">

A row's immutable fields, original starting line and one-based byte positions.

</div>
</article>

<article class="doc-entry" id="t-CsvRecordError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">CsvRecordError</strong></code><span class="doc-sig-links"><a href="#t-CsvRecordError" title="Permalink">#</a><a href="/packages/stdlib/Text.Csv.Record/source/#L157" title="View source at line 157">Source</a></span></div>
<div class="doc-body">

CSV row errors retain original one-based physical lines and byte columns.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-CsvBareQuote"><code><strong class="tok-constructor">CsvBareQuote</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvRecordError" title="Text.Csv.Record.CsvRecordError">CsvRecordError</a></code></li><li id="k-CsvQuote"><code><strong class="tok-constructor">CsvQuote</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvRecordError" title="Text.Csv.Record.CsvRecordError">CsvRecordError</a></code></li><li id="k-CsvUnclosedQuote"><code><strong class="tok-constructor">CsvUnclosedQuote</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvRecordError" title="Text.Csv.Record.CsvRecordError">CsvRecordError</a></code></li><li id="k-CsvFieldCount"><code><strong class="tok-constructor">CsvFieldCount</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvRecordError" title="Text.Csv.Record.CsvRecordError">CsvRecordError</a></code></li><li id="k-CsvFieldEncoding"><code><strong class="tok-constructor">CsvFieldEncoding</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvRecordError" title="Text.Csv.Record.CsvRecordError">CsvRecordError</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-csv-byte-reader">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">csv-byte-reader</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvOptions" title="Text.Csv.Record.CsvOptions">CsvOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvByteReader" title="Text.Csv.Record.CsvByteReader">CsvByteReader</a></code><span class="doc-sig-links"><a href="#v-csv-byte-reader" title="Permalink">#</a><a href="/packages/stdlib/Text.Csv.Record/source/#L260" title="View source at line 260">Source</a></span></div>
<div class="doc-body">

Begin lossless reading of arbitrary CSV bytes, snapshotting the caller's mutable buffer.

</div>
</article>

<article class="doc-entry" id="v-csv-byte-reader-at">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">csv-byte-reader-at</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvOptions" title="Text.Csv.Record.CsvOptions">CsvOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvByteReader" title="Text.Csv.Record.CsvByteReader">CsvByteReader</a></code><span class="doc-sig-links"><a href="#v-csv-byte-reader-at" title="Permalink">#</a><a href="/packages/stdlib/Text.Csv.Record/source/#L264" title="View source at line 264">Source</a></span></div>
<div class="doc-body">

Begin reading an arbitrary byte prefix with an original byte/line origin and explicit EOF status.

</div>
</article>

<article class="doc-entry" id="v-csv-byte-reader-configure">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">csv-byte-reader-configure</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvByteReader" title="Text.Csv.Record.CsvByteReader">CsvByteReader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvOptions" title="Text.Csv.Record.CsvOptions">CsvOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvByteReader" title="Text.Csv.Record.CsvByteReader">CsvByteReader</a></code><span class="doc-sig-links"><a href="#v-csv-byte-reader-configure" title="Permalink">#</a><a href="/packages/stdlib/Text.Csv.Record/source/#L307" title="View source at line 307">Source</a></span></div>
<div class="doc-body">

Reconfigure the next lossless byte record without discarding unread bytes or changing an earlier reader.

</div>
</article>

<article class="doc-entry" id="v-csv-byte-reader-input-offset">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">csv-byte-reader-input-offset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvByteReader" title="Text.Csv.Record.CsvByteReader">CsvByteReader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-csv-byte-reader-input-offset" title="Permalink">#</a><a href="/packages/stdlib/Text.Csv.Record/source/#L268" title="View source at line 268">Source</a></span></div>
<div class="doc-body">

Observe the original input offset after the last consumed raw physical line.

</div>
</article>

<article class="doc-entry" id="v-csv-byte-reader-next-line">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">csv-byte-reader-next-line</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvByteReader" title="Text.Csv.Record.CsvByteReader">CsvByteReader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-csv-byte-reader-next-line" title="Permalink">#</a><a href="/packages/stdlib/Text.Csv.Record/source/#L272" title="View source at line 272">Source</a></span></div>
<div class="doc-body">

Observe the next physical line in an immutable byte-reader continuation.

</div>
</article>

<article class="doc-entry" id="v-csv-byte-reader-options">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">csv-byte-reader-options</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvByteReader" title="Text.Csv.Record.CsvByteReader">CsvByteReader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvOptions" title="Text.Csv.Record.CsvOptions">CsvOptions</a></code><span class="doc-sig-links"><a href="#v-csv-byte-reader-options" title="Permalink">#</a><a href="/packages/stdlib/Text.Csv.Record/source/#L276" title="View source at line 276">Source</a></span></div>
<div class="doc-body">

Observe validated options and the field count inferred by prior byte records.

</div>
</article>

<article class="doc-entry" id="v-csv-byte-reader-read">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">csv-byte-reader-read</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvByteReader" title="Text.Csv.Record.CsvByteReader">CsvByteReader</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvByteRead" title="Text.Csv.Record.CsvByteRead">CsvByteRead</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvByteReader" title="Text.Csv.Record.CsvByteReader">CsvByteReader</a>)</code><span class="doc-sig-links"><a href="#v-csv-byte-reader-read" title="Permalink">#</a><a href="/packages/stdlib/Text.Csv.Record/source/#L631" title="View source at line 631">Source</a></span></div>
<div class="doc-body">

Read the next lossless byte record, retaining complete fields beside later quote/count errors.

</div>
</article>

<article class="doc-entry" id="v-csv-byte-reader-read-all">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">csv-byte-reader-read-all</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvByteReader" title="Text.Csv.Record.CsvByteReader">CsvByteReader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvRecordError" title="Text.Csv.Record.CsvRecordError">CsvRecordError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvByteRecord" title="Text.Csv.Record.CsvByteRecord">CsvByteRecord</a>)</code><span class="doc-sig-links"><a href="#v-csv-byte-reader-read-all" title="Permalink">#</a><a href="/packages/stdlib/Text.Csv.Record/source/#L637" title="View source at line 637">Source</a></span></div>
<div class="doc-body">

Read all remaining byte records atomically, preserving arbitrary field bytes on success.

</div>
</article>

<article class="doc-entry" id="v-csv-byte-record-field-position">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">csv-byte-record-field-position</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvByteRecord" title="Text.Csv.Record.CsvByteRecord">CsvByteRecord</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>)</code><span class="doc-sig-links"><a href="#v-csv-byte-record-field-position" title="Permalink">#</a><a href="/packages/stdlib/Text.Csv.Record/source/#L135" title="View source at line 135">Source</a></span></div>
<div class="doc-body">

Safely observe the one-based physical line and byte column of a zero-based raw field index.

</div>
</article>

<article class="doc-entry" id="v-csv-byte-record-fields">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">csv-byte-record-fields</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvByteRecord" title="Text.Csv.Record.CsvByteRecord">CsvByteRecord</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-csv-byte-record-fields" title="Permalink">#</a><a href="/packages/stdlib/Text.Csv.Record/source/#L117" title="View source at line 117">Source</a></span></div>
<div class="doc-body">

Return independent copies of every field, preserving invalid UTF-8 and embedded NUL bytes.

</div>
</article>

<article class="doc-entry" id="v-csv-byte-record-start-line">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">csv-byte-record-start-line</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvByteRecord" title="Text.Csv.Record.CsvByteRecord">CsvByteRecord</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-csv-byte-record-start-line" title="Permalink">#</a><a href="/packages/stdlib/Text.Csv.Record/source/#L131" title="View source at line 131">Source</a></span></div>
<div class="doc-body">

Observe the starting physical line without decoding a field's bytes.

</div>
</article>

<article class="doc-entry" id="v-csv-byte-record-text">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">csv-byte-record-text</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvByteRecord" title="Text.Csv.Record.CsvByteRecord">CsvByteRecord</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvRecord" title="Text.Csv.Record.CsvRecord">CsvRecord</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvRecordError" title="Text.Csv.Record.CsvRecordError">CsvRecordError</a>)</code><span class="doc-sig-links"><a href="#v-csv-byte-record-text" title="Permalink">#</a><a href="/packages/stdlib/Text.Csv.Record/source/#L207" title="View source at line 207">Source</a></span></div>
<div class="doc-body">

Convert raw fields to immutable text, preserving the valid field prefix on an encoding error.
A failed conversion points at the offending field's original start and leaves the byte record unchanged.

</div>
</article>

<article class="doc-entry" id="v-csv-config-error-message">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">csv-config-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvConfigError" title="Text.Csv.Record.CsvConfigError">CsvConfigError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-csv-config-error-message" title="Permalink">#</a><a href="/packages/stdlib/Text.Csv.Record/source/#L62" title="View source at line 62">Source</a></span></div>
<div class="doc-body">

Render a configuration diagnostic without accessing input or output storage.

</div>
</article>

<article class="doc-entry" id="v-csv-default-options">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">csv-default-options</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvOptions" title="Text.Csv.Record.CsvOptions">CsvOptions</a></code><span class="doc-sig-links"><a href="#v-csv-default-options" title="Permalink">#</a><a href="/packages/stdlib/Text.Csv.Record/source/#L83" title="View source at line 83">Source</a></span></div>
<div class="doc-body">

Comma-separated records with inferred field count, strict quotes and no space trimming or comments.

</div>
</article>

<article class="doc-entry" id="v-csv-options">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">csv-options</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvFieldPolicy" title="Text.Csv.Record.CsvFieldPolicy">CsvFieldPolicy</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvConfigError" title="Text.Csv.Record.CsvConfigError">CsvConfigError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvOptions" title="Text.Csv.Record.CsvOptions">CsvOptions</a></code><span class="doc-sig-links"><a href="#v-csv-options" title="Permalink">#</a><a href="/packages/stdlib/Text.Csv.Record/source/#L73" title="View source at line 73">Source</a></span></div>
<div class="doc-body">

Validate delimiter/comment scalars and choose the row's field/quote policies.

</div>
</article>

<article class="doc-entry" id="v-csv-options-comment">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">csv-options-comment</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvOptions" title="Text.Csv.Record.CsvOptions">CsvOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a></code><span class="doc-sig-links"><a href="#v-csv-options-comment" title="Permalink">#</a><a href="/packages/stdlib/Text.Csv.Record/source/#L95" title="View source at line 95">Source</a></span></div>
<div class="doc-body">

Observe the optional comment marker, which is recognized only at physical column one.

</div>
</article>

<article class="doc-entry" id="v-csv-options-delimiter">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">csv-options-delimiter</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvOptions" title="Text.Csv.Record.CsvOptions">CsvOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a></code><span class="doc-sig-links"><a href="#v-csv-options-delimiter" title="Permalink">#</a><a href="/packages/stdlib/Text.Csv.Record/source/#L87" title="View source at line 87">Source</a></span></div>
<div class="doc-body">

Observe the validated delimiter scalar.

</div>
</article>

<article class="doc-entry" id="v-csv-options-field-policy">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">csv-options-field-policy</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvOptions" title="Text.Csv.Record.CsvOptions">CsvOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvFieldPolicy" title="Text.Csv.Record.CsvFieldPolicy">CsvFieldPolicy</a></code><span class="doc-sig-links"><a href="#v-csv-options-field-policy" title="Permalink">#</a><a href="/packages/stdlib/Text.Csv.Record/source/#L91" title="View source at line 91">Source</a></span></div>
<div class="doc-body">

Observe the current field-count rule, including an inferred count after reading.

</div>
</article>

<article class="doc-entry" id="v-csv-options-lazy-quotes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">csv-options-lazy-quotes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvOptions" title="Text.Csv.Record.CsvOptions">CsvOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-csv-options-lazy-quotes" title="Permalink">#</a><a href="/packages/stdlib/Text.Csv.Record/source/#L99" title="View source at line 99">Source</a></span></div>
<div class="doc-body">

Whether quotation marks inside unquoted fields or non-escaped quoted fields are accepted literally.

</div>
</article>

<article class="doc-entry" id="v-csv-options-trim-leading-space">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">csv-options-trim-leading-space</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvOptions" title="Text.Csv.Record.CsvOptions">CsvOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-csv-options-trim-leading-space" title="Permalink">#</a><a href="/packages/stdlib/Text.Csv.Record/source/#L103" title="View source at line 103">Source</a></span></div>
<div class="doc-body">

Whether leading Unicode White_Space is removed before deciding each field's quoted form.

</div>
</article>

<article class="doc-entry" id="v-csv-reader-configure">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">csv-reader-configure</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvReader" title="Text.Csv.Record.CsvReader">CsvReader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvOptions" title="Text.Csv.Record.CsvOptions">CsvOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvReader" title="Text.Csv.Record.CsvReader">CsvReader</a></code><span class="doc-sig-links"><a href="#v-csv-reader-configure" title="Permalink">#</a><a href="/packages/stdlib/Text.Csv.Record/source/#L303" title="View source at line 303">Source</a></span></div>
<div class="doc-body">

Select validated policies for the next record while preserving the input cursor and original positions.
Retaining the previous reader preserves its previous policies and continuation.

</div>
</article>

<article class="doc-entry" id="v-csv-reader-input-offset">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">csv-reader-input-offset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvReader" title="Text.Csv.Record.CsvReader">CsvReader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-csv-reader-input-offset" title="Permalink">#</a><a href="/packages/stdlib/Text.Csv.Record/source/#L290" title="View source at line 290">Source</a></span></div>
<div class="doc-body">

Original input byte offset immediately after the last consumed physical line.

</div>
</article>

<article class="doc-entry" id="v-csv-reader-next-line">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">csv-reader-next-line</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvReader" title="Text.Csv.Record.CsvReader">CsvReader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-csv-reader-next-line" title="Permalink">#</a><a href="/packages/stdlib/Text.Csv.Record/source/#L294" title="View source at line 294">Source</a></span></div>
<div class="doc-body">

Physical line number that the next read will begin with.

</div>
</article>

<article class="doc-entry" id="v-csv-reader-options">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">csv-reader-options</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvReader" title="Text.Csv.Record.CsvReader">CsvReader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvOptions" title="Text.Csv.Record.CsvOptions">CsvOptions</a></code><span class="doc-sig-links"><a href="#v-csv-reader-options" title="Permalink">#</a><a href="/packages/stdlib/Text.Csv.Record/source/#L298" title="View source at line 298">Source</a></span></div>
<div class="doc-body">

Observe the current options, including any field count learned from a previous row.

</div>
</article>

<article class="doc-entry" id="v-csv-reader-read">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">csv-reader-read</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvReader" title="Text.Csv.Record.CsvReader">CsvReader</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvRead" title="Text.Csv.Record.CsvRead">CsvRead</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvReader" title="Text.Csv.Record.CsvReader">CsvReader</a>)</code><span class="doc-sig-links"><a href="#v-csv-reader-read" title="Permalink">#</a><a href="/packages/stdlib/Text.Csv.Record/source/#L622" title="View source at line 622">Source</a></span></div>
<div class="doc-body">

Read the next validated text record with completed-field progress and an advanced reader.
Input originated as immutable UTF-8 text; byte fields are converted after common CSV parsing.

</div>
</article>

<article class="doc-entry" id="v-csv-reader-read-all">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">csv-reader-read-all</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvReader" title="Text.Csv.Record.CsvReader">CsvReader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvRecordError" title="Text.Csv.Record.CsvRecordError">CsvRecordError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvRecord" title="Text.Csv.Record.CsvRecord">CsvRecord</a>)</code><span class="doc-sig-links"><a href="#v-csv-reader-read-all" title="Permalink">#</a><a href="/packages/stdlib/Text.Csv.Record/source/#L650" title="View source at line 650">Source</a></span></div>
<div class="doc-body">

Read the remaining records atomically, returning the first parse error without a partial document.
An open unfinished prefix returns the same unfinished-quote error vocabulary at its next line.

</div>
</article>

<article class="doc-entry" id="v-csv-record-error-column">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">csv-record-error-column</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvRecordError" title="Text.Csv.Record.CsvRecordError">CsvRecordError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-csv-record-error-column" title="Permalink">#</a><a href="/packages/stdlib/Text.Csv.Record/source/#L188" title="View source at line 188">Source</a></span></div>
<div class="doc-body">

One-based original byte column containing the error.

</div>
</article>

<article class="doc-entry" id="v-csv-record-error-line">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">csv-record-error-line</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvRecordError" title="Text.Csv.Record.CsvRecordError">CsvRecordError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-csv-record-error-line" title="Permalink">#</a><a href="/packages/stdlib/Text.Csv.Record/source/#L179" title="View source at line 179">Source</a></span></div>
<div class="doc-body">

Physical line containing the error, which can differ from its row's starting line.

</div>
</article>

<article class="doc-entry" id="v-csv-record-error-message">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">csv-record-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvRecordError" title="Text.Csv.Record.CsvRecordError">CsvRecordError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-csv-record-error-message" title="Permalink">#</a><a href="/packages/stdlib/Text.Csv.Record/source/#L197" title="View source at line 197">Source</a></span></div>
<div class="doc-body">

Render a stable semantic diagnostic; structured accessors retain the full source position.

</div>
</article>

<article class="doc-entry" id="v-csv-record-error-start-line">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">csv-record-error-start-line</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvRecordError" title="Text.Csv.Record.CsvRecordError">CsvRecordError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-csv-record-error-start-line" title="Permalink">#</a><a href="/packages/stdlib/Text.Csv.Record/source/#L170" title="View source at line 170">Source</a></span></div>
<div class="doc-body">

Starting physical line of the row associated with an error.

</div>
</article>

<article class="doc-entry" id="v-csv-record-field-position">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">csv-record-field-position</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvRecord" title="Text.Csv.Record.CsvRecord">CsvRecord</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>)</code><span class="doc-sig-links"><a href="#v-csv-record-field-position" title="Permalink">#</a><a href="/packages/stdlib/Text.Csv.Record/source/#L147" title="View source at line 147">Source</a></span></div>
<div class="doc-body">

Safely observe a field's one-based physical line and byte column by zero-based field index.

</div>
</article>

<article class="doc-entry" id="v-csv-record-fields">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">csv-record-fields</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvRecord" title="Text.Csv.Record.CsvRecord">CsvRecord</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-csv-record-fields" title="Permalink">#</a><a href="/packages/stdlib/Text.Csv.Record/source/#L139" title="View source at line 139">Source</a></span></div>
<div class="doc-body">

Observe all fields, including completed fields returned with a later parse error.

</div>
</article>

<article class="doc-entry" id="v-csv-record-reader">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">csv-record-reader</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvOptions" title="Text.Csv.Record.CsvOptions">CsvOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvReader" title="Text.Csv.Record.CsvReader">CsvReader</a></code><span class="doc-sig-links"><a href="#v-csv-record-reader" title="Permalink">#</a><a href="/packages/stdlib/Text.Csv.Record/source/#L280" title="View source at line 280">Source</a></span></div>
<div class="doc-body">

Begin reading a complete immutable UTF-8 document at byte zero and physical line one.

</div>
</article>

<article class="doc-entry" id="v-csv-record-reader-at">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">csv-record-reader-at</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvOptions" title="Text.Csv.Record.CsvOptions">CsvOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvReader" title="Text.Csv.Record.CsvReader">CsvReader</a></code><span class="doc-sig-links"><a href="#v-csv-record-reader-at" title="Permalink">#</a><a href="/packages/stdlib/Text.Csv.Record/source/#L286" title="View source at line 286">Source</a></span></div>
<div class="doc-body">

Read a UTF-8 prefix with an original byte origin and first physical line.
A false closed flag returns CsvNeedInput when the last row may continue.
This permits stream adapters to retain one unfinished row rather than a whole source.

</div>
</article>

<article class="doc-entry" id="v-csv-record-start-line">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">csv-record-start-line</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvRecord" title="Text.Csv.Record.CsvRecord">CsvRecord</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-csv-record-start-line" title="Permalink">#</a><a href="/packages/stdlib/Text.Csv.Record/source/#L143" title="View source at line 143">Source</a></span></div>
<div class="doc-body">

The one-based physical line where the row began, before any multiline quoted field.

</div>
</article>

<article class="doc-entry" id="v-csv-render-byte-record">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">csv-render-byte-record</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvOptions" title="Text.Csv.Record.CsvOptions">CsvOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-csv-render-byte-record" title="Permalink">#</a><a href="/packages/stdlib/Text.Csv.Record/source/#L798" title="View source at line 798">Source</a></span></div>
<div class="doc-body">

Render arbitrary byte fields with minimal quoting and an explicit LF or CRLF terminator.
Invalid UTF-8 and embedded NUL bytes are preserved; quoted CR is omitted and LF becomes CRLF when selected.

</div>
</article>

<article class="doc-entry" id="v-csv-render-byte-records">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">csv-render-byte-records</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvOptions" title="Text.Csv.Record.CsvOptions">CsvOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-csv-render-byte-records" title="Permalink">#</a><a href="/packages/stdlib/Text.Csv.Record/source/#L813" title="View source at line 813">Source</a></span></div>
<div class="doc-body">

Render multiple arbitrary-byte records in order without requiring UTF-8 text conversion.

</div>
</article>

<article class="doc-entry" id="v-csv-render-record">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">csv-render-record</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvOptions" title="Text.Csv.Record.CsvOptions">CsvOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-csv-render-record" title="Permalink">#</a><a href="/packages/stdlib/Text.Csv.Record/source/#L825" title="View source at line 825">Source</a></span></div>
<div class="doc-body">

Render one immutable text record with the same minimal quoting and newline policy as byte records.

</div>
</article>

<article class="doc-entry" id="v-csv-render-records">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">csv-render-records</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Csv.Record/#t-CsvOptions" title="Text.Csv.Record.CsvOptions">CsvOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-csv-render-records" title="Permalink">#</a><a href="/packages/stdlib/Text.Csv.Record/source/#L837" title="View source at line 837">Source</a></span></div>
<div class="doc-body">

Render text records in order with a consistent delimiter and LF/CRLF policy.

</div>
</article>
