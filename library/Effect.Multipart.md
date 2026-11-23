# Effect.Multipart

Explicit shared multipart sessions with borrowed byte I/O capabilities.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Effect.Multipart`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Effect.Multipart.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Mutable.Var](Data.Mutable.Var.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Ordering](Data.Ordering.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Effect.Stream](Effect.Stream.md), [Text.Mime](Text.Mime.md), [Text.Multipart](Text.Multipart.md), [Text.QuotedPrintable](Text.QuotedPrintable.md).

Non-embedded source closure: [Data.Bytes.Utf8](Data.Bytes.Utf8.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Mutable.Var](Data.Mutable.Var.md), [Data.Unicode.Kernel](Data.Unicode.Kernel.md), [Data.Unicode.Tables](Data.Unicode.Tables.md), [Effect.Multipart](Effect.Multipart.md), [Effect.Stream](Effect.Stream.md), [Text.Base64.Encoding](Text.Base64.Encoding.md), [Text.Mime](Text.Mime.md), [Text.Multipart](Text.Multipart.md), [Text.QuotedPrintable](Text.QuotedPrintable.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Explicit shared multipart sessions with borrowed byte I/O capabilities.
Callbacks preserve confirmed prefixes alongside arbitrary caller failures.
Session and part cells form an acyclic ownership graph; retained aliases see
progress, metadata updates and part completion through the same named effects.

</details>

# Types and constructors

<a id="type-4d756c746970617274436c65616e7570"></a>

## `MultipartCleanup`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">MultipartCleanup</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>)</code></pre>

A failed cleanup retains both its typed error and exact retry capability.

[Source](/reference-sources/Effect.Multipart.cinch) · line 1242

<a id="type-4d756c7469706172744572726f72"></a>

## `MultipartError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">MultipartError</span> (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>)</code></pre>

Protocol errors remain distinct from arbitrary caller transport values.

