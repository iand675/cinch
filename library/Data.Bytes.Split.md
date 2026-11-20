# Data.Bytes.Split

Retained byte splitting, persistent producer traversals, shared pull cursors and line iteration.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Data.Bytes.Split`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Bytes.Split.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Bytes.Utf8](Data.Bytes.Utf8.md), [Data.Bytes.View](Data.Bytes.View.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Producer](Data.Producer.md).

Non-embedded source closure: [Data.Bytes.Split](Data.Bytes.Split.md), [Data.Bytes.Utf8](Data.Bytes.Utf8.md), [Data.Bytes.View](Data.Bytes.View.md), [Data.Bytes.Word](Data.Bytes.Word.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Retained byte splitting with explicit traversal factories, persistent producers and stateful pull cursors.
Result views share input mutations and retain the complete original allocation.
Raw separators are observed at each step; compiled separators retain their stable snapshots.

</details>

# Types and constructors

<a id="type-4279746553706c6974437572736f72"></a>

## `ByteSplitCursor`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">ByteSplitCursor</span></code></pre>

A separately named stateful pull cursor; copied handles share progress and closure state.

[Source](/reference-sources/Data.Bytes.Split.cinch) · line 49

<a id="type-4279746553706c6974466163746f7279"></a>

## `ByteSplitFactory`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">ByteSplitFactory</span></code></pre>

An immutable traversal recipe retaining input, separator policy, limit and fragment placement.

[Source](/reference-sources/Data.Bytes.Split.cinch) · line 39

<a id="type-4279746553706c69744d6f6465"></a>

## `ByteSplitMode`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">ByteSplitMode</span></code></pre>

Whether a separator belongs to the preceding output fragment.

[Source](/reference-sources/Data.Bytes.Split.cinch) · line 25

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SplitBefore</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">Split</span>.<span class="tok-constructor">ByteSplitMode</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SplitAfter</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">Split</span>.<span class="tok-constructor">ByteSplitMode</span></code></pre>

<a id="type-4279746553706c69745374617465"></a>

## `ByteSplitState`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">ByteSplitState</span></code></pre>

Explicit persistent producer progress; advancing returns a new state without mutating this value.

[Source](/reference-sources/Data.Bytes.Split.cinch) · line 44

# Values

<a id="value-627974652d73706c69742d636c6f7365"></a>

## `byte-split-close`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">byte-split-close</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">Split</span>.<span class="tok-constructor">ByteSplitCursor</span> -&gt; <span class="tok-constructor">Unit</span>)</code></pre>

Close a separately named pull cursor; later pulls return None without reading or searching input.

[Source](/reference-sources/Data.Bytes.Split.cinch) · line 242

<a id="value-627974652d73706c69742d637572736f72"></a>

## `byte-split-cursor`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">byte-split-cursor</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">Split</span>.<span class="tok-constructor">ByteSplitFactory</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">Split</span>.<span class="tok-constructor">ByteSplitCursor</span>)</code></pre>

Begin a fresh stateful pull cursor without changing Producer replay semantics.
Copied cursor handles share progress; raw separator and input contents remain live.

[Source](/reference-sources/Data.Bytes.Split.cinch) · line 226

<a id="value-627974652d73706c69742d6e657874"></a>

## `byte-split-next`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">byte-split-next</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">Split</span>.<span class="tok-constructor">ByteSplitCursor</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span>))</code></pre>

Pull the next mutable alias, returning None after exhaustion or explicit closure on every later call.

[Source](/reference-sources/Data.Bytes.Split.cinch) · line 230

<a id="value-627974652d73706c69742d70726f6475636572"></a>

## `byte-split-producer`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">byte-split-producer</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">Split</span>.<span class="tok-constructor">ByteSplitFactory</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Producer</span>.<span class="tok-constructor">Producer</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">Split</span>.<span class="tok-constructor">ByteSplitState</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span>))</code></pre>

Begin a fresh persistent producer traversal of a retained recipe.
Replaying this Producer follows the existing explicit-state contract and reads current mutable bytes again.

[Source](/reference-sources/Data.Bytes.Split.cinch) · line 128

<a id="value-627974652d766965772d6c696e6573"></a>

## `byte-view-lines`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">byte-view-lines</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Producer</span>.<span class="tok-constructor">Producer</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">Split</span>.<span class="tok-constructor">ByteSplitState</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span>))</code></pre>

Begin a fresh persistent LF-line traversal including line terminators.

[Source](/reference-sources/Data.Bytes.Split.cinch) · line 171

<a id="value-627974652d766965772d6c696e65732d666163746f7279"></a>

## `byte-view-lines-factory`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">byte-view-lines-factory</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">Split</span>.<span class="tok-constructor">ByteSplitFactory</span>)</code></pre>

Create an LF-line recipe that includes terminators and omits a trailing empty fragment.
CR bytes are preserved; an unterminated final line is returned intact.

[Source](/reference-sources/Data.Bytes.Split.cinch) · line 70

<a id="value-627974652d766965772d73706c6974"></a>

## `byte-view-split`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">byte-view-split</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span>)))</code></pre>

Split into mutable aliases around each nonoverlapping raw separator.

[Source](/reference-sources/Data.Bytes.Split.cinch) · line 143

<a id="value-627974652d766965772d73706c69742d6166746572"></a>

## `byte-view-split-after`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">byte-view-split-after</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span>)))</code></pre>

Split after each nonoverlapping raw separator, retaining it in the preceding alias.

[Source](/reference-sources/Data.Bytes.Split.cinch) · line 147

<a id="value-627974652d766965772d73706c69742d61667465722d6c696d6974"></a>

## `byte-view-split-after-limit`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">byte-view-split-after-limit</span> :: ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Nat</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span>))))</code></pre>

Split after separators with an explicit optional maximum and an unsplit final remainder.

[Source](/reference-sources/Data.Bytes.Split.cinch) · line 155

<a id="value-627974652d766965772d73706c69742d61667465722d736571"></a>

## `byte-view-split-after-seq`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">byte-view-split-after-seq</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Producer</span>.<span class="tok-constructor">Producer</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">Split</span>.<span class="tok-constructor">ByteSplitState</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span>)))</code></pre>

Begin a fresh persistent producer that includes each delimiter in its preceding fragment.

[Source](/reference-sources/Data.Bytes.Split.cinch) · line 163

<a id="value-627974652d766965772d73706c69742d666163746f7279"></a>

## `byte-view-split-factory`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">byte-view-split-factory</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">Split</span>.<span class="tok-constructor">ByteSplitMode</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Nat</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">Split</span>.<span class="tok-constructor">ByteSplitFactory</span>))))</code></pre>

Create a reusable recipe without reading input or separator contents.
None means unlimited; Some zero yields nothing; the final limited fragment is the unsplit remainder.

[Source](/reference-sources/Data.Bytes.Split.cinch) · line 55

<a id="value-627974652d766965772d73706c69742d6c696d6974"></a>

## `byte-view-split-limit`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">byte-view-split-limit</span> :: ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Nat</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span>))))</code></pre>

Split with an explicit optional maximum and an unsplit final remainder.

[Source](/reference-sources/Data.Bytes.Split.cinch) · line 151

<a id="value-627974652d766965772d73706c69742d7061747465726e"></a>

## `byte-view-split-pattern`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">byte-view-split-pattern</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">Split</span>.<span class="tok-constructor">ByteSplitMode</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Nat</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">BytePattern</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span>)))))</code></pre>

Materialize fragments using an explicitly stable compiled separator snapshot.

[Source](/reference-sources/Data.Bytes.Split.cinch) · line 167

<a id="value-627974652d766965772d73706c69742d7061747465726e2d666163746f7279"></a>

## `byte-view-split-pattern-factory`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">byte-view-split-pattern-factory</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">Split</span>.<span class="tok-constructor">ByteSplitMode</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Nat</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">BytePattern</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">Split</span>.<span class="tok-constructor">ByteSplitFactory</span>))))</code></pre>

Create a recipe with an explicitly stable compiled separator; input bytes remain live aliases.

[Source](/reference-sources/Data.Bytes.Split.cinch) · line 65

<a id="value-627974652d766965772d73706c69742d736571"></a>

## `byte-view-split-seq`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">byte-view-split-seq</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Producer</span>.<span class="tok-constructor">Producer</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">Split</span>.<span class="tok-constructor">ByteSplitState</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span>)))</code></pre>

Begin a fresh persistent raw-separator producer, observing input and delimiter changes at each step.

[Source](/reference-sources/Data.Bytes.Split.cinch) · line 159

<a id="value-62797465732d6c696e6573"></a>

## `bytes-lines`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bytes-lines</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Producer</span>.<span class="tok-constructor">Producer</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">Split</span>.<span class="tok-constructor">ByteSplitState</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span>))</code></pre>

Begin a fresh persistent LF-line traversal over a complete fixed buffer.

[Source](/reference-sources/Data.Bytes.Split.cinch) · line 199

<a id="value-62797465732d73706c6974"></a>

## `bytes-split`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bytes-split</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span>)))</code></pre>

Split complete fixed buffers into retained mutable input aliases.

[Source](/reference-sources/Data.Bytes.Split.cinch) · line 175

<a id="value-62797465732d73706c69742d6166746572"></a>

## `bytes-split-after`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bytes-split-after</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span>)))</code></pre>

Split complete fixed buffers after each raw delimiter.

[Source](/reference-sources/Data.Bytes.Split.cinch) · line 179

<a id="value-62797465732d73706c69742d61667465722d6c696d6974"></a>

## `bytes-split-after-limit`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bytes-split-after-limit</span> :: ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Nat</span>) -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span>))))</code></pre>

Split complete fixed buffers after delimiters with an explicit optional maximum.

[Source](/reference-sources/Data.Bytes.Split.cinch) · line 187

<a id="value-62797465732d73706c69742d61667465722d736571"></a>

## `bytes-split-after-seq`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bytes-split-after-seq</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Producer</span>.<span class="tok-constructor">Producer</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">Split</span>.<span class="tok-constructor">ByteSplitState</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span>)))</code></pre>

Begin a fresh persistent delimiter-including traversal over complete fixed buffers.

[Source](/reference-sources/Data.Bytes.Split.cinch) · line 195

<a id="value-62797465732d73706c69742d666163746f7279"></a>

## `bytes-split-factory`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bytes-split-factory</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">Split</span>.<span class="tok-constructor">ByteSplitMode</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Nat</span>) -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">Split</span>.<span class="tok-constructor">ByteSplitFactory</span>))))</code></pre>

Retain complete input and separator owners in a fresh raw-separator traversal recipe.

[Source](/reference-sources/Data.Bytes.Split.cinch) · line 59

<a id="value-62797465732d73706c69742d6c696d6974"></a>

## `bytes-split-limit`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bytes-split-limit</span> :: ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Nat</span>) -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span>))))</code></pre>

Split complete fixed buffers with an explicit optional maximum.

[Source](/reference-sources/Data.Bytes.Split.cinch) · line 183

<a id="value-62797465732d73706c69742d736571"></a>

## `bytes-split-seq`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bytes-split-seq</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Producer</span>.<span class="tok-constructor">Producer</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">Split</span>.<span class="tok-constructor">ByteSplitState</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span>)))</code></pre>

Begin a fresh persistent traversal over complete input and raw separator buffers.

[Source](/reference-sources/Data.Bytes.Split.cinch) · line 191

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Data/Bytes/Split.cinch`. Source SHA-256: `2b534f9538e49c5e0d364721a5fca28cbda3808586ceb8c4e163bec1e0b9d26d`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
