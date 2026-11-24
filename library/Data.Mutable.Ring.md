# Data.Mutable.Ring

Mutable circular sequences with stable cursors, alias-preserving joins and splits, explicit value ownership and bounded arena growth.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Data.Mutable.Ring`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Mutable.Ring.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bytes](Data.Bytes.md), [Data.Int](Data.Int.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.Wide](Data.Wide.md).

Non-embedded source closure: [Data.Mutable.Ring](Data.Mutable.Ring.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Mutable circular sequences with stable cursor aliases through joins and splits.
Links are arena indices, so the structure creates no automatic ownership
cycles. Joining distinct backing arenas moves the smaller arena and forwards
its old cursors to the larger one; aliases keep denoting their original nodes.
A surviving cursor retains its entire joined arena, including disconnected
subrings. Use ring-copy and release old aliases when only one cycle is needed.

</details>

# Types and constructors

<a id="type-52696e67"></a>

## `Ring`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Ring</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>)</code></pre>

A nonempty circular-sequence cursor whose node identity survives arena forwarding.

[Source](/reference-sources/Data.Mutable.Ring.cinch) · line 53

<a id="type-52696e674572726f72"></a>

## `RingError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">RingError</span></code></pre>

A checked allocation-size refusal that leaves all existing cycles unchanged.

[Source](/reference-sources/Data.Mutable.Ring.cinch) · line 36

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">RingCapacityExceeded</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Ring</span>.<span class="tok-constructor">RingError</span></code></pre>

# Values

<a id="value-72696e672d636f7079"></a>

## `ring-copy`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ring-copy</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Ring</span>.<span class="tok-constructor">Ring</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span>) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Ring</span>.<span class="tok-constructor">Ring</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span>))</code></pre>

Copy only the current cycle into a fresh arena, preserving ordinary value sharing without cursor aliasing.
Releasing old cursors after this operation can release unrelated subrings
retained by the old joined arena.

[Source](/reference-sources/Data.Mutable.Ring.cinch) · line 380

<a id="value-72696e672d666f722d65616368"></a>

## `ring-for-each`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ring-for-each</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Ring</span>.<span class="tok-constructor">Ring</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span>)) -&gt; ((<span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>))</code></pre>

Visit at most the original cycle length through current links, with None performing no callbacks.
Value mutations are observed at each visit. Structural callback mutations
may change the visit set, but cannot create an unbounded traversal here.

[Source](/reference-sources/Data.Mutable.Ring.cinch) · line 414

<a id="value-72696e672d66726f6d2d6c697374"></a>

## `ring-from-list`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ring-from-list</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Ring</span>.<span class="tok-constructor">RingError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Ring</span>.<span class="tok-constructor">Ring</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span>))))</code></pre>

Construct a circular sequence in source order, returning None for empty input.

[Source](/reference-sources/Data.Mutable.Ring.cinch) · line 363

<a id="value-72696e672d6c656e677468"></a>

## `ring-length`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ring-length</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Ring</span>.<span class="tok-constructor">Ring</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span>)) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Nat</span>)</code></pre>

Count one circular component in linear traversal time, or return zero for None.

[Source](/reference-sources/Data.Mutable.Ring.cinch) · line 209

<a id="value-72696e672d6c696e6b"></a>

## `ring-link`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ring-link</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Ring</span>.<span class="tok-constructor">Ring</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span>) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Ring</span>.<span class="tok-constructor">Ring</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span>)) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Ring</span>.<span class="tok-constructor">RingError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Ring</span>.<span class="tok-constructor">Ring</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span>))))</code></pre>

Link the second cycle after the first cursor, or split when both cursors already share a cycle.
None leaves links unchanged. The result always denotes the first cursor's
original successor. Linking a cursor to itself isolates that node and
returns the remaining cycle's first cursor; linking its current successor
leaves the cycle unchanged. After resolving forwarding chains, same-arena
link updates take constant time. Distinct-arena links move the smaller
complete backing arena; forwarding depth grows at most logarithmically.

[Source](/reference-sources/Data.Mutable.Ring.cinch) · line 336

<a id="value-72696e672d6d6f7665"></a>

## `ring-move`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ring-move</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Ring</span>.<span class="tok-constructor">Ring</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Int</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Ring</span>.<span class="tok-constructor">Ring</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span>)))</code></pre>

Move by an arbitrary signed distance, preserving exact cyclic semantics for very large integers.
Distances from -64 through 64 walk directly, so short moves do not scan
a large cycle. Larger distances scan once, reduce before conversion and
walk the shorter equivalent direction.