[Source](/reference-sources/Effect.Multipart.cinch) · line 53

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MultipartProtocolFailure</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">e</span> . (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartSyntax</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartError</span> <span class="tok-variable">e</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MultipartCallerFailure</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">e</span> . (<span class="tok-variable">e</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartError</span> <span class="tok-variable">e</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MultipartInvalidCount</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">e</span> . (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartError</span> <span class="tok-variable">e</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MultipartShortWrite</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">e</span> . (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartError</span> <span class="tok-variable">e</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MultipartAfterWrite</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">e</span> . <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartError</span> <span class="tok-variable">e</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MultipartFinishedPart</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">e</span> . <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartError</span> <span class="tok-variable">e</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MultipartNextFailure</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">e</span> . (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartError</span> <span class="tok-variable">e</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartError</span> <span class="tok-variable">e</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MultipartQuotedPrintableFailure</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">e</span> . (<span class="tok-constructor">Text</span>.<span class="tok-constructor">QuotedPrintable</span>.<span class="tok-constructor">QuotedPrintableError</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartError</span> <span class="tok-variable">e</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MultipartSeekWhence</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">e</span> . <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartError</span> <span class="tok-variable">e</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MultipartSeekOffset</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">e</span> . <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartError</span> <span class="tok-variable">e</span></code></pre>

<a id="type-4d756c74697061727446696c65"></a>

## `MultipartFile`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">MultipartFile</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">key</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">fileHandle</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">state</span> :: <span class="tok-constructor">Type</span>)</code></pre>

An open memory, whole-disk or section-disk handle with complete file capabilities.

[Source](/reference-sources/Effect.Multipart.cinch) · line 976

<a id="type-4d756c74697061727446696c65486561646572"></a>

## `MultipartFileHeader`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">MultipartFileHeader</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">key</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">fileHandle</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">state</span> :: <span class="tok-constructor">Type</span>)</code></pre>

A stable shared file-header handle that may occur in several form lists.

[Source](/reference-sources/Effect.Multipart.cinch) · line 971

<a id="type-4d756c746970617274466f726d"></a>

## `MultipartForm`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">MultipartForm</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">key</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">fileHandle</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">state</span> :: <span class="tok-constructor">Type</span>)</code></pre>

A mutable form context with full value/file map observation and replacement.

[Source](/reference-sources/Effect.Multipart.cinch) · line 1183

<a id="type-4d756c746970617274466f726d526561644f7574636f6d65"></a>

## `MultipartFormReadOutcome`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">MultipartFormReadOutcome</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">key</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">fileHandle</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">state</span> :: <span class="tok-constructor">Type</span>)</code></pre>

Form and error are independent because deferred close failure can preserve a form.

[Source](/reference-sources/Effect.Multipart.cinch) · line 1313

<a id="type-4d756c7469706172744f757470757450617274"></a>

## `MultipartOutputPart`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">MultipartOutputPart</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>)</code></pre>

A borrowed output handle retaining its writer and independent body state.

[Source](/reference-sources/Effect.Multipart.cinch) · line 159

<a id="type-4d756c74697061727450617274"></a>

## `MultipartPart`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">MultipartPart</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>)</code></pre>

A retained part alias shares its reader and its independent metadata/body cell.

[Source](/reference-sources/Effect.Multipart.cinch) · line 590

<a id="type-4d756c746970617274506f6c696379"></a>

## `MultipartPolicy`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">MultipartPolicy</span></code></pre>

Explicit header/part limits, scratch sharing and filename path policy.

[Source](/reference-sources/Effect.Multipart.cinch) · line 552

<a id="type-4d756c74697061727452656164537461747573"></a>

## `MultipartReadStatus`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">MultipartReadStatus</span> (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>)</code></pre>

Independent status accompanying a possibly positive byte prefix.

[Source](/reference-sources/Effect.Multipart.cinch) · line 76

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MultipartMore</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">e</span> . <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartReadStatus</span> <span class="tok-variable">e</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MultipartEnd</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">e</span> . <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartReadStatus</span> <span class="tok-variable">e</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MultipartReadFailure</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">e</span> . (<span class="tok-variable">e</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartReadStatus</span> <span class="tok-variable">e</span>)</code></pre>

<a id="type-4d756c746970617274526561646572"></a>

## `MultipartReader`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">MultipartReader</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>)</code></pre>

A borrowed input session with explicitly shared mutation and I/O effects.

[Source](/reference-sources/Effect.Multipart.cinch) · line 585

<a id="type-4d756c74697061727453696e6b"></a>

## `MultipartSink`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">MultipartSink</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>)</code></pre>

A borrowed sink with independent confirmed progress and optional failure.

[Source](/reference-sources/Effect.Multipart.cinch) · line 98

<a id="type-4d756c746970617274536f75726365"></a>

## `MultipartSource`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">MultipartSource</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>)</code></pre>

A borrowed source callback returning its explicit advanced continuation.

[Source](/reference-sources/Effect.Multipart.cinch) · line 85

<a id="type-4d756c74697061727453746f72616765"></a>

## `MultipartStorage`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">MultipartStorage</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">key</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">fileHandle</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">state</span> :: <span class="tok-constructor">Type</span>)</code></pre>

Shared explicit scratch storage, borrowing no hidden global filesystem state.

[Source](/reference-sources/Effect.Multipart.cinch) · line 877

<a id="type-4d756c746970617274577269746572"></a>

## `MultipartWriter`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">MultipartWriter</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>)</code></pre>

A shared writer session; operations state mutation and I/O explicitly.

[Source](/reference-sources/Effect.Multipart.cinch) · line 154

# Values

<a id="value-6d756c7469706172742d636c65616e75702d6572726f72"></a>

## `multipart-cleanup-error`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">multipart-cleanup-error</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartCleanup</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartError</span> <span class="tok-variable">e</span>))</code></pre>

Observe the original cleanup error without discarding pending ownership.

[Source](/reference-sources/Effect.Multipart.cinch) · line 1247

<a id="value-6d756c7469706172742d636c65616e75702d7265747279"></a>

## `multipart-cleanup-retry`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">multipart-cleanup-retry</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartCleanup</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span>) -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartError</span> <span class="tok-variable">e</span>) <span class="tok-constructor">Unit</span>))</code></pre>

Retry exactly the retained close/unlink action; failed aliases remain retryable.

[Source](/reference-sources/Effect.Multipart.cinch) · line 1251

<a id="value-6d756c7469706172742d6372656174652d6669656c64"></a>

## `multipart-create-field`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">multipart-create-field</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartWriter</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span>) -&gt; (<span class="tok-constructor">Bytes</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartError</span> <span class="tok-variable">e</span>) (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartOutputPart</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span>))))</code></pre>

Create a named field with the exact byte disposition escaping policy.

[Source](/reference-sources/Effect.Multipart.cinch) · line 278

<a id="value-6d756c7469706172742d6372656174652d66696c65"></a>

## `multipart-create-file`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">multipart-create-file</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartWriter</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span>) -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartError</span> <span class="tok-variable">e</span>) (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartOutputPart</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span>)))))</code></pre>

Create a named file field with an octet-stream content type.

[Source](/reference-sources/Effect.Multipart.cinch) · line 283

<a id="value-6d756c7469706172742d6372656174652d70617274"></a>

## `multipart-create-part`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">multipart-create-part</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartWriter</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span>) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartHeaders</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartError</span> <span class="tok-variable">e</span>) (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartOutputPart</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span>))))</code></pre>

Finish an old body before header output; failed replacement keeps it assigned.

[Source](/reference-sources/Effect.Multipart.cinch) · line 264

<a id="value-6d756c7469706172742d64656661756c742d706f6c696379"></a>

## `multipart-default-policy`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">multipart-default-policy</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartPolicy</span></code></pre>

Conventional ten-MiB headers, ten-thousand entries, thousand parts and POSIX names.

[Source](/reference-sources/Effect.Multipart.cinch) · line 561

<a id="value-6d756c7469706172742d66696c652d636c6f7365"></a>

## `multipart-file-close`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">multipart-file-close</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">key</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">fileHandle</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">state</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartFile</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span> <span class="tok-variable">key</span> <span class="tok-variable">fileHandle</span> <span class="tok-variable">state</span>) -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartError</span> <span class="tok-variable">e</span>) <span class="tok-constructor">Unit</span>))</code></pre>

Close backend handles while memory handles deliberately remain usable.

[Source](/reference-sources/Effect.Multipart.cinch) · line 1128

<a id="value-6d756c7469706172742d66696c652d66696c656e616d65"></a>

## `multipart-file-filename`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">multipart-file-filename</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">key</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">fileHandle</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">state</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartFileHeader</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span> <span class="tok-variable">key</span> <span class="tok-variable">fileHandle</span> <span class="tok-variable">state</span>) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Bytes</span>)</code></pre>

Observe an independently mutable filename snapshot.

[Source](/reference-sources/Effect.Multipart.cinch) · line 991

<a id="value-6d756c7469706172742d66696c652d68656164657273"></a>

## `multipart-file-headers`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">multipart-file-headers</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">key</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">fileHandle</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">state</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartFileHeader</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span> <span class="tok-variable">key</span> <span class="tok-variable">fileHandle</span> <span class="tok-variable">state</span>) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Text</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartHeaders</span>)</code></pre>

Observe full immutable header context with fresh public byte views.

[Source](/reference-sources/Effect.Multipart.cinch) · line 1000

<a id="value-6d756c7469706172742d66696c652d6d656d6f7279"></a>

## `multipart-file-memory`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">multipart-file-memory</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">key</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">fileHandle</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">state</span> :: <span class="tok-constructor">Type</span>). (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartHeaders</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartFileHeader</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span> <span class="tok-variable">key</span> <span class="tok-variable">fileHandle</span> <span class="tok-variable">state</span>))))</code></pre>

Construct an owned memory file header without hidden scratch resources.

[Source](/reference-sources/Effect.Multipart.cinch) · line 985

<a id="value-6d756c7469706172742d66696c652d6f70656e"></a>

## `multipart-file-open`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">multipart-file-open</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">key</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">fileHandle</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">state</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartFileHeader</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span> <span class="tok-variable">key</span> <span class="tok-variable">fileHandle</span> <span class="tok-variable">state</span>) -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartError</span> <span class="tok-variable">e</span>) (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartFile</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span> <span class="tok-variable">key</span> <span class="tok-variable">fileHandle</span> <span class="tok-variable">state</span>)))</code></pre>

Open a fresh independent cursor, applying edited size only to shared disk sections.

[Source](/reference-sources/Effect.Multipart.cinch) · line 1022

<a id="value-6d756c7469706172742d66696c652d72656164"></a>

## `multipart-file-read`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">multipart-file-read</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">key</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">fileHandle</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">state</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartFile</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span> <span class="tok-variable">key</span> <span class="tok-variable">fileHandle</span> <span class="tok-variable">state</span>) -&gt; (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Bytes</span>, (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartReadStatus</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartError</span> <span class="tok-variable">e</span>)))))</code></pre>

Read at most the requested prefix while retaining backing-specific cursor behavior.

[Source](/reference-sources/Effect.Multipart.cinch) · line 1068

<a id="value-6d756c7469706172742d66696c652d726561642d6174"></a>

## `multipart-file-read-at`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">multipart-file-read-at</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">key</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">fileHandle</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">state</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartFile</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span> <span class="tok-variable">key</span> <span class="tok-variable">fileHandle</span> <span class="tok-variable">state</span>) -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">I64</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Bytes</span>, (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartReadStatus</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartError</span> <span class="tok-variable">e</span>))))))</code></pre>

Read at a signed relative position without changing the sequential cursor.

[Source](/reference-sources/Effect.Multipart.cinch) · line 1092

<a id="value-6d756c7469706172742d66696c652d72656d6f7665"></a>

## `multipart-file-remove`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">multipart-file-remove</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">key</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">fileHandle</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">state</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartFileHeader</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span> <span class="tok-variable">key</span> <span class="tok-variable">fileHandle</span> <span class="tok-variable">state</span>) -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartError</span> <span class="tok-variable">e</span>) <span class="tok-constructor">Unit</span>))</code></pre>

Explicitly unlink a retained header's backing, including detached form entries.
A combined scratch key may back several headers; unlink affects every reopen
of that key and deliberately leaves already-open handles to their owners.

[Source](/reference-sources/Effect.Multipart.cinch) · line 1305

<a id="value-6d756c7469706172742d66696c652d7365656b"></a>

## `multipart-file-seek`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">multipart-file-seek</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">key</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">fileHandle</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">state</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartFile</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span> <span class="tok-variable">key</span> <span class="tok-variable">fileHandle</span> <span class="tok-variable">state</span>) -&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartError</span> <span class="tok-variable">e</span>) <span class="tok-constructor">I64</span>))))</code></pre>

Seek through the complete file capability, keeping native backend errors typed.

[Source](/reference-sources/Effect.Multipart.cinch) · line 1119

<a id="value-6d756c7469706172742d66696c652d7365742d66696c656e616d65"></a>

## `multipart-file-set-filename`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">multipart-file-set-filename</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">key</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">fileHandle</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">state</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartFileHeader</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span> <span class="tok-variable">key</span> <span class="tok-variable">fileHandle</span> <span class="tok-variable">state</span>) -&gt; (<span class="tok-constructor">Bytes</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>))</code></pre>

Replace filename metadata without renaming or reopening its backing object.

[Source](/reference-sources/Effect.Multipart.cinch) · line 995

<a id="value-6d756c7469706172742d66696c652d7365742d68656164657273"></a>

## `multipart-file-set-headers`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">multipart-file-set-headers</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">key</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">fileHandle</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">state</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartFileHeader</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span> <span class="tok-variable">key</span> <span class="tok-variable">fileHandle</span> <span class="tok-variable">state</span>) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartHeaders</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>))</code></pre>

Replace every header entry without changing the associated resource.

[Source](/reference-sources/Effect.Multipart.cinch) · line 1004

<a id="value-6d756c7469706172742d66696c652d7365742d73697a65"></a>

## `multipart-file-set-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">multipart-file-set-size</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">key</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">fileHandle</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">state</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartFileHeader</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span> <span class="tok-variable">key</span> <span class="tok-variable">fileHandle</span> <span class="tok-variable">state</span>) -&gt; (<span class="tok-constructor">I64</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>))</code></pre>

Replace signed size; shared disk sections consult this value on the next open.

[Source](/reference-sources/Effect.Multipart.cinch) · line 1013

<a id="value-6d756c7469706172742d66696c652d73697a65"></a>

## `multipart-file-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">multipart-file-size</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">key</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">fileHandle</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">state</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartFileHeader</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span> <span class="tok-variable">key</span> <span class="tok-variable">fileHandle</span> <span class="tok-variable">state</span>) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">I64</span>)</code></pre>

Observe signed public size independently of actual memory content length.

[Source](/reference-sources/Effect.Multipart.cinch) · line 1009

<a id="value-6d756c7469706172742d666f726d2d617070656e642d66696c65"></a>

## `multipart-form-append-file`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">multipart-form-append-file</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">key</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">fileHandle</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">state</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartForm</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span> <span class="tok-variable">key</span> <span class="tok-variable">fileHandle</span> <span class="tok-variable">state</span>) -&gt; (<span class="tok-constructor">Bytes</span> -&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartFileHeader</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span> <span class="tok-variable">key</span> <span class="tok-variable">fileHandle</span> <span class="tok-variable">state</span>) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>)))</code></pre>

Append a stable header handle without implicitly closing or unlinking it.

[Source](/reference-sources/Effect.Multipart.cinch) · line 1232

<a id="value-6d756c7469706172742d666f726d2d617070656e642d76616c7565"></a>

## `multipart-form-append-value`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">multipart-form-append-value</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">key</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">fileHandle</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">state</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartForm</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span> <span class="tok-variable">key</span> <span class="tok-variable">fileHandle</span> <span class="tok-variable">state</span>) -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>)))</code></pre>

Append a privately owned value while retaining original field occurrence order.

[Source](/reference-sources/Effect.Multipart.cinch) · line 1207

<a id="value-6d756c7469706172742d666f726d2d64726f702d66696c6573"></a>

## `multipart-form-drop-files`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">multipart-form-drop-files</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">key</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">fileHandle</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">state</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartForm</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span> <span class="tok-variable">key</span> <span class="tok-variable">fileHandle</span> <span class="tok-variable">state</span>) -&gt; (<span class="tok-constructor">Bytes</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>))</code></pre>

Detach one file list; detached retained headers keep their backing identity.

[Source](/reference-sources/Effect.Multipart.cinch) · line 1237

<a id="value-6d756c7469706172742d666f726d2d64726f702d76616c756573"></a>

## `multipart-form-drop-values`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">multipart-form-drop-values</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">key</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">fileHandle</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">state</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartForm</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span> <span class="tok-variable">key</span> <span class="tok-variable">fileHandle</span> <span class="tok-variable">state</span>) -&gt; (<span class="tok-constructor">Bytes</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>))</code></pre>

Drop a value field without altering any associated file-header lists.

[Source](/reference-sources/Effect.Multipart.cinch) · line 1212

<a id="value-6d756c7469706172742d666f726d2d656d707479"></a>

## `multipart-form-empty`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">multipart-form-empty</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">key</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">fileHandle</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">state</span> :: <span class="tok-constructor">Type</span>). (<span class="tok-constructor">Unit</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartForm</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span> <span class="tok-variable">key</span> <span class="tok-variable">fileHandle</span> <span class="tok-variable">state</span>))</code></pre>

Construct an empty form whose later file backings use explicit capability types.

[Source](/reference-sources/Effect.Multipart.cinch) · line 1188

<a id="value-6d756c7469706172742d666f726d2d66696c6573"></a>

## `multipart-form-files`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">multipart-form-files</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">key</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">fileHandle</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">state</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartForm</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span> <span class="tok-variable">key</span> <span class="tok-variable">fileHandle</span> <span class="tok-variable">state</span>) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-constructor">Bytes</span>, (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartFileHeader</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span> <span class="tok-variable">key</span> <span class="tok-variable">fileHandle</span> <span class="tok-variable">state</span>)))))</code></pre>

Enumerate exact sorted file-field keys while preserving shared header identity.

[Source](/reference-sources/Effect.Multipart.cinch) · line 1217

<a id="value-6d756c7469706172742d666f726d2d66696c65732d6174"></a>

## `multipart-form-files-at`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">multipart-form-files-at</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">key</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">fileHandle</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">state</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartForm</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span> <span class="tok-variable">key</span> <span class="tok-variable">fileHandle</span> <span class="tok-variable">state</span>) -&gt; (<span class="tok-constructor">Bytes</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartFileHeader</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span> <span class="tok-variable">key</span> <span class="tok-variable">fileHandle</span> <span class="tok-variable">state</span>))))</code></pre>

Read one ordered shared header list without copying its metadata identities.

[Source](/reference-sources/Effect.Multipart.cinch) · line 1222

<a id="value-6d756c7469706172742d666f726d2d726561642d636c65616e7570"></a>

## `multipart-form-read-cleanup`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">multipart-form-read-cleanup</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">key</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">fileHandle</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">state</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartFormReadOutcome</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span> <span class="tok-variable">key</span> <span class="tok-variable">fileHandle</span> <span class="tok-variable">state</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartCleanup</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span>)))</code></pre>

Retain every failed close/unlink action needed for an explicit ownership retry.

[Source](/reference-sources/Effect.Multipart.cinch) · line 1326

<a id="value-6d756c7469706172742d666f726d2d726561642d6572726f72"></a>

## `multipart-form-read-error`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">multipart-form-read-error</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">key</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">fileHandle</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">state</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartFormReadOutcome</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span> <span class="tok-variable">key</span> <span class="tok-variable">fileHandle</span> <span class="tok-variable">state</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartError</span> <span class="tok-variable">e</span>)))</code></pre>

Observe the original primary failure independently of the optional form.

[Source](/reference-sources/Effect.Multipart.cinch) · line 1322

<a id="value-6d756c7469706172742d666f726d2d726561642d666f726d"></a>

## `multipart-form-read-form`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">multipart-form-read-form</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">key</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">fileHandle</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">state</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartFormReadOutcome</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span> <span class="tok-variable">key</span> <span class="tok-variable">fileHandle</span> <span class="tok-variable">state</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartForm</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span> <span class="tok-variable">key</span> <span class="tok-variable">fileHandle</span> <span class="tok-variable">state</span>)))</code></pre>

Observe a returned form even when a deferred final close also failed.

[Source](/reference-sources/Effect.Multipart.cinch) · line 1318

<a id="value-6d756c7469706172742d666f726d2d72656d6f76652d616c6c"></a>

## `multipart-form-remove-all`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">multipart-form-remove-all</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">key</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">fileHandle</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">state</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartForm</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span> <span class="tok-variable">key</span> <span class="tok-variable">fileHandle</span> <span class="tok-variable">state</span>) -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartError</span> <span class="tok-variable">e</span>) <span class="tok-constructor">Unit</span>))</code></pre>

Attempt temporary-file removal for all headers currently reachable in Form.File.
Missing objects are ignored; later cleanup still runs after a failure. The
first encountered nonmissing failure is returned in explicit insertion order.

[Source](/reference-sources/Effect.Multipart.cinch) · line 1293

<a id="value-6d756c7469706172742d666f726d2d7365742d66696c6573"></a>

## `multipart-form-set-files`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">multipart-form-set-files</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">key</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">fileHandle</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">state</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartForm</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span> <span class="tok-variable">key</span> <span class="tok-variable">fileHandle</span> <span class="tok-variable">state</span>) -&gt; (<span class="tok-constructor">Bytes</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartFileHeader</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span> <span class="tok-variable">key</span> <span class="tok-variable">fileHandle</span> <span class="tok-variable">state</span>)) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>)))</code></pre>

Replace one whole header list, allowing retained, duplicated or moved headers.

[Source](/reference-sources/Effect.Multipart.cinch) · line 1227

<a id="value-6d756c7469706172742d666f726d2d7365742d76616c756573"></a>

## `multipart-form-set-values`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">multipart-form-set-values</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">key</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">fileHandle</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">state</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartForm</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span> <span class="tok-variable">key</span> <span class="tok-variable">fileHandle</span> <span class="tok-variable">state</span>) -&gt; (<span class="tok-constructor">Bytes</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Bytes</span>) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>)))</code></pre>

Replace every value under one exact field name, including with an empty list.

[Source](/reference-sources/Effect.Multipart.cinch) · line 1202

<a id="value-6d756c7469706172742d666f726d2d76616c756573"></a>

## `multipart-form-values`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">multipart-form-values</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">key</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">fileHandle</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">state</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartForm</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span> <span class="tok-variable">key</span> <span class="tok-variable">fileHandle</span> <span class="tok-variable">state</span>) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-constructor">Bytes</span>, (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Bytes</span>))))</code></pre>

Enumerate sorted field keys and fresh ordered raw value snapshots.

[Source](/reference-sources/Effect.Multipart.cinch) · line 1192

<a id="value-6d756c7469706172742d666f726d2d76616c7565732d6174"></a>

## `multipart-form-values-at`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">multipart-form-values-at</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">key</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">fileHandle</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">state</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartForm</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span> <span class="tok-variable">key</span> <span class="tok-variable">fileHandle</span> <span class="tok-variable">state</span>) -&gt; (<span class="tok-constructor">Bytes</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Bytes</span>)))</code></pre>

Read one field's ordered value snapshots without changing absent/empty map state.

[Source](/reference-sources/Effect.Multipart.cinch) · line 1197

<a id="value-6d756c7469706172742d6e6578742d70617274"></a>

## `multipart-next-part`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">multipart-next-part</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartReader</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span>) -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartError</span> <span class="tok-variable">e</span>) (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartPart</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span>))))</code></pre>

Advance to another part, transparently decoding selected quoted-printable bodies.

[Source](/reference-sources/Effect.Multipart.cinch) · line 851

<a id="value-6d756c7469706172742d6e6578742d7261772d70617274"></a>

## `multipart-next-raw-part`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">multipart-next-raw-part</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartReader</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span>) -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartError</span> <span class="tok-variable">e</span>) (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartPart</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span>))))</code></pre>

Advance without hiding transfer headers or decoding raw body bytes.

[Source](/reference-sources/Effect.Multipart.cinch) · line 857

<a id="value-6d756c7469706172742d6f75747075742d7772697465"></a>

## `multipart-output-write`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">multipart-output-write</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartOutputPart</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span>) -&gt; (<span class="tok-constructor">Bytes</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Nat</span>, (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartError</span> <span class="tok-variable">e</span>)))))</code></pre>

Write once through an active body, retaining the latest nonempty error.

[Source](/reference-sources/Effect.Multipart.cinch) · line 289

<a id="value-6d756c7469706172742d706172742d636c6f7365"></a>

## `multipart-part-close`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">multipart-part-close</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartPart</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span>) -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>)</code></pre>

Drain a part through its selected raw/decoded reader, ignoring the drain error.

[Source](/reference-sources/Effect.Multipart.cinch) · line 791

<a id="value-6d756c7469706172742d706172742d66696c656e616d65"></a>

## `multipart-part-filename`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">multipart-part-filename</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartPart</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span>) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Bytes</span>)</code></pre>

Observe a filename through the same cache and the explicit basename policy.

[Source](/reference-sources/Effect.Multipart.cinch) · line 645

<a id="value-6d756c7469706172742d706172742d666f726d2d6e616d65"></a>

## `multipart-part-form-name`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">multipart-part-form-name</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartPart</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span>) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Bytes</span>)</code></pre>

Observe a form-data name through the shared lazy disposition cache.

[Source](/reference-sources/Effect.Multipart.cinch) · line 639

<a id="value-6d756c7469706172742d706172742d68656164657273"></a>

## `multipart-part-headers`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">multipart-part-headers</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartPart</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span>) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Text</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartHeaders</span>)</code></pre>

Observe immutable header context; byte views obtained from it are fresh snapshots.

[Source](/reference-sources/Effect.Multipart.cinch) · line 612

<a id="value-6d756c7469706172742d706172742d72656164"></a>

## `multipart-part-read`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">multipart-part-read</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartPart</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span>) -&gt; (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Bytes</span>, (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartReadStatus</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartError</span> <span class="tok-variable">e</span>)))))</code></pre>

Read a possibly positive prefix with simultaneous terminal status.
Raw zero reads can prefetch framing; decoded zero reads are inert.

[Source](/reference-sources/Effect.Multipart.cinch) · line 781

<a id="value-6d756c7469706172742d706172742d7365742d68656164657273"></a>

## `multipart-part-set-headers`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">multipart-part-set-headers</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartPart</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span>) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartHeaders</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>))</code></pre>

Replace live headers without invalidating an already initialized disposition cache.

[Source](/reference-sources/Effect.Multipart.cinch) · line 617

<a id="value-6d756c7469706172742d706f6c696379"></a>

## `multipart-policy`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">multipart-policy</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; ((<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Bytes</span>) -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartPolicy</span>)))))</code></pre>

Select every resource limit and filename policy without ambient configuration.

[Source](/reference-sources/Effect.Multipart.cinch) · line 557

<a id="value-6d756c7469706172742d726561642d666f726d"></a>

## `multipart-read-form`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">multipart-read-form</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">key</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">fileHandle</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">state</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartReader</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span>) -&gt; (<span class="tok-constructor">I64</span> -&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartStorage</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span> <span class="tok-variable">key</span> <span class="tok-variable">fileHandle</span> <span class="tok-variable">state</span>) -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartFormReadOutcome</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span> <span class="tok-variable">key</span> <span class="tok-variable">fileHandle</span> <span class="tok-variable">state</span>))))</code></pre>

Parse a complete form with real spill storage and explicit resource-limit policy.
The returned form and error are independently observable: final scratch close
can fail after a form has been produced. Earlier failures return no form.
Cleanup always preserves the primary failure and retains retry capabilities.

[Source](/reference-sources/Effect.Multipart.cinch) · line 1565

<a id="value-6d756c7469706172742d726561646572"></a>

## `multipart-reader`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">multipart-reader</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartSource</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span>) -&gt; (<span class="tok-constructor">Bytes</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartReader</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span>)))</code></pre>

Begin a borrowed reader with explicit default limits and POSIX filename policy.

[Source](/reference-sources/Effect.Multipart.cinch) · line 602

<a id="value-6d756c7469706172742d7265616465722d736f75726365"></a>

## `multipart-reader-source`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">multipart-reader-source</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartReader</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span>) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartSource</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span>))</code></pre>

Observe the current advanced borrowed source, including after protocol failures.

[Source](/reference-sources/Effect.Multipart.cinch) · line 606

<a id="value-6d756c7469706172742d7265616465722d776974682d706f6c696379"></a>

## `multipart-reader-with-policy`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">multipart-reader-with-policy</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartSource</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span>) -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartPolicy</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartReader</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span>))))</code></pre>

Snapshot arbitrary reader boundary bytes without applying writer restrictions.

[Source](/reference-sources/Effect.Multipart.cinch) · line 595

<a id="value-6d756c7469706172742d73696e6b"></a>

## `multipart-sink`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">multipart-sink</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Bytes</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Nat</span>, (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">e</span>)), (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartSink</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span>))) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartSink</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span>))</code></pre>

Construct a sink whose callbacks may return positive progress and failure together.

[Source](/reference-sources/Effect.Multipart.cinch) · line 103

<a id="value-6d756c7469706172742d73696e6b2d66726f6d2d73747265616d"></a>

## `multipart-sink-from-stream`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">multipart-sink-from-stream</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">StreamError</span> -&gt; (<span class="tok-constructor">Nat</span>, <span class="tok-variable">e</span>)) -&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteWriter</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartSink</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span>)))</code></pre>

Adapt a stream sink with explicit caller interpretation of error progress.
Error recovery supplies both the confirmed prefix and the original typed
failure; the engine validates that prefix against its actual offered bytes.

[Source](/reference-sources/Effect.Multipart.cinch) · line 120

<a id="value-6d756c7469706172742d73696e6b2d7772697465"></a>

## `multipart-sink-write`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">multipart-sink-write</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartSink</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span>) -&gt; (<span class="tok-constructor">Bytes</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Nat</span>, (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">e</span>)), (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartSink</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span>))))</code></pre>

Invoke exactly one sink callback, including for empty offers.

[Source](/reference-sources/Effect.Multipart.cinch) · line 107

<a id="value-6d756c7469706172742d736f75726365"></a>

## `multipart-source`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">multipart-source</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Bytes</span>, (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartReadStatus</span> <span class="tok-variable">e</span>)), (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartSource</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span>))) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartSource</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span>))</code></pre>

Construct a source preserving effectful zero reads and simultaneous statuses.

[Source](/reference-sources/Effect.Multipart.cinch) · line 90

<a id="value-6d756c7469706172742d736f757263652d66726f6d2d73747265616d"></a>

## `multipart-source-from-stream`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">multipart-source-from-stream</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">StreamError</span> -&gt; <span class="tok-variable">e</span>) -&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteReader</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartSource</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span>)))</code></pre>

Adapt an ordinary stream explicitly; positive empty success means EOF.
The underlying stream retains its documented inert zero-read policy.

[Source](/reference-sources/Effect.Multipart.cinch) · line 112

<a id="value-6d756c7469706172742d736f757263652d72656164"></a>

## `multipart-source-read`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">multipart-source-read</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartSource</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span>) -&gt; (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Bytes</span>, (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartReadStatus</span> <span class="tok-variable">e</span>)), (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartSource</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span>))))</code></pre>

Invoke one borrowed source callback; zero requests are not silently removed.

[Source](/reference-sources/Effect.Multipart.cinch) · line 94

<a id="value-6d756c7469706172742d73746f72616765"></a>

## `multipart-storage`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">multipart-storage</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">key</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">fileHandle</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">state</span> :: <span class="tok-constructor">Type</span>). (<span class="tok-variable">state</span> -&gt; ((<span class="tok-variable">state</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-variable">e</span> (<span class="tok-variable">key</span>, <span class="tok-variable">fileHandle</span>)), <span class="tok-variable">state</span>)) -&gt; ((<span class="tok-variable">state</span> -&gt; (<span class="tok-variable">fileHandle</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Nat</span>, (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">e</span>)), <span class="tok-variable">state</span>)))) -&gt; ((<span class="tok-variable">state</span> -&gt; (<span class="tok-variable">fileHandle</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Bytes</span>, (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartReadStatus</span> <span class="tok-variable">e</span>)), <span class="tok-variable">state</span>)))) -&gt; ((<span class="tok-variable">state</span> -&gt; (<span class="tok-variable">fileHandle</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">I64</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Bytes</span>, (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartReadStatus</span> <span class="tok-variable">e</span>)), <span class="tok-variable">state</span>))))) -&gt; ((<span class="tok-variable">state</span> -&gt; (<span class="tok-variable">fileHandle</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-variable">e</span> <span class="tok-constructor">I64</span>), <span class="tok-variable">state</span>))))) -&gt; ((<span class="tok-variable">state</span> -&gt; (<span class="tok-variable">fileHandle</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-variable">e</span> <span class="tok-constructor">Unit</span>), <span class="tok-variable">state</span>))) -&gt; ((<span class="tok-variable">state</span> -&gt; (<span class="tok-variable">key</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-variable">e</span> <span class="tok-variable">fileHandle</span>), <span class="tok-variable">state</span>))) -&gt; ((<span class="tok-variable">state</span> -&gt; (<span class="tok-variable">key</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-variable">e</span> <span class="tok-constructor">Bool</span>), <span class="tok-variable">state</span>))) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartStorage</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span> <span class="tok-variable">key</span> <span class="tok-variable">fileHandle</span> <span class="tok-variable">state</span>))))))))))</code></pre>

Supply real temporary creation and all file capabilities with arbitrary typed state.
Unlink returns False for an already missing key. Positioned reads must leave
sequential cursor state unchanged. Every callback returns its advanced state.

[Source](/reference-sources/Effect.Multipart.cinch) · line 884

<a id="value-6d756c7469706172742d73746f726167652d7374617465"></a>

## `multipart-storage-state`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">multipart-storage-state</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">key</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">fileHandle</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">state</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartStorage</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span> <span class="tok-variable">key</span> <span class="tok-variable">fileHandle</span> <span class="tok-variable">state</span>) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-variable">state</span>)</code></pre>

Observe the actual advanced backend state, including failed-operation progress.

[Source](/reference-sources/Effect.Multipart.cinch) · line 899

<a id="value-6d756c7469706172742d77726974652d6669656c64"></a>

## `multipart-write-field`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">multipart-write-field</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartWriter</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span>) -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartError</span> <span class="tok-variable">e</span>) <span class="tok-constructor">Unit</span>))))</code></pre>

Create a field and make one body write; short success is not a new error.

[Source](/reference-sources/Effect.Multipart.cinch) · line 302

<a id="value-6d756c7469706172742d7772697465722d626f756e64617279"></a>

## `multipart-writer-boundary`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">multipart-writer-boundary</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>). (<span class="tok-constructor">Bytes</span> -&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartSink</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span>) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartError</span> <span class="tok-variable">e</span>) (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartWriter</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span>))))</code></pre>

Create a writer with an explicit validated boundary and a borrowed sink.

[Source](/reference-sources/Effect.Multipart.cinch) · line 164

<a id="value-6d756c7469706172742d7772697465722d636f6e74656e742d74797065"></a>

## `multipart-writer-content-type`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">multipart-writer-content-type</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartWriter</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span>) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Bytes</span>)</code></pre>

Render content type from the current session boundary without mutating it.

[Source](/reference-sources/Effect.Multipart.cinch) · line 219

<a id="value-6d756c7469706172742d7772697465722d66696e697368"></a>

## `multipart-writer-finish`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">multipart-writer-finish</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartWriter</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span>) -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartError</span> <span class="tok-variable">e</span>) <span class="tok-constructor">Unit</span>))</code></pre>

Finish the current body and emit one footer without closing the sink/session.

[Source](/reference-sources/Effect.Multipart.cinch) · line 318

<a id="value-6d756c7469706172742d7772697465722d6765742d626f756e64617279"></a>

## `multipart-writer-get-boundary`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">multipart-writer-get-boundary</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartWriter</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span>) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Bytes</span>)</code></pre>

Observe an independently mutable boundary snapshot from the shared session.

[Source](/reference-sources/Effect.Multipart.cinch) · line 209

<a id="value-6d756c7469706172742d7772697465722d72616e646f6d"></a>

## `multipart-writer-random`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">multipart-writer-random</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartSource</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span>) -&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartSink</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span>) -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartError</span> <span class="tok-variable">e</span>) (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartWriter</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span>)), (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartSource</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span>))))</code></pre>

Begin with thirty explicit entropy bytes and return the advanced borrowed source.
Entropy failure is typed; a successful draw yields sixty lowercase boundary bytes.

[Source](/reference-sources/Effect.Multipart.cinch) · line 203

<a id="value-6d756c7469706172742d7772697465722d7365742d626f756e64617279"></a>

## `multipart-writer-set-boundary`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">multipart-writer-set-boundary</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartWriter</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span>) -&gt; (<span class="tok-constructor">Bytes</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartError</span> <span class="tok-variable">e</span>) <span class="tok-constructor">Unit</span>)))</code></pre>

Change a boundary only when no current part remains assigned.

[Source](/reference-sources/Effect.Multipart.cinch) · line 223

<a id="value-6d756c7469706172742d7772697465722d73696e6b"></a>

## `multipart-writer-sink`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">multipart-writer-sink</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartWriter</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span>) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Multipart</span>.<span class="tok-constructor">MultipartSink</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span>))</code></pre>

Observe the actual advanced borrowed sink without closing it.

[Source](/reference-sources/Effect.Multipart.cinch) · line 214

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Effect/Multipart.cinch`. Source SHA-256: `1969a6077e1aafb84226e9f4457ceb0b17395bdd9ad82a3a8256ec02e22a8044`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
