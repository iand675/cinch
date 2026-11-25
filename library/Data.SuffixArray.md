# Data.SuffixArray

Immutable O(n log n) suffix arrays over raw bytes with binary-search lookup, regex search and checked versioned binary serialization.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Data.SuffixArray</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Data.SuffixArray/source/">View source</a><a href="/packages/search/?q=%2BData.SuffixArray">Search module</a><a href="/reference-sources/Data.SuffixArray.cinch" download="Data.SuffixArray.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Int](Data.Int.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Ordering](Data.Ordering.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Text.Regex](Text.Regex.md).

<details class="doc-notes"><summary>Module notes</summary>

In-memory suffix array for fast substring search, sorted match lookup,
regular expression scanning, and validated binary serialization.

A SuffixArray owns an immutable snapshot of its indexed byte sequence and
the lexicographically sorted array of all suffix starting offsets. Neither
construction nor inspection aliases caller-owned byte buffers, so mutating
a buffer passed to suffix-array-new or returned by suffix-array-bytes never
invalidates the index.

Construction uses prefix doubling with linear counting sort in O(n log n)
time and O(n) space over packed little-endian offset tables, with early exit
once all suffix ranks become unique. Substring queries run two binary
searches in O(log(n) * len(pattern) + k) time. Serialization uses a
versioned binary framing ("CSA1") whose decoder checks header integrity,
exact payload length, permutation validity, and full lexicographic order in
O(n) time before accepting an index.

</details>

