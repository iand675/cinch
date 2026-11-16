# Data.Bytes.View

Owner-retaining checked byte views, stable compiled substring patterns and bounded search, cut and comparison operations.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Data.Bytes.View`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Bytes.View.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Bytes.Utf8](Data.Bytes.Utf8.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md).

Non-embedded source closure: [Data.Bytes.Utf8](Data.Bytes.Utf8.md), [Data.Bytes.View](Data.Bytes.View.md), [Data.Bytes.Word](Data.Bytes.Word.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Bounded mutable aliases retaining an entire fixed byte-buffer owner.
Views share observable mutations with their owner and overlapping views.
A small view retains the complete owner allocation until the view is released.
Snapshots and clones copy into fresh storage. Private backing and addresses
are never exposed; all reads, writes and range copies carry checked bounds.

</details>

# Types and constructors

<a id="type-427974655061747465726e"></a>

## `BytePattern`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">BytePattern</span></code></pre>

A stable private needle snapshot and its complete proper-prefix failure table.

[Source](/reference-sources/Data.Bytes.View.cinch) · line 121

<a id="type-4279746556696577"></a>

## `ByteView`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">ByteView</span></code></pre>

An opaque fixed range whose endpoint refinements preserve the owner's bounds.

[Source](/reference-sources/Data.Bytes.View.cinch) · line 31

<a id="type-42797465566965774572726f72"></a>

## `ByteViewError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">ByteViewError</span></code></pre>

A refused view range or complete-copy destination; refusals never mutate storage.

[Source](/reference-sources/Data.Bytes.View.cinch) · line 36

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ByteViewReversedRange</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteViewError</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ByteViewPastEnd</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteViewError</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ByteViewShortDestination</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteViewError</span>)))</code></pre>

# Values

<a id="value-627974652d7061747465726e2d6279746573"></a>

## `byte-pattern-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">byte-pattern-bytes</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">BytePattern</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Copy the compiled needle into fresh caller-owned storage.

[Source](/reference-sources/Data.Bytes.View.cinch) · line 173

<a id="value-627974652d7061747465726e2d6c656e677468"></a>

## `byte-pattern-length`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">byte-pattern-length</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">BytePattern</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Count exact needle bytes without exposing private snapshot storage.

[Source](/reference-sources/Data.Bytes.View.cinch) · line 169

<a id="value-627974652d766965772d636c6f6e65"></a>

## `byte-view-clone`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">byte-view-clone</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span>)</code></pre>

Clone a view into a new independent whole-buffer view, including empty ranges.

[Source](/reference-sources/Data.Bytes.View.cinch) · line 92

<a id="value-627974652d766965772d636f6d70617265"></a>

## `byte-view-compare`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">byte-view-compare</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span> -&gt; <span class="tok-constructor">I64</span>))</code></pre>

Compare raw view bytes lexicographically, returning exactly negative one, zero or one.

[Source](/reference-sources/Data.Bytes.View.cinch) · line 719

<a id="value-627974652d766965772d636f6e7461696e73"></a>

## `byte-view-contains`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">byte-view-contains</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

Test exact raw needle containment without copying the view.

[Source](/reference-sources/Data.Bytes.View.cinch) · line 741

<a id="value-627974652d766965772d636f6e7461696e732d7061747465726e"></a>

## `byte-view-contains-pattern`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">byte-view-contains-pattern</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">BytePattern</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

Test whether a compiled needle occurs, including the empty needle in an empty view.

[Source](/reference-sources/Data.Bytes.View.cinch) · line 598

<a id="value-627974652d766965772d636f70792d696e746f"></a>

## `byte-view-copy-into`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">byte-view-copy-into</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteViewError</span> <span class="tok-constructor">Nat</span>))))</code></pre>

Copy a complete view into a checked destination interval with overlap-safe range copying.
Destination refusal is atomic; the required and available sizes are preserved in the error.

[Source](/reference-sources/Data.Bytes.View.cinch) · line 97

<a id="value-627974652d766965772d636f756e74"></a>

## `byte-view-count`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">byte-view-count</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Nat</span>))</code></pre>

