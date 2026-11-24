# Data.Mutable.Map

A mutable map with captured key policies and deletion-aware live traversal.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Data.Mutable.Map`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Mutable.Map.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Core](Data.Core.md), [Data.Hash](Data.Hash.md), [Data.Machine](Data.Machine.md), [Data.Mutable.HashTable](Data.Mutable.HashTable.md), [Data.Mutable.LinkedList](Data.Mutable.LinkedList.md), [Data.Mutable.Var](Data.Mutable.Var.md), [Data.Nat](Data.Nat.md), [Relation.Eq](Relation.Eq.md), [Relation.Ord](Relation.Ord.md).

Non-embedded source closure: [Data.Mutable.HashTable](Data.Mutable.HashTable.md), [Data.Mutable.LinkedList](Data.Mutable.LinkedList.md), [Data.Mutable.Map](Data.Mutable.Map.md), [Data.Mutable.Var](Data.Mutable.Var.md).

<details><summary>Behaviour and boundary notes from the source</summary>

A mutable map with captured key policies and deletion-aware live traversal.
Each traversal captures entry identities when invoked, then reads current
memberships and values before callbacks. New entries are skipped; removed
entries are never yielded. Traversal setup scans the digest index and retains
the original entry handles until it returns, including removed payloads.

</details>

# Types and constructors

<a id="type-4d61704572726f72"></a>

## `MapError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">MapError</span></code></pre>

A checked refusal; completed prefixes of bulk updates remain applied.

[Source](/reference-sources/Data.Mutable.Map.cinch) · line 34

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MapCapacityExceeded</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Map</span>.<span class="tok-constructor">MapError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MapIdentityViolation</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Map</span>.<span class="tok-constructor">MapError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MapMissingDestination</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Map</span>.<span class="tok-constructor">MapError</span></code></pre>

<a id="type-4d61704b6579506f6c696379"></a>

## `MapKeyPolicy`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">MapKeyPolicy</span> (<span class="tok-variable">k</span> :: <span class="tok-constructor">Type</span>)</code></pre>

Pure key hashing and comparison captured together for the map's lifetime.
Equal keys must have equal hashes, and both functions must remain stable
while a key is stored. Comparison must be symmetric and transitive where
it holds; non-reflexive keys are permitted.

[Source](/reference-sources/Data.Mutable.Map.cinch) · line 29

<a id="type-4d757461626c654d6170"></a>

## `MutableMap`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">MutableMap</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">k</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">v</span> :: <span class="tok-constructor">Type</span>)</code></pre>

A private stable-entry store indexed by captured key digests.

[Source](/reference-sources/Data.Mutable.Map.cinch) · line 43

# Values

<a id="value-6d61702d6b65792d706f6c696379"></a>

## `map-key-policy`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">map-key-policy</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">k</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-variable">k</span> -&gt; <span class="tok-constructor">Nat</span>) -&gt; ((<span class="tok-variable">k</span> -&gt; (<span class="tok-variable">k</span> -&gt; <span class="tok-constructor">Bool</span>)) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Map</span>.<span class="tok-constructor">MapKeyPolicy</span> <span class="tok-variable">k</span>)))</code></pre>

Select pure key functions explicitly, including non-reflexive comparisons.

[Source](/reference-sources/Data.Mutable.Map.cinch) · line 57

<a id="value-6d61702d736f757263652d66726f6d2d6c697374"></a>

## `map-source-from-list`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">map-source-from-list</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">ef</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-variable">k</span>, <span class="tok-variable">v</span>) -&gt; ((<span class="tok-variable">k</span> -&gt; (<span class="tok-variable">v</span> -&gt;{<span class="tok-variable">ef</span>} <span class="tok-constructor">Bool</span>)) -&gt;{<span class="tok-variable">ef</span>} <span class="tok-constructor">Unit</span>))</code></pre>

Supply pairs in list order, ending immediately when the consumer returns False.

[Source](/reference-sources/Data.Mutable.Map.cinch) · line 221

<a id="value-6d757461626c652d6d61702d616c6c"></a>

## `mutable-map-all`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mutable-map-all</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">ef</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Map</span>.<span class="tok-constructor">MutableMap</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> -&gt; ((<span class="tok-variable">k</span> -&gt; (<span class="tok-variable">v</span> -&gt;{<span class="tok-variable">ef</span>} <span class="tok-constructor">Bool</span>)) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span> | <span class="tok-variable">ef</span>} <span class="tok-constructor">Bool</span>))</code></pre>