[Source](/reference-sources/Data.Mutable.Ring.cinch) · line 231

<a id="value-72696e672d6e6577"></a>

## `ring-new`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ring-new</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Ring</span>.<span class="tok-constructor">RingError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Ring</span>.<span class="tok-constructor">Ring</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span>)))))</code></pre>

Create a checked circular sequence; zero returns None and no arena.
The initializer is shared as an ordinary value, while assigning one node's
value later does not replace another node's value cell.

[Source](/reference-sources/Data.Mutable.Ring.cinch) · line 156

<a id="value-72696e672d6e65772d73697a6564"></a>

## `ring-new-sized`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ring-new-sized</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). (<span class="tok-constructor">Int</span> -&gt; (<span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Ring</span>.<span class="tok-constructor">RingError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Ring</span>.<span class="tok-constructor">Ring</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span>)))))</code></pre>

Accept a signed external size, treating nonpositive sizes as an empty sequence before conversion.

[Source](/reference-sources/Data.Mutable.Ring.cinch) · line 164

<a id="value-72696e672d6e657874"></a>

## `ring-next`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ring-next</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Ring</span>.<span class="tok-constructor">Ring</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span>) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Ring</span>.<span class="tok-constructor">Ring</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span>))</code></pre>

Return the following cursor without changing either node.

[Source](/reference-sources/Data.Mutable.Ring.cinch) · line 178

<a id="value-72696e672d70726576"></a>

## `ring-prev`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ring-prev</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Ring</span>.<span class="tok-constructor">Ring</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span>) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Ring</span>.<span class="tok-constructor">Ring</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span>))</code></pre>

Return the preceding cursor without changing either node.

[Source](/reference-sources/Data.Mutable.Ring.cinch) · line 182

<a id="value-72696e672d72657461696e65642d6c656e677468"></a>

## `ring-retained-length`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ring-retained-length</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Ring</span>.<span class="tok-constructor">Ring</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span>) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Nat</span>)</code></pre>

Observe backing-arena retention, which may exceed the current component after a split.

[Source](/reference-sources/Data.Mutable.Ring.cinch) · line 213

<a id="value-72696e672d73616d65"></a>

## `ring-same`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ring-same</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Ring</span>.<span class="tok-constructor">Ring</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span>) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Ring</span>.<span class="tok-constructor">Ring</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span>) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Bool</span>))</code></pre>

Compare stable node identity after resolving any permanent arena forwarding.

[Source](/reference-sources/Data.Mutable.Ring.cinch) · line 186

<a id="value-72696e672d7365742d76616c7565"></a>

## `ring-set-value`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ring-set-value</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Ring</span>.<span class="tok-constructor">Ring</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>))</code></pre>

Replace a cursor's value without changing its node identity, neighbors or aliases.

[Source](/reference-sources/Data.Mutable.Ring.cinch) · line 195

<a id="value-72696e672d73696e676c65746f6e"></a>

## `ring-singleton`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ring-singleton</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). (<span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Ring</span>.<span class="tok-constructor">Ring</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span>))</code></pre>

Construct one nonempty, self-linked node without a fallible size conversion.

[Source](/reference-sources/Data.Mutable.Ring.cinch) · line 168

<a id="value-72696e672d746f2d6c697374"></a>

## `ring-to-list`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ring-to-list</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Ring</span>.<span class="tok-constructor">Ring</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span>)) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>))</code></pre>

Copy one cycle's current values into an immutable list, or return Nil for None.

[Source](/reference-sources/Data.Mutable.Ring.cinch) · line 398

<a id="value-72696e672d756e6c696e6b"></a>

## `ring-unlink`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ring-unlink</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Ring</span>.<span class="tok-constructor">Ring</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Int</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Ring</span>.<span class="tok-constructor">Ring</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span>))))</code></pre>

Remove the positive count modulo the current cycle length, returning its original following cursor.
Nonpositive counts return None unchanged. A positive multiple of the cycle
length leaves links unchanged and still returns Some original-successor.

[Source](/reference-sources/Data.Mutable.Ring.cinch) · line 343

<a id="value-72696e672d76616c7565"></a>

## `ring-value`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ring-value</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Ring</span>.<span class="tok-constructor">Ring</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span>) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-variable">a</span>)</code></pre>

Read a cursor's current value through its canonical node owner.

[Source](/reference-sources/Data.Mutable.Ring.cinch) · line 191

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Data/Mutable/Ring.cinch`. Source SHA-256: `ffbcfc7cfcd54354e261bf018d6f4758ca2f2c103f16fd0205fe7e3b687cc0dc`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