Count a raw needle with bounded malformed-byte counting for empty patterns.

[Source](/reference-sources/Data.Bytes.View.cinch) · line 745

<a id="value-627974652d766965772d636f756e742d7061747465726e"></a>

## `byte-view-count-pattern`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">byte-view-count-pattern</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">BytePattern</span> -&gt; <span class="tok-constructor">Nat</span>))</code></pre>

Count nonoverlapping compiled matches; an empty pattern counts bounded decoded scalars plus one.
Malformed or view-truncated bytes each make one replacement-scalar step without copying the view.

[Source](/reference-sources/Data.Bytes.View.cinch) · line 672

<a id="value-627974652d766965772d637574"></a>

## `byte-view-cut`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">byte-view-cut</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span>, <span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span>))))</code></pre>

Cut at the first raw match into aliases that retain the complete original owner.

[Source](/reference-sources/Data.Bytes.View.cinch) · line 764

<a id="value-627974652d766965772d6375742d6c617374"></a>

## `byte-view-cut-last`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">byte-view-cut-last</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span>, <span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span>))))</code></pre>

Cut at the final raw match, including overlapping candidates.

[Source](/reference-sources/Data.Bytes.View.cinch) · line 768

<a id="value-627974652d766965772d6375742d6c6173742d7061747465726e"></a>

## `byte-view-cut-last-pattern`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">byte-view-cut-last-pattern</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">BytePattern</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span>, <span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span>))))</code></pre>

Cut around the final compiled match, preserving overlapping last-match semantics.

[Source](/reference-sources/Data.Bytes.View.cinch) · line 633

<a id="value-627974652d766965772d6375742d7061747465726e"></a>

## `byte-view-cut-pattern`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">byte-view-cut-pattern</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">BytePattern</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span>, <span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span>))))</code></pre>

Cut around the first compiled match; empty needles produce empty-before and the complete view.

[Source](/reference-sources/Data.Bytes.View.cinch) · line 627

<a id="value-627974652d766965772d6375742d707265666978"></a>

## `byte-view-cut-prefix`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">byte-view-cut-prefix</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span>)))</code></pre>

Remove a raw prefix as an owner-retaining alias, including an unchanged empty prefix.

[Source](/reference-sources/Data.Bytes.View.cinch) · line 772

<a id="value-627974652d766965772d6375742d7072656669782d7061747465726e"></a>

## `byte-view-cut-prefix-pattern`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">byte-view-cut-prefix-pattern</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">BytePattern</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span>)))</code></pre>

Remove an exact compiled prefix as an owner-retaining alias, or report no match.

[Source](/reference-sources/Data.Bytes.View.cinch) · line 639

<a id="value-627974652d766965772d6375742d737566666978"></a>

## `byte-view-cut-suffix`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">byte-view-cut-suffix</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span>)))</code></pre>

Remove a raw suffix as an owner-retaining alias, including an unchanged empty suffix.

[Source](/reference-sources/Data.Bytes.View.cinch) · line 777

<a id="value-627974652d766965772d6375742d7375666669782d7061747465726e"></a>

## `byte-view-cut-suffix-pattern`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">byte-view-cut-suffix-pattern</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">BytePattern</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span>)))</code></pre>

Remove an exact compiled suffix as an owner-retaining alias, or report no match.

[Source](/reference-sources/Data.Bytes.View.cinch) · line 644

<a id="value-627974652d766965772d64726f70"></a>

## `byte-view-drop`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">byte-view-drop</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span>))</code></pre>

Drop up to the requested leading bytes, clamping at an empty view of the original end.

[Source](/reference-sources/Data.Bytes.View.cinch) · line 112

<a id="value-627974652d766965772d657175616c"></a>

## `byte-view-equal`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">byte-view-equal</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

Compare two exact view ranges using complete words and scalar fringes, preserving shared mutations.

[Source](/reference-sources/Data.Bytes.View.cinch) · line 685

<a id="value-627974652d766965772d676574"></a>

## `byte-view-get`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">byte-view-get</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">U8</span>)))</code></pre>

Read an exact relative byte, or no value when the coordinate is outside the view.

[Source](/reference-sources/Data.Bytes.View.cinch) · line 72