Visit live original entries in unspecified order; True means exhaustion, False means stop.
Capture occurs on each invocation. Deleted entries skip, overwritten entries
expose current values, and entries inserted during the invocation are excluded.

[Source](/reference-sources/Data.Mutable.Map.cinch) · line 205

<a id="value-6d757461626c652d6d61702d636c656172"></a>

## `mutable-map-clear`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mutable-map-clear</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">k</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">v</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Map</span>.<span class="tok-constructor">MutableMap</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>)</code></pre>

Clear both stores and invalidate every entry membership captured by older traversals.

[Source](/reference-sources/Data.Mutable.Map.cinch) · line 177

<a id="value-6d757461626c652d6d61702d636c6f6e65"></a>

## `mutable-map-clone`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mutable-map-clone</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">k</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">v</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Map</span>.<span class="tok-constructor">MutableMap</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Map</span>.<span class="tok-constructor">MapError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Map</span>.<span class="tok-constructor">MutableMap</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>)))</code></pre>

Shallowly clone every entry into independent container cells, preserving non-reflexive key multiplicity.

[Source](/reference-sources/Data.Mutable.Map.cinch) · line 282

<a id="value-6d757461626c652d6d61702d636f6c6c656374"></a>

## `mutable-map-collect`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mutable-map-collect</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">ef</span> <span class="tok-variable">r</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">h</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Hash</span>.<span class="tok-constructor">Hash</span> <span class="tok-variable">k</span>, <span class="tok-variable">eq</span> :: <span class="tok-constructor">Relation</span>.<span class="tok-constructor">Eq</span>.<span class="tok-constructor">Eq</span> <span class="tok-variable">k</span>) =&gt; (((<span class="tok-variable">k</span> -&gt; (<span class="tok-variable">v</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span> | <span class="tok-variable">ef</span>} <span class="tok-constructor">Bool</span>)) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span> | <span class="tok-variable">ef</span>} <span class="tok-constructor">Unit</span>) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span> | <span class="tok-variable">ef</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Map</span>.<span class="tok-constructor">MapError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Map</span>.<span class="tok-constructor">MutableMap</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>))</code></pre>

Collect a pair source while capturing the current hashing and equality methods once.

[Source](/reference-sources/Data.Mutable.Map.cinch) · line 252

<a id="value-6d757461626c652d6d61702d636f6c6c6563742d77697468"></a>

## `mutable-map-collect-with`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mutable-map-collect-with</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">ef</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Map</span>.<span class="tok-constructor">MapKeyPolicy</span> <span class="tok-variable">k</span> -&gt; (((<span class="tok-variable">k</span> -&gt; (<span class="tok-variable">v</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span> | <span class="tok-variable">ef</span>} <span class="tok-constructor">Bool</span>)) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span> | <span class="tok-variable">ef</span>} <span class="tok-constructor">Unit</span>) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span> | <span class="tok-variable">ef</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Map</span>.<span class="tok-constructor">MapError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Map</span>.<span class="tok-constructor">MutableMap</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>)))</code></pre>

Collect a pair source into fresh storage under an explicit captured policy, with last binding wins.

[Source](/reference-sources/Data.Mutable.Map.cinch) · line 246

<a id="value-6d757461626c652d6d61702d636f7079"></a>

## `mutable-map-copy`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mutable-map-copy</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">s</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">k</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">v</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Map</span>.<span class="tok-constructor">MutableMap</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Map</span>.<span class="tok-constructor">MutableMap</span> <span class="tok-variable">s</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>, <span class="tok-constructor">Mut</span> <span class="tok-variable">s</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Map</span>.<span class="tok-constructor">MapError</span> <span class="tok-constructor">Unit</span>)))</code></pre>

Copy source bindings into the destination, overwriting under its policy; self-copy visits original identities once.

[Source](/reference-sources/Data.Mutable.Map.cinch) · line 276

<a id="value-6d757461626c652d6d61702d64656c657465"></a>

## `mutable-map-delete`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mutable-map-delete</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">k</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">v</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Map</span>.<span class="tok-constructor">MutableMap</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>) -&gt; (<span class="tok-variable">k</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Map</span>.<span class="tok-constructor">MapError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">v</span>))))</code></pre>

Delete a key under ordinary key comparison; non-reflexive keys remain unfindable.

[Source](/reference-sources/Data.Mutable.Map.cinch) · line 165

