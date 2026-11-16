# Data.Mutable.LinkedList

Stable typed list elements, checked membership and movement, detached value access and bounded self-copy.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Data.Mutable.LinkedList`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Mutable.LinkedList.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bytes](Data.Bytes.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md).

Non-embedded source closure: [Data.Mutable.LinkedList](Data.Mutable.LinkedList.md).

<details><summary>Behaviour and boundary notes from the source</summary>

A mutable doubly linked sequence with stable, independently retained element handles.
Elements retain their values after removal or clearing. Navigation and structural
changes take the owning list explicitly and reject foreign or stale handles.
Links are machine-word arena indices, so the structure creates no ownership
cycles and an element handle never retains the list or its other elements.

</details>

# Types and constructors

<a id="type-4c696e6b65644c697374"></a>

## `LinkedList`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">LinkedList</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>)</code></pre>

An opaque root owning live nodes, reusable holes and independent identity counters.

[Source](/reference-sources/Data.Mutable.LinkedList.cinch) · line 52

<a id="type-4c696e6b65644c6973744572726f72"></a>

## `LinkedListError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">LinkedListError</span></code></pre>

A checked structural refusal; failed handle checks leave both lists unchanged.

[Source](/reference-sources/Data.Mutable.LinkedList.cinch) · line 38

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ForeignListElement</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">LinkedList</span>.<span class="tok-constructor">LinkedListError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">RemovedListElement</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">LinkedList</span>.<span class="tok-constructor">LinkedListError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ListCapacityExceeded</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">LinkedList</span>.<span class="tok-constructor">LinkedListError</span></code></pre>

<a id="type-4c697374456c656d656e74"></a>

## `ListElement`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">ListElement</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>)</code></pre>

A stable element identity with independently mutable value storage.

[Source](/reference-sources/Data.Mutable.LinkedList.cinch) · line 57

# Values

<a id="value-6c6973742d617070656e642d6c697374"></a>

## `list-append-list`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">list-append-list</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> <span class="tok-variable">s</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">LinkedList</span>.<span class="tok-constructor">LinkedList</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">LinkedList</span>.<span class="tok-constructor">LinkedList</span> <span class="tok-variable">s</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>, <span class="tok-constructor">Mut</span> <span class="tok-variable">s</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">LinkedList</span>.<span class="tok-constructor">LinkedListError</span> <span class="tok-constructor">Unit</span>))</code></pre>

Append the source values in order, including a bounded copy when both arguments alias the same list.

[Source](/reference-sources/Data.Mutable.LinkedList.cinch) · line 421

<a id="value-6c6973742d6261636b"></a>

## `list-back`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">list-back</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">LinkedList</span>.<span class="tok-constructor">LinkedList</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">LinkedList</span>.<span class="tok-constructor">ListElement</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span>))</code></pre>

Retain the last live element, or None for an empty list.

[Source](/reference-sources/Data.Mutable.LinkedList.cinch) · line 227

<a id="value-6c6973742d6361706163697479"></a>

## `list-capacity`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">list-capacity</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">LinkedList</span>.<span class="tok-constructor">LinkedList</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Nat</span>)</code></pre>

Observe the arena allocation's current slot capacity, including reusable holes.

[Source](/reference-sources/Data.Mutable.LinkedList.cinch) · line 172

<a id="value-6c6973742d636c656172"></a>

## `list-clear`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">list-clear</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">LinkedList</span>.<span class="tok-constructor">LinkedList</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>)</code></pre>

Remove every membership while detached handles keep their own values.
The root drops its node owners and replaces the generation token. Releasing
unretained nodes costs linear work; no handle retains the cleared root.

[Source](/reference-sources/Data.Mutable.LinkedList.cinch) · line 210

<a id="value-6c6973742d656c656d656e742d736574"></a>

## `list-element-set`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">list-element-set</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">LinkedList</span>.<span class="tok-constructor">ListElement</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>))</code></pre>

Change an element's retained value, including after removal or clearing.

[Source](/reference-sources/Data.Mutable.LinkedList.cinch) · line 246

<a id="value-6c6973742d656c656d656e742d76616c7565"></a>

## `list-element-value`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">list-element-value</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">LinkedList</span>.<span class="tok-constructor">ListElement</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-variable">a</span>)</code></pre>

Read an element's current value independently of list membership.

[Source](/reference-sources/Data.Mutable.LinkedList.cinch) · line 242

<a id="value-6c6973742d66726f6d2d6c697374"></a>

## `list-from-list`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">list-from-list</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">LinkedList</span>.<span class="tok-constructor">LinkedListError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">LinkedList</span>.<span class="tok-constructor">LinkedList</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span>))</code></pre>

Build a list from immutable values, sharing their ordinary value owners.

[Source](/reference-sources/Data.Mutable.LinkedList.cinch) · line 435

<a id="value-6c6973742d66726f6e74"></a>

## `list-front`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">list-front</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">LinkedList</span>.<span class="tok-constructor">LinkedList</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">LinkedList</span>.<span class="tok-constructor">ListElement</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span>))</code></pre>

Retain the first live element, or None for an empty list.