<a id="value-627974652d766965772d6861732d707265666978"></a>

## `byte-view-has-prefix`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">byte-view-has-prefix</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

Test an exact raw prefix with complete-word comparisons and no snapshots or pattern allocation.

[Source](/reference-sources/Data.Bytes.View.cinch) · line 749

<a id="value-627974652d766965772d6861732d7072656669782d7061747465726e"></a>

## `byte-view-has-prefix-pattern`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">byte-view-has-prefix-pattern</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">BytePattern</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

Test an exact compiled prefix using complete-word comparisons and checked view lengths.

[Source](/reference-sources/Data.Bytes.View.cinch) · line 602

<a id="value-627974652d766965772d6861732d737566666978"></a>

## `byte-view-has-suffix`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">byte-view-has-suffix</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

Test an exact raw suffix with no snapshots, pattern allocation or unrelated prefix scan.

[Source](/reference-sources/Data.Bytes.View.cinch) · line 756

<a id="value-627974652d766965772d6861732d7375666669782d7061747465726e"></a>

## `byte-view-has-suffix-pattern`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">byte-view-has-suffix-pattern</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">BytePattern</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

Test an exact compiled suffix without copying or scanning the unrelated prefix.

[Source](/reference-sources/Data.Bytes.View.cinch) · line 610

<a id="value-627974652d766965772d696e646578"></a>

## `byte-view-index`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">byte-view-index</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Nat</span>)))</code></pre>

Find a raw needle relative to a retained view, compiling one stable snapshot for this query.

[Source](/reference-sources/Data.Bytes.View.cinch) · line 733

<a id="value-627974652d766965772d696e6465782d7061747465726e"></a>

## `byte-view-index-pattern`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">byte-view-index-pattern</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">BytePattern</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Nat</span>)))</code></pre>

Find the first compiled needle match relative to the view; an empty needle matches zero.

[Source](/reference-sources/Data.Bytes.View.cinch) · line 573

<a id="value-627974652d766965772d69732d656d707479"></a>

## `byte-view-is-empty`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">byte-view-is-empty</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Test whether a retained range has no logical bytes.

[Source](/reference-sources/Data.Bytes.View.cinch) · line 59

<a id="value-627974652d766965772d6c6173742d696e646578"></a>

## `byte-view-last-index`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">byte-view-last-index</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Nat</span>)))</code></pre>

Find the last raw needle, including overlapping matches and the empty end position.

[Source](/reference-sources/Data.Bytes.View.cinch) · line 737

<a id="value-627974652d766965772d6c6173742d696e6465782d7061747465726e"></a>

## `byte-view-last-index-pattern`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">byte-view-last-index-pattern</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">BytePattern</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Nat</span>)))</code></pre>

Find the final compiled match, including overlaps; an empty needle matches the view end.

[Source](/reference-sources/Data.Bytes.View.cinch) · line 585

<a id="value-627974652d766965772d6c656e677468"></a>

## `byte-view-length`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">byte-view-length</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Count logical bytes in a view without copying or releasing its retained owner.

[Source](/reference-sources/Data.Bytes.View.cinch) · line 55

<a id="value-627974652d766965772d736574"></a>

## `byte-view-set`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">byte-view-set</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">Bool</span>)))</code></pre>

Write one exact relative byte and report whether it fit; refusal leaves all aliases unchanged.

[Source](/reference-sources/Data.Bytes.View.cinch) · line 79

<a id="value-627974652d766965772d736c696365"></a>

## `byte-view-slice`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">byte-view-slice</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteViewError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span>))))</code></pre>

Retain a checked half-open subrange relative to this view, preserving shared mutations.

[Source](/reference-sources/Data.Bytes.View.cinch) · line 63

<a id="value-627974652d766965772d736e617073686f74"></a>

## `byte-view-snapshot`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">byte-view-snapshot</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Copy exactly the logical range into fresh mutable storage without retaining the source owner.

[Source](/reference-sources/Data.Bytes.View.cinch) · line 86

<a id="value-627974652d766965772d74616b65"></a>

## `byte-view-take`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">byte-view-take</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span>))</code></pre>