<a id="value-6d757461626c652d6d61702d64656c6574652d6966"></a>

## `mutable-map-delete-if`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mutable-map-delete-if</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">ef</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Map</span>.<span class="tok-constructor">MutableMap</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> -&gt; ((<span class="tok-variable">k</span> -&gt; (<span class="tok-variable">v</span> -&gt;{<span class="tok-variable">ef</span>} <span class="tok-constructor">Bool</span>)) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span> | <span class="tok-variable">ef</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Map</span>.<span class="tok-constructor">MapError</span> <span class="tok-constructor">Unit</span>))</code></pre>

Delete selected keys after each predicate call; non-reflexive keys remain, and callback replacements are deleted by key.

[Source](/reference-sources/Data.Mutable.Map.cinch) · line 297

<a id="value-6d757461626c652d6d61702d657175616c"></a>

## `mutable-map-equal`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mutable-map-equal</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">s</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">k</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">v</span> :: <span class="tok-constructor">Type</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">values</span> :: <span class="tok-constructor">Relation</span>.<span class="tok-constructor">Eq</span>.<span class="tok-constructor">Eq</span> <span class="tok-variable">v</span>) =&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Map</span>.<span class="tok-constructor">MutableMap</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Map</span>.<span class="tok-constructor">MutableMap</span> <span class="tok-variable">s</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>, <span class="tok-constructor">Mut</span> <span class="tok-variable">s</span>} <span class="tok-constructor">Bool</span>))</code></pre>

Compare bindings using value equality and compatible captured key equivalences.

[Source](/reference-sources/Data.Mutable.Map.cinch) · line 315

<a id="value-6d757461626c652d6d61702d657175616c2d6279"></a>

## `mutable-map-equal-by`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mutable-map-equal-by</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span> <span class="tok-variable">ef</span> <span class="tok-variable">r</span> <span class="tok-variable">s</span> . ((<span class="tok-variable">a</span> -&gt; (<span class="tok-variable">b</span> -&gt;{<span class="tok-variable">ef</span>} <span class="tok-constructor">Bool</span>)) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Map</span>.<span class="tok-constructor">MutableMap</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Map</span>.<span class="tok-constructor">MutableMap</span> <span class="tok-variable">s</span> <span class="tok-variable">k</span> <span class="tok-variable">b</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>, <span class="tok-constructor">Mut</span> <span class="tok-variable">s</span> | <span class="tok-variable">ef</span>} <span class="tok-constructor">Bool</span>)))</code></pre>

Compare sizes and corresponding bindings under compatible key equivalences and an explicit value predicate.
Values may have different types. Missing keys and False predicates stop
immediately; map identity never shortcuts non-reflexive key or value tests.

[Source](/reference-sources/Data.Mutable.Map.cinch) · line 306

<a id="value-6d757461626c652d6d61702d696e73657274"></a>

## `mutable-map-insert`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mutable-map-insert</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">ef</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Map</span>.<span class="tok-constructor">MutableMap</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> -&gt; (((<span class="tok-variable">k</span> -&gt; (<span class="tok-variable">v</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span> | <span class="tok-variable">ef</span>} <span class="tok-constructor">Bool</span>)) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span> | <span class="tok-variable">ef</span>} <span class="tok-constructor">Unit</span>) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span> | <span class="tok-variable">ef</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Map</span>.<span class="tok-constructor">MapError</span> <span class="tok-constructor">Unit</span>))</code></pre>

Consume a stop-obeying pair source; later equivalent keys win and errors preserve the applied prefix.
Yield calls after refusal or after the source returns are inert and return False.

[Source](/reference-sources/Data.Mutable.Map.cinch) · line 237

<a id="value-6d757461626c652d6d61702d69732d656d707479"></a>

## `mutable-map-is-empty`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mutable-map-is-empty</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">k</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">v</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Map</span>.<span class="tok-constructor">MutableMap</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Bool</span>)</code></pre>

Test whether the map has no live bindings.

[Source](/reference-sources/Data.Mutable.Map.cinch) · line 96

<a id="value-6d757461626c652d6d61702d6b657973"></a>

## `mutable-map-keys`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mutable-map-keys</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">ef</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Map</span>.<span class="tok-constructor">MutableMap</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> -&gt; ((<span class="tok-variable">k</span> -&gt;{<span class="tok-variable">ef</span>} <span class="tok-constructor">Bool</span>) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span> | <span class="tok-variable">ef</span>} <span class="tok-constructor">Bool</span>))</code></pre>