[Source](/reference-sources/Data.Mutable.LinkedList.cinch) · line 222

<a id="value-6c6973742d696e736572742d6166746572"></a>

## `list-insert-after`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">list-insert-after</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">LinkedList</span>.<span class="tok-constructor">LinkedList</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">LinkedList</span>.<span class="tok-constructor">ListElement</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">LinkedList</span>.<span class="tok-constructor">LinkedListError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">LinkedList</span>.<span class="tok-constructor">ListElement</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span>))))</code></pre>

Insert immediately after a checked marker, rejecting stale or foreign markers before mutation.

[Source](/reference-sources/Data.Mutable.LinkedList.cinch) · line 330

<a id="value-6c6973742d696e736572742d6265666f7265"></a>

## `list-insert-before`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">list-insert-before</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">LinkedList</span>.<span class="tok-constructor">LinkedList</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">LinkedList</span>.<span class="tok-constructor">ListElement</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">LinkedList</span>.<span class="tok-constructor">LinkedListError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">LinkedList</span>.<span class="tok-constructor">ListElement</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span>))))</code></pre>

Insert immediately before a checked marker, rejecting stale or foreign markers before mutation.

[Source](/reference-sources/Data.Mutable.LinkedList.cinch) · line 324

<a id="value-6c6973742d69732d656d707479"></a>

## `list-is-empty`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">list-is-empty</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">LinkedList</span>.<span class="tok-constructor">LinkedList</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Bool</span>)</code></pre>

Test whether the list has no live elements.

[Source](/reference-sources/Data.Mutable.LinkedList.cinch) · line 176

<a id="value-6c6973742d69732d6d656d626572"></a>

## `list-is-member`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">list-is-member</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">LinkedList</span>.<span class="tok-constructor">LinkedList</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">LinkedList</span>.<span class="tok-constructor">ListElement</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Bool</span>))</code></pre>

Distinguish current membership from foreign, removed and cleared handles.

[Source](/reference-sources/Data.Mutable.LinkedList.cinch) · line 232

<a id="value-6c6973742d6c656e677468"></a>

## `list-length`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">list-length</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">LinkedList</span>.<span class="tok-constructor">LinkedList</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Nat</span>)</code></pre>

Observe the number of live elements in constant time.

[Source](/reference-sources/Data.Mutable.LinkedList.cinch) · line 168

<a id="value-6c6973742d6d6f76652d6166746572"></a>

## `list-move-after`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">list-move-after</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">LinkedList</span>.<span class="tok-constructor">LinkedList</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">LinkedList</span>.<span class="tok-constructor">ListElement</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">LinkedList</span>.<span class="tok-constructor">ListElement</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">LinkedList</span>.<span class="tok-constructor">LinkedListError</span> <span class="tok-constructor">Unit</span>)))</code></pre>

Validate both members before moving one immediately after the other; moving to itself succeeds unchanged.

[Source](/reference-sources/Data.Mutable.LinkedList.cinch) · line 384

<a id="value-6c6973742d6d6f76652d6261636b"></a>

## `list-move-back`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">list-move-back</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">LinkedList</span>.<span class="tok-constructor">LinkedList</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">LinkedList</span>.<span class="tok-constructor">ListElement</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">LinkedList</span>.<span class="tok-constructor">LinkedListError</span> <span class="tok-constructor">Unit</span>))</code></pre>

Move a checked member to the back without changing its identity or value.

[Source](/reference-sources/Data.Mutable.LinkedList.cinch) · line 362

<a id="value-6c6973742d6d6f76652d6265666f7265"></a>

## `list-move-before`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">list-move-before</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">LinkedList</span>.<span class="tok-constructor">LinkedList</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">LinkedList</span>.<span class="tok-constructor">ListElement</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">LinkedList</span>.<span class="tok-constructor">ListElement</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">LinkedList</span>.<span class="tok-constructor">LinkedListError</span> <span class="tok-constructor">Unit</span>)))</code></pre>

Validate both members before moving one immediately before the other; moving to itself succeeds unchanged.

[Source](/reference-sources/Data.Mutable.LinkedList.cinch) · line 372

<a id="value-6c6973742d6d6f76652d66726f6e74"></a>

## `list-move-front`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">list-move-front</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">LinkedList</span>.<span class="tok-constructor">LinkedList</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">LinkedList</span>.<span class="tok-constructor">ListElement</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">LinkedList</span>.<span class="tok-constructor">LinkedListError</span> <span class="tok-constructor">Unit</span>))</code></pre>

Move a checked member to the front without changing its identity or value.

[Source](/reference-sources/Data.Mutable.LinkedList.cinch) · line 352

<a id="value-6c6973742d6e6577"></a>

## `list-new`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">list-new</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Unit</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">LinkedList</span>.<span class="tok-constructor">LinkedList</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span>)</code></pre>

Create an empty list whose first insertion allocates its arena.

[Source](/reference-sources/Data.Mutable.LinkedList.cinch) · line 159

<a id="value-6c6973742d6e657874"></a>