Retain up to the requested leading bytes, clamping the count to this view's length.

[Source](/reference-sources/Data.Bytes.View.cinch) · line 105

<a id="value-62797465732d636f6d70696c652d7061747465726e"></a>

## `bytes-compile-pattern`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bytes-compile-pattern</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">BytePattern</span>)</code></pre>

Compile an independent needle snapshot and a reusable linear-search failure table.
Caller mutation after compilation never changes this pattern or its later search results.

[Source](/reference-sources/Data.Bytes.View.cinch) · line 162

<a id="value-62797465732d636f6e7461696e73"></a>

## `bytes-contains`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bytes-contains</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

Test raw needle containment in a complete fixed buffer.

[Source](/reference-sources/Data.Bytes.View.cinch) · line 792

<a id="value-62797465732d636f756e74"></a>

## `bytes-count`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bytes-count</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Nat</span>))</code></pre>

Count nonoverlapping raw matches, or decoded scalars plus one for an empty needle.

[Source](/reference-sources/Data.Bytes.View.cinch) · line 796

<a id="value-62797465732d637574"></a>

## `bytes-cut`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bytes-cut</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span>, <span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span>))))</code></pre>

Cut a complete fixed buffer at its first raw match into retained mutable aliases.

[Source](/reference-sources/Data.Bytes.View.cinch) · line 808

<a id="value-62797465732d6375742d6c617374"></a>

## `bytes-cut-last`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bytes-cut-last</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span>, <span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span>))))</code></pre>

Cut a complete fixed buffer at its final raw match into retained mutable aliases.

[Source](/reference-sources/Data.Bytes.View.cinch) · line 812

<a id="value-62797465732d6375742d707265666978"></a>

## `bytes-cut-prefix`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bytes-cut-prefix</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span>)))</code></pre>

Remove an exact prefix from a complete buffer while retaining its whole owner.

[Source](/reference-sources/Data.Bytes.View.cinch) · line 816

<a id="value-62797465732d6375742d737566666978"></a>

## `bytes-cut-suffix`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bytes-cut-suffix</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span>)))</code></pre>

Remove an exact suffix from a complete buffer while retaining its whole owner.

[Source](/reference-sources/Data.Bytes.View.cinch) · line 820

<a id="value-62797465732d6861732d707265666978"></a>

## `bytes-has-prefix`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bytes-has-prefix</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

Test whether a complete fixed buffer starts with exact raw bytes.

[Source](/reference-sources/Data.Bytes.View.cinch) · line 800

<a id="value-62797465732d6861732d737566666978"></a>

## `bytes-has-suffix`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bytes-has-suffix</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

Test whether a complete fixed buffer ends with exact raw bytes.

[Source](/reference-sources/Data.Bytes.View.cinch) · line 804

<a id="value-62797465732d696e646578"></a>

## `bytes-index`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bytes-index</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Nat</span>)))</code></pre>

Find the first raw needle in a complete fixed buffer as an optional byte offset.

[Source](/reference-sources/Data.Bytes.View.cinch) · line 784

<a id="value-62797465732d6c6173742d696e646578"></a>

## `bytes-last-index`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bytes-last-index</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Nat</span>)))</code></pre>

Find the last raw needle in a complete fixed buffer, including overlaps.

[Source](/reference-sources/Data.Bytes.View.cinch) · line 788

<a id="value-62797465732d76696577"></a>

## `bytes-view`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bytes-view</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span>)</code></pre>

Retain the whole fixed mutable buffer without copying its contents.

[Source](/reference-sources/Data.Bytes.View.cinch) · line 45

<a id="value-62797465732d766965772d72616e6765"></a>

## `bytes-view-range`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bytes-view-range</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteViewError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-constructor">View</span>.<span class="tok-constructor">ByteView</span>))))</code></pre>

Retain an explicitly checked half-open owner range, including empty ranges at the end.

[Source](/reference-sources/Data.Bytes.View.cinch) · line 49

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Data/Bytes/View.cinch`. Source SHA-256: `c55e57028c56d673d75c29ba91936ee614c3631addda328abe75c12a1bbf3e7e`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