Visit current keys with the same identity, mutation and stop contract as pair traversal.

[Source](/reference-sources/Data.Mutable.Map.cinch) · line 209

<a id="value-6d757461626c652d6d61702d6c6f6f6b7570"></a>

## `mutable-map-lookup`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mutable-map-lookup</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">k</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">v</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Map</span>.<span class="tok-constructor">MutableMap</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>) -&gt; (<span class="tok-variable">k</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">v</span>)))</code></pre>

Find a value under the captured key policy; a non-reflexive key may miss itself.

[Source](/reference-sources/Data.Mutable.Map.cinch) · line 111

<a id="value-6d757461626c652d6d61702d6e6577"></a>

## `mutable-map-new`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mutable-map-new</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">k</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">v</span> :: <span class="tok-constructor">Type</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">h</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Hash</span>.<span class="tok-constructor">Hash</span> <span class="tok-variable">k</span>, <span class="tok-variable">eq</span> :: <span class="tok-constructor">Relation</span>.<span class="tok-constructor">Eq</span>.<span class="tok-constructor">Eq</span> <span class="tok-variable">k</span>) =&gt; (<span class="tok-constructor">Unit</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Map</span>.<span class="tok-constructor">MutableMap</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>))</code></pre>

Capture the current hashing and equality methods once in a fresh map.

[Source](/reference-sources/Data.Mutable.Map.cinch) · line 88

<a id="value-6d757461626c652d6d61702d6e65772d77697468"></a>

## `mutable-map-new-with`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mutable-map-new-with</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">k</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">v</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Map</span>.<span class="tok-constructor">MapKeyPolicy</span> <span class="tok-variable">k</span>) -&gt; (<span class="tok-constructor">Unit</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Map</span>.<span class="tok-constructor">MutableMap</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>)))</code></pre>

Allocate fresh container cells with an explicit immutable key policy.

[Source](/reference-sources/Data.Mutable.Map.cinch) · line 81

<a id="value-6d757461626c652d6d61702d736574"></a>

## `mutable-map-set`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mutable-map-set</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">k</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">v</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Map</span>.<span class="tok-constructor">MutableMap</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>) -&gt; (<span class="tok-variable">k</span> -&gt; (<span class="tok-variable">v</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Map</span>.<span class="tok-constructor">MapError</span> <span class="tok-constructor">Unit</span>))))</code></pre>

Set one binding, retaining its identity on overwrite and storing the new key representative.

[Source](/reference-sources/Data.Mutable.Map.cinch) · line 139

<a id="value-6d757461626c652d6d61702d73697a65"></a>

## `mutable-map-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mutable-map-size</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">k</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">v</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Map</span>.<span class="tok-constructor">MutableMap</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Nat</span>)</code></pre>

Observe the number of live bindings, including distinct non-reflexive keys.

[Source](/reference-sources/Data.Mutable.Map.cinch) · line 92

<a id="value-6d757461626c652d6d61702d736f75726365"></a>

## `mutable-map-source`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mutable-map-source</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">ef</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Map</span>.<span class="tok-constructor">MutableMap</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> -&gt; ((<span class="tok-variable">k</span> -&gt; (<span class="tok-variable">v</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span> | <span class="tok-variable">ef</span>} <span class="tok-constructor">Bool</span>)) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span> | <span class="tok-variable">ef</span>} <span class="tok-constructor">Unit</span>))</code></pre>

Supply the current map to a pair consumer, obeying its stop result.

[Source](/reference-sources/Data.Mutable.Map.cinch) · line 217

<a id="value-6d757461626c652d6d61702d76616c756573"></a>

## `mutable-map-values`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mutable-map-values</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">ef</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Map</span>.<span class="tok-constructor">MutableMap</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> -&gt; ((<span class="tok-variable">v</span> -&gt;{<span class="tok-variable">ef</span>} <span class="tok-constructor">Bool</span>) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span> | <span class="tok-variable">ef</span>} <span class="tok-constructor">Bool</span>))</code></pre>

Visit current values, preserving duplicates and stopping immediately on False.

[Source](/reference-sources/Data.Mutable.Map.cinch) · line 213

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Data/Mutable/Map.cinch`. Source SHA-256: `0fc095f0bdcbe26f9ba3f26e5db8cb76891698784b744bb76e2e420dc14eb7c6`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
