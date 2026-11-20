# Data.Bytes.Fields

Nonempty raw-byte field views with Unicode whitespace, ordered scalar predicates, persistent producers and replayable effect-preserving iteration.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Data.Bytes.Fields`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Bytes.Fields.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Bytes.Utf8](Data.Bytes.Utf8.md), [Data.Bytes.View](Data.Bytes.View.md), [Data.Char](Data.Char.md), [Data.List](Data.List.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Producer](Data.Producer.md), [Data.Unicode.Kernel](Data.Unicode.Kernel.md).

Non-embedded source closure: [Data.Bytes.Fields](Data.Bytes.Fields.md), [Data.Bytes.Utf8](Data.Bytes.Utf8.md), [Data.Bytes.View](Data.Bytes.View.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Unicode.Kernel](Data.Unicode.Kernel.md), [Data.Unicode.Tables](Data.Unicode.Tables.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Retained raw-byte fields with Unicode whitespace and effect-preserving scalar predicates.
Fields are nonempty fixed views into the original owner. Malformed UTF8 bytes
each present U+FFFD to a predicate and remain unchanged in returned aliases.
Predicate callbacks run once per decoded scalar from left to right. Sequence
factories do no scanning at construction and start afresh on every invocation.

</details>

# Types and constructors

<a id="type-427974654669656c64735374617465"></a>

## `ByteFieldsState`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">ByteFieldsState</span></code></pre>

Immutable explicit progress retaining a complete fixed input owner.
Reusing a state replays its scan against current aliases; it does not share a hidden mutable cursor.

[Source](/reference-sources/Data.Bytes.Fields.cinch) · line 26

# Values

<a id="value-627974652d6669656c64732d6e657874"></a>

## `byte-fields-next`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">byte-fields-next</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">Fields</span>.<span class="tok-constructor">ByteFieldsState</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Producer</span>.<span class="tok-constructor">Step</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">Fields</span>.<span class="tok-constructor">ByteFieldsState</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span>))</code></pre>

Advance one default Unicode-whitespace field without copying its bytes.

[Source](/reference-sources/Data.Bytes.Fields.cinch) · line 96

<a id="value-627974652d6669656c64732d6e6578742d77697468"></a>

## `byte-fields-next-with`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">byte-fields-next-with</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">e</span> . ((<span class="tok-constructor">Char</span> -&gt;{<span class="tok-variable">e</span>} <span class="tok-constructor">Bool</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">Fields</span>.<span class="tok-constructor">ByteFieldsState</span> -&gt;{<span class="tok-variable">e</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Producer</span>.<span class="tok-constructor">Step</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">Fields</span>.<span class="tok-constructor">ByteFieldsState</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span>))</code></pre>

Advance one field using a genuinely effectful scalar predicate and return explicit next progress.
Current input mutations remain observable. Each malformed byte is one replacement-scalar predicate call.

[Source](/reference-sources/Data.Bytes.Fields.cinch) · line 66

<a id="value-627974652d766965772d6669656c6473"></a>

## `byte-view-fields`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">byte-view-fields</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span>))</code></pre>

Split a sealed range around Unicode whitespace; discard empty leading, trailing and intermediate runs.

[Source](/reference-sources/Data.Bytes.Fields.cinch) · line 115

<a id="value-627974652d766965772d6669656c64732d65616368"></a>

## `byte-view-fields-each`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">byte-view-fields-each</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">e</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span> -&gt;{<span class="tok-variable">e</span>} <span class="tok-constructor">Bool</span>) -&gt;{<span class="tok-variable">e</span>} <span class="tok-constructor">Unit</span>))</code></pre>

Traverse default fields with an effectful yield callback and no materialized outer list.

[Source](/reference-sources/Data.Bytes.Fields.cinch) · line 163

<a id="value-627974652d766965772d6669656c64732d66756e63"></a>

## `byte-view-fields-func`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">byte-view-fields-func</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">e</span> . ((<span class="tok-constructor">Char</span> -&gt;{<span class="tok-variable">e</span>} <span class="tok-constructor">Bool</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span> -&gt;{<span class="tok-variable">e</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span>))</code></pre>

Eagerly collect fields using once-per-scalar left-to-right predicate calls, including all trailing delimiters.

[Source](/reference-sources/Data.Bytes.Fields.cinch) · line 123

<a id="value-627974652d766965772d6669656c64732d66756e632d65616368"></a>

## `byte-view-fields-func-each`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">byte-view-fields-func-each</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">e</span> . ((<span class="tok-constructor">Char</span> -&gt;{<span class="tok-variable">e</span>} <span class="tok-constructor">Bool</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span> -&gt;{<span class="tok-variable">e</span>} <span class="tok-constructor">Bool</span>) -&gt;{<span class="tok-variable">e</span>} <span class="tok-constructor">Unit</span>)))</code></pre>

Traverse predicate fields with a shared effect row containing every predicate and yield capability.
Use explicit effect weakening wrappers when distinct callback rows need a combined row.

[Source](/reference-sources/Data.Bytes.Fields.cinch) · line 172

<a id="value-627974652d766965772d6669656c64732d66756e632d70726f6475636572"></a>

## `byte-view-fields-func-producer`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">byte-view-fields-func-producer</span> :: ((<span class="tok-constructor">Char</span> -&gt; <span class="tok-constructor">Bool</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Producer</span>.<span class="tok-constructor">Producer</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">Fields</span>.<span class="tok-constructor">ByteFieldsState</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span>)))</code></pre>

Adapt a pure-row predicate to the existing pure Producer abstraction.
Effectful predicates use the next-with or callback sequence operations instead.

[Source](/reference-sources/Data.Bytes.Fields.cinch) · line 140

<a id="value-627974652d766965772d6669656c64732d66756e632d73657175656e6365"></a>

## `byte-view-fields-func-sequence`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">byte-view-fields-func-sequence</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">e</span> . ((<span class="tok-constructor">Char</span> -&gt;{<span class="tok-variable">e</span>} <span class="tok-constructor">Bool</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span> -&gt;{<span class="tok-variable">e</span>} <span class="tok-constructor">Bool</span>) -&gt;{<span class="tok-variable">e</span>} <span class="tok-constructor">Unit</span>)))</code></pre>

Capture an effectful predicate and fixed input range without invoking any callback.
Every traversal is fresh; callback state and mutable input aliases remain shared and observable.
Its row includes the intended yield capabilities too; apply explicit effect weakening before construction when needed.

[Source](/reference-sources/Data.Bytes.Fields.cinch) · line 190

<a id="value-627974652d766965772d6669656c64732d70726f6475636572"></a>

## `byte-view-fields-producer`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">byte-view-fields-producer</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Producer</span>.<span class="tok-constructor">Producer</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">Fields</span>.<span class="tok-constructor">ByteFieldsState</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span>))</code></pre>

Build a persistent default producer; fresh folds observe current input from the beginning.

[Source](/reference-sources/Data.Bytes.Fields.cinch) · line 131

<a id="value-627974652d766965772d6669656c64732d73657175656e6365"></a>

## `byte-view-fields-sequence`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">byte-view-fields-sequence</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">e</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span> -&gt;{<span class="tok-variable">e</span>} <span class="tok-constructor">Bool</span>) -&gt;{<span class="tok-variable">e</span>} <span class="tok-constructor">Unit</span>))</code></pre>

Construct a reusable default sequence without scanning; each invocation starts at the beginning.

[Source](/reference-sources/Data.Bytes.Fields.cinch) · line 180

<a id="value-627974652d766965772d6669656c64732d7374617274"></a>

## `byte-view-fields-start`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">byte-view-fields-start</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">Fields</span>.<span class="tok-constructor">ByteFieldsState</span>)</code></pre>

Start independent explicit progress without reading input contents or invoking callbacks.

[Source](/reference-sources/Data.Bytes.Fields.cinch) · line 31

<a id="value-62797465732d6669656c6473"></a>

## `bytes-fields`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bytes-fields</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span>))</code></pre>

Split complete raw bytes around Unicode whitespace into retained mutable aliases.

[Source](/reference-sources/Data.Bytes.Fields.cinch) · line 119

<a id="value-62797465732d6669656c64732d65616368"></a>

## `bytes-fields-each`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bytes-fields-each</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">e</span> . (<span class="tok-constructor">Bytes</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span> -&gt;{<span class="tok-variable">e</span>} <span class="tok-constructor">Bool</span>) -&gt;{<span class="tok-variable">e</span>} <span class="tok-constructor">Unit</span>))</code></pre>

Traverse complete-buffer default fields while preserving yield effects and early stop.

[Source](/reference-sources/Data.Bytes.Fields.cinch) · line 167

<a id="value-62797465732d6669656c64732d66756e63"></a>

## `bytes-fields-func`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bytes-fields-func</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">e</span> . ((<span class="tok-constructor">Char</span> -&gt;{<span class="tok-variable">e</span>} <span class="tok-constructor">Bool</span>) -&gt; (<span class="tok-constructor">Bytes</span> -&gt;{<span class="tok-variable">e</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span>))</code></pre>

Eagerly collect complete-buffer aliases while preserving the predicate's effects.

[Source](/reference-sources/Data.Bytes.Fields.cinch) · line 127

<a id="value-62797465732d6669656c64732d66756e632d65616368"></a>

## `bytes-fields-func-each`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bytes-fields-func-each</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">e</span> . ((<span class="tok-constructor">Char</span> -&gt;{<span class="tok-variable">e</span>} <span class="tok-constructor">Bool</span>) -&gt; (<span class="tok-constructor">Bytes</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span> -&gt;{<span class="tok-variable">e</span>} <span class="tok-constructor">Bool</span>) -&gt;{<span class="tok-variable">e</span>} <span class="tok-constructor">Unit</span>)))</code></pre>

Traverse complete-buffer predicate fields without restricting either callback to a pure or IO-only row.

[Source](/reference-sources/Data.Bytes.Fields.cinch) · line 176

<a id="value-62797465732d6669656c64732d66756e632d70726f6475636572"></a>

## `bytes-fields-func-producer`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bytes-fields-func-producer</span> :: ((<span class="tok-constructor">Char</span> -&gt; <span class="tok-constructor">Bool</span>) -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Producer</span>.<span class="tok-constructor">Producer</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">Fields</span>.<span class="tok-constructor">ByteFieldsState</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span>)))</code></pre>

Build the pure-row predicate Producer convenience adapter over complete raw bytes.

[Source](/reference-sources/Data.Bytes.Fields.cinch) · line 144

<a id="value-62797465732d6669656c64732d66756e632d73657175656e6365"></a>

## `bytes-fields-func-sequence`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bytes-fields-func-sequence</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">e</span> . ((<span class="tok-constructor">Char</span> -&gt;{<span class="tok-variable">e</span>} <span class="tok-constructor">Bool</span>) -&gt; (<span class="tok-constructor">Bytes</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span> -&gt;{<span class="tok-variable">e</span>} <span class="tok-constructor">Bool</span>) -&gt;{<span class="tok-variable">e</span>} <span class="tok-constructor">Unit</span>)))</code></pre>

Construct a reusable predicate/yield sequence over a retained complete byte buffer.

[Source](/reference-sources/Data.Bytes.Fields.cinch) · line 194

<a id="value-62797465732d6669656c64732d70726f6475636572"></a>

## `bytes-fields-producer`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bytes-fields-producer</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Producer</span>.<span class="tok-constructor">Producer</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">Fields</span>.<span class="tok-constructor">ByteFieldsState</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span>))</code></pre>

Build a persistent default producer over a complete buffer without copying it.

[Source](/reference-sources/Data.Bytes.Fields.cinch) · line 135

<a id="value-62797465732d6669656c64732d73657175656e6365"></a>

## `bytes-fields-sequence`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bytes-fields-sequence</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">e</span> . (<span class="tok-constructor">Bytes</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span> -&gt;{<span class="tok-variable">e</span>} <span class="tok-constructor">Bool</span>) -&gt;{<span class="tok-variable">e</span>} <span class="tok-constructor">Unit</span>))</code></pre>

Retain complete raw bytes in a reusable effectful-yield sequence factory.

[Source](/reference-sources/Data.Bytes.Fields.cinch) · line 184

<a id="value-62797465732d6669656c64732d7374617274"></a>

## `bytes-fields-start`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bytes-fields-start</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">Fields</span>.<span class="tok-constructor">ByteFieldsState</span>)</code></pre>

Retain a whole fixed buffer in fresh explicit progress without copying it.

[Source](/reference-sources/Data.Bytes.Fields.cinch) · line 35

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Data/Bytes/Fields.cinch`. Source SHA-256: `1fe6b9ace994988ba6b82abe785e939f4798f189206ac2412802bfcbc76e6963`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