<details class="doc-synopsis" open><summary>Synopsis (13 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-SuffixArray">SuffixArray</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">SuffixArray</strong></code></li><li><a class="syn-name" href="#t-SuffixArrayError">SuffixArrayError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">SuffixArrayError</strong></code></li><li><a class="syn-name" href="#v-suffix-array-at">suffix-array-at</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">suffix-array-at</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.SuffixArray/#t-SuffixArray" title="Data.SuffixArray.SuffixArray">SuffixArray</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-suffix-array-bytes">suffix-array-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">suffix-array-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.SuffixArray/#t-SuffixArray" title="Data.SuffixArray.SuffixArray">SuffixArray</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-suffix-array-count">suffix-array-count</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">suffix-array-count</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.SuffixArray/#t-SuffixArray" title="Data.SuffixArray.SuffixArray">SuffixArray</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-suffix-array-decode">suffix-array-decode</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">suffix-array-decode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.SuffixArray/#t-SuffixArrayError" title="Data.SuffixArray.SuffixArrayError">SuffixArrayError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.SuffixArray/#t-SuffixArray" title="Data.SuffixArray.SuffixArray">SuffixArray</a></code></li><li><a class="syn-name" href="#v-suffix-array-encode">suffix-array-encode</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">suffix-array-encode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.SuffixArray/#t-SuffixArray" title="Data.SuffixArray.SuffixArray">SuffixArray</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-suffix-array-error-message">suffix-array-error-message</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">suffix-array-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.SuffixArray/#t-SuffixArrayError" title="Data.SuffixArray.SuffixArrayError">SuffixArrayError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-suffix-array-find-all-regex">suffix-array-find-all-regex</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">suffix-array-find-all-regex</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.SuffixArray/#t-SuffixArray" title="Data.SuffixArray.SuffixArray">SuffixArray</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Regex/#t-Regex" title="Text.Regex.Regex">Regex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>)</code></li><li><a class="syn-name" href="#v-suffix-array-len">suffix-array-len</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">suffix-array-len</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.SuffixArray/#t-SuffixArray" title="Data.SuffixArray.SuffixArray">SuffixArray</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-suffix-array-lookup">suffix-array-lookup</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">suffix-array-lookup</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.SuffixArray/#t-SuffixArray" title="Data.SuffixArray.SuffixArray">SuffixArray</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-suffix-array-lookup-sorted">suffix-array-lookup-sorted</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">suffix-array-lookup-sorted</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.SuffixArray/#t-SuffixArray" title="Data.SuffixArray.SuffixArray">SuffixArray</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-suffix-array-new">suffix-array-new</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">suffix-array-new</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.SuffixArray/#t-SuffixArray" title="Data.SuffixArray.SuffixArray">SuffixArray</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-SuffixArray">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">SuffixArray</strong></code><span class="doc-sig-links"><a href="#t-SuffixArray" title="Permalink">#</a><a href="/packages/stdlib/Data.SuffixArray/source/#L39" title="View source at line 39">Source</a></span></div>
<div class="doc-body">

An immutable suffix array over a private snapshot of the indexed bytes
together with its packed sorted suffix offsets.

</div>
</article>

<article class="doc-entry" id="t-SuffixArrayError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">SuffixArrayError</strong></code><span class="doc-sig-links"><a href="#t-SuffixArrayError" title="Permalink">#</a><a href="/packages/stdlib/Data.SuffixArray/source/#L44" title="View source at line 44">Source</a></span></div>
<div class="doc-body">

Reasons why decoding a serialized suffix array can fail.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-SuffixArrayInvalidHeader"><code><strong class="tok-constructor">SuffixArrayInvalidHeader</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.SuffixArray/#t-SuffixArrayError" title="Data.SuffixArray.SuffixArrayError">SuffixArrayError</a></code></li><li id="k-SuffixArrayTruncated"><code><strong class="tok-constructor">SuffixArrayTruncated</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.SuffixArray/#t-SuffixArrayError" title="Data.SuffixArray.SuffixArrayError">SuffixArrayError</a></code></li><li id="k-SuffixArrayCorruptIndex"><code><strong class="tok-constructor">SuffixArrayCorruptIndex</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.SuffixArray/#t-SuffixArrayError" title="Data.SuffixArray.SuffixArrayError">SuffixArrayError</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-suffix-array-at">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">suffix-array-at</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.SuffixArray/#t-SuffixArray" title="Data.SuffixArray.SuffixArray">SuffixArray</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-suffix-array-at" title="Permalink">#</a><a href="/packages/stdlib/Data.SuffixArray/source/#L77" title="View source at line 77">Source</a></span></div>
<div class="doc-body">

Read the suffix starting offset at position pos in the sorted suffix array.

</div>
</article>

<article class="doc-entry" id="v-suffix-array-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">suffix-array-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.SuffixArray/#t-SuffixArray" title="Data.SuffixArray.SuffixArray">SuffixArray</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-suffix-array-bytes" title="Permalink">#</a><a href="/packages/stdlib/Data.SuffixArray/source/#L67" title="View source at line 67">Source</a></span></div>
<div class="doc-body">

Return a fresh copy of the indexed bytes so the caller cannot mutate internal storage.

</div>
</article>

<article class="doc-entry" id="v-suffix-array-count">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">suffix-array-count</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.SuffixArray/#t-SuffixArray" title="Data.SuffixArray.SuffixArray">SuffixArray</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-suffix-array-count" title="Permalink">#</a><a href="/packages/stdlib/Data.SuffixArray/source/#L85" title="View source at line 85">Source</a></span></div>
<div class="doc-body">

Count all occurrences of pattern in the indexed bytes in O(log(n) * len(pattern)) time.
An empty pattern returns zero.

</div>
</article>

<article class="doc-entry" id="v-suffix-array-decode">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">suffix-array-decode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.SuffixArray/#t-SuffixArrayError" title="Data.SuffixArray.SuffixArrayError">SuffixArrayError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.SuffixArray/#t-SuffixArray" title="Data.SuffixArray.SuffixArray">SuffixArray</a></code><span class="doc-sig-links"><a href="#v-suffix-array-decode" title="Permalink">#</a><a href="/packages/stdlib/Data.SuffixArray/source/#L151" title="View source at line 151">Source</a></span></div>
<div class="doc-body">

Decode and validate a "CSA1" binary suffix array representation.
Rejects malformed headers, truncated payloads, trailing bytes, out-of-range offsets,
duplicate offsets, and unsorted suffix tables.

</div>
</article>

<article class="doc-entry" id="v-suffix-array-encode">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">suffix-array-encode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.SuffixArray/#t-SuffixArray" title="Data.SuffixArray.SuffixArray">SuffixArray</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-suffix-array-encode" title="Permalink">#</a><a href="/packages/stdlib/Data.SuffixArray/source/#L135" title="View source at line 135">Source</a></span></div>
<div class="doc-body">

Encode a suffix array into the versioned "CSA1" binary representation containing
the 16-byte header, raw indexed bytes, and packed little-endian suffix offsets.

</div>
</article>

<article class="doc-entry" id="v-suffix-array-error-message">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">suffix-array-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.SuffixArray/#t-SuffixArrayError" title="Data.SuffixArray.SuffixArrayError">SuffixArrayError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-suffix-array-error-message" title="Permalink">#</a><a href="/packages/stdlib/Data.SuffixArray/source/#L53" title="View source at line 53">Source</a></span></div>
<div class="doc-body">

Human-readable explanation of a suffix array decode error.

</div>
</article>

<article class="doc-entry" id="v-suffix-array-find-all-regex">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">suffix-array-find-all-regex</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.SuffixArray/#t-SuffixArray" title="Data.SuffixArray.SuffixArray">SuffixArray</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Regex/#t-Regex" title="Text.Regex.Regex">Regex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>)</code><span class="doc-sig-links"><a href="#v-suffix-array-find-all-regex" title="Permalink">#</a><a href="/packages/stdlib/Data.SuffixArray/source/#L123" title="View source at line 123">Source</a></span></div>
<div class="doc-body">

Find up to limit non-overlapping matches of a compiled regular expression as (start, end)
byte offset pairs in ascending order. A negative limit returns all matches; zero returns Nil.
Returns Nil when the indexed bytes are not valid UTF-8.

</div>
</article>

<article class="doc-entry" id="v-suffix-array-len">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">suffix-array-len</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.SuffixArray/#t-SuffixArray" title="Data.SuffixArray.SuffixArray">SuffixArray</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-suffix-array-len" title="Permalink">#</a><a href="/packages/stdlib/Data.SuffixArray/source/#L72" title="View source at line 72">Source</a></span></div>
<div class="doc-body">

Number of indexed bytes (and suffixes) in the suffix array.

</div>
</article>

<article class="doc-entry" id="v-suffix-array-lookup">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">suffix-array-lookup</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.SuffixArray/#t-SuffixArray" title="Data.SuffixArray.SuffixArray">SuffixArray</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-suffix-array-lookup" title="Permalink">#</a><a href="/packages/stdlib/Data.SuffixArray/source/#L99" title="View source at line 99">Source</a></span></div>
<div class="doc-body">

Return at most limit starting offsets where pat occurs in the indexed bytes.
A negative limit returns all matches; zero or an empty pattern returns Nil.
Offsets are returned in suffix-array order in O(log(n) * len(pat) + k) time.

</div>
</article>

<article class="doc-entry" id="v-suffix-array-lookup-sorted">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">suffix-array-lookup-sorted</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.SuffixArray/#t-SuffixArray" title="Data.SuffixArray.SuffixArray">SuffixArray</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-suffix-array-lookup-sorted" title="Permalink">#</a><a href="/packages/stdlib/Data.SuffixArray/source/#L116" title="View source at line 116">Source</a></span></div>
<div class="doc-body">

Return at most limit starting offsets where pat occurs, sorted in ascending offset order.
A negative limit returns all matches in ascending document order; zero or an empty pattern returns Nil.

</div>
</article>

<article class="doc-entry" id="v-suffix-array-new">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">suffix-array-new</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.SuffixArray/#t-SuffixArray" title="Data.SuffixArray.SuffixArray">SuffixArray</a></code><span class="doc-sig-links"><a href="#v-suffix-array-new" title="Permalink">#</a><a href="/packages/stdlib/Data.SuffixArray/source/#L60" title="View source at line 60">Source</a></span></div>
<div class="doc-body">

Build an immutable suffix array over a fresh snapshot copy of the input bytes.

</div>
</article>