## `list-next`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">list-next</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">LinkedList</span>.<span class="tok-constructor">LinkedList</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">LinkedList</span>.<span class="tok-constructor">ListElement</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">LinkedList</span>.<span class="tok-constructor">LinkedListError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">LinkedList</span>.<span class="tok-constructor">ListElement</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span>))))</code></pre>

Read the following element, with None at the back and a named error for an invalid handle.

[Source](/reference-sources/Data.Mutable.LinkedList.cinch) · line 258

<a id="value-6c6973742d70726570656e642d6c697374"></a>

## `list-prepend-list`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">list-prepend-list</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> <span class="tok-variable">s</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">LinkedList</span>.<span class="tok-constructor">LinkedList</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">LinkedList</span>.<span class="tok-constructor">LinkedList</span> <span class="tok-variable">s</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>, <span class="tok-constructor">Mut</span> <span class="tok-variable">s</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">LinkedList</span>.<span class="tok-constructor">LinkedListError</span> <span class="tok-constructor">Unit</span>))</code></pre>

Prepend the source values in order, including a bounded copy when both arguments alias the same list.

[Source](/reference-sources/Data.Mutable.LinkedList.cinch) · line 425

<a id="value-6c6973742d70726576"></a>

## `list-prev`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">list-prev</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">LinkedList</span>.<span class="tok-constructor">LinkedList</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">LinkedList</span>.<span class="tok-constructor">ListElement</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">LinkedList</span>.<span class="tok-constructor">LinkedListError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">LinkedList</span>.<span class="tok-constructor">ListElement</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span>))))</code></pre>

Read the preceding element, with None at the front and a named error for an invalid handle.

[Source](/reference-sources/Data.Mutable.LinkedList.cinch) · line 262

<a id="value-6c6973742d707573682d6261636b"></a>

## `list-push-back`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">list-push-back</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">LinkedList</span>.<span class="tok-constructor">LinkedList</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">LinkedList</span>.<span class="tok-constructor">LinkedListError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">LinkedList</span>.<span class="tok-constructor">ListElement</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span>)))</code></pre>

Insert a value after the current last element in amortized constant time.

[Source](/reference-sources/Data.Mutable.LinkedList.cinch) · line 320

<a id="value-6c6973742d707573682d66726f6e74"></a>

## `list-push-front`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">list-push-front</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">LinkedList</span>.<span class="tok-constructor">LinkedList</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">LinkedList</span>.<span class="tok-constructor">LinkedListError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">LinkedList</span>.<span class="tok-constructor">ListElement</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span>)))</code></pre>

Insert a value before the current first element in amortized constant time.

[Source](/reference-sources/Data.Mutable.LinkedList.cinch) · line 316

<a id="value-6c6973742d72656d6f7665"></a>

## `list-remove`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">list-remove</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">LinkedList</span>.<span class="tok-constructor">LinkedList</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">LinkedList</span>.<span class="tok-constructor">ListElement</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">LinkedList</span>.<span class="tok-constructor">LinkedListError</span> <span class="tok-variable">a</span>))</code></pre>

Remove one checked membership in constant time and return its retained value.
The old handle stays readable and writable; its slot may be reused by a
different node with a different serial, while every old membership stays stale.

[Source](/reference-sources/Data.Mutable.LinkedList.cinch) · line 338

<a id="value-6c6973742d72657365727665"></a>

## `list-reserve`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">list-reserve</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">LinkedList</span>.<span class="tok-constructor">LinkedList</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">LinkedList</span>.<span class="tok-constructor">LinkedListError</span> <span class="tok-constructor">Unit</span>))</code></pre>

Reserve room for additional live elements, returning a capacity refusal before mutation.

[Source](/reference-sources/Data.Mutable.LinkedList.cinch) · line 200

<a id="value-6c6973742d73616d652d656c656d656e74"></a>

## `list-same-element`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">list-same-element</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">s</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">LinkedList</span>.<span class="tok-constructor">ListElement</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span>) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">LinkedList</span>.<span class="tok-constructor">ListElement</span> <span class="tok-variable">s</span> <span class="tok-variable">a</span>) -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

Compare stable node identity even after removal, clearing or reuse of its old slot.

[Source](/reference-sources/Data.Mutable.LinkedList.cinch) · line 236

<a id="value-6c6973742d746f2d6c697374"></a>

## `list-to-list`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">list-to-list</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">LinkedList</span>.<span class="tok-constructor">LinkedList</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>)</code></pre>

Copy the current value sequence into an immutable list.

[Source](/reference-sources/Data.Mutable.LinkedList.cinch) · line 449

<a id="value-6c6973742d776974682d6361706163697479"></a>

## `list-with-capacity`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">list-with-capacity</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">LinkedList</span>.<span class="tok-constructor">LinkedListError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">LinkedList</span>.<span class="tok-constructor">LinkedList</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span>))</code></pre>

Create an empty list with a bounded minimum arena capacity.

[Source](/reference-sources/Data.Mutable.LinkedList.cinch) · line 163

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Data/Mutable/LinkedList.cinch`. Source SHA-256: `7251889893471322b75e5c8d7006ec87402c929db744b97cd3cdd7c90af38301`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
